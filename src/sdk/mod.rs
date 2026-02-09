use serde_json::Value;
use std::path::Path;

pub use crate::soroban::common::ModuleFunction;
pub use crate::soroban::contract::{ContractSpecs, FunctionContractSpec};

pub trait SdkBackend {
    fn name(&self) -> &'static str;
    fn env_common_modules_result(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>>;
    fn take_common_module(
        &self,
        modules: &[Value],
        module_name: &str,
        field_name: &str,
    ) -> Option<ModuleFunction>;
    fn read_contract_specs(
        &self,
        input: &Path,
    ) -> Result<ContractSpecs, Box<dyn std::error::Error>>;
    fn find_function_specs(
        &self,
        specs: &ContractSpecs,
        function_name_to_find: &str,
    ) -> Option<FunctionContractSpec>;
}

pub struct SorobanBackend;

impl SdkBackend for SorobanBackend {
    fn name(&self) -> &'static str {
        "soroban"
    }

    fn env_common_modules_result(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        crate::soroban::common::env_common_modules_result()
    }

    fn take_common_module(
        &self,
        modules: &[Value],
        module_name: &str,
        field_name: &str,
    ) -> Option<ModuleFunction> {
        crate::soroban::common::take_common_module(modules, module_name, field_name)
    }

    fn read_contract_specs(
        &self,
        input: &Path,
    ) -> Result<ContractSpecs, Box<dyn std::error::Error>> {
        crate::soroban::contract::read_contract_specs(input)
    }

    fn find_function_specs(
        &self,
        specs: &ContractSpecs,
        function_name_to_find: &str,
    ) -> Option<FunctionContractSpec> {
        crate::soroban::contract::find_function_specs(specs, function_name_to_find)
    }
}

static SOROBAN_BACKEND: SorobanBackend = SorobanBackend;

pub fn get_backend() -> &'static dyn SdkBackend {
    &SOROBAN_BACKEND
}
