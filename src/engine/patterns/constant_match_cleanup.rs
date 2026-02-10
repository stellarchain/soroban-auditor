use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ConstantMatchCleanupPattern;

impl ConstantMatchCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ConstantMatchCleanupPattern {
    fn name(&self) -> &'static str {
        "constant_match_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            if let Some(new_line) = simplify_constant_match_line(line) {
                out.push(new_line);
                changed = true;
            } else {
                out.push(line.clone());
            }
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

fn simplify_constant_match_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("match ") || !trimmed.contains(" { 0 => { ") {
        return None;
    }
    let (prefix, rest) = split_once(trimmed, " { 0 => { ")?;
    let selector = prefix.strip_prefix("match ")?.trim();
    let selector_val = parse_selector(selector)?;

    let (arm0, rest) = split_once(rest, " }, 1 => { ")?;
    let (arm1, rest) = split_once(rest, " }, _ => { ")?;
    let (arm_default, suffix) = split_once(rest, " } }")?;
    if !suffix.trim().is_empty() && suffix.trim() != ";" {
        return None;
    }

    let chosen = match selector_val {
        0 => arm0,
        1 => arm1,
        _ => arm_default,
    };
    let chosen = strip_trailing_zero(chosen).trim();
    if chosen.is_empty() {
        return None;
    }

    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    Some(format!("{indent}{chosen};"))
}

fn parse_selector(s: &str) -> Option<i64> {
    let token = s.split_whitespace().next()?;
    token.parse::<i64>().ok()
}

fn strip_trailing_zero(s: &str) -> &str {
    let t = s.trim_end();
    if let Some(prefix) = t.strip_suffix("; 0") {
        return prefix;
    }
    if let Some(prefix) = t.strip_suffix("; 0 /* Void */") {
        return prefix;
    }
    t
}

fn split_once<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let idx = s.find(pat)?;
    Some((&s[..idx], &s[idx + pat.len()..]))
}

