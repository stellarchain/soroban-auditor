use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct ForEachValPattern;

impl ForEachValPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ForEachValPattern {
    fn name(&self) -> &'static str {
        "for_each_val"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_for_each_val(nodes, &mut changed);
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

fn rewrite_for_each_val(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    let mut i = 0;
    while i < nodes.len() {
        if let Node::Block { kind, body, .. } = &nodes[i] {
            if *kind == BlockKind::Loop {
                if let Some(replacement) = try_rewrite_loop(body) {
                    // Optionally drop a simple "let len = vec.len()" line immediately before.
                    if let Some(prev) = out.last() {
                        if let Node::Line(prev_line) = prev {
                            if looks_like_len_line(prev_line) {
                                out.pop();
                            }
                        }
                    }
                    out.extend(replacement.into_iter().map(Node::Line));
                    i += 1;
                    *changed = true;
                    continue;
                }
            }
        }
        match nodes[i].clone() {
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite_for_each_val(body, changed);
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

fn try_rewrite_loop(body: &[Node]) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    for node in body {
        match node {
            Node::Line(l) => lines.push(l.clone()),
            _ => return None,
        }
    }
    let body_text = lines.join("\n");
    if !body_text.contains("get_unchecked(") || !body_text.contains(".len()") {
        return None;
    }
    let mut vec_var: Option<String> = None;
    let mut item_var: Option<String> = None;
    for l in &lines {
        if vec_var.is_none() && l.contains(".get_unchecked(") {
            vec_var = extract_receiver_ident(l, ".get_unchecked(");
        }
        if item_var.is_none() && l.contains(".get_unchecked(") {
            item_var = extract_let_ident(l);
        }
    }
    let vec_var = vec_var?;
    let item_var = item_var.unwrap_or_else(|| "val".to_string());
    let indent = lines
        .first()
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).collect::<String>())
        .unwrap_or_default();

    let mut replacement: Vec<String> = Vec::new();
    replacement.push(format!(
        "{}self.for_each_val(env, &{}, |{}| {{",
        indent, vec_var, item_var
    ));
    for l in &lines {
        if l.contains("get_unchecked(") || l.contains("len()") || l.contains("wrapping_add(1)") {
            continue;
        }
        if l.trim().starts_with("while ") || l.trim().starts_with("loop ") {
            continue;
        }
        if l.trim() == "}" {
            continue;
        }
        replacement.push(l.clone());
    }
    replacement.push(format!("{}}});", indent));
    Some(replacement)
}

fn extract_let_ident(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("let ") {
        return None;
    }
    let rest = &trimmed["let ".len()..];
    let rest = rest.strip_prefix("mut ").unwrap_or(rest);
    let mut ident = String::new();
    for ch in rest.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            ident.push(ch);
        } else {
            break;
        }
    }
    if ident.is_empty() { None } else { Some(ident) }
}

fn extract_receiver_ident(line: &str, suffix: &str) -> Option<String> {
    let idx = line.find(suffix)?;
    let before = &line[..idx];
    let mut ident = String::new();
    for ch in before.chars().rev() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            ident.insert(0, ch);
        } else {
            break;
        }
    }
    if ident.is_empty() { None } else { Some(ident) }
}

fn looks_like_len_line(line: &str) -> bool {
    let t = line.trim();
    t.starts_with("let ") && t.contains(".len()")
}
