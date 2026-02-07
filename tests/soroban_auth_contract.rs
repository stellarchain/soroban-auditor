#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contract]
pub struct Contract;

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Counter(soroban_sdk::Address), }

#[contractimpl]
impl Contract {
    pub fn increment(&mut self, env: Env, user: soroban_sdk::Address, value: u32) -> u32 {
        user.require_auth();
        let key = DataKey::Counter(user.clone());
        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or_default();
        count += value;
        env.storage().persistent().set(&key, &count);
        count
    }
}
