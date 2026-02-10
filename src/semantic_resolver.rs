use crate::engine::sdk_call_mapper::SdkCallMapper;
use crate::fingerprint::Fingerprint;
use crate::fingerprint_registry;
use std::sync::OnceLock;

pub struct SemanticResolver {
    sdk_mapper: SdkCallMapper,
}

impl SemanticResolver {
    pub fn new() -> Self {
        Self {
            sdk_mapper: SdkCallMapper::default(),
        }
    }

    pub fn canonical_sdk_function_name(&self, name: &str) -> Option<&'static str> {
        let base = strip_numeric_suffix(name);
        match base {
            "storage_get_val" => Some("get_contract_data"),
            "storage_set_val" => Some("put_contract_data"),
            "storage_remove_val" => Some("del_contract_data"),
            "require_owner_auth" => Some("require_auth"),
            "require_auth_for_key" => Some("require_auth_for_args"),
            _ => None,
        }
    }

    pub fn suggested_name_for_fingerprint(&self, fp: &Fingerprint) -> Option<&'static str> {
        fingerprint_registry::suggested_name(fp.hash)
    }

    pub fn resolve_sdk_call(&self, function_name: &str, args: &[String]) -> Option<String> {
        let base = strip_numeric_suffix(function_name);
        match base {
            // Wrapper helper around env.invoke_contract::<Val>(...) with Void assertion.
            "invoke_contract_void" if args.len() == 3 => {
                return Some(format!(
                    "let _ = env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64({})), &Symbol::from_val(env, &val_from_i64({})), Vec::<Val>::from_val(env, &val_from_i64({})))",
                    args[0], args[1], args[2]
                ));
            }
            // Common helper wrapper for ledger timestamp extraction.
            "ledger_timestamp_u64" if args.is_empty() => {
                return Some("env.ledger().timestamp() as i64".to_string());
            }
            _ => {}
        }

        let canonical = self
            .canonical_sdk_function_name(function_name)
            .unwrap_or(function_name);
        self.sdk_mapper.map_call(canonical, args)
    }
}

fn strip_numeric_suffix(name: &str) -> &str {
    if let Some((base, suffix)) = name.rsplit_once('_') {
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return base;
        }
    }
    name
}

pub fn resolver() -> &'static SemanticResolver {
    static RESOLVER: OnceLock<SemanticResolver> = OnceLock::new();
    RESOLVER.get_or_init(SemanticResolver::new)
}
