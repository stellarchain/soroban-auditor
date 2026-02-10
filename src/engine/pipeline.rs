use crate::engine::function::{join_functions, split_functions};
use crate::engine::pattern::Pattern;
use crate::engine::preclean;
use crate::engine::patterns::{
    BreakToLabelPattern, ConsolidateCommentsPattern, ConstantMatchCleanupPattern,
    ContinueBreakCleanup, ConversionEliminationPattern, CopyPayloadPattern, CountedLoopPattern,
    DeadTempCleanupPattern, DeduplicateVariablesPattern, EmptyIfBlockPattern, ForEachValPattern,
    ElseCompactionPattern,
    FunctionSignaturePattern, GuardBlockBreaks, GuardBreakUnreachablePattern,
    IfChainToGuardsPattern, IfConditionCleanupPattern, InlineFrameBasePattern,
    InlineValRoundtripPattern, IrLabelCleanup, LabelBlockCollapse, LabelGuardIf, LabelMatchBreakGuard, LabelTrapTailInlinePattern,
    LinearMemoryVecBuildPattern, RedundantTypeCheckPattern, RemoveTerminalReturnPattern,
    RemoveUnusedLocalsPattern,
    LoopBreakTailReturn, LoopGuardChainToIf, LoopGuardToIf, LoopIfUnreachableToBlock, LoopNoControlToBlock, LoopToWhile,
    MathOperationsPattern, MissingSemicolonsPattern, NextStringWhile,
    RedundantScopePattern,
    PruneEmptyIfBlocksPattern, ReturnVoidCleanupPattern, TerminalScopeUnwrapPattern,
    SimpleLoopUnlabel, SinglePassLoopCleanup, SinglePassUnlabeledLoopCleanup, SmartVariableNamingPattern,
    StackCopyVecReturnPattern, StatusResultGuardLoopPattern, StatusResultGuardTextPattern, StorageAccessPattern, SymbolLiteralRecoveryPattern, TrailingUnreachableCleanupPattern,
    StatusResultGuardLabelPattern,
    TypeTagGuardCleanupPattern, VmScaffoldCleanupPattern,
    UndefinedHelpersPattern, UnreachableCleanupPattern,
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
        let precleaned = self.preclean_input(input);
        let mut blocks = split_functions(&precleaned);

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

    fn preclean_input(&self, input: String) -> String {
        preclean::run(input)
    }
}

pub fn default_engine() -> Engine {
    let mut engine = Engine::new();
    register_cfg_phase(&mut engine);
    register_soroban_phase(&mut engine);
    register_cleanup_phase(&mut engine);
    engine
}

fn register_cfg_phase(engine: &mut Engine) {
    engine.register(IrLabelCleanup::new());
    engine.register(SinglePassLoopCleanup::new());
    engine.register(NextStringWhile::new());
    engine.register(LabelBlockCollapse::new());
    engine.register(LabelGuardIf::new());
    engine.register(LabelMatchBreakGuard::new());
    engine.register(LabelTrapTailInlinePattern::new());
    // Disabled: can produce malformed guard if/else around labels in some contracts.
    // engine.register(LabelBlockTailGuard::new());
    // Disabled: may rewrite labeled guards into malformed detached else blocks.
    // engine.register(LabelIfChain::new());
    // Disabled: can erase valid labeled-branch semantics (e.g., token transfer var4==6 path).
    // Re-enable only after data-flow safe conversion constraints are implemented.
    // engine.register(LabelBlockToLoop::new());
    // Disabled: often introduces malformed if/else around guard blocks.
    // engine.register(ExitFlagLoopCollapse::new());
    engine.register(LoopIfUnreachableToBlock::new());
    engine.register(LoopGuardChainToIf::new());
    engine.register(LoopGuardToIf::new());
    engine.register(GuardBlockBreaks::new());
    engine.register(BreakToLabelPattern::new());
    // Disabled together with ExitFlagLoopCollapse to keep control-flow valid.
    // engine.register(ExitFlagDefaultAssign::new());
    // Disabled: can generate invalid if/else + break structure in complex contracts.
    // engine.register(LoopIfBreakElse::new());
    // Disabled: can erase guard semantics by flattening terminal break paths too early.
    // engine.register(LoopBreakTailReturn::new());
    // Disabled: occasionally unwraps into syntactically invalid else placement.
    // engine.register(UnwrapIfElseBlock::new());
    engine.register(LoopNoControlToBlock::new());
    // Disabled: can interfere with label-based CFG and produce broken branches.
    // engine.register(GuardEarlyReturn::new());
    // engine.register(LoopUnreachableElse::new());
    engine.register(SinglePassUnlabeledLoopCleanup::new());
    engine.register(LoopToWhile::new());
    engine.register(SimpleLoopUnlabel::new());
    engine.register(CopyPayloadPattern::new());
    // Disabled: can over-compress label chains and lose guard branches in complex CFGs.
    // engine.register(LabelLadderInline::new());
    // engine.register(LabelLadderReduce::new());
    engine.register(ForEachValPattern::new());
    engine.register(CountedLoopPattern::new());
    engine.register(ContinueBreakCleanup::new());
}

