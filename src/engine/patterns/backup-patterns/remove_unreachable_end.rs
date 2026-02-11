use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Removes unreachable code at the end of functions.
/// WASM often has unreachable instructions after returns, which shouldn't appear in clean Rust code.
///
/// Removes patterns like:
/// - `self.call_unreachable(env);`
/// - `unreachable!();`
/// - `self.unreachable_stub(env);`
///
/// When they appear at the end of a function after a return statement.
pub struct RemoveUnreachableEndPattern;

impl RemoveUnreachableEndPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveUnreachableEndPattern {
    fn name(&self) -> &'static str {
        "RemoveUnreachableEndPattern"
    }

    fn apply(&self, func: &FunctionBlock) -> Option<FunctionBlock> {
        if func.body.is_empty() {
            return None;
        }

        let mut new_body = func.body.clone();
        let mut modified = false;

        // Remove from the end backwards
        while !new_body.is_empty() {
            let last_line = new_body.last().unwrap().trim();

            if last_line == "unreachable!();"
                || last_line == "unreachable!()"
                || last_line.contains("call_unreachable(")
                || last_line.contains("unreachable_stub(") {
                new_body.pop();
                modified = true;
            } else if last_line.is_empty() || last_line.starts_with("//") {
                // Skip empty lines and comments at the end
                new_body.pop();
                modified = true;
            } else {
                break;
            }
        }

        if !modified {
            return None;
        }

        // Also check if there's a pattern like:
        // }
        // self.call_unreachable(env);
        // Before the last closing brace

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
    fn test_remove_unreachable_at_end() {
        let func = FunctionBlock {
            header: "pub fn increment(env: Env) -> u32 {".to_string(),
            body: vec![
                "    let count = 5;".to_string(),
                "    return count;".to_string(),
                "    }".to_string(),
                "    self.call_unreachable(env);".to_string(),
                "    unreachable!();".to_string(),
            ],
            footer: "}".to_string(),
            indent: vec![],
            name: "increment".to_string(),
        };

        let pattern = RemoveUnreachableEndPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.body.len(), 3);
        assert_eq!(result.body[2], "    }");
    }

    #[test]
    fn test_no_unreachable() {
        let func = FunctionBlock {
            header: "pub fn hello(env: Env) -> Vec<Symbol> {".to_string(),
            body: vec!["    vec![&env, Symbol::new(env, \"Hello\")]".to_string()],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = RemoveUnreachableEndPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_none());
    }
}
