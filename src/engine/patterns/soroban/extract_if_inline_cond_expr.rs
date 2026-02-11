use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct ExtractIfInlineCondExprPattern;

impl ExtractIfInlineCondExprPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExtractIfInlineCondExprPattern {
    fn name(&self) -> &'static str {
        "extract_if_inline_cond_expr"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re = Regex::new(
            r"^(?P<indent>\s*)if \(\(if (?P<c>.+?) \{ (?P<t>.+?) \} else \{ (?P<e>.+?) \}\)\) (?P<op>==|!=) (?P<rhs>.+?) \{$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].clone();
            let Some(cap) = re.captures(&line) else {
                i += 1;
                continue;
            };

            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("");
            let c = cap.name("c").map(|m| m.as_str()).unwrap_or("");
            let t = cap.name("t").map(|m| m.as_str()).unwrap_or("");
            let e = cap.name("e").map(|m| m.as_str()).unwrap_or("");
            let op = cap.name("op").map(|m| m.as_str()).unwrap_or("");
            let rhs = cap.name("rhs").map(|m| m.as_str()).unwrap_or("");
            if c.is_empty() || t.is_empty() || e.is_empty() || op.is_empty() || rhs.is_empty() {
                i += 1;
                continue;
            }

            let used = collect_idents(&body);
            let tmp = unique_name("cond_val", &used);
            body[i] = format!("{indent}if {tmp} {op} {rhs} {{");
            body.insert(i, format!("{indent}let {tmp} = if {c} {{ {t} }} else {{ {e} }};"));
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
    fn extracts_inline_if_from_if_condition() {
        let p = ExtractIfInlineCondExprPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if ((if b != 0 { (g > j) as i32 } else { (d > 0) as i32 })) != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "let cond_val = if b != 0 { (g > j) as i32 } else { (d > 0) as i32 };"
        );
        assert_eq!(out.body[1].trim(), "if cond_val != 0 {");
    }
}

