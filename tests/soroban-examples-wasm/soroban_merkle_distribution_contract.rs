#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, Bytes, BytesN, crypto::Hash, Val, FromVal, IntoVal, Map, String, Symbol};

#[contract]
pub struct MerkleDistributionContract;

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
pub enum Error { AlreadyClaimed = 1, InvalidProof = 2, }

#[contractimpl]
impl MerkleDistributionContract {
    pub fn ___constructor(&mut self, mut env: Env, root_hash: soroban_sdk::BytesN<32>, mut token: soroban_sdk::Address, funding_amount: i128, funding_source: soroban_sdk::Address) {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var4 = var7.wrapping_sub(64);
        self.global0 = var4;
        self.func24(env, var4, root_hash);
        'label0: loop {
            let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(token)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
            root_hash = slot_var4_8_i64;
            self.func25(env, var4, funding_amount);
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(funding_source)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
            funding_amount = slot_var4_24_i64;
            let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
            var5 = slot_var4_16_i64;
            var6 = 0;
            let var10 = self.func19(env, 0, var6);
            self.func26(env, var10, root_hash);
            let var12 = self.func19(env, 1, var6);
            self.func26(env, var12, token);
            let var14 = val_to_i64(env.current_contract_address().into_val(env))
            root_hash = var14;
            let var15 = self.func27(env, var5, funding_amount);
            let mut slot_var4_56_i64 = var15 as i64;
            let mut slot_var4_48_i64 = root_hash as i64;
            let mut slot_var4_40_i64 = funding_source as i64;
            'label1: loop {
                'label2: loop {
                    if (var6 != 24) as i32 != 0 {
                        break 'label2;
                    }
                    var6 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var6 == 24) as i32 != 0 {
                                break 'label3;
                            }
                            let var16 = self.memory.load64(var4.wrapping_add(40).wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var4.wrapping_add(var6) as usize, var16 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func28(env, token, var17);
                    self.global0 = var4.wrapping_add(64);
                    return 0 /* Void */;
                    break;
                }
                self.memory.store64(var4.wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn claim(&mut self, mut env: Env, index: u32, mut receiver: soroban_sdk::Address, amount: i128, proof: soroban_sdk::Vec<soroban_sdk::BytesN<32>>) -> Result<(), soroban_sdk::Error> {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let var18 = self.global0;
        var4 = var18.wrapping_sub(160);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                if (index & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(receiver)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                self.func25(env, var4.wrapping_add(64), amount);
                let mut slot_var4_64_i32 = self.memory.load32(var4 as usize + 64) as i32;
                if (slot_var4_64_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_88_i64 = self.memory.load64(var4 as usize + 88) as i64;
                var5 = slot_var4_88_i64;
                let mut slot_var4_80_i64 = self.memory.load64(var4 as usize + 80) as i64;
                var6 = slot_var4_80_i64;
                'label2: loop {
                    var7 = (index as u64).wrapping_shr(32 as u32) as i64 as i32;
                    let var20 = self.func19(env, 2, var7);
                    let var21 = self.func30(env, var20);
                    if (var21 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    index = 4294967299 /* Error(Contract, #1) */;
                    break 'label0;
                    break;
                }
                self.func31(env, var4.wrapping_add(32), var6, var5);
                let mut slot_var4_32_i32 = self.memory.load32(var4 as usize + 32) as i32;
                if (slot_var4_32_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_40_i64 = self.memory.load64(var4 as usize + 40) as i64;
                let mut slot_var4_72_i64 = slot_var4_40_i64 as i64;
                let mut slot_var4_64_i64 = receiver as i64;
                slot_var4_80_i64 = (index & 4294967295) as i64;
                amount = 0;
                var8 = (var4.wrapping_add(64) as u32 as i64).wrapping_shl(32 as u32) | 0;
                let var23 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
                let var24 = val_to_i64(Bytes::from_val(env, &val_from_i64(var23)).into()) /* TODO: serialize_to_bytes */
                let var25 = val_to_i64(env.crypto().sha256(&Bytes::from_val(env, &val_from_i64(var24))).into())
                var9 = var25;
                let var26 = Vec::<Val>::from_val(env, &val_from_i64(proof)).len() as i64
                var10 = (var26 as u64).wrapping_shr(32 as u32) as i64;
                var11 = var4.wrapping_add(64).wrapping_add(32);
                index = 0 /* False */;
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            if (var10 == index) as i32 != 0 {
                                break 'label4;
                            }
                            let var27 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(proof)).get_unchecked(amount as u32))
                            self.func24(env, var4.wrapping_add(64), var27);
                            if (index == 4294967295) as i32 != 0 {
                                break 'label3;
                            }
                            if ((slot_var4_64_i64 == 0) as i32 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            var12 = slot_var4_72_i64;
                            var13 = var4.wrapping_add(64).wrapping_add(24);
                            let mut slot_var13_0_i64 = 0 /* False */ as i64;
                            var14 = var4.wrapping_add(64).wrapping_add(16);
                            let mut slot_var14_0_i64 = 0 /* False */ as i64;
                            var15 = var4.wrapping_add(64).wrapping_add(8);
                            let mut slot_var15_0_i64 = 0 /* False */ as i64;
                            slot_var4_64_i64 = 0 /* False */ as i64;
                            self.func32(env, var9, var4.wrapping_add(64));
                            self.memory.store64(var4.wrapping_add(24) as usize, slot_var13_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(16) as usize, slot_var14_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(8) as usize, slot_var15_0_i64 as u64);
                            let mut slot_var4_0_i64 = slot_var4_64_i64 as i64;
                            slot_var13_0_i64 = 0 /* False */ as i64;
                            slot_var14_0_i64 = 0 /* False */ as i64;
                            slot_var15_0_i64 = 0 /* False */ as i64;
                            slot_var4_64_i64 = 0 /* False */ as i64;
                            self.func32(env, var12, var4.wrapping_add(64));
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(24) as usize, slot_var13_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(16) as usize, slot_var14_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(8) as usize, slot_var15_0_i64 as u64);
                            let mut slot_var4_32_i64 = slot_var4_64_i64 as i64;
                            let var31 = self.func37(env, var4, var4.wrapping_add(32), 32);
                            var16 = (var31 < 0) as i32;
                            var17 = { let a = var4.wrapping_add(32); let b = var4; if var16 != 0 { a } else { b } };
                            let mut slot_var17_0_i64 = self.memory.load64(var17 as usize) as i64;
                            let mut slot_var11_0_i64 = slot_var17_0_i64 as i64;
                            var16 = { let a = var4; let b = var4.wrapping_add(32); if var16 != 0 { a } else { b } };
                            let var32 = self.memory.load64(var16.wrapping_add(24) as usize) as i64;
                            slot_var13_0_i64 = var32 as i64;
                            let var33 = self.memory.load64(var16.wrapping_add(16) as usize) as i64;
                            slot_var14_0_i64 = var33 as i64;
                            let var34 = self.memory.load64(var16.wrapping_add(8) as usize) as i64;
                            slot_var15_0_i64 = var34 as i64;
                            let var35 = self.memory.load64(var17.wrapping_add(8) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(8) as usize, var35 as u64);
                            let var36 = self.memory.load64(var17.wrapping_add(16) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(16) as usize, var36 as u64);
                            let var37 = self.memory.load64(var17.wrapping_add(24) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(24) as usize, var37 as u64);
                            let mut slot_var16_0_i64 = self.memory.load64(var16 as usize) as i64;
                            slot_var4_64_i64 = slot_var16_0_i64 as i64;
                            amount = amount.wrapping_add(4294967296);
                            index = index.wrapping_add(1 /* True */);
                            let var38 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                            let var39 = val_to_i64(env.crypto().sha256(&Bytes::from_val(env, &val_from_i64(var38))).into())
                            var9 = var39;
                            continue 'label5;
                            break;
                        }
                        break;
                    }
                    'label6: loop {
                        let var40 = self.func19(env, 0, var4);
                        index = var40;
                        let var41 = self.func30(env, index);
                        if (var41 == 0) as i32 != 0 {
                            break 'label6;
                        }
                        let var42 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) } };
                        self.func24(env, var4.wrapping_add(64), var42);
                        if (slot_var4_64_i32 == 1) as i32 != 0 {
                            break 'label1;
                        }
                        'label7: loop {
                            let var44 = { let a = val_from_i64(slot_var4_72_i64); let b = val_from_i64(var9); if a < b { -1 } else if a > b { 1 } else { 0 } }
                            if (var44 == 0 /* False */) as i32 != 0 {
                                break 'label7;
                            }
                            index = 8589934595 /* Error(Contract, #2) */;
                            break 'label0;
                            break;
                        }
                        let var45 = self.func19(env, 1, var4);
                        index = var45;
                        let var46 = self.func30(env, index);
                        if (var46 == 0) as i32 != 0 {
                            break 'label6;
                        }
                        let var47 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) } };
                        index = var47;
                        if (!(Address::try_from_val(env, &val_from_i64(index)).is_ok())) as i32 != 0 {
                            break 'label1;
                        }
                        let var48 = val_to_i64(env.current_contract_address().into_val(env))
                        amount = var48;
                        let var49 = self.func27(env, var6, var5);
                        let mut slot_var4_152_i64 = var49 as i64;
                        let mut slot_var4_144_i64 = receiver as i64;
                        let mut slot_var4_136_i64 = amount as i64;
                        var13 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var13 != 24) as i32 != 0 {
                                    break 'label9;
                                }
                                var13 = 0;
                                'label10: loop {
                                    'label11: loop {
                                        if (var13 == 24) as i32 != 0 {
                                            break 'label10;
                                        }
                                        let var50 = self.memory.load64(var4.wrapping_add(136).wrapping_add(var13) as usize) as i64;
                                        self.memory.store64(var4.wrapping_add(64).wrapping_add(var13) as usize, var50 as u64);
                                        var13 = var13.wrapping_add(8);
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                let var51 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                self.func28(env, index, var51);
                                index = 0 /* Void */;
                                let var53 = self.func19(env, 2, var7);
                                self.func26(env, var53, 0 /* Void */);
                                break 'label0;
                                break;
                            }
                            self.memory.store64(var4.wrapping_add(64).wrapping_add(var13) as usize, 0 /* Void */ as u64);
                            var13 = var13.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    self.func34(env);
                    unreachable!();
                    break;
                }
                self.func35(env);
                unreachable!();
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var4.wrapping_add(160);
        index
    }
}

