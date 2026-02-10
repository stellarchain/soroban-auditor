use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct DeadTempCleanupPattern;

impl DeadTempCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for DeadTempCleanupPattern {
    fn name(&self) -> &'static str {
        "dead_temp_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed_any = false;

        loop {
            let mut changed = false;

            // Remove dead zero-init locals.
            let mut i = 0usize;
            while i < body.len() {
                let t = body[i].trim();
                if let Some(name) = parse_zero_init_decl_name(t) {
                    if count_identifier_refs(&body, name) <= 1 {
                        body.remove(i);
                        changed = true;
                        continue;
                    }
                }
                i += 1;
            }

            // Remove dead simple assignments.
            let mut j = 0usize;
            while j < body.len() {
                let t = body[j].trim();
                if let Some(name) = parse_simple_assign_lhs(t) {
                    let refs = count_identifier_refs(&body, name);
                    if refs <= 1 || (refs == 2 && has_only_zero_decl_and_assign(&body, name)) {
                        body.remove(j);
                        changed = true;
                        continue;
                    }
                    if is_trivial_assignment_rhs(t)
                        && !is_identifier_used_after(&body, j + 1, name)
                    {
                        body.remove(j);
                        changed = true;
                        continue;
                    }
                }
                j += 1;
            }

            if !changed {
                break;
            }
            changed_any = true;
        }

        if !changed_any {
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

fn parse_zero_init_decl_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("let mut ")?;
    if !rest.ends_with("= 0;")
        && !rest.ends_with("= 0 as i32;")
        && !rest.ends_with("= 0 as i64;")
        && !rest.ends_with("= 0 as u32;")
        && !rest.ends_with("= 0 as u64;")
    {
        return None;
    }
    let name = rest.split(':').next()?.trim();
    if is_simple_ident(name) {
        Some(name)
    } else {
        None
    }
}

fn parse_simple_assign_lhs(line: &str) -> Option<&str> {
    let (lhs, rhs) = line.split_once(" = ")?;
    if !rhs.ends_with(';') {
        return None;
    }
    let lhs = lhs.trim();
    if is_simple_ident(lhs) {
        Some(lhs)
    } else {
        None
    }
}

fn count_identifier_refs(lines: &[String], ident: &str) -> usize {
    lines.iter().map(|l| count_ident_in_line(l, ident)).sum()
}

fn count_ident_in_line(line: &str, ident: &str) -> usize {
    let bytes = line.as_bytes();
    let mut count = 0usize;
    let mut i = 0usize;
    while i + ident.len() <= bytes.len() {
        if &line[i..i + ident.len()] == ident {
            let left_ok = i == 0
                || !is_ident_char(bytes[i.saturating_sub(1)] as char);
            let right_idx = i + ident.len();
            let right_ok = right_idx >= bytes.len()
                || !is_ident_char(bytes[right_idx] as char);
            if left_ok && right_ok {
                count += 1;
            }
            i += ident.len();
            continue;
        }
        i += 1;
    }
    count
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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

fn has_only_zero_decl_and_assign(lines: &[String], ident: &str) -> bool {
    let mut has_decl = false;
    let mut has_assign = false;
    for line in lines {
        let t = line.trim();
        if count_ident_in_line(t, ident) == 0 {
            continue;
        }
        if parse_zero_init_decl_name(t) == Some(ident) {
            has_decl = true;
            continue;
        }
        if parse_simple_assign_lhs(t) == Some(ident) {
            has_assign = true;
            continue;
        }
        return false;
    }
    has_decl && has_assign
}

fn is_identifier_used_after(lines: &[String], start: usize, ident: &str) -> bool {
    lines[start..]
        .iter()
        .any(|line| count_ident_in_line(line, ident) > 0)
}

fn is_trivial_assignment_rhs(line: &str) -> bool {
    let Some((_, rhs)) = line.split_once(" = ") else {
        return false;
    };
    let rhs = rhs.trim().trim_end_matches(';').trim();
    if rhs.is_empty() {
        return false;
    }
    if rhs.contains('(') || rhs.contains(')') {
        return false;
    }
    if rhs.contains('[') || rhs.contains(']') || rhs.contains('{') || rhs.contains('}') {
        return false;
    }
    if rhs.contains('.') || rhs.contains("::") || rhs.contains("->") {
        return false;
    }
    if rhs.contains(" + ")
        || rhs.contains(" - ")
        || rhs.contains(" * ")
        || rhs.contains(" / ")
        || rhs.contains(" % ")
        || rhs.contains("<<")
        || rhs.contains(">>")
        || rhs.contains('&')
        || rhs.contains('|')
        || rhs.contains('^')
    {
        return false;
    }
    true
}
