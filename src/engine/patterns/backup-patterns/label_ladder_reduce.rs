use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct LabelLadderReduce;

impl LabelLadderReduce {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLadderReduce {
    fn name(&self) -> &'static str {
        "label_ladder_reduce"
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
                kind: BlockKind::Loop,
                label,
                header,
                body,
                footer,
            } => {
                if label.is_some() {
                    if let Some(new_inner) = try_reduce_ladder(&body) {
                        out.push(Node::Block {
                            kind: BlockKind::Loop,
                            label,
                            header,
                            body: vec![new_inner],
                            footer,
                        });
                        *changed = true;
                        continue;
                    }
                }
                let new_body = rewrite(body, changed);
                if label.is_none() {
                    if let Some(inlined) = try_elide_unlabeled_loop(&new_body) {
                        out.push(inlined);
                        *changed = true;
                        continue;
                    }
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
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
    }
    out
}

fn try_reduce_ladder(body: &[Node]) -> Option<Node> {
    let (label1, match_block, label_tails, chain_labels, prefix_lines) =
        collect_chain_and_match(body)?;
    if label_tails.len() < 5 {
        return None;
    }
    let (match_header, match_footer) = match match_block {
        Node::Block { header, footer, .. } => (header.clone(), footer.clone()),
        _ => return None,
    };
    let indent = match_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let arms = parse_match_arms(match_block)?;

    let mut lines: Vec<String> = Vec::new();
    lines.extend(prefix_lines);
    lines.push(match_header);
    for (pat, label) in arms {
        let body_lines = label_tails.get(&label).cloned().unwrap_or_default();
        lines.push(format!("{}    {} => {{", indent, pat));
        for l in body_lines {
            lines.push(l);
        }
        lines.push(format!("{}    }}", indent));
    }
    lines.push(match_footer);

    let inner = Node::Block {
        kind: BlockKind::Loop,
        label: Some(label1.clone()),
        header: format!("{}'{}: loop {{", indent, label1),
        body: lines.into_iter().map(Node::Line).collect(),
        footer: format!("{}}}", indent),
    };
    Some(inner)
}

fn collect_chain_and_match(
    body: &[Node],
) -> Option<(
    String,
    &Node,
    HashMap<String, Vec<String>>,
    Vec<String>,
    Vec<String>,
)> {
    let mut current_body: &[Node] = body;
    let mut label_tails: HashMap<String, Vec<String>> = HashMap::new();
    let mut chain_labels: Vec<String> = Vec::new();
    let mut prefix_lines: Vec<String> = Vec::new();
    let mut label1: Option<String> = None;
    let mut match_block: Option<&Node> = None;

    loop {
        let (label, inner, tail, prefix) = split_labeled_loop(current_body)?;
        if label1.is_none() {
            label1 = Some(label.clone());
        }
        chain_labels.push(label.clone());
        if !prefix.is_empty() {
            prefix_lines.extend(flatten_nodes(&prefix));
        }
        label_tails.insert(label, flatten_nodes(&tail));
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

    let label1 = label1?;
    let match_block = match_block?;
    if has_invalid_control(&label_tails, &chain_labels, &label1) {
        return None;
    }
    Some((label1, match_block, label_tails, chain_labels, prefix_lines))
}

fn split_labeled_loop(body: &[Node]) -> Option<(String, &Node, Vec<Node>, Vec<Node>)> {
    let (label, inner) = find_labeled_loop(body)?;
    let (prefix, inner_first, inner_tail) = split_prefix_and_inner(inner)?;
    Some((label, inner_first, inner_tail, prefix))
}

fn find_labeled_loop(body: &[Node]) -> Option<(String, &Vec<Node>)> {
    for node in body {
        if let Node::Block {
            kind: BlockKind::Loop,
            label: Some(label),
            body,
            ..
        } = node
        {
            return Some((label.clone(), body));
        }
    }
    None
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
    let mut i = 0usize;
    while i < body.len() {
        let line = match &body[i] {
            Node::Line(l) => l.trim().to_string(),
            _ => {
                i += 1;
                continue;
            }
        };
        if let Some((pat, label)) = parse_match_arm_label(&line) {
            out.push((pat, label));
            i += 1;
            continue;
        }
        if let Some(pat) = parse_match_arm_block_start(&line) {
            let mut label: Option<String> = None;
            let mut depth = 0i32;
            i += 1;
            while i < body.len() {
                let inner_line = match &body[i] {
                    Node::Line(l) => l.trim().to_string(),
                    _ => {
                        i += 1;
                        continue;
                    }
                };
                if inner_line.ends_with("{") {
                    depth += 1;
                }
                if inner_line == "}" || inner_line == "}," {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                }
                if label.is_none() {
                    if let Some(found) = parse_control_label(&inner_line) {
                        label = Some(found);
                    }
                }
                i += 1;
            }
            if let Some(label) = label {
                out.push((pat, label));
            }
        }
        i += 1;
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn parse_match_arm_label(line: &str) -> Option<(String, String)> {
    let (pat, rest) = if let Some(pos) = line.find("=> break '") {
        (
            line[..pos].trim().trim_end_matches(',').to_string(),
            &line[pos + "=> break '".len()..],
        )
    } else if let Some(pos) = line.find("=> continue '") {
        (
            line[..pos].trim().trim_end_matches(',').to_string(),
            &line[pos + "=> continue '".len()..],
        )
    } else {
        return None;
    };
    let label = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>();
    if pat.is_empty() || label.is_empty() {
        None
    } else {
        Some((pat, label))
    }
}

fn parse_match_arm_block_start(line: &str) -> Option<String> {
    if let Some(pos) = line.find("=> {") {
        let pat = line[..pos].trim().trim_end_matches(',').to_string();
        if pat.is_empty() {
            None
        } else {
            Some(pat)
        }
    } else {
        None
    }
}

fn try_elide_unlabeled_loop(body: &[Node]) -> Option<Node> {
    if body.is_empty() {
        return None;
    }
    let mut non_trivial: Vec<&Node> = Vec::new();
    for node in body {
        match node {
            Node::Line(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed == "break;" {
                    continue;
                }
            }
            _ => {}
        }
        non_trivial.push(node);
    }
    if non_trivial.len() != 1 {
        return None;
    }
    match non_trivial[0] {
        Node::Block { .. } => Some(non_trivial[0].clone()),
        Node::Line(_) => None,
    }
}

fn has_invalid_control(
    label_tails: &HashMap<String, Vec<String>>,
    chain_labels: &[String],
    root_label: &str,
) -> bool {
    for lines in label_tails.values() {
        for line in lines {
            let trimmed = line.trim();
            if let Some(label) = parse_control_label(trimmed) {
                if chain_labels.contains(&label) && label != root_label {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_control_label(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix("break '") {
        return Some(
            rest.chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect(),
        );
    }
    if let Some(rest) = line.strip_prefix("continue '") {
        return Some(
            rest.chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect(),
        );
    }
    None
}
