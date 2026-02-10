use crate::engine::function::{join_functions, split_functions};
use crate::engine::pattern::Pattern;
use crate::engine::patterns::{
    ConsolidateCommentsPattern, ContinueBreakCleanup, ConversionEliminationPattern,
    CopyPayloadPattern, CountedLoopPattern, DeduplicateVariablesPattern, ExitFlagDefaultAssign,
    ExitFlagLoopCollapse, ForEachValPattern, GuardBlockBreaks, GuardBreakUnreachablePattern, GuardEarlyReturn,
    FunctionSignaturePattern, InitialAssignmentPattern, IrLabelCleanup, LabelBlockCollapse,
    LabelBlockTailGuard, LabelBlockToLoop, LabelGuardIf, LabelIfChain, LabelLadderInline,
    LabelLadderReduce, LinearMemoryVecBuildPattern,
    LoopBreakTailReturn, LoopGuardToIf, LoopIfBreakElse, LoopIfUnreachableToBlock,
    LoopNoControlToBlock, LoopToWhile, LoopUnreachableElse, MathOperationsPattern,
    MissingSemicolonsPattern, NextStringWhile, ReadPriceDataPattern,
    SimpleLoopUnlabel,
    SinglePassLoopCleanup, SinglePassUnlabeledLoopCleanup, SmartVariableNamingPattern,
    StackCopyVecReturnPattern, StackFramePattern, StorageAccessPattern, UndefinedHelpersPattern,
    UnwrapIfElseBlock,
};

pub struct Engine {
    patterns: Vec<Box<dyn Pattern>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn register<P: Pattern + 'static>(&mut self, pattern: P) {
        self.patterns.push(Box::new(pattern));
    }

    pub fn apply(&self, input: String) -> String {
        let mut blocks = split_functions(&input);

        // Apply patterns iteratively until fixpoint
        const MAX_ITERATIONS: usize = 20;

        for block in &mut blocks {
            if block.body.is_empty() {
                continue;
            }

            let mut iteration = 0;
            loop {
                let mut changed = false;
                iteration += 1;

                if iteration > MAX_ITERATIONS {
                    eprintln!(
                        "Warning: Engine reached max iterations ({}) for function: {}",
                        MAX_ITERATIONS, block.name
                    );
                    break;
                }

                for pattern in &self.patterns {
                    if let Some(new_block) = pattern.apply(block) {
                        *block = new_block;
                        changed = true;
                    }
                }

                if !changed {
                    // Reached fixpoint
                    break;
                }
            }
        }

        join_functions(blocks)
    }
}

pub fn default_engine() -> Engine {
    let mut engine = Engine::new();
    engine.register(IrLabelCleanup::new());
    engine.register(SinglePassLoopCleanup::new());
    engine.register(NextStringWhile::new());
    engine.register(LabelBlockCollapse::new());
    engine.register(LabelGuardIf::new());
    engine.register(LabelBlockTailGuard::new());
    engine.register(LabelIfChain::new());
    engine.register(LabelBlockToLoop::new());
    engine.register(ExitFlagLoopCollapse::new());
    engine.register(LoopIfUnreachableToBlock::new());
    engine.register(LoopGuardToIf::new());
    engine.register(GuardBlockBreaks::new());
    engine.register(ExitFlagDefaultAssign::new());
    engine.register(LoopIfBreakElse::new());
    engine.register(LoopBreakTailReturn::new());
    engine.register(UnwrapIfElseBlock::new());
    engine.register(LoopNoControlToBlock::new());
    engine.register(GuardEarlyReturn::new());
    engine.register(LoopUnreachableElse::new());
    engine.register(SinglePassUnlabeledLoopCleanup::new());
    engine.register(LoopToWhile::new());
    engine.register(SimpleLoopUnlabel::new());
    engine.register(CopyPayloadPattern::new());
    engine.register(LabelLadderInline::new());
    engine.register(LabelLadderReduce::new());
    engine.register(ForEachValPattern::new());
    engine.register(CountedLoopPattern::new());
    engine.register(ReadPriceDataPattern::new());
    engine.register(ContinueBreakCleanup::new());
    engine.register(IrLabelCleanup::new());

    // New Soroban-specific patterns
    engine.register(UndefinedHelpersPattern::new()); // Remove undefined helper calls
    engine.register(StackFramePattern::new()); // Must run EARLY to clean up stack artifacts
    engine.register(DeduplicateVariablesPattern::new()); // Remove duplicate variable declarations
    engine.register(ConversionEliminationPattern::new()); // Detect and annotate val_from_i64/val_to_i64 conversions
    engine.register(InitialAssignmentPattern::new()); // Combine zero-inits with first assignment
    engine.register(MissingSemicolonsPattern::new()); // Fix missing semicolons
    engine.register(StorageAccessPattern::new());
    engine.register(MathOperationsPattern::new());
    engine.register(LinearMemoryVecBuildPattern::new());
    // DISABLED: Old VariableNamingPattern causes panics on some contracts
    // engine.register(VariableNamingPattern::new());

    // Smart variable naming (now with single-pass protection)
    engine.register(SmartVariableNamingPattern::new());
    engine.register(StackCopyVecReturnPattern::new());
    engine.register(GuardBreakUnreachablePattern::new());
    engine.register(FunctionSignaturePattern::new());

    // Comment cleanup - MUST RUN LAST to consolidate all diagnostic comments
    engine.register(ConsolidateCommentsPattern::new());

    engine
}
