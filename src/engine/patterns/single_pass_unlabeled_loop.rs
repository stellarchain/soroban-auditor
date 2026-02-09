use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct SinglePassUnlabeledLoopCleanup;

impl SinglePassUnlabeledLoopCleanup {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SinglePassUnlabeledLoopCleanup {
    fn name(&self) -> &'static str {
        "single_pass_unlabeled_loop_cleanup"
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
                label: None,
                header,
                body,
                footer,
            } => {
                let mut new_body = rewrite(body, changed);
                if !contains_continue(&new_body) && ends_with_break(&new_body) {
                    trim_trailing_break(&mut new_body);
                    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: None,
                        header: format!("{indent}{{"),
                        body: new_body,
                        footer: format!("{indent}}}"),
                    });
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label: None,
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

fn contains_continue(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == "continue;" || line.trim().starts_with("continue '") {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_continue(body) {
                    return true;
                }
            }
        }
    }
    false
}

fn ends_with_break(nodes: &[Node]) -> bool {
    for node in nodes.iter().rev() {
        match node {
            Node::Line(line) if line.trim().is_empty() => continue,
            Node::Line(line) => return line.trim() == "break;",
            _ => return false,
        }
    }
    false
}

fn trim_trailing_break(nodes: &mut Vec<Node>) {
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            Node::Line(line) if line.trim() == "break;" => {
                nodes.pop();
                break;
            }
            _ => break,
        }
    }
}
