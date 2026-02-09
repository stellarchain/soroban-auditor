use crate::decompile::{collect_imports, extract_data_segments, extract_data_segments_with_offsets, extract_string_literals};
use crate::patterns::PatternState;
use crate::patterns::templates::{TemplateMatcher, TemplateLibrary};
use crate::format::{format_spec_tokens, format_type_ident};
use parity_wasm::deserialize_file;
use parity_wasm::elements::{CodeSection, ExportSection, ImportCountType, Instruction, Internal, Section, Type};
use crate::sdk::{get_backend, ContractSpecs, FunctionContractSpec};
use crate::fingerprint::fingerprint_function;
use crate::rewrites::suggested_name_for_fingerprint;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

 
pub mod utils;
 
mod specs;
mod raw;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(
        short = "n",
        long = "use-name-section",
        help = "Use the names in the name section for the internal function names"
    )]
    use_name_section: bool,
    #[structopt(help = "Input file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(
        help = "Output file, stored next to wasm file if not specified",
        parse(from_os_str)
    )]
    output: Option<PathBuf>,
}

fn postprocess_memory_macros(mut out: String, contract_name: &str) -> String {
    out = out.replace("self.memory.load8(", "mload8!(");
    out = out.replace("self.memory.load16(", "mload16!(");
    out = out.replace("self.memory.load32(", "mload32!(");
    out = out.replace("self.memory.load64(", "mload64!(");
    out = out.replace("self.memory.store8(", "mstore8!(");
    out = out.replace("self.memory.store16(", "mstore16!(");
    out = out.replace("self.memory.store32(", "mstore32!(");
    out = out.replace("self.memory.store64(", "mstore64!(");
    out = out.replace("self.memory.size()", "msize!()");
    out = out.replace("self.memory.grow(", "mgrow!(");

    out = strip_impl_memory_macros(out);
    let macros = "macro_rules! mload8 {\n    ($addr:expr) => {{\n        let Self { memory, .. } = self;\n        memory.load8($addr)\n    }};\n}\nmacro_rules! mload16 {\n    ($addr:expr) => {{\n        let Self { memory, .. } = self;\n        memory.load16($addr)\n    }};\n}\nmacro_rules! mload32 {\n    ($addr:expr) => {{\n        let Self { memory, .. } = self;\n        memory.load32($addr)\n    }};\n}\nmacro_rules! mload64 {\n    ($addr:expr) => {{\n        let Self { memory, .. } = self;\n        memory.load64($addr)\n    }};\n}\nmacro_rules! mstore8 {\n    ($addr:expr, $value:expr) => {{\n        let Self { memory, .. } = self;\n        memory.store8($addr, $value)\n    }};\n}\nmacro_rules! mstore16 {\n    ($addr:expr, $value:expr) => {{\n        let Self { memory, .. } = self;\n        memory.store16($addr, $value)\n    }};\n}\nmacro_rules! mstore32 {\n    ($addr:expr, $value:expr) => {{\n        let Self { memory, .. } = self;\n        memory.store32($addr, $value)\n    }};\n}\nmacro_rules! mstore64 {\n    ($addr:expr, $value:expr) => {{\n        let Self { memory, .. } = self;\n        memory.store64($addr, $value)\n    }};\n}\nmacro_rules! msize {\n    () => {{\n        let Self { memory, .. } = self;\n        memory.size()\n    }};\n}\nmacro_rules! mgrow {\n    ($pages:expr) => {{\n        let Self { memory, .. } = self;\n        memory.grow($pages)\n    }};\n}\n";
    if !out.contains("macro_rules! mload8") {
        if let Some(pos) = out.find("\n#[contractimpl]") {
            out.insert_str(pos, &format!("\n{}\n", macros));
        } else {
            let impl_line = format!("\nimpl {} {{\n", contract_name);
            if let Some(pos) = out.find(&impl_line) {
                out.insert_str(pos, &format!("\n{}\n", macros));
            }
        }
    }

    let out = crate::engine::default_engine().apply(out);
    let out = fix_guard_breaks(out);
    let out = normalize_indentation(out);
    fix_trailing_braces(out)
}

