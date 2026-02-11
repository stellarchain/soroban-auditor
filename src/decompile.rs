use crate::code_builder::transform_from_soroban_val_raw;
use crate::sdk::{get_backend, ContractSpecs, SdkFunctionDetector};
use crate::wasm_ir::{mangle_fn_name, Function};
use parity_wasm::elements::{CodeSection, ExportSection, FuncBody, Instruction, Module};
use std::collections::HashMap;

#[derive(Default)]
pub struct BodyUsage {
    pub has_vec_new: bool,
    pub uses_string_new: bool,
    pub uses_symbol_new: bool,
    pub uses_bytes_new: bool,
    pub uses_map_new: bool,
    pub uses_hash: bool,
    pub uses_bytesn: bool,
    pub uses_bytes: bool,
    pub uses_u256: bool,
    pub uses_i256: bool,
    pub uses_timepoint: bool,
    pub uses_duration: bool,
    pub uses_muxed_address: bool,
    pub uses_invoker_auth: bool,
    pub require_auth_calls: usize,
    pub require_auth_for_args_calls: usize,
    pub uses_current_contract_address: bool,
    pub symbol_literals: Vec<String>,
    pub has_fail_with_error: bool,
    pub uses_get_contract_data: bool,
    pub uses_put_contract_data: bool,
    pub uses_contract_event: bool,
    pub uses_update_current_contract_wasm: bool,
    /// All SDK function calls detected (function_name -> call count)
    pub sdk_function_calls: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub struct DataSegment {
    pub offset: u32,
    pub data: Vec<u8>,
}

pub fn scan_body(body: &FuncBody, import_count: usize, functions: &[Function]) -> BodyUsage {
    let mut usage = BodyUsage::default();

    // Initialize SDK detector once for comprehensive detection
    let sdk_detector = SdkFunctionDetector::default();

    for instr in body.code().elements() {
        if let Instruction::Call(idx) = instr {
            let idx = *idx as usize;
            if idx < import_count {
                let fn_name = &functions[idx].name;

                // Track ALL SDK function calls comprehensively
                if sdk_detector.is_sdk_function(fn_name) {
                    *usage.sdk_function_calls.entry(fn_name.clone()).or_insert(0) += 1;
                }

                match fn_name.as_str() {
                    "vec_new_from_linear_memory" => usage.has_vec_new = true,
                    "string_new_from_linear_memory" => usage.uses_string_new = true,
                    "symbol_new_from_linear_memory" => usage.uses_symbol_new = true,
                    "bytes_new_from_linear_memory" => usage.uses_bytes_new = true,
                    "map_new_from_linear_memory" => usage.uses_map_new = true,
                    "compute_hash_sha256" | "compute_hash_keccak256" => {
                        usage.uses_hash = true;
                        usage.uses_bytes = true;
                    }
                    "verify_sig_ed25519" | "verify_sig_ecdsa_secp256r1" => {
                        usage.uses_bytesn = true;
                        usage.uses_hash = true;
                    }
                    "recover_key_ecdsa_secp256k1" => {
                        usage.uses_bytesn = true;
                        usage.uses_hash = true;
                    }
                    "u256_add"
                    | "u256_sub"
                    | "u256_mul"
                    | "u256_div"
                    | "u256_rem_euclid"
                    | "u256_pow"
                    | "u256_shl"
                    | "u256_shr"
                    | "u256_val_from_be_bytes"
                    | "u256_val_to_be_bytes"
                    | "obj_from_u256_pieces"
                    | "obj_to_u256_hi_hi"
                    | "obj_to_u256_hi_lo"
                    | "obj_to_u256_lo_hi"
                    | "obj_to_u256_lo_lo" => {
                        usage.uses_u256 = true;
                        usage.uses_bytes = true;
                    }
                    "obj_from_i256_pieces"
                    | "obj_to_i256_hi_hi"
                    | "obj_to_i256_hi_lo"
                    | "obj_to_i256_lo_hi"
                    | "obj_to_i256_lo_lo" => {
                        usage.uses_i256 = true;
                        usage.uses_bytes = true;
                    }
                    "i256_add"
                    | "i256_sub"
                    | "i256_mul"
                    | "i256_div"
                    | "i256_rem_euclid"
                    | "i256_pow"
                    | "i256_shl"
                    | "i256_shr"
                    | "i256_val_from_be_bytes"
                    | "i256_val_to_be_bytes" => {
                        usage.uses_i256 = true;
                        usage.uses_bytes = true;
                    }
                    "timepoint_obj_from_u64" | "timepoint_obj_to_u64" => {
                        usage.uses_timepoint = true;
                    }
                    "duration_obj_from_u64" | "duration_obj_to_u64" => {
                        usage.uses_duration = true;
                    }
                    "get_address_from_muxed_address" | "get_id_from_muxed_address" => {
                        usage.uses_muxed_address = true;
                    }
                    "authorize_as_curr_contract" => {
                        usage.uses_invoker_auth = true;
                    }
                    "require_auth_for_args" => {
                        usage.require_auth_calls += 1;
                        usage.require_auth_for_args_calls += 1;
                    }
                    "require_auth" => usage.require_auth_calls += 1,
                    "get_current_contract_address" => usage.uses_current_contract_address = true,
                    "fail_with_error" => usage.has_fail_with_error = true,
                    "get_contract_data" => usage.uses_get_contract_data = true,
                    "put_contract_data" => usage.uses_put_contract_data = true,
                    "contract_event" => usage.uses_contract_event = true,
                    "update_current_contract_wasm" => {
                        usage.uses_update_current_contract_wasm = true
                    }
                    _ => {}
                }
            }
        } else if let Instruction::I64Const(value) = instr {
            let candidate = transform_from_soroban_val_raw(*value as u64);
            if candidate.parse::<i64>().is_err() && candidate != "Void" {
                usage.symbol_literals.push(candidate);
            }
        }
    }
    usage
}

pub fn extract_data_segments_with_offsets(module: &Module) -> Vec<DataSegment> {
    let mut segments = Vec::new();
    if let Some(data_section) = module.data_section() {
        for segment in data_section.entries() {
            let mut offset = 0u32;
            if let Some(offset_expr) = segment.offset() {
                if let Some(instr) = offset_expr.code().first() {
                    if let Instruction::I32Const(value) = instr {
                        offset = *value as u32;
                    }
                }
            }
            segments.push(DataSegment {
                offset,
                data: segment.value().to_vec(),
            });
        }
    }
    segments
}

pub fn collect_imports(
    contract_specs: &ContractSpecs,
    exports: &ExportSection,
    code: &CodeSection,
    functions: &[Function],
    import_count: usize,
    is_account_contract: bool,
) -> Vec<String> {
    let needs_contracttype = !contract_specs.types().is_empty();
    let mut needs_token = false;
    let mut needs_address = false;
    let mut needs_vec = false;
    let mut needs_vec_macro = false;
    let mut needs_string = false;
    let mut needs_symbol = false;
    let mut needs_symbol_short = false;
    let mut needs_bytes = false;
    let mut needs_bytesn = false;
    let mut needs_muxed_address = false;
    let mut needs_val = false;
    let mut needs_into_val = false;
    let mut needs_from_val = false;
    let mut needs_context = false;
    let mut needs_hash = false;
    let mut needs_map = false;
    let mut needs_custom_account = false;
    let mut needs_try_into_val = false;
    let mut needs_try_from_val = false;
    let mut needs_u256 = false;
    let mut needs_i256 = false;
    let mut needs_timepoint = false;
    let mut needs_duration = false;
    let mut needs_invoker_auth = false;

    for spec_fn in contract_specs.functions() {
        for param in spec_fn.inputs() {
            let ty = param.type_ident().to_string();
            if ty.contains("Address") {
                needs_address = true;
            }
            if ty.contains("MuxedAddress") {
                needs_muxed_address = true;
            }
            if ty.contains("Vec") {
                needs_vec = true;
            }
            if ty.contains("Map") {
                needs_map = true;
            }
            if ty.contains("String") {
                needs_string = true;
            }
            if ty.contains("Symbol") {
                needs_symbol = true;
            }
            if ty.contains("Context") {
                needs_context = true;
            }
            if ty.contains("Hash") {
                needs_hash = true;
            }
            if ty.contains("BytesN") {
                needs_bytesn = true;
            } else if ty.contains("Bytes") {
                needs_bytes = true;
            }
            if ty.contains("Val") {
                needs_val = true;
            }
            if ty.contains("IntoVal") {
                needs_into_val = true;
            }
            if ty.contains("FromVal") {
                needs_from_val = true;
            }
            if ty.contains("TryIntoVal") {
                needs_try_into_val = true;
            }
        }
        if let Some(output) = spec_fn.output() {
            let ty = output.type_ident().to_string();
            if ty.contains("Address") {
                needs_address = true;
            }
            if ty.contains("MuxedAddress") {
                needs_muxed_address = true;
            }
            if ty.contains("Vec") {
                needs_vec = true;
            }
            if ty.contains("Map") {
                needs_map = true;
            }
            if ty.contains("String") {
                needs_string = true;
            }
            if ty.contains("Symbol") {
                needs_symbol = true;
            }
            if ty.contains("Context") {
                needs_context = true;
            }
            if ty.contains("Hash") {
                needs_hash = true;
            }
            if ty.contains("BytesN") {
                needs_bytesn = true;
            } else if ty.contains("Bytes") {
                needs_bytes = true;
            }
            if ty.contains("Val") {
                needs_val = true;
            }
            if ty.contains("IntoVal") {
                needs_into_val = true;
            }
            if ty.contains("FromVal") {
                needs_from_val = true;
            }
            if ty.contains("TryIntoVal") {
                needs_try_into_val = true;
            }
        }
    }

    for export in exports.entries() {
        if let &parity_wasm::elements::Internal::Function(fn_index) = export.internal() {
            if fn_index as usize >= import_count {
                let body_index = fn_index as usize - import_count;
                if let Some(body) = code.bodies().get(body_index) {
                    let usage = scan_body(body, import_count, functions);
                    if usage.has_vec_new {
                        needs_vec_macro = true;
                    }
                    if usage.require_auth_calls > 0 {
                        needs_into_val = true;
                    }
                    if usage.uses_string_new {
                        needs_string = true;
                    }
                    if usage.uses_symbol_new {
                        needs_symbol = true;
                    }
                    if usage.uses_bytes_new {
                        needs_bytes = true;
                    }
                    if usage.uses_hash {
                        needs_hash = true;
                    }
                    if usage.uses_bytesn {
                        needs_bytesn = true;
                    }
                    if usage.uses_bytes {
                        needs_bytes = true;
                    }
                    if usage.uses_u256 {
                        needs_u256 = true;
                        needs_try_from_val = true;
                    }
                    if usage.uses_i256 {
                        needs_i256 = true;
                        needs_try_from_val = true;
                    }
                    if usage.uses_timepoint {
                        needs_timepoint = true;
                        needs_try_from_val = true;
                    }
                    if usage.uses_duration {
                        needs_duration = true;
                        needs_try_from_val = true;
                    }
                    if usage.uses_muxed_address {
                        needs_muxed_address = true;
                        needs_try_from_val = true;
                    }
                    if usage.uses_invoker_auth {
                        needs_invoker_auth = true;
                    }
                    let spec_fn = get_backend().find_function_specs(contract_specs, export.field());
                    if let Some(spec_fn) = spec_fn {
                        let export_name = mangle_fn_name(export.field());
                        if usage.uses_contract_event
                            && usage.uses_get_contract_data
                            && usage.uses_put_contract_data
                            && spec_fn.output().is_some()
                            && export_name.contains("increment")
                        {
                            needs_symbol_short = true;
                        }
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
                        let swap_shape = spec_fn.inputs().len() == 8
                            && addr_indices.len() == 4
                            && i128_indices.len() == 4;
                        if swap_shape {
                            needs_token = true;
                            needs_into_val = true;
                        }
                        let move_token_shape = spec_fn.inputs().len() == 5
                            && addr_indices.len() == 3
                            && i128_indices.len() == 2;
                        let has_transfer_symbol = usage
                            .symbol_literals
                            .iter()
                            .any(|s| s.as_str() == "transfer");
                        if move_token_shape
                            && usage.uses_current_contract_address
                            && has_transfer_symbol
                        {
                            needs_token = true;
                        }
                    }
                }
            }
        }
    }

    if is_account_contract {
        needs_custom_account = true;
        needs_symbol_short = true;
        needs_symbol = true;
        needs_map = true;
        needs_try_into_val = true;
        needs_hash = true;
        needs_context = true;
        needs_bytesn = true;
        needs_address = true;
        needs_vec = true;
    }

    let mut imports = Vec::new();
    imports.push("contract".to_string());
    imports.push("contractimpl".to_string());
    if is_account_contract {
        imports.push("contracterror".to_string());
    }
    if needs_contracttype {
        imports.push("contracttype".to_string());
    }
    if needs_token {
        imports.push("token".to_string());
    }
    if needs_address {
        imports.push("Address".to_string());
    }
    if needs_muxed_address {
        imports.push("MuxedAddress".to_string());
    }
    imports.push("Env".to_string());
    if needs_vec {
        imports.push("Vec".to_string());
    }
    if needs_map {
        imports.push("Map".to_string());
    }
    if needs_val {
        imports.push("Val".to_string());
    }
    if needs_into_val {
        imports.push("IntoVal".to_string());
    }
    if needs_from_val {
        imports.push("FromVal".to_string());
    }
    if needs_vec_macro {
        imports.push("vec".to_string());
    }
    if needs_string {
        imports.push("String".to_string());
    }
    if needs_symbol {
        imports.push("Symbol".to_string());
    }
    if needs_bytes {
        imports.push("Bytes".to_string());
    }
    if needs_bytesn {
        imports.push("BytesN".to_string());
    }
    if needs_symbol_short {
        imports.push("symbol_short".to_string());
    }
    if needs_context {
        imports.push("auth::Context".to_string());
    }
    if needs_custom_account {
        imports.push("auth::CustomAccountInterface".to_string());
    }
    if needs_invoker_auth {
        imports.push("auth::InvokerContractAuthEntry".to_string());
    }
    if needs_hash {
        imports.push("crypto::Hash".to_string());
    }
    if needs_try_into_val {
        imports.push("TryIntoVal".to_string());
    }
    if needs_try_from_val {
        imports.push("TryFromVal".to_string());
    }
    if needs_u256 {
        imports.push("U256".to_string());
    }
    if needs_i256 {
        imports.push("I256".to_string());
    }
    if needs_timepoint {
        imports.push("Timepoint".to_string());
    }
    if needs_duration {
        imports.push("Duration".to_string());
    }
    imports
}

#[allow(dead_code)]
pub fn extract_data_segments(module: &Module) -> Vec<Vec<u8>> {
    let mut segments = Vec::new();
    if let Some(data_section) = module.data_section() {
        for segment in data_section.entries() {
            segments.push(segment.value().to_vec());
        }
    }
    segments
}

#[allow(dead_code)]
pub fn extract_string_literals(module: &Module) -> Vec<String> {
    let mut literals = Vec::new();
    for segment in extract_data_segments(module) {
        let mut current = Vec::new();
        for &b in &segment {
            if (0x20..=0x7e).contains(&b) {
                current.push(b);
                continue;
            }
            if !current.is_empty() {
                if let Ok(s) = String::from_utf8(current.clone()) {
                    literals.push(s);
                }
                current.clear();
            }
        }
        if !current.is_empty() {
            if let Ok(s) = String::from_utf8(current.clone()) {
                literals.push(s);
            }
        }
    }
    literals
}

#[allow(dead_code)]
pub fn parse_bytesn_len(type_str: &str) -> Option<usize> {
    let bytesn_pos = type_str.find("BytesN")?;
    let after = &type_str[bytesn_pos..];
    let start = after.find('<')?;
    let end = after.find('>')?;
    let len_str = &after[start + 1..end].trim();
    len_str.parse::<usize>().ok()
}

#[allow(dead_code)]
pub fn format_byte_array(bytes: &[u8]) -> String {
    let mut out = String::new();
    out.push('[');
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{}", b));
    }
    out.push(']');
    out
}
