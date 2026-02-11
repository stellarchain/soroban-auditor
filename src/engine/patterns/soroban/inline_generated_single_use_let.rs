use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct InlineGeneratedSingleUseLetPattern;

impl InlineGeneratedSingleUseLetPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineGeneratedSingleUseLetPattern {
    fn name(&self) -> &'static str {
        "inline_generated_single_use_let"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let re_decl = Regex::new(
            r"^(?P<indent>\s*)let(?: mut)? (?P<var>\w+)(?::\s*[^=]+)? = (?P<rhs>.+);$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let Some(cap) = re_decl.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let var = cap.name("var").map(|m| m.as_str()).unwrap_or("");
            if !looks_like_generated_temp(var) {
                i += 1;
                continue;
            }
            let rhs = cap.name("rhs").map(|m| m.as_str()).unwrap_or("").trim();
            if rhs.is_empty() || rhs == "0" {
                i += 1;
                continue;
            }
            if !looks_like_inline_safe_expr(rhs) {
                i += 1;
                continue;
            }

            let refs_after = count_ident_refs_in_range(&body, var, i + 1, body.len());
            if refs_after != 1 {
                i += 1;
                continue;
            }
            let Some(use_idx) = next_use_of_ident(&body, i + 1, var) else {
                i += 1;
                continue;
            };
            if has_reassign_between(&body, i + 1, use_idx, var) {
                i += 1;
                continue;
            }

            body[use_idx] = replace_ident(&body[use_idx], var, rhs);
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

fn looks_like_generated_temp(name: &str) -> bool {
    name.starts_with("sv") || name.starts_with("slot_var")
}

fn looks_like_inline_safe_expr(rhs: &str) -> bool {
    rhs.starts_with("mload32!(")
        || rhs.starts_with("mload64!(")
        || rhs.starts_with("val_from_i64(")
        || rhs.starts_with("val_to_i64(")
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

fn count_ident_refs_in_range(lines: &[String], ident: &str, from: usize, to: usize) -> usize {
    lines
        .iter()
        .take(to)
        .skip(from)
        .map(|l| count_ident_in_line(l, ident))
        .sum()
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

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inlines_single_use_generated_mload_let() {
        let p = InlineGeneratedSingleUseLetPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut sv4_8_i64 = mload64!(amount_val.wrapping_add(8) as usize);".to_string(),
                "    env.events().publish(val_from_i64(from), val_from_i64(sv4_8_i64));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "env.events().publish(val_from_i64(from), val_from_i64(mload64!(amount_val.wrapping_add(8) as usize)));"
        );
    }
}

