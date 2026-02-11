use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct GuardBreakUnreachablePattern;

impl GuardBreakUnreachablePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for GuardBreakUnreachablePattern {
    fn name(&self) -> &'static str {
        "guard_break_unreachable"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 6 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i < body.len() {
            if body[i].trim() != "{" {
                i += 1;
                continue;
            }
            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            if end <= i + 2 {
                i = end + 1;
                continue;
            }
            let inner = &body[i + 1..end];
            let Some((if_start, if_end, if_header, if_indent)) = find_break_guard(inner) else {
                i = end + 1;
                continue;
            };

            if !has_unreachable_tail(&body, end + 1) {
                i = end + 1;
                continue;
            }

            let mut replacement = Vec::<String>::new();
            replacement.extend_from_slice(&inner[..if_start]);
            replacement.push(if_header.to_string());
            replacement.push(format!("{if_indent}    unreachable!();"));
            replacement.push(format!("{if_indent}}}"));
            replacement.extend_from_slice(&inner[if_end + 1..]);

            body.splice(i..=end, replacement);
            remove_unreachable_tail(&mut body, i);
            changed = true;
            i += 1;
        }

        if remove_dead_unreachable_after_return(&mut body) {
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

fn find_block_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        depth += opens - closes;
        if depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn find_break_guard(inner: &[String]) -> Option<(usize, usize, &str, &str)> {
    for idx in 0..inner.len().saturating_sub(2) {
        let header = inner[idx].as_str();
        let mid = inner[idx + 1].trim();
        let tail = inner[idx + 2].trim();
        let header_trim = header.trim();
        if !header_trim.starts_with("if ") || !header_trim.ends_with('{') {
            continue;
        }
        if mid != "break;" || tail != "}" {
            continue;
        }
        let indent_len = header.chars().take_while(|c| c.is_whitespace()).count();
        let indent = &header[..indent_len];
        return Some((idx, idx + 2, header, indent));
    }
    None
}

fn has_unreachable_tail(lines: &[String], from: usize) -> bool {
    for line in lines.iter().skip(from) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "unreachable!();" {
            return true;
        }
        return false;
    }
    false
}

fn remove_unreachable_tail(lines: &mut Vec<String>, from: usize) {
    let mut idx = from;
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= lines.len() || lines[idx].trim() != "unreachable!();" {
        return;
    }
    let mut end = idx;
    if end + 1 < lines.len() && lines[end + 1].trim_start().starts_with("// There should've been") {
        end += 1;
        if end + 1 < lines.len() && lines[end + 1].trim() == "unreachable!();" {
            end += 1;
        }
    }
    lines.drain(idx..=end);
}

fn remove_dead_unreachable_after_return(lines: &mut Vec<String>) -> bool {
    let mut changed = false;
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "unreachable!();" {
            i += 1;
            continue;
        }
        let mut prev = i;
        while prev > 0 && lines[prev - 1].trim().is_empty() {
            prev -= 1;
        }
        if prev == 0 {
            i += 1;
            continue;
        }
        let prev_trim = lines[prev - 1].trim();
        if !prev_trim.starts_with("return ") {
            i += 1;
            continue;
        }

        let mut end = i;
        if end + 1 < lines.len() && lines[end + 1].trim_start().starts_with("// There should've been")
        {
            end += 1;
            if end + 1 < lines.len() && lines[end + 1].trim() == "unreachable!();" {
                end += 1;
            }
        }
        lines.drain(i..=end);
        changed = true;
    }
    changed
}
