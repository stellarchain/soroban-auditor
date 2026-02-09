#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockKind {
    Loop,
    If,
    Else,
    Other,
}

#[derive(Debug, Clone)]
pub enum Node {
    Line(String),
    Block {
        kind: BlockKind,
        label: Option<String>,
        header: String,
        body: Vec<Node>,
        footer: String,
    },
}

pub fn parse_lines(lines: &[String]) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    let mut stack: Vec<(BlockKind, Option<String>, String, Vec<Node>)> = Vec::new();

    let mut expanded: Vec<String> = Vec::new();
    for line in lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with("} else if ") || trimmed.starts_with("} else if(") {
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            let rest = trimmed.trim_start_matches("} ").trim_start();
            expanded.push(format!("{indent}}}"));
            expanded.push(format!("{indent}{rest}"));
            continue;
        }
        if trimmed.starts_with("} else {") {
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            expanded.push(format!("{indent}}}"));
            expanded.push(format!("{indent}else {{"));
            continue;
        }
        expanded.push(line.clone());
    }

    for line in &expanded {
        let trimmed = line.trim();
        if trimmed.ends_with("{") {
            if let Some((kind, label)) = classify_block_start(trimmed) {
                stack.push((kind, label, line.clone(), Vec::new()));
                continue;
            }
        }

        if trimmed == "}" {
            if let Some((kind, label, header, body)) = stack.pop() {
                let node = Node::Block {
                    kind,
                    label,
                    header,
                    body,
                    footer: line.clone(),
                };
                if let Some(top) = stack.last_mut() {
                    top.3.push(node);
                } else {
                    out.push(node);
                }
                continue;
            }
        }

        if let Some(top) = stack.last_mut() {
            top.3.push(Node::Line(line.clone()));
        } else {
            out.push(Node::Line(line.clone()));
        }
    }

    while let Some((kind, label, header, body)) = stack.pop() {
        out.push(Node::Block {
            kind,
            label,
            header,
            body,
            footer: String::new(),
        });
    }

    out
}

pub fn flatten_nodes(nodes: &[Node]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for node in nodes {
        match node {
            Node::Line(line) => out.push(line.clone()),
            Node::Block {
                header,
                body,
                footer,
                ..
            } => {
                out.push(header.clone());
                out.extend(flatten_nodes(body));
                if !footer.is_empty() {
                    out.push(footer.clone());
                }
            }
        }
    }
    out
}

fn classify_block_start(line: &str) -> Option<(BlockKind, Option<String>)> {
    if let Some(pos) = line.find(": while {") {
        let label = line[..pos].trim().trim_start_matches('\'').to_string();
        return Some((BlockKind::Loop, Some(label)));
    }
    if let Some(pos) = line.find(": {") {
        let prefix = &line[..pos];
        if prefix.trim_start().starts_with('\'') {
            let label = prefix.trim().trim_start_matches('\'').to_string();
            return Some((BlockKind::Other, Some(label)));
        }
    }
    if let Some(pos) = line.find(": loop {") {
        let label = line[..pos].trim().trim_start_matches('\'').to_string();
        return Some((BlockKind::Loop, Some(label)));
    }
    if line.starts_with("loop {") {
        return Some((BlockKind::Loop, None));
    }
    if line.starts_with("while ") && line.ends_with("{") {
        return Some((BlockKind::Loop, None));
    }
    if line.starts_with("if ") && line.ends_with("{") {
        return Some((BlockKind::If, None));
    }
    if line.starts_with("else {") {
        return Some((BlockKind::Else, None));
    }
    if (line.starts_with("match ") || line.starts_with("match(")) && line.ends_with("{") {
        return Some((BlockKind::Other, None));
    }
    Some((BlockKind::Other, None))
}
