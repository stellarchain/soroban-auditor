use std::collections::HashSet;
use regex::Regex;

const HELPER_METHODS: &[&str] = &[
    "copy_bytes_to_linear_memory",
    "copy_string_to_linear_memory",
    "for_each_val",
    "for_each_string",
    "for_each_string_checked",
    "next_string_checked",
    "require_len_match",
    "require_len_match_len",
    "vec_new_val",
    "vec_push_val",
    "pack_ok_val",
    "zero_24_bytes",
];

const TOP_LEVEL_HELPERS: &[&str] = &["address_from_i64", "err_contract", "val_to_i64", "val_from_i64"];

pub fn run_all(output: String, contract_name: &str) -> String {
    // Always prune unused top-level helpers; this is safe and keeps headers lean.
    let output = postprocess_remove_unused_top_level_helpers(output);
    // Keep only imports that are actually needed by the generated output.
    let output = prune_unused_soroban_imports(output);
    // Always normalize indentation; this is syntax-preserving and prevents drift
    // from upstream scope/unwrap transforms.
    let output = normalize_indentation(output);
    let output = cleanup_synthetic_single_pass_labels(output);
    let output = normalize_single_pass_guard_blocks(output);
    let output = compact_match_equivalent_arms(output);
    let output = fix_match_break_without_loop(output);
    let output = cleanup_immediate_overwrite_assignments(output);
    let output = cleanup_redundant_negations(output);
    let output = cleanup_empty_condition_blocks(output);
    let output = cleanup_linear_loop_unreachable(output);
    let output = cleanup_status_guard_loops(output);
    let output = cleanup_vm_frame_noise(output);
    let output = cleanup_dead_return_trailers(output);
    let output = cleanup_unit_tail_unreachable(output);
    let output = cleanup_placeholder_comments(output);
    let output = cleanup_duplicate_unreachable(output);
    let output = cleanup_noop_identifier_statements(output);
    let output = cleanup_break_after_unreachable(output);
    let output = cleanup_dead_type_tag_guards(output);
    let output = cleanup_vec_builder_append(output);
    let output = cleanup_missing_loop_break_guard(output);
    let output = cleanup_noop_match_break_loops(output);
    let output = cleanup_loop_if_only_to_while(output);
    let output = cleanup_guard_fallthrough_unreachable(output);
    let output = cleanup_terminal_return_unreachable(output);
    let output = cleanup_unused_let_bindings(output);
    let output = normalize_sdk_type_paths(output);
    let output = cleanup_trivial_if_shells_regex(output);
    let output = add_blank_line_before_functions(output);
    let output = normalize_indentation(output);
    let output = postprocess_remove_unused_methods(output, contract_name);
    // Keep the rest of postprocess as opt-in cosmetic cleanup.
    // Default path preserves code_builder output as-is.
    let enable_postprocess = std::env::var("SOROBAN_AUDITOR_ENABLE_POSTPROCESS").is_ok();
    if !enable_postprocess || std::env::var("SOROBAN_AUDITOR_SKIP_POSTPROCESS").is_ok() {
        return output;
    }
    safe_cleanup(output, contract_name)
}

fn add_blank_line_before_functions(output: String) -> String {
    let mut out: Vec<String> = Vec::new();
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();

    for line in lines {
        let trimmed = line.trim();
        let is_fn_start = trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ");
        if is_fn_start {
            let prev_non_empty = out.iter().rev().find(|l| !l.trim().is_empty()).map(|s| s.trim());
            let should_insert_blank = match prev_non_empty {
                Some(prev) => prev != "{" && !prev.starts_with("#["),
                None => false,
            };
            if should_insert_blank
                && out.last().map(|l| !l.trim().is_empty()).unwrap_or(false)
            {
                out.push(String::new());
            }
        }
        out.push(line);
    }

    out.join("\n")
}

fn cleanup_loop_if_only_to_while(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop {" {
            i += 1;
            continue;
        }
        let Some(loop_end) = find_block_end(&lines, i) else {
            i += 1;
            continue;
        };

        let mut first_non_empty = None;
        for (idx, line) in lines.iter().enumerate().take(loop_end).skip(i + 1) {
            if !line.trim().is_empty() {
                first_non_empty = Some(idx);
                break;
            }
        }
        let Some(if_start) = first_non_empty else {
            i += 1;
            continue;
        };
        let if_line = lines[if_start].trim().to_string();
        if !if_line.starts_with("if ") || !if_line.ends_with(" {") {
            i += 1;
            continue;
        }
        let cond = if_line
            .trim_start_matches("if ")
            .trim_end_matches(" {")
            .trim()
            .to_string();
        let Some(if_end) = find_block_end(&lines, if_start) else {
            i += 1;
            continue;
        };
        if if_end + 1 != loop_end {
            i += 1;
            continue;
        }

        let indent = leading_ws(&lines[i]);
        let mut replacement = Vec::new();
        replacement.push(format!("{indent}while {cond} {{"));
        for line in lines.iter().take(if_end).skip(if_start + 1) {
            replacement.push(dedent_line(line, 4));
        }
        replacement.push(format!("{indent}}}"));

        lines.splice(i..=loop_end, replacement);
        continue;
    }

    lines.join("\n")
}

fn cleanup_noop_match_break_loops(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop {" {
            i += 1;
            continue;
        }
        let Some(loop_end) = find_block_end(&lines, i) else {
            i += 1;
            continue;
        };
        if is_noop_match_break_loop(&lines[i + 1..loop_end]) {
            lines.drain(i..=loop_end);
            continue;
        }
        i = loop_end + 1;
    }
    lines.join("\n")
}

fn is_noop_match_break_loop(lines: &[String]) -> bool {
    if lines.is_empty() {
        return false;
    }

    let mut saw_match = false;
    let mut saw_arm = false;
    let mut depth = 0i32;

    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("match ") && t.ends_with('{') {
            saw_match = true;
            depth += 1;
            continue;
        }
        if t == "}" {
            if depth <= 0 {
                return false;
            }
            depth -= 1;
            continue;
        }
        if t.contains("=> break") {
            saw_arm = true;
            continue;
        }
        return false;
    }

    saw_match && saw_arm && depth == 0
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

