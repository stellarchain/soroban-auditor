use crate::engine::function::{join_functions, split_functions};
use crate::engine::pattern::Pattern;
use crate::engine::preclean;
use crate::engine::patterns::{
    BreakToLabelPattern, ConsolidateCommentsPattern, ConstantMatchCleanupPattern,
    ContinueBreakCleanup, CompactTempNamesPattern, CompoundAssignCleanupPattern, ConversionEliminationPattern, CopyPayloadPattern, CountedLoopPattern,
    DeadTempCleanupPattern, DecodeStatusGuardPattern, DeduplicateVariablesPattern, EmptyIfBlockPattern, ForEachValPattern,
    ElseCompactionPattern,
    FunctionSignaturePattern, GuardBlockBreaks, GuardBreakUnreachablePattern,
    IfChainToGuardsPattern, IfConditionCleanupPattern, I128DecodeSlotsPattern, I128SemanticPropagationPattern, InlineFrameBasePattern, InlineVecBuilderMacroPattern,
    InlineValRoundtripPattern, IrLabelCleanup, LabelBlockCollapse, LabelGuardIf, LabelIfBreakToIfElsePattern, LabelMatchBreakGuard, LabelLoopIfContinueToWhilePattern, LabelLoopIfElseToWhilePattern, LabelTrapTailInlinePattern, LabeledSingleLoopBreakToWhilePattern,
    LinearMemoryVecBuildPattern, RedundantTypeCheckPattern, RemoveTerminalReturnPattern,
    RemoveUnusedLocalsPattern,
    RemoveUnusedParamMutPattern,
    LoopComplementaryIfUnwrapPattern, LoopGuardChainToIf, LoopGuardToIf, LoopIfBreakTailToWhilePattern, LoopIfOnlyToWhileTextPattern, LoopIfUnreachableToBlock, LoopSingleIfToWhilePattern, LoopToWhile,
    MathOperationsPattern, MissingSemicolonsPattern, MloadTempAssignFoldPattern, NextStringWhile,
    RedundantScopePattern,
    PruneEmptyIfBlocksPattern, ReturnVoidCleanupPattern, TerminalScopeUnwrapPattern,
    SerializeBytesFixPattern, SimpleLoopUnlabel, SinglePassLoopCleanup, SinglePassUnlabeledLoopCleanup, SmartVariableNamingPattern,
    StackCopyVecReturnPattern, StatusGuardBlockUnwrapPattern, StatusResultGuardLoopPattern, StatusResultGuardTextPattern, StorageAccessPattern, SymbolLiteralRecoveryPattern, TrailingUnreachableCleanupPattern,
    StatusResultGuardLabelPattern,
    TypeTagGuardCleanupPattern, TypeTagGuardStripPattern, VmScaffoldCleanupPattern,
    VecBuilderAssignmentPattern,
    UnreachableCleanupPattern,
    RemoveMutSelfPattern,
    RemoveUnnecessaryReturnPattern,
    RemoveUnreachableEndPattern,
    RemoveTypeTagChecksPattern,
    EliminateStackFramePattern,
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
    engine.patterns = default_patterns();
    engine
}

pub fn default_patterns() -> Vec<Box<dyn Pattern>> {
    let mut patterns: Vec<Box<dyn Pattern>> = Vec::new();
    register_cfg_phase(&mut patterns);
    register_soroban_phase(&mut patterns);
    register_cleanup_phase(&mut patterns);
    patterns
}

