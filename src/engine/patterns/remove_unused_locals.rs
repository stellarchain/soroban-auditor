use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct RemoveUnusedLocalsPattern;

impl RemoveUnusedLocalsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RemoveUnusedLocalsPattern {
    fn name(&self) -> &'static str {
        "remove_unused_locals"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        for line in &block.body {
            for tok in tokens(line) {
                *counts.entry(tok).or_insert(0) += 1;
            }
        }

        let mut changed = false;
        let mut out = Vec::<String>::with_capacity(block.body.len());
        for line in &block.body {
            let trimmed = line.trim();
            let Some(name) = extract_decl_name(trimmed) else {
                out.push(line.clone());
                continue;
            };
            // Keep declaration heads that start a multiline expression
            // (for example: `let var = self.func(`). Removing only the first
            // line leaves dangling argument lines and breaks syntax.
            if is_multiline_decl_head(trimmed) {
                out.push(line.clone());
                continue;
            }
            if !looks_like_temp_name(name) {
                out.push(line.clone());
                continue;
            }
            if counts.get(name).copied().unwrap_or(0) == 1 {
                changed = true;
                continue;
            }
            out.push(line.clone());
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

fn is_multiline_decl_head(trimmed: &str) -> bool {
    if !(trimmed.starts_with("let ") || trimmed.starts_with("let mut ")) {
        return false;
    }
    if trimmed.ends_with(';') {
        return false;
    }
    // Typical multiline call/constructor starts.
    trimmed.ends_with('(')
        || trimmed.contains(" = self.")
        || trimmed.contains(" = match ")
        || trimmed.contains(" = {")
}

fn extract_decl_name(line: &str) -> Option<&str> {
    let rest = if let Some(v) = line.strip_prefix("let mut ") {
        v
    } else if let Some(v) = line.strip_prefix("let ") {
        v
    } else {
        return None;
    };
    let name = rest.split(':').next()?.split('=').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn looks_like_temp_name(name: &str) -> bool {
    name.starts_with("var")
        || name.starts_with("slot_var")
        || name.starts_with("sv")
        || name == "vec_builder"
        || name == "loaded_val"
}

fn tokens(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in line.chars() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            cur.push(ch);
        } else if !cur.is_empty() {
            out.push(cur.clone());
            cur.clear();
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}
