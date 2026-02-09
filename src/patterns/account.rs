use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

fn error_type_from_output(spec_fn: &FunctionContractSpec) -> Option<String> {
    let out = spec_fn.output()?.type_ident().to_string();
    if out.contains("AccError") {
        return Some("AccError".to_string());
    }
    if out.contains("AccountError") {
        return Some("AccountError".to_string());
    }
    if out.contains("Error") {
        return Some("Error".to_string());
    }
    None
}

pub fn try_emit_constructor<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    let name = ctx.export_name;
    if name != "__constructor" && name != "___constructor" {
        if name != "init" {
            return false;
        }
    }

    // Constructor with signers list
    if spec_fn.inputs().len() == 1
        && format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
            .contains("Vec")
        && format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
            .contains("BytesN<32>")
        && (ctx.has_signer_variant || ctx.is_account_contract)
    {
        let emit_name = if ctx.is_account_contract && name == "___constructor" {
            "__constructor"
        } else {
            name
        };
        if ctx.is_account_contract {
            write!(writer, "    pub fn {}(env: Env", emit_name).unwrap();
            let arg_name = spec_fn.inputs()[0].name();
            write!(writer, ", {}: Vec<BytesN<32>>", arg_name).unwrap();
            write!(writer, ")").unwrap();
        } else {
            write!(writer, "    pub fn {}(&mut self, env: Env", emit_name).unwrap();
            for argument in spec_fn.inputs() {
                write!(
                    writer,
                    ", {}: {}",
                    argument.name(),
                    format_type_ident(&argument.type_ident().to_string())
                )
                .unwrap();
            }
            write!(writer, ")").unwrap();
        }
        writeln!(writer, " {{").unwrap();
        let signers = spec_fn.inputs()[0].name();
        writeln!(writer, "        for signer in {}.iter() {{", signers).unwrap();
        writeln!(writer, "            env.storage().instance().set(&DataKey::Signer(signer), &());").unwrap();
        writeln!(writer, "        }}").unwrap();
        if ctx.has_signer_cnt_variant || ctx.is_account_contract {
            writeln!(writer, "        env.storage().instance().set(&DataKey::SignerCnt, &{}.len());", signers).unwrap();
        }
        writeln!(writer, "    }}").unwrap();
        return true;
    }

    // Constructor with admin + signer
    if spec_fn.inputs().len() == 2
        && format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
            .contains("Address")
        && format_type_ident(&spec_fn.inputs()[1].type_ident().to_string())
            .contains("BytesN<32>")
        && ctx.has_admin_variant
        && ctx.has_signer_variant
    {
        write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
        for argument in spec_fn.inputs() {
            write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
        }
        write!(writer, ")").unwrap();
        writeln!(writer, " {{").unwrap();
        let admin = spec_fn.inputs()[0].name();
        let signer = spec_fn.inputs()[1].name();
        writeln!(writer, "        env.storage().instance().set(&DataKey::Admin, &{});", admin).unwrap();
        writeln!(writer, "        env.storage().instance().set(&DataKey::Signer({}), &());", signer).unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }

    // Constructor with owner public key
    if spec_fn.inputs().len() == 1
        && format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
            .contains("BytesN<32>")
        && ctx.has_owner_variant
    {
        write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
        for argument in spec_fn.inputs() {
            write!(
                writer,
                ", {}: {}",
                argument.name(),
                format_type_ident(&argument.type_ident().to_string())
            )
            .unwrap();
        }
        write!(writer, ")").unwrap();
        writeln!(writer, " {{").unwrap();
        if ctx
            .string_literals
            .iter()
            .any(|s| s.as_str() == "owner is already set")
        {
            writeln!(
                writer,
                "        if env.storage().instance().has(&DataKey::Owner) {{"
            )
            .unwrap();
            writeln!(writer, "            panic!(\"owner is already set\");").unwrap();
            writeln!(writer, "        }}").unwrap();
        }
        let owner = spec_fn.inputs()[0].name();
        writeln!(writer, "        env.storage().instance().set(&DataKey::Owner, &{});", owner).unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }

    false
}

