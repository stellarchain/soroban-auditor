use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

/// Pattern pentru consolidarea comment-urilor de diagnosticare
///
/// Colectează toate TODO-urile și comentariile de diagnosticare dintr-o funcție
/// și le consolidează într-un singur comment block la început.
///
/// Elimină:
/// - `// TODO: helper function call removed: ...`
/// - `// Removed duplicate declaration: ...`
/// - `// TODO: Conversion from ... to ... for ...`
/// - `// TODO: Type check for ... as ...`
///
/// Și le înlocuiește cu un singur comment block informativ.
pub struct ConsolidateCommentsPattern;

impl ConsolidateCommentsPattern {
    pub fn new() -> Self {
        Self
    }

    /// Detectează dacă o linie este un comment de diagnosticare
    fn is_diagnostic_comment(line: &str) -> bool {
        let trimmed = line.trim();

        trimmed.starts_with("// TODO: helper function call removed:")
            || trimmed.starts_with("// Removed duplicate declaration:")
            || trimmed.starts_with("// TODO: Conversion from")
            || trimmed.starts_with("// TODO: Remove val_to_i64")
            || trimmed.starts_with("// TODO: Type check for")
    }

    /// Clasifică comment-ul și returnează categoria
    fn classify_comment(line: &str) -> Option<CommentType> {
        let trimmed = line.trim();

        if trimmed.starts_with("// TODO: helper function call removed:") {
            Some(CommentType::HelperRemoved)
        } else if trimmed.starts_with("// Removed duplicate declaration:") {
            Some(CommentType::DuplicateRemoved)
        } else if trimmed.starts_with("// TODO: Conversion from")
            || trimmed.starts_with("// TODO: Remove val_to_i64")
        {
            Some(CommentType::ConversionDetected)
        } else if trimmed.starts_with("// TODO: Type check for") {
            Some(CommentType::TypeCheck)
        } else {
            None
        }
    }

    /// Generează comment block consolidat
    fn generate_consolidated_comment(stats: &CommentStats, indent: &str) -> Vec<String> {
        if stats.is_empty() {
            return vec![];
        }

        let mut lines = vec![format!(
            "{}// NOTE: This function has been partially decompiled:",
            indent
        )];

        if stats.helper_removed > 0 {
            lines.push(format!(
                "{}//   - {} helper function call(s) removed",
                indent, stats.helper_removed
            ));
        }

        if stats.duplicate_removed > 0 {
            lines.push(format!(
                "{}//   - {} duplicate variable declaration(s) removed",
                indent, stats.duplicate_removed
            ));
        }

        if stats.conversion_detected > 0 {
            lines.push(format!(
                "{}//   - {} type conversion(s) detected",
                indent, stats.conversion_detected
            ));
        }

        if stats.type_check > 0 {
            lines.push(format!(
                "{}//   - {} type check(s) detected",
                indent, stats.type_check
            ));
        }

        lines.push(format!("{}//", indent));

        lines
    }
}

#[derive(Debug, Clone, Copy)]
enum CommentType {
    HelperRemoved,
    DuplicateRemoved,
    ConversionDetected,
    TypeCheck,
}

#[derive(Debug, Default)]
struct CommentStats {
    helper_removed: usize,
    duplicate_removed: usize,
    conversion_detected: usize,
    type_check: usize,
}

impl CommentStats {
    fn is_empty(&self) -> bool {
        self.helper_removed == 0
            && self.duplicate_removed == 0
            && self.conversion_detected == 0
            && self.type_check == 0
    }

    fn add(&mut self, comment_type: CommentType) {
        match comment_type {
            CommentType::HelperRemoved => self.helper_removed += 1,
            CommentType::DuplicateRemoved => self.duplicate_removed += 1,
            CommentType::ConversionDetected => self.conversion_detected += 1,
            CommentType::TypeCheck => self.type_check += 1,
        }
    }
}

impl Pattern for ConsolidateCommentsPattern {
    fn name(&self) -> &'static str {
        "consolidate_comments"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        // Check if consolidation was already done (prevent duplicates in iterative mode)
        for line in &block.body {
            if line
                .trim()
                .starts_with("// NOTE: This function has been partially decompiled:")
            {
                return None; // Already consolidated
            }
        }

        // First pass: collect statistics
        let mut stats = CommentStats::default();
        for line in &block.body {
            if Self::is_diagnostic_comment(line) {
                if let Some(comment_type) = Self::classify_comment(line) {
                    stats.add(comment_type);
                }
            }
        }

        // If no diagnostic comments, nothing to do
        if stats.is_empty() {
            return None;
        }

        // Second pass: remove diagnostic comments and build new body
        let mut new_body: Vec<String> = Vec::new();
        let mut consolidated_added = false;

        for line in &block.body {
            // Skip diagnostic comments
            if Self::is_diagnostic_comment(line) {
                continue;
            }

            // Add consolidated comment block before first real code line
            if !consolidated_added && !line.trim().is_empty() && !line.trim().starts_with("//") {
                let indent = line.len() - line.trim_start().len();
                let indent_str = " ".repeat(indent);

                let consolidated = Self::generate_consolidated_comment(&stats, &indent_str);
                new_body.extend(consolidated);
                consolidated_added = true;
            }

            new_body.push(line.clone());
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
    fn test_is_diagnostic_comment() {
        assert!(ConsolidateCommentsPattern::is_diagnostic_comment(
            "    // TODO: helper function call removed: self.foo()"
        ));
        assert!(ConsolidateCommentsPattern::is_diagnostic_comment(
            "    // Removed duplicate declaration: let mut x = 0;"
        ));
        assert!(!ConsolidateCommentsPattern::is_diagnostic_comment(
            "    // This is a normal comment"
        ));
    }

    #[test]
    fn test_consolidates_comments() {
        let pattern = ConsolidateCommentsPattern::new();
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "    // TODO: helper function call removed: self.foo()".to_string(),
                "    // Removed duplicate declaration: let mut x = 0;".to_string(),
                "    // TODO: Conversion from var to Type for dest".to_string(),
                "    let x = 42;".to_string(),
                "    x + 1".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).unwrap();

        // Should have consolidated comment + actual code
        assert!(result
            .body
            .iter()
            .any(|l| l.contains("NOTE: This function has been partially decompiled")));
        assert!(result
            .body
            .iter()
            .any(|l| l.contains("1 helper function call")));
        assert!(result
            .body
            .iter()
            .any(|l| l.contains("1 duplicate variable declaration")));
        assert!(result.body.iter().any(|l| l.contains("1 type conversion")));

        // Should not have individual diagnostic comments
        assert!(!result
            .body
            .iter()
            .any(|l| l.contains("TODO: helper function call removed")));
        assert!(!result
            .body
            .iter()
            .any(|l| l.contains("Removed duplicate declaration")));
    }
}
