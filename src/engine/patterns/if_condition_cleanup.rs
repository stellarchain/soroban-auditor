use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

pub struct IfConditionCleanupPattern;

impl IfConditionCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for IfConditionCleanupPattern {
    fn name(&self) -> &'static str {
        "if_condition_cleanup"
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
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        match node {
            Node::Block {
                kind,
                label,
                mut header,
                body,
                footer,
            } => {
                if let Some(new_header) = rewrite_if_header(&header) {
                    header = new_header;
                    *changed = true;
                }
                let new_body = rewrite(body, changed);
                let has_body_code = new_body.iter().any(|n| match n {
                    Node::Line(l) => !l.trim().is_empty(),
                    Node::Block { .. } => true,
                });
                if (kind == crate::engine::ir::BlockKind::If
                    || kind == crate::engine::ir::BlockKind::Else)
                    && !has_body_code
                {
                    *changed = true;
                    continue;
                }
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

fn rewrite_if_header(header: &str) -> Option<String> {
    let trimmed = header.trim();
    if !trimmed.starts_with("if ") || !trimmed.ends_with(" {") {
        return None;
    }
    let indent: String = header.chars().take_while(|c| c.is_whitespace()).collect();
    let cond = trimmed.strip_prefix("if ")?.strip_suffix(" {")?.trim();
    let new_cond = normalize_if_condition(cond)?;
    Some(format!("{indent}if {new_cond} {{"))
}

fn normalize_if_condition(cond: &str) -> Option<String> {
    if let Some(unwrapped) = unwrap_double_negations(cond) {
        return Some(unwrapped);
    }
    if let Some(inner) = cond.strip_prefix("!!(").and_then(|s| s.strip_suffix(')')) {
        return Some(inner.trim().to_string());
    }
    if let Some(v) = normalize_bool_cast_condition(cond) {
        return Some(v);
    }
    if let Some(inner) = cond.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
        if let Some(inv) = invert_comparison(inner.trim()) {
            return Some(inv);
        }
    }
    None
}

fn unwrap_double_negations(cond: &str) -> Option<String> {
    let mut changed = false;
    let bytes = cond.as_bytes();
    let mut out = String::with_capacity(cond.len());
    let mut i = 0usize;

    while i < bytes.len() {
        if cond[i..].starts_with("!(!(") {
            let inner_start = i + 4;
            let mut depth = 1i32;
            let mut j = inner_start;
            while j < bytes.len() {
                let ch = bytes[j] as char;
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                j += 1;
            }
            if depth == 0 {
                let outer_close = j + 1;
                if outer_close < bytes.len() && bytes[outer_close] as char == ')' {
                    out.push_str(&cond[inner_start..j]);
                    i = outer_close + 1;
                    changed = true;
                    continue;
                }
            }
        }

        out.push(bytes[i] as char);
        i += 1;
    }

    if changed {
        Some(out)
    } else {
        None
    }
}

fn normalize_bool_cast_condition(cond: &str) -> Option<String> {
    if let Some(inner) = cond
        .strip_prefix("!((")
        .and_then(|s| s.strip_suffix(") as i32 != 0)"))
    {
        return Some(negate_expr(inner));
    }

    if let Some(inner) = cond
        .strip_prefix("((")
        .and_then(|s| s.strip_suffix(") as i32 != 0)"))
    {
        return Some(if inner.starts_with("!(") && inner.ends_with(')') {
            let x = &inner[2..inner.len().saturating_sub(1)];
            format!("!({x})")
        } else {
            inner.to_string()
        });
    }

    None
}

fn negate_expr(expr: &str) -> String {
    let e = expr.trim();
    if e.starts_with("!(") && e.ends_with(')') {
        return e[2..e.len().saturating_sub(1)].to_string();
    }
    if let Some(inv) = invert_comparison(e) {
        return inv;
    }
    format!("!({e})")
}

fn invert_comparison(expr: &str) -> Option<String> {
    if expr.contains("&&") || expr.contains("||") {
        return None;
    }
    for (op, inv) in [
        (" != ", " == "),
        (" == ", " != "),
        (" >= ", " < "),
        (" <= ", " > "),
        (" > ", " <= "),
        (" < ", " >= "),
    ] {
        if let Some(pos) = expr.find(op) {
            let left = expr[..pos].trim();
            let right = expr[pos + op.len()..].trim();
            if left.is_empty() || right.is_empty() {
                return None;
            }
            return Some(format!("{left}{inv}{right}"));
        }
    }
    None
}