pub fn try_emit_add_limit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if ctx.export_name != "add_limit"
        || spec_fn.inputs().len() != 2
        || (!ctx.has_spend_limit_variant && !ctx.is_account_contract)
    {
        return false;
    }
    if ctx.is_account_contract {
        write!(writer, "    pub fn {}(env: Env", ctx.export_name).unwrap();
        let token = spec_fn.inputs()[0].name();
        let limit = spec_fn.inputs()[1].name();
        write!(writer, ", {}: Address, {}: i128", token, limit).unwrap();
        write!(writer, ")").unwrap();
    } else {
        write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
        for argument in spec_fn.inputs() {
            write!(
                writer,
                ", {}: {}",
                argument.name(),
                format_type_ident(&argument.type_ident().to_string())
            )
            .unwrap();
        }
        write!(writer, ")").unwrap();
    }
    writeln!(writer, " {{").unwrap();
    let token = spec_fn.inputs()[0].name();
    let limit = spec_fn.inputs()[1].name();
    writeln!(writer, "        env.current_contract_address().require_auth();").unwrap();
    writeln!(writer, "        env.storage().instance().set(&DataKey::SpendLimit({}), &{});", token, limit).unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}

pub fn try_emit_check_auth<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if ctx.export_name != "__check_auth" && ctx.export_name != "___check_auth" {
        return false;
    }

    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
    if let Some(return_type) = spec_fn.output() {
        write!(writer, " -> {}", format_type_ident(&return_type.type_ident().to_string())).unwrap();
    }
    writeln!(writer, " {{").unwrap();

    let output_err = error_type_from_output(spec_fn);
    let signature_payload = spec_fn.inputs().get(0).map(|p| p.name()).unwrap_or("signature_payload");
    let signatures = spec_fn.inputs().get(1).map(|p| p.name()).unwrap_or("signatures");

    let sig_ty = spec_fn
        .inputs()
        .get(1)
        .map(|p| format_type_ident(&p.type_ident().to_string()))
        .unwrap_or_default();

    if sig_ty.contains("Vec") && (sig_ty.contains("Signature") || sig_ty.contains("AccSignature")) {
        writeln!(writer, "        for signature in {}.iter() {{", signatures).unwrap();
        writeln!(writer, "            if !env.storage().instance().has(&DataKey::Signer(signature.public_key.clone())) {{").unwrap();
        if let Some(err) = output_err.as_ref() {
            writeln!(writer, "                return Err({}::UnknownSigner);", err).unwrap();
        } else {
            writeln!(writer, "                return Err(());" ).unwrap();
        }
        writeln!(writer, "            }}").unwrap();
        writeln!(writer, "            env.crypto().ed25519_verify(&signature.public_key, &{}.clone().into(), &signature.signature);", signature_payload).unwrap();
        writeln!(writer, "        }}").unwrap();
        writeln!(writer, "        Ok(())").unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }

    if sig_ty.contains("BytesN<64>") && ctx.has_owner_variant {
        writeln!(writer, "        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::Owner).unwrap();").unwrap();
        writeln!(writer, "        env.crypto().ed25519_verify(&public_key, &{}.into(), &{});", signature_payload, signatures).unwrap();
        writeln!(writer, "    }}").unwrap();
        return true;
    }

    if output_err.is_some() {
        writeln!(writer, "        Ok(())").unwrap();
        writeln!(writer, "    }}").unwrap();
    } else {
        writeln!(writer, "        // TODO: verify signatures").unwrap();
        writeln!(writer, "    }}").unwrap();
    }
    true
}

