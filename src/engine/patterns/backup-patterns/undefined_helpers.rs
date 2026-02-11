use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Pattern pentru eliminarea apelurilor la helper functions undefined
///
/// Elimină sau comentează:
/// - `self.entry_decode(...);`
/// - `self.decode_val_or_error(...);`
/// - `self.vec_pair_iter(...);`
/// - `self.copy_val_if_present(...);`
/// - `self.write_ok_val(...);`
///
/// Acestea sunt helper functions care NU sunt emise în output final!
pub struct UndefinedHelpersPattern;

impl UndefinedHelpersPattern {
    pub fn new() -> Self {
        Self
    }

    fn is_undefined_helper_call(line: &str) -> bool {
        let trimmed = line.trim();

        // Never touch control-flow heads. Removing these can orphan `else` blocks.
        if trimmed.starts_with("if ")
            || trimmed.starts_with("while ")
            || trimmed.starts_with("match ")
            || trimmed.starts_with("return ")
        {
            return false;
        }

        // Only rewrite standalone call statements or direct call let-bindings.
        // Avoid control-expression bindings like `let x = if ... { ... } else { ... }`.
        let is_direct_self_stmt = trimmed.starts_with("self.");
        let is_direct_let_self_call = trimmed.starts_with("let ") && trimmed.contains("= self.");
        if !(is_direct_self_stmt || is_direct_let_self_call) {
            return false;
        }

        // List of known undefined helpers
        let undefined_helpers = [
            "self.entry_decode(",
            "self.decode_val_or_error(",
            "self.vec_pair_iter(",
            "self.copy_val_if_present(",
            "self.write_ok_val(",
            "self.write_ok_err(",
            "self.copy_payload(",
        ];

        for helper in &undefined_helpers {
            if trimmed.contains(helper) {
                return true;
            }
        }

        false
    }
}

impl Pattern for UndefinedHelpersPattern {
    fn name(&self) -> &'static str {
        "undefined_helpers"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut new_body: Vec<String> = Vec::with_capacity(block.body.len());
        let mut i = 0usize;
        while i < block.body.len() {
            let line = &block.body[i];
            let trimmed = line.trim();

            // Skip if already has TODO comment (prevent duplicates in iterative mode)
            if trimmed.starts_with("// TODO:") {
                new_body.push(line.clone());
                i += 1;
                continue;
            }

            if Self::is_undefined_helper_call(line) {
                changed = true;
                let indent = line.len() - line.trim_start().len();
                new_body.push(format!(
                    "{}// TODO: helper function call removed: {}",
                    " ".repeat(indent),
                    trimmed
                ));

                // If this is a multiline call, skip remaining argument lines until call end.
                // This prevents orphan lines like:
                //   env,
                //   arg1,
                // );
                if !trimmed.contains(");") {
                    i += 1;
                    while i < block.body.len() {
                        let t = block.body[i].trim();
                        if t == ");" || t.ends_with(");") {
                            break;
                        }
                        i += 1;
                    }
                }
                i += 1;
                continue;
            }

            new_body.push(line.clone());
            i += 1;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comments_out_entry_decode() {
        let pattern = UndefinedHelpersPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    self.entry_decode(env, value, feed_ids, payload);".to_string(),
                "    let x = 42;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert_eq!(result.body.len(), 2);
        assert!(result.body[0].contains("// TODO:"));
        assert!(result.body[0].contains("entry_decode"));
        assert!(result.body[1].contains("let x = 42"));
    }

    #[test]
    fn test_comments_out_vec_pair_iter() {
        let pattern = UndefinedHelpersPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "        self.vec_pair_iter(env, value, value.wrapping_add(48));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert!(result.body[0].contains("// TODO:"));
        assert!(result.body[0].contains("vec_pair_iter"));
    }
}
