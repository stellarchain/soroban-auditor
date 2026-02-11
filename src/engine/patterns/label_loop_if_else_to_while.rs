use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LabelLoopIfElseToWhilePattern;

impl LabelLoopIfElseToWhilePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLoopIfElseToWhilePattern {
    fn name(&self) -> &'static str {
        "label_loop_if_else_to_while"
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
                label,
                header,
                body,
                footer,
            } => {
                let rewritten_body = rewrite(body, changed);
                if let Some((cond, then_body, mut else_body, tail_body)) =
                    extract_loop_if_else_shape(&rewritten_body, label.as_deref(), &header)
                {
                    strip_terminal_continue(&mut else_body);
                    let indent = header
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    let while_header = format!("{indent}while {} {{", invert_condition(&cond));
                    out.push(Node::Block {
                        kind: BlockKind::Loop,
                        label: None,
                        header: while_header,
                        body: else_body,
                        footer: format!("{indent}}}"),
                    });
                    out.extend(then_body);
                    out.extend(tail_body);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label,
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
            line => out.push(line),
        }
    }
    out
}

fn extract_loop_if_else_shape(
    body: &[Node],
    loop_label: Option<&str>,
    loop_header: &str,
) -> Option<(String, Vec<Node>, Vec<Node>, Vec<Node>)> {
    let debug = std::env::var("SOROBAN_AUDITOR_DEBUG_LOOPCFG").is_ok();
    let sig = significant_indices(body);
    if sig.len() < 2 {
        if debug {
            eprintln!("loopcfg: sig<2 @ {}", loop_header.trim());
        }
        return None;
    }
    let if_idx = sig[0];
    let else_idx = sig[1];
    if else_idx <= if_idx {
        return None;
    }

    let (if_header, then_body) = match &body[if_idx] {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => (header, body.clone()),
        _ => {
            if debug {
                eprintln!("loopcfg: first sig not if @ {}", loop_header.trim());
            }
            return None;
        }
    };
    let else_body = match &body[else_idx] {
        Node::Block {
            kind: BlockKind::Else,
            body,
            ..
        } => body.clone(),
        _ => {
            if debug {
                eprintln!("loopcfg: second sig not else @ {}", loop_header.trim());
            }
            return None;
        }
    };

    // Keep transformation safe:
    // - else branch must terminate the loop iteration with continue to same loop
    // - if-branch must not break/continue same label
    if !ends_with_continue_for_loop(&else_body, loop_label) {
        if debug {
            eprintln!(
                "loopcfg: else not ending continue for {:?} @ {}",
                loop_label,
                loop_header.trim()
            );
        }
        return None;
    }
    if contains_control_to_label(&then_body, loop_label) {
        if debug {
            eprintln!(
                "loopcfg: then contains control to label {:?} @ {}",
                loop_label,
                loop_header.trim()
            );
        }
        return None;
    }

    // Remaining significant nodes in loop body must be only `unreachable!();`
    let tail_start = else_idx + 1;
    let tail_body = if tail_start < body.len() {
        body[tail_start..].to_vec()
    } else {
        Vec::new()
    };

    let cond = parse_if_condition(if_header)?;
    Some((cond, then_body, else_body, tail_body))
}

fn significant_indices(nodes: &[Node]) -> Vec<usize> {
    nodes.iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            Node::Line(line) if line.trim().is_empty() => None,
            _ => Some(i),
        })
        .collect()
}

fn parse_if_condition(header: &str) -> Option<String> {
    let t = header.trim();
    let cond = t.strip_prefix("if ")?.strip_suffix(" {")?;
    Some(cond.trim().to_string())
}

fn ends_with_continue_for_loop(nodes: &[Node], loop_label: Option<&str>) -> bool {
    for n in nodes.iter().rev() {
        match n {
            Node::Line(line) if line.trim().is_empty() => continue,
            Node::Line(line) => return is_continue_for_loop(line.trim(), loop_label),
            _ => return false,
        }
    }
    false
}

fn is_continue_for_loop(line: &str, loop_label: Option<&str>) -> bool {
    if let Some(label) = loop_label {
        return line == format!("continue '{label};");
    }
    line == "continue;"
}

fn strip_terminal_continue(nodes: &mut Vec<Node>) {
    while let Some(last) = nodes.last() {
        match last {
            Node::Line(line) if line.trim().is_empty() => {
                nodes.pop();
            }
            Node::Line(line)
                if line.trim() == "continue;" || line.trim().starts_with("continue '") =>
            {
                nodes.pop();
                break;
            }
            _ => break,
        }
    }
}

fn contains_control_to_label(nodes: &[Node], loop_label: Option<&str>) -> bool {
    for n in nodes {
        match n {
            Node::Line(line) => {
                let t = line.trim();
                if let Some(label) = loop_label {
                    if t == format!("break '{label};") || t == format!("continue '{label};") {
                        return true;
                    }
                } else if t == "break;" || t == "continue;" {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_control_to_label(body, loop_label) {
                    return true;
                }
            }
        }
    }
    false
}

fn invert_condition(cond: &str) -> String {
    let t = cond.trim();
    if let Some((lhs, rhs)) = t.split_once(" == ") {
        return format!("{} != {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = t.split_once(" != ") {
        return format!("{} == {}", lhs.trim(), rhs.trim());
    }
    format!("!({t})")
}
