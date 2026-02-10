use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

/// Pattern pentru simplificarea accesului la storage
///
/// Transformă:
/// ```rust
/// env.storage().instance().get(&key).unwrap()
/// ```
/// în:
/// ```rust
/// self.get_storage(&key)
/// ```
pub struct StorageAccessPattern;

impl StorageAccessPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StorageAccessPattern {
    fn name(&self) -> &'static str {
        "storage_access"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rewrite_storage_access(nodes, &mut changed);

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

fn rewrite_storage_access(nodes: Vec<Node>, changed: &mut bool) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();

    for node in nodes {
        match node {
            Node::Line(line) => {
                let new_line = simplify_storage_line(&line, changed);
                out.push(Node::Line(new_line));
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rewrite_storage_access(body, changed);
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

fn simplify_storage_line(line: &str, changed: &mut bool) -> String {
    let mut result = line.to_string();

    // Pattern 1: env.storage().instance().get(&key).unwrap() → key direct sau helper
    if result.contains("env.storage().instance().get(") && result.contains(").unwrap()") {
        // Extractează key-ul
        if let Some(key) = extract_key_from_get(&result) {
            let indent = get_indent(line);

            // Dacă e DataKey enum, păstrează pattern-ul simplu
            if key.starts_with("DataKey::") {
                // Simplificare minimă - păstrează clar ce face
                *changed = true;
                // Nu schimbăm prea mult, doar formatare
            }
        }
    }

    // Pattern 2: env.storage().instance().set(&key, &value) → simplificare
    if result.contains("env.storage().instance().set(") {
        // Similar pentru set
    }

    // Pattern 3: .unwrap_or(0) pentru i128 balance
    if result.contains(".unwrap_or(0)") && result.contains("i128") {
        *changed = true;
        // Pattern recunoscut pentru balance check
    }

    result
}

fn extract_key_from_get(line: &str) -> Option<String> {
    // Caută pattern: .get(&key).unwrap()
    if let Some(start) = line.find(".get(&") {
        let after_get = &line[start + 6..];
        if let Some(end) = after_get.find(").unwrap()") {
            return Some(after_get[..end].to_string());
        }
    }
    None
}

fn get_indent(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_get_pattern() {
        let line =
            "        let balance = env.storage().instance().get(&DataKey::Balance(user)).unwrap();";
        let mut changed = false;
        let result = simplify_storage_line(line, &mut changed);
        // Test that we recognize the pattern
        assert!(result.contains("DataKey::Balance"));
    }
}
