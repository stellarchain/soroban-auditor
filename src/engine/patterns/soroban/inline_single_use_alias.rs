use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct InlineSingleUseAliasPattern;

impl InlineSingleUseAliasPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineSingleUseAliasPattern {
    fn name(&self) -> &'static str {
        "inline_single_use_alias"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i + 1 < body.len() {
            let t = body[i].trim().to_string();
            let Some((dst, src)) = parse_ident_assign(&t) else {
                i += 1;
                continue;
            };
            let Some(use_idx) = next_use_of_ident(&body, i + 1, &dst) else {
                i += 1;
                continue;
            };
            if has_reassign_between(&body, i + 1, use_idx, &dst) {
                i += 1;
                continue;
            }
            let refs = count_ident_refs(&body, &dst);
            if refs < 2 || refs > 3 {
                i += 1;
                continue;
            }
            if refs == 3 && !has_decl_before(&body, i, &dst) {
                i += 1;
                continue;
            }

            body[use_idx] = replace_ident(&body[use_idx], &dst, &src);
            body.remove(i);
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

fn parse_ident_assign(t: &str) -> Option<(String, String)> {
    if !t.ends_with(';') {
        return None;
    }
    let (lhs, rhs) = t.trim_end_matches(';').split_once(" = ")?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();
    if !is_ident(lhs) || !is_ident(rhs) {
        return None;
    }
    Some((lhs.to_string(), rhs.to_string()))
}

fn next_use_of_ident(lines: &[String], mut idx: usize, ident: &str) -> Option<usize> {
    while idx < lines.len() {
        if count_ident_in_line(&lines[idx], ident) > 0 {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn has_reassign_between(lines: &[String], from: usize, to: usize, ident: &str) -> bool {
    for line in lines.iter().take(to).skip(from) {
        let t = line.trim();
        if let Some((lhs, _)) = t.trim_end_matches(';').split_once(" = ") {
            if lhs.trim() == ident {
                return true;
            }
        }
    }
    false
}

fn count_ident_refs(lines: &[String], ident: &str) -> usize {
    lines.iter().map(|l| count_ident_in_line(l, ident)).sum()
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

fn replace_ident(line: &str, ident: &str, replacement: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let left_ok = start == 0 || !is_ident_char(line.as_bytes()[start - 1] as char);
        let right_ok = end >= line.len() || !is_ident_char(line.as_bytes()[end] as char);
        if left_ok && right_ok {
            out.push_str(&line[idx..start]);
            out.push_str(replacement);
            idx = end;
        } else {
            out.push_str(&line[idx..end]);
            idx = end;
        }
    }
    out.push_str(&line[idx..]);
    out
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

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn has_decl_before(lines: &[String], before: usize, ident: &str) -> bool {
    for line in lines.iter().take(before) {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("let mut ").or_else(|| t.strip_prefix("let ")) {
            let name = rest.split(':').next().unwrap_or_default().split('=').next().unwrap_or_default().trim();
            if name == ident {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inlines_alias_with_decl_assign_use() {
        let p = InlineSingleUseAliasPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut c: i64 = 0;".to_string(),
                "    c = to_muxed;".to_string(),
                "    self.credit_balance(env, c, b, amount);".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("should apply");
        assert!(out.body.iter().any(|l| l.contains("credit_balance(env, to_muxed")));
        assert!(!out.body.iter().any(|l| l.trim() == "c = to_muxed;"));
    }
}
