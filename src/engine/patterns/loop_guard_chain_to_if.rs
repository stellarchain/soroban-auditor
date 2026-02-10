use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopGuardChainToIf;

impl LoopGuardChainToIf {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopGuardChainToIf {
    fn name(&self) -> &'static str {
        "loop_guard_chain_to_if"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_sequence(nodes, &mut changed);
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

fn rewrite_sequence(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < nodes.len() {
        if let Some((replacement, consumed)) = try_rewrite(&nodes, i) {
            out.extend(replacement);
            i += consumed;
            *changed = true;
            continue;
        }

        let mut node = nodes[i].clone();
        if let Node::Block { body, .. } = &mut node {
            *body = rewrite_sequence(std::mem::take(body), changed);
        }
        out.push(node);
        i += 1;
    }

    out
}

fn try_rewrite(nodes: &[Node], idx: usize) -> Option<(Vec<Node>, usize)> {
    let (loop_header, loop_body, loop_footer) = match nodes.get(idx)? {
        Node::Block {
            kind: BlockKind::Loop,
            header,
            body,
            footer,
            ..
        } => (header.clone(), body.clone(), footer.clone()),
        _ => return None,
    };

    let (next_idx, next_line) = next_non_empty_line(nodes, idx + 1)?;
    if next_line.trim() != "unreachable!();" {
        return None;
    }

    let (guards, tail) = split_leading_guard_ifs(&loop_body)?;
    if guards.is_empty() {
        return None;
    }
    if !tail_has_return_without_loop_control(&tail) {
        return None;
    }

    let indent = indent_of(&loop_header);
    let total = guards.len();
    let mut nested = tail;
    indent_nodes(&mut nested, 4 * total);
    for (k, cond) in guards.into_iter().rev().enumerate() {
        let depth_from_outer = total.saturating_sub(k + 1);
        let guard_indent = format!("{indent}{}", " ".repeat(4 * (depth_from_outer + 1)));
        nested = vec![Node::Block {
            kind: BlockKind::If,
            label: None,
            header: format!("{guard_indent}if !({cond}) {{"),
            body: nested,
            footer: format!("{guard_indent}}}"),
        }];
    }

    let mut out = Vec::new();
    // Preserve loop footer indentation context by keeping rewritten ifs at the same level as loop.
    out.extend(nested);
    out.push(Node::Line(next_line));

    // Require loop footer to exist to avoid malformed parse trees.
    if loop_footer.is_empty() {
        return None;
    }
    Some((out, next_idx - idx + 1))
}

fn split_leading_guard_ifs(body: &[Node]) -> Option<(Vec<String>, Vec<Node>)> {
    let mut guards: Vec<String> = Vec::new();
    let mut idx = 0usize;

    while idx < body.len() {
        let node = &body[idx];
        let Node::Block {
            kind: BlockKind::If,
            header,
            body: if_body,
            ..
        } = node
        else {
            break;
        };

        if !if_body_is_simple_break(if_body) {
            break;
        }

        let cond = extract_if_condition(header)?;
        guards.push(cond);
        idx += 1;
    }

    if guards.is_empty() {
        return None;
    }
    Some((guards, body[idx..].to_vec()))
}

fn if_body_is_simple_break(body: &[Node]) -> bool {
    let mut saw_break = false;
    for node in body {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t.is_empty() {
                    continue;
                }
                if t == "break;" {
                    if saw_break {
                        return false;
                    }
                    saw_break = true;
                    continue;
                }
                return false;
            }
            Node::Block { .. } => return false,
        }
    }
    saw_break
}

fn extract_if_condition(header: &str) -> Option<String> {
    let trimmed = header.trim();
    let rest = trimmed.strip_prefix("if ")?;
    let cond = rest.strip_suffix(" {")?.trim();
    if cond.is_empty() {
        None
    } else {
        Some(cond.to_string())
    }
}

fn tail_has_return_without_loop_control(nodes: &[Node]) -> bool {
    let mut has_return = false;
    for node in nodes {
        match node {
            Node::Line(line) => {
                let t = line.trim();
                if t.starts_with("return") {
                    has_return = true;
                }
                if t == "break;"
                    || t == "continue;"
                    || t.starts_with("break '")
                    || t.starts_with("continue '")
                {
                    return false;
                }
            }
            Node::Block { kind, body, .. } => {
                if *kind == BlockKind::Loop {
                    return false;
                }
                if !tail_has_return_without_loop_control(body) {
                    return false;
                }
            }
        }
    }
    has_return
}

fn next_non_empty_line(nodes: &[Node], mut idx: usize) -> Option<(usize, String)> {
    while idx < nodes.len() {
        match &nodes[idx] {
            Node::Line(line) if line.trim().is_empty() => idx += 1,
            Node::Line(line) => return Some((idx, line.clone())),
            _ => return None,
        }
    }
    None
}

fn indent_of(s: &str) -> String {
    s.chars().take_while(|c| c.is_whitespace()).collect()
}

fn indent_nodes(nodes: &mut [Node], spaces: usize) {
    let pad = " ".repeat(spaces);
    for node in nodes {
        match node {
            Node::Line(line) => {
                *line = format!("{pad}{line}");
            }
            Node::Block {
                header,
                body,
                footer,
                ..
            } => {
                *header = format!("{pad}{header}");
                indent_nodes(body, spaces);
                *footer = format!("{pad}{footer}");
            }
        }
    }
}
