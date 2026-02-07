use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

pub fn try_emit_balance_allowance<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if !ctx.has_datakey_type {
        return false;
    }
    if ctx.export_name == "balance" && spec_fn.inputs().len() == 1 {
        write!(
            writer,
            "    pub fn {}(&mut self, env: Env, {}: {}) -> i128",
            ctx.export_name,
            spec_fn.inputs()[0].name(),
            format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
        )
        .unwrap();
        writeln!(writer, " {{").unwrap();
        writeln!(
            writer,
            "        env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0)",
            spec_fn.inputs()[0].name()
        )
        .unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }
    if ctx.export_name == "allowance"
        && spec_fn.inputs().len() == 2
        && ctx.has_allowance_key_type
        && ctx.has_allowance_value_type
    {
        let from = spec_fn.inputs()[0].name();
        let spender = spec_fn.inputs()[1].name();
        write!(
            writer,
            "    pub fn {}(&mut self, env: Env, {}: {}, {}: {}) -> i128",
            ctx.export_name,
            from,
            format_type_ident(&spec_fn.inputs()[0].type_ident().to_string()),
            spender,
            format_type_ident(&spec_fn.inputs()[1].type_ident().to_string())
        )
        .unwrap();
        writeln!(writer, " {{").unwrap();
        writeln!(
            writer,
            "        let key = DataKey::Allowance(AllowanceDataKey {{ from, spender }});"
        )
        .unwrap();
        writeln!(
            writer,
            "        let allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});"
        )
        .unwrap();
        writeln!(writer, "        allowance.amount").unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }
    false
}

