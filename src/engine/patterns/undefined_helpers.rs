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
        let new_body: Vec<String> = block
            .body
            .iter()
            .filter_map(|line| {
                let trimmed = line.trim();

                // Skip if already has TODO comment (prevent duplicates in iterative mode)
                if trimmed.starts_with("// TODO:") {
                    return Some(line.clone());
                }

                if Self::is_undefined_helper_call(line) {
                    changed = true;
                    // Comment out the helper call instead of removing it
                    // This helps with debugging and understanding what was removed
                    let indent = line.len() - line.trim_start().len();
                    Some(format!(
                        "{}// TODO: helper function call removed: {}",
                        " ".repeat(indent),
                        trimmed
                    ))
                } else {
                    Some(line.clone())
                }
            })
            .collect();

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
