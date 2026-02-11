use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

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
    let re = Regex::new(
        r#"^(?P<indent>\s*)(?:(?P<prefix>let\s+[A-Za-z_][A-Za-z0-9_]*\s*=\s*))?match\s+(?P<sel>-?\d+)(?:\s*/\*.*?\*/)?\s*\{\s*0\s*=>\s*\{\s*(?P<a0>.*?)\s*\}\s*,\s*1\s*=>\s*\{\s*(?P<a1>.*?)\s*\}\s*,\s*_\s*=>\s*\{\s*(?P<ad>.*?)\s*\}\s*\}\s*;?\s*$"#,
    )
    .ok()?;
    let caps = re.captures(line)?;
    let indent = caps.name("indent").map(|m| m.as_str()).unwrap_or("");
    let prefix = caps.name("prefix").map(|m| m.as_str()).unwrap_or("");
    let selector_val = parse_selector(caps.name("sel")?.as_str())?;
    let arm0 = caps.name("a0")?.as_str();
    let arm1 = caps.name("a1")?.as_str();
    let arm_default = caps.name("ad")?.as_str();

    let chosen = match selector_val {
        0 => arm0,
        1 => arm1,
        _ => arm_default,
    };
    let chosen = strip_trailing_zero(chosen).trim();
    if chosen.is_empty() {
        return None;
    }

    Some(format!("{indent}{prefix}{chosen};"))
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
