use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

/// Pattern pentru îmbunătățirea numelor de variabile
///
/// Transformă:
/// - `var1`, `var2` → nume sugestive bazate pe tip și utilizare
/// - `slot_varX_Y_i64` → nume mai clare
pub struct VariableNamingPattern;

impl VariableNamingPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for VariableNamingPattern {
    fn name(&self) -> &'static str {
        "variable_naming"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        // Analizează toate variabilele și contextul lor
        let var_suggestions = analyze_variables(&block.body);

        if var_suggestions.is_empty() {
            return None;
        }

        let nodes = parse_lines(&block.body);
        let mut changed = false;
        let new_nodes = rename_variables(nodes, &var_suggestions, &mut changed);

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

fn analyze_variables(lines: &[String]) -> HashMap<String, String> {
    let mut suggestions = HashMap::new();

    for line in lines {
        // Caută declarații de variabile
        if let Some((var_name, suggested_name)) = analyze_line(line) {
            // Evită suprascrierea dacă deja avem o sugestie bună
            if !suggestions.contains_key(&var_name) {
                suggestions.insert(var_name, suggested_name);
            }
        }
    }

    suggestions
}

fn analyze_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();

    // Pattern 1: let mut var1: Address = ...
    if trimmed.starts_with("let ") && trimmed.contains(": ") {
        if let Some(var_name) = extract_var_name(trimmed) {
            if var_name.starts_with("var") || var_name.starts_with("slot_") {
                if let Some(type_hint) = extract_type(trimmed) {
                    let suggested = suggest_name_from_type(&type_hint, trimmed);
                    return Some((var_name, suggested));
                }
            }
        }
    }

    // Pattern 2: Variabile folosite în context specific
    // Ex: var1.require_auth() → ar putea fi "user" sau "from"
    if trimmed.contains(".require_auth()") {
        if let Some(var_name) = extract_subject_before(trimmed, ".require_auth()") {
            if var_name.starts_with("var") {
                return Some((var_name, "user".to_string()));
            }
        }
    }

    // Pattern 3: Variabile folosite ca key în storage
    // Ex: DataKey::Balance(var1) → var1 ar putea fi "address" sau "user"
    if trimmed.contains("DataKey::") {
        if let Some((var_name, key_type)) = extract_datakey_param(trimmed) {
            if var_name.starts_with("var") {
                let suggested = match key_type.as_str() {
                    "Balance" | "Allowance" => "address",
                    "Admin" => "admin",
                    "Launch" => "launch_id",
                    _ => "key",
                };
                return Some((var_name.to_string(), suggested.to_string()));
            }
        }
    }

    // Pattern 4: Storage operation results
    // Ex: let var1 = env.storage().instance().get(...) → "key" or "value"
    if trimmed.starts_with("let ") && trimmed.contains("env.storage()") {
        if let Some(var_name) = extract_var_name_simple(trimmed) {
            if var_name.starts_with("var") {
                if trimmed.contains(".get(") {
                    return Some((var_name, "stored_value".to_string()));
                } else if trimmed.contains(".set(") {
                    return Some((var_name, "result".to_string()));
                }
            }
        }
    }

    // Pattern 5: Match expressions
    // Ex: let var2 = match 0 { ... } → "match_result"
    if trimmed.starts_with("let ") && trimmed.contains("= match ") {
        if let Some(var_name) = extract_var_name_simple(trimmed) {
            if var_name.starts_with("var") {
                return Some((var_name, "match_result".to_string()));
            }
        }
    }

    // Pattern 6: Function call results (likely storage keys or conversions)
    // Ex: let var1 = self.func18(...) → "key" or "converted_value"
    if trimmed.starts_with("let ") && trimmed.contains("self.func") {
        if let Some(var_name) = extract_var_name_simple(trimmed) {
            if var_name.starts_with("var") {
                // If the number looks like a memory address (1056, 1048680, etc.), likely a key builder
                if trimmed.contains(", 10") {
                    return Some((var_name, "storage_key".to_string()));
                } else {
                    return Some((var_name, "helper_result".to_string()));
                }
            }
        }
    }

    // Pattern 7: val_from_i64 conversions
    // Ex: let var3 = val_from_i64(...) → "converted_val"
    if trimmed.starts_with("let ") && trimmed.contains("val_from_i64") {
        if let Some(var_name) = extract_var_name_simple(trimmed) {
            if var_name.starts_with("var") {
                return Some((var_name, "converted_val".to_string()));
            }
        }
    }

    None
}

