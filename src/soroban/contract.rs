use crate::wasm_ir::mangle_fn_name;
use proc_macro2::TokenStream;
use soroban_spec::read::from_wasm;
use soroban_spec_rust::types::generate_enum;
use soroban_spec_rust::types::generate_error_enum;
use soroban_spec_rust::types::generate_event;
use soroban_spec_rust::types::generate_struct;
use soroban_spec_rust::types::generate_type_ident;
use soroban_spec_rust::types::generate_union;
use std::fmt;
use std::fs::File;
use std::io::Read;
use soroban_sdk::xdr::ScSpecEntry;
use soroban_sdk::xdr::ScSpecTypeDef;

#[derive(Clone)]
pub struct FunctionSpecResults {
    type_ident: TokenStream,
}

impl FunctionSpecResults {
    pub fn type_ident(&self) -> &TokenStream {
        &self.type_ident
    }
}

impl fmt::Display for FunctionSpecResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Formatted representation of ExtendedFunctionReturnType")
    }
}
#[derive(Clone)]
pub struct FunctionSpecParam {
    name: String,
    #[allow(dead_code)]
    spec_def: ScSpecTypeDef,
    type_ident: TokenStream,
}

impl FunctionSpecParam {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_ident(&self) -> &TokenStream {
        &self.type_ident
    }
    #[allow(dead_code)]
    pub fn spec_def(&self) -> &ScSpecTypeDef {
        &self.spec_def
    }
}

impl fmt::Display for FunctionSpecParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_ident)
    }
}

impl fmt::Display for FunctionContractSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs_str: Vec<String> = self
            .inputs
            .iter()
            .map(|param| format!(", {}: {}", param.name, param.type_ident))
            .collect();

        let output_str = match &self.output {
            Some(return_type) => format!("-> {}", return_type.type_ident),
            None => "".to_string(),
        };

        write!(f, "\tpub fn {}(&mut self{}) {}", mangle_fn_name(self.name.as_str()), inputs_str.join(""), output_str)
    }
}
#[derive(Clone)]
pub struct FunctionContractSpec {
    name: String,
    inputs: Vec<FunctionSpecParam>,
    output: Option<FunctionSpecResults>,
}

impl FunctionContractSpec {
    #[allow(dead_code)]
    pub fn default() -> Self {
        FunctionContractSpec {
            name: String::new(),
            inputs: Vec::new(),
            output: None,
        }
    }

    #[allow(dead_code)]
    pub fn default_inputs(inputs: Vec<FunctionSpecParam>) -> Self {
        FunctionContractSpec {
            name: String::new(),
            inputs,
            output: None,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inputs(&self) -> &[FunctionSpecParam] {
        &self.inputs
    }

    pub fn output(&self) -> Option<&FunctionSpecResults> {
        self.output.as_ref()
    }
}

#[derive(Clone, Default)]
pub struct ContractSpecs {
    functions: Vec<FunctionContractSpec>,
    types: Vec<TokenStream>,
}

impl ContractSpecs {
    pub fn functions(&self) -> &[FunctionContractSpec] {
        &self.functions
    }

    pub fn types(&self) -> &[TokenStream] {
        &self.types
    }
}

pub fn find_function_specs(specs: &ContractSpecs, function_name_to_find: &str) -> Option<FunctionContractSpec> {
     for function_info in &specs.functions {
        if function_info.name == function_name_to_find {
            return Some(function_info.clone());
        }
    }
    None
}

pub fn read_contract_specs<P: AsRef<::std::path::Path>>(
    file_path: P,
) -> Result<ContractSpecs, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let entries = from_wasm(&buffer).unwrap();

    let mut spec_fns = Vec::new();
    let mut spec_types = Vec::new();

    for s in entries.iter() {
        match s {
            ScSpecEntry::FunctionV0(f) => {
                let name = f.name.to_utf8_string().unwrap();
                let inputs: Vec<_> = f
                    .inputs
                    .iter()
                    .map(|input| {
                        let name = input.name.to_utf8_string().unwrap();
                        let type_ident = generate_type_ident(&input.type_);
                        FunctionSpecParam {
                            name,
                            spec_def: input.type_.clone(),
                            type_ident,
                        }
                    })
                    .collect();

                let output = f.outputs.to_option().map(|t| FunctionSpecResults {
                    type_ident: generate_type_ident(&t),
                });

                spec_fns.push(FunctionContractSpec {
                    name,
                    inputs,
                    output,
                });
            }
            ScSpecEntry::UdtStructV0(s) => {
                spec_types.push(generate_struct(s));
            }
            ScSpecEntry::UdtUnionV0(u) => {
                spec_types.push(generate_union(u));
            }
            ScSpecEntry::UdtEnumV0(e) => {
                spec_types.push(generate_enum(e));
            }
            ScSpecEntry::UdtErrorEnumV0(e) => {
                spec_types.push(generate_error_enum(e));
            }
            ScSpecEntry::EventV0(e) => {
                spec_types.push(generate_event(e));
            }
        }
    }
    Ok(ContractSpecs {
        functions: spec_fns,
        types: spec_types,
    })
}
