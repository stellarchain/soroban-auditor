use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ElseCompactionPattern;

impl ElseCompactionPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ElseCompactionPattern {
    fn name(&self) -> &'static str {
        "else_compaction"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut out: Vec<String> = Vec::with_capacity(block.body.len());
        let mut changed = false;
        let mut i = 0usize;

        while i < block.body.len() {
            let cur = &block.body[i];
            if i + 1 < block.body.len() {
                let next = &block.body[i + 1];
                if cur.trim() == "}" && next.trim() == "else {" {
                    let indent = cur
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(format!("{indent}}} else {{"));
                    changed = true;
                    i += 2;
                    continue;
                }
            }
            out.push(cur.clone());
            i += 1;
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

