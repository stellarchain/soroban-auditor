use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct I128PartsCallUnpackPattern;

impl I128PartsCallUnpackPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for I128PartsCallUnpackPattern {
    fn name(&self) -> &'static str {
        "i128_parts_call_unpack"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re_call = Regex::new(
            r"^(?P<indent>\s*)let(?: mut)? (?P<dst>\w+)(?::\s*[^=]+)? = self\.i128_parts_to_val\((?P<env>[^,]+),\s*mload64!\((?P<base_lo>\w+) as usize\),\s*mload64!\((?P<base_hi>\w+)\.wrapping_add\(8\) as usize\)\);$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;

        let mut i = 0usize;
        while i < body.len() {
            let Some(cap) = re_call.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("").to_string();
            let dst = cap.name("dst").map(|m| m.as_str()).unwrap_or("").to_string();
            let env = cap.name("env").map(|m| m.as_str()).unwrap_or("").to_string();
            let base_lo = cap.name("base_lo").map(|m| m.as_str()).unwrap_or("").to_string();
            let base_hi = cap.name("base_hi").map(|m| m.as_str()).unwrap_or("").to_string();
            if dst.is_empty() || env.is_empty() || base_lo.is_empty() || base_hi.is_empty() {
                i += 1;
                continue;
            }
            if base_lo != base_hi {
                i += 1;
                continue;
            }
            let base = base_lo;

            let mut used = collect_idents(&body);
            let semantic_base = semantic_base(&base);
            let lo_name = unique_name(&format!("{semantic_base}_lo"), &used);
            used.insert(lo_name.clone());
            let hi_name = unique_name(&format!("{semantic_base}_hi"), &used);

            let replacement = vec![
                format!("{indent}let {lo_name} = mload64!({base} as usize);"),
                format!("{indent}let {hi_name} = mload64!({base}.wrapping_add(8) as usize);"),
                format!("{indent}let {dst} = self.i128_parts_to_val({env}, {lo_name}, {hi_name});"),
            ];
            body.splice(i..=i, replacement);
            changed = true;
            i += 3;
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

fn semantic_base(base: &str) -> String {
    if let Some(prefix) = base.strip_suffix("_val") {
        if is_ident(prefix) {
            return prefix.to_string();
        }
    }
    if base.to_ascii_lowercase().contains("amount") {
        return "amount".to_string();
    }
    if base.starts_with("var") || base.starts_with("sv") {
        return "value".to_string();
    }
    base.to_string()
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
    fn expands_inline_mload_pair_in_i128_parts_call() {
        let p = I128PartsCallUnpackPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let c = self.i128_parts_to_val(env, mload64!(a as usize), mload64!(a.wrapping_add(8) as usize));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "let a_lo = mload64!(a as usize);");
        assert_eq!(out.body[1].trim(), "let a_hi = mload64!(a.wrapping_add(8) as usize);");
        assert_eq!(out.body[2].trim(), "let c = self.i128_parts_to_val(env, a_lo, a_hi);");
    }
}
