use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct I128PairUnpackPattern;

impl I128PairUnpackPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for I128PairUnpackPattern {
    fn name(&self) -> &'static str {
        "i128_pair_unpack"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let re_assign = Regex::new(
            r"^(?P<indent>\s*)(?P<lhs>\w+)\s*=\s*mload64!\((?P<base>\w+)\.wrapping_add\((?P<off>16|24)\) as usize\);$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i + 1 < body.len() {
            let Some(c1) = re_assign.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(c2) = re_assign.captures(&body[i + 1]) else {
                i += 1;
                continue;
            };

            let lhs1 = c1.name("lhs").map(|m| m.as_str()).unwrap_or("").to_string();
            let lhs2 = c2.name("lhs").map(|m| m.as_str()).unwrap_or("").to_string();
            let base1 = c1.name("base").map(|m| m.as_str()).unwrap_or("").to_string();
            let base2 = c2.name("base").map(|m| m.as_str()).unwrap_or("").to_string();
            let off1 = c1.name("off").map(|m| m.as_str()).unwrap_or("").to_string();
            let off2 = c2.name("off").map(|m| m.as_str()).unwrap_or("").to_string();
            let indent = c1.name("indent").map(|m| m.as_str()).unwrap_or("").to_string();

            if lhs1.is_empty()
                || lhs2.is_empty()
                || base1.is_empty()
                || base2.is_empty()
                || off1 == off2
                || base1 != base2
            {
                i += 1;
                continue;
            }

            let semantic_base = semantic_base(&base1);
            let mut used = collect_idents(&body);
            let lo_name = unique_name(&format!("{semantic_base}_lo"), &used);
            used.insert(lo_name.clone());
            let hi_name = unique_name(&format!("{semantic_base}_hi"), &used);

            let (lo_lhs, hi_lhs) = if off1 == "16" {
                (lhs1.as_str(), lhs2.as_str())
            } else {
                (lhs2.as_str(), lhs1.as_str())
            };

            body[i] = format!(
                "{indent}let {lo_name} = mload64!({base1}.wrapping_add(16) as usize);"
            );
            body[i + 1] = format!(
                "{indent}let {hi_name} = mload64!({base1}.wrapping_add(24) as usize);"
            );

            let mut j = i + 2;
            while j < body.len() {
                if reassigns_ident(&body[j], lo_lhs) || reassigns_ident(&body[j], hi_lhs) {
                    break;
                }
                let line = replace_ident(&replace_ident(&body[j], lo_lhs, &lo_name), hi_lhs, &hi_name);
                body[j] = line;
                j += 1;
            }

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

fn reassigns_ident(line: &str, ident: &str) -> bool {
    let t = line.trim();
    if let Some((lhs, _)) = t.trim_end_matches(';').split_once(" = ") {
        lhs.trim() == ident
    } else {
        false
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
    fn rewrites_i128_slot_pair_to_semantic_pair() {
        let p = I128PairUnpackPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    amount = mload64!(amount_val.wrapping_add(16) as usize);".to_string(),
                "    a = mload64!(amount_val.wrapping_add(24) as usize);".to_string(),
                "    self.require_nonnegative_i128(env, amount, a);".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);"
        );
        assert_eq!(
            out.body[1].trim(),
            "let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);"
        );
        assert_eq!(
            out.body[2].trim(),
            "self.require_nonnegative_i128(env, amount_lo, amount_hi);"
        );
    }
}
