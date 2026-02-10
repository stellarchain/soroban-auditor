use crate::wasm_ir::Function;
use parity_wasm::elements::{FuncBody, FunctionType, Instruction};
use std::collections::BTreeSet;

pub fn infer_helper_name(
    body: &FuncBody,
    fn_type: &FunctionType,
    import_count: usize,
    functions: &[Function],
) -> Option<String> {
    let mut imports: BTreeSet<&str> = BTreeSet::new();
    let mut internal_calls = 0usize;
    let mut has_store = false;
    let mut has_load = false;

    for instr in body.code().elements() {
        match instr {
            Instruction::Call(n) => {
                if (*n as usize) < import_count {
                    if let Some(f) = functions.get(*n as usize) {
                        imports.insert(f.name.as_str());
                    }
                } else {
                    internal_calls += 1;
                }
            }
            Instruction::I32Load(_, _)
            | Instruction::I64Load(_, _)
            | Instruction::I32Load8S(_, _)
            | Instruction::I32Load8U(_, _)
            | Instruction::I32Load16S(_, _)
            | Instruction::I32Load16U(_, _)
            | Instruction::I64Load8S(_, _)
            | Instruction::I64Load8U(_, _)
            | Instruction::I64Load16S(_, _)
            | Instruction::I64Load16U(_, _)
            | Instruction::I64Load32S(_, _)
            | Instruction::I64Load32U(_, _) => {
                has_load = true;
            }
            Instruction::I32Store(_, _)
            | Instruction::I64Store(_, _)
            | Instruction::I32Store8(_, _)
            | Instruction::I32Store16(_, _)
            | Instruction::I64Store8(_, _)
            | Instruction::I64Store16(_, _)
            | Instruction::I64Store32(_, _) => {
                has_store = true;
            }
            _ => {}
        }
    }

    // Storage helpers
    if imports.contains("del_contract_data") {
        return Some("storage_remove_val".to_string());
    }
    if imports.contains("put_contract_data") {
        return Some("storage_set_val".to_string());
    }
    if imports.contains("get_contract_data") {
        return Some("storage_get_val".to_string());
    }

    // Auth helpers
    if imports.contains("require_auth_for_args") {
        return Some("require_auth_for_key".to_string());
    }
    if imports.contains("require_auth") && !imports.contains("require_auth_for_args") {
        return Some("require_owner_auth".to_string());
    }

    // Map/Vec constructor wrappers (common in helper thunks)
    if imports.contains("map_new")
        && fn_type.results().len() == 1
        && internal_calls == 0
    {
        return Some("map_new_val".to_string());
    }
    if imports.contains("vec_new")
        && imports.contains("vec_push_back")
        && fn_type.results().len() == 1
    {
        return Some("vec_pair_builder".to_string());
    }

    // Frequent low-level copy helper shape
    if has_load && has_store && imports.is_empty() && internal_calls == 0 {
        return Some("memcpy_like".to_string());
    }

    None
}