fn register_cfg_phase(patterns: &mut Vec<Box<dyn Pattern>>) {
    patterns.push(Box::new(IrLabelCleanup::new()));
    patterns.push(Box::new(SinglePassLoopCleanup::new()));
    patterns.push(Box::new(NextStringWhile::new()));
    patterns.push(Box::new(LabelBlockCollapse::new()));
    patterns.push(Box::new(LabelGuardIf::new()));
    patterns.push(Box::new(LabeledSingleLoopBreakToWhilePattern::new()));
    patterns.push(Box::new(LabelLoopIfContinueToWhilePattern::new()));
    patterns.push(Box::new(LabelIfBreakToIfElsePattern::new()));
    patterns.push(Box::new(LabelMatchBreakGuard::new()));
    patterns.push(Box::new(LabelLoopIfElseToWhilePattern::new()));
    patterns.push(Box::new(LoopIfBreakTailToWhilePattern::new()));
    patterns.push(Box::new(LabelTrapTailInlinePattern::new()));
    // Disabled: can produce malformed guard if/else around labels in some contracts.
    // engine.register(LabelBlockTailGuard::new());
    // Disabled: may rewrite labeled guards into malformed detached else blocks.
    // engine.register(LabelIfChain::new());
    // Disabled: can erase valid labeled-branch semantics (e.g., token transfer var4==6 path).
    // Re-enable only after data-flow safe conversion constraints are implemented.
    // engine.register(LabelBlockToLoop::new());
    // Disabled: often introduces malformed if/else around guard blocks.
    // engine.register(ExitFlagLoopCollapse::new());
    patterns.push(Box::new(LoopIfUnreachableToBlock::new()));
    patterns.push(Box::new(LoopGuardChainToIf::new()));
    patterns.push(Box::new(LoopGuardToIf::new()));
    patterns.push(Box::new(GuardBlockBreaks::new()));
    patterns.push(Box::new(BreakToLabelPattern::new()));
    // Disabled together with ExitFlagLoopCollapse to keep control-flow valid.
    // engine.register(ExitFlagDefaultAssign::new());
    // Disabled: can generate invalid if/else + break structure in complex contracts.
    // engine.register(LoopIfBreakElse::new());
    // Disabled: can erase guard semantics by flattening terminal break paths too early.
    // engine.register(LoopBreakTailReturn::new());
    // Disabled: occasionally unwraps into syntactically invalid else placement.
    // engine.register(UnwrapIfElseBlock::new());
    // Disabled: can interfere with label-based CFG and produce broken branches.
    // engine.register(GuardEarlyReturn::new());
    // engine.register(LoopUnreachableElse::new());
    patterns.push(Box::new(SinglePassUnlabeledLoopCleanup::new()));
    patterns.push(Box::new(LoopComplementaryIfUnwrapPattern::new()));
    patterns.push(Box::new(LoopSingleIfToWhilePattern::new()));
    patterns.push(Box::new(LoopIfOnlyToWhileTextPattern::new()));
    patterns.push(Box::new(LoopToWhile::new()));
    patterns.push(Box::new(SimpleLoopUnlabel::new()));
    patterns.push(Box::new(CopyPayloadPattern::new()));
    // Disabled: can over-compress label chains and lose guard branches in complex CFGs.
    // engine.register(LabelLadderInline::new());
    // engine.register(LabelLadderReduce::new());
    patterns.push(Box::new(ForEachValPattern::new()));
    patterns.push(Box::new(CountedLoopPattern::new()));
    patterns.push(Box::new(ContinueBreakCleanup::new()));
}

fn register_soroban_phase(patterns: &mut Vec<Box<dyn Pattern>>) {
    // New Soroban-specific patterns
    // Disabled: removing helper calls drops real control/data-flow in complex contracts
    // (e.g. write_prices). Prefer preserving calls and recovering helper semantics via
    // fingerprints/patterns.
    // engine.register(UndefinedHelpersPattern::new());
    // Disabled: current implementation is too aggressive and drops required stack-pointer locals.
    // engine.register(StackFramePattern::new());
    patterns.push(Box::new(DeduplicateVariablesPattern::new())); // Remove duplicate variable declarations
    patterns.push(Box::new(ConversionEliminationPattern::new())); // Detect and annotate val_from_i64/val_to_i64 conversions
    // Disabled for now: can produce invalid declaration order (e.g. init from not-yet-defined slot vars).
    // Re-enable after adding data-flow safety checks.
    // engine.register(InitialAssignmentPattern::new());
    patterns.push(Box::new(MissingSemicolonsPattern::new())); // Fix missing semicolons
    patterns.push(Box::new(StorageAccessPattern::new()));
    patterns.push(Box::new(MathOperationsPattern::new()));
    patterns.push(Box::new(LinearMemoryVecBuildPattern::new()));
    patterns.push(Box::new(ConstantMatchCleanupPattern::new()));
    // DISABLED: Old VariableNamingPattern causes panics on some contracts
    // engine.register(VariableNamingPattern::new());
}

