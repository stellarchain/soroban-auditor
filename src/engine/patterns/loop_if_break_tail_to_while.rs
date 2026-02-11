use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopIfBreakTailToWhilePattern;

impl LoopIfBreakTailToWhilePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopIfBreakTailToWhilePattern {
    fn name(&self) -> &'static str {
        "loop_if_break_tail_to_while"
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
                let rewritten_body = rewrite(body, changed);
                if let Some(mut lowered) =
                    lower_loop_if_break_tail(label.as_deref(), &header, &rewritten_body)
                {
                    out.append(&mut lowered);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label,
                    header,
                    body: rewritten_body,
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
                let rewritten_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: rewritten_body,
                    footer,
                });
            }
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn lower_loop_if_break_tail(
    loop_label: Option<&str>,
    loop_header: &str,
    body: &[Node],
) -> Option<Vec<Node>> {
    let sig = significant_indices(body);
    if sig.len() < 2 {
        return None;
    }
    let if_idx = sig[0];
    let (if_header, if_body) = match &body[if_idx] {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => (header, body.clone()),
        _ => return None,
    };

    let (then_body, is_matching_break) = strip_terminal_break(&if_body, loop_label);
    if !is_matching_break {
        return None;
    }
    if contains_control_to_loop(&then_body, loop_label) {
        return None;
    }

    let mut tail_body = if_idx + 1;
    while tail_body < body.len() && is_empty_line(&body[tail_body]) {
        tail_body += 1;
    }
    if tail_body >= body.len() {
        return None;
    }
    let while_body = body[tail_body..].to_vec();
    if contains_control_to_loop(&while_body, loop_label) {
        return None;
    }

    let cond = parse_if_condition(if_header)?;
    let indent = loop_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut out = Vec::new();
    out.push(Node::Block {
        kind: BlockKind::Loop,
        label: None,
        header: format!("{indent}while {} {{", invert_condition(&cond)),
        body: while_body,
        footer: format!("{indent}}}"),
    });
    out.extend(then_body);
    Some(out)
}

fn strip_terminal_break(nodes: &[Node], loop_label: Option<&str>) -> (Vec<Node>, bool) {
    let mut out = nodes.to_vec();
    while let Some(last) = out.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                out.pop();
            }
            Node::Line(line) => {
                let trimmed = line.trim();
                if is_matching_break(trimmed, loop_label) {
                    out.pop();
                    return (out, true);
                }
                return (nodes.to_vec(), false);
            }
            _ => return (nodes.to_vec(), false),
        }
    }
    (nodes.to_vec(), false)
}

fn is_matching_break(line: &str, loop_label: Option<&str>) -> bool {
    if let Some(label) = loop_label {
        return line == format!("break '{label};");
    }
    line == "break;"
}

fn contains_control_to_loop(nodes: &[Node], loop_label: Option<&str>) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if let Some(label) = loop_label {
                    if t == format!("break '{label};") || t == format!("continue '{label};") {
                        return true;
                    }
                } else if t == "break;" || t == "continue;" {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_control_to_loop(body, loop_label) {
                    return true;
                }
            }
        }
    }
    false
}

fn significant_indices(nodes: &[Node]) -> Vec<usize> {
    nodes.iter()
        .enumerate()
        .filter_map(|(idx, node)| if is_empty_line(node) { None } else { Some(idx) })
        .collect()
}

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}

fn parse_if_condition(header: &str) -> Option<String> {
    let t = header.trim();
    let cond = t.strip_prefix("if ")?.strip_suffix(" {")?;
    Some(cond.trim().to_string())
}

fn invert_condition(cond: &str) -> String {
    if let Some((lhs, rhs)) = cond.split_once(" == ") {
        return format!("{} != {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = cond.split_once(" != ") {
        return format!("{} == {}", lhs.trim(), rhs.trim());
    }
    format!("!({})", cond.trim())
}
