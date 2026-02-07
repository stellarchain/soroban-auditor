use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use crate::decompile::{format_byte_array, parse_bytesn_len};
use std::io::Write;

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if !spec_fn.inputs().is_empty() || !ctx.uses_bytes_new {
        return false;
    }
    let output_ty = spec_fn
        .output()
        .map(|o| format_type_ident(&o.type_ident().to_string()))
        .unwrap_or_default();
    if !output_ty.contains("Bytes") {
        return false;
    }
    if ctx.data_segments.len() != 1 {
        return false;
    }
    let bytes = &ctx.data_segments[0];
    let bytesn_len = parse_bytesn_len(&output_ty);
    if !bytesn_len.map(|n| n == bytes.len()).unwrap_or(true) {
        return false;
    }

    write!(writer, "    pub fn {}(&mut self, env: Env)", ctx.export_name).unwrap();
    if !output_ty.is_empty() {
        write!(writer, " -> {}", output_ty).unwrap();
    }
    writeln!(writer, " {{").unwrap();
    let array_literal = format_byte_array(bytes);
    if let Some(n) = bytesn_len {
        writeln!(
            writer,
            "        BytesN::<{}>::from_array(&env, &{})",
            n, array_literal
        )
        .unwrap();
    } else {
        writeln!(writer, "        Bytes::from_array(&env, &{})", array_literal).unwrap();
    }
    writeln!(writer, "    }}").unwrap();
    true
}