fn strip_impl_memory_macros(input: String) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut skipping = false;
    let mut mgrow_seen = false;
    for line in input.lines() {
        if !skipping && line.trim_start().starts_with("macro_rules! mload8") {
            skipping = true;
            mgrow_seen = false;
            continue;
        }
        if skipping {
            if line.trim_start().starts_with("macro_rules! mgrow") {
                mgrow_seen = true;
                continue;
            }
            if mgrow_seen && line.trim() == "}" {
                skipping = false;
                mgrow_seen = false;
            }
            continue;
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

fn fix_guard_breaks(output: String) -> String {
    let mut out_lines: Vec<String> = Vec::new();
    let mut guard_stack: Vec<(String, i32)> = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(label) = parse_guard_label(trimmed) {
            guard_stack.push((label, 1));
            out_lines.push(line.to_string());
            continue;
        }
        if let Some((label, depth)) = guard_stack.last_mut() {
            let mut new_line = line.to_string();
            if new_line.contains("break;") && !new_line.contains("break '") {
                new_line = new_line.replace("break;", &format!("break '{label};"));
            }
            let opens = new_line.chars().filter(|&c| c == '{').count() as i32;
            let closes = new_line.chars().filter(|&c| c == '}').count() as i32;
            *depth += opens - closes;
            if *depth <= 0 {
                guard_stack.pop();
            }
            out_lines.push(new_line);
            continue;
        }
        out_lines.push(line.to_string());
    }
    out_lines.join("\n")
}

fn parse_guard_label(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with("'__if_guard") || !trimmed.ends_with('{') {
        return None;
    }
    let before = trimmed.trim_end_matches('{').trim();
    let parts: Vec<&str> = before.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let label = parts[0].trim().trim_start_matches('\'').to_string();
    if label.starts_with("__if_guard") {
        Some(label)
    } else {
        None
    }
}

fn normalize_indentation(output: String) -> String {
    let lines: Vec<String> = output.lines().map(|l| l.to_string()).collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut depth: i32 = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            out.push(String::new());
            continue;
        }

        let mut effective_depth = depth;
        if trimmed.starts_with('}') {
            effective_depth = (depth - 1).max(0);
        }

        let indent = " ".repeat((effective_depth as usize) * 4);
        out.push(format!("{indent}{trimmed}"));

        let opens = trimmed.chars().filter(|&c| c == '{').count() as i32;
        let closes = trimmed.chars().filter(|&c| c == '}').count() as i32;
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }
    }
    out.join("\n")
}

fn fix_trailing_braces(output: String) -> String {
    let mut open = 0i32;
    let mut close = 0i32;
    for ch in output.chars() {
        if ch == '{' {
            open += 1;
        } else if ch == '}' {
            close += 1;
        }
    }
    if open <= close {
        return output;
    }
    let mut out = output;
    for _ in 0..(open - close) {
        out.push('\n');
        out.push('}');
    }
    out
}


fn collect_self_calls(output: &str) -> std::collections::HashSet<String> {
    let mut calls: std::collections::HashSet<String> = std::collections::HashSet::new();
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
    let helper_methods: std::collections::HashSet<&'static str> = [
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
    ]
    .iter()
    .copied()
    .collect();
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
                while j > 0 {
                    j -= 1;
                    let prev = lines[j].trim();
                    if prev.is_empty() {
                        continue;
                    }
                    eligible_impl = prev == "#[allow(dead_code)]";
                    break;
                }
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

fn parse_len_line(line: &str) -> Option<String> {
    // expects: let <var> = Vec::<Val>::from_val(env, &val_from_i64(<x>)).len() as i64;
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains("Vec::<Val>::from_val(env, &val_from_i64(") {
        return None;
    }
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    if !(trimmed.ends_with(".len() as i64;") || trimmed.ends_with(".len() as i64")) {
        return None;
    }
    Some(var)
}

fn parse_len_cmp_line(line: &str) -> Option<(String, String)> {
    // expects: if ((a ^ b) as u64 >= 4294967296 as u64) as i32 != 0 {
    let trimmed = line.trim();
    if !trimmed.starts_with("if ((") || !trimmed.contains(") as u64 >= 4294967296 as u64) as i32 != 0 {") {
        return None;
    }
    let inner = trimmed.strip_prefix("if ((")?.strip_suffix(") as u64 >= 4294967296 as u64) as i32 != 0 {")?;
    let mut parts = inner.split('^').map(|s| s.trim());
    let a = parts.next()?.to_string();
    let b = parts.next()?.to_string();
    Some((a, b))
}

fn parse_err_contract_line(line: &str) -> Option<String> {
    // expects: let <var> = err_contract(10);
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.ends_with(" = err_contract(10);") {
        return None;
    }
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some(var)
}

fn parse_assign_var_line(line: &str, rhs: &str) -> Option<String> {
    // expects: <var> = <rhs>;
    let trimmed = line.trim();
    if !trimmed.ends_with(';') {
        return None;
    }
    let mut parts = trimmed[..trimmed.len() - 1].splitn(2, '=').map(|s| s.trim());
    let left = parts.next()?;
    let right = parts.next()?;
    if right != rhs {
        return None;
    }
    Some(left.to_string())
}

fn parse_break_label_line(line: &str) -> Option<String> {
    // expects: break 'labelX;
    let trimmed = line.trim();
    if !trimmed.starts_with("break '") || !trimmed.ends_with(";") {
        return None;
    }
    let inner = trimmed.strip_prefix("break '")?.strip_suffix(";")?;
    Some(inner.to_string())
}


fn parse_vec_new_line(line: &str) -> Option<(String, String)> {
    // expects: let <var> = val_to_i64(Vec::<Val>::new(env).into_val(env))
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains("= val_to_i64(Vec::<Val>::new(env).into_val(env))") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some((indent, var))
}


