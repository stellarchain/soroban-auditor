use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ConstantConditionFoldPattern;

impl ConstantConditionFoldPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ConstantConditionFoldPattern {
    fn name(&self) -> &'static str {
        "constant_condition_fold"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let Some((var, op, cond_value)) = parse_simple_if_cond(body[i].trim()) else {
                i += 1;
                continue;
            };
            let Some(if_end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };

            let (has_else, else_start, else_end) = if let Some(es) = next_else_start(&body, if_end) {
                let Some(ee) = find_block_end(&body, es) else {
                    i += 1;
                    continue;
                };
                (true, es, ee)
            } else {
                (false, 0usize, 0usize)
            };

            let Some((assign_idx, assigned_value)) = find_last_const_assign(&body, i, &var) else {
                i += 1;
                continue;
            };
            if has_reassignment(&body, assign_idx + 1, i, &var) {
                i += 1;
                continue;
            }

            let cond_true = match op {
                "==" => assigned_value == cond_value,
                "!=" => assigned_value != cond_value,
                _ => false,
            };

            if has_else {
                if cond_true {
                    let replacement = dedent_slice(&body[i + 1..if_end], 4);
                    body.splice(i..=else_end, replacement);
                } else {
                    let replacement = dedent_slice(&body[else_start + 1..else_end], 4);
                    body.splice(i..=else_end, replacement);
                }
            } else if cond_true {
                let replacement = dedent_slice(&body[i + 1..if_end], 4);
                body.splice(i..=if_end, replacement);
            } else {
                body.drain(i..=if_end);
            }
            changed = true;
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

fn parse_simple_if_cond(t: &str) -> Option<(String, &'static str, i64)> {
    if !t.starts_with("if ") || !t.ends_with(" {") {
        return None;
    }
    let cond = t.strip_prefix("if ")?.strip_suffix(" {")?.trim();
    if let Some((lhs, rhs)) = cond.split_once(" != ") {
        let value = rhs.trim().parse::<i64>().ok()?;
        return Some((lhs.trim().to_string(), "!=", value));
    }
    if let Some((lhs, rhs)) = cond.split_once(" == ") {
        let value = rhs.trim().parse::<i64>().ok()?;
        return Some((lhs.trim().to_string(), "==", value));
    }
    None
}

fn parse_assignment_lhs(t: &str) -> Option<&str> {
    if !t.ends_with(';') || !t.contains(" = ") {
        return None;
    }
    let (lhs, _) = t.trim_end_matches(';').split_once(" = ")?;
    let lhs = lhs.trim();
    if lhs.is_empty() || !lhs.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some(lhs)
}

fn parse_const_assignment(t: &str) -> Option<(String, i64)> {
    let lhs = parse_assignment_lhs(t)?.to_string();
    let rhs = t.trim_end_matches(';').split_once(" = ")?.1.trim();
    let rhs = rhs.split("/*").next().unwrap_or(rhs).trim();
    let value = rhs.parse::<i64>().ok()?;
    Some((lhs, value))
}

fn find_last_const_assign(lines: &[String], before: usize, var: &str) -> Option<(usize, i64)> {
    let mut idx = before;
    while idx > 0 {
        idx -= 1;
        let t = lines[idx].trim();
        if t.is_empty() {
            continue;
        }
        if let Some((lhs, value)) = parse_const_assignment(t) {
            if lhs == var {
                return Some((idx, value));
            }
        }
    }
    None
}

fn has_reassignment(lines: &[String], from: usize, to: usize, var: &str) -> bool {
    for line in lines.iter().take(to).skip(from) {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(lhs) = parse_assignment_lhs(t) {
            if lhs == var {
                return true;
            }
        }
    }
    false
}

fn find_block_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut opened = false;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if opens > 0 {
            opened = true;
        }
        depth += opens - closes;
        if opened && depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn next_else_start(lines: &[String], if_end: usize) -> Option<usize> {
    let mut idx = if_end + 1;
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() {
            idx += 1;
            continue;
        }
        if t.starts_with("else {") {
            return Some(idx);
        }
        return None;
    }
    None
}

fn dedent_slice(lines: &[String], spaces: usize) -> Vec<String> {
    lines.iter().map(|l| dedent_line(l, spaces)).collect()
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut consumed = 0usize;
    for ch in line.chars() {
        if ch == ' ' && consumed < spaces {
            consumed += 1;
        } else {
            break;
        }
    }
    line[consumed..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_if_false_without_else() {
        let p = ConstantConditionFoldPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    a = 0;".to_string(),
                "    x = 1;".to_string(),
                "    if a != 0 {".to_string(),
                "        y = 2;".to_string(),
                "    }".to_string(),
                "    z = 3;".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("should apply");
        assert!(!out.body.iter().any(|l| l.contains("if a !=")));
        assert!(out.body.iter().any(|l| l.trim() == "z = 3;"));
    }

    #[test]
    fn folds_if_true_with_else() {
        let p = ConstantConditionFoldPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    a = 0;".to_string(),
                "    if a == 0 {".to_string(),
                "        t = 1;".to_string(),
                "    }".to_string(),
                "    else {".to_string(),
                "        t = 2;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("should apply");
        assert!(out.body.iter().any(|l| l.trim() == "t = 1;"));
        assert!(!out.body.iter().any(|l| l.trim() == "t = 2;"));
    }

    #[test]
    fn folds_transfer_like_if_after_status_guard() {
        let p = ConstantConditionFoldPattern::new();
        let block = FunctionBlock {
            header: "pub fn transfer_like() {".to_string(),
            body: vec![
                "    a = 0;".to_string(),
                "    let mut sv3_0_i32 = mload32!(amount_val as usize) as i32;".to_string(),
                "    if sv3_0_i32 != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    c = to_muxed;".to_string(),
                "    if a != 0 {".to_string(),
                "        c = f;".to_string(),
                "    }".to_string(),
                "    done = 1;".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "transfer_like".to_string(),
        };
        let out = p.apply(&block).expect("should apply");
        assert!(!out.body.iter().any(|l| l.contains("if a != 0")));
        assert!(out.body.iter().any(|l| l.trim() == "done = 1;"));
    }
}
