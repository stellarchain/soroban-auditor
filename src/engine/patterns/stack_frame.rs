use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

/// Pattern pentru eliminarea stack frame emulation
///
/// Detectează și elimină:
/// - `let varN = self.global0;`
/// - `value = varN.wrapping_sub(SIZE);`
/// - `self.global0 = value;`
/// - `self.global0 = value.wrapping_add(SIZE);`
///
/// Acestea sunt artefacte WASM de management stack - inutile în Rust!
pub struct StackFramePattern;

impl StackFramePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StackFramePattern {
    fn name(&self) -> &'static str {
        "stack_frame"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        // First pass: collect variables that are assigned from self.global0
        let mut stack_vars: Vec<String> = Vec::new();
        for line in &block.body {
            let trimmed = line.trim();
            // Pattern: let varN = self.global0;
            if trimmed.starts_with("let ") && trimmed.contains("= self.global0") {
                if let Some(var_name) = extract_var_name_from_let(trimmed) {
                    stack_vars.push(var_name);
                }
            }
        }

        let mut changed = false;
        let new_body: Vec<String> = block
            .body
            .iter()
            .filter(|line| {
                let trimmed = line.trim();

                // Pattern 1: let varN = self.global0;
                if trimmed.starts_with("let ") && trimmed.contains("= self.global0") {
                    changed = true;
                    return false; // Remove line
                }

                // Pattern 2: self.global0 = value/var;
                if trimmed.starts_with("self.global0 = ") {
                    changed = true;
                    return false; // Remove line
                }

                // Pattern 3: value = stackVar.wrapping_sub(...); (stack allocation using removed var)
                if trimmed.starts_with("value = ") && trimmed.contains(".wrapping_sub(") {
                    // Check if it uses a stack variable
                    for stack_var in &stack_vars {
                        if trimmed.contains(&format!("{}.wrapping_sub(", stack_var)) {
                            changed = true;
                            return false; // Remove line
                        }
                    }

                    // Also check for generic pattern value = varN.wrapping_sub(NUMBER)
                    if let Some(num_start) = trimmed.find(".wrapping_sub(") {
                        let before = &trimmed["value = ".len()..num_start];
                        // If it's a var-like name, likely stack allocation
                        if before.trim().starts_with("var") {
                            let after = &trimmed[num_start + 14..];
                            if let Some(close) = after.find(')') {
                                let num_str = &after[..close];
                                if num_str.chars().all(|c| c.is_numeric()) {
                                    changed = true;
                                    return false;
                                }
                            }
                        }
                    }
                }

                // Pattern 4: value = stackVar; (assignment from stack pointer var)
                if trimmed.starts_with("value = ") && !trimmed.contains('(') {
                    for stack_var in &stack_vars {
                        let pattern = format!("value = {};", stack_var);
                        if trimmed == pattern.trim() {
                            changed = true;
                            return false; // Remove line
                        }
                    }
                }

                true // Keep line
            })
            .cloned()
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

fn extract_var_name_from_let(line: &str) -> Option<String> {
    // Extract var name from: let mut? varN = ...
    let after_let = line.strip_prefix("let ")?.trim_start();
    // Optional mut keyword
    let after_let = after_let.strip_prefix("mut ").unwrap_or(after_let).trim_start();

    let name: String = after_let
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removes_global0_save() {
        let pattern = StackFramePattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let var5 = self.global0;".to_string(),
                "    let x = 42;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert_eq!(result.body.len(), 1);
        assert!(result.body[0].contains("let x = 42"));
    }

    #[test]
    fn test_removes_stack_allocation() {
        let pattern = StackFramePattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    value = var5.wrapping_sub(96);".to_string(),
                "    let x = 42;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert_eq!(result.body.len(), 1);
        assert!(result.body[0].contains("let x = 42"));
    }

    #[test]
    fn test_removes_global0_update() {
        let pattern = StackFramePattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    self.global0 = value;".to_string(),
                "    let x = 42;".to_string(),
                "    self.global0 = value.wrapping_add(96);".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert_eq!(result.body.len(), 1);
        assert!(result.body[0].contains("let x = 42"));
    }
}
