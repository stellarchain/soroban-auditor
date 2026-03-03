// src/engine/patterns/soroban/line_length_formatter.rs
//
// Format extremely long lines into multi-line blocks.
// This fixes issues like:
// - Match statements on single line (500+ chars)
// - Complex chain calls on single line
// - If statements with large bodies
//
// Goal: Make code parseable by rustfmt

use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LineFormatterPattern;

impl LineFormatterPattern {
    pub fn new() -> Self {
        Self {}
    }

    /// Break lines longer than 120 characters at logical boundaries
    fn should_break_line(line: &str) -> bool {
        line.len() > 120 && (line.contains("match ") || line.contains("=>") || line.contains("{") && line.contains("}"))
    }

    /// Format match statements: "let x = match y { 0 => {...}, 1 => {...} }" 
    /// becomes multi-line
    fn format_match_statement(line: &str) -> Vec<String> {
        if !line.contains("match ") || line.len() < 120 {
            return vec![line.to_string()];
        }

        let mut result = Vec::new();
        
        // Simple heuristic: if line has match, split at "match" and format arms
        if let Some(match_pos) = line.find("match ") {
            // Preamble before match (e.g., "        let a = ")
            let pre_match = &line[..match_pos + 6];
            let rest = &line[match_pos + 6..];
            
            // Try to extract: "var { arm1 => body1, arm2 => body2, ... }"
            if let Some(open_brace) = rest.find('{') {
                let var_part = &rest[..open_brace].trim();
                let arms_part = &rest[open_brace + 1..];
                
                result.push(format!("{}{} {{", pre_match, var_part));
                
                // Split arms at ", " but be careful about nested braces
                let mut current_arm = String::new();
                let mut brace_depth = 0;
                
                for ch in arms_part.chars() {
                    match ch {
                        '{' => {
                            brace_depth += 1;
                            current_arm.push(ch);
                        }
                        '}' => {
                            brace_depth -= 1;
                            current_arm.push(ch);
                            
                            if brace_depth == 0 && current_arm.contains("=>") {
                                // End of this arm, emit it
                                result.push(format!("    {},", current_arm.trim()));
                                current_arm.clear();
                            }
                        }
                        ',' if brace_depth == 0 => {
                            // Arm separator at top level
                            if !current_arm.trim().is_empty() {
                                result.push(format!("    {},", current_arm.trim()));
                                current_arm.clear();
                            }
                        }
                        _ => current_arm.push(ch),
                    }
                }
                
                // Remainder (closing brace)
                result.push("        }".to_string());
                return result;
            }
        }

        vec![line.to_string()]
    }
}

impl Pattern for LineFormatterPattern {
    fn name(&self) -> &'static str {
        "LineFormatter"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        let mut new_body: Vec<String> = Vec::new();
        let mut changed = false;

        for line in &block.body {
            if Self::should_break_line(line) {
                let formatted = Self::format_match_statement(line);
                if formatted.len() > 1 {
                    new_body.extend(formatted);
                    changed = true;
                } else {
                    new_body.push(line.clone());
                }
            } else {
                new_body.push(line.clone());
            }
        }

        if changed {
            Some(FunctionBlock {
                header: block.header.clone(),
                body: new_body,
                footer: block.footer.clone(),
                indent: block.indent.clone(),
                name: block.name.clone(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_long_match() {
        let line = "let b = match arg1 { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(a), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0}";
        assert!(LineFormatterPattern::should_break_line(line));
    }

    #[test]
    fn test_format_match_simple() {
        let line = "let x = match y { 0 => 1, 1 => 2 }";
        let result = LineFormatterPattern::format_match_statement(line);
        assert!(result.len() > 1);
    }
}
