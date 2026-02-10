use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct StatusResultGuardLoopPattern;

impl StatusResultGuardLoopPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StatusResultGuardLoopPattern {
    fn name(&self) -> &'static str {
        "status_result_guard_loop"
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
        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&new_nodes),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::with_capacity(nodes.len());
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
                if let Some(replacement) = collapse_status_loop(&header, &new_body) {
                    out.push(replacement);
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
            } => out.push(Node::Block {
                kind,
                label,
                header,
                body: rewrite(body, changed),
                footer,
            }),
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn collapse_status_loop(loop_header: &str, body: &[Node]) -> Option<Node> {
    let mut effective: Vec<String> = Vec::new();
    for n in body {
        match n {
            Node::Line(line) => {
                let t = line.trim();
                if !t.is_empty() {
                    effective.push(line.clone());
                }
            }
            Node::Block { .. } => return None,
        }
    }
    if effective.len() != 2 {
        return None;
    }
    let let_line = effective[0].trim().to_string();
    if effective[1].trim() != "unreachable!();" {
        return None;
    }
    if !is_status_let_line(&let_line) {
        return None;
    }
    let var_name = parse_let_name(&let_line)?;
    let indent = loop_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let inner_indent = format!("{indent}    ");
    Some(Node::Block {
        kind: BlockKind::Other,
        label: None,
        header: format!("{indent}{{"),
        body: vec![
            Node::Line(format!("{inner_indent}{let_line}")),
            Node::Block {
                kind: BlockKind::If,
                label: None,
                header: format!("{inner_indent}if {var_name} != 0 {{"),
                body: vec![Node::Line(format!("{inner_indent}    unreachable!();"))],
                footer: format!("{inner_indent}}}"),
            },
        ],
        footer: format!("{indent}}}"),
    })
}

fn is_status_let_line(line: &str) -> bool {
    line.starts_with("let ")
        && line.contains(" = ")
        && line.ends_with(';')
        && line.contains("mload32!(")
        && line.contains(" as i32;")
}

fn parse_let_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("let ")?;
    let lhs = rest.split('=').next()?.trim();
    let lhs = lhs.strip_prefix("mut ").unwrap_or(lhs);
    let name = lhs.split(':').next()?.trim();
    if is_ident(name) {
        Some(name.to_string())
    } else {
        None
    }
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
