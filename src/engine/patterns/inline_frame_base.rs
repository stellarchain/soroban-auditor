use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct InlineFrameBasePattern;

impl InlineFrameBasePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineFrameBasePattern {
    fn name(&self) -> &'static str {
        "inline_frame_base"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 2 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 1 < body.len() {
            let l0 = body[i].trim();
            let l1 = body[i + 1].trim();

            let Some(base_var) = parse_global0_let(l0) else {
                i += 1;
                continue;
            };
            let Some((lhs, rhs)) = parse_assignment(l1) else {
                i += 1;
                continue;
            };

            if !rhs.starts_with(&format!("{base_var}.wrapping_")) {
                i += 1;
                continue;
            }
            if count_ident_refs(&body, &base_var) != 2 {
                i += 1;
                continue;
            }

            let indent = body[i + 1]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let rewritten_rhs = rhs.replacen(&format!("{base_var}."), "self.global0.", 1);
            body[i + 1] = format!("{indent}{lhs} = {rewritten_rhs};");
            body.remove(i);
            changed = true;
            i = i.saturating_sub(1);
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

fn parse_global0_let(line: &str) -> Option<String> {
    let rest = line.strip_prefix("let ")?;
    let (lhs, rhs) = rest.split_once(" = ")?;
    if rhs.trim().trim_end_matches(';') != "self.global0" {
        return None;
    }
    Some(lhs.trim().split(':').next()?.trim().to_string())
}

fn parse_assignment(line: &str) -> Option<(String, String)> {
    let (lhs, rhs) = line.split_once(" = ")?;
    Some((
        lhs.trim().to_string(),
        rhs.trim().trim_end_matches(';').to_string(),
    ))
}

fn count_ident_refs(lines: &[String], ident: &str) -> usize {
    lines.iter().map(|l| count_ident_in_line(l, ident)).sum()
}

fn count_ident_in_line(line: &str, ident: &str) -> usize {
    let mut count = 0usize;
    let mut token = String::new();

    let flush = |tok: &mut String, count: &mut usize| {
        if tok == ident {
            *count += 1;
        }
        tok.clear();
    };

    for ch in line.chars() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            token.push(ch);
        } else {
            flush(&mut token, &mut count);
        }
    }
    flush(&mut token, &mut count);
    count
}