#[allow(dead_code)]
impl MerkleDistributionContract {
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match arg0 {
                                    0 => break 'label5,
                                    1 => break 'label4,
                                    2 => break 'label3,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            self.func20(env, var2, 1048576, 8);
                            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                            if slot_var2_0_i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                            self.func21(env, var2, slot_var2_8_i64);
                            break 'label2;
                            break;
                        }
                        self.func20(env, var2, 1048584, 12);
                        if slot_var2_0_i32 != 0 {
                            break 'label1;
                        }
                        self.func21(env, var2, slot_var2_8_i64);
                        break 'label2;
                        break;
                    }
                    self.func20(env, var2, 1048596, 7);
                    if slot_var2_0_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var2_0_i64 = slot_var2_8_i64 as i64;
                    slot_var2_8_i64 = ((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
                    let var10 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    var3 = var10;
                    break 'label0;
                    break;
                }
                var3 = slot_var2_8_i64;
                if (slot_var2_0_i64 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var2.wrapping_add(16);
        var3
    }
    fn func20(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (arg2 as u32 > 9 as u32) as i32 != 0 {
                    break 'label1;
                }
                var3 = 0 /* False */;
                var4 = arg2;
                var5 = arg1;
                'label2: loop {
                    'label3: loop {
                        if var4 != 0 {
                            break 'label3;
                        }
                        var3 = var3.wrapping_shl(0 as u32) | 0 /* Symbol() */;
                        break 'label0;
                        break;
                    }
                    var6 = 1;
                    'label4: loop {
                        let var8 = self.memory.load8(var5 as usize) as i32;
                        var7 = var8;
                        if (var7 == 95) as i32 != 0 {
                            break 'label4;
                        }
                        'label5: loop {
                            if (((var7.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            'label6: loop {
                                if (((var7.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                    break 'label6;
                                }
                                if ((var7.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                    break 'label1;
                                }
                                var6 = var7.wrapping_add(-59);
                                break 'label4;
                                break;
                            }
                            var6 = var7.wrapping_add(-53);
                            break 'label4;
                            break;
                        }
                        var6 = var7.wrapping_add(-46);
                        break;
                    }
                    var3 = var3.wrapping_shl(0 as u32) | var6 as u32 as i64 & 255;
                    var4 = var4.wrapping_add(-1);
                    var5 = var5.wrapping_add(1);
                    continue 'label2;
                    break;
                }
                break;
            }
            let var9 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
            var3 = var9;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, var3 as u64);
    }
    fn func21(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func22(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func23(&mut self, env: &Env, mut root_hash: i64, mut token: i64, mut funding_amount: i64, mut funding_source: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var4 = var7.wrapping_sub(64);
        self.global0 = var4;
        self.func24(env, var4, root_hash);
        'label0: loop {
            let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(token)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
            root_hash = slot_var4_8_i64;
            self.func25(env, var4, funding_amount);
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(funding_source)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
            funding_amount = slot_var4_24_i64;
            let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
            var5 = slot_var4_16_i64;
            var6 = 0;
            let var10 = self.func19(env, 0, var6);
            self.func26(env, var10, root_hash);
            let var12 = self.func19(env, 1, var6);
            self.func26(env, var12, token);
            let var14 = val_to_i64(env.current_contract_address().into_val(env))
            root_hash = var14;
            let var15 = self.func27(env, var5, funding_amount);
            let mut slot_var4_56_i64 = var15 as i64;
            let mut slot_var4_48_i64 = root_hash as i64;
            let mut slot_var4_40_i64 = funding_source as i64;
            'label1: loop {
                'label2: loop {
                    if (var6 != 24) as i32 != 0 {
                        break 'label2;
                    }
                    var6 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var6 == 24) as i32 != 0 {
                                break 'label3;
                            }
                            let var16 = self.memory.load64(var4.wrapping_add(40).wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var4.wrapping_add(var6) as usize, var16 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func28(env, token, var17);
                    self.global0 = var4.wrapping_add(64);
                    return 0 /* Void */;
                    break;
                }
                self.memory.store64(var4.wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func24(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        var2 = arg1 as i32 & 255;
                        if (var2 == 69) as i32 != 0 {
                            break 'label3;
                        }
                        if (var2 != 11) as i32 != 0 {
                            break 'label1;
                        }
                        self.memory.store64(arg0 as usize + 24, arg1.wrapping_shr(63 as u32) as u64);
                        self.memory.store64(arg0 as usize + 16, arg1.wrapping_shr(0 as u32) as u64);
                        break 'label2;
                        break;
                    }
                    let var4 = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64)
                    var3 = var4;
                    let var5 = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
                    arg1 = var5;
                    self.memory.store64(arg0 as usize + 24, var3 as u64);
                    self.memory.store64(arg0 as usize + 16, arg1 as u64);
                    break;
                }
                arg1 = 0 /* False */;
                break 'label0;
                break;
            }
            self.memory.store64(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
            arg1 = 1 /* True */;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
    }
    fn func26(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn func27(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func31(env, var2, arg0, arg1);
        'label0: loop {
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if (slot_var2_0_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
        arg1 = slot_var2_8_i64;
        self.global0 = var2.wrapping_add(16);
        arg1
    }
    fn func28(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        'label0: loop {
            let var2 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(transfer)), Vec::<Val>::from_val(env, &val_from_i64(arg1))))
            if (var2 & 255 == 0 /* Void */) as i32 != 0 {
                break 'label0;
            }
            self.func35(env);
            unreachable!();
            break;
        }
    }
    fn func29(&mut self, env: &Env, mut index: i64, mut receiver: i64, mut amount: i64, mut proof: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let var18 = self.global0;
        var4 = var18.wrapping_sub(160);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                if (index & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(receiver)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                self.func25(env, var4.wrapping_add(64), amount);
                let mut slot_var4_64_i32 = self.memory.load32(var4 as usize + 64) as i32;
                if (slot_var4_64_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_88_i64 = self.memory.load64(var4 as usize + 88) as i64;
                var5 = slot_var4_88_i64;
                let mut slot_var4_80_i64 = self.memory.load64(var4 as usize + 80) as i64;
                var6 = slot_var4_80_i64;
                'label2: loop {
                    var7 = (index as u64).wrapping_shr(32 as u32) as i64 as i32;
                    let var20 = self.func19(env, 2, var7);
                    let var21 = self.func30(env, var20);
                    if (var21 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    index = 4294967299 /* Error(Contract, #1) */;
                    break 'label0;
                    break;
                }
                self.func31(env, var4.wrapping_add(32), var6, var5);
                let mut slot_var4_32_i32 = self.memory.load32(var4 as usize + 32) as i32;
                if (slot_var4_32_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_40_i64 = self.memory.load64(var4 as usize + 40) as i64;
                let mut slot_var4_72_i64 = slot_var4_40_i64 as i64;
                let mut slot_var4_64_i64 = receiver as i64;
                slot_var4_80_i64 = (index & 4294967295) as i64;
                amount = 0;
                var8 = (var4.wrapping_add(64) as u32 as i64).wrapping_shl(32 as u32) | 0;
                let var23 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
                let var24 = val_to_i64(Bytes::from_val(env, &val_from_i64(var23)).into()) /* TODO: serialize_to_bytes */
                let var25 = val_to_i64(env.crypto().sha256(&Bytes::from_val(env, &val_from_i64(var24))).into())
                var9 = var25;
                let var26 = Vec::<Val>::from_val(env, &val_from_i64(proof)).len() as i64
                var10 = (var26 as u64).wrapping_shr(32 as u32) as i64;
                var11 = var4.wrapping_add(64).wrapping_add(32);
                index = 0 /* False */;
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            if (var10 == index) as i32 != 0 {
                                break 'label4;
                            }
                            let var27 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(proof)).get_unchecked(amount as u32))
                            self.func24(env, var4.wrapping_add(64), var27);
                            if (index == 4294967295) as i32 != 0 {
                                break 'label3;
                            }
                            if ((slot_var4_64_i64 == 0) as i32 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            var12 = slot_var4_72_i64;
                            var13 = var4.wrapping_add(64).wrapping_add(24);
                            let mut slot_var13_0_i64 = 0 /* False */ as i64;
                            var14 = var4.wrapping_add(64).wrapping_add(16);
                            let mut slot_var14_0_i64 = 0 /* False */ as i64;
                            var15 = var4.wrapping_add(64).wrapping_add(8);
                            let mut slot_var15_0_i64 = 0 /* False */ as i64;
                            slot_var4_64_i64 = 0 /* False */ as i64;
                            self.func32(env, var9, var4.wrapping_add(64));
                            self.memory.store64(var4.wrapping_add(24) as usize, slot_var13_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(16) as usize, slot_var14_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(8) as usize, slot_var15_0_i64 as u64);
                            let mut slot_var4_0_i64 = slot_var4_64_i64 as i64;
                            slot_var13_0_i64 = 0 /* False */ as i64;
                            slot_var14_0_i64 = 0 /* False */ as i64;
                            slot_var15_0_i64 = 0 /* False */ as i64;
                            slot_var4_64_i64 = 0 /* False */ as i64;
                            self.func32(env, var12, var4.wrapping_add(64));
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(24) as usize, slot_var13_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(16) as usize, slot_var14_0_i64 as u64);
                            self.memory.store64(var4.wrapping_add(32).wrapping_add(8) as usize, slot_var15_0_i64 as u64);
                            let mut slot_var4_32_i64 = slot_var4_64_i64 as i64;
                            let var31 = self.func37(env, var4, var4.wrapping_add(32), 32);
                            var16 = (var31 < 0) as i32;
                            var17 = { let a = var4.wrapping_add(32); let b = var4; if var16 != 0 { a } else { b } };
                            let mut slot_var17_0_i64 = self.memory.load64(var17 as usize) as i64;
                            let mut slot_var11_0_i64 = slot_var17_0_i64 as i64;
                            var16 = { let a = var4; let b = var4.wrapping_add(32); if var16 != 0 { a } else { b } };
                            let var32 = self.memory.load64(var16.wrapping_add(24) as usize) as i64;
                            slot_var13_0_i64 = var32 as i64;
                            let var33 = self.memory.load64(var16.wrapping_add(16) as usize) as i64;
                            slot_var14_0_i64 = var33 as i64;
                            let var34 = self.memory.load64(var16.wrapping_add(8) as usize) as i64;
                            slot_var15_0_i64 = var34 as i64;
                            let var35 = self.memory.load64(var17.wrapping_add(8) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(8) as usize, var35 as u64);
                            let var36 = self.memory.load64(var17.wrapping_add(16) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(16) as usize, var36 as u64);
                            let var37 = self.memory.load64(var17.wrapping_add(24) as usize) as i64;
                            self.memory.store64(var11.wrapping_add(24) as usize, var37 as u64);
                            let mut slot_var16_0_i64 = self.memory.load64(var16 as usize) as i64;
                            slot_var4_64_i64 = slot_var16_0_i64 as i64;
                            amount = amount.wrapping_add(4294967296);
                            index = index.wrapping_add(1 /* True */);
                            let var38 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                            let var39 = val_to_i64(env.crypto().sha256(&Bytes::from_val(env, &val_from_i64(var38))).into())
                            var9 = var39;
                            continue 'label5;
                            break;
                        }
                        break;
                    }
                    'label6: loop {
                        let var40 = self.func19(env, 0, var4);
                        index = var40;
                        let var41 = self.func30(env, index);
                        if (var41 == 0) as i32 != 0 {
                            break 'label6;
                        }
                        let var42 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) } };
                        self.func24(env, var4.wrapping_add(64), var42);
                        if (slot_var4_64_i32 == 1) as i32 != 0 {
                            break 'label1;
                        }
                        'label7: loop {
                            let var44 = { let a = val_from_i64(slot_var4_72_i64); let b = val_from_i64(var9); if a < b { -1 } else if a > b { 1 } else { 0 } }
                            if (var44 == 0 /* False */) as i32 != 0 {
                                break 'label7;
                            }
                            index = 8589934595 /* Error(Contract, #2) */;
                            break 'label0;
                            break;
                        }
                        let var45 = self.func19(env, 1, var4);
                        index = var45;
                        let var46 = self.func30(env, index);
                        if (var46 == 0) as i32 != 0 {
                            break 'label6;
                        }
                        let var47 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(index)).unwrap_or(val_from_i64(0))) } };
                        index = var47;
                        if (!(Address::try_from_val(env, &val_from_i64(index)).is_ok())) as i32 != 0 {
                            break 'label1;
                        }
                        let var48 = val_to_i64(env.current_contract_address().into_val(env))
                        amount = var48;
                        let var49 = self.func27(env, var6, var5);
                        let mut slot_var4_152_i64 = var49 as i64;
                        let mut slot_var4_144_i64 = receiver as i64;
                        let mut slot_var4_136_i64 = amount as i64;
                        var13 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var13 != 24) as i32 != 0 {
                                    break 'label9;
                                }
                                var13 = 0;
                                'label10: loop {
                                    'label11: loop {
                                        if (var13 == 24) as i32 != 0 {
                                            break 'label10;
                                        }
                                        let var50 = self.memory.load64(var4.wrapping_add(136).wrapping_add(var13) as usize) as i64;
                                        self.memory.store64(var4.wrapping_add(64).wrapping_add(var13) as usize, var50 as u64);
                                        var13 = var13.wrapping_add(8);
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                let var51 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                self.func28(env, index, var51);
                                index = 0 /* Void */;
                                let var53 = self.func19(env, 2, var7);
                                self.func26(env, var53, 0 /* Void */);
                                break 'label0;
                                break;
                            }
                            self.memory.store64(var4.wrapping_add(64).wrapping_add(var13) as usize, 0 /* Void */ as u64);
                            var13 = var13.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    self.func34(env);
                    unreachable!();
                    break;
                }
                self.func35(env);
                unreachable!();
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var4.wrapping_add(160);
        index
    }
    fn func30(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func31(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        'label0: loop {
            'label1: loop {
                if (arg1.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                    break 'label1;
                }
                if (arg1 ^ arg1 | arg2 ^ arg1.wrapping_shr(63 as u32) != 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                arg1 = arg1.wrapping_shl(0 as u32) | 0;
                break 'label0;
                break;
            }
            let var3 = val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)))
            arg1 = var3;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
    }
    fn func32(&mut self, env: &Env, mut arg0: i64, mut arg1: i32) {
        let var2 = 0 /* TODO: bytes_copy_to_linear_memory */
        var2;
    }
    fn func33(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let var1 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) } }
        var1
    }
    fn func34(&mut self, env: &Env) {
        self.func35(env);
        unreachable!();
    }
    fn func35(&mut self, env: &Env) {
        self.func36(env);
        unreachable!();
    }
    fn func36(&mut self, env: &Env) {
        unreachable!();
    }
    fn func37(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
}
