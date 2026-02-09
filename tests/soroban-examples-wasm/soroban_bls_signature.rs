#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, IntoVal, BytesN, auth::Context, Val, Address, FromVal, Map, Bytes, String, Symbol};

#[contract]
pub struct BlsSignature;

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
pub enum DataKey { Owners, Counter, Dst, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AccError { InvalidSignature = 1, }

#[contractimpl]
impl BlsSignature {
    pub fn ___constructor(&mut self, mut env: Env, agg_pk: soroban_sdk::BytesN<96>) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        self.func17(env, var1, agg_pk);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        agg_pk = slot_var1_8_i64;
        let var4 = self.func12(env, 0);
        let var5 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 } }
        var5;
        let var6 = val_to_i64(Bytes::from_slice(env, &[66, 76, 83, 83, 73, 71, 45, 86, 48, 49, 45, 67, 83, 48, 49, 45, 119, 105, 116, 104, 45, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 95, 88, 77, 68, 58, 83, 72, 65, 45, 50, 53, 54, 95, 83, 83, 87, 85, 95, 82, 79, 95]).into_val(env));
        agg_pk = var6;
        let var7 = self.func12(env, 2);
        let var8 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 } }
        var8;
        self.func11(env, 0);
        self.global0 = var1.wrapping_add(16);
        0 /* Void */
    }
    pub fn increment(&mut self, env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, agg_sig: soroban_sdk::BytesN<192>, auth_contexts: soroban_sdk::Vec<Context>) -> Result<(), AccError> {
        Ok(())
    }
}

