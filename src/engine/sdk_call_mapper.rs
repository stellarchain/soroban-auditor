/// SDK Call Mapper - Transforms low-level SDK calls to high-level Rust syntax
/// Uses SdkFunctionDetector for data-driven mapping (NO HARDCODING!)
use crate::sdk::SdkFunctionDetector;
use std::collections::HashMap;

pub struct SdkCallMapper {
    detector: SdkFunctionDetector,
    // Cache for quick lookups
    special_mappings: HashMap<String, MappingRule>,
    // Fingerprint-derived helper aliases -> canonical SDK call names
    aliases: HashMap<String, String>,
}

#[derive(Clone)]
enum MappingRule {
    /// Method call: vec_len(x) → x.len()
    MethodCall {
        method_name: String,
        receiver_arg: usize,
    },
    /// Static method: vec_new() → Vec::new(env)
    StaticMethod {
        type_name: String,
        method_name: String,
    },
    /// Namespace call: bls12_381_g1_mul(a, b) → env.crypto().bls12_381().g1_mul(&a, &b)
    NamespaceCall {
        namespace: Vec<String>,
        method: String,
    },
    /// Direct function: compute_hash_sha256(data) → env.crypto().compute_hash_sha256(data)
    DirectFunction { prefix: String },
    /// Keep as-is
    Passthrough,
}

impl SdkCallMapper {
    pub fn new() -> Self {
        let detector = SdkFunctionDetector::default();
        let mut mapper = Self {
            detector,
            special_mappings: HashMap::new(),
            aliases: HashMap::new(),
        };

        mapper.initialize_mappings();
        mapper
    }

    fn initialize_mappings(&mut self) {
        // Fingerprint-suggested helper names (from rewrites) mapped to canonical SDK calls.
        self.aliases
            .insert("storage_get_val".to_string(), "get_contract_data".to_string());
        self.aliases
            .insert("storage_set_val".to_string(), "put_contract_data".to_string());
        self.aliases
            .insert("require_owner_auth".to_string(), "require_auth".to_string());
        self.aliases.insert(
            "require_auth_for_key".to_string(),
            "require_auth_for_args".to_string(),
        );

        // Vec operations → method calls
        self.add_method("vec_len", "len", 0);
        self.add_method("vec_is_empty", "is_empty", 0);
        self.add_method("vec_first", "first", 0);
        self.add_method("vec_last", "last", 0);

        // String operations → method calls
        self.add_method("string_len", "len", 0);
        self.add_method("bytes_len", "len", 0);

        // Map operations → method calls
        self.add_method("map_len", "len", 0);
        self.add_method("map_keys", "keys", 0);

        // BLS12-381 crypto → namespace calls
        self.add_namespace("bls12_381_g1_add", vec!["crypto", "bls12_381"], "g1_add");
        self.add_namespace("bls12_381_g1_mul", vec!["crypto", "bls12_381"], "g1_mul");
        self.add_namespace("bls12_381_g1_msm", vec!["crypto", "bls12_381"], "g1_msm");
        self.add_namespace(
            "bls12_381_map_to_g1",
            vec!["crypto", "bls12_381"],
            "map_to_g1",
        );
        self.add_namespace("bls12_381_g2_add", vec!["crypto", "bls12_381"], "g2_add");
        self.add_namespace("bls12_381_g2_mul", vec!["crypto", "bls12_381"], "g2_mul");
        self.add_namespace("bls12_381_g2_msm", vec!["crypto", "bls12_381"], "g2_msm");
        self.add_namespace(
            "bls12_381_map_to_g2",
            vec!["crypto", "bls12_381"],
            "map_to_g2",
        );
        self.add_namespace(
            "bls12_381_pairing_check",
            vec!["crypto", "bls12_381"],
            "pairing_check",
        );
        self.add_namespace(
            "bls12_381_multi_pairing_check",
            vec!["crypto", "bls12_381"],
            "multi_pairing_check",
        );
        self.add_namespace("bls12_381_fr_add", vec!["crypto", "bls12_381"], "fr_add");
        self.add_namespace("bls12_381_fr_sub", vec!["crypto", "bls12_381"], "fr_sub");
        self.add_namespace("bls12_381_fr_mul", vec!["crypto", "bls12_381"], "fr_mul");
        self.add_namespace("bls12_381_fr_pow", vec!["crypto", "bls12_381"], "fr_pow");
        self.add_namespace("bls12_381_fr_inv", vec!["crypto", "bls12_381"], "fr_inv");

        // Hash functions → direct
        self.add_direct("compute_hash_sha256", "crypto");
        self.add_direct("compute_hash_keccak256", "crypto");

        // Storage operations → direct
        self.add_direct("get_contract_data", "storage");
        self.add_direct("has_contract_data", "storage");
        self.add_direct("put_contract_data", "storage");
        self.add_direct("del_contract_data", "storage");
    }

    fn add_method(&mut self, sdk_name: &str, method_name: &str, receiver_arg: usize) {
        self.special_mappings.insert(
            sdk_name.to_string(),
            MappingRule::MethodCall {
                method_name: method_name.to_string(),
                receiver_arg,
            },
        );
    }

    fn add_namespace(&mut self, sdk_name: &str, namespace: Vec<&str>, method: &str) {
        self.special_mappings.insert(
            sdk_name.to_string(),
            MappingRule::NamespaceCall {
                namespace: namespace.iter().map(|s| s.to_string()).collect(),
                method: method.to_string(),
            },
        );
    }

