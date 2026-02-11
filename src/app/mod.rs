use crate::decompile::{collect_imports, extract_data_segments_with_offsets};
use crate::fingerprint::fingerprint_function;
use crate::helper_semantics::infer_helper_name;
use crate::semantic_resolver::resolver;
use crate::sdk::{get_backend, ContractSpecs};
use parity_wasm::{deserialize_buffer, deserialize_file};
use parity_wasm::elements::{
    ImportCountType, Section, Type,
};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod utils;

mod raw;
mod postprocess;
mod header;
mod exports;
mod specs;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(
        short = "n",
        long = "use-name-section",
        help = "Use the names in the name section for the internal function names"
    )]
    use_name_section: bool,
    #[structopt(help = "Read wasm payload from stdin instead of input file", long = "stdin-payload")]
    stdin_payload: bool,
    #[structopt(help = "Write generated output to stdout", long = "stdout")]
    stdout: bool,
    #[structopt(
        help = "Input file",
        parse(from_os_str),
        required_unless = "stdin-payload"
    )]
    input: Option<PathBuf>,
    #[structopt(
        help = "Output file, stored next to wasm file if not specified",
        parse(from_os_str)
    )]
    output: Option<PathBuf>,
}

fn postprocess_output(output: String, contract_name: &str) -> String {
    postprocess::run_all(output, contract_name)
}

fn apply_engine_output(output: String) -> String {
    if std::env::var("SOROBAN_AUDITOR_SKIP_ENGINE").is_ok() {
        output
    } else {
        crate::engine::default_engine().apply(output)
    }
}

fn brace_counts(code: &str) -> (i32, i32) {
    let mut open = 0i32;
    let mut close = 0i32;
    for ch in code.chars() {
        if ch == '{' {
            open += 1;
        } else if ch == '}' {
            close += 1;
        }
    }
    (open, close)
}

pub fn run(opt: Opt) -> Result<(), String> {
    let (input_path, output_path) = if opt.stdin_payload {
        if opt.output.is_some() {
            return Err(
                "When using --stdin-payload, provide at most one positional argument: [output]"
                    .to_string(),
            );
        }
        if opt.stdout && opt.input.is_some() {
            return Err("When using --stdout, do not provide an output path".to_string());
        }
        let output_path = if opt.stdout {
            None
        } else {
            Some(
                opt.input
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("output.rs")),
            )
        };
        (None, output_path)
    } else {
        let input_path = opt
            .input
            .clone()
            .ok_or_else(|| "Missing input file".to_string())?;
        if opt.stdout && opt.output.is_some() {
            return Err("When using --stdout, do not provide an output path".to_string());
        }
        let output_path = if opt.stdout {
            None
        } else {
            Some(opt.output.unwrap_or_else(|| input_path.with_extension("rs")))
        };
        (Some(input_path), output_path)
    };
    let sdk = get_backend();
    let modules = sdk
        .env_common_modules_result()
        .map_err(|err| format!("Error: {}", err))?;
    let module = if let Some(input) = input_path.as_ref() {
        deserialize_file(input).map_err(|err| format!("{}", err))?
    } else if opt.stdin_payload {
        let mut bytes = Vec::new();
        std::io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|e| format!("Failed reading stdin: {}", e))?;
        if bytes.is_empty() {
            return Err("stdin payload is empty".to_string());
        }
        deserialize_buffer(&bytes).map_err(|err| format!("{}", err))?
    } else {
        return Err("Missing input file or --stdin-payload".to_string());
    };
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);
    let data_segments_with_offsets = extract_data_segments_with_offsets(&module);

    let mut writer: Vec<u8> = Vec::new();
    let contract_name = if let Some(input) = input_path.as_ref() {
        utils::contract_name_from_module_or_path(&module, input)
    } else {
        utils::contract_name_from_module_or_path(&module, &PathBuf::from("stdin.wasm"))
    };
    let contract_specs = if let Some(input) = input_path.as_ref() {
        match sdk.read_contract_specs(input) {
            Ok(specs) => specs,
            Err(_err) => ContractSpecs::default(),
        }
    } else {
        ContractSpecs::default()
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
        let suggested_name = resolver()
            .suggested_name_for_fingerprint(&fingerprint)
            .map(|s| s.to_string())
            .or_else(|| infer_helper_name(body, fn_type, import_count, &functions));

        if let Some(suggested) = suggested_name {
            let mut name = suggested;
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

    header::emit_contract_scaffold(
        &mut writer,
        &import_line,
        &contract_name,
        is_account_contract,
        &contract_specs,
    )?;

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
    let merged_forwarders =
        utils::merge_forwarders(&call_forwarders, &complex_forwarders);

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
        eprintln!("Merged forwarders: {}", merged_forwarders.len());
    }

    let _has_dynamic_element_section_offset = false;

    let (globals, imported_globals_count) =
        utils::build_globals(module.import_section(), &module, exports);

    let impl_block = r#"
#[contractimpl]
impl {contract_name} {"#
        .replace("{contract_name}", &contract_name);
    writeln!(writer, "{}", impl_block).map_err(|e| e.to_string())?;
    let emit_raw_helpers = if std::env::var("SOROBAN_AUDITOR_SKIP_RAW_FUNCTIONS").is_ok() {
        false
    } else {
        true
    };

    exports::emit_public_spec_functions(
        &mut writer,
        &contract_specs,
        exports,
        &code,
        fns,
        types,
        &functions,
        import_count,
        imported_globals_count,
        &globals,
        &mut indirect_fns,
        &spec_by_fn_index,
        &data_segments_with_offsets,
        &merged_forwarders,
    )?;
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
            &merged_forwarders,
            Some(&helper_filter),
            false,
        )?;
        writeln!(writer, "}}").map_err(|e| e.to_string())?;
        writeln!(writer).map_err(|e| e.to_string())?;
    }
    let output = String::from_utf8(writer).map_err(|e| e.to_string())?;
    let output = apply_engine_output(output);
    let output = postprocess_output(output, &contract_name);
    let (open_braces, close_braces) = brace_counts(&output);
    if open_braces != close_braces {
        return Err(format!(
            "Unbalanced braces in generated output: {{={open_braces}, }}={close_braces}. Refusing to auto-fix."
        ));
    }
    if let Some(path) = output_path {
        std::fs::write(path, output).map_err(|e| e.to_string())?;
    } else {
        std::io::stdout()
            .write_all(output.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
