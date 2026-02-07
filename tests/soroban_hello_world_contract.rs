#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(&mut self, env: Env, to: soroban_sdk::Symbol) -> soroban_sdk::Vec<soroban_sdk::Symbol> {
        vec![&env, Symbol::new(&env, "Hello"), to]
    }
}
