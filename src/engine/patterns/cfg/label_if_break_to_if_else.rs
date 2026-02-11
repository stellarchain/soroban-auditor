use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelIfBreakToIfElsePattern;

impl LabelIfBreakToIfElsePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelIfBreakToIfElsePattern {
    fn name(&self) -> &'static str {
        "label_if_break_to_if_else"
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
                if let Some(mut lowered) = lower_label_block(&label, &rewritten_body, &header) {
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

fn lower_label_block(label: &str, body: &[Node], header: &str) -> Option<Vec<Node>> {
    let sig = significant_indices(body);
    if sig.len() < 2 {
        return None;
    }
    let if_idx = sig[0];
    let (if_header, if_body, if_footer) = match &body[if_idx] {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            footer,
            ..
        } => (header.clone(), body.clone(), footer.clone()),
        _ => return None,
    };

    let (then_body, is_matching_break) = strip_terminal_break(&if_body, label);
    if !is_matching_break || then_body.is_empty() {
        return None;
    }
    if contains_label_control(&then_body, label) {
        return None;
    }

    let else_body = body[(if_idx + 1)..].to_vec();
    if else_body.is_empty() || contains_label_control(&else_body, label) {
        return None;
    }

    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut out = Vec::new();
    out.push(Node::Block {
        kind: BlockKind::If,
        label: None,
        header: if_header,
        body: then_body,
        footer: if_footer,
    });
    out.push(Node::Block {
        kind: BlockKind::Else,
        label: None,
        header: format!("{indent}else {{"),
        body: else_body,
        footer: format!("{indent}}}"),
    });
    Some(out)
}

fn strip_terminal_break(nodes: &[Node], label: &str) -> (Vec<Node>, bool) {
    let mut out = nodes.to_vec();
    while let Some(last) = out.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                out.pop();
            }
            Node::Line(line) => {
                if line.trim() == format!("break '{label};") {
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
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == format!("break '{label};") || t == format!("continue '{label};") {
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
        .filter_map(|(idx, node)| {
            if matches!(node, Node::Line(line) if line.trim().is_empty()) {
                None
            } else {
                Some(idx)
            }
        })
        .collect()
}
