use crate::code_builder::forwarders::resolve_forwarder_chain;
use crate::code_builder::memory as mememit;
use crate::code_builder::sdk_map::map_imported_call;
use crate::code_builder::utils::{parse_stack_addr, slot_name};
use crate::decompile::DataSegment;
use crate::expr_builder::ExprBuilder;
use crate::forwarder::CallForwarder;
use crate::precedence;
use crate::semantic_resolver::resolver;
use crate::wasm_ir::{Function, Indentation};
use parity_wasm::elements::{Type, TypeSection};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;

fn write_internal_call<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    name: &str,
    args: &[String],
    real_name: Option<&String>,
) {
    if args.len() <= 2 {
        write!(writer, "self.{}(env", name).unwrap();
        for expr in args {
            write!(writer, ", {}", expr).unwrap();
        }
        if let Some(real_name) = real_name {
            writeln!(writer, "); // {}", real_name).unwrap();
        } else {
            writeln!(writer, ");").unwrap();
        }
        return;
    }

    writeln!(writer, "self.{}(", name).unwrap();
    writeln!(writer, "{}    env,", indentation).unwrap();
    for expr in args {
        writeln!(writer, "{}    {},", indentation, expr).unwrap();
    }
    if let Some(real_name) = real_name {
        writeln!(writer, "{}); // {}", indentation, real_name).unwrap();
    } else {
        writeln!(writer, "{});", indentation).unwrap();
    }
}

pub(super) fn emit_call<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    expr_index: &mut usize,
    fn_index: u32,
    import_count: usize,
    functions: &[Function],
    data_segments: &[DataSegment],
    forwarders: &BTreeMap<u32, CallForwarder>,
    tainted_bases: &mut HashSet<String>,
) {
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
        write!(writer, "let var{} = ", *expr_index).unwrap();
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
    } else if let Some(inlined) = resolve_forwarder_chain(
        fn_index,
        &args,
        forwarders,
        import_count,
        functions,
        data_segments,
    ) {
        write!(writer, "{}", inlined).unwrap();
        writeln!(writer, ";").unwrap();
    } else if let Some(mapped) = resolver().resolve_sdk_call(name, &args) {
        write!(writer, "{}", mapped).unwrap();
        if let Some(real_name) = real_name {
            writeln!(writer, "; // {}", real_name).unwrap();
        } else {
            writeln!(writer, ";").unwrap();
        }
    } else {
        write_internal_call(writer, indentation, name, &args, real_name);
    }

    for _ in fn_type.results() {
        expr_builder.push((precedence::PATH, format!("var{}", *expr_index)));
    }
    *expr_index += 1;
}

pub(super) fn emit_call_indirect<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    expr_index: &mut usize,
    type_index: u32,
    types: &TypeSection,
    indirect_fns: &mut BTreeMap<u32, Vec<(u32, u32)>>,
) {
    let Type::Function(ref fn_type) = types.types()[type_index as usize];
    indirect_fns.entry(type_index).or_insert_with(Vec::new);
    let table_index_expr = expr_builder.pop_formatted(precedence::AS).unwrap();
    let arg_count = fn_type.params().len();
    let args_start = expr_builder.len().saturating_sub(arg_count);
    let args: Vec<String> = expr_builder
        .inner()
        .drain(args_start..)
        .map(|(_, expr)| expr)
        .collect();

    write!(writer, "{}", indentation).unwrap();
    for _ in fn_type.results() {
        write!(writer, "let var{} = ", *expr_index).unwrap();
    }
    if fn_type.results().is_empty() {
        writeln!(
            writer,
            "{{ let _ = ({}{}); unimplemented!(\"call_indirect type {}\"); }};",
            table_index_expr,
            if args.is_empty() {
                String::new()
            } else {
                format!(", {}", args.join(", "))
            },
            type_index
        )
        .unwrap();
    } else {
        writeln!(
            writer,
            "{{ let _ = ({}{}); unimplemented!(\"call_indirect type {}\") }};",
            table_index_expr,
            if args.is_empty() {
                String::new()
            } else {
                format!(", {}", args.join(", "))
            },
            type_index
        )
        .unwrap();
    }

    for _ in fn_type.results() {
        expr_builder.push((precedence::PATH, format!("var{}", *expr_index)));
    }
    *expr_index += 1;
}

