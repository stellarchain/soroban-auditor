use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct UnusedLetSideEffectPreservePattern;

impl UnusedLetSideEffectPreservePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for UnusedLetSideEffectPreservePattern {
    fn name(&self) -> &'static str {
        "unused_let_side_effect_preserve"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let t = line.trim();
            let Some((indent, name, rhs)) = parse_let_binding_line(line) else {
                out.push(line.clone());
                continue;
            };
            if has_ident_refs_after(&block.body, &name, line) && !rhs.contains(".set(") {
                out.push(line.clone());
                continue;
            }
            if !rhs.contains('(') {
                out.push(line.clone());
                continue;
            }
            if t.contains(" = match ") {
                out.push(line.clone());
                continue;
            }
            changed = true;
            out.push(format!("{indent}{rhs};"));
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

fn parse_let_binding_line(line: &str) -> Option<(String, String, String)> {
    let indent = line
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let t = line.trim();
    if !t.starts_with("let ") || !t.ends_with(';') || !t.contains(" = ") {
        return None;
    }
    let rest = t.strip_prefix("let ")?;
    let (lhs, rhs_with_semi) = rest.split_once(" = ")?;
    let name_part = lhs.strip_prefix("mut ").unwrap_or(lhs);
    let name = name_part.split(':').next()?.trim();
    if !is_ident(name) {
        return None;
    }
    let rhs = rhs_with_semi.trim_end_matches(';').trim().to_string();
    if rhs.is_empty() {
        return None;
    }
    Some((indent, name.to_string(), rhs))
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn has_ident_refs_after(lines: &[String], ident: &str, current_line: &str) -> bool {
    let mut passed_current = false;
    for line in lines {
        if !passed_current {
            if line == current_line {
                passed_current = true;
            }
            continue;
        }
        if count_ident_in_line(line, ident) > 0 {
            return true;
        }
    }
    false
}

fn count_ident_in_line(line: &str, ident: &str) -> usize {
    let mut count = 0usize;
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let left_ok = start == 0 || !is_ident_char(line.as_bytes()[start - 1] as char);
        let right_ok = end >= line.len() || !is_ident_char(line.as_bytes()[end] as char);
        if left_ok && right_ok {
            count += 1;
        }
        idx = end;
        if idx >= line.len() {
            break;
        }
    }
    count
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