#[allow(dead_code)]
impl BlsSignature {
    fn func11(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func12(env, 1);
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0)); 0 } }
        var2;
    }
    fn func12(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match arg0 & 255 {
                                    0 => break 'label5,
                                    1 => break 'label4,
                                    2 => break 'label3,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            self.func13(env, var1, 1048576, 6);
                            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                            if slot_var1_0_i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                            self.func14(env, var1, slot_var1_8_i64);
                            break 'label2;
                            break;
                        }
                        self.func13(env, var1, 1048582, 7);
                        if slot_var1_0_i32 != 0 {
                            break 'label1;
                        }
                        self.func14(env, var1, slot_var1_8_i64);
                        break 'label2;
                        break;
                    }
                    self.func13(env, var1, 1048589, 3);
                    if slot_var1_0_i32 != 0 {
                        break 'label1;
                    }
                    self.func14(env, var1, slot_var1_8_i64);
                    break;
                }
                var2 = slot_var1_8_i64;
                if (slot_var1_0_i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(16);
        var2
    }
    fn func13(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var3 = var9.wrapping_sub(16);
        self.global0 = var3;
        var4 = 0 /* False */;
        var5 = arg2;
        var6 = arg1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var5 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var7 = 1;
                    'label3: loop {
                        'label4: loop {
                            let var10 = self.memory.load8(var6 as usize) as i32;
                            var8 = var10;
                            if (var8 == 95) as i32 != 0 {
                                break 'label4;
                            }
                            'label5: loop {
                                if (((var8.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                    break 'label5;
                                }
                                'label6: loop {
                                    if (((var8.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if ((var8.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    var7 = var8.wrapping_add(-59);
                                    break 'label4;
                                    break;
                                }
                                var7 = var8.wrapping_add(-53);
                                break 'label4;
                                break;
                            }
                            var7 = var8.wrapping_add(-46);
                            break;
                        }
                        var4 = var4.wrapping_shl(0 as u32) | var7 as u32 as i64 & 255;
                        var5 = var5.wrapping_add(-1);
                        var6 = var6.wrapping_add(1);
                        continue 'label2;
                        break;
                    }
                    break;
                }
                let mut slot_var3_0_i64 = ((var8 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                let var11 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                var4 = var11;
                break 'label0;
                break;
            }
            var4 = var4.wrapping_shl(0 as u32) | 0 /* Symbol() */;
            let mut slot_var3_4_i64 = var4 as i64;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, var4 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        let mut slot_var2_8_i64 = arg1 as i64;
        let var4 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        arg1 = var4;
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
        self.global0 = var2.wrapping_add(16);
    }
    fn func15(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func16(&mut self, env: &Env, mut agg_pk: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        self.func17(env, var1, agg_pk);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        agg_pk = slot_var1_8_i64;
        let var4 = self.func12(env, 0);
        let var5 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var4), &val_from_i64(agg_pk)); 0 } }
        var5;
        let var6 = val_to_i64(Bytes::from_slice(env, &[66, 76, 83, 83, 73, 71, 45, 86, 48, 49, 45, 67, 83, 48, 49, 45, 119, 105, 116, 104, 45, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 95, 88, 77, 68, 58, 83, 72, 65, 45, 50, 53, 54, 95, 83, 83, 87, 85, 95, 82, 79, 95]).into_val(env));
        agg_pk = var6;
        let var7 = self.func12(env, 2);
        let var8 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(agg_pk)); 0 } }
        var8;
        self.func11(env, 0);
        self.global0 = var1.wrapping_add(16);
        0 /* Void */
    }
    fn func17(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var3 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var3 & 18446744069414584320 != 412316860416) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize + 8, arg1 as u64);
            var2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var2 as u64);
    }
    fn func18(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func19(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = val_to_i64(env.current_contract_address().into_val(env))
        let var3 = { address_from_i64(env, var2).require_auth(); 0 }
        var3;
        var0 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var4 = self.func12(env, 1);
                    var1 = var4;
                    let var5 = self.func20(env, var1, 0 /* Void */);
                    if (var5 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    let var6 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) } }
                    var1 = var6;
                    if (var1 & 255 != 0) as i32 != 0 {
                        break 'label1;
                    }
                    var0 = (var1 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    break;
                }
                var0 = var0.wrapping_add(1);
                if (var0 == 0) as i32 != 0 {
                    break 'label0;
                }
                self.func11(env, var0);
                return (var0 as u32 as i64).wrapping_shl(32 as u32) | 0;
                break;
            }
            unreachable!();
            break;
        }
        self.func21(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func20(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var2 == 1 /* True */) as i32
    }
    fn func21(&mut self, env: &Env) {
        self.func24(env);
        unreachable!();
    }
    fn func22(&mut self, env: &Env, mut signature_payload: i64, mut agg_sig: i64, mut auth_contexts: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var3 = var7.wrapping_sub(32);
        self.global0 = var3;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(signature_payload)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var8 = Bytes::from_val(env, &val_from_i64(signature_payload)).len() as i64
            if (var8 & 18446744069414584320 != 137438953472) as i32 != 0 {
                break 'label0;
            }
            if (!(Bytes::try_from_val(env, &val_from_i64(agg_sig)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var9 = Bytes::from_val(env, &val_from_i64(agg_sig)).len() as i64
            if (var9 & 18446744069414584320 != 824633720832) as i32 != 0 {
                break 'label0;
            }
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(auth_contexts)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                let var10 = self.func12(env, 0);
                auth_contexts = var10;
                let var11 = self.func20(env, auth_contexts, 1 /* True */);
                if (var11 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var12 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) } }
                self.func17(env, var3.wrapping_add(16), var12);
                let mut slot_var3_16_i32 = self.memory.load32(var3 as usize + 16) as i32;
                if (slot_var3_16_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                var4 = slot_var3_24_i64;
                let var14 = self.func12(env, 2);
                auth_contexts = var14;
                let var15 = self.func20(env, auth_contexts, 0 /* Void */);
                if (var15 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var16 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(auth_contexts)).unwrap_or(val_from_i64(0))) } }
                auth_contexts = var16;
                if (!(Bytes::try_from_val(env, &val_from_i64(auth_contexts)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                let var17 = val_to_i64(Bytes::from_slice(env, &[23, 241, 211, 167, 49, 151, 215, 148, 38, 149, 99, 140, 79, 169, 172, 15, 195, 104, 140, 79, 151, 116, 185, 5, 161, 78, 58, 63, 23, 27, 172, 88, 108, 85, 232, 63, 249, 122, 26, 239, 251, 58, 240, 10, 219, 34, 198, 187, 17, 77, 29, 104, 85, 213, 69, 168, 170, 125, 118, 200, 207, 46, 33, 242, 103, 129, 106, 239, 29, 181, 7, 201, 102, 85, 185, 213, 202, 172, 66, 54, 78, 111, 56, 186, 14, 203, 117, 27, 173, 84, 220, 214, 185, 57, 194, 202]).into_val(env));
                var5 = var17;
                let var18 = 0 /* TODO: bls12_381_hash_to_g2 */
                signature_payload = var18;
                let mut slot_var3_8_i64 = var5 as i64;
                let mut slot_var3_0_i64 = var4 as i64;
                var6 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var6 != 16) as i32 != 0 {
                            break 'label3;
                        }
                        var6 = 0;
                        'label4: loop {
                            'label5: loop {
                                if (var6 == 16) as i32 != 0 {
                                    break 'label4;
                                }
                                let var19 = self.memory.load64(var3.wrapping_add(var6) as usize) as i64;
                                self.memory.store64(var3.wrapping_add(16).wrapping_add(var6) as usize, var19 as u64);
                                var6 = var6.wrapping_add(8);
                                continue 'label5;
                                break;
                            }
                            break;
                        }
                        let var20 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        auth_contexts = var20;
                        slot_var3_8_i64 = agg_sig as i64;
                        slot_var3_0_i64 = signature_payload as i64;
                        var6 = 0;
                        'label6: loop {
                            'label7: loop {
                                if (var6 != 16) as i32 != 0 {
                                    break 'label7;
                                }
                                var6 = 0;
                                'label8: loop {
                                    'label9: loop {
                                        if (var6 == 16) as i32 != 0 {
                                            break 'label8;
                                        }
                                        let var21 = self.memory.load64(var3.wrapping_add(var6) as usize) as i64;
                                        self.memory.store64(var3.wrapping_add(16).wrapping_add(var6) as usize, var21 as u64);
                                        var6 = var6.wrapping_add(8);
                                        continue 'label9;
                                        break;
                                    }
                                    break;
                                }
                                let var22 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                let var23 = 0 /* TODO: bls12_381_multi_pairing_check */
                                signature_payload = var23;
                                self.global0 = var3.wrapping_add(32);
                                return { let a = 0 /* Void */; let b = 4294967299 /* Error(Contract, #1) */; if (signature_payload == 1 /* True */) as i32 != 0 { a } else { b } };
                                break;
                            }
                            self.memory.store64(var3.wrapping_add(16).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label6;
                            break;
                        }
                        break;
                    }
                    self.memory.store64(var3.wrapping_add(16).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                    var6 = var6.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func23(env);
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func23(&mut self, env: &Env) {
        self.func21(env);
        unreachable!();
    }
    fn func24(&mut self, env: &Env) {
        unreachable!();
    }
}
