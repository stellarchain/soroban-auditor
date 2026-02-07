use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if !spec_fn.inputs().is_empty() {
        return false;
    }
    let output_ty = spec_fn
        .output()
        .map(|o| format_type_ident(&o.type_ident().to_string()))
        .unwrap_or_else(|| "u32".to_string());
    let has_counter = ctx
        .symbol_literals
        .iter()
        .chain(ctx.string_literals.iter())
        .any(|s| s.as_str() == "COUNTER");
    let name_matches = ctx.export_name.contains("increment") || ctx.export_name.contains("inc");
    if !name_matches && !ctx.uses_contract_event {
        return false;
    }
    let counter_symbol = if has_counter { "COUNTER" } else { "COUNTER" };

    write!(
        writer,
        "    pub fn {}(&mut self, env: Env) -> {}",
        ctx.export_name, output_ty
    )
    .unwrap();
    writeln!(writer, " {{").unwrap();
    writeln!(
        writer,
        "        let mut count: {} = env.storage().instance().get(&symbol_short!(\"{}\")).unwrap_or(0);",
        output_ty, counter_symbol
    )
    .unwrap();
    writeln!(writer, "        count += 1;").unwrap();
    writeln!(
        writer,
        "        env.storage().instance().set(&symbol_short!(\"{}\"), &count);",
        counter_symbol
    )
    .unwrap();
    writeln!(
        writer,
        "        env.events().publish((symbol_short!(\"COUNTER\"), symbol_short!(\"increment\")), count);"
    )
    .unwrap();
    writeln!(writer, "        count").unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}
