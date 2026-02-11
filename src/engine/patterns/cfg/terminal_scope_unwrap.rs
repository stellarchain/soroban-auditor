use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct TerminalScopeUnwrapPattern;

impl TerminalScopeUnwrapPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for TerminalScopeUnwrapPattern {
    fn name(&self) -> &'static str {
        "terminal_scope_unwrap"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let is_unit_return = !block.header.contains("->");
        let mut nodes = parse_lines(&block.body);
        let last_idx = last_non_empty_index(&nodes)?;
        let mut changed = false;

        // Common decompiler artifact for unit-return functions:
        // { ... self.global0 = frame + N; } unreachable!();
        // Lift the block and drop the synthetic trap tail.
        if is_unit_return && is_unreachable_node(nodes.get(last_idx)) {
            if let Some(prev_idx) = prev_non_empty_index(&nodes, last_idx) {
                if let Some(Node::Block {
                    kind,
                    label,
                    header,
                    body,
                    footer,
                }) = nodes.get(prev_idx)
                {
                    if *kind == BlockKind::Other
                        && label.is_none()
                        && header.trim() == "{"
                        && footer.trim() == "}"
                        && body_contains_stack_restore(body)
                    {
                        let mut lifted = body.clone();
                        let header_indent = indent_len(header);
                        let min_indent = min_indent_in_nodes(&lifted).unwrap_or(header_indent);
                        let dedent = min_indent.saturating_sub(header_indent);
                        if dedent > 0 {
                            dedent_nodes(&mut lifted, dedent);
                        }
                        nodes.splice(prev_idx..=last_idx, lifted);
                        changed = true;
                    }
                }
            }
        }

        let last_idx = last_non_empty_index(&nodes)?;
        if let Node::Block {
            kind,
            label,
            header,
            body,
            footer,
        } = &nodes[last_idx]
        {
            if *kind == BlockKind::Other
                && label.is_none()
                && header.trim() == "{"
                && footer.trim() == "}"
                && body_terminates(body)
            {
                let mut lifted = body.clone();
                let header_indent = indent_len(header);
                let min_indent = min_indent_in_nodes(&lifted).unwrap_or(header_indent);
                let dedent = min_indent.saturating_sub(header_indent);
                if dedent > 0 {
                    dedent_nodes(&mut lifted, dedent);
                }
                nodes.splice(last_idx..=last_idx, lifted);
                changed = true;
            }
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&nodes),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn last_non_empty_index(nodes: &[Node]) -> Option<usize> {
    nodes.iter().enumerate().rev().find_map(|(i, n)| match n {
        Node::Line(line) if line.trim().is_empty() => None,
        _ => Some(i),
    })
}

fn prev_non_empty_index(nodes: &[Node], before: usize) -> Option<usize> {
    if before == 0 {
        return None;
    }
    nodes[..before]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, n)| match n {
            Node::Line(line) if line.trim().is_empty() => None,
            _ => Some(i),
        })
}

fn is_unreachable_node(node: Option<&Node>) -> bool {
    matches!(node, Some(Node::Line(line)) if line.trim() == "unreachable!();")
}

fn body_contains_stack_restore(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t.contains("self.global0 =") && t.contains("wrapping_add(") {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if body_contains_stack_restore(body) {
                    return true;
                }
            }
        }
    }
    false
}

fn body_terminates(nodes: &[Node]) -> bool {
    let last = nodes.iter().rev().find(|n| match n {
        Node::Line(line) => !line.trim().is_empty(),
        Node::Block { .. } => true,
    });
    match last {
        Some(Node::Line(line)) => {
            let t = line.trim();
            t == "return;" || t == "unreachable!();" || t.starts_with("return ")
        }
        Some(Node::Block { body, .. }) => body_terminates(body),
        None => false,
    }
}

fn dedent_nodes(nodes: &mut [Node], spaces: usize) {
    for node in nodes {
        match node {
            Node::Line(line) => *line = dedent_line(line, spaces),
            Node::Block {
                header,
                body,
                footer,
                ..
            } => {
                *header = dedent_line(header, spaces);
                dedent_nodes(body, spaces);
                *footer = dedent_line(footer, spaces);
            }
        }
    }
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut count = 0usize;
    for ch in line.chars() {
        if ch == ' ' && count < spaces {
            count += 1;
        } else {
            break;
        }
    }
    line[count..].to_string()
}

fn indent_len(s: &str) -> usize {
    s.chars().take_while(|c| *c == ' ').count()
}

fn min_indent_in_nodes(nodes: &[Node]) -> Option<usize> {
    let mut min_indent: Option<usize> = None;
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                let n = indent_len(line);
                min_indent = Some(min_indent.map_or(n, |m| m.min(n)));
            }
            Node::Block {
                header,
                body,
                footer,
                ..
            } => {
                if !header.trim().is_empty() {
                    let n = indent_len(header);
                    min_indent = Some(min_indent.map_or(n, |m| m.min(n)));
                }
                if let Some(nested) = min_indent_in_nodes(body) {
                    min_indent = Some(min_indent.map_or(nested, |m| m.min(nested)));
                }
                if !footer.trim().is_empty() {
                    let n = indent_len(footer);
                    min_indent = Some(min_indent.map_or(n, |m| m.min(n)));
                }
            }
        }
    }
    min_indent
}
