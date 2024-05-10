use proc_macro2::TokenStream;
use soroban_spec::read::from_wasm;
use soroban_spec_rust::types::generate_enum;
use soroban_spec_rust::types::generate_error_enum;
use soroban_spec_rust::types::generate_struct;
use soroban_spec_rust::types::generate_type_ident;
use soroban_spec_rust::types::generate_union;
use stellar_xdr::curr::ScSpecTypeDef;
use std::fs::File;
use std::io::Read;
use std::fmt;
use stellar_xdr::curr::ScSpecEntry;


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

pub struct FunctionSpecParam {
    name: String,
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
    pub fn spec_def(&self) -> &ScSpecTypeDef {
        &self.spec_def
    }
}

impl fmt::Display for FunctionSpecParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_ident)
    }
}


pub struct FunctionContractSpec {
    name: String,
    inputs: Vec<FunctionSpecParam>,
    output: Option<FunctionSpecResults>,
}

impl FunctionContractSpec {
    pub fn default() -> Self {
        FunctionContractSpec {
            name: String::new(),
            inputs: Vec::new(),
            output: None,
        }
    }

    pub fn default_inputs(inputs: Vec<FunctionSpecParam>) -> Self {
        FunctionContractSpec {
            name: String::new(),
            inputs,
            output: None,
        }
    }

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

pub fn read_contract_specs<P: AsRef<::std::path::Path>>(
    file_path: P,
) -> Result<Vec<FunctionContractSpec>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let entries = from_wasm(&buffer).unwrap();


    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    let mut spec_enums = Vec::new();
    let mut spec_error_enums = Vec::new();

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
                        FunctionSpecParam { name, spec_def: input.type_.clone(), type_ident }
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
                let struct_info = generate_struct(s);
                spec_structs.push(struct_info);
            }
            ScSpecEntry::UdtUnionV0(u) => {
                let union_info = generate_union(u);
                spec_unions.push(union_info);
            }
            ScSpecEntry::UdtEnumV0(e) => {
                let enum_info = generate_enum(e);
                spec_enums.push(enum_info);
            }
            ScSpecEntry::UdtErrorEnumV0(e) => {
                let error_enum_info = generate_error_enum(e);
                spec_error_enums.push(error_enum_info);
            }
        }
    }
    let spec_entries = (&spec_fns, spec_structs, spec_unions, spec_enums, spec_error_enums);
    //TODO: add all spec entry types
    Ok(spec_fns)
}
