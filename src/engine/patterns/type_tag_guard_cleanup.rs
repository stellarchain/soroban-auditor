use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct TypeTagGuardCleanupPattern;

impl TypeTagGuardCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for TypeTagGuardCleanupPattern {
    fn name(&self) -> &'static str {
        "type_tag_guard_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 8 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 6 < body.len() {
            let l0 = body[i].trim();
            let l1 = body[i + 1].trim();
            let l2 = body[i + 2].trim();
            let l3 = body[i + 3].trim();
            let l4 = body[i + 4].trim();

            // varX = something as i32 & 255;
            let Some((tag_var, _rhs)) = parse_assignment(l0) else {
                i += 1;
                continue;
            };
            if !l0.contains(" as i32 & 255;") {
                i += 1;
                continue;
            }

            // if varX == 14 { unreachable!(); }
            if l1 != format!("if {tag_var} == 14 {{") || l2 != "unreachable!();" || l3 != "}" {
                i += 1;
                continue;
            }

            // if varX != 74 { ... }
            if l4 != format!("if {tag_var} != 74 {{") {
                i += 1;
                continue;
            }
            let Some(guard_end) = find_block_end(&body, i + 4) else {
                i += 1;
                continue;
            };

            let guard_body = &body[i + 5..guard_end];
            if !is_safe_vec_return_guard_body(guard_body) {
                i += 1;
                continue;
            }

            // Replace whole type-tag guard cluster with just the guard body.
            let dedented = dedent_by(guard_body, 4);
            body.splice(i..=guard_end, dedented);
            changed = true;
            continue;
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

fn parse_assignment(line: &str) -> Option<(&str, &str)> {
    let (lhs, rhs) = line.split_once(" = ")?;
    Some((lhs.trim(), rhs.trim()))
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

fn is_safe_vec_return_guard_body(lines: &[String]) -> bool {
    if lines.is_empty() {
        return false;
    }
    let mut vec_return_count = 0usize;
    for l in lines {
        let t = l.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("return vec![&env,") {
            vec_return_count += 1;
            continue;
        }
        if t.starts_with("self.global0 = ") {
            continue;
        }
        if t.ends_with(';') && t.contains(" = ") {
            continue;
        }
        return false;
    }
    vec_return_count == 1
}

fn dedent_by(lines: &[String], spaces: usize) -> Vec<String> {
    lines.iter().map(|l| dedent_line(l, spaces)).collect()
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut count = 0usize;
    for ch in line.chars() {
        if ch == ' ' && count < spaces {
            count += 1;
        } else {
            break;
        }
    }
    line[count..].to_string()
}
