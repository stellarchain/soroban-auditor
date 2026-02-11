use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

pub struct SymbolLiteralRecoveryPattern;

impl SymbolLiteralRecoveryPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SymbolLiteralRecoveryPattern {
    fn name(&self) -> &'static str {
        "symbol_literal_recovery"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let known_idents = collect_known_identifiers(&block.header, &block.body);
        let mut changed = false;
        let mut new_body = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let rewritten = rewrite_symbol_from_val_line(line, &known_idents, &mut changed);
            new_body.push(rewritten);
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite_symbol_from_val_line(
    line: &str,
    known_idents: &HashSet<String>,
    changed: &mut bool,
) -> String {
    let needle = "Symbol::from_val(env, &val_from_i64(";
    let mut out = line.to_string();
    let mut search_from = 0usize;

    loop {
        let Some(rel_pos) = out[search_from..].find(needle) else {
            break;
        };
        let start = search_from + rel_pos;
        let expr_start = start + needle.len();
        let Some(end_rel) = out[expr_start..].find("))") else {
            break;
        };
        let expr_end = expr_start + end_rel;
        let expr = out[expr_start..expr_end].trim();

        if is_recoverable_symbol_literal(expr, known_idents) {
            let replacement = format!("Symbol::new(env, \"{}\")", expr);
            out.replace_range(start..expr_end + 2, &replacement);
            *changed = true;
            search_from = start + replacement.len();
            continue;
        }

        search_from = expr_end + 2;
    }

    out
}

fn is_recoverable_symbol_literal(expr: &str, known_idents: &HashSet<String>) -> bool {
    if !is_simple_ident(expr) {
        return false;
    }
    let Some(first) = expr.chars().next() else {
        return false;
    };
    // Keep it conservative: recover only unresolved identifier-style symbol literals.
    if !first.is_ascii_uppercase() {
        return false;
    }
    !known_idents.contains(expr)
}

fn collect_known_identifiers(header: &str, body: &[String]) -> HashSet<String> {
    let mut out = HashSet::new();

    for line in header.lines() {
        let trimmed = line.trim().trim_end_matches(',');
        if let Some((lhs, _)) = trimmed.split_once(':') {
            let name = lhs.trim().trim_start_matches("mut ").trim();
            if is_simple_ident(name) {
                out.insert(name.to_string());
            }
        } else if trimmed == "&mut self" || trimmed == "&self" {
            out.insert("self".to_string());
        }
    }

    for line in body {
        let t = line.trim();
        if let Some(name) = parse_let_name(t) {
            out.insert(name.to_string());
        }
        if let Some((lhs, _)) = t.split_once(" = ") {
            let lhs = lhs.trim();
            if is_simple_ident(lhs) {
                out.insert(lhs.to_string());
            }
        }
    }

    out
}

fn parse_let_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("let mut ").or_else(|| line.strip_prefix("let "))?;
    let name = rest
        .split_once(':')
        .map(|(n, _)| n)
        .or_else(|| rest.split_once(" = ").map(|(n, _)| n))?
        .trim();
    if is_simple_ident(name) {
        Some(name)
    } else {
        None
    }
}

fn is_simple_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
