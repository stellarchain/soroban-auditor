use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct VmScaffoldCleanupPattern;

impl VmScaffoldCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for VmScaffoldCleanupPattern {
    fn name(&self) -> &'static str {
        "vm_scaffold_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 6 {
            return None;
        }

        // Stay conservative: only touch already high-level functions.
        if block
            .body
            .iter()
            .any(|l| l.contains("mload") || l.contains("mstore"))
        {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;

        let (frame_var, frame_size, prologue_range) = find_frame_setup(&body)?;
        let (restore_idx, ret_idx) = find_frame_restore_and_return(&body, &frame_var, frame_size)?;

        // Ensure frame vars are not used in semantic body.
        let sem_start = prologue_range.1 + 1;
        if sem_start < restore_idx {
            for line in &body[sem_start..restore_idx] {
                let t = line.trim();
                if t.contains(&frame_var) || t.contains("self.global0") {
                    return None;
                }
            }
        }

        // Remove restore line.
        body.remove(restore_idx);
        let _ = ret_idx.saturating_sub(1);
        // Remove prologue.
        body.drain(prologue_range.0..=prologue_range.1);
        changed = true;

        if changed {
            Some(FunctionBlock {
                header: block.header.clone(),
                body,
                footer: block.footer.clone(),
                indent: block.indent.clone(),
                name: block.name.clone(),
            })
        } else {
            None
        }
    }
}

fn find_frame_setup(lines: &[String]) -> Option<(String, i32, (usize, usize))> {
    // Pattern A:
    // let base = self.global0;
    // frame = base.wrapping_sub(SIZE);
    // self.global0 = frame;
    for i in 0..lines.len().saturating_sub(2) {
        let l0 = lines[i].trim();
        let l1 = lines[i + 1].trim();
        let l2 = lines[i + 2].trim();

        let (base_var, rhs0) = parse_assignment(l0)?;
        if rhs0 != "self.global0" {
            continue;
        }
        let (frame_var, rhs1) = parse_assignment(l1)?;
        let Some(size) = parse_wrapping_sub(rhs1, base_var) else {
            continue;
        };
        if l2 != format!("self.global0 = {frame_var};") {
            continue;
        }
        return Some((frame_var.to_string(), size, (i, i + 2)));
    }

    // Pattern B:
    // frame = self.global0.wrapping_sub(SIZE);
    // self.global0 = frame;
    for i in 0..lines.len().saturating_sub(1) {
        let l0 = lines[i].trim();
        let l1 = lines[i + 1].trim();
        let Some((frame_var, rhs0)) = parse_assignment(l0) else {
            continue;
        };
        let prefix = "self.global0.wrapping_sub(";
        let Some(rest) = rhs0.strip_prefix(prefix) else {
            continue;
        };
        let Some(size_str) = rest.strip_suffix(')') else {
            continue;
        };
        let Ok(size) = size_str.parse::<i32>() else {
            continue;
        };
        if l1 != format!("self.global0 = {frame_var};") {
            continue;
        }
        return Some((frame_var.to_string(), size, (i, i + 1)));
    }
    None
}

fn find_frame_restore_and_return(
    lines: &[String],
    frame_var: &str,
    size: i32,
) -> Option<(usize, usize)> {
    for i in 0..lines.len().saturating_sub(1) {
        let l0 = lines[i].trim();
        let l1 = lines[i + 1].trim();
        if l0 == format!("self.global0 = {frame_var}.wrapping_add({size});") && l1.starts_with("return ") {
            return Some((i, i + 1));
        }
    }
    None
}

fn parse_assignment(line: &str) -> Option<(&str, &str)> {
    let (lhs, rhs) = line.split_once(" = ")?;
    let lhs = lhs
        .trim()
        .strip_prefix("let mut ")
        .or_else(|| lhs.trim().strip_prefix("let "))
        .unwrap_or(lhs.trim())
        .trim();
    Some((lhs, rhs.trim().trim_end_matches(';')))
}

fn parse_wrapping_sub(rhs: &str, base_var: &str) -> Option<i32> {
    let prefix = format!("{base_var}.wrapping_sub(");
    let rest = rhs.strip_prefix(&prefix)?;
    let num = rest.strip_suffix(')')?;
    num.parse::<i32>().ok()
}
