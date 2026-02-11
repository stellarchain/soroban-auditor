use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct GuardEarlyReturn;

impl GuardEarlyReturn {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for GuardEarlyReturn {
    fn name(&self) -> &'static str {
        "guard_early_return"
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
        if let Some((label, tail_return)) = match_guard_block(&nodes, i) {
            if let Node::Block {
                kind,
                label: block_label,
                header,
                body,
                footer,
            } = nodes[i].clone()
            {
                let new_body = replace_guard_breaks(body, &label, &tail_return, changed);
                out.push(Node::Block {
                    kind,
                    label: block_label,
                    header,
                    body: new_body,
                    footer,
                });
                i += 1;
                continue;
            }
        }
        let mut node = nodes[i].clone();
        if let Node::Block { body, .. } = &mut node {
            let new_body = rewrite(std::mem::take(body), changed);
            *body = new_body;
        }
        out.push(node);
        i += 1;
    }
    out
}

fn match_guard_block(nodes: &[Node], idx: usize) -> Option<(String, Vec<String>)> {
    let block = nodes.get(idx)?;
    let (label, indent) = match block {
        Node::Block {
            kind: BlockKind::Other,
            label: Some(label),
            header,
            ..
        } if label.starts_with("__if_guard") => {
            let indent = header
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            (label.clone(), indent)
        }
        _ => return None,
    };
    let tail = find_tail_return(nodes, idx + 1)?;
    let mut out = Vec::new();
    out.extend(tail.into_iter().map(|l| format!("{indent}{l}")));
    Some((label, out))
}

fn find_tail_return(nodes: &[Node], start: usize) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    for node in nodes.iter().skip(start) {
        match node {
            Node::Line(line) if line.trim().is_empty() => continue,
            Node::Line(line) => {
                lines.push(line.trim().to_string());
                if lines.len() >= 2 {
                    break;
                }
            }
            _ => break,
        }
    }
    if lines.len() < 2 {
        return None;
    }
    if !lines[0].starts_with("self.global0 =") {
        return None;
    }
    if !lines[1].starts_with("return ") && !lines[1].starts_with("var") {
        return None;
    }
    if !lines[1].starts_with("return ") {
        lines[1] = format!("return {};", lines[1]);
    }
    Some(lines)
}

fn replace_guard_breaks(
    body: Vec<Node>,
    label: &str,
    tail_return: &[String],
    changed: &mut bool,
) -> Vec<Node> {
    let mut out = Vec::new();
    for node in body {
        match node {
            Node::Line(line) => {
                if line.contains(&format!("break '{label};")) {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    for tail in tail_return {
                        out.push(Node::Line(format!("{indent}{}", tail)));
                    }
                    *changed = true;
                } else {
                    out.push(Node::Line(line));
                }
            }
            Node::Block {
                kind,
                label: block_label,
                header,
                body,
                footer,
            } => {
                let new_body = replace_guard_breaks(body, label, tail_return, changed);
                out.push(Node::Block {
                    kind,
                    label: block_label,
                    header,
                    body: new_body,
                    footer,
                });
            }
        }
    }
    out
}
