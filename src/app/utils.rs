use crate::soroban::common::take_common_module;
use crate::soroban::contract::{find_function_specs, ContractSpecs, FunctionContractSpec};
use crate::wasm_ir::{mangle_fn_name, to_rs_type, Function, Global};
use parity_wasm::elements::{
    External, ExportSection, FunctionNameSubsection, ImportSection, Instruction, Module, Type,
    TypeSection,
};
use serde_json::Value;
use std::collections::HashMap;

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

pub fn contract_name_from_module(module: &Module) -> String {
    module
        .names_section()
        .and_then(|names| names.module())
        .map(|module_name| mangle_fn_name(module_name.name()))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Contract".to_string())
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
            if let Some(spec_fn) = find_function_specs(contract_specs, export.field()) {
                spec_by_fn_index.insert(fn_index, spec_fn);
            }
        }
    }
    spec_by_fn_index
}
