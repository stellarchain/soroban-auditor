use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

/// Generate a simple fingerprint for a function body.
/// The algorithm strips comments and whitespace, normalizes identifiers
/// by replacing sequences of letters/digits with a placeholder, and then
/// computes a 64-bit hash (hex string) of the resulting text.
pub fn fingerprint_body(lines: &[String]) -> String {
    let mut normalized = String::new();
    for line in lines {
        // remove comments
        let code = if let Some(idx) = line.find("//") {
            &line[..idx]
        } else {
            &line
        };
        // collapse whitespace
        let mut last_space = false;
        for ch in code.chars() {
            if ch.is_whitespace() {
                if !last_space {
                    normalized.push(' ');
                    last_space = true;
                }
            } else if ch.is_ascii_alphanumeric() || ch == '_' {
                normalized.push('x');
                last_space = false;
            } else {
                normalized.push(ch);
                last_space = false;
            }
        }
        normalized.push('\n');
    }
    // hash
    let mut hasher = ahash::AHasher::default();
    normalized.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Scan a directory recursively for .rs files and build a fingerprint
/// database mapping function names to fingerprint hashes.
pub fn build_fingerprint_db(root: &Path) -> io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    process_file(entry.path(), &mut map)?;
                }
            }
        }
    }
    Ok(map)
}

fn process_file(path: &Path, map: &mut HashMap<String, String>) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    let mut lines = Vec::new();
    let mut in_fn = false;
    let mut brace_depth = 0;
    let mut fn_name = String::new();

    for line in content.lines() {
        if !in_fn {
            // look for fn declaration
            if let Some(pos) = line.find("fn ") {
                // crude name extraction
                let after = &line[pos + 3..];
                let name: String = after
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    fn_name = name;
                    in_fn = true;
                    brace_depth = line.chars().filter(|&c| c == '{').count() as i32
                        - line.chars().filter(|&c| c == '}').count() as i32;
                    lines.clear();
                    continue;
                }
            }
        } else {
            // inside function body
            lines.push(line.to_string());
            brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
            brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;
            if brace_depth <= 0 {
                // end of function
                let fp = fingerprint_body(&lines);
                if !fn_name.is_empty() {
                    map.insert(fn_name.clone(), fp);
                }
                in_fn = false;
                fn_name.clear();
            }
        }
    }
    Ok(())
}

/// Save a fingerprint DB to JSON file.
pub fn save_db(db: &HashMap<String, String>, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(db).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, json)
}

/// Load fingerprint DB from JSON file.
pub fn load_db(path: &Path) -> io::Result<HashMap<String, String>> {
    let data = fs::read_to_string(path)?;
    let map: HashMap<String, String> = serde_json::from_str(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(map)
}

/// Given a function body, compute its fingerprint and look up in the DB.
pub fn match_body<'a>(db: &'a HashMap<String, String>, lines: &[String]) -> Option<&'a String> {
    let fp = fingerprint_body(lines);
    db.iter().find_map(|(name, h)| if *h == fp { Some(name) } else { None })
}

/// Extend an existing fingerprint DB by scanning Rust source files under
/// `root` and adding any helper_*-named functions that aren't already present.
/// The helper name itself (e.g. "helper_3") becomes the value in the map; the
/// fingerprint is the key.
pub fn extend_db_from_decompiled(root: &Path, db: &mut HashMap<String, String>) -> io::Result<()> {
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    // parse file
                    let content = fs::read_to_string(entry.path())?;
                    let mut lines_vec: Vec<String> = Vec::new();
                    let mut in_fn = false;
                    let mut brace_depth = 0;
                    let mut fn_name = String::new();
                    for line in content.lines() {
                        if !in_fn {
                            if let Some(pos) = line.find("fn ") {
                                let after = &line[pos + 3..];
                                let name: String = after
                                    .chars()
                                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                                    .collect();
                                if name.starts_with("helper_") {
                                    fn_name = name.clone();
                                    in_fn = true;
                                    brace_depth = line.chars().filter(|&c| c == '{').count() as i32
                                        - line.chars().filter(|&c| c == '}').count() as i32;
                                    lines_vec.clear();
                                    continue;
                                }
                            }
                        } else {
                            lines_vec.push(line.to_string());
                            brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
                            brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;
                            if brace_depth <= 0 {
                                let fp = fingerprint_body(&lines_vec);
                                db.entry(fp).or_insert(fn_name.clone());
                                in_fn = false;
                                fn_name.clear();
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fp_consistency() {
        let body = vec![
            "let x = a + b;".to_string(),
            "return x;".to_string(),
        ];
        let f1 = fingerprint_body(&body);
        let f2 = fingerprint_body(&body);
        assert_eq!(f1, f2);
    }
    #[test]
    fn test_match() {
        let mut db = HashMap::new();
        db.insert("foo".to_string(), fingerprint_body(&["a".to_string(), "b".to_string()]));
        assert_eq!(match_body(&db, &["a".to_string(), "b".to_string()]), Some(&"foo".to_string()));
    }
}
