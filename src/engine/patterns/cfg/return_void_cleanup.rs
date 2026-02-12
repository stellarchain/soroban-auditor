use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ReturnVoidCleanupPattern;

impl ReturnVoidCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ReturnVoidCleanupPattern {
    fn name(&self) -> &'static str {
        "return_void_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let is_unit_return = !block.header.contains("->");
        let top_level_indent = format!("{}    ", block.indent);
        let last_non_empty = block.body.iter().rposition(|line| !line.trim().is_empty());
        let mut changed = false;
        let mut out: Vec<String> = Vec::with_capacity(block.body.len());
        let mut i = 0usize;

        while i < block.body.len() {
            let line = &block.body[i];
            let trimmed = line.trim();

            if is_unit_return && trimmed == "return 0 /* Void */;" {
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                out.push(format!("{indent}return;"));
                changed = true;
                i += 1;
                continue;
            }

            if is_unit_return
                && Some(i) == last_non_empty
                && line.starts_with(&top_level_indent)
                && (trimmed == "0 /* Void */" || trimmed == "0 /* Void */;")
            {
                changed = true;
                i += 1;
                continue;
            }

            if let Some((indent, var_name, rhs)) = parse_void_match_assignment(line) {
                if let Some(next) = block.body.get(i + 1) {
                    if next.trim() == format!("{var_name};") {
                        out.push(format!("{indent}{rhs};"));
                        changed = true;
                        i += 2;
                        continue;
                    }
                }
            }

            if let Some((indent, var_name, inner)) = parse_zero_block_assignment(line) {
                if let Some(next) = block.body.get(i + 1) {
                    if next.trim() == format!("{var_name};") {
                        out.push(format!("{indent}{inner};"));
                        changed = true;
                        i += 2;
                        continue;
                    }
                }
            }

            out.push(line.clone());
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

fn parse_void_match_assignment(line: &str) -> Option<(String, String, String)> {
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains(" = match 0 /* Void */ ") {
        return None;
    }
    if !trimmed.ends_with('}') {
        return None;
    }

    let rest = trimmed.strip_prefix("let ")?;
    let eq_pos = rest.find(" = ")?;
    let var_name = rest[..eq_pos].trim().to_string();
    if var_name.is_empty() {
        return None;
    }
    let rhs = rest[eq_pos + 3..].trim().to_string();
    Some((indent, var_name, rhs))
}

fn parse_zero_block_assignment(line: &str) -> Option<(String, String, String)> {
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains(" = { ") || !trimmed.ends_with(" };") {
        return None;
    }

    let rest = trimmed.strip_prefix("let ")?;
    let eq_pos = rest.find(" = ")?;
    let var_name = rest[..eq_pos].trim().to_string();
    if var_name.is_empty() {
        return None;
    }

    let rhs = rest[eq_pos + 3..].trim();
    let inner = rhs.strip_prefix("{ ")?.strip_suffix(" };")?;
    if !(inner.ends_with("; 0") || inner.ends_with("; 0 /* Void */")) {
        return None;
    }

    let cut = if inner.ends_with("; 0 /* Void */") {
        inner.len().saturating_sub("; 0 /* Void */".len())
    } else {
        inner.len().saturating_sub("; 0".len())
    };
    let expr = inner[..cut].trim();
    if expr.is_empty() {
        return None;
    }
    Some((indent, var_name, expr.to_string()))
}
