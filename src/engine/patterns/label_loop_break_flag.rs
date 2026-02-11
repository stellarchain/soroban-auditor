use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelLoopBreakFlagPattern;

impl LabelLoopBreakFlagPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLoopBreakFlagPattern {
    fn name(&self) -> &'static str {
        "label_loop_break_flag"
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
                if let Some(new_body) = lower_label_loop_break(&label, &header, &rewritten_body) {
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header,
                        body: new_body,
                        footer,
                    });
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

fn lower_label_loop_break(label: &str, header: &str, body: &[Node]) -> Option<Vec<Node>> {
    let sig = significant_indices(body);
    if sig.len() < 2 {
        return None;
    }

    let loop_idx = sig
        .iter()
        .copied()
        .find(|idx| matches!(body[*idx], Node::Block { kind: BlockKind::Loop, .. }))?;

    let tail_start = loop_idx + 1;
    if tail_start >= body.len() {
        return None;
    }
    let tail = body[tail_start..].to_vec();
    if tail.iter().all(is_empty_line) {
        return None;
    }
    let prefix = body[..loop_idx].to_vec();

    if contains_label_control(&prefix, label) || contains_label_control(&tail, label) {
        return None;
    }

    let exit_label = format!("__exit_{label}");
    let break_flag = format!("__broke_{label}");

    let loop_node = body[loop_idx].clone();
    let (rewritten_loop, replaced_count) =
        rewrite_break_label_in_node(loop_node, label, &exit_label, &break_flag);
    if replaced_count == 0 {
        return None;
    }

    let indent = header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let inner_indent = format!("{indent}    ");

    let mut lowered = Vec::new();
    lowered.extend(prefix);
    lowered.push(Node::Line(format!("{inner_indent}let mut {break_flag}: bool = false;")));
    lowered.push(Node::Block {
        kind: BlockKind::Loop,
        label: Some(exit_label.clone()),
        header: format!("{inner_indent}'{exit_label}: loop {{"),
        body: vec![
            rewritten_loop,
            Node::Line(format!("{inner_indent}    break '{exit_label};")),
        ],
        footer: format!("{inner_indent}}}"),
    });
    lowered.push(Node::Block {
        kind: BlockKind::If,
        label: None,
        header: format!("{inner_indent}if !{break_flag} {{"),
        body: tail,
        footer: format!("{inner_indent}}}"),
    });
    Some(lowered)
}

fn rewrite_break_label_in_node(
    node: Node,
    source_label: &str,
    exit_label: &str,
    break_flag: &str,
) -> (Node, usize) {
    match node {
        Node::Line(line) => {
            let trimmed = line.trim();
            let target = format!("break '{source_label};");
            if trimmed == target {
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                let replacement = Node::Block {
                    kind: BlockKind::Other,
                    label: None,
                    header: format!("{indent}{{"),
                    body: vec![
                        Node::Line(format!("{indent}    {break_flag} = true;")),
                        Node::Line(format!("{indent}    break '{exit_label};")),
                    ],
                    footer: format!("{indent}}}"),
                };
                (replacement, 1)
            } else {
                (Node::Line(line), 0)
            }
        }
        Node::Block {
            kind,
            label,
            header,
            body,
            footer,
        } => {
            let mut replaced = 0usize;
            let mut new_body = Vec::with_capacity(body.len());
            for child in body {
                let (child_new, child_count) =
                    rewrite_break_label_in_node(child, source_label, exit_label, break_flag);
                replaced += child_count;
                new_body.push(child_new);
            }
            (
                Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                },
                replaced,
            )
        }
    }
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
        .filter_map(|(idx, node)| if is_empty_line(node) { None } else { Some(idx) })
        .collect()
}

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}
