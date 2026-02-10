use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Comprehensive SDK function detector that uses the common_env_soroban.json data
pub struct SdkFunctionDetector {
    /// Map from export name (e.g., "x.1") to function info
    export_to_info: HashMap<String, SdkFunctionInfo>,
    /// Map from function name (e.g., "vec_new") to function info
    name_to_info: HashMap<String, SdkFunctionInfo>,
    /// Categories of SDK functions for detection
    categories: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone)]
pub struct SdkFunctionInfo {
    pub module_name: String,
    pub function_name: String,
    pub export: String,
    pub args: Vec<SdkArg>,
    pub return_type: String,
    pub docs: String,
}

#[derive(Debug, Clone)]
pub struct SdkArg {
    pub name: String,
    pub ty: String,
}

impl SdkFunctionDetector {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = include_str!("../soroban/common_env_soroban.json");
        let data: Value = serde_json::from_str(json_data)?;

        let mut export_to_info = HashMap::new();
        let mut name_to_info = HashMap::new();
        let mut categories: HashMap<String, HashSet<String>> = HashMap::new();

        if let Some(modules) = data.get("modules").and_then(|v| v.as_array()) {
            for module in modules {
                let module_name = module
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let module_export = module
                    .get("export")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                if let Some(functions) = module.get("functions").and_then(|v| v.as_array()) {
                    for function in functions {
                        let function_name = function
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let function_export = function
                            .get("export")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let docs = function
                            .get("docs")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let return_type = function
                            .get("return")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Void")
                            .to_string();

                        let mut args = Vec::new();
                        if let Some(args_array) = function.get("args").and_then(|v| v.as_array()) {
                            for arg in args_array {
                                let arg_name = arg
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let arg_type = arg
                                    .get("type")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                args.push(SdkArg {
                                    name: arg_name,
                                    ty: arg_type,
                                });
                            }
                        }

                        let info = SdkFunctionInfo {
                            module_name: module_name.clone(),
                            function_name: function_name.clone(),
                            export: function_export.clone(),
                            args,
                            return_type,
                            docs,
                        };

                        // Map by full export path: "module_export.function_export"
                        let full_export = format!("{}.{}", module_export, function_export);
                        export_to_info.insert(full_export, info.clone());

                        // Map by function name
                        name_to_info.insert(function_name.clone(), info.clone());

                        // Categorize by module
                        categories
                            .entry(module_name.clone())
                            .or_insert_with(HashSet::new)
                            .insert(function_name);
                    }
                }
            }
        }

        Ok(Self {
            export_to_info,
            name_to_info,
            categories,
        })
    }

    /// Look up SDK function by export path (e.g., "x.1")
    pub fn get_by_export(&self, export: &str) -> Option<&SdkFunctionInfo> {
        self.export_to_info.get(export)
    }

    /// Look up SDK function by name (e.g., "vec_new_from_linear_memory")
    pub fn get_by_name(&self, name: &str) -> Option<&SdkFunctionInfo> {
        self.name_to_info.get(name)
    }

    /// Check if a function name is an SDK function
    pub fn is_sdk_function(&self, name: &str) -> bool {
        self.name_to_info.contains_key(name)
    }

    /// Get all functions in a category/module
    pub fn get_category_functions(&self, category: &str) -> Option<&HashSet<String>> {
        self.categories.get(category)
    }

    /// Get all available categories
    pub fn get_categories(&self) -> Vec<String> {
        self.categories.keys().cloned().collect()
    }

    /// Check if a function is in a specific category
    pub fn is_in_category(&self, function_name: &str, category: &str) -> bool {
        self.categories
            .get(category)
            .map(|funcs| funcs.contains(function_name))
            .unwrap_or(false)
    }

    /// Get high-level categorization of a function
    pub fn categorize_function(&self, name: &str) -> Option<FunctionCategory> {
        if let Some(info) = self.get_by_name(name) {
            return Some(match info.module_name.as_str() {
                "ledger" => FunctionCategory::Ledger,
                "context" => FunctionCategory::Context,
                "address" => FunctionCategory::Address,
                "buf" | "vec" | "map" => FunctionCategory::Collections,
                "int" => FunctionCategory::Math,
                "prng" => FunctionCategory::Random,
                "call" => FunctionCategory::ContractCall,
                "test" => FunctionCategory::Testing,
                _ => FunctionCategory::Other,
            });
        }
        None
    }

    /// Get all storage-related functions
    pub fn get_storage_functions(&self) -> Vec<String> {
        self.name_to_info
            .keys()
            .filter(|name| {
                name.contains("storage")
                    || name.contains("contract_data")
                    || name.contains("put_")
                    || name.contains("get_")
                    || name.contains("has_")
                    || name.contains("del_")
            })
            .cloned()
            .collect()
    }

    /// Get all event-related functions
    pub fn get_event_functions(&self) -> Vec<String> {
        self.name_to_info
            .keys()
            .filter(|name| name.contains("event"))
            .cloned()
            .collect()
    }

    /// Get all auth-related functions
    pub fn get_auth_functions(&self) -> Vec<String> {
        self.name_to_info
            .keys()
            .filter(|name| {
                name.contains("auth") || name.contains("require_auth") || name.contains("authorize")
            })
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCategory {
    Ledger,
    Context,
    Address,
    Collections,
    Math,
    Random,
    ContractCall,
    Testing,
    Other,
}

impl Default for SdkFunctionDetector {
    fn default() -> Self {
        Self::new().expect("Failed to initialize SDK function detector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_initialization() {
        let detector = SdkFunctionDetector::new().unwrap();
        assert!(!detector.name_to_info.is_empty());
        assert!(!detector.categories.is_empty());
    }

    #[test]
    fn test_lookup_by_name() {
        let detector = SdkFunctionDetector::new().unwrap();
        assert!(detector.get_by_name("vec_new_from_linear_memory").is_some());
        assert!(detector.get_by_name("require_auth").is_some());
        assert!(detector.get_by_name("nonexistent_function").is_none());
    }

    #[test]
    fn test_categorization() {
        let detector = SdkFunctionDetector::new().unwrap();
        let storage_funcs = detector.get_storage_functions();
        assert!(!storage_funcs.is_empty());

        let auth_funcs = detector.get_auth_functions();
        assert!(!auth_funcs.is_empty());
    }
}
