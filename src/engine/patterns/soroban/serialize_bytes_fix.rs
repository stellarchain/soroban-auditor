use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct SerializeBytesFixPattern;

impl SerializeBytesFixPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for SerializeBytesFixPattern {
    fn name(&self) -> &'static str {
        "serialize_bytes_fix"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            let mut rewritten = line.replace(
                ").into()) /* TODO: serialize_to_bytes */",
                ").into_val(env))",
            );
            rewritten = rewritten.replace(
                ").into())/* TODO: serialize_to_bytes */",
                ").into_val(env))",
            );
            if rewritten != *line {
                changed = true;
            }
            out.push(rewritten);
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

