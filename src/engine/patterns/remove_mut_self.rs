use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Removes `&mut self` from function signatures since Soroban contracts don't use mutable self.
/// This is a WASM calling convention artifact that shouldn't appear in decompiled Rust code.
///
/// Transforms:
/// ```rust
/// pub fn hello(&mut self, env: Env, to: Symbol) -> Vec<Symbol> {
/// ```
///
/// Into:
/// ```rust
/// pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
/// ```
pub struct RemoveMutSelfPattern;

impl RemoveMutSelfPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveMutSelfPattern {
    fn name(&self) -> &'static str {
        "RemoveMutSelfPattern"
    }

    fn apply(&self, func: &FunctionBlock) -> Option<FunctionBlock> {
        let header = &func.header;

        // Check if header contains &mut self
        if !header.contains("&mut self") {
            return None;
        }

        // Remove &mut self parameter from the signature
        let mut new_header = header.clone();

        // Pattern 1: "&mut self, " - remove including comma and space
        if new_header.contains("&mut self, ") {
            new_header = new_header.replace("&mut self, ", "");
        }
        // Pattern 2: "&mut self," - remove including comma (no space)
        else if new_header.contains("&mut self,") {
            new_header = new_header.replace("&mut self,", "");
        }
        // Pattern 3: "(&mut self)" - only parameter, remove entirely
        else if new_header.contains("(&mut self)") {
            new_header = new_header.replace("(&mut self)", "()");
        }

        if new_header == *header {
            return None;
        }

        Some(FunctionBlock {
            header: new_header,
            body: func.body.clone(),
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
    fn test_remove_mut_self_with_params() {
        let func = FunctionBlock {
            header: "pub fn hello(&mut self, env: Env, to: Symbol) -> Vec<Symbol> {".to_string(),
            body: vec!["    return vec![&env, Symbol::new(env, \"Hello\"), to];".to_string()],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = RemoveMutSelfPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.header, "pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {");
    }

    #[test]
    fn test_remove_mut_self_only_param() {
        let func = FunctionBlock {
            header: "pub fn get_count(&mut self) -> u32 {".to_string(),
            body: vec!["    return 42;".to_string()],
            footer: "}".to_string(),
            indent: vec![],
            name: "get_count".to_string(),
        };

        let pattern = RemoveMutSelfPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.header, "pub fn get_count() -> u32 {");
    }

    #[test]
    fn test_no_mut_self() {
        let func = FunctionBlock {
            header: "pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {".to_string(),
            body: vec!["    vec![&env, Symbol::new(env, \"Hello\"), to]".to_string()],
            footer: "}".to_string(),
            indent: vec![],
            name: "hello".to_string(),
        };

        let pattern = RemoveMutSelfPattern::new();
        let result = pattern.apply(&func);

        assert!(result.is_none());
    }
}
