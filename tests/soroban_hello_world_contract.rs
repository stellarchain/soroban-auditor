#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, Symbol, Val, IntoVal};

#[contract]
pub struct HelloWorldContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}

fn val_to_i64(v: Val) -> i64 {
    (unsafe { core::mem::transmute::<Val, u64>(v) }) as i64
}

#[contractimpl]
impl HelloWorldContract {

    pub fn hello(
        &mut self,
        env: Env,
        to: Symbol,
    ) -> Vec<Symbol> {
        return vec![&env, Symbol::new(env, "Hello"), to];
    }
}

impl HelloWorldContract {


}