use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ShiftByZeroCleanupPattern;

impl ShiftByZeroCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ShiftByZeroCleanupPattern {
    fn name(&self) -> &'static str {
        "shift_by_zero_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out: Vec<String> = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let mut updated = line.clone();
            let before = updated.clone();
            updated = updated.replace(".wrapping_shl(0 as u32)", "");
            updated = updated.replace(".wrapping_shr(0 as u32)", "");
            updated = updated.replace(".wrapping_shl(0)", "");
            updated = updated.replace(".wrapping_shr(0)", "");
            if updated != before {
                changed = true;
            }
            out.push(updated);
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: out,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}
