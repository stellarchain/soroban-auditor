use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct UnwrapTypeTagOkIfPattern;

impl UnwrapTypeTagOkIfPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for UnwrapTypeTagOkIfPattern {
    fn name(&self) -> &'static str {
        "unwrap_type_tag_ok_if"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let header = body[i].trim();
            if !is_type_tag_ok_if(header) {
                i += 1;
                continue;
            }
            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            if has_else_peer(&body, end) {
                i = end + 1;
                continue;
            }
            let replacement = dedent_slice(&body[i + 1..end], 4);
            body.splice(i..=end, replacement);
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

fn is_type_tag_ok_if(line: &str) -> bool {
    line.starts_with("if ")
        && line.ends_with(" {")
        && line.contains("& 255 == 0")
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
    fn unwraps_positive_type_tag_if() {
        let p = UnwrapTypeTagOkIfPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if decimal & 255 == 0 {".to_string(),
                "        x = 1;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body, vec!["    x = 1;".to_string()]);
    }

    #[test]
    fn keeps_if_with_else() {
        let p = UnwrapTypeTagOkIfPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if decimal & 255 == 0 {".to_string(),
                "        x = 1;".to_string(),
                "    }".to_string(),
                "    else {".to_string(),
                "        x = 2;".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        assert!(p.apply(&block).is_none());
    }
}
