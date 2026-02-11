use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LoopComplementaryIfUnwrapPattern;

impl LoopComplementaryIfUnwrapPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopComplementaryIfUnwrapPattern {
    fn name(&self) -> &'static str {
        "loop_complementary_if_unwrap"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 8 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;
        while i < body.len() {
            if body[i].trim() != "loop {" && !body[i].trim().ends_with(": loop {") {
                i += 1;
                continue;
            }

            let Some(loop_end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };
            let loop_inner_start = i + 1;
            let loop_inner_end = loop_end;

            let mut cursor = loop_inner_start;
            let mut applied_in_this_loop = false;
            while cursor + 1 < loop_inner_end {
                let Some((first_if_start, first_if_end, cond1)) =
                    find_if_block_at_or_after(&body, cursor, loop_inner_end)
                else {
                    break;
                };
                if !if_body_is_single_break(&body, first_if_start, first_if_end) {
                    cursor = first_if_end + 1;
                    continue;
                }

                let Some((second_if_start, second_if_end, cond2)) =
                    find_if_block_at_or_after(&body, first_if_end + 1, loop_inner_end)
                else {
                    break;
                };
                if !is_complementary(&cond1, &cond2) {
                    cursor = second_if_end + 1;
                    continue;
                }

                let lifted = dedent_slice(&body[second_if_start + 1..second_if_end], 4);
                if lifted.is_empty() {
                    cursor = second_if_end + 1;
                    continue;
                }

                body.splice(second_if_start..=second_if_end, lifted);
                changed = true;
                applied_in_this_loop = true;
                break;
            }

            if applied_in_this_loop {
                // Re-run from the same loop header with fresh block bounds after mutation.
                continue;
            }
            i = loop_end + 1;
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

fn find_if_block_at_or_after(
    lines: &[String],
    start: usize,
    limit_exclusive: usize,
) -> Option<(usize, usize, String)> {
    let mut idx = start;
    while idx < limit_exclusive {
        let line = lines[idx].trim();
        if line.is_empty() {
            idx += 1;
            continue;
        }
        let Some(cond) = parse_if_condition(line) else {
            idx += 1;
            continue;
        };
        let end = find_block_end(lines, idx)?;
        if end >= limit_exclusive {
            return None;
        }
        return Some((idx, end, cond));
    }
    None
}

fn parse_if_condition(line: &str) -> Option<String> {
    if !line.starts_with("if ") || !line.ends_with(" {") {
        return None;
    }
    Some(
        line.trim_start_matches("if ")
            .trim_end_matches(" {")
            .trim()
            .to_string(),
    )
}

fn if_body_is_single_break(lines: &[String], if_start: usize, if_end: usize) -> bool {
    let mut significant: Vec<String> = Vec::new();
    for line in lines.iter().take(if_end).skip(if_start + 1) {
        let t = line.trim();
        if !t.is_empty() {
            significant.push(t.to_string());
        }
    }
    if significant.len() != 1 {
        return false;
    }
    let only = significant[0].as_str();
    only == "break;" || only.starts_with("break '")
}

fn is_complementary(c1: &str, c2: &str) -> bool {
    if let Some((l1, r1)) = split_binop(c1, "==") {
        if let Some((l2, r2)) = split_binop(c2, "!=") {
            return normalize(l1) == normalize(l2) && normalize(r1) == normalize(r2);
        }
    }
    if let Some((l1, r1)) = split_binop(c1, "!=") {
        if let Some((l2, r2)) = split_binop(c2, "==") {
            return normalize(l1) == normalize(l2) && normalize(r1) == normalize(r2);
        }
    }
    false
}

fn split_binop<'a>(cond: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let (lhs, rhs) = cond.split_once(op)?;
    Some((lhs.trim(), rhs.trim()))
}

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<String>()
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
    let mut taken = 0usize;
    for ch in line.chars() {
        if ch == ' ' && taken < spaces {
            taken += 1;
        } else {
            break;
        }
    }
    line[taken..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwraps_complementary_if_inside_loop() {
        let p = LoopComplementaryIfUnwrapPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        d = e;".to_string(),
                "        if d == 16 {".to_string(),
                "            break;".to_string(),
                "        }".to_string(),
                "        if d != 16 {".to_string(),
                "            e = d.wrapping_add(8);".to_string(),
                "        }".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };

        let out = p.apply(&block).expect("must apply");
        assert_eq!(
            out.body,
            vec![
                "    loop {".to_string(),
                "        d = e;".to_string(),
                "        if d == 16 {".to_string(),
                "            break;".to_string(),
                "        }".to_string(),
                "        e = d.wrapping_add(8);".to_string(),
                "    }".to_string(),
            ]
        );
    }
}
