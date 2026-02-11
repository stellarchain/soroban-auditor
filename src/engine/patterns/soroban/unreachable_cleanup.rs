use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct UnreachableCleanupPattern;

impl UnreachableCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for UnreachableCleanupPattern {
    fn name(&self) -> &'static str {
        "unreachable_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out: Vec<String> = Vec::with_capacity(block.body.len());

        let mut i = 0usize;
        while i < block.body.len() {
            let line = &block.body[i];
            let trimmed = line.trim();

            if trimmed == "self.unreachable_stub(env);" {
                if let Some(next_idx) = next_non_empty_index(&block.body, i + 1) {
                    if block.body[next_idx].trim() == "unreachable!();" {
                        changed = true;
                        i += 1;
                        continue;
                    }
                }
            }

            if trimmed.starts_with("// There should've been an expression value here") {
                let prev_unreachable = out
                    .iter()
                    .rev()
                    .find(|l| !l.trim().is_empty())
                    .map(|l| l.trim() == "unreachable!();")
                    .unwrap_or(false);
                let next_unreachable = next_non_empty_index(&block.body, i + 1)
                    .map(|idx| block.body[idx].trim() == "unreachable!();")
                    .unwrap_or(false);
                if prev_unreachable && next_unreachable {
                    changed = true;
                    i += 1;
                    continue;
                }
            }

            if trimmed == "unreachable!();" {
                let prev_unreachable = out
                    .iter()
                    .rev()
                    .find(|l| !l.trim().is_empty())
                    .map(|l| l.trim() == "unreachable!();")
                    .unwrap_or(false);
                if prev_unreachable {
                    changed = true;
                    i += 1;
                    continue;
                }
            }

            out.push(line.clone());
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

fn next_non_empty_index(lines: &[String], mut idx: usize) -> Option<usize> {
    while idx < lines.len() {
        if !lines[idx].trim().is_empty() {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