fn leading_ws(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

fn dedent_line(line: &str, spaces: usize) -> String {
    let mut consumed = 0usize;
    for ch in line.chars() {
        if ch == ' ' && consumed < spaces {
            consumed += 1;
        } else {
            break;
        }
    }
    line[consumed..].to_string()
}

fn prune_unused_soroban_imports(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let import_idx = lines
        .iter()
        .position(|l| l.trim_start().starts_with("use soroban_sdk::{"));
    let Some(import_idx) = import_idx else {
        return output;
    };

    let import_line = lines[import_idx].trim();
    let prefix = "use soroban_sdk::{";
    let suffix = "};";
    if !(import_line.starts_with(prefix) && import_line.ends_with(suffix)) {
        return output;
    }

    let items_raw = &import_line[prefix.len()..import_line.len() - suffix.len()];
    let items: Vec<String> = items_raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    if items.is_empty() {
        return output;
    }

    let body_text = lines
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != import_idx)
        .map(|(_, l)| l.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    let mut kept: Vec<String> = Vec::new();
    for item in items {
        if soroban_import_is_used(&body_text, &item) {
            kept.push(item);
        }
    }
    if kept.is_empty() {
        return output;
    }

    let mut out = lines;
    out[import_idx] = format!("use soroban_sdk::{{{}}};", kept.join(", "));
    out.join("\n")
}

fn soroban_import_is_used(body: &str, item: &str) -> bool {
    match item {
        // Attribute macros
        "contract" => body.contains("#[contract]"),
        "contractimpl" => body.contains("#[contractimpl]"),
        "contracttype" => body.contains("#[soroban_sdk::contracttype")
            || body.contains("#[contracttype"),
        "contractevent" => body.contains("#[soroban_sdk::contractevent")
            || body.contains("#[contractevent"),

        // Macro usage
        "vec" => body.contains("vec!["),

        // Trait-method driven imports
        "IntoVal" => body.contains(".into_val(") || body.contains(" IntoVal"),
        "FromVal" => body.contains("::from_val(") || body.contains(" FromVal"),
        "TryFromVal" => body.contains("::try_from_val(") || body.contains(" TryFromVal"),

        // Default token check with identifier boundaries
        _ => contains_ident(body, item),
    }
}

fn contains_ident(text: &str, ident: &str) -> bool {
    let mut idx = 0usize;
    while let Some(pos) = text[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let bytes = text.as_bytes();
        let left_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
        let right_ok = end >= bytes.len() || !is_ident_char(bytes[end] as char);
        if left_ok && right_ok {
            return true;
        }
        idx = end;
    }
    false
}

fn cleanup_vec_builder_append(output: String) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut skip_vec_builder_assign_from: Option<String> = None;
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(var_name) = &skip_vec_builder_assign_from {
            if trimmed == format!("vec_builder = {};", var_name) {
                skip_vec_builder_assign_from = None;
                continue;
            }
            // Stop skipping once we move past the immediate assignment shape.
            if !trimmed.is_empty() {
                skip_vec_builder_assign_from = None;
            }
        }
        if trimmed.contains("Vec::<Val>::from_val(env, &val_from_i64(vec_builder));")
            && trimmed.contains("v.push_back(val_from_i64(")
            && trimmed.contains("val_to_i64(v.into_val(env))")
        {
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            if let Some((lhs, rhs)) = trimmed
                .strip_prefix("let ")
                .and_then(|s| s.split_once(" = "))
            {
                let lhs = lhs.trim().to_string();
                out.push(format!("{indent}vec_builder = {rhs}"));
                skip_vec_builder_assign_from = Some(lhs);
            } else {
                out.push(format!("{indent}vec_builder = {trimmed}"));
            }
            continue;
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

fn cleanup_missing_loop_break_guard(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop {" {
            i += 1;
            continue;
        }
        let Some(end) = find_block_end_lines(&lines, i) else {
            i += 1;
            continue;
        };
        let inner_indent_len = lines[i].chars().take_while(|c| c.is_whitespace()).count() + 4;
        let mut assign_idx: Option<usize> = None;
        let mut if_idx: Option<usize> = None;
        let mut var_name = String::new();
        let mut limit_expr = String::new();
        let mut has_top_control = false;

        for idx in (i + 1)..end {
            let line = &lines[idx];
            let t = line.trim();
            if t.is_empty() {
                continue;
            }
            let indent_len = line.chars().take_while(|c| c.is_whitespace()).count();
            if indent_len != inner_indent_len {
                continue;
            }
            if t == "break;" || t == "continue;" || t.starts_with("break '") || t.starts_with("continue '")
            {
                has_top_control = true;
                break;
            }
            if assign_idx.is_none() {
                if let Some((lhs, _rhs)) = parse_simple_assignment_line(t) {
                    assign_idx = Some(idx);
                    var_name = lhs.to_string();
                } else {
                    break;
                }
                continue;
            }
            if if_idx.is_none() {
                if let Some((lhs, rhs)) = parse_if_not_equal_header_line(t) {
                    if lhs == var_name {
                        if_idx = Some(idx);
                        limit_expr = rhs.to_string();
                    }
                }
                break;
            }
        }

        if has_top_control || assign_idx.is_none() || if_idx.is_none() {
            i = end + 1;
            continue;
        }

        let assign_idx = assign_idx.unwrap_or(i + 1);
        let if_idx = if_idx.unwrap_or(i + 2);
        if if_idx != assign_idx + 1 {
            i = end + 1;
            continue;
        }

        let inner_indent = " ".repeat(inner_indent_len);
        let guard_indent = format!("{inner_indent}    ");
        lines.insert(if_idx, format!("{inner_indent}if {} == {} {{", var_name, limit_expr));
        lines.insert(if_idx + 1, format!("{guard_indent}break;"));
        lines.insert(if_idx + 2, format!("{inner_indent}}}"));
        i = end + 4;
    }
    lines.join("\n")
}

fn parse_simple_assignment_line(t: &str) -> Option<(&str, &str)> {
    if !t.ends_with(';') || !t.contains(" = ") {
        return None;
    }
    let no_semi = t.trim_end_matches(';').trim();
    let (lhs, rhs) = no_semi.split_once(" = ")?;
    let lhs = lhs.trim();
    if lhs.is_empty() || !lhs.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((lhs, rhs.trim()))
}

fn parse_if_not_equal_header_line(t: &str) -> Option<(&str, &str)> {
    if !t.starts_with("if ") || !t.ends_with(" {") || !t.contains(" != ") {
        return None;
    }
    let cond = t.strip_prefix("if ")?.strip_suffix(" {")?.trim();
    let (lhs, rhs) = cond.split_once(" != ")?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();
    if lhs.is_empty() || rhs.is_empty() {
        return None;
    }
    if !lhs.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((lhs, rhs))
}

fn find_block_end_lines(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        if idx > start && depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn cleanup_synthetic_single_pass_labels(output: String) -> String {
    output
}

fn cleanup_guard_fallthrough_unreachable(output: String) -> String {
    output
}

fn cleanup_vm_frame_noise(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();

    let mut i = 0usize;
    while i + 2 < lines.len() {
        let l0 = lines[i].trim().to_string();
        let l1 = lines.get(i + 1).map(|s| s.trim().to_string()).unwrap_or_default();
        let l2 = lines.get(i + 2).map(|s| s.trim().to_string()).unwrap_or_default();

        let mut frame_var: Option<String> = None;
        let mut frame_size: Option<i32> = None;
        let mut prologue_end = i;

        // Shape A:
        // let base = self.global0;
        // frame = base.wrapping_sub(N);
        // self.global0 = frame;
        if i + 2 < lines.len() {
            if let Some((base_var, rhs0)) = parse_assignment_line(&l0) {
                if rhs0 == "self.global0" {
                    if let Some((fvar, rhs1)) = parse_assignment_line(&l1) {
                        if let Some(fsize) = parse_wrapping_sub_rhs(&rhs1, &base_var) {
                            if l2 == format!("self.global0 = {fvar};") {
                                frame_var = Some(fvar);
                                frame_size = Some(fsize);
                                prologue_end = i + 2;
                            }
                        }
                    }
                }
            }
        }

        // Shape B:
        // frame = self.global0.wrapping_sub(N);
        // self.global0 = frame;
        if frame_var.is_none() && i + 1 < lines.len() {
            if let Some((fvar, rhs0)) = parse_assignment_line(&l0) {
                if let Some(rest) = rhs0.strip_prefix("self.global0.wrapping_sub(") {
                    if let Some(size_str) = rest.strip_suffix(')') {
                        if let Ok(fsize) = size_str.parse::<i32>() {
                            if l1 == format!("self.global0 = {fvar};") {
                                frame_var = Some(fvar);
                                frame_size = Some(fsize);
                                prologue_end = i + 1;
                            }
                        }
                    }
                }
            }
        }

        let (Some(frame_var), Some(frame_size)) = (frame_var, frame_size) else {
            i += 1;
            continue;
        };

        let mut restore_idx: Option<usize> = None;
        let mut ret_idx: Option<usize> = None;
        let search_end = usize::min(lines.len(), i + 12);
        for idx in (prologue_end + 1)..search_end {
            let t = lines[idx].trim();
            if t == format!("self.global0 = {frame_var}.wrapping_add({frame_size});") {
                restore_idx = Some(idx);
                if idx + 1 < lines.len() {
                    let next = lines[idx + 1].trim();
                    if next.starts_with("return ") || next == "return;" {
                        ret_idx = Some(idx + 1);
                    }
                }
                break;
            }
        }
        let (Some(restore), Some(_ret)) = (restore_idx, ret_idx) else {
            i += 1;
            continue;
        };

        // Safety gate: do not touch frame if semantic body still references frame pointer
        // or writes self.global0 directly (except prologue/restore).
        let mut semantic_uses_frame = false;
        for idx in (prologue_end + 1)..restore {
            let t = lines[idx].trim();
            if contains_ident(t, &frame_var) || t.contains("self.global0") {
                semantic_uses_frame = true;
                break;
            }
        }
        if semantic_uses_frame {
            i += 1;
            continue;
        }

        // Remove frame setup and restore.
        lines.remove(restore);
        lines.drain(i..=prologue_end);

        // Remove dead simple zero assigns/declarations in the same function block.
        remove_dead_temps_in_place(&mut lines);
        i = i.saturating_sub(1);
    }

    lines.join("\n")
}

fn cleanup_dead_return_trailers(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;

    while i < lines.len() {
        let cur = lines[i].trim();
        let next = lines.get(i + 1).map(|s| s.trim()).unwrap_or("");
        let next2 = lines.get(i + 2).map(|s| s.trim()).unwrap_or("");
        let next3 = lines.get(i + 3).map(|s| s.trim()).unwrap_or("");

        if cur == "unreachable!();" {
            let mut p1 = i;
            while p1 > 0 && lines[p1 - 1].trim().is_empty() {
                p1 -= 1;
            }
            let prev1 = if p1 > 0 { lines[p1 - 1].trim() } else { "" };
            let mut p2 = p1.saturating_sub(1);
            while p2 > 0 && lines[p2 - 1].trim().is_empty() {
                p2 -= 1;
            }
            let prev2 = if p2 > 0 { lines[p2 - 1].trim() } else { "" };
            let mut n1 = i + 1;
            while n1 < lines.len() && lines[n1].trim().is_empty() {
                n1 += 1;
            }
            let next1 = if n1 < lines.len() { lines[n1].trim() } else { "" };
            if prev1 == "}"
                && next1 == "}"
                && (prev2 == "return;" || prev2.starts_with("return "))
                && !is_closing_labeled_block(&lines, p1 - 1)
            {
                i += 1;
                continue;
            }
        }

        if cur.starts_with("return ")
            && cur.ends_with(';')
            && next.starts_with("// There should've been an expression value here")
            && next2 == "unreachable!();"
        {
            out.push(lines[i].clone());
            i += 3;
            continue;
        }

        if cur.starts_with("return ")
            && cur.ends_with(';')
            && next == "unreachable!();"
        {
            out.push(lines[i].clone());
            i += 2;
            continue;
        }

        if cur.starts_with("return ")
            && cur.ends_with(';')
            && next == "}"
            && next2 == "unreachable!();"
            && next3 == "}"
        {
            out.push(lines[i].clone());
            out.push(lines[i + 1].clone());
            i += 3;
            continue;
        }

        out.push(lines[i].clone());
        i += 1;
    }

    out.join("\n")
}

fn cleanup_unit_tail_unreachable(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;

    while i < lines.len() {
        let cur = lines[i].trim();
        if cur == "}" {
            let mut prev = i;
            let mut prev_line = "";
            while prev > 0 {
                prev -= 1;
                let t = lines[prev].trim();
                if !t.is_empty() {
                    prev_line = t;
                    break;
                }
            }
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim() == "unreachable!();" {
                let mut k = j + 1;
                while k < lines.len() && lines[k].trim().is_empty() {
                    k += 1;
                }
                if k < lines.len() && lines[k].trim() == "}" {
                    let prev_is_return = prev_line == "return;" || prev_line.starts_with("return ");
                    if prev_is_return && !is_closing_labeled_block(&lines, i) {
                        out.push(lines[i].clone());
                        i = j + 1;
                        continue;
                    }
                }
            }
        }
        let is_frame_restore = cur.starts_with("self.global0 =") && cur.contains("wrapping_add(");
        if is_frame_restore {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim() == "unreachable!();" {
                let mut prev = j;
                let mut prev_non_empty = "";
                while prev > 0 {
                    prev -= 1;
                    let t = lines[prev].trim();
                    if !t.is_empty() {
                        prev_non_empty = t;
                        break;
                    }
                }
                if prev_non_empty == "}" {
                    out.push(lines[i].clone());
                    i += 1;
                    continue;
                }
                let mut k = j + 1;
                while k < lines.len() && lines[k].trim().is_empty() {
                    k += 1;
                }
                if k < lines.len() && lines[k].trim() == "}" {
                    out.push(lines[i].clone());
                    i = j + 1;
                    continue;
                }
            }
        }
        out.push(lines[i].clone());
        i += 1;
    }

    out.join("\n")
}

fn cleanup_terminal_return_unreachable(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;

    while i < lines.len() {
        let cur = lines[i].trim();
        let next = lines.get(i + 1).map(|s| s.trim()).unwrap_or("");
        let next2 = lines.get(i + 2).map(|s| s.trim()).unwrap_or("");
        let next3 = lines.get(i + 3).map(|s| s.trim()).unwrap_or("");

        if (cur == "return;" || cur.starts_with("return "))
            && next == "}"
            && next2 == "unreachable!();"
            && next3 == "}"
        {
            out.push(lines[i].clone());
            out.push(lines[i + 1].clone());
            i += 3;
            continue;
        }

        out.push(lines[i].clone());
        i += 1;
    }

    out.join("\n")
}

fn cleanup_redundant_negations(output: String) -> String {
    let mut out = Vec::new();
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(cond) = trimmed.strip_prefix("if ").and_then(|s| s.strip_suffix(" {")) {
            let fixed = simplify_triple_neg(cond);
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            out.push(format!("{indent}if {fixed} {{"));
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

fn fix_match_break_without_loop(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim().to_string();
        if !(trimmed.starts_with("match ") && trimmed.ends_with('{')) {
            i += 1;
            continue;
        }
        let Some(match_end) = find_brace_block_end(&lines, i) else {
            i += 1;
            continue;
        };
        if match_end <= i + 1 {
            i = match_end.saturating_add(1);
            continue;
        }

        let body = &lines[(i + 1)..match_end];
        let mut has_break_match_arm = false;
        for line in body {
            let t = line.trim();
            if t.contains("=> break,") || t.contains("=> break '") {
                has_break_match_arm = true;
            }
        }

        if has_break_match_arm && is_noop_break_match_body(body) {
            lines.drain(i..=match_end);
            continue;
        }

        if has_break_match_arm && !is_inside_loop_before_line(&lines, i) {
            let indent = lines[i]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            lines.insert(i, format!("{indent}loop {{"));
            lines.insert(match_end + 2, format!("{indent}}}"));
            i = match_end + 3;
            continue;
        }

        i = match_end.saturating_add(1);
    }
    lines.join("\n")
}

fn is_noop_break_match_body(body: &[String]) -> bool {
    let mut saw_arm = false;
    for line in body {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t == "}" {
            continue;
        }
        if is_break_only_match_arm(t) {
            saw_arm = true;
            continue;
        }
        return false;
    }
    saw_arm
}

fn is_break_only_match_arm(t: &str) -> bool {
    let Some((_, rhs)) = t.split_once("=>") else {
        return false;
    };
    let rhs = rhs.trim().trim_end_matches(',');
    if rhs == "break" {
        return true;
    }
    if let Some(rest) = rhs.strip_prefix("break '") {
        return rest
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == ';');
    }
    false
}

fn cleanup_immediate_overwrite_assignments(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;
    while i < lines.len() {
        if i + 1 < lines.len() {
            let cur = lines[i].trim();
            let next = lines[i + 1].trim();
            if let Some((lhs1, rhs1)) = parse_simple_assignment_line(cur) {
                if let Some((lhs2, _rhs2)) = parse_simple_assignment_line(next) {
                    if lhs1 == lhs2 && is_trivial_rhs(rhs1) {
                        i += 1;
                        continue;
                    }
                }
            }
        }
        out.push(lines[i].clone());
        i += 1;
    }
    out.join("\n")
}

fn is_trivial_rhs(rhs: &str) -> bool {
    let r = rhs.trim();
    if r.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ' || c == '(' || c == ')' || c == '-' || c == '+')
    {
        return true;
    }
    r.ends_with("/* True */") || r.ends_with("/* False */")
}

fn is_inside_loop_before_line(lines: &[String], target: usize) -> bool {
    let mut depth: i32 = 0;
    for line in lines.iter().take(target).rev() {
        let trimmed = line.trim();
        depth += trimmed.chars().filter(|&c| c == '}').count() as i32;
        depth -= trimmed.chars().filter(|&c| c == '{').count() as i32;
        if depth < 0 {
            if trimmed.starts_with("loop ")
                || trimmed == "loop {"
                || trimmed.starts_with("while ")
                || trimmed.starts_with("for ")
                || (trimmed.starts_with('\'') && trimmed.ends_with(": loop {"))
            {
                return true;
            }
            depth = 0;
        }
    }
    false
}

fn compact_match_equivalent_arms(output: String) -> String {
    let arm_re = Regex::new(
        r"^(\s*)([^=][^=]*?)\s=>\s((?:break|continue)(?:\s'[^;]+)?),\s*$",
    )
    .ok();
    let Some(arm_re) = arm_re else {
        return output;
    };

    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i].trim();
        if !(line.starts_with("match ") && line.ends_with('{')) {
            i += 1;
            continue;
        }
        let Some(end) = find_brace_block_end(&lines, i) else {
            i += 1;
            continue;
        };

        let mut j = i + 1;
        while j < end {
            let cur = lines[j].clone();
            let Some(caps) = arm_re.captures(cur.trim_end()) else {
                j += 1;
                continue;
            };
            let indent = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let lhs = caps.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            let rhs = caps.get(3).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            if lhs.is_empty() || rhs.is_empty() {
                j += 1;
                continue;
            }

            let mut group_end = j;
            let mut lhs_parts = vec![lhs];
            let mut k = j + 1;
            while k < end {
                let next = lines[k].clone();
                let Some(next_caps) = arm_re.captures(next.trim_end()) else {
                    break;
                };
                let next_indent = next_caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let next_rhs = next_caps
                    .get(3)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                if next_indent != indent || next_rhs != rhs {
                    break;
                }
                let next_lhs = next_caps
                    .get(2)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                if next_lhs.is_empty() {
                    break;
                }
                lhs_parts.push(next_lhs);
                group_end = k;
                k += 1;
            }

            if group_end > j {
                lines[j] = format!("{indent}{} => {rhs},", lhs_parts.join(" | "));
                lines.drain((j + 1)..=group_end);
            }
            j += 1;
        }

        i = end.saturating_add(1);
    }

    lines.join("\n")
}

