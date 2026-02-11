use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct MloadExpressionCleanupPattern;

impl MloadExpressionCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for MloadExpressionCleanupPattern {
    fn name(&self) -> &'static str {
        "mload_expression_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re_addr_add = Regex::new(r"(?P<base>\w+) as usize \+ (?P<off>\d+)").ok()?;

        let mut changed = false;
        let mut body = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let mut l = line.clone();

            // `amount_val as usize + 24` -> `amount_val.wrapping_add(24) as usize`
            let normalized = re_addr_add
                .replace_all(&l, "$base.wrapping_add($off) as usize")
                .to_string();
            if normalized != l {
                l = normalized;
                changed = true;
            }

            // Remove redundant explicit cast to the same mload width.
            let no_mload64_cast = strip_mload_cast(&l, "mload64!(", " as i64");
            if no_mload64_cast != l {
                l = no_mload64_cast;
                changed = true;
            }
            let no_mload32_cast = strip_mload_cast(&l, "mload32!(", " as i32");
            if no_mload32_cast != l {
                l = no_mload32_cast;
                changed = true;
            }

            body.push(l);
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

fn strip_mload_cast(line: &str, macro_start: &str, cast_suffix: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut i = 0usize;
    while i < line.len() {
        let Some(rel) = line[i..].find(macro_start) else {
            out.push_str(&line[i..]);
            break;
        };
        let start = i + rel;
        out.push_str(&line[i..start]);
        out.push_str(macro_start);
        let mut j = start + macro_start.len();
        let mut depth = 1i32;
        while j < line.len() && depth > 0 {
            let ch = line.as_bytes()[j] as char;
            out.push(ch);
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
            }
            j += 1;
        }
        if depth == 0 && line[j..].starts_with(cast_suffix) {
            j += cast_suffix.len();
        }
        i = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_mload_casts_and_address_math() {
        let p = MloadExpressionCleanupPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut s = mload32!(amount_val as usize) as i32;".to_string(),
                "    amount = mload64!(amount_val as usize + 24) as i64;".to_string(),
                "    a = mload64!(amount_val as usize + 16) as i64;".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "let mut s = mload32!(amount_val as usize);");
        assert_eq!(out.body[1].trim(), "amount = mload64!(amount_val.wrapping_add(24) as usize);");
        assert_eq!(out.body[2].trim(), "a = mload64!(amount_val.wrapping_add(16) as usize);");
    }
}
