use crate::decompile::DataSegment;
use crate::semantic_resolver::resolver;

fn storage_match_expr(
    storage_type: &str,
    persistent_expr: &str,
    temporary_expr: &str,
    instance_expr: &str,
) -> String {
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

fn parse_u64_literal(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        s.parse::<u64>().ok()
    } else {
        None
    }
}

fn parse_u32_from_expr_token(expr: &str) -> Option<u32> {
    let mut token = expr.trim();
    token = token.trim_matches(|c| c == '(' || c == ')');
    if let Some((left, _)) = token.split_once(" as ") {
        token = left.trim();
    }
    parse_u32_literal(token)
}

fn decode_packed_linear_u32(s: &str) -> Option<u32> {
    if let Some(v) = parse_u32_literal(s) {
        return Some(v);
    }
    let trimmed = s.trim();
    if let Some((left, _)) = trimmed.split_once(".wrapping_shl(32 as u32)") {
        if let Some(v) = parse_u32_from_expr_token(left) {
            return Some(v);
        }
    }
    let packed = parse_u64_literal(s)?;
    let hi = (packed >> 32) as u32;
    let lo = packed as u32;
    if hi != 0 {
        Some(hi)
    } else {
        Some(lo)
    }
}

fn decode_linear_memory_addr_len(addr_expr: &str, len_expr: &str) -> Option<(u32, u32)> {
    let addr = decode_packed_linear_u32(addr_expr)?;
    let len = decode_packed_linear_u32(len_expr)?;
    Some((addr, len))
}

fn resolve_linear_memory_bytes(
    addr: u32,
    len: u32,
    data_segments: &[DataSegment],
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
    data_segments: &[DataSegment],
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
    data_segments: &[DataSegment],
) -> Option<String> {
    let bytes = resolve_linear_memory_bytes(addr, len, data_segments)?;
    let s = String::from_utf8(bytes).ok()?;
    Some(
        s.replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\"', "\\\""),
    )
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

pub fn map_imported_call(
    name: &str,
    args: &[String],
    data_segments: &[DataSegment],
) -> Option<String> {
    let canonical_name = resolver().canonical_sdk_function_name(name).unwrap_or(name);
    match canonical_name {
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
            if let Some((addr, len)) = decode_linear_memory_addr_len(addr, len) {
                if let Some(vals) = resolve_linear_memory_u64_array(addr, len, data_segments) {
                    return Some(format_val_vec_literal(&vals));
                }
            }
            Some("val_to_i64(Vec::<Val>::new(env).into_val(env))".to_string())
        }
        "vec_unpack_to_linear_memory" => Some("0".to_string()),
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
                decode_packed_linear_u32(keys_addr),
                decode_packed_linear_u32(vals_addr),
                decode_packed_linear_u32(len),
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
            Some("val_to_i64(Map::<Val, Val>::new(env).into_val(env))".to_string())
        }
        "map_unpack_to_linear_memory" => Some("0".to_string()),
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
            if let Some((addr, len)) = decode_linear_memory_addr_len(addr, len) {
                if let Some(s) = resolve_linear_memory_string(addr, len, data_segments) {
                    return Some(format!("val_to_i64(Symbol::new(env, \"{}\"))", s));
                }
            }
            Some("val_to_i64(Symbol::new(env, \"\"))".to_string())
        }
        "symbol_len" => {
            let sym = args.get(0)?;
            Some(format!(
                "Symbol::from_val(env, &val_from_i64({})).len() as i64",
                sym
            ))
        }
        "symbol_copy_to_linear_memory" => Some("0".to_string()),
        "symbol_index_in_linear_memory" => Some("0".to_string()),
        "string_new_from_linear_memory" => {
            let addr = args.get(0)?;
            let len = args.get(1)?;
            if let Some((addr, len)) = decode_linear_memory_addr_len(addr, len) {
                if let Some(s) = resolve_linear_memory_string(addr, len, data_segments) {
                    return Some(format!("val_to_i64(String::from_str(env, \"{}\"))", s));
                }
            }
            Some("val_to_i64(String::from_str(env, \"\"))".to_string())
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
            if let Some((addr, len)) = decode_linear_memory_addr_len(addr, len) {
                if let Some(bytes) = resolve_linear_memory_bytes(addr, len, data_segments) {
                    let literal = format_bytes_literal(&bytes);
                    return Some(format!(
                        "val_to_i64(Bytes::from_slice(env, &{}).into_val(env))",
                        literal
                    ));
                }
            }
            Some("val_to_i64(Bytes::new(env).into_val(env))".to_string())
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
        "bytes_copy_from_linear_memory" => Some("0".to_string()),
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
                Some(storage_match_expr(
                    storage_type,
                    &persistent,
                    &temporary,
                    &instance,
                ))
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
                Some(storage_match_expr(
                    storage_type,
                    &persistent,
                    &temporary,
                    &instance,
                ))
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
                Some(storage_match_expr(
                    storage_type,
                    &persistent,
                    &temporary,
                    &instance,
                ))
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
                let instance =
                    format!("env.storage().instance().remove(&val_from_i64({})); 0", key);
                Some(storage_match_expr(
                    storage_type,
                    &persistent,
                    &temporary,
                    &instance,
                ))
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
            Some(storage_match_expr(
                storage_type,
                &persistent,
                &temporary,
                &instance,
            ))
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
        "get_current_contract_address" => {
            Some("val_to_i64(env.current_contract_address().into_val(env))".to_string())
        }
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
        "bn254_g1_add"
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
        _ => {
            // Fallback: unified semantic resolver (canonicalization + SDK mapping).
            resolver().resolve_sdk_call(canonical_name, args)
        }
    }
}
