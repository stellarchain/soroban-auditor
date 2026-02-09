use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct UnwrapIfElseBlock;

impl UnwrapIfElseBlock {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for UnwrapIfElseBlock {
    fn name(&self) -> &'static str {
        "unwrap_if_else_block"
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
            Node::Block { kind: BlockKind::Other, label: None, header, body, footer } => {
                if header.trim() == "{" && footer.trim() == "}" && can_unwrap(&body) {
                    let mut dedented = body;
                    for child in dedented.iter_mut() {
                        dedent_node(child, 4);
                    }
                    out.extend(dedented);
                    *changed = true;
                    continue;
                }
                let new_body = rewrite(body, changed);
                out.push(Node::Block { kind: BlockKind::Other, label: None, header, body: new_body, footer });
            }
            Node::Block { kind, label, header, body, footer } => {
                let new_body = rewrite(body, changed);
                out.push(Node::Block { kind, label, header, body: new_body, footer });
            }
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn can_unwrap(body: &[Node]) -> bool {
    let nodes: Vec<&Node> = body.iter().filter(|n| !matches!(n, Node::Line(line) if line.trim().is_empty())).collect();
    if nodes.len() < 2 {
        return false;
    }
    let mut idx = 0usize;
    while idx < nodes.len() {
        match nodes[idx] {
            Node::Line(_) => idx += 1,
            _ => break,
        }
    }
    if idx + 1 >= nodes.len() {
        return false;
    }
    matches!(nodes[idx], Node::Block { kind: BlockKind::If, .. }) &&
        matches!(nodes[idx + 1], Node::Block { kind: BlockKind::Else, .. })
}

fn dedent_node(node: &mut Node, spaces: usize) {
    match node {
        Node::Line(line) => {
            *line = dedent_line(line, spaces);
        }
        Node::Block { header, body, footer, .. } => {
            *header = dedent_line(header, spaces);
            *footer = dedent_line(footer, spaces);
            for child in body.iter_mut() {
                dedent_node(child, spaces);
            }
        }
    }
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut count = 0;
    for ch in line.chars() {
        if ch == ' ' && count < spaces {
            count += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return line.to_string();
    }
    line[count..].to_string()
}
