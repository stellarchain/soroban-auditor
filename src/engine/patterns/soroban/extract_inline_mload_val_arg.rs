use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct ExtractInlineMloadValArgPattern;

impl ExtractInlineMloadValArgPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExtractInlineMloadValArgPattern {
    fn name(&self) -> &'static str {
        "extract_inline_mload_val_arg"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re_inline = Regex::new(
            r"val_from_i64\(mload64!\((?P<base>\w+)(?:\.wrapping_add\((?P<off>\d+)\))? as usize\)\)",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].clone();
            let Some(cap) = re_inline.captures(&line) else {
                i += 1;
                continue;
            };

            let base = cap.name("base").map(|m| m.as_str()).unwrap_or("");
            if base.is_empty() {
                i += 1;
                continue;
            }
            let off = cap.name("off").map(|m| m.as_str()).unwrap_or("0");
            let full = cap.get(0).map(|m| m.as_str()).unwrap_or("");
            if full.is_empty() {
                i += 1;
                continue;
            }

            let indent = leading_ws(&line);
            let mut used = collect_idents(&body);
            let tmp_base = semantic_name(base, off);
            let tmp = unique_name(&tmp_base, &used);
            used.insert(tmp.clone());

            let load_expr = if off == "0" {
                format!("mload64!({base} as usize)")
            } else {
                format!("mload64!({base}.wrapping_add({off}) as usize)")
            };

            body[i] = line.replacen(full, &format!("val_from_i64({tmp})"), 1);
            body.insert(i, format!("{indent}let {tmp} = {load_expr};"));
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

fn semantic_name(base: &str, off: &str) -> String {
    match off {
        "0" => format!("{base}_lo"),
        "8" => format!("{base}_hi"),
        "16" => format!("{base}_lo"),
        "24" => format!("{base}_hi"),
        _ => format!("{base}_part_{off}"),
    }
}

fn leading_ws(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
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
    fn extracts_inline_mload_in_publish_call() {
        let p = ExtractInlineMloadValArgPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    env.events().publish(val_from_i64(from), val_from_i64(mload64!(amount_val.wrapping_add(8) as usize)));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "let amount_val_hi = mload64!(amount_val.wrapping_add(8) as usize);"
        );
        assert_eq!(
            out.body[1].trim(),
            "env.events().publish(val_from_i64(from), val_from_i64(amount_val_hi));"
        );
    }
}

