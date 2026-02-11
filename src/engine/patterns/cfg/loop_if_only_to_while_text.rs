use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LoopIfOnlyToWhileTextPattern;

impl LoopIfOnlyToWhileTextPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoopIfOnlyToWhileTextPattern {
    fn name(&self) -> &'static str {
        "loop_if_only_to_while_text"
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
            let Some(loop_end) = find_block_end(&body, i) else {
                i += 1;
                continue;
            };

            let Some((if_start, if_end, cond)) = find_single_if_in_loop(&body, i + 1, loop_end)
            else {
                i += 1;
                continue;
            };
            if if_start != i + 1 {
                i += 1;
                continue;
            }

            let indent = leading_ws(&body[i]);
            let lifted_body = dedent_slice(&body[if_start + 1..if_end], 4);
            let mut replacement = Vec::new();
            replacement.push(format!("{indent}while {cond} {{"));
            replacement.extend(lifted_body);
            replacement.push(format!("{indent}}}"));

            body.splice(i..=loop_end, replacement);
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

fn find_single_if_in_loop(
    lines: &[String],
    start: usize,
    end_exclusive: usize,
) -> Option<(usize, usize, String)> {
    let mut if_start = None;
    for (idx, line) in lines.iter().enumerate().take(end_exclusive).skip(start) {
        if !line.trim().is_empty() {
            if_start = Some(idx);
            break;
        }
    }
    let if_start = if_start?;

    let cond = parse_if_cond(lines[if_start].trim())?;
    let if_end = find_block_end(lines, if_start)?;
    for line in lines.iter().take(end_exclusive).skip(if_end + 1) {
        if !line.trim().is_empty() {
            return None;
        }
    }
    Some((if_start, if_end, cond))
}

fn parse_if_cond(line: &str) -> Option<String> {
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

fn leading_ws(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
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
    fn lowers_loop_if_only_to_while() {
        let p = LoopIfOnlyToWhileTextPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        if b != 16 {".to_string(),
                "            b = b.wrapping_add(8);".to_string(),
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
                "    while b != 16 {".to_string(),
                "        b = b.wrapping_add(8);".to_string(),
                "    }".to_string(),
            ]
        );
    }
}
