use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct PruneEmptyIfBlocksPattern;

impl PruneEmptyIfBlocksPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for PruneEmptyIfBlocksPattern {
    fn name(&self) -> &'static str {
        "prune_empty_if_blocks"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let rewritten = rewrite_nodes(nodes, &mut changed);
        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&rewritten),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite_nodes(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::with_capacity(nodes.len());

    for node in nodes {
        match node {
            Node::Line(_) => out.push(node),
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite_nodes(body, changed);
                let body_has_code = new_body.iter().any(|n| match n {
                    Node::Line(l) => !l.trim().is_empty(),
                    Node::Block { .. } => true,
                });

                // Remove syntactic if/else shells with empty bodies.
                if (kind == BlockKind::If || kind == BlockKind::Else) && !body_has_code {
                    *changed = true;
                    continue;
                }

                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
        }
    }

    out
}
