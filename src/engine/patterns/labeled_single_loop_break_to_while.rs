use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabeledSingleLoopBreakToWhilePattern;

impl LabeledSingleLoopBreakToWhilePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabeledSingleLoopBreakToWhilePattern {
    fn name(&self) -> &'static str {
        "labeled_single_loop_break_to_while"
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
                kind: BlockKind::Other,
                label: Some(label),
                header,
                body,
                footer,
            } => {
                let rewritten_body = rewrite(body, changed);
                if let Some(mut lowered) = lower_labeled_single_loop(&label, &rewritten_body) {
                    out.append(&mut lowered);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Other,
                    label: Some(label),
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

fn lower_labeled_single_loop(label: &str, body: &[Node]) -> Option<Vec<Node>> {
    let sig = significant_indices(body);
    if sig.len() != 1 {
        return None;
    }

    let loop_node = match &body[sig[0]] {
        Node::Block {
            kind: BlockKind::Loop,
            label: None,
            header,
            body,
            ..
        } => (header.clone(), body.clone()),
        _ => return None,
    };

    let lowered = lower_loop_if_break_tail_to_while(&loop_node.0, &loop_node.1, label)?;
    Some(lowered)
}

fn lower_loop_if_break_tail_to_while(
    loop_header: &str,
    body: &[Node],
    break_label: &str,
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

    let (then_body, is_matching_break) = strip_terminal_break_to_label(&if_body, break_label);
    if !is_matching_break {
        return None;
    }
    if contains_label_control(&then_body, break_label) {
        return None;
    }

    let mut tail_start = if_idx + 1;
    while tail_start < body.len() && is_empty_line(&body[tail_start]) {
        tail_start += 1;
    }
    if tail_start >= body.len() {
        return None;
    }

    let while_body = body[tail_start..].to_vec();
    if contains_label_control(&while_body, break_label) {
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

fn strip_terminal_break_to_label(nodes: &[Node], label: &str) -> (Vec<Node>, bool) {
    let mut out = nodes.to_vec();
    while let Some(last) = out.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                out.pop();
            }
            Node::Line(line) => {
                let trimmed = line.trim();
                if trimmed == format!("break '{label};") {
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

fn contains_label_control(nodes: &[Node], label: &str) -> bool {
    let break_target = format!("break '{label};");
    let continue_target = format!("continue '{label};");
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == break_target || t == continue_target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_label_control(body, label) {
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
