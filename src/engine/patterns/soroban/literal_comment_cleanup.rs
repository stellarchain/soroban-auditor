use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LiteralCommentCleanupPattern;

impl LiteralCommentCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LiteralCommentCleanupPattern {
    fn name(&self) -> &'static str {
        "literal_comment_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let mut updated = line.clone();
            let before = updated.clone();
            updated = updated.replace("0 /* False */", "0");
            updated = updated.replace("1 /* True */", "1");
            updated = updated.replace("0 /* Void */", "0");
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
