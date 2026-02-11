use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LabelBreakTrapGuardPattern;

impl LabelBreakTrapGuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelBreakTrapGuardPattern {
    fn name(&self) -> &'static str {
        "label_break_trap_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        // Keep this conservative for unit-return functions where trap tails are common.
        if block.body.is_empty() || block.header.contains("->") {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i < body.len() {
            let Some((indent, label)) = parse_label_block_start(&body[i]) else {
                i += 1;
                continue;
            };
            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            let Some(tail_idx) = next_non_empty_index(&body, end + 1) else {
                i = end + 1;
                continue;
            };
            if !is_trap_tail(body[tail_idx].trim()) {
                i = end + 1;
                continue;
            }

            let mut saw_break = false;
            let mut has_continue = false;
            for line in &body[i + 1..end] {
                let t = line.trim();
                if t == format!("break '{label};") {
                    saw_break = true;
                }
                if t == format!("continue '{label};") {
                    has_continue = true;
                    break;
                }
            }
            if !saw_break || has_continue {
                i = end + 1;
                continue;
            }

            for line in &mut body[i + 1..end] {
                if line.trim() == format!("break '{label};") {
                    let line_indent = leading_ws(line);
                    *line = format!("{line_indent}unreachable!();");
                }
            }
            body[i] = format!("{indent}{{");
            body.remove(tail_idx);
            changed = true;
            i = end + 1;
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

fn parse_label_block_start(line: &str) -> Option<(String, String)> {
    let indent = leading_ws(line);
    let t = line.trim();
    let (label_part, rest) = t.split_once(':')?;
    if rest.trim() != "{" {
        return None;
    }
    let label = label_part.trim().trim_start_matches('\'');
    if label.is_empty() || !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((indent, label.to_string()))
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

fn next_non_empty_index(lines: &[String], mut idx: usize) -> Option<usize> {
    while idx < lines.len() {
        if !lines[idx].trim().is_empty() {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn is_trap_tail(t: &str) -> bool {
    t == "unreachable!();" || t.contains("unreachable_stub(") || t.contains("call_unreachable(")
}

fn leading_ws(s: &str) -> String {
    s.chars().take_while(|c| c.is_whitespace()).collect()
}
