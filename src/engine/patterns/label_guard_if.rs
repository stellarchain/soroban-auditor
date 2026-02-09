use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelGuardIf;

impl LabelGuardIf {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelGuardIf {
    fn name(&self) -> &'static str {
        "label_guard_if"
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
                if let Some(rewritten) = try_guard_if(&label, &header, &new_body) {
                    out.push(rewritten);
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

fn try_guard_if(label: &str, header: &str, body: &[Node]) -> Option<Node> {
    let mut iter = body.iter().filter(|node| !is_empty_line(node));
    let first = iter.next()?;
    let if_block = match first {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => (header, body),
        _ => return None,
    };
    if !is_single_break_label(if_block.1, label) {
        return None;
    }

    let mut rest: Vec<Node> = iter.cloned().collect();
    if contains_break_label(&rest, label) {
        return None;
    }

    let cond = extract_if_condition(if_block.0)?;
    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let new_header = format!("{indent}if !({cond}) {{");
    let new_footer = format!("{indent}}}");

    Some(Node::Block {
        kind: BlockKind::If,
        label: None,
        header: new_header,
        body: rest.drain(..).collect(),
        footer: new_footer,
    })
}

fn extract_if_condition(header: &str) -> Option<String> {
    let trimmed = header.trim();
    if !trimmed.starts_with("if ") || !trimmed.ends_with('{') {
        return None;
    }
    let inner = trimmed.trim_end_matches('{').trim();
    Some(inner["if ".len()..].trim().to_string())
}

fn is_single_break_label(body: &[Node], label: &str) -> bool {
    let mut lines: Vec<String> = Vec::new();
    for node in body {
        match node {
            Node::Line(line) => {
                if !line.trim().is_empty() {
                    lines.push(line.trim().to_string());
                }
            }
            Node::Block { .. } => return false,
        }
    }
    lines.len() == 1 && lines[0] == format!("break '{};", label)
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

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}