fn normalize_single_pass_guard_blocks(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim().to_string();
        let Some(rest) = trimmed.strip_prefix("'__sp") else {
            i += 1;
            continue;
        };
        let Some(label_suffix) = rest.strip_suffix(": {") else {
            i += 1;
            continue;
        };
        if label_suffix.is_empty() || !label_suffix.chars().all(|c| c.is_ascii_digit()) {
            i += 1;
            continue;
        }
        let label = format!("__sp{label_suffix}");

        let Some(end) = find_brace_block_end(&lines, i) else {
            i += 1;
            continue;
        };

        // Drop synthetic label marker: `'__spN: {` -> `{`
        let indent = lines[i]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        lines[i] = format!("{indent}{{");

        // Rewrite `break '__spN;` into local trap at the guard site.
        for line in lines.iter_mut().take(end).skip(i + 1) {
            if line.trim() == format!("break '{label};") {
                let inner_indent = line
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                *line = format!("{inner_indent}unreachable!();");
            }
        }

        // Remove one trailing `unreachable!();` after synthetic single-pass block.
        let mut next = end + 1;
        while next < lines.len() && lines[next].trim().is_empty() {
            next += 1;
        }
        if next < lines.len() && lines[next].trim() == "unreachable!();" {
            lines.remove(next);
        }

        i = end.saturating_add(1);
    }
    lines.join("\n")
}

