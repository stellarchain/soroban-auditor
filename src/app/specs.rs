use crate::sdk::get_backend;
use crate::wasm_ir::mangle_fn_name;
use parity_wasm::elements::CodeSection;

pub struct PatternContextData {
    pub export_name: String,
}

pub fn build_pattern_context_data(
    export: &parity_wasm::elements::ExportEntry,
    _import_count: usize,
    _code: &CodeSection,
    _functions: &[crate::wasm_ir::Function],
) -> PatternContextData {
    PatternContextData {
        export_name: normalize_special_export_name(mangle_fn_name(export.field())),
    }
}

fn normalize_special_export_name(name: String) -> String {
    match name.as_str() {
        "___constructor" => "__constructor".to_string(),
        "___check_auth" => "__check_auth".to_string(),
        _ => name,
    }
}

pub fn find_spec_for_export(
    contract_specs: &crate::sdk::soroban::contract::ContractSpecs,
    export: &parity_wasm::elements::ExportEntry,
) -> Option<crate::sdk::soroban::contract::FunctionContractSpec> {
    get_backend().find_function_specs(contract_specs, export.field())
}