fn register_soroban_phase(engine: &mut Engine) {
    // New Soroban-specific patterns
    engine.register(UndefinedHelpersPattern::new()); // Remove undefined helper calls
    // Disabled: current implementation is too aggressive and drops required stack-pointer locals.
    // engine.register(StackFramePattern::new());
    engine.register(DeduplicateVariablesPattern::new()); // Remove duplicate variable declarations
    engine.register(ConversionEliminationPattern::new()); // Detect and annotate val_from_i64/val_to_i64 conversions
    // Disabled for now: can produce invalid declaration order (e.g. init from not-yet-defined slot vars).
    // Re-enable after adding data-flow safety checks.
    // engine.register(InitialAssignmentPattern::new());
    engine.register(MissingSemicolonsPattern::new()); // Fix missing semicolons
    engine.register(StorageAccessPattern::new());
    engine.register(MathOperationsPattern::new());
    engine.register(LinearMemoryVecBuildPattern::new());
    engine.register(ConstantMatchCleanupPattern::new());
    // DISABLED: Old VariableNamingPattern causes panics on some contracts
    // engine.register(VariableNamingPattern::new());
}

fn register_cleanup_phase(engine: &mut Engine) {
    // Smart variable naming (now with single-pass protection)
    engine.register(SmartVariableNamingPattern::new());
    engine.register(InlineFrameBasePattern::new());
    engine.register(StackCopyVecReturnPattern::new());
    engine.register(SymbolLiteralRecoveryPattern::new());
    engine.register(TypeTagGuardCleanupPattern::new());
    engine.register(VmScaffoldCleanupPattern::new());
    engine.register(GuardBreakUnreachablePattern::new());
    engine.register(RedundantTypeCheckPattern::new());
    engine.register(IfConditionCleanupPattern::new());
    engine.register(IfChainToGuardsPattern::new());
    engine.register(IfConditionCleanupPattern::new());
    // Disabled: too aggressive; can degrade source-like output (e.g. hello_world guards).
    // engine.register(GuardUnreachableAssertPattern::new());
    // engine.register(AssertConditionCleanupPattern::new());
    engine.register(InlineValRoundtripPattern::new());
    engine.register(RedundantScopePattern::new());
    engine.register(RemoveUnusedLocalsPattern::new());
    engine.register(DeadTempCleanupPattern::new());
    engine.register(RemoveTerminalReturnPattern::new());
    engine.register(ReturnVoidCleanupPattern::new());
    engine.register(UnreachableCleanupPattern::new());
    engine.register(TerminalScopeUnwrapPattern::new());
    engine.register(ElseCompactionPattern::new());
    engine.register(EmptyIfBlockPattern::new());
    engine.register(PruneEmptyIfBlocksPattern::new());
    engine.register(StatusResultGuardLoopPattern::new());
    engine.register(StatusResultGuardLabelPattern::new());
    engine.register(StatusResultGuardTextPattern::new());
    engine.register(FunctionSignaturePattern::new());
    engine.register(TrailingUnreachableCleanupPattern::new());

    // Comment cleanup - MUST RUN LAST to consolidate all diagnostic comments
    engine.register(ConsolidateCommentsPattern::new());
}
