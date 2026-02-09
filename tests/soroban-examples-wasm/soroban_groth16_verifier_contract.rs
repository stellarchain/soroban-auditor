#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, Bytes, Val, Address, FromVal, IntoVal, Map, BytesN, String, Symbol};

#[contract]
pub struct Groth16VerifierContract;

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
pub enum Groth16Error { MalformedVerifyingKey = 0, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VerificationKey { pub alpha: soroban_sdk::BytesN<96>, pub beta: soroban_sdk::BytesN<192>, pub delta: soroban_sdk::BytesN<192>, pub gamma: soroban_sdk::BytesN<192>, pub ic: soroban_sdk::Vec<soroban_sdk::BytesN<96>>, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Proof { pub a: soroban_sdk::BytesN<96>, pub b: soroban_sdk::BytesN<192>, pub c: soroban_sdk::BytesN<96>, }

#[contractimpl]
impl Groth16VerifierContract {
    pub fn verify_proof(&mut self, mut env: Env, mut vk: VerificationKey, mut proof: Proof, pub_signals: soroban_sdk::Vec<soroban_sdk::U256>) -> Result<bool, Groth16Error> {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var3 = var17.wrapping_sub(192);
        self.global0 = var3;
        var4 = 0;
        'label0: loop {
            'label1: loop {
                if (var4 == 40) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                var4 = var4.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func17(env, vk, 1048600, 5, var3.wrapping_add(8), 5);
                let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                self.func15(env, var3.wrapping_add(96), slot_var3_8_i64);
                let mut slot_var3_96_i32 = self.memory.load32(var3 as usize + 96) as i32;
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                let mut slot_var3_104_i64 = self.memory.load64(var3 as usize + 104) as i64;
                var5 = slot_var3_104_i64;
                let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_16_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var6 = slot_var3_104_i64;
                let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_24_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var7 = slot_var3_104_i64;
                let mut slot_var3_32_i64 = self.memory.load64(var3 as usize + 32) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_32_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                let mut slot_var3_40_i64 = self.memory.load64(var3 as usize + 40) as i64;
                vk = slot_var3_40_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                var8 = slot_var3_104_i64;
                var4 = 0;
                'label4: loop {
                    'label5: loop {
                        if (var4 == 24) as i32 != 0 {
                            break 'label4;
                        }
                        self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label5;
                        break;
                    }
                    break;
                }
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func17(env, proof, 1048644, 3, var3.wrapping_add(8), 3);
                self.func15(env, var3.wrapping_add(96), slot_var3_8_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var9 = slot_var3_104_i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_16_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var10 = slot_var3_104_i64;
                self.func15(env, var3.wrapping_add(96), slot_var3_24_i64);
                if (slot_var3_96_i32 == 1) as i32 != 0 {
                    break 'label3;
                }
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(pub_signals)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                var11 = slot_var3_104_i64;
                'label6: loop {
                    'label7: loop {
                        let var27 = Vec::<Val>::from_val(env, &val_from_i64(pub_signals)).len() as i64
                        var4 = ((var27 as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_add(1);
                        if (var4 == 0) as i32 != 0 {
                            break 'label7;
                        }
                        proof = 3 /* Error(Contract, #0) */;
                        let var28 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        if (var4 != (var28 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                            break 'label2;
                        }
                        let var29 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        if ((var29 as u64) < 4294967296 as u64) as i32 != 0 {
                            break 'label6;
                        }
                        let var30 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(vk)).get_unchecked(0 as u32))
                        self.func15(env, var3.wrapping_add(8), var30);
                        if (slot_var3_8_i64 == 1) as i32 != 0 {
                            break 'label3;
                        }
                        var12 = slot_var3_16_i64;
                        let var32 = Vec::<Val>::from_val(env, &val_from_i64(pub_signals)).len() as i64
                        proof = var32;
                        let var33 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        var13 = var33;
                        let mut slot_var3_56_i32 = 0 as i32;
                        let mut slot_var3_48_i64 = 0 /* False */ as i64;
                        let mut slot_var3_40_i32 = 1 as i32;
                        self.memory.store32(var3 as usize + 36, (var13 as u64).wrapping_shr(32 as u32) as i64 as u32);
                        let mut slot_var3_32_i32 = 0 as i32;
                        slot_var3_24_i64 = vk as i64;
                        var14 = (proof as u64).wrapping_shr(32 as u32) as i64 as i32;
                        let mut slot_var3_20_i32 = var14 as i32;
                        slot_var3_8_i64 = pub_signals as i64;
                        var15 = var3.wrapping_add(24);
                        var4 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var4 as u32 >= var14 as u32) as i32 != 0 {
                                    break 'label8;
                                }
                                'label10: loop {
                                    'label11: loop {
                                        let var34 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(slot_var3_8_i64)).get_unchecked((var4 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
                                        vk = var34;
                                        var14 = vk as i32 & 255;
                                        var16 = (var14 == 70) as i32;
                                        if var16 != 0 {
                                            break 'label11;
                                        }
                                        if (var14 != 12) as i32 != 0 {
                                            break 'label10;
                                        }
                                        break;
                                    }
                                    pub_signals = vk;
                                    break;
                                }
                                var4 = var4.wrapping_add(1);
                                if (var4 == 0) as i32 != 0 {
                                    break 'label7;
                                }
                                let mut slot_var3_16_i32 = var4 as i32;
                                'label12: loop {
                                    if (var14 == 12) as i32 != 0 {
                                        break 'label12;
                                    }
                                    if (var16 == 0) as i32 != 0 {
                                        break 'label7;
                                    }
                                    break;
                                }
                                'label13: loop {
                                    'label14: loop {
                                        var4 = slot_var3_40_i32;
                                        if var4 != 0 {
                                            break 'label14;
                                        }
                                        self.func14(env, var3.wrapping_add(96), var15);
                                        self.func12(env, var3.wrapping_add(64), slot_var3_96_i32, slot_var3_104_i64);
                                        break 'label13;
                                        break;
                                    }
                                    slot_var3_40_i32 = 0 as i32;
                                    'label15: loop {
                                        'label16: loop {
                                            self.func14(env, var3.wrapping_add(96), var15);
                                            self.func12(env, var3.wrapping_add(80), slot_var3_96_i32, slot_var3_104_i64);
                                            let mut slot_var3_80_i32 = self.memory.load32(var3 as usize + 80) as i32;
                                            if (slot_var3_80_i32 != 1) as i32 != 0 {
                                                break 'label15;
                                            }
                                            var4 = var4.wrapping_add(-1);
                                            if var4 != 0 {
                                                continue 'label16;
                                            }
                                            break;
                                        }
                                        self.func14(env, var3.wrapping_add(96), var15);
                                        self.func12(env, var3.wrapping_add(64), slot_var3_96_i32, slot_var3_104_i64);
                                        break 'label13;
                                        break;
                                    }
                                    let mut slot_var3_64_i64 = 0 /* False */ as i64;
                                    break;
                                }
                                if (slot_var3_64_i64 == 0) as i32 != 0 {
                                    break 'label8;
                                }
                                let mut slot_var3_72_i64 = self.memory.load64(var3 as usize + 72) as i64;
                                let var41 = 0 /* TODO: bls12_381_g1_mul */
                                let var42 = 0 /* TODO: bls12_381_g1_add */
                                var12 = var42;
                                var14 = slot_var3_20_i32;
                                var4 = slot_var3_16_i32;
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var43 = Bytes::from_val(env, &val_from_i64(var9)).len() as i64
                        let var44 = val_to_i64(Bytes::from_val(env, &val_from_i64(var9)).slice(48 as u32..var43 & 18446744069414584320 | 0 as u32).into_val(env))
                        vk = var44;
                        if (!(Bytes::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        let var45 = Bytes::from_val(env, &val_from_i64(vk)).len() as i64
                        if (var45 & 18446744069414584320 != 206158430208) as i32 != 0 {
                            break 'label3;
                        }
                        let var46 = self.func28(env, var3.wrapping_add(8), 0, 48);
                        var46;
                        self.func19(env, vk, var3.wrapping_add(8));
                        let var48 = self.func27(env, var3.wrapping_add(96), var3.wrapping_add(8), 48);
                        var48;
                        let var49 = self.func28(env, var3.wrapping_add(144), 0, 48);
                        var49;
                        var4 = 40;
                        var14 = var3.wrapping_add(96);
                        'label17: loop {
                            'label18: loop {
                                if (var4 == -8) as i32 != 0 {
                                    break 'label17;
                                }
                                let mut slot_var14_0_i64 = self.memory.load64(var14 as usize) as i64;
                                pub_signals = slot_var14_0_i64;
                                self.memory.store64(var3.wrapping_add(144).wrapping_add(var4) as usize, (pub_signals.wrapping_shl(56 as u32) | (pub_signals & 65280).wrapping_shl(40 as u32) | (pub_signals & 16711680).wrapping_shl(24 as u32) | (pub_signals & 4278190080).wrapping_shl(0 as u32) | (pub_signals as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (pub_signals as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (pub_signals as u64).wrapping_shr(40 as u32) as i64 & 65280 | (pub_signals as u64).wrapping_shr(56 as u32) as i64) as u64);
                                var14 = var14.wrapping_add(8);
                                var4 = var4.wrapping_add(-8);
                                continue 'label18;
                                break;
                            }
                            break;
                        }
                        'label19: loop {
                            'label20: loop {
                                let var50 = self.func25(env, var3.wrapping_add(144), 1048720, 48);
                                if (var50 == 0) as i32 != 0 {
                                    break 'label20;
                                }
                                let var51 = self.func27(env, var3.wrapping_add(96), 1048672, 48);
                                var51;
                                var4 = 0;
                                var16 = 0;
                                'label21: loop {
                                    'label22: loop {
                                        if (var4 == 48) as i32 != 0 {
                                            break 'label21;
                                        }
                                        var14 = var3.wrapping_add(96).wrapping_add(var4);
                                        vk = slot_var14_0_i64;
                                        proof = var16 as u32 as i64 & 255;
                                        let var52 = self.memory.load64(var3.wrapping_add(144).wrapping_add(var4) as usize) as i64;
                                        pub_signals = proof.wrapping_add(var52);
                                        slot_var14_0_i64 = vk.wrapping_sub(pub_signals) as i64;
                                        var16 = ((((pub_signals as u64) < proof as u64) as i32 as u32 as i64).wrapping_add(((vk as u64) < pub_signals as u64) as i32 as u32 as i64) == 1 /* True */) as i32;
                                        var4 = var4.wrapping_add(8);
                                        continue 'label22;
                                        break;
                                    }
                                    break;
                                }
                                if var16 & 255 != 0 {
                                    break 'label19;
                                }
                                let var53 = self.func28(env, var3.wrapping_add(8), 0, 48);
                                var53;
                                var14 = var3.wrapping_add(8);
                                var4 = 40;
                                'label23: loop {
                                    'label24: loop {
                                        if (var4 == -8) as i32 != 0 {
                                            break 'label23;
                                        }
                                        let var54 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                        pub_signals = var54;
                                        slot_var14_0_i64 = (pub_signals.wrapping_shl(56 as u32) | (pub_signals & 65280).wrapping_shl(40 as u32) | (pub_signals & 16711680).wrapping_shl(24 as u32) | (pub_signals & 4278190080).wrapping_shl(0 as u32) | (pub_signals as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (pub_signals as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (pub_signals as u64).wrapping_shr(40 as u32) as i64 & 65280 | (pub_signals as u64).wrapping_shr(56 as u32) as i64) as i64;
                                        var4 = var4.wrapping_add(-8);
                                        var14 = var14.wrapping_add(8);
                                        continue 'label24;
                                        break;
                                    }
                                    break;
                                }
                                let var55 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                                vk = var55;
                                if var16 & 255 != 0 {
                                    break 'label19;
                                }
                                break;
                            }
                            var4 = 0;
                            let var56 = self.func28(env, var3.wrapping_add(8), 0, 48);
                            var56;
                            self.func19(env, vk, var3.wrapping_add(8));
                            let var58 = self.func27(env, var3.wrapping_add(96), var3.wrapping_add(8), 48);
                            var58;
                            let var59 = 0 /* TODO: bytes_copy_from_linear_memory */
                            self.func20(env, var3.wrapping_add(8), var59);
                            if (slot_var3_8_i64 == 1) as i32 != 0 {
                                break 'label3;
                            }
                            pub_signals = slot_var3_16_i64;
                            let mut slot_var3_120_i64 = var11 as i64;
                            let mut slot_var3_112_i64 = var12 as i64;
                            slot_var3_104_i64 = var5 as i64;
                            let mut slot_var3_96_i64 = pub_signals as i64;
                            'label25: loop {
                                'label26: loop {
                                    if (var4 != 32) as i32 != 0 {
                                        break 'label26;
                                    }
                                    var4 = 0;
                                    'label27: loop {
                                        'label28: loop {
                                            if (var4 == 32) as i32 != 0 {
                                                break 'label27;
                                            }
                                            let var61 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                            self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, var61 as u64);
                                            var4 = var4.wrapping_add(8);
                                            continue 'label28;
                                            break;
                                        }
                                        break;
                                    }
                                    let var62 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                    pub_signals = var62;
                                    slot_var3_120_i64 = var7 as i64;
                                    slot_var3_112_i64 = var8 as i64;
                                    slot_var3_104_i64 = var6 as i64;
                                    slot_var3_96_i64 = var10 as i64;
                                    var4 = 0;
                                    'label29: loop {
                                        'label30: loop {
                                            if (var4 != 32) as i32 != 0 {
                                                break 'label30;
                                            }
                                            var4 = 0;
                                            'label31: loop {
                                                'label32: loop {
                                                    if (var4 == 32) as i32 != 0 {
                                                        break 'label31;
                                                    }
                                                    let var63 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                                    self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, var63 as u64);
                                                    var4 = var4.wrapping_add(8);
                                                    continue 'label32;
                                                    break;
                                                }
                                                break;
                                            }
                                            let var64 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                            let var65 = 0 /* TODO: bls12_381_multi_pairing_check */
                                            proof = (var65 == 1 /* True */) as i32 as u32 as i64;
                                            break 'label2;
                                            break;
                                        }
                                        self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                                        var4 = var4.wrapping_add(8);
                                        continue 'label29;
                                        break;
                                    }
                                    break;
                                }
                                self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                                var4 = var4.wrapping_add(8);
                                continue 'label25;
                                break;
                            }
                            break;
                        }
                        self.func22(env);
                        unreachable!();
                        break;
                    }
                    self.func13(env);
                    unreachable!();
                    break;
                }
                self.func23(env);
                unreachable!();
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var3.wrapping_add(192);
        proof
    }
}