fn cleanup_empty_condition_blocks(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;

    while i < lines.len() {
        let cur = lines[i].trim();
        let is_cond_start = (cur.starts_with("if ") && cur.ends_with('{')) || cur == "else {";
        if is_cond_start {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim() == "}" {
                i = j + 1;
                continue;
            }
        }
        out.push(lines[i].clone());
        i += 1;
    }

    out.join("\n")
}

fn cleanup_linear_loop_unreachable(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop {" {
            i += 1;
            continue;
        }

        let Some(loop_end) = find_brace_block_end(&lines, i) else {
            i += 1;
            continue;
        };
        if loop_end <= i + 1 {
            i = loop_end.saturating_add(1);
            continue;
        }

        if has_loop_control_tokens_depth0(&lines[(i + 1)..loop_end]) {
            i = loop_end.saturating_add(1);
            continue;
        }

        let mut next = loop_end + 1;
        while next < lines.len() && lines[next].trim().is_empty() {
            next += 1;
        }
        if next >= lines.len() || lines[next].trim() != "unreachable!();" {
            i = loop_end.saturating_add(1);
            continue;
        }

        let indent = lines[i]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        lines[i] = format!("{indent}{{");
        lines.remove(next);
        i = loop_end.saturating_add(1);
    }
    lines.join("\n")
}

