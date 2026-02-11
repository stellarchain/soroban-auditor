use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct InlineStatusMloadGuardPattern;

impl InlineStatusMloadGuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineStatusMloadGuardPattern {
    fn name(&self) -> &'static str {
        "inline_status_mload_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 4 {
            return None;
        }

        let re_load =
            Regex::new(r"^(?P<indent>\s*)let(?: mut)? (?P<var>\w+)(?::\s*i32)? = mload32!\((?P<addr>.+)\);$")
                .ok()?;
        let re_if = Regex::new(r"^(?P<indent>\s*)if (?P<var>\w+) != 0 \{$").ok()?;
        let re_unreachable = Regex::new(r"^\s*unreachable!\(\);\s*$").ok()?;
        let re_close = Regex::new(r"^\s*}\s*$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 3 < body.len() {
            let Some(load_cap) = re_load.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let var = load_cap.name("var").map(|m| m.as_str()).unwrap_or("");
            if !looks_like_generated_temp(var) {
                i += 1;
                continue;
            }
            let addr = load_cap.name("addr").map(|m| m.as_str()).unwrap_or("");
            let indent = load_cap.name("indent").map(|m| m.as_str()).unwrap_or("");

            let Some(if_cap) = re_if.captures(&body[i + 1]) else {
                i += 1;
                continue;
            };
            if if_cap.name("var").map(|m| m.as_str()) != Some(var) {
                i += 1;
                continue;
            }
            if !re_unreachable.is_match(&body[i + 2]) || !re_close.is_match(&body[i + 3]) {
                i += 1;
                continue;
            }
            if contains_ident_after(&body, i + 4, var) {
                i += 1;
                continue;
            }

            body[i] = format!("{indent}if mload32!({addr}) != 0 {{");
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

fn looks_like_generated_temp(name: &str) -> bool {
    name.starts_with("sv") || name.starts_with("slot_var") || name.starts_with("var")
}

fn contains_ident_after(lines: &[String], start: usize, ident: &str) -> bool {
    lines
        .iter()
        .skip(start)
        .any(|line| contains_ident(line, ident))
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
    fn inlines_status_guard_load() {
        let p = InlineStatusMloadGuardPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut sv2_0_i32 = mload32!(amount_val as usize);".to_string(),
                "    if sv2_0_i32 != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    amount = mload64!(amount_val.wrapping_add(16) as usize);".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "if mload32!(amount_val as usize) != 0 {");
        assert_eq!(out.body[1].trim(), "unreachable!();");
        assert_eq!(out.body[2].trim(), "}");
    }
}

