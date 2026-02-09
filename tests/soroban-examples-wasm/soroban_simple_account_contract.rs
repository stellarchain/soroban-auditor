#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, BytesN, auth::Context, crypto::Hash, Val, Address, FromVal, IntoVal, Map, Bytes, String, Symbol};

#[contract]
pub struct SimpleAccountContract;

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
pub enum DataKey { Owner, }

#[contractimpl]
impl SimpleAccountContract {
    pub fn ___constructor(&mut self, env: Env, public_key: soroban_sdk::BytesN<32>) {
        env.storage().instance().set(&DataKey::Owner, &public_key);
    }
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, signature: soroban_sdk::BytesN<64>, auth_context: soroban_sdk::Vec<Context>) {
        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::Owner).unwrap();
        env.crypto().ed25519_verify(&public_key, &signature_payload.into(), &signature);
    }
}

#[allow(dead_code)]
impl SimpleAccountContract {
    fn func7(&mut self, env: &Env) -> i64 {
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
    fn func8(&mut self, env: &Env, mut public_key: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        self.func9(env, var1, public_key);
        'label0: loop {
            'label1: loop {
                let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                if (slot_var1_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                public_key = slot_var1_8_i64;
                let var4 = self.func7(env);
                let var5 = self.func10(env, var4);
                if var5 != 0 {
                    break 'label0;
                }
                let var6 = self.func7(env);
                let var7 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var6), &val_from_i64(public_key)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var6), &val_from_i64(public_key)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var6), &val_from_i64(public_key)); 0 } }
                var7;
                self.global0 = var1.wrapping_add(16);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func11(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func9(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var3 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var3 & 18446744069414584320 != 137438953472) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize + 8, arg1 as u64);
            var2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var2 as u64);
    }
    fn func10(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func11(&mut self, env: &Env) {
        unreachable!();
    }
    fn func12(&mut self, env: &Env, mut signature_payload: i64, mut signature: i64, mut auth_context: i64) -> i64 {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        self.func9(env, var3, signature_payload);
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
                    if (slot_var3_0_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    if (!(Bytes::try_from_val(env, &val_from_i64(signature)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                    signature_payload = slot_var3_8_i64;
                    let var6 = Bytes::from_val(env, &val_from_i64(signature)).len() as i64
                    if (var6 & 18446744069414584320 != 274877906944) as i32 != 0 {
                        break 'label2;
                    }
                    if (!(Vec::<Val>::try_from_val(env, &val_from_i64(auth_context)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let var7 = self.func7(env);
                    auth_context = var7;
                    let var8 = self.func10(env, auth_context);
                    if (var8 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    let var9 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(auth_context)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(auth_context)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(auth_context)).unwrap_or(val_from_i64(0))) } }
                    self.func9(env, var3, var9);
                    if (slot_var3_0_i32 != 1) as i32 != 0 {
                        break 'label0;
                    }
                    break;
                }
                unreachable!();
                break;
            }
            self.func13(env);
            unreachable!();
            break;
        }
        let var12 = { env.crypto().ed25519_verify(&BytesN::<32>::from_val(env, &val_from_i64(slot_var3_8_i64)), &BytesN::<32>::from_val(env, &val_from_i64(signature_payload)).into(), &BytesN::<64>::from_val(env, &val_from_i64(signature))); 0 }
        var12;
        self.global0 = var3.wrapping_add(16);
        0 /* Void */
    }
    fn func13(&mut self, env: &Env) {
        self.func14(env);
        unreachable!();
    }
    fn func14(&mut self, env: &Env) {
        self.func11(env);
        unreachable!();
    }
}
