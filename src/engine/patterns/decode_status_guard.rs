use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct DecodeStatusGuardPattern;

impl DecodeStatusGuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for DecodeStatusGuardPattern {
    fn name(&self) -> &'static str {
        "decode_status_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }

        let re_status_let =
            Regex::new(r"^\s*let mut (?P<var>\w+) = mload32!\((?P<addr>.+)\) as i32;$").ok()?;
        let re_if_ne_1 = Regex::new(r"^\s*if (?P<var>\w+) != 1 \{$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].trim();
            let Some(cap) = re_status_let.captures(line) else {
                i += 1;
                continue;
            };
            let Some(var) = cap.name("var").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            if !looks_like_generated_temp(var) {
                i += 1;
                continue;
            }

            let Some(if_idx) = next_non_empty_index(&body, i + 1) else {
                i += 1;
                continue;
            };
            let Some(if_cap) = re_if_ne_1.captures(body[if_idx].trim()) else {
                i = if_idx;
                continue;
            };
            let Some(if_var) = if_cap.name("var").map(|m| m.as_str()) else {
                i = if_idx;
                continue;
            };
            if if_var != var {
                i = if_idx;
                continue;
            }

            let Some(if_end) = find_block_end(&body, if_idx) else {
                i = if_idx + 1;
                continue;
            };
            let if_indent = leading_ws(&body[if_idx]);

            let moved_body = dedent_block(&body[if_idx + 1..if_end], 4);
            if moved_body.is_empty() {
                i = if_end + 1;
                continue;
            }

            let mut replacement: Vec<String> = Vec::new();
            replacement.push(format!("{if_indent}if {var} != 0 {{"));
            replacement.push(format!("{if_indent}    unreachable!();"));
            replacement.push(format!("{if_indent}}}"));
            replacement.extend(moved_body);

            body.splice(if_idx..=if_end, replacement);
            changed = true;
            i = if_idx + 1;
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

fn next_non_empty_index(lines: &[String], mut start: usize) -> Option<usize> {
    while start < lines.len() {
        if !lines[start].trim().is_empty() {
            return Some(start);
        }
        start += 1;
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

fn dedent_block(lines: &[String], spaces: usize) -> Vec<String> {
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

fn leading_ws(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

fn looks_like_generated_temp(name: &str) -> bool {
    name.starts_with("sv") || name.starts_with("slot_var") || name.starts_with("var")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_status_if_body_to_guard_plus_body() {
        let p = DecodeStatusGuardPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut sv2_0_i32 = mload32!(arg0 as usize) as i32;".to_string(),
                "    if sv2_0_i32 != 1 {".to_string(),
                "        amount = mload64!(arg0 as usize + 16) as i64;".to_string(),
                "        a = mload64!(arg0 as usize + 24) as i64;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    let mut sv2_0_i32 = mload32!(arg0 as usize) as i32;".to_string(),
                "    if sv2_0_i32 != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    amount = mload64!(arg0 as usize + 16) as i64;".to_string(),
                "    a = mload64!(arg0 as usize + 24) as i64;".to_string(),
            ]
        );
    }
}

