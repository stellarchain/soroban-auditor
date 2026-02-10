use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopToWhile;

impl LoopToWhile {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopToWhile {
    fn name(&self) -> &'static str {
        "loop_to_while"
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
                if let Some((cond, trimmed_body, keep_label)) = try_loop_to_while(&label, &new_body)
                {
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    let header = if keep_label {
                        format!("{indent}'label{}: while {cond} {{", label)
                    } else {
                        format!("{indent}while {cond} {{")
                    };
                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: if keep_label { Some(label) } else { None },
                        header,
                        body: trimmed_body,
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

fn try_loop_to_while(label: &str, body: &[Node]) -> Option<(String, Vec<Node>, bool)> {
    let (first_idx, first_node) = first_non_empty(body)?;
    let (last_idx, last_node) = last_non_empty(body)?;

    let break_label = format!("break '{};", label);
    let continue_label = format!("continue '{};", label);

    let head_if = match first_node {
        Node::Block {
            kind: BlockKind::If,
            header,
            body: if_body,
            ..
        } => {
            if if_body.len() == 1 {
                if let Node::Line(line) = &if_body[0] {
                    if line.trim() == break_label {
                        Some(extract_if_condition(header)?)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };

    let tail_if = match last_node {
        Node::Block {
            kind: BlockKind::If,
            header,
            body: if_body,
            ..
        } => {
            if if_body.len() == 1 {
                if let Node::Line(line) = &if_body[0] {
                    if line.trim() == break_label {
                        Some(extract_if_condition(header)?)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };

    let mut remove_first = false;
    let mut remove_last = false;
    let cond = if let Some(cond) = head_if {
        remove_first = true;
        cond
    } else if let Some(cond) = tail_if {
        remove_last = true;
        cond
    } else {
        return None;
    };

    if has_other_label_control(
        body,
        label,
        first_idx,
        last_idx,
        &break_label,
        &continue_label,
    ) {
        return None;
    }

    let mut out = Vec::new();
    for (idx, node) in body.iter().enumerate() {
        if (remove_first && idx == first_idx) || (remove_last && idx == last_idx) {
            continue;
        }
        out.push(node.clone());
    }

    let while_cond = invert_condition(&cond);
    let keep_label = has_continue_label(&out, label);
    Some((while_cond, out, keep_label))
}

fn first_non_empty(body: &[Node]) -> Option<(usize, &Node)> {
    for (idx, node) in body.iter().enumerate() {
        if !node_is_empty_line(node) {
            return Some((idx, node));
        }
    }
    None
}

fn last_non_empty(body: &[Node]) -> Option<(usize, &Node)> {
    for (i, node) in body.iter().enumerate().rev() {
        if !node_is_empty_line(node) {
            return Some((i, node));
        }
    }
    None
}

fn node_is_empty_line(node: &Node) -> bool {
    match node {
        Node::Line(line) => line.trim().is_empty(),
        _ => false,
    }
}

fn extract_if_condition(header: &str) -> Option<String> {
    let t = header.trim();
    if !t.starts_with("if ") || !t.ends_with('{') {
        return None;
    }
    let inner = t.trim_start_matches("if ").trim_end_matches('{').trim();
    Some(inner.to_string())
}

fn invert_condition(cond: &str) -> String {
    if let Some(pos) = cond.find("!= 0") {
        let mut s = cond.to_string();
        s.replace_range(pos..pos + 4, "== 0");
        return s;
    }
    if let Some(pos) = cond.find("== 0") {
        let mut s = cond.to_string();
        s.replace_range(pos..pos + 4, "!= 0");
        return s;
    }
    format!("({cond}) == 0")
}

fn has_other_label_control(
    body: &[Node],
    label: &str,
    first_idx: usize,
    last_idx: usize,
    break_label: &str,
    continue_label: &str,
) -> bool {
    for (idx, node) in body.iter().enumerate() {
        if idx == first_idx || idx == last_idx {
            continue;
        }
        if contains_label_control(node, label, break_label, continue_label) {
            return true;
        }
    }
    false
}

fn has_continue_label(body: &[Node], label: &str) -> bool {
    for node in body {
        match node {
            Node::Line(line) => {
                if line.trim() == format!("continue '{};", label) {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if has_continue_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
}

fn contains_label_control(
    node: &Node,
    label: &str,
    break_label: &str,
    continue_label: &str,
) -> bool {
    match node {
        Node::Line(line) => {
            let t = line.trim();
            if t == break_label || t == continue_label {
                return true;
            }
            if t.contains(&format!("break '{};", label))
                || t.contains(&format!("continue '{};", label))
            {
                return true;
            }
            false
        }
        Node::Block { body, .. } => body
            .iter()
            .any(|n| contains_label_control(n, label, break_label, continue_label)),
    }
}
