use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct AssertConditionCleanupPattern;

impl AssertConditionCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for AssertConditionCleanupPattern {
    fn name(&self) -> &'static str {
        "assert_condition_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            let new_line = cleanup_assert_line(line, &mut changed);
            out.push(new_line);
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

fn cleanup_assert_line(line: &str, changed: &mut bool) -> String {
    let trimmed = line.trim();
    if !trimmed.starts_with("assert!(") || !trimmed.ends_with(");") {
        return line.to_string();
    }

    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    let inside = trimmed
        .strip_prefix("assert!(")
        .and_then(|s| s.strip_suffix(");"))
        .map(str::trim);
    let Some(cond) = inside else {
        return line.to_string();
    };

    let Some(normalized) = normalize_condition(cond) else {
        return line.to_string();
    };
    *changed = true;
    format!("{indent}assert!({normalized});")
}

fn normalize_condition(cond: &str) -> Option<String> {
    if let Some(inner) = cond.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
        let inner = inner.trim();
        if let Some(inv) = invert_comparison(inner) {
            return Some(inv);
        }
        if let Some(double_inner) = inner.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
            return Some(double_inner.trim().to_string());
        }
    }
    None
}

fn invert_comparison(expr: &str) -> Option<String> {
    if expr.contains("&&") || expr.contains("||") {
        return None;
    }
    for (op, inv) in [
        (" != ", " == "),
        (" == ", " != "),
        (" >= ", " < "),
        (" <= ", " > "),
        (" > ", " <= "),
        (" < ", " >= "),
    ] {
        if let Some(pos) = expr.find(op) {
            let left = expr[..pos].trim();
            let right = expr[pos + op.len()..].trim();
            if left.is_empty() || right.is_empty() {
                return None;
            }
            return Some(format!("{left}{inv}{right}"));
        }
    }
    None
}

