// src/engine/patterns/function_classifier.rs
//
// Classification system for Soroban contract functions based on:
// 1. Fingerprint (CFG structure)
// 2. SDK calls (storage, auth, invoke)
// 3. Data flow patterns
//
// NO hardcoding by function name!

use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionCategory {
    /// Transfer token to another address
    TokenTransfer,
    /// Query token balance or metadata
    TokenQuery,
    /// Mint new tokens
    TokenMint,
    /// Burn tokens
    TokenBurn,
    /// Contract initialization
    Initialization,
    /// Authorization check
    Authorization,
    /// Storage get/set operations
    DataAccess,
    /// Cross-contract invocation
    ContractInvoke,
    /// Pure computation (math, encoding)
    Computation,
    /// Internal helper/utility
    Helper,
    /// Unknown category
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SDKCallPattern {
    pub storage_reads: u32,
    pub storage_writes: u32,
    pub has_require_auth: bool,
    pub has_invoke_contract: bool,
    pub has_emit_event: bool,
    pub return_value_type: ReturnType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnType {
    Void,
    SingleValue,
    Option,
    Result,
    Complex,
}

impl FunctionClassifier {
    pub fn new() -> Self {
        Self {}
    }

    /// Classify a function based on its code structure
    pub fn classify(&self, block: &FunctionBlock) -> FunctionCategory {
        let patterns = self.extract_patterns(block);

        // Classification logic (no name-based assumptions)
        match &patterns {
            // Authorization check: if require_auth present
            p if p.has_require_auth && p.storage_reads > 0 => {
                FunctionCategory::Authorization
            }

            // Initialization: mass storage writes, no reads, returns void
            p if p.storage_writes > 2 && p.storage_reads == 0 && p.return_value_type == ReturnType::Void => {
                FunctionCategory::Initialization
            }

            // Cross-contract call
            p if p.has_invoke_contract => FunctionCategory::ContractInvoke,

            // Token transfer: write to 2 storage keys (from + to balance) + emit event
            p if p.storage_writes >= 2 && p.has_emit_event && !p.has_require_auth => {
                // Additional heuristic: check if value flows through parameters
                if block.body.iter().any(|line| {
                    line.contains("balance") || line.contains("amount")
                }) {
                    FunctionCategory::TokenTransfer
                } else {
                    FunctionCategory::Computation
                }
            }

            // Mint: write to balance storage, emit event
            p if p.storage_writes >= 1 && p.has_emit_event && !p.storage_reads.gt(&0) => {
                FunctionCategory::TokenMint
            }

            // Query: read storage, single return value, no writes
            p if p.storage_reads > 0 && p.storage_writes == 0 && p.return_value_type == ReturnType::SingleValue => {
                FunctionCategory::TokenQuery
            }

            // Storage access: pure get/set
            p if (p.storage_reads > 0 || p.storage_writes > 0)
                && !p.has_require_auth
                && !p.has_invoke_contract => {
                FunctionCategory::DataAccess
            }

            // Default
            _ => FunctionCategory::Unknown,
        }
    }

    /// Extract SDK call patterns from function body
    fn extract_patterns(&self, block: &FunctionBlock) -> SDKCallPattern {
        let body_text = block.body.join("\n");
        let signature_text = block.header.replace('\n', " ");

        let storage_reads = count_pattern(&body_text, r"\.get\(|\.get_opt\(|\.get_unchecked\(");
        let storage_writes = count_pattern(&body_text, r"\.set\(|\.set_and_return\(");
        let has_require_auth = body_text.contains("require_auth");
        let has_invoke_contract = body_text.contains("invoke_contract");
        let has_emit_event = body_text.contains("event::emit");

        let return_type = if signature_text.contains("-> Result<") {
            ReturnType::Result
        } else if signature_text.contains("-> Option<") {
            ReturnType::Option
        } else if !signature_text.contains("->") {
            ReturnType::Void
        } else if block.body.iter().any(|l| l.trim().starts_with("return")) || block.body.len() == 1 {
            ReturnType::SingleValue
        } else {
            ReturnType::Complex
        };

        SDKCallPattern {
            storage_reads,
            storage_writes,
            has_require_auth,
            has_invoke_contract,
            has_emit_event,
            return_value_type: return_type,
        }
    }
}

pub struct FunctionClassifier {}

/// Simple pattern counter (regex-like but minimal)
fn count_pattern(text: &str, pattern: &str) -> u32 {
    Regex::new(pattern)
        .expect("valid classifier regex")
        .find_iter(text)
        .count() as u32
}

/// Pattern to apply function classification during engine pass
pub struct FunctionClassificationPattern;

impl Pattern for FunctionClassificationPattern {
    fn name(&self) -> &'static str {
        "FunctionClassification"
    }

    fn apply(&self, _block: &FunctionBlock) -> Option<FunctionBlock> {
        // This pattern doesn't modify code - it's informational.
        // In a real implementation, we'd:
        // 1. Classify the function
        // 2. Emit a comment like "// Category: TokenTransfer"
        // 3. Or store classification in metadata for later use
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_token_transfer() {
        let classifier = FunctionClassifier::new();
        let block = FunctionBlock {
            name: "transfer".to_string(),
            header: "fn transfer(&mut self, from: Address, to: Address, amount: i128) {".to_string(),
            body: vec![
                "let from_balance = self.storage.get(&key(&from)).unwrap_or(0);".to_string(),
                "let to_balance = self.storage.get(&key(&to)).unwrap_or(0);".to_string(),
                "self.storage.set(&key(&from), &(from_balance - amount));".to_string(),
                "self.storage.set(&key(&to), &(to_balance + amount));".to_string(),
                "event::emit(TransferEvent { from, to, amount });".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
        };

        let category = classifier.classify(&block);
        assert_eq!(category, FunctionCategory::TokenTransfer);
    }

    #[test]
    fn test_classify_balance_query() {
        let classifier = FunctionClassifier::new();
        let block = FunctionBlock {
            name: "balance_of".to_string(),
            header: "fn balance_of(&self, account: Address) -> i128 {".to_string(),
            body: vec![
                "self.storage.get(&key(&account)).unwrap_or(0)".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
        };

        let category = classifier.classify(&block);
        assert_eq!(category, FunctionCategory::TokenQuery);
    }
}