fn extract_var_name_simple(line: &str) -> Option<String> {
    // Extract var name from: let var_name = ... (without type annotation)
    let after_let = line.strip_prefix("let ")?.trim_start();
    let after_let = after_let.strip_prefix("mut ")?.trim_start();

    let name: String = after_let
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn extract_var_name(line: &str) -> Option<String> {
    // Extract var name from: let mut var_name: Type = ...
    let after_let = line.strip_prefix("let ")?.trim_start();
    let after_let = after_let.strip_prefix("mut ")?.trim_start();

    let name: String = after_let
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn extract_type(line: &str) -> Option<String> {
    // Extract type from: let var: Type = ...
    if let Some(colon_pos) = line.find(':') {
        let after_colon = &line[colon_pos + 1..];
        let type_str: String = after_colon
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':' || *c == '<' || *c == '>')
            .collect();

        if !type_str.is_empty() {
            return Some(type_str);
        }
    }
    None
}

fn suggest_name_from_type(type_hint: &str, context: &str) -> String {
    match type_hint {
        t if t.contains("Address") => {
            // Context-aware naming pentru Address
            if context.contains("from") {
                "from".to_string()
            } else if context.contains("to") {
                "to".to_string()
            } else if context.contains("admin") {
                "admin".to_string()
            } else if context.contains("owner") {
                "owner".to_string()
            } else if context.contains("user") {
                "user".to_string()
            } else {
                "address".to_string()
            }
        }
        t if t.contains("i128") => {
            if context.contains("amount") {
                "amount".to_string()
            } else if context.contains("balance") {
                "balance".to_string()
            } else if context.contains("fee") {
                "fee".to_string()
            } else if context.contains("price") {
                "price".to_string()
            } else {
                "value".to_string()
            }
        }
        t if t.contains("u64") => {
            if context.contains("timestamp") {
                "timestamp".to_string()
            } else if context.contains("ledger") {
                "ledger".to_string()
            } else if context.contains("id") {
                "id".to_string()
            } else {
                "number".to_string()
            }
        }
        t if t.contains("bool") => "flag".to_string(),
        t if t.contains("String") => "text".to_string(),
        t if t.contains("Symbol") => "symbol".to_string(),
        t if t.contains("Vec") => "items".to_string(),
        t if t.contains("Map") => "map".to_string(),
        _ => "value".to_string(),
    }
}

fn extract_subject_before(line: &str, marker: &str) -> Option<String> {
    if let Some(pos) = line.find(marker) {
        let before = &line[..pos];
        // Extract last identifier before marker
        let chars: Vec<char> = before.chars().collect();
        let mut ident = String::new();
        for &ch in chars.iter().rev() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.insert(0, ch);
            } else if !ident.is_empty() {
                break;
            }
        }
        if !ident.is_empty() {
            return Some(ident);
        }
    }
    None
}

fn extract_datakey_param(line: &str) -> Option<(String, String)> {
    // Extract from: DataKey::Balance(var1)
    if let Some(datakey_pos) = line.find("DataKey::") {
        let after = &line[datakey_pos + 9..];
        if let Some(open_paren) = after.find('(') {
            let key_type = &after[..open_paren];
            // Look for close paren AFTER open paren
            let after_open = &after[open_paren + 1..];
            if let Some(close_paren) = after_open.find(')') {
                let param = &after_open[..close_paren];
                let param = param.trim();
                if param.starts_with("var") {
                    return Some((param.to_string(), key_type.to_string()));
                }
            }
        }
    }
    None
}

fn rename_variables(
    nodes: Vec<Node>,
    suggestions: &HashMap<String, String>,
    changed: &mut bool,
) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();

    for node in nodes {
        match node {
            Node::Line(line) => {
                let new_line = rename_in_line(&line, suggestions, changed);
                out.push(Node::Line(new_line));
            }
            Node::Block {
                kind,
                label,
                header,
                body,
                footer,
            } => {
                let new_body = rename_variables(body, suggestions, changed);
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

fn rename_in_line(line: &str, suggestions: &HashMap<String, String>, changed: &mut bool) -> String {
    let mut result = line.to_string();

    for (old_name, new_name) in suggestions {
        // Only rename whole words (not substrings)
        // Escape special regex characters in old_name
        let escaped = old_name
            .replace('\\', "\\\\")
            .replace('.', "\\.")
            .replace('*', "\\*")
            .replace('+', "\\+")
            .replace('?', "\\?")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('^', "\\^")
            .replace('$', "\\$")
            .replace('|', "\\|");

        let pattern = format!(r"\b{}\b", escaped);
        if let Ok(re) = regex::Regex::new(&pattern) {
            let new_result = re.replace_all(&result, new_name.as_str()).to_string();
            if new_result != result {
                *changed = true;
                result = new_result;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_type_suggestion() {
        let line = "let mut var1: Address = address_from_i64(env, user);";
        let (var, suggested) = analyze_line(line).unwrap();
        assert_eq!(var, "var1");
        assert_eq!(suggested, "user");
    }

    #[test]
    fn test_i128_amount_suggestion() {
        let line = "let mut var2: i128 = amount;";
        let (var, suggested) = analyze_line(line).unwrap();
        assert_eq!(var, "var2");
        assert_eq!(suggested, "amount");
    }

    #[test]
    fn test_require_auth_context() {
        let line = "        var1.require_auth();";
        let (var, suggested) = analyze_line(line).unwrap();
        assert_eq!(var, "var1");
        assert_eq!(suggested, "user");
    }
}
