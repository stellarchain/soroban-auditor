use crate::format::format_type_ident;
use crate::patterns::PatternContext;
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
) -> bool {
    if ctx.require_auth_calls != 1 || ctx.addr_indices.is_empty() {
        return false;
    }
    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        write!(writer, ", {}: {}", argument.name(), format_type_ident(&argument.type_ident().to_string())).unwrap();
    }
    write!(writer, ")").unwrap();
    writeln!(writer, " {{").unwrap();
    let addr = spec_fn.inputs()[ctx.addr_indices[0]].name();
    let tuple_args: Vec<&str> = spec_fn
        .inputs()
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if i == ctx.addr_indices[0] { None } else { Some(p.name()) })
        .collect();
    if !tuple_args.is_empty() {
        writeln!(
            writer,
            "        {}.require_auth_for_args(({}).into_val(&env));",
            addr,
            tuple_args.join(", ")
        )
        .unwrap();
    }
    writeln!(writer, "    }}").unwrap();
    true
}
