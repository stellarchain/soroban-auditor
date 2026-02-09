/// Template-based function generation system
///
/// Instead of hardcoding specific functions, this system uses templates
/// to match and generate code for common Soroban patterns.

use regex::Regex;
use crate::soroban::contract::FunctionContractSpec;
use crate::patterns::PatternContext;
use std::collections::HashMap;

/// Condition for selecting template variant
#[derive(Clone, Debug)]
pub enum VariantCondition {
    /// Requires AllowanceDataKey + AllowanceValue structs
    HasAllowanceStructs,
    /// Simple tuple: Allowance(Address, Address)
    SimpleAllowanceTuple,
    /// Always matches (default fallback)
    Always,
}

/// A variant of a template for different DataKey structures
#[derive(Clone)]
pub struct TemplateVariant {
    pub condition: VariantCondition,
    pub template: String,
    pub priority: u32,
}

/// Represents a function template that can match and generate code
#[derive(Clone)]
pub struct FunctionTemplate {
    /// Name pattern (regex) to match function name
    pub name_pattern: String,

    /// Expected number of parameters
    pub param_count_min: usize,
    pub param_count_max: usize,

    /// Type patterns for parameters
    pub param_types: Vec<TypePattern>,

    /// Return type pattern
    pub return_type: Option<TypePattern>,

    /// Body patterns to detect
    pub body_patterns: Vec<BodyPattern>,

    /// Code template to generate (deprecated - use variants instead)
    pub template: String,

    /// Priority (higher = checked first)
    pub priority: u32,

    /// Runtime condition checks
    pub requires_datakey: bool,
    pub requires_allowance_structs: bool,
    pub requires_token_metadata: bool,

    /// Template variants for different structures
    pub variants: Vec<TemplateVariant>,
}

/// Type pattern for matching parameter/return types
#[derive(Clone, Debug, PartialEq)]
pub enum TypePattern {
    Address,
    I128,
    U64,
    U32,
    Bool,
    String,
    Symbol,
    Vec(Box<TypePattern>),
    Map(Box<TypePattern>, Box<TypePattern>),
    Option(Box<TypePattern>),
    Tuple(Vec<TypePattern>),
    Any,
    Named(String),  // For custom types like DataKey
}

/// Body pattern for detecting specific code patterns
#[derive(Clone, Debug)]
pub enum BodyPattern {
    StorageGet {
        storage_type: String,  // "instance", "persistent", "temporary"
        key_pattern: Option<String>,
    },
    StorageSet {
        storage_type: String,
        key_pattern: Option<String>,
    },
    RequireAuth {
        with_args: bool,
    },
    TokenTransfer,
    EventPublish,
    MathOperation {
        operation: String,  // "add", "sub", "mul", "div"
    },
    Any,
}

