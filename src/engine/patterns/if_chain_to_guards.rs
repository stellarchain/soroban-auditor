use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct IfChainToGuardsPattern;

impl IfChainToGuardsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for IfChainToGuardsPattern {
    fn name(&self) -> &'static str {
        "if_chain_to_guards"
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
    let if_node = match nodes.get(idx)? {
        Node::Block {
            kind: BlockKind::If,
            ..
        } => nodes[idx].clone(),
        _ => return None,
    };

    let (next_idx, next_line) = next_non_empty_line(nodes, idx + 1)?;
    if next_line.trim() != "unreachable!();" {
        return None;
    }

    let (conditions, mut tail_body, base_indent) = extract_if_chain(&if_node)?;
    if conditions.len() < 2 {
        return None;
    }
    if !contains_return(&tail_body) {
        return None;
    }
    dedent_nodes(&mut tail_body, 4 * conditions.len());

    let mut rewritten: Vec<Node> = Vec::new();
    for cond in conditions {
        rewritten.push(Node::Block {
            kind: BlockKind::If,
            label: None,
            header: format!("{base_indent}if !({cond}) {{"),
            body: vec![Node::Line(format!("{base_indent}    unreachable!();"))],
            footer: format!("{base_indent}}}"),
        });
    }
    rewritten.append(&mut tail_body);

    Some((rewritten, next_idx - idx + 1))
}

fn extract_if_chain(node: &Node) -> Option<(Vec<String>, Vec<Node>, String)> {
    let mut conditions = Vec::new();
    let mut current = node;
    let mut base_indent = String::new();

    loop {
        let Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } = current
        else {
            return None;
        };

        if base_indent.is_empty() {
            base_indent = indent_of(header);
        }
        let cond = extract_if_condition(header)?;
        conditions.push(cond);

        let non_empty: Vec<&Node> = body
            .iter()
            .filter(|n| !matches!(n, Node::Line(line) if line.trim().is_empty()))
            .collect();
        if non_empty.len() == 1
            && matches!(
                non_empty[0],
                Node::Block {
                    kind: BlockKind::If,
                    ..
                }
            )
        {
            current = non_empty[0];
            continue;
        }

        let tail = body.clone();
        return Some((conditions, tail, base_indent));
    }
}

fn extract_if_condition(header: &str) -> Option<String> {
    let t = header.trim();
    let rest = t.strip_prefix("if ")?;
    let cond = rest.strip_suffix(" {")?.trim();
    if cond.is_empty() {
        None
    } else {
        Some(cond.to_string())
    }
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

fn contains_return(nodes: &[Node]) -> bool {
    for node in nodes {
        match node {
            Node::Line(line) => {
                if line.trim_start().starts_with("return") {
                    return true;
                }
            }
            Node::Block { body, .. } => {
                if contains_return(body) {
                    return true;
                }
            }
        }
    }
    false
}

fn dedent_nodes(nodes: &mut [Node], spaces: usize) {
    for node in nodes {
        match node {
            Node::Line(line) => *line = dedent_line(line, spaces),
            Node::Block {
                header,
                body,
                footer,
                ..
            } => {
                *header = dedent_line(header, spaces);
                dedent_nodes(body, spaces);
                *footer = dedent_line(footer, spaces);
            }
        }
    }
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut count = 0usize;
    for ch in line.chars() {
        if ch == ' ' && count < spaces {
            count += 1;
        } else {
            break;
        }
    }
    line[count..].to_string()
}
