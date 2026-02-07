use crate::format::format_type_ident;
use crate::patterns::{PatternContext, PatternState};
use crate::soroban::contract::FunctionContractSpec;
use std::io::Write;

pub fn try_emit<W: Write>(
    writer: &mut W,
    spec_fn: &FunctionContractSpec,
    ctx: &PatternContext<'_>,
    state: &mut PatternState,
) -> bool {
    let has_swap_shape =
        spec_fn.inputs().len() == 8 && ctx.addr_indices.len() == 4 && ctx.i128_indices.len() == 4;
    if !has_swap_shape {
        return false;
    }

    let a = spec_fn.inputs()[ctx.addr_indices[0]].name();
    let b = spec_fn.inputs()[ctx.addr_indices[1]].name();
    let token_a = spec_fn.inputs()[ctx.addr_indices[2]].name();
    let token_b = spec_fn.inputs()[ctx.addr_indices[3]].name();
    let amount_a = spec_fn.inputs()[ctx.i128_indices[0]].name();
    let min_b_for_a = spec_fn.inputs()[ctx.i128_indices[1]].name();
    let amount_b = spec_fn.inputs()[ctx.i128_indices[2]].name();
    let min_a_for_b = spec_fn.inputs()[ctx.i128_indices[3]].name();

    write!(writer, "    pub fn {}(&mut self, env: Env", ctx.export_name).unwrap();
    for argument in spec_fn.inputs() {
        let ty = format_type_ident(&format_type_ident(&argument.type_ident().to_string()).to_string());
        write!(writer, ", {}: {}", argument.name(), ty).unwrap();
    }
    write!(writer, ")").unwrap();
    writeln!(writer, " {{").unwrap();

    let has_panic_a = ctx
        .string_literals
        .iter()
        .any(|s| s.as_str() == "not enough token B for token A");
    let has_panic_b = ctx
        .string_literals
        .iter()
        .any(|s| s.as_str() == "not enough token A for token B");
    if has_panic_a {
        writeln!(
            writer,
            "        if {} < {} {{ panic!(\"not enough token B for token A\"); }}",
            amount_b, min_b_for_a
        )
        .unwrap();
    }
    if has_panic_b {
        writeln!(
            writer,
            "        if {} < {} {{ panic!(\"not enough token A for token B\"); }}",
            amount_a, min_a_for_b
        )
        .unwrap();
    }
    writeln!(
        writer,
        "        {}.require_auth_for_args(({}.clone(), {}.clone(), {}, {}).into_val(&env));",
        a, token_a, token_b, amount_a, min_b_for_a
    )
    .unwrap();
    writeln!(
        writer,
        "        {}.require_auth_for_args(({}.clone(), {}.clone(), {}, {}).into_val(&env));",
        b, token_b, token_a, amount_b, min_a_for_b
    )
    .unwrap();
    writeln!(
        writer,
        "        self.move_token(&env, &{}, &{}, &{}, {}, {});",
        token_a, a, b, amount_a, min_a_for_b
    )
    .unwrap();
    writeln!(
        writer,
        "        self.move_token(&env, &{}, &{}, &{}, {}, {});",
        token_b, b, a, amount_b, min_b_for_a
    )
    .unwrap();
    writeln!(writer, "    }}").unwrap();

    if !state.emitted_move_token {
        writeln!(writer, "    fn move_token(&self, env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: i128, transfer_amount: i128) {{").unwrap();
        writeln!(writer, "        let token = token::Client::new(env, token);").unwrap();
        writeln!(writer, "        let contract_address = env.current_contract_address();").unwrap();
        writeln!(writer, "        token.transfer(from, &contract_address, &max_spend_amount);").unwrap();
        writeln!(writer, "        token.transfer(&contract_address, to, &transfer_amount);").unwrap();
        writeln!(writer, "        token.transfer(&contract_address, from, &(max_spend_amount - transfer_amount));").unwrap();
        writeln!(writer, "    }}").unwrap();
        state.emitted_move_token = true;
    }
    true
}
