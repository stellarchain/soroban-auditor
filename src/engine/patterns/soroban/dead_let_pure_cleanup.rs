use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct DeadLetPureCleanupPattern;

impl DeadLetPureCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for DeadLetPureCleanupPattern {
    fn name(&self) -> &'static str {
        "dead_let_pure_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out: Vec<String> = Vec::with_capacity(block.body.len());

        for line in &block.body {
            let t = line.trim();
            let Some((name, rhs)) = parse_let_binding_expr(t) else {
                out.push(line.clone());
                continue;
            };

            let refs = count_ident_refs(&block.body, &name);
            if refs == 1 && is_pure_rhs(&rhs) {
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

fn parse_let_binding_expr(line: &str) -> Option<(String, String)> {
    let rest = if let Some(v) = line.strip_prefix("let mut ") {
        v
    } else if let Some(v) = line.strip_prefix("let ") {
        v
    } else {
        return None;
    };
    let (lhs, rhs) = rest.split_once(" = ")?;
    let name = lhs.split(':').next()?.trim();
    if !is_ident(name) {
        return None;
    }
    let rhs = rhs.trim().trim_end_matches(';').trim();
    if rhs.is_empty() {
        return None;
    }
    Some((name.to_string(), rhs.to_string()))
}

fn is_pure_rhs(rhs: &str) -> bool {
    if rhs.contains('(') {
        return false;
    }
    rhs.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(c, '_' | ' ' | ':' | '<' | '>' | '&' | '|' | '^' | '!' | '+' | '-' | '*' | '/' | '%' | '=')
    })
}

fn count_ident_refs(lines: &[String], ident: &str) -> usize {
    lines.iter().map(|l| count_ident_in_line(l, ident)).sum()
}

fn count_ident_in_line(line: &str, ident: &str) -> usize {
    let mut count = 0usize;
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let left_ok = start == 0 || !is_ident_char(line.as_bytes()[start - 1] as char);
        let right_ok = end >= line.len() || !is_ident_char(line.as_bytes()[end] as char);
        if left_ok && right_ok {
            count += 1;
        }
        idx = end;
        if idx >= line.len() {
            break;
        }
    }
    count
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_unused_pure_let_with_type() {
        let p = DeadLetPureCleanupPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    let mut amount_lo: i64 = b as i64;".to_string(),
                "    let mut amount_hi: i64 = amount as i64;".to_string(),
                "    self.publish_transfer_event(env, amount_val);".to_string(),
            ],
            footer: "}".to_string(),
            indent: String::new(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("should apply");
        assert_eq!(out.body.len(), 1);
        assert!(out.body[0].contains("publish_transfer_event"));
    }
}
