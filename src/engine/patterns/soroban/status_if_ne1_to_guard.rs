use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct StatusIfNe1ToGuardPattern;

impl StatusIfNe1ToGuardPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StatusIfNe1ToGuardPattern {
    fn name(&self) -> &'static str {
        "status_if_ne1_to_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }

        let re_if =
            Regex::new(r"^(?P<indent>\s*)if (?P<cond>mload32!\(.+\)\s*!=\s*1)\s*\{$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            let Some(cap) = re_if.captures(&body[i]) else {
                i += 1;
                continue;
            };

            let Some(end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            if end <= i + 1 {
                i = end + 1;
                continue;
            }

            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("");
            let cond = cap.name("cond").map(|m| m.as_str()).unwrap_or("");
            let guard_cond = cond.replacen("!= 1", "!= 0", 1);
            let moved_body = dedent_block(&body[i + 1..end], 4);

            let mut replacement = Vec::with_capacity(moved_body.len() + 3);
            replacement.push(format!("{indent}if {guard_cond} {{"));
            replacement.push(format!("{indent}    unreachable!();"));
            replacement.push(format!("{indent}}}"));
            replacement.extend(moved_body);

            body.splice(i..=end, replacement);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_mload_ne1_status_if_to_guard() {
        let p = StatusIfNe1ToGuardPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if mload32!(amount_val as usize) != 1 {".to_string(),
                "        amount = mload64!(amount_val.wrapping_add(16) as usize);".to_string(),
                "        a = mload64!(amount_val.wrapping_add(24) as usize);".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "if mload32!(amount_val as usize) != 0 {");
        assert_eq!(out.body[1].trim(), "unreachable!();");
        assert_eq!(out.body[2].trim(), "}");
        assert_eq!(
            out.body[3].trim(),
            "amount = mload64!(amount_val.wrapping_add(16) as usize);"
        );
    }
}

