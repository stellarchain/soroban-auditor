use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct TrailingUnreachableCleanupPattern;

impl TrailingUnreachableCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for TrailingUnreachableCleanupPattern {
    fn name(&self) -> &'static str {
        "trailing_unreachable_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let is_unit_return = !block.header.contains("->");
        if is_unit_return {
            let mut body = block.body.clone();
            if let Some(last_idx) = body.iter().rposition(|l| !l.trim().is_empty()) {
                if body[last_idx].trim() == "unreachable!();"
                {
                    let prev_idx = body[..last_idx].iter().rposition(|l| !l.trim().is_empty());
                    let prev_is_safe_tail = prev_idx
                        .and_then(|prev| body.get(prev))
                        .map(|l| {
                            let t = l.trim();
                            t == "}"
                                || (t.starts_with("self.global0 =") && t.contains("wrapping_add("))
                        })
                        .unwrap_or(false);
                    if prev_is_safe_tail {
                        body.remove(last_idx);
                        return Some(FunctionBlock {
                            header: block.header.clone(),
                            body,
                            footer: block.footer.clone(),
                            indent: block.indent.clone(),
                            name: block.name.clone(),
                        });
                    }
                }
            }
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_sequence(nodes, &mut changed, 0, is_unit_return);
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

fn rewrite_sequence(
    nodes: Vec<Node>,
    changed: &mut bool,
    depth: usize,
    is_unit_return: bool,
) -> Vec<Node> {
    let mut transformed = Vec::with_capacity(nodes.len());
    for node in nodes {
        match node {
            Node::Line(line) => transformed.push(Node::Line(line)),
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let body = rewrite_sequence(body, changed, depth + 1, is_unit_return);
                transformed.push(Node::Block {
                    kind,
                    label,
                    header,
                    body,
                    footer,
                });
            }
        }
    }

    let last_non_empty_idx = transformed
        .iter()
        .rposition(|n| !matches!(n, Node::Line(line) if line.trim().is_empty()));
    let mut out = Vec::with_capacity(transformed.len());
    for (idx, node) in transformed.into_iter().enumerate() {
        if is_unreachable_line(&node) {
            if let Some(prev) = last_non_empty_node(&out) {
                if node_guarantees_return(prev) {
                    *changed = true;
                    continue;
                }
                if depth == 0
                    && is_unit_return
                    && Some(idx) == last_non_empty_idx
                    && node_falls_through(prev)
                {
                    *changed = true;
                    continue;
                }
            }
        }
        out.push(node);
    }
    out
}

fn is_unreachable_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim() == "unreachable!();")
}

fn last_non_empty_node(nodes: &[Node]) -> Option<&Node> {
    nodes.iter().rev().find(|n| match n {
        Node::Line(line) => !line.trim().is_empty(),
        Node::Block { .. } => true,
    })
}

fn node_guarantees_return(node: &Node) -> bool {
    match node {
        Node::Line(line) => line.trim() == "return;",
        Node::Block {
            kind,
            label,
            header,
            body,
            ..
        } => {
            if *kind != BlockKind::Other || label.is_some() || header.trim() != "{" {
                return false;
            }
            if contains_break_or_continue(body) {
                return false;
            }
            body_last_non_empty_line(body)
                .map(|line| line.trim() == "return;")
                .unwrap_or(false)
        }
    }
}

fn node_falls_through(node: &Node) -> bool {
    match node {
        Node::Line(_) => false,
        Node::Block {
            kind,
            label,
            header,
            body,
            ..
        } => {
            if *kind != BlockKind::Other || label.is_some() || header.trim() != "{" {
                return false;
            }
            body_last_non_empty_line(body)
                .map(|line| {
                    let t = line.trim();
                    !t.is_empty()
                        && t != "unreachable!();"
                        && t != "panic_with_error!();"
                        && !t.starts_with("return")
                })
                .unwrap_or(false)
        }
    }
}

fn body_last_non_empty_line(nodes: &[Node]) -> Option<&String> {
    for node in nodes.iter().rev() {
        match node {
            Node::Line(line) if !line.trim().is_empty() => return Some(line),
            Node::Block { .. } => return None,
            _ => {}
        }
    }
    None
}

fn contains_break_or_continue(nodes: &[Node]) -> bool {
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
            Node::Block { body, .. } => {
                if contains_break_or_continue(body) {
                    return true;
                }
            }
        }
    }
    false
}
