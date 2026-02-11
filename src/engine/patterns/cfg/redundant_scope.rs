use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct RedundantScopePattern;

impl RedundantScopePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RedundantScopePattern {
    fn name(&self) -> &'static str {
        "redundant_scope"
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

        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&new_nodes),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite_sequence(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::new();

    for node in nodes {
        match node {
            Node::Line(line) => out.push(Node::Line(line)),
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let rewritten_body = rewrite_sequence(body, changed);
                if is_redundant_scope_block(&kind, label.as_deref(), &header, &footer, &rewritten_body) {
                    let mut lifted = rewritten_body;
                    let header_indent = indent_len(&header);
                    let min_indent = min_indent_in_nodes(&lifted).unwrap_or(header_indent);
                    let dedent = min_indent.saturating_sub(header_indent);
                    if dedent > 0 {
                        dedent_nodes(&mut lifted, dedent);
                    }
                    out.extend(lifted);
                    *changed = true;
                } else {
                    out.push(Node::Block {
                        kind,
                        label,
                        header,
                        body: rewritten_body,
                        footer,
                    });
                }
            }
        }
    }

    out
}

fn is_redundant_scope_block(
    kind: &BlockKind,
    label: Option<&str>,
    header: &str,
    footer: &str,
    body: &[Node],
) -> bool {
    if *kind != BlockKind::Other {
        return false;
    }
    if label.is_some() {
        return false;
    }
    if header.trim() != "{" || footer.trim() != "}" {
        return false;
    }
    if body.is_empty() {
        return true;
    }
    if has_top_level_scope_sensitive_decl(body) {
        return false;
    }
    true
}

fn has_top_level_scope_sensitive_decl(nodes: &[Node]) -> bool {
    for node in nodes {
        if let Node::Line(line) = node {
            let t = line.trim_start();
            if t.starts_with("let ") || t.starts_with("const ") || t.starts_with("type ") {
                return true;
            }
        }
    }
    false
}

fn dedent_nodes(nodes: &mut [Node], spaces: usize) {
    for node in nodes {
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
