# soroban-auditor

Decompiler for Soroban WASM contracts -> source-like Rust (focus: reverse engineering, not perfect recompilation).

## Quick Start

```bash
cargo build
cargo run --bin soroban-auditor -- <input.wasm> [output.rs]
```

Useful option:

```bash
cargo run --bin soroban-auditor -- --use-name-section <input.wasm> [output.rs]
```

## Know-How (Real Pipeline)

1. Parse WASM + imports + exports + Soroban specs.
2. Emit raw Rust from `code_builder` (no contract-specific hardcoding).
3. Run engine patterns (CFG + Soroban + cleanup) until fixpoint.
4. Run minimal safe postprocess (signatures, indentation, structural cleanup).
5. Write final output file.

## Useful Toggles

- `SOROBAN_AUDITOR_SKIP_ENGINE=1`: view pre-pattern output.
- `SOROBAN_AUDITOR_SKIP_POSTPROCESS=1`: view pre-postprocess output.
- `SOROBAN_AUDITOR_ENABLE_POSTPROCESS=1`: enable extra cosmetic cleanup.
- `SOROBAN_AUDITOR_EMIT_RAW_FUNCTIONS=1`: emit internal raw helper functions too.
- `SOROBAN_AUDITOR_DUMP_FP=1`: dump per-function fingerprint.

## Code Layout

- `src/app`: decompilation orchestration + header/spec/export/raw/postprocess.
- `src/code_builder`: opcode-to-Rust line-by-line emitter.
- `src/engine`: pattern engine (CFG/Soroban/cleanup).
- `src/sdk`: Soroban SDK detection + mapping.
- `src/fingerprint*.rs`: fingerprint + registry.

## Rules for Proper Reverse Engineering

- No hardcoding by contract/function name.
- Patterns must be CFG/dataflow-structural.
- Semantic validity first, cosmetics second.
- Add a test for every new pattern.

## Testing

```bash
cargo test -q --test pattern_golden --test pattern_per_pattern --test pattern_engine
cargo check -q
```
