use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct CompoundAssignCleanupPattern;

impl CompoundAssignCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for CompoundAssignCleanupPattern {
    fn name(&self) -> &'static str {
        "compound_assign_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            let rewritten = rewrite_line(line);
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

fn rewrite_line(line: &str) -> String {
    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    let t = line.trim();
    let Some((lhs, rhs)) = t.split_once(" = ") else {
        return line.to_string();
    };
    let lhs = lhs.trim();
    for op in ["+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="] {
        let prefix = format!("{lhs} {op} ");
        if let Some(rest) = rhs.strip_prefix(&prefix) {
            let rest = rest.trim_end_matches(';').trim();
            return format!("{indent}{lhs} {op} {rest};");
        }
    }
    line.to_string()
}

