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

            let mut ret_idx: Option<usize> = None;
            let lookahead_end = usize::min(new_body.len(), j + 7);
            for idx in (j + 2)..lookahead_end {
                let cur = new_body[idx].trim();
                if let Some(ret_name) = parse_simple_return(cur) {
                    if ret_name == assigned_name.0 {
                        ret_idx = Some(idx);
                    }
                    break;
                }
                if cur.contains(&tmp_name) || cur.contains(&assigned_name.0) {
                    ret_idx = None;
                    break;
                }
            }
            let Some(ret_idx) = ret_idx else {
                j += 1;
                continue;
            };

            let indent = new_body[ret_idx]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let replacement = format!(
                "{}return vec![&env, {}, {}];",
                indent,
                converter(&arg1),
                converter(&arg2)
            );
            new_body.remove(j + 1); // remove alias assignment (e.g., to = var5)
            new_body.remove(j); // remove temp vec builder
            let adj_ret_idx = ret_idx - 2;
            new_body[adj_ret_idx] = replacement;
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

        // Collapse labeled stack-copy loops that guard a vec-return path:
        // 'labelX: loop { if ... { (copy loop) ; global0=...; return vec![..] } ... }
        let mut p = 0usize;
        while p < new_body.len() {
            let trimmed = new_body[p].trim();
            if !trimmed.starts_with("'label") || !trimmed.ends_with("loop {") {
                p += 1;
                continue;
            }
            let Some(loop_end) = find_block_end(&new_body, p) else {
                p += 1;
                continue;
            };
            let slice = &new_body[p..=loop_end];
            if let Some(replacement) = collapse_labeled_stack_copy_vec_loop(slice) {
                new_body.splice(p..=loop_end, replacement);
                changed = true;
                p += 1;
                continue;
            }
            p = loop_end + 1;
        }

        // Fallback: detect stack-seeded pair (`slot_var*_0_i64`, `slot_var*_8_i64`)
        // followed by empty Vec construction and direct return.
        if let Some((a0, a1, ret_start, ret_end)) = find_slot_seeded_vec_return(&new_body) {
            let indent = new_body[ret_start]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let replacement = format!(
                "{}return vec![&env, {}, {}];",
                indent,
                converter(&a0),
                converter(&a1)
            );
            new_body.splice(ret_start..=ret_end, vec![replacement]);
            changed = true;
        }

        // Generic fallback:
        // `let t = { let mut v = Vec::<Val>::new(env); ... }; x = t; ... return x;`
        // -> `return vec![&env, ...];`
        let mut q = 0usize;
        while q < new_body.len() {
            let Some((tmp_name, raw_args)) = parse_vec_builder_args_line(new_body[q].trim()) else {
                q += 1;
                continue;
            };
            if raw_args.is_empty() {
                q += 1;
                continue;
            }

            let prefix_aliases = build_aliases_from_prefix(&new_body[..q]);
            let resolved_args: Vec<String> = raw_args
                .iter()
                .map(|a| resolve_alias(a, &prefix_aliases))
                .collect();

            let mut assign_idx: Option<usize> = None;
            let mut ret_target = tmp_name.clone();
            for idx in (q + 1)..usize::min(new_body.len(), q + 4) {
                if let Some((lhs, rhs)) = parse_simple_assignment(new_body[idx].trim()) {
                    if rhs == tmp_name {
                        assign_idx = Some(idx);
                        ret_target = lhs;
                        break;
                    }
                }
            }

            let mut ret_idx: Option<usize> = None;
            let search_start = assign_idx.unwrap_or(q) + 1;
            for idx in search_start..usize::min(new_body.len(), q + 14) {
                if let Some(ret_name) = parse_simple_return(new_body[idx].trim()) {
                    if ret_name == ret_target || ret_name == tmp_name {
                        ret_idx = Some(idx);
                    }
                    break;
                }
            }
            let Some(ret_idx) = ret_idx else {
                q += 1;
                continue;
            };

            let indent = new_body[ret_idx]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let items = resolved_args
                .iter()
                .map(|a| converter(a))
                .collect::<Vec<_>>()
                .join(", ");
            new_body[ret_idx] = format!("{indent}return vec![&env, {items}];");

            if let Some(aidx) = assign_idx {
                new_body.remove(aidx);
                if ret_idx > aidx {
                    let adj = ret_idx - 1;
                    if q < aidx {
                        new_body.remove(q);
                        let _ = adj - 1;
                    } else {
                        new_body.remove(q);
                    }
                } else {
                    new_body.remove(q);
                }
            } else {
                new_body.remove(q);
            }
            changed = true;
            continue;
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

fn find_slot_seeded_vec_return(lines: &[String]) -> Option<(String, String, usize, usize)> {
    let mut slot0: Option<String> = None;
    let mut slot8: Option<String> = None;
    let mut slot_base: Option<String> = None;
    let mut vec_line_idx: Option<usize> = None;
    let mut ret_line_idx: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.contains(" = val_to_i64(Vec::<Val>::new(env).into_val(env))") {
            vec_line_idx = Some(i);
        }
        if t.starts_with("return ") && t.ends_with(';') {
            ret_line_idx = Some(i);
        }
        if let Some((lhs, rhs)) = parse_simple_assignment(t) {
            if is_stack_slot_name(&lhs) && lhs.ends_with("_0_i64") && rhs.ends_with(" as i64") {
                let base = lhs.trim_end_matches("_0_i64");
                slot_base = Some(base.to_string());
                slot0 = Some(rhs.trim_end_matches(" as i64").trim().to_string());
            } else if is_stack_slot_name(&lhs)
                && lhs.ends_with("_8_i64")
                && rhs.ends_with(" as i64")
            {
                let base = lhs.trim_end_matches("_8_i64");
                slot_base = Some(base.to_string());
                slot8 = Some(rhs.trim_end_matches(" as i64").trim().to_string());
            }
        }
    }

    let vec_idx = vec_line_idx?;
    let ret_idx = ret_line_idx?;
    if ret_idx <= vec_idx {
        return None;
    }
    let base = slot_base?;
    let has_pair = lines.iter().any(|l| l.trim().starts_with(&format!("{base}_0_i64")))
        && lines.iter().any(|l| l.trim().starts_with(&format!("{base}_8_i64")));
    if !has_pair {
        return None;
    }

    Some((slot0?, slot8?, vec_idx, ret_idx))
}

