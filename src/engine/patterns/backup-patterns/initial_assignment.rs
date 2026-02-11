use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashSet;

pub struct InitialAssignmentPattern;

impl InitialAssignmentPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InitialAssignmentPattern {
    fn name(&self) -> &'static str {
        "initial_assignment"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        #[derive(Clone)]
        struct Replacement {
            decl_idx: usize,
            assign_idx: usize,
            line: String,
        }

        let mut replacements: Vec<Replacement> = Vec::new();
        let mut removal_idxs: Vec<usize> = Vec::new();
        let mut processed: HashSet<String> = HashSet::new();
        let lines = &block.body;

        for (idx, line) in lines.iter().enumerate() {
            if let Some(info) = parse_zero_init(line) {
                if processed.contains(&info.name) {
                    continue;
                }

                if let Some(assign) = find_initial_assignment(lines, idx, &info.name) {
                    if has_loop_between(lines, idx + 1, assign.assign_idx) {
                        continue;
                    }
                    if has_usage_between(lines, idx + 1, assign.assign_idx, &info.name) {
                        continue;
                    }

                    let new_line = format!(
                        "{}let mut {}: {} = {};",
                        info.indent, info.name, info.ty, assign.expr
                    );

                    replacements.push(Replacement {
                        decl_idx: idx,
                        assign_idx: assign.assign_idx,
                        line: new_line,
                    });
                    removal_idxs.push(assign.assign_idx);
                    processed.insert(info.name);
                }
            }
        }

        if replacements.is_empty() {
            return None;
        }

        let mut new_body = block.body.clone();
        for replacement in &replacements {
            new_body[replacement.decl_idx] = replacement.line.clone();
        }
        removal_idxs.sort_unstable();
        removal_idxs.dedup();
        for idx in removal_idxs.into_iter().rev() {
            if idx < new_body.len() {
                new_body.remove(idx);
            }
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

struct ZeroInitInfo {
    name: String,
    ty: String,
    indent: String,
}

struct AssignmentInfo {
    assign_idx: usize,
    expr: String,
}

fn parse_zero_init(line: &str) -> Option<ZeroInitInfo> {
    let trimmed = line.trim();
    if !trimmed.starts_with("let mut ") {
        return None;
    }

    let indent = line
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let after_mut = trimmed.strip_prefix("let mut ")?.trim_start();
    let colon = after_mut.find(':')?;
    let eq = after_mut.find('=')?;
    if colon >= eq {
        return None;
    }

    let name = after_mut[..colon].trim().to_string();
    let ty = after_mut[colon + 1..eq].trim().to_string();
    let expr = after_mut[eq + 1..].trim().trim_end_matches(';').trim();

    if !expr.starts_with('0') {
        return None;
    }

    let follows_zero = expr[1..].trim_start();
    if !follows_zero.is_empty() && !follows_zero.starts_with("as ") {
        return None;
    }

    Some(ZeroInitInfo { name, ty, indent })
}

fn find_initial_assignment(
    lines: &[String],
    decl_idx: usize,
    var_name: &str,
) -> Option<AssignmentInfo> {
    for (idx, line) in lines.iter().enumerate().skip(decl_idx + 1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if is_loop_start(trimmed) {
            break;
        }

        if let Some(expr) = parse_assignment(trimmed, var_name) {
            if contains_token(&expr, var_name) {
                continue;
            }
            return Some(AssignmentInfo {
                assign_idx: idx,
                expr,
            });
        }
    }

    None
}

fn parse_assignment(line: &str, var_name: &str) -> Option<String> {
    let targets = [format!("{} =", var_name), format!("{}=", var_name)];
    for target in &targets {
        if let Some(rest) = line.trim().strip_prefix(target) {
            let expr = rest.trim().trim_end_matches(';').trim();
            if expr.is_empty() {
                continue;
            }
            return Some(expr.to_string());
        }
    }
    None
}

fn is_loop_start(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("loop {") || trimmed.contains("while ")
}

fn has_loop_between(lines: &[String], start: usize, end: usize) -> bool {
    for line in lines.iter().take(end).skip(start) {
        if is_loop_start(line) {
            return true;
        }
    }
    false
}

fn contains_token(text: &str, token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let mut start = 0;
    while let Some(pos) = text[start..].find(token) {
        let absolute = start + pos;
        let before = text[..absolute]
            .chars()
            .rev()
            .next()
            .map(|c| c.is_alphanumeric() || c == '_');
        let after = text[absolute + token.len()..]
            .chars()
            .next()
            .map(|c| c.is_alphanumeric() || c == '_');
        if !before.unwrap_or(false) && !after.unwrap_or(false) {
            return true;
        }
        start = absolute + token.len();
    }
    false
}

fn has_usage_between(lines: &[String], start: usize, end: usize, var_name: &str) -> bool {
    for line in lines.iter().take(end).skip(start) {
        if contains_token(line, var_name) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::function::FunctionBlock;

    #[test]
    fn rewrites_initial_assignment() {
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let mut var1: i32 = 0;".to_string(),
                "    let mut var2: i32 = 0;".to_string(),
                "    var1 = base.wrapping_sub(16);".to_string(),
                "    var2 = var1.wrapping_add(8);".to_string(),
                "    loop {".to_string(),
                "        var2 = var2.wrapping_add(1);".to_string(),
                "        break;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };
        let pattern = InitialAssignmentPattern::new();
        let result = pattern.apply(&block).unwrap();
        assert!(result
            .body
            .iter()
            .any(|line| line.contains("let mut var1: i32 = base")));
        assert!(result
            .body
            .iter()
            .any(|line| line.contains("let mut var2: i32 = var1")));
        assert!(!result
            .body
            .iter()
            .any(|line| line.trim().starts_with("var1 =")));
        assert!(!result
            .body
            .iter()
            .any(|line| line.trim().starts_with("var2 =")));
    }
}
