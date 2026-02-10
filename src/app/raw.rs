use crate::fingerprint::fingerprint_function;
use crate::forwarder::CallForwarder;
use crate::rewrites::try_emit_raw_rewrite;
use crate::wasm_ir::{mangle_fn_name, to_rs_type, Function, Global};
use parity_wasm::elements::{CodeSection, Type, TypeSection};
use std::collections::BTreeMap;
use std::io::Write;

pub fn emit_raw_functions<W: Write>(
    writer: &mut W,
    code: &CodeSection,
    fns: &parity_wasm::elements::FunctionSection,
    types: &TypeSection,
    import_count: usize,
    imported_globals_count: usize,
    functions: &[Function],
    globals: &[Global],
    indirect_fns: &mut BTreeMap<u32, Vec<(u32, u32)>>,
    spec_by_fn_index: &std::collections::HashMap<
        u32,
        crate::soroban::contract::FunctionContractSpec,
    >,
    data_segments: &[crate::decompile::DataSegment],
    forwarders: &std::collections::BTreeMap<u32, CallForwarder>,
    filter: Option<&std::collections::HashSet<u32>>,
    skip_pure: bool,
) -> Result<(), String> {
    for (i, (body, func)) in code.bodies().iter().zip(fns.entries()).enumerate() {
        let type_index = func.type_ref();
        let typ = &types.types()[type_index as usize];
        let fn_type = match *typ {
            Type::Function(ref t) => t,
        };
        let fn_index = import_count + i;
        if let Some(filter) = filter {
            if !filter.contains(&(fn_index as u32)) {
                continue;
            }
        }
        let function = &functions[fn_index];
        let fingerprint = fingerprint_function(body, fn_type, import_count, functions);
        if std::env::var("SOROBAN_AUDITOR_DUMP_FINGERPRINTS").is_ok() {
            eprintln!("fingerprint {} {}", function.name, fingerprint.short());
        }
        if let Ok(target) = std::env::var("SOROBAN_AUDITOR_FP_TARGET") {
            let mut target = target.trim().to_ascii_lowercase();
            if let Some(stripped) = target.strip_prefix("h=") {
                target = stripped.to_string();
            }
            if let Some(stripped) = target.strip_prefix("0x") {
                target = stripped.to_string();
            }
            let current = format!("{:016x}", fingerprint.hash);
            if target == current {
                eprintln!("fp_target {} {}", function.name, fingerprint.short());
                for instr in body.code().elements() {
                    eprintln!("  {:?}", instr);
                }
            }
        }
        if skip_pure && filter.is_none() {
            let mut has_host_call = false;
            for instr in body.code().elements() {
                match instr {
                    parity_wasm::elements::Instruction::Call(n) => {
                        if (*n as usize) < import_count {
                            has_host_call = true;
                            break;
                        }
                    }
                    parity_wasm::elements::Instruction::CallIndirect(_, _) => {
                        has_host_call = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !has_host_call {
                if try_emit_raw_rewrite(
                    writer,
                    function,
                    fn_type,
                    &fingerprint,
                    body,
                    import_count,
                    functions,
                    data_segments,
                )? {
                    continue;
                }
                continue;
            }
        }
        if try_emit_raw_rewrite(
            writer,
            function,
            fn_type,
            &fingerprint,
            body,
            import_count,
            functions,
            data_segments,
        )? {
            continue;
        }
        if let Some(real_name) = function.real_name {
            writeln!(writer, "    // {}", real_name).map_err(|e| e.to_string())?;
        }
        write!(writer, "    ").map_err(|e| e.to_string())?;
        if function.make_public {
            write!(writer, "pub ").map_err(|e| e.to_string())?;
        }
        writeln!(writer, "fn {}(", function.name).map_err(|e| e.to_string())?;
        writeln!(writer, "        &mut self,").map_err(|e| e.to_string())?;
        writeln!(writer, "        env: &Env,").map_err(|e| e.to_string())?;
        let spec_fn = spec_by_fn_index.get(&(fn_index as u32));
        for (i, &param) in fn_type.params().iter().enumerate() {
            let param_name = spec_fn
                .and_then(|spec| spec.inputs().get(i))
                .map(|param| mangle_fn_name(param.name()))
                .unwrap_or_else(|| format!("arg{}", i));
            writeln!(
                writer,
                "        mut {}: {},",
                param_name,
                to_rs_type(param)
            )
            .map_err(|e| e.to_string())?;
        }
        write!(writer, "    )").map_err(|e| e.to_string())?;
        for result in fn_type.results() {
            write!(writer, " -> {}", to_rs_type(*result)).map_err(|e| e.to_string())?;
        }
        writeln!(writer, " {{").map_err(|e| e.to_string())?;

        let mut expr_index = fn_type.params().len();
        for local in body.locals() {
            let ty = to_rs_type(local.value_type());
            let decimals = if ty.starts_with("f") { ".0" } else { "" };
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
            fn_index,
            data_segments,
            forwarders,
        );
    }

    Ok(())
}
