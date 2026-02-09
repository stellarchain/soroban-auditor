use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use crate::engine::patterns::*;

/// Postprocessing pipeline for cleaning up generated code
pub struct PostProcessor {
    patterns: Vec<Box<dyn Pattern>>,
}

impl PostProcessor {
    pub fn new() -> Self {
        let mut patterns: Vec<Box<dyn Pattern>> = Vec::new();

        // Order matters! Apply patterns in logical sequence:

        // 1. Remove undefined helper calls first (makes code compilable)
        patterns.push(Box::new(UndefinedHelpersPattern::new()));

        // 2. Clean up stack frame artifacts
        patterns.push(Box::new(StackFramePattern::new()));

        // 3. Eliminate val_from_i64/val_to_i64 conversions
        patterns.push(Box::new(ConversionEliminationPattern::new()));

        // 4. Add missing semicolons (syntax cleanup)
        patterns.push(Box::new(MissingSemicolonsPattern::new()));

        // TODO: Add more patterns as they're developed
        // - VariableNamingPattern (var5 → meaningful names)
        // - StorageAccessPattern (optimize storage calls)
        // - MathOperationsPattern (simplify arithmetic)
        // - Label cleanup patterns

        Self { patterns }
    }

    /// Apply all patterns to a function's code
    pub fn process_function(&self, code: &str, fn_name: &str) -> String {
        // Parse function into header + body + footer
        let block = match Self::parse_function(code, fn_name) {
            Some(b) => b,
            None => return code.to_string(), // Can't parse, return as-is
        };

        // Apply patterns iteratively until no more changes
        let mut current = block;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 10;

        loop {
            let mut changed = false;
            iteration += 1;

            if iteration > MAX_ITERATIONS {
                eprintln!("Warning: PostProcessor hit max iterations for {}", fn_name);
                break;
            }

            for pattern in &self.patterns {
                if let Some(new_block) = pattern.apply(&current) {
                    current = new_block;
                    changed = true;
                }
            }

            if !changed {
                break; // Reached fixpoint
            }
        }

        // Reconstruct function
        Self::reconstruct_function(&current)
    }

    /// Parse a function string into FunctionBlock
    fn parse_function(code: &str, fn_name: &str) -> Option<FunctionBlock> {
        let lines: Vec<&str> = code.lines().collect();
        if lines.is_empty() {
            return None;
        }

        // Find function header (pub fn NAME(...) ...)
        let header_idx = lines.iter().position(|l| {
            l.trim_start().starts_with("pub fn") && l.contains(fn_name)
        })?;

        let header = lines[header_idx].to_string();

        // Find opening brace of function body
        let mut body_start = header_idx + 1;
        if !header.trim_end().ends_with('{') {
            // Header might span multiple lines
            while body_start < lines.len() && !lines[body_start].contains('{') {
                body_start += 1;
            }
            body_start += 1; // Skip the line with '{'
        }

        // Find closing brace (matching brace depth)
        let mut depth = 1;
        let mut body_end = body_start;
        while body_end < lines.len() && depth > 0 {
            let line = lines[body_end];
            depth += line.matches('{').count() as i32;
            depth -= line.matches('}').count() as i32;
            if depth == 0 {
                break;
            }
            body_end += 1;
        }

        if depth != 0 {
            return None; // Unmatched braces
        }

        let body: Vec<String> = lines[body_start..body_end]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let footer = lines[body_end].to_string();

        // Detect indentation
        let indent = if let Some(first_body_line) = body.first() {
            let trimmed = first_body_line.trim_start();
            let indent_len = first_body_line.len() - trimmed.len();
            " ".repeat(indent_len)
        } else {
            "    ".to_string()
        };

        Some(FunctionBlock {
            header,
            body,
            footer,
            indent,
            name: fn_name.to_string(),
        })
    }

    /// Reconstruct function from FunctionBlock
    fn reconstruct_function(block: &FunctionBlock) -> String {
        let mut result = String::new();
        result.push_str(&block.header);
        result.push('\n');
        for line in &block.body {
            result.push_str(line);
            result.push('\n');
        }
        result.push_str(&block.footer);
        result.push('\n');
        result
    }
}

impl Default for PostProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = r#"    pub fn test(&mut self, env: Env) -> i32 {
        let x = 42;
        x
    }
"#;
        let block = PostProcessor::parse_function(code, "test").unwrap();
        assert_eq!(block.name, "test");
        assert!(block.header.contains("pub fn test"));
        assert_eq!(block.body.len(), 2);
        assert!(block.footer.contains('}'));
    }

    #[test]
    fn test_process_with_undefined_helpers() {
        let processor = PostProcessor::new();
        let code = r#"    pub fn test(&mut self, env: Env) {
        self.entry_decode(env, 5);
        let x = 42;
    }
"#;
        let result = processor.process_function(code, "test");
        // Should contain TODO comment for removed helper
        assert!(result.contains("TODO"));
    }
}
