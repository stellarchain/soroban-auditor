#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct AuthContract;

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
pub enum DataKey { Counter(soroban_sdk::Address), }

#[contractimpl]
impl AuthContract {
    pub fn increment(&mut self, env: Env, user: soroban_sdk::Address, value: u32) -> u32 {
        user.require_auth();
        let key = DataKey::Counter(user.clone());
        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or_default();
        count += value;
        env.storage().persistent().set(&key, &count);
        count
    }
}

#[allow(dead_code)]
impl AuthContract {
    fn func6(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(16);
        self.global0 = var1;
        var2 = 0 /* False */;
        var3 = -7;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var3 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var4 = 1;
                    'label3: loop {
                        'label4: loop {
                            let var7 = self.memory.load8(var3.wrapping_add(1048583) as usize) as i32;
                            var5 = var7;
                            if (var5 == 95) as i32 != 0 {
                                break 'label4;
                            }
                            'label5: loop {
                                if (((var5.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                    break 'label5;
                                }
                                'label6: loop {
                                    if (((var5.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if ((var5.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    var4 = var5.wrapping_add(-59);
                                    break 'label4;
                                    break;
                                }
                                var4 = var5.wrapping_add(-53);
                                break 'label4;
                                break;
                            }
                            var4 = var5.wrapping_add(-46);
                            break;
                        }
                        var2 = var2.wrapping_shl(0 as u32) | var4 as u32 as i64 & 255;
                        var3 = var3.wrapping_add(1);
                        continue 'label2;
                        break;
                    }
                    break;
                }
                let mut slot_var1_0_i64 = ((var5 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                let var8 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                var2 = var8;
                break 'label0;
                break;
            }
            var2 = var2.wrapping_shl(0 as u32) | 0 /* Symbol() */;
            let mut slot_var1_4_i64 = var2 as i64;
            break;
        }
        let mut slot_var1_8_i64 = arg0 as i64;
        slot_var1_0_i64 = var2 as i64;
        let var9 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2 = var9;
        self.global0 = var1.wrapping_add(16);
        var2
    }
    fn func7(&mut self, env: &Env, mut user: i64, mut value: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(user)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (value & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                let var5 = { address_from_i64(env, user).require_auth(); 0 }
                var5;
                var2 = 0;
                'label2: loop {
                    let var6 = self.func6(env, user);
                    var3 = var6;
                    let var7 = match 1 /* True */ { 0 => { if env.storage().persistent().has(&val_from_i64(var3)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var3)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var3)) { 1 } else { 0 } } }
                    if (var7 != 1 /* True */) as i32 != 0 {
                        break 'label2;
                    }
                    let var8 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                    var3 = var8;
                    if (var3 & 255 != 0) as i32 != 0 {
                        break 'label1;
                    }
                    var2 = (var3 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    break;
                }
                var4 = var2.wrapping_add((value as u64).wrapping_shr(32 as u32) as i64 as i32);
                if ((var4 as u32) < var2 as u32) as i32 != 0 {
                    break 'label0;
                }
                let var9 = self.func6(env, user);
                user = (var4 as u32 as i64).wrapping_shl(32 as u32) | 0;
                let var10 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var9), &val_from_i64(user)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var9), &val_from_i64(user)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var9), &val_from_i64(user)); 0 } }
                var10;
                return user;
                break;
            }
            unreachable!();
            break;
        }
        self.func8(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func8(&mut self, env: &Env) {
        self.func9(env);
        unreachable!();
    }
    fn func9(&mut self, env: &Env) {
        unreachable!();
    }
}
