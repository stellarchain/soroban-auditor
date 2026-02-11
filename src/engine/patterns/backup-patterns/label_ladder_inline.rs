use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct LabelLadderInline;

impl LabelLadderInline {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLadderInline {
    fn name(&self) -> &'static str {
        "label_ladder_inline"
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
                    if let Some(repl) = try_inline_ladder(&body) {
                        out.extend(repl);
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

fn try_inline_ladder(body: &[Node]) -> Option<Vec<Node>> {
    let (chain, match_block, prefix_lines) = collect_chain_and_match(body)?;
    if chain.len() < 2 {
        return None;
    }
    let arms = parse_match_arms(match_block)?;
    let mut label_bodies: HashMap<String, Vec<String>> = HashMap::new();
    for (label, tail) in chain {
        let flat = flatten_nodes(&tail);
        if !tail_is_simple(&flat) {
            return None;
        }
        label_bodies.insert(label, flat);
    }

    let (match_header, match_footer) = match match_block {
        Node::Block { header, footer, .. } => (header, footer),
        _ => return None,
    };
    let indent = match_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let mut out_lines: Vec<String> = Vec::new();
    out_lines.extend(prefix_lines);
    out_lines.push(match_header.clone());
    for (pat, label) in arms {
        let body_lines = label_bodies.get(&label).cloned().unwrap_or_default();
        out_lines.push(format!("{}    {} => {{", indent, pat));
        for l in body_lines {
            out_lines.push(l);
        }
        out_lines.push(format!("{}    }}", indent));
    }
    out_lines.push(match_footer.clone());
    Some(out_lines.into_iter().map(Node::Line).collect())
}

fn collect_chain_and_match(
    body: &[Node],
) -> Option<(Vec<(String, Vec<Node>)>, &Node, Vec<String>)> {
    let mut chain: Vec<(String, Vec<Node>)> = Vec::new();
    let mut current_body: &[Node] = body;
    let mut match_block: Option<&Node> = None;
    let mut prefix_lines: Vec<String> = Vec::new();
    loop {
        let (label, inner, tail, prefix) = split_labeled_loop(current_body)?;
        if !prefix.is_empty() {
            if !prefix_lines.is_empty() {
                return None;
            }
            prefix_lines = flatten_nodes(&prefix);
        }
        chain.push((label, tail));
        match inner {
            Node::Block {
                kind: BlockKind::Loop,
                body,
                ..
            } => {
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
    Some((chain, m, prefix_lines))
}

fn split_labeled_loop(body: &[Node]) -> Option<(String, &Node, Vec<Node>, Vec<Node>)> {
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
    let (prefix, inner_first, inner_tail) = split_prefix_and_inner(inner)?;
    Some((label, inner_first, inner_tail, prefix))
}

fn split_prefix_and_inner(inner: &[Node]) -> Option<(Vec<Node>, &Node, Vec<Node>)> {
    let mut idx: Option<usize> = None;
    for (i, node) in inner.iter().enumerate() {
        match node {
            Node::Block {
                kind: BlockKind::Loop,
                ..
            } => {
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
    let prefix = inner[..i].to_vec();
    let inner_first = &inner[i];
    let tail = inner[i + 1..].to_vec();
    Some((prefix, inner_first, tail))
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
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn tail_is_simple(lines: &[String]) -> bool {
    for l in lines {
        let t = l.trim();
        if t.starts_with("break '") || t.starts_with("continue '") {
            return false;
        }
        if t.starts_with("match ") {
            return false;
        }
        if t.contains(": loop {") {
            return false;
        }
    }
    true
}
