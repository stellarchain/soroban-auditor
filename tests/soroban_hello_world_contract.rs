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

        env: Env,
        to: Symbol,
    ) -> Vec<Symbol> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = var3.wrapping_sub(32);
        b = to as i32 & 255;
        b = 0;
        while b != 16 {
            mstore64!(a.wrapping_add(16).wrapping_add(b) as usize, 0 /* Void */ as u64);
            b = b.wrapping_add(8);
        }
        b = 0;
        while b != 16 {
            let c = mload64!(a.wrapping_add(b) as usize) as i64;
            mstore64!(a.wrapping_add(16).wrapping_add(b) as usize, c as u64);
            b = b.wrapping_add(8);
        }
        vec![&env, Symbol::new(env, "Hello"), to];
    }
}

impl HelloWorldContract {


}