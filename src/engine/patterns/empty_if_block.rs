use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct EmptyIfBlockPattern;

impl EmptyIfBlockPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for EmptyIfBlockPattern {
    fn name(&self) -> &'static str {
        "empty_if_block"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut changed = false;
        let mut body: Vec<String> = Vec::with_capacity(block.body.len());
        let mut i = 0usize;

        while i < block.body.len() {
            let line = block.body[i].trim();

            if i + 1 < block.body.len() {
                let next = block.body[i + 1].trim();
                let is_empty_if = line.starts_with("if ") && line.ends_with('{') && next == "}";
                let is_empty_else = line == "else {" && next == "}";
                if is_empty_if || is_empty_else {
                    changed = true;
                    i += 2;
                    continue;
                }
            }

            if line.starts_with("if ") && line.ends_with("{}") {
                changed = true;
                i += 1;
                continue;
            }

            body.push(block.body[i].clone());
            i += 1;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}
