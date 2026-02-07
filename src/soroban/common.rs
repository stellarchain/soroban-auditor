use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ModuleFunction {
    pub module_name: String,
    pub function_name: String,
    #[allow(dead_code)]
    pub function: Value,
}

fn read_env_common_from_file() -> Result<Value, Box<dyn std::error::Error>> {
    let file_content = include_str!("common_env_soroban.json");
    serde_json::from_str(file_content).map_err(|e| e.into())
}

pub fn env_common_modules_result() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    read_env_common_from_file().and_then(|common_env_imports| {
        common_env_imports
            .get("modules")
            .and_then(Value::as_array)
            .map(|modules| modules.clone())
            .ok_or_else(|| "No modules found in the JSON data.".into())
    })
}

pub fn take_common_module(modules: &[Value], module_name: &str, field_name: &str) -> Option<ModuleFunction> {
    for module in modules {
        if let Some(module_export) = module.get("export").and_then(Value::as_str) {
            if module_export == module_name {
                if let Some(functions) = module.get("functions").and_then(Value::as_array) {
                    for function in functions {
                        if let Some(function_obj) = function.as_object() {
                            if let Some(export_value) = function_obj.get("export").and_then(Value::as_str) {
                                if export_value == field_name {
                                    return Some(ModuleFunction {
                                        module_name: module.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
                                        function_name: function_obj.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
                                        function: function.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn take_common_module_by_name(modules: &[Value], module_name: &str, field_name: &str) -> Option<ModuleFunction> {
  for module in modules {
        if let Some(name_value) = module.get("name").and_then(Value::as_str) {
            if name_value == module_name {
                if let Some(functions) = module.get("functions").and_then(Value::as_array) {
                    for function in functions {
                        if let Some(function_obj) = function.as_object() {
                            if let Some(export_value) = function_obj.get("name").and_then(Value::as_str) {
                                if export_value == field_name {
                                    return Some(ModuleFunction {
                                        module_name: name_value.to_string(),
                                        function_name: function_obj.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
                                        function: function.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
