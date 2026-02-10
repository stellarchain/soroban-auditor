use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct StatusResultGuardTextPattern;

impl StatusResultGuardTextPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StatusResultGuardTextPattern {
    fn name(&self) -> &'static str {
        "status_result_guard_text"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            if body[i].trim() != "loop {" {
                i += 1;
                continue;
            }
            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            let mut effective: Vec<String> = Vec::new();
            for line in body.iter().take(end).skip(i + 1) {
                if !line.trim().is_empty() {
                    effective.push(line.trim().to_string());
                }
            }
            if effective.len() != 2 {
                i = end.saturating_add(1);
                continue;
            }
            let let_line = effective[0].clone();
            let trap_line = effective[1].clone();
            if trap_line != "unreachable!();" || !is_status_let_line(&let_line) {
                i = end.saturating_add(1);
                continue;
            }
            let Some(var_name) = parse_let_name(&let_line) else {
                i = end.saturating_add(1);
                continue;
            };
            let indent = body[i]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let inner = format!("{indent}    ");
            let replacement = vec![
                format!("{indent}{{"),
                format!("{inner}{let_line}"),
                format!("{inner}if {var_name} != 0 {{"),
                format!("{inner}    unreachable!();"),
                format!("{inner}}}"),
                format!("{indent}}}"),
            ];
            body.splice(i..=end, replacement);
            changed = true;
            i += 6;
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
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        if idx > start && depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn is_status_let_line(line: &str) -> bool {
    line.starts_with("let ")
        && line.contains(" = ")
        && line.ends_with(';')
        && line.contains("mload32!(")
        && line.contains(" as i32;")
}

fn parse_let_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("let ")?;
    let lhs = rest.split('=').next()?.trim();
    let lhs = lhs.strip_prefix("mut ").unwrap_or(lhs);
    let name = lhs.split(':').next()?.trim();
    if is_ident(name) {
        Some(name.to_string())
    } else {
        None
    }
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