pub fn try_emit_check_auth_trait<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if !ctx.is_account_contract {
        return false;
    }
    if ctx.export_name != "__check_auth" && ctx.export_name != "___check_auth" {
        return false;
    }
    writeln!(writer, "    #[allow(non_snake_case)]").unwrap();
    write!(writer, "    fn __check_auth(env: Env").unwrap();
    let signature_payload = spec_fn
        .inputs()
        .get(0)
        .map(|p| p.name())
        .unwrap_or("signature_payload");
    let signatures = spec_fn.inputs().get(1).map(|p| p.name()).unwrap_or("signatures");
    let auth_context = spec_fn.inputs().get(2).map(|p| p.name()).unwrap_or("auth_context");
    write!(writer, ", {}: Hash<32>", signature_payload).unwrap();
    write!(writer, ", {}: Self::Signature", signatures).unwrap();
    write!(writer, ", {}: Vec<Context>", auth_context).unwrap();
    write!(writer, ")").unwrap();
    if let Some(return_type) = spec_fn.output() {
        write!(
            writer,
            " -> {}",
            format_type_ident(&return_type.type_ident().to_string())
        )
        .unwrap();
    }
    writeln!(writer, " {{").unwrap();
    writeln!(
        writer,
        "        authenticate(&env, &{}, &{})?;",
        signature_payload,
        signatures
    )
    .unwrap();
    writeln!(
        writer,
        "        let tot_signers: u32 = env.storage().instance().get::<_, u32>(&DataKey::SignerCnt).unwrap();"
    )
    .unwrap();
    writeln!(
        writer,
        "        let all_signed = tot_signers == signatures.len();"
    )
    .unwrap();
    writeln!(
        writer,
        "        let curr_contract = env.current_contract_address();"
    )
    .unwrap();
    writeln!(
        writer,
        "        let mut spend_left_per_token = Map::<Address, i128>::new(&env);"
    )
    .unwrap();
    writeln!(writer, "        for context in {}.iter() {{", auth_context).unwrap();
    writeln!(
        writer,
        "            verify_authorization_policy(&env, &context, &curr_contract, all_signed, &mut spend_left_per_token)?;"
    )
    .unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(writer, "        Ok(())").unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}

