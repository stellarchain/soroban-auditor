use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, BlockKind, Node};
use crate::engine::pattern::Pattern;

pub struct CountedLoopPattern;

impl CountedLoopPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for CountedLoopPattern {
    fn name(&self) -> &'static str {
        "counted_loop"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite(nodes, &mut changed);
        if !changed {
            return None;
        }

        let new_body = flatten_nodes(&new_nodes);
        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out = Vec::new();

    for node in nodes {
        match node {
            Node::Block {
                kind: BlockKind::Loop,
                label,
                header,
                body,
                footer,
            } => {
                if label.is_none() {
                    if let Some(replacement) = try_rewrite_loop(&header, &body) {
                        for line in replacement {
                            out.push(Node::Line(line));
                        }
                        *changed = true;
                        continue;
                    }
                }
                let new_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind: BlockKind::Loop,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite(body, changed);
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
            Node::Line(line) => out.push(Node::Line(line)),
        }
    }

    out
}

fn try_rewrite_loop(header: &str, body: &[Node]) -> Option<Vec<String>> {
    let lines = flatten_nodes(body);
    if let Some((cond_idx, break_idx, close_idx, counter, bound)) = locate_counter_condition(&lines)
    {
        if let Some(incr_idx) = find_increment_index(&lines, &counter) {
            if incr_idx <= break_idx {
                return None;
            }
            if break_idx > 4 {
                // guard against loops where the break is not near the top
                return None;
            }

            let mut remaining = Vec::new();
            for (idx, line) in lines.iter().enumerate() {
                if idx == cond_idx || idx == break_idx || idx == close_idx || idx == incr_idx {
                    continue;
                }
                remaining.push(line.clone());
            }

            if remaining.is_empty() {
                return None;
            }

            let indent = header
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let mut replacement = Vec::new();
            replacement.push(format!("{}for {counter} in 0..{bound} {{", indent));
            replacement.extend(remaining);
            replacement.push(format!("{} }}", indent));
            return Some(replacement);
        }
    }

    None
}

fn locate_counter_condition(lines: &[String]) -> Option<(usize, usize, usize, String, String)> {
    for (idx, line) in lines.iter().enumerate() {
        if line.trim() == "break;" && idx >= 2 && idx + 1 < lines.len() {
            let header_idx = idx - 1;
            let footer_idx = idx + 1;
            let header_line = lines[header_idx].trim();
            if !header_line.starts_with("if ") {
                continue;
            }
            if lines[footer_idx].trim() != "}" {
                continue;
            }
            if let Some((counter, bound)) = parse_ge_condition(header_line) {
                if !is_simple_variable(&counter) {
                    continue;
                }
                return Some((header_idx, idx, footer_idx, counter, bound));
            }
        }
    }
    None
}

fn parse_ge_condition(line: &str) -> Option<(String, String)> {
    let cond = extract_condition(line)?;
    if !cond.contains(">=") {
        return None;
    }
    let mut normalized = cond.replace("!= 0", "");
    let casts = [
        " as i32",
        " as u32",
        " as i64",
        " as u64",
        " as usize",
        " as i128",
        " as u128",
    ];
    for cast in &casts {
        normalized = normalized.replace(cast, "");
    }
    normalized = strip_outer_parens(&normalized);
    let (left, right) = normalized.split_once(">=")?;
    let left = strip_outer_parens(left.trim());
    let right = strip_outer_parens(right.trim());
    if left.is_empty() || right.is_empty() {
        return None;
    }
    Some((left, right))
}

fn extract_condition(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let body = trimmed.strip_prefix("if ")?;
    let without_brace = if let Some(idx) = body.rfind('{') {
        let slice = &body[..idx];
        slice.trim_end()
    } else {
        body.trim_end()
    };
    Some(without_brace.trim().to_string())
}

fn strip_outer_parens(expr: &str) -> String {
    let mut cleaned = expr.trim().to_string();

    loop {
        let trimmed = cleaned.trim();
        if trimmed.len() >= 2 && trimmed.starts_with('(') && trimmed.ends_with(')') {
            if parens_balanced(trimmed) {
                cleaned = trimmed[1..trimmed.len() - 1].trim().to_string();
                continue;
            }
        }
        break;
    }

    cleaned
}

fn parens_balanced(expr: &str) -> bool {
    let mut depth = 0;
    for ch in expr.chars() {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth < 0 {
                return false;
            }
        }
    }
    depth == 0
}

fn is_simple_variable(expr: &str) -> bool {
    !expr.is_empty() && expr.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn find_increment_index(lines: &[String], counter: &str) -> Option<usize> {
    for (idx, line) in lines.iter().enumerate().rev() {
        let trimmed = line.trim();
        if trimmed.contains(&format!("{} = {}.wrapping_add", counter, counter))
            && trimmed.contains("wrapping_add(1")
        {
            return Some(idx);
        }
        if trimmed.contains(&format!("{} += 1", counter)) {
            return Some(idx);
        }
        if trimmed.contains(&format!("{} = {} + 1", counter, counter)) {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::function::FunctionBlock;

    #[test]
    fn rewrite_counted_loop_to_for() {
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    loop {".to_string(),
                "        if ((idx as u32 >= len as u32) as i32 != 0) {".to_string(),
                "            break;".to_string(),
                "        }".to_string(),
                "        let item = vec.get_unchecked(idx as usize);".to_string(),
                "        idx = idx.wrapping_add(1);".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };
        let pattern = CountedLoopPattern::new();
        let result = pattern.apply(&block).unwrap();
        assert!(result
            .body
            .iter()
            .any(|line| line.contains("for idx in 0..len")));
        assert!(!result.body.iter().any(|line| line.contains("loop {")));
        assert!(!result.body.iter().any(|line| line.contains("break;")));
        assert!(!result.body.iter().any(|line| line.contains("wrapping_add")));
    }
}
