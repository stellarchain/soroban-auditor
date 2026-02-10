use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct LinearMemoryVecBuildPattern;

impl LinearMemoryVecBuildPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LinearMemoryVecBuildPattern {
    fn name(&self) -> &'static str {
        "linear_memory_vec_build"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut new_body = block.body.clone();
        let mut changed = false;

        for i in 0..new_body.len() {
            let line = new_body[i].trim();
            let old_expr = "val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */";
            let new_expr = "val_to_i64(Vec::<Val>::new(env).into_val(env))";
            let matched_expr = if line.contains(old_expr) {
                Some(old_expr)
            } else if line.contains(new_expr) {
                Some(new_expr)
            } else {
                None
            };
            let Some(matched_expr) = matched_expr else {
                continue;
            };

            if let Some((slot0, slot8)) = find_recent_slot_pair(&new_body, i) {
                let indent = new_body[i]
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                let replacement = format!(
                    "{}{}",
                    indent,
                    line.replacen(
                        matched_expr,
                        &format!(
                            "{{ let mut v = Vec::<Val>::new(env); v.push_back(val_from_i64({})); v.push_back(val_from_i64({})); val_to_i64(v.into_val(env)) }}",
                            slot0, slot8
                        ),
                        1
                    )
                );
                new_body[i] = replacement;
                changed = true;
            }
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn find_recent_slot_pair(lines: &[String], at: usize) -> Option<(String, String)> {
    let start = at.saturating_sub(40);
    let mut slot0_by_group: HashMap<String, String> = HashMap::new();
    let mut slot8_by_group: HashMap<String, String> = HashMap::new();

    for line in lines.iter().take(at).skip(start) {
        let trimmed = line.trim();
        if !trimmed.starts_with("let mut slot_var") || !trimmed.contains(" = ") {
            continue;
        }
        let lhs = trimmed
            .strip_prefix("let mut ")?
            .split_once(" = ")?
            .0
            .trim();
        if let Some(group) = lhs.strip_suffix("_0_i64") {
            slot0_by_group.insert(group.to_string(), lhs.to_string());
        } else if let Some(group) = lhs.strip_suffix("_8_i64") {
            slot8_by_group.insert(group.to_string(), lhs.to_string());
        }
    }

    for (group, slot0) in slot0_by_group {
        if let Some(slot8) = slot8_by_group.get(&group) {
            return Some((slot0, slot8.clone()));
        }
    }
    None
}
