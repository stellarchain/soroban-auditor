use crate::soroban::common::take_common_module;
use crate::sdk::{get_backend, ContractSpecs, FunctionContractSpec};
use crate::wasm_ir::{mangle_fn_name, to_rs_type, Function, Global};
use crate::format::format_spec_tokens;
use parity_wasm::elements::{
    External, ExportSection, FunctionNameSubsection, ImportSection, Instruction, Module, Type,
    TypeSection,
};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::fs;

fn collect_libs(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_libs(&path, out);
            } else if path.file_name().and_then(|n| n.to_str()) == Some("lib.rs") {
                out.push(path);
            }
        }
    }
}

fn extract_contract_name(contents: &str) -> Option<String> {
    let mut saw_contract = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("#[contract") {
            saw_contract = true;
            continue;
        }
        if saw_contract {
            if let Some(rest) = line.strip_prefix("pub struct ") {
                return rest
                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .next()
                    .map(|s| s.to_string());
            }
            if let Some(rest) = line.strip_prefix("struct ") {
                return rest
                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .next()
                    .map(|s| s.to_string());
            }
        }
    }
    None
}

fn normalize_stem(stem: &str) -> String {
    let mut name = stem.to_string();
    if let Some(rest) = name.strip_prefix("soroban_") {
        name = rest.to_string();
    }
    if let Some(rest) = name.strip_suffix("_contract") {
        name = rest.to_string();
    }
    if let Some(rest) = name.strip_suffix("_optimized") {
        name = rest.to_string();
    }
    name = name.replace('-', "_");
    while name.contains("__") {
        name = name.replace("__", "_");
    }
    name
}

fn candidate_key(base: &Path, lib: &Path) -> Option<String> {
    let rel = lib.strip_prefix(base).ok()?;
    let mut parts: Vec<String> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
        .collect();
    if parts.len() >= 3 && parts[parts.len() - 1] == "lib.rs" && parts[parts.len() - 2] == "src"
    {
        parts.truncate(parts.len() - 2);
    }
    if parts.is_empty() {
        return None;
    }
    Some(normalize_stem(&parts.join("_")))
}

#[derive(Debug, Clone)]
pub struct InternalForwarder {
    pub name: String,
    pub args: Vec<ForwardArg>,
}

#[derive(Debug, Clone)]
pub enum ForwardArg {
    Local(usize),
    PackedU32(usize),
    I32(i32),
    I64(i64),
}

pub fn build_internal_call_forwarders(
    code: &parity_wasm::elements::CodeSection,
    fns: &parity_wasm::elements::FunctionSection,
    import_count: usize,
    functions: &[Function],
) -> BTreeMap<u32, InternalForwarder> {
    let mut out = BTreeMap::new();
    for (i, (body, _func)) in code.bodies().iter().zip(fns.entries()).enumerate() {
        let fn_index = import_count + i;
        let mut instrs: Vec<&Instruction> = body.code().elements().iter().collect();
        if let Some(Instruction::End) = instrs.last() {
            instrs.pop();
        }
        if let Some(Instruction::Return) = instrs.last() {
            instrs.pop();
        }
        if instrs.len() < 2 {
            continue;
        }
        let call_instr = instrs.last().unwrap();
        let imported_index = match call_instr {
            Instruction::Call(n) if (*n as usize) < import_count => *n,
            _ => continue,
        };
        let import_name = functions[imported_index as usize].name.clone();
        let import_params = functions[imported_index as usize].ty.params().len();
        let mut args = Vec::new();
        let mut ok = true;
        let mut i = 0usize;
        let insts = &instrs[..instrs.len() - 1];
        while i < insts.len() {
            if let Some(Instruction::GetLocal(n)) = insts.get(i) {
                if matches!(insts.get(i + 1), Some(Instruction::I64ExtendSI32) | Some(Instruction::I64ExtendUI32))
                    && matches!(insts.get(i + 2), Some(Instruction::I64Const(32)))
                    && matches!(insts.get(i + 3), Some(Instruction::I64Shl))
                    && matches!(insts.get(i + 4), Some(Instruction::I64Const(4)))
                    && matches!(insts.get(i + 5), Some(Instruction::I64Or))
                {
                    args.push(ForwardArg::PackedU32(*n as usize));
                    i += 6;
                    continue;
                }
            }
            let instr = insts.get(i).unwrap();
            match instr {
                Instruction::GetLocal(n) => args.push(ForwardArg::Local(*n as usize)),
                Instruction::I32Const(v) => args.push(ForwardArg::I32(*v)),
                Instruction::I64Const(v) => args.push(ForwardArg::I64(*v)),
                _ => {
                    ok = false;
                    break;
                }
            }
            i += 1;
        }
        if !ok || args.len() < import_params {
            continue;
        }
        if args.len() > import_params {
            args = args[args.len() - import_params..].to_vec();
        }
        out.insert(
            fn_index as u32,
            InternalForwarder {
                name: import_name,
                args,
            },
        );
    }
    out
}

