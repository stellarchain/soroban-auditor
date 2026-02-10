use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct GuardUnreachableAssertPattern;

impl GuardUnreachableAssertPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for GuardUnreachableAssertPattern {
    fn name(&self) -> &'static str {
        "guard_unreachable_assert"
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
            Node::Line(line) => out.push(Node::Line(line)),
            Node::Block {
                kind: BlockKind::If,
                label,
                header,
                body,
                footer,
            } => {
                if label.is_none() && is_unreachable_guard_body(&body) {
                    if let Some((indent, cond)) = parse_if_header(&header) {
                        out.push(Node::Line(format!("{indent}assert!({});", negate_condition(&cond))));
                        *changed = true;
                        continue;
                    }
                }
                let new_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind: BlockKind::If,
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
        }
    }
    out
}

fn is_unreachable_guard_body(body: &[Node]) -> bool {
    let non_empty: Vec<&Node> = body
        .iter()
        .filter(|n| !matches!(n, Node::Line(line) if line.trim().is_empty()))
        .collect();
    if non_empty.len() != 1 {
        return false;
    }
    matches!(non_empty[0], Node::Line(line) if line.trim() == "unreachable!();")
}

fn parse_if_header(header: &str) -> Option<(String, String)> {
    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let trimmed = header.trim();
    let cond = trimmed.strip_prefix("if ")?.strip_suffix(" {")?.trim();
    if cond.is_empty() {
        None
    } else {
        Some((indent, cond.to_string()))
    }
}

fn negate_condition(cond: &str) -> String {
    let c = cond.trim();
    if let Some(inner) = c.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
        return inner.trim().to_string();
    }
    format!("!({c})")
}