fn detect_vec_element_converter(
    header: &str,
) -> Option<Box<dyn Fn(&str) -> String + Send + Sync + 'static>> {
    let string_params = find_params_with_types(header, &["soroban_sdk::String", "String"]);
    let symbol_params = find_params_with_types(header, &["soroban_sdk::Symbol", "Symbol"]);

    if header.contains("Vec<soroban_sdk::String>") || header.contains("Vec<String>") {
        let string_params = string_params.clone();
        return Some(Box::new(move |v: &str| {
            if v.starts_with("String::from_str(") || v.starts_with("String::from_val(") {
                return v.to_string();
            }
            if string_params.iter().any(|p| p == v) {
                return v.to_string();
            }
            format!("String::from_val(env, &val_from_i64({}))", v)
        }));
    }
    if header.contains("Vec<soroban_sdk::Symbol>") || header.contains("Vec<Symbol>") {
        let symbol_params = symbol_params.clone();
        return Some(Box::new(move |v: &str| {
            if v.starts_with("Symbol::from_str(")
                || v.starts_with("Symbol::new(")
                || v.starts_with("Symbol::from_val(")
            {
                return v.to_string();
            }
            if symbol_params.iter().any(|p| p == v) {
                return v.to_string();
            }
            if is_bare_symbol_literal(v) {
                return format!("Symbol::new(env, \"{}\")", v);
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
    if is_stack_slot_name(arg) {
        return false;
    }
    if arg.starts_with("vec_builder") {
        return false;
    }
    true
}

fn is_stack_slot_name(name: &str) -> bool {
    name.starts_with("slot_var") || name.starts_with("sv")
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

fn parse_vec_builder_args_line(line: &str) -> Option<(String, Vec<String>)> {
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
    Some((tmp_name, args))
}

fn is_bare_symbol_literal(v: &str) -> bool {
    let mut chars = v.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_uppercase() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
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

fn collapse_labeled_stack_copy_vec_loop(lines: &[String]) -> Option<Vec<String>> {
    let mut has_mload = false;
    let mut has_mstore = false;
    let mut has_copy_loop = false;
    let mut vec_return: Option<String> = None;
    let mut global_restore: Option<String> = None;

    for raw in lines {
        let t = raw.trim();
        if t.contains("mload64!(") {
            has_mload = true;
        }
        if t.contains("mstore64!(") {
            has_mstore = true;
        }
        if t == "loop {" || t.starts_with("'label") {
            has_copy_loop = true;
        }
        if t.starts_with("return vec![&env,") {
            if vec_return.is_some() {
                return None;
            }
            vec_return = Some(t.to_string());
        }
        if t.starts_with("self.global0 = ") {
            // Keep the last restore before return.
            global_restore = Some(t.to_string());
        }
    }

    if !has_mload || !has_mstore || !has_copy_loop {
        return None;
    }
    let vec_return = vec_return?;
    let global_restore = global_restore?;
    let indent = lines
        .first()
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).collect::<String>())
        .unwrap_or_default();

    Some(vec![
        format!("{indent}{global_restore}"),
        format!("{indent}{vec_return}"),
    ])
}
