use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct LabelLadderFlatten;

impl LabelLadderFlatten {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLadderFlatten {
    fn name(&self) -> &'static str {
        "label_ladder_flatten"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite(nodes, &mut changed);
        if !changed {
            return None;
        }
        let new_body = flatten_nodes(&new_nodes);
        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    for node in nodes {
        match node {
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                if kind == BlockKind::Loop && label.is_some() {
                    if let Some(new_body) = try_flatten_chain(&body) {
                        out.push(Node::Block {
                            kind,
                            label,
                            header,
                            body: new_body,
                            footer,
                        });
                        *changed = true;
                        continue;
                    }
                }
                let new_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn try_flatten_chain(body: &[Node]) -> Option<Vec<Node>> {
    let (chain, match_block) = collect_chain_and_match(body)?;
    if chain.len() < 2 {
        return None;
    }
    let mut label_tails: HashMap<String, Vec<String>> = HashMap::new();
    for (label, tail) in &chain {
        label_tails.insert(label.clone(), flatten_nodes(tail));
    }

    let arms = parse_match_arms(match_block)?;
    let (match_header, match_footer) = match match_block {
        Node::Block {
            header, footer, ..
        } => (header, footer),
        _ => return None,
    };
    let indent = match_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut new_lines: Vec<String> = Vec::new();
    new_lines.push(match_header.clone());
    for (pat, label) in arms {
        let tail = label_tails.get(&label).cloned().unwrap_or_default();
        new_lines.push(format!("{}    {} => {{", indent, pat));
        for line in tail {
            new_lines.push(line);
        }
        new_lines.push(format!("{}    }}", indent));
    }
    new_lines.push(match_footer.clone());

    Some(new_lines.into_iter().map(Node::Line).collect())
}

fn collect_chain_and_match(body: &[Node]) -> Option<(Vec<(String, Vec<Node>)>, &Node)> {
    let mut chain: Vec<(String, Vec<Node>)> = Vec::new();
    let mut current_body: &[Node] = body;
    let mut match_block: Option<&Node> = None;

    loop {
        let (label, inner, tail, prefix) = split_labeled_loop(current_body)?;
        let mut combined_tail = prefix;
        combined_tail.extend(tail);
        chain.push((label, combined_tail));
        match inner {
            Node::Block { kind: BlockKind::Loop, body, .. } => {
                current_body = body;
                continue;
            }
            Node::Block { header, .. } if header.trim_start().starts_with("match ") => {
                match_block = Some(inner);
                break;
            }
            _ => return None,
        }
    }

    let m = match_block?;
    Some((chain, m))
}

fn split_labeled_loop(body: &[Node]) -> Option<(String, &Node, Vec<Node>, Vec<Node>)> {
    if body.is_empty() {
        return None;
    }
    let first = body.first()?;
    let (label, inner) = match first {
        Node::Block {
            kind: BlockKind::Loop,
            label: Some(label),
            body,
            ..
        } => (label.clone(), body),
        _ => return None,
    };
    let (prefix_nodes, inner_first, inner_tail) = split_prefix_and_inner(inner)?;
    let mut tail_nodes: Vec<Node> = inner_tail;
    trim_tail_breaks(&mut tail_nodes);
    Some((label, inner_first, tail_nodes, prefix_nodes))
}

fn split_prefix_and_inner(inner: &[Node]) -> Option<(Vec<Node>, &Node, Vec<Node>)> {
    let mut idx: Option<usize> = None;
    for (i, node) in inner.iter().enumerate() {
        match node {
            Node::Block { kind: BlockKind::Loop, .. } => {
                idx = Some(i);
                break;
            }
            Node::Block { header, .. } if header.trim_start().starts_with("match ") => {
                idx = Some(i);
                break;
            }
            _ => {}
        }
    }
    let i = idx?;
    let prefix_nodes = inner[..i].to_vec();
    let inner_first = &inner[i];
    let tail_nodes = inner[i + 1..].to_vec();
    Some((prefix_nodes, inner_first, tail_nodes))
}

fn parse_match_arms(match_block: &Node) -> Option<Vec<(String, String)>> {
    let body = match match_block {
        Node::Block { body, .. } => body,
        _ => return None,
    };
    let mut out: Vec<(String, String)> = Vec::new();
    for node in body {
        let line = match node {
            Node::Line(l) => l.trim().to_string(),
            _ => continue,
        };
        if let Some(pos) = line.find("=> break '") {
            let pat = line[..pos].trim().trim_end_matches(',').to_string();
            let rest = &line[pos + "=> break '".len()..];
            let label = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<String>();
            if !pat.is_empty() && !label.is_empty() {
                out.push((pat, label));
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn trim_tail_breaks(nodes: &mut Vec<Node>) {
    while let Some(last) = nodes.last() {
        if let Node::Line(line) = last {
            let t = line.trim();
            if t == "break;" || t.starts_with("break '") {
                nodes.pop();
                continue;
            }
        }
        break;
    }
}
