use soroban_auditor::engine::function::FunctionBlock;
use soroban_auditor::engine::pattern::Pattern;
use soroban_auditor::engine::patterns::{
    CompoundAssignCleanupPattern, FunctionSignaturePattern, IfConditionCleanupPattern,
    InlineVecBuilderMacroPattern, LoopIfBreakTailToWhilePattern, LoopToWhile,
    MissingSemicolonsPattern, RemoveTerminalReturnPattern, ReturnVoidCleanupPattern, StackCopyVecReturnPattern,
    SerializeBytesFixPattern, ShiftByZeroCleanupPattern, LiteralCommentCleanupPattern, InitReassignFoldPattern, UnusedLetSideEffectPreservePattern, TerminalScopeUnwrapPattern, VecBuilderAssignmentPattern,
};

fn apply<P: Pattern>(pattern: P, block: FunctionBlock) -> FunctionBlock {
    pattern.apply(&block).unwrap_or(block)
}

fn mk_block(header: &str, body: &[&str]) -> FunctionBlock {
    FunctionBlock {
        header: header.to_string(),
        body: body.iter().map(|s| s.to_string()).collect(),
        footer: "}".to_string(),
        indent: "    ".to_string(),
        name: "test".to_string(),
    }
}

#[test]
fn golden_missing_semicolons_adds_expected_semicolons() {
    let block = mk_block(
        "pub fn t() {",
        &["    let a = 1", "    a = a.wrapping_add(1)", "    return a"],
    );
    let out = apply(MissingSemicolonsPattern::new(), block);
    assert_eq!(out.body[0], "    let a = 1;");
    assert_eq!(out.body[1], "    a = a.wrapping_add(1);");
    assert_eq!(out.body[2], "    return a;");
}

#[test]
fn golden_remove_terminal_return_removes_only_unit_return() {
    let block = mk_block("pub fn t() {", &["    let a = 1;", "    return;"]);
    let out = apply(RemoveTerminalReturnPattern::new(), block);
    assert_eq!(out.body, vec!["    let a = 1;".to_string()]);
}

#[test]
fn golden_return_void_cleanup_drops_terminal_void_literal() {
    let block = mk_block("    pub fn t() {", &["        do_work();", "        0 /* Void */"]);
    let out = apply(ReturnVoidCleanupPattern::new(), block);
    assert_eq!(out.body, vec!["        do_work();".to_string()]);
}

#[test]
fn golden_function_signature_splits_params_multiline() {
    let block = mk_block(
        "    pub fn mint(&mut self, env: Env, to: Address, amount: i128) -> i128 {",
        &["    return amount;"],
    );
    let out = apply(FunctionSignaturePattern::new(), block);
    assert!(out.header.contains("\n            &mut self,"));
    assert!(out.header.contains("\n            env: Env,"));
    assert!(out.header.contains("\n            to: Address,"));
    assert!(out.header.contains("\n            amount: i128"));
    assert!(out.header.contains(") -> i128 {"));
}

#[test]
fn golden_if_condition_cleanup_normalizes_bool_cast_if() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    if ((a != b) as i32 != 0) {",
            "        x = 1;",
            "    }",
        ],
    );
    let out = apply(IfConditionCleanupPattern::new(), block);
    assert!(out.body.iter().any(|l| l.trim() == "if a != b {"));
}

#[test]
fn golden_if_condition_cleanup_unwraps_double_negation() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    if !(!(Address::try_from_val(env, &val_from_i64(addr)).is_ok())) {",
            "        x = 1;",
            "    }",
        ],
    );
    let out = apply(IfConditionCleanupPattern::new(), block);
    assert!(
        out.body
            .iter()
            .any(|l| l.trim() == "if Address::try_from_val(env, &val_from_i64(addr)).is_ok() {")
    );
}

#[test]
fn golden_if_condition_cleanup_unwraps_double_negation_inside_complex_cond() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    if (!(Address::try_from_val(env, &val_from_i64(a)).is_ok())) as i32 | !(!(Address::try_from_val(env, &val_from_i64(b)).is_ok())) {",
            "        x = 1;",
            "    }",
        ],
    );
    let out = apply(IfConditionCleanupPattern::new(), block);
    let head = out.body[0].trim().to_string();
    assert!(!head.contains("!(!("));
    assert!(head.contains("Address::try_from_val(env, &val_from_i64(b)).is_ok()"));
}