fn cleanup_status_guard_loops(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop {" {
            i += 1;
            continue;
        }
        let Some(end) = find_brace_block_end(&lines, i) else {
            i += 1;
            continue;
        };
        if end <= i + 2 {
            i = end.saturating_add(1);
            continue;
        }

        let mut body_idx: Vec<usize> = Vec::new();
        for idx in (i + 1)..end {
            if !lines[idx].trim().is_empty() {
                body_idx.push(idx);
            }
        }
        if body_idx.is_empty() {
            i = end.saturating_add(1);
            continue;
        }
        let let_line = lines[body_idx[0]].trim().to_string();
        if !let_line.starts_with("let ") || !let_line.contains(" = ") || !let_line.ends_with(';') {
            i = end.saturating_add(1);
            continue;
        }
        let Some(rest) = let_line.strip_prefix("let ") else {
            i = end.saturating_add(1);
            continue;
        };
        let lhs = rest.split('=').next().unwrap_or_default().trim();
        let lhs = lhs.strip_prefix("mut ").unwrap_or(lhs);
        let var_name = lhs.split(':').next().unwrap_or_default().trim();
        if !is_ident(var_name) {
            i = end.saturating_add(1);
            continue;
        }
        let loop_indent = lines[i]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let inner_indent = format!("{loop_indent}    ");
        // Shape A: loop { let status = ...; unreachable!(); }
        if body_idx.len() == 2 && lines[body_idx[1]].trim() == "unreachable!();" {
            let replacement = vec![
                format!("{loop_indent}{let_line}"),
                format!("{loop_indent}if {var_name} != 0 {{"),
                format!("{inner_indent}unreachable!();"),
                format!("{loop_indent}}}"),
            ];
            lines.splice(i..=end, replacement);
            i += 4;
            continue;
        }

        // Shape B: loop { let status = ...; if status != 0 { break; } unreachable!(); }
        if body_idx.len() == 5
            && lines[body_idx[2]].trim() == "break;"
            && lines[body_idx[3]].trim() == "}"
            && lines[body_idx[4]].trim() == "unreachable!();"
        {
            let if_line = lines[body_idx[1]].trim();
            if let Some(cond) = if_line
                .strip_prefix("if ")
                .and_then(|s| s.strip_suffix(" {"))
            {
                let neg = negate_simple_cond(cond);
                let replacement = vec![
                    format!("{loop_indent}{let_line}"),
                    format!("{loop_indent}if {neg} {{"),
                    format!("{inner_indent}unreachable!();"),
                    format!("{loop_indent}}}"),
                ];
                lines.splice(i..=end, replacement);
                i += 4;
                continue;
            }
        }

        i = end.saturating_add(1);
    }
    lines.join("\n")
}

