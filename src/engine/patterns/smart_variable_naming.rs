use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

/// Pattern pentru redenumirea inteligentă a variabilelor generice
///
/// Analizează cum sunt folosite variabilele și le redenumește contextual:
/// - slot_var2_80_i64 → feed_ids_val (based on context)
/// - var9 → feed_count (based on .len() call)
/// - var10 → result_vec (based on Vec::new() call)
pub struct SmartVariableNamingPattern;

impl SmartVariableNamingPattern {
    pub fn new() -> Self {
        Self
    }

    /// Detectează și extrage numele variabilei dintr-o declarație
    fn extract_var_declaration(line: &str) -> Option<String> {
        let trimmed = line.trim();

        if !trimmed.starts_with("let ") {
            return None;
        }

        let after_let = trimmed.strip_prefix("let ")?.trim_start();
        // Handle optional "mut"
        let after_mut = after_let
            .strip_prefix("mut ")
            .unwrap_or(after_let)
            .trim_start();

        // Extract variable name (până la : sau =)
        let var_name = if let Some(colon_pos) = after_mut.find(':') {
            &after_mut[..colon_pos]
        } else if let Some(eq_pos) = after_mut.find('=') {
            &after_mut[..eq_pos]
        } else {
            return None;
        };

        Some(var_name.trim().to_string())
    }

    /// Analizează contextul și determină un nume mai bun pentru variabilă
    fn suggest_better_name(
        var_name: &str,
        declaration_line: &str,
        usage_context: &[String],
    ) -> Option<String> {
        // Skip if already well-named
        if !var_name.starts_with("var") && !var_name.starts_with("slot_var") {
            return None;
        }

        // Analyze declaration for hints
        if let Some(name) = Self::analyze_declaration(declaration_line) {
            return Some(name);
        }

        // Analyze usage patterns
        if let Some(name) = Self::analyze_usage(usage_context) {
            return Some(name);
        }

        None
    }

    /// Analizează linia de declarație pentru hints despre nume
    fn analyze_declaration(line: &str) -> Option<String> {
        let line_lower = line.to_lowercase();

        // Vec::new() or Vec::<T>::new() → result_vec
        if line.contains("::new(") && (line.contains("Vec::") || line.contains("vec::")) {
            return Some("result_vec".to_string());
        }

        // .len() → count or length (more specific patterns)
        if line.contains(".len()") {
            if line_lower.contains("feed") {
                return Some("feed_count".to_string());
            }
            if line.contains("Vec::<") || line.contains("vec::<") {
                return Some("item_count".to_string());
            }
            return Some("count_val".to_string());
        }

        // String operations
        if line.contains("String::from_val") || line.contains("string::from_val") {
            return Some("string_val".to_string());
        }

        // Address operations
        if line.contains("Address::from_val") || line.contains("address_from_i64") {
            if line_lower.contains("from") && !line_lower.contains("from_val") {
                return Some("from_addr".to_string());
            } else if line_lower.contains("to") {
                return Some("to_addr".to_string());
            }
            return Some("addr_val".to_string());
        }

        // Error values
        if line.contains("Error(") {
            return Some("error_code".to_string());
        }

        // mload operations with specific patterns
        if line.contains("mload64!") {
            if line.contains("+ 72") || line.contains("+ 80") || line.contains("+ 56") {
                return Some("loaded_val".to_string());
            }
        }

        None
    }

    /// Analizează cum e folosită variabila în cod
    fn analyze_usage(usage_lines: &[String]) -> Option<String> {
        let usage_text = usage_lines.join(" ").to_lowercase();

        // Check for specific patterns
        if usage_text.contains(".push_back") {
            return Some("vec_builder".to_string());
        }

        if usage_text.contains(".require_auth") {
            return Some("authorized_addr".to_string());
        }

        if usage_text.contains("balance") {
            return Some("balance_val".to_string());
        }

        if usage_text.contains("amount") {
            return Some("amount_val".to_string());
        }

        None
    }

    /// Colectează usage context pentru o variabilă
    fn collect_usage_context(var_name: &str, body: &[String], start_idx: usize) -> Vec<String> {
        let mut context = Vec::new();

        // Collect next 5 lines that mention this variable
        let mut collected = 0;
        for line in &body[start_idx..] {
            if line.contains(var_name) {
                context.push(line.clone());
                collected += 1;
                if collected >= 5 {
                    break;
                }
            }
        }

        context
    }

