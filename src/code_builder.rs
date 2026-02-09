use crate::reorder_analysis::can_local_be_reordered;
use crate::wasm_ir::{call_indirect_name, mangle_fn_name, to_rs_type, BlockKind, Function, Global, Indentation};
use crate::precedence;
use crate::{expr_builder::ExprBuilder, soroban::contract::FunctionContractSpec};
use parity_wasm::elements::{BlockType, Instruction, Type, TypeSection};
use regex::Regex;
use soroban_sdk::Val;
use soroban_sdk::IntoVal;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

fn storage_match_expr(storage_type: &str, persistent_expr: &str, temporary_expr: &str, instance_expr: &str) -> String {
    format!(
        "match {} {{ 0 => {{ {} }}, 1 => {{ {} }}, _ => {{ {} }} }}",
        storage_type, persistent_expr, temporary_expr, instance_expr
    )
}

fn parse_u32_literal(s: &str) -> Option<u32> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        s.parse::<u32>().ok()
    } else {
        None
    }
}

fn resolve_linear_memory_bytes(
    addr: u32,
    len: u32,
    data_segments: &[crate::decompile::DataSegment],
) -> Option<Vec<u8>> {
    for seg in data_segments {
        let start = seg.offset;
        let end = start.saturating_add(seg.data.len() as u32);
        if addr >= start && addr.saturating_add(len) <= end {
            let offset = (addr - start) as usize;
            let end_off = offset + len as usize;
            return Some(seg.data[offset..end_off].to_vec());
        }
    }
    None
}

fn resolve_linear_memory_u64_array(
    addr: u32,
    len: u32,
    data_segments: &[crate::decompile::DataSegment],
) -> Option<Vec<u64>> {
    let byte_len = len.checked_mul(8)?;
    let bytes = resolve_linear_memory_bytes(addr, byte_len, data_segments)?;
    let mut out = Vec::with_capacity(len as usize);
    for chunk in bytes.chunks_exact(8) {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(chunk);
        out.push(u64::from_le_bytes(buf));
    }
    Some(out)
}

fn resolve_linear_memory_string(
    addr: u32,
    len: u32,
    data_segments: &[crate::decompile::DataSegment],
) -> Option<String> {
    let bytes = resolve_linear_memory_bytes(addr, len, data_segments)?;
    let s = String::from_utf8(bytes).ok()?;
    Some(s.replace('\n', "\\n").replace('\r', "\\r").replace('\"', "\\\""))
}

fn format_bytes_literal(bytes: &[u8]) -> String {
    let mut out = String::from("[");
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&b.to_string());
    }
    out.push(']');
    out
}

fn format_val_vec_literal(vals: &[u64]) -> String {
    let mut out = String::from("{ let mut v = Vec::<Val>::new(env);");
    for v in vals {
        out.push_str(&format!(" v.push_back(Val::from_u64({}));", *v));
    }
    out.push_str(" val_to_i64(v.into_val(env)) }");
    out
}

