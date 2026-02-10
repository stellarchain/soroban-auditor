use crate::precedence;
use crate::reorder_analysis::can_local_be_reordered;
use crate::forwarder::CallForwarder;
use crate::wasm_ir::{
    mangle_fn_name, to_rs_type, BlockKind, Function, Global, Indentation,
};
use crate::{expr_builder::ExprBuilder, soroban::contract::FunctionContractSpec};
use parity_wasm::elements::{BlockType, Instruction, TypeSection};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;

mod utils;
mod forwarders;
mod emit;
mod memory;
mod sdk_map;
mod literals;
mod ops;

use self::utils::{compute_label_needed, is_breakable_if, try_format_tag_compare};
use self::emit::{emit_br_table_arm, emit_br_table_default};
use self::memory as mememit;
use self::ops::{
    emit_call,
    emit_call_indirect,
    emit_i32_load,
    emit_i32_store,
    emit_i64_load,
    emit_i64_store,
};
pub use self::literals::transform_from_soroban_val_raw;
use self::literals::transform_from_soroban_val;


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
    forwarders: &std::collections::BTreeMap<u32, CallForwarder>,
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
    let declared_unit_signature = spec_fn_opt
        .and_then(|spec| spec.output())
        .is_none();
    let spec_input_offset = if spec_fn_opt.is_some() && spec_len + 1 == param_count {
        1
    } else {
        0
    };
    let param_name_for = |idx: u32| -> String {
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
    let local_name_for = |idx: u32| -> String {
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
                let is_terminal_unit_sentinel = matches!(
                    blocks.last(),
                    Some(BlockKind::Function {
                        evaluates_to_value: false
                    })
                ) && matches!(code.as_slice().first(), Some(Instruction::End));
                let is_spec_unit_terminal = declared_unit_signature
                    && matches!(
                        blocks.last(),
                        Some(BlockKind::Function { .. })
                    )
                    && matches!(code.as_slice().first(), Some(Instruction::End));
                if !is_terminal_unit_sentinel {
                    if !is_spec_unit_terminal {
                        writeln!(writer, "{}unreachable!();", indentation).unwrap();
                    }
                }
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
                    blocks.push(BlockKind::Block {
                        label: None,
                        dst_var,
                    });
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
                    blocks.push(BlockKind::Loop {
                        label: None,
                        dst_var,
                    });
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
                        if label.is_some() {
                            indentation.0 -= 1;
                            writeln!(writer, "{}}}", indentation).unwrap();
                        }
                    }
                    BlockKind::Function { evaluates_to_value } => {
                        if evaluates_to_value {
                            if let Some((_, expr)) = expr_builder.pop() {
                                writeln!(writer, "{}{}", indentation, expr).unwrap();
                            } else {
                                if !declared_unit_signature {
                                    writeln!(writer, "{}// There should've been an expression value here, but this may be unreachable", indentation).unwrap();
                                    writeln!(writer, "{}unreachable!();", indentation).unwrap();
                                }
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
                    let target_block = blocks.iter().rev().nth(relative_depth as usize);
                    emit_br_table_arm(writer, &indentation, index, target_block);
                }
                let default_block = blocks.iter().rev().nth(table.default as usize);
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
                emit_call(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    &mut expr_index,
                    fn_index,
                    import_count,
                    functions,
                    data_segments,
                    forwarders,
                    &mut tainted_bases,
                );
            }
            CallIndirect(type_index, _) => {
                emit_call_indirect(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    &mut expr_index,
                    type_index,
                    types,
                    indirect_fns,
                );
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
                emit_i32_load(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    &mut expr_index,
                    offset,
                    &mut stack_slots_i32,
                    &mut stack_slots_i64,
                    &tainted_bases,
                    allow_tainted_slot_lift,
                );
            }
            I64Load(_log_align, offset) => {
                emit_i64_load(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    &mut expr_index,
                    offset,
                    &mut stack_slots_i32,
                    &mut stack_slots_i64,
                    &tainted_bases,
                    allow_tainted_slot_lift,
                );
            }
            F32Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = f32::from_bits(mload32!({}));",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            F64Load(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = f64::from_bits(mload64!({}));",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load8S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload8!({}) as i8 as i32;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load8U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload8!({}) as i32;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load16S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload16!({}) as i16 as i32;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Load16U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload16!({}) as i32;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load8S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload8!({}) as i8 as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load8U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload8!({}) as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load16S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload16!({}) as i16 as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load16U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload16!({}) as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load32S(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload32!({}) as i32 as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I64Load32U(_log_align, offset) => {
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                writeln!(
                    writer,
                    "{}let var{} = mload32!({}) as i64;",
                    indentation,
                    expr_index,
                    mememit::mem_addr(&addr, offset)
                )
                .unwrap();
                expr_builder.push((precedence::PATH, format!("var{}", expr_index)));
                expr_index += 1;
            }
            I32Store(_log_align, offset) => {
                emit_i32_store(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    offset,
                    &mut stack_slots_i32,
                    &tainted_bases,
                    allow_tainted_slot_lift,
                );
            }
            I64Store(_log_align, offset) => {
                emit_i64_store(
                    writer,
                    &indentation,
                    &mut expr_builder,
                    offset,
                    &mut stack_slots_i64,
                    &tainted_bases,
                    allow_tainted_slot_lift,
                );
            }
            F32Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::METHOD_CALL).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore32",
                    &addr,
                    offset,
                    &format!("{}.to_bits()", value),
                    "",
                )
                .unwrap();
            }
            F64Store(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::METHOD_CALL).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore64",
                    &addr,
                    offset,
                    &format!("{}.to_bits()", value),
                    "",
                )
                .unwrap();
            }
            I32Store8(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore8",
                    &addr,
                    offset,
                    &value.to_string(),
                    " as u8",
                )
                .unwrap();
            }
            I32Store16(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore16",
                    &addr,
                    offset,
                    &value.to_string(),
                    " as u16",
                )
                .unwrap();
            }
            I64Store8(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore8",
                    &addr,
                    offset,
                    &value.to_string(),
                    " as u8",
                )
                .unwrap();
            }
            I64Store16(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore16",
                    &addr,
                    offset,
                    &value.to_string(),
                    " as u16",
                )
                .unwrap();
            }
            I64Store32(_log_align, offset) => {
                let value = expr_builder.pop_formatted(precedence::AS).unwrap();
                let addr = expr_builder.pop_formatted(precedence::AS).unwrap().to_string();
                mememit::emit_store_line(
                    writer,
                    &indentation,
                    "mstore32",
                    &addr,
                    offset,
                    &value.to_string(),
                    " as u32",
                )
                .unwrap();
            }
            CurrentMemory(_) => {
                let dst = format!("var{}", expr_index);
                writeln!(writer, "{}let {} = msize!();", indentation, dst).unwrap();
                expr_index += 1;
                expr_builder.push((precedence::PATH, dst));
            }
            GrowMemory(_) => {
                let pages = expr_builder.pop_formatted(precedence::AS).unwrap();
                let dst = format!("var{}", expr_index);
                writeln!(
                    writer,
                    "{}let {} = mgrow!({} as usize);",
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
                        if let Some(mapped) =
                            try_format_tag_compare(&a.to_string(), &b.to_string(), true)
                        {
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
                        if let Some(mapped) =
                            try_format_tag_compare(&a.to_string(), &b.to_string(), false)
                        {
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
                        if let Some(mapped) =
                            try_format_tag_compare(&a.to_string(), &b.to_string(), true)
                        {
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
                        if let Some(mapped) =
                            try_format_tag_compare(&a.to_string(), &b.to_string(), false)
                        {
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