impl FunctionTemplate {
    /// Check if this template matches the given function
    pub fn matches(&self, spec_fn: &FunctionContractSpec, fn_name: &str) -> bool {
        // Check name pattern
        if let Ok(re) = Regex::new(&self.name_pattern) {
            if !re.is_match(fn_name) {
                return false;
            }
        } else {
            return false;
        }

        // Check parameter count
        let param_count = spec_fn.inputs().len();
        if param_count < self.param_count_min || param_count > self.param_count_max {
            return false;
        }

        // Check parameter types
        for (i, type_pattern) in self.param_types.iter().enumerate() {
            if i >= spec_fn.inputs().len() {
                break;
            }
            let param_type = spec_fn.inputs()[i].type_ident().to_string();
            if !type_pattern.matches(&param_type) {
                return false;
            }
        }

        // Check return type
        if let Some(return_pattern) = &self.return_type {
            if let Some(output) = spec_fn.output() {
                let output_type = output.type_ident().to_string();
                if !return_pattern.matches(&output_type) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Select appropriate variant based on context
    pub fn select_variant<'a>(&'a self, ctx: &PatternContext) -> Option<&'a TemplateVariant> {
        // If no variants, use legacy template field
        if self.variants.is_empty() {
            return None;
        }

        // Check each variant's condition against context
        for variant in &self.variants {
            match &variant.condition {
                VariantCondition::HasAllowanceStructs => {
                    if ctx.has_allowance_key_type && ctx.has_allowance_value_type {
                        return Some(variant);
                    }
                }
                VariantCondition::SimpleAllowanceTuple => {
                    // Check if DataKey::Allowance has 2 fields (Address, Address)
                    let has_simple = ctx.data_key_variants.iter()
                        .any(|v| v.name == "Allowance" && v.fields == 2);
                    if has_simple && !ctx.has_allowance_key_type {
                        return Some(variant);
                    }
                }
                VariantCondition::Always => {
                    return Some(variant);
                }
            }
        }
        None
    }

    /// Generate code from this template
    pub fn generate(&self, spec_fn: &FunctionContractSpec, fn_name: &str) -> String {
        let mut result = self.template.clone();

        // Replace {name}
        result = result.replace("{name}", fn_name);

        // Replace {param0}, {param1}, etc.
        for (i, param) in spec_fn.inputs().iter().enumerate() {
            let param_name = param.name();
            let param_type = crate::format::format_type_ident(&param.type_ident().to_string());
            result = result.replace(&format!("{{param{}}}", i), param_name);
            result = result.replace(&format!("{{param{}_type}}", i), &param_type);
        }

        // Replace {return_type}
        if let Some(output) = spec_fn.output() {
            let return_type = crate::format::format_type_ident(&output.type_ident().to_string());
            result = result.replace("{return_type}", &return_type);
        }

        result
    }
}

impl TypePattern {
    pub fn matches(&self, type_str: &str) -> bool {
        match self {
            TypePattern::Any => true,
            TypePattern::Address => type_str.contains("Address"),
            TypePattern::I128 => type_str.contains("i128"),
            TypePattern::U64 => type_str.contains("u64"),
            TypePattern::U32 => type_str.contains("u32"),
            TypePattern::Bool => type_str.contains("bool"),
            TypePattern::String => type_str.contains("String"),
            TypePattern::Symbol => type_str.contains("Symbol"),
            TypePattern::Vec(_) => type_str.contains("Vec"),
            TypePattern::Map(_, _) => type_str.contains("Map"),
            TypePattern::Option(_) => type_str.contains("Option"),
            TypePattern::Tuple(_) => type_str.starts_with('(') && type_str.ends_with(')'),
            TypePattern::Named(name) => type_str.contains(name),
        }
    }
}

/// Template library containing common Soroban patterns
pub struct TemplateLibrary {
    templates: Vec<FunctionTemplate>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            templates: Vec::new(),
        };

        // Add default templates
        lib.add_balance_template();
        lib.add_transfer_template();
        lib.add_mint_template();
        lib.add_burn_template();
        lib.add_approve_template();
        lib.add_allowance_template();
        lib.add_admin_getter_template();
        lib.add_version_template();
        lib.add_transfer_from_template();
        lib.add_burn_from_template();
        lib.add_decimals_template();
        lib.add_name_template();
        lib.add_symbol_template();
        lib.add_set_admin_template();
        lib.add_initialize_template();
        lib.add_minter_template();
        lib.add_total_supply_template();
        lib.add_set_minter_template();

