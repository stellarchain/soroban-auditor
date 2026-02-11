use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LoopWithoutControlToBlockPattern;

impl LoopWithoutControlToBlockPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopWithoutControlToBlockPattern {
    fn name(&self) -> &'static str {
        "loop_without_control_to_block"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }
        // Keep source-like vec-return functions untouched (e.g. hello_world style),
        // where loop cleanup can accidentally reintroduce low-level guard artifacts.
        if block.body.iter().any(|l| l.contains("return vec![&env,")) {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i < body.len() {
            let t = body[i].trim();
            if !(t == "loop {" || t.ends_with(": loop {")) {
                i += 1;
                continue;
            }
            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            if end <= i + 1 {
                i = end + 1;
                continue;
            }

            let indent = body[i]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let body_indent = indent.len() + 4;
            if has_loop_control(&body[(i + 1)..end], body_indent) {
                i = end + 1;
                continue;
            }

            if t == "loop {" {
                body[i] = format!("{indent}{{");
            } else if let Some((label, _)) = t.split_once(": loop {") {
                body[i] = format!("{indent}{label}: {{");
            } else {
                i = end + 1;
                continue;
            }

            if end + 1 < body.len() && body[end + 1].trim() == "unreachable!();" {
                body.remove(end + 1);
            }

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

fn has_loop_control(lines: &[String], body_indent: usize) -> bool {
    for line in lines {
        let indent = line.chars().take_while(|c| c.is_whitespace()).count();
        let t = line.trim();
        if indent != body_indent {
            continue;
        }
        if t == "break;"
            || t == "continue;"
            || t.starts_with("break '")
            || t.starts_with("continue '")
        {
            return true;
        }
    }
    false
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
