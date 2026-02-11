use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct I32FlagIfSimplifyPattern;

impl I32FlagIfSimplifyPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for I32FlagIfSimplifyPattern {
    fn name(&self) -> &'static str {
        "i32_flag_if_simplify"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let re_assign =
            Regex::new(r"^(?P<indent>\s*)(?P<var>\w+)\s*=\s*\((?P<expr>.+)\)\s*as i32;$").ok()?;
        let re_if = Regex::new(r"^(?P<indent>\s*)if (?P<var>\w+) (?P<op>!=|==) 0 \{$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i + 1 < body.len() {
            let Some(a) = re_assign.captures(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(b) = re_if.captures(&body[i + 1]) else {
                i += 1;
                continue;
            };

            let var_a = a.name("var").map(|m| m.as_str()).unwrap_or("");
            let var_b = b.name("var").map(|m| m.as_str()).unwrap_or("");
            if var_a.is_empty() || var_b.is_empty() || var_a != var_b {
                i += 1;
                continue;
            }
            if has_use_before_reassign(&body, i + 2, var_a) {
                i += 1;
                continue;
            }

            let expr = a.name("expr").map(|m| m.as_str()).unwrap_or("").trim();
            let if_indent = b.name("indent").map(|m| m.as_str()).unwrap_or("");
            let op = b.name("op").map(|m| m.as_str()).unwrap_or("");
            if expr.is_empty() || op.is_empty() {
                i += 1;
                continue;
            }

            let cond = if op == "!=" {
                expr.to_string()
            } else {
                format!("!({expr})")
            };

            body[i + 1] = format!("{if_indent}if {cond} {{");
            body.remove(i);
            changed = true;
            continue;
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

fn has_use_before_reassign(lines: &[String], start: usize, ident: &str) -> bool {
    for line in lines.iter().skip(start) {
        let t = line.trim();
        if let Some((lhs, _)) = t.trim_end_matches(';').split_once(" = ") {
            if lhs.trim() == ident {
                return false;
            }
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
    fn simplifies_i32_flag_if_chain() {
        let p = I32FlagIfSimplifyPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    b = (d == 0) as i32;".to_string(),
                "    if b != 0 {".to_string(),
                "        x = 1;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "if d == 0 {");
    }
}
