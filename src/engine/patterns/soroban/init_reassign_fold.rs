use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct InitReassignFoldPattern;

impl InitReassignFoldPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InitReassignFoldPattern {
    fn name(&self) -> &'static str {
        "init_reassign_fold"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i < body.len() {
            let Some((indent, var, lhs_decl)) = parse_mut_decl_head(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(j) = next_non_empty_non_comment(&body, i + 1) else {
                i += 1;
                continue;
            };
            let Some((assign_lhs, assign_rhs)) = parse_simple_assignment(&body[j]) else {
                i += 1;
                continue;
            };
            if assign_lhs != var {
                i += 1;
                continue;
            }
            if count_ident_in_line(&assign_rhs, &var) > 0 {
                i += 1;
                continue;
            }

            body[i] = format!("{indent}let mut {lhs_decl} = {assign_rhs};");
            body.remove(j);
            changed = true;
            continue;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn parse_mut_decl_head(line: &str) -> Option<(String, String, String)> {
    let indent = line
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let t = line.trim();
    if !t.starts_with("let mut ") || !t.ends_with(';') || !t.contains(" = ") {
        return None;
    }
    let rest = t.strip_prefix("let mut ")?;
    let (lhs_decl, _) = rest.split_once(" = ")?;
    let var = lhs_decl.split(':').next()?.trim();
    if !is_ident(var) {
        return None;
    }
    Some((indent, var.to_string(), lhs_decl.trim().to_string()))
}

fn parse_simple_assignment(line: &str) -> Option<(String, String)> {
    let t = line.trim();
    if !t.ends_with(';') || !t.contains(" = ") || t.starts_with("let ") {
        return None;
    }
    let (lhs, rhs) = t.trim_end_matches(';').split_once(" = ")?;
    let lhs = lhs.trim();
    if !is_ident(lhs) {
        return None;
    }
    let rhs = rhs.trim();
    if rhs.is_empty() {
        return None;
    }
    Some((lhs.to_string(), rhs.to_string()))
}

fn next_non_empty_non_comment(lines: &[String], mut idx: usize) -> Option<usize> {
    while idx < lines.len() {
        let t = lines[idx].trim();
        if !t.is_empty() && !t.starts_with("//") {
            return Some(idx);
        }
        idx += 1;
    }
    None
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
