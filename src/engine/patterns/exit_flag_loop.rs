use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct ExitFlagLoopCollapse;

impl ExitFlagLoopCollapse {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExitFlagLoopCollapse {
    fn name(&self) -> &'static str {
        "exit_flag_loop_collapse"
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
                if let Some(collapsed) = try_collapse_exit_flag_loop(&header, &new_body) {
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

fn try_collapse_exit_flag_loop(header: &str, body: &[Node]) -> Option<Node> {
    if !header.trim_start().starts_with("loop ") {
        return None;
    }
    if body.len() < 3 {
        return None;
    }
    let exit_var = match &body[0] {
        Node::Line(line) => parse_exit_flag_var(line)?,
        _ => return None,
    };
    let inner_loop = match &body[1] {
        Node::Block {
            kind: BlockKind::Loop,
            ..
        } => body[1].clone(),
        _ => return None,
    };
    let (if_header, if_body, _if_footer) = match &body[2] {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            footer,
            ..
        } => (header, body, footer),
        _ => return None,
    };
    if if_header.contains(" else ") || if_header.trim_start().starts_with("else ") {
        return None;
    }
    if !if_header_contains_exit_zero(if_header, &exit_var) {
        return None;
    }
    let mut if_body = if_body.clone();
    if !strip_trailing_break(&mut if_body) {
        return None;
    }
    if contains_control_in_nodes(&if_body) {
        return None;
    }
    let tail = body[3..].to_vec();
    if tail.is_empty() || contains_control_in_nodes(&tail) {
        return None;
    }

    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let if_block = Node::Block {
        kind: BlockKind::If,
        label: None,
        header: if_header.clone(),
        body: if_body,
        footer: format!("{indent}}}"),
    };
    let else_block = Node::Block {
        kind: BlockKind::Else,
        label: None,
        header: format!("{indent}else {{"),
        body: tail,
        footer: format!("{indent}}}"),
    };
    let mut new_body = Vec::new();
    new_body.push(body[0].clone());
    new_body.push(inner_loop);
    new_body.push(if_block);
    new_body.push(else_block);

    Some(Node::Block {
        kind: BlockKind::Other,
        label: None,
        header: format!("{indent}{{"),
        body: new_body,
        footer: format!("{indent}}}"),
    })
}

fn parse_exit_flag_var(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("let mut __exit_") {
        return None;
    }
    let parts: Vec<&str> = trimmed.split(':').collect();
    if parts.len() < 2 {
        return None;
    }
    let name = parts[0].trim().trim_start_matches("let mut ").to_string();
    if !trimmed.ends_with("= 0;") {
        return None;
    }
    Some(name)
}

fn if_header_contains_exit_zero(header: &str, exit_var: &str) -> bool {
    let t = header.trim();
    t.starts_with("if ")
        && t.contains(exit_var)
        && t.contains("== 0")
        && t.ends_with('{')
}

fn contains_control_in_nodes(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == "break;" || t.starts_with("break '") || t == "continue;" || t.starts_with("continue '") {
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

fn strip_trailing_break(nodes: &mut Vec<Node>) -> bool {
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            Node::Line(line) if line.trim() == "break;" => {
                nodes.pop();
                return true;
            }
            _ => return false,
        }
    }
    false
}
