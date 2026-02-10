use crate::decompile::{collect_imports, extract_data_segments_with_offsets};
use crate::fingerprint::fingerprint_function;
use crate::format::{format_spec_tokens, format_type_ident};
use crate::rewrites::suggested_name_for_fingerprint;
use crate::sdk::{get_backend, ContractSpecs};
use parity_wasm::deserialize_file;
use parity_wasm::elements::{
    ImportCountType, Instruction, Internal, Section, Type,
};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod utils;

mod raw;
mod specs;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(
        short = "n",
        long = "use-name-section",
        help = "Use the names in the name section for the internal function names"
    )]
    use_name_section: bool,
    #[structopt(
        long = "sdk-report",
        help = "Print detailed SDK usage report to stderr"
    )]
    sdk_report: bool,
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

    let include_macro = "include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/memory_macros.rs\"));";
    if !out.contains(include_macro) {
        if let Some(pos) = out.find("\n#[contractimpl]") {
            out.insert_str(pos, &format!("\n{}\n\n", include_macro));
        } else {
            let impl_line = format!("\nimpl {} {{\n", contract_name);
            if let Some(pos) = out.find(&impl_line) {
                out.insert_str(pos, &format!("\n{}\n\n", include_macro));
            }
        }
    }

    let out = crate::engine::default_engine().apply(out);
    let out = fix_guard_breaks(out);
    let out = format_function_signatures(out);
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
    let mut in_signature = false;

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

        if !in_signature && trimmed.starts_with("pub fn") && trimmed.ends_with('(') {
            in_signature = true;
        }

        let mut indent_width = (effective_depth as usize) * 4;
        if in_signature && !trimmed.starts_with("pub fn") && !trimmed.starts_with(')') {
            indent_width += 4;
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
    }
    out.join("\n")
}