        lib
    }

    pub fn add_template(&mut self, template: FunctionTemplate) {
        self.templates.push(template);
        // Sort by priority (descending)
        self.templates.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn find_match(&self, spec_fn: &FunctionContractSpec, fn_name: &str) -> Option<&FunctionTemplate> {
        self.templates.iter().find(|t| t.matches(spec_fn, fn_name))
    }

    fn add_balance_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^(get_)?balance$".to_string(),
            param_count_min: 1,
            param_count_max: 1,
            param_types: vec![TypePattern::Address],
            return_type: Some(TypePattern::I128),
            body_patterns: vec![
                BodyPattern::StorageGet {
                    storage_type: "persistent".to_string(),
                    key_pattern: Some("Balance".to_string()),
                },
            ],
            template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) -> i128 {
        env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0)
    }"#.to_string(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![
                TemplateVariant {
                    condition: VariantCondition::Always,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) -> i128 {
        env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0)
    }"#.to_string(),
                    priority: 100,
                }
            ],
        });
    }

    fn add_transfer_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^transfer$".to_string(),
            param_count_min: 3,
            param_count_max: 3,
            param_types: vec![TypePattern::Address, TypePattern::Address, TypePattern::I128],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}) {
        {param0}.require_auth();
        if {param2} < 0 { panic!("negative amount"); }
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param1})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param0}), &(from_balance - {param2}));
        env.storage().persistent().set(&DataKey::Balance({param1}), &(to_balance + {param2}));
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_mint_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^mint$".to_string(),
            param_count_min: 2,
            param_count_max: 2,
            param_types: vec![TypePattern::Address, TypePattern::I128],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if {param1} < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param0}), &(balance + {param1}));
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_burn_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^burn$".to_string(),
            param_count_min: 2,
            param_count_max: 2,
            param_types: vec![TypePattern::Address, TypePattern::I128],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}) {
        {param0}.require_auth();
        if {param1} < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param0}), &(balance - {param1}));
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_approve_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^approve$".to_string(),
            param_count_min: 4,
            param_count_max: 4,
            param_types: vec![TypePattern::Address, TypePattern::Address, TypePattern::I128, TypePattern::U32],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![
                TemplateVariant {
                    condition: VariantCondition::HasAllowanceStructs,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}, {param3}: {param3_type}) {
        {param0}.require_auth();
        if {param2} < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey {{ from: {param0}, spender: {param1} }});
        let value = AllowanceValue {{ amount: {param2}, expiration_ledger: {param3} }};
        if {param2} > 0 {
            env.storage().temporary().set(&key, &value);
        } else {
            env.storage().temporary().remove(&key);
        }
    }"#.to_string(),
                    priority: 100,
                },
                TemplateVariant {
                    condition: VariantCondition::SimpleAllowanceTuple,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}, {param3}: {param3_type}) {
        {param0}.require_auth();
        if {param2} < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance({param0}, {param1});
        if {param2} > 0 {
            env.storage().temporary().set(&key, &{param2});
        } else {
            env.storage().temporary().remove(&key);
        }
    }"#.to_string(),
                    priority: 90,
                },
            ],
        });
    }

    fn add_allowance_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^allowance$".to_string(),
            param_count_min: 2,
            param_count_max: 2,
            param_types: vec![TypePattern::Address, TypePattern::Address],
            return_type: Some(TypePattern::I128),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![
                TemplateVariant {
                    condition: VariantCondition::HasAllowanceStructs,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}) -> i128 {
        let key = DataKey::Allowance(AllowanceDataKey {{ from: {param0}, spender: {param1} }});
        let allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});
        allowance.amount
    }"#.to_string(),
                    priority: 100,
                },
                TemplateVariant {
                    condition: VariantCondition::SimpleAllowanceTuple,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}) -> i128 {
        let key = DataKey::Allowance({param0}, {param1});
        env.storage().temporary().get(&key).unwrap_or(0)
    }"#.to_string(),
                    priority: 90,
                },
            ],
        });
    }

    fn add_admin_getter_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^(get_)?admin$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::Address),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_version_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^version$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::Tuple(vec![TypePattern::U32, TypePattern::U32, TypePattern::U32])),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: false,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> (u32, u32, u32) {
        (0, 0, 0)
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_transfer_from_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^transfer_from$".to_string(),
            param_count_min: 4,
            param_count_max: 4,
            param_types: vec![TypePattern::Address, TypePattern::Address, TypePattern::Address, TypePattern::I128],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![
                TemplateVariant {
                    condition: VariantCondition::HasAllowanceStructs,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}, {param3}: {param3_type}) {
        {param0}.require_auth();
        if {param3} < 0 {{ panic!("negative amount"); }}
        let key = DataKey::Allowance(AllowanceDataKey {{ from: {param1}, spender: {param0} }});
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});
        allowance.amount -= {param3};
        env.storage().temporary().set(&key, &allowance);
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param1})).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param2})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param1}), &(from_balance - {param3}));
        env.storage().persistent().set(&DataKey::Balance({param2}), &(to_balance + {param3}));
    }"#.to_string(),
                    priority: 100,
                },
                TemplateVariant {
                    condition: VariantCondition::SimpleAllowanceTuple,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}, {param3}: {param3_type}) {
        {param0}.require_auth();
        if {param3} < 0 {{ panic!("negative amount"); }}
        let key = DataKey::Allowance({param1}, {param0});
        let mut allowance: i128 = env.storage().temporary().get(&key).unwrap_or(0);
        allowance -= {param3};
        env.storage().temporary().set(&key, &allowance);
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param1})).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance({param2})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param1}), &(from_balance - {param3}));
        env.storage().persistent().set(&DataKey::Balance({param2}), &(to_balance + {param3}));
    }"#.to_string(),
                    priority: 90,
                },
            ],
        });
    }

    fn add_burn_from_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^burn_from$".to_string(),
            param_count_min: 3,
            param_count_max: 3,
            param_types: vec![TypePattern::Address, TypePattern::Address, TypePattern::I128],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![
                TemplateVariant {
                    condition: VariantCondition::HasAllowanceStructs,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}) {
        {param0}.require_auth();
        if {param2} < 0 {{ panic!("negative amount"); }}
        let key = DataKey::Allowance(AllowanceDataKey {{ from: {param1}, spender: {param0} }});
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});
        allowance.amount -= {param2};
        env.storage().temporary().set(&key, &allowance);
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({param1})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param1}), &(balance - {param2}));
    }"#.to_string(),
                    priority: 100,
                },
                TemplateVariant {
                    condition: VariantCondition::SimpleAllowanceTuple,
                    template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}, {param1}: {param1_type}, {param2}: {param2_type}) {
        {param0}.require_auth();
        if {param2} < 0 {{ panic!("negative amount"); }}
        let key = DataKey::Allowance({param1}, {param0});
        let mut allowance: i128 = env.storage().temporary().get(&key).unwrap_or(0);
        allowance -= {param2};
        env.storage().temporary().set(&key, &allowance);
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({param1})).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance({param1}), &(balance - {param2}));
    }"#.to_string(),
                    priority: 90,
                },
            ],
        });
    }

    fn add_decimals_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^decimals$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::U32),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: false,
            requires_allowance_structs: false,
            requires_token_metadata: true,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> u32 {{
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.decimal
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_name_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^name$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::String),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: false,
            requires_allowance_structs: false,
            requires_token_metadata: true,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> soroban_sdk::String {{
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.name
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_symbol_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^symbol$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::String),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: false,
            requires_allowance_structs: false,
            requires_token_metadata: true,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> soroban_sdk::String {{
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.symbol
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_set_admin_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^set_admin$".to_string(),
            param_count_min: 1,
            param_count_max: 1,
            param_types: vec![TypePattern::Address],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) {{
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &{param0});
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_initialize_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^(initialize|__constructor)$".to_string(),
            param_count_min: 1,
            param_count_max: 5,
            param_types: vec![TypePattern::Address],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) {{
        if let Some(_) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {{
            panic!("already initialized");
        }}
        env.storage().instance().set(&DataKey::Admin, &{param0});
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_minter_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^(get_)?minter$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::Address),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> Address {
        env.storage().instance().get(&DataKey::Minter).unwrap()
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_total_supply_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^total_supply$".to_string(),
            param_count_min: 0,
            param_count_max: 0,
            param_types: vec![],
            return_type: Some(TypePattern::I128),
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }"#.to_string(),
                priority: 100,
            }],
        });
    }

    fn add_set_minter_template(&mut self) {
        self.add_template(FunctionTemplate {
            name_pattern: r"^set_minter$".to_string(),
            param_count_min: 1,
            param_count_max: 1,
            param_types: vec![TypePattern::Address],
            return_type: None,
            body_patterns: vec![],
            template: String::new(),
            priority: 100,
            requires_datakey: true,
            requires_allowance_structs: false,
            requires_token_metadata: false,
            variants: vec![TemplateVariant {
                condition: VariantCondition::Always,
                template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) {{
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Minter, &{param0});
    }}"#.to_string(),
                priority: 100,
            }],
        });
    }
}

