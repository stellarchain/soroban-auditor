# Soroban Auditor Pipeline Architecture

## Scope

Acest document descrie pipeline-ul real de decompilare + postprocesare din codul curent, nu un design teoretic.

## End-to-end flow

Entry point este `run()` in `src/app/mod.rs:1009`.

1. Input + parse wasm
- Citeste modulul wasm (`deserialize_file`), parseaza names section, extrage segmente/date/string-uri.
- Incarca contract specs din SDK backend.

2. Build metadata IR
- Construieste tabelele de functii/importuri/globals si detecteaza forwarders:
  - simple (`build_call_forwarders`)
  - complexe (`ForwarderAnalyzer`)
- Face fingerprinting pe functiile interne si poate redenumi functie daca exista fingerprint cunoscut (`suggested_name_for_fingerprint`).

3. Emit header + type scaffolding
- Scrie `#![no_std]`, `use soroban_sdk::{...}`, `#[contract] struct ...`.
- Emite helper-ele low-level: `val_from_i64`, `val_to_i64`, `err_contract`, `address_from_i64`.
- Emite tipuri din specs (sau account-specific blocks).

4. Emit `#[contractimpl]` functions (path principal)
- Pentru fiecare export:
  - gaseste spec (`find_spec_for_export` in `src/app/specs.rs:189`)
  - construieste `PatternContextData` din scan pe body wasm (`src/app/specs.rs:23`)
  - construieste `PatternContext`
- Ordinea de generare este:
  1. template-based (`TemplateMatcher::try_generate_with_context`)
  2. hardcoded semantic patterns (`crate::patterns::try_emit` in `src/patterns/mod.rs:56`)
  3. fallback raw decompilation (`crate::code_builder::build`) direct in functia spec.

5. Optional raw helper impl
- Daca `SOROBAN_AUDITOR_EMIT_RAW_FUNCTIONS` este setat:
  - emite un al doilea `impl ContractName { ... }` cu functii interne non-publice.
  - fluxul este `raw::emit_raw_functions` din `src/app/raw.rs:8`.
  - Pentru fiecare functie: `try_emit_raw_rewrite(...)` first, apoi fallback `code_builder::build(...)`.

6. Account trait emission
- Pentru account contracts, emite separat `#[contractimpl(contracttrait)] impl CustomAccountInterface ...`.

7. Final postprocess chain
- `postprocess_memory_macros` (`src/app/mod.rs:49`)
  - inlocuieste `self.memory.load/store/...` cu `mload!/mstore!/msize!/mgrow!`
  - injecteaza `include!(.../memory_macros.rs)` daca lipseste
  - ruleaza engine pipeline: `crate::engine::default_engine().apply(out)`
  - apoi `fix_guard_breaks`, `format_function_signatures`, `normalize_indentation`, `fix_trailing_braces`
- `postprocess_remove_unused_methods` (`src/app/mod.rs:338`)
- `postprocess_apply_patterns` (`src/app/mod.rs:461`) este momentan no-op (dezactivat)
- Scrie output `.rs`.

## Engine pipeline

`default_engine()` este definit in `src/engine/pipeline.rs:76`.

- Engine-ul sparge fisierul in blocuri functie (`split_functions`), ruleaza pattern-uri iterativ pana la fixpoint, max 20 iteratii/functie.
- Daca depaseste 20 iteratii, afiseaza warning si continua cu starea curenta.
- Recompune fisierul cu `join_functions`.

Pattern families (in ordine):
- CFG/label cleanup: `IrLabelCleanup`, `LabelBlockCollapse`, `LabelGuardIf`, `LabelIfChain`, `LabelBlockToLoop`, etc.
- Loop normalization: `LoopIf...`, `LoopToWhile`, `SimpleLoopUnlabel`, `CountedLoopPattern`.
- Soroban cleanups: `StackFramePattern`, `DeduplicateVariablesPattern`, `StorageAccessPattern`, `MathOperationsPattern`.
- Syntax/signature polish: `MissingSemicolonsPattern`, `SmartVariableNamingPattern`, `FunctionSignaturePattern`.
- Final comment consolidation: `ConsolidateCommentsPattern` (last).

## Raw path internals

`emit_raw_functions` (`src/app/raw.rs:8`) face:
- filtrare pe functie (`filter`, `skip_pure`)
- fingerprint logging (`SOROBAN_AUDITOR_DUMP_FINGERPRINTS`, `SOROBAN_AUDITOR_FP_TARGET`)
- rewrite shortcuts prin `try_emit_raw_rewrite`
- fallback universal prin `code_builder::build` (instr-by-instr translation).

`code_builder::build` este emitter-ul low-level folosit in ambele locuri:
- fallback pentru functii spec nerecunoscute
- raw helper impl optional

## Where name-based behavior still exists

Exista inca pattern-uri dependente de nume:
- `GetPricesPattern` aplica doar pe `block.name == "get_prices"` (`src/engine/patterns/get_prices.rs:18`)
- `ReadPricesPattern` aplica doar pe `block.name == "read_prices"` (`src/engine/patterns/read_prices.rs:18`)
- Mai multe hardcoded patterns din `src/patterns/*` conditioneaza pe `ctx.export_name`.

Impact:
- daca functia este semantic identica dar redenumita (`get_all_prices`), pattern-ul name-based poate sa nu se aplice.
- pentru reverse engineering robust, aceste pattern-uri trebuie mutate pe semnal structural:
  - fingerprint + shape matching pe instructiuni
  - pattern pe access/storage flow
  - signature + dataflow constraints.

## Practical debugging toggles

- `SOROBAN_AUDITOR_DEBUG`:
  - logheaza forwarders detectati.
- `SOROBAN_AUDITOR_DUMP_FP`:
  - dump fingerprint pentru functii in path-ul principal.
- `SOROBAN_AUDITOR_DUMP_FINGERPRINTS` + `SOROBAN_AUDITOR_FP_TARGET`:
  - dump fingerprint/instructions in raw path.
- `SOROBAN_AUDITOR_EMIT_RAW_FUNCTIONS`:
  - emite impl-ul secundar cu functii raw helper.

## Current architecture summary

Pipeline-ul are trei straturi de reconstructie, in ordinea de prioritate:
1. high-level template emit
2. high-level hardcoded semantic emit
3. low-level wasm decompile + cleanup pipeline

Pentru reverse engineering "proper" (name-agnostic), directia corecta este sa reduci dependenta de nume in straturile 1-2 si sa promovezi detection pe fingerprint/structura/dataflow.
