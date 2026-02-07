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
