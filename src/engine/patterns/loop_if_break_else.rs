use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopIfBreakElse;

impl LoopIfBreakElse {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopIfBreakElse {
    fn name(&self) -> &'static str {
        "loop_if_break_else"
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
                kind: BlockKind::Loop,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite(body, changed);
                if let Some(converted) = try_loop_if_break_else(&header, &new_body) {
                    out.push(converted);
                    *changed = true;
                    continue;
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

fn try_loop_if_break_else(header: &str, body: &[Node]) -> Option<Node> {
    if !header.trim_start().starts_with("loop ") {
        return None;
    }
    let mut prefix: Vec<Node> = Vec::new();
    let mut idx = 0usize;
    while idx < body.len() {
        match &body[idx] {
            Node::Line(line) => {
                if line.trim().is_empty() {
                    prefix.push(body[idx].clone());
                    idx += 1;
                    continue;
                }
                if line.trim().starts_with("let ")
                    || line.trim().starts_with("slot_")
                    || line.contains("mload")
                {
                    prefix.push(body[idx].clone());
                    idx += 1;
                    continue;
                }
                break;
            }
            _ => break,
        }
    }
    let if_block = match body.get(idx) {
        Some(Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        }) => (header, body),
        _ => return None,
    };
    let mut if_body = if_block.1.clone();
    if !strip_trailing_break(&mut if_body) {
        return None;
    }
    if contains_break_or_continue_outside_loops(&if_body) {
        return None;
    }
    let else_body = body[idx + 1..].to_vec();
    if contains_break_or_continue_outside_loops(&else_body) {
        return None;
    }
    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let if_node = Node::Block {
        kind: BlockKind::If,
        label: None,
        header: if_block.0.clone(),
        body: if_body,
        footer: format!("{indent}}}"),
    };
    let else_node = Node::Block {
        kind: BlockKind::Else,
        label: None,
        header: format!("{indent}else {{"),
        body: else_body,
        footer: format!("{indent}}}"),
    };
    let mut body_nodes = Vec::new();
    body_nodes.extend(prefix);
    body_nodes.push(if_node);
    body_nodes.push(else_node);
    Some(Node::Block {
        kind: BlockKind::Other,
        label: None,
        header: format!("{indent}{{"),
        body: body_nodes,
        footer: format!("{indent}}}"),
    })
}

fn strip_trailing_break(nodes: &mut Vec<Node>) -> bool {
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            Node::Line(line) if line.trim() == "break;" => {
                nodes.pop();
                return true;
            }
            _ => return false,
        }
    }
    false
}

fn contains_break_or_continue_outside_loops(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == "break;"
                    || t == "continue;"
                    || t.starts_with("break '")
                    || t.starts_with("continue '")
                {
                    return true;
                }
            }
            Node::Block {
                kind: BlockKind::Loop,
                ..
            } => {
                // Ignore break/continue inside nested loops.
            }
            Node::Block { body, .. } => {
                if contains_break_or_continue_outside_loops(body) {
                    return true;
                }
            }
        }
    }
    false
}
