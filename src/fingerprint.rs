use crate::wasm_ir::Function;
use parity_wasm::elements::{FuncBody, FunctionType, Instruction, ValueType};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Fingerprint {
    pub hash: u64,
    pub instrs: u32,
    pub locals: u32,
    pub calls_import: u16,
    pub calls_internal: u16,
    pub memops: u16,
    pub blocks: u16,
}

impl Fingerprint {
    pub fn short(&self) -> String {
        format!(
            "h={:016x} i={} l={} ci={} cn={} m={} b={}",
            self.hash, self.instrs, self.locals, self.calls_import, self.calls_internal, self.memops, self.blocks
        )
    }
}

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn hash_bytes(state: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *state ^= b as u64;
        *state = state.wrapping_mul(FNV_PRIME);
    }
}

fn hash_u32(state: &mut u64, v: u32) {
    hash_bytes(state, &v.to_le_bytes());
}

fn hash_u64(state: &mut u64, v: u64) {
    hash_bytes(state, &v.to_le_bytes());
}

fn hash_valtype(state: &mut u64, ty: ValueType) {
    let b = match ty {
        ValueType::I32 => 0x01,
        ValueType::I64 => 0x02,
        ValueType::F32 => 0x03,
        ValueType::F64 => 0x04,
    };
    hash_bytes(state, &[b]);
}

pub fn fingerprint_function(
    body: &FuncBody,
    fn_type: &FunctionType,
    import_count: usize,
    functions: &[Function],
) -> Fingerprint {
    let mut hash = FNV_OFFSET;
    let mut instrs: u32 = 0;
    let mut locals: u32 = 0;
    let mut calls_import: u16 = 0;
    let mut calls_internal: u16 = 0;
    let mut memops: u16 = 0;
    let mut blocks: u16 = 0;

    for param in fn_type.params() {
        hash_valtype(&mut hash, *param);
    }
    for result in fn_type.results() {
        hash_valtype(&mut hash, *result);
    }

    for local in body.locals() {
        locals = locals.saturating_add(local.count() as u32);
        hash_valtype(&mut hash, local.value_type());
        hash_u32(&mut hash, local.count());
    }

    for instr in body.code().elements() {
        instrs = instrs.saturating_add(1);
        match instr {
            Instruction::Call(n) => {
                if (*n as usize) < import_count {
                    calls_import = calls_import.saturating_add(1);
                    let name = functions
                        .get(*n as usize)
                        .and_then(|f| f.real_name)
                        .map_or("import", |v| v);
                    hash_bytes(&mut hash, b"call_import:");
                    hash_bytes(&mut hash, name.as_bytes());
                } else {
                    calls_internal = calls_internal.saturating_add(1);
                    hash_bytes(&mut hash, b"call_internal");
                }
            }
            Instruction::CallIndirect(type_idx, table_idx) => {
                calls_internal = calls_internal.saturating_add(1);
                hash_bytes(&mut hash, b"call_indirect");
                hash_u32(&mut hash, *type_idx);
                hash_u32(&mut hash, (*table_idx).into());
            }
            Instruction::I32Load(_, _)
            | Instruction::I64Load(_, _)
            | Instruction::F32Load(_, _)
            | Instruction::F64Load(_, _)
            | Instruction::I32Load8S(_, _)
            | Instruction::I32Load8U(_, _)
            | Instruction::I32Load16S(_, _)
            | Instruction::I32Load16U(_, _)
            | Instruction::I64Load8S(_, _)
            | Instruction::I64Load8U(_, _)
            | Instruction::I64Load16S(_, _)
            | Instruction::I64Load16U(_, _)
            | Instruction::I64Load32S(_, _)
            | Instruction::I64Load32U(_, _)
            | Instruction::I32Store(_, _)
            | Instruction::I64Store(_, _)
            | Instruction::F32Store(_, _)
            | Instruction::F64Store(_, _)
            | Instruction::I32Store8(_, _)
            | Instruction::I32Store16(_, _)
            | Instruction::I64Store8(_, _)
            | Instruction::I64Store16(_, _)
            | Instruction::I64Store32(_, _) => {
                memops = memops.saturating_add(1);
                hash_bytes(&mut hash, format!("{:?}", instr).as_bytes());
            }
            Instruction::Block(_) | Instruction::Loop(_) | Instruction::If(_) => {
                blocks = blocks.saturating_add(1);
                hash_bytes(&mut hash, format!("{:?}", instr).as_bytes());
            }
            Instruction::Br(_)
            | Instruction::BrIf(_)
            | Instruction::BrTable(_)
            | Instruction::Return
            | Instruction::Unreachable
            | Instruction::Nop => {
                hash_bytes(&mut hash, format!("{:?}", instr).as_bytes());
            }
            Instruction::I32Const(v) => {
                hash_bytes(&mut hash, b"i32.const");
                hash_u32(&mut hash, *v as u32);
            }
            Instruction::I64Const(v) => {
                hash_bytes(&mut hash, b"i64.const");
                hash_u64(&mut hash, *v as u64);
            }
            Instruction::F32Const(v) => {
                hash_bytes(&mut hash, b"f32.const");
                hash_u32(&mut hash, *v);
            }
            Instruction::F64Const(v) => {
                hash_bytes(&mut hash, b"f64.const");
                hash_u64(&mut hash, *v);
            }
            _ => {
                hash_bytes(&mut hash, format!("{:?}", instr).as_bytes());
            }
        }
        hash_bytes(&mut hash, &[0xff]);
    }

    Fingerprint {
        hash,
        instrs,
        locals,
        calls_import,
        calls_internal,
        memops,
        blocks,
    }
}
