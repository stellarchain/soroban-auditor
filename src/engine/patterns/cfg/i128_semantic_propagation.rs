use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct I128SemanticPropagationPattern;

impl I128SemanticPropagationPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for I128SemanticPropagationPattern {
    fn name(&self) -> &'static str {
        "i128_semantic_propagation"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let re_load_decl =
            Regex::new(r"^(?P<indent>\s*)let mut (?P<tmp>\w+) = (?P<rhs>mload64!\(.+\) as i64);$")
                .ok()?;
        let re_assign = Regex::new(r"^\s*(?P<dst>\w+) = (?P<src>\w+);$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;

        let mut i = 0usize;
        while i + 1 < body.len() {
            let Some(load_cap) = re_load_decl.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(tmp) = load_cap.name("tmp").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            if !looks_like_decode_temp(tmp) {
                i += 1;
                continue;
            }
            let Some(rhs) = load_cap.name("rhs").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            let indent = load_cap.name("indent").map(|m| m.as_str()).unwrap_or("");

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

fn looks_like_decode_temp(name: &str) -> bool {
    name.ends_with("_lo")
        || name.ends_with("_hi")
        || name.starts_with("sv")
        || name.starts_with("slot_var")
        || name.starts_with("var")
}

fn has_use_before_redeclaration(lines: &[String], start: usize, ident: &str) -> bool {
    for line in lines.iter().skip(start) {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("let mut {ident} =")) || trimmed.starts_with(&format!("let {ident} =")) {
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
    fn collapses_temp_load_and_assignment_pair() {
        let p = I128SemanticPropagationPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut amount_lo = mload64!(amount_val as usize + 16) as i64;".to_string(),
                "    amount = amount_lo;".to_string(),
                "    let mut amount_hi = mload64!(amount_val as usize + 24) as i64;".to_string(),
                "    a = amount_hi;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    amount = mload64!(amount_val as usize + 16) as i64;".to_string(),
                "    a = mload64!(amount_val as usize + 24) as i64;".to_string(),
            ]
        );
    }
}
