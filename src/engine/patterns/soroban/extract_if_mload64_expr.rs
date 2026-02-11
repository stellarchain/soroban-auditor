use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct ExtractIfMload64ExprPattern;

impl ExtractIfMload64ExprPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExtractIfMload64ExprPattern {
    fn name(&self) -> &'static str {
        "extract_if_mload64_expr"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re_mload = Regex::new(
            r"mload64!\((?P<base>\w+)(?:\.wrapping_add\((?P<off>\d+)\))? as usize\)",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].clone();
            let trimmed = line.trim_start();
            if !trimmed.starts_with("if ") || !trimmed.ends_with('{') || !line.contains("mload64!(") {
                i += 1;
                continue;
            }

            let mut replacements: Vec<(String, String)> = Vec::new();
            for cap in re_mload.captures_iter(&line) {
                let Some(full) = cap.get(0).map(|m| m.as_str().to_string()) else {
                    continue;
                };
                if replacements.iter().any(|(src, _)| src == &full) {
                    continue;
                }
                let base = cap.name("base").map(|m| m.as_str()).unwrap_or("");
                if base.is_empty() {
                    continue;
                }
                let off = cap.name("off").map(|m| m.as_str()).unwrap_or("0");
                let used = collect_idents(&body);
                let tmp = unique_name(&semantic_name(base, off), &used);
                replacements.push((full, tmp));
            }

            if replacements.is_empty() {
                i += 1;
                continue;
            }

            let indent = leading_ws(&line);
            let mut new_if_line = line;
            let mut decls: Vec<String> = Vec::new();
            for (full, tmp) in &replacements {
                new_if_line = new_if_line.replace(full, tmp);
                let load_expr = full.clone();
                decls.push(format!("{indent}let {tmp} = {load_expr};"));
            }

            body[i] = new_if_line;
            for (offset, decl) in decls.into_iter().enumerate() {
                body.insert(i + offset, decl);
            }
            changed = true;
            i += replacements.len() + 1;
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
    fn extracts_mload64_from_if_expression() {
        let p = ExtractIfMload64ExprPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if d.wrapping_add(mload64!(a.wrapping_add(32) as usize)) > 0 {".to_string(),
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
            "let a_part_32 = mload64!(a.wrapping_add(32) as usize);"
        );
        assert_eq!(out.body[1].trim(), "if d.wrapping_add(a_part_32) > 0 {");
    }
}

