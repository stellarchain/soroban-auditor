use crate::patterns::PatternContext;
use crate::sdk::get_backend;
use crate::wasm_ir::mangle_fn_name;
use parity_wasm::elements::{CodeSection, Internal};

pub struct PatternContextData {
    pub export_name: String,
    pub has_vec_new: bool,
    pub uses_string_new: bool,
    pub uses_symbol_new: bool,
    pub uses_bytes_new: bool,
    pub require_auth_calls: usize,
    pub require_auth_for_args_calls: usize,
    pub uses_current_contract_address: bool,
    pub symbol_literals: Vec<String>,
    pub has_fail_with_error: bool,
    pub uses_get_contract_data: bool,
    pub uses_put_contract_data: bool,
    pub uses_contract_event: bool,
    pub uses_update_current_contract_wasm: bool,
}

pub fn build_pattern_context_data(
    export: &parity_wasm::elements::ExportEntry,
    import_count: usize,
    code: &CodeSection,
    functions: &[crate::wasm_ir::Function],
) -> PatternContextData {
    let export_name = mangle_fn_name(export.field());
    let (
        has_vec_new,
        uses_string_new,
        uses_symbol_new,
        uses_bytes_new,
        _uses_map_new,
        require_auth_calls,
        require_auth_for_args_calls,
        uses_current_contract_address,
        symbol_literals,
        has_fail_with_error,
        uses_get_contract_data,
        uses_put_contract_data,
        uses_contract_event,
        uses_update_current_contract_wasm,
    ) = if let &Internal::Function(fn_index) = export.internal() {
        if fn_index as usize >= import_count {
            let body_index = fn_index as usize - import_count;
            if let Some(body) = code.bodies().get(body_index) {
                let usage = crate::decompile::scan_body(body, import_count, functions);
                (
                    usage.has_vec_new,
                    usage.uses_string_new,
                    usage.uses_symbol_new,
                    usage.uses_bytes_new,
                    usage.uses_map_new,
                    usage.require_auth_calls,
                    usage.require_auth_for_args_calls,
                    usage.uses_current_contract_address,
                    usage.symbol_literals,
                    usage.has_fail_with_error,
                    usage.uses_get_contract_data,
                    usage.uses_put_contract_data,
                    usage.uses_contract_event,
                    usage.uses_update_current_contract_wasm,
                )
            } else {
                (
                    false,
                    false,
                    false,
                    false,
                    false,
                    0,
                    0,
                    false,
                    Vec::new(),
                    false,
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
                0,
                false,
                Vec::new(),
                false,
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
            0,
            false,
            Vec::new(),
            false,
            false,
            false,
            false,
            false,
        )
    };

    PatternContextData {
        export_name,
        has_vec_new,
        uses_string_new,
        uses_symbol_new,
        uses_bytes_new,
        require_auth_calls,
        require_auth_for_args_calls,
        uses_current_contract_address,
        symbol_literals,
        has_fail_with_error,
        uses_get_contract_data,
        uses_put_contract_data,
        uses_contract_event,
        uses_update_current_contract_wasm,
    }
}

pub fn build_pattern_context<'a>(
    ctx_data: &'a PatternContextData,
    _spec_fn: &'a crate::soroban::contract::FunctionContractSpec,
    string_literals: &'a [String],
    data_segments: &'a [Vec<u8>],
    has_datakey_type: bool,
    has_allowance_value_type: bool,
    has_allowance_key_type: bool,
    has_token_metadata_type: bool,
    has_signer_variant: bool,
    has_signer_cnt_variant: bool,
    has_admin_variant: bool,
    has_spend_limit_variant: bool,
    has_counter_variant: bool,
    has_owner_variant: bool,
    data_key_variants: &'a [crate::app::utils::DataKeyVariant],
    struct_defs: &'a [crate::app::utils::StructDef],
    input_types: &'a [String],
    addr_indices: &'a [usize],
    i128_indices: &'a [usize],
    is_account_contract: bool,
) -> PatternContext<'a> {
    PatternContext {
        export_name: &ctx_data.export_name,
        input_types,
        addr_indices,
        i128_indices,
        has_vec_new: ctx_data.has_vec_new,
        uses_string_new: ctx_data.uses_string_new,
        uses_symbol_new: ctx_data.uses_symbol_new,
        uses_bytes_new: ctx_data.uses_bytes_new,
        require_auth_calls: ctx_data.require_auth_calls,
        require_auth_for_args_calls: ctx_data.require_auth_for_args_calls,
        uses_current_contract_address: ctx_data.uses_current_contract_address,
        symbol_literals: &ctx_data.symbol_literals,
        string_literals,
        data_segments,
        has_fail_with_error: ctx_data.has_fail_with_error,
        uses_get_contract_data: ctx_data.uses_get_contract_data,
        uses_put_contract_data: ctx_data.uses_put_contract_data,
        uses_contract_event: ctx_data.uses_contract_event,
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
        data_key_variants,
        struct_defs,
        uses_update_current_contract_wasm: ctx_data.uses_update_current_contract_wasm,
        is_account_contract,
    }
}

pub fn find_spec_for_export(
    contract_specs: &crate::soroban::contract::ContractSpecs,
    export: &parity_wasm::elements::ExportEntry,
) -> Option<crate::soroban::contract::FunctionContractSpec> {
    get_backend().find_function_specs(contract_specs, export.field())
}
