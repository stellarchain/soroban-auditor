use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

/// Eliminates stack frame emulation patterns completely.
///
/// WASM uses global0 as a stack pointer. Functions often have this pattern:
/// ```rust
/// let mut frame_var: i32 = 0;
/// frame_var = self.global0.wrapping_sub(SIZE);
/// self.global0 = frame_var;
/// // ... function body ...
/// self.global0 = frame_var.wrapping_add(SIZE);
/// ```
///
/// This pattern:
/// 1. Identifies stack frame variables (assigned from self.global0 operations)
/// 2. Removes all lines involving stack frame operations
/// 3. Cleans up the function to remove stack frame artifacts
pub struct EliminateStackFramePattern;

impl EliminateStackFramePattern {
    pub fn new() -> Self {
        Self
    }

    /// Check if a line is a stack frame operation
    fn is_stack_frame_operation(line: &str) -> bool {
        let trimmed = line.trim();

        // Pattern 1: self.global0 = ...
        if trimmed.starts_with("self.global0 = ") || trimmed.starts_with("self.global0=") {
            return true;
        }

        // Pattern 2: var = self.global0...
        if trimmed.contains("= self.global0.wrapping_") {
            return true;
        }

        // Pattern 3: let x = self.global0
        if trimmed.contains("let ") && trimmed.contains("= self.global0") {
            return true;
        }

        false
    }

    /// Extract variable name from stack frame assignment
    /// e.g., "a = self.global0.wrapping_sub(32);" → Some("a")
    fn extract_frame_var(line: &str) -> Option<String> {
        let trimmed = line.trim();

        // Pattern: var = self.global0.wrapping_xxx
        if trimmed.contains("= self.global0.wrapping_") {
            if let Some(eq_pos) = trimmed.find('=') {
                let var_part = &trimmed[..eq_pos].trim();
                // Remove "let mut" or "let" prefix
                let var_name = var_part
                    .strip_prefix("let mut ")
                    .or_else(|| var_part.strip_prefix("let "))
                    .unwrap_or(var_part);

                // Remove type annotation
                let var_name = var_name.split(':').next().unwrap_or(var_name).trim();

                if !var_name.is_empty() && !var_name.contains(' ') {
                    return Some(var_name.to_string());
                }
            }
        }

        None
    }

    /// Check if a variable is only used in stack frame operations
    fn is_only_stack_frame_var(body: &[String], var: &str) -> bool {
        let mut uses = 0;
        let mut stack_frame_uses = 0;

        for line in body {
            let trimmed = line.trim();

            // Count all uses of the variable
            if trimmed.contains(var) {
                uses += 1;

                // Check if this use is in a stack frame operation
                if Self::is_stack_frame_operation(trimmed) {
                    stack_frame_uses += 1;
                } else if trimmed == format!("let mut {}: i32 = 0;", var)
                       || trimmed == format!("let mut {}:i32=0;", var)
                       || trimmed == format!("let {}: i32 = 0;", var) {
                    // Variable declaration initialized to 0 (stack frame pattern)
                    stack_frame_uses += 1;
                }
            }
        }

        // If all uses are in stack frame operations, it's a frame variable
        uses > 0 && uses == stack_frame_uses
    }
}

impl Pattern for EliminateStackFramePattern {
    fn name(&self) -> &'static str {
        "EliminateStackFramePattern"
    }

    fn apply(&self, func: &FunctionBlock) -> Option<FunctionBlock> {
        if func.body.is_empty() {
            return None;
        }

        // Step 1: Identify stack frame variables
        let mut frame_vars = HashSet::new();

        for line in &func.body {
            if let Some(var) = Self::extract_frame_var(line) {
                if Self::is_only_stack_frame_var(&func.body, &var) {
                    frame_vars.insert(var);
                }
            }
        }

        // Step 2: Remove all lines involving stack frame operations
        let mut new_body = Vec::new();
        let mut modified = false;

        for line in &func.body {
            let trimmed = line.trim();

            // Skip stack frame operations
            if Self::is_stack_frame_operation(trimmed) {
                modified = true;
                continue;
            }

            // Skip frame variable declarations (let mut a: i32 = 0;)
            let mut skip = false;
            for var in &frame_vars {
                if trimmed == format!("let mut {}: i32 = 0;", var)
                    || trimmed == format!("let mut {}:i32=0;", var)
                    || trimmed == format!("let {}: i32 = 0;", var)
                    || trimmed == format!("let {}:i32=0;", var) {
                    skip = true;
                    modified = true;
                    break;
                }
            }

            if skip {
                continue;
            }

            new_body.push(line.clone());
        }

        if !modified {
            return None;
        }

        // Step 3: Clean up any remaining frame variable references
        // (In case there are complex patterns we didn't catch)
        for var in &frame_vars {
            new_body.retain(|line| {
                let trimmed = line.trim();
                // Keep the line unless it's just a frame variable operation
                !trimmed.starts_with(&format!("{} = ", var))
                    || !trimmed.contains("self.global0")
            });
        }

        Some(FunctionBlock {
            header: func.header.clone(),
            body: new_body,
            footer: func.footer.clone(),
            indent: func.indent.clone(),
            name: func.name.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eliminate_stack_frame() {
        let func = FunctionBlock {
            header: "pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {".to_string(),
            body: vec![
                "    let mut a: i32 = 0;".to_string(),
                "    a = self.global0.wrapping_sub(32);".to_string(),
                "    self.global0 = a;".to_string(),
                "    let result = vec![&env, Symbol::new(env, \"Hello\"), to];".to_string(),
                "    self.global0 = a.wrapping_add(32);".to_string(),
                "    result".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = EliminateStackFramePattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();

        // Should only have the actual logic, no stack frame operations
        assert_eq!(result.body.len(), 2);
        assert!(result.body[0].contains("let result = vec!"));
        assert!(result.body[1].contains("result"));
    }

    #[test]
    fn test_no_stack_frame() {
        let func = FunctionBlock {
            header: "pub fn test(x: i32) -> i32 {".to_string(),
            body: vec![
                "    let y = x + 1;".to_string(),
                "    y".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "test".to_string(),
        };

        let pattern = EliminateStackFramePattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_none());
    }
}
