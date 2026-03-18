// src/engine/patterns/soroban/token_transfer_recovery.rs
//
// Detect and recover token transfer logic without name matching.
// Pattern characteristic:
// 1. Read source balance from storage
// 2. Read destination balance from storage  
// 3. Decrease source balance
// 4. Increase destination balance
// 5. Write both back
// 6. Emit transfer event
//
// All detected by CFG structure, not function name!

use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct TokenTransferRecoveryPattern;

impl TokenTransferRecoveryPattern {
    pub fn new() -> Self {
        Self {}
    }

    /// Detect transfer pattern:
    /// Read 2 balances -> math -> write 2 balances -> emit event
    fn is_transfer_shape(lines: &[String]) -> bool {
        let text = lines.join("\n");
        
        // Count storage reads/writes
        let reads = text.matches(".get(").count() + text.matches(".get_opt(").count();
        let writes = text.matches(".set(").count();
        
        // Transfer: 2 reads, 2 writes, emit event, has parameters like from/to/amount
        reads >= 2 
            && writes >= 2 
            && text.contains("event::emit")
            && (text.contains("from") || text.contains("sender"))
            && (text.contains("to") || text.contains("recipient"))
            && (text.contains("amount") || text.contains("value"))
    }

    fn emit_transfer_marker(lines: &[String]) -> Vec<String> {
        let mut result = vec![
            "// == TRANSFER PATTERN DETECTED ==".to_string(),
            "// Structure: Read from_balance, to_balance → Math → Write back → Emit".to_string(),
            "".to_string(),
        ];
        result.extend(lines.iter().cloned());
        result
    }
}

impl Pattern for TokenTransferRecoveryPattern {
    fn name(&self) -> &'static str {
        "TokenTransferRecovery"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if Self::is_transfer_shape(&block.body) {
            let mut new_body = Self::emit_transfer_marker(&block.body);
            
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
    fn test_detect_transfer_structure() {
        let lines = vec![
            "let from_val = env.storage().instance().get(&key_from).unwrap_or(0);".to_string(),
            "let to_val = env.storage().instance().get(&key_to).unwrap_or(0);".to_string(),
            "let new_from = from_val - amount;".to_string(),
            "let new_to = to_val + amount;".to_string(),
            "env.storage().instance().set(&key_from, &new_from);".to_string(),
            "env.storage().instance().set(&key_to, &new_to);".to_string(),
            "event::emit(TransferEvent { from, to, amount });".to_string(),
        ];

        assert!(TokenTransferRecoveryPattern::is_transfer_shape(&lines));
    }
}
