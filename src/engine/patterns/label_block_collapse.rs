use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelBlockCollapse;

impl LabelBlockCollapse {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelBlockCollapse {
    fn name(&self) -> &'static str {
        "label_block_collapse"
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
        let new_body = flatten_nodes(&new_nodes);
        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
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
                let new_body = rewrite(body, changed);
                if let Some(inner) = collapse_chain(&label, &new_body) {
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header,
                        body: inner,
                        footer,
                    });
                    *changed = true;
                    continue;
                }
                if !contains_control_for_label(&new_body, &label) {
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
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Other,
                    label: Some(label),
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
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn collapse_chain(label: &str, body: &[Node]) -> Option<Vec<Node>> {
    let mut labels = vec![label.to_string()];
    let mut current: Vec<Node> = body.to_vec();
    loop {
        let inner = single_non_trivial(&current)?;
        match inner {
            Node::Block {
                kind: BlockKind::Other,
                label: Some(next),
                body: next_body,
                ..
            } => {
                labels.push(next.clone());
                current = next_body.clone();
                continue;
            }
            _ => break,
        }
    }
    if labels.len() <= 1 {
        return None;
    }
    if contains_continue_label(&current, &labels) {
        return None;
    }
    let mut rewritten = current.clone();
    rewrite_breaks_to_outer(&mut rewritten, &labels, &labels[0]);
    Some(rewritten)
}

fn single_non_trivial(nodes: &[Node]) -> Option<Node> {
    let mut out: Vec<Node> = Vec::new();
    for node in nodes {
        match node {
            Node::Line(line) if line.trim().is_empty() => {}
            _ => out.push(node.clone()),
        }
    }
    if out.len() == 1 {
        Some(out.remove(0))
    } else {
        None
    }
}

fn contains_continue_label(nodes: &[Node], labels: &[String]) -> bool {
    let continues: Vec<String> = labels.iter().map(|l| format!("continue '{};", l)).collect();
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if continues.iter().any(|c| c == t) {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_continue_label(body, labels) {
                    return true;
                }
            }
        }
    }
    false
}

fn contains_control_for_label(nodes: &[Node], label: &str) -> bool {
    let break_target = format!("break '{};", label);
    let continue_target = format!("continue '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == break_target || t == continue_target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_control_for_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
}

fn rewrite_breaks_to_outer(nodes: &mut [Node], labels: &[String], outer: &str) {
    let breaks: Vec<String> = labels.iter().map(|l| format!("break '{};", l)).collect();
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if breaks.iter().any(|b| b == t) {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *line = format!("{indent}break '{};", outer);
                }
            }
            Node::Block { body, .. } => {
                rewrite_breaks_to_outer(body, labels, outer);
            }
        }
    }
}
