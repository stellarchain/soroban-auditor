use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct RemoveUnusedLocalMutPattern;

impl RemoveUnusedLocalMutPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveUnusedLocalMutPattern {
    fn name(&self) -> &'static str {
        "remove_unused_local_mut"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = block.body.clone();

        for i in 0..out.len() {
            let line = out[i].clone();
            let trimmed = line.trim_start();
            if !trimmed.starts_with("let mut ") {
                continue;
            }
            let Some(name) = extract_decl_name(trimmed) else {
                continue;
            };
            let rest = &out[i + 1..];
            if !is_mutated(name, rest) {
                out[i] = line.replacen("let mut ", "let ", 1);
                changed = true;
            }
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: out,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn extract_decl_name(trimmed: &str) -> Option<&str> {
    let after = trimmed.strip_prefix("let mut ")?;
    let end = after.find(':').or_else(|| after.find('='))?;
    let name = after[..end].trim();
    if name.is_empty() {
        return None;
    }
    Some(name)
}

fn is_mutated(name: &str, body: &[String]) -> bool {
    body.iter().any(|line| {
        has_assignment_to_name(line, name)
            || line.contains(&format!("&mut {name}"))
            || has_mutating_method_call(line, name)
    })
}

fn has_assignment_to_name(line: &str, name: &str) -> bool {
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(name) {
        let start = idx + pos;
        let end = start + name.len();
        let bytes = line.as_bytes();

        let left_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
        let right_ok = end >= bytes.len() || !is_ident_char(bytes[end] as char);
        if !(left_ok && right_ok) {
            idx = end;
            continue;
        }

        let mut j = end;
        while j < bytes.len() && (bytes[j] as char).is_whitespace() {
            j += 1;
        }

        let ops = ["+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "="];
        for op in ops {
            if line[j..].starts_with(op) {
                if op == "=" && line[j..].starts_with("==") {
                    continue;
                }
                return true;
            }
        }
        idx = end;
    }
    false
}

fn has_mutating_method_call(line: &str, name: &str) -> bool {
    let needle = format!("{name}.");
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(&needle) {
        let start = idx + pos;
        if start > 0 {
            let prev = line.as_bytes()[start - 1] as char;
            if is_ident_char(prev) {
                idx = start + needle.len();
                continue;
            }
        }
        let method_start = start + needle.len();
        let rest = &line[method_start..];
        let mut method = String::new();
        for ch in rest.chars() {
            if is_ident_char(ch) {
                method.push(ch);
            } else {
                break;
            }
        }
        if !method.is_empty() {
            let after = &rest[method.len()..];
            if after.trim_start().starts_with('(') && is_mutating_method_name(&method) {
                return true;
            }
        }
        idx = method_start;
    }
    false
}

fn is_mutating_method_name(method: &str) -> bool {
    matches!(
        method,
        "push_back"
            | "push_front"
            | "pop_back"
            | "pop_front"
            | "append"
            | "insert"
            | "remove"
            | "remove_unchecked"
            | "set"
            | "set_unchecked"
            | "clear"
            | "retain"
            | "truncate"
            | "shuffle"
            | "sort"
            | "sort_unstable"
    )
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
