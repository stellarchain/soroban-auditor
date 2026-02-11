/// Enhanced forwarder detection - automatically identifies helper functions
/// that should be inlined without any hardcoding.
///
/// Strategy: Analyze function structure to detect various forwarding patterns
use crate::forwarder::collect_forwarder_args;
use parity_wasm::elements::{FuncBody, Instruction};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ForwarderType {
    /// Simple pass-through: GetLocal* + Call
    DirectForwarder,
    /// Minor manipulation then call: GetLocal + arithmetic + Call
    TransformingForwarder,
    /// Unpacks struct/tuple then calls: GetLocal + field access + Call
    UnpackingForwarder,
}

#[derive(Debug, Clone)]
pub struct ForwarderInfo {
    pub forwarder_type: ForwarderType,
    pub target_function: u32,
    pub complexity_score: usize,
    pub args: Vec<crate::forwarder::ForwardArg>,
}

pub struct ForwarderAnalyzer {
    import_count: usize,
}

impl ForwarderAnalyzer {
    pub fn new(import_count: usize) -> Self {
        Self { import_count }
    }

    /// Main entry point: analyze if a function is a forwarder
    pub fn analyze_function(
        &self,
        body: &FuncBody,
        functions: &[crate::wasm_ir::Function],
    ) -> Option<ForwarderInfo> {
        let instrs: Vec<&Instruction> = body.code().elements().iter().collect();

        // Remove trailing End/Return
        let instrs = Self::trim_instructions(&instrs);

        if instrs.is_empty() {
            return None;
        }

        // Check for SDK call OR internal call (multi-level forwarders!)
        let target_call = Self::find_any_call(&instrs, self.import_count)?;

        // Analyze complexity
        let complexity = Self::compute_complexity(&instrs);

        // Determine forwarder type
        let forwarder_type = Self::classify_forwarder(&instrs, target_call, self.import_count);

        // TEMPORARY: Accept ALL complexity levels to see what we're missing
        // TODO: Re-enable threshold after debugging
        // if complexity > 100 {
        //     return None;
        // }

        let target_params = functions[target_call as usize].ty.params().len();
        let call_instrs = &instrs[..instrs.len() - 1];
        let args = collect_forwarder_args(call_instrs, target_params)?;

        Some(ForwarderInfo {
            forwarder_type,
            target_function: target_call,
            complexity_score: complexity,
            args,
        })
    }