    /// Înlocuiește toate aparițiile unei variabile cu noul nume
    /// Uses manual token-based replacement to avoid regex issues
    fn rename_variable(body: &[String], old_name: &str, new_name: &str) -> Vec<String> {
        body.iter()
            .map(|line| {
                let mut result = String::new();
                let mut chars = line.chars().peekable();
                let mut current_token = String::new();

                while let Some(ch) = chars.next() {
                    if ch.is_alphanumeric() || ch == '_' {
                        current_token.push(ch);
                    } else {
                        // End of token - check if it matches
                        if !current_token.is_empty() {
                            if current_token == old_name {
                                result.push_str(new_name);
                            } else {
                                result.push_str(&current_token);
                            }
                            current_token.clear();
                        }
                        result.push(ch);
                    }
                }

                // Handle last token
                if !current_token.is_empty() {
                    if current_token == old_name {
                        result.push_str(new_name);
                    } else {
                        result.push_str(&current_token);
                    }
                }

                result
            })
            .collect()
    }
}

impl Pattern for SmartVariableNamingPattern {
    fn name(&self) -> &'static str {
        "smart_variable_naming"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        // Check if already applied (prevent re-application in iterative mode)
        for line in &block.body {
            if line.contains("// Variables renamed by SmartVariableNamingPattern") {
                return None; // Already applied
            }
        }

        let mut changed = false;
        let mut current_body = block.body.clone();
        let mut rename_map: HashMap<String, String> = HashMap::new();

        // First pass: identify variables to rename
        for (idx, line) in block.body.iter().enumerate() {
            if let Some(var_name) = Self::extract_var_declaration(line) {
                let usage_context = Self::collect_usage_context(&var_name, &block.body, idx);

                if let Some(new_name) = Self::suggest_better_name(&var_name, line, &usage_context) {
                    // Avoid conflicts - check both keys and values
                    if !rename_map.contains_key(&new_name)
                        && !rename_map.values().any(|v| v == &new_name)
                        && new_name != var_name
                    {
                        rename_map.insert(var_name, new_name);
                    }
                }
            }
        }

        if rename_map.is_empty() {
            return None; // Nothing to rename
        }

        // Second pass: apply renames ONE AT A TIME to avoid conflicts
        for (old_name, new_name) in &rename_map {
            current_body = Self::rename_variable(&current_body, old_name, new_name);
            changed = true;
        }

        if !changed {
            return None;
        }

        // Add marker comment at the start to prevent re-application
        if !current_body.is_empty() {
            let first_line_indent = current_body[0].len() - current_body[0].trim_start().len();
            let marker = format!(
                "{}// Variables renamed by SmartVariableNamingPattern",
                " ".repeat(first_line_indent)
            );
            current_body.insert(0, marker);
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: current_body,
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
    fn test_extract_var_declaration() {
        assert_eq!(
            SmartVariableNamingPattern::extract_var_declaration("let mut var9: i64 = 0;"),
            Some("var9".to_string())
        );
        assert_eq!(
            SmartVariableNamingPattern::extract_var_declaration("let var10 = Vec::new();"),
            Some("var10".to_string())
        );
    }

    #[test]
    fn test_analyze_declaration() {
        assert_eq!(
            SmartVariableNamingPattern::analyze_declaration("let var9 = Vec::new(env);"),
            Some("result_vec".to_string())
        );
        assert_eq!(
            SmartVariableNamingPattern::analyze_declaration("let var10 = feed_ids.len() as i64;"),
            Some("feed_count".to_string())
        );
    }

    #[test]
    fn test_renames_variables() {
        let pattern = SmartVariableNamingPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let mut var9 = Vec::new(env);".to_string(),
                "    var9.push_back(item);".to_string(),
                "    return var9;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();

        // Should rename var9 to something better
        assert!(!result.body.iter().any(|l| l.contains("var9")));
        assert!(result
            .body
            .iter()
            .any(|l| l.contains("result_vec") || l.contains("vec_builder")));
    }
}
