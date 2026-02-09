#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String};

#[contract]
pub struct LoggingContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[contractimpl]
impl LoggingContract {
    pub fn hello(&mut self, env: Env, value: soroban_sdk::Symbol) {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = value as i32 & 255;
            if (var1 == 14) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 74) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        0 /* Void */
    }
}

#[allow(dead_code)]
impl LoggingContract {
    fn func0(&mut self, env: &Env, mut value: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = value as i32 & 255;
            if (var1 == 14) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 74) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        0 /* Void */
    }
}