#[test]
fn golden_vec_builder_assignment_rewrites_to_vec_builder_assignment() {
    let block = mk_block(
        "pub fn t() {",
        &[r#"    { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(a)); val_to_i64(v.into_val(env)) };"#],
    );
    let out = apply(VecBuilderAssignmentPattern::new(), block);
    assert_eq!(
        out.body[0],
        "    vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(a)); val_to_i64(v.into_val(env)) };;"
    );
}

#[test]
fn golden_inline_vec_builder_macro_rewrites_inline_builder_expr() {
    let block = mk_block(
        "pub fn t() {",
        &[r#"    let e = { let mut v = Vec::<Val>::new(env); v.push_back(val_from_i64(a)); v.push_back(val_from_i64(b)); val_to_i64(v.into_val(env)) };"#],
    );
    let out = apply(InlineVecBuilderMacroPattern::new(), block);
    assert_eq!(
        out.body[0],
        "    let e = val_to_i64(vec![&env, val_from_i64(a), val_from_i64(b)].into_val(env));"
    );
}

#[test]
fn golden_compound_assign_cleanup_simplifies_invalid_self_assign() {
    let block = mk_block("pub fn t() {", &["    b = b += 1;"]);
    let out = apply(CompoundAssignCleanupPattern::new(), block);
    assert_eq!(out.body[0], "    b += 1;");
}

#[test]
fn golden_serialize_bytes_fix_removes_todo_and_uses_into_val() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    let ac = val_to_i64(Bytes::from_val(env, &val_from_i64(ab)).into()) /* TODO: serialize_to_bytes */;",
        ],
    );
    let out = apply(SerializeBytesFixPattern::new(), block);
    assert_eq!(
        out.body[0],
        "    let ac = val_to_i64(Bytes::from_val(env, &val_from_i64(ab)).into_val(env));"
    );
}

#[test]
fn golden_stack_copy_vec_return_collapses_to_vec_macro_return() {
    let header = "pub fn hello(\n        &mut self,\n        env: Env,\n        to: Symbol,\n    ) -> Vec<Symbol> {";
    let block = mk_block(
        header,
        &[
            "    let mut a: i32 = 0;",
            "    let mut b: i32 = 0;",
            "    a = 0;",
            "    b = 0;",
            "    let tmp = { let mut v = Vec::<Val>::new(env); v.push_back(val_from_i64(Hello)); v.push_back(val_from_i64(to)); val_to_i64(v.into_val(env)) };",
            "    to = tmp;",
            "    self.global0 = 32;",
            "    return to;",
        ],
    );
    let out = apply(StackCopyVecReturnPattern::new(), block);
    assert!(out.body.iter().any(|l| l.contains("return vec![&env,")));
    assert!(out.body.iter().all(|l| !l.contains("let tmp = { let mut v")));
}

#[test]
fn golden_loop_if_break_tail_to_while_lowers_structure() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    loop {",
            "        if i == 0 {",
            "            break;",
            "        }",
            "        i = i.wrapping_add(1);",
            "    }",
            "    x = 1;",
        ],
    );
    let out = apply(LoopIfBreakTailToWhilePattern::new(), block);
    assert!(out.body.iter().any(|l| l.trim() == "while i != 0 {"));
}

#[test]
fn golden_loop_to_while_lowers_labeled_loop_head_break() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    'label3: loop {",
            "        if i != 0 {",
            "            break 'label3;",
            "        }",
            "        i = i.wrapping_add(1);",
            "    }",
        ],
    );
    let out = apply(LoopToWhile::new(), block);
    assert!(out.body.iter().any(|l| l.trim() == "while i == 0 {"));
}

#[test]
fn golden_terminal_scope_unwrap_lifts_terminal_return_block() {
    let block = mk_block(
        "    pub fn t() -> i128 {",
        &[
            "        a = -32;",
            "        {",
            "            x = y;",
            "            return x;",
            "        }",
        ],
    );
    let out = apply(TerminalScopeUnwrapPattern::new(), block);
    assert_eq!(
        out.body,
        vec![
            "        a = -32;".to_string(),
            "        x = y;".to_string(),
            "        return x;".to_string()
        ]
    );
}

#[test]
fn golden_shift_by_zero_cleanup_simplifies_wrapping_shifts() {
    let block = mk_block(
        "pub fn t() -> i64 {",
        &[
            "    let a = value.wrapping_shl(0 as u32);",
            "    let b = value.wrapping_shr(0 as u32);",
            "    return a.wrapping_add(b);",
        ],
    );
    let out = apply(ShiftByZeroCleanupPattern::new(), block);
    assert_eq!(out.body[0], "    let a = value;");
    assert_eq!(out.body[1], "    let b = value;");
}

#[test]
fn golden_literal_comment_cleanup_strips_tagged_numeric_comments() {
    let block = mk_block(
        "pub fn t() {",
        &[
            "    a = 0 /* False */;",
            "    b = 1 /* True */;",
            "    c = 0 /* Void */ as i64;",
        ],
    );
    let out = apply(LiteralCommentCleanupPattern::new(), block);
    assert_eq!(out.body[0], "    a = 0;");
    assert_eq!(out.body[1], "    b = 1;");
    assert_eq!(out.body[2], "    c = 0 as i64;");
}

#[test]
fn golden_init_reassign_fold_compacts_simple_local_init() {
    let block = mk_block(
        "pub fn t() {",
        &["    let mut a: i32 = 0;", "    a = -32;", "    work(a);"],
    );
    let out = apply(InitReassignFoldPattern::new(), block);
    assert_eq!(out.body[0], "    let mut a: i32 = -32;");
    assert_eq!(out.body[1], "    work(a);");
}

#[test]
fn golden_unused_let_side_effect_preserve_keeps_call_without_temp() {
    let block = mk_block(
        "pub fn t() {",
        &["    let e = env.storage().persistent().set(&k, &v);", "    return;"],
    );
    let out = apply(UnusedLetSideEffectPreservePattern::new(), block);
    assert_eq!(out.body[0], "    env.storage().persistent().set(&k, &v);");
}