fn parse_vec_push_line(line: &str) -> Option<(String, String, String, String)> {
    // expects: let <var> = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(<vec>)); v.push_back(val_from_i64(<val>)); val_to_i64(v.into_val(env)) }
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains("Vec::<Val>::from_val(env, &val_from_i64(") {
        return None;
    }
    if !trimmed.contains("v.push_back(val_from_i64(") || !trimmed.contains("val_to_i64(v.into_val(env))") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let target = parts.next()?.trim().to_string();
    let vec_start = trimmed.find("Vec::<Val>::from_val(env, &val_from_i64(")? + "Vec::<Val>::from_val(env, &val_from_i64(".len();
    let vec_end = trimmed[vec_start..].find("))")? + vec_start;
    let vec_var = trimmed[vec_start..vec_end].trim().to_string();
    let val_start = trimmed.find("v.push_back(val_from_i64(")? + "v.push_back(val_from_i64(".len();
    let val_end = trimmed[val_start..].find("))")? + val_start;
    let val_var = trimmed[val_start..val_end].trim().to_string();
    Some((indent, target, vec_var, val_var))
}


struct VecStringIterInfo {
    indent: String,
    vec_var: String,
    body_line: String,
    end_index: usize,
}

fn parse_vec_string_iter_block(lines: &[&str], start: usize) -> Option<VecStringIterInfo> {
    let line = lines.get(start)?;
    let trimmed = line.trim();
    if trimmed != "'label0: loop {" {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let l1 = lines.get(start + 1)?.trim();
    if l1 != "'label1: loop {" {
        return None;
    }
    let l2 = lines.get(start + 2)?.trim();
    if !l2.contains("self.vec_next_string_flag(env,") {
        return None;
    }
    let vec_var = extract_vec_var_from_next_string(l2)?;
    // scan for guard+break and body line we can inline
    let mut idx = start + 3;
    let mut body_line: Option<String> = None;
    let mut end_index = None;
    while idx + 1 < lines.len() {
        let cur = lines[idx].trim();
        if cur.starts_with("if (slot_") && cur.contains("== 0") && lines[idx + 1].trim() == "break 'label0;" {
            idx += 2;
            continue;
        }
        if cur.starts_with("break;") || cur.starts_with("break 'label1") {
            idx += 1;
            continue;
        }
        if cur.starts_with("var") && cur.contains("= Error(Storage, MissingValue)") {
            // outer error handled by caller
            idx += 1;
            continue;
        }
        if cur.starts_with("let ") && cur.contains("self.vec_push_val(env") {
            body_line = Some(cur.to_string());
        }
        if cur == "}" && lines.get(idx + 1).map(|s| s.trim()) == Some("}") {
            end_index = Some(idx + 1);
            break;
        }
        idx += 1;
    }
    let body_line = body_line?;
    let end_index = end_index?;
    Some(VecStringIterInfo {
        indent,
        vec_var,
        body_line,
        end_index,
    })
}

fn extract_vec_var_from_next_string(line: &str) -> Option<String> {
    // expects: self.vec_next_string_flag(env, <tmp>, <vec_ptr>);
    let inner = line.strip_prefix("self.vec_next_string_flag(env,")?.strip_suffix(");")?;
    let mut parts = inner.split(',').map(|s| s.trim());
    parts.next()?;
    parts.next()?;
    let vec = parts.next()?.to_string();
    Some(vec)
}


fn parse_vec_new_assign(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains("= self.vec_new_val(env);") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some((indent, var))
}

fn parse_vec_push_assign(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.contains("= self.vec_push_val(env,") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some((indent, var))
}

fn parse_vec_push_args(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    let start = trimmed.find("self.vec_push_val(env,")? + "self.vec_push_val(env,".len();
    let inner = trimmed[start..].trim();
    let end = inner.find(");")?;
    let args = inner[..end].split(',').map(|s| s.trim()).collect::<Vec<_>>();
    if args.len() != 2 {
        return None;
    }
    Some((args[0].to_string(), args[1].to_string()))
}

fn parse_simple_assign(line: &str, rhs: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.ends_with(';') {
        return None;
    }
    let mut parts = trimmed[..trimmed.len() - 1].splitn(2, '=').map(|s| s.trim());
    let left = parts.next()?;
    let right = parts.next()?;
    if right != rhs {
        return None;
    }
    Some(left.to_string())
}

fn parse_dead_slot_pair(line1: &str, line2: &str) -> Option<String> {
    // line1: let mut slot_varX_0_i64 = mload64!(<src> as usize) as i64;
    // line2: let mut slot_varY_0_i64 = slot_varX_0_i64 as i64;
    let t1 = line1.trim();
    let t2 = line2.trim();
    if !t1.starts_with("let mut slot_") || !t1.contains("= mload64!(") {
        return None;
    }
    if !t2.starts_with("let mut slot_") || !t2.contains("= ") || !t2.contains(" as i64;") {
        return None;
    }
    let src = parse_base_from_mload64(t1)?;
    Some(src)
}

fn parse_mload64_line(line: &str, src: &str) -> Option<String> {
    // line: let var = mload64!(<src>.wrapping_add(8) as usize) as i64;
    let t = line.trim();
    if !t.starts_with("let ") || !t.contains("= mload64!(") {
        return None;
    }
    if !t.contains(src) {
        return None;
    }
    let mut parts = t.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some(var)
}

fn is_mstore64_line(line: &str, var: &str) -> bool {
    // line: mstore64!(... , <var> as u64);
    let t = line.trim();
    t.starts_with("mstore64!(") && t.contains(&format!("{var} as u64"))
}

fn parse_slot_from_feed_id(line: &str) -> Option<String> {
    let t = line.trim();
    if !t.starts_with("let mut ") || !t.ends_with(" = feed_id as i64;") {
        return None;
    }
    let mut parts = t.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some(var)
}


struct VecNextStringBlock {
    indent: String,
    base: String,
    tmp: String,
    iter_ptr: String,
    break_label: String,
    end_index: usize,
}

fn parse_vec_next_string_block(lines: &[&str], start: usize) -> Option<VecNextStringBlock> {
    let line0 = lines.get(start)?.trim();
    if !line0.starts_with("self.vec_next_string_flag(env,") {
        return None;
    }
    let indent = lines[start].chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let (tmp, iter_ptr) = parse_vec_next_string_args(line0)?;
    let line1 = lines.get(start + 1)?.trim();
    let line2 = lines.get(start + 2)?.trim();
    let line3 = lines.get(start + 3)?.trim();
    let line4 = lines.get(start + 4)?.trim();
    let line5 = lines.get(start + 5)?.trim();
    let base = parse_base_from_mload64(line1)?;
    if parse_base_from_mload64(line2)? != base {
        return None;
    }
    if !line3.starts_with("self.guard_nonzero_ptr(env,") || !line3.contains(&base) {
        return None;
    }
    if parse_base_from_mload32(line4)? != base {
        return None;
    }
    if !line5.starts_with("if (") || !line5.ends_with("{") {
        return None;
    }
    let break_line = lines.get(start + 6)?.trim();
    let label = parse_break_label_line(break_line)?;
    let close_line = lines.get(start + 7)?.trim();
    if close_line != "}" {
        return None;
    }
    Some(VecNextStringBlock {
        indent,
        base,
        tmp,
        iter_ptr,
        break_label: label,
        end_index: start + 7,
    })
}

fn parse_vec_next_string_args(line: &str) -> Option<(String, String)> {
    let inner = line.strip_prefix("self.vec_next_string_flag(env,")?.strip_suffix(");")?;
    let mut parts = inner.split(',').map(|s| s.trim());
    let tmp = parts.next()?.to_string();
    let iter_ptr = parts.next()?.to_string();
    Some((tmp, iter_ptr))
}

fn parse_base_from_mload64(line: &str) -> Option<String> {
    // expects: let mut slot_* = mload64!(<base> as usize + 64) as i64;
    let trimmed = line.trim();
    let idx = trimmed.find("mload64!(")?;
    let inner = &trimmed[idx + "mload64!(".len()..];
    if let Some(end) = inner.find("as usize +") {
        return Some(inner[..end].trim().to_string());
    }
    if let Some(end) = inner.find("as usize)") {
        return Some(inner[..end].trim().to_string());
    }
    None
}

fn parse_base_from_mload32(line: &str) -> Option<String> {
    // expects: let mut slot_* = mload32!(<base> as usize + 24) as i32;
    let trimmed = line.trim();
    let idx = trimmed.find("mload32!(")?;
    let inner = &trimmed[idx + "mload32!(".len()..];
    let end = inner.find("as usize +")?;
    Some(inner[..end].trim().to_string())
}

fn parse_break_label_from_if(line: &str) -> Option<String> {
    let _ = line;
    None
}

fn parse_frame_decl_line(line: &str) -> Option<(String, String)> {
    // expects: let frame = self.global0.wrapping_sub(96);
    let trimmed = line.trim();
    if !trimmed.starts_with("let ") || !trimmed.ends_with(" = self.global0.wrapping_sub(96);") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let mut parts = trimmed.splitn(3, ' ');
    parts.next()?;
    let var = parts.next()?.trim().to_string();
    Some((var, indent))
}

fn is_global0_assign(line: &str, var: &str) -> bool {
    line.trim() == format!("self.global0 = {};", var)
}

fn is_global0_restore(line: &str, var: &str) -> bool {
    line.trim() == format!("self.global0 = {}.wrapping_add(96);", var)
}

fn is_pack_ok_val_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("let _ = self.pack_ok_val(") && trimmed.ends_with(");")
}

