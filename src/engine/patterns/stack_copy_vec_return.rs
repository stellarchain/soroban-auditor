use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct StackCopyVecReturnPattern;

impl StackCopyVecReturnPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for StackCopyVecReturnPattern {
    fn name(&self) -> &'static str {
        "stack_copy_vec_return"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 8 {
            return None;
        }

        let converter = detect_vec_element_converter(&block.header)?;
        let mut new_body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 2 < new_body.len() {
            let line = new_body[i].trim();
            let next = new_body[i + 1].trim();
            if !line.starts_with("let mut ")
                || !line.contains(" = ")
                || !line.ends_with(" as i64;")
                || !next.starts_with("let mut ")
                || !next.contains(" = ")
                || !next.ends_with(" as i64;")
            {
                i += 1;
                continue;
            }

            let loop_start = find_loop_start(&new_body, i + 2);
            let Some(loop_start) = loop_start else {
                i += 1;
                continue;
            };
            let Some(loop_end) = find_block_end(&new_body, loop_start) else {
                i += 1;
                continue;
            };

            let Some((arg1, arg2)) =
                find_vec_build_args(&new_body[loop_start..=loop_end], &new_body[..i])
            else {
                i += 1;
                continue;
            };

            let indent = new_body[loop_start]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let replacement = format!(
                "{}return vec![&env, {}, {}];",
                indent,
                converter(&arg1),
                converter(&arg2)
            );

            new_body.splice(loop_start..=loop_end, vec![replacement]);
            changed = true;
            i += 1;
        }

        // Fallback: collapse temp vec builder + assign + return into direct vec return.
        let mut j = 0usize;
        while j + 2 < new_body.len() {
            let line = new_body[j].trim();
            let next = new_body[j + 1].trim();
            let next2 = new_body[j + 2].trim();
            let Some((tmp_name, raw_arg1, raw_arg2)) = parse_vec_builder_assignment(line) else {
                j += 1;
                continue;
            };
            let prefix_aliases = build_aliases_from_prefix(&new_body[..j]);
            let arg1 = resolve_alias(&raw_arg1, &prefix_aliases);
            let arg2 = resolve_alias(&raw_arg2, &prefix_aliases);
            let Some(assigned_name) = parse_simple_assignment(next) else {
                j += 1;
                continue;
            };
            if assigned_name.1 != tmp_name {
                j += 1;
                continue;
            }
            let Some(ret_name) = parse_simple_return(next2) else {
                j += 1;
                continue;
            };
            if ret_name != assigned_name.0 {
                j += 1;
                continue;
            }

            let indent = new_body[j]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let replacement = format!(
                "{}return vec![&env, {}, {}];",
                indent,
                converter(&arg1),
                converter(&arg2)
            );
            new_body.splice(j..=j + 2, vec![replacement]);
            changed = true;
            j += 1;
        }

        // Collapse stack-copy loops that only guard a single vec return.
        let mut k = 0usize;
        while k < new_body.len() {
            let Some(loop_start) = find_loop_start(&new_body, k) else {
                break;
            };
            let Some(loop_end) = find_block_end(&new_body, loop_start) else {
                break;
            };
            let loop_slice = &new_body[loop_start..=loop_end];
            let return_lines: Vec<String> = loop_slice
                .iter()
                .map(|l| l.trim().to_string())
                .filter(|l| l.starts_with("return vec![&env,"))
                .collect();
            if return_lines.len() == 1 && is_stack_copy_noise_loop(loop_slice) {
                let indent = new_body[loop_start]
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                let replacement = format!("{}{}", indent, return_lines[0]);
                new_body.splice(loop_start..=loop_end, vec![replacement]);
                changed = true;
                k = loop_start + 1;
                continue;
            }
            k = loop_end + 1;
        }

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

fn detect_vec_element_converter(
    header: &str,
) -> Option<Box<dyn Fn(&str) -> String + Send + Sync + 'static>> {
    let string_params = find_params_with_types(header, &["soroban_sdk::String", "String"]);
    let symbol_params = find_params_with_types(header, &["soroban_sdk::Symbol", "Symbol"]);

