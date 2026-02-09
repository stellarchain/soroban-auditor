use crate::decompile::DataSegment;
use crate::fingerprint::Fingerprint;
use crate::wasm_ir::{to_rs_type, Function};
use parity_wasm::elements::{FuncBody, FunctionType, Instruction};
use std::io::Write;

struct RawRewriteRule {
    name: &'static str,
    fingerprint: Fingerprint,
    emit: fn(&mut dyn Write, &Function, &FunctionType, &FuncBody, usize, &[Function], &[DataSegment]) -> Result<(), String>,
}

fn resolve_cstr(addr: u32, data_segments: &[DataSegment]) -> Option<String> {
    for seg in data_segments {
        let start = seg.offset;
        let end = start.saturating_add(seg.data.len() as u32);
        if addr >= start && addr < end {
            let offset = (addr - start) as usize;
            let bytes = &seg.data[offset..];
            let mut len = 0usize;
            while len < bytes.len() {
                let b = bytes[len];
                if b == 0 {
                    break;
                }
                if !(b == b'\n' || b == b'\r' || (0x20..=0x7e).contains(&b)) {
                    return None;
                }
                len += 1;
                if len > 160 {
                    return None;
                }
            }
            if len == 0 {
                return None;
            }
            let s = String::from_utf8(bytes[..len].to_vec()).ok()?;
            return Some(s.replace('\n', "\\n").replace('\r', "\\r").replace('\"', "\\\""));
        }
    }
    None
}

fn format_cstr_comment(addr: u32, data_segments: &[DataSegment]) -> String {
    resolve_cstr(addr, data_segments)
        .map(|s| format!(" // \"{}\"", s))
        .unwrap_or_default()
}

fn collect_i32_consts(body: &FuncBody, min: u32, max: u32) -> Vec<u32> {
    let mut out = std::collections::BTreeSet::new();
    for instr in body.code().elements() {
        if let Instruction::I32Const(v) = instr {
            if *v >= min as i32 && *v <= max as i32 {
                out.insert(*v as u32);
            }
        }
    }
    out.into_iter().collect()
}

fn split_msg_tab_consts(all: &[u32]) -> Option<([u32; 4], [u32; 4])> {
    let mut msgs: Vec<u32> = all.iter().copied().filter(|v| *v < 1_049_700).collect();
    let mut tabs: Vec<u32> = all.iter().copied().filter(|v| *v >= 1_049_700).collect();
    msgs.sort_unstable();
    tabs.sort_unstable();
    if msgs.len() == 4 && tabs.len() == 4 {
        return Some(([msgs[0], msgs[1], msgs[2], msgs[3]], [tabs[0], tabs[1], tabs[2], tabs[3]]));
    }
    None
}

