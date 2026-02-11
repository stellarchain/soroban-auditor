use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

/// Pattern pentru simplificarea operațiilor matematice
///
/// Transformă:
/// - `x.wrapping_add(y)` → `x + y` (când e safe)
/// - `(amount * fee) / 10_000` → pattern recunoscut de fee calculation
/// - `x.wrapping_sub(y)` → `x - y` (când e safe)
pub struct MathOperationsPattern;

impl MathOperationsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for MathOperationsPattern {
    fn name(&self) -> &'static str {
        "math_operations"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = simplify_math(nodes, &mut changed);

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

fn simplify_math(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();

    for node in nodes {
        match node {
            Node::Line(line) => {
                let new_line = simplify_math_line(&line, changed);
                out.push(Node::Line(new_line));
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = simplify_math(body, changed);
                out.push(Node::Block {
                    kind,
                    label,
                    header,
                    body: new_body,
                    footer,
                });
            }
        }
    }

    out
}

fn simplify_math_line(line: &str, changed: &mut bool) -> String {
    let mut result = line.to_string();

    // Pattern 1: wrapping_add(1) în context de increment
    if result.contains(".wrapping_add(1)") {
        // În majoritatea cazurilor, wrapping_add(1) poate fi += 1
        // DAR: verifică contextul - dacă e într-un loop counter, e ok
        if is_simple_increment_context(&result) {
            result = result.replace(".wrapping_add(1)", " += 1");
            *changed = true;
        }
    }

    // Pattern 2: wrapping_sub(1) în context de decrement
    if result.contains(".wrapping_sub(1)") {
        if is_simple_decrement_context(&result) {
            result = result.replace(".wrapping_sub(1)", " -= 1");
            *changed = true;
        }
    }

    // Pattern 3: Fee calculation (amount * fee) / 10_000 sau similar
    if is_fee_calculation(&result) {
        // Adaugă un comment pentru claritate
        let indent = get_indent(&result);
        if !result.contains("//") && !result.contains("fee") {
            // Adaugă comment doar dacă nu există deja
            result = format!("{}// Fee calculation\n{}", indent, result);
            *changed = true;
        }
    }

    // Pattern 4: wrapping_mul pentru overflow safety
    // Păstrează wrapping_mul pentru i128/u128 - e intenționat pentru overflow

    result
}

fn is_simple_increment_context(line: &str) -> bool {
    // Verifică dacă e un context simplu de increment
    // Ex: var1 = var1.wrapping_add(1) sau similar
    let trimmed = line.trim();

    // Cazuri unsafe pentru simplificare:
    // - În calcule complexe
    // - Cu valori mari (> 1)
    if trimmed.contains("wrapping_add") && !trimmed.contains("wrapping_add(1)") {
        return false;
    }

    // Caz safe: assignment simplu
    if trimmed.contains(" = ") && trimmed.contains(".wrapping_add(1)") {
        return true;
    }

    false
}

fn is_simple_decrement_context(line: &str) -> bool {
    // Similar cu increment
    let trimmed = line.trim();
    trimmed.contains(" = ") && trimmed.contains(".wrapping_sub(1)")
}

fn is_fee_calculation(line: &str) -> bool {
    // Pattern: (x * y) / constant sau x * y / constant
    // Unde constant e un număr mare (10000, 10_000, 1000000, etc.)

    let has_mul = line.contains(" * ");
    let has_div = line.contains(" / ");
    let has_large_constant = line.contains("10_000")
        || line.contains("10000")
        || line.contains("1_000_000")
        || line.contains("1000000");

    has_mul && has_div && has_large_constant
}

fn get_indent(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_add_simplification() {
        let line = "        var1 = var1.wrapping_add(1);";
        let mut changed = false;
        let result = simplify_math_line(line, &mut changed);
        assert!(changed);
        assert!(result.contains("+="));
    }

    #[test]
    fn test_fee_calculation_detection() {
        let line = "        let fee = (amount * 25) / 10_000;";
        assert!(is_fee_calculation(line));
    }

    #[test]
    fn test_complex_wrapping_preserved() {
        let line = "        let result = a.wrapping_add(b.wrapping_mul(c));";
        let mut changed = false;
        let result = simplify_math_line(line, &mut changed);
        // Ar trebui să rămână wrapping pentru operații complexe
        assert!(result.contains("wrapping"));
    }
}
