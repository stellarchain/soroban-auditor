#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct WorkspaceContractA;

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
impl WorkspaceContractA {
    pub fn add(&mut self, mut env: Env, mut x: u32, y: u32) -> u32 {
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (x & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (y & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                x = (x as u64).wrapping_shr(32 as u32) as i64;
                var2 = x as i32;
                y = (y as u64).wrapping_shr(32 as u32) as i64;
                if ((var2.wrapping_add(y as i32) as u32) < var2 as u32) as i32 != 0 {
                    break 'label0;
                }
                return y.wrapping_add(x).wrapping_shl(32 as u32) | 0;
                break;
            }
            unreachable!();
            break;
        }
        self.func1(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
}

#[allow(dead_code)]
impl WorkspaceContractA {
    fn func0(&mut self, env: &Env, mut x: i64, mut y: i64) -> i64 {
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (x & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (y & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                x = (x as u64).wrapping_shr(32 as u32) as i64;
                var2 = x as i32;
                y = (y as u64).wrapping_shr(32 as u32) as i64;
                if ((var2.wrapping_add(y as i32) as u32) < var2 as u32) as i32 != 0 {
                    break 'label0;
                }
                return y.wrapping_add(x).wrapping_shl(32 as u32) | 0;
                break;
            }
            unreachable!();
            break;
        }
        self.func1(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func1(&mut self, env: &Env) {
        self.func2(env);
        unreachable!();
    }
    fn func2(&mut self, env: &Env) {
        unreachable!();
    }
}
