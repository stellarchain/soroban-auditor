#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol, Val, Address, FromVal, IntoVal, Map, Bytes, BytesN, String};

#[contract]
pub struct HelloWorldContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn err_contract(code: u32) -> i64 {
    ((soroban_sdk::xdr::ScErrorType::Contract as u32 as i64) & 255).wrapping_shl(32 as u32) | (code as i64)
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

macro_rules! mload8 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load8($addr)
        }};
}
macro_rules! mload16 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load16($addr)
        }};
}
macro_rules! mload32 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load32($addr)
        }};
}
macro_rules! mload64 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load64($addr)
        }};
}
macro_rules! mstore8 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store8($addr, $value)
        }};
}
macro_rules! mstore16 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store16($addr, $value)
        }};
}
macro_rules! mstore32 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store32($addr, $value)
        }};
}
macro_rules! mstore64 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store64($addr, $value)
        }};
}
macro_rules! msize {
    () => {{
            let Self { memory, .. } = self;
            memory.size()
        }};
}
macro_rules! mgrow {
    ($pages:expr) => {{
            let Self { memory, .. } = self;
            memory.grow($pages)
        }};
}


#[contractimpl]
impl HelloWorldContract {
    pub fn hello(&mut self, env: Env, to: soroban_sdk::Symbol) -> soroban_sdk::Vec<soroban_sdk::Symbol> {
        vec![&env, Symbol::new(&env, "Hello"), to]
    }
}