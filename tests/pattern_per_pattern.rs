use soroban_auditor::engine::function::FunctionBlock;
use soroban_auditor::engine::pattern::Pattern;
use soroban_auditor::engine::patterns::*;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn corpus() -> Vec<FunctionBlock> {
    vec![
        FunctionBlock {
            header: "pub fn sample(&mut self, env: Env) -> i32 {".to_string(),
            body: vec![
                "    let mut a: i32 = 0;".to_string(),
                "    loop {".to_string(),
                "        if a == 3 {".to_string(),
                "            break;".to_string(),
                "        }".to_string(),
                "        a = a.wrapping_add(1);".to_string(),
                "        continue;".to_string(),
                "    }".to_string(),
                "    return a;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "sample".to_string(),
        },
        FunctionBlock {
            header: "pub fn sample_vec(&mut self, env: Env, to: Symbol) -> Vec<Symbol> {"
                .to_string(),
            body: vec![
                "    let mut vec_builder: i64 = to as i64;".to_string(),
                "    let mut slot_var1_0_i64: i64 = Hello as i64;".to_string(),
                "    let tmp = { let mut v = Vec::<Val>::new(env); v.push_back(val_from_i64(slot_var1_0_i64)); v.push_back(val_from_i64(vec_builder)); val_to_i64(v.into_val(env)) };".to_string(),
                "    let to = tmp;".to_string(),
                "    return to;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "sample_vec".to_string(),
        },
        FunctionBlock {
            header: "pub fn sample_labels(&mut self, env: Env) -> i32 {".to_string(),
            body: vec![
                "    let mut b: i32 = 0;".to_string(),
                "    'label1: loop {".to_string(),
                "        if b == 0 {".to_string(),
                "            b = b.wrapping_add(1);".to_string(),
                "            continue 'label1;".to_string(),
                "        }".to_string(),
                "        if b != 2 {".to_string(),
                "            break 'label1;".to_string(),
                "        }".to_string(),
                "        break;".to_string(),
                "    }".to_string(),
                "    return b;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "    ".to_string(),
            name: "sample_labels".to_string(),
        },
    ]
}

fn assert_no_panic<P: Pattern>(pattern: &P) {
    for block in corpus() {
        let result = catch_unwind(AssertUnwindSafe(|| pattern.apply(&block)));
        assert!(
            result.is_ok(),
            "pattern `{}` panicked on `{}`",
            pattern.name(),
            block.name
        );
    }
}

macro_rules! pattern_test {
    ($test_name:ident, $pattern_ty:ty) => {
        #[test]
        fn $test_name() {
            let pattern = <$pattern_ty>::new();
            assert_no_panic(&pattern);
        }
    };
}

pattern_test!(pattern_ir_label_cleanup, IrLabelCleanup);
pattern_test!(pattern_single_pass_loop_cleanup, SinglePassLoopCleanup);
pattern_test!(pattern_next_string_while, NextStringWhile);
pattern_test!(pattern_label_block_collapse, LabelBlockCollapse);
pattern_test!(pattern_label_guard_if, LabelGuardIf);
pattern_test!(
    pattern_labeled_single_loop_break_to_while,
    LabeledSingleLoopBreakToWhilePattern
);
pattern_test!(
    pattern_label_loop_if_continue_to_while,
    LabelLoopIfContinueToWhilePattern
);
pattern_test!(
    pattern_label_if_break_to_if_else,
    LabelIfBreakToIfElsePattern
);
pattern_test!(pattern_label_match_break_guard, LabelMatchBreakGuard);
pattern_test!(
    pattern_label_loop_if_else_to_while,
    LabelLoopIfElseToWhilePattern
);
pattern_test!(
    pattern_loop_if_break_tail_to_while,
    LoopIfBreakTailToWhilePattern
);
pattern_test!(pattern_label_trap_tail_inline, LabelTrapTailInlinePattern);
pattern_test!(pattern_loop_if_unreachable_to_block, LoopIfUnreachableToBlock);
pattern_test!(pattern_loop_guard_chain_to_if, LoopGuardChainToIf);
pattern_test!(pattern_loop_guard_to_if, LoopGuardToIf);
pattern_test!(pattern_guard_block_breaks, GuardBlockBreaks);
pattern_test!(pattern_break_to_label, BreakToLabelPattern);
pattern_test!(
    pattern_single_pass_unlabeled_loop_cleanup,
    SinglePassUnlabeledLoopCleanup
);
pattern_test!(pattern_loop_to_while, LoopToWhile);
pattern_test!(pattern_simple_loop_unlabel, SimpleLoopUnlabel);
pattern_test!(pattern_copy_payload, CopyPayloadPattern);
pattern_test!(pattern_for_each_val, ForEachValPattern);
pattern_test!(pattern_counted_loop, CountedLoopPattern);
pattern_test!(pattern_continue_break_cleanup, ContinueBreakCleanup);
pattern_test!(
    pattern_deduplicate_variables,
    DeduplicateVariablesPattern
);
pattern_test!(
    pattern_conversion_elimination,
    ConversionEliminationPattern
);
pattern_test!(pattern_missing_semicolons, MissingSemicolonsPattern);
pattern_test!(pattern_storage_access, StorageAccessPattern);
pattern_test!(pattern_literal_comment_cleanup, LiteralCommentCleanupPattern);
pattern_test!(pattern_math_operations, MathOperationsPattern);
pattern_test!(pattern_linear_memory_vec_build, LinearMemoryVecBuildPattern);
pattern_test!(pattern_constant_match_cleanup, ConstantMatchCleanupPattern);
pattern_test!(pattern_smart_variable_naming, SmartVariableNamingPattern);
pattern_test!(pattern_compact_temp_names, CompactTempNamesPattern);
pattern_test!(pattern_inline_frame_base, InlineFrameBasePattern);
pattern_test!(pattern_stack_copy_vec_return, StackCopyVecReturnPattern);
pattern_test!(pattern_symbol_literal_recovery, SymbolLiteralRecoveryPattern);
pattern_test!(pattern_type_tag_guard_cleanup, TypeTagGuardCleanupPattern);
pattern_test!(pattern_vm_scaffold_cleanup, VmScaffoldCleanupPattern);
pattern_test!(
    pattern_guard_break_unreachable,
    GuardBreakUnreachablePattern
);
pattern_test!(pattern_redundant_type_check, RedundantTypeCheckPattern);
pattern_test!(pattern_if_condition_cleanup, IfConditionCleanupPattern);
pattern_test!(pattern_if_chain_to_guards, IfChainToGuardsPattern);
pattern_test!(pattern_inline_val_roundtrip, InlineValRoundtripPattern);
pattern_test!(pattern_init_reassign_fold, InitReassignFoldPattern);
pattern_test!(pattern_vec_builder_assignment, VecBuilderAssignmentPattern);
pattern_test!(pattern_inline_vec_builder_macro, InlineVecBuilderMacroPattern);
pattern_test!(pattern_serialize_bytes_fix, SerializeBytesFixPattern);
pattern_test!(
    pattern_unused_let_side_effect_preserve,
    UnusedLetSideEffectPreservePattern
);
pattern_test!(pattern_shift_by_zero_cleanup, ShiftByZeroCleanupPattern);
pattern_test!(pattern_compound_assign_cleanup, CompoundAssignCleanupPattern);
pattern_test!(pattern_redundant_scope, RedundantScopePattern);
pattern_test!(pattern_remove_unused_locals, RemoveUnusedLocalsPattern);
pattern_test!(pattern_dead_temp_cleanup, DeadTempCleanupPattern);
pattern_test!(pattern_remove_terminal_return, RemoveTerminalReturnPattern);
pattern_test!(pattern_return_void_cleanup, ReturnVoidCleanupPattern);
pattern_test!(pattern_unreachable_cleanup, UnreachableCleanupPattern);
pattern_test!(pattern_terminal_scope_unwrap, TerminalScopeUnwrapPattern);
pattern_test!(pattern_else_compaction, ElseCompactionPattern);
pattern_test!(pattern_empty_if_block, EmptyIfBlockPattern);
pattern_test!(pattern_prune_empty_if_blocks, PruneEmptyIfBlocksPattern);
pattern_test!(pattern_status_result_guard_loop, StatusResultGuardLoopPattern);
pattern_test!(
    pattern_status_result_guard_label,
    StatusResultGuardLabelPattern
);
pattern_test!(
    pattern_status_result_guard_text,
    StatusResultGuardTextPattern
);
pattern_test!(pattern_function_signature, FunctionSignaturePattern);
pattern_test!(
    pattern_trailing_unreachable_cleanup,
    TrailingUnreachableCleanupPattern
);
pattern_test!(pattern_consolidate_comments, ConsolidateCommentsPattern);
