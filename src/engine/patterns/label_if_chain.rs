use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelIfChain;

impl LabelIfChain {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelIfChain {
    fn name(&self) -> &'static str {
        "label_if_chain"
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
    let mut out = Vec::new();
    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::Other,
                label: Some(label),
                header,
                body,
                footer,
            } => {
                let new_body = rewrite(body, changed);
                if let Some(lines) = try_if_chain(&label, &header, &new_body) {
                    out.extend(lines.into_iter().map(Node::Line));
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Other,
                    label: Some(label),
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

fn try_if_chain(label: &str, header: &str, body: &[Node]) -> Option<Vec<String>> {
    let mut idx = 0;
    let mut ifs: Vec<(String, Vec<String>)> = Vec::new();
    let mut tail: Vec<Node> = Vec::new();

    while idx < body.len() {
        let node = &body[idx];
        if is_empty_line(node) {
            idx += 1;
            continue;
        }
        match node {
            Node::Block {
                kind: BlockKind::If,
                header,
                body,
                ..
            } => {
                let cond = extract_if_condition(header)?;
                let mut body_lines = flatten_nodes(body);
                if !strip_trailing_break_label(&mut body_lines, label) {
                    return None;
                }
                if contains_break_label(body, label) || contains_continue_label(body, label) {
                    return None;
                }
                ifs.push((cond, body_lines));
                idx += 1;
                continue;
            }
            _ => {
                tail.extend_from_slice(&body[idx..]);
                break;
            }
        }
    }

    if ifs.is_empty() {
        return None;
    }

    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut lines: Vec<String> = Vec::new();
    for (i, (cond, body_lines)) in ifs.iter().enumerate() {
        if i == 0 {
            lines.push(format!("{indent}if {cond} {{"));
        } else {
            lines.push(format!("{indent}else if {cond} {{"));
        }
        for line in body_lines {
            lines.push(line.clone());
        }
        lines.push(format!("{indent}}}"));
    }
    if !tail.is_empty() {
        lines.push(format!("{indent}else {{"));
        lines.extend(flatten_nodes(&tail));
        lines.push(format!("{indent}}}"));
    }
    Some(lines)
}

fn extract_if_condition(header: &str) -> Option<String> {
    let trimmed = header.trim();
    if !trimmed.starts_with("if ") || !trimmed.ends_with('{') {
        return None;
    }
    let inner = trimmed.trim_end_matches('{').trim();
    Some(inner["if ".len()..].trim().to_string())
}

fn strip_trailing_break_label(lines: &mut Vec<String>, label: &str) -> bool {
    let target = format!("break '{};", label);
    while let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
            continue;
        }
        if last.trim() == target {
            lines.pop();
            return true;
        }
        return false;
    }
    false
}

fn contains_break_label(nodes: &[Node], label: &str) -> bool {
    let target = format!("break '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_break_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
}

fn contains_continue_label(nodes: &[Node], label: &str) -> bool {
    let target = format!("continue '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_continue_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}