    if header.contains("Vec<soroban_sdk::String>") || header.contains("Vec<String>") {
        let string_params = string_params.clone();
        return Some(Box::new(move |v: &str| {
            if string_params.iter().any(|p| p == v) {
                return v.to_string();
            }
            format!("String::from_val(env, &val_from_i64({}))", v)
        }));
    }
    if header.contains("Vec<soroban_sdk::Symbol>") || header.contains("Vec<Symbol>") {
        let symbol_params = symbol_params.clone();
        return Some(Box::new(move |v: &str| {
            if symbol_params.iter().any(|p| p == v) {
                return v.to_string();
            }
            format!("Symbol::from_val(env, &val_from_i64({}))", v)
        }));
    }
    None
}

fn find_params_with_types(header: &str, ty_markers: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for line in header.lines() {
        let trimmed = line.trim().trim_end_matches(',');
        let Some((name, ty)) = trimmed.split_once(':') else {
            continue;
        };
        let param_name = name.trim();
        let param_ty = ty.trim();
        if param_name.is_empty() {
            continue;
        }
        if ty_markers.iter().any(|m| param_ty.contains(m)) {
            out.push(param_name.to_string());
        }
    }
    out
}

fn find_loop_start(lines: &[String], from: usize) -> Option<usize> {
    for (idx, line) in lines.iter().enumerate().skip(from) {
        if line.trim() == "loop {" {
            return Some(idx);
        }
    }
    None
}

fn find_block_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut opened = false;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if opens > 0 {
            opened = true;
        }
        depth += opens - closes;
        if opened && depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn find_vec_build_args(lines: &[String], prefix: &[String]) -> Option<(String, String)> {
    let mut aliases = std::collections::HashMap::<String, String>::new();
    let prefix_start = prefix.len().saturating_sub(80);
    for line in prefix.iter().skip(prefix_start) {
        if let Some((lhs, rhs)) = parse_alias_assignment(line.trim()) {
            aliases.insert(lhs.to_string(), rhs.to_string());
        }
    }
    for line in lines {
        if let Some((lhs, rhs)) = parse_alias_assignment(line.trim()) {
            aliases.insert(lhs.to_string(), rhs.to_string());
        }
    }

    for line in lines {
        let trimmed = line.trim();
        if !trimmed.contains("let mut v = Vec::<Val>::new(env);") {
            continue;
        }
        let mut args = Vec::new();
        let mut rest = trimmed;
        while let Some(pos) = rest.find("v.push_back(val_from_i64(") {
            let start = pos + "v.push_back(val_from_i64(".len();
            let tail = &rest[start..];
            let end = tail.find("))")?;
            args.push(tail[..end].trim().to_string());
            rest = &tail[end + 2..];
        }
        if args.len() >= 2 {
            let resolved1 = resolve_alias(&args[0], &aliases);
            let resolved2 = resolve_alias(&args[1], &aliases);
            if !is_valid_vec_arg(&resolved1) || !is_valid_vec_arg(&resolved2) {
                return None;
            }
            if !is_known_in_prefix(&resolved1, prefix)
                && !is_simple_expr_identifier_or_number(&resolved1)
            {
                return None;
            }
            if !is_known_in_prefix(&resolved2, prefix)
                && !is_simple_expr_identifier_or_number(&resolved2)
            {
                return None;
            }
            return Some((resolved1, resolved2));
        }
    }
    None
}

fn parse_alias_assignment(line: &str) -> Option<(&str, &str)> {
    let mut lhs_rhs = None;
    if let Some(rest) = line.strip_prefix("let mut ") {
        lhs_rhs = rest.split_once(" = ");
    } else if let Some(rest) = line.strip_prefix("let ") {
        lhs_rhs = rest.split_once(" = ");
    } else if line.contains(" = ") {
        lhs_rhs = line.split_once(" = ");
    }
    let (lhs_raw, rhs_raw) = lhs_rhs?;
    let lhs = lhs_raw.trim().split(':').next()?.trim();
    if lhs.is_empty() {
        return None;
    }
    let mut rhs = rhs_raw.trim().trim_end_matches(';').trim();
    if let Some(stripped) = rhs.strip_suffix(" as i64") {
        rhs = stripped.trim();
    }
    if rhs.is_empty() {
        return None;
    }
    Some((lhs, rhs))
}