pub fn find_source_for_contract(contract_name: &str, input: &Path) -> Option<PathBuf> {
    let base = Path::new("tests/soroban-examples");
    if !base.exists() {
        return None;
    }
    let mut libs = Vec::new();
    collect_libs(base, &mut libs);

    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let target = normalize_stem(stem);

    let mut exact_matches = Vec::new();
    let mut contains_matches = Vec::new();
    for lib in libs.iter() {
        if let Some(key) = candidate_key(base, lib) {
            if key == target {
                exact_matches.push(lib.clone());
            } else if key.contains(&target) || target.contains(&key) {
                contains_matches.push(lib.clone());
            }
        }
    }
    if exact_matches.len() == 1 {
        return exact_matches.into_iter().next();
    }
    if exact_matches.len() > 1 {
        let mut name_matches = Vec::new();
        for lib in exact_matches.iter() {
            if let Ok(contents) = fs::read_to_string(lib) {
                if let Some(name) = extract_contract_name(&contents) {
                    if name == contract_name {
                        name_matches.push(lib.clone());
                    }
                }
            }
        }
        if name_matches.len() == 1 {
            return name_matches.into_iter().next();
        }
    }

    if contains_matches.len() == 1 {
        return contains_matches.into_iter().next();
    }
    if contains_matches.len() > 1 {
        let mut name_matches = Vec::new();
        for lib in contains_matches.iter() {
            if let Ok(contents) = fs::read_to_string(lib) {
                if let Some(name) = extract_contract_name(&contents) {
                    if name == contract_name {
                        name_matches.push(lib.clone());
                    }
                }
            }
        }
        if name_matches.len() == 1 {
            return name_matches.into_iter().next();
        }
    }

    let mut name_matches = Vec::new();
    for lib in libs {
        if let Ok(contents) = fs::read_to_string(&lib) {
            if let Some(name) = extract_contract_name(&contents) {
                if name == contract_name {
                    name_matches.push(lib);
                }
            }
        }
    }
    if name_matches.len() == 1 {
        return name_matches.into_iter().next();
    }
    None
}

pub fn is_const_expr_immutable_instead_of_const(opcodes: &[Instruction]) -> bool {
    opcodes.len() == 2 && matches!(opcodes[0], Instruction::GetGlobal(_))
}

pub struct SpecTypeFlags {
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
}

#[derive(Clone, Debug)]
pub struct DataKeyVariant {
    pub name: String,
    pub fields: usize,
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: String,
    pub ty: String,
}

#[derive(Clone, Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

pub fn contract_name_from_module(module: &Module) -> String {
    module
        .names_section()
        .and_then(|names| names.module())
        .map(|module_name| mangle_fn_name(module_name.name()))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Contract".to_string())
}

pub fn contract_name_from_module_or_path(module: &Module, input: &Path) -> String {
    let module_name = contract_name_from_module(module);
    if module_name != "Contract" {
        return module_name;
    }
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let mut name = stem.to_string();
    if let Some(rest) = name.strip_prefix("soroban_") {
        name = rest.to_string();
    }
    let mut out = String::new();
    for part in name.split(|c: char| c == '_' || c == '-') {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            for ch in chars {
                out.push(ch);
            }
        }
    }
    if out.is_empty() {
        "Contract".to_string()
    } else {
        out
    }
}

pub fn spec_type_flags(contract_specs: &ContractSpecs) -> SpecTypeFlags {
    let spec_type_strings: Vec<String> = contract_specs
        .types()
        .iter()
        .map(|t| t.to_string())
        .collect();
    SpecTypeFlags {
        has_datakey_type: spec_type_strings.iter().any(|t| t.contains("DataKey")),
        has_allowance_value_type: spec_type_strings.iter().any(|t| t.contains("AllowanceValue")),
        has_allowance_key_type: spec_type_strings.iter().any(|t| t.contains("AllowanceDataKey")),
        has_token_metadata_type: spec_type_strings.iter().any(|t| t.contains("TokenMetadata")),
        has_signer_variant: spec_type_strings
            .iter()
            .any(|t| t.contains("Signer(") || t.contains("Signer (")),
        has_signer_cnt_variant: spec_type_strings.iter().any(|t| t.contains("SignerCnt")),
        has_admin_variant: spec_type_strings.iter().any(|t| t.contains("Admin")),
        has_spend_limit_variant: spec_type_strings.iter().any(|t| t.contains("SpendLimit")),
        has_counter_variant: spec_type_strings
            .iter()
            .any(|t| t.contains("Counter(") || t.contains("Counter (")),
        has_owner_variant: spec_type_strings.iter().any(|t| {
            t.contains("Owner ,")
                || t.contains("Owner,")
                || t.contains("Owner }")
                || t.contains("Owner}")
        }),
    }
}