fn register_cleanup_phase(patterns: &mut Vec<Box<dyn Pattern>>) {
    // NEW: Source-like code improvements - run early
    patterns.push(Box::new(EliminateStackFramePattern::new()));  // MUST run first!
    patterns.push(Box::new(RemoveMutSelfPattern::new()));
    patterns.push(Box::new(RemoveTypeTagChecksPattern::new()));
    patterns.push(Box::new(RemoveUnreachableEndPattern::new()));

    // Smart variable naming (now with single-pass protection)
    patterns.push(Box::new(SmartVariableNamingPattern::new()));
    patterns.push(Box::new(DecodeStatusGuardPattern::new()));
    patterns.push(Box::new(I128DecodeSlotsPattern::new()));
    patterns.push(Box::new(I128SemanticPropagationPattern::new()));
    patterns.push(Box::new(MloadTempAssignFoldPattern::new()));
    patterns.push(Box::new(CompactTempNamesPattern::new()));
    patterns.push(Box::new(DeduplicateVariablesPattern::new()));
    patterns.push(Box::new(InlineFrameBasePattern::new()));
    patterns.push(Box::new(StackCopyVecReturnPattern::new()));
    patterns.push(Box::new(SymbolLiteralRecoveryPattern::new()));
    patterns.push(Box::new(TypeTagGuardCleanupPattern::new()));
    patterns.push(Box::new(TypeTagGuardStripPattern::new()));
    patterns.push(Box::new(VmScaffoldCleanupPattern::new()));
    patterns.push(Box::new(GuardBreakUnreachablePattern::new()));
    patterns.push(Box::new(RedundantTypeCheckPattern::new()));
    patterns.push(Box::new(IfConditionCleanupPattern::new()));
    patterns.push(Box::new(IfChainToGuardsPattern::new()));
    patterns.push(Box::new(IfConditionCleanupPattern::new()));
    // Disabled: too aggressive; can degrade source-like output (e.g. hello_world guards).
    // engine.register(GuardUnreachableAssertPattern::new());
    // engine.register(AssertConditionCleanupPattern::new());
    patterns.push(Box::new(InlineValRoundtripPattern::new()));
    patterns.push(Box::new(VecBuilderAssignmentPattern::new()));
    patterns.push(Box::new(InlineVecBuilderMacroPattern::new()));
    patterns.push(Box::new(SerializeBytesFixPattern::new()));
    patterns.push(Box::new(CompoundAssignCleanupPattern::new()));
    patterns.push(Box::new(RedundantScopePattern::new()));
    patterns.push(Box::new(RemoveUnusedLocalsPattern::new()));
    patterns.push(Box::new(DeadTempCleanupPattern::new()));
    patterns.push(Box::new(RemoveTerminalReturnPattern::new()));
    patterns.push(Box::new(RemoveUnnecessaryReturnPattern::new()));
    patterns.push(Box::new(ReturnVoidCleanupPattern::new()));
    patterns.push(Box::new(UnreachableCleanupPattern::new()));
    patterns.push(Box::new(TerminalScopeUnwrapPattern::new()));
    patterns.push(Box::new(ElseCompactionPattern::new()));
    patterns.push(Box::new(EmptyIfBlockPattern::new()));
    patterns.push(Box::new(PruneEmptyIfBlocksPattern::new()));
    patterns.push(Box::new(LoopIfOnlyToWhileTextPattern::new()));
    patterns.push(Box::new(StatusResultGuardLoopPattern::new()));
    patterns.push(Box::new(StatusResultGuardLabelPattern::new()));
    patterns.push(Box::new(StatusResultGuardTextPattern::new()));
    patterns.push(Box::new(StatusGuardBlockUnwrapPattern::new()));
    patterns.push(Box::new(FunctionSignaturePattern::new()));
    patterns.push(Box::new(RemoveUnusedParamMutPattern::new()));
    patterns.push(Box::new(TrailingUnreachableCleanupPattern::new()));

    // Comment cleanup - MUST RUN LAST to consolidate all diagnostic comments
    patterns.push(Box::new(ConsolidateCommentsPattern::new()));
}
