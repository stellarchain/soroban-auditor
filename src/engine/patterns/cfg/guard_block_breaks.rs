use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct GuardBlockBreaks;

impl GuardBlockBreaks {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for GuardBlockBreaks {
    fn name(&self) -> &'static str {
        "guard_block_breaks"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let (new_body, changed) = rewrite_lines(&block.body);
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

fn rewrite_lines(lines: &[String]) -> (Vec<String>, bool) {
    let mut out = Vec::with_capacity(lines.len());
    let mut changed = false;
    let mut guard_stack: Vec<(String, i32)> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if let Some(label) = parse_guard_label(trimmed) {
            guard_stack.push((label, 1));
            out.push(line.clone());
            continue;
        }
        if let Some((label, depth)) = guard_stack.last_mut() {
            let mut new_line = line.clone();
            if new_line.contains("break;") && !new_line.contains("break '") {
                new_line = new_line.replace("break;", &format!("break '{label};"));
                changed = true;
            }
            let opens = new_line.chars().filter(|&c| c == '{').count() as i32;
            let closes = new_line.chars().filter(|&c| c == '}').count() as i32;
            *depth += opens - closes;
            if *depth <= 0 {
                guard_stack.pop();
            }
            out.push(new_line);
            continue;
        }
        out.push(line.clone());
    }
    (out, changed)
}

fn parse_guard_label(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with("'__if_guard") || !trimmed.ends_with("{") {
        return None;
    }
    let before = trimmed.trim_end_matches('{').trim();
    let parts: Vec<&str> = before.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let label = parts[0].trim().trim_start_matches('\'').to_string();
    if label.starts_with("__if_guard") {
        Some(label)
    } else {
        None
    }
}