fn parse_write_ok_val_line(line: &str) -> Option<(String, String, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("self.write_ok_val(") || !trimmed.ends_with(");") {
        return None;
    }
    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
    let inner = trimmed.strip_prefix("self.write_ok_val(")?.strip_suffix(");")?;
    let mut parts = inner.split(',').map(|s| s.trim());
    let _env = parts.next()?;
    let tmp = parts.next()?.to_string();
    let val_expr = parts.next()?.to_string();
    Some((tmp, val_expr, indent))
}

fn is_slot_load32_line(line: &str, tmp: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("let mut slot_") && trimmed.contains(&format!("mload32!({} as usize)", tmp))
}

fn is_slot_load64_line(line: &str, tmp: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("let mut slot_") && trimmed.contains(&format!("mload64!({} as usize + 8)", tmp))
}

fn is_zero_flag_if_line(if_line: &str, slot_line: &str) -> bool {
    let slot_name = slot_line
        .trim()
        .split_whitespace()
        .nth(2)
        .unwrap_or("");
    let trimmed = if_line.trim();
    trimmed == format!("if {} != 0 {{", slot_name)
}

fn is_direct_flag_if_line(if_line: &str, tmp: &str) -> bool {
    let trimmed = if_line.trim();
    trimmed == format!("if (mload32!({} as usize) != 0) as i32 != 0 {{", tmp)
}