fn emit_func90(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i64" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(writer, "    fn {}(&mut self, _env: &Env, arg0: i32, arg1: i64) {{", function.name)
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let packed = if (arg1 as u64) <= 72057594037927935 {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            arg1").map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            val_to_i64(Val::from_u64(arg1 as u64))").map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(arg0 as usize, 0);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(arg0 as usize + 8, packed as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_func42(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(writer, "    fn {}(&mut self, _env: &Env, dst: i32, src: i32) {{", function.name)
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let tag = self.memory.load64(src as usize) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        if tag != 0 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            if tag as i32 & 1 != 0 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            let hi = self.memory.load64(src as usize + 16) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            let lo = self.memory.load64(src as usize + 8) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(dst as usize + 16, hi as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(dst as usize + 8, lo as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(dst as usize, 1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(dst as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_func43(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(writer, "    fn {}(&mut self, env: &Env, dst: i32, src: i32) {{", function.name)
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let tag = self.memory.load8(src as usize) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut flag: i64 = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut out: i64;").map_err(|e| e.to_string())?;
    writeln!(writer, "        if tag == 27 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            let hi = self.memory.load64(src as usize + 16) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(dst as usize + 16, hi as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            out = self.memory.load64(src as usize + 8) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            flag = 1;").map_err(|e| e.to_string())?;
    writeln!(writer, "            out = self.func41(env, src);").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(dst as usize, flag as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(dst as usize + 8, out as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_get_prices_impl(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i64" || to_rs_type(params[1]) != "i64" {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i64" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, mut feed_ids: i64, mut payload: i64) -> i64 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | (!(Bytes::try_from_val(env, &val_from_i64(payload)).is_ok())) as i32 != 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        let entries_val = match self.decode_entries_from_payload(env, feed_ids, payload) {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            Ok(v) => v,").map_err(|e| e.to_string())?;
    writeln!(writer, "            Err(err) => return err,").map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let entries_vec = Vec::<Val>::from_val(env, &val_from_i64(entries_val));"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let feed_vec = Vec::<Val>::from_val(env, &val_from_i64(feed_ids));"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if let Err(err) = self.require_len_match(&entries_vec, &feed_vec) {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return err;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let entries = entries_vec;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let mut out = Vec::<U256>::new(env);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.for_each_val(env, &entries, |val| {{ let price = U256::from_val(env, &val); out.push_back(price); }});"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let out_vec = val_to_i64(out.into_val(env));"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        out_vec").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

pub fn suggested_name_for_fingerprint(fingerprint: &Fingerprint) -> Option<&'static str> {
    if *fingerprint == FP_ENTRY_DECODE {
        Some("entry_decode")
    } else if *fingerprint == FP_VEC_PAIR_ITER {
        Some("vec_pair_iter")
    } else if *fingerprint == FP_FUNC43 {
        Some("decode_val_or_error")
    } else if *fingerprint == FP_FUNC42 {
        Some("copy_val_if_present")
    } else if *fingerprint == FP_FUNC90 {
        Some("write_ok_val")
    } else if *fingerprint == FP_FUNC36 {
        Some("storage_get_val")
    } else if *fingerprint == FP_FUNC38 {
        Some("map_unpack_to_val")
    } else if *fingerprint == FP_FUNC100 {
        Some("val_to_i64_checked")
    } else if *fingerprint == FP_FUNC94 {
        Some("check_recent_timestamp")
    } else if *fingerprint == FP_FUNC39 {
        Some("storage_set_val")
    } else if *fingerprint == FP_FUNC40 {
        Some("storage_key_from_str")
    } else if *fingerprint == FP_FUNC50 {
        Some("result_unwrap_or_panic")
    } else if *fingerprint == FP_FUNC51 {
        Some("result_from_val_unchecked")
    } else if *fingerprint == FP_FUNC53 {
        Some("result_from_val")
    } else if *fingerprint == FP_FUNC54 {
        Some("require_owner_auth")
    } else if *fingerprint == FP_FUNC55 {
        Some("require_auth_for_key")
    } else if *fingerprint == FP_FUNC93 {
        Some("read_prices_loop")
    } else if *fingerprint == FP_FUNC91 {
        Some("write_prices_impl")
    } else if *fingerprint == FP_FUNC92 {
        Some("map_new_val")
    } else if *fingerprint == FP_FUNC83 {
        Some("init_owner_internal")
    } else if *fingerprint == FP_FUNC84 {
        Some("set_pending_owner_internal")
    } else if *fingerprint == FP_FUNC85 {
        Some("accept_ownership_internal")
    } else if *fingerprint == FP_FUNC86 {
        Some("storage_remove_val")
    } else if *fingerprint == FP_FUNC87 {
        Some("cancel_pending_owner_internal")
    } else if *fingerprint == FP_FUNC88 {
        Some("upgrade_wasm_internal")
    } else if *fingerprint == FP_FUNC95 {
        Some("read_timestamp_val")
    } else if *fingerprint == FP_FUNC96 {
        Some("read_price_data_for_feed_val")
    } else if *fingerprint == FP_FUNC98 {
        Some("check_price_data_val")
    } else if *fingerprint == FP_DECODE_ERROR_FROM_VAL {
        Some("decode_error_from_val")
    } else if *fingerprint == FP_LEDGER_TIMESTAMP_VAL {
        Some("ledger_timestamp_val")
    } else if *fingerprint == FP_BYTES_TO_FIXED32_STRUCT {
        Some("bytes_to_fixed32_struct")
    } else if *fingerprint == FP_GET_PRICES_IMPL {
        Some("get_prices_impl")
    } else if *fingerprint == FP_FUNC127_DISPATCH {
        Some("dispatch_table")
    } else if *fingerprint == FP_ALLOC_TRAP {
        Some("alloc_trap")
    } else if *fingerprint == FP_ALLOC_REALLOC {
        Some("alloc_realloc")
    } else if *fingerprint == FP_ALLOC_SIZE_ALIGN {
        Some("alloc_size_align")
    } else if *fingerprint == FP_ALLOC_CORE {
        Some("alloc_core")
    } else if *fingerprint == FP_BUILD_ENTRY_FROM_BYTES {
        Some("build_entry_from_bytes")
    } else if *fingerprint == FP_ENTRY_MATCH_COPY {
        Some("entry_match_copy")
    } else if *fingerprint == FP_ENTRY_COPY_IF_OK {
        Some("entry_copy_if_ok")
    } else if *fingerprint == FP_ENTRY_FROM_BYTES_VAL {
        Some("entry_from_bytes_val")
    } else if *fingerprint == FP_ALLOC_RANGE_FILL {
        Some("alloc_range_fill")
    } else if *fingerprint == FP_ALLOC_RANGE {
        Some("alloc_range")
    } else if *fingerprint == FP_ALLOC_RANGE_1_1 {
        Some("alloc_range_one")
    } else if *fingerprint == FP_ALLOC_COPY {
        Some("alloc_copy")
    } else if *fingerprint == FP_REQUIRE_ALLOC {
        Some("require_alloc")
    } else if *fingerprint == FP_SPAN_FROM_RANGE {
        Some("span_from_range")
    } else if *fingerprint == FP_SPAN_TAKE {
        Some("span_take")
    } else if *fingerprint == FP_SPAN_SET {
        Some("span_set")
    } else if *fingerprint == FP_MEMCPY_CHECKED {
        Some("memcpy_checked")
    } else if *fingerprint == FP_MEMMOVE {
        Some("memmove")
    } else if *fingerprint == FP_MEMCMP {
        Some("memcmp")
    } else if *fingerprint == FP_MEMCMP_SIGN32 {
        Some("memcmp_sign32")
    } else if *fingerprint == FP_MEMEQ32 {
        Some("memeq32")
    } else if *fingerprint == FP_PTR_INDEX32 {
        Some("ptr_index32")
    } else if *fingerprint == FP_COPY_TO_LINEAR_MEMORY {
        Some("copy_to_linear_memory")
    } else if *fingerprint == FP_VEC_NEXT_STRING_FLAG {
        Some("vec_next_string_flag")
    } else if *fingerprint == FP_GUARD_NONZERO_PTR {
        Some("guard_nonzero_ptr")
    } else if *fingerprint == FP_VEC_NEXT_STRING_TO_LINEAR {
        Some("vec_next_string_to_linear")
    } else if *fingerprint == FP_BYTES_TO_FIXED32 {
        Some("bytes_to_fixed32")
    } else if *fingerprint == FP_PACK_U32_CALL_IMPORT {
        Some("pack_u32_call_import")
    } else if *fingerprint == FP_CALL_EQ_ONE {
        Some("call_eq_one")
    } else if *fingerprint == FP_CALL_UNREACHABLE {
        Some("call_unreachable")
    } else if *fingerprint == FP_UNREACHABLE {
        Some("unreachable_stub")
    } else if *fingerprint == FP_NOP {
        Some("nop")
    } else {
        None
    }
}

fn emit_entry_decode(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i64"
        || to_rs_type(params[2]) != "i64"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    let mut out = String::from(include_str!("entry_decode_template.rs"));
    out = out.replace("{FUNC_NAME}", &function.name);
    write!(writer, "{}", out).map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_decode_error_from_val(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 1 || to_rs_type(params[0]) != "i32" {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i64" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, mut arg0: i32) -> i64 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let tag = mload8!(arg0 as usize) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut code: i32 = match tag {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            1 => 509,").map_err(|e| e.to_string())?;
    writeln!(writer, "            2 => 510,").map_err(|e| e.to_string())?;
    writeln!(writer, "            3 => {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                let kind = (mload32!(arg0 as usize + 4) as i32) ^ -2147483648;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                let val = if (kind as u32) >= 4 || kind == 1 || kind == 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                    mload8!(arg0 as usize + 8) as i32")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                }} else if kind == 2 {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                    mload32!(arg0 as usize + 8) as i32")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                }} else {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                    mload32!(arg0 as usize + 12) as i32")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                }};").map_err(|e| e.to_string())?;
    writeln!(writer, "                let mut code = (val & 65535).wrapping_add(700);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                if (code & 65535) != code {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                code").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            4 => mload8!(arg0 as usize + 16) as i32,")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            5 => 511,").map_err(|e| e.to_string())?;
    writeln!(writer, "            6 => 512,").map_err(|e| e.to_string())?;
    writeln!(writer, "            7 => 520,").map_err(|e| e.to_string())?;
    writeln!(writer, "            8 => {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                let base = 2000i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                let extra = mload32!(arg0 as usize + 8) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                let code = base.wrapping_add(extra);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                if (base as u32) > (code as u32) {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                code").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            9 => {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                let code = (mload16!(arg0 as usize + 4) as i32).wrapping_add(600);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                if (code & 65535) != code {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                code").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            10 => {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                let code = (mload16!(arg0 as usize + 4) as i32).wrapping_add(1000);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                if (code & 65535) != code {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                code").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            11 => 513,").map_err(|e| e.to_string())?;
    writeln!(writer, "            12 => 514,").map_err(|e| e.to_string())?;
    writeln!(writer, "            13 => 515,").map_err(|e| e.to_string())?;
    writeln!(writer, "            14 => 521,").map_err(|e| e.to_string())?;
    writeln!(writer, "            15 => 516,").map_err(|e| e.to_string())?;
    writeln!(writer, "            16 => 517,").map_err(|e| e.to_string())?;
    writeln!(writer, "            17 => 522,").map_err(|e| e.to_string())?;
    writeln!(writer, "            18 => 523,").map_err(|e| e.to_string())?;
    writeln!(writer, "            19 => 518,").map_err(|e| e.to_string())?;
    writeln!(writer, "            20 => 519,").map_err(|e| e.to_string())?;
    writeln!(writer, "            21 => 1101,").map_err(|e| e.to_string())?;
    writeln!(writer, "            22 => 1102,").map_err(|e| e.to_string())?;
    writeln!(writer, "            23 => 1200,").map_err(|e| e.to_string())?;
    writeln!(writer, "            24 => 1300,").map_err(|e| e.to_string())?;
    writeln!(writer, "            25 => 1400,").map_err(|e| e.to_string())?;
    writeln!(writer, "            26 => 1500,").map_err(|e| e.to_string())?;
    writeln!(writer, "            _ => {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                let code = (mload16!(arg0 as usize + 4) as i32).wrapping_add(1050);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "                if (code & 65535) != code {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                code").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        return (code as u32 as i64 & 65535).wrapping_shl(32) | 3 /* Error(Contract, #0) */;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_call_unreachable(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if !fn_type.params().is_empty() || !fn_type.results().is_empty() {
        return Ok(());
    }
    let instrs: Vec<&Instruction> = body
        .code()
        .elements()
        .iter()
        .filter(|i| !matches!(i, Instruction::End))
        .collect();
    if instrs.len() != 2 {
        return Ok(());
    }
    let callee = match instrs[0] {
        Instruction::Call(n) => *n as usize,
        _ => return Ok(()),
    };
    if !matches!(instrs[1], Instruction::Unreachable) {
        return Ok(());
    }
    if callee < import_count {
        return Ok(());
    }
    let callee_name = functions[callee].name.as_str();
    writeln!(writer, "    fn {}(&mut self, env: &Env) {{", function.name)
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.{}(env);", callee_name).map_err(|e| e.to_string())?;
    writeln!(writer, "        unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_nop(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    write!(writer, "    fn {}(&mut self, _env: &Env", function.name).map_err(|e| e.to_string())?;
    for (i, &param) in fn_type.params().iter().enumerate() {
        write!(writer, ", arg{}: {}", i, to_rs_type(param)).map_err(|e| e.to_string())?;
    }
    writeln!(writer, ") {{").map_err(|e| e.to_string())?;
    if !fn_type.params().is_empty() {
        write!(writer, "        let _ = (").map_err(|e| e.to_string())?;
        for i in 0..fn_type.params().len() {
            if i > 0 {
                write!(writer, ", ").map_err(|e| e.to_string())?;
            }
            write!(writer, "arg{}", i).map_err(|e| e.to_string())?;
        }
        writeln!(writer, ");").map_err(|e| e.to_string())?;
    }
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_pack_u32_call_import(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 2 || fn_type.results().len() != 1 {
        return Ok(());
    }
    if to_rs_type(fn_type.params()[0]) != "i32" || to_rs_type(fn_type.params()[1]) != "i32" {
        return Ok(());
    }
    let instrs: Vec<&Instruction> = body
        .code()
        .elements()
        .iter()
        .filter(|i| !matches!(i, Instruction::End))
        .collect();
    if instrs.len() != 13 {
        return Ok(());
    }
    let call_index = match instrs[12] {
        Instruction::Call(n) => *n as usize,
        _ => return Ok(()),
    };
    if call_index >= import_count {
        return Ok(());
    }
    let expect = [
        Instruction::GetLocal(0),
        Instruction::I64ExtendUI32,
        Instruction::I64Const(32),
        Instruction::I64Shl,
        Instruction::I64Const(4),
        Instruction::I64Or,
        Instruction::GetLocal(1),
        Instruction::I64ExtendUI32,
        Instruction::I64Const(32),
        Instruction::I64Shl,
        Instruction::I64Const(4),
        Instruction::I64Or,
    ];
    for (i, exp) in expect.iter().enumerate() {
        if std::mem::discriminant(instrs[i]) != std::mem::discriminant(exp) {
            return Ok(());
        }
        if let (Instruction::GetLocal(a), Instruction::GetLocal(b)) = (instrs[i], exp) {
            if a != b {
                return Ok(());
            }
        }
        if let (Instruction::I64Const(a), Instruction::I64Const(b)) = (instrs[i], exp) {
            if a != b {
                return Ok(());
            }
        }
    }
    let callee = functions[call_index].name.as_str();
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, arg0: i32, arg1: i32) -> i64 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let a = ((arg0 as u64) << 32) | 4;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let b = ((arg1 as u64) << 32) | 4;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.{}(env, a as i64, b as i64)", callee)
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_unreachable(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    write!(writer, "    fn {}(&mut self, _env: &Env", function.name).map_err(|e| e.to_string())?;
    for (i, &param) in fn_type.params().iter().enumerate() {
        write!(writer, ", arg{}: {}", i, to_rs_type(param)).map_err(|e| e.to_string())?;
    }
    writeln!(writer, ") {{").map_err(|e| e.to_string())?;
    if !fn_type.params().is_empty() {
        write!(writer, "        let _ = (").map_err(|e| e.to_string())?;
        for i in 0..fn_type.params().len() {
            if i > 0 {
                write!(writer, ", ").map_err(|e| e.to_string())?;
            }
            write!(writer, "arg{}", i).map_err(|e| e.to_string())?;
        }
        writeln!(writer, ");").map_err(|e| e.to_string())?;
    }
    writeln!(writer, "        unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_call_eq_one(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 2 || fn_type.results().len() != 1 {
        return Ok(());
    }
    if to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    let instrs: Vec<&Instruction> = body
        .code()
        .elements()
        .iter()
        .filter(|i| !matches!(i, Instruction::End))
        .collect();
    if instrs.len() != 5 {
        return Ok(());
    }
    let call_index = match instrs[2] {
        Instruction::Call(n) => *n as usize,
        _ => return Ok(()),
    };
    if call_index >= import_count {
        return Ok(());
    }
    if !matches!(instrs[0], Instruction::GetLocal(0))
        || !matches!(instrs[1], Instruction::GetLocal(1))
        || !matches!(instrs[3], Instruction::I64Const(1))
        || !matches!(instrs[4], Instruction::I64Eq)
    {
        return Ok(());
    }
    let callee = functions[call_index].name.as_str();
    write!(writer, "    fn {}(&mut self, env: &Env", function.name).map_err(|e| e.to_string())?;
    for (i, &param) in fn_type.params().iter().enumerate() {
        write!(writer, ", arg{}: {}", i, to_rs_type(param)).map_err(|e| e.to_string())?;
    }
    writeln!(writer, ") -> i32 {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if self.{}(env, arg0, arg1) == 1 {{ 1 }} else {{ 0 }}",
        callee
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_memmove(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 3 || fn_type.results().len() != 1 {
        return Ok(());
    }
    if to_rs_type(fn_type.params()[0]) != "i32"
        || to_rs_type(fn_type.params()[1]) != "i32"
        || to_rs_type(fn_type.params()[2]) != "i32"
    {
        return Ok(());
    }
    if to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, dst: i32, src: i32, len: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if len <= 0 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            return dst;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        let count = len as u32;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut tmp: Vec<u8> = Vec::with_capacity(count as usize);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut i: u32 = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        while i < count {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            let b = self.memory.load8((src as u32 + i) as usize) as u8;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            tmp.push(b);").map_err(|e| e.to_string())?;
    writeln!(writer, "            i += 1;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        i = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        while i < count {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8((dst as u32 + i) as usize, tmp[i as usize] as u8);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            i += 1;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        dst").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_copy_to_linear_memory(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 3 || !fn_type.results().is_empty() {
        return Ok(());
    }
    if to_rs_type(fn_type.params()[0]) != "i64"
        || to_rs_type(fn_type.params()[1]) != "i32"
        || to_rs_type(fn_type.params()[2]) != "i32"
    {
        return Ok(());
    }
    let instrs: Vec<&Instruction> = body
        .code()
        .elements()
        .iter()
        .filter(|i| !matches!(i, Instruction::End))
        .collect();
    let call_index = match instrs.iter().rev().find_map(|i| {
        if let Instruction::Call(n) = i { Some(*n as usize) } else { None }
    }) {
        Some(idx) => idx,
        None => return Ok(()),
    };
    if call_index >= import_count {
        return Ok(());
    }
    let real = functions[call_index].real_name.map_or("", |v| v);
    let (ty, method) = if real.contains("bytes_copy_to_linear_memory") {
        ("Bytes", "copy_into_slice")
    } else if real.contains("string_copy_to_linear_memory") {
        ("String", "copy_into_slice")
    } else {
        return Ok(());
    };
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, obj: i64, lm_pos: i32, len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let obj = {}::from_val(env, &val_from_i64(obj));",
        ty
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut buf: Vec<u8> = vec![0; len as usize];")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        obj.{}(&mut buf);", method).map_err(|e| e.to_string())?;
    writeln!(writer, "        for (i, b) in buf.iter().enumerate() {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8(lm_pos as usize + i, *b as u8);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_memcmp(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, a: i32, b: i32, len: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut i: i32 = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        while i < len {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let va = self.memory.load8((a + i) as usize) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let vb = self.memory.load8((b + i) as usize) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            if va != vb {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                return va.wrapping_sub(vb);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            i = i.wrapping_add(1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        0").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_memeq32(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, a: i32, b: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        // memeq32").map_err(|e| e.to_string())?;
    writeln!(writer, "        let diff = self.func129(env, a, b, 32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        (diff == 0) as i32").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_memcmp_sign32(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, a: i32, b: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        // memcmp_sign32").map_err(|e| e.to_string())?;
    writeln!(writer, "        let diff = self.func129(env, a, b, 32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        if diff < 0 {{ -1 }} else {{ (diff != 0) as i32 }}")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_ptr_index32(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if fn_type.results().len() != 1 || to_rs_type(fn_type.results()[0]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, base: i32, len: i32, idx: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        // entry_ptr = base + idx * 32").map_err(|e| e.to_string())?;
    writeln!(writer, "        if (len as u32) <= (idx as u32) {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        base.wrapping_add(idx.wrapping_shl(5))")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_span_from_range(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, out_ptr: i32, start: i32, end: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if (start as u32) > (end as u32) {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        let len = end.wrapping_sub(start);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 4, 1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 8, len as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize, len as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_alloc_copy(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, src: i32, len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.func45(env, out_ptr, len, 1, 1);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let dst = self.memory.load32(out_ptr as usize + 4) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let _ = self.func131(env, dst, src, len);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 8, len as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize, self.memory.load32(out_ptr as usize) as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_alloc_range(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 4
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
        || to_rs_type(params[3]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, len: i32, align: i32, fill: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let var4 = self.global0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let tmp = var4.wrapping_sub(16);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = tmp;").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.func49(env, tmp.wrapping_add(4), len, 0, align, fill);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let ptr = self.memory.load32(tmp as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (self.memory.load32(tmp as usize + 4) as i32) == 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let size = self.memory.load32(tmp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(out_ptr as usize + 4, size as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(out_ptr as usize, ptr as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.global0 = tmp.wrapping_add(16);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let size = self.memory.load32(tmp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func48(env, ptr, size);").map_err(|e| e.to_string())?;
    writeln!(writer, "        unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_require_alloc(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 4
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
        || to_rs_type(params[3]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let var5 = self.global0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let tmp = var5.wrapping_sub(16);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = tmp;").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.func47(env, tmp.wrapping_add(8), arg0, arg1, arg2, 1, arg3);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let code = self.memory.load32(tmp as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if code != -2147483647 {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let sz = self.memory.load32(tmp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.func48(env, code, sz);").map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = tmp.wrapping_add(16);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_vec_next_string_flag(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, iter_ptr: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let idx = self.memory.load32(iter_ptr as usize + 8) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let len = self.memory.load32(iter_ptr as usize + 12) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        if (idx as u32) >= (len as u32) {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize, 0);").map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        let vec = self.memory.load64(iter_ptr as usize) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let val = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(vec)).get_unchecked(((idx as u32 as i64).wrapping_shl(32)) | 0));"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let next = idx.wrapping_add(1);").map_err(|e| e.to_string())?;
    writeln!(writer, "        if next == 0 {{ unreachable!(); }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize + 8, val as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(iter_ptr as usize + 8, next as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.memory.store64(out_ptr as usize, (!(String::try_from_val(env, &val_from_i64(val)).is_ok())) as i32 as u32 as i64 as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_guard_nonzero_ptr(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i64"
        || to_rs_type(params[2]) != "i64"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, out_ptr: i32, flag: i64, val: i64) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let ok = if flag != 0 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            if (flag as i32) & 1 != 0 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize + 8, val as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            1").map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else {{ 0 }};").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize, ok as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_vec_next_string_to_linear(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, iter_ptr: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let sp = self.global0.wrapping_sub(48);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let tmp = sp.wrapping_add(16);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func33(env, tmp, iter_ptr);").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let flag = self.memory.load64(tmp as usize) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let val = self.memory.load64(tmp as usize + 8) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func34(env, sp, flag, val);").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (self.memory.load32(sp as usize) as i32) == 1 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let s_val = self.memory.load64(sp as usize + 8) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let len = String::from_val(env, &val_from_i64(s_val)).len() as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            if (len as u64) >= 141733920768 {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let len2 = String::from_val(env, &val_from_i64(s_val)).len() as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            if ((len2 ^ len) as u64) >= 4294967296 {{ unreachable!(); }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.copy_string_to_linear_memory(env, s_val, 0, ((tmp as u32 as i64) << 32) | 0, len & 0x3f00000000 | 0);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            let lm_ptr = self.memory.load64(sp.wrapping_add(40) as usize) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize + 25, lm_ptr as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize + 17, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize + 9, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize + 1, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8(out_ptr as usize, 1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.global0 = sp.wrapping_add(48);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store8(out_ptr as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp.wrapping_add(48);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_bytes_to_fixed32(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, bytes_ref: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let len = self.memory.load32(bytes_ref as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if (len as u32) <= 32 {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let inline = self.memory.load64(bytes_ref as usize) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize, inline as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let l = self.memory.load32(bytes_ref as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store32(out_ptr as usize + 8, l as u32);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let ptr = self.memory.load32(bytes_ref as usize + 4) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let pad = len.wrapping_sub(32);").map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut i: i32 = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "        while i < pad {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            if self.memory.load8((ptr + i) as usize) as i32 != 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "                unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            i = i.wrapping_add(1);").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.func125(env, out_ptr, bytes_ref, pad);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_span_set(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, out_ptr: i32, ptr: i32, len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if (len as u32) >= 33 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 4, len as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize, ptr as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_memcpy_checked(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 4
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
        || to_rs_type(params[3]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, dst: i32, len: i32, src: i32, expected_len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if len != expected_len {{ unreachable!(); }}")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let _ = self.func131(env, dst, src, len);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_alloc_range_1_1(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func49(env, out_ptr.wrapping_add(4), len, 1, 1, 1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let ptr = self.memory.load32(out_ptr as usize + 8) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (self.memory.load32(out_ptr as usize + 4) as i32) == 1 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let size = self.memory.load32(out_ptr as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.func48(env, ptr, size);").map_err(|e| e.to_string())?;
    writeln!(writer, "            unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let size = self.memory.load32(out_ptr as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 8, len as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 4, size as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize, ptr as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_span_take(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, span_ref: i32, len: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let span_len = self.memory.load32(span_ref as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (len as u32) > (span_len as u32) {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8(out_ptr as usize, 26);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (len as u32) < (span_len as u32) {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.func125(env, out_ptr.wrapping_add(4), span_ref, span_len.wrapping_sub(len));"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let ptr = self.memory.load64(span_ref as usize) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(span_ref as usize, 4294967296);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let idx = span_ref.wrapping_add(8);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let p = self.memory.load32(idx as usize) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(idx as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize + 4, ptr as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store8(out_ptr as usize, 27);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(out_ptr as usize + 12, p as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_build_entry_from_bytes(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, bytes_ref: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let sp = self.global0.wrapping_sub(64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp;").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func79(env, sp.wrapping_add(20), bytes_ref);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let len = self.memory.load32(sp as usize + 28) as i32;")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.func80(env, sp.wrapping_add(8), sp.wrapping_add(32), len);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let dst = self.memory.load32(sp as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let dst_len = self.memory.load32(sp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let src = self.memory.load32(sp as usize + 24) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func81(env, dst, dst_len, src, len);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let hi = self.memory.load64(sp as usize + 56) as i64;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.memory.store64(out_ptr as usize + 24, hi as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.memory.store64(out_ptr as usize + 16, 0);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        self.memory.store64(out_ptr as usize + 8, 0);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp.wrapping_add(64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_entry_match_copy(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 4
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
        || to_rs_type(params[3]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, entry_ptr: i32, a: i32, b: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let ok = (self.func121(env, entry_ptr, a) != 0 && self.func122(env, entry_ptr, b) != 0)\n            || (self.func121(env, entry_ptr, b) != 0 && self.func122(env, entry_ptr, a) != 0);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        if ok {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr as usize + 1, self.memory.load64(entry_ptr as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr as usize + 25, self.memory.load64(entry_ptr.wrapping_add(24) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr as usize + 17, self.memory.load64(entry_ptr.wrapping_add(16) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr as usize + 9, self.memory.load64(entry_ptr.wrapping_add(8) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8(out_ptr as usize, 1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store8(out_ptr as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_entry_copy_if_ok(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i32" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, _env: &Env, out_ptr: i32, entry_ref: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (self.memory.load8(entry_ref as usize) as i32) == 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr as usize, self.memory.load64(entry_ref as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr.wrapping_add(24) as usize, self.memory.load64(entry_ref.wrapping_add(25) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr.wrapping_add(16) as usize, self.memory.load64(entry_ref.wrapping_add(17) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.store64(out_ptr.wrapping_add(8) as usize, self.memory.load64(entry_ref.wrapping_add(9) as usize) as u64);"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_entry_from_bytes_val(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 2 || to_rs_type(params[0]) != "i32" || to_rs_type(params[1]) != "i64" {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, bytes_val: i64) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let sp = self.global0.wrapping_sub(32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp;").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func64(env, bytes_val, sp, 32);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr.wrapping_add(24) as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr.wrapping_add(16) as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr.wrapping_add(8) as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize, 0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize + 32, bytes_val as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp.wrapping_add(32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_alloc_range_fill(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    let params = fn_type.params();
    if params.len() != 3
        || to_rs_type(params[0]) != "i32"
        || to_rs_type(params[1]) != "i32"
        || to_rs_type(params[2]) != "i32"
    {
        return Ok(());
    }
    if !fn_type.results().is_empty() {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, len: i32, fill: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let sp = self.global0.wrapping_sub(16);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = sp;").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func49(env, sp.wrapping_add(4), len, 0, 1, fill);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let ptr = self.memory.load32(sp as usize + 8) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        if (self.memory.load32(sp as usize + 4) as i32) == 0 {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let size = self.memory.load32(sp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(out_ptr as usize + 4, size as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(out_ptr as usize, ptr as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.global0 = sp.wrapping_add(16);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "        let size = self.memory.load32(sp as usize + 12) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.func48(env, ptr, size);").map_err(|e| e.to_string())?;
    writeln!(writer, "        unreachable!();").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_vec_pair_iter(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    _body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    _data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 2 || fn_type.results().len() != 0 {
        return Ok(());
    }
    if to_rs_type(fn_type.params()[0]) != "i32" || to_rs_type(fn_type.params()[1]) != "i32" {
        return Ok(());
    }
    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, out_ptr: i32, iter_ptr: i32) {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        let idx = self.memory.load32(iter_ptr as usize + 8) as u32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let len = self.memory.load32(iter_ptr as usize + 12) as u32;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        if idx >= len {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(out_ptr as usize, 0 /* Void */ as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            return;").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        let vec_val = self.memory.load64(iter_ptr as usize) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let elem = Vec::<Val>::from_val(env, &val_from_i64(vec_val))")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            .get_unchecked((idx as i64) << 32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut is_err: i64 = 1;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut err: i64 = Error(Value, UnexpectedType);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut value: i64 = val_to_i64(elem);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        if Vec::<Val>::try_from_val(env, &val_from_i64(value)).is_ok() {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            let inner = Vec::<Val>::from_val(env, &val_from_i64(value));")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            if inner.len() == 2 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                let key = inner.get_unchecked(0);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                let val = inner.get_unchecked(1);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                let key_i64 = val_to_i64(key);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                if String::try_from_val(env, &val_from_i64(key_i64)).is_ok() {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                    let val_i64 = val_to_i64(val);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                    let tag = (val_i64 as i32) & 255;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                    if tag == 12 || tag == 70 {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "                        is_err = 0;").map_err(|e| e.to_string())?;
    writeln!(writer, "                        err = key_i64;").map_err(|e| e.to_string())?;
    writeln!(writer, "                        value = val_i64;").map_err(|e| e.to_string())?;
    writeln!(writer, "                    }}").map_err(|e| e.to_string())?;
    writeln!(writer, "                }}").map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize + 16, value as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize + 8, err as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store64(out_ptr as usize, is_err as u64);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        self.memory.store32(iter_ptr as usize + 8, (idx + 1) as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_func127_dispatch(
    writer: &mut dyn Write,
    function: &Function,
    fn_type: &FunctionType,
    body: &FuncBody,
    _import_count: usize,
    _functions: &[Function],
    data_segments: &[DataSegment],
) -> Result<(), String> {
    if fn_type.params().len() != 2 || fn_type.results().len() != 1 {
        return Ok(());
    }
    if to_rs_type(fn_type.params()[0]) != "i32"
        || to_rs_type(fn_type.params()[1]) != "i32"
        || to_rs_type(fn_type.results()[0]) != "i32"
    {
        return Ok(());
    }

    let defaults_msg = [1_049_600u32, 1_049_628u32, 1_049_660u32, 1_049_684u32];
    let defaults_tab = [1_049_708u32, 1_049_748u32, 1_049_788u32, 1_049_828u32];
    let consts = collect_i32_consts(body, 1_049_000, 1_050_500);
    let (msgs, tabs) = split_msg_tab_consts(&consts).unwrap_or((defaults_msg, defaults_tab));
    let msg_a = format_cstr_comment(msgs[0], data_segments);
    let msg_b = format_cstr_comment(msgs[1], data_segments);
    let msg_c = format_cstr_comment(msgs[2], data_segments);
    let msg_d = format_cstr_comment(msgs[3], data_segments);

    writeln!(
        writer,
        "    fn {}(&mut self, env: &Env, arg0: i32, arg1: i32) -> i32 {{",
        function.name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        const MSG_A: u32 = {};{}",
        msgs[0],
        msg_a).map_err(|e| e.to_string())?;
    writeln!(writer, "        const MSG_B: u32 = {};{}",
        msgs[1],
        msg_b).map_err(|e| e.to_string())?;
    writeln!(writer, "        const MSG_C: u32 = {};{}",
        msgs[2],
        msg_c).map_err(|e| e.to_string())?;
    writeln!(writer, "        const MSG_D: u32 = {};{}",
        msgs[3],
        msg_d).map_err(|e| e.to_string())?;
    writeln!(writer, "        const TAB_A: u32 = {};", tabs[0]).map_err(|e| e.to_string())?;
    writeln!(writer, "        const TAB_B: u32 = {};", tabs[1]).map_err(|e| e.to_string())?;
    writeln!(writer, "        const TAB_C: u32 = {};", tabs[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "        const TAB_D: u32 = {};", tabs[3]).map_err(|e| e.to_string())?;
    writeln!(writer, "        let raw = self.memory.load64(arg0 as usize) as i64;")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        let tag = raw as i32;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let idx = ((tag as u32) >> 8) as i32;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let hi = (raw as u64 >> 32) as i32;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let stack = self.global0.wrapping_sub(64);").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = stack;").map_err(|e| e.to_string())?;
    writeln!(writer, "        let load_tab = |base: u32, idx: i32| -> i32 {{")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            self.memory.load32((base + ((idx as u32) << 2)) as usize) as i32"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(writer, "        let mut write_frame = |msg_ptr: u32, flag60: i32, flag52: i32, a: Option<i32>, b: Option<i32>, slot56: u32, slot48: u32| -> i32 {{")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 28, 3);").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 24, msg_ptr);").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store64(stack as usize + 36, 2);").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 60, flag60 as u32);").map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 52, flag52 as u32);").map_err(|e| e.to_string())?;
    writeln!(writer, "            if let Some(v) = a {{ self.memory.store32(stack as usize + 20, v as u32); }}")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            if let Some(v) = b {{ self.memory.store32(stack as usize + 16, v as u32); }}")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 32, (stack + 48) as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 56, (stack + slot56) as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.memory.store32(stack as usize + 48, (stack + slot48) as u32);")
        .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let a0 = self.memory.load32(arg1 as usize + 28) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "            let a1 = self.memory.load32(arg1 as usize + 32) as i32;"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "            self.func128(env, a0, a1, stack + 24)").map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(writer, "        let res = if (tag as u32) <= 2559 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            if (tag as u32) < 256 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                write_frame(MSG_B, 2, 1, Some(load_tab(TAB_C, idx)), Some(load_tab(TAB_D, idx)), 4, 16)")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            }} else if (raw as u64) < 42949672960 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                write_frame(MSG_A, 1, 1, Some(load_tab(TAB_A, hi)), Some(load_tab(TAB_B, hi)), 16, 8)")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            }} else {{").map_err(|e| e.to_string())?;
    writeln!(writer, "                write_frame(MSG_B, 2, 1, Some(load_tab(TAB_C, idx)), Some(load_tab(TAB_D, idx)), 4, 16)")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "            }}").map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else if (raw as u64) < 42949672960 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            write_frame(MSG_C, 1, 2, Some(load_tab(TAB_A, hi)), Some(load_tab(TAB_B, hi)), 16, 0)")
        .map_err(|e| e.to_string())?;
    writeln!(writer, "        }} else {{").map_err(|e| e.to_string())?;
    writeln!(writer, "            write_frame(MSG_D, 2, 2, None, None, 4, 0)").map_err(|e| e.to_string())?;
    writeln!(writer, "        }};").map_err(|e| e.to_string())?;
    writeln!(writer, "        self.global0 = stack.wrapping_add(64);").map_err(|e| e.to_string())?;
    writeln!(writer, "        res").map_err(|e| e.to_string())?;
    writeln!(writer, "    }}").map_err(|e| e.to_string())?;
    Ok(())
}

// TODO: add more fingerprints as we catalogue patterns.
const FP_FUNC42: Fingerprint = Fingerprint {
    hash: 0xdb7c15da4a0100b1,
    instrs: 39,
    locals: 2,
    calls_import: 0,
    calls_internal: 0,
    memops: 6,
    blocks: 2,
};
const FP_FUNC43: Fingerprint = Fingerprint {
    hash: 0x879c5f6c8e7ccaf7,
    instrs: 27,
    locals: 2,
    calls_import: 0,
    calls_internal: 1,
    memops: 6,
    blocks: 2,
};
const FP_FUNC38: Fingerprint = Fingerprint {
    hash: 0xa2a432aa2d1fc87d,
    instrs: 120,
    locals: 4,
    calls_import: 1,
    calls_internal: 2,
    memops: 16,
    blocks: 7,
};
const FP_FUNC39: Fingerprint = Fingerprint {
    hash: 0xcb8bc1b0363e5c87,
    instrs: 8,
    locals: 0,
    calls_import: 1,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_FUNC40: Fingerprint = Fingerprint {
    hash: 0x0bba25866190612f,
    instrs: 4,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_FUNC100: Fingerprint = Fingerprint {
    hash: 0xb6b622399f6cd662,
    instrs: 34,
    locals: 2,
    calls_import: 1,
    calls_internal: 0,
    memops: 2,
    blocks: 3,
};
const FP_FUNC36: Fingerprint = Fingerprint {
    hash: 0x7123f936ad42eb61,
    instrs: 54,
    locals: 1,
    calls_import: 1,
    calls_internal: 2,
    memops: 8,
    blocks: 2,
};
const FP_ENTRY_DECODE: Fingerprint = Fingerprint {
    hash: 0xb1d5ab9d54552386,
    instrs: 4466,
    locals: 46,
    calls_import: 10,
    calls_internal: 104,
    memops: 696,
    blocks: 152,
};
const FP_FUNC90: Fingerprint = Fingerprint {
    hash: 0x4dc2a27373875c29,
    instrs: 21,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 2,
    blocks: 1,
};
const FP_FUNC94: Fingerprint = Fingerprint {
    hash: 0x0b46848e2c70879d,
    instrs: 70,
    locals: 4,
    calls_import: 0,
    calls_internal: 2,
    memops: 13,
    blocks: 2,
};
const FP_FUNC50: Fingerprint = Fingerprint {
    hash: 0x9d0bda6242cf05dd,
    instrs: 22,
    locals: 2,
    calls_import: 0,
    calls_internal: 1,
    memops: 2,
    blocks: 1,
};
const FP_FUNC51: Fingerprint = Fingerprint {
    hash: 0xd9b6ed776a84028c,
    instrs: 65,
    locals: 4,
    calls_import: 0,
    calls_internal: 3,
    memops: 13,
    blocks: 2,
};
const FP_FUNC53: Fingerprint = Fingerprint {
    hash: 0xa1ffe0a52367d6e1,
    instrs: 35,
    locals: 3,
    calls_import: 0,
    calls_internal: 1,
    memops: 4,
    blocks: 3,
};
const FP_FUNC54: Fingerprint = Fingerprint {
    hash: 0x937cf98725c5ce93,
    instrs: 5,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_FUNC55: Fingerprint = Fingerprint {
    hash: 0x9a262c0072f8c409,
    instrs: 38,
    locals: 1,
    calls_import: 2,
    calls_internal: 2,
    memops: 2,
    blocks: 3,
};
const FP_FUNC93: Fingerprint = Fingerprint {
    hash: 0x4272f3bcef47b5f6,
    instrs: 116,
    locals: 6,
    calls_import: 3,
    calls_internal: 4,
    memops: 16,
    blocks: 5,
};
const FP_FUNC91: Fingerprint = Fingerprint {
    hash: 0x9e5b7725c3501de7,
    instrs: 296,
    locals: 9,
    calls_import: 11,
    calls_internal: 11,
    memops: 26,
    blocks: 12,
};
const FP_FUNC92: Fingerprint = Fingerprint {
    hash: 0xda81fcbe9ede3f70,
    instrs: 26,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 0,
    blocks: 1,
};
const FP_FUNC83: Fingerprint = Fingerprint {
    hash: 0x7896ef8f57401c71,
    instrs: 24,
    locals: 0,
    calls_import: 0,
    calls_internal: 3,
    memops: 0,
    blocks: 2,
};
const FP_FUNC84: Fingerprint = Fingerprint {
    hash: 0xfe1f3f06a749dd3c,
    instrs: 35,
    locals: 1,
    calls_import: 0,
    calls_internal: 2,
    memops: 2,
    blocks: 3,
};
const FP_FUNC85: Fingerprint = Fingerprint {
    hash: 0x65894f641b4e7637,
    instrs: 33,
    locals: 2,
    calls_import: 0,
    calls_internal: 4,
    memops: 2,
    blocks: 1,
};
const FP_FUNC86: Fingerprint = Fingerprint {
    hash: 0x99ee79378e08c33d,
    instrs: 5,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_FUNC87: Fingerprint = Fingerprint {
    hash: 0x950c57d29576441f,
    instrs: 26,
    locals: 2,
    calls_import: 0,
    calls_internal: 3,
    memops: 2,
    blocks: 2,
};
const FP_FUNC88: Fingerprint = Fingerprint {
    hash: 0x10e1cafa5cc73328,
    instrs: 42,
    locals: 1,
    calls_import: 2,
    calls_internal: 1,
    memops: 2,
    blocks: 3,
};
const FP_FUNC95: Fingerprint = Fingerprint {
    hash: 0x30fb0e969eaddad1,
    instrs: 72,
    locals: 2,
    calls_import: 0,
    calls_internal: 3,
    memops: 11,
    blocks: 4,
};
const FP_FUNC96: Fingerprint = Fingerprint {
    hash: 0xc01829002e3dc9f7,
    instrs: 66,
    locals: 1,
    calls_import: 0,
    calls_internal: 3,
    memops: 9,
    blocks: 3,
};
const FP_FUNC98: Fingerprint = Fingerprint {
    hash: 0xe2f2ba431b096cb6,
    instrs: 52,
    locals: 2,
    calls_import: 0,
    calls_internal: 3,
    memops: 7,
    blocks: 1,
};
const FP_DECODE_ERROR_FROM_VAL: Fingerprint = Fingerprint {
    hash: 0x647b009204d6332d,
    instrs: 243,
    locals: 3,
    calls_import: 0,
    calls_internal: 0,
    memops: 11,
    blocks: 32,
};
const FP_LEDGER_TIMESTAMP_VAL: Fingerprint = Fingerprint {
    hash: 0x1f151b05224cb69f,
    instrs: 47,
    locals: 3,
    calls_import: 2,
    calls_internal: 1,
    memops: 2,
    blocks: 3,
};
const FP_BYTES_TO_FIXED32_STRUCT: Fingerprint = Fingerprint {
    hash: 0x5061910c5e571ce1,
    instrs: 86,
    locals: 4,
    calls_import: 0,
    calls_internal: 3,
    memops: 16,
    blocks: 1,
};
const FP_GET_PRICES_IMPL: Fingerprint = Fingerprint {
    hash: 0x00b4ff08e04e697f,
    instrs: 119,
    locals: 3,
    calls_import: 5,
    calls_internal: 6,
    memops: 12,
    blocks: 5,
};
const FP_ALLOC_TRAP: Fingerprint = Fingerprint {
    hash: 0x9acb1bdbb4f5953e,
    instrs: 7,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 0,
    blocks: 1,
};
const FP_ALLOC_REALLOC: Fingerprint = Fingerprint {
    hash: 0x0b022d3cb5c62c80,
    instrs: 32,
    locals: 1,
    calls_import: 0,
    calls_internal: 2,
    memops: 3,
    blocks: 1,
};
const FP_ALLOC_SIZE_ALIGN: Fingerprint = Fingerprint {
    hash: 0x07026fc0ea266fa7,
    instrs: 162,
    locals: 7,
    calls_import: 0,
    calls_internal: 5,
    memops: 11,
    blocks: 8,
};
const FP_ALLOC_CORE: Fingerprint = Fingerprint {
    hash: 0xa925555186d08445,
    instrs: 332,
    locals: 4,
    calls_import: 0,
    calls_internal: 3,
    memops: 31,
    blocks: 20,
};
const FP_CALL_UNREACHABLE: Fingerprint = Fingerprint {
    hash: 0x364ac65856297d45,
    instrs: 3,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_NOP: Fingerprint = Fingerprint {
    hash: 0x0db294ab5049420d,
    instrs: 2,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_PACK_U32_CALL_IMPORT: Fingerprint = Fingerprint {
    hash: 0x256576a6ea8f6e64,
    instrs: 14,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_UNREACHABLE: Fingerprint = Fingerprint {
    hash: 0x953734e6ccf68b58,
    instrs: 2,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_CALL_EQ_ONE: Fingerprint = Fingerprint {
    hash: 0x653f3babe0f53681,
    instrs: 6,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_MEMMOVE: Fingerprint = Fingerprint {
    hash: 0xf5e1308c33e7f13d,
    instrs: 350,
    locals: 8,
    calls_import: 0,
    calls_internal: 0,
    memops: 41,
    blocks: 14,
};
const FP_FUNC127_DISPATCH: Fingerprint = Fingerprint {
    hash: 0x86b014f96e71aec5,
    instrs: 313,
    locals: 4,
    calls_import: 0,
    calls_internal: 5,
    memops: 73,
    blocks: 5,
};
const FP_VEC_PAIR_ITER: Fingerprint = Fingerprint {
    hash: 0x1009eee0145251ab,
    instrs: 121,
    locals: 6,
    calls_import: 2,
    calls_internal: 0,
    memops: 11,
    blocks: 7,
};
const FP_COPY_TO_LINEAR_MEMORY: Fingerprint = Fingerprint {
    hash: 0x9050d9a54155ed34,
    instrs: 17,
    locals: 0,
    calls_import: 1,
    calls_internal: 0,
    memops: 0,
    blocks: 0,
};
const FP_MEMCMP: Fingerprint = Fingerprint {
    hash: 0x3cc5936dca5012a7,
    instrs: 36,
    locals: 3,
    calls_import: 0,
    calls_internal: 0,
    memops: 2,
    blocks: 3,
};
const FP_MEMEQ32: Fingerprint = Fingerprint {
    hash: 0x2562423c21425ae1,
    instrs: 6,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_MEMCMP_SIGN32: Fingerprint = Fingerprint {
    hash: 0xca687ee8429aafd5,
    instrs: 13,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 0,
};
const FP_PTR_INDEX32: Fingerprint = Fingerprint {
    hash: 0xb7569875fc80cae4,
    instrs: 12,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 0,
    blocks: 1,
};
const FP_SPAN_FROM_RANGE: Fingerprint = Fingerprint {
    hash: 0x24749d211cc73413,
    instrs: 19,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 3,
    blocks: 1,
};
const FP_ALLOC_COPY: Fingerprint = Fingerprint {
    hash: 0xec0e4fe013b81985,
    instrs: 35,
    locals: 2,
    calls_import: 0,
    calls_internal: 2,
    memops: 5,
    blocks: 0,
};
const FP_ALLOC_RANGE: Fingerprint = Fingerprint {
    hash: 0x81246b72c9f87605,
    instrs: 39,
    locals: 1,
    calls_import: 0,
    calls_internal: 2,
    memops: 6,
    blocks: 1,
};
const FP_REQUIRE_ALLOC: Fingerprint = Fingerprint {
    hash: 0xf7d64f41080eec9b,
    instrs: 31,
    locals: 1,
    calls_import: 0,
    calls_internal: 2,
    memops: 2,
    blocks: 1,
};
const FP_VEC_NEXT_STRING_FLAG: Fingerprint = Fingerprint {
    hash: 0x62de53692a71ca0a,
    instrs: 45,
    locals: 2,
    calls_import: 1,
    calls_internal: 0,
    memops: 7,
    blocks: 2,
};
const FP_GUARD_NONZERO_PTR: Fingerprint = Fingerprint {
    hash: 0x6154e70a5ff920c0,
    instrs: 32,
    locals: 1,
    calls_import: 0,
    calls_internal: 0,
    memops: 2,
    blocks: 2,
};
const FP_VEC_NEXT_STRING_TO_LINEAR: Fingerprint = Fingerprint {
    hash: 0x0974f7c88ff97554,
    instrs: 114,
    locals: 6,
    calls_import: 3,
    calls_internal: 2,
    memops: 17,
    blocks: 3,
};
const FP_BYTES_TO_FIXED32: Fingerprint = Fingerprint {
    hash: 0x6a35c4e8ca5e1033,
    instrs: 53,
    locals: 4,
    calls_import: 0,
    calls_internal: 1,
    memops: 7,
    blocks: 3,
};
const FP_SPAN_SET: Fingerprint = Fingerprint {
    hash: 0x10d6fbb841cd9d43,
    instrs: 13,
    locals: 0,
    calls_import: 0,
    calls_internal: 0,
    memops: 2,
    blocks: 1,
};
const FP_MEMCPY_CHECKED: Fingerprint = Fingerprint {
    hash: 0x1847b78abdb23830,
    instrs: 12,
    locals: 0,
    calls_import: 0,
    calls_internal: 1,
    memops: 0,
    blocks: 1,
};
const FP_ALLOC_RANGE_1_1: Fingerprint = Fingerprint {
    hash: 0x263e051d2f5ebb27,
    instrs: 44,
    locals: 3,
    calls_import: 0,
    calls_internal: 2,
    memops: 7,
    blocks: 1,
};
const FP_SPAN_TAKE: Fingerprint = Fingerprint {
    hash: 0x12d3e998ce20e0f3,
    instrs: 86,
    locals: 3,
    calls_import: 0,
    calls_internal: 1,
    memops: 13,
    blocks: 6,
};
const FP_BUILD_ENTRY_FROM_BYTES: Fingerprint = Fingerprint {
    hash: 0x07699f75c970dbb6,
    instrs: 76,
    locals: 4,
    calls_import: 0,
    calls_internal: 3,
    memops: 16,
    blocks: 0,
};
const FP_ENTRY_MATCH_COPY: Fingerprint = Fingerprint {
    hash: 0x627d5b4ca0d455a5,
    instrs: 57,
    locals: 1,
    calls_import: 0,
    calls_internal: 4,
    memops: 9,
    blocks: 3,
};
const FP_ENTRY_COPY_IF_OK: Fingerprint = Fingerprint {
    hash: 0x0452222ee75faff8,
    instrs: 45,
    locals: 1,
    calls_import: 0,
    calls_internal: 0,
    memops: 9,
    blocks: 1,
};
const FP_ENTRY_FROM_BYTES_VAL: Fingerprint = Fingerprint {
    hash: 0x3bcac7da12f21925,
    instrs: 60,
    locals: 4,
    calls_import: 0,
    calls_internal: 1,
    memops: 13,
    blocks: 0,
};
const FP_ALLOC_RANGE_FILL: Fingerprint = Fingerprint {
    hash: 0xc5d0fa81f6ec8912,
    instrs: 39,
    locals: 1,
    calls_import: 0,
    calls_internal: 2,
    memops: 6,
    blocks: 1,
};

const RAW_REWRITES: &[RawRewriteRule] = &[
    RawRewriteRule {
        name: "func90_pack_val",
        fingerprint: FP_FUNC90,
        emit: emit_func90,
    },
    RawRewriteRule {
        name: "func42_opt_clone",
        fingerprint: FP_FUNC42,
        emit: emit_func42,
    },
    RawRewriteRule {
        name: "func43_opt_parse",
        fingerprint: FP_FUNC43,
        emit: emit_func43,
    },
    RawRewriteRule {
        name: "entry_decode",
        fingerprint: FP_ENTRY_DECODE,
        emit: emit_entry_decode,
    },
    RawRewriteRule {
        name: "decode_error_from_val",
        fingerprint: FP_DECODE_ERROR_FROM_VAL,
        emit: emit_decode_error_from_val,
    },
    RawRewriteRule {
        name: "call_unreachable",
        fingerprint: FP_CALL_UNREACHABLE,
        emit: emit_call_unreachable,
    },
    RawRewriteRule {
        name: "nop",
        fingerprint: FP_NOP,
        emit: emit_nop,
    },
    RawRewriteRule {
        name: "pack_u32_call_import",
        fingerprint: FP_PACK_U32_CALL_IMPORT,
        emit: emit_pack_u32_call_import,
    },
    RawRewriteRule {
        name: "unreachable",
        fingerprint: FP_UNREACHABLE,
        emit: emit_unreachable,
    },
    RawRewriteRule {
        name: "call_eq_one",
        fingerprint: FP_CALL_EQ_ONE,
        emit: emit_call_eq_one,
    },
    RawRewriteRule {
        name: "memmove",
        fingerprint: FP_MEMMOVE,
        emit: emit_memmove,
    },
    RawRewriteRule {
        name: "func127_dispatch",
        fingerprint: FP_FUNC127_DISPATCH,
        emit: emit_func127_dispatch,
    },
    RawRewriteRule {
        name: "vec_pair_iter",
        fingerprint: FP_VEC_PAIR_ITER,
        emit: emit_vec_pair_iter,
    },
    RawRewriteRule {
        name: "get_prices_impl",
        fingerprint: FP_GET_PRICES_IMPL,
        emit: emit_get_prices_impl,
    },
    RawRewriteRule {
        name: "copy_to_linear_memory",
        fingerprint: FP_COPY_TO_LINEAR_MEMORY,
        emit: emit_copy_to_linear_memory,
    },
    RawRewriteRule {
        name: "memcmp",
        fingerprint: FP_MEMCMP,
        emit: emit_memcmp,
    },
    RawRewriteRule {
        name: "memeq32",
        fingerprint: FP_MEMEQ32,
        emit: emit_memeq32,
    },
    RawRewriteRule {
        name: "memcmp_sign32",
        fingerprint: FP_MEMCMP_SIGN32,
        emit: emit_memcmp_sign32,
    },
    RawRewriteRule {
        name: "ptr_index32",
        fingerprint: FP_PTR_INDEX32,
        emit: emit_ptr_index32,
    },
    RawRewriteRule {
        name: "span_from_range",
        fingerprint: FP_SPAN_FROM_RANGE,
        emit: emit_span_from_range,
    },
    RawRewriteRule {
        name: "alloc_copy",
        fingerprint: FP_ALLOC_COPY,
        emit: emit_alloc_copy,
    },
    RawRewriteRule {
        name: "alloc_range",
        fingerprint: FP_ALLOC_RANGE,
        emit: emit_alloc_range,
    },
    RawRewriteRule {
        name: "require_alloc",
        fingerprint: FP_REQUIRE_ALLOC,
        emit: emit_require_alloc,
    },
    RawRewriteRule {
        name: "vec_next_string_flag",
        fingerprint: FP_VEC_NEXT_STRING_FLAG,
        emit: emit_vec_next_string_flag,
    },
    RawRewriteRule {
        name: "guard_nonzero_ptr",
        fingerprint: FP_GUARD_NONZERO_PTR,
        emit: emit_guard_nonzero_ptr,
    },
    RawRewriteRule {
        name: "vec_next_string_to_linear",
        fingerprint: FP_VEC_NEXT_STRING_TO_LINEAR,
        emit: emit_vec_next_string_to_linear,
    },
    RawRewriteRule {
        name: "bytes_to_fixed32",
        fingerprint: FP_BYTES_TO_FIXED32,
        emit: emit_bytes_to_fixed32,
    },
    RawRewriteRule {
        name: "span_set",
        fingerprint: FP_SPAN_SET,
        emit: emit_span_set,
    },
    RawRewriteRule {
        name: "memcpy_checked",
        fingerprint: FP_MEMCPY_CHECKED,
        emit: emit_memcpy_checked,
    },
    RawRewriteRule {
        name: "alloc_range_1_1",
        fingerprint: FP_ALLOC_RANGE_1_1,
        emit: emit_alloc_range_1_1,
    },
    RawRewriteRule {
        name: "span_take",
        fingerprint: FP_SPAN_TAKE,
        emit: emit_span_take,
    },
    RawRewriteRule {
        name: "build_entry_from_bytes",
        fingerprint: FP_BUILD_ENTRY_FROM_BYTES,
        emit: emit_build_entry_from_bytes,
    },
    RawRewriteRule {
        name: "entry_match_copy",
        fingerprint: FP_ENTRY_MATCH_COPY,
        emit: emit_entry_match_copy,
    },
    RawRewriteRule {
        name: "entry_copy_if_ok",
        fingerprint: FP_ENTRY_COPY_IF_OK,
        emit: emit_entry_copy_if_ok,
    },
    RawRewriteRule {
        name: "entry_from_bytes_val",
        fingerprint: FP_ENTRY_FROM_BYTES_VAL,
        emit: emit_entry_from_bytes_val,
    },
    RawRewriteRule {
        name: "alloc_range_fill",
        fingerprint: FP_ALLOC_RANGE_FILL,
        emit: emit_alloc_range_fill,
    },
];

pub fn try_emit_raw_rewrite<W: Write>(
    writer: &mut W,
    function: &Function,
    fn_type: &FunctionType,
    fingerprint: &Fingerprint,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    data_segments: &[DataSegment],
) -> Result<bool, String> {
    for rule in RAW_REWRITES {
        if rule.fingerprint == *fingerprint {
            if std::env::var("SOROBAN_AUDITOR_DISABLE_TEMPLATES").is_ok() {
                if rule.name == "entry_decode" {
                    return Ok(false);
                }
            }
            (rule.emit)(writer, function, fn_type, body, import_count, functions, data_segments)?;
            return Ok(true);
        }
    }
    Ok(false)
}
