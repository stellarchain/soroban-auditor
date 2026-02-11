use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct StatusGuardBlockUnwrapPattern;

impl StatusGuardBlockUnwrapPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StatusGuardBlockUnwrapPattern {
    fn name(&self) -> &'static str {
        "status_guard_block_unwrap"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 5 {
            return None;
        }

        let re_open = Regex::new(r"^\s*\{$").ok()?;
        let re_let = Regex::new(r"^\s*let mut (?P<var>\w+) = mload32!\(.+\) as i32;$").ok()?;
        let re_if = Regex::new(r"^\s*if (?P<var>\w+) (?P<op>==|!=) 0 \{$").ok()?;

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i + 4 < body.len() {
            if !re_open.is_match(body[i].trim()) {
                i += 1;
                continue;
            }
            let Some(let_cap) = re_let.captures(body[i + 1].trim()) else {
                i += 1;
                continue;
            };
            let Some(var) = let_cap.name("var").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            if !looks_like_generated_temp(var) {
                i += 1;
                continue;
            }
            let Some(if_cap) = re_if.captures(body[i + 2].trim()) else {
                i += 1;
                continue;
            };
            let Some(if_var) = if_cap.name("var").map(|m| m.as_str()) else {
                i += 1;
                continue;
            };
            if if_var != var {
                i += 1;
                continue;
            }
            if body[i + 3].trim() != "unreachable!();" || body[i + 4].trim() != "}" {
                i += 1;
                continue;
            }

            let close_idx = i + 5;
            if close_idx >= body.len() || body[close_idx].trim() != "}" {
                i += 1;
                continue;
            }

            let dedented_let = dedent_line(&body[i + 1], 4);
            let dedented_if = dedent_line(&body[i + 2], 4);
            let dedented_unreach = dedent_line(&body[i + 3], 4);
            let dedented_if_close = dedent_line(&body[i + 4], 4);

            let replacement = vec![dedented_let, dedented_if, dedented_unreach, dedented_if_close];
            body.splice(i..=close_idx, replacement);
            changed = true;
            i += 4;
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

fn looks_like_generated_temp(name: &str) -> bool {
    name.starts_with("sv") || name.starts_with("slot_var") || name.starts_with("var")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwraps_status_guard_block() {
        let p = StatusGuardBlockUnwrapPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    {".to_string(),
                "        let mut sv0_0_i32 = mload32!(a as usize) as i32;".to_string(),
                "        if sv0_0_i32 == 0 {".to_string(),
                "            unreachable!();".to_string(),
                "        }".to_string(),
                "    }".to_string(),
                "    x = 1;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    let mut sv0_0_i32 = mload32!(a as usize) as i32;".to_string(),
                "    if sv0_0_i32 == 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    x = 1;".to_string(),
            ]
        );
    }
}
