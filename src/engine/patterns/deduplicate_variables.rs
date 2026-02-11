use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

/// Pattern pentru eliminarea declarațiilor duplicate de variabile
///
/// Detectează și elimină:
/// - Multiple `let mut value: i64 = 0;` declarations
/// - Duplicate variable declarations cu același tip
///
/// Păstrează doar prima declarație pentru fiecare variabilă.
pub struct DeduplicateVariablesPattern;

impl DeduplicateVariablesPattern {
    pub fn new() -> Self {
        Self
    }

    /// Extrage numele variabilei dintr-o declarație let
    fn extract_var_name(line: &str) -> Option<String> {
        let trimmed = line.trim();

        if !trimmed.starts_with("let ") {
            return None;
        }

        let after_let = trimmed.strip_prefix("let ")?.trim_start();
        let after_let = after_let.strip_prefix("mut ")?.trim_start();

        // Extract variable name (până la : sau =)
        let var_name = if let Some(colon_pos) = after_let.find(':') {
            &after_let[..colon_pos]
        } else if let Some(eq_pos) = after_let.find('=') {
            &after_let[..eq_pos]
        } else {
            return None;
        };

        Some(var_name.trim().to_string())
    }

    /// Check if this is a simple initialization (let mut var: Type = default;)
    fn is_simple_initialization(line: &str) -> bool {
        let trimmed = line.trim();

        // Must be a let declaration
        if !trimmed.starts_with("let ") {
            return false;
        }

        // Must have type annotation and default value
        if !trimmed.contains(':') || !trimmed.contains(" = ") {
            return false;
        }

        // Check for simple default values
        trimmed.ends_with(" = 0;")
            || trimmed.ends_with(" = 0")
            || trimmed.ends_with(" = false;")
            || trimmed.ends_with(" = false")
            || trimmed.contains(" = 0 as ")
    }
}

impl Pattern for DeduplicateVariablesPattern {
    fn name(&self) -> &'static str {
        "deduplicate_variables"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut declared_vars: HashSet<String> = HashSet::new();
        let mut new_body: Vec<String> = Vec::new();

        for line in &block.body {
            // Check if this is a simple variable initialization
            if Self::is_simple_initialization(line) {
                if let Some(var_name) = Self::extract_var_name(line) {
                    // If already declared, skip this duplicate
                    if declared_vars.contains(&var_name) {
                        changed = true;
                        continue;
                    }

                    // First declaration - keep it and mark as declared
                    declared_vars.insert(var_name);
                }
            }

            // Keep the line
            new_body.push(line.clone());
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
    fn test_extract_var_name() {
        assert_eq!(
            DeduplicateVariablesPattern::extract_var_name("let mut value: i64 = 0;"),
            Some("value".to_string())
        );
        assert_eq!(
            DeduplicateVariablesPattern::extract_var_name("let mut var5: i32 = 0;"),
            Some("var5".to_string())
        );
    }

    #[test]
    fn test_is_simple_initialization() {
        assert!(DeduplicateVariablesPattern::is_simple_initialization(
            "let mut value: i64 = 0;"
        ));
        assert!(DeduplicateVariablesPattern::is_simple_initialization(
            "let mut flag: i32 = 0 as i32;"
        ));
        assert!(!DeduplicateVariablesPattern::is_simple_initialization(
            "let mut value = compute();"
        ));
    }

    #[test]
    fn test_removes_duplicates() {
        let pattern = DeduplicateVariablesPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let mut value: i64 = 0;".to_string(),
                "    let mut value: i64 = 0;".to_string(),
                "    let mut value: i64 = 0;".to_string(),
                "    value = 42;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert_eq!(result.body.len(), 2);
        assert!(result.body[0].contains("let mut value"));
        assert!(result.body[1].contains("value = 42"));
    }
}
