use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if spec_fn.inputs().len() != 1 || !ctx.has_vec_new {
        return false;
    }
    let literal = ctx
        .symbol_literals
        .iter()
        .find(|s| s.as_str() == "Hello")
        .or_else(|| ctx.symbol_literals.first())
        .or_else(|| ctx.string_literals.iter().find(|s| s.as_str() == "Hello"))
        .or_else(|| ctx.string_literals.first());
    let Some(literal) = literal else { return false };

    let output_ty = spec_fn
        .output()
        .map(|o| format_type_ident(&o.type_ident().to_string()))
        .unwrap_or_default();
    let elem_is_symbol = output_ty.contains("Vec") && output_ty.contains("Symbol");
    let elem_is_string = output_ty.contains("Vec") && output_ty.contains("String");
    let prefer_string = elem_is_string || ctx.uses_string_new;
    let prefer_symbol = elem_is_symbol || ctx.uses_symbol_new;
    if !prefer_string && !prefer_symbol {
        return false;
    }

    write!(
        writer,
        "    pub fn {}(&mut self, env: Env, {}: {})",
        ctx.export_name,
        spec_fn.inputs()[0].name(),
        format_type_ident(&spec_fn.inputs()[0].type_ident().to_string())
    )
    .unwrap();
    write!(writer, " -> {}", output_ty).unwrap();
    writeln!(writer, " {{").unwrap();
    let const_expr = if prefer_string {
        format!("String::from_str(&env, \"{}\")", literal)
    } else {
        format!("Symbol::new(&env, \"{}\")", literal)
    };
    writeln!(
        writer,
        "        vec![&env, {}, {}]",
        const_expr,
        spec_fn.inputs()[0].name()
    )
    .unwrap();
    writeln!(writer, "    }}").unwrap();
    true
}