pub fn emit_account_helpers<W: Write>(writer: &mut W) {
    writeln!(
        writer,
        "fn authenticate(env: &Env, signature_payload: &Hash<32>, signatures: &Vec<AccSignature>) -> Result<(), AccError> {{"
    )
    .unwrap();
    writeln!(writer, "    for i in 0..signatures.len() {{").unwrap();
    writeln!(writer, "        let signature = signatures.get_unchecked(i);").unwrap();
    writeln!(writer, "        if i > 0 {{").unwrap();
    writeln!(
        writer,
        "            let prev_signature = signatures.get_unchecked(i - 1);"
    )
    .unwrap();
    writeln!(
        writer,
        "            if prev_signature.public_key >= signature.public_key {{"
    )
    .unwrap();
    writeln!(writer, "                return Err(AccError::BadSignatureOrder);").unwrap();
    writeln!(writer, "            }}").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(
        writer,
        "        if !env.storage().instance().has(&DataKey::Signer(signature.public_key.clone())) {{"
    )
    .unwrap();
    writeln!(writer, "            return Err(AccError::UnknownSigner);").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(
        writer,
        "        env.crypto().ed25519_verify(&signature.public_key, &signature_payload.clone().into(), &signature.signature);"
    )
    .unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "    Ok(())").unwrap();
    writeln!(writer, "}}").unwrap();
    writeln!(writer).unwrap();
    writeln!(
        writer,
        "fn verify_authorization_policy(env: &Env, context: &Context, curr_contract: &Address, all_signed: bool, spend_left_per_token: &mut Map<Address, i128>) -> Result<(), AccError> {{"
    )
    .unwrap();
    writeln!(writer, "    if all_signed {{").unwrap();
    writeln!(writer, "        return Ok(());").unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "    let contract_context = match context {{").unwrap();
    writeln!(writer, "        Context::Contract(c) => {{").unwrap();
    writeln!(writer, "            if &c.contract == curr_contract {{").unwrap();
    writeln!(writer, "                return Err(AccError::NotEnoughSigners);").unwrap();
    writeln!(writer, "            }}").unwrap();
    writeln!(writer, "            c").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(
        writer,
        "        Context::CreateContractHostFn(_) | Context::CreateContractWithCtorHostFn(_) => {{"
    )
    .unwrap();
    writeln!(writer, "            return Err(AccError::NotEnoughSigners);").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(writer, "    }};").unwrap();
    writeln!(
        writer,
        "    if contract_context.fn_name != TRANSFER_FN && contract_context.fn_name != APPROVE_FN && contract_context.fn_name != BURN_FN {{"
    )
    .unwrap();
    writeln!(writer, "        return Ok(());").unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(
        writer,
        "    let spend_left: Option<i128> = if let Some(spend_left) = spend_left_per_token.get(contract_context.contract.clone()) {{"
    )
    .unwrap();
    writeln!(writer, "        Some(spend_left)").unwrap();
    writeln!(
        writer,
        "    }} else if let Some(limit_left) = env.storage().instance().get::<_, i128>(&DataKey::SpendLimit(contract_context.contract.clone())) {{"
    )
    .unwrap();
    writeln!(writer, "        Some(limit_left)").unwrap();
    writeln!(writer, "    }} else {{").unwrap();
    writeln!(writer, "        None").unwrap();
    writeln!(writer, "    }};").unwrap();
    writeln!(writer, "    if let Some(spend_left) = spend_left {{").unwrap();
    writeln!(
        writer,
        "        let spent: i128 = contract_context.args.get(2).unwrap().try_into_val(env).unwrap();"
    )
    .unwrap();
    writeln!(writer, "        if spent < 0 {{").unwrap();
    writeln!(writer, "            return Err(AccError::NegativeAmount);").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(
        writer,
        "        if !all_signed && spent > spend_left {{"
    )
    .unwrap();
    writeln!(writer, "            return Err(AccError::NotEnoughSigners);").unwrap();
    writeln!(writer, "        }}").unwrap();
    writeln!(
        writer,
        "        spend_left_per_token.set(contract_context.contract.clone(), spend_left - spent);"
    )
    .unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "    Ok(())").unwrap();
    writeln!(writer, "}}").unwrap();
}

pub fn try_emit_upgrade<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if ctx.export_name != "upgrade" || !ctx.uses_update_current_contract_wasm {
        return false;
    }
    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
    writeln!(writer, " {{").unwrap();
    if ctx.has_admin_variant {
        writeln!(writer, "        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();").unwrap();
        writeln!(writer, "        admin.require_auth();").unwrap();
    }
    if let Some(arg) = spec_fn.inputs().get(0) {
        writeln!(writer, "        env.deployer().update_current_contract_wasm({});", arg.name()).unwrap();
    }
    writeln!(writer, "    }}").unwrap();
    true
}

pub fn try_emit_increment_counter<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if ctx.export_name != "increment" || spec_fn.inputs().len() != 2 || !ctx.has_counter_variant {
        return false;
    }
    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
    if let Some(return_type) = spec_fn.output() {
        write!(writer, " -> {}", format_type_ident(&return_type.type_ident().to_string())).unwrap();
    }
    writeln!(writer, " {{").unwrap();
    let user = spec_fn.inputs()[0].name();
    let value = spec_fn.inputs()[1].name();
    writeln!(writer, "        {}.require_auth();", user).unwrap();
    writeln!(writer, "        let key = DataKey::Counter({}.clone());", user).unwrap();
    writeln!(writer, "        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or_default();").unwrap();
    writeln!(writer, "        count += {};", value).unwrap();
    writeln!(writer, "        env.storage().persistent().set(&key, &count);").unwrap();
    writeln!(writer, "        count").unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}