    fn add_direct(&mut self, sdk_name: &str, prefix: &str) {
        self.special_mappings.insert(
            sdk_name.to_string(),
            MappingRule::DirectFunction {
                prefix: prefix.to_string(),
            },
        );
    }

    /// Map an SDK function call to high-level Rust syntax
    pub fn map_call(&self, sdk_function: &str, args: &[String]) -> Option<String> {
        let canonical = self
            .aliases
            .get(sdk_function)
            .map(|s| s.as_str())
            .unwrap_or(sdk_function);

        // Check if we have a special mapping
        if let Some(rule) = self.special_mappings.get(canonical) {
            return Some(self.apply_rule(rule, canonical, args));
        }

        // Check if it's a known SDK function
        if let Some(info) = self.detector.get_by_name(canonical) {
            // Use module name to infer structure
            match info.module_name.as_str() {
                "vec" | "map" | "buf" => {
                    // Collection operations - try method call
                    if let Some(method) = self.infer_method_name(canonical) {
                        if !args.is_empty() {
                            return Some(format!(
                                "{}.{}({})",
                                args[0],
                                method,
                                args[1..].join(", ")
                            ));
                        }
                    }
                }
                "crypto" => {
                    // Crypto operations - use env.crypto()
                    return Some(format!(
                        "env.crypto().{}({})",
                        canonical,
                        args.join(", ")
                    ));
                }
                "ledger" => {
                    // Ledger operations - use env.storage() or env.ledger()
                    if canonical.contains("contract_data") {
                        return Some(format!(
                            "env.storage().{}({})",
                            canonical,
                            args.join(", ")
                        ));
                    }
                }
                _ => {}
            }
        }

        // Fallback: keep original name
        None
    }

    fn apply_rule(&self, rule: &MappingRule, sdk_function: &str, args: &[String]) -> String {
        match rule {
            MappingRule::MethodCall {
                method_name,
                receiver_arg,
            } => {
                if args.len() > *receiver_arg {
                    let receiver = &args[*receiver_arg];
                    let other_args: Vec<_> = args
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i != receiver_arg)
                        .map(|(_, arg)| arg.as_str())
                        .collect();

                    if other_args.is_empty() {
                        format!("{}.{}()", receiver, method_name)
                    } else {
                        format!("{}.{}({})", receiver, method_name, other_args.join(", "))
                    }
                } else {
                    format!("{}({})", sdk_function, args.join(", "))
                }
            }
            MappingRule::StaticMethod {
                type_name,
                method_name,
            } => {
                format!("{}::{}(env, {})", type_name, method_name, args.join(", "))
            }
            MappingRule::NamespaceCall { namespace, method } => {
                let ns_path = namespace
                    .iter()
                    .map(|s| format!("{}()", s))
                    .collect::<Vec<_>>()
                    .join(".");

                // Add references to complex types
                let formatted_args: Vec<String> = args
                    .iter()
                    .map(|arg| {
                        // If argument looks like a complex value, add &
                        if arg.starts_with("val_from") || arg.contains("::") {
                            format!("&{}", arg)
                        } else {
                            arg.clone()
                        }
                    })
                    .collect();

                format!("env.{}.{}({})", ns_path, method, formatted_args.join(", "))
            }
            MappingRule::DirectFunction { prefix } => {
                format!("env.{}().{}({})", prefix, sdk_function, args.join(", "))
            }
            MappingRule::Passthrough => {
                format!("{}({})", sdk_function, args.join(", "))
            }
        }
    }

    fn infer_method_name(&self, sdk_function: &str) -> Option<String> {
        // Try to extract method name from SDK function name
        // E.g., "vec_push_back" → "push_back"
        if let Some(underscore_pos) = sdk_function.find('_') {
            Some(sdk_function[underscore_pos + 1..].to_string())
        } else {
            None
        }
    }

    /// Check if a function is a known SDK function
    pub fn is_sdk_function(&self, name: &str) -> bool {
        self.detector.is_sdk_function(name)
    }

    /// Get info about an SDK function
    pub fn get_function_info(&self, name: &str) -> Option<crate::sdk::SdkFunctionInfo> {
        self.detector.get_by_name(name).cloned()
    }
}

impl Default for SdkCallMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_method_mapping() {
        let mapper = SdkCallMapper::new();

        let result = mapper.map_call("vec_len", &["my_vec".to_string()]);
        assert_eq!(result, Some("my_vec.len()".to_string()));
    }

    #[test]
    fn test_bls_namespace_mapping() {
        let mapper = SdkCallMapper::new();

        let result = mapper.map_call(
            "bls12_381_g1_mul",
            &["point".to_string(), "scalar".to_string()],
        );
        assert!(result.unwrap().contains("env.crypto().bls12_381().g1_mul"));
    }

    #[test]
    fn test_unknown_function() {
        let mapper = SdkCallMapper::new();

        let result = mapper.map_call("unknown_func", &["arg1".to_string()]);
        assert!(result.is_none());
    }

    #[test]
    fn test_fingerprint_alias_mapping() {
        let mapper = SdkCallMapper::new();
        let result = mapper.map_call("storage_get_val", &["key".to_string()]);
        assert_eq!(result, Some("env.storage().get_contract_data(key)".to_string()));
    }
}