fn resolve_alias(arg: &str, aliases: &std::collections::HashMap<String, String>) -> String {
    let mut cur = arg.trim().to_string();
    for _ in 0..8 {
        let Some(next) = aliases.get(&cur) else {
            break;
        };
        if next == &cur {
            break;
        }
        if !is_simple_expr_identifier_or_number(next) {
            break;
        }
        cur = next.clone();
    }
    cur
}

fn is_valid_vec_arg(arg: &str) -> bool {
    if arg.is_empty() {
        return false;
    }
    if arg.starts_with("slot_var") {
        return false;
    }
    if arg.starts_with("vec_builder") {
        return false;
    }
    true
}

fn is_known_in_prefix(arg: &str, prefix: &[String]) -> bool {
    if arg.parse::<i64>().is_ok() {
        return true;
    }
    if arg == "env" {
        return true;
    }
    for line in prefix.iter().rev() {
        let trimmed = line.trim();
        if let Some((lhs, _rhs)) = parse_alias_assignment(trimmed) {
            if lhs == arg {
                return true;
            }
        }
        if trimmed.contains(&format!("{arg}:")) {
            return true;
        }
    }
    false
}

fn is_simple_expr_identifier_or_number(expr: &str) -> bool {
    if expr.parse::<i64>().is_ok() {
        return true;
    }
    let mut chars = expr.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn parse_vec_builder_assignment(line: &str) -> Option<(String, String, String)> {
    let (lhs, rhs) = line.strip_prefix("let ")?.split_once(" = ")?;
    let tmp_name = lhs.trim().split(':').next()?.trim().to_string();
    if !rhs.contains("let mut v = Vec::<Val>::new(env);") {
        return None;
    }
    let mut args = Vec::new();
    let mut rest = rhs;
    while let Some(pos) = rest.find("v.push_back(val_from_i64(") {
        let start = pos + "v.push_back(val_from_i64(".len();
        let tail = &rest[start..];
        let end = tail.find("))")?;
        args.push(tail[..end].trim().to_string());
        rest = &tail[end + 2..];
    }
    if args.len() < 2 {
        return None;
    }
    Some((tmp_name, args[0].clone(), args[1].clone()))
}

fn parse_simple_assignment(line: &str) -> Option<(String, String)> {
    let (lhs, rhs) = line.split_once(" = ")?;
    let lhs = lhs.trim().to_string();
    let rhs = rhs.trim().trim_end_matches(';').trim().to_string();
    Some((lhs, rhs))
}

fn parse_simple_return(line: &str) -> Option<String> {
    let ret = line.strip_prefix("return ")?;
    Some(ret.trim().trim_end_matches(';').trim().to_string())
}

fn build_aliases_from_prefix(prefix: &[String]) -> std::collections::HashMap<String, String> {
    let mut aliases = std::collections::HashMap::<String, String>::new();
    let start = prefix.len().saturating_sub(80);
    for line in prefix.iter().skip(start) {
        if let Some((lhs, rhs)) = parse_alias_assignment(line.trim()) {
            aliases.insert(lhs.to_string(), rhs.to_string());
        }
    }
    aliases
}

fn is_stack_copy_noise_loop(lines: &[String]) -> bool {
    let mut has_mload = false;
    let mut has_mstore = false;
    for raw in lines {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("return vec![&env,") {
            continue;
        }
        if line.contains("mload64!(") {
            has_mload = true;
        }
        if line.contains("mstore64!(") {
            has_mstore = true;
        }
        if line == "loop {"
            || line == "{"
            || line == "}"
            || line == "continue;"
            || line == "break;"
            || line.starts_with("if ")
            || line.starts_with("if(")
            || line.starts_with("if (")
            || line.starts_with("let ")
            || line.starts_with("let mut ")
            || line.contains(" = ")
            || line.starts_with("mload")
            || line.starts_with("mstore")
        {
            continue;
        }
        if line.starts_with("self.") || line.contains("env.") {
            return false;
        }
    }
    has_mload && has_mstore
}
