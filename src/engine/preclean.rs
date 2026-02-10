/// Early, syntax-only normalization pass executed before function splitting/patterns.
/// This keeps downstream patterns focused on semantics, not formatting artifacts.
pub fn run(input: String) -> String {
    let unified = input.replace("\r\n", "\n");
    let mut out: Vec<String> = Vec::new();
    let mut prev_blank = false;

    for raw in unified.lines() {
        let mut line = raw.trim_end().to_string();

        // Guard against malformed multiline call heads that sometimes get emitted as
        // `callee(;` by downstream text transforms. This is syntax-only cleanup.
        if line.trim_end().ends_with("(;") {
            line = line.replacen("(;", "(", 1);
        }

        // Canonicalize detached else form as early as possible.
        if line.trim() == "else {" && out.last().map(|l| l.trim() == "}").unwrap_or(false) {
            if let Some(last) = out.pop() {
                let indent = last.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                out.push(format!("{indent}}} else {{"));
                prev_blank = false;
                continue;
            }
        }

        // Normalize arrow spacing for malformed signatures encountered in output text passes.
        if line.contains(")->") {
            line = line.replace(")->", ") ->");
        }
        line = normalize_bool_cast_line(&line);
        line = normalize_select_block_line(&line);

        let is_blank = line.trim().is_empty();
        if is_blank && prev_blank {
            continue;
        }
        prev_blank = is_blank;
        out.push(line);
    }

    out.join("\n")
}

fn normalize_bool_cast_line(line: &str) -> String {
    let trimmed = line.trim();
    if let Some(prefix) = trimmed.strip_prefix("if ").and_then(|s| s.strip_suffix(" {")) {
        let normalized = normalize_numeric_truthy_condition(&normalize_bool_cast_expr(prefix));
        let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
        return format!("{indent}if {normalized} {{");
    }
    if let Some(prefix) = trimmed.strip_prefix("while ").and_then(|s| s.strip_suffix(" {")) {
        let normalized = normalize_numeric_truthy_condition(&normalize_bool_cast_expr(prefix));
        let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
        return format!("{indent}while {normalized} {{");
    }
    if let Some(prefix) = trimmed.strip_prefix("assert!(").and_then(|s| s.strip_suffix(");")) {
        let normalized = normalize_numeric_truthy_condition(&normalize_bool_cast_expr(prefix));
        let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
        return format!("{indent}assert!({normalized});");
    }
    line.to_string()
}

fn normalize_bool_cast_expr(expr: &str) -> String {
    let mut cur = expr.to_string();
    loop {
        let next = simplify_bool_cast_once(&simplify_triple_negation_once(&cur));
        if next == cur {
            break;
        }
        cur = next;
    }
    cur
}

fn simplify_triple_negation_once(expr: &str) -> String {
    let t = expr.trim();
    if t.starts_with("!(!(!(") && t.ends_with(")))") {
        let inner = &t[6..t.len() - 3];
        return format!("!({})", inner.trim());
    }
    expr.to_string()
}

fn normalize_select_block_line(line: &str) -> String {
    // Rewrite wasm-style select blocks on a single line:
    // { let a = X; let b = Y; if C { a } else { b } } -> (if C { X } else { Y })
    let mut cur = line.to_string();
    loop {
        let Some(start) = cur.find("{ let ") else {
            break;
        };
        let Some(end) = find_balanced_block_end(&cur, start) else {
            break;
        };
        let candidate = &cur[start..end];
        let Some(rewritten) = rewrite_select_block(candidate) else {
            break;
        };
        cur.replace_range(start..end, &rewritten);
    }
    cur
}

