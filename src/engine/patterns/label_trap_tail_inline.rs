use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelTrapTailInlinePattern;

impl LabelTrapTailInlinePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelTrapTailInlinePattern {
    fn name(&self) -> &'static str {
        "label_trap_tail_inline"
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
    let mut i = 0usize;
    while i < nodes.len() {
        let mut node = nodes[i].clone();
        if let Node::Block {
            kind: BlockKind::Other,
            label: Some(label),
            header,
            body,
            footer,
        } = node
        {
            let rewritten_body = rewrite_sequence(body, changed);
            if let Some(next_idx) = next_non_empty_line_idx(&nodes, i + 1) {
                if is_unreachable_line(&nodes[next_idx]) && !contains_continue_label(&rewritten_body, &label)
                {
                    let mut new_body = rewritten_body.clone();
                    if rewrite_break_label_to_unreachable(&mut new_body, &label) {
                        let indent = header
                            .chars()
                            .take_while(|c| c.is_whitespace())
                            .collect::<String>();
                        out.push(Node::Block {
                            kind: BlockKind::Other,
                            label: None,
                            header: format!("{indent}{{"),
                            body: new_body,
                            footer,
                        });
                        i = next_idx + 1;
                        *changed = true;
                        continue;
                    }
                }
            }
            out.push(Node::Block {
                kind: BlockKind::Other,
                label: Some(label),
                header,
                body: rewritten_body,
                footer,
            });
            i += 1;
            continue;
        }

        if let Node::Block { body, .. } = &mut node {
            *body = rewrite_sequence(std::mem::take(body), changed);
        }
        out.push(node);
        i += 1;
    }
    out
}

fn next_non_empty_line_idx(nodes: &[Node], mut idx: usize) -> Option<usize> {
    while idx < nodes.len() {
        match &nodes[idx] {
            Node::Line(line) if line.trim().is_empty() => idx += 1,
            Node::Line(_) => return Some(idx),
            _ => return None,
        }
    }
    None
}

fn is_unreachable_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim() == "unreachable!();")
}

fn contains_continue_label(nodes: &[Node], label: &str) -> bool {
    let target = format!("continue '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_continue_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
}

fn rewrite_break_label_to_unreachable(nodes: &mut [Node], label: &str) -> bool {
    let target = format!("break '{};", label);
    let mut changed = false;
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *line = format!("{indent}unreachable!();");
                    changed = true;
                }
            }
            Node::Block { body, .. } => {
                if rewrite_break_label_to_unreachable(body, label) {
                    changed = true;
                }
            }
        }
    }
    changed
}
