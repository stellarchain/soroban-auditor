use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct LoopSingleIfToWhilePattern;

impl LoopSingleIfToWhilePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopSingleIfToWhilePattern {
    fn name(&self) -> &'static str {
        "loop_single_if_to_while"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let rewritten = rewrite(nodes, &mut changed);
        if !changed {
            return None;
        }
        Some(FunctionBlock {
            header: block.header.clone(),
            body: flatten_nodes(&rewritten),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::Loop,
                label,
                header,
                body,
                footer,
            } => {
                let body = rewrite(body, changed);
                if let Some(while_node) = lower_loop_single_if(&header, body.clone()) {
                    out.push(while_node);
                    *changed = true;
                    continue;
                }
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label,
                    header,
                    body,
                    footer,
                });
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => out.push(Node::Block {
                kind,
                label,
                header,
                body: rewrite(body, changed),
                footer,
            }),
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }
    out
}

fn lower_loop_single_if(loop_header: &str, body: Vec<Node>) -> Option<Node> {
    let mut significant: Vec<Node> = Vec::new();
    for n in body {
        if !is_empty_line(&n) {
            significant.push(n);
        }
    }
    if significant.len() != 1 {
        return None;
    }

    let only = significant.remove(0);
    let (if_header, if_body) = match only {
        Node::Block {
            kind: BlockKind::If,
            header,
            body,
            ..
        } => (header, body),
        _ => return None,
    };

    let cond = parse_if_condition(&if_header)?;
    let indent = loop_header
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    Some(Node::Block {
        kind: BlockKind::Loop,
        label: None,
        header: format!("{indent}while {cond} {{"),
        body: dedent_nodes(if_body, 4),
        footer: format!("{indent}}}"),
    })
}

fn parse_if_condition(header: &str) -> Option<String> {
    let t = header.trim();
    let cond = t.strip_prefix("if ")?.strip_suffix(" {")?;
    Some(cond.trim().to_string())
}

fn is_empty_line(node: &Node) -> bool {
    matches!(node, Node::Line(line) if line.trim().is_empty())
}

fn dedent_nodes(nodes: Vec<Node>, spaces: usize) -> Vec<Node> {
    nodes
        .into_iter()
        .map(|n| match n {
            Node::Line(line) => Node::Line(dedent_line(&line, spaces)),
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => Node::Block {
                kind,
                label,
                header: dedent_line(&header, spaces),
                body: dedent_nodes(body, spaces),
                footer: dedent_line(&footer, spaces),
            },
        })
        .collect()
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut taken = 0usize;
    for ch in line.chars() {
        if ch == ' ' && taken < spaces {
            taken += 1;
        } else {
            break;
        }
    }
    line[taken..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowers_loop_single_if_to_while() {
        let p = LoopSingleIfToWhilePattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        if b != 16 {".to_string(),
                "            b = b.wrapping_add(8);".to_string(),
                "        }".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    while b != 16 {".to_string(),
                "        b = b.wrapping_add(8);".to_string(),
                "    }".to_string(),
            ]
        );
    }
}
