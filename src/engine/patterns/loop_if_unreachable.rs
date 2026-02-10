use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

pub struct LoopIfUnreachableToBlock;

impl LoopIfUnreachableToBlock {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopIfUnreachableToBlock {
    fn name(&self) -> &'static str {
        "loop_if_unreachable_to_block"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut defined = HashSet::new();
        collect_defined_labels(&nodes, &mut defined);
        let mut changed = false;
        let new_nodes = rewrite(nodes, &mut defined, &mut changed);
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

fn rewrite(nodes: Vec<Node>, defined: &mut HashSet<String>, changed: &mut bool) -> Vec<Node> {
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
                let new_body = rewrite(body, defined, changed);
                if let Some(collapsed) = try_collapse_loop(&header, &new_body, defined) {
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
                let new_body = rewrite(body, defined, changed);
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

fn try_collapse_loop(header: &str, body: &[Node], defined: &mut HashSet<String>) -> Option<Node> {
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
            ..
        } => nodes[0].clone(),
        _ => return None,
    };
    match &nodes[1] {
        Node::Line(line) if line.trim() == "unreachable!();" => {}
        _ => return None,
    }
    if contains_continue_at_depth0(&if_node, 0) {
        return None;
    }
    let label = fresh_label(defined);
    defined.insert(label.clone());

    let mut if_node = if_node;
    rewrite_breaks_to_label(&mut if_node, &label, 0);

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
    let block_header = format!("{indent}'{label}: {{");
    Some(Node::Block {
        kind: BlockKind::Other,
        label: Some(label),
        header: block_header,
        body: vec![if_node, else_block],
        footer: format!("{indent}}}"),
    })
}

fn fresh_label(defined: &HashSet<String>) -> String {
    for idx in 0..1000 {
        let name = format!("__if_guard{idx}");
        if !defined.contains(&name) {
            return name;
        }
    }
    "__if_guard".to_string()
}

fn collect_defined_labels(nodes: &[Node], defined: &mut HashSet<String>) {
    for node in nodes {
        match node {
            Node::Block {
                label: Some(label),
                body,
                ..
            } => {
                defined.insert(label.clone());
                collect_defined_labels(body, defined);
            }
            Node::Block {
                label: None, body, ..
            } => collect_defined_labels(body, defined),
            Node::Line(_) => {}
        }
    }
}

fn rewrite_breaks_to_label(node: &mut Node, label: &str, loop_depth: usize) {
    match node {
        Node::Line(line) => {
            if loop_depth == 0 && line.contains("break;") && !line.contains("break '") {
                let replaced = line.replace("break;", &format!("break '{label};"));
                *line = replaced;
            }
        }
        Node::Block { kind, body, .. } => {
            let next_depth = if *kind == BlockKind::Loop {
                loop_depth + 1
            } else {
                loop_depth
            };
            for child in body {
                rewrite_breaks_to_label(child, label, next_depth);
            }
        }
    }
}

fn contains_continue_at_depth0(node: &Node, loop_depth: usize) -> bool {
    match node {
        Node::Line(line) => loop_depth == 0 && line.trim() == "continue;",
        Node::Block { kind, body, .. } => {
            let next_depth = if *kind == BlockKind::Loop {
                loop_depth + 1
            } else {
                loop_depth
            };
            body.iter()
                .any(|n| contains_continue_at_depth0(n, next_depth))
        }
    }
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
