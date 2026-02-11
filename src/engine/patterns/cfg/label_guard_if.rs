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
                if let Some(rewritten_body) = try_guard_if(&label, &new_body) {
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header,
                        body: rewritten_body,
                        footer,
                    });
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

fn try_guard_if(label: &str, body: &[Node]) -> Option<Vec<Node>> {
    let mut guard_idx = None;
    let mut cond = String::new();
    let mut then_without_break: Vec<Node> = Vec::new();
    let mut if_indent = String::new();

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
                if let Some(stripped_then) = strip_terminal_break_label(body, label) {
                    cond = extract_if_condition(header)?;
                    then_without_break = stripped_then;
                    if_indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    guard_idx = Some(idx);
                    break;
                }
                continue;
            }
            Node::Block { .. } => continue,
        }
    }

    let guard_idx = guard_idx?;
    let prefix = body[..guard_idx].to_vec();
    if contains_label_control(&prefix, label) {
        return None;
    }
    if contains_label_control(&then_without_break, label) {
        return None;
    }
    let rest: Vec<Node> = body[guard_idx + 1..].to_vec();
    let rewritten_tail = if rest.is_empty() {
        rest
    } else {
        let mut nested = rest;
        while let Some(next) = try_guard_if(label, &nested) {
            nested = next;
        }
        nested
    };

    if then_without_break.iter().all(is_empty_line) {
        let rewritten_if = Node::Block {
            kind: BlockKind::If,
            label: None,
            header: format!("{if_indent}if !({cond}) {{"),
            body: rewritten_tail,
            footer: format!("{if_indent}}}"),
        };

        let mut rewritten = prefix;
        rewritten.push(rewritten_if);
        return Some(rewritten);
    }

    let rewritten_if = Node::Block {
        kind: BlockKind::If,
        label: None,
        header: format!("{if_indent}if {cond} {{"),
        body: then_without_break,
        footer: format!("{if_indent}}}"),
    };

    let mut rewritten = prefix;
    rewritten.push(rewritten_if);
    if !rewritten_tail.is_empty() {
        rewritten.push(Node::Block {
            kind: BlockKind::Else,
            label: None,
            header: format!("{if_indent}else {{"),
            body: Vec::new(),
            footer: format!("{if_indent}}}"),
        });
        // Replace placeholder to avoid cloning tail in the common case.
        if let Some(Node::Block { body, .. }) = rewritten.last_mut() {
            *body = rewritten_tail;
        }
    }
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

fn strip_terminal_break_label(body: &[Node], label: &str) -> Option<Vec<Node>> {
    let mut out = body.to_vec();
    while let Some(last) = out.last() {
        if is_empty_line(last) {
            out.pop();
            continue;
        }
        break;
    }
    match out.pop() {
        Some(Node::Line(line)) if line.trim() == format!("break '{};", label) => Some(out),
        _ => None,
    }
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

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}
