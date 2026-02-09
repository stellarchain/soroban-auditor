#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct TtlExample;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { MyKey, }

#[contractimpl]
impl TtlExample {
    pub fn setup(&mut self, env: Env) {
        // TODO: set storage value
    }
    pub fn extend_persistent(&mut self, env: Env) {
        self.func5(env, 1 /* True */, 1000, 5000);
        0 /* Void */
    }
    pub fn extend_instance(&mut self, env: Env) {
        let var0 = { env.storage().instance().extend_ttl(2000 as u32, 10000 as u32); 0 }
        var0;
        0 /* Void */
    }
    pub fn extend_temporary(&mut self, env: Env) {
        self.func5(env, 0 /* False */, 3000, 7000);
        0 /* Void */
    }
}

#[allow(dead_code)]
impl TtlExample {
    fn func5(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) {
        let var3 = self.func6(env);
        let var4 = match arg0 { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(var3), (arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(var3), (arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, _ => { env.storage().instance().extend_ttl((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 } }
        var4;
    }
    fn func6(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var0 = var5.wrapping_sub(16);
        self.global0 = var0;
        var1 = 0 /* False */;
        var2 = -5;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var2 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var3 = 1;
                    'label3: loop {
                        'label4: loop {
                            let var6 = self.memory.load8(var2.wrapping_add(1048581) as usize) as i32;
                            var4 = var6;
                            if (var4 == 95) as i32 != 0 {
                                break 'label4;
                            }
                            'label5: loop {
                                if (((var4.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                    break 'label5;
                                }
                                'label6: loop {
                                    if (((var4.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if ((var4.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    var3 = var4.wrapping_add(-59);
                                    break 'label4;
                                    break;
                                }
                                var3 = var4.wrapping_add(-53);
                                break 'label4;
                                break;
                            }
                            var3 = var4.wrapping_add(-46);
                            break;
                        }
                        var1 = var1.wrapping_shl(0 as u32) | var3 as u32 as i64 & 255;
                        var2 = var2.wrapping_add(1);
                        continue 'label2;
                        break;
                    }
                    break;
                }
                let mut slot_var0_0_i64 = ((var4 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                let var7 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                var1 = var7;
                break 'label0;
                break;
            }
            var1 = var1.wrapping_shl(0 as u32) | 0 /* Symbol() */;
            let mut slot_var0_4_i64 = var1 as i64;
            break;
        }
        slot_var0_0_i64 = var1 as i64;
        let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var1 = var8;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func7(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let var2 = self.func6(env);
        let var3 = match arg1 { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 } }
        var3;
    }
    fn func8(&mut self, env: &Env) -> i64 {
        self.func7(env, 0, 1 /* True */);
        self.func7(env, 1, 0 /* Void */);
        self.func7(env, 2, 0 /* False */);
        0 /* Void */
    }
    fn func9(&mut self, env: &Env) -> i64 {
        self.func5(env, 1 /* True */, 1000, 5000);
        0 /* Void */
    }
    fn func10(&mut self, env: &Env) -> i64 {
        let var0 = { env.storage().instance().extend_ttl(2000 as u32, 10000 as u32); 0 }
        var0;
        0 /* Void */
    }
    fn func11(&mut self, env: &Env) -> i64 {
        self.func5(env, 0 /* False */, 3000, 7000);
        0 /* Void */
    }
}
