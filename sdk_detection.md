# SDK Detection (Soroban)

## What It Does

The system detects Soroban host calls from WASM and uses them for:

- clearer Rust call mapping;
- usage reporting by modules/categories;
- pattern/fingerprint signals.

## Sources

- function catalog: `src/sdk/soroban/common_env_soroban.json`
- detector: `src/sdk/detector.rs`
- usage analyzer: `src/sdk/analyzer.rs`
- call mappers: `src/engine/sdk_call_mapper.rs` and `src/code_builder/sdk_map.rs`

## Simple Flow

1. Detect host imports.
2. Normalize names (including fingerprint aliases).
3. Apply mapping rule:
   - method-style (`vec_len(x)` -> `x.len()`),
   - namespace-style (`env.crypto().bls12_381().*`),
   - direct-style (`env.storage().*`).
4. If there is no safe rule, keep low-level form (no guessed semantics).

## Useful Commands

```bash
cargo run --bin sdk-analyze -- <contract.wasm>
cargo run --bin sdk-analyze -- <contract.wasm> --detailed
```

## Quality Rule

- No contract-specific hardcoding.
- No aggressive mapping when semantics are uncertain.
- Every new mapping must be test-covered.
