#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, String, Symbol, Bytes, BytesN, TryFromVal, U256, Val, FromVal, Map};

#[contract]
pub struct PrivacyPools;

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
pub enum Error { NullifierUsed = 1, InsufficientBalance = 2, CoinOwnershipProofFailed = 3, OnlyAdmin = 4, TreeAtCapacity = 5, AssociationRootMismatch = 6, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Groth16Error { MalformedVerifyingKey = 0, }

#[contractimpl]
impl PrivacyPools {
    pub fn ___constructor(&mut self, mut env: Env, mut vk_bytes: soroban_sdk::Bytes, mut token_address: soroban_sdk::Address, mut admin: soroban_sdk::Address, groth16_verifier: soroban_sdk::Address) {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(48);
        self.global0 = var4;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(vk_bytes)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(token_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(admin)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(groth16_verifier)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func42(env, admin, admin);
            self.func43(env, vk, vk_bytes);
            self.func42(env, token, token_address);
            self.func42(env, g16v, groth16_verifier);
            let var11 = val_to_i64(Vec::<Val>::new(env).into_val(env))
            vk_bytes = var11;
            let var12 = self.func49(env, 1048859);
            token_address = var12;
            let var13 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
            admin = var13;
            let var14 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
            groth16_verifier = var14;
            let mut slot_var4_40_i64 = 4503599627370516 as i64;
            let mut slot_var4_8_i64 = vk_bytes as i64;
            let mut slot_var4_32_i64 = groth16_verifier as i64;
            let mut slot_var4_24_i64 = admin as i64;
            let mut slot_var4_16_i64 = token_address as i64;
            self.func50(env, var4.wrapping_add(8));
            vk_bytes = slot_var4_16_i64;
            var5 = slot_var4_40_i64;
            self.func46(env, leaves, slot_var4_8_i64);
            self.func44(env, var5);
            self.func45(env, root, vk_bytes);
            self.global0 = var4.wrapping_add(48);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn deposit(&mut self, env: Env, from: soroban_sdk::Address, commitment: soroban_sdk::BytesN<32>) {
        from.require_auth_for_args((commitment).into_val(&env));
    }
    pub fn withdraw(&mut self, env: Env, to: soroban_sdk::Address, proof_bytes: soroban_sdk::Bytes, pub_signals_bytes: soroban_sdk::Bytes) {
        to.require_auth_for_args((proof_bytes, pub_signals_bytes).into_val(&env));
    }
    pub fn get_merkle_root(&mut self, env: Env) -> soroban_sdk::BytesN<32> {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func39(env, var0, root);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = self.func49(env, 1048859);
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    pub fn get_merkle_depth(&mut self, env: Env) -> u32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func35(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        var1 = slot_var0_8_i32;
        let var5 = self.memory.load32(var0 as usize + 12) as i64;
        var2 = var5;
        self.global0 = var0.wrapping_add(16);
        { let a = var2.wrapping_shl(32 as u32) | 0; let b = 0; if var1 & 1 != 0 { a } else { b } }
    }
    pub fn get_commitment_count(&mut self, env: Env) -> u32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, leaves);
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var1 = slot_var0_0_i32;
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        let var6 = Vec::<Val>::from_val(env, &val_from_i64({ let a = slot_var0_8_i64; let b = var5; if var1 != 0 { a } else { b } })).len() as i64
        var2 = var6;
        self.global0 = var0.wrapping_add(16);
        var2 & 18446744069414584320 | 0
    }
    pub fn get_commitments(&mut self, env: Env) -> soroban_sdk::Vec<soroban_sdk::BytesN<32>> {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, leaves);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    pub fn get_nullifiers(&mut self, env: Env) -> soroban_sdk::Vec<soroban_sdk::BytesN<32>> {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, null);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    pub fn get_balance(&mut self, env: Env) -> i128 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func38(env, var0, token);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func58(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        let var5 = val_to_i64(env.current_contract_address().into_val(env))
        self.func61(env, var0, slot_var0_8_i64, var5);
        let var7 = self.func32(env, slot_var0_0_i32, slot_var0_8_i64);
        var1 = var7;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    pub fn set_association_root(&mut self, env: Env, caller: soroban_sdk::Address, association_root: soroban_sdk::BytesN<32>) {
        caller.require_auth_for_args((association_root).into_val(&env));
    }
    pub fn get_association_root(&mut self, env: Env) -> soroban_sdk::BytesN<32> {
        let var0 = self.func68(env);
        var0
    }
    pub fn has_association_set(&mut self, env: Env) -> bool {
        let var0 = self.func60(env);
        var0 as u32 as i64
    }
    pub fn get_admin(&mut self, env: Env) -> soroban_sdk::Address {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func38(env, var0, admin);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func58(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        self.global0 = var0.wrapping_add(16);
        var1
    }
}

#[allow(dead_code)]
impl PrivacyPools {
    fn func31(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(48);
        self.global0 = var3;
        let var6 = self.func32(env, 1000000000, 0 /* False */);
        let mut slot_var3_16_i64 = var6 as i64;
        let mut slot_var3_8_i64 = arg2 as i64;
        let mut slot_var3_0_i64 = arg1 as i64;
        var4 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var4 != 24) as i32 != 0 {
                        break 'label2;
                    }
                    var4 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var4 == 24) as i32 != 0 {
                                break 'label3;
                            }
                            let var7 = self.memory.load64(var3.wrapping_add(var4) as usize) as i64;
                            self.memory.store64(var3.wrapping_add(24).wrapping_add(var4) as usize, var7 as u64);
                            var4 = var4.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    let var9 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(transfer)), Vec::<Val>::from_val(env, &val_from_i64(var8))))
                    if (var9 & 255 != 0 /* Void */) as i32 != 0 {
                        break 'label0;
                    }
                    self.global0 = var3.wrapping_add(48);
                    return;
                    break;
                }
                self.memory.store64(var3.wrapping_add(24).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                var4 = var4.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        self.func34(env);
        unreachable!();
    }
    fn func32(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        'label0: loop {
            if (arg0.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                break 'label0;
            }
            if (arg0 ^ arg0 | arg1 ^ arg0.wrapping_shr(63 as u32) != 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            return arg0.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var2 = val_to_i64(Val::from_i128(((arg1 as i128) << 64) | (arg0 as u64 as i128)))
        var2
    }
    fn func33(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func34(&mut self, env: &Env) {
        self.func72(env);
        unreachable!();
    }
    fn func35(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var4 = self.func36(env, depth);
                    if var4 != 0 {
                        break 'label2;
                    }
                    var1 = 0;
                    break 'label1;
                    break;
                }
                let var5 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(depth)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(depth)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(depth)).unwrap_or(val_from_i64(0))) } };
                var2 = var5;
                if (var2 & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = (var2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                var1 = 1;
                break;
            }
            self.memory.store32(arg0 as usize + 4, var3 as u32);
            self.memory.store32(arg0 as usize, var1 as u32);
            return;
            break;
        }
        unreachable!();
    }
    fn func36(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func37(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let var1 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) } }
        var1
    }
    fn func38(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var3 = self.func36(env, arg1);
                if (var3 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var4 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } };
                arg1 = var4;
                if (!(Address::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(arg0 as usize + 8, arg1 as u64);
                var2 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var2 as u64);
            return;
            break;
        }
        unreachable!();
    }
    fn func39(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(16);
        self.global0 = var2;
        var3 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var5 = self.func36(env, arg1);
                if (var5 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var6 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } };
                self.func40(env, var2, var6);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                if (slot_var2_0_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                self.memory.store64(arg0 as usize + 8, slot_var2_8_i64 as u64);
                var3 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var3 as u64);
            self.global0 = var2.wrapping_add(16);
            return;
            break;
        }
        unreachable!();
    }
    fn func40(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        'label0: loop {
            if (Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize, 1 /* True */ as u64);
            return;
            break;
        }
        self.func89(env, arg0, arg1);
    }
    fn func41(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var3 = self.func36(env, arg1);
                if (var3 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var4 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } };
                arg1 = var4;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(arg0 as usize + 8, arg1 as u64);
                var2 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var2 as u64);
            return;
            break;
        }
        unreachable!();
    }
    fn func42(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func43(env, arg0, arg1);
    }
    fn func43(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn func44(&mut self, env: &Env, mut arg0: i32) {
        self.func43(env, depth, (arg0 as u32 as i64).wrapping_shl(32 as u32) | 0);
    }
    fn func45(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func43(env, arg0, arg1);
    }
    fn func46(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func43(env, arg0, arg1);
    }
    fn func47(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = { let a = val_from_i64(arg0); let b = val_from_i64(arg1); if a < b { -1 } else if a > b { 1 } else { 0 } }
        (var2 != 0 /* False */) as i32
    }
    fn func48(&mut self, env: &Env, mut vk_bytes: i64, mut token_address: i64, mut admin: i64, mut groth16_verifier: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(48);
        self.global0 = var4;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(vk_bytes)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(token_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(admin)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(groth16_verifier)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func42(env, admin, admin);
            self.func43(env, vk, vk_bytes);
            self.func42(env, token, token_address);
            self.func42(env, g16v, groth16_verifier);
            let var11 = val_to_i64(Vec::<Val>::new(env).into_val(env))
            vk_bytes = var11;
            let var12 = self.func49(env, 1048859);
            token_address = var12;
            let var13 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
            admin = var13;
            let var14 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
            groth16_verifier = var14;
            let mut slot_var4_40_i64 = 4503599627370516 as i64;
            let mut slot_var4_8_i64 = vk_bytes as i64;
            let mut slot_var4_32_i64 = groth16_verifier as i64;
            let mut slot_var4_24_i64 = admin as i64;
            let mut slot_var4_16_i64 = token_address as i64;
            self.func50(env, var4.wrapping_add(8));
            vk_bytes = slot_var4_16_i64;
            var5 = slot_var4_40_i64;
            self.func46(env, leaves, slot_var4_8_i64);
            self.func44(env, var5);
            self.func45(env, root, vk_bytes);
            self.global0 = var4.wrapping_add(48);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func49(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let var1 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
        var1
    }
    fn func50(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var1 = var7.wrapping_sub(32);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                let var8 = self.memory.load32(arg0 as usize + 32) as i32;
                var2 = var8;
                if var2 != 0 {
                    break 'label1;
                }
                let var9 = self.func49(env, 1048859);
                self.memory.store64(arg0 as usize + 8, var9 as u64);
                break 'label0;
                break;
            }
            self.func53(env, var1);
            let var11 = self.memory.load64(arg0 as usize + 16) as i64;
            var3 = var11;
            var4 = 0;
            var5 = 0;
            var6 = 0;
            'label2: loop {
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            if var6 & 1 != 0 {
                                break 'label5;
                            }
                            if (var4 as u32 > var2 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            if var4 != 0 {
                                break 'label4;
                            }
                            var5 = 0;
                            break 'label3;
                            break;
                        }
                        let var12 = self.func56(env, var5);
                        self.memory.store64(arg0 as usize + 8, var12 as u64);
                        break 'label0;
                        break;
                    }
                    let var13 = self.func57(env, var1, var5, var5);
                    var5 = var13;
                    break;
                }
                var6 = (var4 as u32 >= var2 as u32) as i32;
                let var14 = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(var3)); m.set(val_from_i64((var4 as u32 as i64).wrapping_shl(32 as u32) | 0), val_from_i64(var5)); val_to_i64(m.into_val(env)) }
                var3 = var14;
                self.memory.store64(arg0 as usize + 16, var3 as u64);
                var4 = var4.wrapping_add(((var4 as u32) < var2 as u32) as i32);
                continue 'label2;
                break;
            }
            break;
        }
        self.global0 = var1.wrapping_add(32);
    }
    fn func51(&mut self, env: &Env, mut from: i64, mut commitment: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i32 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let var12 = self.global0;
        var2 = var12.wrapping_sub(112);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        self.func40(env, var2.wrapping_add(56), commitment);
                        let mut slot_var2_56_i32 = self.memory.load32(var2 as usize + 56) as i32;
                        if (slot_var2_56_i32 == 1) as i32 != 0 {
                            break 'label3;
                        }
                        let mut slot_var2_64_i64 = self.memory.load64(var2 as usize + 64) as i64;
                        var3 = slot_var2_64_i64;
                        let var14 = { address_from_i64(env, from).require_auth(); 0 }
                        var14;
                        self.func38(env, var2.wrapping_add(56), token);
                        'label4: loop {
                            if (slot_var2_56_i32 == 0) as i32 != 0 {
                                break 'label4;
                            }
                            let var16 = val_to_i64(env.current_contract_address().into_val(env))
                            self.func31(env, slot_var2_64_i64, from, var16);
                            self.func41(env, var2.wrapping_add(56), leaves);
                            from = slot_var2_64_i64;
                            var4 = slot_var2_56_i32;
                            let var19 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                            commitment = var19;
                            self.func35(env, var2.wrapping_add(8));
                            let mut slot_var2_12_i32 = self.memory.load32(var2 as usize + 12) as i32;
                            var5 = slot_var2_12_i32;
                            let mut slot_var2_8_i32 = self.memory.load32(var2 as usize + 8) as i32;
                            var6 = slot_var2_8_i32;
                            self.func39(env, var2.wrapping_add(56), root);
                            var7 = slot_var2_64_i64;
                            var8 = slot_var2_56_i32;
                            let var22 = self.func49(env, 1048859);
                            var9 = var22;
                            let var23 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
                            var10 = var23;
                            let var24 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
                            var11 = var24;
                            from = { let a = from; let b = commitment; if var4 != 0 { a } else { b } };
                            let mut slot_var2_56_i64 = from as i64;
                            let mut slot_var2_80_i64 = var11 as i64;
                            let mut slot_var2_72_i64 = var10 as i64;
                            slot_var2_64_i64 = ({ let a = var7; let b = var9; if var8 != 0 { a } else { b } }) as i64;
                            var4 = { let a = var5; let b = 0; if var6 & 1 != 0 { a } else { b } };
                            let mut slot_var2_88_i32 = var4 as i32;
                            let mut slot_var2_92_i32 = ({ let a = 1.wrapping_shl(var4 as u32); let b = -1; if ((var4 as u32) < 32 as u32) as i32 != 0 { a } else { b } }) as i32;
                            'label5: loop {
                                'label6: loop {
                                    let var25 = Vec::<Val>::from_val(env, &val_from_i64(from)).len() as i64
                                    if ((var25 as u64) < 4294967296 as u64) as i32 != 0 {
                                        break 'label6;
                                    }
                                    let var26 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
                                    slot_var2_72_i64 = var26 as i64;
                                    let var27 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
                                    slot_var2_80_i64 = var27 as i64;
                                    break 'label5;
                                    break;
                                }
                                self.func50(env, var2.wrapping_add(56));
                                break;
                            }
                            let var29 = self.func91(env, var2.wrapping_add(16), var2.wrapping_add(56), 40);
                            var29;
                            let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                            commitment = slot_var2_16_i64;
                            let var30 = Vec::<Val>::from_val(env, &val_from_i64(commitment)).len() as i64
                            from = var30;
                            let mut slot_var2_52_i32 = self.memory.load32(var2 as usize + 52) as i32;
                            if (slot_var2_52_i32 as u32 <= (from as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                break 'label2;
                            }
                            let var31 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(commitment)); v.push_back(val_from_i64(var3)); val_to_i64(v.into_val(env)) }
                            from = var31;
                            slot_var2_16_i64 = from as i64;
                            let var32 = Vec::<Val>::from_val(env, &val_from_i64(from)).len() as i64
                            commitment = var32;
                            if ((commitment as u64) < 4294967296 as u64) as i32 != 0 {
                                break 'label0;
                            }
                            var4 = ((commitment as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_add(-1);
                            let var33 = Vec::<Val>::from_val(env, &val_from_i64(from)).len() as i64
                            if (var4 as u32 >= (var33 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                break 'label4;
                            }
                            commitment = (var4 as u32 as i64).wrapping_shl(32 as u32) | 0;
                            let var34 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(from)).get_unchecked(commitment as u32))
                            self.func40(env, var2.wrapping_add(56), var34);
                            if (slot_var2_56_i32 == 1) as i32 != 0 {
                                break 'label3;
                            }
                            let var36 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(slot_var2_64_i64))).into_val(env))
                            self.func52(env, var2.wrapping_add(16), 0, var4, var36);
                            from = slot_var2_16_i64;
                            let var38 = Vec::<Val>::from_val(env, &val_from_i64(from)).len() as i64
                            if (var4 as u32 >= (var38 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                break 'label4;
                            }
                            let var39 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(from)).get_unchecked(commitment as u32))
                            self.func40(env, var2.wrapping_add(56), var39);
                            if (slot_var2_56_i32 == 1) as i32 != 0 {
                                break 'label3;
                            }
                            let var41 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(slot_var2_64_i64))).into_val(env))
                            from = var41;
                            self.func53(env, var2.wrapping_add(56));
                            var5 = 0;
                            'label7: loop {
                                'label8: loop {
                                    'label9: loop {
                                        'label10: loop {
                                            let mut slot_var2_48_i32 = self.memory.load32(var2 as usize + 48) as i32;
                                            if (var5 as u32 >= slot_var2_48_i32 as u32) as i32 != 0 {
                                                break 'label10;
                                            }
                                            var8 = var4 & 1;
                                            var6 = { let a = var4.wrapping_add(-1); let b = var4 | 1; if var8 != 0 { a } else { b } };
                                            'label11: loop {
                                                if var5 != 0 {
                                                    break 'label11;
                                                }
                                                commitment = 0;
                                                let var43 = Vec::<Val>::from_val(env, &val_from_i64(slot_var2_16_i64)).len() as i64
                                                if (var6 as u32 >= (var43 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                                    break 'label8;
                                                }
                                                let var44 = Vec::<Val>::from_val(env, &val_from_i64(slot_var2_16_i64)).len() as i64
                                                if (var6 as u32 >= (var44 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                let var45 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(slot_var2_16_i64)).get_unchecked((var6 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
                                                self.func40(env, var2.wrapping_add(96), var45);
                                                let mut slot_var2_96_i32 = self.memory.load32(var2 as usize + 96) as i32;
                                                if (slot_var2_96_i32 == 1) as i32 != 0 {
                                                    break 'label3;
                                                }
                                                let mut slot_var2_104_i64 = self.memory.load64(var2 as usize + 104) as i64;
                                                let var47 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(slot_var2_104_i64))).into_val(env))
                                                commitment = var47;
                                                break 'label8;
                                                break;
                                            }
                                            let mut slot_var2_32_i64 = self.memory.load64(var2 as usize + 32) as i64;
                                            let mut slot_var2_40_i64 = self.memory.load64(var2 as usize + 40) as i64;
                                            self.func54(env, var2.wrapping_add(96), slot_var2_32_i64, slot_var2_40_i64, var5, var6);
                                            if slot_var2_96_i32 != 0 {
                                                break 'label9;
                                            }
                                            let var49 = self.func55(env, var2.wrapping_add(16), var6, var5);
                                            commitment = var49;
                                            break 'label8;
                                            break;
                                        }
                                        let var50 = self.func56(env, from);
                                        let mut slot_var2_24_i64 = var50 as i64;
                                        let var51 = Vec::<Val>::from_val(env, &val_from_i64(slot_var2_16_i64)).len() as i64
                                        from = var51;
                                        if (from as u64 <= 4294967295 as u64) as i32 != 0 {
                                            break 'label0;
                                        }
                                        commitment = slot_var2_24_i64;
                                        var4 = slot_var2_48_i32;
                                        self.func46(env, leaves, slot_var2_16_i64);
                                        self.func44(env, var4);
                                        self.func45(env, root, commitment);
                                        from = (from & 18446744069414584320).wrapping_add(4294967295);
                                        break 'label1;
                                        break;
                                    }
                                    commitment = slot_var2_104_i64;
                                    break;
                                }
                                'label12: loop {
                                    'label13: loop {
                                        if var8 != 0 {
                                            break 'label13;
                                        }
                                        let var55 = self.func57(env, var2.wrapping_add(56), from, commitment);
                                        from = var55;
                                        break 'label12;
                                        break;
                                    }
                                    let var56 = self.func57(env, var2.wrapping_add(56), commitment, from);
                                    from = var56;
                                    break;
                                }
                                var5 = var5.wrapping_add(1);
                                var4 = (var4 as u32).wrapping_shr(1 as u32) as i32;
                                self.func52(env, var2.wrapping_add(16), var5, var4, from);
                                continue 'label7;
                                break;
                            }
                            break;
                        }
                        self.func58(env);
                        unreachable!();
                        break;
                    }
                    unreachable!();
                    break;
                }
                from = 21474836483 /* Error(Contract, #5) */;
                break;
            }
            self.global0 = var2.wrapping_add(112);
            return from;
            break;
        }
        self.func34(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func52(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i64) {
        let var4 = self.memory.load64(arg0 as usize + 24) as i64;
        let var5 = self.func87(env, arg1, arg2);
        let var6 = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(var4)); m.set(val_from_i64(var5), val_from_i64(arg3)); val_to_i64(m.into_val(env)) }
        self.memory.store64(arg0 as usize + 24, var6 as u64);
    }
    fn func53(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var1 = var9.wrapping_sub(80);
        self.global0 = var1;
        let var10 = self.func49(env, 1048923);
        let var11 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var10))).into_val(env))
        var2 = var11;
        let var12 = self.func49(env, 1048955);
        let var13 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var12))).into_val(env))
        var3 = var13;
        let var14 = self.func49(env, 1048987);
        let var15 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var14))).into_val(env))
        let mut slot_var1_72_i64 = var15 as i64;
        let mut slot_var1_64_i64 = var3 as i64;
        let mut slot_var1_56_i64 = var2 as i64;
        var4 = 0;
        'label0: loop {
            'label1: loop {
                if (var4 != 24) as i32 != 0 {
                    break 'label1;
                }
                var4 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var4 == 24) as i32 != 0 {
                            break 'label2;
                        }
                        let var16 = self.memory.load64(var1.wrapping_add(56).wrapping_add(var4) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(var4) as usize, var16 as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var2 = var17;
                let var18 = self.func49(env, 1049019);
                let var19 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var18))).into_val(env))
                var3 = var19;
                let var20 = self.func49(env, 1049051);
                let var21 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var20))).into_val(env))
                var5 = var21;
                let var22 = self.func49(env, 1049083);
                let var23 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var22))).into_val(env))
                slot_var1_72_i64 = var23 as i64;
                slot_var1_64_i64 = var5 as i64;
                slot_var1_56_i64 = var3 as i64;
                var4 = 0;
                'label4: loop {
                    'label5: loop {
                        if (var4 != 24) as i32 != 0 {
                            break 'label5;
                        }
                        var4 = 0;
                        'label6: loop {
                            'label7: loop {
                                if (var4 == 24) as i32 != 0 {
                                    break 'label6;
                                }
                                let var24 = self.memory.load64(var1.wrapping_add(56).wrapping_add(var4) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(var4) as usize, var24 as u64);
                                var4 = var4.wrapping_add(8);
                                continue 'label7;
                                break;
                            }
                            break;
                        }
                        let var25 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        var3 = var25;
                        let var26 = self.func49(env, 1049115);
                        let var27 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var26))).into_val(env))
                        var5 = var27;
                        let var28 = self.func49(env, 1049147);
                        let var29 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var28))).into_val(env))
                        var6 = var29;
                        let var30 = self.func49(env, 1049179);
                        let var31 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var30))).into_val(env))
                        slot_var1_72_i64 = var31 as i64;
                        slot_var1_64_i64 = var6 as i64;
                        slot_var1_56_i64 = var5 as i64;
                        var4 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var4 != 24) as i32 != 0 {
                                    break 'label9;
                                }
                                var4 = 0;
                                'label10: loop {
                                    'label11: loop {
                                        if (var4 == 24) as i32 != 0 {
                                            break 'label10;
                                        }
                                        let var32 = self.memory.load64(var1.wrapping_add(56).wrapping_add(var4) as usize) as i64;
                                        self.memory.store64(var1.wrapping_add(var4) as usize, var32 as u64);
                                        var4 = var4.wrapping_add(8);
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                let var33 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                let mut slot_var1_48_i64 = var33 as i64;
                                let mut slot_var1_40_i64 = var3 as i64;
                                let mut slot_var1_32_i64 = var2 as i64;
                                var4 = 0;
                                'label12: loop {
                                    'label13: loop {
                                        if (var4 != 24) as i32 != 0 {
                                            break 'label13;
                                        }
                                        var4 = 0;
                                        'label14: loop {
                                            'label15: loop {
                                                if (var4 == 24) as i32 != 0 {
                                                    break 'label14;
                                                }
                                                let var34 = self.memory.load64(var1.wrapping_add(32).wrapping_add(var4) as usize) as i64;
                                                self.memory.store64(var1.wrapping_add(var4) as usize, var34 as u64);
                                                var4 = var4.wrapping_add(8);
                                                continue 'label15;
                                                break;
                                            }
                                            break;
                                        }
                                        let var35 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                        var2 = var35;
                                        let var36 = self.func84(env);
                                        var3 = var36;
                                        let var37 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                        var5 = var37;
                                        var4 = var1.wrapping_add(24);
                                        let mut slot_var4_0_i64 = 939524096 as i64;
                                        var7 = var1.wrapping_add(16);
                                        let mut slot_var7_0_i64 = var3 as i64;
                                        var8 = var1.wrapping_add(8);
                                        let mut slot_var8_0_i64 = var2 as i64;
                                        let mut slot_var1_0_i64 = var5 as i64;
                                        self.func85(env, var1);
                                        self.memory.store64(arg0.wrapping_add(24) as usize, slot_var4_0_i64 as u64);
                                        self.memory.store64(arg0.wrapping_add(16) as usize, slot_var7_0_i64 as u64);
                                        self.memory.store64(arg0.wrapping_add(8) as usize, slot_var8_0_i64 as u64);
                                        self.memory.store64(arg0 as usize, slot_var1_0_i64 as u64);
                                        self.global0 = var1.wrapping_add(80);
                                        return;
                                        break;
                                    }
                                    self.memory.store64(var1.wrapping_add(var4) as usize, 0 /* Void */ as u64);
                                    var4 = var4.wrapping_add(8);
                                    continue 'label12;
                                    break;
                                }
                                break;
                            }
                            self.memory.store64(var1.wrapping_add(var4) as usize, 0 /* Void */ as u64);
                            var4 = var4.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    self.memory.store64(var1.wrapping_add(var4) as usize, 0 /* Void */ as u64);
                    var4 = var4.wrapping_add(8);
                    continue 'label4;
                    break;
                }
                break;
            }
            self.memory.store64(var1.wrapping_add(var4) as usize, 0 /* Void */ as u64);
            var4 = var4.wrapping_add(8);
            continue 'label0;
            break;
        }
    }
    fn func54(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32, mut arg4: i32) {
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        var5 = 1 /* True */;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var7 = self.func87(env, arg3, arg4);
                    var6 = var7;
                    let var8 = if Map::<Val, Val>::from_val(env, &val_from_i64(arg2)).has(val_from_i64(var6)) { 1 } else { 0 }
                    if (var8 != 1 /* True */) as i32 != 0 {
                        break 'label2;
                    }
                    let var9 = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg2)).get(val_from_i64(var6)).unwrap_or(val_from_i64(0)))
                    arg2 = var9;
                    arg3 = arg2 as i32 & 255;
                    if (arg3 == 12) as i32 != 0 {
                        break 'label1;
                    }
                    if (arg3 == 70) as i32 != 0 {
                        break 'label1;
                    }
                    break 'label0;
                    break;
                }
                'label3: loop {
                    arg2 = (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0;
                    let var10 = if Map::<Val, Val>::from_val(env, &val_from_i64(arg1)).has(val_from_i64(arg2)) { 1 } else { 0 }
                    if (var10 == 1 /* True */) as i32 != 0 {
                        break 'label3;
                    }
                    var5 = 0 /* False */;
                    break 'label1;
                    break;
                }
                var5 = 1 /* True */;
                let var11 = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg1)).get(val_from_i64(arg2)).unwrap_or(val_from_i64(0)))
                arg2 = var11;
                arg3 = arg2 as i32 & 255;
                if (arg3 == 12) as i32 != 0 {
                    break 'label1;
                }
                if (arg3 != 70) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            self.memory.store64(arg0 as usize + 8, arg2 as u64);
            self.memory.store64(arg0 as usize, var5 as u64);
            return;
            break;
        }
        unreachable!();
    }
    fn func55(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var3 = var7.wrapping_sub(48);
        self.global0 = var3;
        var4 = 0;
        'label0: loop {
            let var8 = self.memory.load32(arg0 as usize + 32) as i32;
            if (arg2 as u32 > var8 as u32) as i32 != 0 {
                break 'label0;
            }
            let var9 = self.memory.load64(arg0 as usize + 16) as i64;
            let var10 = self.memory.load64(arg0 as usize + 24) as i64;
            self.func54(env, var3.wrapping_add(16), var9, var10, arg2, arg1);
            'label1: loop {
                let mut slot_var3_16_i32 = self.memory.load32(var3 as usize + 16) as i32;
                if (slot_var3_16_i32 != 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                var4 = slot_var3_24_i64;
                break 'label0;
                break;
            }
            'label2: loop {
                'label3: loop {
                    'label4: loop {
                        if (arg2 == 0) as i32 != 0 {
                            break 'label4;
                        }
                        var5 = 0;
                        if (arg1 < 0) as i32 != 0 {
                            break 'label2;
                        }
                        arg1 = arg1.wrapping_shl(1 as u32);
                        arg2 = arg2.wrapping_add(-1);
                        let var12 = self.func55(env, arg0, arg1, arg2);
                        var4 = var12;
                        let var13 = self.func55(env, arg0, arg1 | 1, arg2);
                        let mut slot_var3_8_i64 = var13 as i64;
                        let mut slot_var3_0_i64 = var4 as i64;
                        break 'label3;
                        break;
                    }
                    let var14 = self.memory.load64(arg0 as usize) as i64;
                    var6 = var14;
                    let var15 = Vec::<Val>::from_val(env, &val_from_i64(var6)).len() as i64
                    if (arg1 as u32 >= (var15 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                        break 'label0;
                    }
                    'label5: loop {
                        'label6: loop {
                            let var16 = Vec::<Val>::from_val(env, &val_from_i64(var6)).len() as i64
                            if (arg1 as u32 >= (var16 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32) as i32 != 0 {
                                break 'label6;
                            }
                            let var17 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var6)).get_unchecked((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
                            self.func40(env, var3.wrapping_add(16), var17);
                            if (slot_var3_16_i32 != 1) as i32 != 0 {
                                break 'label5;
                            }
                            unreachable!();
                            break;
                        }
                        self.func58(env);
                        unreachable!();
                        break;
                    }
                    let var20 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(slot_var3_24_i64))).into_val(env))
                    var4 = var20;
                    break 'label0;
                    break;
                }
                'label7: loop {
                    'label8: loop {
                        if (var5 == 16) as i32 != 0 {
                            break 'label7;
                        }
                        self.memory.store64(var3.wrapping_add(16).wrapping_add(var5) as usize, 0 /* Void */ as u64);
                        var5 = var5.wrapping_add(8);
                        continue 'label8;
                        break;
                    }
                    break;
                }
                var5 = 0;
                'label9: loop {
                    'label10: loop {
                        if (var5 == 16) as i32 != 0 {
                            break 'label9;
                        }
                        let var21 = self.memory.load64(var3.wrapping_add(var5) as usize) as i64;
                        self.memory.store64(var3.wrapping_add(16).wrapping_add(var5) as usize, var21 as u64);
                        var5 = var5.wrapping_add(8);
                        continue 'label10;
                        break;
                    }
                    break;
                }
                let var22 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var4 = var22;
                self.func53(env, var3.wrapping_add(16));
                let var24 = self.func86(env, var3.wrapping_add(16), var4);
                var4 = var24;
                break 'label0;
                break;
            }
            self.func34(env);
            unreachable!();
            break;
        }
        self.global0 = var3.wrapping_add(48);
        var4
    }
    fn func56(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        let var3 = val_to_i64(U256::try_from_val(env, &val_from_i64(arg0)).unwrap().to_be_bytes().into_val(env))
        self.func89(env, var1, var3);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        arg0 = slot_var1_8_i64;
        self.global0 = var1.wrapping_add(16);
        arg0
    }
    fn func57(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32);
        self.global0 = var3;
        let mut slot_var3_8_i64 = arg2 as i64;
        let mut slot_var3_0_i64 = arg1 as i64;
        var4 = 0;
        let var6: i64;
        'label0: loop {
            'label1: loop {
                if (var4 != 16) as i32 != 0 {
                    break 'label1;
                }
                var4 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var4 == 16) as i32 != 0 {
                            break 'label2;
                        }
                        let var7 = self.memory.load64(var3.wrapping_add(var4) as usize) as i64;
                        self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, var7 as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                let var9 = self.func86(env, arg0, var8);
                arg2 = var9;
                self.global0 = var3.wrapping_add(32);
                return arg2;
                break;
            }
            self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, 0 /* Void */ as u64);
            var4 = var4.wrapping_add(8);
            continue 'label0;
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func58(&mut self, env: &Env) {
        self.func83(env, 43);
        unreachable!();
    }
    fn func59(&mut self, env: &Env, mut to: i64, mut proof_bytes: i64, mut pub_signals_bytes: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let mut var25: i64 = 0;
        let mut var26: i64 = 0;
        let mut var27: i64 = 0;
        let var28 = self.global0;
        var3 = var28.wrapping_sub(400);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    if (!(Bytes::try_from_val(env, &val_from_i64(proof_bytes)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    if (!(Bytes::try_from_val(env, &val_from_i64(pub_signals_bytes)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let var29 = { address_from_i64(env, to).require_auth(); 0 }
                    var29;
                    'label3: loop {
                        let var30 = self.func60(env);
                        if (var30 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        self.func38(env, var3.wrapping_add(208), token);
                        'label4: loop {
                            let mut slot_var3_208_i32 = self.memory.load32(var3 as usize + 208) as i32;
                            if (slot_var3_208_i32 == 0) as i32 != 0 {
                                break 'label4;
                            }
                            let mut slot_var3_216_i64 = self.memory.load64(var3 as usize + 216) as i64;
                            var4 = slot_var3_216_i64;
                            let var32 = val_to_i64(env.current_contract_address().into_val(env))
                            self.func61(env, var3.wrapping_add(208), var4, var32);
                            'label5: loop {
                                var5 = slot_var3_216_i64;
                                if ({ let a = ((slot_var3_208_i32 as u64) < 1000000000 as u64) as i32; let b = (var5 < 0 /* False */) as i32; if (var5 == 0) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label5;
                                }
                                let var34 = self.func36(env, vk);
                                if (var34 == 0) as i32 != 0 {
                                    break 'label4;
                                }
                                let var35 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(vk)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(vk)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(vk)).unwrap_or(val_from_i64(0))) } };
                                var5 = var35;
                                if (!(Bytes::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                                    break 'label2;
                                }
                                let mut slot_var3_16_i32 = 0 as i32;
                                self.func62(env, var3.wrapping_add(208), var5, var3.wrapping_add(16));
                                let var37 = self.func63(env, var3.wrapping_add(208));
                                var6 = var37;
                                self.func64(env, var3.wrapping_add(208), var5, var3.wrapping_add(16));
                                let var39 = self.func65(env, var3.wrapping_add(208));
                                var7 = var39;
                                self.func64(env, var3.wrapping_add(208), var5, var3.wrapping_add(16));
                                let var41 = self.func65(env, var3.wrapping_add(208));
                                var8 = var41;
                                self.func64(env, var3.wrapping_add(208), var5, var3.wrapping_add(16));
                                let var43 = self.func65(env, var3.wrapping_add(208));
                                var9 = var43;
                                var10 = slot_var3_16_i32;
                                var11 = var10.wrapping_add(4);
                                if ((var11 as u32) < var10 as u32) as i32 != 0 {
                                    break 'label0;
                                }
                                slot_var3_208_i32 = 0 as i32;
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                let var44 = val_to_i64(Bytes::from_val(env, &val_from_i64(var5)).slice(var10 as u32..var11 as u32).into_val(env));
                                                var12 = var44;
                                                let var45 = Bytes::from_val(env, &val_from_i64(var12)).len() as i64
                                                if (var45 & 18446744069414584320 != 17179869184) as i32 != 0 {
                                                    break 'label9;
                                                }
                                                self.func67(env, var12, var3.wrapping_add(208), 4);
                                                slot_var3_16_i32 = var11 as i32;
                                                var10 = slot_var3_208_i32;
                                                var10 = var10.wrapping_shl(24 as u32) | (var10 & 65280).wrapping_shl(8 as u32) | (var10 as u32).wrapping_shr(8 as u32) as i32 & 65280 | (var10 as u32).wrapping_shr(24 as u32) as i32;
                                                let var47 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                var12 = var47;
                                                'label10: loop {
                                                    'label11: loop {
                                                        if (var10 == 0) as i32 != 0 {
                                                            break 'label10;
                                                        }
                                                        self.func62(env, var3.wrapping_add(208), var5, var3.wrapping_add(16));
                                                        var10 = var10.wrapping_add(-1);
                                                        let var49 = self.func63(env, var3.wrapping_add(208));
                                                        let var50 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var12)); v.push_back(val_from_i64(var49)); val_to_i64(v.into_val(env)) }
                                                        var12 = var50;
                                                        continue 'label11;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                let mut slot_var3_12_i32 = 0 as i32;
                                                self.func62(env, var3.wrapping_add(208), proof_bytes, var3.wrapping_add(12));
                                                let var52 = self.func63(env, var3.wrapping_add(208));
                                                var13 = var52;
                                                var10 = slot_var3_12_i32;
                                                var11 = var10.wrapping_add(192);
                                                if ((var11 as u32) < var10 as u32) as i32 != 0 {
                                                    break 'label0;
                                                }
                                                let var53 = self.func92(env, var3.wrapping_add(208), 0, 192);
                                                var53;
                                                let var54 = val_to_i64(Bytes::from_val(env, &val_from_i64(proof_bytes)).slice(var10 as u32..var11 as u32).into_val(env));
                                                var5 = var54;
                                                let var55 = Bytes::from_val(env, &val_from_i64(var5)).len() as i64
                                                if (var55 & 18446744069414584320 != 824633720832) as i32 != 0 {
                                                    break 'label9;
                                                }
                                                self.func67(env, var5, var3.wrapping_add(208), 192);
                                                slot_var3_12_i32 = var11 as i32;
                                                let var57 = self.func91(env, var3.wrapping_add(16), var3.wrapping_add(208), 192);
                                                var57;
                                                let var58 = self.func65(env, var3.wrapping_add(16));
                                                var14 = var58;
                                                self.func62(env, var3.wrapping_add(208), proof_bytes, var3.wrapping_add(12));
                                                let var60 = self.func63(env, var3.wrapping_add(208));
                                                var15 = var60;
                                                var10 = 0;
                                                slot_var3_208_i32 = 0 as i32;
                                                let var61 = val_to_i64(Bytes::from_val(env, &val_from_i64(pub_signals_bytes)).slice(0 as u32..4 as u32).into_val(env))
                                                var5 = var61;
                                                let var62 = Bytes::from_val(env, &val_from_i64(var5)).len() as i64
                                                if (var62 & 18446744069414584320 != 17179869184) as i32 != 0 {
                                                    break 'label9;
                                                }
                                                self.func67(env, var5, var3.wrapping_add(208), 4);
                                                var11 = slot_var3_208_i32;
                                                var16 = var11.wrapping_shl(24 as u32) | (var11 & 65280).wrapping_shl(8 as u32) | (var11 as u32).wrapping_shr(8 as u32) as i32 & 65280 | (var11 as u32).wrapping_shr(24 as u32) as i32;
                                                var11 = 36;
                                                let var64 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                var5 = var64;
                                                var17 = var3.wrapping_add(208).wrapping_add(16);
                                                'label12: loop {
                                                    'label13: loop {
                                                        if (var16 == var10) as i32 != 0 {
                                                            break 'label12;
                                                        }
                                                        if (var10 == 134217727) as i32 != 0 {
                                                            break 'label0;
                                                        }
                                                        var18 = var3.wrapping_add(208).wrapping_add(24);
                                                        let mut slot_var18_0_i64 = 0 /* False */ as i64;
                                                        let mut slot_var17_0_i64 = 0 /* False */ as i64;
                                                        var19 = var3.wrapping_add(208).wrapping_add(8);
                                                        let mut slot_var19_0_i64 = 0 /* False */ as i64;
                                                        let mut slot_var3_208_i64 = 0 /* False */ as i64;
                                                        let var65 = val_to_i64(Bytes::from_val(env, &val_from_i64(pub_signals_bytes)).slice(var11.wrapping_add(-32) as u32..var11 as u32).into_val(env));
                                                        proof_bytes = var65;
                                                        let var66 = Bytes::from_val(env, &val_from_i64(proof_bytes)).len() as i64
                                                        if (var66 & 18446744069414584320 != 137438953472) as i32 != 0 {
                                                            break 'label9;
                                                        }
                                                        self.func67(env, proof_bytes, var3.wrapping_add(208), 32);
                                                        self.memory.store64(var3.wrapping_add(16).wrapping_add(24) as usize, slot_var18_0_i64 as u64);
                                                        self.memory.store64(var3.wrapping_add(16).wrapping_add(16) as usize, slot_var17_0_i64 as u64);
                                                        self.memory.store64(var3.wrapping_add(16).wrapping_add(8) as usize, slot_var19_0_i64 as u64);
                                                        let mut slot_var3_16_i64 = slot_var3_208_i64 as i64;
                                                        var10 = var10.wrapping_add(1);
                                                        var11 = var11.wrapping_add(32);
                                                        let var68 = self.func49(env, var3.wrapping_add(16));
                                                        let var69 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var68))).into_val(env))
                                                        let var70 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var5)); v.push_back(val_from_i64(var69)); val_to_i64(v.into_val(env)) }
                                                        var5 = var70;
                                                        continue 'label13;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                let var71 = Vec::<Val>::from_val(env, &val_from_i64(var5)).len() as i64
                                                if ((var71 as u64) < 4294967296 as u64) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                'label14: loop {
                                                    let var72 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var5)).get_unchecked(0 as u32))
                                                    pub_signals_bytes = var72;
                                                    var10 = pub_signals_bytes as i32 & 255;
                                                    if (var10 == 12) as i32 != 0 {
                                                        break 'label14;
                                                    }
                                                    if (var10 != 70) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break;
                                                }
                                                let var73 = Vec::<Val>::from_val(env, &val_from_i64(var5)).len() as i64
                                                if ((var73 as u64) < 8589934592 as u64) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                'label15: loop {
                                                    let var74 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var5)).get_unchecked(1 as u32))
                                                    var10 = var74 as i32 & 255;
                                                    if (var10 == 12) as i32 != 0 {
                                                        break 'label15;
                                                    }
                                                    if (var10 != 70) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break;
                                                }
                                                let var75 = Vec::<Val>::from_val(env, &val_from_i64(var5)).len() as i64
                                                if ((var75 as u64) < 12884901888 as u64) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                'label16: loop {
                                                    let var76 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var5)).get_unchecked(2 as u32))
                                                    var20 = var76;
                                                    var10 = var20 as i32 & 255;
                                                    if (var10 == 12) as i32 != 0 {
                                                        break 'label16;
                                                    }
                                                    if (var10 != 70) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break;
                                                }
                                                let var77 = Vec::<Val>::from_val(env, &val_from_i64(var5)).len() as i64
                                                if ((var77 as u64) < 17179869184 as u64) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                'label17: loop {
                                                    let var78 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var5)).get_unchecked(3 as u32))
                                                    proof_bytes = var78;
                                                    var10 = proof_bytes as i32 & 255;
                                                    if (var10 == 12) as i32 != 0 {
                                                        break 'label17;
                                                    }
                                                    if (var10 != 70) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break;
                                                }
                                                let var79 = self.func68(env);
                                                let var80 = self.func56(env, proof_bytes);
                                                let var81 = self.func47(env, var79, var80);
                                                if var81 != 0 {
                                                    break 'label6;
                                                }
                                                self.func41(env, var3.wrapping_add(208), null);
                                                var10 = slot_var3_208_i32;
                                                let var83 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                var21 = { let a = slot_var3_216_i64; let b = var83; if var10 != 0 { a } else { b } };
                                                let var84 = self.func56(env, pub_signals_bytes);
                                                var22 = var84;
                                                let var85 = { let v = Vec::<Val>::from_val(env, &val_from_i64(var21)); match v.first_index_of(val_from_i64(var22)) { Some(i) => i as i64, None => 0 /* Void */ } }
                                                if (var85 != 0 /* Void */) as i32 != 0 {
                                                    break 'label7;
                                                }
                                                self.func39(env, var3.wrapping_add(208), root);
                                                var10 = slot_var3_208_i32;
                                                let var87 = self.func49(env, 1048859);
                                                let var88 = self.func56(env, var20);
                                                let var89 = self.func47(env, { let a = slot_var3_216_i64; let b = var87; if var10 != 0 { a } else { b } }, var88);
                                                if var89 != 0 {
                                                    break 'label8;
                                                }
                                                self.func38(env, var3.wrapping_add(208), g16v);
                                                if (slot_var3_208_i32 == 0) as i32 != 0 {
                                                    break 'label4;
                                                }
                                                var23 = slot_var3_216_i64;
                                                let var91 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                var24 = var91;
                                                let var92 = Vec::<Val>::from_val(env, &val_from_i64(var12)).len() as i64
                                                var25 = (var92 as u64).wrapping_shr(32 as u32) as i64;
                                                proof_bytes = 0 /* False */;
                                                pub_signals_bytes = 0;
                                                'label18: loop {
                                                    'label19: loop {
                                                        if (var25 == proof_bytes) as i32 != 0 {
                                                            break 'label18;
                                                        }
                                                        'label20: loop {
                                                            let var93 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var12)).get_unchecked(pub_signals_bytes as u32))
                                                            var20 = var93;
                                                            if (!(Bytes::try_from_val(env, &val_from_i64(var20)).is_ok())) as i32 != 0 {
                                                                break 'label20;
                                                            }
                                                            let var94 = Bytes::from_val(env, &val_from_i64(var20)).len() as i64
                                                            var26 = var94;
                                                            if (proof_bytes == 4294967295) as i32 != 0 {
                                                                break 'label0;
                                                            }
                                                            var26 = var26 & 18446744069414584320;
                                                            if (var26 != 412316860416) as i32 != 0 {
                                                                break 'label0;
                                                            }
                                                            pub_signals_bytes = pub_signals_bytes.wrapping_add(4294967296);
                                                            proof_bytes = proof_bytes.wrapping_add(1 /* True */);
                                                            var27 = { let a = var27; let b = var20; if (var26 != 412316860416) as i32 != 0 { a } else { b } };
                                                            let var95 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var24)); v.push_back(val_from_i64(var27)); val_to_i64(v.into_val(env)) }
                                                            var24 = var95;
                                                            continue 'label19;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    (proof_bytes != 4294967295) as i32;
                                                    break 'label0;
                                                    break;
                                                }
                                                let var96 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                pub_signals_bytes = var96;
                                                let var97 = Vec::<Val>::from_val(env, &val_from_i64(var5)).len() as i64
                                                var26 = (var97 as u64).wrapping_shr(32 as u32) as i64;
                                                var12 = 0 /* False */;
                                                proof_bytes = 0;
                                                'label21: loop {
                                                    'label22: loop {
                                                        if (var26 == var12) as i32 != 0 {
                                                            break 'label21;
                                                        }
                                                        'label23: loop {
                                                            'label24: loop {
                                                                let var98 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var5)).get_unchecked(proof_bytes as u32))
                                                                var27 = var98;
                                                                var10 = var27 as i32 & 255;
                                                                var11 = (var10 == 70) as i32;
                                                                if var11 != 0 {
                                                                    break 'label24;
                                                                }
                                                                if (var10 != 12) as i32 != 0 {
                                                                    break 'label23;
                                                                }
                                                                break;
                                                            }
                                                            var20 = var27;
                                                            break;
                                                        }
                                                        if (var12 == 4294967295) as i32 != 0 {
                                                            break 'label0;
                                                        }
                                                        'label25: loop {
                                                            if (var10 == 12) as i32 != 0 {
                                                                break 'label25;
                                                            }
                                                            if (var11 == 0) as i32 != 0 {
                                                                break 'label0;
                                                            }
                                                            break;
                                                        }
                                                        proof_bytes = proof_bytes.wrapping_add(4294967296);
                                                        var12 = var12.wrapping_add(1 /* True */);
                                                        let var99 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(pub_signals_bytes)); v.push_back(val_from_i64(var20)); val_to_i64(v.into_val(env)) }
                                                        pub_signals_bytes = var99;
                                                        continue 'label22;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                let var100 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                                                var5 = var100;
                                                let mut slot_var3_240_i64 = var24 as i64;
                                                let mut slot_var3_232_i64 = var8 as i64;
                                                let mut slot_var3_224_i64 = var9 as i64;
                                                slot_var3_216_i64 = var7 as i64;
                                                slot_var3_208_i64 = var6 as i64;
                                                let var101 = self.func69(env, 1048612, 5, var3.wrapping_add(208), 5);
                                                var12 = var101;
                                                slot_var3_224_i64 = var15 as i64;
                                                slot_var3_216_i64 = var14 as i64;
                                                slot_var3_208_i64 = var13 as i64;
                                                let var102 = self.func69(env, 1048656, 3, var3.wrapping_add(208), 3);
                                                proof_bytes = var102;
                                                let mut slot_var3_32_i64 = pub_signals_bytes as i64;
                                                let mut slot_var3_24_i64 = proof_bytes as i64;
                                                slot_var3_16_i64 = var12 as i64;
                                                var10 = 0;
                                                'label26: loop {
                                                    'label27: loop {
                                                        if (var10 != 24) as i32 != 0 {
                                                            break 'label27;
                                                        }
                                                        var10 = 0;
                                                        'label28: loop {
                                                            'label29: loop {
                                                                if (var10 == 24) as i32 != 0 {
                                                                    break 'label28;
                                                                }
                                                                let var103 = self.memory.load64(var3.wrapping_add(16).wrapping_add(var10) as usize) as i64;
                                                                self.memory.store64(var3.wrapping_add(208).wrapping_add(var10) as usize, var103 as u64);
                                                                var10 = var10.wrapping_add(8);
                                                                continue 'label29;
                                                                break;
                                                            }
                                                            break;
                                                        }
                                                        'label30: loop {
                                                            'label31: loop {
                                                                let var104 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                let var105 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(var23)), &Symbol::from_val(env, &val_from_i64(var5)), Vec::<Val>::from_val(env, &val_from_i64(var104))))
                                                                match var105 as i32 & 255 {
                                                                    0 => break 'label31,
                                                                    1 => break 'label30,
                                                                    _ => break 'label0,
                                                                }
                                                                break;
                                                            }
                                                            let var106 = val_to_i64(String::from_str(env, "Couldn't verify coin ownership proof"));
                                                            slot_var3_208_i64 = var106 as i64;
                                                            let var107 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                            var5 = var107;
                                                            break 'label1;
                                                            break;
                                                        }
                                                        let var108 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var21)); v.push_back(val_from_i64(var22)); val_to_i64(v.into_val(env)) }
                                                        self.func46(env, null, var108);
                                                        let var110 = val_to_i64(env.current_contract_address().into_val(env))
                                                        self.func31(env, var4, var110, to);
                                                        let var112 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                        var5 = var112;
                                                        break 'label1;
                                                        break;
                                                    }
                                                    self.memory.store64(var3.wrapping_add(208).wrapping_add(var10) as usize, 0 /* Void */ as u64);
                                                    var10 = var10.wrapping_add(8);
                                                    continue 'label26;
                                                    break;
                                                }
                                                break;
                                            }
                                            self.func71(env);
                                            unreachable!();
                                            break;
                                        }
                                        let var114 = val_to_i64(String::from_str(env, "Couldn't verify coin ownership proof"));
                                        slot_var3_208_i64 = var114 as i64;
                                        let var115 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                        var5 = var115;
                                        break 'label1;
                                        break;
                                    }
                                    let var116 = val_to_i64(String::from_str(env, "Nullifier already used"));
                                    slot_var3_208_i64 = var116 as i64;
                                    let var117 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                    var5 = var117;
                                    break 'label1;
                                    break;
                                }
                                let var118 = val_to_i64(String::from_str(env, "Association set root mismatch"));
                                slot_var3_208_i64 = var118 as i64;
                                let var119 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                var5 = var119;
                                break 'label1;
                                break;
                            }
                            let var120 = val_to_i64(String::from_str(env, "Insufficient balance"));
                            slot_var3_208_i64 = var120 as i64;
                            let var121 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                            var5 = var121;
                            break 'label1;
                            break;
                        }
                        self.func58(env);
                        unreachable!();
                        break;
                    }
                    self.func72(env);
                    unreachable!();
                    break;
                }
                unreachable!();
                break;
            }
            self.global0 = var3.wrapping_add(400);
            return var5;
            break;
        }
        self.func34(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func60(&mut self, env: &Env) -> i32 {
        let var0 = self.func68(env);
        let var1 = self.func49(env, 1048859);
        let var2 = self.func47(env, var0, var1);
        var2
    }
    fn func61(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(16);
        self.global0 = var3;
        let mut slot_var3_8_i64 = arg2 as i64;
        'label0: loop {
            'label1: loop {
                let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                let var7 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg1)), &Symbol::from_val(env, &val_from_i64(balance)), Vec::<Val>::from_val(env, &val_from_i64(var6))))
                arg2 = var7;
                var4 = arg2 as i32 & 255;
                if (var4 == 69) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    if (var4 != 11) as i32 != 0 {
                        break 'label2;
                    }
                    arg1 = arg2.wrapping_shr(63 as u32);
                    arg2 = arg2.wrapping_shr(0 as u32);
                    break 'label0;
                    break;
                }
                self.func34(env);
                unreachable!();
                break;
            }
            let var9 = ((val_from_i64(arg2).as_i128().unwrap_or(0) >> 64) as i64)
            arg1 = var9;
            let var10 = ((val_from_i64(arg2).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
            arg2 = var10;
            break;
        }
        self.memory.store64(arg0 as usize, arg2 as u64);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func62(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_sub(96);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                let var7 = self.memory.load32(arg2 as usize) as i32;
                var4 = var7;
                var5 = var4.wrapping_add(96);
                if ((var5 as u32) < var4 as u32) as i32 != 0 {
                    break 'label1;
                }
                let var8 = self.func92(env, var3, 0, 96);
                var3 = var8;
                let var9 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(var4 as u32..var5 as u32).into_val(env));
                arg1 = var9;
                let var10 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
                if (var10 & 18446744069414584320 == 412316860416) as i32 != 0 {
                    break 'label0;
                }
                self.func71(env);
                unreachable!();
                break;
            }
            self.func34(env);
            unreachable!();
            break;
        }
        self.func67(env, arg1, var3, 96);
        self.memory.store32(arg2 as usize, var5 as u32);
        let var14 = self.func91(env, arg0, var3, 96);
        var14;
        self.global0 = var3.wrapping_add(96);
    }
    fn func63(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let var1 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
        var1
    }
    fn func64(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_sub(192);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                let var7 = self.memory.load32(arg2 as usize) as i32;
                var4 = var7;
                var5 = var4.wrapping_add(192);
                if ((var5 as u32) < var4 as u32) as i32 != 0 {
                    break 'label1;
                }
                let var8 = self.func92(env, var3, 0, 192);
                var3 = var8;
                let var9 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(var4 as u32..var5 as u32).into_val(env));
                arg1 = var9;
                let var10 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
                if (var10 & 18446744069414584320 == 824633720832) as i32 != 0 {
                    break 'label0;
                }
                self.func71(env);
                unreachable!();
                break;
            }
            self.func34(env);
            unreachable!();
            break;
        }
        self.func67(env, arg1, var3, 192);
        self.memory.store32(arg2 as usize, var5 as u32);
        let var14 = self.func91(env, arg0, var3, 192);
        var14;
        self.global0 = var3.wrapping_add(192);
    }
    fn func65(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let var1 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
        var1
    }
    fn func66(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) -> i64 {
        let var3 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg0)).slice((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32..(arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32).into_val(env))
        var3
    }
    fn func67(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) {
        let var3 = 0 /* TODO: bytes_copy_to_linear_memory */
        var3;
    }
    fn func68(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func39(env, var0, assoc);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = self.func49(env, 1048859);
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    fn func69(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
        'label0: loop {
            if (arg1 == arg3) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var4 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
        var4
    }
    fn func70(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(String::from_str(env, "")) /* TODO: linear memory */
        var2
    }
    fn func71(&mut self, env: &Env) {
        self.func34(env);
        unreachable!();
    }
    fn func72(&mut self, env: &Env) {
        unreachable!();
    }
    fn func73(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func39(env, var0, root);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = self.func49(env, 1048859);
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    fn func74(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func35(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        var1 = slot_var0_8_i32;
        let var5 = self.memory.load32(var0 as usize + 12) as i64;
        var2 = var5;
        self.global0 = var0.wrapping_add(16);
        { let a = var2.wrapping_shl(32 as u32) | 0; let b = 0; if var1 & 1 != 0 { a } else { b } }
    }
    fn func75(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, leaves);
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var1 = slot_var0_0_i32;
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        let var6 = Vec::<Val>::from_val(env, &val_from_i64({ let a = slot_var0_8_i64; let b = var5; if var1 != 0 { a } else { b } })).len() as i64
        var2 = var6;
        self.global0 = var0.wrapping_add(16);
        var2 & 18446744069414584320 | 0
    }
    fn func76(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, leaves);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    fn func77(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var0 = var4.wrapping_sub(16);
        self.global0 = var0;
        self.func41(env, var0, null);
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
        var2 = slot_var0_0_i32;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env))
        var3 = var6;
        self.global0 = var0.wrapping_add(16);
        { let a = var1; let b = var3; if var2 != 0 { a } else { b } }
    }
    fn func78(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func38(env, var0, token);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func58(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        let var5 = val_to_i64(env.current_contract_address().into_val(env))
        self.func61(env, var0, slot_var0_8_i64, var5);
        let var7 = self.func32(env, slot_var0_0_i32, slot_var0_8_i64);
        var1 = var7;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func79(&mut self, env: &Env, mut caller: i64, mut association_root: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(caller)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                self.func40(env, var2, association_root);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                if (slot_var2_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                association_root = slot_var2_8_i64;
                let var5 = { address_from_i64(env, caller).require_auth(); 0 }
                var5;
                self.func38(env, var2, admin);
                if (slot_var2_0_i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                'label2: loop {
                    'label3: loop {
                        let var7 = { let a = val_from_i64(caller); let b = val_from_i64(slot_var2_8_i64); if a < b { -1 } else if a > b { 1 } else { 0 } }
                        if (var7 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        let var8 = val_to_i64(String::from_str(env, "Only the admin can set association root"));
                        let mut slot_var2_0_i64 = var8 as i64;
                        let var9 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        caller = var9;
                        break 'label2;
                        break;
                    }
                    self.func45(env, assoc, association_root);
                    let var11 = val_to_i64(String::from_str(env, "Association root set successfully"));
                    slot_var2_0_i64 = var11 as i64;
                    let var12 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    caller = var12;
                    break;
                }
                self.global0 = var2.wrapping_add(16);
                return caller;
                break;
            }
            unreachable!();
            break;
        }
        self.func58(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func80(&mut self, env: &Env) -> i64 {
        let var0 = self.func68(env);
        var0
    }
    fn func81(&mut self, env: &Env) -> i64 {
        let var0 = self.func60(env);
        var0 as u32 as i64
    }
    fn func82(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func38(env, var0, admin);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func58(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func83(&mut self, env: &Env, mut arg0: i32) {
        self.func72(env);
        unreachable!();
    }
    fn func84(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let mut var25: i64 = 0;
        let mut var26: i64 = 0;
        let mut var27: i64 = 0;
        let mut var28: i64 = 0;
        let mut var29: i64 = 0;
        let mut var30: i64 = 0;
        let mut var31: i64 = 0;
        let mut var32: i64 = 0;
        let mut var33: i64 = 0;
        let mut var34: i64 = 0;
        let mut var35: i64 = 0;
        let mut var36: i64 = 0;
        let mut var37: i64 = 0;
        let mut var38: i64 = 0;
        let mut var39: i64 = 0;
        let mut var40: i64 = 0;
        let mut var41: i64 = 0;
        let mut var42: i64 = 0;
        let mut var43: i64 = 0;
        let mut var44: i64 = 0;
        let mut var45: i64 = 0;
        let mut var46: i64 = 0;
        let mut var47: i64 = 0;
        let mut var48: i64 = 0;
        let mut var49: i64 = 0;
        let mut var50: i64 = 0;
        let mut var51: i64 = 0;
        let mut var52: i64 = 0;
        let mut var53: i64 = 0;
        let mut var54: i64 = 0;
        let mut var55: i64 = 0;
        let mut var56: i64 = 0;
        let mut var57: i64 = 0;
        let mut var58: i64 = 0;
        let mut var59: i64 = 0;
        let mut var60: i64 = 0;
        let mut var61: i64 = 0;
        let mut var62: i64 = 0;
        let mut var63: i64 = 0;
        let mut var64: i64 = 0;
        let mut var65: i64 = 0;
        let mut var66: i64 = 0;
        let var67 = self.global0;
        var0 = var67.wrapping_sub(1056);
        self.global0 = var0;
        let var68 = self.func49(env, 1049211);
        let var69 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var68))).into_val(env))
        var1 = var69;
        let var70 = self.func49(env, 1049243);
        let var71 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var70))).into_val(env))
        var2 = var71;
        let var72 = self.func49(env, 1049275);
        let var73 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var72))).into_val(env))
        let mut slot_var0_24_i64 = var73 as i64;
        let mut slot_var0_16_i64 = var2 as i64;
        let mut slot_var0_8_i64 = var1 as i64;
        var3 = 0;
        let var74: i64;
        'label0: loop {
            'label1: loop {
                if (var3 != 24) as i32 != 0 {
                    break 'label1;
                }
                var3 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var3 == 24) as i32 != 0 {
                            break 'label2;
                        }
                        let var75 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var75 as u64);
                        var3 = var3.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var76 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var1 = var76;
                let var77 = self.func49(env, 1049307);
                let var78 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var77))).into_val(env))
                var2 = var78;
                let var79 = self.func49(env, 1049339);
                let var80 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var79))).into_val(env))
                var4 = var80;
                let var81 = self.func49(env, 1049371);
                let var82 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var81))).into_val(env))
                slot_var0_24_i64 = var82 as i64;
                slot_var0_16_i64 = var4 as i64;
                slot_var0_8_i64 = var2 as i64;
                var3 = 0;
                'label4: loop {
                    'label5: loop {
                        if (var3 != 24) as i32 != 0 {
                            break 'label5;
                        }
                        var3 = 0;
                        'label6: loop {
                            'label7: loop {
                                if (var3 == 24) as i32 != 0 {
                                    break 'label6;
                                }
                                let var83 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var83 as u64);
                                var3 = var3.wrapping_add(8);
                                continue 'label7;
                                break;
                            }
                            break;
                        }
                        let var84 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        var2 = var84;
                        let var85 = self.func49(env, 1049403);
                        let var86 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var85))).into_val(env))
                        var4 = var86;
                        let var87 = self.func49(env, 1049435);
                        let var88 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var87))).into_val(env))
                        var5 = var88;
                        let var89 = self.func49(env, 1049467);
                        let var90 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var89))).into_val(env))
                        slot_var0_24_i64 = var90 as i64;
                        slot_var0_16_i64 = var5 as i64;
                        slot_var0_8_i64 = var4 as i64;
                        var3 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var3 != 24) as i32 != 0 {
                                    break 'label9;
                                }
                                var3 = 0;
                                'label10: loop {
                                    'label11: loop {
                                        if (var3 == 24) as i32 != 0 {
                                            break 'label10;
                                        }
                                        let var91 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var91 as u64);
                                        var3 = var3.wrapping_add(8);
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                let var92 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                var4 = var92;
                                let var93 = self.func49(env, 1049499);
                                let var94 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var93))).into_val(env))
                                var5 = var94;
                                let var95 = self.func49(env, 1049531);
                                let var96 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var95))).into_val(env))
                                var6 = var96;
                                let var97 = self.func49(env, 1049563);
                                let var98 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var97))).into_val(env))
                                slot_var0_24_i64 = var98 as i64;
                                slot_var0_16_i64 = var6 as i64;
                                slot_var0_8_i64 = var5 as i64;
                                var3 = 0;
                                'label12: loop {
                                    'label13: loop {
                                        if (var3 != 24) as i32 != 0 {
                                            break 'label13;
                                        }
                                        var3 = 0;
                                        'label14: loop {
                                            'label15: loop {
                                                if (var3 == 24) as i32 != 0 {
                                                    break 'label14;
                                                }
                                                let var99 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var99 as u64);
                                                var3 = var3.wrapping_add(8);
                                                continue 'label15;
                                                break;
                                            }
                                            break;
                                        }
                                        let var100 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                        var5 = var100;
                                        let var101 = self.func49(env, 1049595);
                                        let var102 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var101))).into_val(env))
                                        var6 = var102;
                                        let var103 = self.func49(env, 1049627);
                                        let var104 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var103))).into_val(env))
                                        var7 = var104;
                                        let var105 = self.func49(env, 1049659);
                                        let var106 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var105))).into_val(env))
                                        slot_var0_24_i64 = var106 as i64;
                                        slot_var0_16_i64 = var7 as i64;
                                        slot_var0_8_i64 = var6 as i64;
                                        var3 = 0;
                                        'label16: loop {
                                            'label17: loop {
                                                if (var3 != 24) as i32 != 0 {
                                                    break 'label17;
                                                }
                                                var3 = 0;
                                                'label18: loop {
                                                    'label19: loop {
                                                        if (var3 == 24) as i32 != 0 {
                                                            break 'label18;
                                                        }
                                                        let var107 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var107 as u64);
                                                        var3 = var3.wrapping_add(8);
                                                        continue 'label19;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                let var108 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                var6 = var108;
                                                let var109 = self.func49(env, 1049691);
                                                let var110 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var109))).into_val(env))
                                                var7 = var110;
                                                let var111 = self.func49(env, 1049723);
                                                let var112 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var111))).into_val(env))
                                                var8 = var112;
                                                let var113 = self.func49(env, 1049755);
                                                let var114 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var113))).into_val(env))
                                                slot_var0_24_i64 = var114 as i64;
                                                slot_var0_16_i64 = var8 as i64;
                                                slot_var0_8_i64 = var7 as i64;
                                                var3 = 0;
                                                'label20: loop {
                                                    'label21: loop {
                                                        if (var3 != 24) as i32 != 0 {
                                                            break 'label21;
                                                        }
                                                        var3 = 0;
                                                        'label22: loop {
                                                            'label23: loop {
                                                                if (var3 == 24) as i32 != 0 {
                                                                    break 'label22;
                                                                }
                                                                let var115 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var115 as u64);
                                                                var3 = var3.wrapping_add(8);
                                                                continue 'label23;
                                                                break;
                                                            }
                                                            break;
                                                        }
                                                        let var116 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                        var7 = var116;
                                                        let var117 = self.func49(env, 1049787);
                                                        let var118 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var117))).into_val(env))
                                                        var8 = var118;
                                                        let var119 = self.func49(env, 1049819);
                                                        let var120 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var119))).into_val(env))
                                                        var9 = var120;
                                                        let var121 = self.func49(env, 1049851);
                                                        let var122 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var121))).into_val(env))
                                                        slot_var0_24_i64 = var122 as i64;
                                                        slot_var0_16_i64 = var9 as i64;
                                                        slot_var0_8_i64 = var8 as i64;
                                                        var3 = 0;
                                                        'label24: loop {
                                                            'label25: loop {
                                                                if (var3 != 24) as i32 != 0 {
                                                                    break 'label25;
                                                                }
                                                                var3 = 0;
                                                                'label26: loop {
                                                                    'label27: loop {
                                                                        if (var3 == 24) as i32 != 0 {
                                                                            break 'label26;
                                                                        }
                                                                        let var123 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var123 as u64);
                                                                        var3 = var3.wrapping_add(8);
                                                                        continue 'label27;
                                                                        break;
                                                                    }
                                                                    break;
                                                                }
                                                                let var124 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                var8 = var124;
                                                                let var125 = self.func49(env, 1049883);
                                                                let var126 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var125))).into_val(env))
                                                                var9 = var126;
                                                                let var127 = self.func49(env, 1049915);
                                                                let var128 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var127))).into_val(env))
                                                                var10 = var128;
                                                                let var129 = self.func49(env, 1049947);
                                                                let var130 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var129))).into_val(env))
                                                                slot_var0_24_i64 = var130 as i64;
                                                                slot_var0_16_i64 = var10 as i64;
                                                                slot_var0_8_i64 = var9 as i64;
                                                                var3 = 0;
                                                                'label28: loop {
                                                                    'label29: loop {
                                                                        if (var3 != 24) as i32 != 0 {
                                                                            break 'label29;
                                                                        }
                                                                        var3 = 0;
                                                                        'label30: loop {
                                                                            'label31: loop {
                                                                                if (var3 == 24) as i32 != 0 {
                                                                                    break 'label30;
                                                                                }
                                                                                let var131 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var131 as u64);
                                                                                var3 = var3.wrapping_add(8);
                                                                                continue 'label31;
                                                                                break;
                                                                            }
                                                                            break;
                                                                        }
                                                                        let var132 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                        var9 = var132;
                                                                        let var133 = self.func49(env, 1049979);
                                                                        let var134 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var133))).into_val(env))
                                                                        var10 = var134;
                                                                        let var135 = self.func49(env, 1050011);
                                                                        let var136 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var135))).into_val(env))
                                                                        var11 = var136;
                                                                        let var137 = self.func49(env, 1050043);
                                                                        let var138 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var137))).into_val(env))
                                                                        slot_var0_24_i64 = var138 as i64;
                                                                        slot_var0_16_i64 = var11 as i64;
                                                                        slot_var0_8_i64 = var10 as i64;
                                                                        var3 = 0;
                                                                        'label32: loop {
                                                                            'label33: loop {
                                                                                if (var3 != 24) as i32 != 0 {
                                                                                    break 'label33;
                                                                                }
                                                                                var3 = 0;
                                                                                'label34: loop {
                                                                                    'label35: loop {
                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                            break 'label34;
                                                                                        }
                                                                                        let var139 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var139 as u64);
                                                                                        var3 = var3.wrapping_add(8);
                                                                                        continue 'label35;
                                                                                        break;
                                                                                    }
                                                                                    break;
                                                                                }
                                                                                let var140 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                var10 = var140;
                                                                                let var141 = self.func49(env, 1050075);
                                                                                let var142 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var141))).into_val(env))
                                                                                var11 = var142;
                                                                                let var143 = self.func49(env, 1050107);
                                                                                let var144 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var143))).into_val(env))
                                                                                var12 = var144;
                                                                                let var145 = self.func49(env, 1050139);
                                                                                let var146 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var145))).into_val(env))
                                                                                slot_var0_24_i64 = var146 as i64;
                                                                                slot_var0_16_i64 = var12 as i64;
                                                                                slot_var0_8_i64 = var11 as i64;
                                                                                var3 = 0;
                                                                                'label36: loop {
                                                                                    'label37: loop {
                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                            break 'label37;
                                                                                        }
                                                                                        var3 = 0;
                                                                                        'label38: loop {
                                                                                            'label39: loop {
                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                    break 'label38;
                                                                                                }
                                                                                                let var147 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var147 as u64);
                                                                                                var3 = var3.wrapping_add(8);
                                                                                                continue 'label39;
                                                                                                break;
                                                                                            }
                                                                                            break;
                                                                                        }
                                                                                        let var148 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                        var11 = var148;
                                                                                        let var149 = self.func49(env, 1050171);
                                                                                        let var150 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var149))).into_val(env))
                                                                                        var12 = var150;
                                                                                        let var151 = self.func49(env, 1050203);
                                                                                        let var152 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var151))).into_val(env))
                                                                                        var13 = var152;
                                                                                        let var153 = self.func49(env, 1050235);
                                                                                        let var154 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var153))).into_val(env))
                                                                                        slot_var0_24_i64 = var154 as i64;
                                                                                        slot_var0_16_i64 = var13 as i64;
                                                                                        slot_var0_8_i64 = var12 as i64;
                                                                                        var3 = 0;
                                                                                        'label40: loop {
                                                                                            'label41: loop {
                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                    break 'label41;
                                                                                                }
                                                                                                var3 = 0;
                                                                                                'label42: loop {
                                                                                                    'label43: loop {
                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                            break 'label42;
                                                                                                        }
                                                                                                        let var155 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var155 as u64);
                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                        continue 'label43;
                                                                                                        break;
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                let var156 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                var12 = var156;
                                                                                                let var157 = self.func49(env, 1050267);
                                                                                                let var158 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var157))).into_val(env))
                                                                                                var13 = var158;
                                                                                                let var159 = self.func49(env, 1050299);
                                                                                                let var160 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var159))).into_val(env))
                                                                                                var14 = var160;
                                                                                                let var161 = self.func49(env, 1050331);
                                                                                                let var162 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var161))).into_val(env))
                                                                                                slot_var0_24_i64 = var162 as i64;
                                                                                                slot_var0_16_i64 = var14 as i64;
                                                                                                slot_var0_8_i64 = var13 as i64;
                                                                                                var3 = 0;
                                                                                                'label44: loop {
                                                                                                    'label45: loop {
                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                            break 'label45;
                                                                                                        }
                                                                                                        var3 = 0;
                                                                                                        'label46: loop {
                                                                                                            'label47: loop {
                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                    break 'label46;
                                                                                                                }
                                                                                                                let var163 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var163 as u64);
                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                continue 'label47;
                                                                                                                break;
                                                                                                            }
                                                                                                            break;
                                                                                                        }
                                                                                                        let var164 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                        var13 = var164;
                                                                                                        let var165 = self.func49(env, 1050363);
                                                                                                        let var166 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var165))).into_val(env))
                                                                                                        var14 = var166;
                                                                                                        let var167 = self.func49(env, 1050395);
                                                                                                        let var168 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var167))).into_val(env))
                                                                                                        var15 = var168;
                                                                                                        let var169 = self.func49(env, 1050427);
                                                                                                        let var170 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var169))).into_val(env))
                                                                                                        slot_var0_24_i64 = var170 as i64;
                                                                                                        slot_var0_16_i64 = var15 as i64;
                                                                                                        slot_var0_8_i64 = var14 as i64;
                                                                                                        var3 = 0;
                                                                                                        'label48: loop {
                                                                                                            'label49: loop {
                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                    break 'label49;
                                                                                                                }
                                                                                                                var3 = 0;
                                                                                                                'label50: loop {
                                                                                                                    'label51: loop {
                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                            break 'label50;
                                                                                                                        }
                                                                                                                        let var171 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var171 as u64);
                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                        continue 'label51;
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    break;
                                                                                                                }
                                                                                                                let var172 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                var14 = var172;
                                                                                                                let var173 = self.func49(env, 1050459);
                                                                                                                let var174 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var173))).into_val(env))
                                                                                                                var15 = var174;
                                                                                                                let var175 = self.func49(env, 1050491);
                                                                                                                let var176 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var175))).into_val(env))
                                                                                                                var16 = var176;
                                                                                                                let var177 = self.func49(env, 1050523);
                                                                                                                let var178 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var177))).into_val(env))
                                                                                                                slot_var0_24_i64 = var178 as i64;
                                                                                                                slot_var0_16_i64 = var16 as i64;
                                                                                                                slot_var0_8_i64 = var15 as i64;
                                                                                                                var3 = 0;
                                                                                                                'label52: loop {
                                                                                                                    'label53: loop {
                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                            break 'label53;
                                                                                                                        }
                                                                                                                        var3 = 0;
                                                                                                                        'label54: loop {
                                                                                                                            'label55: loop {
                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                    break 'label54;
                                                                                                                                }
                                                                                                                                let var179 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var179 as u64);
                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                continue 'label55;
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        let var180 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                        var15 = var180;
                                                                                                                        let var181 = self.func49(env, 1050555);
                                                                                                                        let var182 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var181))).into_val(env))
                                                                                                                        var16 = var182;
                                                                                                                        let var183 = self.func49(env, 1050587);
                                                                                                                        let var184 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var183))).into_val(env))
                                                                                                                        var17 = var184;
                                                                                                                        let var185 = self.func49(env, 1050619);
                                                                                                                        let var186 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var185))).into_val(env))
                                                                                                                        slot_var0_24_i64 = var186 as i64;
                                                                                                                        slot_var0_16_i64 = var17 as i64;
                                                                                                                        slot_var0_8_i64 = var16 as i64;
                                                                                                                        var3 = 0;
                                                                                                                        'label56: loop {
                                                                                                                            'label57: loop {
                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                    break 'label57;
                                                                                                                                }
                                                                                                                                var3 = 0;
                                                                                                                                'label58: loop {
                                                                                                                                    'label59: loop {
                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                            break 'label58;
                                                                                                                                        }
                                                                                                                                        let var187 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var187 as u64);
                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                        continue 'label59;
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                let var188 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                var16 = var188;
                                                                                                                                let var189 = self.func49(env, 1050651);
                                                                                                                                let var190 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var189))).into_val(env))
                                                                                                                                var17 = var190;
                                                                                                                                let var191 = self.func49(env, 1050683);
                                                                                                                                let var192 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var191))).into_val(env))
                                                                                                                                var18 = var192;
                                                                                                                                let var193 = self.func49(env, 1050715);
                                                                                                                                let var194 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var193))).into_val(env))
                                                                                                                                slot_var0_24_i64 = var194 as i64;
                                                                                                                                slot_var0_16_i64 = var18 as i64;
                                                                                                                                slot_var0_8_i64 = var17 as i64;
                                                                                                                                var3 = 0;
                                                                                                                                'label60: loop {
                                                                                                                                    'label61: loop {
                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                            break 'label61;
                                                                                                                                        }
                                                                                                                                        var3 = 0;
                                                                                                                                        'label62: loop {
                                                                                                                                            'label63: loop {
                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                    break 'label62;
                                                                                                                                                }
                                                                                                                                                let var195 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var195 as u64);
                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                continue 'label63;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        let var196 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                        var17 = var196;
                                                                                                                                        let var197 = self.func49(env, 1050747);
                                                                                                                                        let var198 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var197))).into_val(env))
                                                                                                                                        var18 = var198;
                                                                                                                                        let var199 = self.func49(env, 1050779);
                                                                                                                                        let var200 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var199))).into_val(env))
                                                                                                                                        var19 = var200;
                                                                                                                                        let var201 = self.func49(env, 1050811);
                                                                                                                                        let var202 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var201))).into_val(env))
                                                                                                                                        slot_var0_24_i64 = var202 as i64;
                                                                                                                                        slot_var0_16_i64 = var19 as i64;
                                                                                                                                        slot_var0_8_i64 = var18 as i64;
                                                                                                                                        var3 = 0;
                                                                                                                                        'label64: loop {
                                                                                                                                            'label65: loop {
                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                    break 'label65;
                                                                                                                                                }
                                                                                                                                                var3 = 0;
                                                                                                                                                'label66: loop {
                                                                                                                                                    'label67: loop {
                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                            break 'label66;
                                                                                                                                                        }
                                                                                                                                                        let var203 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var203 as u64);
                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                        continue 'label67;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                let var204 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                var18 = var204;
                                                                                                                                                let var205 = self.func49(env, 1050843);
                                                                                                                                                let var206 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var205))).into_val(env))
                                                                                                                                                var19 = var206;
                                                                                                                                                let var207 = self.func49(env, 1050875);
                                                                                                                                                let var208 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var207))).into_val(env))
                                                                                                                                                var20 = var208;
                                                                                                                                                let var209 = self.func49(env, 1050907);
                                                                                                                                                let var210 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var209))).into_val(env))
                                                                                                                                                slot_var0_24_i64 = var210 as i64;
                                                                                                                                                slot_var0_16_i64 = var20 as i64;
                                                                                                                                                slot_var0_8_i64 = var19 as i64;
                                                                                                                                                var3 = 0;
                                                                                                                                                'label68: loop {
                                                                                                                                                    'label69: loop {
                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                            break 'label69;
                                                                                                                                                        }
                                                                                                                                                        var3 = 0;
                                                                                                                                                        'label70: loop {
                                                                                                                                                            'label71: loop {
                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                    break 'label70;
                                                                                                                                                                }
                                                                                                                                                                let var211 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var211 as u64);
                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                continue 'label71;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        let var212 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                        var19 = var212;
                                                                                                                                                        let var213 = self.func49(env, 1050939);
                                                                                                                                                        let var214 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var213))).into_val(env))
                                                                                                                                                        var20 = var214;
                                                                                                                                                        let var215 = self.func49(env, 1050971);
                                                                                                                                                        let var216 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var215))).into_val(env))
                                                                                                                                                        var21 = var216;
                                                                                                                                                        let var217 = self.func49(env, 1051003);
                                                                                                                                                        let var218 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var217))).into_val(env))
                                                                                                                                                        slot_var0_24_i64 = var218 as i64;
                                                                                                                                                        slot_var0_16_i64 = var21 as i64;
                                                                                                                                                        slot_var0_8_i64 = var20 as i64;
                                                                                                                                                        var3 = 0;
                                                                                                                                                        'label72: loop {
                                                                                                                                                            'label73: loop {
                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                    break 'label73;
                                                                                                                                                                }
                                                                                                                                                                var3 = 0;
                                                                                                                                                                'label74: loop {
                                                                                                                                                                    'label75: loop {
                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                            break 'label74;
                                                                                                                                                                        }
                                                                                                                                                                        let var219 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var219 as u64);
                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                        continue 'label75;
                                                                                                                                                                        break;
                                                                                                                                                                    }
                                                                                                                                                                    break;
                                                                                                                                                                }
                                                                                                                                                                let var220 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                var20 = var220;
                                                                                                                                                                let var221 = self.func49(env, 1051035);
                                                                                                                                                                let var222 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var221))).into_val(env))
                                                                                                                                                                var21 = var222;
                                                                                                                                                                let var223 = self.func49(env, 1051067);
                                                                                                                                                                let var224 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var223))).into_val(env))
                                                                                                                                                                var22 = var224;
                                                                                                                                                                let var225 = self.func49(env, 1051099);
                                                                                                                                                                let var226 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var225))).into_val(env))
                                                                                                                                                                slot_var0_24_i64 = var226 as i64;
                                                                                                                                                                slot_var0_16_i64 = var22 as i64;
                                                                                                                                                                slot_var0_8_i64 = var21 as i64;
                                                                                                                                                                var3 = 0;
                                                                                                                                                                'label76: loop {
                                                                                                                                                                    'label77: loop {
                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                            break 'label77;
                                                                                                                                                                        }
                                                                                                                                                                        var3 = 0;
                                                                                                                                                                        'label78: loop {
                                                                                                                                                                            'label79: loop {
                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                    break 'label78;
                                                                                                                                                                                }
                                                                                                                                                                                let var227 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var227 as u64);
                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                continue 'label79;
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        let var228 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                        var21 = var228;
                                                                                                                                                                        let var229 = self.func49(env, 1051131);
                                                                                                                                                                        let var230 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var229))).into_val(env))
                                                                                                                                                                        var22 = var230;
                                                                                                                                                                        let var231 = self.func49(env, 1051163);
                                                                                                                                                                        let var232 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var231))).into_val(env))
                                                                                                                                                                        var23 = var232;
                                                                                                                                                                        let var233 = self.func49(env, 1051195);
                                                                                                                                                                        let var234 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var233))).into_val(env))
                                                                                                                                                                        slot_var0_24_i64 = var234 as i64;
                                                                                                                                                                        slot_var0_16_i64 = var23 as i64;
                                                                                                                                                                        slot_var0_8_i64 = var22 as i64;
                                                                                                                                                                        var3 = 0;
                                                                                                                                                                        'label80: loop {
                                                                                                                                                                            'label81: loop {
                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                    break 'label81;
                                                                                                                                                                                }
                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                'label82: loop {
                                                                                                                                                                                    'label83: loop {
                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                            break 'label82;
                                                                                                                                                                                        }
                                                                                                                                                                                        let var235 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var235 as u64);
                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                        continue 'label83;
                                                                                                                                                                                        break;
                                                                                                                                                                                    }
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                let var236 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                var22 = var236;
                                                                                                                                                                                let var237 = self.func49(env, 1051227);
                                                                                                                                                                                let var238 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var237))).into_val(env))
                                                                                                                                                                                var23 = var238;
                                                                                                                                                                                let var239 = self.func49(env, 1051259);
                                                                                                                                                                                let var240 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var239))).into_val(env))
                                                                                                                                                                                var24 = var240;
                                                                                                                                                                                let var241 = self.func49(env, 1051291);
                                                                                                                                                                                let var242 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var241))).into_val(env))
                                                                                                                                                                                slot_var0_24_i64 = var242 as i64;
                                                                                                                                                                                slot_var0_16_i64 = var24 as i64;
                                                                                                                                                                                slot_var0_8_i64 = var23 as i64;
                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                'label84: loop {
                                                                                                                                                                                    'label85: loop {
                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                            break 'label85;
                                                                                                                                                                                        }
                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                        'label86: loop {
                                                                                                                                                                                            'label87: loop {
                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                    break 'label86;
                                                                                                                                                                                                }
                                                                                                                                                                                                let var243 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var243 as u64);
                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                continue 'label87;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            break;
                                                                                                                                                                                        }
                                                                                                                                                                                        let var244 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                        var23 = var244;
                                                                                                                                                                                        let var245 = self.func49(env, 1051323);
                                                                                                                                                                                        let var246 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var245))).into_val(env))
                                                                                                                                                                                        var24 = var246;
                                                                                                                                                                                        let var247 = self.func49(env, 1051355);
                                                                                                                                                                                        let var248 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var247))).into_val(env))
                                                                                                                                                                                        var25 = var248;
                                                                                                                                                                                        let var249 = self.func49(env, 1051387);
                                                                                                                                                                                        let var250 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var249))).into_val(env))
                                                                                                                                                                                        slot_var0_24_i64 = var250 as i64;
                                                                                                                                                                                        slot_var0_16_i64 = var25 as i64;
                                                                                                                                                                                        slot_var0_8_i64 = var24 as i64;
                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                        'label88: loop {
                                                                                                                                                                                            'label89: loop {
                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                    break 'label89;
                                                                                                                                                                                                }
                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                'label90: loop {
                                                                                                                                                                                                    'label91: loop {
                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                            break 'label90;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        let var251 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var251 as u64);
                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                        continue 'label91;
                                                                                                                                                                                                        break;
                                                                                                                                                                                                    }
                                                                                                                                                                                                    break;
                                                                                                                                                                                                }
                                                                                                                                                                                                let var252 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                var24 = var252;
                                                                                                                                                                                                let var253 = self.func49(env, 1051419);
                                                                                                                                                                                                let var254 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var253))).into_val(env))
                                                                                                                                                                                                var25 = var254;
                                                                                                                                                                                                let var255 = self.func49(env, 1051451);
                                                                                                                                                                                                let var256 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var255))).into_val(env))
                                                                                                                                                                                                var26 = var256;
                                                                                                                                                                                                let var257 = self.func49(env, 1051515);
                                                                                                                                                                                                let var258 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var257))).into_val(env))
                                                                                                                                                                                                slot_var0_24_i64 = var258 as i64;
                                                                                                                                                                                                slot_var0_16_i64 = var26 as i64;
                                                                                                                                                                                                slot_var0_8_i64 = var25 as i64;
                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                'label92: loop {
                                                                                                                                                                                                    'label93: loop {
                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                            break 'label93;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                        'label94: loop {
                                                                                                                                                                                                            'label95: loop {
                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                    break 'label94;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                let var259 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var259 as u64);
                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                continue 'label95;
                                                                                                                                                                                                                break;
                                                                                                                                                                                                            }
                                                                                                                                                                                                            break;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        let var260 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                        var25 = var260;
                                                                                                                                                                                                        let var261 = self.func49(env, 1051547);
                                                                                                                                                                                                        let var262 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var261))).into_val(env))
                                                                                                                                                                                                        var26 = var262;
                                                                                                                                                                                                        let var263 = self.func49(env, 1051579);
                                                                                                                                                                                                        let var264 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var263))).into_val(env))
                                                                                                                                                                                                        var27 = var264;
                                                                                                                                                                                                        let var265 = self.func49(env, 1051611);
                                                                                                                                                                                                        let var266 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var265))).into_val(env))
                                                                                                                                                                                                        slot_var0_24_i64 = var266 as i64;
                                                                                                                                                                                                        slot_var0_16_i64 = var27 as i64;
                                                                                                                                                                                                        slot_var0_8_i64 = var26 as i64;
                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                        'label96: loop {
                                                                                                                                                                                                            'label97: loop {
                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                    break 'label97;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                'label98: loop {
                                                                                                                                                                                                                    'label99: loop {
                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                            break 'label98;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        let var267 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var267 as u64);
                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                        continue 'label99;
                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                let var268 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                var26 = var268;
                                                                                                                                                                                                                let var269 = self.func49(env, 1051643);
                                                                                                                                                                                                                let var270 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var269))).into_val(env))
                                                                                                                                                                                                                var27 = var270;
                                                                                                                                                                                                                let var271 = self.func49(env, 1051675);
                                                                                                                                                                                                                let var272 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var271))).into_val(env))
                                                                                                                                                                                                                var28 = var272;
                                                                                                                                                                                                                let var273 = self.func49(env, 1051707);
                                                                                                                                                                                                                let var274 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var273))).into_val(env))
                                                                                                                                                                                                                slot_var0_24_i64 = var274 as i64;
                                                                                                                                                                                                                slot_var0_16_i64 = var28 as i64;
                                                                                                                                                                                                                slot_var0_8_i64 = var27 as i64;
                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                'label100: loop {
                                                                                                                                                                                                                    'label101: loop {
                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                            break 'label101;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                        'label102: loop {
                                                                                                                                                                                                                            'label103: loop {
                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                    break 'label102;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                let var275 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var275 as u64);
                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                continue 'label103;
                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        let var276 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                        var27 = var276;
                                                                                                                                                                                                                        let var277 = self.func49(env, 1051739);
                                                                                                                                                                                                                        let var278 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var277))).into_val(env))
                                                                                                                                                                                                                        var28 = var278;
                                                                                                                                                                                                                        let var279 = self.func49(env, 1051771);
                                                                                                                                                                                                                        let var280 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var279))).into_val(env))
                                                                                                                                                                                                                        var29 = var280;
                                                                                                                                                                                                                        let var281 = self.func49(env, 1051803);
                                                                                                                                                                                                                        let var282 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var281))).into_val(env))
                                                                                                                                                                                                                        slot_var0_24_i64 = var282 as i64;
                                                                                                                                                                                                                        slot_var0_16_i64 = var29 as i64;
                                                                                                                                                                                                                        slot_var0_8_i64 = var28 as i64;
                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                        'label104: loop {
                                                                                                                                                                                                                            'label105: loop {
                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                    break 'label105;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                'label106: loop {
                                                                                                                                                                                                                                    'label107: loop {
                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                            break 'label106;
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        let var283 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var283 as u64);
                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                        continue 'label107;
                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                let var284 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                var28 = var284;
                                                                                                                                                                                                                                let var285 = self.func49(env, 1051835);
                                                                                                                                                                                                                                let var286 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var285))).into_val(env))
                                                                                                                                                                                                                                var29 = var286;
                                                                                                                                                                                                                                let var287 = self.func49(env, 1051867);
                                                                                                                                                                                                                                let var288 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var287))).into_val(env))
                                                                                                                                                                                                                                var30 = var288;
                                                                                                                                                                                                                                let var289 = self.func49(env, 1051899);
                                                                                                                                                                                                                                let var290 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var289))).into_val(env))
                                                                                                                                                                                                                                slot_var0_24_i64 = var290 as i64;
                                                                                                                                                                                                                                slot_var0_16_i64 = var30 as i64;
                                                                                                                                                                                                                                slot_var0_8_i64 = var29 as i64;
                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                'label108: loop {
                                                                                                                                                                                                                                    'label109: loop {
                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                            break 'label109;
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                        'label110: loop {
                                                                                                                                                                                                                                            'label111: loop {
                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                    break 'label110;
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                let var291 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var291 as u64);
                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                continue 'label111;
                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        let var292 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                        var29 = var292;
                                                                                                                                                                                                                                        let var293 = self.func49(env, 1051931);
                                                                                                                                                                                                                                        let var294 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var293))).into_val(env))
                                                                                                                                                                                                                                        var30 = var294;
                                                                                                                                                                                                                                        let var295 = self.func49(env, 1051963);
                                                                                                                                                                                                                                        let var296 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var295))).into_val(env))
                                                                                                                                                                                                                                        var31 = var296;
                                                                                                                                                                                                                                        let var297 = self.func49(env, 1051995);
                                                                                                                                                                                                                                        let var298 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var297))).into_val(env))
                                                                                                                                                                                                                                        slot_var0_24_i64 = var298 as i64;
                                                                                                                                                                                                                                        slot_var0_16_i64 = var31 as i64;
                                                                                                                                                                                                                                        slot_var0_8_i64 = var30 as i64;
                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                        'label112: loop {
                                                                                                                                                                                                                                            'label113: loop {
                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                    break 'label113;
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                'label114: loop {
                                                                                                                                                                                                                                                    'label115: loop {
                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                            break 'label114;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        let var299 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var299 as u64);
                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                        continue 'label115;
                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                let var300 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                var30 = var300;
                                                                                                                                                                                                                                                let var301 = self.func49(env, 1052027);
                                                                                                                                                                                                                                                let var302 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var301))).into_val(env))
                                                                                                                                                                                                                                                var31 = var302;
                                                                                                                                                                                                                                                let var303 = self.func49(env, 1052059);
                                                                                                                                                                                                                                                let var304 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var303))).into_val(env))
                                                                                                                                                                                                                                                var32 = var304;
                                                                                                                                                                                                                                                let var305 = self.func49(env, 1052091);
                                                                                                                                                                                                                                                let var306 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var305))).into_val(env))
                                                                                                                                                                                                                                                slot_var0_24_i64 = var306 as i64;
                                                                                                                                                                                                                                                slot_var0_16_i64 = var32 as i64;
                                                                                                                                                                                                                                                slot_var0_8_i64 = var31 as i64;
                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                'label116: loop {
                                                                                                                                                                                                                                                    'label117: loop {
                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                            break 'label117;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                        'label118: loop {
                                                                                                                                                                                                                                                            'label119: loop {
                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                    break 'label118;
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                let var307 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var307 as u64);
                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                continue 'label119;
                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        let var308 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                        var31 = var308;
                                                                                                                                                                                                                                                        let var309 = self.func49(env, 1052123);
                                                                                                                                                                                                                                                        let var310 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var309))).into_val(env))
                                                                                                                                                                                                                                                        var32 = var310;
                                                                                                                                                                                                                                                        let var311 = self.func49(env, 1052155);
                                                                                                                                                                                                                                                        let var312 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var311))).into_val(env))
                                                                                                                                                                                                                                                        var33 = var312;
                                                                                                                                                                                                                                                        let var313 = self.func49(env, 1052187);
                                                                                                                                                                                                                                                        let var314 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var313))).into_val(env))
                                                                                                                                                                                                                                                        slot_var0_24_i64 = var314 as i64;
                                                                                                                                                                                                                                                        slot_var0_16_i64 = var33 as i64;
                                                                                                                                                                                                                                                        slot_var0_8_i64 = var32 as i64;
                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                        'label120: loop {
                                                                                                                                                                                                                                                            'label121: loop {
                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                    break 'label121;
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                'label122: loop {
                                                                                                                                                                                                                                                                    'label123: loop {
                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                            break 'label122;
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        let var315 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var315 as u64);
                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                        continue 'label123;
                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                let var316 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                var32 = var316;
                                                                                                                                                                                                                                                                let var317 = self.func49(env, 1052219);
                                                                                                                                                                                                                                                                let var318 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var317))).into_val(env))
                                                                                                                                                                                                                                                                var33 = var318;
                                                                                                                                                                                                                                                                let var319 = self.func49(env, 1052251);
                                                                                                                                                                                                                                                                let var320 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var319))).into_val(env))
                                                                                                                                                                                                                                                                var34 = var320;
                                                                                                                                                                                                                                                                let var321 = self.func49(env, 1052283);
                                                                                                                                                                                                                                                                let var322 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var321))).into_val(env))
                                                                                                                                                                                                                                                                slot_var0_24_i64 = var322 as i64;
                                                                                                                                                                                                                                                                slot_var0_16_i64 = var34 as i64;
                                                                                                                                                                                                                                                                slot_var0_8_i64 = var33 as i64;
                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                'label124: loop {
                                                                                                                                                                                                                                                                    'label125: loop {
                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                            break 'label125;
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                        'label126: loop {
                                                                                                                                                                                                                                                                            'label127: loop {
                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                    break 'label126;
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                let var323 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var323 as u64);
                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                continue 'label127;
                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        let var324 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                        var33 = var324;
                                                                                                                                                                                                                                                                        let var325 = self.func49(env, 1052315);
                                                                                                                                                                                                                                                                        let var326 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var325))).into_val(env))
                                                                                                                                                                                                                                                                        var34 = var326;
                                                                                                                                                                                                                                                                        let var327 = self.func49(env, 1052347);
                                                                                                                                                                                                                                                                        let var328 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var327))).into_val(env))
                                                                                                                                                                                                                                                                        var35 = var328;
                                                                                                                                                                                                                                                                        let var329 = self.func49(env, 1052379);
                                                                                                                                                                                                                                                                        let var330 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var329))).into_val(env))
                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var330 as i64;
                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var35 as i64;
                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var34 as i64;
                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                        'label128: loop {
                                                                                                                                                                                                                                                                            'label129: loop {
                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                    break 'label129;
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                'label130: loop {
                                                                                                                                                                                                                                                                                    'label131: loop {
                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                            break 'label130;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        let var331 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var331 as u64);
                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                        continue 'label131;
                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                let var332 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                var34 = var332;
                                                                                                                                                                                                                                                                                let var333 = self.func49(env, 1052411);
                                                                                                                                                                                                                                                                                let var334 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var333))).into_val(env))
                                                                                                                                                                                                                                                                                var35 = var334;
                                                                                                                                                                                                                                                                                let var335 = self.func49(env, 1052443);
                                                                                                                                                                                                                                                                                let var336 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var335))).into_val(env))
                                                                                                                                                                                                                                                                                var36 = var336;
                                                                                                                                                                                                                                                                                let var337 = self.func49(env, 1052475);
                                                                                                                                                                                                                                                                                let var338 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var337))).into_val(env))
                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var338 as i64;
                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var36 as i64;
                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var35 as i64;
                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                'label132: loop {
                                                                                                                                                                                                                                                                                    'label133: loop {
                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                            break 'label133;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                        'label134: loop {
                                                                                                                                                                                                                                                                                            'label135: loop {
                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                    break 'label134;
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                let var339 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var339 as u64);
                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                continue 'label135;
                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        let var340 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                        var35 = var340;
                                                                                                                                                                                                                                                                                        let var341 = self.func49(env, 1052507);
                                                                                                                                                                                                                                                                                        let var342 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var341))).into_val(env))
                                                                                                                                                                                                                                                                                        var36 = var342;
                                                                                                                                                                                                                                                                                        let var343 = self.func49(env, 1052539);
                                                                                                                                                                                                                                                                                        let var344 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var343))).into_val(env))
                                                                                                                                                                                                                                                                                        var37 = var344;
                                                                                                                                                                                                                                                                                        let var345 = self.func49(env, 1052571);
                                                                                                                                                                                                                                                                                        let var346 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var345))).into_val(env))
                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var346 as i64;
                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var37 as i64;
                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var36 as i64;
                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                        'label136: loop {
                                                                                                                                                                                                                                                                                            'label137: loop {
                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                    break 'label137;
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                'label138: loop {
                                                                                                                                                                                                                                                                                                    'label139: loop {
                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                            break 'label138;
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        let var347 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var347 as u64);
                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                        continue 'label139;
                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                let var348 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                var36 = var348;
                                                                                                                                                                                                                                                                                                let var349 = self.func49(env, 1052603);
                                                                                                                                                                                                                                                                                                let var350 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var349))).into_val(env))
                                                                                                                                                                                                                                                                                                var37 = var350;
                                                                                                                                                                                                                                                                                                let var351 = self.func49(env, 1052635);
                                                                                                                                                                                                                                                                                                let var352 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var351))).into_val(env))
                                                                                                                                                                                                                                                                                                var38 = var352;
                                                                                                                                                                                                                                                                                                let var353 = self.func49(env, 1052667);
                                                                                                                                                                                                                                                                                                let var354 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var353))).into_val(env))
                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var354 as i64;
                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var38 as i64;
                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var37 as i64;
                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                'label140: loop {
                                                                                                                                                                                                                                                                                                    'label141: loop {
                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                            break 'label141;
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                        'label142: loop {
                                                                                                                                                                                                                                                                                                            'label143: loop {
                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                    break 'label142;
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                let var355 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var355 as u64);
                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                continue 'label143;
                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        let var356 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                        var37 = var356;
                                                                                                                                                                                                                                                                                                        let var357 = self.func49(env, 1052699);
                                                                                                                                                                                                                                                                                                        let var358 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var357))).into_val(env))
                                                                                                                                                                                                                                                                                                        var38 = var358;
                                                                                                                                                                                                                                                                                                        let var359 = self.func49(env, 1052731);
                                                                                                                                                                                                                                                                                                        let var360 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var359))).into_val(env))
                                                                                                                                                                                                                                                                                                        var39 = var360;
                                                                                                                                                                                                                                                                                                        let var361 = self.func49(env, 1052763);
                                                                                                                                                                                                                                                                                                        let var362 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var361))).into_val(env))
                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var362 as i64;
                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var39 as i64;
                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var38 as i64;
                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                        'label144: loop {
                                                                                                                                                                                                                                                                                                            'label145: loop {
                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                    break 'label145;
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                'label146: loop {
                                                                                                                                                                                                                                                                                                                    'label147: loop {
                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                            break 'label146;
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        let var363 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var363 as u64);
                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                        continue 'label147;
                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                let var364 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                var38 = var364;
                                                                                                                                                                                                                                                                                                                let var365 = self.func49(env, 1052795);
                                                                                                                                                                                                                                                                                                                let var366 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var365))).into_val(env))
                                                                                                                                                                                                                                                                                                                var39 = var366;
                                                                                                                                                                                                                                                                                                                let var367 = self.func49(env, 1052827);
                                                                                                                                                                                                                                                                                                                let var368 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var367))).into_val(env))
                                                                                                                                                                                                                                                                                                                var40 = var368;
                                                                                                                                                                                                                                                                                                                let var369 = self.func49(env, 1052859);
                                                                                                                                                                                                                                                                                                                let var370 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var369))).into_val(env))
                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var370 as i64;
                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var40 as i64;
                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var39 as i64;
                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                'label148: loop {
                                                                                                                                                                                                                                                                                                                    'label149: loop {
                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                            break 'label149;
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                        'label150: loop {
                                                                                                                                                                                                                                                                                                                            'label151: loop {
                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                    break 'label150;
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                let var371 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var371 as u64);
                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                continue 'label151;
                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        let var372 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                        var39 = var372;
                                                                                                                                                                                                                                                                                                                        let var373 = self.func49(env, 1052891);
                                                                                                                                                                                                                                                                                                                        let var374 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var373))).into_val(env))
                                                                                                                                                                                                                                                                                                                        var40 = var374;
                                                                                                                                                                                                                                                                                                                        let var375 = self.func49(env, 1052923);
                                                                                                                                                                                                                                                                                                                        let var376 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var375))).into_val(env))
                                                                                                                                                                                                                                                                                                                        var41 = var376;
                                                                                                                                                                                                                                                                                                                        let var377 = self.func49(env, 1052955);
                                                                                                                                                                                                                                                                                                                        let var378 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var377))).into_val(env))
                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var378 as i64;
                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var41 as i64;
                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var40 as i64;
                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                        'label152: loop {
                                                                                                                                                                                                                                                                                                                            'label153: loop {
                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                    break 'label153;
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                'label154: loop {
                                                                                                                                                                                                                                                                                                                                    'label155: loop {
                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                            break 'label154;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        let var379 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var379 as u64);
                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                        continue 'label155;
                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                let var380 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                var40 = var380;
                                                                                                                                                                                                                                                                                                                                let var381 = self.func49(env, 1052987);
                                                                                                                                                                                                                                                                                                                                let var382 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var381))).into_val(env))
                                                                                                                                                                                                                                                                                                                                var41 = var382;
                                                                                                                                                                                                                                                                                                                                let var383 = self.func49(env, 1053019);
                                                                                                                                                                                                                                                                                                                                let var384 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var383))).into_val(env))
                                                                                                                                                                                                                                                                                                                                var42 = var384;
                                                                                                                                                                                                                                                                                                                                let var385 = self.func49(env, 1053051);
                                                                                                                                                                                                                                                                                                                                let var386 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var385))).into_val(env))
                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var386 as i64;
                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var42 as i64;
                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var41 as i64;
                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                'label156: loop {
                                                                                                                                                                                                                                                                                                                                    'label157: loop {
                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                            break 'label157;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                        'label158: loop {
                                                                                                                                                                                                                                                                                                                                            'label159: loop {
                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                    break 'label158;
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                let var387 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var387 as u64);
                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                continue 'label159;
                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        let var388 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                        var41 = var388;
                                                                                                                                                                                                                                                                                                                                        let var389 = self.func49(env, 1053083);
                                                                                                                                                                                                                                                                                                                                        let var390 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var389))).into_val(env))
                                                                                                                                                                                                                                                                                                                                        var42 = var390;
                                                                                                                                                                                                                                                                                                                                        let var391 = self.func49(env, 1053115);
                                                                                                                                                                                                                                                                                                                                        let var392 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var391))).into_val(env))
                                                                                                                                                                                                                                                                                                                                        var43 = var392;
                                                                                                                                                                                                                                                                                                                                        let var393 = self.func49(env, 1053147);
                                                                                                                                                                                                                                                                                                                                        let var394 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var393))).into_val(env))
                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var394 as i64;
                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var43 as i64;
                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var42 as i64;
                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                        'label160: loop {
                                                                                                                                                                                                                                                                                                                                            'label161: loop {
                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                    break 'label161;
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                'label162: loop {
                                                                                                                                                                                                                                                                                                                                                    'label163: loop {
                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                            break 'label162;
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        let var395 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var395 as u64);
                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                        continue 'label163;
                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                let var396 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                var42 = var396;
                                                                                                                                                                                                                                                                                                                                                let var397 = self.func49(env, 1053179);
                                                                                                                                                                                                                                                                                                                                                let var398 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var397))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                var43 = var398;
                                                                                                                                                                                                                                                                                                                                                let var399 = self.func49(env, 1053211);
                                                                                                                                                                                                                                                                                                                                                let var400 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var399))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                var44 = var400;
                                                                                                                                                                                                                                                                                                                                                let var401 = self.func49(env, 1053243);
                                                                                                                                                                                                                                                                                                                                                let var402 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var401))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var402 as i64;
                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var44 as i64;
                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var43 as i64;
                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                'label164: loop {
                                                                                                                                                                                                                                                                                                                                                    'label165: loop {
                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                            break 'label165;
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                        'label166: loop {
                                                                                                                                                                                                                                                                                                                                                            'label167: loop {
                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                    break 'label166;
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                let var403 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var403 as u64);
                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                continue 'label167;
                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        let var404 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                        var43 = var404;
                                                                                                                                                                                                                                                                                                                                                        let var405 = self.func49(env, 1053275);
                                                                                                                                                                                                                                                                                                                                                        let var406 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var405))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                        var44 = var406;
                                                                                                                                                                                                                                                                                                                                                        let var407 = self.func49(env, 1053307);
                                                                                                                                                                                                                                                                                                                                                        let var408 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var407))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                        var45 = var408;
                                                                                                                                                                                                                                                                                                                                                        let var409 = self.func49(env, 1053339);
                                                                                                                                                                                                                                                                                                                                                        let var410 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var409))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var410 as i64;
                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var45 as i64;
                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var44 as i64;
                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                        'label168: loop {
                                                                                                                                                                                                                                                                                                                                                            'label169: loop {
                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                    break 'label169;
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                'label170: loop {
                                                                                                                                                                                                                                                                                                                                                                    'label171: loop {
                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                            break 'label170;
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                        let var411 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var411 as u64);
                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                        continue 'label171;
                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                let var412 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                var44 = var412;
                                                                                                                                                                                                                                                                                                                                                                let var413 = self.func49(env, 1053371);
                                                                                                                                                                                                                                                                                                                                                                let var414 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var413))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                var45 = var414;
                                                                                                                                                                                                                                                                                                                                                                let var415 = self.func49(env, 1053403);
                                                                                                                                                                                                                                                                                                                                                                let var416 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var415))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                var46 = var416;
                                                                                                                                                                                                                                                                                                                                                                let var417 = self.func49(env, 1053435);
                                                                                                                                                                                                                                                                                                                                                                let var418 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var417))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var418 as i64;
                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var46 as i64;
                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var45 as i64;
                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                'label172: loop {
                                                                                                                                                                                                                                                                                                                                                                    'label173: loop {
                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                            break 'label173;
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                        'label174: loop {
                                                                                                                                                                                                                                                                                                                                                                            'label175: loop {
                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                    break 'label174;
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                let var419 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var419 as u64);
                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                continue 'label175;
                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                        let var420 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                        var45 = var420;
                                                                                                                                                                                                                                                                                                                                                                        let var421 = self.func49(env, 1053467);
                                                                                                                                                                                                                                                                                                                                                                        let var422 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var421))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                        var46 = var422;
                                                                                                                                                                                                                                                                                                                                                                        let var423 = self.func49(env, 1053499);
                                                                                                                                                                                                                                                                                                                                                                        let var424 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var423))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                        var47 = var424;
                                                                                                                                                                                                                                                                                                                                                                        let var425 = self.func49(env, 1053531);
                                                                                                                                                                                                                                                                                                                                                                        let var426 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var425))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var426 as i64;
                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var47 as i64;
                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var46 as i64;
                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                        'label176: loop {
                                                                                                                                                                                                                                                                                                                                                                            'label177: loop {
                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                    break 'label177;
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                'label178: loop {
                                                                                                                                                                                                                                                                                                                                                                                    'label179: loop {
                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                            break 'label178;
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                        let var427 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var427 as u64);
                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                        continue 'label179;
                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                let var428 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                var46 = var428;
                                                                                                                                                                                                                                                                                                                                                                                let var429 = self.func49(env, 1053563);
                                                                                                                                                                                                                                                                                                                                                                                let var430 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var429))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                var47 = var430;
                                                                                                                                                                                                                                                                                                                                                                                let var431 = self.func49(env, 1053595);
                                                                                                                                                                                                                                                                                                                                                                                let var432 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var431))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                var48 = var432;
                                                                                                                                                                                                                                                                                                                                                                                let var433 = self.func49(env, 1053627);
                                                                                                                                                                                                                                                                                                                                                                                let var434 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var433))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var434 as i64;
                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var48 as i64;
                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var47 as i64;
                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                'label180: loop {
                                                                                                                                                                                                                                                                                                                                                                                    'label181: loop {
                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                            break 'label181;
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                        'label182: loop {
                                                                                                                                                                                                                                                                                                                                                                                            'label183: loop {
                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                    break 'label182;
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                let var435 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var435 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                continue 'label183;
                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                        let var436 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                        var47 = var436;
                                                                                                                                                                                                                                                                                                                                                                                        let var437 = self.func49(env, 1053659);
                                                                                                                                                                                                                                                                                                                                                                                        let var438 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var437))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                        var48 = var438;
                                                                                                                                                                                                                                                                                                                                                                                        let var439 = self.func49(env, 1053691);
                                                                                                                                                                                                                                                                                                                                                                                        let var440 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var439))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                        var49 = var440;
                                                                                                                                                                                                                                                                                                                                                                                        let var441 = self.func49(env, 1053723);
                                                                                                                                                                                                                                                                                                                                                                                        let var442 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var441))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var442 as i64;
                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var49 as i64;
                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var48 as i64;
                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                        'label184: loop {
                                                                                                                                                                                                                                                                                                                                                                                            'label185: loop {
                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                    break 'label185;
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                'label186: loop {
                                                                                                                                                                                                                                                                                                                                                                                                    'label187: loop {
                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                            break 'label186;
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        let var443 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var443 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label187;
                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                let var444 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                var48 = var444;
                                                                                                                                                                                                                                                                                                                                                                                                let var445 = self.func49(env, 1053755);
                                                                                                                                                                                                                                                                                                                                                                                                let var446 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var445))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                var49 = var446;
                                                                                                                                                                                                                                                                                                                                                                                                let var447 = self.func49(env, 1053787);
                                                                                                                                                                                                                                                                                                                                                                                                let var448 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var447))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                var50 = var448;
                                                                                                                                                                                                                                                                                                                                                                                                let var449 = self.func49(env, 1053819);
                                                                                                                                                                                                                                                                                                                                                                                                let var450 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var449))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var450 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var50 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var49 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                'label188: loop {
                                                                                                                                                                                                                                                                                                                                                                                                    'label189: loop {
                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                            break 'label189;
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                        'label190: loop {
                                                                                                                                                                                                                                                                                                                                                                                                            'label191: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label190;
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                let var451 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var451 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label191;
                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        let var452 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                        var49 = var452;
                                                                                                                                                                                                                                                                                                                                                                                                        let var453 = self.func49(env, 1053851);
                                                                                                                                                                                                                                                                                                                                                                                                        let var454 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var453))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                        var50 = var454;
                                                                                                                                                                                                                                                                                                                                                                                                        let var455 = self.func49(env, 1053883);
                                                                                                                                                                                                                                                                                                                                                                                                        let var456 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var455))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                        var51 = var456;
                                                                                                                                                                                                                                                                                                                                                                                                        let var457 = self.func49(env, 1053915);
                                                                                                                                                                                                                                                                                                                                                                                                        let var458 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var457))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var458 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var51 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var50 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                        'label192: loop {
                                                                                                                                                                                                                                                                                                                                                                                                            'label193: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label193;
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                'label194: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                    'label195: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label194;
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                        let var459 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var459 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label195;
                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                let var460 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                var50 = var460;
                                                                                                                                                                                                                                                                                                                                                                                                                let var461 = self.func49(env, 1053947);
                                                                                                                                                                                                                                                                                                                                                                                                                let var462 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var461))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                var51 = var462;
                                                                                                                                                                                                                                                                                                                                                                                                                let var463 = self.func49(env, 1053979);
                                                                                                                                                                                                                                                                                                                                                                                                                let var464 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var463))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                var52 = var464;
                                                                                                                                                                                                                                                                                                                                                                                                                let var465 = self.func49(env, 1054011);
                                                                                                                                                                                                                                                                                                                                                                                                                let var466 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var465))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var466 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var52 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var51 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                'label196: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                    'label197: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label197;
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                        'label198: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                            'label199: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label198;
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                let var467 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var467 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label199;
                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                        let var468 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                        var51 = var468;
                                                                                                                                                                                                                                                                                                                                                                                                                        let var469 = self.func49(env, 1054043);
                                                                                                                                                                                                                                                                                                                                                                                                                        let var470 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var469))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                        var52 = var470;
                                                                                                                                                                                                                                                                                                                                                                                                                        let var471 = self.func49(env, 1054075);
                                                                                                                                                                                                                                                                                                                                                                                                                        let var472 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var471))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                        var53 = var472;
                                                                                                                                                                                                                                                                                                                                                                                                                        let var473 = self.func49(env, 1054107);
                                                                                                                                                                                                                                                                                                                                                                                                                        let var474 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var473))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var474 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var53 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var52 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                        'label200: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                            'label201: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label201;
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                'label202: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                    'label203: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label202;
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var475 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var475 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label203;
                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                let var476 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                var52 = var476;
                                                                                                                                                                                                                                                                                                                                                                                                                                let var477 = self.func49(env, 1054139);
                                                                                                                                                                                                                                                                                                                                                                                                                                let var478 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var477))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                var53 = var478;
                                                                                                                                                                                                                                                                                                                                                                                                                                let var479 = self.func49(env, 1054171);
                                                                                                                                                                                                                                                                                                                                                                                                                                let var480 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var479))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                var54 = var480;
                                                                                                                                                                                                                                                                                                                                                                                                                                let var481 = self.func49(env, 1054203);
                                                                                                                                                                                                                                                                                                                                                                                                                                let var482 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var481))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var482 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var54 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var53 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                'label204: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                    'label205: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label205;
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                        'label206: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                            'label207: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label206;
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var483 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var483 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label207;
                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var484 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                        var53 = var484;
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var485 = self.func49(env, 1054235);
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var486 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var485))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                        var54 = var486;
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var487 = self.func49(env, 1054267);
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var488 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var487))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                        var55 = var488;
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var489 = self.func49(env, 1054299);
                                                                                                                                                                                                                                                                                                                                                                                                                                        let var490 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var489))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var490 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var55 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var54 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                        'label208: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                            'label209: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label209;
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                'label210: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label211: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label210;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var491 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var491 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label211;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var492 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                var54 = var492;
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var493 = self.func49(env, 1054331);
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var494 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var493))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                var55 = var494;
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var495 = self.func49(env, 1054363);
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var496 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var495))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                var56 = var496;
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var497 = self.func49(env, 1054395);
                                                                                                                                                                                                                                                                                                                                                                                                                                                let var498 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var497))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var498 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var56 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var55 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                'label212: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label213: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label213;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label214: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label215: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label214;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var499 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var499 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label215;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var500 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var55 = var500;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var501 = self.func49(env, 1054427);
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var502 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var501))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var56 = var502;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var503 = self.func49(env, 1054459);
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var504 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var503))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var57 = var504;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var505 = self.func49(env, 1054491);
                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var506 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var505))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var506 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var57 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var56 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label216: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label217: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label217;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label218: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label219: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label218;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var507 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var507 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label219;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var508 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var56 = var508;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var509 = self.func49(env, 1054523);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var510 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var509))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var57 = var510;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var511 = self.func49(env, 1054555);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var512 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var511))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var58 = var512;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var513 = self.func49(env, 1054587);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var514 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var513))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var514 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var58 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var57 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label220: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label221: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label221;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label222: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label223: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label222;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var515 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var515 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label223;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var516 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var57 = var516;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var517 = self.func49(env, 1054619);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var518 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var517))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var58 = var518;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var519 = self.func49(env, 1054651);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var520 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var519))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var59 = var520;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var521 = self.func49(env, 1054683);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var522 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var521))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var522 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var59 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var58 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label224: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label225: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label225;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label226: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label227: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label226;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var523 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var523 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label227;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var524 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var58 = var524;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var525 = self.func49(env, 1054715);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var526 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var525))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var59 = var526;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var527 = self.func49(env, 1054747);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var528 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var527))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var60 = var528;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var529 = self.func49(env, 1054779);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var530 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var529))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var530 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var60 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var59 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label228: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label229: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label229;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label230: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label231: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label230;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var531 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var531 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label231;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var532 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var59 = var532;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var533 = self.func49(env, 1051483);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var534 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var533))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var60 = var534;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var535 = self.func49(env, 1054811);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var536 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var535))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var61 = var536;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var537 = self.func49(env, 1054843);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var538 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var537))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var538 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var61 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var60 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label232: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label233: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label233;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label234: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label235: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label234;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var539 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var539 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label235;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var540 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var60 = var540;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var541 = self.func49(env, 1054875);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var542 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var541))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var61 = var542;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var543 = self.func49(env, 1054907);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var544 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var543))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var62 = var544;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var545 = self.func49(env, 1054939);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var546 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var545))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var546 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var62 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var61 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label236: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label237: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label237;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label238: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label239: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label238;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var547 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var547 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label239;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var548 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var61 = var548;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var549 = self.func49(env, 1054971);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var550 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var549))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var62 = var550;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var551 = self.func49(env, 1055003);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var552 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var551))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var63 = var552;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var553 = self.func49(env, 1055035);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var554 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var553))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var554 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var63 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var62 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label240: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label241: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label241;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label242: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label243: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label242;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var555 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var555 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label243;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var556 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var62 = var556;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var557 = self.func49(env, 1055067);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var558 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var557))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var63 = var558;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var559 = self.func49(env, 1055099);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var560 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var559))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var64 = var560;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var561 = self.func49(env, 1055131);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var562 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var561))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_24_i64 = var562 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_16_i64 = var64 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                slot_var0_8_i64 = var63 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label244: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label245: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label245;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label246: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label247: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label246;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var563 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var563 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label247;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var564 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var63 = var564;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var565 = self.func49(env, 1055163);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var566 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var565))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var64 = var566;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var567 = self.func49(env, 1055195);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var568 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var567))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var65 = var568;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var569 = self.func49(env, 1055227);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var570 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var569))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var570 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var65 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var64 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label248: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label249: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label249;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label250: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label251: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label250;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var571 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var571 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label251;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var572 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var64 = var572;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var573 = self.func49(env, 1055259);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var574 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var573))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var65 = var574;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var575 = self.func49(env, 1055291);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var576 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var575))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var66 = var576;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var577 = self.func49(env, 1055323);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var578 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var577))).into_val(env))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut slot_var0_536_i64 = var578 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut slot_var0_528_i64 = var66 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let mut slot_var0_520_i64 = var65 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label252: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label253: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 != 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label253;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label254: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label255: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 == 24) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label254;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var579 = self.memory.load64(var0.wrapping_add(520).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var579 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                continue 'label255;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var580 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_512_i64 = var580 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_504_i64 = var64 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_496_i64 = var63 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_488_i64 = var62 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_480_i64 = var61 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_472_i64 = var60 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_464_i64 = var59 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_456_i64 = var58 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_448_i64 = var57 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_440_i64 = var56 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_432_i64 = var55 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_424_i64 = var54 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_416_i64 = var53 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_408_i64 = var52 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_400_i64 = var51 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_392_i64 = var50 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_384_i64 = var49 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_376_i64 = var48 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_368_i64 = var47 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_360_i64 = var46 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_352_i64 = var45 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_344_i64 = var44 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_336_i64 = var43 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_328_i64 = var42 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_320_i64 = var41 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_312_i64 = var40 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_304_i64 = var39 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_296_i64 = var38 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_288_i64 = var37 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_280_i64 = var36 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_272_i64 = var35 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_264_i64 = var34 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_256_i64 = var33 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_248_i64 = var32 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_240_i64 = var31 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_232_i64 = var30 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_224_i64 = var29 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_216_i64 = var28 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_208_i64 = var27 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_200_i64 = var26 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_192_i64 = var25 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_184_i64 = var24 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_176_i64 = var23 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_168_i64 = var22 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_160_i64 = var21 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_152_i64 = var20 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_144_i64 = var19 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_136_i64 = var18 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_128_i64 = var17 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_120_i64 = var16 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_112_i64 = var15 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_104_i64 = var14 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_96_i64 = var13 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_88_i64 = var12 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_80_i64 = var11 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_72_i64 = var10 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_64_i64 = var9 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_56_i64 = var8 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_48_i64 = var7 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_40_i64 = var6 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let mut slot_var0_32_i64 = var5 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_24_i64 = var4 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_16_i64 = var2 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        slot_var0_8_i64 = var1 as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        'label256: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'label257: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (var3 != 512) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break 'label257;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var3 = 0;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                'label258: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    'label259: loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if (var3 == 512) as i32 != 0 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break 'label258;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        let var581 = self.memory.load64(var0.wrapping_add(8).wrapping_add(var3) as usize) as i64;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, var581 as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        continue 'label259;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                let var582 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                var1 = var582;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                self.global0 = var0.wrapping_add(1056);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                return var1;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label256;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label252;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label248;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label244;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label240;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label236;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label232;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label228;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label224;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label220;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label216;
                                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label212;
                                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label208;
                                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label204;
                                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label200;
                                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label196;
                                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                            continue 'label192;
                                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                                    continue 'label188;
                                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                            continue 'label184;
                                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                                    continue 'label180;
                                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                            continue 'label176;
                                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                                    continue 'label172;
                                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                            continue 'label168;
                                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                                    continue 'label164;
                                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                            continue 'label160;
                                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                                    continue 'label156;
                                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                            continue 'label152;
                                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                                    continue 'label148;
                                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                            continue 'label144;
                                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                                    continue 'label140;
                                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                            continue 'label136;
                                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                                    continue 'label132;
                                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                            continue 'label128;
                                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                                    continue 'label124;
                                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                            continue 'label120;
                                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                                    continue 'label116;
                                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                            continue 'label112;
                                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                                    continue 'label108;
                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                break;
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                            continue 'label104;
                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                                    continue 'label100;
                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                break;
                                                                                                                                                                                                            }
                                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                                            continue 'label96;
                                                                                                                                                                                                            break;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        break;
                                                                                                                                                                                                    }
                                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                                    continue 'label92;
                                                                                                                                                                                                    break;
                                                                                                                                                                                                }
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                                            continue 'label88;
                                                                                                                                                                                            break;
                                                                                                                                                                                        }
                                                                                                                                                                                        break;
                                                                                                                                                                                    }
                                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                                    continue 'label84;
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                                            continue 'label80;
                                                                                                                                                                            break;
                                                                                                                                                                        }
                                                                                                                                                                        break;
                                                                                                                                                                    }
                                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                                    continue 'label76;
                                                                                                                                                                    break;
                                                                                                                                                                }
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                                            continue 'label72;
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                                    continue 'label68;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                                            continue 'label64;
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                                    continue 'label60;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                                            continue 'label56;
                                                                                                                            break;
                                                                                                                        }
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                                    continue 'label52;
                                                                                                                    break;
                                                                                                                }
                                                                                                                break;
                                                                                                            }
                                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                            var3 = var3.wrapping_add(8);
                                                                                                            continue 'label48;
                                                                                                            break;
                                                                                                        }
                                                                                                        break;
                                                                                                    }
                                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                                    var3 = var3.wrapping_add(8);
                                                                                                    continue 'label44;
                                                                                                    break;
                                                                                                }
                                                                                                break;
                                                                                            }
                                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                            var3 = var3.wrapping_add(8);
                                                                                            continue 'label40;
                                                                                            break;
                                                                                        }
                                                                                        break;
                                                                                    }
                                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                                    var3 = var3.wrapping_add(8);
                                                                                    continue 'label36;
                                                                                    break;
                                                                                }
                                                                                break;
                                                                            }
                                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                            var3 = var3.wrapping_add(8);
                                                                            continue 'label32;
                                                                            break;
                                                                        }
                                                                        break;
                                                                    }
                                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                                    var3 = var3.wrapping_add(8);
                                                                    continue 'label28;
                                                                    break;
                                                                }
                                                                break;
                                                            }
                                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                            var3 = var3.wrapping_add(8);
                                                            continue 'label24;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                    var3 = var3.wrapping_add(8);
                                                    continue 'label20;
                                                    break;
                                                }
                                                break;
                                            }
                                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                            var3 = var3.wrapping_add(8);
                                            continue 'label16;
                                            break;
                                        }
                                        break;
                                    }
                                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                    var3 = var3.wrapping_add(8);
                                    continue 'label12;
                                    break;
                                }
                                break;
                            }
                            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                            var3 = var3.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                    var3 = var3.wrapping_add(8);
                    continue 'label4;
                    break;
                }
                break;
            }
            self.memory.store64(var0.wrapping_add(544).wrapping_add(var3) as usize, 0 /* Void */ as u64);
            var3 = var3.wrapping_add(8);
            continue 'label0;
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func85(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16);
        self.global0 = var1;
        let mut slot_var1_0_i64 = 0 as i64;
        var2 = 0 /* Void */;
        var3 = 1;
        'label0: loop {
            'label1: loop {
                if (var3 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var3.wrapping_add(-1);
                var2 = 0;
                continue 'label1;
                break;
            }
            break;
        }
        let mut slot_var1_8_i64 = var2 as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        var2 = var5;
        var3 = 2;
        'label2: loop {
            'label3: loop {
                self.memory.store64(arg0 as usize, var2 as u64);
                if (var3 == 0) as i32 != 0 {
                    break 'label2;
                }
                var3 = var3.wrapping_add(-1);
                let var6 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var2)); v.push_back(val_from_i64(0)); val_to_i64(v.into_val(env)) }
                var2 = var6;
                continue 'label3;
                break;
            }
            break;
        }
        self.global0 = var1.wrapping_add(16);
    }
    fn func86(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) -> i64 {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        'label0: loop {
            let var10 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
            if ((var10 as u64) < 4294967296 as u64) as i32 != 0 {
                break 'label0;
            }
            self.func85(env, arg0);
            'label1: loop {
                'label2: loop {
                    let var12 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
                    if (var12 as u64 >= 12884901888 as u64) as i32 != 0 {
                        break 'label2;
                    }
                    let var13 = self.func49(env, 1048891);
                    let var14 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var13))).into_val(env))
                    var2 = var14;
                    var3 = (var2 as u64).wrapping_shr(0 as u32) as i64;
                    var4 = var2 & 255;
                    let var15 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
                    var5 = (var15 as u64).wrapping_shr(32 as u32) as i64;
                    let var16 = self.memory.load64(arg0 as usize) as i64;
                    var6 = var16;
                    var7 = 1;
                    'label3: loop {
                        'label4: loop {
                            if (var5 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            'label5: loop {
                                let var17 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1)).get_unchecked(var7.wrapping_add(18446744069414584320) as u32))
                                var8 = var17;
                                var9 = var8 as i32 & 255;
                                if (var9 == 12) as i32 != 0 {
                                    break 'label5;
                                }
                                if (var9 != 70) as i32 != 0 {
                                    break 'label1;
                                }
                                break;
                            }
                            'label6: loop {
                                'label7: loop {
                                    'label8: loop {
                                        if (var8 & 78 /* MuxedAddress(obj#0) */ != 0) as i32 != 0 {
                                            break 'label8;
                                        }
                                        if (var4 == 0) as i32 != 0 {
                                            break 'label7;
                                        }
                                        break;
                                    }
                                    let var18 = { let a = val_from_i64(var8); let b = val_from_i64(var2); if a < b { -1 } else if a > b { 1 } else { 0 } }
                                    if (var18 < 0 /* False */) as i32 != 0 {
                                        break 'label6;
                                    }
                                    break 'label0;
                                    break;
                                }
                                if ((var8 as u64).wrapping_shr(0 as u32) as i64 as u64 >= var3 as u64) as i32 != 0 {
                                    break 'label0;
                                }
                                break;
                            }
                            let var19 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(var6)); v.set(var7 as u32, val_from_i64(var8)); val_to_i64(v.into_val(env)) }
                            var6 = var19;
                            self.memory.store64(arg0 as usize, var6 as u64);
                            var5 = var5.wrapping_add(18446744073709551615);
                            var7 = var7.wrapping_add(4294967296);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var20 = self.memory.load32(arg0 as usize + 24) as i64;
                    let var21 = self.memory.load32(arg0 as usize + 28) as i64;
                    let var22 = self.memory.load64(arg0 as usize + 8) as i64;
                    let var23 = self.memory.load64(arg0 as usize + 16) as i64;
                    let var24 = 0 /* TODO: poseidon_permutation */
                    var7 = var24;
                    self.memory.store64(arg0 as usize, var7 as u64);
                    'label9: loop {
                        let var25 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var7)).get_unchecked(0 as u32))
                        var7 = var25;
                        var9 = var7 as i32 & 255;
                        if (var9 == 12) as i32 != 0 {
                            break 'label9;
                        }
                        if (var9 != 70) as i32 != 0 {
                            break 'label1;
                        }
                        break;
                    }
                    return var7;
                    break;
                }
                self.func83(env, 44);
                break;
            }
            unreachable!();
            break;
        }
        self.func72(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func87(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(16);
        self.global0 = var2;
        let mut slot_var2_8_i64 = ((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
        let mut slot_var2_0_i64 = ((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        var3 = var5;
        self.global0 = var2.wrapping_add(16);
        var3
    }
    fn func88(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func89(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        'label0: loop {
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
    fn func90(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let var15 = self.global0;
        var3 = var15.wrapping_sub(16);
        'label0: loop {
            'label1: loop {
                if (arg2 as u32 >= 16 as u32) as i32 != 0 {
                    break 'label1;
                }
                var4 = arg0;
                break 'label0;
                break;
            }
            'label2: loop {
                var5 = 0.wrapping_sub(arg0) & 3;
                var6 = arg0.wrapping_add(var5);
                if (arg0 as u32 >= var6 as u32) as i32 != 0 {
                    break 'label2;
                }
                var7 = var5.wrapping_add(-1);
                var4 = arg0;
                var8 = arg1;
                'label3: loop {
                    if (var5 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    var9 = var5;
                    var4 = arg0;
                    var8 = arg1;
                    'label4: loop {
                        let var16 = self.memory.load8(var8 as usize) as i32;
                        self.memory.store8(var4 as usize, var16 as u8);
                        var8 = var8.wrapping_add(1);
                        var4 = var4.wrapping_add(1);
                        var9 = var9.wrapping_add(-1);
                        if var9 != 0 {
                            continue 'label4;
                        }
                        break;
                    }
                    break;
                }
                if ((var7 as u32) < 7 as u32) as i32 != 0 {
                    break 'label2;
                }
                'label5: loop {
                    let var17 = self.memory.load8(var8 as usize) as i32;
                    self.memory.store8(var4 as usize, var17 as u8);
                    let var18 = self.memory.load8(var8.wrapping_add(1) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(1) as usize, var18 as u8);
                    let var19 = self.memory.load8(var8.wrapping_add(2) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(2) as usize, var19 as u8);
                    let var20 = self.memory.load8(var8.wrapping_add(3) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(3) as usize, var20 as u8);
                    let var21 = self.memory.load8(var8.wrapping_add(4) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(4) as usize, var21 as u8);
                    let var22 = self.memory.load8(var8.wrapping_add(5) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(5) as usize, var22 as u8);
                    let var23 = self.memory.load8(var8.wrapping_add(6) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(6) as usize, var23 as u8);
                    let var24 = self.memory.load8(var8.wrapping_add(7) as usize) as i32;
                    self.memory.store8(var4.wrapping_add(7) as usize, var24 as u8);
                    var8 = var8.wrapping_add(8);
                    var4 = var4.wrapping_add(8);
                    if (var4 != var6) as i32 != 0 {
                        continue 'label5;
                    }
                    break;
                }
                break;
            }
            var9 = arg2.wrapping_sub(var5);
            var7 = var9 & -4;
            var4 = var6.wrapping_add(var7);
            'label6: loop {
                'label7: loop {
                    var8 = arg1.wrapping_add(var5);
                    arg1 = var8 & 3;
                    if arg1 != 0 {
                        break 'label7;
                    }
                    if (var6 as u32 >= var4 as u32) as i32 != 0 {
                        break 'label6;
                    }
                    arg1 = var8;
                    'label8: loop {
                        let var25 = self.memory.load32(arg1 as usize) as i32;
                        let mut slot_var6_0_i32 = var25 as i32;
                        arg1 = arg1.wrapping_add(4);
                        var6 = var6.wrapping_add(4);
                        if ((var6 as u32) < var4 as u32) as i32 != 0 {
                            continue 'label8;
                        }
                        break 'label6;
                        break;
                    }
                    break;
                }
                arg2 = 0;
                let mut slot_var3_12_i32 = 0 as i32;
                var5 = var3.wrapping_add(12) | arg1;
                'label9: loop {
                    var10 = 4.wrapping_sub(arg1);
                    if (var10 & 1 == 0) as i32 != 0 {
                        break 'label9;
                    }
                    let var26 = self.memory.load8(var8 as usize) as i32;
                    self.memory.store8(var5 as usize, var26 as u8);
                    arg2 = 1;
                    break;
                }
                'label10: loop {
                    if (var10 & 2 == 0) as i32 != 0 {
                        break 'label10;
                    }
                    let var27 = self.memory.load16(var8.wrapping_add(arg2) as usize) as i32;
                    self.memory.store16(var5.wrapping_add(arg2) as usize, var27 as u16);
                    break;
                }
                arg2 = var8.wrapping_sub(arg1);
                var11 = arg1.wrapping_shl(3 as u32);
                var5 = slot_var3_12_i32;
                'label11: loop {
                    'label12: loop {
                        if ((var6.wrapping_add(4) as u32) < var4 as u32) as i32 != 0 {
                            break 'label12;
                        }
                        var12 = var6;
                        break 'label11;
                        break;
                    }
                    var13 = 0.wrapping_sub(var11) & 24;
                    'label13: loop {
                        let var28 = var5;
                        arg2 = arg2.wrapping_add(4);
                        let var29 = self.memory.load32(arg2 as usize) as i32;
                        var5 = var29;
                        slot_var6_0_i32 = ((var28 as u32).wrapping_shr(var11 as u32) as i32 | var5.wrapping_shl(var13 as u32)) as i32;
                        var10 = var6.wrapping_add(8);
                        var12 = var6.wrapping_add(4);
                        var6 = var12;
                        if ((var10 as u32) < var4 as u32) as i32 != 0 {
                            continue 'label13;
                        }
                        break;
                    }
                    break;
                }
                var6 = 0;
                self.memory.store8(var3 as usize + 8, 0 as u8);
                self.memory.store8(var3 as usize + 6, 0 as u8);
                'label14: loop {
                    'label15: loop {
                        if (arg1 != 1) as i32 != 0 {
                            break 'label15;
                        }
                        var13 = var3.wrapping_add(8);
                        arg1 = 0;
                        var10 = 0;
                        var14 = 0;
                        break 'label14;
                        break;
                    }
                    let var30 = self.memory.load8(arg2.wrapping_add(5) as usize) as i32;
                    var10 = var30;
                    let var31 = self.memory.load8(arg2.wrapping_add(4) as usize) as i32;
                    arg1 = var31;
                    self.memory.store8(var3 as usize + 8, arg1 as u8);
                    var10 = var10.wrapping_shl(8 as u32);
                    var14 = 2;
                    var13 = var3.wrapping_add(6);
                    break;
                }
                'label16: loop {
                    if (var8 & 1 == 0) as i32 != 0 {
                        break 'label16;
                    }
                    let var32 = self.memory.load8(arg2.wrapping_add(4).wrapping_add(var14) as usize) as i32;
                    self.memory.store8(var13 as usize, var32 as u8);
                    let var33 = self.memory.load8(var3 as usize + 6) as i32;
                    var6 = var33.wrapping_shl(16 as u32);
                    let var34 = self.memory.load8(var3 as usize + 8) as i32;
                    arg1 = var34;
                    break;
                }
                let mut slot_var12_0_i32 = ((var10 | var6 | arg1 & 255).wrapping_shl((0.wrapping_sub(var11) & 24) as u32) | (var5 as u32).wrapping_shr(var11 as u32) as i32) as i32;
                break;
            }
            arg2 = var9 & 3;
            arg1 = var8.wrapping_add(var7);
            break;
        }
        'label17: loop {
            var6 = var4.wrapping_add(arg2);
            if (var4 as u32 >= var6 as u32) as i32 != 0 {
                break 'label17;
            }
            var9 = arg2.wrapping_add(-1);
            'label18: loop {
                var8 = arg2 & 7;
                if (var8 == 0) as i32 != 0 {
                    break 'label18;
                }
                'label19: loop {
                    let var35 = self.memory.load8(arg1 as usize) as i32;
                    self.memory.store8(var4 as usize, var35 as u8);
                    arg1 = arg1.wrapping_add(1);
                    var4 = var4.wrapping_add(1);
                    var8 = var8.wrapping_add(-1);
                    if var8 != 0 {
                        continue 'label19;
                    }
                    break;
                }
                break;
            }
            if ((var9 as u32) < 7 as u32) as i32 != 0 {
                break 'label17;
            }
            'label20: loop {
                let var36 = self.memory.load8(arg1 as usize) as i32;
                self.memory.store8(var4 as usize, var36 as u8);
                let var37 = self.memory.load8(arg1.wrapping_add(1) as usize) as i32;
                self.memory.store8(var4.wrapping_add(1) as usize, var37 as u8);
                let var38 = self.memory.load8(arg1.wrapping_add(2) as usize) as i32;
                self.memory.store8(var4.wrapping_add(2) as usize, var38 as u8);
                let var39 = self.memory.load8(arg1.wrapping_add(3) as usize) as i32;
                self.memory.store8(var4.wrapping_add(3) as usize, var39 as u8);
                let var40 = self.memory.load8(arg1.wrapping_add(4) as usize) as i32;
                self.memory.store8(var4.wrapping_add(4) as usize, var40 as u8);
                let var41 = self.memory.load8(arg1.wrapping_add(5) as usize) as i32;
                self.memory.store8(var4.wrapping_add(5) as usize, var41 as u8);
                let var42 = self.memory.load8(arg1.wrapping_add(6) as usize) as i32;
                self.memory.store8(var4.wrapping_add(6) as usize, var42 as u8);
                let var43 = self.memory.load8(arg1.wrapping_add(7) as usize) as i32;
                self.memory.store8(var4.wrapping_add(7) as usize, var43 as u8);
                arg1 = arg1.wrapping_add(8);
                var4 = var4.wrapping_add(8);
                if (var4 != var6) as i32 != 0 {
                    continue 'label20;
                }
                break;
            }
            break;
        }
        arg0
    }
    fn func91(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let var3 = self.func90(env, arg0, arg1, arg2);
        var3
    }
    fn func92(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (arg2 as u32 >= 16 as u32) as i32 != 0 {
                    break 'label1;
                }
                var3 = arg0;
                break 'label0;
                break;
            }
            'label2: loop {
                var4 = 0.wrapping_sub(arg0) & 3;
                var5 = arg0.wrapping_add(var4);
                if (arg0 as u32 >= var5 as u32) as i32 != 0 {
                    break 'label2;
                }
                var6 = var4.wrapping_add(-1);
                var3 = arg0;
                'label3: loop {
                    if (var4 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    var7 = var4;
                    var3 = arg0;
                    'label4: loop {
                        self.memory.store8(var3 as usize, arg1 as u8);
                        var3 = var3.wrapping_add(1);
                        var7 = var7.wrapping_add(-1);
                        if var7 != 0 {
                            continue 'label4;
                        }
                        break;
                    }
                    break;
                }
                if ((var6 as u32) < 7 as u32) as i32 != 0 {
                    break 'label2;
                }
                'label5: loop {
                    self.memory.store8(var3 as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(7) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(6) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(5) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(4) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(3) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(2) as usize, arg1 as u8);
                    self.memory.store8(var3.wrapping_add(1) as usize, arg1 as u8);
                    var3 = var3.wrapping_add(8);
                    if (var3 != var5) as i32 != 0 {
                        continue 'label5;
                    }
                    break;
                }
                break;
            }
            'label6: loop {
                arg2 = arg2.wrapping_sub(var4);
                var3 = var5.wrapping_add(arg2 & -4);
                if (var5 as u32 >= var3 as u32) as i32 != 0 {
                    break 'label6;
                }
                var7 = (arg1 & 255).wrapping_mul(16843009);
                'label7: loop {
                    let mut slot_var5_0_i32 = var7 as i32;
                    var5 = var5.wrapping_add(4);
                    if ((var5 as u32) < var3 as u32) as i32 != 0 {
                        continue 'label7;
                    }
                    break;
                }
                break;
            }
            arg2 = arg2 & 3;
            break;
        }
        'label8: loop {
            var7 = var3.wrapping_add(arg2);
            if (var3 as u32 >= var7 as u32) as i32 != 0 {
                break 'label8;
            }
            var4 = arg2.wrapping_add(-1);
            'label9: loop {
                var5 = arg2 & 7;
                if (var5 == 0) as i32 != 0 {
                    break 'label9;
                }
                'label10: loop {
                    self.memory.store8(var3 as usize, arg1 as u8);
                    var3 = var3.wrapping_add(1);
                    var5 = var5.wrapping_add(-1);
                    if var5 != 0 {
                        continue 'label10;
                    }
                    break;
                }
                break;
            }
            if ((var4 as u32) < 7 as u32) as i32 != 0 {
                break 'label8;
            }
            'label11: loop {
                self.memory.store8(var3 as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(7) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(6) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(5) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(4) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(3) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(2) as usize, arg1 as u8);
                self.memory.store8(var3.wrapping_add(1) as usize, arg1 as u8);
                var3 = var3.wrapping_add(8);
                if (var3 != var7) as i32 != 0 {
                    continue 'label11;
                }
                break;
            }
            break;
        }
        arg0
    }
}
