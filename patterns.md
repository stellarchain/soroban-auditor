# Patterns Guide

## Goal

Patterns transform raw output into source-like Rust/Soroban code without project-specific hardcoding.

## Pipeline Order

1. `CFG phase` (`src/engine/pipeline.rs`): clean labels/loops/breaks/guards.
2. `Soroban phase`: recover meaningful SDK/stack/memory structures.
3. `Cleanup phase`: naming, semicolons, scope cleanup, signatures.

## Design Rules

- Patterns must be structural (AST/CFG), not name-based hacks.
- Preserve semantics; if ambiguous, do not rewrite.
- Prefer small, composable transforms.
- Avoid regex-only solutions for complex control flow.

## New Pattern Checklist

1. Define exact input shape.
2. Define minimal semantically-correct output.
3. Add tests:
   - `tests/pattern_per_pattern.rs` (smoke),
   - `tests/pattern_golden.rs` (golden transform).
4. Register pattern in `src/engine/pipeline.rs` in the correct phase.
5. Run:

```bash
cargo test -q --test pattern_golden --test pattern_per_pattern --test pattern_engine
```

## Acceptance Criteria

- generated code remains valid Rust;
- no regressions on playground WASM set;
- real artifact reduction (`label*`, synthetic loops, TODO noise).
