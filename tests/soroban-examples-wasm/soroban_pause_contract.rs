#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct PauseContract;

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
impl PauseContract {
    pub fn paused(&mut self, env: Env) -> bool {
        let mut var0: i64 = 0;
        let mut var1: i64 = 0;
        var0 = 1 /* True */;
        'label0: loop {
            'label1: loop {
                let var2 = self.func3(env);
                var1 = var2;
                let var3 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(var1)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var1)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var1)) { 1 } else { 0 } } }
                if (var3 != 1 /* True */) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    let var4 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) } }
                    match var4 as i32 & 255 {
                        0 => break 'label1,
                        1 => break 'label0,
                        _ => break 'label2,
                    }
                    break;
                }
                unreachable!();
                break;
            }
            var0 = 0 /* False */;
            break;
        }
        var0
    }
    pub fn set(&mut self, env: Env, paused: bool) {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = paused as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var2 = self.func3(env);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 } }
        var3;
        0 /* Void */
    }
}

#[allow(dead_code)]
impl PauseContract {
    fn func3(&mut self, env: &Env) -> i64 {
        PAUSED
    }
    fn func4(&mut self, env: &Env) -> i64 {
        let mut var0: i64 = 0;
        let mut var1: i64 = 0;
        var0 = 1 /* True */;
        'label0: loop {
            'label1: loop {
                let var2 = self.func3(env);
                var1 = var2;
                let var3 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(var1)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var1)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var1)) { 1 } else { 0 } } }
                if (var3 != 1 /* True */) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    let var4 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) } }
                    match var4 as i32 & 255 {
                        0 => break 'label1,
                        1 => break 'label0,
                        _ => break 'label2,
                    }
                    break;
                }
                unreachable!();
                break;
            }
            var0 = 0 /* False */;
            break;
        }
        var0
    }
    fn func5(&mut self, env: &Env, mut paused: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = paused as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var2 = self.func3(env);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(var1 as u32 as i64)); 0 } }
        var3;
        0 /* Void */
    }
}
