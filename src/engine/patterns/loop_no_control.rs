use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopNoControlToBlock;

impl LoopNoControlToBlock {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopNoControlToBlock {
    fn name(&self) -> &'static str {
        "loop_no_control_to_block"
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
                if !has_control_at_depth0(&new_body, 0) {
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label,
                        header: format!("{indent}{{"),
                        body: new_body,
                        footer: format!("{indent}}}"),
                    });
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

fn has_control_at_depth0(nodes: &[Node], loop_depth: usize) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if loop_depth == 0
                    && (t == "break;"
                        || t == "continue;"
                        || t.starts_with("break '")
                        || t.starts_with("continue '"))
                {
                    return true;
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                if has_control_at_depth0(body, next_depth) {
                    return true;
                }
            }
        }
    }
    false
}
