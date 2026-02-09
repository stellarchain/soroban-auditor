#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, String, Val, FromVal, Vec, Map, Bytes, BytesN, Symbol};

#[contract]
pub struct TokenContract.optimized;

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

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceDataKey { pub from: soroban_sdk::Address, pub spender: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceValue { pub amount: i128, pub expiration_ledger: u32, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Allowance(AllowanceDataKey), Balance(soroban_sdk::Address), State(soroban_sdk::Address), Admin, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokenMetadata { pub decimal: u32, pub name: soroban_sdk::String, pub symbol: soroban_sdk::String, }

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
impl TokenContract.optimized {
    pub fn initialize(&mut self, env: Env, admin: soroban_sdk::Address, decimal: u32, name: soroban_sdk::String, symbol: soroban_sdk::String) {
        if let Some(_) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        let metadata = TokenMetadata { decimal: decimal, name: name, symbol: symbol };
        env.storage().instance().set(&DataKey::State(env.current_contract_address()), &metadata);
    }
    pub fn mint(&mut self, env: Env, to: soroban_sdk::Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
    }
    pub fn set_admin(&mut self, env: Env, new_admin: soroban_sdk::Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    pub fn allowance(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address) -> i128 {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount
    }
    pub fn approve(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let value = AllowanceValue { amount: amount, expiration_ledger: expiration_ledger };
        if amount > 0 {
            env.storage().temporary().set(&key, &value);
        } else {
            env.storage().temporary().remove(&key);
        }
    }
    pub fn balance(&mut self, env: Env, id: soroban_sdk::Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }
    pub fn transfer(&mut self, env: Env, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }
    pub fn transfer_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        spender.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount -= amount;
        env.storage().temporary().set(&key, &allowance);
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }
    pub fn burn(&mut self, env: Env, from: soroban_sdk::Address, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(balance - amount));
    }
    pub fn burn_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, amount: i128) {
        spender.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount -= amount;
        env.storage().temporary().set(&key, &allowance);
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(balance - amount));
    }
    pub fn decimals(&mut self, env: Env) -> u32 {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.decimal
    }
    pub fn name(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.name
    }
    pub fn symbol(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.symbol
    }