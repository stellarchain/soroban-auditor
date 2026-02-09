#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Val, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct IncrementWithPauseContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error { Paused = 1, }

#[contractimpl]
impl IncrementWithPauseContract {
    pub fn ___constructor(&mut self, env: Env, pause: soroban_sdk::Address) {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(pause)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func7(env, PAUSE, pause);
        0 /* Void */
    }
    pub fn increment(&mut self, env: Env) -> Result<u32, soroban_sdk::Error> {
        let mut count: Result<u32, soroban_sdk::Error> = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
}

#[allow(dead_code)]
impl IncrementWithPauseContract {
    fn func6(&mut self, env: &Env, mut pause: i64) -> i64 {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(pause)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func7(env, PAUSE, pause);
        0 /* Void */
    }
    fn func7(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn func8(&mut self, env: &Env) -> i64 {
        let mut var0: i64 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var3 = self.func9(env, PAUSE);
                    if (var3 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    let var4 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(PAUSE)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(PAUSE)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(PAUSE)).unwrap_or(val_from_i64(0))) } };
                    var0 = var4;
                    if (Address::try_from_val(env, &val_from_i64(var0)).is_ok()) as i32 != 0 {
                        break 'label1;
                    }
                    break 'label0;
                    break;
                }
                self.func11(env);
                unreachable!();
                break;
            }
            var1 = 4294967299 /* Error(Contract, #1) */;
            'label3: loop {
                'label4: loop {
                    'label5: loop {
                        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                        let var7 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(var0)), &Symbol::from_val(env, &val_from_i64(paused)), Vec::<Val>::from_val(env, &val_from_i64(var6))))
                        match var7 as i32 & 255 {
                            0 => break 'label5,
                            1 => break 'label4,
                            _ => break 'label3,
                        }
                        break;
                    }
                    var2 = 0;
                    'label6: loop {
                        let var8 = self.func9(env, COUNTER);
                        if (var8 == 0) as i32 != 0 {
                            break 'label6;
                        }
                        let var9 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) } };
                        var1 = var9;
                        if (var1 & 255 != 0) as i32 != 0 {
                            break 'label0;
                        }
                        var2 = (var1 as u64).wrapping_shr(32 as u32) as i64 as i32;
                        break;
                    }
                    var2 = var2.wrapping_add(1);
                    if (var2 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    var1 = (var2 as u32 as i64).wrapping_shl(32 as u32) | 0;
                    self.func7(env, COUNTER, var1);
                    let var11 = { env.storage().instance().extend_ttl(50 as u32, 100 as u32); 0 }
                    var11;
                    break;
                }
                return var1;
                break;
            }
            self.func12(env);
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func9(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func10(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let var1 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) } }
        var1
    }
    fn func11(&mut self, env: &Env) {
        self.func12(env);
        unreachable!();
    }
    fn func12(&mut self, env: &Env) {
        self.func13(env);
        unreachable!();
    }
    fn func13(&mut self, env: &Env) {
        unreachable!();
    }
}
