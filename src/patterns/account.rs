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
        && ctx.has_signer_variant
    {
        write!(writer, "    pub fn {}(&mut self, env: Env", name).unwrap();
        for argument in spec_fn.inputs() {
            write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
        }
        write!(writer, ")").unwrap();
        writeln!(writer, " {{").unwrap();
        let signers = spec_fn.inputs()[0].name();
        writeln!(writer, "        for signer in {}.iter() {{", signers).unwrap();
        writeln!(writer, "            env.storage().instance().set(&DataKey::Signer(signer), &());").unwrap();
        writeln!(writer, "        }}").unwrap();
        if ctx.has_signer_cnt_variant {
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
    if ctx.export_name != "add_limit" || spec_fn.inputs().len() != 2 || !ctx.has_spend_limit_variant {
        return false;
    }
    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
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
