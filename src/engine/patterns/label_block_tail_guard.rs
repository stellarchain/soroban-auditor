use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelBlockTailGuard;

impl LabelBlockTailGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelBlockTailGuard {
    fn name(&self) -> &'static str {
        "label_block_tail_guard"
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
                let new_body = rewrite(body, changed);
                if let Some(rewritten) = try_guard_tail(&label, &header, &new_body) {
                    out.extend(rewritten);
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

fn try_guard_tail(label: &str, header: &str, body: &[Node]) -> Option<Vec<Node>> {
    let (first_idx, first_node) = first_non_empty_node(body)?;
    let child_label = match first_node {
        Node::Block {
            kind: BlockKind::Other,
            label: Some(ref label),
            ..
        } => label.clone(),
        _ => return None,
    };

    let mut tail: Vec<Node> = Vec::new();
    for (idx, node) in body.iter().enumerate() {
        if idx <= first_idx {
            continue;
        }
        tail.push(node.clone());
    }
    if tail.is_empty() {
        return None;
    }
    let mut tail = strip_trailing_break_label(tail, label)?;
    if contains_break_label(&tail, label) || contains_continue_label(&tail, label) {
        return None;
    }
    if !contains_break_label(&[first_node.clone()], label) {
        return None;
    }

    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let flag = format!("__exit_{}", label);
    let mut out: Vec<Node> = Vec::new();
    out.push(Node::Line(format!("{indent}let mut {flag}: i32 = 0;")));

    let mut child = first_node.clone();
    rewrite_breaks_to_child(&mut child, label, &child_label, &flag);
    let child = dedent_node(child, &indent);
    out.push(child);

    let if_header = format!("{indent}if {flag} == 0 {{");
    let if_footer = format!("{indent}}}");
    let tail = dedent_nodes(tail, &indent);
    out.push(Node::Block {
        kind: BlockKind::If,
        label: None,
        header: if_header,
        body: tail,
        footer: if_footer,
    });
    Some(out)
}

fn first_non_empty_node(nodes: &[Node]) -> Option<(usize, Node)> {
    for (idx, node) in nodes.iter().enumerate() {
        if !is_empty_line(node) {
            return Some((idx, node.clone()));
        }
    }
    None
}

fn rewrite_breaks_to_child(node: &mut Node, label: &str, child_label: &str, flag: &str) {
    let break_target = format!("break '{};", label);
    match node {
        Node::Line(line) => {
            let trimmed = line.trim();
            if trimmed == break_target {
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                *line = format!("{indent}{flag} = 1; break '{};", child_label);
            }
        }
        Node::Block { body, .. } => {
            for child in body {
                rewrite_breaks_to_child(child, label, child_label, flag);
            }
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

fn strip_trailing_break_label(mut nodes: Vec<Node>, label: &str) -> Option<Vec<Node>> {
    let target = format!("break '{};", label);
    let mut saw_break = false;
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            Node::Line(line) if line.trim() == target => {
                nodes.pop();
                saw_break = true;
            }
            _ => break,
        }
    }
    if saw_break {
        Some(nodes)
    } else {
        Some(nodes)
    }
}

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}

fn dedent_nodes(nodes: Vec<Node>, indent: &str) -> Vec<Node> {
    nodes.into_iter().map(|n| dedent_node(n, indent)).collect()
}

fn dedent_node(mut node: Node, indent: &str) -> Node {
    let prefix = format!("{indent}    ");
    match &mut node {
        Node::Line(line) => {
            if line.starts_with(&prefix) {
                *line = format!("{}{}", indent, &line[prefix.len()..]);
            }
        }
        Node::Block { header, body, footer, .. } => {
            if header.starts_with(&prefix) {
                *header = format!("{}{}", indent, &header[prefix.len()..]);
            }
            if footer.starts_with(&prefix) {
                *footer = format!("{}{}", indent, &footer[prefix.len()..]);
            }
            let new_body = body.iter().cloned().map(|n| dedent_node(n, indent)).collect();
            *body = new_body;
        }
    }
    node
}
