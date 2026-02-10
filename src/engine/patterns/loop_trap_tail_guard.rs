use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopTrapTailGuardPattern;

impl LoopTrapTailGuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopTrapTailGuardPattern {
    fn name(&self) -> &'static str {
        "loop_trap_tail_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_nodes(nodes, &mut changed);
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

fn rewrite_nodes(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut rewritten: Vec<Node> = Vec::with_capacity(nodes.len());
    for node in nodes {
        rewritten.push(rewrite_node(node, changed));
    }

    let mut out: Vec<Node> = Vec::with_capacity(rewritten.len());
    let mut i = 0usize;
    while i < rewritten.len() {
        if i + 1 >= rewritten.len() {
            out.push(rewritten[i].clone());
            i += 1;
            continue;
        }

        let (loop_node, next_node) = (&rewritten[i], &rewritten[i + 1]);
        let Node::Block {
            kind: BlockKind::Loop,
            label,
            header,
            body,
            ..
        } = loop_node
        else {
            out.push(loop_node.clone());
            i += 1;
            continue;
        };
        if label.is_some() {
            out.push(loop_node.clone());
            i += 1;
            continue;
        }
        let Node::Line(next_line) = next_node else {
            out.push(loop_node.clone());
            i += 1;
            continue;
        };
        if next_line.trim() != "unreachable!();" {
            out.push(loop_node.clone());
            i += 1;
            continue;
        }
        if contains_continue_at_depth0(body, 0) || !contains_unlabeled_break_at_depth0(body, 0) {
            out.push(loop_node.clone());
            i += 1;
            continue;
        }

        let mut new_body = body.clone();
        rewrite_unlabeled_break_to_unreachable(&mut new_body, 0);
        let indent = header
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        out.push(Node::Block {
            kind: BlockKind::Other,
            label: None,
            header: format!("{indent}{{"),
            body: new_body,
            footer: format!("{indent}}}"),
        });
        *changed = true;
        i += 2; // consume trailing `unreachable!();`
    }

    out
}

fn rewrite_node(node: Node, changed: &mut bool) -> Node {
    match node {
        Node::Line(line) => Node::Line(line),
        Node::Block {
            kind,
            label,
            header,
            body,
            footer,
        } => Node::Block {
            kind,
            label,
            header,
            body: rewrite_nodes(body, changed),
            footer,
        },
    }
}

fn contains_continue_at_depth0(nodes: &[Node], loop_depth: usize) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if loop_depth == 0 && (t == "continue;" || t.starts_with("continue '")) {
                    return true;
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                if contains_continue_at_depth0(body, next_depth) {
                    return true;
                }
            }
        }
    }
    false
}

fn contains_unlabeled_break_at_depth0(nodes: &[Node], loop_depth: usize) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                if loop_depth == 0 && line.trim() == "break;" {
                    return true;
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                if contains_unlabeled_break_at_depth0(body, next_depth) {
                    return true;
                }
            }
        }
    }
    false
}

fn rewrite_unlabeled_break_to_unreachable(nodes: &mut [Node], loop_depth: usize) {
    for node in nodes.iter_mut() {
        match node {
            Node::Line(line) => {
                if loop_depth == 0 && line.trim() == "break;" {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *line = format!("{indent}unreachable!();");
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                rewrite_unlabeled_break_to_unreachable(body, next_depth);
            }
        }
    }
}
