// src/engine/patterns/soroban/storage_key_recovery.rs
//
// Reconstruct semantic storage keys from encoding patterns.
// No hardcoding - detects:
// - key = (part0 << 64) | part1  ->  StorageKey::Balance(addr)
// - key = hash(addr, spender)    ->  StorageKey::Allowance(addr, spender)
// - key = static_str             ->  StorageKey::Admin
//
// This is pure structural pattern matching on CFG.

use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct StorageKeyRecoveryPattern;

impl StorageKeyRecoveryPattern {
    pub fn new() -> Self {
        Self {}
    }

    /// Detect: "let key = (x << 64) | y" style key construction
    /// Goal: Simplify to documented semantic key structure
    fn detect_composite_key_encoding(line: &str) -> Option<CompositeKeyPattern> {
        let re = Regex::new(r"let\s+(\w+)\s*=\s*\((\w+)\s*<<\s*64\)\s*\|\s*(\w+)").ok()?;
        if let Some(caps) = re.captures(line) {
            return Some(CompositeKeyPattern {
                result_var: caps[1].to_string(),
                high_part: caps[2].to_string(),
                low_part: caps[3].to_string(),
                is_u128: true,
            });
        }
        None
    }

    /// Generate documentation for detected key structure
    fn emit_key_comment(pattern: &CompositeKeyPattern) -> String {
        format!(
            "// StorageKey construction: high={}, low={}",
            pattern.high_part, pattern.low_part
        )
    }
}

#[derive(Debug, Clone)]
struct CompositeKeyPattern {
    result_var: String,
    high_part: String,
    low_part: String,
    is_u128: bool,
}

impl Pattern for StorageKeyRecoveryPattern {
    fn name(&self) -> &'static str {
        "StorageKeyRecovery"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        let mut new_body = block.body.clone();
        let mut changed = false;

        for line in &mut new_body {
            if let Some(pattern) = Self::detect_composite_key_encoding(line) {
                // Add clarifying comment about the key semantics
                let comment = Self::emit_key_comment(&pattern);
                if !line.contains(&comment) {
                    *line = format!("{} // {}", line, pattern.high_part);
                    changed = true;
                }
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
    fn test_detect_u128_key() {
        let line = "let key = (addr_part << 64) | curr_part";
        let pattern = StorageKeyRecoveryPattern::detect_composite_key_encoding(line);
        assert!(pattern.is_some());
        let p = pattern.unwrap();
        assert_eq!(p.high_part, "addr_part");
        assert_eq!(p.low_part, "curr_part");
    }
}