#[allow(dead_code)]
impl Groth16VerifierContract {
    fn func12(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i64 = 0;
        var3 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                if (arg1 == 0 /* Void */) as i32 != 0 {
                    break 'label1;
                }
                if arg1 as i32 & 1 != 0 {
                    break 'label0;
                }
                self.memory.store64(arg0 as usize + 8, arg2 as u64);
                var3 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var3 as u64);
            return;
            break;
        }
        self.func13(env);
        unreachable!();
    }
    fn func13(&mut self, env: &Env) {
        self.func24(env);
        unreachable!();
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                let var6 = self.memory.load32(arg1 as usize + 8) as i32;
                var3 = var6;
                let var7 = self.memory.load32(arg1 as usize + 12) as i32;
                if ((var3 as u32) < var7 as u32) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(arg0 as usize, 0 /* Void */ as u64);
                break 'label0;
                break;
            }
            let var8 = self.memory.load64(arg1 as usize) as i64;
            let var9 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var8)).get_unchecked((var3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
            self.func15(env, var2, var9);
            'label2: loop {
                var3 = var3.wrapping_add(1);
                if (var3 == 0) as i32 != 0 {
                    break 'label2;
                }
                let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
                var4 = slot_var2_0_i64;
                let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                self.memory.store64(arg0 as usize + 8, slot_var2_8_i64 as u64);
                self.memory.store64(arg0 as usize, var4 as u64);
                self.memory.store32(arg1 as usize + 8, var3 as u32);
                break 'label0;
                break;
            }
            self.func13(env);
            unreachable!();
            break;
        }
        self.global0 = var2.wrapping_add(16);
    }
    fn func15(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func20(env, var2, arg1);
        arg1 = 1 /* True */;
        'label0: loop {
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if slot_var2_0_i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
            self.memory.store64(arg0 as usize + 8, slot_var2_8_i64 as u64);
            arg1 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
        self.global0 = var2.wrapping_add(16);
    }
    fn func16(&mut self, env: &Env, mut vk: i64, mut proof: i64, mut pub_signals: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var3 = var17.wrapping_sub(192);
        self.global0 = var3;
        var4 = 0;
        'label0: loop {
            'label1: loop {
                if (var4 == 40) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                var4 = var4.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func17(env, vk, 1048600, 5, var3.wrapping_add(8), 5);
                let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                self.func15(env, var3.wrapping_add(96), slot_var3_8_i64);
                let mut slot_var3_96_i32 = self.memory.load32(var3 as usize + 96) as i32;
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                let mut slot_var3_104_i64 = self.memory.load64(var3 as usize + 104) as i64;
                var5 = slot_var3_104_i64;
                let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_16_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var6 = slot_var3_104_i64;
                let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_24_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var7 = slot_var3_104_i64;
                let mut slot_var3_32_i64 = self.memory.load64(var3 as usize + 32) as i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_32_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                let mut slot_var3_40_i64 = self.memory.load64(var3 as usize + 40) as i64;
                vk = slot_var3_40_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                var8 = slot_var3_104_i64;
                var4 = 0;
                'label4: loop {
                    'label5: loop {
                        if (var4 == 24) as i32 != 0 {
                            break 'label4;
                        }
                        self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label5;
                        break;
                    }
                    break;
                }
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func17(env, proof, 1048644, 3, var3.wrapping_add(8), 3);
                self.func15(env, var3.wrapping_add(96), slot_var3_8_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var9 = slot_var3_104_i64;
                self.func18(env, var3.wrapping_add(96), slot_var3_16_i64);
                if slot_var3_96_i32 != 0 {
                    break 'label3;
                }
                var10 = slot_var3_104_i64;
                self.func15(env, var3.wrapping_add(96), slot_var3_24_i64);
                if (slot_var3_96_i32 == 1) as i32 != 0 {
                    break 'label3;
                }
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(pub_signals)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                var11 = slot_var3_104_i64;
                'label6: loop {
                    'label7: loop {
                        let var27 = Vec::<Val>::from_val(env, &val_from_i64(pub_signals)).len() as i64
                        var4 = ((var27 as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_add(1);
                        if (var4 == 0) as i32 != 0 {
                            break 'label7;
                        }
                        proof = 3 /* Error(Contract, #0) */;
                        let var28 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        if (var4 != (var28 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                            break 'label2;
                        }
                        let var29 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        if ((var29 as u64) < 4294967296 as u64) as i32 != 0 {
                            break 'label6;
                        }
                        let var30 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(vk)).get_unchecked(0 as u32))
                        self.func15(env, var3.wrapping_add(8), var30);
                        if (slot_var3_8_i64 == 1) as i32 != 0 {
                            break 'label3;
                        }
                        var12 = slot_var3_16_i64;
                        let var32 = Vec::<Val>::from_val(env, &val_from_i64(pub_signals)).len() as i64
                        proof = var32;
                        let var33 = Vec::<Val>::from_val(env, &val_from_i64(vk)).len() as i64
                        var13 = var33;
                        let mut slot_var3_56_i32 = 0 as i32;
                        let mut slot_var3_48_i64 = 0 /* False */ as i64;
                        let mut slot_var3_40_i32 = 1 as i32;
                        self.memory.store32(var3 as usize + 36, (var13 as u64).wrapping_shr(32 as u32) as i64 as u32);
                        let mut slot_var3_32_i32 = 0 as i32;
                        slot_var3_24_i64 = vk as i64;
                        var14 = (proof as u64).wrapping_shr(32 as u32) as i64 as i32;
                        let mut slot_var3_20_i32 = var14 as i32;
                        slot_var3_8_i64 = pub_signals as i64;
                        var15 = var3.wrapping_add(24);
                        var4 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var4 as u32 >= var14 as u32) as i32 != 0 {
                                    break 'label8;
                                }
                                'label10: loop {
                                    'label11: loop {
                                        let var34 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(slot_var3_8_i64)).get_unchecked((var4 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
                                        vk = var34;
                                        var14 = vk as i32 & 255;
                                        var16 = (var14 == 70) as i32;
                                        if var16 != 0 {
                                            break 'label11;
                                        }
                                        if (var14 != 12) as i32 != 0 {
                                            break 'label10;
                                        }
                                        break;
                                    }
                                    pub_signals = vk;
                                    break;
                                }
                                var4 = var4.wrapping_add(1);
                                if (var4 == 0) as i32 != 0 {
                                    break 'label7;
                                }
                                let mut slot_var3_16_i32 = var4 as i32;
                                'label12: loop {
                                    if (var14 == 12) as i32 != 0 {
                                        break 'label12;
                                    }
                                    if (var16 == 0) as i32 != 0 {
                                        break 'label7;
                                    }
                                    break;
                                }
                                'label13: loop {
                                    'label14: loop {
                                        var4 = slot_var3_40_i32;
                                        if var4 != 0 {
                                            break 'label14;
                                        }
                                        self.func14(env, var3.wrapping_add(96), var15);
                                        self.func12(env, var3.wrapping_add(64), slot_var3_96_i32, slot_var3_104_i64);
                                        break 'label13;
                                        break;
                                    }
                                    slot_var3_40_i32 = 0 as i32;
                                    'label15: loop {
                                        'label16: loop {
                                            self.func14(env, var3.wrapping_add(96), var15);
                                            self.func12(env, var3.wrapping_add(80), slot_var3_96_i32, slot_var3_104_i64);
                                            let mut slot_var3_80_i32 = self.memory.load32(var3 as usize + 80) as i32;
                                            if (slot_var3_80_i32 != 1) as i32 != 0 {
                                                break 'label15;
                                            }
                                            var4 = var4.wrapping_add(-1);
                                            if var4 != 0 {
                                                continue 'label16;
                                            }
                                            break;
                                        }
                                        self.func14(env, var3.wrapping_add(96), var15);
                                        self.func12(env, var3.wrapping_add(64), slot_var3_96_i32, slot_var3_104_i64);
                                        break 'label13;
                                        break;
                                    }
                                    let mut slot_var3_64_i64 = 0 /* False */ as i64;
                                    break;
                                }
                                if (slot_var3_64_i64 == 0) as i32 != 0 {
                                    break 'label8;
                                }
                                let mut slot_var3_72_i64 = self.memory.load64(var3 as usize + 72) as i64;
                                let var41 = 0 /* TODO: bls12_381_g1_mul */
                                let var42 = 0 /* TODO: bls12_381_g1_add */
                                var12 = var42;
                                var14 = slot_var3_20_i32;
                                var4 = slot_var3_16_i32;
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var43 = Bytes::from_val(env, &val_from_i64(var9)).len() as i64
                        let var44 = val_to_i64(Bytes::from_val(env, &val_from_i64(var9)).slice(48 as u32..var43 & 18446744069414584320 | 0 as u32).into_val(env))
                        vk = var44;
                        if (!(Bytes::try_from_val(env, &val_from_i64(vk)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        let var45 = Bytes::from_val(env, &val_from_i64(vk)).len() as i64
                        if (var45 & 18446744069414584320 != 206158430208) as i32 != 0 {
                            break 'label3;
                        }
                        let var46 = self.func28(env, var3.wrapping_add(8), 0, 48);
                        var46;
                        self.func19(env, vk, var3.wrapping_add(8));
                        let var48 = self.func27(env, var3.wrapping_add(96), var3.wrapping_add(8), 48);
                        var48;
                        let var49 = self.func28(env, var3.wrapping_add(144), 0, 48);
                        var49;
                        var4 = 40;
                        var14 = var3.wrapping_add(96);
                        'label17: loop {
                            'label18: loop {
                                if (var4 == -8) as i32 != 0 {
                                    break 'label17;
                                }
                                let mut slot_var14_0_i64 = self.memory.load64(var14 as usize) as i64;
                                pub_signals = slot_var14_0_i64;
                                self.memory.store64(var3.wrapping_add(144).wrapping_add(var4) as usize, (pub_signals.wrapping_shl(56 as u32) | (pub_signals & 65280).wrapping_shl(40 as u32) | (pub_signals & 16711680).wrapping_shl(24 as u32) | (pub_signals & 4278190080).wrapping_shl(0 as u32) | (pub_signals as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (pub_signals as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (pub_signals as u64).wrapping_shr(40 as u32) as i64 & 65280 | (pub_signals as u64).wrapping_shr(56 as u32) as i64) as u64);
                                var14 = var14.wrapping_add(8);
                                var4 = var4.wrapping_add(-8);
                                continue 'label18;
                                break;
                            }
                            break;
                        }
                        'label19: loop {
                            'label20: loop {
                                let var50 = self.func25(env, var3.wrapping_add(144), 1048720, 48);
                                if (var50 == 0) as i32 != 0 {
                                    break 'label20;
                                }
                                let var51 = self.func27(env, var3.wrapping_add(96), 1048672, 48);
                                var51;
                                var4 = 0;
                                var16 = 0;
                                'label21: loop {
                                    'label22: loop {
                                        if (var4 == 48) as i32 != 0 {
                                            break 'label21;
                                        }
                                        var14 = var3.wrapping_add(96).wrapping_add(var4);
                                        vk = slot_var14_0_i64;
                                        proof = var16 as u32 as i64 & 255;
                                        let var52 = self.memory.load64(var3.wrapping_add(144).wrapping_add(var4) as usize) as i64;
                                        pub_signals = proof.wrapping_add(var52);
                                        slot_var14_0_i64 = vk.wrapping_sub(pub_signals) as i64;
                                        var16 = ((((pub_signals as u64) < proof as u64) as i32 as u32 as i64).wrapping_add(((vk as u64) < pub_signals as u64) as i32 as u32 as i64) == 1 /* True */) as i32;
                                        var4 = var4.wrapping_add(8);
                                        continue 'label22;
                                        break;
                                    }
                                    break;
                                }
                                if var16 & 255 != 0 {
                                    break 'label19;
                                }
                                let var53 = self.func28(env, var3.wrapping_add(8), 0, 48);
                                var53;
                                var14 = var3.wrapping_add(8);
                                var4 = 40;
                                'label23: loop {
                                    'label24: loop {
                                        if (var4 == -8) as i32 != 0 {
                                            break 'label23;
                                        }
                                        let var54 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                        pub_signals = var54;
                                        slot_var14_0_i64 = (pub_signals.wrapping_shl(56 as u32) | (pub_signals & 65280).wrapping_shl(40 as u32) | (pub_signals & 16711680).wrapping_shl(24 as u32) | (pub_signals & 4278190080).wrapping_shl(0 as u32) | (pub_signals as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (pub_signals as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (pub_signals as u64).wrapping_shr(40 as u32) as i64 & 65280 | (pub_signals as u64).wrapping_shr(56 as u32) as i64) as i64;
                                        var4 = var4.wrapping_add(-8);
                                        var14 = var14.wrapping_add(8);
                                        continue 'label24;
                                        break;
                                    }
                                    break;
                                }
                                let var55 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                                vk = var55;
                                if var16 & 255 != 0 {
                                    break 'label19;
                                }
                                break;
                            }
                            var4 = 0;
                            let var56 = self.func28(env, var3.wrapping_add(8), 0, 48);
                            var56;
                            self.func19(env, vk, var3.wrapping_add(8));
                            let var58 = self.func27(env, var3.wrapping_add(96), var3.wrapping_add(8), 48);
                            var58;
                            let var59 = 0 /* TODO: bytes_copy_from_linear_memory */
                            self.func20(env, var3.wrapping_add(8), var59);
                            if (slot_var3_8_i64 == 1) as i32 != 0 {
                                break 'label3;
                            }
                            pub_signals = slot_var3_16_i64;
                            let mut slot_var3_120_i64 = var11 as i64;
                            let mut slot_var3_112_i64 = var12 as i64;
                            slot_var3_104_i64 = var5 as i64;
                            let mut slot_var3_96_i64 = pub_signals as i64;
                            'label25: loop {
                                'label26: loop {
                                    if (var4 != 32) as i32 != 0 {
                                        break 'label26;
                                    }
                                    var4 = 0;
                                    'label27: loop {
                                        'label28: loop {
                                            if (var4 == 32) as i32 != 0 {
                                                break 'label27;
                                            }
                                            let var61 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                            self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, var61 as u64);
                                            var4 = var4.wrapping_add(8);
                                            continue 'label28;
                                            break;
                                        }
                                        break;
                                    }
                                    let var62 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                    pub_signals = var62;
                                    slot_var3_120_i64 = var7 as i64;
                                    slot_var3_112_i64 = var8 as i64;
                                    slot_var3_104_i64 = var6 as i64;
                                    slot_var3_96_i64 = var10 as i64;
                                    var4 = 0;
                                    'label29: loop {
                                        'label30: loop {
                                            if (var4 != 32) as i32 != 0 {
                                                break 'label30;
                                            }
                                            var4 = 0;
                                            'label31: loop {
                                                'label32: loop {
                                                    if (var4 == 32) as i32 != 0 {
                                                        break 'label31;
                                                    }
                                                    let var63 = self.memory.load64(var3.wrapping_add(96).wrapping_add(var4) as usize) as i64;
                                                    self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, var63 as u64);
                                                    var4 = var4.wrapping_add(8);
                                                    continue 'label32;
                                                    break;
                                                }
                                                break;
                                            }
                                            let var64 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                            let var65 = 0 /* TODO: bls12_381_multi_pairing_check */
                                            proof = (var65 == 1 /* True */) as i32 as u32 as i64;
                                            break 'label2;
                                            break;
                                        }
                                        self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                                        var4 = var4.wrapping_add(8);
                                        continue 'label29;
                                        break;
                                    }
                                    break;
                                }
                                self.memory.store64(var3.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                                var4 = var4.wrapping_add(8);
                                continue 'label25;
                                break;
                            }
                            break;
                        }
                        self.func22(env);
                        unreachable!();
                        break;
                    }
                    self.func13(env);
                    unreachable!();
                    break;
                }
                self.func23(env);
                unreachable!();
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var3.wrapping_add(192);
        proof
    }
    fn func17(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        'label0: loop {
            if (arg2 == arg4) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var5 = 0 /* TODO: map_unpack_to_linear_memory */
        var5;
    }
    fn func18(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var3 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var3 & 18446744069414584320 != 824633720832) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize + 8, arg1 as u64);
            var2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var2 as u64);
    }
    fn func19(&mut self, env: &Env, mut arg0: i64, mut arg1: i32) {
        let var2 = 0 /* TODO: bytes_copy_to_linear_memory */
        var2;
    }
    fn func20(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func21(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let var1 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var1
    }
    fn func22(&mut self, env: &Env) {
        self.func13(env);
        unreachable!();
    }
    fn func23(&mut self, env: &Env) {
        self.func13(env);
        unreachable!();
    }
    fn func24(&mut self, env: &Env) {
        unreachable!();
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        var3 = 0;
        'label0: loop {
            if (arg2 == 0) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    let var6 = self.memory.load8(arg0 as usize) as i32;
                    var4 = var6;
                    let var7 = self.memory.load8(arg1 as usize) as i32;
                    var5 = var7;
                    if (var4 != var5) as i32 != 0 {
                        break 'label1;
                    }
                    arg0 = arg0.wrapping_add(1);
                    arg1 = arg1.wrapping_add(1);
                    arg2 = arg2.wrapping_add(-1);
                    if (arg2 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    continue 'label2;
                    break;
                }
                break;
            }
            var3 = var4.wrapping_sub(var5);
            break;
        }
        var3
    }
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let var3 = self.func26(env, arg0, arg1, arg2);
        var3
    }
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