pub fn try_emit_token_actions<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if !ctx.has_datakey_type {
        return false;
    }

    let name = ctx.export_name;
    let params: Vec<&str> = spec_fn.inputs().iter().map(|p| p.name()).collect();

    match name {
        "initialize" | "__constructor" if params.len() >= 1 => {
            if !ctx.has_token_metadata_type {
                return false;
            }
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            if let Some(admin) = params.get(0) {
                writeln!(writer, "        if let Some(_) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {{").unwrap();
                writeln!(writer, "            panic!(\"already initialized\");").unwrap();
                writeln!(writer, "        }}").unwrap();
                writeln!(
                    writer,
                    "        env.storage().instance().set(&DataKey::Admin, &{});",
                    admin
                )
                .unwrap();
            }
            if params.len() >= 4 {
                let decimal = params[1];
                let name_param = params[2];
                let symbol_param = params[3];
                writeln!(writer, "        let metadata = TokenMetadata {{ decimal: {}, name: {}, symbol: {} }};", decimal, name_param, symbol_param).unwrap();
                writeln!(writer, "        env.storage().instance().set(&DataKey::State(env.current_contract_address()), &metadata);").unwrap();
            }
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "set_admin" if params.len() == 1 => {
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(
                writer,
                "        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();"
            )
            .unwrap();
            writeln!(writer, "        admin.require_auth();").unwrap();
            writeln!(
                writer,
                "        env.storage().instance().set(&DataKey::Admin, &{});",
                params[0]
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "mint" if params.len() == 2 => {
            let to = params[0];
            let amount = params[1];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(
                writer,
                "        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();"
            )
            .unwrap();
            writeln!(writer, "        admin.require_auth();").unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                to
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(balance + {}));",
                to, amount
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "burn" if params.len() == 2 => {
            let from = params[0];
            let amount = params[1];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(writer, "        {}.require_auth();", from).unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                from
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(balance - {}));",
                from, amount
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "transfer" if params.len() == 3 => {
            let from = params[0];
            let to = params[1];
            let amount = params[2];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(writer, "        {}.require_auth();", from).unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                from
            )
            .unwrap();
            writeln!(
                writer,
                "        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                to
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(from_balance - {}));",
                from, amount
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(to_balance + {}));",
                to, amount
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "approve" if params.len() == 4 && ctx.has_allowance_key_type && ctx.has_allowance_value_type => {
            let from = params[0];
            let spender = params[1];
            let amount = params[2];
            let expiration = params[3];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(writer, "        {}.require_auth();", from).unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let key = DataKey::Allowance(AllowanceDataKey {{ from: {}, spender: {} }});",
                from, spender
            )
            .unwrap();
            writeln!(
                writer,
                "        let value = AllowanceValue {{ amount: {}, expiration_ledger: {} }};",
                amount, expiration
            )
            .unwrap();
            writeln!(writer, "        if {} > 0 {{", amount).unwrap();
            writeln!(writer, "            env.storage().temporary().set(&key, &value);").unwrap();
            writeln!(writer, "        }} else {{").unwrap();
            writeln!(writer, "            env.storage().temporary().remove(&key);").unwrap();
            writeln!(writer, "        }}").unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "transfer_from" if params.len() == 4 && ctx.has_allowance_key_type && ctx.has_allowance_value_type => {
            let spender = params[0];
            let from = params[1];
            let to = params[2];
            let amount = params[3];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(writer, "        {}.require_auth();", spender).unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let key = DataKey::Allowance(AllowanceDataKey {{ from: {}, spender: {} }});",
                from, spender
            )
            .unwrap();
            writeln!(
                writer,
                "        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});"
            )
            .unwrap();
            writeln!(writer, "        allowance.amount -= {};", amount).unwrap();
            writeln!(writer, "        env.storage().temporary().set(&key, &allowance);").unwrap();
            writeln!(
                writer,
                "        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                from
            )
            .unwrap();
            writeln!(
                writer,
                "        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                to
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(from_balance - {}));",
                from, amount
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(to_balance + {}));",
                to, amount
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "burn_from" if params.len() == 3 && ctx.has_allowance_key_type && ctx.has_allowance_value_type => {
            let spender = params[0];
            let from = params[1];
            let amount = params[2];
            write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
            for argument in spec_fn.inputs() {
                write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
            }
            write!(writer, ")").unwrap();
            writeln!(writer, " {{").unwrap();
            writeln!(writer, "        {}.require_auth();", spender).unwrap();
            writeln!(writer, "        if {} < 0 {{ panic!(\"negative amount\"); }}", amount).unwrap();
            writeln!(
                writer,
                "        let key = DataKey::Allowance(AllowanceDataKey {{ from: {}, spender: {} }});",
                from, spender
            )
            .unwrap();
            writeln!(
                writer,
                "        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue {{ amount: 0, expiration_ledger: 0 }});"
            )
            .unwrap();
            writeln!(writer, "        allowance.amount -= {};", amount).unwrap();
            writeln!(writer, "        env.storage().temporary().set(&key, &allowance);").unwrap();
            writeln!(
                writer,
                "        let balance: i128 = env.storage().persistent().get(&DataKey::Balance({})).unwrap_or(0);",
                from
            )
            .unwrap();
            writeln!(
                writer,
                "        env.storage().persistent().set(&DataKey::Balance({}), &(balance - {}));",
                from, amount
            )
            .unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "decimals" if params.is_empty() => {
            write!(writer, "    pub fn {}(&mut self, env: Env)", name).unwrap();
            if let Some(return_type) = spec_fn.output() {
                write!(writer, " -> {}", format_type_ident(&return_type.type_ident().to_string())).unwrap();
            }
            writeln!(writer, " {{").unwrap();
            writeln!(
                writer,
                "        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();"
            )
            .unwrap();
            writeln!(writer, "        metadata.decimal").unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "name" if params.is_empty() => {
            write!(writer, "    pub fn {}(&mut self, env: Env)", name).unwrap();
            if let Some(return_type) = spec_fn.output() {
                write!(writer, " -> {}", format_type_ident(&return_type.type_ident().to_string())).unwrap();
            }
            writeln!(writer, " {{").unwrap();
            writeln!(
                writer,
                "        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();"
            )
            .unwrap();
            writeln!(writer, "        metadata.name").unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        "symbol" if params.is_empty() => {
            write!(writer, "    pub fn {}(&mut self, env: Env)", name).unwrap();
            if let Some(return_type) = spec_fn.output() {
                write!(writer, " -> {}", format_type_ident(&return_type.type_ident().to_string())).unwrap();
            }
            writeln!(writer, " {{").unwrap();
            writeln!(
                writer,
                "        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();"
            )
            .unwrap();
            writeln!(writer, "        metadata.symbol").unwrap();
            writeln!(writer, "    }}").unwrap();
            return true;
        }
        _ => {}
    }

    false
}

pub fn try_emit_move_token<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    let move_token_shape =
        spec_fn.inputs().len() == 5 && ctx.addr_indices.len() == 3 && ctx.i128_indices.len() == 2;
    let has_transfer_symbol = ctx
        .symbol_literals
        .iter()
        .any(|s| s.as_str() == "transfer");
    if !move_token_shape || !ctx.uses_current_contract_address || !has_transfer_symbol {
        return false;
    }
    let token = spec_fn.inputs()[ctx.addr_indices[0]].name();
    let from = spec_fn.inputs()[ctx.addr_indices[1]].name();
    let to = spec_fn.inputs()[ctx.addr_indices[2]].name();
    let max_spend_amount = spec_fn.inputs()[ctx.i128_indices[0]].name();
    let transfer_amount = spec_fn.inputs()[ctx.i128_indices[1]].name();

    write!(
        writer,
        "    pub fn {}(&mut self, env: Env",
        ctx.export_name
    )
    .unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
    writeln!(writer, " {{").unwrap();
    writeln!(writer, "        let token = token::Client::new(&env, &{});", token).unwrap();
    writeln!(
        writer,
        "        let contract_address = env.current_contract_address();"
    )
    .unwrap();
    writeln!(
        writer,
        "        token.transfer(&{}, &contract_address, &{});",
        from, max_spend_amount
    )
    .unwrap();
    writeln!(
        writer,
        "        token.transfer(&contract_address, &{}, &{});",
        to, transfer_amount
    )
    .unwrap();
    writeln!(
        writer,
        "        token.transfer(&contract_address, &{}, &({} - {}));",
        from, max_spend_amount, transfer_amount
    )
    .unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}