/// Template matcher - decides which template to use for a function
pub struct TemplateMatcher {
    library: TemplateLibrary,
}

impl TemplateMatcher {
    pub fn new() -> Self {
        Self {
            library: TemplateLibrary::new(),
        }
    }

    pub fn with_library(library: TemplateLibrary) -> Self {
        Self { library }
    }

    /// Try to match and generate code for a function (legacy - without context)
    pub fn try_generate(&self, spec_fn: &FunctionContractSpec, fn_name: &str) -> Option<String> {
        if let Some(template) = self.library.find_match(spec_fn, fn_name) {
            Some(template.generate(spec_fn, fn_name))
        } else {
            None
        }
    }

    /// Try to match and generate code with context awareness
    pub fn try_generate_with_context(
        &self,
        spec_fn: &FunctionContractSpec,
        fn_name: &str,
        ctx: &PatternContext,
    ) -> Option<String> {
        // Find matching template
        let template = self.library.find_match(spec_fn, fn_name)?;

        // Check runtime conditions
        if template.requires_datakey && !ctx.has_datakey_type {
            return None;
        }

        if template.requires_allowance_structs {
            if !ctx.has_allowance_key_type || !ctx.has_allowance_value_type {
                return None;
            }
        }

        if template.requires_token_metadata && !ctx.has_token_metadata_type {
            return None;
        }

        // Select appropriate variant
        if let Some(variant) = template.select_variant(ctx) {
            Some(self.generate_from_variant(variant, spec_fn, fn_name))
        } else {
            // Fallback to legacy template if no variants
            Some(template.generate(spec_fn, fn_name))
        }
    }

