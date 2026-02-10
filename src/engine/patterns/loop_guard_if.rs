use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopGuardToIf;

impl LoopGuardToIf {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopGuardToIf {
    fn name(&self) -> &'static str {
        "loop_guard_to_if"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_sequence(nodes, &mut changed);
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

fn rewrite_sequence(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < nodes.len() {
        if let Some((new_nodes, consumed)) = try_rewrite_guard_loop(&nodes, i) {
            out.extend(new_nodes);
            i += consumed;
            *changed = true;
            continue;
        }
        let mut node = nodes[i].clone();
        if let Node::Block { body, .. } = &mut node {
            let new_body = rewrite_sequence(std::mem::take(body), changed);
            *body = new_body;
        }
        out.push(node);
        i += 1;
    }
    out
}

fn try_rewrite_guard_loop(nodes: &[Node], idx: usize) -> Option<(Vec<Node>, usize)> {
    let loop_node = match nodes.get(idx)? {
        Node::Block {
            kind: BlockKind::Loop,
            header,
            body,
            ..
        } => (header, body),
        _ => return None,
    };
    let (next_idx, _next_line) = next_non_empty_line(nodes, idx + 1)?;
    if !is_unreachable_line(nodes.get(next_idx)?) {
        return None;
    }
    let (guard_if, rest) = split_guard_if(loop_node.1)?;
    if contains_break_or_continue_at_depth0(&rest) {
        return None;
    }
    let indent = loop_node
        .0
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut guard_body = strip_trailing_break(&guard_if.body)?;
    if guard_body.is_empty() {
        guard_body.push(Node::Line(format!("{indent}    unreachable!();")));
    }
    let mut guard_if = Node::Block {
        kind: BlockKind::If,
        label: None,
        header: guard_if.header.clone(),
        body: guard_body,
        footer: format!("{indent}}}"),
    };
    let mut else_block = Node::Block {
        kind: BlockKind::Else,
        label: None,
        header: format!("{indent}else {{"),
        body: rest,
        footer: format!("{indent}}}"),
    };
    dedent_block(&mut guard_if, 4);
    dedent_block(&mut else_block, 4);
    Some((vec![guard_if, else_block], next_idx - idx + 1))
}

fn next_non_empty_line(nodes: &[Node], mut idx: usize) -> Option<(usize, String)> {
    while idx < nodes.len() {
        match &nodes[idx] {
            Node::Line(line) if line.trim().is_empty() => {
                idx += 1;
                continue;
            }
            Node::Line(line) => return Some((idx, line.clone())),
            _ => return None,
        }
    }
    None
}

fn is_unreachable_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim() == "unreachable!();")
}

struct GuardIf {
    header: String,
    body: Vec<Node>,
}

fn split_guard_if(body: &[Node]) -> Option<(GuardIf, Vec<Node>)> {
    let first = body.first()?;
    let guard = match first {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => GuardIf {
            header: header.clone(),
            body: body.clone(),
        },
        _ => return None,
    };
    let rest = body[1..].to_vec();
    Some((guard, rest))
}

fn strip_trailing_break(nodes: &[Node]) -> Option<Vec<Node>> {
    let mut out = nodes.to_vec();
    while let Some(last) = out.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                out.pop();
            }
            Node::Line(line) if line.trim() == "break;" => {
                out.pop();
                return Some(out);
            }
            _ => return None,
        }
    }
    None
}

fn contains_break_or_continue_at_depth0(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == "break;"
                    || t == "continue;"
                    || t.starts_with("break '")
                    || t.starts_with("continue '")
                {
                    return true;
                }
            }
            Node::Block { kind, body, .. } => {
                if *kind == BlockKind::Loop {
                    continue;
                }
                if contains_break_or_continue_at_depth0(body) {
                    return true;
                }
            }
        }
    }
    false
}

fn dedent_block(node: &mut Node, spaces: usize) {
    match node {
        Node::Block {
            header,
            body,
            footer,
            ..
        } => {
            *header = dedent_line(header, spaces);
            *footer = dedent_line(footer, spaces);
            for child in body.iter_mut() {
                dedent_node(child, spaces);
            }
        }
        Node::Line(_) => {}
    }
}

fn dedent_node(node: &mut Node, spaces: usize) {
    match node {
        Node::Line(line) => {
            *line = dedent_line(line, spaces);
        }
        Node::Block {
            header,
            body,
            footer,
            ..
        } => {
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
