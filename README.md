# Soroban Auditor

**soroban-auditor** is a decompiler for WebAssembly smart contract binaries deployed on the Stellar ledger. It targets Soroban MVP v1 contracts and uses embedded contract specs (when present) to recover function signatures and types. The project is still under development, so decompilation can be partial and may fall back to stubs.

## Building from source
Building or installing **soroban-auditor** from source requires a recent Rust toolchain (Rust 2021 edition).

## Features
- Parses Soroban contract specs embedded in WASM and generates Rust types and function signatures.
- Dynamically generates `soroban_sdk` imports based on specs and detected host calls.
- Pattern-based emitters for common flows (token, atomic swap, events, vec/string/bytes).
- Optional raw function emission (off by default) for low-level inspection.
- Uses the WASM name section for function naming with `--use-name-section`.

## Changelog
**2026-02-07**
- Refactored the decompiler into smaller modules (`app/`, `decompile/`, `wasm_ir/`, `patterns/`) to reduce `main.rs` complexity.
- Added dynamic import generation based on contract specs and detected host calls.
- Added generalized pattern matching for common token/atomic swap/events/vec/string/bytes flows.
- Removed verbose-mode emission and kept raw functions optional (off by default).
- Cleaned warnings and consolidated utility helpers for functions/globals/spec indexing.
- Expanded account-pattern emitters (constructor/init, check_auth, add_limit, upgrade, counters).
- Improved import inference for swap flows and `require_auth_for_args` usage.
- Hardened spec type detection against formatting variations.
- Added lightweight formatting for emitted spec types and function signatures.

```
cargo build
./soroban-auditor tests/test_contract.wasm
./soroban-auditor --use-name-section tests/test_contract.wasm tests/test_contract.rs
```


### Example (Original vs Decompilation)

Original source (`tests/soroban_events_contract.original.rs`):
```rust
#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

// Define two static topics for the event: "COUNTER" and "increment".
// Also set the data format to "single-value", which means that the event data
// payload will contain a single value not nested into any data structure.
#[contractevent(topics = ["COUNTER", "increment"], data_format = "single-value")]
struct IncrementEvent {
    count: u32,
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        IncrementEvent { count }.publish(&env);
        count
    }
}
```

Decompiled output (`tests/soroban_events_contract.rs`):
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, vec, symbol_short};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn increment(&mut self, env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
}
```

Original source (`tests/soroban_atomic_swap_contract.original.rs`):
```rust
//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice; both parties could have
    // received the full amount as well).
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        // Verify preconditions on the minimum price for both parties.
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }
        // Require authorization for a subset of arguments specific to a party.
        // Notice, that arguments are symmetric - there is no difference between
        // `a` and `b` in the call and hence their signatures can be used
        // either for `a` or for `b` role.
        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env),
        );

        // Perform the swap by moving tokens from a to b and from b to a.
        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    // This call needs to be authorized by `from` address. It transfers the
    // maximum spend amount to the swap contract's address in order to decouple
    // the signature from `to` address (so that parties don't need to know each
    // other).
    token.transfer(from, &contract_address, &max_spend_amount);
    // Transfer the necessary amount to `to`.
    token.transfer(&contract_address, to, &transfer_amount);
    // Refund the remaining balance to `from`.
    token.transfer(
        &contract_address,
        from,
        &(max_spend_amount - transfer_amount),
    );
}
```

Decompiled output (`tests/soroban_atomic_swap_contract.rs`):
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn swap(&mut self, env: Env, a: soroban_sdk::Address, b: soroban_sdk::Address, token_a: soroban_sdk::Address, token_b: soroban_sdk::Address, amount_a: i128, min_b_for_a: i128, amount_b: i128, min_a_for_b: i128) {
        a.require_auth_for_args((token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env));
        b.require_auth_for_args((token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env));
        self.move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        self.move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
    fn move_token(&self, env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: i128, transfer_amount: i128) {
        let token = token::Client::new(env, token);
        let contract_address = env.current_contract_address();
        token.transfer(from, &contract_address, &max_spend_amount);
        token.transfer(&contract_address, to, &transfer_amount);
        token.transfer(&contract_address, from, &(max_spend_amount - transfer_amount));
    }
}
```

## Developers
[stellarchain.io](https://stellarchain.io)

## Development Plan
https://stellarchain.notion.site/Stellar-Soroban-Smart-Contract-Audit-235524ad585a4183a7e0f0025cb18abf