fn map_imported_call(
    name: &str,
    args: &[String],
    data_segments: &[crate::decompile::DataSegment],
) -> Option<String> {
    match name {
        "vec_new" => Some("val_to_i64(Vec::<Val>::new(env).into_val(env))".to_string()),
        "vec_len" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "Vec::<Val>::from_val(env, &val_from_i64({})).len() as i64",
                vec_val
            ))
        }
        "vec_get" => {
            let vec_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "val_to_i64(Vec::<Val>::from_val(env, &val_from_i64({})).get_unchecked({} as u32))",
                vec_val, idx
            ))
        }
        "vec_put" => {
            let vec_val = args.get(0)?;
            let idx = args.get(1)?;
            let item = args.get(2)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.set({} as u32, val_from_i64({})); val_to_i64(v.into_val(env)) }}",
                vec_val, idx, item
            ))
        }
        "vec_insert" => {
            let vec_val = args.get(0)?;
            let idx = args.get(1)?;
            let item = args.get(2)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.insert({} as u32, val_from_i64({})); val_to_i64(v.into_val(env)) }}",
                vec_val, idx, item
            ))
        }
        "vec_del" => {
            let vec_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.remove_unchecked({} as u32); val_to_i64(v.into_val(env)) }}",
                vec_val, idx
            ))
        }
        "vec_front" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "val_to_i64(Vec::<Val>::from_val(env, &val_from_i64({})).first_unchecked())",
                vec_val
            ))
        }
        "vec_back" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "val_to_i64(Vec::<Val>::from_val(env, &val_from_i64({})).last_unchecked())",
                vec_val
            ))
        }
        "vec_push_front" => {
            let vec_val = args.get(0)?;
            let item = args.get(1)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.push_front(val_from_i64({})); val_to_i64(v.into_val(env)) }}",
                vec_val, item
            ))
        }
        "vec_pop_front" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); let _ = v.pop_front_unchecked(); val_to_i64(v.into_val(env)) }}",
                vec_val
            ))
        }
        "vec_push_back" => {
            let vec_val = args.get(0)?;
            let item = args.get(1)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.push_back(val_from_i64({})); val_to_i64(v.into_val(env)) }}",
                vec_val, item
            ))
        }
        "vec_pop_back" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); let _ = v.pop_back_unchecked(); val_to_i64(v.into_val(env)) }}",
                vec_val
            ))
        }
        "vec_append" => {
            let vec_val = args.get(0)?;
            let other = args.get(1)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); v.append(&Vec::<Val>::from_val(env, &val_from_i64({}))); val_to_i64(v.into_val(env)) }}",
                vec_val, other
            ))
        }
        "vec_slice" => {
            let vec_val = args.get(0)?;
            let start = args.get(1)?;
            let end = args.get(2)?;
            Some(format!(
                "val_to_i64(Vec::<Val>::from_val(env, &val_from_i64({})).slice({} as u32..{} as u32).into_val(env))",
                vec_val, start, end
            ))
        }
        "vec_first_index_of" => {
            let vec_val = args.get(0)?;
            let item = args.get(1)?;
            Some(format!(
                "{{ let v = Vec::<Val>::from_val(env, &val_from_i64({})); match v.first_index_of(val_from_i64({})) {{ Some(i) => i as i64, None => 0 /* Void */ }} }}",
                vec_val, item
            ))
        }
        "vec_last_index_of" => {
            let vec_val = args.get(0)?;
            let item = args.get(1)?;
            Some(format!(
                "{{ let v = Vec::<Val>::from_val(env, &val_from_i64({})); match v.last_index_of(val_from_i64({})) {{ Some(i) => i as i64, None => 0 /* Void */ }} }}",
                vec_val, item
            ))
        }
        "vec_binary_search" => {
            let vec_val = args.get(0)?;
            let item = args.get(1)?;
            Some(format!(
                "{{ let v = Vec::<Val>::from_val(env, &val_from_i64({})); match v.binary_search(val_from_i64({})) {{ Ok(i) => (((1u64 << 32) | (i as u64)) as i64), Err(i) => (i as i64) }} }}",
                vec_val, item
            ))
        }
        "vec_new_from_linear_memory" => {
            let addr = args.get(0)?;
            let len = args.get(1)?;
            if let (Some(addr), Some(len)) = (parse_u32_literal(addr), parse_u32_literal(len)) {
                if let Some(vals) = resolve_linear_memory_u64_array(addr, len, data_segments) {
                    return Some(format_val_vec_literal(&vals));
                }
            }
            Some("val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */"
                .to_string())
        }
        "vec_unpack_to_linear_memory" => {
            Some("0 /* TODO: vec_unpack_to_linear_memory */".to_string())
        }
        "map_new" => Some("val_to_i64(Map::<Val, Val>::new(env).into_val(env))".to_string()),
        "map_len" => {
            let map_val = args.get(0)?;
            Some(format!(
                "Map::<Val, Val>::from_val(env, &val_from_i64({})).len() as i64",
                map_val
            ))
        }
        "map_get" => {
            let map_val = args.get(0)?;
            let key = args.get(1)?;
            Some(format!(
                "val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64({})).get(val_from_i64({})).unwrap_or(val_from_i64(0)))",
                map_val, key
            ))
        }
        "map_put" => {
            let map_val = args.get(0)?;
            let key = args.get(1)?;
            let val = args.get(2)?;
            Some(format!(
                "{{ let mut m = Map::<Val, Val>::from_val(env, &val_from_i64({})); m.set(val_from_i64({}), val_from_i64({})); val_to_i64(m.into_val(env)) }}",
                map_val, key, val
            ))
        }
        "map_keys" => {
            let map_val = args.get(0)?;
            Some(format!(
                "val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64({})).keys().into_val(env))",
                map_val
            ))
        }
        "map_values" => {
            let map_val = args.get(0)?;
            Some(format!(
                "val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64({})).values().into_val(env))",
                map_val
            ))
        }
        "map_has" => {
            let map_val = args.get(0)?;
            let key = args.get(1)?;
            Some(format!(
                "if Map::<Val, Val>::from_val(env, &val_from_i64({})).has(val_from_i64({})) {{ 1 }} else {{ 0 }}",
                map_val, key
            ))
        }
        "map_del" => {
            let map_val = args.get(0)?;
            let key = args.get(1)?;
            Some(format!(
                "{{ let mut m = Map::<Val, Val>::from_val(env, &val_from_i64({})); m.remove(val_from_i64({})); val_to_i64(m.into_val(env)) }}",
                map_val, key
            ))
        }
        "map_key_by_pos" => {
            let map_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64({})).keys().get_unchecked({} as u32))",
                map_val, idx
            ))
        }
        "map_val_by_pos" => {
            let map_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64({})).values().get_unchecked({} as u32))",
                map_val, idx
            ))
        }
        "map_new_from_linear_memory" => {
            let keys_addr = args.get(0)?;
            let vals_addr = args.get(1)?;
            let len = args.get(2)?;
            if let (Some(keys_addr), Some(vals_addr), Some(len)) = (
                parse_u32_literal(keys_addr),
                parse_u32_literal(vals_addr),
                parse_u32_literal(len),
            ) {
                if let (Some(keys), Some(vals)) = (
                    resolve_linear_memory_u64_array(keys_addr, len, data_segments),
                    resolve_linear_memory_u64_array(vals_addr, len, data_segments),
                ) {
                    let mut out = String::from("{ let mut m = Map::<Val, Val>::new(env);");
                    for (k, v) in keys.iter().zip(vals.iter()) {
                        out.push_str(&format!(
                            " m.set(Val::from_u64({}), Val::from_u64({}));",
                            k, v
                        ));
                    }
                    out.push_str(" val_to_i64(m.into_val(env)) }");
                    return Some(out);
                }
            }
            Some("val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */"
                .to_string())
        }
        "map_unpack_to_linear_memory" => {
            Some("0 /* TODO: map_unpack_to_linear_memory */".to_string())
        }
        "contract_event" => {
            if args.len() >= 2 {
                let topics = if args.len() == 2 { &args[0] } else { &args[1] };
                let data = if args.len() == 2 { &args[1] } else { &args[2] };
                Some(format!(
                    "{{ env.events().publish(val_from_i64({}), val_from_i64({})); 0 }}",
                    topics, data
                ))
            } else {
                Some("0 /* TODO: contract_event */".to_string())
            }
        }
        "symbol_new_from_linear_memory" => {
            let addr = args.get(0)?;
            let len = args.get(1)?;
            if let (Some(addr), Some(len)) = (parse_u32_literal(addr), parse_u32_literal(len)) {
                if let Some(s) = resolve_linear_memory_string(addr, len, data_segments) {
                    return Some(format!("val_to_i64(Symbol::new(env, \"{}\"))", s));
                }
            }
            Some("val_to_i64(Symbol::new(env, \"\")) /* TODO: linear memory */".to_string())
        }
        "symbol_len" => {
            let sym = args.get(0)?;
            Some(format!(
                "Symbol::from_val(env, &val_from_i64({})).len() as i64",
                sym
            ))
        }
        "symbol_copy_to_linear_memory" => Some("0 /* TODO: symbol_copy_to_linear_memory */".to_string()),
        "symbol_index_in_linear_memory" => Some("0 /* TODO: symbol_index_in_linear_memory */".to_string()),
        "string_new_from_linear_memory" => {
            let addr = args.get(0)?;
            let len = args.get(1)?;
            if let (Some(addr), Some(len)) = (parse_u32_literal(addr), parse_u32_literal(len)) {
                if let Some(s) = resolve_linear_memory_string(addr, len, data_segments) {
                    return Some(format!("val_to_i64(String::from_str(env, \"{}\"))", s));
                }
            }
            Some("val_to_i64(String::from_str(env, \"\")) /* TODO: linear memory */".to_string())
        }
        "string_len" => {
            let s = args.get(0)?;
            Some(format!(
                "String::from_val(env, &val_from_i64({})).len() as i64",
                s
            ))
        }
        "string_to_bytes" => {
            let s = args.get(0)?;
            Some(format!(
                "val_to_i64(String::from_val(env, &val_from_i64({})).to_bytes().into_val(env))",
                s
            ))
        }
        "string_copy_to_linear_memory" => {
            let s = args.get(0)?;
            let s_pos = args.get(1)?;
            let lm_pos = args.get(2)?;
            let len = args.get(3)?;
            Some(format!(
                "{{ self.copy_string_to_linear_memory(env, {}, {}, {}, {}); 0 }}",
                s, s_pos, lm_pos, len
            ))
        }
        "bytes_new_from_linear_memory" => {
            let addr = args.get(0)?;
            let len = args.get(1)?;
            if let (Some(addr), Some(len)) = (parse_u32_literal(addr), parse_u32_literal(len)) {
                if let Some(bytes) = resolve_linear_memory_bytes(addr, len, data_segments) {
                    let literal = format_bytes_literal(&bytes);
                    return Some(format!(
                        "val_to_i64(Bytes::from_slice(env, &{}).into_val(env))",
                        literal
                    ));
                }
            }
            Some("val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */".to_string())
        }
        "bytes_new" => Some("val_to_i64(Bytes::new(env).into_val(env))".to_string()),
        "bytes_len" => {
            let bytes_val = args.get(0)?;
            Some(format!(
                "Bytes::from_val(env, &val_from_i64({})).len() as i64",
                bytes_val
            ))
        }
        "bytes_get" => {
            let bytes_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "Bytes::from_val(env, &val_from_i64({})).get_unchecked({} as u32) as i64",
                bytes_val, idx
            ))
        }
        "bytes_put" => {
            let bytes_val = args.get(0)?;
            let idx = args.get(1)?;
            let value = args.get(2)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); b.set({} as u32, {} as u8); val_to_i64(b.into_val(env)) }}",
                bytes_val, idx, value
            ))
        }
        "bytes_insert" => {
            let bytes_val = args.get(0)?;
            let idx = args.get(1)?;
            let value = args.get(2)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); b.insert({} as u32, {} as u8); val_to_i64(b.into_val(env)) }}",
                bytes_val, idx, value
            ))
        }
        "bytes_del" => {
            let bytes_val = args.get(0)?;
            let idx = args.get(1)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); b.remove_unchecked({} as u32); val_to_i64(b.into_val(env)) }}",
                bytes_val, idx
            ))
        }
        "bytes_append" => {
            let bytes_val = args.get(0)?;
            let other = args.get(1)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); b.append(&Bytes::from_val(env, &val_from_i64({}))); val_to_i64(b.into_val(env)) }}",
                bytes_val, other
            ))
        }
        "bytes_slice" => {
            let bytes_val = args.get(0)?;
            let start = args.get(1)?;
            let end = args.get(2)?;
            Some(format!(
                "val_to_i64(Bytes::from_val(env, &val_from_i64({})).slice({} as u32..{} as u32).into_val(env))",
                bytes_val, start, end
            ))
        }
        "bytes_front" => {
            let bytes_val = args.get(0)?;
            Some(format!(
                "Bytes::from_val(env, &val_from_i64({})).get(0) as i64",
                bytes_val
            ))
        }
        "bytes_back" => {
            let bytes_val = args.get(0)?;
            Some(format!(
                "Bytes::from_val(env, &val_from_i64({})).last_unchecked() as i64",
                bytes_val
            ))
        }
        "bytes_push" => {
            let bytes_val = args.get(0)?;
            let value = args.get(1)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); b.push_back({} as u8); val_to_i64(b.into_val(env)) }}",
                bytes_val, value
            ))
        }
        "bytes_pop" => {
            let bytes_val = args.get(0)?;
            Some(format!(
                "{{ let mut b = Bytes::from_val(env, &val_from_i64({})); let _ = b.pop_back_unchecked(); val_to_i64(b.into_val(env)) }}",
                bytes_val
            ))
        }
        "bytes_copy_to_linear_memory" => {
            let b = args.get(0)?;
            let b_pos = args.get(1)?;
            let lm_pos = args.get(2)?;
            let len = args.get(3)?;
            Some(format!(
                "{{ self.copy_bytes_to_linear_memory(env, {}, {}, {}, {}); 0 }}",
                b, b_pos, lm_pos, len
            ))
        }
        "bytes_copy_from_linear_memory" => {
            Some("0 /* TODO: bytes_copy_from_linear_memory */".to_string())
        }
        "bytes_to_string" => {
            let bytes_val = args.get(0)?;
            Some(format!(
                "val_to_i64(Bytes::from_val(env, &val_from_i64({})).to_string().into_val(env))",
                bytes_val
            ))
        }
        "put_contract_data" => {
            let key = args.get(0)?;
            let val = args.get(1)?;
            if let Some(storage_type) = args.get(2) {
                let persistent = format!(
                    "env.storage().persistent().set(&val_from_i64({}), &val_from_i64({})); 0",
                    key, val
                );
                let temporary = format!(
                    "env.storage().temporary().set(&val_from_i64({}), &val_from_i64({})); 0",
                    key, val
                );
                let instance = format!(
                    "env.storage().instance().set(&val_from_i64({}), &val_from_i64({})); 0",
                    key, val
                );
                Some(storage_match_expr(storage_type, &persistent, &temporary, &instance))
            } else {
                Some(format!(
                    "{{ env.storage().instance().set(&val_from_i64({}), &val_from_i64({})); 0 }}",
                    key, val
                ))
            }
        }
        "get_contract_data" => {
            let key = args.get(0)?;
            if let Some(storage_type) = args.get(1) {
                let persistent = format!(
                    "val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64({})).unwrap_or(val_from_i64(0)))",
                    key
                );
                let temporary = format!(
                    "val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64({})).unwrap_or(val_from_i64(0)))",
                    key
                );
                let instance = format!(
                    "val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64({})).unwrap_or(val_from_i64(0)))",
                    key
                );
                Some(storage_match_expr(storage_type, &persistent, &temporary, &instance))
            } else {
                Some(format!(
                    "val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64({})).unwrap_or(val_from_i64(0)))",
                    key
                ))
            }
        }
        "has_contract_data" => {
            let key = args.get(0)?;
            if let Some(storage_type) = args.get(1) {
                let persistent = format!(
                    "if env.storage().persistent().has(&val_from_i64({})) {{ 1 }} else {{ 0 }}",
                    key
                );
                let temporary = format!(
                    "if env.storage().temporary().has(&val_from_i64({})) {{ 1 }} else {{ 0 }}",
                    key
                );
                let instance = format!(
                    "if env.storage().instance().has(&val_from_i64({})) {{ 1 }} else {{ 0 }}",
                    key
                );
                Some(storage_match_expr(storage_type, &persistent, &temporary, &instance))
            } else {
                Some(format!(
                    "if env.storage().instance().has(&val_from_i64({})) {{ 1 }} else {{ 0 }}",
                    key
                ))
            }
        }
        "del_contract_data" => {
            let key = args.get(0)?;
            if let Some(storage_type) = args.get(1) {
                let persistent = format!(
                    "env.storage().persistent().remove(&val_from_i64({})); 0",
                    key
                );
                let temporary = format!(
                    "env.storage().temporary().remove(&val_from_i64({})); 0",
                    key
                );
                let instance = format!(
                    "env.storage().instance().remove(&val_from_i64({})); 0",
                    key
                );
                Some(storage_match_expr(storage_type, &persistent, &temporary, &instance))
            } else {
                Some(format!(
                    "{{ env.storage().instance().remove(&val_from_i64({})); 0 }}",
                    key
                ))
            }
        }
        "extend_contract_data_ttl" => {
            let key = args.get(0)?;
            let storage_type = args.get(1)?;
            let threshold = args.get(2)?;
            let extend_to = args.get(3)?;
            let persistent = format!(
                "env.storage().persistent().extend_ttl(&val_from_i64({}), {} as u32, {} as u32); 0",
                key, threshold, extend_to
            );
            let temporary = format!(
                "env.storage().temporary().extend_ttl(&val_from_i64({}), {} as u32, {} as u32); 0",
                key, threshold, extend_to
            );
            let instance = format!(
                "env.storage().instance().extend_ttl({} as u32, {} as u32); 0",
                threshold, extend_to
            );
            Some(storage_match_expr(storage_type, &persistent, &temporary, &instance))
        }
        "extend_current_contract_instance_and_code_ttl" => {
            let threshold = args.get(0)?;
            let extend_to = args.get(1)?;
            Some(format!(
                "{{ env.storage().instance().extend_ttl({} as u32, {} as u32); 0 }}",
                threshold, extend_to
            ))
        }
        "extend_current_contract_instance_ttl" | "extend_current_contract_code_ttl" => {
            let threshold = args.get(0)?;
            let extend_to = args.get(1)?;
            Some(format!(
                "{{ env.storage().instance().extend_ttl({} as u32, {} as u32); 0 }}",
                threshold, extend_to
            ))
        }
        "extend_contract_instance_and_code_ttl" => {
            let addr = args.get(0)?;
            let threshold = args.get(1)?;
            let extend_to = args.get(2)?;
            Some(format!(
                "{{ env.deployer().extend_ttl(Address::from_val(env, &val_from_i64({})), {} as u32, {} as u32); 0 }}",
                addr, threshold, extend_to
            ))
        }
        "extend_contract_instance_ttl" => {
            let addr = args.get(0)?;
            let threshold = args.get(1)?;
            let extend_to = args.get(2)?;
            Some(format!(
                "{{ env.deployer().extend_ttl_for_contract_instance(Address::from_val(env, &val_from_i64({})), {} as u32, {} as u32); 0 }}",
                addr, threshold, extend_to
            ))
        }
        "extend_contract_code_ttl" => {
            let addr = args.get(0)?;
            let threshold = args.get(1)?;
            let extend_to = args.get(2)?;
            Some(format!(
                "{{ env.deployer().extend_ttl_for_code(Address::from_val(env, &val_from_i64({})), {} as u32, {} as u32); 0 }}",
                addr, threshold, extend_to
            ))
        }
        "get_current_contract_address" => Some(
            "val_to_i64(env.current_contract_address().into_val(env))".to_string(),
        ),
        "get_ledger_version" => Some("env.ledger().protocol_version() as i64".to_string()),
        "get_ledger_sequence" => Some("env.ledger().sequence() as i64".to_string()),
        "get_ledger_timestamp" => Some("env.ledger().timestamp() as i64".to_string()),
        "get_ledger_network_id" => {
            Some("val_to_i64(env.ledger().network_id().into_val(env))".to_string())
        }
        "get_max_live_until_ledger" => {
            Some("env.ledger().max_live_until_ledger() as i64".to_string())
        }
        "require_auth" => {
            let addr = args.get(0)?;
            Some(format!(
                "{{ address_from_i64(env, {}).require_auth(); 0 }}",
                addr
            ))
        }
        "require_auth_for_args" => {
            let addr = args.get(0)?;
            let auth_args = args.get(1)?;
            Some(format!(
                "{{ address_from_i64(env, {}).require_auth_for_args(val_from_i64({})); 0 }}",
                addr, auth_args
            ))
        }
        "authorize_as_curr_contract" => {
            let entries = args.get(0)?;
            Some(format!(
                "{{ env.authorize_as_current_contract(Vec::<InvokerContractAuthEntry>::from_val(env, &val_from_i64({}))); 0 }}",
                entries
            ))
        }
        "verify_sig_ed25519" => {
            let pk = args.get(0)?;
            let payload = args.get(1)?;
            let sig = args.get(2)?;
            Some(format!(
                "{{ env.crypto().ed25519_verify(&BytesN::<32>::from_val(env, &val_from_i64({})), &BytesN::<32>::from_val(env, &val_from_i64({})).into(), &BytesN::<64>::from_val(env, &val_from_i64({}))); 0 }}",
                pk, payload, sig
            ))
        }
        "compute_hash_sha256" => {
            let bytes = args.get(0)?;
            Some(format!(
                "val_to_i64(env.crypto().sha256(&Bytes::from_val(env, &val_from_i64({}))).into())",
                bytes
            ))
        }
        "compute_hash_keccak256" => {
            let bytes = args.get(0)?;
            Some(format!(
                "val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64({}))).into())",
                bytes
            ))
        }
        "recover_key_ecdsa_secp256k1" => {
            let msg = args.get(0)?;
            let sig = args.get(1)?;
            let rec_id = args.get(2)?;
            Some(format!(
                "val_to_i64(env.crypto().secp256k1_recover(&Hash::<32>::from_val(env, &val_from_i64({})), &BytesN::<64>::from_val(env, &val_from_i64({})), {} as u32).into())",
                msg, sig, rec_id
            ))
        }
        "verify_sig_ecdsa_secp256r1" => {
            let pk = args.get(0)?;
            let msg = args.get(1)?;
            let sig = args.get(2)?;
            Some(format!(
                "{{ env.crypto().secp256r1_verify(&BytesN::<65>::from_val(env, &val_from_i64({})), &Hash::<32>::from_val(env, &val_from_i64({})), &BytesN::<64>::from_val(env, &val_from_i64({}))); 0 }}",
                pk, msg, sig
            ))
        }
        "call" => {
            let addr = args.get(0)?;
            let func = args.get(1)?;
            let call_args = args.get(2)?;
            Some(format!(
                "val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64({})), &Symbol::from_val(env, &val_from_i64({})), Vec::<Val>::from_val(env, &val_from_i64({}))))",
                addr, func, call_args
            ))
        }
        "try_call" => Some("0 /* TODO: try_call */".to_string()),
        "upload_wasm" => {
            let wasm = args.get(0)?;
            Some(format!(
                "val_to_i64(env.deployer().upload_contract_wasm(Bytes::from_val(env, &val_from_i64({}))).into_val(env))",
                wasm
            ))
        }
        "update_current_contract_wasm" => {
            let wasm = args.get(0)?;
            Some(format!(
                "{{ env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64({}))); 0 }}",
                wasm
            ))
        }
        "create_contract" => {
            let deployer = args.get(0)?;
            let wasm = args.get(1)?;
            let salt = args.get(2)?;
            Some(format!(
                "val_to_i64(env.deployer().with_address(Address::from_val(env, &val_from_i64({})), BytesN::<32>::from_val(env, &val_from_i64({}))).deploy(BytesN::<32>::from_val(env, &val_from_i64({}))).into_val(env))",
                deployer, salt, wasm
            ))
        }
        "create_contract_with_constructor" => {
            let deployer = args.get(0)?;
            let wasm = args.get(1)?;
            let salt = args.get(2)?;
            let ctor = args.get(3)?;
            Some(format!(
                "val_to_i64(env.deployer().with_address(Address::from_val(env, &val_from_i64({})), BytesN::<32>::from_val(env, &val_from_i64({}))).deploy_v2(BytesN::<32>::from_val(env, &val_from_i64({})), Vec::<Val>::from_val(env, &val_from_i64({}))).into_val(env))",
                deployer, salt, wasm, ctor
            ))
        }
        "create_asset_contract" => {
            let asset = args.get(0)?;
            Some(format!(
                "val_to_i64(env.deployer().with_stellar_asset(Bytes::from_val(env, &val_from_i64({}))).deploy().into_val(env))",
                asset
            ))
        }
        "get_contract_id" => {
            let deployer = args.get(0)?;
            let salt = args.get(1)?;
            Some(format!(
                "val_to_i64(env.deployer().with_address(Address::from_val(env, &val_from_i64({})), BytesN::<32>::from_val(env, &val_from_i64({}))).deployed_address().into_val(env))",
                deployer, salt
            ))
        }
        "get_asset_contract_id" => {
            let asset = args.get(0)?;
            Some(format!(
                "val_to_i64(env.deployer().with_stellar_asset(Bytes::from_val(env, &val_from_i64({}))).deployed_address().into_val(env))",
                asset
            ))
        }
        "strkey_to_address" => {
            let strkey = args.get(0)?;
            Some(format!(
                "val_to_i64(Address::from_string_bytes(&Bytes::from_val(env, &val_from_i64({}))).into_val(env))",
                strkey
            ))
        }
        "address_to_strkey" => {
            let addr = args.get(0)?;
            Some(format!(
                "val_to_i64(Address::from_val(env, &val_from_i64({})).to_string().into_val(env))",
                addr
            ))
        }
        "get_address_from_muxed_address" => {
            let muxed = args.get(0)?;
            Some(format!(
                "val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64({})).unwrap().address().into_val(env))",
                muxed
            ))
        }
        "get_id_from_muxed_address" => {
            let muxed = args.get(0)?;
            Some(format!(
                "val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64({})).unwrap().id().into_val(env))",
                muxed
            ))
        }
        "timepoint_obj_from_u64" => {
            let v = args.get(0)?;
            Some(format!(
                "val_to_i64(Timepoint::from_unix(env, {} as u64).into_val(env))",
                v
            ))
        }
        "timepoint_obj_to_u64" => {
            let v = args.get(0)?;
            Some(format!(
                "Timepoint::try_from_val(env, &val_from_i64({})).unwrap().to_unix() as i64",
                v
            ))
        }
        "duration_obj_from_u64" => {
            let v = args.get(0)?;
            Some(format!(
                "val_to_i64(Duration::from_seconds(env, {} as u64).into_val(env))",
                v
            ))
        }
        "duration_obj_to_u64" => {
            let v = args.get(0)?;
            Some(format!(
                "Duration::try_from_val(env, &val_from_i64({})).unwrap().to_seconds() as i64",
                v
            ))
        }
        "prng_reseed" => {
            let seed = args.get(0)?;
            Some(format!(
                "{{ env.prng().seed(Bytes::from_val(env, &val_from_i64({}))); 0 }}",
                seed
            ))
        }
        "prng_bytes_new" => {
            let len = args.get(0)?;
            Some(format!(
                "val_to_i64(env.prng().gen_len::<Bytes>({} as u32).into_val(env))",
                len
            ))
        }
        "prng_u64_in_inclusive_range" => {
            let lo = args.get(0)?;
            let hi = args.get(1)?;
            Some(format!(
                "env.prng().gen_range::<u64>({} as u64..={} as u64) as i64",
                lo, hi
            ))
        }
        "prng_vec_shuffle" => {
            let vec_val = args.get(0)?;
            Some(format!(
                "{{ let mut v = Vec::<Val>::from_val(env, &val_from_i64({})); env.prng().shuffle(&mut v); val_to_i64(v.into_val(env)) }}",
                vec_val
            ))
        }
        "u256_add" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().add(&U256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "u256_sub" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().sub(&U256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "u256_mul" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().mul(&U256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "u256_div" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().div(&U256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "u256_rem_euclid" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().rem_euclid(&U256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "u256_pow" => {
            let a = args.get(0)?;
            let pow = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().pow({} as u32).into_val(env))",
                a, pow
            ))
        }
        "u256_shl" => {
            let a = args.get(0)?;
            let bits = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().shl({} as u32).into_val(env))",
                a, bits
            ))
        }
        "u256_shr" => {
            let a = args.get(0)?;
            let bits = args.get(1)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().shr({} as u32).into_val(env))",
                a, bits
            ))
        }
        "u256_val_from_be_bytes" => {
            let bytes = args.get(0)?;
            Some(format!(
                "val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64({}))).into_val(env))",
                bytes
            ))
        }
        "u256_val_to_be_bytes" => {
            let val = args.get(0)?;
            Some(format!(
                "val_to_i64(U256::try_from_val(env, &val_from_i64({})).unwrap().to_be_bytes().into_val(env))",
                val
            ))
        }
        "obj_from_u256_pieces" => {
            let hi_hi = args.get(0)?;
            let hi_lo = args.get(1)?;
            let lo_hi = args.get(2)?;
            let lo_lo = args.get(3)?;
            Some(format!(
                "val_to_i64(U256::from_parts(env, {} as u64, {} as u64, {} as u64, {} as u64).into_val(env))",
                hi_hi, hi_lo, lo_hi, lo_lo
            ))
        }
        "obj_to_u256_hi_hi" => Some("0 /* TODO: obj_to_u256_hi_hi */".to_string()),
        "obj_to_u256_hi_lo" => Some("0 /* TODO: obj_to_u256_hi_lo */".to_string()),
        "obj_to_u256_lo_hi" => Some("0 /* TODO: obj_to_u256_lo_hi */".to_string()),
        "obj_to_u256_lo_lo" => Some("0 /* TODO: obj_to_u256_lo_lo */".to_string()),
        "obj_from_i256_pieces" => {
            let hi_hi = args.get(0)?;
            let hi_lo = args.get(1)?;
            let lo_hi = args.get(2)?;
            let lo_lo = args.get(3)?;
            Some(format!(
                "val_to_i64(I256::from_parts(env, {} as i64, {} as u64, {} as u64, {} as u64).into_val(env))",
                hi_hi, hi_lo, lo_hi, lo_lo
            ))
        }
        "obj_to_i256_hi_hi" => Some("0 /* TODO: obj_to_i256_hi_hi */".to_string()),
        "obj_to_i256_hi_lo" => Some("0 /* TODO: obj_to_i256_hi_lo */".to_string()),
        "obj_to_i256_lo_hi" => Some("0 /* TODO: obj_to_i256_lo_hi */".to_string()),
        "obj_to_i256_lo_lo" => Some("0 /* TODO: obj_to_i256_lo_lo */".to_string()),
        "i256_add" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().add(&I256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "i256_sub" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().sub(&I256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "i256_mul" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().mul(&I256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "i256_div" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().div(&I256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "i256_rem_euclid" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().rem_euclid(&I256::try_from_val(env, &val_from_i64({})).unwrap()).into_val(env))",
                a, b
            ))
        }
        "i256_pow" => {
            let a = args.get(0)?;
            let pow = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().pow({} as u32).into_val(env))",
                a, pow
            ))
        }
        "i256_shl" => {
            let a = args.get(0)?;
            let bits = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().shl({} as u32).into_val(env))",
                a, bits
            ))
        }
        "i256_shr" => {
            let a = args.get(0)?;
            let bits = args.get(1)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().shr({} as u32).into_val(env))",
                a, bits
            ))
        }
        "i256_val_from_be_bytes" => {
            let bytes = args.get(0)?;
            Some(format!(
                "val_to_i64(I256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64({}))).into_val(env))",
                bytes
            ))
        }
        "i256_val_to_be_bytes" => {
            let val = args.get(0)?;
            Some(format!(
                "val_to_i64(I256::try_from_val(env, &val_from_i64({})).unwrap().to_be_bytes().into_val(env))",
                val
            ))
        }
        "serialize_to_bytes" => {
            let v = args.get(0)?;
            Some(format!(
                "val_to_i64(Bytes::from_val(env, &val_from_i64({})).into()) /* TODO: serialize_to_bytes */",
                v
            ))
        }
        "deserialize_from_bytes" => Some("0 /* TODO: deserialize_from_bytes */".to_string()),
        "fail_with_error" => Some("0 /* TODO: fail_with_error */".to_string()),
        "log_from_linear_memory" => Some("0 /* TODO: log_from_linear_memory */".to_string()),
        "protocol_gated_dummy" | "dummy0" => Some("0 /* TODO: dummy */".to_string()),
        "obj_to_u64" => {
            let v = args.get(0)?;
            Some(format!("val_from_i64({}).as_u64().unwrap_or(0) as i64", v))
        }
        "obj_to_i64" => {
            let v = args.get(0)?;
            Some(format!("val_from_i64({}).as_i64().unwrap_or(0) as i64", v))
        }
        "obj_from_u64" => {
            let v = args.get(0)?;
            Some(format!("val_to_i64(Val::from_u64({} as u64))", v))
        }
        "obj_from_i64" => {
            let v = args.get(0)?;
            Some(format!("val_to_i64(Val::from_i64({} as i64))", v))
        }
        "obj_to_i128_hi64" => {
            let v = args.get(0)?;
            Some(format!(
                "((val_from_i64({}).as_i128().unwrap_or(0) >> 64) as i64)",
                v
            ))
        }
        "obj_to_i128_lo64" => {
            let v = args.get(0)?;
            Some(format!(
                "((val_from_i64({}).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)",
                v
            ))
        }
        "obj_from_i128_pieces" => {
            let hi = args.get(0)?;
            let lo = args.get(1)?;
            Some(format!(
                "val_to_i64(Val::from_i128((({} as i128) << 64) | ({} as u64 as i128)))",
                hi, lo
            ))
        }
        "obj_to_u128_hi64" => {
            let v = args.get(0)?;
            Some(format!(
                "((val_from_i64({}).as_u128().unwrap_or(0) >> 64) as i64)",
                v
            ))
        }
        "obj_to_u128_lo64" => {
            let v = args.get(0)?;
            Some(format!(
                "((val_from_i64({}).as_u128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)",
                v
            ))
        }
        "obj_from_u128_pieces" => {
            let hi = args.get(0)?;
            let lo = args.get(1)?;
            Some(format!(
                "val_to_i64(Val::from_u128((({} as u128) << 64) | ({} as u64 as u128)))",
                hi, lo
            ))
        }
        "obj_cmp" => {
            let a = args.get(0)?;
            let b = args.get(1)?;
            Some(format!(
                "{{ let a = val_from_i64({}); let b = val_from_i64({}); if a < b {{ -1 }} else if a > b {{ 1 }} else {{ 0 }} }}",
                a, b
            ))
        }
        "get_address_executable" => Some("0 /* TODO: get_address_executable */".to_string()),
        "get_contract_data_live_until_ledger" => {
            Some("0 /* TODO: get_contract_data_live_until_ledger */".to_string())
        }
        "extend_contract_data_ttl_v2" => {
            Some("0 /* TODO: extend_contract_data_ttl_v2 */".to_string())
        }
        "extend_contract_instance_and_code_ttl_v2" => {
            Some("0 /* TODO: extend_contract_instance_and_code_ttl_v2 */".to_string())
        }
        "get_contract_data_live_until_ledger_v2" => {
            Some("0 /* TODO: get_contract_data_live_until_ledger_v2 */".to_string())
        }
        "bls12_381_check_g1_is_in_subgroup"
        | "bls12_381_check_g2_is_in_subgroup"
        | "bls12_381_g1_add"
        | "bls12_381_g1_mul"
        | "bls12_381_g1_msm"
        | "bls12_381_g1_is_on_curve"
        | "bls12_381_g2_add"
        | "bls12_381_g2_mul"
        | "bls12_381_g2_msm"
        | "bls12_381_g2_is_on_curve"
        | "bls12_381_map_fp_to_g1"
        | "bls12_381_map_fp2_to_g2"
        | "bls12_381_hash_to_g1"
        | "bls12_381_hash_to_g2"
        | "bls12_381_multi_pairing_check"
        | "bls12_381_fr_add"
        | "bls12_381_fr_sub"
        | "bls12_381_fr_mul"
        | "bls12_381_fr_pow"
        | "bls12_381_fr_inv"
        | "bn254_g1_add"
        | "bn254_g1_mul"
        | "bn254_g1_msm"
        | "bn254_g1_is_on_curve"
        | "bn254_multi_pairing_check"
        | "bn254_fr_add"
        | "bn254_fr_sub"
        | "bn254_fr_mul"
        | "bn254_fr_pow"
        | "bn254_fr_inv"
        | "poseidon_permutation"
        | "poseidon2_permutation" => Some(format!("0 /* TODO: {} */", name)),
        _ => None,
    }
}

