#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, vec, symbol_short, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct EventsContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contractevent(topics = ["COUNTER", "increment",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IncrementEvent { pub count: u32, }

#[contractimpl]
impl EventsContract {
    pub fn increment(&mut self, env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
}

#[allow(dead_code)]
impl EventsContract {
    fn func5(&mut self, env: &Env) -> i64 {
        COUNTER
    }
    fn func6(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let var1 = self.memory.load64(arg0 as usize) as i64;
        var1
    }
    fn func7(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(32);
        self.global0 = var0;
        var1 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var5 = self.func5(env);
                    var2 = var5;
                    let var6 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(var2)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var2)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var2)) { 1 } else { 0 } } }
                    if (var6 != 1 /* True */) as i32 != 0 {
                        break 'label2;
                    }
                    let var7 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) } }
                    var3 = var7;
                    if (var3 & 255 != 0) as i32 != 0 {
                        break 'label1;
                    }
                    var1 = (var3 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    break;
                }
                var1 = var1.wrapping_add(1);
                if (var1 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = (var1 as u32 as i64).wrapping_shl(32 as u32) | 0;
                let var8 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(var3)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(var3)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(var3)); 0 } }
                var8;
                let var9 = self.func6(env, 1048576);
                var2 = var9;
                let var10 = self.func6(env, 1048584);
                let mut slot_var0_8_i64 = var10 as i64;
                let mut slot_var0_0_i64 = var2 as i64;
                var1 = 0;
                'label3: loop {
                    'label4: loop {
                        if (var1 != 16) as i32 != 0 {
                            break 'label4;
                        }
                        var1 = 0;
                        'label5: loop {
                            'label6: loop {
                                if (var1 == 16) as i32 != 0 {
                                    break 'label5;
                                }
                                let var11 = self.memory.load64(var0.wrapping_add(var1) as usize) as i64;
                                self.memory.store64(var0.wrapping_add(16).wrapping_add(var1) as usize, var11 as u64);
                                var1 = var1.wrapping_add(8);
                                continue 'label6;
                                break;
                            }
                            break;
                        }
                        let var12 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
                        let var13 = { env.events().publish(val_from_i64(var12), val_from_i64(var3)); 0 }
                        var13;
                        self.global0 = var0.wrapping_add(32);
                        return var3;
                        break;
                    }
                    self.memory.store64(var0.wrapping_add(16).wrapping_add(var1) as usize, 0 /* Void */ as u64);
                    var1 = var1.wrapping_add(8);
                    continue 'label3;
                    break;
                }
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
