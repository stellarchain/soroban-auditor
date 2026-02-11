use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct InlineIfAssignUnwrapPattern;

impl InlineIfAssignUnwrapPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineIfAssignUnwrapPattern {
    fn name(&self) -> &'static str {
        "inline_if_assign_unwrap"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let re = Regex::new(
            r"^(?P<indent>\s*)(?P<lhs>\w+)\s*=\s*\(if (?P<cond>.+?) \{ (?P<then>.+?) \} else \{ (?P<else>.+?) \}\);$",
        )
        .ok()?;

        let mut body = block.body.clone();
        let mut changed = false;

        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].clone();
            let Some(cap) = re.captures(&line) else {
                i += 1;
                continue;
            };

            let indent = cap.name("indent").map(|m| m.as_str()).unwrap_or("");
            let lhs = cap.name("lhs").map(|m| m.as_str()).unwrap_or("");
            let cond = cap.name("cond").map(|m| m.as_str()).unwrap_or("");
            let then_expr = cap.name("then").map(|m| m.as_str()).unwrap_or("");
            let else_expr = cap.name("else").map(|m| m.as_str()).unwrap_or("");
            if lhs.is_empty() || cond.is_empty() || then_expr.is_empty() || else_expr.is_empty() {
                i += 1;
                continue;
            }

            let replacement = vec![
                format!("{indent}if {cond} {{"),
                format!("{indent}    {lhs} = {then_expr};"),
                format!("{indent}}} else {{"),
                format!("{indent}    {lhs} = {else_expr};"),
                format!("{indent}}}"),
            ];
            body.splice(i..=i, replacement);
            changed = true;
            i += 5;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwraps_inline_if_assignment() {
        let p = InlineIfAssignUnwrapPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    g = (if b != 0 { a_hi } else { 0 /* False */ });".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body[0].trim(), "if b != 0 {");
        assert_eq!(out.body[1].trim(), "g = a_hi;");
        assert_eq!(out.body[2].trim(), "} else {");
        assert_eq!(out.body[3].trim(), "g = 0 /* False */;");
        assert_eq!(out.body[4].trim(), "}");
    }
}

