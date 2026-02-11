#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol};

#[contract]
pub struct HelloWorldContract;


#[contractimpl]
impl HelloWorldContract {

    pub fn hello(
        env: Env,
        to: Symbol,
    ) -> Vec<Symbol> {
        return vec![&env, Symbol::new(env, "Hello"), to];
    }
}
