use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

/// Pattern pentru eliminarea conversiilor val_from_i64/val_to_i64 inutile
///
/// Detectează și elimină:
/// - `val_to_i64(x.into_val(&env))` → poate fi înlocuit cu tracking direct
/// - `T::from_val(env, &val_from_i64(v))` → v reprezintă deja un T
/// - Conversion chains redundante
pub struct ConversionEliminationPattern;

impl ConversionEliminationPattern {
    pub fn new() -> Self {
        Self
    }

    /// Detectează și elimină conversii redundante: val_to_i64(val_from_i64(x)) → x
    fn simplify_double_conversion(line: &str) -> Option<String> {
        // Pattern: val_to_i64(val_from_i64(var))
        if !line.contains("val_to_i64(val_from_i64(") {
            return None;
        }

        // Extract the variable name
        let start = line.find("val_to_i64(val_from_i64(")?;
        let after = &line[start + "val_to_i64(val_from_i64(".len()..];
        let end = after.find("))")?;
        let var_name = after[..end].trim();

        // Reconstruct line with simplified expression
        let before = &line[..start];
        let after_pattern = &after[end + 2..]; // Skip past ))

        Some(format!("{}{}{}", before, var_name, after_pattern))
    }

    /// Detectează pattern-ul: val_to_i64(something.into_val(&env))
    /// Returns: Some(variable_name) dacă pattern-ul este detectat
    fn extract_into_val_conversion(line: &str) -> Option<String> {
        // Pattern: val_to_i64(var.into_val(&env))
        if !line.contains("val_to_i64(") || !line.contains(".into_val(&env))") {
            return None;
        }

        // Extract: val_to_i64(XXXX.into_val(&env))
        let start = line.find("val_to_i64(")?;
        let after_start = &line[start + "val_to_i64(".len()..];
        let end = after_start.find(".into_val(&env))")?;
        let var_name = after_start[..end].trim();

        Some(var_name.to_string())
    }

    fn extract_type_name_before_anchor(line: &str, anchor: &str) -> Option<String> {
        let anchor_pos = line.find(anchor)?;
        let before = &line[..anchor_pos];
        let mut start = 0usize;
        for (idx, ch) in before.char_indices().rev() {
            if matches!(
                ch,
                '=' | '(' | ')' | '{' | '}' | '[' | ']' | '|' | '&' | '!' | '+' | '-'
                    | '*' | '/' | ';'
            ) {
                start = idx + 1;
                break;
            }
        }
        let candidate = before[start..].trim();
        if candidate.is_empty() {
            return None;
        }
        Some(Self::normalize_type_name(candidate))
    }

    /// Detectează pattern-ul: T::from_val(env, &val_from_i64(var))
    /// Returns: Some((type_name, var_name))
    fn extract_from_val_conversion(line: &str) -> Option<(String, String)> {
        // Pattern: Type::from_val(env, &val_from_i64(var))
        if !line.contains("::from_val(") || !line.contains("&val_from_i64(") {
            return None;
        }

        let type_name = Self::extract_type_name_before_anchor(line, "::from_val(")?;

        // Extract var: &val_from_i64(XXXX)
        let val_from_start = line.find("&val_from_i64(")?;
        let after_val_from = &line[val_from_start + "&val_from_i64(".len()..];
        let paren_end = after_val_from.find(')')?;
        let var_name = after_val_from[..paren_end].trim().to_string();

        Some((type_name, var_name))
    }

    /// Detectează type checks: (Type::try_from_val(env, &val_from_i64(var)).is_ok())
    fn extract_type_check(line: &str) -> Option<(String, String)> {
        if !line.contains("::try_from_val(") || !line.contains("&val_from_i64(") {
            return None;
        }

        let type_name = Self::extract_type_name_before_anchor(line, "::try_from_val(")?;

        // Extract var
        let val_from_start = line.find("&val_from_i64(")?;
        let after_val_from = &line[val_from_start + "&val_from_i64(".len()..];
        let paren_end = after_val_from.find(')')?;
        let var_name = after_val_from[..paren_end].trim();

        Some((type_name, var_name.to_string()))
    }

