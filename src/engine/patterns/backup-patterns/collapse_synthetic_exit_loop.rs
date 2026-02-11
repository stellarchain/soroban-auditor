use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct CollapseSyntheticExitLoopPattern;

impl CollapseSyntheticExitLoopPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for CollapseSyntheticExitLoopPattern {
    fn name(&self) -> &'static str {
        "collapse_synthetic_exit_loop"
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
    let mut out = Vec::new();
    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::Other,
                label: Some(label),
                header,
                body,
                footer,
            } if label.starts_with("__exit_") => {
                let rewritten_body = rewrite(body, changed);
                if let Some(collapsed) = collapse_exit_block(&label, &rewritten_body) {
                    out.push(collapsed);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Other,
                    label: Some(label),
                    header,
                    body: rewritten_body,
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
                let rewritten_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: rewritten_body,
                    footer,
                });
            }
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn collapse_exit_block(label: &str, body: &[Node]) -> Option<Node> {
    let sig = significant_indices(body);
    if sig.is_empty() {
        return None;
    }
    let loop_idx = sig[0];
    let loop_node = match &body[loop_idx] {
        Node::Block {
            kind: BlockKind::Loop,
            label: _,
            header,
            body,
            footer,
        } => Node::Block {
            kind: BlockKind::Loop,
            label: None,
            header: header.clone(),
            body: rewrite_break_to_unlabeled(body, label),
            footer: footer.clone(),
        },
        _ => return None,
    };

    for idx in sig.iter().skip(1) {
        match &body[*idx] {
            Node::Line(line) if line.trim() == format!("break '{label};") => {}
            _ => return None,
        }
    }

    Some(loop_node)
}

fn rewrite_break_to_unlabeled(nodes: &[Node], label: &str) -> Vec<Node> {
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == format!("break '{label};") {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(Node::Line(format!("{indent}break;")));
                } else {
                    out.push(Node::Line(line.clone()));
                }
            }
            Node::Block {
                kind,
                label: inner_label,
                header,
                body,
                footer,
            } => out.push(Node::Block {
                kind: kind.clone(),
                label: inner_label.clone(),
                header: header.clone(),
                body: rewrite_break_to_unlabeled(body, label),
                footer: footer.clone(),
            }),
        }
    }
    out
}

fn significant_indices(nodes: &[Node]) -> Vec<usize> {
    nodes.iter()
        .enumerate()
        .filter_map(|(idx, node)| match node {
            Node::Line(line) if line.trim().is_empty() => None,
            _ => Some(idx),
        })
        .collect()
}
