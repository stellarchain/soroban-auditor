use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct VecBuilderAssignmentPattern;

impl VecBuilderAssignmentPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for VecBuilderAssignmentPattern {
    fn name(&self) -> &'static str {
        "vec_builder_assignment"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            let trimmed = line.trim();
            if trimmed.starts_with("{ let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder));")
                && trimmed.contains("v.push_back(val_from_i64(")
                && trimmed.ends_with("};")
            {
                let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                out.push(format!("{indent}vec_builder = {trimmed};"));
                changed = true;
            } else {
                out.push(line.clone());
            }
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
