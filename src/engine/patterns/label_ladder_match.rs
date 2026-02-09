use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct LabelLadderMatch;

impl LabelLadderMatch {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelLadderMatch {
    fn name(&self) -> &'static str {
        "label_ladder_match"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_label_ladders(nodes, &mut changed);
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

fn rewrite_label_ladders(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
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
                if kind == BlockKind::Loop && label.is_some() {
                    if let Some(repl) = try_rewrite_ladder(&Node::Block {
                        kind: kind.clone(),
                        label: label.clone(),
                        header: header.clone(),
                        body: body.clone(),
                        footer: footer.clone(),
                    }) {
                        out.extend(repl.into_iter().map(Node::Line));
                        *changed = true;
                        continue;
                    }
                }
                let new_body = rewrite_label_ladders(body, changed);
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

fn try_rewrite_ladder(node: &Node) -> Option<Vec<String>> {
    let (match_block, label_tails, prefix_lines) = extract_ladder(node)?;
    if has_label_control(&prefix_lines) {
        return None;
    }
    for tail in label_tails.values() {
        if has_label_control(tail) {
            return None;
        }
    }
    let (match_header, match_body, match_footer) = match match_block {
        Node::Block {
            header, body, footer, ..
        } => (header, body, footer),
        _ => return None,
    };
    let arms = parse_match_arms(&match_body)?;
    let indent = match_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let mut out: Vec<String> = Vec::new();
    out.extend(prefix_lines);
    out.push(match_header);
    for (pat, label) in arms {
        if let Some(tail) = label_tails.get(&label) {
            let mut arm_lines: Vec<String> = Vec::new();
            arm_lines.push(format!("{}    {} => {{", indent, pat));
            for line in tail {
                arm_lines.push(line.clone());
            }
            arm_lines.push(format!("{}    }}", indent));
            out.extend(arm_lines);
        } else {
            out.push(format!("{}    {} => {{", indent, pat));
            out.push(format!("{}    }}", indent));
        }
    }
    out.push(match_footer);
    Some(out)
}

fn extract_ladder(node: &Node) -> Option<(Node, HashMap<String, Vec<String>>, Vec<String>)> {
    let mut label_tails: HashMap<String, Vec<String>> = HashMap::new();
    let mut current = node.clone();
    let mut last_match: Option<Node> = None;
    let mut prefix_lines: Vec<String> = Vec::new();

    loop {
        match current {
            Node::Block {
                kind: BlockKind::Loop,
                label: Some(label),
                body,
                ..
            } => {
                if body.is_empty() {
                    return None;
                }
                let (prefix_nodes, inner_block, tail_nodes) = split_loop_body(body)?;
                let mut tail_lines = flatten_nodes(&tail_nodes);
                trim_trailing_breaks(&mut tail_lines);
                if !tail_lines.is_empty() {
                    label_tails.insert(label, tail_lines);
                } else {
                    label_tails.insert(label, Vec::new());
                }

                if !prefix_nodes.is_empty() {
                    prefix_lines.extend(flatten_nodes(&prefix_nodes));
                }

                match inner_block {
                    Node::Block { kind: BlockKind::Loop, .. } => {
                        current = inner_block;
                        continue;
                    }
                    Node::Block { header, body, footer, .. } => {
                        if header.trim_start().starts_with("match ") {
                            last_match = Some(Node::Block {
                                kind: BlockKind::Other,
                                label: None,
                                header,
                                body,
                                footer,
                            });
                            break;
                        }
                        return None;
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }
    }

    let match_block = last_match?;
    Some((match_block, label_tails, prefix_lines))
}

fn parse_match_arms(lines: &[Node]) -> Option<Vec<(String, String)>> {
    let mut out: Vec<(String, String)> = Vec::new();
    for node in lines {
        let line = match node {
            Node::Line(l) => l.trim().to_string(),
            _ => continue,
        };
        if let Some(pos) = line.find("=> break '") {
            let pat = line[..pos].trim().trim_end_matches(',').to_string();
            let rest = &line[pos + "=> break '".len()..];
            let label = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<String>();
            if !pat.is_empty() && !label.is_empty() {
                out.push((pat, label));
            }
        } else if let Some(pos) = line.find("=> break ") {
            let pat = line[..pos].trim().trim_end_matches(',').to_string();
            let label = "break".to_string();
            if !pat.is_empty() {
                out.push((pat, label));
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn trim_trailing_breaks(lines: &mut Vec<String>) {
    while let Some(last) = lines.last() {
        let t = last.trim();
        if t == "break;" || t.starts_with("break '") {
            lines.pop();
            continue;
        }
        if t.is_empty() {
            lines.pop();
            continue;
        }
        break;
    }
}

fn has_label_control(lines: &[String]) -> bool {
    for line in lines {
        if line.contains("break '") || line.contains("continue '") {
            return true;
        }
    }
    false
}

fn split_loop_body(body: Vec<Node>) -> Option<(Vec<Node>, Node, Vec<Node>)> {
    let mut idx = None;
    for (i, node) in body.iter().enumerate() {
        match node {
            Node::Block { kind: BlockKind::Loop, .. } => {
                idx = Some(i);
                break;
            }
            Node::Block { header, .. } if header.trim_start().starts_with("match ") => {
                idx = Some(i);
                break;
            }
            _ => {}
        }
    }
    let i = idx?;
    let mut iter = body.into_iter();
    let prefix_nodes: Vec<Node> = iter.by_ref().take(i).collect();
    let inner = iter.next()?;
    let tail_nodes: Vec<Node> = iter.collect();
    Some((prefix_nodes, inner, tail_nodes))
}
