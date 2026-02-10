use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct LabelMatchBreakGuard;

impl LabelMatchBreakGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LabelMatchBreakGuard {
    fn name(&self) -> &'static str {
        "label_match_break_guard"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 10 {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        let mut i = 0usize;
        while i < block.body.len() {
            if let Some((replacement, consumed)) = rewrite_at(&block.body, i) {
                out.extend(replacement);
                i += consumed;
                changed = true;
                continue;
            }
            out.push(block.body[i].clone());
            i += 1;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: out,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite_at(lines: &[String], start: usize) -> Option<(Vec<String>, usize)> {
    let l0 = lines.get(start)?;
    let (indent, outer_label) = parse_labeled_block_start(l0)?;

    let l1 = lines.get(start + 1)?;
    let (indent2, inner_label) = parse_labeled_block_start(l1)?;
    if indent2 != format!("{indent}    ") {
        return None;
    }

    let l2 = lines.get(start + 2)?;
    let match_indent = format!("{indent}        ");
    let match_expr = parse_match_header(l2, &match_indent)?;

    let l3 = lines.get(start + 3)?;
    let (_key_inner, rhs_inner) = parse_match_arm(l3, &format!("{indent}            "))?;
    if rhs_inner != format!("break '{}", inner_label) {
        return None;
    }

    let l4 = lines.get(start + 4)?;
    let (outer_key, rhs_outer) = parse_match_arm(l4, &format!("{indent}            "))?;
    if rhs_outer != format!("break '{}", outer_label) {
        return None;
    }

    let l5 = lines.get(start + 5)?;
    let (_key_default, rhs_default) = parse_match_arm(l5, &format!("{indent}            "))?;
    if rhs_default != "break" && rhs_default != format!("break '{}", inner_label) {
        return None;
    }

    if lines.get(start + 6)?.trim() != "}" {
        return None;
    }
    if lines.get(start + 7)?.trim() != "}" {
        return None;
    }

    let assignment = lines.get(start + 8)?;
    if !assignment.starts_with(&format!("{indent}    ")) {
        return None;
    }
    if assignment.contains("break '")
        || assignment.contains("continue '")
        || assignment.trim().is_empty()
    {
        return None;
    }

    if lines.get(start + 9)?.trim() != "}" {
        return None;
    }

    let rewritten = vec![
        format!("{indent}if ({match_expr}) != {outer_key} {{"),
        assignment.clone(),
        format!("{indent}}}"),
    ];
    Some((rewritten, 10))
}

fn parse_labeled_block_start(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('\'') || !trimmed.ends_with(": {") {
        return None;
    }
    let indent = line
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let inner = trimmed.trim_end_matches(": {").trim_start_matches('\'');
    if inner.is_empty() {
        return None;
    }
    Some((indent, inner.to_string()))
}

fn parse_match_header(line: &str, expected_indent: &str) -> Option<String> {
    if !line.starts_with(expected_indent) {
        return None;
    }
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("match ")?;
    let expr = rest.strip_suffix('{')?.trim();
    if expr.is_empty() {
        None
    } else {
        Some(expr.to_string())
    }
}

fn parse_match_arm(line: &str, expected_indent: &str) -> Option<(String, String)> {
    if !line.starts_with(expected_indent) {
        return None;
    }
    let trimmed = line.trim().trim_end_matches(',');
    let (lhs, rhs) = trimmed.split_once("=>")?;
    let lhs = lhs.trim().to_string();
    let rhs = rhs.trim().trim_end_matches(';').trim().to_string();
    if lhs.is_empty() || rhs.is_empty() {
        return None;
    }
    Some((lhs, rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_label_match_break_guard_shape() {
        let lines = vec![
            "            'label1: {".to_string(),
            "                'label2: {".to_string(),
            "                    match (to_muxed as i32 & 255).wrapping_add(-77) {".to_string(),
            "                        0 => break 'label2,".to_string(),
            "                        1 => break 'label1,".to_string(),
            "                        _ => break,".to_string(),
            "                    }".to_string(),
            "                }".to_string(),
            "                var4 = 0;".to_string(),
            "            }".to_string(),
        ];

        let (rewritten, consumed) = rewrite_at(&lines, 0).expect("must rewrite");
        assert_eq!(consumed, 10);
        assert_eq!(
            rewritten,
            vec![
                "            if ((to_muxed as i32 & 255).wrapping_add(-77)) != 1 {".to_string(),
                "                var4 = 0;".to_string(),
                "            }".to_string(),
            ]
        );
    }
}