    /// Generate code from a template variant
    fn generate_from_variant(
        &self,
        variant: &TemplateVariant,
        spec_fn: &FunctionContractSpec,
        fn_name: &str,
    ) -> String {
        let mut result = variant.template.clone();

        // Replace {name}
        result = result.replace("{name}", fn_name);

        // Replace {param0}, {param1}, etc.
        for (i, param) in spec_fn.inputs().iter().enumerate() {
            let param_name = param.name();
            let param_type = crate::format::format_type_ident(&param.type_ident().to_string());
            result = result.replace(&format!("{{param{}}}", i), param_name);
            result = result.replace(&format!("{{param{}_type}}", i), &param_type);
        }

        // Replace {return_type}
        if let Some(output) = spec_fn.output() {
            let return_type = crate::format::format_type_ident(&output.type_ident().to_string());
            result = result.replace("{return_type}", &return_type);
        }

        // Fix escaped braces: {{ -> { and }} -> }
        // This is needed because raw strings preserve literal {{ and }}
        result = result.replace("{{", "{").replace("}}", "}");

        result
    }

    /// Add a custom template
    pub fn add_template(&mut self, template: FunctionTemplate) {
        self.library.add_template(template);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_pattern_matching() {
        assert!(TypePattern::Address.matches("soroban_sdk::Address"));
        assert!(TypePattern::I128.matches("i128"));
        assert!(TypePattern::Vec(Box::new(TypePattern::Any)).matches("Vec<Symbol>"));
        assert!(!TypePattern::Address.matches("i128"));
    }

    #[test]
    fn test_template_name_matching() {
        let template = FunctionTemplate {
            name_pattern: r"^(get_)?balance$".to_string(),
            param_count_min: 1,
            param_count_max: 1,
            param_types: vec![TypePattern::Address],
            return_type: Some(TypePattern::I128),
            body_patterns: vec![],
            template: "".to_string(),
            priority: 100,
        };

        // Should match "balance" and "get_balance"
        // Note: We can't easily test matches() without a real FunctionContractSpec
        // This is just a placeholder test
    }
}
