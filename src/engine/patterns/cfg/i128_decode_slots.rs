use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct I128DecodeSlotsPattern;

impl I128DecodeSlotsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for I128DecodeSlotsPattern {
    fn name(&self) -> &'static str {
        "i128_decode_slots"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re_plain = Regex::new(
            r"^\s*let mut (?P<var>\w+) = mload64!\((?P<base>\w+) as usize \+ (?P<off>16|24)\) as i64;$",
        )
        .ok()?;
        let re_wrap = Regex::new(
            r"^\s*let mut (?P<var>\w+) = mload64!\((?P<base>\w+)\.wrapping_add\((?P<off>16|24)\) as usize\) as i64;$",
        )
        .ok()?;

        let mut used_idents = collect_idents(&block.body);
        let mut renames: HashMap<String, String> = HashMap::new();

        for line in &block.body {
            let caps = re_plain
                .captures(line)
                .or_else(|| re_wrap.captures(line));
            let Some(caps) = caps else {
                continue;
            };
            let var = caps.name("var").map(|m| m.as_str()).unwrap_or("");
            let base = caps.name("base").map(|m| m.as_str()).unwrap_or("");
            let off = caps.name("off").map(|m| m.as_str()).unwrap_or("");
            if var.is_empty() || base.is_empty() || off.is_empty() {
                continue;
            }
            if !looks_like_generated_temp(var) {
                continue;
            }
            if renames.contains_key(var) {
                continue;
            }

            let semantic = semantic_base(base);
            let suffix = if off == "16" { "lo" } else { "hi" };
            let candidate = format!("{semantic}_{suffix}");
            let unique = unique_name(&candidate, &used_idents);
            used_idents.insert(unique.clone());
            renames.insert(var.to_string(), unique);
        }

        if renames.is_empty() {
            return None;
        }

        let mut body = block.body.clone();
        for (old_name, new_name) in &renames {
            body = rename_ident(&body, old_name, new_name);
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

fn semantic_base(base: &str) -> String {
    if base.to_ascii_lowercase().contains("amount") {
        return "amount".to_string();
    }
    if let Some(prefix) = base.strip_suffix("_val") {
        if is_ident(prefix) {
            return prefix.to_string();
        }
    }
    if base.starts_with("var") || base.starts_with("sv") {
        return "value".to_string();
    }
    base.to_string()
}

fn looks_like_generated_temp(name: &str) -> bool {
    name.starts_with("sv") || name.starts_with("slot_var") || name.starts_with("var")
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

fn rename_ident(lines: &[String], old_name: &str, new_name: &str) -> Vec<String> {
    lines
        .iter()
        .map(|line| replace_ident(line, old_name, new_name))
        .collect()
}

fn replace_ident(line: &str, old_name: &str, new_name: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(old_name) {
        let start = idx + pos;
        let end = start + old_name.len();
        let bytes = line.as_bytes();
        let left_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
        let right_ok = end >= bytes.len() || !is_ident_char(bytes[end] as char);
        if left_ok && right_ok {
            result.push_str(&line[idx..start]);
            result.push_str(new_name);
            idx = end;
        } else {
            result.push_str(&line[idx..end]);
            idx = end;
        }
    }
    result.push_str(&line[idx..]);
    result
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

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(is_ident_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renames_amount_decode_slots_to_semantic_names() {
        let p = I128DecodeSlotsPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut sv2_16_i64 = mload64!(amount_val as usize + 16) as i64;".to_string(),
                "    amount = sv2_16_i64;".to_string(),
                "    let mut sv2_24_i64 = mload64!(amount_val as usize + 24) as i64;".to_string(),
                "    a = sv2_24_i64;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert!(out.body.iter().any(|l| l.contains("amount_lo")));
        assert!(out.body.iter().any(|l| l.contains("amount_hi")));
        assert!(!out.body.iter().any(|l| l.contains("sv2_16_i64")));
        assert!(!out.body.iter().any(|l| l.contains("sv2_24_i64")));
    }
}
