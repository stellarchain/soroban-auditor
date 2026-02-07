use crate::decompile::scan_body;
use crate::patterns::{PatternContext, PatternState};
use crate::soroban::contract::find_function_specs;
use crate::wasm_ir::mangle_fn_name;
use parity_wasm::elements::{CodeSection, ExportSection, Internal};
use std::io::Write;

pub fn emit_contract_functions<W: Write>(
    writer: &mut W,
    exports: &ExportSection,
    import_count: usize,
    code: &CodeSection,
    contract_specs: &crate::soroban::contract::ContractSpecs,
    data_segments: &[Vec<u8>],
    string_literals: &[String],
    has_datakey_type: bool,
    has_allowance_value_type: bool,
    has_allowance_key_type: bool,
    pattern_state: &mut PatternState,
) -> Result<(), String> {
    let impl_block = r#"

#[contractimpl]
impl {contract_name} {"#
        .replace("{contract_name}", "Contract");
    writeln!(writer, "{}", impl_block).map_err(|e| e.to_string())?;

    for export in exports.entries() {
        if let &Internal::Function(fn_index) = export.internal() {
            let export_name = mangle_fn_name(export.field());
            let (
                has_vec_new,
                uses_string_new,
                uses_symbol_new,
                uses_bytes_new,
                _uses_map_new,
                require_auth_calls,
                uses_current_contract_address,
                symbol_literals,
                has_fail_with_error,
                uses_get_contract_data,
                uses_put_contract_data,
                uses_contract_event,
            ) = if fn_index as usize >= import_count {
                let body_index = fn_index as usize - import_count;
                if let Some(body) = code.bodies().get(body_index) {
                    let usage = scan_body(body, import_count, &[]);
                    (
                        usage.has_vec_new,
                        usage.uses_string_new,
                        usage.uses_symbol_new,
                        usage.uses_bytes_new,
                        usage.uses_map_new,
                        usage.require_auth_calls,
                        usage.uses_current_contract_address,
                        usage.symbol_literals,
                        usage.has_fail_with_error,
                        usage.uses_get_contract_data,
                        usage.uses_put_contract_data,
                        usage.uses_contract_event,
                    )
                } else {
                    (
                        false,
                        false,
                        false,
                        false,
                        false,
                        0,
                        false,
                        Vec::new(),
                        false,
                        false,
                        false,
                        false,
                    )
                }
            } else {
                (
                    false,
                    false,
                    false,
                    false,
                    false,
                    0,
                    false,
                    Vec::new(),
                    false,
                    false,
                    false,
                    false,
                )
            };
            let spec_fn = find_function_specs(contract_specs, export.field());

            if let Some(spec_fn) = spec_fn {
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
                let ctx = PatternContext {
                    export_name: &export_name,
                    input_types: &input_types,
                    addr_indices: &addr_indices,
                    i128_indices: &i128_indices,
                    has_vec_new,
                    uses_string_new,
                    uses_symbol_new,
                    uses_bytes_new,
                    require_auth_calls,
                    uses_current_contract_address,
                    symbol_literals: &symbol_literals,
                    string_literals,
                    data_segments,
                    has_fail_with_error,
                    uses_get_contract_data,
                    uses_put_contract_data,
                    uses_contract_event,
                    has_datakey_type,
                    has_allowance_value_type,
                    has_allowance_key_type,
                };
                if crate::patterns::try_emit(writer, &spec_fn, &ctx, pattern_state) {
                    continue;
                }

                write!(writer, "    pub fn {}(&mut self, env: Env", export_name)
                    .map_err(|e| e.to_string())?;
                for argument in spec_fn.inputs() {
                    write!(writer, ", {}: {}", argument.name(), argument.type_ident())
                        .map_err(|e| e.to_string())?;
                }
                write!(writer, ")").map_err(|e| e.to_string())?;
                if let Some(return_type) = spec_fn.output() {
                    write!(writer, " -> {}", return_type.type_ident()).map_err(|e| e.to_string())?;
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
    }

    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    Ok(())
}
