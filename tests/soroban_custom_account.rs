#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, Val, IntoVal, BytesN, crypto::Hash, FromVal, Map, Bytes, String, Symbol};

#[contract]
pub struct CustomAccount;

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
impl CustomAccount {
    pub fn init(&mut self, env: Env, owners: soroban_sdk::Vec<soroban_sdk::BytesN<32>>) {
        let var1 = self.func18(env, 1056);
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64(owners)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64(owners)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64(owners)); 0 } }
        var2;
        0 /* Void */
    }
    pub fn add_limit(&mut self, env: Env, token: soroban_sdk::Address, limit: i128) {
        token.require_auth_for_args((limit).into_val(&env));
    }
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, signatures: soroban_sdk::Vec<soroban_sdk::Val>, auth_context: soroban_sdk::Vec<soroban_sdk::Val>) -> Result<(), AccError> {
        Ok(())
    }
}