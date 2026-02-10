use crate::format::format_spec_tokens;
use crate::sdk::ContractSpecs;
use std::io::Write;

const ACCOUNT_FALLBACK_DECLS: &str = r#"
#[contracttype]
#[derive(Clone)]
pub struct AccSignature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    SignerCnt,
    Signer(BytesN<32>),
    SpendLimit(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccError {
    NotEnoughSigners = 1,
    NegativeAmount = 2,
    BadSignatureOrder = 3,
    UnknownSigner = 4,
}

const TRANSFER_FN: Symbol = symbol_short!("transfer");
const APPROVE_FN: Symbol = symbol_short!("approve");
const BURN_FN: Symbol = symbol_short!("burn");
"#;

pub fn emit_contract_scaffold<W: Write>(
    writer: &mut W,
    import_line: &str,
    contract_name: &str,
    is_account_contract: bool,
    contract_specs: &ContractSpecs,
) -> Result<(), String> {
    let contract_struct_vis = if is_account_contract { "" } else { "pub " };
    writeln!(
        writer,
        "#![no_std]\n{}\n\n#[contract]\n{}struct {};",
        import_line, contract_struct_vis, contract_name
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer).map_err(|e| e.to_string())?;
    writeln!(writer, "fn val_from_i64(v: i64) -> Val {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "    unsafe {{ core::mem::transmute::<u64, Val>(v as u64) }}"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(writer, "fn val_to_i64(v: Val) -> i64 {{").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "    (unsafe {{ core::mem::transmute::<Val, u64>(v) }}) as i64"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(writer, "fn err_contract(code: u32) -> i64 {{").map_err(|e| e.to_string())?;
    writeln!(writer, "    ((soroban_sdk::xdr::ScErrorType::Contract as u32 as i64) & 255).wrapping_shl(32 as u32) | (code as i64)").map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;
    writeln!(
        writer,
        "fn address_from_i64(env: &Env, v: i64) -> Address {{"
    )
    .map_err(|e| e.to_string())?;
    writeln!(writer, "    Address::from_val(env, &val_from_i64(v))").map_err(|e| e.to_string())?;
    writeln!(writer, "}}").map_err(|e| e.to_string())?;

    if !contract_specs.types().is_empty() {
        writeln!(writer).map_err(|e| e.to_string())?;
        for ty in contract_specs.types() {
            let formatted = format_spec_tokens(&ty.to_string());
            writeln!(writer, "{}", formatted).map_err(|e| e.to_string())?;
        }
    } else if is_account_contract {
        emit_account_fallback_decls(writer)?;
    }

    Ok(())
}

fn emit_account_fallback_decls<W: Write>(writer: &mut W) -> Result<(), String> {
    writeln!(writer).map_err(|e| e.to_string())?;
    writeln!(writer, "{}", ACCOUNT_FALLBACK_DECLS.trim()).map_err(|e| e.to_string())?;
    Ok(())
}
