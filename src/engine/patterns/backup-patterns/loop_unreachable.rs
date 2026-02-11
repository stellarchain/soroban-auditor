use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopUnreachableElse;

impl LoopUnreachableElse {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopUnreachableElse {
    fn name(&self) -> &'static str {
        "loop_unreachable_else"
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
                if let Some(collapsed) = try_collapse_loop(&header, &new_body) {
                    out.push(collapsed);
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

fn try_collapse_loop(header: &str, body: &[Node]) -> Option<Node> {
    if !header.trim_start().starts_with("loop ") {
        return None;
    }
    let mut nodes = body.to_vec();
    trim_trailing_empty(&mut nodes);
    if nodes.len() != 2 {
        return None;
    }
    let if_node = match &nodes[0] {
        Node::Block {
            kind: BlockKind::If,
            body,
            ..
        } => {
            if contains_control_in_nodes(body) {
                return None;
            }
            nodes[0].clone()
        }
        _ => return None,
    };
    match &nodes[1] {
        Node::Line(line) if line.trim() == "unreachable!();" => {}
        _ => return None,
    }
    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let else_block = Node::Block {
        kind: BlockKind::Else,
        label: None,
        header: format!("{indent}else {{"),
        body: vec![Node::Line(format!("{indent}    unreachable!();"))],
        footer: format!("{indent}}}"),
    };
    Some(Node::Block {
        kind: BlockKind::Other,
        label: None,
        header: format!("{indent}{{"),
        body: vec![if_node, else_block],
        footer: format!("{indent}}}"),
    })
}

fn trim_trailing_empty(nodes: &mut Vec<Node>) {
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            _ => break,
        }
    }
}

fn contains_control_in_nodes(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == "break;"
                    || t.starts_with("break '")
                    || t == "continue;"
                    || t.starts_with("continue '")
                {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_control_in_nodes(body) {
                    return true;
                }
            }
        }
    }
    false
}
