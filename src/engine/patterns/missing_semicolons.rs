use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Pattern pentru adăugarea semicolons lipsă
///
/// Detectează și fixează:
/// - `let varN = expr` fără `;` la final
/// - Statements care ar trebui să aibă `;`
pub struct MissingSemicolonsPattern;

impl MissingSemicolonsPattern {
    pub fn new() -> Self {
        Self
    }

    fn needs_semicolon(line: &str) -> bool {
        let trimmed = line.trim();

        // Skip if already has semicolon
        if trimmed.ends_with(';') || trimmed.ends_with('{') {
            return false;
        }

        // Skip if empty or comment
        if trimmed.is_empty() || trimmed.starts_with("//") {
            return false;
        }

        // Add semicolon for let statements (even if they end with })
        if trimmed.starts_with("let ") {
            return true;
        }

        // Add semicolon for assignments (even if they end with })
        if trimmed.contains(" = ") && !trimmed.ends_with(',') && !trimmed.ends_with('{') {
            return true;
        }

        // Skip if ends with } and not a let/assignment
        if trimmed.ends_with('}') {
            return false;
        }

        // Add semicolon for expression statements
        if trimmed.contains("(") && !trimmed.starts_with("if ") && !trimmed.starts_with("while ")
            && !trimmed.starts_with("for ") && !trimmed.starts_with("match ") {
            return true;
        }

        false
    }
}

impl Pattern for MissingSemicolonsPattern {
    fn name(&self) -> &'static str {
        "missing_semicolons"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let new_body: Vec<String> = block
            .body
            .iter()
            .map(|line| {
                if Self::needs_semicolon(line) {
                    changed = true;
                    format!("{};", line.trim_end())
                } else {
                    line.clone()
                }
            })
            .collect();

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
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
    fn test_adds_semicolon_to_let() {
        let pattern = MissingSemicolonsPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    let var9 = Vec::new()".to_string(),
                "    let x = 42;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();
        assert!(result.body[0].ends_with(";"));
        assert_eq!(result.body[1], "    let x = 42;");
    }
}
