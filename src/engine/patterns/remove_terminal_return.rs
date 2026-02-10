use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct RemoveTerminalReturnPattern;

impl RemoveTerminalReturnPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveTerminalReturnPattern {
    fn name(&self) -> &'static str {
        "remove_terminal_return"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() || block.header.contains("->") {
            return None;
        }

        let mut body = block.body.clone();
        let last_idx = body.iter().rposition(|l| !l.trim().is_empty())?;
        if body[last_idx].trim() != "return;" {
            return None;
        }
        body.remove(last_idx);

        Some(FunctionBlock {
            header: block.header.clone(),
            body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

