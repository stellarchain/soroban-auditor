use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::{HashMap, HashSet};

pub struct CompactTempNamesPattern;

impl CompactTempNamesPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for CompactTempNamesPattern {
    fn name(&self) -> &'static str {
        "compact_temp_names"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut declared: Vec<String> = Vec::new();
        let mut used: HashSet<String> = HashSet::new();
        collect_idents(&block.header, &mut used);
        for line in &block.body {
            collect_idents(line, &mut used);
            if let Some(name) = parse_declared_name(line) {
                if is_plain_var(&name) {
                    declared.push(name);
                }
            }
        }

        if declared.is_empty() {
            return None;
        }

        let mut map: HashMap<String, String> = HashMap::new();
        let mut idx = 0usize;
        for old in declared {
            if map.contains_key(&old) {
                continue;
            }
            let mut candidate = short_name_for_index(idx);
            idx += 1;
            while used.contains(&candidate) || map.values().any(|v| v == &candidate) {
                candidate = short_name_for_index(idx);
                idx += 1;
            }
            map.insert(old, candidate.clone());
            used.insert(candidate);
        }

        if map.is_empty() {
            return None;
        }

        let mut changed = false;
        let new_body: Vec<String> = block
            .body
            .iter()
            .map(|line| {
                let out = replace_idents(line, &map);
                if out != *line {
                    changed = true;
                }
                out
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

fn is_plain_var(name: &str) -> bool {
    if !name.starts_with("var") || name.len() <= 3 {
        return false;
    }
    name[3..].chars().all(|c| c.is_ascii_digit())
}

fn parse_declared_name(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("let ") {
        return None;
    }
    let mut rest = &t[4..];
    if rest.starts_with("mut ") {
        rest = &rest[4..];
    }
    let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn collect_idents(line: &str, out: &mut HashSet<String>) {
    let mut cur = String::new();
    for ch in line.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            cur.push(ch);
        } else if !cur.is_empty() {
            out.insert(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.insert(cur);
    }
}

fn replace_idents(line: &str, map: &HashMap<String, String>) -> String {
    let mut out = String::with_capacity(line.len());
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let token: String = chars[start..i].iter().collect();
            if let Some(repl) = map.get(&token) {
                out.push_str(repl);
            } else {
                out.push_str(&token);
            }
            continue;
        }
        out.push(ch);
        i += 1;
    }
    out
}

fn short_name_for_index(mut idx: usize) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyz";
    let mut buf: Vec<u8> = Vec::new();
    loop {
        buf.push(alphabet[idx % 26]);
        if idx < 26 {
            break;
        }
        idx = idx / 26 - 1;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap_or_else(|_| "v".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renames_plain_vars_to_short_names() {
        let pattern = CompactTempNamesPattern::new();
        let block = FunctionBlock {
            header: "pub fn test(&mut self, env: Env) {".to_string(),
            body: vec![
                "    let mut var1: i32 = 0;".to_string(),
                "    let mut var2: i64 = 0;".to_string(),
                "    var1 = var1.wrapping_add(1);".to_string(),
                "    var2 = var2.wrapping_add(var1 as i64);".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "test".to_string(),
        };

        let result = pattern.apply(&block).expect("should rename");
        assert!(result.body.iter().any(|l| l.contains("let mut a: i32")));
        assert!(result.body.iter().any(|l| l.contains("let mut b: i64")));
        assert!(result.body.iter().any(|l| l.contains("a = a.wrapping_add(1);")));
        assert!(result.body.iter().any(|l| l.contains("b = b.wrapping_add(a as i64);")));
    }
}
