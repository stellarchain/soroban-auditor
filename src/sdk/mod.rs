use serde_json::Value;
use std::path::Path;

pub mod soroban;

pub use crate::sdk::soroban::contract::{ContractSpecs, FunctionContractSpec};

mod analyzer;
mod detector;

#[allow(unused_imports)]
pub use analyzer::SdkUsageAnalyzer;
pub use detector::SdkFunctionDetector;

pub trait SdkBackend {
    fn env_common_modules_result(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>>;
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
    fn env_common_modules_result(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        crate::sdk::soroban::common::env_common_modules_result()
    }

    fn read_contract_specs(
        &self,
        input: &Path,
    ) -> Result<ContractSpecs, Box<dyn std::error::Error>> {
        crate::sdk::soroban::contract::read_contract_specs(input)
    }

    fn find_function_specs(
        &self,
        specs: &ContractSpecs,
        function_name_to_find: &str,
    ) -> Option<FunctionContractSpec> {
        crate::sdk::soroban::contract::find_function_specs(specs, function_name_to_find)
    }
}

static SOROBAN_BACKEND: SorobanBackend = SorobanBackend;

pub fn get_backend() -> &'static dyn SdkBackend {
    &SOROBAN_BACKEND
}
