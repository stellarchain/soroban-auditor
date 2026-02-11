use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelBlockToLoop;

impl LabelBlockToLoop {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelBlockToLoop {
    fn name(&self) -> &'static str {
        "label_block_to_loop"
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
                if label.starts_with("__sp") {
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header,
                        body: rewrite(body, changed),
                        footer,
                    });
                    continue;
                }
                let mut new_body = rewrite(body, changed);
                if !contains_continue_label(&new_body, &label)
                    && contains_break_label(&new_body, &label)
                {
                    rewrite_breaks_to_unlabeled(&mut new_body, &label);
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: None,
                        header: format!("{indent}loop {{"),
                        body: new_body,
                        footer: format!("{indent}}}"),
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

fn rewrite_breaks_to_unlabeled(nodes: &mut [Node], label: &str) {
    let target = format!("break '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *line = format!("{indent}break;");
                }
            }
            Node::Block { body, .. } => rewrite_breaks_to_unlabeled(body, label),
        }
    }
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