    /// Remove trailing End/Return instructions
    fn trim_instructions<'a>(instrs: &[&'a Instruction]) -> Vec<&'a Instruction> {
        let mut result = instrs.to_vec();
        while let Some(last) = result.last() {
            match last {
                Instruction::End | Instruction::Return => {
                    result.pop();
                }
                _ => break,
            }
        }
        result
    }

    /// Find if there's a call to an SDK function (import)
    fn find_sdk_call(instrs: &[&Instruction], import_count: usize) -> Option<u32> {
        for instr in instrs.iter().rev() {
            if let Instruction::Call(idx) = instr {
                if (*idx as usize) < import_count {
                    return Some(*idx);
                }
            }
        }
        None
    }

    /// Find ANY call (SDK import OR internal function) for multi-level forwarders
    fn find_any_call(instrs: &[&Instruction], import_count: usize) -> Option<u32> {
        // First try to find SDK call (preferred)
        if let Some(sdk_call) = Self::find_sdk_call(instrs, import_count) {
            return Some(sdk_call);
        }

        // If no SDK call, look for ANY function call (internal forwarder)
        for instr in instrs.iter().rev() {
            if let Instruction::Call(idx) = instr {
                return Some(*idx);
            }
        }

        None
    }

    /// Compute complexity score (0-100)
    fn compute_complexity(instrs: &[&Instruction]) -> usize {
        let mut score = 0;

        for instr in instrs {
            match instr {
                // Simple operations
                Instruction::GetLocal(_) | Instruction::SetLocal(_) => score += 1,
                Instruction::I32Const(_) | Instruction::I64Const(_) => score += 1,
                Instruction::Call(_) => score += 2,

                // Arithmetic
                Instruction::I32Add
                | Instruction::I64Add
                | Instruction::I32Sub
                | Instruction::I64Sub
                | Instruction::I32Mul
                | Instruction::I64Mul => score += 2,

                // Bit operations
                Instruction::I32And
                | Instruction::I64And
                | Instruction::I32Or
                | Instruction::I64Or
                | Instruction::I32Shl
                | Instruction::I64Shl
                | Instruction::I32ShrU
                | Instruction::I64ShrU => score += 2,

                // Type conversions
                Instruction::I32WrapI64
                | Instruction::I64ExtendSI32
                | Instruction::I64ExtendUI32 => score += 2,

                // Memory operations (more complex)
                Instruction::I32Load(_, _)
                | Instruction::I64Load(_, _)
                | Instruction::I32Store(_, _)
                | Instruction::I64Store(_, _) => score += 5,

                // Control flow (very complex)
                Instruction::Block(_)
                | Instruction::Loop(_)
                | Instruction::If(_)
                | Instruction::Br(_)
                | Instruction::BrIf(_)
                | Instruction::BrTable(_) => score += 20,

                _ => score += 3,
            }
        }

        score
    }

    /// Classify the type of forwarder
    fn classify_forwarder(
        instrs: &[&Instruction],
        _target_call: u32,
        _import_count: usize,
    ) -> ForwarderType {
        let has_memory_ops = instrs.iter().any(|i| {
            matches!(
                i,
                Instruction::I32Load(_, _)
                    | Instruction::I64Load(_, _)
                    | Instruction::I32Store(_, _)
                    | Instruction::I64Store(_, _)
            )
        });

        let has_arithmetic = instrs.iter().any(|i| {
            matches!(
                i,
                Instruction::I32Add
                    | Instruction::I64Add
                    | Instruction::I32Sub
                    | Instruction::I64Sub
                    | Instruction::I32Shl
                    | Instruction::I64Shl
            )
        });

        let has_only_locals_and_consts = instrs.iter().all(|i| {
            matches!(
                i,
                Instruction::GetLocal(_)
                    | Instruction::SetLocal(_)
                    | Instruction::I32Const(_)
                    | Instruction::I64Const(_)
                    | Instruction::Call(_)
            )
        });

        if has_memory_ops {
            ForwarderType::UnpackingForwarder
        } else if has_arithmetic {
            ForwarderType::TransformingForwarder
        } else if has_only_locals_and_consts {
            ForwarderType::DirectForwarder
        } else {
            ForwarderType::TransformingForwarder
        }
    }

    /// Batch analyze all functions and return forwarder map
    pub fn analyze_all_functions(
        &self,
        code: &parity_wasm::elements::CodeSection,
        import_count: usize,
        functions: &[crate::wasm_ir::Function],
    ) -> HashMap<u32, ForwarderInfo> {
        let mut forwarders = HashMap::new();

        for (i, body) in code.bodies().iter().enumerate() {
            let fn_index = (import_count + i) as u32;

            if let Some(info) = self.analyze_function(body, functions) {
                forwarders.insert(fn_index, info);
            }
        }

        forwarders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_scoring() {
        // Simple: GetLocal + Call
        let simple = vec![
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::Call(5),
        ];
        let complexity = ForwarderAnalyzer::compute_complexity(&simple.iter().collect::<Vec<_>>());
        assert!(complexity < 10);

        // Complex: With memory ops
        let complex = vec![
            Instruction::GetLocal(0),
            Instruction::I32Load(2, 0),
            Instruction::GetLocal(1),
            Instruction::Call(5),
        ];
        let complexity = ForwarderAnalyzer::compute_complexity(&complex.iter().collect::<Vec<_>>());
        assert!(complexity > 5);
    }

    #[test]
    fn test_forwarder_classification() {
        // Direct forwarder
        let direct = vec![Instruction::GetLocal(0), Instruction::Call(5)];
        let ftype =
            ForwarderAnalyzer::classify_forwarder(&direct.iter().collect::<Vec<_>>(), 5, 10);
        assert_eq!(ftype, ForwarderType::DirectForwarder);

        // Transforming forwarder
        let transform = vec![
            Instruction::GetLocal(0),
            Instruction::I32Const(1),
            Instruction::I32Add,
            Instruction::Call(5),
        ];
        let ftype =
            ForwarderAnalyzer::classify_forwarder(&transform.iter().collect::<Vec<_>>(), 5, 10);
        assert_eq!(ftype, ForwarderType::TransformingForwarder);
    }
}
