use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct ExtractIfMload32GuardPattern;

impl ExtractIfMload32GuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExtractIfMload32GuardPattern {
    fn name(&self) -> &'static str {
        "extract_if_mload32_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        // Intentionally targets only wrapping_add(offset) form to avoid noisy rewrites
        // for simple status guards like mload32!(amount_val as usize).
        let re_if = Regex::new(
            r"^(?P<indent>\s*)if mload32!\((?P<base>\w+)\.wrapping_add\((?P<off>\d+)\) as usize\)\s*(?P<op>==|!=)\s*(?P<rhs>-?\d+)\s*\{$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].clone();
            let Some(cap) = re_if.captures(&line) else {
                i += 1;
                continue;
            };

            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("").to_string();
            let base = cap.name("base").map(|m| m.as_str()).unwrap_or("").to_string();
            let off = cap.name("off").map(|m| m.as_str()).unwrap_or("").to_string();
            let op = cap.name("op").map(|m| m.as_str()).unwrap_or("").to_string();
            let rhs = cap.name("rhs").map(|m| m.as_str()).unwrap_or("").to_string();
            if base.is_empty() || off.is_empty() || op.is_empty() || rhs.is_empty() {
                i += 1;
                continue;
            }

            let used = collect_idents(&body);
            let tmp = unique_name(&format!("{base}_i32_{off}"), &used);
            body[i] = format!("{indent}if {tmp} {op} {rhs} {{");
            body.insert(
                i,
                format!(
                    "{indent}let {tmp} = mload32!({base}.wrapping_add({off}) as usize);"
                ),
            );
            changed = true;
            i += 2;
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

fn collect_idents(lines: &[String]) -> HashSet<String> {
    let mut out = HashSet::new();
    for line in lines {
        let mut cur = String::new();
        for ch in line.chars() {
            if is_ident_char(ch) {
                cur.push(ch);
            } else if !cur.is_empty() {
                out.insert(cur.clone());
                cur.clear();
            }
        }
        if !cur.is_empty() {
            out.insert(cur);
        }
    }
    out
}

fn unique_name(base: &str, used: &HashSet<String>) -> String {
    if !used.contains(base) {
        return base.to_string();
    }
    let mut idx = 2usize;
    loop {
        let c = format!("{base}_{idx}");
        if !used.contains(&c) {
            return c;
        }
        idx += 1;
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_mload32_from_if_guard_with_offset() {
        let p = ExtractIfMload32GuardPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if mload32!(a.wrapping_add(20) as usize) == 0 {".to_string(),
                "        x = 1;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "let a_i32_20 = mload32!(a.wrapping_add(20) as usize);"
        );
        assert_eq!(out.body[1].trim(), "if a_i32_20 == 0 {");
    }
}