pub(super) fn emit_i32_load<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    expr_index: &mut usize,
    offset: u32,
    stack_slots_i32: &mut HashMap<(String, i32), String>,
    stack_slots_i64: &mut HashMap<(String, i32), String>,
    tainted_bases: &HashSet<String>,
    allow_tainted_slot_lift: bool,
) {
    let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr_str = addr.to_string();
    let mut used_slot = false;
    if let Some((base, extra)) = parse_stack_addr(&addr_str) {
        if !tainted_bases.contains(&base) || allow_tainted_slot_lift {
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
                    "{}let mut {} = mload32!({}) as i32;",
                    indentation,
                    name,
                    mememit::mem_addr(&addr_str, offset)
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
            "{}let var{} = mload32!({}) as i32;",
            indentation,
            *expr_index,
            mememit::mem_addr(&addr_str, offset)
        )
        .unwrap();
        expr_builder.push((precedence::PATH, format!("var{}", *expr_index)));
        *expr_index += 1;
    }
}

pub(super) fn emit_i64_load<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    expr_index: &mut usize,
    offset: u32,
    stack_slots_i32: &mut HashMap<(String, i32), String>,
    stack_slots_i64: &mut HashMap<(String, i32), String>,
    tainted_bases: &HashSet<String>,
    allow_tainted_slot_lift: bool,
) {
    let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr_str = addr.to_string();
    let mut used_slot = false;
    if let Some((base, extra)) = parse_stack_addr(&addr_str) {
        if !tainted_bases.contains(&base) || allow_tainted_slot_lift {
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
                    "{}let mut {} = mload64!({}) as i64;",
                    indentation,
                    name,
                    mememit::mem_addr(&addr_str, offset)
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
            "{}let var{} = mload64!({}) as i64;",
            indentation,
            *expr_index,
            mememit::mem_addr(&addr_str, offset)
        )
        .unwrap();
        expr_builder.push((precedence::PATH, format!("var{}", *expr_index)));
        *expr_index += 1;
    }
}

pub(super) fn emit_i32_store<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    offset: u32,
    stack_slots_i32: &mut HashMap<(String, i32), String>,
    tainted_bases: &HashSet<String>,
    allow_tainted_slot_lift: bool,
) {
    let value = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr_str = addr.to_string();
    let mut used_slot = false;
    if let Some((base, extra)) = parse_stack_addr(&addr_str) {
        if !tainted_bases.contains(&base) || allow_tainted_slot_lift {
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
                    writeln!(writer, "{}let mut {} = {} as i32;", indentation, name, value)
                        .unwrap();
                }
            }
            used_slot = true;
        }
    }

    if !used_slot {
        mememit::emit_store_line(
            writer,
            indentation,
            "mstore32",
            &addr_str,
            offset,
            &value.to_string(),
            " as u32",
        )
        .unwrap();
    }
}

pub(super) fn emit_i64_store<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    expr_builder: &mut ExprBuilder,
    offset: u32,
    stack_slots_i64: &mut HashMap<(String, i32), String>,
    tainted_bases: &HashSet<String>,
    allow_tainted_slot_lift: bool,
) {
    let value = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr = expr_builder.pop_formatted(precedence::AS).unwrap();
    let addr_str = addr.to_string();
    let mut used_slot = false;
    if let Some((base, extra)) = parse_stack_addr(&addr_str) {
        if !tainted_bases.contains(&base) || allow_tainted_slot_lift {
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
                    writeln!(writer, "{}let mut {} = {} as i64;", indentation, name, value)
                        .unwrap();
                }
            }
            used_slot = true;
        }
    }

    if !used_slot {
        mememit::emit_store_line(
            writer,
            indentation,
            "mstore64",
            &addr_str,
            offset,
            &value.to_string(),
            " as u64",
        )
        .unwrap();
    }
}
