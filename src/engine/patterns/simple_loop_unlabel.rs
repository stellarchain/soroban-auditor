use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct SimpleLoopUnlabel;

impl SimpleLoopUnlabel {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SimpleLoopUnlabel {
    fn name(&self) -> &'static str {
        "simple_loop_unlabel"
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
                label: Some(label),
                header,
                body,
                footer,
            } => {
                if header.contains("while {") {
                    let new_body = rewrite(body, changed);
                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: Some(label),
                        header,
                        body: new_body,
                        footer,
                    });
                    continue;
                }
                let mut new_body = rewrite(body, changed);
                if !has_nested_loop(&new_body) {
                    rewrite_label_controls(&mut new_body, &label, changed);
                    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    let new_header = format!("{indent}loop {{");
                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: None,
                        header: new_header,
                        body: new_body,
                        footer,
                    });
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
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

fn has_nested_loop(body: &[Node]) -> bool {
    for node in body {
        match node {
            Node::Block { kind, body, .. } => {
                if *kind == BlockKind::Loop || has_nested_loop(body) {
                    return true;
                }
            }
            Node::Line(_) => {}
        }
    }
    false
}

fn rewrite_label_controls(body: &mut [Node], label: &str, changed: &mut bool) {
    for node in body.iter_mut() {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                let break_label = format!("break '{};", label);
                let continue_label = format!("continue '{};", label);
                if t == break_label {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    *line = format!("{indent}break;");
                    *changed = true;
                } else if t == continue_label {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    *line = format!("{indent}continue;");
                    *changed = true;
                }
            }
            Node::Block { body, .. } => {
                rewrite_label_controls(body, label, changed);
            }
        }
    }
}
