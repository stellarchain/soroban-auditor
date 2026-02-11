use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelLoopIfContinueToWhilePattern;

impl LabelLoopIfContinueToWhilePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLoopIfContinueToWhilePattern {
    fn name(&self) -> &'static str {
        "label_loop_if_continue_to_while"
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
                label: Some(label),
                header,
                body,
                footer,
            } => {
                let rewritten_body = rewrite(body, changed);
                if let Some(mut lowered) = lower_label_loop_if_continue(&label, &header, &rewritten_body) {
                    out.append(&mut lowered);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
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

fn lower_label_loop_if_continue(
    label: &str,
    loop_header: &str,
    body: &[Node],
) -> Option<Vec<Node>> {
    let sig = significant_indices(body);
    if sig.len() < 2 {
        return None;
    }

    let if_idx = sig[0];
    let (if_header, then_body) = match &body[if_idx] {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => (header.as_str(), body.clone()),
        _ => return None,
    };

    if contains_label_control(&then_body, label) {
        return None;
    }

    let mut tail = body[if_idx + 1..].to_vec();
    trim_trailing_empty(&mut tail);
    if tail.is_empty() {
        return None;
    }

    let tail_last = tail.last()?;
    let last_line = match tail_last {
        Node::Line(line) => line.trim().to_string(),
        _ => return None,
    };
    if last_line != format!("continue '{label};") {
        return None;
    }
    tail.pop();

    if contains_label_control(&tail, label) {
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
        body: tail,
        footer: format!("{indent}}}"),
    });
    out.extend(then_body);
    Some(out)
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

fn trim_trailing_empty(nodes: &mut Vec<Node>) {
    while matches!(nodes.last(), Some(Node::Line(line)) if line.trim().is_empty()) {
        nodes.pop();
    }
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
