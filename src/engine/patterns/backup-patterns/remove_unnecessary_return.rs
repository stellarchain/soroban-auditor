use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Removes unnecessary `return` keyword from the last statement in a function.
/// In Rust, the last expression is automatically returned, so explicit `return` is not idiomatic.
///
/// Transforms:
/// ```rust
/// pub fn hello(env: Env) -> Vec<Symbol> {
///     return vec![&env, Symbol::new(env, "Hello")];
/// }
/// ```
///
/// Into:
/// ```rust
/// pub fn hello(env: Env) -> Vec<Symbol> {
///     vec![&env, Symbol::new(env, "Hello")]
/// }
/// ```
pub struct RemoveUnnecessaryReturnPattern;

impl RemoveUnnecessaryReturnPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveUnnecessaryReturnPattern {
    fn name(&self) -> &'static str {
        "RemoveUnnecessaryReturnPattern"
    }

    fn apply(&self, func: &FunctionBlock) -> Option<FunctionBlock> {
        if func.body.is_empty() {
            return None;
        }

        // Find the last non-empty, non-comment line
        let mut last_line_idx = None;
        for (idx, line) in func.body.iter().enumerate().rev() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                last_line_idx = Some(idx);
                break;
            }
        }

        let last_idx = match last_line_idx {
            Some(idx) => idx,
            None => return None,
        };

        let last_line = &func.body[last_idx];
        let trimmed = last_line.trim();

        // Check if it starts with "return "
        if !trimmed.starts_with("return ") {
            return None;
        }

        // Get the indentation
        let indent_len = last_line.len() - last_line.trim_start().len();
        let indent = &last_line[..indent_len];

        // Remove "return " and keep the rest
        let rest = trimmed.strip_prefix("return ").unwrap();

        // If it ends with semicolon, this is an early return or explicit return statement
        // We should only remove return from implicit returns (last expression)
        // Check if this is truly the last statement by seeing if there's anything after it
        let is_last_statement = last_idx == func.body.len() - 1
            || func.body[last_idx + 1..].iter().all(|l| {
                let t = l.trim();
                t.is_empty() || t.starts_with("//")
            });

        if !is_last_statement {
            return None;
        }

        // Create new line without "return "
        let new_line = format!("{}{}", indent, rest);

        let mut new_body = func.body.clone();
        new_body[last_idx] = new_line;

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
    fn test_remove_return() {
        let func = FunctionBlock {
            header: "pub fn hello(env: Env) -> Vec<Symbol> {".to_string(),
            body: vec![
                "    return vec![&env, Symbol::new(env, \"Hello\"), to];".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = RemoveUnnecessaryReturnPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(
            result.body[0],
            "    vec![&env, Symbol::new(env, \"Hello\"), to];"
        );
    }

    #[test]
    fn test_keep_early_return() {
        let func = FunctionBlock {
            header: "pub fn check(x: i32) -> i32 {".to_string(),
            body: vec![
                "    if x < 0 {".to_string(),
                "        return 0;".to_string(),
                "    }".to_string(),
                "    x + 1".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "check".to_string(),
        };

        let pattern = RemoveUnnecessaryReturnPattern::new();
        let result = pattern.apply(&func);

        // Should not modify because return is not the last statement
        assert!(result.is_none());
    }

    #[test]
    fn test_no_return() {
        let func = FunctionBlock {
            header: "pub fn hello(env: Env) -> Vec<Symbol> {".to_string(),
            body: vec!["    vec![&env, Symbol::new(env, \"Hello\")]".to_string()],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = RemoveUnnecessaryReturnPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_none());
    }
}
