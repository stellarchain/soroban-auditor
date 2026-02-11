use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ConstantGuardIfPattern;

impl ConstantGuardIfPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ConstantGuardIfPattern {
    fn name(&self) -> &'static str {
        "constant_guard_if"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 1 < body.len() {
            let Some((var, value)) = parse_int_assignment(body[i].trim()) else {
                i += 1;
                continue;
            };

            let Some(if_idx) = find_next_linear_if_for_var(&body, i + 1, &var) else {
                i += 1;
                continue;
            };

            let Some((cond_var, op, cond_value)) = parse_simple_if_cond(body[if_idx].trim()) else {
                i += 1;
                continue;
            };
            if cond_var != var || !segment_is_linear(&body[i + 1..if_idx]) {
                i += 1;
                continue;
            }
            let Some(if_end) = find_block_end(&body, if_idx) else {
                i += 1;
                continue;
            };
            if has_else_peer(&body, if_end) {
                i = if_end + 1;
                continue;
            }

            let cond_true = match op {
                "==" => value == cond_value,
                "!=" => value != cond_value,
                _ => false,
            };

            if cond_true {
                let replacement = dedent_slice(&body[if_idx + 1..if_end], 4);
                body.splice(if_idx..=if_end, replacement);
                changed = true;
                i += 1;
                continue;
            }

            body.drain(if_idx..=if_end);
            changed = true;
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

fn parse_int_assignment(t: &str) -> Option<(String, i64)> {
    if !t.ends_with(';') {
        return None;
    }
    let (lhs, rhs) = t.trim_end_matches(';').split_once(" = ")?;
    let lhs = lhs.trim();
    if lhs.is_empty() || !lhs.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    let rhs = rhs.trim();
    let rhs = rhs.split("/*").next().unwrap_or(rhs).trim();
    let value = rhs.parse::<i64>().ok()?;
    Some((lhs.to_string(), value))
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

fn has_else_peer(lines: &[String], if_end: usize) -> bool {
    let mut idx = if_end + 1;
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() {
            idx += 1;
            continue;
        }
        return t.starts_with("else ");
    }
    false
}

fn find_next_linear_if_for_var(lines: &[String], start: usize, var: &str) -> Option<usize> {
    let mut idx = start;
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() {
            idx += 1;
            continue;
        }
        if t == "{" || t == "}" || t.starts_with("'") {
            return None;
        }
        if t.starts_with("if ") && t.contains(var) {
            return Some(idx);
        }
        if let Some(lhs) = parse_assignment_lhs(t) {
            if lhs == var {
                return None;
            }
        }
        if parse_int_assignment(t).is_some() {
            return None;
        }
        idx += 1;
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

fn segment_is_linear(lines: &[String]) -> bool {
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("if ")
            || t.starts_with("else")
            || t.starts_with("match ")
            || t.starts_with("loop ")
            || t.starts_with("while ")
            || t.starts_with("for ")
            || t.starts_with('\'')
            || t == "{"
            || t == "}"
        {
            return false;
        }
    }
    true
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
