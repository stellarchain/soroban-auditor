use crate::decompile::{collect_imports, extract_data_segments, extract_string_literals};
use crate::patterns::PatternState;
use crate::format::{format_spec_tokens, format_type_ident};
use parity_wasm::deserialize_file;
use parity_wasm::elements::{ImportCountType, Section};
use crate::soroban::common::env_common_modules_result;
use crate::soroban::contract::{read_contract_specs, ContractSpecs};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

 
mod utils;
 
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

pub fn run(opt: Opt) -> Result<(), String> {
    let input = opt.input;
    let output = opt.output.unwrap_or_else(|| input.with_extension("rs"));
    let modules = env_common_modules_result().map_err(|err| format!("Error: {}", err))?;
    let module = deserialize_file(&input).map_err(|err| format!("{}", err))?;
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);
    let data_segments = extract_data_segments(&module);
    let string_literals = extract_string_literals(&module);

    let mut writer = BufWriter::new(File::create(output).map_err(|e| e.to_string())?);
    let contract_name = utils::contract_name_from_module(&module);
    let contract_specs = match read_contract_specs(&input) {
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

    let functions = utils::build_functions(
        &modules,
        module.import_section(),
        types,
        fns,
        function_names,
        opt.use_name_section,
    );

    let import_items = collect_imports(&contract_specs, &exports, &code, &functions, import_count);
    let import_line = format!("use soroban_sdk::{{{}}};", import_items.join(", "));

    writeln!(
        writer,
        "#![no_std]\n{}\n\n#[contract]\npub struct {};",
        import_line,
        contract_name
    )
    .map_err(|e| e.to_string())?;

    if !contract_specs.types().is_empty() {
        writeln!(writer).map_err(|e| e.to_string())?;
        for ty in contract_specs.types() {
            let formatted = format_spec_tokens(&ty.to_string());
            writeln!(writer, "{}", formatted).map_err(|e| e.to_string())?;
        }
    }

    let mut indirect_fns = BTreeMap::new();
    let mut pattern_state = PatternState::default();
    let spec_by_fn_index = utils::build_spec_index(&contract_specs, exports);

    let _has_dynamic_element_section_offset = false;

    let (globals, imported_globals_count) =
        utils::build_globals(module.import_section(), &module, exports);

 

    let impl_block = r#"
#[contractimpl]
impl {contract_name} {"#
        .replace("{contract_name}", &contract_name);
    writeln!(writer, "{}", impl_block).map_err(|e| e.to_string())?;

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
                &input_types,
                &addr_indices,
                &i128_indices,
            );

            if crate::patterns::try_emit(&mut writer, &spec_fn, &ctx, &mut pattern_state) {
                continue;
            }

            write!(writer, "    pub fn {}(&mut self, env: Env", ctx_data.export_name)
                .map_err(|e| e.to_string())?;
            for argument in spec_fn.inputs() {
                let ty = format_type_ident(&argument.type_ident().to_string());
                write!(writer, ", {}: {}", argument.name(), ty)
                    .map_err(|e| e.to_string())?;
            }
            write!(writer, ")").map_err(|e| e.to_string())?;
            if let Some(return_type) = spec_fn.output() {
                let ty = format_type_ident(&return_type.type_ident().to_string());
                write!(writer, " -> {}", ty).map_err(|e| e.to_string())?;
            }
            writeln!(writer, " {{").map_err(|e| e.to_string())?;
            if ctx.has_fail_with_error {
                writeln!(writer, "        panic!(\"contract error\");").map_err(|e| e.to_string())?;
            } else {
                writeln!(writer, "        panic!(\"decompilation pending\");")
                    .map_err(|e| e.to_string())?;
            }
            writeln!(writer, "    }}").map_err(|e| e.to_string())?;
        }
    }

    let emit_raw_functions = false;
    if emit_raw_functions {
        raw::emit_raw_functions(
            &mut writer,
            &code,
            fns,
            types,
            import_count,
            imported_globals_count,
            &functions,
            &globals,
            &mut indirect_fns,
            &spec_by_fn_index,
        )?;
    }

    // verbose removed

    writeln!(writer, "}}").map_err(|e| e.to_string())?;

    Ok(())
}