fn format_function_signatures(code: String) -> String {
    let mut out = String::new();
    let mut buffer = String::new();
    let mut in_signature = false;
    let mut paren_depth: i32 = 0;

    for line in code.lines() {
        let trimmed = line.trim_start();
        if !in_signature && trimmed.starts_with("pub fn") && trimmed.contains('(') {
            in_signature = true;
            paren_depth = trimmed.chars().filter(|&c| c == '(').count() as i32
                - trimmed.chars().filter(|&c| c == ')').count() as i32;
            buffer = format!("{}\n", line);
            continue;
        }

        if in_signature {
            buffer.push_str(&format!("{}\n", line));
            paren_depth += line.chars().filter(|&c| c == '(').count() as i32;
            paren_depth -= line.chars().filter(|&c| c == ')').count() as i32;
            if paren_depth <= 0 {
                out.push_str(&reflow_signature(&buffer));
                buffer.clear();
                in_signature = false;
            }
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }

    if !buffer.is_empty() {
        out.push_str(&reflow_signature(&buffer));
    }

    out
}

fn reflow_signature(block: &str) -> String {
    let mut result = String::new();
    for line in block.lines() {
        let leading = line
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let mut trimmed = line.trim_start().to_string();
        if trimmed.starts_with(")->") {
            trimmed = trimmed.replacen(")->", ") ->", 1);
        }
        if trimmed.starts_with("){") {
            trimmed = trimmed.replacen("){", ") {", 1);
        }
        trimmed = cleanup_signature_trailing_commas(trimmed);
        if trimmed.starts_with("pub fn") {
            result.push_str(&leading);
            result.push_str(&trimmed);
            result.push('\n');
            continue;
        }
        if trimmed.is_empty() {
            result.push('\n');
            continue;
        }
        if trimmed.ends_with(',') {
            result.push_str("            ");
            result.push_str(&trimmed);
            result.push('\n');
        } else if trimmed.ends_with(')') {
            result.push_str("            ");
            result.push_str(&trimmed);
            result.push('\n');
        } else {
            result.push_str("            ");
            result.push_str(&trimmed);
            result.push('\n');
        }
    }
    result
}

fn cleanup_signature_trailing_commas(mut line: String) -> String {
    while line.contains(",),") {
        line = line.replace(",),", "),");
    }
    while line.contains(",) ->") {
        line = line.replace(",) ->", ") ->");
    }
    while line.contains(",) {") {
        line = line.replace(",) {", ") {");
    }
    while line.contains(",)") {
        line = line.replace(",)", ")");
    }
    line
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
    for helper in ["address_from_i64", "err_contract"] {
        out = remove_top_level_helper(&out, helper);
    }
    out
}

/// Apply engine patterns to clean up generated code
/// Currently disabled due to parsing issues - returns input unchanged
fn postprocess_apply_patterns(output: String) -> String {
    // TODO: Re-enable when function parsing logic is fixed
    // The current implementation has issues with brace matching
    // causing parts of the file to be skipped
    output
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
    let data_segments_with_offsets = extract_data_segments_with_offsets(&module);

    let mut writer: Vec<u8> = Vec::new();
    let contract_name = utils::contract_name_from_module_or_path(&module, &input);
    let contract_specs = match sdk.read_contract_specs(&input) {
        Ok(specs) => specs,
        Err(_err) => ContractSpecs::default(),
    };
    let is_account_contract = utils::is_account_contract(&contract_specs);

    let import_count = module.import_count(ImportCountType::Function);
    let code = module.code_section().ok_or("Missing code section")?;
    let fns = module
        .function_section()
        .ok_or("Missing function section")?;
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

    let mut import_items = collect_imports(
        &contract_specs,
        &exports,
        &code,
        &functions,
        import_count,
        is_account_contract,
    );
    for extra in [
        "Val", "Address", "FromVal", "IntoVal", "Vec", "Map", "Bytes", "BytesN", "String", "Symbol",
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
        import_line, contract_struct_vis, contract_name
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
        "    (unsafe {{ core::mem::transmute::<Val, u64>(v) }}) as i64"
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
    writeln!(writer, "    Address::from_val(env, &val_from_i64(v))").map_err(|e| e.to_string())?;
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
        writeln!(writer, "const BURN_FN: Symbol = symbol_short!(\"burn\");")
            .map_err(|e| e.to_string())?;
    } else if !contract_specs.types().is_empty() {
        writeln!(writer).map_err(|e| e.to_string())?;
        for ty in contract_specs.types() {
            let formatted = format_spec_tokens(&ty.to_string());
            writeln!(writer, "{}", formatted).map_err(|e| e.to_string())?;
        }
    }

    let mut indirect_fns = BTreeMap::new();
    let spec_by_fn_index = utils::build_spec_index(&contract_specs, exports);

    // Use both forwarder detectors:
    // 1. Old system for simple forwarders (compatible with existing code)
    let call_forwarders = utils::build_call_forwarders(&code, fns, import_count, &functions);

    // 2. New ForwarderAnalyzer for complex patterns
    use crate::engine::forwarder_analyzer::ForwarderAnalyzer;
    let forwarder_analyzer = ForwarderAnalyzer::new(import_count);
    let complex_forwarders =
        forwarder_analyzer.analyze_all_functions(&code, import_count, &functions);

    // Log detected forwarders for debugging
    if std::env::var("SOROBAN_AUDITOR_DEBUG").is_ok() {
        eprintln!("Detected {} simple forwarders", call_forwarders.len());
        for (fn_idx, fwd) in &call_forwarders {
            eprintln!(
                "  Simple: func{} -> {}",
                fn_idx - import_count as u32,
                fwd.target
            );
        }
        eprintln!("Detected {} complex forwarders", complex_forwarders.len());
        for (fn_idx, info) in &complex_forwarders {
            eprintln!(
                "  Complex: func{} (complexity={}, type={:?}, target={})",
                fn_idx - import_count as u32,
                info.complexity_score,
                info.forwarder_type,
                info.target_function
            );
        }
    }

    let _has_dynamic_element_section_offset = false;

    let (globals, imported_globals_count) =
        utils::build_globals(module.import_section(), &module, exports);

    let impl_block = r#"
#[contractimpl]
impl {contract_name} {"#
        .replace("{contract_name}", &contract_name);
    writeln!(writer, "{}", impl_block).map_err(|e| e.to_string())?;
    let emit_raw_helpers = std::env::var("SOROBAN_AUDITOR_EMIT_RAW_FUNCTIONS").is_ok();

    for export in exports.entries() {
        let spec_fn = specs::find_spec_for_export(&contract_specs, export);
        if let Some(spec_fn) = spec_fn {
            let ctx_data =
                specs::build_pattern_context_data(export, import_count, &code, &functions);

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
                                if let Instruction::SetLocal(i) | Instruction::TeeLocal(i) = *instr
                                {
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
            writeln!(writer, "    pub fn {}(", ctx_data.export_name).map_err(|e| e.to_string())?;
            writeln!(writer, "        &mut self,").map_err(|e| e.to_string())?;
            writeln!(writer, "        {},", env_param).map_err(|e| e.to_string())?;
            for (idx, argument) in spec_fn.inputs().iter().enumerate() {
                let ty = format_type_ident(&argument.type_ident().to_string());
                let is_mut = assigned_params.get(idx + 1).copied().unwrap_or(false);
                if is_mut {
                    writeln!(writer, "        mut {}: {},", argument.name(), ty)
                } else {
                    writeln!(writer, "        {}: {},", argument.name(), ty)
                }
                .map_err(|e| e.to_string())?;
            }
            write!(writer, "    )").map_err(|e| e.to_string())?;
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
                        &call_forwarders,
                        &complex_forwarders,
                    );
                    emitted_raw = true;
                }
            }
            if !emitted_raw {
                writeln!(writer, "        panic!(\"decompilation pending\");")
                    .map_err(|e| e.to_string())?;
                writeln!(writer, "    }}").map_err(|e| e.to_string())?;
            }
        }
    }
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(writer).map_err(|e| e.to_string())?;

    if emit_raw_helpers {
        let helper_filter: HashSet<u32> = code
            .bodies()
            .iter()
            .enumerate()
            .filter_map(|(i, _)| {
                let fn_index = import_count + i;
                functions
                    .get(fn_index)
                    .and_then(|f| if !f.make_public { Some(fn_index as u32) } else { None })
            })
            .collect();

        writeln!(writer, "impl {} {{", contract_name).map_err(|e| e.to_string())?;
        raw::emit_raw_functions(
            &mut writer,
            code,
            fns,
            types,
            import_count,
            imported_globals_count,
            &functions,
            &globals,
            &mut indirect_fns,
            &spec_by_fn_index,
            &data_segments_with_offsets,
            &call_forwarders,
            &complex_forwarders,
            Some(&helper_filter),
            false,
        )?;
        writeln!(writer, "}}").map_err(|e| e.to_string())?;
        writeln!(writer).map_err(|e| e.to_string())?;
    }
    let output = String::from_utf8(writer).map_err(|e| e.to_string())?;
    let output = postprocess_memory_macros(output, &contract_name);
    let output = postprocess_remove_unused_methods(output, &contract_name);
    let output = postprocess_remove_unused_top_level_helpers(output);
    let output = postprocess_apply_patterns(output);
    std::fs::write(output_path, output).map_err(|e| e.to_string())?;
    Ok(())
}
