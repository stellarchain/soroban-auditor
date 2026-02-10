use parity_wasm::elements::Instruction;

/// Argument description for a forwarder-to-call chain.
#[derive(Debug, Clone)]
pub enum ForwardArg {
    Local(usize),
    PackedU32(usize),
    I32(i32),
    I64(i64),
}

/// Recorded info about a call forwarder.
#[derive(Debug, Clone)]
pub struct CallForwarder {
    pub target: u32,
    pub args: Vec<ForwardArg>,
}

/// Attempt to parse a forwarder's argument list from its pre-call instructions.
pub fn collect_forwarder_args(
    insts: &[&Instruction],
    param_count: usize,
) -> Option<Vec<ForwardArg>> {
    let mut args = Vec::new();
    let mut i = 0usize;
    while i < insts.len() {
        if let Some(Instruction::GetLocal(n)) = insts.get(i) {
            if matches!(
                insts.get(i + 1),
                Some(Instruction::I64ExtendSI32) | Some(Instruction::I64ExtendUI32)
            ) && matches!(insts.get(i + 2), Some(Instruction::I64Const(32)))
                && matches!(insts.get(i + 3), Some(Instruction::I64Shl))
                && matches!(insts.get(i + 4), Some(Instruction::I64Const(4)))
                && matches!(insts.get(i + 5), Some(Instruction::I64Or))
            {
                args.push(ForwardArg::PackedU32(*n as usize));
                i += 6;
                continue;
            }
        }
        match insts.get(i) {
            Some(Instruction::GetLocal(n)) => args.push(ForwardArg::Local(*n as usize)),
            Some(Instruction::I32Const(v)) => args.push(ForwardArg::I32(*v)),
            Some(Instruction::I64Const(v)) => args.push(ForwardArg::I64(*v)),
            _ => return None,
        }
        i += 1;
    }
    if args.len() < param_count {
        return None;
    }
    if args.len() > param_count {
        args = args[args.len() - param_count..].to_vec();
    }
    Some(args)
}

/// Evaluate forwarder arg spec into concrete expressions using the caller's args.
pub fn map_forwarder_args(specs: &[ForwardArg], current_args: &[String]) -> Option<Vec<String>> {
    let mut out = Vec::with_capacity(specs.len());
    for spec in specs {
        match spec {
            ForwardArg::Local(idx) | ForwardArg::PackedU32(idx) => {
                out.push(current_args.get(*idx)?.clone());
            }
            ForwardArg::I32(v) => out.push(v.to_string()),
            ForwardArg::I64(v) => out.push(v.to_string()),
        }
    }
    Some(out)
}