fn negate_simple_cond(cond: &str) -> String {
    let c = cond.trim();
    if let Some((lhs, rhs)) = c.split_once(" != ") {
        return format!("{} == {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = c.split_once(" == ") {
        return format!("{} != {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = c.split_once(" < ") {
        return format!("{} >= {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = c.split_once(" <= ") {
        return format!("{} > {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = c.split_once(" > ") {
        return format!("{} <= {}", lhs.trim(), rhs.trim());
    }
    if let Some((lhs, rhs)) = c.split_once(" >= ") {
        return format!("{} < {}", lhs.trim(), rhs.trim());
    }
    format!("!({c})")
}

fn find_brace_block_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        if idx > start && depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn is_closing_labeled_block(lines: &[String], close_idx: usize) -> bool {
    if close_idx >= lines.len() || lines[close_idx].trim() != "}" {
        return false;
    }
    let mut depth = 0i32;
    for idx in (0..=close_idx).rev() {
        let line = &lines[idx];
        for ch in line.chars().rev() {
            if ch == '}' {
                depth += 1;
            } else if ch == '{' {
                depth -= 1;
                if depth == 0 {
                    let t = lines[idx].trim();
                    return t.starts_with('\'') && t.ends_with(": {");
                }
            }
        }
    }
    false
}

fn has_loop_control_tokens_depth0(lines: &[String]) -> bool {
    let mut depth = 0i32;
    for l in lines {
        let t = l.trim();
        if depth == 0
            && (t == "break;"
                || t == "continue;"
                || t.starts_with("break '")
                || t.starts_with("continue '"))
        {
            return true;
        }
        for ch in l.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        if depth < 0 {
            depth = 0;
        }
    }
    false
}


fn cleanup_trivial_if_shells_regex(output: String) -> String {
    let mut out = output;
    let re_if = Regex::new(r"(?m)^[ \t]*if [^\n]+\{\n[ \t]*\}\n?").ok();
    let re_else = Regex::new(r"(?m)^[ \t]*else \{\n[ \t]*\}\n?").ok();
    if let Some(re) = re_if {
        out = re.replace_all(&out, "").to_string();
    }
    if let Some(re) = re_else {
        out = re.replace_all(&out, "").to_string();
    }
    out
}

fn cleanup_placeholder_comments(output: String) -> String {
    output
        .lines()
        .filter(|l| !l.trim().starts_with("// There should've been an expression value here"))
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn cleanup_duplicate_unreachable(output: String) -> String {
    let mut out = Vec::new();
    let mut prev_unreachable = false;
    for line in output.lines() {
        let t = line.trim();
        if t == "unreachable!();" {
            if prev_unreachable {
                continue;
            }
            prev_unreachable = true;
            out.push(line.to_string());
            continue;
        }
        if !t.is_empty() {
            prev_unreachable = false;
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

fn cleanup_noop_identifier_statements(output: String) -> String {
    output
        .lines()
        .filter(|l| {
            let t = l.trim();
            if !t.ends_with(';') || t.contains(' ') {
                return true;
            }
            let ident = t.trim_end_matches(';');
            !is_ident(ident)
        })
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn cleanup_break_after_unreachable(output: String) -> String {
    let mut out = Vec::new();
    let mut prev_code: Option<String> = None;
    for line in output.lines() {
        let t = line.trim();
        if t == "break;" && prev_code.as_deref() == Some("unreachable!();") {
            continue;
        }
        if !t.is_empty() && !t.starts_with("//") {
            prev_code = Some(t.to_string());
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

fn cleanup_dead_type_tag_guards(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut i = 0usize;
    while i + 4 < lines.len() {
        let l0 = lines[i].trim().to_string();
        let l1 = lines[i + 1].trim().to_string();
        let l2 = lines[i + 2].trim().to_string();
        let l3 = lines[i + 3].trim().to_string();
        let l4 = lines[i + 4].trim().to_string();

        let Some((var_name, rhs)) = parse_assignment_line(&l0) else {
            i += 1;
            continue;
        };
        if !rhs.contains("& 255") {
            i += 1;
            continue;
        }
        if l1 != format!("if {var_name} == 14 {{") || l2 != "unreachable!();" || l3 != "}" {
            i += 1;
            continue;
        }
        if l4 != format!("{var_name} = 0;") {
            i += 1;
            continue;
        }
        let mut outside_refs = 0usize;
        for (idx, line) in lines.iter().enumerate() {
            if idx >= i && idx <= i + 4 {
                continue;
            }
            if parse_temp_decl(line.trim()).as_deref() == Some(&var_name) {
                continue;
            }
            outside_refs += count_ident_in_line(line, &var_name);
        }
        if outside_refs > 0 {
            i += 1;
            continue;
        }

        lines.drain(i..=i + 4);
    }
    lines.join("\n")
}

fn cleanup_unused_let_bindings(output: String) -> String {
    let mut lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    // Iterate to fixpoint because one removal may expose more unused lets.
    loop {
        let mut changed = false;
        for i in 0..lines.len() {
            let t = lines[i].trim();
            let Some((name, rhs)) = parse_let_binding_expr(t) else {
                continue;
            };
            if count_ident_refs(&lines, &name) != 1 {
                continue;
            }
            let indent = lines[i]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            lines[i] = format!("{indent}{rhs};");
            changed = true;
        }
        if !changed {
            break;
        }
    }
    lines.join("\n")
}

fn normalize_sdk_type_paths(output: String) -> String {
    let mut s = output;
    // Type-path shortening (safe when matching exact fully qualified type names).
    for (from, to) in [
        ("soroban_sdk::Vec<", "Vec<"),
        ("soroban_sdk::Map<", "Map<"),
        ("soroban_sdk::Address", "Address"),
        ("soroban_sdk::MuxedAddress", "MuxedAddress"),
        ("soroban_sdk::String", "String"),
        ("soroban_sdk::Bytes", "Bytes"),
        ("soroban_sdk::BytesN<", "BytesN<"),
        ("soroban_sdk::Symbol", "Symbol"),
        ("soroban_sdk::Val", "Val"),
        ("soroban_sdk::Error", "Error"),
        ("soroban_sdk::U256", "U256"),
    ] {
        s = s.replace(from, to);
    }
    s
}

fn simplify_triple_neg(cond: &str) -> String {
    let t = cond.trim();
    if t.starts_with("!(!(!(") && t.ends_with(")))") {
        let inner = &t[6..t.len() - 3];
        return format!("!({})", inner.trim());
    }
    t.to_string()
}

fn parse_assignment_line(line: &str) -> Option<(String, String)> {
    let (lhs, rhs) = line.split_once(" = ")?;
    let lhs = lhs
        .trim()
        .strip_prefix("let mut ")
        .or_else(|| lhs.trim().strip_prefix("let "))
        .unwrap_or(lhs.trim())
        .trim()
        .to_string();
    Some((lhs, rhs.trim().trim_end_matches(';').to_string()))
}

fn parse_wrapping_sub_rhs(rhs: &str, base_var: &str) -> Option<i32> {
    let prefix = format!("{base_var}.wrapping_sub(");
    let rest = rhs.strip_prefix(&prefix)?;
    let num = rest.strip_suffix(')')?;
    num.parse::<i32>().ok()
}

fn remove_dead_temps_in_place(lines: &mut Vec<String>) {
    loop {
        let mut changed = false;
        let mut i = 0usize;
        while i < lines.len() {
            let t = lines[i].trim();
            if let Some(name) = parse_temp_decl(t) {
                let refs = count_ident_refs(lines, &name);
                if refs <= 1 || (refs == 2 && has_only_decl_and_assign(lines, &name)) {
                    lines.remove(i);
                    changed = true;
                    continue;
                }
            }
            if let Some(name) = parse_temp_assign(t) {
                let refs = count_ident_refs(lines, &name);
                if refs <= 1 || (refs == 2 && has_only_decl_and_assign(lines, &name)) {
                    lines.remove(i);
                    changed = true;
                    continue;
                }
            }
            i += 1;
        }
        if !changed {
            break;
        }
    }
}

fn has_only_decl_and_assign(lines: &[String], ident: &str) -> bool {
    let mut refs = 0usize;
    let mut has_decl = false;
    let mut has_assign = false;
    for l in lines {
        let t = l.trim();
        let cnt = count_ident_in_line(t, ident);
        if cnt == 0 {
            continue;
        }
        refs += cnt;
        if parse_temp_decl(t).as_deref() == Some(ident) {
            has_decl = true;
        } else if parse_temp_assign(t).as_deref() == Some(ident) {
            has_assign = true;
        } else {
            return false;
        }
    }
    refs == 2 && has_decl && has_assign
}

fn parse_temp_decl(line: &str) -> Option<String> {
    let rest = line.strip_prefix("let mut ")?;
    if !rest.contains("= 0") {
        return None;
    }
    let name = rest.split(':').next()?.trim();
    if is_ident(name) {
        Some(name.to_string())
    } else {
        None
    }
}

fn parse_let_binding_expr(line: &str) -> Option<(String, String)> {
    // let var = expr;
    let rest = line.strip_prefix("let ")?;
    let (lhs, rhs) = rest.split_once(" = ")?;
    if lhs.contains(':') || lhs.contains("mut ") {
        return None;
    }
    let name = lhs.trim();
    if !is_ident(name) {
        return None;
    }
    let rhs = rhs.trim().trim_end_matches(';').trim();
    if rhs.is_empty() {
        return None;
    }
    Some((name.to_string(), rhs.to_string()))
}

fn parse_temp_assign(line: &str) -> Option<String> {
    let (lhs, rhs) = line.split_once(" = ")?;
    let lhs = lhs.trim();
    if !rhs.trim().ends_with(';') {
        return None;
    }
    if is_ident(lhs) {
        Some(lhs.to_string())
    } else {
        None
    }
}

fn count_ident_refs(lines: &[String], ident: &str) -> usize {
    lines.iter().map(|l| count_ident_in_line(l, ident)).sum()
}

fn count_ident_in_line(line: &str, ident: &str) -> usize {
    let mut count = 0usize;
    let mut idx = 0usize;
    while let Some(pos) = line[idx..].find(ident) {
        let start = idx + pos;
        let end = start + ident.len();
        let left_ok = start == 0 || !is_ident_char(line.as_bytes()[start - 1] as char);
        let right_ok = end >= line.len() || !is_ident_char(line.as_bytes()[end] as char);
        if left_ok && right_ok {
            count += 1;
        }
        idx = end;
        if idx >= line.len() {
            break;
        }
    }
    count
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn safe_cleanup(output: String, contract_name: &str) -> String {
    let output = postprocess_remove_unused_methods(output, contract_name);
    let output = postprocess_remove_unused_top_level_helpers(output);
    normalize_indentation(output)
}

fn normalize_indentation(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut depth: i32 = 0;
    let mut in_signature = false;
    let mut in_call_args = false;
    let mut call_base_indent: usize = 0;

    for line in lines {
        let mut line = line;
        if line.trim_end().ends_with("(;") {
            line = line.replacen("(;", "(", 1);
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            out.push(String::new());
            continue;
        }

        let mut effective_depth = depth;
        if trimmed.starts_with('}') {
            effective_depth = (depth - 1).max(0);
        }

        let mut indent_width = (effective_depth as usize) * 4;
        let is_fn_signature_start =
            (trimmed.starts_with("pub fn") || trimmed.starts_with("fn ")) && trimmed.ends_with('(');
        if !in_signature && is_fn_signature_start {
            in_signature = true;
        }
        if !in_signature
            && !in_call_args
            && trimmed.ends_with('(')
            && (trimmed.contains(" = self.") || trimmed.starts_with("self."))
        {
            in_call_args = true;
            call_base_indent = indent_width;
        }
        if in_signature
            && !trimmed.starts_with("pub fn")
            && !trimmed.starts_with("fn ")
            && !trimmed.starts_with(')')
        {
            indent_width += 4;
        }
        if in_call_args {
            if trimmed == ");" || trimmed.ends_with("); //") || trimmed.ends_with(");") {
                indent_width = call_base_indent;
            } else if !(trimmed.contains(" = self.") || trimmed.starts_with("self.")) {
                indent_width = call_base_indent + 4;
            }
        }
        let indent = " ".repeat(indent_width);
        out.push(format!("{indent}{trimmed}"));

        let opens = trimmed.chars().filter(|&c| c == '{').count() as i32;
        let closes = trimmed.chars().filter(|&c| c == '}').count() as i32;
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }

        if in_signature {
            let is_signature_end =
                trimmed.starts_with(')') || trimmed.contains(")->") || trimmed.contains(") ->");
            if is_signature_end {
                in_signature = false;
            }
        }
        if in_call_args && (trimmed == ");" || trimmed.ends_with("); //") || trimmed.ends_with(");"))
        {
            in_call_args = false;
        }
    }
    out.join("\n")
}

fn collect_self_calls(output: &str) -> HashSet<String> {
    let mut calls: HashSet<String> = HashSet::new();
    for line in output.lines() {
        let mut rest = line;
        while let Some(pos) = rest.find("self.") {
            rest = &rest[pos + 5..];
            let mut end = 0;
            for ch in rest.chars() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    end += ch.len_utf8();
                } else {
                    break;
                }
            }
            if end > 0 {
                calls.insert(rest[..end].to_string());
                rest = &rest[end..];
            } else {
                break;
            }
        }
    }
    calls
}

fn postprocess_remove_unused_methods(output: String, contract_name: &str) -> String {
    let referenced = collect_self_calls(&output);
    let helper_methods: HashSet<&'static str> = HELPER_METHODS.iter().copied().collect();
    let mut out_lines: Vec<String> = Vec::new();
    let mut in_impl = false;
    let mut impl_brace_depth: i32 = 0;
    let mut eligible_impl = false;
    let mut skipping_fn = false;
    let mut fn_brace_depth: i32 = 0;
    let mut fn_saw_open = false;

    let lines: Vec<&str> = output.lines().collect();
    let mut i: usize = 0;
    while i < lines.len() {
        let line = lines[i];
        if skipping_fn {
            let opens = line.chars().filter(|&c| c == '{').count() as i32;
            let closes = line.chars().filter(|&c| c == '}').count() as i32;
            if opens > 0 {
                fn_saw_open = true;
            }
            if fn_saw_open {
                fn_brace_depth += opens - closes;
                if fn_brace_depth <= 0 {
                    skipping_fn = false;
                }
            }
            i += 1;
            continue;
        }

        if !in_impl {
            if line.trim() == format!("impl {} {{", contract_name) {
                in_impl = true;
                impl_brace_depth = 1;
                let mut j = i;
                let mut prev_marker = String::new();
                while j > 0 {
                    j -= 1;
                    let prev = lines[j].trim();
                    if prev.is_empty() {
                        continue;
                    }
                    prev_marker = prev.to_string();
                    break;
                }
                eligible_impl = prev_marker != "#[contractimpl]";
                out_lines.push(line.to_string());
                i += 1;
                continue;
            }
            out_lines.push(line.to_string());
            i += 1;
            continue;
        }

        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        impl_brace_depth += opens - closes;
        if impl_brace_depth <= 0 {
            in_impl = false;
            eligible_impl = false;
            out_lines.push(line.to_string());
            i += 1;
            continue;
        }

        if eligible_impl {
            let trimmed = line.trim_start();
            if let Some(fn_pos) = trimmed.find("fn ") {
                if fn_pos == 0 || trimmed[..fn_pos].trim().is_empty() {
                    let name_start = fn_pos + 3;
                    let name = trimmed[name_start..]
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect::<String>();
                    if !name.is_empty()
                        && !referenced.contains(&name)
                        && !helper_methods.contains(name.as_str())
                    {
                        skipping_fn = true;
                        fn_brace_depth = 0;
                        fn_saw_open = false;
                        let opens = line.chars().filter(|&c| c == '{').count() as i32;
                        let closes = line.chars().filter(|&c| c == '}').count() as i32;
                        if opens > 0 {
                            fn_saw_open = true;
                        }
                        if fn_saw_open {
                            fn_brace_depth += opens - closes;
                            if fn_brace_depth <= 0 {
                                skipping_fn = false;
                            }
                        }
                        i += 1;
                        continue;
                    }
                }
            }
        }

        out_lines.push(line.to_string());
        i += 1;
    }
    out_lines.join("\n")
}

fn remove_top_level_helper(output: &str, helper_name: &str) -> String {
    let call_count = output.matches(&format!("{}(", helper_name)).count();
    if call_count > 1 {
        return output.to_string();
    }

    let lines: Vec<&str> = output.lines().collect();
    let mut out_lines: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        if trimmed.starts_with(&format!("fn {}(", helper_name)) {
            let mut depth: i32 = line.chars().filter(|&c| c == '{').count() as i32
                - line.chars().filter(|&c| c == '}').count() as i32;
            i += 1;
            while i < lines.len() && depth > 0 {
                depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                i += 1;
            }
            continue;
        }
        out_lines.push(line.to_string());
        i += 1;
    }
    out_lines.join("\n")
}

fn postprocess_remove_unused_top_level_helpers(output: String) -> String {
    let mut out = output;
    for helper in TOP_LEVEL_HELPERS {
        out = remove_top_level_helper(&out, helper);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{fix_match_break_without_loop, prune_unused_soroban_imports};

    #[test]
    fn wraps_match_break_when_outside_loop() {
        let input = [
            "fn sample() {",
            "    match x {",
            "        0 | _ => break,",
            "    }",
            "}",
        ]
        .join("\n");

        let output = fix_match_break_without_loop(input);
        assert!(output.contains("    loop {"));
        assert!(output.contains("match x {"));
    }

    #[test]
    fn keeps_match_break_inside_loop() {
        let input = [
            "fn sample() {",
            "    loop {",
            "        match x {",
            "            0 | _ => break,",
            "        }",
            "    }",
            "}",
        ]
        .join("\n");

        let output = fix_match_break_without_loop(input);
        assert_eq!(output.matches("loop {").count(), 1);
    }

    #[test]
    fn prunes_unused_soroban_imports() {
        let input = [
            "#![no_std]",
            "use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol, Val, Address, FromVal, IntoVal, Map, Bytes, BytesN, String};",
            "#[contract]",
            "pub struct C;",
            "#[contractimpl]",
            "impl C {",
            "    pub fn hello(&mut self, env: Env, to: Symbol) -> Vec<Symbol> {",
            "        vec![&env, Symbol::new(&env, \"Hello\"), to]",
            "    }",
            "}",
        ]
        .join("\n");
        let output = prune_unused_soroban_imports(input);
        assert!(output.contains("use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol};"));
        assert!(!output.contains("Val,"));
        assert!(!output.contains("Address,"));
    }
}
