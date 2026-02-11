use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct RemoveUnusedParamMutPattern;

impl RemoveUnusedParamMutPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveUnusedParamMutPattern {
    fn name(&self) -> &'static str {
        "remove_unused_param_mut"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.header.is_empty() || block.body.is_empty() || !block.header.contains('\n') {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::new();
        let lines: Vec<&str> = block.header.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            // Keep function declaration line and closing `)` / `)->` line intact.
            if idx == 0 || line.trim_start().starts_with(')') {
                out.push((*line).to_string());
                continue;
            }

            let trimmed = line.trim_start();
            if let Some(after_mut) = trimmed.strip_prefix("mut ") {
                if let Some((name_part, _ty_part)) = after_mut.split_once(':') {
                    let name = name_part.trim();
                    if !name.is_empty() && !is_reassigned(name, &block.body) {
                        let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                        out.push(format!("{indent}{after_mut}"));
                        changed = true;
                        continue;
                    }
                }
            }

            out.push((*line).to_string());
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: out.join("\n"),
            body: block.body.clone(),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn is_reassigned(name: &str, body: &[String]) -> bool {
    body.iter().any(|line| has_assignment_to_name(line, name))
}

fn has_assignment_to_name(line: &str, name: &str) -> bool {
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(name) {
        let start = idx + pos;
        let end = start + name.len();
        let bytes = line.as_bytes();

        let left_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
        let right_ok = end >= bytes.len() || !is_ident_char(bytes[end] as char);
        if !(left_ok && right_ok) {
            idx = end;
            continue;
        }

        let mut j = end;
        while j < bytes.len() && (bytes[j] as char).is_whitespace() {
            j += 1;
        }

        // Assignment operators that imply mutation of `name`.
        let ops = ["+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "="];
        for op in ops {
            if line[j..].starts_with(op) {
                // Exclude comparison / equality contexts.
                if op == "=" {
                    if line[j..].starts_with("==") {
                        continue;
                    }
                }
                return true;
            }
        }

        idx = end;
    }
    false
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_unused_mut_param() {
        let p = RemoveUnusedParamMutPattern::new();
        let block = FunctionBlock {
            header: "    pub fn f(\n        &mut self,\n        mut env: Env,\n        mut to: Address,\n    ) {".to_string(),
            body: vec!["        self.x(env, to);".to_string()],
            footer: "    }".to_string(),
            indent: "    ".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("pattern must apply");
        assert!(out.header.contains("        env: Env,"));
        assert!(out.header.contains("        to: Address,"));
    }

    #[test]
    fn keeps_mut_when_reassigned() {
        let p = RemoveUnusedParamMutPattern::new();
        let block = FunctionBlock {
            header: "    pub fn f(\n        &mut self,\n        mut x: i32,\n    ) -> i32 {".to_string(),
            body: vec!["        x = x + 1;".to_string(), "        x".to_string()],
            footer: "    }".to_string(),
            indent: "    ".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block);
        assert!(out.is_none());
    }
}