fn is_break_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("break '") && trimmed.ends_with(";")
}

fn parse_assign_from_slot_line(line: &str, slot_line: &str) -> Option<String> {
    let slot_name = slot_line
        .trim()
        .split_whitespace()
        .nth(2)
        .unwrap_or("");
    let trimmed = line.trim();
    if !trimmed.ends_with(';') {
        return None;
    }
    let mut parts = trimmed[..trimmed.len() - 1].splitn(2, '=').map(|s| s.trim());
    let left = parts.next()?;
    let right = parts.next()?;
    if right != slot_name {
        return None;
    }
    Some(left.to_string())
}

fn collect_needed_function_indices(
    module: &parity_wasm::elements::Module,
    code: &CodeSection,
    import_count: usize,
    exports: &ExportSection,
) -> HashSet<u32> {
    let mut needed: HashSet<u32> = HashSet::new();
    let mut stack: Vec<u32> = Vec::new();

    for export in exports.entries() {
        if let &Internal::Function(func_index) = export.internal() {
            stack.push(func_index);
        }
    }

    let mut table_elems: Vec<u32> = Vec::new();
    if let Some(elem_section) = module.elements_section() {
        for segment in elem_section.entries() {
            table_elems.extend(segment.members().iter().copied());
        }
    }
    let mut table_added = false;

    while let Some(func_index) = stack.pop() {
        if needed.contains(&func_index) {
            continue;
        }
        needed.insert(func_index);
        if (func_index as usize) < import_count {
            continue;
        }
        let body_index = func_index as usize - import_count;
        if let Some(body) = code.bodies().get(body_index) {
            for instr in body.code().elements() {
                match instr {
                    Instruction::Call(n) => {
                        stack.push(*n);
                    }
                    Instruction::CallIndirect(_, _) => {
                        if !table_added && !table_elems.is_empty() {
                            stack.extend(table_elems.iter().copied());
                            table_added = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    needed
}

pub fn run(opt: Opt) -> Result<(), String> {
    let input = opt.input;
    let output_path = opt.output.unwrap_or_else(|| input.with_extension("rs"));
    let sdk = get_backend();
    let modules = sdk
        .env_common_modules_result()
        .map_err(|err| format!("Error: {}", err))?;
    let module = deserialize_file(&input).map_err(|err| format!("{}", err))?;
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);
    let data_segments = extract_data_segments(&module);
    let data_segments_with_offsets = extract_data_segments_with_offsets(&module);
    let string_literals = extract_string_literals(&module);

    let mut writer: Vec<u8> = Vec::new();
    let contract_name = utils::contract_name_from_module_or_path(&module, &input);
    let contract_specs = match sdk.read_contract_specs(&input) {
        Ok(specs) => specs,
        Err(_err) => ContractSpecs::default(),
    };
    let type_flags = utils::spec_type_flags(&contract_specs);
    let has_datakey_type = type_flags.has_datakey_type;
    let has_allowance_value_type = type_flags.has_allowance_value_type;
    let has_allowance_key_type = type_flags.has_allowance_key_type;
    let has_token_metadata_type = type_flags.has_token_metadata_type;
    let has_signer_variant = type_flags.has_signer_variant;
    let has_signer_cnt_variant = type_flags.has_signer_cnt_variant;
    let has_admin_variant = type_flags.has_admin_variant;
    let has_spend_limit_variant = type_flags.has_spend_limit_variant;
    let has_counter_variant = type_flags.has_counter_variant;
    let has_owner_variant = type_flags.has_owner_variant;
    let data_key_variants = utils::parse_datakey_variants(&contract_specs);
    let struct_defs = utils::parse_struct_defs(&contract_specs);
    let is_account_contract = utils::is_account_contract(&contract_specs);

    let import_count = module.import_count(ImportCountType::Function);
    let code = module.code_section().ok_or("Missing code section")?;
    let fns = module.function_section().ok_or("Missing function section")?;
    let types = module.type_section().ok_or("Missing type section")?;
    let exports = module.export_section().ok_or("Missing export section")?;

    let function_names = module
        .sections()
        .iter()
        .filter_map(|s| match s {
            Section::Name(name_section) => name_section.functions(),
            _ => None,
        })
        .next();

    let mut functions = utils::build_functions(
        &modules,
        module.import_section(),
        types,
        fns,
        function_names,
        opt.use_name_section,
    );
    let mut used_names: HashSet<String> = functions.iter().map(|f| f.name.clone()).collect();
    for (i, (body, func)) in code.bodies().iter().zip(fns.entries()).enumerate() {
        let type_index = func.type_ref();
        let typ = &types.types()[type_index as usize];
        let fn_type = match *typ {
            Type::Function(ref t) => t,
        };
        let fn_index = import_count + i;
        if fn_index >= functions.len() {
            continue;
        }
        if functions[fn_index].make_public {
            continue;
        }
        if !functions[fn_index].name.starts_with("func") {
            continue;
        }
        let fingerprint = fingerprint_function(body, fn_type, import_count, &functions);
        if std::env::var("SOROBAN_AUDITOR_DUMP_FP").is_ok() {
            eprintln!("fp {} {}", functions[fn_index].name, fingerprint.short());
        }
        if let Some(suggested) = suggested_name_for_fingerprint(&fingerprint) {
            let mut name = suggested.to_string();
            if used_names.contains(&name) {
                let mut n = 2;
                loop {
                    let candidate = format!("{}_{}", name, n);
                    if !used_names.contains(&candidate) {
                        name = candidate;
                        break;
                    }
                    n += 1;
                }
            }
            used_names.remove(&functions[fn_index].name);
            functions[fn_index].name = name.clone();
            used_names.insert(name);
        }
    }

    let mut import_items =
        collect_imports(&contract_specs, &exports, &code, &functions, import_count, is_account_contract);
    for extra in [
        "Val",
        "Address",
        "FromVal",
        "IntoVal",
        "Vec",
        "Map",
        "Bytes",
        "BytesN",
        "String",
        "Symbol",
    ] {
        if !import_items.iter().any(|item| item == extra) {
            import_items.push(extra.to_string());
        }
    }
    let import_line = format!("use soroban_sdk::{{{}}};", import_items.join(", "));

    let contract_struct_vis = if is_account_contract { "" } else { "pub " };
    writeln!(
        writer,
        "#![no_std]\n{}\n\n#[contract]\n{}struct {};",
        import_line,
        contract_struct_vis,
        contract_name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer).map_err(|e| e.to_string())?;
    writeln!(writer, "fn val_from_i64(v: i64) -> Val {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "    unsafe {{ core::mem::transmute::<u64, Val>(v as u64) }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(writer, "fn val_to_i64(v: Val) -> i64 {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "    unsafe {{ core::mem::transmute::<Val, u64>(v) }} as i64"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(writer, "fn err_contract(code: u32) -> i64 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "    ((soroban_sdk::xdr::ScErrorType::Contract as u32 as i64) & 255).wrapping_shl(32 as u32) | (code as i64)").map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "fn address_from_i64(env: &Env, v: i64) -> Address {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    Address::from_val(env, &val_from_i64(v))")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;

    if is_account_contract {
        writeln!(writer).map_err(|e| e.to_string())?;
        writeln!(writer, "#[contracttype]").map_err(|e| e.to_string())?;
        writeln!(writer, "#[derive(Clone)]").map_err(|e| e.to_string())?;
        writeln!(writer, "pub struct AccSignature {{").map_err(|e| e.to_string())?;
        writeln!(writer, "    pub public_key: BytesN<32>,").map_err(|e| e.to_string())?;
        writeln!(writer, "    pub signature: BytesN<64>,").map_err(|e| e.to_string())?;
        writeln!(writer, "}}").map_err(|e| e.to_string())?;
        writeln!(writer).map_err(|e| e.to_string())?;
        writeln!(writer, "#[contracttype]").map_err(|e| e.to_string())?;
        writeln!(writer, "#[derive(Clone)]").map_err(|e| e.to_string())?;
        writeln!(writer, "enum DataKey {{").map_err(|e| e.to_string())?;
        writeln!(writer, "    SignerCnt,").map_err(|e| e.to_string())?;
        writeln!(writer, "    Signer(BytesN<32>),").map_err(|e| e.to_string())?;
        writeln!(writer, "    SpendLimit(Address),").map_err(|e| e.to_string())?;
        writeln!(writer, "}}").map_err(|e| e.to_string())?;
        writeln!(writer).map_err(|e| e.to_string())?;
        writeln!(writer, "#[contracterror]").map_err(|e| e.to_string())?;
        writeln!(
            writer,
            "#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]"
        )
        .map_err(|e| e.to_string())?;
        writeln!(writer, "#[repr(u32)]").map_err(|e| e.to_string())?;
        writeln!(writer, "pub enum AccError {{").map_err(|e| e.to_string())?;
        writeln!(writer, "    NotEnoughSigners = 1,").map_err(|e| e.to_string())?;
        writeln!(writer, "    NegativeAmount = 2,").map_err(|e| e.to_string())?;
        writeln!(writer, "    BadSignatureOrder = 3,").map_err(|e| e.to_string())?;
        writeln!(writer, "    UnknownSigner = 4,").map_err(|e| e.to_string())?;
        writeln!(writer, "}}").map_err(|e| e.to_string())?;
        writeln!(writer).map_err(|e| e.to_string())?;
        writeln!(
            writer,
            "const TRANSFER_FN: Symbol = symbol_short!(\"transfer\");"
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            writer,
            "const APPROVE_FN: Symbol = symbol_short!(\"approve\");"
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            writer,
            "const BURN_FN: Symbol = symbol_short!(\"burn\");"
        )
        .map_err(|e| e.to_string())?;
    } else if !contract_specs.types().is_empty() {
        writeln!(writer).map_err(|e| e.to_string())?;
        for ty in contract_specs.types() {
            let formatted = format_spec_tokens(&ty.to_string());
            writeln!(writer, "{}", formatted).map_err(|e| e.to_string())?;
        }
    }

    let mut indirect_fns = BTreeMap::new();
    let mut pattern_state = PatternState::default();
    let spec_by_fn_index = utils::build_spec_index(&contract_specs, exports);
    let internal_call_forwarders =
        utils::build_internal_call_forwarders(&code, fns, import_count, &functions);

    let _has_dynamic_element_section_offset = false;

    let (globals, imported_globals_count) =
        utils::build_globals(module.import_section(), &module, exports);

 

    let impl_block = r#"
#[contractimpl]
impl {contract_name} {"#
        .replace("{contract_name}", &contract_name);
    writeln!(writer, "{}", impl_block).map_err(|e| e.to_string())?;

    // Create template matcher for pattern-based code generation
    let template_matcher = TemplateMatcher::with_library(TemplateLibrary::new());

    let mut account_check_auth: Option<FunctionContractSpec> = None;
    for export in exports.entries() {
        let spec_fn = specs::find_spec_for_export(&contract_specs, export);
        if let Some(spec_fn) = spec_fn {
            let ctx_data = specs::build_pattern_context_data(export, import_count, &code, &functions);
            let input_types: Vec<String> = spec_fn
                .inputs()
                .iter()
                .map(|p| p.type_ident().to_string())
                .collect();
            let addr_indices: Vec<usize> = input_types
                .iter()
                .enumerate()
                .filter_map(|(i, t)| if t.contains("Address") { Some(i) } else { None })
                .collect();
            let i128_indices: Vec<usize> = input_types
                .iter()
                .enumerate()
                .filter_map(|(i, t)| if t.contains("i128") { Some(i) } else { None })
                .collect();

            let ctx = specs::build_pattern_context(
                &ctx_data,
                &spec_fn,
                &string_literals,
                &data_segments,
                has_datakey_type,
                has_allowance_value_type,
                has_allowance_key_type,
                has_token_metadata_type,
                has_signer_variant,
                has_signer_cnt_variant,
                has_admin_variant,
                has_spend_limit_variant,
                has_counter_variant,
                has_owner_variant,
                &data_key_variants,
                &struct_defs,
                &input_types,
                &addr_indices,
                &i128_indices,
                is_account_contract,
            );

            if is_account_contract
                && (ctx_data.export_name == "__check_auth"
                    || ctx_data.export_name == "___check_auth")
            {
                account_check_auth = Some(spec_fn);
                continue;
            }

            // Try template-based generation first (higher priority than hardcoded patterns)
            if let Some(generated) = template_matcher.try_generate_with_context(&spec_fn, &ctx_data.export_name, &ctx) {
                writeln!(writer, "{}", generated).map_err(|e| e.to_string())?;
                continue;
            }

            // Fallback to existing hardcoded patterns
            if crate::patterns::try_emit(&mut writer, &spec_fn, &ctx, &mut pattern_state) {
                continue;
            }

            let mut assigned_params: Vec<bool> = Vec::new();
            let mut fn_index_for_body: Option<u32> = None;
            let mut body_for_fallback = None;
            let mut fn_type_for_body = None;
            if let &Internal::Function(fn_index) = export.internal() {
                if fn_index as usize >= import_count {
                    let body_index = fn_index as usize - import_count;
                    if let Some(body) = code.bodies().get(body_index) {
                        let type_index = fns.entries()[body_index].type_ref();
                        if let Type::Function(ref fn_type) = types.types()[type_index as usize] {
                            assigned_params = vec![false; fn_type.params().len()];
                            for instr in body.code().elements() {
                                if let Instruction::SetLocal(i) | Instruction::TeeLocal(i) = *instr {
                                    if (i as usize) < assigned_params.len() {
                                        assigned_params[i as usize] = true;
                                    }
                                }
                            }
                            fn_index_for_body = Some(fn_index);
                            body_for_fallback = Some(body);
                            fn_type_for_body = Some(fn_type);
                        }
                    }
                }
            }

            let env_mut = assigned_params.get(0).copied().unwrap_or(false);
            let env_param = if env_mut { "mut env: Env" } else { "env: Env" };
            write!(
                writer,
                "    pub fn {}(&mut self, {}",
                ctx_data.export_name, env_param
            )
            .map_err(|e| e.to_string())?;
            for (idx, argument) in spec_fn.inputs().iter().enumerate() {
                let ty = format_type_ident(&argument.type_ident().to_string());
                let is_mut = assigned_params.get(idx + 1).copied().unwrap_or(false);
                if is_mut {
                    write!(writer, ", mut {}: {}", argument.name(), ty)
                } else {
                    write!(writer, ", {}: {}", argument.name(), ty)
                }
                .map_err(|e| e.to_string())?;
            }
            write!(writer, ")").map_err(|e| e.to_string())?;
            if let Some(return_type) = spec_fn.output() {
                let ty = format_type_ident(&return_type.type_ident().to_string());
                write!(writer, " -> {}", ty).map_err(|e| e.to_string())?;
            }
            writeln!(writer, " {{").map_err(|e| e.to_string())?;
            let enable_raw_fallback = true;
            let mut emitted_raw = false;
            if enable_raw_fallback {
                if let (Some(body), Some(fn_type), Some(fn_index)) =
                    (body_for_fallback, fn_type_for_body, fn_index_for_body)
                {
                    let mut expr_index = fn_type.params().len();
                    for local in body.locals() {
                        let ty = crate::wasm_ir::to_rs_type(local.value_type());
                        let decimals = if ty.starts_with('f') { ".0" } else { "" };
                        for _ in 0..local.count() {
                            writeln!(
                                writer,
                                "        let mut var{}: {} = 0{};",
                                expr_index, ty, decimals
                            )
                            .map_err(|e| e.to_string())?;
                            expr_index += 1;
                        }
                    }
                    crate::code_builder::build(
                        &mut writer,
                        expr_index,
                        fn_type.results().first().is_some(),
                        import_count,
                        imported_globals_count,
                        &functions,
                        &mut indirect_fns,
                        &globals,
                        types,
                        body.code().elements(),
                        2,
                        &spec_by_fn_index,
                        fn_index as usize,
                        &data_segments_with_offsets,
                        &internal_call_forwarders,
                    );
                    emitted_raw = true;
                }
            }
            if !emitted_raw {
                if ctx.has_fail_with_error {
                    writeln!(writer, "        panic!(\"contract error\");")
                        .map_err(|e| e.to_string())?;
                } else {
                    writeln!(writer, "        panic!(\"decompilation pending\");")
                        .map_err(|e| e.to_string())?;
                }
                writeln!(writer, "    }}").map_err(|e| e.to_string())?;
            }
        }
    }

        writeln!(writer).map_err(|e| e.to_string())?;
    if is_account_contract {
        if let Some(spec_fn) = account_check_auth {
            let ctx_data = specs::PatternContextData {
                export_name: spec_fn.name().to_string(),
                has_vec_new: false,
                uses_string_new: false,
                uses_symbol_new: false,
                uses_bytes_new: false,
                require_auth_calls: 0,
                require_auth_for_args_calls: 0,
                uses_current_contract_address: false,
                symbol_literals: Vec::new(),
                has_fail_with_error: false,
                uses_get_contract_data: false,
                uses_put_contract_data: false,
                uses_contract_event: false,
                uses_update_current_contract_wasm: false,
            };
            let input_types: Vec<String> = spec_fn
                .inputs()
                .iter()
                .map(|p| p.type_ident().to_string())
                .collect();
            let addr_indices: Vec<usize> = input_types
                .iter()
                .enumerate()
                .filter_map(|(i, t)| if t.contains("Address") { Some(i) } else { None })
                .collect();
            let i128_indices: Vec<usize> = input_types
                .iter()
                .enumerate()
                .filter_map(|(i, t)| if t.contains("i128") { Some(i) } else { None })
                .collect();
            let ctx = specs::build_pattern_context(
                &ctx_data,
                &spec_fn,
                &string_literals,
                &data_segments,
                has_datakey_type,
                has_allowance_value_type,
                has_allowance_key_type,
                has_token_metadata_type,
                has_signer_variant,
                has_signer_cnt_variant,
                has_admin_variant,
                has_spend_limit_variant,
                has_counter_variant,
                has_owner_variant,
                &data_key_variants,
                &struct_defs,
                &input_types,
                &addr_indices,
                &i128_indices,
                is_account_contract,
            );
            writeln!(writer).map_err(|e| e.to_string())?;
            writeln!(writer, "#[contractimpl(contracttrait)]").map_err(|e| e.to_string())?;
            writeln!(
                writer,
                "impl CustomAccountInterface for {} {{",
                contract_name
            )
            .map_err(|e| e.to_string())?;
            writeln!(writer, "    type Signature = Vec<AccSignature>;").map_err(|e| e.to_string())?;
            writeln!(writer, "    type Error = AccError;").map_err(|e| e.to_string())?;
            crate::patterns::account::try_emit_check_auth_trait(&mut writer, &spec_fn, &ctx);
            writeln!(writer, "}}").map_err(|e| e.to_string())?;
            writeln!(writer).map_err(|e| e.to_string())?;
            crate::patterns::account::emit_account_helpers(&mut writer);
        }
    }

    let output = String::from_utf8(writer).map_err(|e| e.to_string())?;
    let output = postprocess_memory_macros(output, &contract_name);
    let output = postprocess_remove_unused_methods(output, &contract_name);
    std::fs::write(output_path, output).map_err(|e| e.to_string())?;
    Ok(())
}
