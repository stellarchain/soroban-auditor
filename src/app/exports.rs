use crate::decompile::DataSegment;
use crate::format::format_type_ident;
use crate::forwarder::CallForwarder;
use crate::soroban::contract::{ContractSpecs, FunctionContractSpec};
use crate::wasm_ir::{Function, Global};
use parity_wasm::elements::{
    CodeSection, ExportSection, FunctionSection, Instruction, Internal, Type, TypeSection,
};
use std::collections::{BTreeMap, HashMap};
use std::io::Write;

pub fn emit_public_spec_functions<W: Write>(
    writer: &mut W,
    contract_specs: &ContractSpecs,
    exports: &ExportSection,
    code: &CodeSection,
    fns: &FunctionSection,
    types: &TypeSection,
    functions: &[Function],
    import_count: usize,
    imported_globals_count: usize,
    globals: &[Global],
    indirect_fns: &mut BTreeMap<u32, Vec<(u32, u32)>>,
    spec_by_fn_index: &HashMap<u32, FunctionContractSpec>,
    data_segments_with_offsets: &[DataSegment],
    forwarders: &BTreeMap<u32, CallForwarder>,
) -> Result<(), String> {
    for export in exports.entries() {
        let spec_fn = crate::app::specs::find_spec_for_export(contract_specs, export);
        if let Some(spec_fn) = spec_fn {
            let ctx_data = crate::app::specs::build_pattern_context_data(
                export,
                import_count,
                code,
                functions,
            );

            let mut assigned_params: Vec<bool> = Vec::new();
            let mut fn_index_for_body: Option<u32> = None;
            let mut body_for_fallback = None;
            let mut fn_type_for_body = None;
            if let &Internal::Function(fn_index) = export.internal() {
                if fn_index as usize >= import_count {
                    let body_index = fn_index as usize - import_count;
                    if let Some(body) = code.bodies().get(body_index) {
                        let type_index = fns.entries()[body_index].type_ref();
                        let fn_type = match &types.types()[type_index as usize] {
                            Type::Function(fn_type) => fn_type,
                        };
                        assigned_params = vec![false; fn_type.params().len()];
                        for instr in body.code().elements() {
                            if let Instruction::SetLocal(i) | Instruction::TeeLocal(i) = *instr {
                                if (i as usize) < assigned_params.len() {
                                    assigned_params[i as usize] = true;
                                }
                            }
                        }
                        fn_index_for_body = Some(fn_index);
                        body_for_fallback = Some(body);
                        fn_type_for_body = Some(fn_type);
                    }
                }
            }

            let env_mut = assigned_params.get(0).copied().unwrap_or(false);
            let env_param = if env_mut { "mut env: Env" } else { "env: Env" };
            writeln!(writer, "    pub fn {}(", ctx_data.export_name).map_err(|e| e.to_string())?;
            writeln!(writer, "        &mut self,").map_err(|e| e.to_string())?;
            writeln!(writer, "        {},", env_param).map_err(|e| e.to_string())?;
            for (idx, argument) in spec_fn.inputs().iter().enumerate() {
                let ty = format_type_ident(&argument.type_ident().to_string());
                let is_mut = assigned_params.get(idx + 1).copied().unwrap_or(false);
                if is_mut {
                    writeln!(writer, "        mut {}: {},", argument.name(), ty)
                } else {
                    writeln!(writer, "        {}: {},", argument.name(), ty)
                }
                .map_err(|e| e.to_string())?;
            }
            write!(writer, "    )").map_err(|e| e.to_string())?;
            if let Some(return_type) = spec_fn.output() {
                let ty = format_type_ident(&return_type.type_ident().to_string());
                write!(writer, " -> {}", ty).map_err(|e| e.to_string())?;
            }
            writeln!(writer, " {{").map_err(|e| e.to_string())?;

            let mut emitted_raw = false;
            if let (Some(body), Some(fn_type), Some(fn_index)) =
                (body_for_fallback, fn_type_for_body, fn_index_for_body)
            {
                let mut expr_index = fn_type.params().len();
                for local in body.locals() {
                    let ty = crate::wasm_ir::to_rs_type(local.value_type());
                    let decimals = if ty.starts_with('f') { ".0" } else { "" };
                    for _ in 0..local.count() {
                        writeln!(
                            writer,
                            "        let mut var{}: {} = 0{};",
                            expr_index, ty, decimals
                        )
                        .map_err(|e| e.to_string())?;
                        expr_index += 1;
                    }
                }
                crate::code_builder::build(
                    writer,
                    expr_index,
                    fn_type.results().first().is_some(),
                    import_count,
                    imported_globals_count,
                    functions,
                    indirect_fns,
                    globals,
                    types,
                    body.code().elements(),
                    2,
                    spec_by_fn_index,
                    fn_index as usize,
                    data_segments_with_offsets,
                    forwarders,
                );
                emitted_raw = true;
            }
            if !emitted_raw {
                writeln!(writer, "        panic!(\"decompilation pending\");")
                    .map_err(|e| e.to_string())?;
                writeln!(writer, "    }}").map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}
