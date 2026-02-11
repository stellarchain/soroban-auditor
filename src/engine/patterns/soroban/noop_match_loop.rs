use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct NoopMatchLoopPattern;

impl NoopMatchLoopPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for NoopMatchLoopPattern {
    fn name(&self) -> &'static str {
        "noop_match_loop"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 5 {
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
            if !is_noop_match_loop_lines(&body[i + 1..end]) {
                i = end + 1;
                continue;
            }

            body.drain(i..=end);
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

fn is_noop_match_loop_lines(lines: &[String]) -> bool {
    if lines.is_empty() {
        return false;
    }

    let mut saw_match = false;
    let mut saw_arm = false;
    let mut match_depth = 0i32;

    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        if t.starts_with("match ") && t.ends_with('{') {
            saw_match = true;
            match_depth += 1;
            continue;
        }
        if t == "}" {
            if match_depth <= 0 {
                return false;
            }
            match_depth -= 1;
            continue;
        }
        if t.contains("=> break") {
            saw_arm = true;
            continue;
        }
        return false;
    }

    saw_match && saw_arm && match_depth == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_single_match_break_loop() {
        let p = NoopMatchLoopPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        match x {".to_string(),
                "            0 | _ => break,".to_string(),
                "        }".to_string(),
                "    }".to_string(),
                "    y = 1;".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("pattern should apply");
        assert_eq!(out.body, vec!["    y = 1;".to_string()]);
    }

    #[test]
    fn keeps_real_loop() {
        let p = NoopMatchLoopPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        match x {".to_string(),
                "            0 => continue,".to_string(),
                "            _ => break,".to_string(),
                "        }".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        assert!(p.apply(&block).is_none());
    }
}
