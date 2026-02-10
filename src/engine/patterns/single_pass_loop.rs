use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct SinglePassLoopCleanup;

impl SinglePassLoopCleanup {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SinglePassLoopCleanup {
    fn name(&self) -> &'static str {
        "single_pass_loop_cleanup"
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
                kind: BlockKind::Loop,
                label: Some(label),
                header,
                body,
                footer,
            } => {
                let mut new_body = rewrite(body, changed);
                if let Some(inlined) = try_elide_single_pass(label.as_str(), &new_body) {
                    out.extend(inlined);
                    *changed = true;
                    continue;
                }
                if can_convert_label_loop_to_block(label.as_str(), &new_body) {
                    rewrite_top_level_breaks_to_label(label.as_str(), &mut new_body, changed);
                    let new_header = header.replacen(": loop {", ": {", 1);
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label: Some(label),
                        header: new_header,
                        body: new_body,
                        footer,
                    });
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
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

fn try_elide_single_pass(label: &str, body: &[Node]) -> Option<Vec<Node>> {
    if has_nested_block(body) {
        return None;
    }
    let mut lines: Vec<String> = Vec::new();
    for node in body {
        if let Node::Line(line) = node {
            lines.push(line.clone());
        } else {
            return None;
        }
    }
    if lines.is_empty() {
        return None;
    }
    let mut trimmed = lines.clone();
    while trimmed.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        trimmed.pop();
    }
    if trimmed.is_empty() {
        return None;
    }
    let last = trimmed.last().unwrap().trim();
    let break_line = format!("break '{};", label);
    if last != break_line {
        return None;
    }
    // Ensure no other break/continue to this label inside the body.
    for line in &trimmed[..trimmed.len() - 1] {
        let t = line.trim();
        if t.contains(&break_line) || t.contains(&format!("continue '{};", label)) {
            return None;
        }
    }
    trimmed.pop();
    let out = trimmed.into_iter().map(Node::Line).collect::<Vec<_>>();
    Some(out)
}

fn has_nested_block(body: &[Node]) -> bool {
    for node in body {
        if let Node::Block { .. } = node {
            return true;
        }
    }
    false
}

fn can_convert_label_loop_to_block(label: &str, body: &[Node]) -> bool {
    if contains_continue_label(body, label) {
        return false;
    }
    !contains_continue_at_loop_depth0(body, 0)
}

fn contains_continue_label(body: &[Node], label: &str) -> bool {
    for node in body {
        match node {
            Node::Line(line) => {
                if line.trim().contains(&format!("continue '{};", label)) {
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

fn contains_continue_at_loop_depth0(body: &[Node], loop_depth: usize) -> bool {
    for node in body {
        match node {
            Node::Line(line) => {
                if loop_depth == 0 && line.trim() == "continue;" {
                    return true;
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                if contains_continue_at_loop_depth0(body, next_depth) {
                    return true;
                }
            }
        }
    }
    false
}

fn rewrite_top_level_breaks_to_label(label: &str, body: &mut [Node], changed: &mut bool) {
    rewrite_breaks_in_nodes(label, body, changed, 0);
}

fn rewrite_breaks_in_nodes(label: &str, body: &mut [Node], changed: &mut bool, loop_depth: usize) {
    for node in body.iter_mut() {
        match node {
            Node::Line(line) => {
                if loop_depth == 0 && line.trim() == "break;" {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    *line = format!("{indent}break '{};", label);
                    *changed = true;
                }
            }
            Node::Block { kind, body, .. } => {
                let next_depth = if *kind == BlockKind::Loop {
                    loop_depth + 1
                } else {
                    loop_depth
                };
                rewrite_breaks_in_nodes(label, body, changed, next_depth);
            }
        }
    }
}
