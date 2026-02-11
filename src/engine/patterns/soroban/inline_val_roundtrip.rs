use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct InlineValRoundtripPattern;

impl InlineValRoundtripPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineValRoundtripPattern {
    fn name(&self) -> &'static str {
        "inline_val_roundtrip"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut defs: HashMap<String, String> = HashMap::new();
        for line in &block.body {
            if let Some((name, expr)) = parse_roundtrip_decl(line.trim()) {
                defs.insert(name.to_string(), expr.to_string());
            }
        }
        if defs.is_empty() {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        for line in &mut body {
            let mut updated = line.clone();
            for (name, expr) in &defs {
                let needle = format!("val_from_i64({name})");
                if updated.contains(&needle) {
                    updated = updated.replace(&needle, expr);
                }
            }
            updated = simplify_redundant_from_val(updated);
            if *line != updated {
                *line = updated;
                changed = true;
            }
        }

        if !changed {
            return None;
        }

        // Drop declarations that became dead.
        let mut counts: HashMap<String, usize> = HashMap::new();
        for line in &body {
            for tok in tokens(line) {
                *counts.entry(tok).or_insert(0) += 1;
            }
        }
        body.retain(|line| {
            let Some((name, _expr)) = parse_roundtrip_decl(line.trim()) else {
                return true;
            };
            counts.get(name).copied().unwrap_or(0) > 1
        });

        Some(FunctionBlock {
            header: block.header.clone(),
            body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn parse_roundtrip_decl(line: &str) -> Option<(&str, &str)> {
    let rest = if let Some(v) = line.strip_prefix("let mut ") {
        v
    } else if let Some(v) = line.strip_prefix("let ") {
        v
    } else {
        return None;
    };
    let (lhs, rhs) = rest.split_once(" = ")?;
    let name = lhs.split(':').next()?.trim();
    let inner = rhs.strip_prefix("val_to_i64(")?.strip_suffix(");")?;
    let allowed = inner.starts_with("String::from_str(")
        || inner.starts_with("Symbol::from_str(")
        || inner.starts_with("Symbol::new(");
    if !allowed {
        return None;
    }
    Some((name, inner))
}

fn tokens(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in line.chars() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            cur.push(ch);
        } else if !cur.is_empty() {
            out.push(cur.clone());
            cur.clear();
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn simplify_redundant_from_val(mut line: String) -> String {
    line = replace_wrapper(line, "String::from_val(env, &", ")");
    line = replace_wrapper(line, "Symbol::from_val(env, &", ")");
    line
}

fn replace_wrapper(mut line: String, prefix: &str, suffix: &str) -> String {
    loop {
        let Some(start) = line.find(prefix) else {
            break;
        };
        let inner_start = start + prefix.len();
        let Some(end_rel) = line[inner_start..].find(suffix) else {
            break;
        };
        let end = inner_start + end_rel;
        let inner = &line[inner_start..end];
        if !(inner.starts_with("String::from_str(")
            || inner.starts_with("String::from_val(")
            || inner.starts_with("Symbol::from_str(")
            || inner.starts_with("Symbol::new(")
            || inner.starts_with("Symbol::from_val("))
        {
            break;
        }
        let before = &line[..start];
        let after = &line[end + suffix.len()..];
        line = format!("{before}{inner}{after}");
    }
    line
}
