use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct StatusResultGuardLabelPattern;

impl StatusResultGuardLabelPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StatusResultGuardLabelPattern {
    fn name(&self) -> &'static str {
        "status_result_guard_label"
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
        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&new_nodes),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::Other,
                label: Some(label),
                header,
                body,
                footer,
            } => {
                let mut body = rewrite(body, changed);
                let mut replaced_in_this_block = false;
                rewrite_guard_breaks_to_unreachable(&mut body, &label, &mut replaced_in_this_block);
                if replaced_in_this_block {
                    *changed = true;
                }

                let label_still_used = contains_label_control(&body, &label);
                if !label_still_used && replaced_in_this_block {
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: None,
                        header: format!("{indent}{{"),
                        body,
                        footer,
                    });
                    *changed = true;
                } else {
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header,
                        body,
                        footer,
                    });
                }
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => out.push(Node::Block {
                kind,
                label,
                header,
                body: rewrite(body, changed),
                footer,
            }),
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn rewrite_guard_breaks_to_unreachable(nodes: &mut [Node], label: &str, changed: &mut bool) {
    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::If,
                header,
                body,
                ..
            } => {
                rewrite_guard_breaks_to_unreachable(body, label, changed);
                if is_guard_condition(header)
                    && is_single_break_label(body, label)
                    && !contains_nested_control(body)
                {
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *body = vec![Node::Line(format!("{indent}    unreachable!();"))];
                    *changed = true;
                }
            }
            Node::Block { body, .. } => rewrite_guard_breaks_to_unreachable(body, label, changed),
            Node::Line(_) => {}
        }
    }
}

fn is_guard_condition(header: &str) -> bool {
    let trimmed = header.trim();
    if !trimmed.starts_with("if ") || !trimmed.ends_with('{') {
        return false;
    }
    let cond = trimmed
        .trim_end_matches('{')
        .trim()
        .strip_prefix("if ")
        .unwrap_or("")
        .trim();
    cond.contains("try_from_val")
        || cond.contains("is_ok()")
        || cond.contains("_0_i32 == 1")
        || cond.contains("== 0")
        || cond.contains("!= 0")
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

fn contains_nested_control(body: &[Node]) -> bool {
    for node in body {
        if let Node::Block { .. } = node {
            return true;
        }
    }
    false
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
                if trimmed.contains(&break_target) || trimmed.contains(&continue_target) {
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
