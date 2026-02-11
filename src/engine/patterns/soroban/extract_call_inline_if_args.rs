use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;
use std::collections::HashSet;

pub struct ExtractCallInlineIfArgsPattern;

impl ExtractCallInlineIfArgsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExtractCallInlineIfArgsPattern {
    fn name(&self) -> &'static str {
        "extract_call_inline_if_args"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re = Regex::new(
            r"^(?P<indent>\s*)(?P<prefix>(?:let(?: mut)? )?\w+(?::\s*[^=]+)?\s*=\s*.+?,)\s*\(if (?P<c1>.+?) \{ (?P<t1>.+?) \} else \{ (?P<e1>.+?) \}\),\s*\(if (?P<c2>.+?) \{ (?P<t2>.+?) \} else \{ (?P<e2>.+?) \}\)\);$",
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
            let prefix = cap.name("prefix").map(|m| m.as_str()).unwrap_or("");
            let c1 = cap.name("c1").map(|m| m.as_str()).unwrap_or("");
            let t1 = cap.name("t1").map(|m| m.as_str()).unwrap_or("");
            let e1 = cap.name("e1").map(|m| m.as_str()).unwrap_or("");
            let c2 = cap.name("c2").map(|m| m.as_str()).unwrap_or("");
            let t2 = cap.name("t2").map(|m| m.as_str()).unwrap_or("");
            let e2 = cap.name("e2").map(|m| m.as_str()).unwrap_or("");
            if prefix.is_empty()
                || c1.is_empty()
                || t1.is_empty()
                || e1.is_empty()
                || c2.is_empty()
                || t2.is_empty()
                || e2.is_empty()
            {
                i += 1;
                continue;
            }

            let used = collect_idents(&body);
            let arg1 = unique_name("call_arg1", &used);
            let mut used2 = used;
            used2.insert(arg1.clone());
            let arg2 = unique_name("call_arg2", &used2);

            let replacement = vec![
                format!("{indent}let {arg1} = if {c1} {{ {t1} }} else {{ {e1} }};"),
                format!("{indent}let {arg2} = if {c2} {{ {t2} }} else {{ {e2} }};"),
                format!("{indent}{prefix} {arg1}, {arg2});"),
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
    fn extracts_two_inline_if_call_args() {
        let p = ExtractCallInlineIfArgsPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let g = self.pack_i128_val(env, (if b != 0 { value_lo } else { 0 /* False */ }), (if b != 0 { f } else { 0 /* False */ }));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body[0].trim(),
            "let call_arg1 = if b != 0 { value_lo } else { 0 /* False */ };"
        );
        assert_eq!(
            out.body[1].trim(),
            "let call_arg2 = if b != 0 { f } else { 0 /* False */ };"
        );
        assert_eq!(
            out.body[2].trim(),
            "let g = self.pack_i128_val(env, call_arg1, call_arg2);"
        );
    }
}

