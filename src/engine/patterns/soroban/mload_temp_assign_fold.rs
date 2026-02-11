use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct MloadTempAssignFoldPattern;

impl MloadTempAssignFoldPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for MloadTempAssignFoldPattern {
    fn name(&self) -> &'static str {
        "mload_temp_assign_fold"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let re_decl = Regex::new(
            r"^(?P<indent>\s*)let(?: mut)? (?P<tmp>\w+)(?:\s*:\s*[^=]+)? = (?P<rhs>mload(?:32|64)!\(.+\) as i(?:32|64));$",
        )
        .ok()?;
        let re_assign = Regex::new(r"^\s*(?P<dst>\w+)\s*=\s*(?P<src>\w+);$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i + 1 < body.len() {
            let Some(cap) = re_decl.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(tmp) = cap.name("tmp").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            let Some(rhs) = cap.name("rhs").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("");

            let Some(assign_cap) = re_assign.captures(&body[i + 1]) else {
                i += 1;
                continue;
            };
            let Some(src) = assign_cap.name("src").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            if src != tmp {
                i += 1;
                continue;
            }
            let Some(dst) = assign_cap.name("dst").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };

            if has_use_before_redeclaration(&body, i + 2, tmp) {
                i += 1;
                continue;
            }

            body[i] = format!("{indent}{dst} = {rhs};");
            body.remove(i + 1);
            changed = true;
            i += 1;
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

fn has_use_before_redeclaration(lines: &[String], start: usize, ident: &str) -> bool {
    for line in lines.iter().skip(start) {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("let mut {ident} ="))
            || trimmed.starts_with(&format!("let {ident} ="))
        {
            return false;
        }
        if contains_ident(line, ident) {
            return true;
        }
    }
    false
}

fn contains_ident(line: &str, ident: &str) -> bool {
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let bytes = line.as_bytes();
        let left_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
        let right_ok = end >= bytes.len() || !is_ident_char(bytes[end] as char);
        if left_ok && right_ok {
            return true;
        }
        idx = end;
    }
    false
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_mload_temp_assignment() {
        let p = MloadTempAssignFoldPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let o = mload64!(a.wrapping_add(96) as usize) as i64;".to_string(),
                "    e = o;".to_string(),
                "    let mut t: i64 = mload64!(a as usize + 88) as i64;".to_string(),
                "    f = t;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    e = mload64!(a.wrapping_add(96) as usize) as i64;".to_string(),
                "    f = mload64!(a as usize + 88) as i64;".to_string(),
            ]
        );
    }
}

