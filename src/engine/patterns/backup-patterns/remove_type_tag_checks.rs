use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Removes type tag validation checks that are artifacts of WASM's type system.
/// These checks validate that values have the correct type tag in their lower 8 bits,
/// but they're implementation details that shouldn't appear in decompiled Rust code.
///
/// Removes patterns like:
/// ```rust
/// if c & 255 != 0 {
///     unreachable!();
/// }
/// ```
///
/// Also removes:
/// ```rust
/// if !(Type::try_from_val(env, &val_from_i64(x)).is_ok()) {
///     unreachable!();
/// }
/// ```
pub struct RemoveTypeTagChecksPattern;

impl RemoveTypeTagChecksPattern {
    pub fn new() -> Self {
        Self
    }

    fn is_type_tag_check(line: &str) -> bool {
        let trimmed = line.trim();

        // Pattern 1: if x & 255 != 0 {
        if trimmed.contains("& 255") && trimmed.contains("!= 0") && trimmed.starts_with("if ") {
            return true;
        }

        // Pattern 2: if !(Type::try_from_val(...).is_ok()) {
        if trimmed.starts_with("if !(")
            && trimmed.contains("::try_from_val(")
            && trimmed.contains(".is_ok())")
        {
            return true;
        }

        // Pattern 3: if x & 255 == 0 { (opposite check)
        if trimmed.contains("& 255") && trimmed.contains("== 0") && trimmed.starts_with("if ") {
            return true;
        }

        false
    }
}

impl Pattern for RemoveTypeTagChecksPattern {
    fn name(&self) -> &'static str {
        "RemoveTypeTagChecksPattern"
    }

    fn apply(&self, func: &FunctionBlock) -> Option<FunctionBlock> {
        let mut new_body = Vec::new();
        let mut modified = false;
        let mut skip_until_closing_brace = 0;

        for line in &func.body {
            if skip_until_closing_brace > 0 {
                let trimmed = line.trim();
                if trimmed == "}" {
                    skip_until_closing_brace -= 1;
                    if skip_until_closing_brace == 0 {
                        modified = true;
                        continue; // Skip the closing brace
                    }
                } else if trimmed.starts_with("if ") || trimmed == "{" {
                    // Nested if or block
                    if trimmed.ends_with("{") {
                        skip_until_closing_brace += 1;
                    }
                }
                continue; // Skip lines inside the if block
            }

            if Self::is_type_tag_check(line) {
                // Check if the next line(s) are unreachable!() and closing brace
                skip_until_closing_brace = 1;
                continue;
            }

            new_body.push(line.clone());
        }

        if !modified {
            return None;
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
    fn test_remove_type_tag_check() {
        let func = FunctionBlock {
            header: "pub fn increment(env: Env) -> u32 {".to_string(),
            body: vec![
                "    let c: i64 = 0;".to_string(),
                "    if c & 255 != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    return c;".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "increment".to_string(),
        };

        let pattern = RemoveTypeTagChecksPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.body.len(), 2);
        assert_eq!(result.body[0], "    let c: i64 = 0;");
        assert_eq!(result.body[1], "    return c;");
    }

    #[test]
    fn test_remove_try_from_val_check() {
        let func = FunctionBlock {
            header: "pub fn test(env: Env, admin: Address) {".to_string(),
            body: vec![
                "    if !(Address::try_from_val(env, &val_from_i64(admin)).is_ok()) {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    // do something".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "test".to_string(),
        };

        let pattern = RemoveTypeTagChecksPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.body.len(), 1);
        assert_eq!(result.body[0], "    // do something");
    }

    #[test]
    fn test_no_type_tag_check() {
        let func = FunctionBlock {
            header: "pub fn test(env: Env) -> u32 {".to_string(),
            body: vec![
                "    let x = 5;".to_string(),
                "    if x > 0 {".to_string(),
                "        return x;".to_string(),
                "    }".to_string(),
                "    0".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "test".to_string(),
        };

        let pattern = RemoveTypeTagChecksPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_none());
    }
}
