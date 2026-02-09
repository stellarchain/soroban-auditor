use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

mod auth;
pub mod account;
mod bytes;
mod events;
mod swap;
mod storage;
mod token;
mod vec_string_symbol;

pub mod templates;

#[allow(dead_code)]
pub struct PatternContext<'a> {
    pub export_name: &'a str,
    pub input_types: &'a [String],
    pub addr_indices: &'a [usize],
    pub i128_indices: &'a [usize],
    pub has_vec_new: bool,
    pub uses_string_new: bool,
    pub uses_symbol_new: bool,
    pub uses_bytes_new: bool,
    pub require_auth_calls: usize,
    pub require_auth_for_args_calls: usize,
    pub uses_current_contract_address: bool,
    pub symbol_literals: &'a [String],
    pub string_literals: &'a [String],
    pub data_segments: &'a [Vec<u8>],
    pub has_fail_with_error: bool,
    pub uses_get_contract_data: bool,
    pub uses_put_contract_data: bool,
    pub uses_contract_event: bool,
    pub has_datakey_type: bool,
    pub has_allowance_value_type: bool,
    pub has_allowance_key_type: bool,
    pub has_token_metadata_type: bool,
    pub has_signer_variant: bool,
    pub has_signer_cnt_variant: bool,
    pub has_admin_variant: bool,
    pub has_spend_limit_variant: bool,
    pub has_counter_variant: bool,
    pub has_owner_variant: bool,
    pub data_key_variants: &'a [crate::app::utils::DataKeyVariant],
    pub struct_defs: &'a [crate::app::utils::StructDef],
    pub uses_update_current_contract_wasm: bool,
    pub is_account_contract: bool,
}

#[derive(Default)]
pub struct PatternState {
    pub emitted_move_token: bool,
}

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
    state: &mut PatternState,
) -> bool {
    if swap::try_emit(writer, spec_fn, ctx, state) {
        return true;
    }
    if account::try_emit_increment_counter(writer, spec_fn, ctx) {
        return true;
    }
    if account::try_emit_constructor(writer, spec_fn, ctx) {
        return true;
    }
    if account::try_emit_add_limit(writer, spec_fn, ctx) {
        return true;
    }
    if account::try_emit_check_auth(writer, spec_fn, ctx) {
        return true;
    }
    if account::try_emit_upgrade(writer, spec_fn, ctx) {
        return true;
    }
    if events::try_emit(writer, spec_fn, ctx) {
        return true;
    }
    if token::try_emit_token_actions(writer, spec_fn, ctx) {
        return true;
    }
    if token::try_emit_balance_allowance(writer, spec_fn, ctx) {
        return true;
    }
    if storage::try_emit_setter(writer, spec_fn, ctx) {
        return true;
    }
    if storage::try_emit_getter(writer, spec_fn, ctx) {
        return true;
    }
    if storage::try_emit_owner_management(writer, spec_fn, ctx) {
        return true;
    }
    if storage::try_emit_version(writer, spec_fn, ctx) {
        return true;
    }
    if storage::try_emit_calculate_buy_sell(writer, spec_fn, ctx) {
        return true;
    }
    if bytes::try_emit(writer, spec_fn, ctx) {
        return true;
    }
    if token::try_emit_move_token(writer, spec_fn, ctx) {
        return true;
    }
    if vec_string_symbol::try_emit(writer, spec_fn, ctx) {
        return true;
    }
    if auth::try_emit(writer, spec_fn, ctx) {
        return true;
    }
    false
}