fn strip_parens(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') && s.len() > 2 {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn strip_tag_expr(s: &str) -> Option<String> {
    let s = strip_parens(s).trim();
    let idx = s.rfind('&')?;
    let (left, right) = s.split_at(idx);
    if right[1..].trim() == "255" {
        Some(left.trim().to_string())
    } else {
        None
    }
}

fn extract_obj_type(s: &str) -> Option<String> {
    let s = s.trim();
    let start = s.find("/*")?;
    let end = s[start + 2..].find("*/")?;
    let inside = s[start + 2..start + 2 + end].trim();
    let paren = inside.find("(obj#0)")?;
    Some(inside[..paren].trim().to_string())
}

fn extract_obj_type_direct(s: &str) -> Option<String> {
    let s = s.trim();
    let pos = s.find("(obj#0)")?;
    let head = s[..pos].trim();
    if head.is_empty() {
        None
    } else {
        Some(head.trim_matches(|c| c == '(' || c == ')').to_string())
    }
}

fn try_format_tag_compare(a: &str, b: &str, equal: bool) -> Option<String> {
    let a_type = extract_obj_type(a).or_else(|| extract_obj_type_direct(a));
    let b_type = extract_obj_type(b).or_else(|| extract_obj_type_direct(b));
    let a_base = strip_tag_expr(a);
    let b_base = strip_tag_expr(b);
    let (base, ty) = if let (Some(base), Some(ty)) = (a_base, b_type) {
        (base, ty)
    } else if let (Some(base), Some(ty)) = (b_base, a_type) {
        (base, ty)
    } else {
        return None;
    };
    let check = match ty.as_str() {
        "Address" => format!("Address::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "Vec" => format!("Vec::<Val>::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "Map" => format!("Map::<Val, Val>::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "Bytes" => format!("Bytes::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "String" => format!("String::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "Symbol" => format!("Symbol::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "MuxedAddress" => {
            format!("MuxedAddress::try_from_val(env, &val_from_i64({})).is_ok()", base)
        }
        _ => return None,
    };
    if equal {
        Some(check)
    } else {
        Some(format!("!({})", check))
    }
}

fn parse_stack_addr(addr: &str) -> Option<(String, i32)> {
    let s = addr.trim();
    let s = s.trim_start_matches('(').trim_end_matches(')');
    let s = s
        .replace(" as i32", "")
        .replace(" as i64", "")
        .replace(" as usize", "");
    if let Some(caps) = Regex::new(r"^(var\d+)$").ok()?.captures(&s) {
        return Some((caps[1].to_string(), 0));
    }
    if let Some(caps) = Regex::new(r"^(var\d+)\.wrapping_add\(([-\d]+)\)$")
        .ok()?
        .captures(&s)
    {
        let base = caps[1].to_string();
        let off = caps[2].parse::<i32>().ok()?;
        return Some((base, off));
    }
    if let Some(caps) = Regex::new(r"^(var\d+)\.wrapping_sub\(([-\d]+)\)$")
        .ok()?
        .captures(&s)
    {
        let base = caps[1].to_string();
        let off = caps[2].parse::<i32>().ok()?;
        return Some((base, -off));
    }
    None
}

fn slot_name(base: &str, offset: i32, suffix: &str) -> String {
    let mut name = format!("slot_{}_{}_{}", base, offset, suffix);
    name = name.replace('-', "m");
    name
}

fn is_breakable_if(remaining_ops: &[Instruction]) -> bool {
    let mut stack = 0;

    for opcode in remaining_ops {
        use parity_wasm::elements::Instruction::*;
        match *opcode {
            Block(_) | If(_) | Loop(_) => {
                stack += 1;
            }
            End => {
                if stack == 0 {
                    return false;
                }
                stack -= 1;
            }
            Br(relative_depth) | BrIf(relative_depth) => {
                if relative_depth == stack {
                    return true;
                }
            }
            BrTable(ref table) => {
                if table.table.iter().any(|&i| i == stack) || table.default == stack {
                    return true;
                }
            }
            _ => {}
        }
    }
    panic!("Unclosed block")
}

pub fn transform_from_soroban_val_raw(val: u64) -> String {
    let mut fmt = format!("{}", val);
    let v = Val::from_payload(val);
    if v.is_good() {
        fmt = format!("{:?}", v);
        let re = Regex::new(r"\((?<value>\w+)\)").unwrap();
        if let Some(captures) = re.captures(fmt.as_str()) {
            if let Some(value) = captures.get(1) {
                fmt = format!("{}", value.as_str());
            }
        }
        replace_format_string(&mut fmt);
    }
    fmt
}

pub fn transform_from_soroban_val(val: u64) -> String {
    let raw = transform_from_soroban_val_raw(val);
    sanitize_val_literal(val, &raw)
}

fn sanitize_val_literal(val: u64, raw: &str) -> String {
    match raw {
        "True" => return "1 /* True */".to_string(),
        "False" => return "0 /* False */".to_string(),
        "Void" => return "0 /* Void */".to_string(),
        "Symbol()" => return "0 /* Symbol() */".to_string(),
        _ => {}
    }
    if raw.contains('#') {
        return format!("{} /* {} */", val, raw);
    }

    if let Some(rest) = raw.strip_prefix("Error(Contract, #") {
        if let Some(num) = rest.strip_suffix(")") {
            return format!("0 /* Error(Contract, #{}) */", num);
        }
    }

    for prefix in ["I32(", "U32(", "I64(", "U64("] {
        if let Some(rest) = raw.strip_prefix(prefix) {
            if let Some(num) = rest.strip_suffix(")") {
                return format!("{} /* {} */", num, raw);
            }
        }
    }

    raw.to_string()
}
fn replace_format_string(t: &mut String) {
    let mut replacements = HashMap::new();
    replacements.insert(
        format!("{}", INSTANCE_LIFETIME_THRESHOLD),
        "INSTANCE_LIFETIME_THRESHOLD".to_string(),
    );
    replacements.insert(
        format!("{}", BALANCE_BUMP_AMOUNT),
        "BALANCE_BUMP_AMOUNT".to_string(),
    );
    replacements.insert(
        format!("{}", BALANCE_LIFETIME_THRESHOLD),
        "BALANCE_LIFETIME_THRESHOLD".to_string(),
    );
    replacements.insert(
        format!("{}", INSTANCE_BUMP_AMOUNT),
        "INSTANCE_BUMP_AMOUNT".to_string(),
    );
    for (format_string, replacement) in &replacements {
        if *t == *format_string {
            *t = replacement.clone();
            break; // Exit the loop once a match is found
        }
    }
}

fn compute_label_needed(code: &[Instruction]) -> std::collections::HashSet<usize> {
    use parity_wasm::elements::Instruction::*;
    let mut stack: Vec<usize> = Vec::new();
    let mut needed: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (idx, instr) in code.iter().enumerate() {
        match instr {
            Block(_) | Loop(_) | If(_) => stack.push(idx),
            Else => {}
            End => {
                stack.pop();
            }
            Br(depth) | BrIf(depth) => {
                let depth = *depth as usize;
                if depth > 0 && depth < stack.len() {
                    let target = stack[stack.len() - 1 - depth];
                    needed.insert(target);
                }
            }
            BrTable(table) => {
                for depth in table.table.iter().copied() {
                    let depth = depth as usize;
                    if depth > 0 && depth < stack.len() {
                        let target = stack[stack.len() - 1 - depth];
                        needed.insert(target);
                    }
                }
                let depth = table.default as usize;
                if depth > 0 && depth < stack.len() {
                    let target = stack[stack.len() - 1 - depth];
                    needed.insert(target);
                }
            }
            _ => {}
        }
    }
    needed
}

pub fn build<W: Write>(
    writer: &mut W,
    mut expr_index: usize,
    evaluates_to_value: bool,
    import_count: usize,
    imported_globals_count: usize,
    functions: &[Function],
    indirect_fns: &mut BTreeMap<u32, Vec<(u32, u32)>>,
    globals: &[Global],
    types: &TypeSection,
    code: &[Instruction],
    base_indentation: usize,
    spec_by_fn_index: &HashMap<u32, FunctionContractSpec>,
    func_index: usize,
    data_segments: &[crate::decompile::DataSegment],
    internal_call_forwarders: &std::collections::BTreeMap<u32, crate::app::utils::InternalForwarder>,
) {
    let label_needed = compute_label_needed(code);
    let mut expr_builder = ExprBuilder::new();
    let mut blocks = Vec::new();
    let mut indentation = Indentation(base_indentation);
    let mut loop_count = 0;
    let mut stack_slots_i32: HashMap<(String, i32), String> = HashMap::new();
    let mut stack_slots_i64: HashMap<(String, i32), String> = HashMap::new();
    let mut tainted_bases: std::collections::HashSet<String> = std::collections::HashSet::new();
    let allow_tainted_slot_lift = !functions[func_index].make_public;

    blocks.push(BlockKind::Function { evaluates_to_value });

    let code_vec: Vec<Instruction> = code.iter().cloned().collect();
    let mut code = code_vec.iter();
    let param_count = functions[func_index].ty.params().len();
    let spec_fn_opt = spec_by_fn_index.get(&(func_index as u32));
    let spec_len = spec_fn_opt.map(|spec| spec.inputs().len()).unwrap_or(0);
    let spec_input_offset = if spec_fn_opt.is_some() && spec_len + 1 == param_count {
        1
    } else {
        0
    };
    let mut param_name_for = |idx: u32| -> String {
        if spec_input_offset == 1 && idx == 0 {
            return "env".to_string();
        }
        if let Some(spec_fn) = spec_fn_opt {
            let spec_idx = idx.saturating_sub(spec_input_offset as u32) as usize;
            if spec_idx < spec_len {
                return mangle_fn_name(spec_fn.inputs()[spec_idx].name());
            }
        }
        format!("arg{}", idx)
    };
    let mut local_name_for = |idx: u32| -> String {
        if (idx as usize) < param_count {
            param_name_for(idx)
        } else {
            format!("var{}", idx)
        }
    };
    let mut instr_index: usize = 0;
    while let Some(opcode) = code.next() {
        // writeln!(
        //     writer,
        //     "{}// opcode: {:?} stack: {:?} block types: {:?}",
        //     indentation, opcode, expr_builder, blocks
        // ).unwrap();
        use parity_wasm::elements::Instruction::*;
        match *opcode {
            Unreachable => {
                writeln!(writer, "{}unreachable!();", indentation).unwrap();
            }
            Nop => {
                // TODO Activate this again
                // assert!(expr_builder.is_empty());
            }
            Block(block_type) => {
                let dst_var = if let BlockType::Value(ty) = block_type {
                    let var_name = format!("var{}", expr_index);
                    writeln!(
                        writer,
                        "{}let {}: {};",
                        indentation,
                        var_name,
                        to_rs_type(ty)
                    )
                    .unwrap();
                    expr_index += 1;
                    Some(var_name)
                } else {
                    None
                };

                let needs_label = label_needed.contains(&instr_index);
                if needs_label {
                    writeln!(writer, "{}'label{}: {{", indentation, loop_count).unwrap();
                    blocks.push(BlockKind::Block {
                        label: Some(loop_count),
                        dst_var,
                    });
                    loop_count += 1;
                } else {
                    writeln!(writer, "{}{{", indentation).unwrap();
                    blocks.push(BlockKind::Block { label: None, dst_var });
                }
                indentation.0 += 1;
            }
            Loop(block_type) => {
                let dst_var = if let BlockType::Value(ty) = block_type {
                    let var_name = format!("var{}", expr_index);
                    writeln!(
                        writer,
                        "{}let {}: {};",
                        indentation,
                        var_name,
                        to_rs_type(ty)
                    )
                    .unwrap();
                    expr_index += 1;
                    Some(var_name)
                } else {
                    None
                };

                let needs_label = label_needed.contains(&instr_index);
                if needs_label {
                    writeln!(writer, "{}'label{}: loop {{", indentation, loop_count).unwrap();
                    blocks.push(BlockKind::Loop {
                        label: Some(loop_count),
                        dst_var,
                    });
                    loop_count += 1;
                } else {
                    writeln!(writer, "{}loop {{", indentation).unwrap();
                    blocks.push(BlockKind::Loop { label: None, dst_var });
                }
                indentation.0 += 1;
            }
            If(block_type) => {
                let dst_var = if let BlockType::Value(ty) = block_type {
                    let var_name = format!("var{}", expr_index);
                    writeln!(
                        writer,
                        "{}let {}: {};",
                        indentation,
                        var_name,
                        to_rs_type(ty)
                    )
                    .unwrap();
                    expr_index += 1;
                    Some(var_name)
                } else {
                    None
                };

                let is_breakable = is_breakable_if(code.as_slice());
                let needs_label = label_needed.contains(&instr_index) || is_breakable;

                if needs_label {
                    writeln!(writer, "{}'label{}: {{", indentation, loop_count).unwrap();
                }
                blocks.push(BlockKind::If {
                    label: if needs_label { Some(loop_count) } else { None },
                    dst_var,
                    is_breakable,
                });
                if needs_label {
                    loop_count += 1;
                    indentation.0 += 1;
                }

                let expr = expr_builder.pop_formatted(precedence::COMPARISON).unwrap();
                writeln!(writer, "{}if {} != 0 {{", indentation, expr).unwrap();
                indentation.0 += 1;
            }
            Else => {
                if let Some(&BlockKind::If { ref dst_var, .. }) = blocks.last() {
                    if let &Some(ref dst_var) = dst_var {
                        let (_, expr) = expr_builder.pop().unwrap();
                        writeln!(writer, "{}{} = {};", indentation, dst_var, expr).unwrap();
                    }
                    indentation.0 -= 1;
                    writeln!(writer, "{}}} else {{", indentation).unwrap();
                    indentation.0 += 1;
                } else {
                    panic!("Else can only be used with an if");
                }
            }
            End => {
                match blocks.pop().expect("End used outside of a block") {
                    BlockKind::Block { dst_var, label } => {
                        if let Some(dst_var) = dst_var {
                            if let Some((_, expr)) = expr_builder.pop() {
                                writeln!(writer, "{}{} = {};", indentation, dst_var, expr).unwrap();
                                expr_builder.push((precedence::PATH, dst_var));
                            } else {
                                writeln!(writer, "{}// There should've been an expression value here, but this may be unreachable", indentation).unwrap();
                                writeln!(writer, "{}unreachable!();", indentation).unwrap();
                            }
                        }
                        let _ = label;
                    }
                    BlockKind::Loop { dst_var, .. } => {
                        if let Some(dst_var) = dst_var {
                            if let Some((_, expr)) = expr_builder.pop() {
                                writeln!(writer, "{}{} = {};", indentation, dst_var, expr).unwrap();
                                expr_builder.push((precedence::PATH, dst_var));
                            } else {
                                writeln!(writer, "{}// There should've been an expression value here, but this may be unreachable", indentation).unwrap();
                                writeln!(writer, "{}unreachable!();", indentation).unwrap();
                            }
                        }
                        writeln!(writer, "{}break;", indentation).unwrap();
                    }
                    BlockKind::If {
                        dst_var,
                        is_breakable,
                        label,
                    } => {
                        if let Some(dst_var) = dst_var {
                            if let Some((_, expr)) = expr_builder.pop() {
                                writeln!(writer, "{}{} = {};", indentation, dst_var, expr).unwrap();
                                expr_builder.push((precedence::PATH, dst_var));
                            } else {
                                writeln!(writer, "{}// There should've been an expression value here, but this may be unreachable", indentation).unwrap();
                                writeln!(writer, "{}unreachable!();", indentation).unwrap();
                            }
                        }
                        if is_breakable && label.is_some() {
                            indentation.0 -= 1;
                            writeln!(writer, "{}}}", indentation).unwrap();
                        }
                    }
                    BlockKind::Function { evaluates_to_value } => {
                        if evaluates_to_value {
                            if let Some((_, expr)) = expr_builder.pop() {
                                writeln!(writer, "{}{}", indentation, expr).unwrap();
                            } else {
                                writeln!(writer, "{}// There should've been an expression value here, but this may be unreachable", indentation).unwrap();
                                writeln!(writer, "{}unreachable!();", indentation).unwrap();
                            }
                        }
                    }
                }

                indentation.0 -= 1;
                writeln!(writer, "{}}}", indentation).unwrap();
            }
            Br(relative_depth) => {
                let block = blocks
                    .iter()
                    .rev()
                    .nth(relative_depth as usize)
                    .expect("Branch Index out of Bounds");

                match *block {
                    BlockKind::Function { evaluates_to_value } => {
                        if evaluates_to_value {
                            let (_, expr) = expr_builder.pop().unwrap();
                            writeln!(writer, "{}return {};", indentation, expr).unwrap();
                        } else {
                            writeln!(writer, "{}return;", indentation).unwrap();
                        }
                    }
                    BlockKind::Block { ref dst_var, label }
                    | BlockKind::If {
                        ref dst_var, label, ..
                    } => {
                        if let &Some(ref dst_var) = dst_var {
                            let (_, expr) = expr_builder.pop().unwrap();
                            writeln!(writer, "{}{} = {};", indentation, dst_var, expr).unwrap();
                        }
                        if let Some(label) = label {
                            writeln!(writer, "{}break 'label{};", indentation, label).unwrap();
                        } else {
                            writeln!(writer, "{}break;", indentation).unwrap();
                        }
                    }
                    BlockKind::Loop { label, .. } => {
                        if let Some(label) = label {
                            writeln!(writer, "{}continue 'label{};", indentation, label).unwrap();
                        } else {
                            writeln!(writer, "{}continue;", indentation).unwrap();
                        }
                    }
                }
            }
            BrIf(relative_depth) => {
                let cond_expr = expr_builder.pop_formatted(precedence::COMPARISON).unwrap();
                let block = blocks
                    .iter()
                    .rev()
                    .nth(relative_depth as usize)
                    .expect("Branch Index out of Bounds");

                let evaluates_to_value = match *block {
                    BlockKind::Block { ref dst_var, .. } | BlockKind::If { ref dst_var, .. } => {
                        dst_var.is_some()
                    }
                    BlockKind::Function { evaluates_to_value } => evaluates_to_value,
                    BlockKind::Loop { .. } => false,
                };

                // TODO This evaluates cond and val out of order. So this relies
                // on them not having any side effects

                let tmp_var = if evaluates_to_value {
                    let tmp_var = format!("var{}", expr_index);
                    {
                        let &(_, ref tmp_var_val) = expr_builder.inner().last().unwrap();
                        writeln!(writer, "{}let {} = {};", indentation, tmp_var, tmp_var_val)
                            .unwrap();
                    }
                    expr_index += 1;
                    expr_builder.push((precedence::PATH, tmp_var.clone()));
                    Some(tmp_var)
                } else {
                    None
                };

                writeln!(writer, "{}if {} != 0 {{", indentation, cond_expr).unwrap();

                indentation.0 += 1;

                match *block {
                    BlockKind::Block { label, ref dst_var }
                    | BlockKind::If {
                        label, ref dst_var, ..
                    } => {
                        if let Some(tmp_var) = tmp_var {
                            writeln!(
                                writer,
                                "{}{} = {};",
                                indentation,
                                dst_var.as_ref().unwrap(),
                                tmp_var,
                            )
                            .unwrap();
                        }
                        if let Some(label) = label {
                            writeln!(writer, "{}break 'label{};", indentation, label).unwrap();
                        } else {
                            writeln!(writer, "{}break;", indentation).unwrap();
                        }
                    }
                    BlockKind::Loop { label, .. } => {
                        if let Some(label) = label {
                            writeln!(writer, "{}continue 'label{};", indentation, label).unwrap();
                        } else {
                            writeln!(writer, "{}continue;", indentation).unwrap();
                        }
                    }
                    BlockKind::Function { .. } => {
                        if let Some(tmp_var) = tmp_var {
                            writeln!(writer, "{}return {};", indentation, tmp_var).unwrap();
                        } else {
                            writeln!(writer, "{}return;", indentation).unwrap();
                        }
                    }
                }

                indentation.0 -= 1;

                writeln!(writer, "{}}}", indentation).unwrap();
            }
            BrTable(ref table) => {
                let (_, expr) = expr_builder.pop().unwrap();
                writeln!(writer, "{}match {} {{", indentation, expr).unwrap();
                indentation.0 += 1;
                for (index, &relative_depth) in table.table.iter().enumerate() {
                    let target_block = blocks
                        .iter()
                        .rev()
                        .nth(relative_depth as usize);
                    emit_br_table_arm(writer, &indentation, index, target_block);
                }
                let default_block = blocks
                    .iter()
                    .rev()
                    .nth(table.default as usize);
                emit_br_table_default(writer, &indentation, default_block);
                indentation.0 -= 1;
                writeln!(writer, "{}}}", indentation).unwrap();
            }
            Return => {
                if let Some((_, expr)) = expr_builder.pop() {
                    writeln!(writer, "{}return {};", indentation, expr).unwrap();
                } else {
                    writeln!(writer, "{}return;", indentation).unwrap();
                }
            }
            Call(fn_index) => {
                let function = &functions[fn_index as usize];
                let name = &function.name;
                let fn_type = function.ty;
                let real_name = function.real_name;
                let arg_count = fn_type.params().len();
                let index = expr_builder.len() - arg_count;
                let args: Vec<String> = expr_builder
                    .inner()
                    .drain(index..)
                    .map(|(_, expr)| expr)
                    .collect();
                for arg in &args {
                    if let Some((base, _)) = parse_stack_addr(arg) {
                        tainted_bases.insert(base);
                    }
                }
                write!(writer, "{}", indentation).unwrap();
                for _ in fn_type.results() {
                    write!(writer, "let var{} = ", expr_index).unwrap();
                }
                let is_imported = (fn_index as usize) < import_count;
                if is_imported {
                    if let Some(mapped) = map_imported_call(name, &args, data_segments) {
                        write!(writer, "{}", mapped).unwrap();
                        if let Some(real_name) = real_name {
                            writeln!(writer, " // {}", real_name).unwrap();
                        } else {
                            writeln!(writer).unwrap();
                        }
                    } else {
                        write!(writer, "/* TODO: host call {} */ 0", name).unwrap();
                        if let Some(real_name) = real_name {
                            writeln!(writer, "; // {}", real_name).unwrap();
                        } else {
                            writeln!(writer, ";").unwrap();
                        }
                    }
                } else if let Some(forwarder) = internal_call_forwarders.get(&fn_index) {
                    let mut fwd_args = Vec::with_capacity(forwarder.args.len());
                    let mut ok = true;
                    for spec in &forwarder.args {
                        match spec {
                            crate::app::utils::ForwardArg::Local(idx) => {
                                if let Some(arg) = args.get(*idx) {
                                    fwd_args.push(arg.clone());
                                } else {
                                    ok = false;
                                    break;
                                }
                            }
                            crate::app::utils::ForwardArg::PackedU32(idx) => {
                                if let Some(arg) = args.get(*idx) {
                                    fwd_args.push(arg.clone());
                                } else {
                                    ok = false;
                                    break;
                                }
                            }
                            crate::app::utils::ForwardArg::I32(v) => fwd_args.push(v.to_string()),
                            crate::app::utils::ForwardArg::I64(v) => fwd_args.push(v.to_string()),
                        }
                    }
                    if !ok || fwd_args.len() != forwarder.args.len() {
                        write!(writer, "/* TODO: internal forward {} */ 0;", forwarder.name).unwrap();
                        writeln!(writer).unwrap();
                    } else if let Some(mapped) = map_imported_call(&forwarder.name, &fwd_args, data_segments) {
                        write!(writer, "{}", mapped).unwrap();
                        writeln!(writer, ";").unwrap();
                    } else {
                        write!(writer, "/* TODO: internal forward {} */ 0;", forwarder.name).unwrap();
                        writeln!(writer).unwrap();
                    }
                } else {
                    write!(writer, "self.{}(env", name).unwrap();
                    for expr in &args {
                        write!(writer, ", {}", expr).unwrap();
                    }
                    if let Some(real_name) = real_name {
                        writeln!(writer, "); // {}", real_name).unwrap();
                    } else {
                        writeln!(writer, ");").unwrap();
                    }
                }
                for _ in fn_type.results() {
                    expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                }
                expr_index += 1;
            }
            CallIndirect(type_index, _) => {
                let Type::Function(ref fn_type) = types.types()[type_index as usize];
                indirect_fns.entry(type_index).or_insert_with(Vec::new);

                write!(writer, "{}", indentation).unwrap();
                for _ in fn_type.results() {
                    write!(writer, "let var{} = ", expr_index).unwrap();
                }
                let _ = expr_builder.pop();
                write!(writer, "/* TODO: call_indirect */ 0").unwrap();
                writeln!(writer, ";").unwrap();
                for _ in fn_type.results() {
                    expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                }
                expr_index += 1;
            }
            Drop => {
                let (_, a) = expr_builder.pop().unwrap();
                // We actually write out the expression, as it may have side
                // effects. Atm we contain pretty much every side effect as its
                // own non-dropped statement. However there are also side
                // effects in just expressions, like a division by 0, which
                // causes a panic. We want to preserve these semantics.
                writeln!(writer, "{}{};", indentation, a).unwrap();
            }
            Select => {
                let c = expr_builder.pop_formatted(precedence::COMPARISON).unwrap();
                let (_, b) = expr_builder.pop().unwrap();
                let (_, a) = expr_builder.pop().unwrap();
                // Just like with drop, we have to evaluate both branches early,
                // as they may have panics that wouldn't happen if there branch
                // doesn't get chosen.
                expr_builder.push((
                    precedence::MAX,
                    format!(
                        "{{ let a = {}; let b = {}; if {} != 0 {{ a }} else {{ b }} }}",
                        a, b, c
                    ),
                ));
            }
            GetLocal(i) => {
                let is_param = (i as usize) < param_count;
                if can_local_be_reordered(i, &blocks, functions, types, code.as_slice()) {
                    let dst = local_name_for(i);
                    expr_builder.push((precedence::PATH, dst));
                } else {
                    let dst = format!("var{}", expr_index);
                    let src = if is_param {
                        param_name_for(i)
                    } else {
                        format!("var{}", i)
                    };
                    writeln!(writer, "{}let {} = {};", indentation, dst, src).unwrap();
                    expr_index += 1;
                    expr_builder.push((precedence::PATH, dst));
                }
            }
            SetLocal(i) => {
                let (_, expr) = expr_builder.pop().unwrap();
                let dst = if (i as usize) < param_count {
                    param_name_for(i)
                } else {
                    format!("var{}", i)
                };
                writeln!(writer, "{}{} = {};", indentation, dst, expr).unwrap();
            }
            TeeLocal(i) => {
                let (_, expr) = expr_builder.pop().unwrap();
                let dst = if (i as usize) < param_count {
                    param_name_for(i)
                } else {
                    format!("var{}", i)
                };
                writeln!(writer, "{}{} = {};", indentation, dst, expr).unwrap();

                if can_local_be_reordered(i, &blocks, functions, types, code.as_slice()) {
                    let dst = local_name_for(i);
                    expr_builder.push((precedence::PATH, dst));
                } else {
                    let dst = format!("var{}", expr_index);
                    let src = local_name_for(i);
                    writeln!(writer, "{}let {} = {};", indentation, dst, src).unwrap();
                    expr_index += 1;
                    expr_builder.push((precedence::PATH, dst));
                }
            }
            GetGlobal(i) => {
                let global = &globals[i as usize];
                let name = &global.name;
                if (i as usize) < imported_globals_count {
                    let dst = format!("var{}", expr_index);
                    writeln!(
                        writer,
                        "{}let {} = *imports.{}(self);",
                        indentation, dst, name
                    )
                    .unwrap();
                    expr_index += 1;
                    expr_builder.push((precedence::PATH, dst));
                } else if global.is_mutable {
                    let dst = format!("var{}", expr_index);
                    writeln!(writer, "{}let {} = self.{};", indentation, dst, name).unwrap();
                    expr_index += 1;
                    expr_builder.push((precedence::PATH, dst));
                } else {
                    expr_builder.push((precedence::PATH, format!("consts::{}", name)));
                }
            }
            SetGlobal(i) => {
                let (_, expr) = expr_builder.pop().unwrap();
                let global = &globals[i as usize];
                let name = &global.name;
                assert!(global.is_mutable);
                if (i as usize) < imported_globals_count {
                    writeln!(writer, "{}*imports.{}(self) = {};", indentation, name, expr).unwrap();
                } else {
                    writeln!(writer, "{}self.{} = {};", indentation, name, expr).unwrap();
                }
            }
            I32Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr_str = addr.to_string();
                let mut used_slot = false;
                if let Some((base, extra)) = parse_stack_addr(&addr_str) {
                    if tainted_bases.contains(&base) && !allow_tainted_slot_lift {
                        // Avoid slot lifting when base is passed to a call.
                        used_slot = false;
                    } else {
                    let off = extra.saturating_add(offset as i32);
                    if let Some(name) = stack_slots_i32.get(&(base.clone(), off)) {
                        expr_builder.push((precedence::PATH, name.clone()));
                        used_slot = true;
                    } else if let Some(name) = stack_slots_i64.get(&(base.clone(), off)) {
                        expr_builder.push((precedence::PATH, name.clone()));
                        used_slot = true;
                    } else {
                        let name = slot_name(&base, off, "i32");
                        stack_slots_i32.insert((base.clone(), off), name.clone());
                        writeln!(
                            writer,
                            "{}let mut {} = self.memory.load32({} as usize{}) as i32;",
                            indentation,
                            name,
                            addr_str,
                            if offset != 0 { format!(" + {}", offset) } else { String::new() }
                        )
                        .unwrap();
                        expr_builder.push((precedence::PATH, name));
                        used_slot = true;
                    }
                    }
                }
                if !used_slot {
                    writeln!(
                        writer,
                        "{}let var{} = self.memory.load32({} as usize{}) as i32;",
                        indentation,
                        expr_index,
                        addr_str,
                        if offset != 0 {
                            format!(" + {}", offset)
                        } else {
                            String::new()
                        }
                    )
                    .unwrap();
                    expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                    expr_index += 1;
                }
            }
            I64Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr_str = addr.to_string();
                let mut used_slot = false;
                if let Some((base, extra)) = parse_stack_addr(&addr_str) {
                    if tainted_bases.contains(&base) && !allow_tainted_slot_lift {
                        used_slot = false;
                    } else {
                    let off = extra.saturating_add(offset as i32);
                    if let Some(name) = stack_slots_i64.get(&(base.clone(), off)) {
                        expr_builder.push((precedence::PATH, name.clone()));
                        used_slot = true;
                    } else if let Some(name) = stack_slots_i32.get(&(base.clone(), off)) {
                        expr_builder.push((precedence::PATH, name.clone()));
                        used_slot = true;
                    } else {
                        let name = slot_name(&base, off, "i64");
                        stack_slots_i64.insert((base.clone(), off), name.clone());
                        writeln!(
                            writer,
                            "{}let mut {} = self.memory.load64({} as usize{}) as i64;",
                            indentation,
                            name,
                            addr_str,
                            if offset != 0 { format!(" + {}", offset) } else { String::new() }
                        )
                        .unwrap();
                        expr_builder.push((precedence::PATH, name));
                        used_slot = true;
                    }
                    }
                }
                if !used_slot {
                    writeln!(
                        writer,
                        "{}let var{} = self.memory.load64({} as usize{}) as i64;",
                        indentation,
                        expr_index,
                        addr_str,
                        if offset != 0 {
                            format!(" + {}", offset)
                        } else {
                            String::new()
                        }
                    )
                    .unwrap();
                    expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                    expr_index += 1;
                }
            }
            F32Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = f32::from_bits(self.memory.load32({} as usize{}));",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            F64Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = f64::from_bits(self.memory.load64({} as usize{}));",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load8S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load8({} as usize{}) as i8 as i32;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load8U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load8({} as usize{}) as i32;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load16S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load16({} as usize{}) as i16 as i32;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load16U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load16({} as usize{}) as i32;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load8S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load8({} as usize{}) as i8 as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load8U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load8({} as usize{}) as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load16S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load16({} as usize{}) as i16 as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load16U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load16({} as usize{}) as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load32S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load32({} as usize{}) as i32 as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load32U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}let var{} = self.memory.load32({} as usize{}) as i64;",
                    indentation,
                    expr_index,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    }
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr_str = addr.to_string();
                let mut used_slot = false;
                if let Some((base, extra)) = parse_stack_addr(&addr_str) {
                    if tainted_bases.contains(&base) && !allow_tainted_slot_lift {
                        used_slot = false;
                    } else {
                    use std::collections::hash_map::Entry;
                    let off = extra.saturating_add(offset as i32);
                    let key = (base.clone(), off);
                    match stack_slots_i32.entry(key) {
                        Entry::Occupied(entry) => {
                            let name = entry.get().clone();
                            writeln!(writer, "{}{} = {} as i32;", indentation, name, value).unwrap();
                        }
                        Entry::Vacant(entry) => {
                            let name = slot_name(&base, off, "i32");
                            entry.insert(name.clone());
                            writeln!(writer, "{}let mut {} = {} as i32;", indentation, name, value).unwrap();
                        }
                    }
                    used_slot = true;
                    }
                }
                if !used_slot {
                    writeln!(
                        writer,
                        "{}self.memory.store32({} as usize{}, {} as u32);",
                        indentation,
                        addr_str,
                        if offset != 0 {
                            format!(" + {}", offset)
                        } else {
                            String::new()
                        },
                        value
                    )
                    .unwrap();
                }
            }
            I64Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr_str = addr.to_string();
                let mut used_slot = false;
                if let Some((base, extra)) = parse_stack_addr(&addr_str) {
                    if tainted_bases.contains(&base) && !allow_tainted_slot_lift {
                        used_slot = false;
                    } else {
                    use std::collections::hash_map::Entry;
                    let off = extra.saturating_add(offset as i32);
                    let key = (base.clone(), off);
                    match stack_slots_i64.entry(key) {
                        Entry::Occupied(entry) => {
                            let name = entry.get().clone();
                            writeln!(writer, "{}{} = {} as i64;", indentation, name, value).unwrap();
                        }
                        Entry::Vacant(entry) => {
                            let name = slot_name(&base, off, "i64");
                            entry.insert(name.clone());
                            writeln!(writer, "{}let mut {} = {} as i64;", indentation, name, value).unwrap();
                        }
                    }
                    used_slot = true;
                    }
                }
                if !used_slot {
                    writeln!(
                        writer,
                        "{}self.memory.store64({} as usize{}, {} as u64);",
                        indentation,
                        addr_str,
                        if offset != 0 {
                            format!(" + {}", offset)
                        } else {
                            String::new()
                        },
                        value
                    )
                    .unwrap();
                }
            }
            F32Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::METHOD_CALL).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store32({} as usize{}, {}.to_bits());",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            F64Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::METHOD_CALL).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store64({} as usize{}, {}.to_bits());",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            I32Store8(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store8({} as usize{}, {} as u8);",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            I32Store16(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store16({} as usize{}, {} as u16);",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            I64Store8(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store8({} as usize{}, {} as u8);",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            I64Store16(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store16({} as usize{}, {} as u16);",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            I64Store32(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
                writeln!(
                    writer,
                    "{}self.memory.store32({} as usize{}, {} as u32);",
                    indentation,
                    addr,
                    if offset != 0 {
                        format!(" + {}", offset)
                    } else {
                        String::new()
                    },
                    value
                )
                .unwrap();
            }
            CurrentMemory(_) => {
                let dst = format!("var{}", expr_index);
                writeln!(writer, "{}let {} = self.memory.size();", indentation, dst).unwrap();
                expr_index += 1;
                expr_builder.push((precedence::PATH, dst));
            }
            GrowMemory(_) => {
                let pages = expr_builder.pop_formatted(precedence::AS).unwrap();
                let dst = format!("var{}", expr_index);
                writeln!(
                    writer,
                    "{}let {} = self.memory.grow({} as usize);",
                    indentation, dst, pages
                )
                .unwrap();
                expr_builder.push((precedence::PATH, dst));
                expr_index += 1;
            }
            I32Const(c) => {
                let precedence = if c < 0 {
                    precedence::UNARY
                } else {
                    precedence::PATH
                };
                expr_builder.push((precedence, format!("{}", c)));
            }
            I64Const(c) => {
                let precedence = if c < 0 {
                    precedence::UNARY
                } else {
                    precedence::PATH
                };
                let fmt = transform_from_soroban_val(c as u64);
                expr_builder.push((precedence, fmt));
            }
            F32Const(c) => {
                expr_builder.push((
                    precedence::FUNCTION_CALL,
                    format!("f32::from_bits({:#X})", c as u32),
                ));
            }
            F64Const(c) => {
                expr_builder.push((
                    precedence::FUNCTION_CALL,
                    format!("f64::from_bits({:#X})", c as u64),
                ));
            }
            I32Eqz => {
                expr_builder.unary_individual(precedence::COMPARISON, precedence::AS, |a| {
                    format!("({} == 0) as i32", a)
                });
            }
            I32Eq => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| {
                        if let Some(mapped) = try_format_tag_compare(&a.to_string(), &b.to_string(), true) {
                            format!("({}) as i32", mapped)
                        } else {
                            format!("({} == {}) as i32", a, b)
                        }
                    },
                );
            }
            I32Ne => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| {
                        if let Some(mapped) = try_format_tag_compare(&a.to_string(), &b.to_string(), false) {
                            format!("({}) as i32", mapped)
                        } else {
                            format!("({} != {}) as i32", a, b)
                        }
                    },
                );
            }
            I32LtS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} < {}) as i32", a, b),
                );
            }
            I32LtU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("(({} as u32) < {} as u32) as i32", a, b)
                });
            }
            I32GtS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} > {}) as i32", a, b),
                );
            }
            I32GtU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32 > {} as u32) as i32", a, b)
                });
            }
            I32LeS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} <= {}) as i32", a, b),
                );
            }
            I32LeU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32 <= {} as u32) as i32", a, b)
                });
            }
            I32GeS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} >= {}) as i32", a, b),
                );
            }
            I32GeU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32 >= {} as u32) as i32", a, b)
                });
            }
            I64Eqz => {
                expr_builder.unary_individual(precedence::COMPARISON, precedence::AS, |a| {
                    format!("({} == 0) as i32", a)
                });
            }
            I64Eq => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| {
                        if let Some(mapped) = try_format_tag_compare(&a.to_string(), &b.to_string(), true) {
                            format!("({}) as i32", mapped)
                        } else {
                            format!("({} == {}) as i32", a, b)
                        }
                    },
                );
            }
            I64Ne => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| {
                        if let Some(mapped) = try_format_tag_compare(&a.to_string(), &b.to_string(), false) {
                            format!("({}) as i32", mapped)
                        } else {
                            format!("({} != {}) as i32", a, b)
                        }
                    },
                );
            }
            I64LtS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} < {}) as i32", a, b),
                );
            }
            I64LtU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("(({} as u64) < {} as u64) as i32", a, b)
                });
            }
            I64GtS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} > {}) as i32", a, b),
                );
            }
            I64GtU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64 > {} as u64) as i32", a, b)
                });
            }
            I64LeS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} <= {}) as i32", a, b),
                );
            }
            I64LeU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64 <= {} as u64) as i32", a, b)
                });
            }
            I64GeS => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} >= {}) as i32", a, b),
                );
            }
            I64GeU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64 >= {} as u64) as i32", a, b)
                });
            }
            F32Eq => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} == {}) as i32", a, b),
                );
            }
            F32Ne => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} != {}) as i32", a, b),
                );
            }
            F32Lt => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} < {}) as i32", a, b),
                );
            }
            F32Gt => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} > {}) as i32", a, b),
                );
            }
            F32Le => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} <= {}) as i32", a, b),
                );
            }
            F32Ge => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} >= {}) as i32", a, b),
                );
            }
            F64Eq => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} == {}) as i32", a, b),
                );
            }
            F64Ne => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} != {}) as i32", a, b),
                );
            }
            F64Lt => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} < {}) as i32", a, b),
                );
            }
            F64Gt => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} > {}) as i32", a, b),
                );
            }
            F64Le => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} <= {}) as i32", a, b),
                );
            }
            F64Ge => {
                expr_builder.binary_individual(
                    precedence::COMPARISON,
                    precedence::COMPARISON,
                    precedence::AS,
                    |a, b| format!("({} >= {}) as i32", a, b),
                );
            }
            I32Clz => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.leading_zeros() as i32", a)
                });
            }
            I32Ctz => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.trailing_zeros() as i32", a)
                });
            }
            I32Popcnt => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.count_ones() as i32", a)
                });
            }
            I32Add => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_add({})", a, b)
                });
            }
            I32Sub => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_sub({})", a, b)
                });
            }
            I32Mul => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_mul({})", a, b)
                });
            }
            I32DivS => {
                expr_builder.binary_lr(precedence::DIV, |a, b| format!("{} / {}", a, b));
            }
            I32DivU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32 / {} as u32) as i32", a, b)
                });
            }
            I32RemS => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_rem({})", a, b)
                });
            }
            I32RemU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32).wrapping_rem({} as u32) as i32", a, b)
                });
            }
            I32And => {
                expr_builder.binary(precedence::BIT_AND, |a, b| format!("{} & {}", a, b));
            }
            I32Or => {
                expr_builder.binary(precedence::BIT_OR, |a, b| format!("{} | {}", a, b));
            }
            I32Xor => {
                expr_builder.binary(precedence::BIT_XOR, |a, b| format!("{} ^ {}", a, b));
            }
            I32Shl => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.wrapping_shl({} as u32)", a, b),
                );
            }
            I32ShrS => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.wrapping_shr({} as u32)", a, b),
                );
            }
            I32ShrU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u32).wrapping_shr({} as u32) as i32", a, b)
                });
            }
            I32Rotl => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.rotate_left({} as u32)", a, b),
                );
            }
            I32Rotr => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.rotate_right({} as u32)", a, b),
                );
            }
            I64Clz => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.leading_zeros() as i64", a)
                });
            }
            I64Ctz => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.trailing_zeros() as i64", a)
                });
            }
            I64Popcnt => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.count_ones() as i64", a)
                });
            }
            I64Add => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_add({})", a, b)
                });
            }
            I64Sub => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_sub({})", a, b)
                });
            }
            I64Mul => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_mul({})", a, b)
                });
            }
            I64DivS => {
                expr_builder.binary_lr(precedence::DIV, |a, b| format!("{} / {}", a, b));
            }
            I64DivU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64 / {} as u64) as i64", a, b)
                });
            }
            I64RemS => {
                expr_builder.method_call_one_arg(precedence::METHOD_CALL, |a, b| {
                    format!("{}.wrapping_rem({})", a, b)
                });
            }
            I64RemU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64).wrapping_rem({} as u64) as i64", a, b)
                });
            }
            I64And => {
                expr_builder.binary(precedence::BIT_AND, |a, b| format!("{} & {}", a, b));
            }
            I64Or => {
                expr_builder.binary(precedence::BIT_OR, |a, b| format!("{} | {}", a, b));
            }
            I64Xor => {
                expr_builder.binary(precedence::BIT_XOR, |a, b| format!("{} ^ {}", a, b));
            }
            I64Shl => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.wrapping_shl({} as u32)", a, b),
                );
            }
            I64ShrS => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.wrapping_shr({} as u32)", a, b),
                );
            }
            I64ShrU => {
                expr_builder.binary(precedence::AS, |a, b| {
                    format!("({} as u64).wrapping_shr({} as u32) as i64", a, b)
                });
            }
            I64Rotl => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.rotate_left({} as u32)", a, b),
                );
            }
            I64Rotr => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::AS,
                    precedence::METHOD_CALL,
                    |a, b| format!("{}.rotate_right({} as u32)", a, b),
                );
            }
            F32Abs => {
                expr_builder.unary_individual(
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a| format!("f32::from_bits({}.to_bits() & 0x7FFF_FFFF)", a),
                );
            }
            F32Neg => {
                expr_builder.unary_individual(
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a| format!("f32::from_bits({}.to_bits() ^ 0x8000_0000)", a),
                );
            }
            F32Ceil => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.ceil()", a));
            }
            F32Floor => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.floor()", a));
            }
            F32Trunc => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.trunc()", a));
            }
            F32Nearest => {
                let (_, val) = expr_builder.pop().unwrap();
                writeln!(
                    writer,
                    "{0}let var{1} = {{
{0}    let val = {2};
{0}    let round = val.round();
{0}    if val.fract().abs() != 0.5 {{
{0}        round
{0}    }} else if round % 2.0 == 1.0 {{
{0}        val.floor()
{0}    }} else if round % 2.0 == -1.0 {{
{0}        val.ceil()
{0}    }} else {{
{0}        round
{0}    }}
{0}}};",
                    indentation, expr_index, val
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            F32Sqrt => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.sqrt()", a));
            }
            F32Add => {
                expr_builder.binary_lr(precedence::ADD, |a, b| format!("{} + {}", a, b));
            }
            F32Sub => {
                expr_builder.binary_lr(precedence::SUB, |a, b| format!("{} - {}", a, b));
            }
            F32Mul => {
                expr_builder.binary_lr(precedence::MUL, |a, b| format!("{} * {}", a, b));
            }
            F32Div => {
                expr_builder.binary_lr(precedence::DIV, |a, b| format!("{} / {}", a, b));
            }
            F32Min => {
                let (_, b) = expr_builder.pop().unwrap();
                let (_, a) = expr_builder.pop().unwrap();
                expr_builder.push((
                        precedence::MAX,
                        format!("{{ let a = {}; let b = {}; if a.is_nan() || b.is_nan() {{ a }} else {{ a.min(b) }} }}", a, b),
                    ));
            }
            F32Max => {
                let (_, b) = expr_builder.pop().unwrap();
                let (_, a) = expr_builder.pop().unwrap();
                expr_builder.push((
                        precedence::MAX,
                        format!("{{ let a = {}; let b = {}; if a.is_nan() || b.is_nan() {{ a }} else {{ a.max(b) }} }}", a, b),
                    ));
            }
            F32Copysign => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a, b| {
                        format!("f32::from_bits(({}.to_bits() & !(1 << 31)) | ({}.to_bits() & (1 << 31)))", a, b)
                    },
                );
            }
            F64Abs => {
                expr_builder.unary_individual(
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a| format!("f64::from_bits({}.to_bits() & 0x7FFF_FFFF_FFFF_FFFF)", a),
                );
            }
            F64Neg => {
                expr_builder.unary_individual(
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a| format!("f64::from_bits({}.to_bits() ^ 0x8000_0000_0000_0000)", a),
                );
            }
            F64Ceil => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.ceil()", a));
            }
            F64Floor => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.floor()", a));
            }
            F64Trunc => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.trunc()", a));
            }
            F64Nearest => {
                let (_, val) = expr_builder.pop().unwrap();
                writeln!(
                    writer,
                    "{0}let var{1} = {{
{0}    let val = {2};
{0}    let round = val.round();
{0}    if val.fract().abs() != 0.5 {{
{0}        round
{0}    }} else if round % 2.0 == 1.0 {{
{0}        val.floor()
{0}    }} else if round % 2.0 == -1.0 {{
{0}        val.ceil()
{0}    }} else {{
{0}        round
{0}    }}
{0}}};",
                    indentation, expr_index, val
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            F64Sqrt => {
                expr_builder.unary(precedence::METHOD_CALL, |a| format!("{}.sqrt()", a));
            }
            F64Add => {
                expr_builder.binary_lr(precedence::ADD, |a, b| format!("{} + {}", a, b));
            }
            F64Sub => {
                expr_builder.binary_lr(precedence::SUB, |a, b| format!("{} - {}", a, b));
            }
            F64Mul => {
                expr_builder.binary_lr(precedence::MUL, |a, b| format!("{} * {}", a, b));
            }
            F64Div => {
                expr_builder.binary_lr(precedence::DIV, |a, b| format!("{} / {}", a, b));
            }
            F64Min => {
                let (_, b) = expr_builder.pop().unwrap();
                let (_, a) = expr_builder.pop().unwrap();
                expr_builder.push((
                        precedence::MAX,
                        format!("{{ let a = {}; let b = {}; if a.is_nan() || b.is_nan() {{ a }} else {{ a.min(b) }} }}", a, b),
                    ));
            }
            F64Max => {
                let (_, b) = expr_builder.pop().unwrap();
                let (_, a) = expr_builder.pop().unwrap();
                expr_builder.push((
                        precedence::MAX,
                        format!("{{ let a = {}; let b = {}; if a.is_nan() || b.is_nan() {{ a }} else {{ a.max(b) }} }}", a, b),
                    ));
            }
            F64Copysign => {
                expr_builder.binary_individual(
                    precedence::METHOD_CALL,
                    precedence::METHOD_CALL,
                    precedence::FUNCTION_CALL,
                    |a, b| {
                        format!("f64::from_bits(({}.to_bits() & !(1 << 63)) | ({}.to_bits() & (1 << 63)))", a, b)
                    },
                );
            }
            I32WrapI64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i32", a));
            }
            I32TruncSF32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i32", a));
            }
            I32TruncUF32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u32 as i32", a));
            }
            I32TruncSF64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i32", a));
            }
            I32TruncUF64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u32 as i32", a));
            }
            I64ExtendSI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i64", a));
            }
            I64ExtendUI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u32 as i64", a));
            }
            I64TruncSF32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i64", a));
            }
            I64TruncUF32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u64 as i64", a));
            }
            I64TruncSF64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as i64", a));
            }
            I64TruncUF64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u64 as i64", a));
            }
            F32ConvertSI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f32", a));
            }
            F32ConvertUI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u32 as f32", a));
            }
            F32ConvertSI64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f32", a));
            }
            F32ConvertUI64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u64 as f32", a));
            }
            F32DemoteF64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f32", a));
            }
            F64ConvertSI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f64", a));
            }
            F64ConvertUI32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u32 as f64", a));
            }
            F64ConvertSI64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f64", a));
            }
            F64ConvertUI64 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as u64 as f64", a));
            }
            F64PromoteF32 => {
                expr_builder.unary(precedence::AS, |a| format!("{} as f64", a));
            }
            I32ReinterpretF32 => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.to_bits() as i32", a)
                });
            }
            I64ReinterpretF64 => {
                expr_builder.unary_individual(precedence::METHOD_CALL, precedence::AS, |a| {
                    format!("{}.to_bits() as i64", a)
                });
            }
            F32ReinterpretI32 => {
                expr_builder.unary_individual(precedence::AS, precedence::FUNCTION_CALL, |a| {
                    format!("f32::from_bits({} as u32)", a)
                });
            }
            F64ReinterpretI64 => {
                expr_builder.unary_individual(precedence::AS, precedence::FUNCTION_CALL, |a| {
                    format!("f64::from_bits({} as u64)", a)
                });
            }
        }
        instr_index += 1;
    }
}

