#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct IncrementWithFuzzContract;

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
impl IncrementWithFuzzContract {
    pub fn increment(&mut self, env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
}

#[allow(dead_code)]
impl IncrementWithFuzzContract {
    fn func4(&mut self, env: &Env) -> i64 {
        COUNTER
    }
    fn func5(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i64 = 0;
        var0 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var3 = self.func4(env);
                    var1 = var3;
                    let var4 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(var1)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var1)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var1)) { 1 } else { 0 } } }
                    if (var4 != 1 /* True */) as i32 != 0 {
                        break 'label2;
                    }
                    let var5 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) } }
                    var2 = var5;
                    if (var2 & 255 != 0) as i32 != 0 {
                        break 'label1;
                    }
                    var0 = (var2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    break;
                }
                var0 = var0.wrapping_add(1);
                if (var0 == 0) as i32 != 0 {
                    break 'label0;
                }
                var2 = (var0 as u32 as i64).wrapping_shl(32 as u32) | 0;
                let var6 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64(var2)); 0 } }
                var6;
                let var7 = { env.storage().instance().extend_ttl(50 as u32, 100 as u32); 0 }
                var7;
                return var2;
                break;
            }
            unreachable!();
            break;
        }
        self.func6(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func6(&mut self, env: &Env) {
        self.func7(env);
        unreachable!();
    }
    fn func7(&mut self, env: &Env) {
        unreachable!();
    }
}