fn rewrite_select_block(block: &str) -> Option<String> {
    // Strict parser for:
    // { let a = <expr>; let b = <expr>; if <cond> { a } else { b } }
    let s = block.trim();
    let s = s.strip_prefix('{')?.strip_suffix('}')?.trim();
    let s = s.strip_prefix("let ")?;
    let (a_name, s) = split_once_trim(s, "=")?;
    let (a_expr, s) = split_once_trim(s, ";")?;
    let s = s.strip_prefix("let ")?;
    let (b_name, s) = split_once_trim(s, "=")?;
    let (b_expr, s) = split_once_trim(s, ";")?;
    let s = s.strip_prefix("if ")?;
    let (cond, s) = split_once_trim(s, "{")?;
    let (if_ret, s) = split_once_trim(s, "}")?;
    let s = s.trim();
    let s = s.strip_prefix("else")?.trim();
    let s = s.strip_prefix("{")?.trim();
    let (else_ret, s) = split_once_trim(s, "}")?;
    if !s.trim().is_empty() {
        return None;
    }

    if a_name != if_ret || b_name != else_ret {
        return None;
    }

    Some(format!(
        "(if {} {{ {} }} else {{ {} }})",
        cond.trim(),
        a_expr.trim(),
        b_expr.trim()
    ))
}

fn find_balanced_block_end(s: &str, start: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    if start >= bytes.len() || bytes[start] != b'{' {
        return None;
    }
    let mut depth = 0i32;
    for (i, b) in bytes.iter().enumerate().skip(start) {
        if *b == b'{' {
            depth += 1;
        } else if *b == b'}' {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
    }
    None
}

fn split_once_trim<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let idx = s.find(sep)?;
    let left = s[..idx].trim();
    let right = s[idx + sep.len()..].trim();
    Some((left, right))
}

fn simplify_bool_cast_once(expr: &str) -> String {
    let mut s = expr.to_string();
    for (pat, negate) in [(" as i32 != 0", false), (" as i32 == 0", true)] {
        while let Some(pos) = s.find(pat) {
            let end_before = pos;
            let mut idx = end_before;
            while idx > 0 && s.as_bytes()[idx - 1].is_ascii_whitespace() {
                idx -= 1;
            }
            if idx == 0 || s.as_bytes()[idx - 1] != b')' {
                break;
            }
            let close = idx - 1;
            let Some(open) = find_matching_open_paren(&s, close) else {
                break;
            };
            let inner = s[open + 1..close].trim();
            let replacement = if negate {
                format!("!({inner})")
            } else {
                inner.to_string()
            };
            let range_end = pos + pat.len();
            s.replace_range(open..range_end, &replacement);
        }
    }
    s = simplify_simple_token_cast(&s, " as i32 != 0", false);
    s = simplify_simple_token_cast(&s, " as i32 == 0", true);
    s
}

fn find_matching_open_paren(s: &str, close_idx: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    if close_idx >= bytes.len() || bytes[close_idx] != b')' {
        return None;
    }
    let mut depth = 0i32;
    for i in (0..=close_idx).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn simplify_simple_token_cast(input: &str, pat: &str, negate: bool) -> String {
    let mut s = input.to_string();
    while let Some(pos) = s.find(pat) {
        let start = match find_token_start(&s, pos) {
            Some(v) => v,
            None => break,
        };
        if start == pos {
            break;
        }
        let token = s[start..pos].trim();
        if token.is_empty() || token.contains(' ') {
            break;
        }
        let replacement = if negate {
            format!("!({token})")
        } else {
            format!("({token} != 0)")
        };
        s.replace_range(start..pos + pat.len(), &replacement);
    }
    s
}

fn find_token_start(s: &str, end: usize) -> Option<usize> {
    if end == 0 || end > s.len() {
        return None;
    }
    let bytes = s.as_bytes();
    let mut i = end;
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    let token_end = i;
    while i > 0 {
        let c = bytes[i - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            i -= 1;
        } else {
            break;
        }
    }
    if i == token_end {
        None
    } else {
        Some(i)
    }
}

fn normalize_numeric_truthy_condition(cond: &str) -> String {
    let trimmed = cond.trim();
    if trimmed.is_empty() {
        return cond.to_string();
    }
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_string();
    }
    if let Some(inner) = trimmed.strip_prefix('!') {
        let inner = inner.trim().trim_start_matches('(').trim_end_matches(')');
        if is_simple_ident(inner) {
            return format!("{inner} == 0");
        }
    }
    if is_simple_ident(trimmed) {
        return format!("{trimmed} != 0");
    }
    if let Some(inner) = trimmed.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        let inner = inner.trim();
        if is_simple_ident(inner) {
            return format!("{inner} != 0");
        }
    }
    cond.to_string()
}

fn is_simple_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
