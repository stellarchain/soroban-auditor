use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopBreakTailReturn;

impl LoopBreakTailReturn {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopBreakTailReturn {
    fn name(&self) -> &'static str {
        "loop_break_tail_return"
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
    let mut i = 0usize;
    while i < nodes.len() {
        let node = nodes[i].clone();
        if let Node::Block { kind: BlockKind::Loop, label, header, body, footer } = node {
            if let Some((tail_return, tail_len)) = find_tail_return(&nodes, i + 1) {
                let mut local_changed = false;
                let new_body = replace_breaks(&body, &tail_return, &mut local_changed);
                if local_changed && !contains_break_or_continue_outside_loops(&new_body) {
                    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    out.push(Node::Block {
                        kind: BlockKind::Other,
                        label,
                        header: format!("{indent}{{"),
                        body: new_body,
                        footer: format!("{indent}}}"),
                    });
                    *changed = true;
                    i += 1 + tail_len;
                    continue;
                }
            }
            let new_body = rewrite(body, changed);
            out.push(Node::Block { kind: BlockKind::Loop, label, header, body: new_body, footer });
            i += 1;
            continue;
        }
        let mut next = node;
        if let Node::Block { body, .. } = &mut next {
            let new_body = rewrite(std::mem::take(body), changed);
            *body = new_body;
        }
        out.push(next);
        i += 1;
    }
    out
}

fn find_tail_return(nodes: &[Node], start: usize) -> Option<(Vec<String>, usize)> {
    let mut lines: Vec<String> = Vec::new();
    let mut consumed = 0usize;
    let mut found_global = false;
    let mut found_return = false;
    for node in nodes.iter().skip(start) {
        match node {
            Node::Line(line) => {
                consumed += 1;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    if lines.is_empty() {
                        continue;
                    }
                    break;
                }
                let entry = trimmed.to_string();
                lines.push(entry.clone());
                if entry.starts_with("self.global0 =") {
                    found_global = true;
                    continue;
                }
                if entry.starts_with("return ") {
                    found_return = true;
                    break;
                }
                // allow other prepended statements
            }
            _ => break,
        }
    }
    if !lines.is_empty() {
        if let Some(last) = lines.last_mut() {
            if !last.ends_with(';') {
                *last = format!("{};", last);
            }
        }
    }
    if !found_global || !found_return {
        return None;
    }
    Some((lines, consumed))
}

fn replace_breaks(nodes: &[Node], tail_return: &[String], changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::new();
    for node in nodes.iter() {
        match node {
            Node::Line(line) => {
                if line.trim() == "break;" {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    for tail in tail_return {
                        out.push(Node::Line(format!("{indent}{}", tail)));
                    }
                    *changed = true;
                } else {
                    out.push(Node::Line(line.clone()));
                }
            }
            Node::Block { kind: BlockKind::Loop, .. } => {
                // Do not rewrite breaks inside nested loops.
                out.push(node.clone());
            }
            Node::Block { kind, label, header, body, footer } => {
                let new_body = replace_breaks(body, tail_return, changed);
                out.push(Node::Block { kind: kind.clone(), label: label.clone(), header: header.clone(), body: new_body, footer: footer.clone() });
            }
        }
    }
    out
}

fn contains_break_or_continue_outside_loops(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t == "break;" || t == "continue;" || t.starts_with("break '") || t.starts_with("continue '") {
                    return true;
                }
            }
            Node::Block { kind: BlockKind::Loop, .. } => {}
            Node::Block { body, .. } => {
                if contains_break_or_continue_outside_loops(body) {
                    return true;
                }
            }
        }
    }
    false
}
