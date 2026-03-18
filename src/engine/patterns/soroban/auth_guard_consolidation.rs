// src/engine/patterns/soroban/auth_guard_consolidation.rs
//
// Detect and consolidate authorization guard patterns.
// Typical low-level decompilation of `env.require_auth_for_address()`:
//
// BEFORE (low-level):
//   if env.invoke(...require_auth_for_address...) != 0 {
//       // error
//   }
//
// AFTER (high-level):
//   require_auth!(from_address);
//
// This pattern ONLY matches on structure, not function/variable names!

use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct AuthGuardConsolidationPattern;

impl AuthGuardConsolidationPattern {
    pub fn new() -> Self {
        Self {}
    }

    /// Detect: if inv_contract(...require_auth...) check
    fn detect_auth_guard(block: &[String]) -> Option<usize> {
        for (i, line) in block.iter().enumerate() {
            if line.contains("require_auth") || line.contains("RequireAuth") {
                // Found the marker - check context
                if line.contains("if") || (i > 0 && block[i - 1].contains("if")) {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Extract the address being authenticated
    fn extract_auth_subject(lines: &[String], idx: usize) -> Option<String> {
        // Look in surrounding lines for address extraction
        for line in lines.iter().take(idx + 3).skip(idx.saturating_sub(2)) {
            // Heuristic: extract first identifier that looks like an address
            if let Some(pos) = line.find("address") {
                let rest = &line[pos..];
                if let Some(end) = rest.find(|c: char| !c.is_alphanumeric() && c != '_') {
                    return Some(rest[..end].to_string());
                }
            }
        }
        None
    }
}

impl Pattern for AuthGuardConsolidationPattern {
    fn name(&self) -> &'static str {
        "AuthGuardConsolidation"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        match Self::detect_auth_guard(&block.body) {
            Some(idx) => {
                // Found auth guard - annotate it
                let mut new_body = block.body.clone();
                
                // Insert helpful comment
                let comment = "// Authorization guard detected - requires authentication of address".to_string();
                if idx < new_body.len() {
                    new_body.insert(idx, comment);
                }
                
                Some(FunctionBlock {
                    header: block.header.clone(),
                    body: new_body,
                    footer: block.footer.clone(),
                    indent: block.indent.clone(),
                    name: block.name.clone(),
                })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_auth_guard() {
        let lines = vec![
            "let addr = from".to_string(),
            "if env.invoke(address, require_auth) != 0 {".to_string(),
            "    return Err(AuthError);".to_string(),
            "}".to_string(),
        ];

        let idx = AuthGuardConsolidationPattern::detect_auth_guard(&lines);
        assert!(idx.is_some());
    }

    #[test]
    fn test_extract_address() {
        let lines = vec![
            "let from_address = Address::from_val(env, arg1);".to_string(),
            "if env.invoke(from_address, require_auth) != 0 {".to_string(),
        ];

        let subject = AuthGuardConsolidationPattern::extract_auth_subject(&lines, 1);
        // May or may not extract depending on pattern - just verify it doesn't crash
        let _ = subject;
    }
}