    fn normalize_type_name(raw: &str) -> String {
        let mut s = raw.trim().to_string();
        while matches!(
            s.chars().next(),
            Some('(' | '!' | '&' | '[' | '{' | '|' | '+' | '-' | '*')
        ) {
            s.remove(0);
        }
        while matches!(
            s.chars().last(),
            Some(')' | ']' | '}' | ',' | ';' | ':' | '|' | '&' | '!')
        ) {
            s.pop();
        }
        // Drop trailing unmatched angle brackets caused by partial parsing.
        let mut balance = 0i32;
        let mut cleaned = String::new();
        for ch in s.chars() {
            if ch == '<' {
                balance += 1;
                cleaned.push(ch);
            } else if ch == '>' {
                if balance > 0 {
                    balance -= 1;
                    cleaned.push(ch);
                }
            } else {
                cleaned.push(ch);
            }
        }
        let mut out = cleaned.trim().to_string();
        if !out.contains('<') {
            while out.ends_with('>') {
                out.pop();
            }
        }
        out
    }
}

impl Pattern for ConversionEliminationPattern {
    fn name(&self) -> &'static str {
        "conversion_elimination"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut var_types: HashMap<String, String> = HashMap::new();
        let mut new_body: Vec<String> = Vec::new();

        for line in &block.body {
            let trimmed = line.trim();
            let indent = line.len() - line.trim_start().len();
            let indent_str = " ".repeat(indent);

            // Skip lines that already have TODO comments from previous iterations
            if trimmed.starts_with("// TODO:") {
                new_body.push(line.clone());
                continue;
            }

            // QUICK FIX: Eliminate obvious double conversions: val_to_i64(val_from_i64(x)) → x
            if let Some(simplified) = Self::simplify_double_conversion(line) {
                changed = true;
                new_body.push(simplified);
                continue;
            }

            // Track variable types from assignments with conversions
            if trimmed.starts_with("let ") && trimmed.contains(" = ") {
                // Extract: let var = T::from_val(env, &val_from_i64(other_var))
                if let Some((type_name, source_var)) = Self::extract_from_val_conversion(trimmed) {
                    // Extract destination var name
                    let after_let = trimmed.strip_prefix("let ").unwrap().trim_start();
                    let after_let = after_let
                        .strip_prefix("mut ")
                        .unwrap_or(after_let)
                        .trim_start();
                    if let Some(eq_pos) = after_let.find(" = ") {
                        let _dest_var = after_let[..eq_pos].trim();
                        var_types.insert(source_var.clone(), type_name.clone());
                        new_body.push(line.clone());
                        continue;
                    }
                }

                // Extract: let res_val = ... val_to_i64(x.into_val(&env))
                if let Some(source_var) = Self::extract_into_val_conversion(trimmed) {
                    let _ = source_var;
                    new_body.push(line.clone());
                    continue;
                }
            }

            // Simplify type checks: if (Type::try_from_val(...).is_ok()) as i32 != 0
            if trimmed.contains("::try_from_val(") && trimmed.contains(".is_ok()") {
                if let Some((type_name, var_name)) = Self::extract_type_check(trimmed) {
                    var_types.insert(var_name.clone(), type_name.clone());
                    new_body.push(line.clone());
                    continue;
                }
            }

            // Keep original line
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
    fn test_extract_into_val_conversion() {
        let line = "let res = val_to_i64(feed_ids.into_val(&env));";
        let result = ConversionEliminationPattern::extract_into_val_conversion(line);
        assert_eq!(result, Some("feed_ids".to_string()));
    }

    #[test]
    fn test_extract_from_val_conversion() {
        let line = "let var = Vec::<Val>::from_val(env, &val_from_i64(feed_ids));";
        let result = ConversionEliminationPattern::extract_from_val_conversion(line);
        assert_eq!(
            result,
            Some(("Vec::<Val>".to_string(), "feed_ids".to_string()))
        );
    }

    #[test]
    fn test_detects_conversions() {
        let pattern = ConversionEliminationPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let res_val = val_to_i64(data.into_val(&env));".to_string(),
                "    let result = Result::from_val(&env, &val_from_i64(res_val));".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block);
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.body.iter().any(|l| l.contains("TODO")));
    }
}
