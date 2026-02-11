use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LabelTerminalBreakCleanupPattern;

impl LabelTerminalBreakCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelTerminalBreakCleanupPattern {
    fn name(&self) -> &'static str {
        "label_terminal_break_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
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

            if let Some(first_break) = find_top_level_label_break(&body, i + 1, end, &label) {
                if has_non_empty_between(&body, first_break + 1, end) {
                    body.drain(first_break + 1..end);
                    changed = true;
                }
            }

            let Some(end2) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            if let Some(last_non_empty) = prev_non_empty_index(&body, end2) {
                if body[last_non_empty].trim() == format!("break '{label};")
                    && !has_continue_label(&body, i + 1, last_non_empty, &label)
                {
                    body.remove(last_non_empty);
                    body[i] = format!("{indent}{{");
                    changed = true;
                }
            }

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

fn parse_label_block_start(line: &str) -> Option<(String, String)> {
    let indent = leading_ws(line);
    let t = line.trim();
    let (lhs, rhs) = t.split_once(':')?;
    if rhs.trim() != "{" {
        return None;
    }
    let label = lhs.trim().trim_start_matches('\'');
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

fn find_top_level_label_break(lines: &[String], from: usize, to: usize, label: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().take(to).skip(from) {
        let t = line.trim();
        if depth == 0 && t == format!("break '{label};") {
            return Some(idx);
        }
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }
    }
    None
}

fn has_non_empty_between(lines: &[String], from: usize, to: usize) -> bool {
    for line in lines.iter().take(to).skip(from) {
        if !line.trim().is_empty() {
            return true;
        }
    }
    false
}

fn prev_non_empty_index(lines: &[String], mut before: usize) -> Option<usize> {
    while before > 0 {
        before -= 1;
        if !lines[before].trim().is_empty() {
            return Some(before);
        }
    }
    None
}

fn has_continue_label(lines: &[String], from: usize, to: usize, label: &str) -> bool {
    for line in lines.iter().take(to).skip(from) {
        if line.trim() == format!("continue '{label};") {
            return true;
        }
    }
    false
}

fn leading_ws(s: &str) -> String {
    s.chars().take_while(|c| c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_dead_tail_after_terminal_break() {
        let p = LabelTerminalBreakCleanupPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    'label4: {".to_string(),
                "        to_muxed = 0;".to_string(),
                "        break 'label4;".to_string(),
                "        x = 1;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert!(!out.body.iter().any(|l| l.trim() == "x = 1;"));
        assert!(!out.body.iter().any(|l| l.contains("break 'label4;")));
        assert!(out.body.iter().any(|l| l.trim() == "{"));
    }

    #[test]
    fn trims_transfer_like_dead_tail() {
        let p = LabelTerminalBreakCleanupPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    'label4: {".to_string(),
                "        to_muxed = 0 /* False */;".to_string(),
                "        break 'label4;".to_string(),
                "        {".to_string(),
                "            let g = x;".to_string(),
                "            if a != 6 {".to_string(),
                "                break 'label4;".to_string(),
                "            }".to_string(),
                "        }".to_string(),
                "        d = 1;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    {".to_string(),
                "        to_muxed = 0 /* False */;".to_string(),
                "    }".to_string()
            ]
        );
    }
}
