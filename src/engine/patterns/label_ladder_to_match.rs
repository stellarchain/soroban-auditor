use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::{HashMap, HashSet};

pub struct LabelLadderToMatch;

impl LabelLadderToMatch {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLadderToMatch {
    fn name(&self) -> &'static str {
        "label_ladder_to_match"
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
    let mut i = 0;
    while i < nodes.len() {
        if let Node::Block {
            kind: BlockKind::Loop,
            label: Some(_),
            header,
            body,
            footer,
        } = &nodes[i]
        {
            if let Some(repl) = try_rewrite_ladder(header, body, footer) {
                out.extend(repl.into_iter().map(Node::Line));
                *changed = true;
                i += 1;
                continue;
            }
        }
        match nodes[i].clone() {
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
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
        i += 1;
    }
    out
}

fn try_rewrite_ladder(header: &str, body: &[Node], _footer: &str) -> Option<Vec<String>> {
    let (chain, match_block, prefix_lines, ladder_labels, outer_labels) = collect_chain_and_match(body)?;
    if chain.len() < 2 {
        return None;
    }
    let arms = parse_match_arms(match_block)?;
    let (match_header, match_footer) = match match_block {
        Node::Block { header, footer, .. } => (header.clone(), footer.clone()),
        _ => return None,
    };
    let indent = match_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut label_bodies: HashMap<String, Vec<String>> = HashMap::new();
    for (label, tail) in chain {
        let mut flat = flatten_nodes(&tail);
        if !tail_is_simple(&flat, &ladder_labels, &outer_labels) {
            return None;
        }
        strip_breaks(&mut flat, &ladder_labels, &outer_labels);
        label_bodies.insert(label, flat);
    }

    let mut out: Vec<String> = Vec::new();
    out.push(header.to_string());
    out.extend(prefix_lines);
    out.push(match_header);
    for (pat, label) in arms {
        let body_lines = label_bodies.get(&label).cloned().unwrap_or_default();
        out.push(format!("{}    {} => {{", indent, pat));
        for l in body_lines {
            out.push(l);
        }
        out.push(format!("{}    }}", indent));
    }
    out.push(match_footer);
    out.push(format!("{}    break;", indent));
    out.push("}".to_string());
    Some(out)
}

fn collect_chain_and_match(
    body: &[Node],
) -> Option<(Vec<(String, Vec<Node>)>, &Node, Vec<String>, HashSet<String>, HashSet<String>)> {
    let mut chain: Vec<(String, Vec<Node>)> = Vec::new();
    let mut current_body: &[Node] = body;
    let mut match_block: Option<&Node> = None;
    let mut prefix_lines: Vec<String> = Vec::new();
    let mut labels: HashSet<String> = HashSet::new();
    let mut outer_labels: Vec<String> = Vec::new();
    loop {
        let (label, inner, tail, prefix) = split_labeled_loop(current_body)?;
        labels.insert(label.clone());
        if outer_labels.len() < 2 {
            outer_labels.push(label.clone());
        }
        if !prefix.is_empty() {
            if !prefix_lines.is_empty() {
                return None;
            }
            prefix_lines = flatten_nodes(&prefix);
        }
        chain.push((label, tail));
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
    let outer_set: HashSet<String> = outer_labels.into_iter().collect();
    Some((chain, m, prefix_lines, labels, outer_set))
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
    if out.is_empty() { None } else { Some(out) }
}

fn tail_is_simple(
    lines: &[String],
    ladder_labels: &HashSet<String>,
    outer_labels: &HashSet<String>,
) -> bool {
    for l in lines {
        let t = l.trim();
        if t.starts_with("continue '") {
            return false;
        }
        if t.starts_with("break '") {
            let label = t.trim_start_matches("break '").trim_end_matches(';');
            if ladder_labels.contains(label) && !outer_labels.contains(label) {
                return false;
            }
        }
    }
    true
}

fn strip_breaks(
    lines: &mut Vec<String>,
    ladder_labels: &HashSet<String>,
    outer_labels: &HashSet<String>,
) {
    lines.retain(|l| {
        let t = l.trim();
        if t == "break;" {
            return false;
        }
        if t.starts_with("break '") {
            let label = t.trim_start_matches("break '").trim_end_matches(';');
            if ladder_labels.contains(label) && outer_labels.contains(label) {
                return false;
            }
        }
        true
    });
}
