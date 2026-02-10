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
                    out.extend(rewritten);
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

fn try_guard_if(label: &str, header: &str, body: &[Node]) -> Option<Vec<Node>> {
    let mut guard_idx = None;
    let mut cond = String::new();

    for (idx, node) in body.iter().enumerate() {
        match node {
            Node::Line(line) if line.trim().is_empty() => continue,
            Node::Line(_) => continue,
            Node::Block {
                kind: BlockKind::If,
                header,
                body,
                ..
            } => {
                if !is_single_break_label(body, label) {
                    return None;
                }
                cond = extract_if_condition(header)?;
                guard_idx = Some(idx);
                break;
            }
            Node::Block { .. } => return None,
        }
    }

    let guard_idx = guard_idx?;
    let prefix = body[..guard_idx].to_vec();
    let mut rest: Vec<Node> = body[guard_idx + 1..].to_vec();
    if rest.is_empty() {
        return None;
    }
    if contains_label_control(&prefix, label) || contains_label_control(&rest, label) {
        return None;
    }

    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let new_header = format!("{indent}if !({cond}) {{");
    let new_footer = format!("{indent}}}");

    let rewritten_if = Node::Block {
        kind: BlockKind::If,
        label: None,
        header: new_header,
        body: rest.drain(..).collect(),
        footer: new_footer,
    };

    let mut rewritten = prefix;
    rewritten.push(rewritten_if);
    Some(rewritten)
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

fn contains_label_control(nodes: &[Node], label: &str) -> bool {
    let break_target = format!("break '{};", label);
    let continue_target = format!("continue '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                let trimmed = line.trim();
                if trimmed == break_target || trimmed == continue_target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_label_control(body, label) {
                    return true;
                }
            }
        }
    }
    false
}