fn emit_br_table_arm<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    index: usize,
    target: Option<&BlockKind>,
) {
    match target {
        Some(BlockKind::Block { label, .. }) | Some(BlockKind::If { label, .. }) => {
            if label.is_some() {
                writeln!(writer, "{}{} => break,", indentation, index).unwrap();
            } else {
                writeln!(writer, "{}{} => break,", indentation, index).unwrap();
            }
        }
        Some(BlockKind::Loop { label, .. }) => {
            if label.is_some() {
                writeln!(writer, "{}{} => continue,", indentation, index).unwrap();
            } else {
                writeln!(writer, "{}{} => continue,", indentation, index).unwrap();
            }
        }
        Some(BlockKind::Function { .. }) | None => {
            writeln!(writer, "{}{} => return,", indentation, index).unwrap();
        }
    }
}

fn emit_br_table_default<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    target: Option<&BlockKind>,
) {
    match target {
        Some(BlockKind::Block { label, .. }) | Some(BlockKind::If { label, .. }) => {
            if label.is_some() {
                writeln!(writer, "{}_ => break,", indentation).unwrap();
            } else {
                writeln!(writer, "{}_ => break,", indentation).unwrap();
            }
        }
        Some(BlockKind::Loop { label, .. }) => {
            if label.is_some() {
                writeln!(writer, "{}_ => continue,", indentation).unwrap();
            } else {
                writeln!(writer, "{}_ => continue,", indentation).unwrap();
            }
        }
        Some(BlockKind::Function { .. }) | None => {
            writeln!(writer, "{}_ => return,", indentation).unwrap();
        }
    }
}
