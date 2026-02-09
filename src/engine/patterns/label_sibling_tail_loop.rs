use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelSiblingTailLoop;

impl LabelSiblingTailLoop {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelSiblingTailLoop {
    fn name(&self) -> &'static str {
        "label_sibling_tail_loop"
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
    let mut out: Vec<Node> = Vec::new();
    let mut i = 0;
    while i < nodes.len() {
        let node = nodes[i].clone();
        if let Node::Block {
            kind: BlockKind::Other,
            label: Some(ref label),
            ref header,
            ref body,
            ..
        } = node
        {
            if i + 1 < nodes.len() {
                let tail = nodes[i + 1..].to_vec();
                if !contains_break_label(&tail, label)
                    && !contains_continue_label(&tail, label)
                    && contains_break_label(body, label)
                {
                    let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    let flag = format!("__exit_{}", label);
                    out.push(Node::Line(format!("{indent}let mut {flag}: i32 = 0;")));

                    let mut loop_body = rewrite(body.clone(), changed);
                    rewrite_breaks_to_loop(&mut loop_body, label, &flag);
                    if !ends_with_break(&loop_body) {
                        loop_body.push(Node::Line(format!("{indent}    break;")));
                    }

                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: None,
                        header: format!("{indent}loop {{"),
                        body: loop_body,
                        footer: format!("{indent}}}"),
                    });

                    let tail = rewrite(tail, changed);
                    out.push(Node::Block {
                        kind: BlockKind::If,
                        label: None,
                        header: format!("{indent}if {flag} == 0 {{"),
                        body: tail,
                        footer: format!("{indent}}}"),
                    });
                    *changed = true;
                    break;
                }
            }
        }

        match node {
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
        i += 1;
    }
    out
}

fn rewrite_breaks_to_loop(nodes: &mut [Node], label: &str, flag: &str) {
    let target = format!("break '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    *line = format!("{indent}{flag} = 1; break;");
                }
            }
            Node::Block { body, .. } => rewrite_breaks_to_loop(body, label, flag),
        }
    }
}

fn contains_break_label(nodes: &[Node], label: &str) -> bool {
    let target = format!("break '{};", label);
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim() == target {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_break_label(body, label) {
                    return true;
                }
            }
        }
    }
    false
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

fn ends_with_break(nodes: &[Node]) -> bool {
    for node in nodes.iter().rev() {
        match node {
            Node::Line(line) if line.trim().is_empty() => continue,
            Node::Line(line) => return line.trim() == "break;",
            _ => return false,
        }
    }
    false
}
