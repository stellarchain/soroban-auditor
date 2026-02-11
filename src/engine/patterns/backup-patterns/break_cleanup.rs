use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

pub struct BreakCleanup;

impl BreakCleanup {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for BreakCleanup {
    fn name(&self) -> &'static str {
        "break_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_breaks(nodes, &mut changed);
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

fn rewrite_breaks(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    let mut i = 0;
    while i < nodes.len() {
        match &nodes[i] {
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite_breaks(body.clone(), changed);
                out.push(Node::Block {
                    kind: kind.clone(),
                    label: label.clone(),
                    header: header.clone(),
                    body: new_body,
                    footer: footer.clone(),
                });
                i += 1;
            }
            Node::Line(line) => {
                if i + 1 < nodes.len() {
                    if let Node::Line(next) = &nodes[i + 1] {
                        if line.trim().starts_with("break '") && next.trim() == "break;" {
                            out.push(Node::Line(line.clone()));
                            i += 2;
                            *changed = true;
                            continue;
                        }
                    }
                }
                out.push(Node::Line(line.clone()));
                i += 1;
            }
        }
    }
    out
}