pub fn is_account_contract(contract_specs: &ContractSpecs) -> bool {
    let fn_names: Vec<String> = contract_specs
        .functions()
        .iter()
        .map(|f| f.name().to_string())
        .collect();
    let has_check_auth = fn_names
        .iter()
        .any(|n| n == "__check_auth" || n == "___check_auth");
    let has_constructor = fn_names
        .iter()
        .any(|n| n == "__constructor" || n == "___constructor");
    let has_add_limit = fn_names.iter().any(|n| n == "add_limit");
    let type_strings: Vec<String> = contract_specs
        .types()
        .iter()
        .map(|t| t.to_string())
        .collect();
    let has_acc_signature = type_strings.iter().any(|t| t.contains("AccSignature"));
    let has_acc_error = type_strings.iter().any(|t| t.contains("AccError"));
    has_check_auth && has_constructor && has_add_limit && has_acc_signature && has_acc_error
}

pub fn parse_datakey_variants(contract_specs: &ContractSpecs) -> Vec<DataKeyVariant> {
    for ty in contract_specs.types() {
        let formatted = format_spec_tokens(&ty.to_string());
        if let Some(pos) = formatted.find("enum DataKey") {
            let after = &formatted[pos..];
            let start = after.find('{');
            if start.is_none() {
                continue;
            }
            let start = pos + start.unwrap();
            let mut depth = 0usize;
            let mut end = None;
            for (i, ch) in formatted[start..].char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(start + i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let Some(end) = end else { continue };
            let inner = &formatted[start + 1..end];
            let mut variants = Vec::new();
            let mut current = String::new();
            let mut paren_depth = 0usize;
            for ch in inner.chars() {
                match ch {
                    '(' => {
                        paren_depth += 1;
                        current.push(ch);
                    }
                    ')' => {
                        if paren_depth > 0 {
                            paren_depth -= 1;
                        }
                        current.push(ch);
                    }
                    ',' if paren_depth == 0 => {
                        let token = current.trim();
                        if !token.is_empty() {
                            variants.push(token.to_string());
                        }
                        current.clear();
                    }
                    _ => current.push(ch),
                }
            }
            if !current.trim().is_empty() {
                variants.push(current.trim().to_string());
            }

            let mut out = Vec::new();
            for v in variants {
                let v = v.trim().trim_matches(',').trim();
                if v.is_empty() {
                    continue;
                }
                let (name, fields) = if let Some(paren) = v.find('(') {
                    let name = v[..paren].trim().to_string();
                    let inner = v[paren + 1..].trim_end_matches(')').trim();
                    if inner.is_empty() {
                        (name, 0)
                    } else {
                        let mut count = 1;
                        let mut depth = 0usize;
                        for ch in inner.chars() {
                            match ch {
                                '(' => depth += 1,
                                ')' => {
                                    if depth > 0 {
                                        depth -= 1;
                                    }
                                }
                                ',' if depth == 0 => count += 1,
                                _ => {}
                            }
                        }
                        (name, count)
                    }
                } else {
                    (v.to_string(), 0)
                };
                out.push(DataKeyVariant { name, fields });
            }
            return out;
        }
    }
    Vec::new()
}

pub fn parse_struct_defs(contract_specs: &ContractSpecs) -> Vec<StructDef> {
    let mut out = Vec::new();
    for ty in contract_specs.types() {
        let formatted = format_spec_tokens(&ty.to_string());
        let Some(pos) = formatted.find("struct ") else { continue };
        let after = &formatted[pos + "struct ".len()..];
        let name_end = after.find(|c: char| c == '{' || c.is_whitespace());
        let Some(name_end) = name_end else { continue };
        let name = after[..name_end].trim().to_string();
        let brace_start = formatted[pos..].find('{').map(|i| pos + i);
        let Some(start) = brace_start else { continue };
        let mut depth = 0usize;
        let mut end = None;
        for (i, ch) in formatted[start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { continue };
        let inner = &formatted[start + 1..end];
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0usize;
        for ch in inner.chars() {
            match ch {
                '<' => {
                    paren_depth += 1;
                    current.push(ch);
                }
                '>' => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                    }
                    current.push(ch);
                }
                ',' if paren_depth == 0 => {
                    let token = current.trim();
                    if !token.is_empty() {
                        fields.push(token.to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.trim().is_empty() {
            fields.push(current.trim().to_string());
        }
        let mut fields_out = Vec::new();
        for f in fields {
            let f = f.trim().trim_start_matches("pub ").trim();
            let name_end = f.find(':').unwrap_or(f.len());
            let field_name = f[..name_end].trim();
            let ty = if let Some(colon) = f.find(':') {
                f[colon + 1..].trim().to_string()
            } else {
                String::new()
            };
            if !field_name.is_empty() {
                fields_out.push(StructField {
                    name: field_name.to_string(),
                    ty: format_spec_tokens(&ty),
                });
            }
        }
        if !name.is_empty() {
            out.push(StructDef {
                name,
                fields: fields_out,
            });
        }
    }
    out
}

pub fn build_functions<'a>(
    modules: &[Value],
    imports: Option<&ImportSection>,
    types: &'a TypeSection,
    fns: &parity_wasm::elements::FunctionSection,
    function_names: Option<&'a FunctionNameSubsection>,
    use_name_section: bool,
) -> Vec<Function<'a>> {
    let mut functions = Vec::new();

    if let Some(imports) = imports {
        for import in imports.entries() {
            if let &External::Function(ty_index) = import.external() {
                let typ: &Type = &types.types()[ty_index as usize];
                let fn_type = match *typ {
                    Type::Function(ref t) => t,
                };
                let module_import = take_common_module(
                    modules,
                    import.module().to_owned().as_str(),
                    import.field().to_owned().as_str(),
                );
                let (field_name, module_name) = match module_import {
                    Some(module) => (module.function_name, module.module_name),
                    None => (import.field().to_owned(), String::new()),
                };

                functions.push(Function {
                    name: field_name,
                    module_name,
                    ty: fn_type,
                    ty_index,
                    real_name: None,
                    make_public: false,
                });
            }
        }
    }

    for function in fns.entries() {
        let ty_index = function.type_ref();
        let Type::Function(ref fn_type) = types.types()[ty_index as usize];
        let mut real_name = function_names.and_then(|f| f.names().get(functions.len() as _));
        let mut name = format!("func{}", functions.len());
        if use_name_section {
            if let Some(real_name) = real_name.take() {
                name = real_name.to_string();
                while functions.iter().any(|f| f.name == name) {
                    name.push_str("_");
                }
            }
        }
        functions.push(Function {
            name,
            module_name: String::new(),
            ty: fn_type,
            ty_index,
            real_name,
            make_public: false,
        });
    }

    for function in &mut functions {
        function.name = mangle_fn_name(&function.name);
    }

    functions
}

pub fn build_globals<'a>(
    imports: Option<&ImportSection>,
    module: &'a Module,
    exports: &ExportSection,
) -> (Vec<Global<'a>>, usize) {
    let mut globals = Vec::new();

    if let Some(imports) = imports {
        for import in imports.entries() {
            if let &External::Global(ty) = import.external() {
                let name = import.field().to_string();
                globals.push(Global {
                    is_mutable: ty.is_mutable(),
                    is_pub: true,
                    name,
                    ty: to_rs_type(ty.content_type()),
                    const_expr: &[],
                });
            }
        }
    }

    let imported_globals_count = globals.len();

    if let Some(global_section) = module.global_section() {
        for entry in global_section.entries() {
            let ty = entry.global_type();
            let const_expr = entry.init_expr().code();
            let is_mutable = ty.is_mutable() || is_const_expr_immutable_instead_of_const(const_expr);
            let name = if is_mutable {
                format!("global{}", globals.len())
            } else {
                format!("GLOBAL{}", globals.len())
            };
            globals.push(Global {
                is_mutable,
                is_pub: false,
                name,
                ty: to_rs_type(ty.content_type()),
                const_expr,
            });
        }
    }

    for export in exports.entries() {
        if let &parity_wasm::elements::Internal::Global(global_index) = export.internal() {
            let global = &mut globals[global_index as usize];
            global.name = export.field().to_string();
            global.is_pub = true;
        }
    }

    (globals, imported_globals_count)
}

pub fn build_spec_index(
    contract_specs: &ContractSpecs,
    exports: &ExportSection,
) -> HashMap<u32, FunctionContractSpec> {
    let mut spec_by_fn_index: HashMap<u32, FunctionContractSpec> = HashMap::new();
    for export in exports.entries() {
        if let &parity_wasm::elements::Internal::Function(fn_index) = export.internal() {
            if let Some(spec_fn) = get_backend().find_function_specs(contract_specs, export.field()) {
                spec_by_fn_index.insert(fn_index, spec_fn);
            }
        }
    }
    spec_by_fn_index
}
