use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

pub struct IrLabelCleanup;

impl IrLabelCleanup {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for IrLabelCleanup {
    fn name(&self) -> &'static str {
        "ir_label_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut used = HashSet::new();
        let mut defined = HashSet::new();
        collect_used_labels(&nodes, &mut used);
        collect_defined_labels(&nodes, &mut defined);
        let mut changed = false;
        let new_nodes = rewrite_unused_labels(nodes, &used, &defined, &mut changed);
        if !changed {
            let cleaned = replace_unknown_label_controls(&block.body, &defined);
            if cleaned == block.body {
                return None;
            }
            return Some(FunctionBlock {
                header: block.header.clone(),
                body: cleaned,
                footer: block.footer.clone(),
                indent: block.indent.clone(),
                name: block.name.clone(),
            });
        }
        let new_body = flatten_nodes(&new_nodes);
        let new_body = replace_unknown_label_controls(&new_body, &defined);
        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn collect_used_labels(nodes: &[Node], used: &mut HashSet<String>) {
    for node in nodes {
        match node {
            Node::Line(line) => {
                if let Some(pos) = line.find("break '") {
                    let rest = &line[pos + "break '".len()..];
                    let label = rest
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect::<String>();
                    if !label.is_empty() {
                        used.insert(label);
                    }
                }
                if let Some(pos) = line.find("continue '") {
                    let rest = &line[pos + "continue '".len()..];
                    let label = rest
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect::<String>();
                    if !label.is_empty() {
                        used.insert(label);
                    }
                }
            }
            Node::Block { body, .. } => collect_used_labels(body, used),
        }
    }
}

fn rewrite_unused_labels(
    nodes: Vec<Node>,
    used: &HashSet<String>,
    defined: &HashSet<String>,
    changed: &mut bool,
) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    for node in nodes {
        match node {
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite_unused_labels(body, used, defined, changed);
                if let Some(ref label) = label {
                    if !used.contains(label) {
                        if kind == BlockKind::Loop {
                            let prefix_len = header.len() - header.trim_start().len();
                            let prefix = &header[..prefix_len];
                            let new_header = format!("{}loop {{", prefix);
                            out.push(Node::Block {
                                kind,
                                label: None,
                                header: new_header,
                                body: new_body,
                                footer,
                            });
                            *changed = true;
                            continue;
                        }
                        if kind == BlockKind::Other {
                            let prefix_len = header.len() - header.trim_start().len();
                            let prefix = &header[..prefix_len];
                            let new_header = format!("{}{{", prefix);
                            out.push(Node::Block {
                                kind,
                                label: None,
                                header: new_header,
                                body: new_body,
                                footer,
                            });
                            *changed = true;
                            continue;
                        }
                    }
                }
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
            Node::Line(line) => {
                if let Some((kind, label)) = parse_label_control(&line) {
                    if !defined.contains(&label) {
                        let indent = line
                            .chars()
                            .take_while(|c| c.is_whitespace())
                            .collect::<String>();
                        let new_line = match kind {
                            ControlKind::Break => format!("{indent}break;"),
                            ControlKind::Continue => format!("{indent}continue;"),
                        };
                        out.push(Node::Line(new_line));
                        *changed = true;
                        continue;
                    }
                }
                if let Some(replaced) = replace_inline_label_controls(&line, defined) {
                    out.push(Node::Line(replaced));
                    *changed = true;
                    continue;
                }
                out.push(Node::Line(line));
            }
        }
    }
    out
}

#[derive(Clone, Copy)]
enum ControlKind {
    Break,
    Continue,
}

fn parse_label_control(line: &str) -> Option<(ControlKind, String)> {
    let trimmed = line.trim();
    let (kind, rest) = if let Some(rest) = trimmed.strip_prefix("break '") {
        (ControlKind::Break, rest)
    } else if let Some(rest) = trimmed.strip_prefix("continue '") {
        (ControlKind::Continue, rest)
    } else {
        return None;
    };
    let label = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>();
    if label.is_empty() {
        return None;
    }
    Some((kind, label))
}

fn replace_inline_label_controls(line: &str, defined: &HashSet<String>) -> Option<String> {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    let mut changed = false;
    loop {
        let break_pos = rest.find("break '");
        let cont_pos = rest.find("continue '");
        let (pos, kind) = match (break_pos, cont_pos) {
            (None, None) => {
                out.push_str(rest);
                break;
            }
            (Some(b), None) => (b, ControlKind::Break),
            (None, Some(c)) => (c, ControlKind::Continue),
            (Some(b), Some(c)) => {
                if b <= c {
                    (b, ControlKind::Break)
                } else {
                    (c, ControlKind::Continue)
                }
            }
        };
        out.push_str(&rest[..pos]);
        let after = &rest[pos
            + if matches!(kind, ControlKind::Break) {
                "break '".len()
            } else {
                "continue '".len()
            }..];
        let label = after
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>();
        if label.is_empty() {
            out.push_str(&rest[pos..]);
            break;
        }
        let tail = &after[label.len()..];
        if !defined.contains(&label) {
            match kind {
                ControlKind::Break => out.push_str("break"),
                ControlKind::Continue => out.push_str("continue"),
            }
            changed = true;
        } else {
            match kind {
                ControlKind::Break => out.push_str("break '"),
                ControlKind::Continue => out.push_str("continue '"),
            }
            out.push_str(&label);
        }
        rest = tail;
    }
    if changed {
        Some(out)
    } else {
        None
    }
}

fn collect_defined_labels(nodes: &[Node], defined: &mut HashSet<String>) {
    for node in nodes {
        match node {
            Node::Block {
                label: Some(label),
                body,
                ..
            } => {
                defined.insert(label.clone());
                collect_defined_labels(body, defined);
            }
            Node::Block {
                label: None, body, ..
            } => collect_defined_labels(body, defined),
            Node::Line(_) => {}
        }
    }
}

fn replace_unknown_label_controls(lines: &[String], defined: &HashSet<String>) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        if let Some(replaced) = replace_inline_label_controls(line, defined) {
            out.push(replaced);
        } else {
            out.push(line.clone());
        }
    }
    out
}
