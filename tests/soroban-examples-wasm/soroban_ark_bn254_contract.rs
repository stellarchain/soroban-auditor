#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct ArkBn254Contract;

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
pub struct MockProof { pub g1: soroban_sdk::BytesN<64>, pub g2: soroban_sdk::BytesN<128>, }

#[contractimpl]
impl ArkBn254Contract {
    pub fn mock_verify(&mut self, mut env: Env, proof: MockProof) -> bool {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i64 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let var20 = self.global0;
        var1 = var20.wrapping_sub(9600);
        self.global0 = var1;
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 16) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var1.wrapping_add(1856).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var21 = 0 /* TODO: map_unpack_to_linear_memory */
                var21;
                let mut slot_var1_1856_i64 = self.memory.load64(var1 as usize + 1856) as i64;
                proof = slot_var1_1856_i64;
                if (!(Bytes::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var22 = Bytes::from_val(env, &val_from_i64(proof)).len() as i64
                if (var22 & 18446744069414584320 != 274877906944) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_1864_i64 = self.memory.load64(var1 as usize + 1864) as i64;
                var3 = slot_var1_1864_i64;
                if (!(Bytes::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var23 = Bytes::from_val(env, &val_from_i64(var3)).len() as i64
                if (var23 & 18446744069414584320 != 549755813888) as i32 != 0 {
                    break 'label3;
                }
                var2 = 0;
                let var24 = self.func122(env, var1.wrapping_add(8), 0, 64);
                var24;
                self.func84(env, proof, var1.wrapping_add(8), 64);
                let mut slot_var1_7300_i32 = 64 as i32;
                let mut slot_var1_7296_i32 = var1.wrapping_add(8) as i32;
                self.func71(env, var1.wrapping_add(8832));
                'label4: loop {
                    'label5: loop {
                        if (var2 != 24) as i32 != 0 {
                            break 'label5;
                        }
                        self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8856));
                        'label6: loop {
                            let var28 = self.memory.load8(var1 as usize + 1856) as i32;
                            if (var28 == 2) as i32 != 0 {
                                break 'label6;
                            }
                            let var29 = self.memory.load8(var1 as usize + 1856) as i64;
                            if (var29 != 0 /* Void */) as i32 != 0 {
                                break 'label2;
                            }
                            break;
                        }
                        self.func48(env, 4, var2);
                        proof = 0 /* False */;
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        let mut slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        let mut slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        let mut slot_var5_0_i64 = 0 /* False */ as i64;
                        let mut slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var31 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var31 as u64);
                        var6 = var1.wrapping_add(1856).wrapping_add(32);
                        let var32 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var6_0_i64 = var32 as i64;
                        var7 = var1.wrapping_add(1856).wrapping_add(24);
                        let var33 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var7_0_i64 = var33 as i64;
                        let mut slot_var1_8832_i64 = self.memory.load64(var1 as usize + 8832) as i64;
                        let mut slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        let mut slot_var1_1912_i32 = 0 as i32;
                        let mut slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        var8 = var1.wrapping_add(1088).wrapping_add(32);
                        let mut slot_var1_1860_i32 = var8 as i32;
                        let mut slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        let mut slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        'label7: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label7;
                            }
                            let mut slot_var8_0_i64 = slot_var6_0_i64 as i64;
                            slot_var2_0_i64 = slot_var7_0_i64 as i64;
                            let var36 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var4_0_i64 = var36 as i64;
                            let mut slot_var1_1096_i64 = slot_var1_1864_i64 as i64;
                            proof = 1 /* True */;
                            break;
                        }
                        slot_var1_1088_i64 = proof as i64;
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func52(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let mut slot_var1_9216_i32 = self.memory.load32(var1 as usize + 9216) as i32;
                        if slot_var1_9216_i32 & 1 != 0 {
                            break 'label2;
                        }
                        let var38 = self.memory.load64(var1.wrapping_add(9236) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8064).wrapping_add(8) as usize, var38 as u64);
                        let var39 = self.memory.load64(var1.wrapping_add(9244) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8080) as usize, var39 as u64);
                        let var40 = self.memory.load32(var1.wrapping_add(9252) as usize) as i32;
                        self.memory.store32(var1.wrapping_add(8064).wrapping_add(24) as usize, var40 as u32);
                        let mut slot_var1_9228_i64 = self.memory.load64(var1 as usize + 9228) as i64;
                        let mut slot_var1_8064_i64 = slot_var1_9228_i64 as i64;
                        let mut slot_var1_9224_i32 = self.memory.load32(var1 as usize + 9224) as i32;
                        var7 = slot_var1_9224_i32;
                        self.func71(env, var1.wrapping_add(8832));
                        var2 = 0;
                        'label8: loop {
                            'label9: loop {
                                'label10: loop {
                                    if (var2 != 24) as i32 != 0 {
                                        break 'label10;
                                    }
                                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8856));
                                    let var43 = self.memory.load8(var1 as usize + 1856) as i32;
                                    if (var43 == 2) as i32 != 0 {
                                        break 'label8;
                                    }
                                    let var44 = self.memory.load8(var1 as usize + 1856) as i64;
                                    if (var44 == 0 /* Void */) as i32 != 0 {
                                        break 'label8;
                                    }
                                    break 'label2;
                                    break;
                                }
                                self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8832).wrapping_add(var2));
                                'label11: loop {
                                    let var46 = self.memory.load8(var1 as usize + 1856) as i32;
                                    if (var46 == 2) as i32 != 0 {
                                        break 'label11;
                                    }
                                    let var47 = self.memory.load8(var1 as usize + 1856) as i64;
                                    if (var47 != 0 /* Void */) as i32 != 0 {
                                        break 'label2;
                                    }
                                    break;
                                }
                                var2 = var2.wrapping_add(8);
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var48 = self.func40(env, var1.wrapping_add(8863));
                        var2 = var48;
                        self.memory.store8(var1 as usize + 1856, 4 as u8);
                        self.func50(env, var1.wrapping_add(1088), var2, var1.wrapping_add(1856));
                        'label12: loop {
                            'label13: loop {
                                let var50 = self.memory.load8(var1 as usize + 1088) as i32;
                                if (var50 != 5) as i32 != 0 {
                                    break 'label13;
                                }
                                let var51 = self.memory.load8(var1 as usize + 1089) as i32;
                                var8 = var51;
                                break 'label12;
                                break;
                            }
                            proof = slot_var1_1088_i64;
                            if (proof & 255 != 0) as i32 != 0 {
                                break 'label2;
                            }
                            var8 = (proof as u64).wrapping_shr(0 as u32) as i64 as i32;
                            break;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var52 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var52 as u64);
                        var9 = var1.wrapping_add(1856).wrapping_add(32);
                        let var53 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var9_0_i64 = var53 as i64;
                        var10 = var1.wrapping_add(1856).wrapping_add(24);
                        let var54 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var10_0_i64 = var54 as i64;
                        slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        slot_var1_1912_i32 = 0 as i32;
                        slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        slot_var1_1860_i32 = var1.wrapping_add(1088).wrapping_add(32) as i32;
                        slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        var6 = 129;
                        'label14: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label14;
                            }
                            slot_var2_0_i64 = slot_var9_0_i64 as i64;
                            slot_var4_0_i64 = slot_var10_0_i64 as i64;
                            let var57 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var5_0_i64 = var57 as i64;
                            slot_var1_1088_i64 = slot_var1_1864_i64 as i64;
                            var6 = var8;
                            break;
                        }
                        self.memory.store8(var1 as usize + 1120, var6 as u8);
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func51(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let var59 = self.memory.load8(var1 as usize + 9248) as i32;
                        var2 = var59;
                        if (var2 == 129) as i32 != 0 {
                            break 'label2;
                        }
                        proof = slot_var1_9216_i32;
                        var4 = var1.wrapping_add(7680).wrapping_add(16);
                        var5 = var1.wrapping_add(9216).wrapping_add(24);
                        slot_var4_0_i64 = slot_var5_0_i64 as i64;
                        var6 = var1.wrapping_add(7680).wrapping_add(8);
                        var8 = var1.wrapping_add(9216).wrapping_add(16);
                        slot_var6_0_i64 = slot_var8_0_i64 as i64;
                        let mut slot_var1_7680_i64 = slot_var1_9224_i32 as i64;
                        'label15: loop {
                            'label16: loop {
                                if (var2 == 64) as i32 != 0 {
                                    break 'label16;
                                }
                                let var60 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(1868) as usize, var60 as u64);
                                let var61 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(1876) as usize, var61 as u64);
                                let var62 = self.memory.load32(var1.wrapping_add(8064).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(1884) as usize, var62 as u32);
                                self.memory.store64(var1.wrapping_add(1904) as usize, slot_var6_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(1912) as usize, slot_var4_0_i64 as u64);
                                slot_var1_1856_i32 = var7 as i32;
                                let mut slot_var1_1860_i64 = slot_var1_8064_i64 as i64;
                                let mut slot_var1_1888_i64 = proof as i64;
                                let mut slot_var1_1896_i64 = slot_var1_7680_i64 as i64;
                                self.memory.store8(var1 as usize + 1920, 0 as u8);
                                self.func76(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                self.func70(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                let var65 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(24) as usize) as i64;
                                slot_var5_0_i64 = var65 as i64;
                                let var66 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(16) as usize) as i64;
                                slot_var8_0_i64 = var66 as i64;
                                let var67 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(9216).wrapping_add(8) as usize, var67 as u64);
                                let mut slot_var1_9216_i64 = slot_var1_1088_i64 as i64;
                                'label17: loop {
                                    let var68 = self.func29(env, 1050656);
                                    if var68 != 0 {
                                        break 'label17;
                                    }
                                    self.func13(env, var1.wrapping_add(9216), 1050656);
                                    break;
                                }
                                var2 = var1.wrapping_add(1888);
                                let var70 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(24) as usize, var70 as u64);
                                let var71 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(16) as usize, var71 as u64);
                                let var72 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(8) as usize, var72 as u64);
                                slot_var1_8832_i64 = slot_var1_9216_i64 as i64;
                                var4 = var1.wrapping_add(1088).wrapping_add(24);
                                slot_var4_0_i64 = 0 /* False */ as i64;
                                var5 = var1.wrapping_add(1088).wrapping_add(16);
                                slot_var5_0_i64 = 0 /* False */ as i64;
                                var6 = var1.wrapping_add(1088).wrapping_add(8);
                                slot_var6_0_i64 = 0 /* False */ as i64;
                                slot_var1_1088_i64 = 0 /* False */ as i64;
                                'label18: loop {
                                    let var73 = self.func29(env, var1.wrapping_add(1088));
                                    if var73 != 0 {
                                        break 'label18;
                                    }
                                    slot_var4_0_i64 = 0 /* False */ as i64;
                                    slot_var5_0_i64 = 0 /* False */ as i64;
                                    slot_var6_0_i64 = 0 /* False */ as i64;
                                    slot_var1_1088_i64 = 0 /* False */ as i64;
                                    self.func13(env, var1.wrapping_add(8832), var1.wrapping_add(1088));
                                    break;
                                }
                                self.func76(env, var1.wrapping_add(1088), var2);
                                let var76 = self.func44(env, var1.wrapping_add(1088), var1.wrapping_add(8832));
                                if (var76 == 0) as i32 != 0 {
                                    break 'label2;
                                }
                                proof = slot_var1_1856_i64;
                                let var77 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1864), 56);
                                var77;
                                var9 = 0;
                                break 'label15;
                                break;
                            }
                            let var78 = self.func122(env, var1.wrapping_add(1088), 0, 56);
                            var78;
                            proof = 0 /* False */;
                            var9 = 1;
                            break;
                        }
                        let var79 = self.func121(env, var1.wrapping_add(80), var1.wrapping_add(1088), 56);
                        var79;
                        let var80 = self.memory.load32(var1.wrapping_add(1859) as usize) as i32;
                        let mut slot_var1_75_i32 = var80 as i32;
                        let mut slot_var1_72_i32 = slot_var1_1856_i32 as i32;
                        let var81 = self.func122(env, var1.wrapping_add(136), 0, 128);
                        var81;
                        self.func84(env, var3, var1.wrapping_add(136), 128);
                        let mut slot_var1_652_i32 = 128 as i32;
                        let mut slot_var1_648_i32 = var1.wrapping_add(136) as i32;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(8);
                        var4 = var1.wrapping_add(1876);
                        slot_var2_0_i64 = slot_var4_0_i64 as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(16);
                        var6 = var1.wrapping_add(1884);
                        slot_var5_0_i64 = slot_var6_0_i64 as i64;
                        var7 = var1.wrapping_add(1088).wrapping_add(24);
                        var8 = var1.wrapping_add(1892);
                        let mut slot_var7_0_i32 = slot_var8_0_i64 as i32;
                        let mut slot_var1_1868_i64 = self.memory.load64(var1 as usize + 1868) as i64;
                        slot_var1_1088_i64 = slot_var1_1868_i64 as i64;
                        var10 = slot_var1_1864_i64;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(264).wrapping_add(8) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(264).wrapping_add(16) as usize, slot_var6_0_i64 as u64);
                        self.memory.store32(var1.wrapping_add(264).wrapping_add(24) as usize, slot_var8_0_i64 as u32);
                        self.memory.store64(var1.wrapping_add(2624).wrapping_add(8) as usize, slot_var2_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(2624).wrapping_add(16) as usize, slot_var5_0_i64 as u64);
                        self.memory.store32(var1.wrapping_add(2624).wrapping_add(24) as usize, slot_var7_0_i32 as u32);
                        let mut slot_var1_264_i64 = slot_var1_1868_i64 as i64;
                        let mut slot_var1_2624_i64 = slot_var1_1088_i64 as i64;
                        var7 = slot_var1_1864_i64;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        let var86 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8064).wrapping_add(8) as usize, var86 as u64);
                        let var87 = self.memory.load64(var1.wrapping_add(1888) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8080) as usize, var87 as u64);
                        slot_var1_8064_i64 = slot_var1_1872_i64 as i64;
                        var3 = slot_var1_1864_i64;
                        self.func71(env, var1.wrapping_add(8832));
                        var2 = 0;
                        'label19: loop {
                            'label20: loop {
                                'label21: loop {
                                    'label22: loop {
                                        if (var2 != 24) as i32 != 0 {
                                            break 'label22;
                                        }
                                        self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(648), var1.wrapping_add(8856));
                                        'label23: loop {
                                            let var90 = self.memory.load8(var1 as usize + 1856) as i32;
                                            if (var90 == 2) as i32 != 0 {
                                                break 'label23;
                                            }
                                            let var91 = self.memory.load8(var1 as usize + 1856) as i64;
                                            if (var91 != 0 /* Void */) as i32 != 0 {
                                                break 'label2;
                                            }
                                            break;
                                        }
                                        let var92 = self.func40(env, var1.wrapping_add(8863));
                                        var2 = var92;
                                        self.memory.store8(var1 as usize + 1856, 4 as u8);
                                        self.func50(env, var1.wrapping_add(1088), var2, var1.wrapping_add(1856));
                                        let var94 = self.memory.load8(var1 as usize + 1088) as i32;
                                        if (var94 != 5) as i32 != 0 {
                                            break 'label20;
                                        }
                                        let var95 = self.memory.load8(var1 as usize + 1089) as i32;
                                        var8 = var95;
                                        break 'label19;
                                        break;
                                    }
                                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(648), var1.wrapping_add(8832).wrapping_add(var2));
                                    'label24: loop {
                                        let var97 = self.memory.load8(var1 as usize + 1856) as i32;
                                        if (var97 == 2) as i32 != 0 {
                                            break 'label24;
                                        }
                                        let var98 = self.memory.load8(var1 as usize + 1856) as i64;
                                        if (var98 != 0 /* Void */) as i32 != 0 {
                                            break 'label2;
                                        }
                                        break;
                                    }
                                    var2 = var2.wrapping_add(8);
                                    continue 'label21;
                                    break;
                                }
                                break;
                            }
                            var11 = slot_var1_1088_i64;
                            if (var11 & 255 != 0) as i32 != 0 {
                                break 'label2;
                            }
                            var8 = (var11 as u64).wrapping_shr(0 as u32) as i64 as i32;
                            break;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var99 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var99 as u64);
                        var12 = var1.wrapping_add(1856).wrapping_add(32);
                        let var100 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var12_0_i64 = var100 as i64;
                        var13 = var1.wrapping_add(1856).wrapping_add(24);
                        let var101 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var13_0_i64 = var101 as i64;
                        slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        slot_var1_1912_i32 = 0 as i32;
                        slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        slot_var1_1860_i32 = var1.wrapping_add(1088).wrapping_add(32) as i32;
                        slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        var6 = 129;
                        'label25: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label25;
                            }
                            slot_var2_0_i64 = slot_var12_0_i64 as i64;
                            slot_var4_0_i64 = slot_var13_0_i64 as i64;
                            let var104 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var5_0_i64 = var104 as i64;
                            slot_var1_1088_i64 = slot_var1_1864_i64 as i64;
                            var6 = var8;
                            break;
                        }
                        self.memory.store8(var1 as usize + 1120, var6 as u8);
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func51(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let var106 = self.memory.load8(var1 as usize + 9248) as i32;
                        var2 = var106;
                        if (var2 == 129) as i32 != 0 {
                            break 'label2;
                        }
                        var11 = slot_var1_9216_i64;
                        var4 = var1.wrapping_add(656).wrapping_add(16);
                        let var107 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                        slot_var4_0_i64 = var107 as i64;
                        var5 = var1.wrapping_add(656).wrapping_add(8);
                        let var108 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                        slot_var5_0_i64 = var108 as i64;
                        var6 = var1.wrapping_add(680).wrapping_add(8);
                        let var109 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(8) as usize) as i64;
                        slot_var6_0_i64 = var109 as i64;
                        var8 = var1.wrapping_add(680).wrapping_add(16);
                        let var110 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(16) as usize) as i64;
                        slot_var8_0_i64 = var110 as i64;
                        let mut slot_var1_656_i64 = slot_var1_9224_i32 as i64;
                        let mut slot_var1_680_i64 = slot_var1_8064_i64 as i64;
                        'label26: loop {
                            'label27: loop {
                                if (var2 == 64) as i32 != 0 {
                                    break 'label27;
                                }
                                let var111 = self.memory.load64(var1.wrapping_add(2624).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8844) as usize, var111 as u64);
                                let var112 = self.memory.load64(var1.wrapping_add(2624).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8852) as usize, var112 as u64);
                                let var113 = self.memory.load32(var1.wrapping_add(2624).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(8860) as usize, var113 as u32);
                                let var114 = self.memory.load64(var1.wrapping_add(264).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8876) as usize, var114 as u64);
                                let var115 = self.memory.load64(var1.wrapping_add(264).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8884) as usize, var115 as u64);
                                let var116 = self.memory.load32(var1.wrapping_add(264).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(8892) as usize, var116 as u32);
                                let mut slot_var1_8832_i32 = var10 as i32;
                                let mut slot_var1_8836_i64 = slot_var1_2624_i64 as i64;
                                let mut slot_var1_8864_i32 = var7 as i32;
                                let mut slot_var1_8868_i64 = slot_var1_264_i64 as i64;
                                self.memory.store64(var1.wrapping_add(8912) as usize, slot_var6_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8920) as usize, slot_var8_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8944) as usize, slot_var5_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8952) as usize, slot_var4_0_i64 as u64);
                                let mut slot_var1_8896_i64 = var3 as i64;
                                let mut slot_var1_8928_i64 = var11 as i64;
                                self.memory.store8(var1 as usize + 8960, 0 as u8);
                                let mut slot_var1_8904_i64 = slot_var1_680_i64 as i64;
                                let mut slot_var1_8936_i64 = slot_var1_656_i64 as i64;
                                self.func7(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                self.func5(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                let var119 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 64);
                                var119;
                                'label28: loop {
                                    let var120 = self.func27(env, 1050688);
                                    if var120 != 0 {
                                        break 'label28;
                                    }
                                    self.func10(env, var1.wrapping_add(1088), 1050688);
                                    break;
                                }
                                var7 = var1.wrapping_add(8832).wrapping_add(64);
                                let var122 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1088), 64);
                                var122;
                                let var123 = self.func122(env, var1.wrapping_add(1856), 0, 64);
                                var123;
                                'label29: loop {
                                    let var124 = self.func27(env, var1.wrapping_add(1856));
                                    if var124 != 0 {
                                        break 'label29;
                                    }
                                    let var125 = self.func122(env, var1.wrapping_add(1856), 0, 64);
                                    var125;
                                    self.func10(env, var1.wrapping_add(9216), var1.wrapping_add(1856));
                                    break;
                                }
                                self.func7(env, var1.wrapping_add(1856), var7);
                                let var128 = self.func16(env, var1.wrapping_add(1856), var1.wrapping_add(9216));
                                if (var128 == 0) as i32 != 0 {
                                    break 'label2;
                                }
                                self.func85(env, var1.wrapping_add(1856));
                                var8 = var1.wrapping_add(1120);
                                var13 = var1.wrapping_add(1088).wrapping_add(64);
                                var14 = var1.wrapping_add(8448).wrapping_add(64);
                                var12 = var1.wrapping_add(1856).wrapping_add(64);
                                var10 = var1.wrapping_add(1856).wrapping_add(128);
                                var4 = 128;
                                var6 = 0;
                                'label30: loop {
                                    var2 = var4;
                                    'label31: loop {
                                        'label32: loop {
                                            'label33: loop {
                                                'label34: loop {
                                                    if (var2 == 0) as i32 != 0 {
                                                        break 'label33;
                                                    }
                                                    var4 = var2.wrapping_add(-1);
                                                    var5 = (var4 as u32).wrapping_shr(6 as u32) as i32;
                                                    if (var2 as u32 >= 129 as u32) as i32 != 0 {
                                                        break 'label32;
                                                    }
                                                    var2 = var4;
                                                    let var130 = self.memory.load64(var5.wrapping_shl(3 as u32).wrapping_add(1051448) as usize) as i64;
                                                    var5 = (var130 as u64).wrapping_shr((var4 & 63) as u32 as i64 as u32) as i64 as i32;
                                                    if ((var6 | var5) & 1 == 0) as i32 != 0 {
                                                        continue 'label34;
                                                    }
                                                    break;
                                                }
                                                var6 = 1;
                                                let var131 = self.func86(env, var1.wrapping_add(1856));
                                                var2 = var131;
                                                if (var5 & 1 == 0) as i32 != 0 {
                                                    continue 'label30;
                                                }
                                                let var132 = self.func87(env, var2);
                                                if var132 != 0 {
                                                    break 'label31;
                                                }
                                                let var133 = self.func121(env, var1.wrapping_add(3840), var10, 64);
                                                var133;
                                                let var134 = self.func88(env, var1.wrapping_add(3840));
                                                var5 = var134;
                                                let var135 = self.func121(env, var1.wrapping_add(4224), var1.wrapping_add(8832), 64);
                                                var135;
                                                self.func89(env, var1.wrapping_add(4224), var5);
                                                let var137 = self.func121(env, var1.wrapping_add(4608), var10, 64);
                                                var137;
                                                self.func89(env, var1.wrapping_add(4608), var7);
                                                self.func89(env, var1.wrapping_add(4608), var5);
                                                'label35: loop {
                                                    let var140 = self.func90(env, var2, var1.wrapping_add(4224));
                                                    if var140 != 0 {
                                                        break 'label35;
                                                    }
                                                    let var141 = self.func121(env, var1.wrapping_add(4992), var1.wrapping_add(4224), 64);
                                                    var141;
                                                    self.func9(env, var1.wrapping_add(4992), var2);
                                                    let var143 = self.func121(env, var1.wrapping_add(5376), var1.wrapping_add(4992), 64);
                                                    var143;
                                                    let var144 = self.func88(env, var1.wrapping_add(5376));
                                                    let var145 = self.func121(env, var1.wrapping_add(5760), var144, 64);
                                                    var145;
                                                    let var146 = self.func11(env, var1.wrapping_add(5760));
                                                    let var147 = self.func11(env, var146);
                                                    var5 = var147;
                                                    let var148 = self.func121(env, var1.wrapping_add(6144), var1.wrapping_add(4992), 64);
                                                    var148;
                                                    let var149 = self.func15(env, var1.wrapping_add(6144));
                                                    var15 = var149;
                                                    self.func89(env, var15, var5);
                                                    let var151 = self.func121(env, var1.wrapping_add(6528), var1.wrapping_add(4608), 64);
                                                    var151;
                                                    self.func9(env, var1.wrapping_add(6528), var12);
                                                    let var153 = self.func11(env, var1.wrapping_add(6528));
                                                    var16 = var153;
                                                    let var154 = self.func121(env, var1.wrapping_add(6912), var2, 64);
                                                    var154;
                                                    self.func89(env, var1.wrapping_add(6912), var5);
                                                    self.func7(env, var2, var16);
                                                    self.func10(env, var2, var15);
                                                    self.func8(env, var1.wrapping_add(1088), var1.wrapping_add(6912));
                                                    self.func9(env, var2, var1.wrapping_add(1088));
                                                    self.func9(env, var1.wrapping_add(6912), var2);
                                                    let var161 = self.func11(env, var12);
                                                    var5 = var161;
                                                    let var162 = self.func121(env, var1.wrapping_add(8448), var16, 64);
                                                    var162;
                                                    let var163 = self.func121(env, var14, var5, 64);
                                                    var163;
                                                    let var164 = self.func121(env, var13, var15, 64);
                                                    var164;
                                                    let var165 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(6912), 64);
                                                    var165;
                                                    var2 = 0;
                                                    let var166 = self.func122(env, var1.wrapping_add(7296), 0, 64);
                                                    var166;
                                                    'label36: loop {
                                                        'label37: loop {
                                                            if (var2 == 128) as i32 != 0 {
                                                                break 'label36;
                                                            }
                                                            let var167 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(8448).wrapping_add(var2), 64);
                                                            var167;
                                                            self.func89(env, var1.wrapping_add(8064), var1.wrapping_add(1088).wrapping_add(var2));
                                                            let var169 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(8064), 64);
                                                            var169;
                                                            var2 = var2.wrapping_add(64);
                                                            self.func10(env, var1.wrapping_add(7296), var1.wrapping_add(7680));
                                                            continue 'label37;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    let var171 = self.func121(env, var5, var1.wrapping_add(7296), 64);
                                                    var171;
                                                    self.func89(env, var10, var1.wrapping_add(4992));
                                                    let var173 = self.func11(env, var10);
                                                    var173;
                                                    continue 'label30;
                                                    break;
                                                }
                                                'label38: loop {
                                                    let var174 = self.func90(env, var12, var1.wrapping_add(4608));
                                                    if var174 != 0 {
                                                        break 'label38;
                                                    }
                                                    self.func85(env, var2);
                                                    continue 'label30;
                                                    break;
                                                }
                                                let var176 = self.func86(env, var2);
                                                var176;
                                                continue 'label30;
                                                break;
                                            }
                                            let var177 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1856), 192);
                                            var177;
                                            let var178 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 136);
                                            var178;
                                            self.func70(env, var1.wrapping_add(1888), 1051352);
                                            self.func70(env, var1.wrapping_add(1952), 1051352);
                                            self.func89(env, var1.wrapping_add(1856), 1051464);
                                            self.func89(env, var1.wrapping_add(1856).wrapping_add(64), 1051528);
                                            let var183 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(1856), 128);
                                            var183;
                                            let var184 = self.memory.load8(var1 as usize + 1984) as i32;
                                            var2 = var184;
                                            self.func85(env, var1.wrapping_add(1856));
                                            'label39: loop {
                                                'label40: loop {
                                                    if var2 & 1 != 0 {
                                                        break 'label40;
                                                    }
                                                    let var186 = self.func121(env, var1.wrapping_add(1088).wrapping_add(64), var1.wrapping_add(8448).wrapping_add(64), 64);
                                                    var186;
                                                    self.memory.store64(var1.wrapping_add(8120) as usize, 0 /* False */ as u64);
                                                    self.memory.store64(var1.wrapping_add(8112) as usize, 0 /* False */ as u64);
                                                    self.memory.store64(var1.wrapping_add(8104) as usize, 0 /* False */ as u64);
                                                    let var187 = self.memory.load64(0 as usize + 1051232) as i64;
                                                    self.memory.store64(var1.wrapping_add(8072) as usize, var187 as u64);
                                                    let var188 = self.memory.load64(0 as usize + 1051240) as i64;
                                                    self.memory.store64(var1.wrapping_add(8080) as usize, var188 as u64);
                                                    let var189 = self.memory.load64(0 as usize + 1051248) as i64;
                                                    self.memory.store64(var1.wrapping_add(8088) as usize, var189 as u64);
                                                    let mut slot_var1_8096_i64 = 0 /* False */ as i64;
                                                    let var190 = self.memory.load64(0 as usize + 1051224) as i64;
                                                    slot_var1_8064_i64 = var190 as i64;
                                                    let var191 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8448), 64);
                                                    var191;
                                                    let var192 = self.func121(env, var1.wrapping_add(1088).wrapping_add(128), var1.wrapping_add(8064), 64);
                                                    var192;
                                                    break 'label39;
                                                    break;
                                                }
                                                let var193 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 192);
                                                var193;
                                                break;
                                            }
                                            let var194 = self.func87(env, var1.wrapping_add(9216));
                                            var2 = var194;
                                            let var195 = self.func87(env, var1.wrapping_add(1088));
                                            var4 = var195;
                                            'label41: loop {
                                                'label42: loop {
                                                    if var2 != 0 {
                                                        break 'label42;
                                                    }
                                                    if var4 != 0 {
                                                        break 'label42;
                                                    }
                                                    var2 = var1.wrapping_add(9216).wrapping_add(128);
                                                    self.func7(env, var1.wrapping_add(5760), var2);
                                                    var4 = var1.wrapping_add(1088).wrapping_add(128);
                                                    self.func7(env, var1.wrapping_add(6144), var4);
                                                    let var198 = self.func121(env, var1.wrapping_add(6528), var1.wrapping_add(9216), 64);
                                                    var198;
                                                    self.func89(env, var1.wrapping_add(6528), var1.wrapping_add(6144));
                                                    let var200 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1088), 64);
                                                    var200;
                                                    self.func89(env, var1.wrapping_add(6912), var1.wrapping_add(5760));
                                                    let var202 = self.func90(env, var1.wrapping_add(6528), var1.wrapping_add(6912));
                                                    if (var202 == 0) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    let var203 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(9216).wrapping_add(64), 64);
                                                    var203;
                                                    let var204 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(6144), 64);
                                                    var204;
                                                    self.func89(env, var1.wrapping_add(7680), var4);
                                                    self.func89(env, var1.wrapping_add(7296), var1.wrapping_add(7680));
                                                    let var207 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1088).wrapping_add(64), 64);
                                                    var207;
                                                    let var208 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(5760), 64);
                                                    var208;
                                                    self.func89(env, var1.wrapping_add(1856), var2);
                                                    self.func89(env, var1.wrapping_add(8064), var1.wrapping_add(1856));
                                                    let var211 = self.func90(env, var1.wrapping_add(7296), var1.wrapping_add(8064));
                                                    if (var211 == 0) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break 'label41;
                                                    break;
                                                }
                                                if (var2 & var4 == 0) as i32 != 0 {
                                                    break 'label2;
                                                }
                                                break;
                                            }
                                            var3 = slot_var1_8832_i64;
                                            let var212 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8840), 120);
                                            var212;
                                            let var213 = self.memory.load32(var1.wrapping_add(8964) as usize) as i32;
                                            let mut slot_var1_9219_i32 = var213 as i32;
                                            let mut slot_var1_8961_i32 = self.memory.load32(var1 as usize + 8961) as i32;
                                            slot_var1_9216_i32 = slot_var1_8961_i32 as i32;
                                            var2 = 0;
                                            break 'label26;
                                            break;
                                        }
                                        self.func91(env, var5, 2);
                                        unreachable!();
                                        break;
                                    }
                                    let var215 = self.func121(env, var2, var1.wrapping_add(8832), 64);
                                    var215;
                                    let var216 = self.func121(env, var12, var7, 64);
                                    var216;
                                    self.memory.store64(var8.wrapping_add(24) as usize, 0 /* False */ as u64);
                                    self.memory.store64(var8.wrapping_add(16) as usize, 0 /* False */ as u64);
                                    self.memory.store64(var8.wrapping_add(8) as usize, 0 /* False */ as u64);
                                    slot_var8_0_i64 = 0 /* False */ as i64;
                                    let var217 = self.memory.load64(0 as usize + 1051232) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(8) as usize, var217 as u64);
                                    let var218 = self.memory.load64(0 as usize + 1051240) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(16) as usize, var218 as u64);
                                    let var219 = self.memory.load64(0 as usize + 1051248) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(24) as usize, var219 as u64);
                                    let var220 = self.memory.load64(0 as usize + 1051224) as i64;
                                    slot_var1_1088_i64 = var220 as i64;
                                    let var221 = self.func121(env, var10, var1.wrapping_add(1088), 64);
                                    var221;
                                    continue 'label30;
                                    break;
                                }
                                break;
                            }
                            let var222 = self.func122(env, var1.wrapping_add(1856), 0, 120);
                            var222;
                            var2 = 1;
                            var3 = 0 /* False */;
                            break;
                        }
                        let var223 = self.func121(env, var1.wrapping_add(1184), var1.wrapping_add(1856), 120);
                        var223;
                        self.memory.store32(var1.wrapping_add(1308) as usize, slot_var1_9219_i32 as u32);
                        let mut slot_var1_1305_i32 = slot_var1_9216_i32 as i32;
                        slot_var1_1096_i64 = proof as i64;
                        slot_var1_1088_i64 = 4294967296 as i64;
                        let var224 = self.func121(env, var1.wrapping_add(1104), var1.wrapping_add(80), 56);
                        var224;
                        self.memory.store32(var1.wrapping_add(1164) as usize, slot_var1_75_i32 as u32);
                        self.memory.store8(var1 as usize + 1160, var9 as u8);
                        self.memory.store8(var1 as usize + 1304, var2 as u8);
                        let mut slot_var1_1176_i64 = var3 as i64;
                        let mut slot_var1_1168_i64 = 4294967296 as i64;
                        let mut slot_var1_1161_i32 = slot_var1_72_i32 as i32;
                        self.func30(env, var1.wrapping_add(8832), var1.wrapping_add(1088));
                        var17 = 8;
                        var18 = 0;
                        var12 = 0;
                        'label43: loop {
                            'label44: loop {
                                'label45: loop {
                                    'label46: loop {
                                        let var226 = self.memory.load8(var1 as usize + 8896) as i32;
                                        if (var226 == 2) as i32 != 0 {
                                            break 'label46;
                                        }
                                        let var227 = self.func92(env, 352, 8);
                                        var17 = var227;
                                        if (var17 == 0) as i32 != 0 {
                                            break 'label45;
                                        }
                                        let var228 = self.func121(env, var17, var1.wrapping_add(8832), 88);
                                        var2 = var228;
                                        let mut slot_var1_8456_i32 = 1 as i32;
                                        let mut slot_var1_8452_i32 = var2 as i32;
                                        let mut slot_var1_8448_i32 = 4 as i32;
                                        let var229 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(1088), 224);
                                        var229;
                                        var2 = 88;
                                        var12 = 1;
                                        'label47: loop {
                                            'label48: loop {
                                                self.func30(env, var1.wrapping_add(9216), var1.wrapping_add(1856));
                                                let var231 = self.memory.load8(var1 as usize + 9280) as i32;
                                                if (var231 == 2) as i32 != 0 {
                                                    break 'label47;
                                                }
                                                'label49: loop {
                                                    if (var12 != slot_var1_8448_i32) as i32 != 0 {
                                                        break 'label49;
                                                    }
                                                    self.func58(env, var1, var1.wrapping_add(8448), var12, 8, 88);
                                                    let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                                                    var4 = slot_var1_0_i32;
                                                    if (var4 != -2147483647) as i32 != 0 {
                                                        break 'label44;
                                                    }
                                                    var17 = slot_var1_8452_i32;
                                                    break;
                                                }
                                                let var233 = self.func121(env, var17.wrapping_add(var2), var1.wrapping_add(9216), 88);
                                                var233;
                                                var12 = var12.wrapping_add(1);
                                                slot_var1_8456_i32 = var12 as i32;
                                                var2 = var2.wrapping_add(88);
                                                continue 'label48;
                                                break;
                                            }
                                            break;
                                        }
                                        var18 = slot_var1_8448_i32;
                                        break;
                                    }
                                    let var234 = self.func121(env, var1.wrapping_add(1856), 1048960, 64);
                                    var234;
                                    let var235 = self.func122(env, var1.wrapping_add(1856).wrapping_add(64), 0, 128);
                                    var235;
                                    let var236 = self.func122(env, var1.wrapping_add(8832).wrapping_add(192), 0, 192);
                                    var236;
                                    let var237 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 192);
                                    var237;
                                    var14 = var1.wrapping_add(9216).wrapping_add(64);
                                    var16 = var1.wrapping_add(1856).wrapping_add(384);
                                    var19 = var1.wrapping_add(1088).wrapping_add(384);
                                    var15 = var1.wrapping_add(1856).wrapping_add(192);
                                    var8 = var17;
                                    var10 = var12;
                                    'label50: loop {
                                        'label51: loop {
                                            if (var10 == 0) as i32 != 0 {
                                                break 'label50;
                                            }
                                            let var238 = self.func121(env, var1.wrapping_add(9216), 1048960, 64);
                                            var238;
                                            let var239 = self.func122(env, var14, 0, 128);
                                            var239;
                                            let var240 = self.func122(env, var15, 0, 192);
                                            var240;
                                            let var241 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(9216), 192);
                                            var241;
                                            var2 = { let a = var10; let b = 4; if ((var10 as u32) < 4 as u32) as i32 != 0 { a } else { b } };
                                            var10 = var10.wrapping_sub(var2);
                                            var9 = var2.wrapping_mul(88);
                                            var13 = var8.wrapping_add(var9);
                                            var6 = 65;
                                            'label52: loop {
                                                'label53: loop {
                                                    if ((var6 as u32) < 2 as u32) as i32 != 0 {
                                                        break 'label52;
                                                    }
                                                    'label54: loop {
                                                        var7 = var6.wrapping_add(-1);
                                                        if (var7 == 64) as i32 != 0 {
                                                            break 'label54;
                                                        }
                                                        let var242 = self.func18(env, var1.wrapping_add(1856));
                                                        var242;
                                                        break;
                                                    }
                                                    var4 = var9;
                                                    var2 = var8;
                                                    'label55: loop {
                                                        'label56: loop {
                                                            if (var4 == 0) as i32 != 0 {
                                                                break 'label55;
                                                            }
                                                            let mut slot_var2_76_i32 = self.memory.load32(var2 as usize + 76) as i32;
                                                            var5 = slot_var2_76_i32;
                                                            let mut slot_var2_84_i32 = self.memory.load32(var2 as usize + 84) as i32;
                                                            if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                                break 'label43;
                                                            }
                                                            slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                            let var243 = self.func121(env, var1.wrapping_add(9216), var5, 192);
                                                            var243;
                                                            self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(9216), var2);
                                                            var4 = var4.wrapping_add(-88);
                                                            var2 = var2.wrapping_add(88);
                                                            continue 'label56;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    'label57: loop {
                                                        let var245 = self.memory.load8(var6.wrapping_add(1050558) as usize) as i32;
                                                        var2 = var245;
                                                        if (var2 == 255) as i32 != 0 {
                                                            break 'label57;
                                                        }
                                                        var6 = var7;
                                                        if (var2 != 1) as i32 != 0 {
                                                            continue 'label53;
                                                        }
                                                        break;
                                                    }
                                                    var4 = var9;
                                                    var2 = var8;
                                                    'label58: loop {
                                                        'label59: loop {
                                                            if var4 != 0 {
                                                                break 'label59;
                                                            }
                                                            var6 = var7;
                                                            continue 'label53;
                                                            break;
                                                        }
                                                        var5 = slot_var2_76_i32;
                                                        if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                            break 'label43;
                                                        }
                                                        slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                        let var246 = self.func121(env, var1.wrapping_add(9216), var5, 192);
                                                        var246;
                                                        self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(9216), var2);
                                                        var4 = var4.wrapping_add(-88);
                                                        var2 = var2.wrapping_add(88);
                                                        continue 'label58;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                break;
                                            }
                                            let var248 = self.func121(env, var19, var1.wrapping_add(1856), 384);
                                            var2 = var248;
                                            let var249 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 384);
                                            var249;
                                            let var250 = self.func121(env, var16, var2, 384);
                                            var2 = var250;
                                            let var251 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(8832), 384);
                                            var251;
                                            self.func42(env, var1.wrapping_add(9216), var2);
                                            let var253 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(9216), 384);
                                            var253;
                                            var8 = var13;
                                            continue 'label51;
                                            break;
                                        }
                                        break;
                                    }
                                    let var254 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 384);
                                    var254;
                                    var4 = var12.wrapping_mul(88);
                                    var2 = var17;
                                    'label60: loop {
                                        'label61: loop {
                                            'label62: loop {
                                                if var4 != 0 {
                                                    break 'label62;
                                                }
                                                var4 = var12.wrapping_mul(88);
                                                var2 = var17;
                                                'label63: loop {
                                                    if (var4 == 0) as i32 != 0 {
                                                        break 'label60;
                                                    }
                                                    var5 = slot_var2_76_i32;
                                                    if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                        break 'label43;
                                                    }
                                                    slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                    let var255 = self.func121(env, var1.wrapping_add(1088), var5, 192);
                                                    var255;
                                                    self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(1088), var2);
                                                    var4 = var4.wrapping_add(-88);
                                                    var2 = var2.wrapping_add(88);
                                                    continue 'label63;
                                                    break;
                                                }
                                                break;
                                            }
                                            var5 = slot_var2_76_i32;
                                            if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                break 'label43;
                                            }
                                            slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                            let var257 = self.func121(env, var1.wrapping_add(1088), var5, 192);
                                            var257;
                                            self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(1088), var2);
                                            var4 = var4.wrapping_add(-88);
                                            var2 = var2.wrapping_add(88);
                                            continue 'label61;
                                            break;
                                        }
                                        break;
                                    }
                                    let var259 = self.func121(env, var1.wrapping_add(704), var1.wrapping_add(1856), 384);
                                    var259;
                                    var2 = var17.wrapping_add(80);
                                    'label64: loop {
                                        'label65: loop {
                                            if (var12 == 0) as i32 != 0 {
                                                break 'label64;
                                            }
                                            let var260 = self.memory.load32(var2.wrapping_add(-8) as usize) as i32;
                                            self.func39(env, slot_var2_0_i64, var260);
                                            var12 = var12.wrapping_add(-1);
                                            var2 = var2.wrapping_add(88);
                                            continue 'label65;
                                            break;
                                        }
                                        break;
                                    }
                                    self.func47(env, var18, var17, 8, 88);
                                    let var263 = self.func121(env, var1.wrapping_add(2624), var1.wrapping_add(704), 384);
                                    var263;
                                    let var264 = self.func65(env, var1.wrapping_add(2624));
                                    var264;
                                    let var265 = self.func28(env, var1.wrapping_add(704));
                                    if var265 != 0 {
                                        break 'label43;
                                    }
                                    var18 = var1.wrapping_add(704).wrapping_add(192);
                                    self.func6(env, var1.wrapping_add(8832), var18);
                                    self.func6(env, var1.wrapping_add(1856), var1.wrapping_add(704));
                                    var2 = var1.wrapping_add(8832).wrapping_add(64);
                                    let var268 = self.func121(env, var1.wrapping_add(8448), var2, 64);
                                    var268;
                                    let var269 = self.func121(env, var2, var1.wrapping_add(8832), 64);
                                    var10 = var269;
                                    var6 = var1.wrapping_add(8960);
                                    let var270 = self.func121(env, var1.wrapping_add(8832), var6, 64);
                                    var270;
                                    let var271 = self.func121(env, var1.wrapping_add(9216), var6, 64);
                                    var271;
                                    let var272 = self.func11(env, var1.wrapping_add(9216));
                                    let var273 = self.func11(env, var272);
                                    let var274 = self.func11(env, var273);
                                    var5 = var274;
                                    var13 = var1.wrapping_add(7680).wrapping_add(24);
                                    let var275 = self.memory.load64(var1.wrapping_add(9016) as usize) as i64;
                                    slot_var13_0_i64 = var275 as i64;
                                    var14 = var1.wrapping_add(7680).wrapping_add(16);
                                    let var276 = self.memory.load64(var1.wrapping_add(9008) as usize) as i64;
                                    let mut slot_var14_0_i64 = var276 as i64;
                                    var8 = var1.wrapping_add(7680).wrapping_add(8);
                                    let var277 = self.memory.load64(var1.wrapping_add(9000) as usize) as i64;
                                    slot_var8_0_i64 = var277 as i64;
                                    let mut slot_var1_8992_i64 = self.memory.load64(var1 as usize + 8992) as i64;
                                    slot_var1_7680_i64 = slot_var1_8992_i64 as i64;
                                    self.func12(env, var1.wrapping_add(7680));
                                    self.func13(env, var1.wrapping_add(7680), var5);
                                    self.func13(env, var1.wrapping_add(7680), var1.wrapping_add(8832));
                                    var2 = var1.wrapping_add(1088).wrapping_add(24);
                                    let var281 = self.memory.load64(var5.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var281 as i64;
                                    var4 = var1.wrapping_add(1088).wrapping_add(16);
                                    let var282 = self.memory.load64(var5.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var282 as i64;
                                    var7 = var1.wrapping_add(1088).wrapping_add(8);
                                    let var283 = self.memory.load64(var5.wrapping_add(40) as usize) as i64;
                                    slot_var7_0_i64 = var283 as i64;
                                    let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
                                    slot_var1_1088_i64 = slot_var5_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(8992));
                                    var5 = var1.wrapping_add(8064).wrapping_add(24);
                                    slot_var5_0_i64 = slot_var2_0_i64 as i64;
                                    var9 = var1.wrapping_add(8064).wrapping_add(16);
                                    slot_var9_0_i64 = slot_var4_0_i64 as i64;
                                    var12 = var1.wrapping_add(8064).wrapping_add(8);
                                    slot_var12_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var1_8064_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(8064), var6);
                                    var15 = var1.wrapping_add(1088).wrapping_add(56);
                                    let mut slot_var15_0_i64 = slot_var5_0_i64 as i64;
                                    var16 = var1.wrapping_add(1088).wrapping_add(48);
                                    let mut slot_var16_0_i64 = slot_var9_0_i64 as i64;
                                    var19 = var1.wrapping_add(1088).wrapping_add(40);
                                    let mut slot_var19_0_i64 = slot_var12_0_i64 as i64;
                                    slot_var7_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    let mut slot_var1_1120_i64 = slot_var1_8064_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_7680_i64 as i64;
                                    let var286 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1088), 64);
                                    var286;
                                    let var287 = self.func121(env, var6, var1.wrapping_add(8448), 64);
                                    var12 = var287;
                                    self.func20(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                    let var289 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 192);
                                    var289;
                                    let var290 = self.func26(env, var1.wrapping_add(8832));
                                    if var290 != 0 {
                                        break 'label43;
                                    }
                                    self.func7(env, var1.wrapping_add(3008), var1.wrapping_add(8832));
                                    self.func7(env, var1.wrapping_add(3072), var10);
                                    self.func7(env, var1.wrapping_add(3136), var12);
                                    let var294 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var294;
                                    self.func5(env, var1.wrapping_add(1088), var10);
                                    let var296 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1088), 64);
                                    var296;
                                    let var297 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var297;
                                    self.func5(env, var1.wrapping_add(1088), var12);
                                    let var299 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(1088), 64);
                                    var299;
                                    let var300 = self.func121(env, var1.wrapping_add(3200), var10, 64);
                                    var300;
                                    self.func5(env, var1.wrapping_add(3200), var12);
                                    let var302 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3200), 64);
                                    var302;
                                    let var303 = self.func11(env, var1.wrapping_add(9216));
                                    let var304 = self.func11(env, var303);
                                    let var305 = self.func11(env, var304);
                                    var6 = var305;
                                    var7 = var1.wrapping_add(6528).wrapping_add(24);
                                    let var306 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(56) as usize) as i64;
                                    slot_var7_0_i64 = var306 as i64;
                                    var8 = var1.wrapping_add(6528).wrapping_add(16);
                                    let var307 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(48) as usize) as i64;
                                    slot_var8_0_i64 = var307 as i64;
                                    var9 = var1.wrapping_add(6528).wrapping_add(8);
                                    let var308 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(40) as usize) as i64;
                                    slot_var9_0_i64 = var308 as i64;
                                    let mut slot_var1_3232_i64 = self.memory.load64(var1 as usize + 3232) as i64;
                                    let mut slot_var1_6528_i64 = slot_var1_3232_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6528));
                                    self.func13(env, var1.wrapping_add(6528), var6);
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(3200));
                                    let var312 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var312 as i64;
                                    let var313 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var313 as i64;
                                    var5 = var1.wrapping_add(1088).wrapping_add(8);
                                    let var314 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var314 as i64;
                                    let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(3200).wrapping_add(32));
                                    slot_var13_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var14_0_i64 = slot_var4_0_i64 as i64;
                                    var6 = var1.wrapping_add(7680).wrapping_add(8);
                                    slot_var6_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_7680_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(7680), var1.wrapping_add(3200));
                                    slot_var15_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var6_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_7680_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6528_i64 as i64;
                                    let var317 = self.func121(env, var1.wrapping_add(3200), var1.wrapping_add(1088), 64);
                                    var317;
                                    let var318 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(3200), 64);
                                    var318;
                                    self.func9(env, var1.wrapping_add(3008), var1.wrapping_add(7680));
                                    let var320 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3136), 64);
                                    var320;
                                    let var321 = self.func11(env, var1.wrapping_add(9216));
                                    let var322 = self.func11(env, var321);
                                    let var323 = self.func11(env, var322);
                                    var6 = var323;
                                    var13 = var1.wrapping_add(6144).wrapping_add(24);
                                    let var324 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(56) as usize) as i64;
                                    slot_var13_0_i64 = var324 as i64;
                                    var14 = var1.wrapping_add(6144).wrapping_add(16);
                                    let var325 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(48) as usize) as i64;
                                    slot_var14_0_i64 = var325 as i64;
                                    var17 = var1.wrapping_add(6144).wrapping_add(8);
                                    let var326 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(40) as usize) as i64;
                                    let mut slot_var17_0_i64 = var326 as i64;
                                    let mut slot_var1_3168_i64 = self.memory.load64(var1 as usize + 3168) as i64;
                                    let mut slot_var1_6144_i64 = slot_var1_3168_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.func13(env, var1.wrapping_add(6144), var6);
                                    self.func13(env, var1.wrapping_add(6144), var1.wrapping_add(3136));
                                    let var330 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var330 as i64;
                                    let var331 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var331 as i64;
                                    let var332 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var332 as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(3136).wrapping_add(32));
                                    slot_var7_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var8_0_i64 = slot_var4_0_i64 as i64;
                                    slot_var9_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(3136));
                                    slot_var15_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var17_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_6528_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6144_i64 as i64;
                                    let var335 = self.func121(env, var1.wrapping_add(3136), var1.wrapping_add(1088), 64);
                                    var335;
                                    self.func9(env, var1.wrapping_add(3136), var1.wrapping_add(6912));
                                    self.func9(env, var1.wrapping_add(3072), var1.wrapping_add(7296));
                                    let var338 = self.func121(env, var1.wrapping_add(1088), var12, 64);
                                    var338;
                                    self.func5(env, var1.wrapping_add(1088), var1.wrapping_add(3136));
                                    let var340 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1088), 64);
                                    var340;
                                    let var341 = self.func121(env, var1.wrapping_add(3264), var10, 64);
                                    var341;
                                    self.func5(env, var1.wrapping_add(3264), var1.wrapping_add(3072));
                                    self.func10(env, var1.wrapping_add(8064), var1.wrapping_add(3264));
                                    let var344 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(8064), 64);
                                    var344;
                                    let var345 = self.func11(env, var1.wrapping_add(9216));
                                    let var346 = self.func11(env, var345);
                                    let var347 = self.func11(env, var346);
                                    var6 = var347;
                                    let var348 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(56) as usize) as i64;
                                    slot_var13_0_i64 = var348 as i64;
                                    let var349 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(48) as usize) as i64;
                                    slot_var14_0_i64 = var349 as i64;
                                    let var350 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(40) as usize) as i64;
                                    slot_var17_0_i64 = var350 as i64;
                                    slot_var1_6144_i64 = slot_var1_8096_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.func13(env, var1.wrapping_add(6144), var6);
                                    self.func13(env, var1.wrapping_add(6144), var1.wrapping_add(8064));
                                    let var354 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var354 as i64;
                                    let var355 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var355 as i64;
                                    let var356 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var356 as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(8064).wrapping_add(32));
                                    slot_var7_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var8_0_i64 = slot_var4_0_i64 as i64;
                                    slot_var9_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(8064));
                                    slot_var15_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var17_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_6528_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6144_i64 as i64;
                                    let var359 = self.func121(env, var1.wrapping_add(3328), var1.wrapping_add(1088), 64);
                                    var359;
                                    let var360 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var360;
                                    self.func5(env, var1.wrapping_add(1088), var1.wrapping_add(3008));
                                    let var362 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1088), 64);
                                    var362;
                                    self.func10(env, var1.wrapping_add(9216), var1.wrapping_add(3328));
                                    let var364 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(9216), 64);
                                    var364;
                                    let var365 = self.func27(env, var1.wrapping_add(1088));
                                    if var365 != 0 {
                                        break 'label43;
                                    }
                                    var2 = var1.wrapping_add(1088).wrapping_add(32);
                                    self.func76(env, var1.wrapping_add(5760), var2);
                                    self.func76(env, var1.wrapping_add(9216), var1.wrapping_add(1088));
                                    self.func12(env, var1.wrapping_add(5760));
                                    self.func43(env, var1.wrapping_add(9216), var1.wrapping_add(5760));
                                    let var370 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(24) as usize, var370 as u64);
                                    let var371 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(16) as usize, var371 as u64);
                                    let var372 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(8) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(8) as usize, var372 as u64);
                                    let mut slot_var1_5760_i64 = slot_var1_9216_i64 as i64;
                                    self.func33(env, var1.wrapping_add(9216), var1.wrapping_add(5760));
                                    if (slot_var1_9216_i32 == 0) as i32 != 0 {
                                        break 'label43;
                                    }
                                    var4 = var1.wrapping_add(6528).wrapping_add(24);
                                    let var374 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(24) as usize) as i64;
                                    slot_var4_0_i64 = var374 as i64;
                                    var5 = var1.wrapping_add(6528).wrapping_add(16);
                                    let var375 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(16) as usize) as i64;
                                    slot_var5_0_i64 = var375 as i64;
                                    var6 = var1.wrapping_add(6528).wrapping_add(8);
                                    let var376 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(8) as usize) as i64;
                                    slot_var6_0_i64 = var376 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    var7 = var1.wrapping_add(9216).wrapping_add(8);
                                    self.func70(env, var1.wrapping_add(6528), var7);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(24) as usize, slot_var4_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(16) as usize, slot_var5_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(8) as usize, slot_var6_0_i64 as u64);
                                    let mut slot_var1_3392_i64 = slot_var1_6528_i64 as i64;
                                    let var378 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                                    slot_var4_0_i64 = var378 as i64;
                                    let var379 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                                    slot_var5_0_i64 = var379 as i64;
                                    let var380 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                                    slot_var6_0_i64 = var380 as i64;
                                    slot_var1_6528_i64 = slot_var2_0_i64 as i64;
                                    self.func70(env, var1.wrapping_add(6528), var7);
                                    var2 = var1.wrapping_add(6144).wrapping_add(24);
                                    slot_var2_0_i64 = slot_var4_0_i64 as i64;
                                    var4 = var1.wrapping_add(6144).wrapping_add(16);
                                    slot_var4_0_i64 = slot_var5_0_i64 as i64;
                                    var5 = var1.wrapping_add(6144).wrapping_add(8);
                                    slot_var5_0_i64 = slot_var6_0_i64 as i64;
                                    slot_var1_6144_i64 = slot_var1_6528_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.memory.store64(var1.wrapping_add(3448) as usize, slot_var2_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3440) as usize, slot_var4_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3432) as usize, slot_var5_0_i64 as u64);
                                    let mut slot_var1_3424_i64 = slot_var1_6144_i64 as i64;
                                    let var383 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(3392), 64);
                                    var383;
                                    let var384 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3392), 64);
                                    var384;
                                    self.func5(env, var1.wrapping_add(9216), var1.wrapping_add(3008));
                                    let var386 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(9216), 64);
                                    var386;
                                    self.func5(env, var1.wrapping_add(3392), var1.wrapping_add(3136));
                                    let var388 = self.func121(env, var1.wrapping_add(1088).wrapping_add(64), var1.wrapping_add(3392), 64);
                                    var388;
                                    self.func5(env, var1.wrapping_add(8448), var1.wrapping_add(3072));
                                    let var390 = self.func121(env, var1.wrapping_add(1216), var1.wrapping_add(8448), 64);
                                    var390;
                                    let var391 = self.func121(env, var1.wrapping_add(1856).wrapping_add(8), var1.wrapping_add(1088), 192);
                                    var2 = var391;
                                    slot_var1_1856_i64 = 1 /* True */ as i64;
                                    let var392 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(704), 192);
                                    var392;
                                    self.func21(env, var1.wrapping_add(9216), var2);
                                    let var394 = self.func121(env, var1.wrapping_add(1088), var18, 192);
                                    var394;
                                    self.func21(env, var1.wrapping_add(1088), var2);
                                    self.func14(env, var1.wrapping_add(3456).wrapping_add(192), var1.wrapping_add(1088));
                                    let var397 = self.func121(env, var1.wrapping_add(3456), var1.wrapping_add(9216), 192);
                                    var397;
                                    let var398 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(2624), 384);
                                    var398;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(3456));
                                    let var400 = self.func121(env, var1.wrapping_add(3840), var1.wrapping_add(1856), 384);
                                    var400;
                                    let var401 = self.func121(env, var1.wrapping_add(3456), var1.wrapping_add(3840), 384);
                                    var401;
                                    self.func24(env, var1.wrapping_add(3840), 2);
                                    self.func42(env, var1.wrapping_add(3840), var1.wrapping_add(3456));
                                    let var404 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(3840), 384);
                                    var404;
                                    self.func59(env, var1.wrapping_add(4224), var1.wrapping_add(1856));
                                    self.func72(env, var1.wrapping_add(4608), var1.wrapping_add(4224));
                                    self.func72(env, var1.wrapping_add(4992), var1.wrapping_add(4608));
                                    self.func42(env, var1.wrapping_add(4992), var1.wrapping_add(4608));
                                    let var409 = self.func121(env, var1.wrapping_add(5376), var1.wrapping_add(4992), 384);
                                    var409;
                                    let var410 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(4992), 384);
                                    var410;
                                    self.func59(env, var1.wrapping_add(5760), var1.wrapping_add(1856));
                                    self.func72(env, var1.wrapping_add(6144), var1.wrapping_add(5760));
                                    self.func59(env, var1.wrapping_add(6528), var1.wrapping_add(6144));
                                    let var414 = self.func65(env, var1.wrapping_add(5376));
                                    var414;
                                    let var415 = self.func65(env, var1.wrapping_add(6528));
                                    var415;
                                    let var416 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6528), 384);
                                    var416;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(5760));
                                    let var418 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1856), 384);
                                    var418;
                                    self.func42(env, var1.wrapping_add(6912), var1.wrapping_add(5376));
                                    let var420 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(6912), 384);
                                    var420;
                                    let var421 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6912), 384);
                                    var421;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(4608));
                                    let var423 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(1856), 384);
                                    var423;
                                    let var424 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6912), 384);
                                    var424;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(5760));
                                    let var426 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1856), 384);
                                    var426;
                                    self.func42(env, var1.wrapping_add(8064), var1.wrapping_add(3840));
                                    let var428 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(7680), 384);
                                    var428;
                                    self.func24(env, var1.wrapping_add(8448), 1);
                                    let var430 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8448), 384);
                                    var430;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(8064));
                                    let var432 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 384);
                                    var432;
                                    self.func24(env, var1.wrapping_add(7296), 2);
                                    let var434 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(7296), 384);
                                    var434;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                    let var436 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1856), 384);
                                    var436;
                                    let var437 = self.func65(env, var1.wrapping_add(3840));
                                    var437;
                                    let var438 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(3840), 384);
                                    var438;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(7680));
                                    let var440 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 384);
                                    var440;
                                    self.func24(env, var1.wrapping_add(1088), 3);
                                    let var442 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(1088), 384);
                                    var442;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(9216));
                                    let var444 = self.func121(env, var1.wrapping_add(264), var1.wrapping_add(1856), 384);
                                    var444;
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(616));
                                    self.func81(env, var1.wrapping_add(1856), 1051160);
                                    var2 = 0;
                                    var4 = 0;
                                    'label66: loop {
                                        let var447 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var447 & 255 != 0 {
                                            break 'label66;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(584));
                                        self.func81(env, var1.wrapping_add(1856), 1051128);
                                        let var450 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        var4 = (var450 & 255 == 0) as i32;
                                        break;
                                    }
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(552));
                                    self.func81(env, var1.wrapping_add(1856), 1051096);
                                    'label67: loop {
                                        let var453 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var453 & 255 != 0 {
                                            break 'label67;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(520));
                                        self.func81(env, var1.wrapping_add(1856), 1051064);
                                        let var456 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        var2 = (var456 & 255 == 0) as i32;
                                        break;
                                    }
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(488));
                                    self.func81(env, var1.wrapping_add(1856), 1051032);
                                    proof = 0 /* False */;
                                    'label68: loop {
                                        let var459 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var459 & 255 != 0 {
                                            break 'label68;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(264).wrapping_add(192));
                                        self.func81(env, var1.wrapping_add(1856), 1051000);
                                        let var462 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if (var4 & var2 & (var462 & 255 == 0) as i32 != 1) as i32 != 0 {
                                            break 'label68;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(424));
                                        self.func81(env, var1.wrapping_add(1856), 1050968);
                                        var4 = 0;
                                        var2 = 0;
                                        'label69: loop {
                                            let var465 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var465 & 255 != 0 {
                                                break 'label69;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(392));
                                            self.func81(env, var1.wrapping_add(1856), 1050936);
                                            let var468 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var2 = (var468 & 255 == 0) as i32;
                                            break;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(360));
                                        self.func81(env, var1.wrapping_add(1856), 1050904);
                                        'label70: loop {
                                            let var471 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var471 & 255 != 0 {
                                                break 'label70;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(328));
                                            self.func81(env, var1.wrapping_add(1856), 1050872);
                                            let var474 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var4 = (var474 & 255 == 0) as i32;
                                            break;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(296));
                                        self.func81(env, var1.wrapping_add(1856), 1050840);
                                        var5 = 0;
                                        'label71: loop {
                                            let var477 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var477 & 255 != 0 {
                                                break 'label71;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(264));
                                            self.func81(env, var1.wrapping_add(1856), 1050808);
                                            let var480 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var5 = (var480 & 255 == 0) as i32;
                                            break;
                                        }
                                        if (var2 == 0) as i32 != 0 {
                                            break 'label68;
                                        }
                                        proof = (var4 & var5) as u32 as i64;
                                        break;
                                    }
                                    self.global0 = var1.wrapping_add(9600);
                                    return proof;
                                    break;
                                }
                                self.func56(env, 8, 352);
                                unreachable!();
                                break;
                            }
                            let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
                            self.func56(env, var4, slot_var1_4_i32);
                            unreachable!();
                            break;
                        }
                        self.func37(env);
                        unreachable!();
                        break;
                    }
                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8832).wrapping_add(var2));
                    'label72: loop {
                        let var485 = self.memory.load8(var1 as usize + 1856) as i32;
                        if (var485 == 2) as i32 != 0 {
                            break 'label72;
                        }
                        let var486 = self.memory.load8(var1 as usize + 1856) as i64;
                        if (var486 != 0 /* Void */) as i32 != 0 {
                            break 'label2;
                        }
                        break;
                    }
                    var2 = var2.wrapping_add(8);
                    continue 'label4;
                    break;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.func73(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
}

#[allow(dead_code)]
impl ArkBn254Contract {
    fn func3(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        self.func4(env, arg0, arg1);
        var2 = arg0.wrapping_add(64);
        self.func4(env, var2, arg1);
        arg0 = arg0.wrapping_add(128);
        self.func4(env, arg0, arg1);
        arg1 = arg1.wrapping_shl(6 as u32);
        self.func5(env, var2, arg1.wrapping_add(1049024));
        self.func5(env, arg0, arg1.wrapping_add(1049408));
    }
    fn func4(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func70(env, arg0.wrapping_add(32), (arg1 & 1).wrapping_shl(5 as u32).wrapping_add(1051320));
    }
    fn func5(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var2 = var9.wrapping_sub(320);
        self.global0 = var2;
        var3 = arg0.wrapping_add(24);
        let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
        self.memory.store64(var2.wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        var4 = arg0.wrapping_add(16);
        let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
        self.memory.store64(var2.wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        var5 = arg0.wrapping_add(8);
        let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
        self.memory.store64(var2.wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        var6 = arg0.wrapping_add(40);
        let mut slot_var6_0_i64 = self.memory.load64(var6 as usize) as i64;
        self.memory.store64(var2.wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        var7 = arg0.wrapping_add(48);
        let mut slot_var7_0_i64 = self.memory.load64(var7 as usize) as i64;
        self.memory.store64(var2.wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        var8 = arg0.wrapping_add(56);
        let mut slot_var8_0_i64 = self.memory.load64(var8 as usize) as i64;
        self.memory.store64(var2.wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var10 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var2_0_i64 = var10 as i64;
        let var11 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var2_32_i64 = var11 as i64;
        self.func12(env, arg0.wrapping_add(32));
        self.memory.store64(var2.wrapping_add(128).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var13 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var2_128_i64 = var13 as i64;
        let var14 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var2_160_i64 = var14 as i64;
        var3 = arg1.wrapping_add(56);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(56) as usize, slot_var3_0_i64 as u64);
        var4 = arg1.wrapping_add(48);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(48) as usize, slot_var4_0_i64 as u64);
        var5 = arg1.wrapping_add(40);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(40) as usize, slot_var5_0_i64 as u64);
        var6 = arg1.wrapping_add(8);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(8) as usize, slot_var6_0_i64 as u64);
        var7 = arg1.wrapping_add(16);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(16) as usize, slot_var7_0_i64 as u64);
        var8 = arg1.wrapping_add(24);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(24) as usize, slot_var8_0_i64 as u64);
        let var15 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_224_i64 = var15 as i64;
        let var16 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_192_i64 = var16 as i64;
        self.func41(env, var2.wrapping_add(64), var2.wrapping_add(128), var2.wrapping_add(192));
        self.memory.store64(var2.wrapping_add(256).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var18 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_256_i64 = var18 as i64;
        let var19 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_288_i64 = var19 as i64;
        self.func41(env, var2.wrapping_add(64).wrapping_add(32), var2, var2.wrapping_add(256));
        let var21 = self.func121(env, arg0, var2.wrapping_add(64), 64);
        var21;
        self.global0 = var2.wrapping_add(320);
    }
    fn func6(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
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
        let mut var15: i32 = 0;
        let var16 = self.global0;
        var2 = var16.wrapping_sub(960);
        self.global0 = var2;
        let var17 = self.func121(env, var2, arg1, 192);
        var2 = var17;
        var3 = arg1.wrapping_add(64);
        let var18 = self.func121(env, var2.wrapping_add(192), var3, 64);
        var18;
        self.func7(env, var2.wrapping_add(256), arg1);
        let var20 = self.func121(env, var2.wrapping_add(320), arg1, 64);
        var20;
        self.func5(env, var2.wrapping_add(320), var3);
        self.func8(env, var2.wrapping_add(384), var2.wrapping_add(320));
        let var23 = self.func121(env, var2.wrapping_add(896), arg1, 64);
        var23;
        self.func9(env, var2.wrapping_add(896), var2.wrapping_add(192));
        let var25 = self.func121(env, var2.wrapping_add(768), var2.wrapping_add(896), 64);
        var25;
        arg1 = arg1.wrapping_add(128);
        self.func10(env, var2.wrapping_add(768), arg1);
        let var27 = self.func121(env, var2.wrapping_add(896), var2.wrapping_add(768), 64);
        var27;
        self.func7(env, var2.wrapping_add(448), var2.wrapping_add(896));
        let var29 = self.func121(env, var2.wrapping_add(512), var3, 64);
        var29;
        self.func5(env, var2.wrapping_add(512), arg1);
        self.func8(env, var2.wrapping_add(576), var2.wrapping_add(512));
        self.func7(env, var2.wrapping_add(640), arg1);
        let var33 = self.func121(env, var2, var2.wrapping_add(576), 64);
        var2 = var33;
        let var34 = self.func121(env, var2.wrapping_add(768), var2.wrapping_add(576), 64);
        var34;
        let var35 = self.func11(env, var2.wrapping_add(768));
        let var36 = self.func11(env, var35);
        let var37 = self.func11(env, var36);
        arg1 = var37;
        var4 = var2.wrapping_add(832).wrapping_add(24);
        let var38 = self.memory.load64(var2.wrapping_add(576).wrapping_add(56) as usize) as i64;
        let mut slot_var4_0_i64 = var38 as i64;
        var5 = var2.wrapping_add(832).wrapping_add(16);
        let var39 = self.memory.load64(var2.wrapping_add(576).wrapping_add(48) as usize) as i64;
        let mut slot_var5_0_i64 = var39 as i64;
        var6 = var2.wrapping_add(832).wrapping_add(8);
        let var40 = self.memory.load64(var2.wrapping_add(576).wrapping_add(40) as usize) as i64;
        let mut slot_var6_0_i64 = var40 as i64;
        let mut slot_var2_608_i64 = self.memory.load64(var2 as usize + 608) as i64;
        let mut slot_var2_832_i64 = slot_var2_608_i64 as i64;
        self.func12(env, var2.wrapping_add(832));
        self.func13(env, var2.wrapping_add(832), arg1);
        self.func13(env, var2.wrapping_add(832), var2);
        var3 = var2.wrapping_add(896).wrapping_add(24);
        let var44 = self.memory.load64(arg1.wrapping_add(56) as usize) as i64;
        let mut slot_var3_0_i64 = var44 as i64;
        var7 = var2.wrapping_add(896).wrapping_add(16);
        let var45 = self.memory.load64(arg1.wrapping_add(48) as usize) as i64;
        let mut slot_var7_0_i64 = var45 as i64;
        var8 = var2.wrapping_add(896).wrapping_add(8);
        let var46 = self.memory.load64(arg1.wrapping_add(40) as usize) as i64;
        let mut slot_var8_0_i64 = var46 as i64;
        let var47 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_896_i64 = var47 as i64;
        self.func13(env, var2.wrapping_add(896), var2.wrapping_add(576).wrapping_add(32));
        var9 = var2.wrapping_add(864).wrapping_add(24);
        let mut slot_var9_0_i64 = slot_var3_0_i64 as i64;
        var10 = var2.wrapping_add(864).wrapping_add(16);
        let mut slot_var10_0_i64 = slot_var7_0_i64 as i64;
        var11 = var2.wrapping_add(864).wrapping_add(8);
        let mut slot_var11_0_i64 = slot_var8_0_i64 as i64;
        let mut slot_var2_864_i64 = slot_var2_896_i64 as i64;
        self.func13(env, var2.wrapping_add(864), var2.wrapping_add(576));
        var12 = var2.wrapping_add(896).wrapping_add(56);
        let mut slot_var12_0_i64 = slot_var9_0_i64 as i64;
        var13 = var2.wrapping_add(896).wrapping_add(48);
        let mut slot_var13_0_i64 = slot_var10_0_i64 as i64;
        var14 = var2.wrapping_add(896).wrapping_add(40);
        let mut slot_var14_0_i64 = slot_var11_0_i64 as i64;
        slot_var8_0_i64 = slot_var6_0_i64 as i64;
        slot_var7_0_i64 = slot_var5_0_i64 as i64;
        slot_var3_0_i64 = slot_var4_0_i64 as i64;
        let mut slot_var2_928_i64 = slot_var2_864_i64 as i64;
        slot_var2_896_i64 = slot_var2_832_i64 as i64;
        let var50 = self.func121(env, var2, var2.wrapping_add(896), 64);
        var2 = var50;
        self.func10(env, var2, var2.wrapping_add(256));
        let var52 = self.func121(env, var2.wrapping_add(64), var2.wrapping_add(640), 64);
        var15 = var52;
        let var53 = self.func121(env, var2.wrapping_add(768), var2.wrapping_add(640), 64);
        var53;
        let var54 = self.func11(env, var2.wrapping_add(768));
        let var55 = self.func11(env, var54);
        let var56 = self.func11(env, var55);
        arg1 = var56;
        let var57 = self.memory.load64(var2.wrapping_add(640).wrapping_add(56) as usize) as i64;
        slot_var4_0_i64 = var57 as i64;
        let var58 = self.memory.load64(var2.wrapping_add(640).wrapping_add(48) as usize) as i64;
        slot_var5_0_i64 = var58 as i64;
        let var59 = self.memory.load64(var2.wrapping_add(640).wrapping_add(40) as usize) as i64;
        slot_var6_0_i64 = var59 as i64;
        let mut slot_var2_672_i64 = self.memory.load64(var2 as usize + 672) as i64;
        slot_var2_832_i64 = slot_var2_672_i64 as i64;
        self.func12(env, var2.wrapping_add(832));
        self.func13(env, var2.wrapping_add(832), arg1);
        self.func13(env, var2.wrapping_add(832), var15);
        let var63 = self.memory.load64(arg1.wrapping_add(56) as usize) as i64;
        slot_var3_0_i64 = var63 as i64;
        let var64 = self.memory.load64(arg1.wrapping_add(48) as usize) as i64;
        slot_var7_0_i64 = var64 as i64;
        let var65 = self.memory.load64(arg1.wrapping_add(40) as usize) as i64;
        slot_var8_0_i64 = var65 as i64;
        let var66 = self.memory.load64(arg1 as usize + 32) as i64;
        slot_var2_896_i64 = var66 as i64;
        self.func13(env, var2.wrapping_add(896), var2.wrapping_add(640).wrapping_add(32));
        slot_var9_0_i64 = slot_var3_0_i64 as i64;
        slot_var10_0_i64 = slot_var7_0_i64 as i64;
        slot_var11_0_i64 = slot_var8_0_i64 as i64;
        slot_var2_864_i64 = slot_var2_896_i64 as i64;
        self.func13(env, var2.wrapping_add(864), var2.wrapping_add(640));
        slot_var12_0_i64 = slot_var9_0_i64 as i64;
        slot_var13_0_i64 = slot_var10_0_i64 as i64;
        slot_var14_0_i64 = slot_var11_0_i64 as i64;
        slot_var8_0_i64 = slot_var6_0_i64 as i64;
        slot_var7_0_i64 = slot_var5_0_i64 as i64;
        slot_var3_0_i64 = slot_var4_0_i64 as i64;
        slot_var2_928_i64 = slot_var2_864_i64 as i64;
        slot_var2_896_i64 = slot_var2_832_i64 as i64;
        let var69 = self.func121(env, var15, var2.wrapping_add(896), 64);
        self.func10(env, var69, var2.wrapping_add(384));
        let var71 = self.func121(env, var2.wrapping_add(896), var2.wrapping_add(384), 64);
        var71;
        self.func10(env, var2.wrapping_add(896), var2.wrapping_add(448));
        let var73 = self.func121(env, var2.wrapping_add(768), var2.wrapping_add(896), 64);
        var73;
        self.func10(env, var2.wrapping_add(768), var2.wrapping_add(576));
        let var75 = self.func121(env, var2.wrapping_add(704), var2.wrapping_add(768), 64);
        var75;
        self.func9(env, var2.wrapping_add(704), var2.wrapping_add(256));
        self.func9(env, var2.wrapping_add(704), var2.wrapping_add(640));
        let var78 = self.func121(env, var2.wrapping_add(128), var2.wrapping_add(704), 64);
        var78;
        let var79 = self.func121(env, arg0, var2, 192);
        var79;
        self.global0 = var2.wrapping_add(960);
    }
    fn func7(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(64);
        self.global0 = var2;
        let var4 = self.func121(env, var2, arg1, 64);
        arg1 = var4;
        let var5 = self.func88(env, arg1);
        let var6 = self.func121(env, arg0, var5, 64);
        var6;
        self.global0 = arg1.wrapping_add(64);
    }
    fn func8(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(64);
        self.global0 = var2;
        let var4 = self.func121(env, var2, arg1, 64);
        arg1 = var4;
        let var5 = self.func11(env, arg1);
        let var6 = self.func121(env, arg0, var5, 64);
        var6;
        self.global0 = arg1.wrapping_add(64);
    }
    fn func9(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func43(env, arg0, arg1);
        self.func43(env, arg0.wrapping_add(32), arg1.wrapping_add(32));
    }
    fn func10(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func13(env, arg0, arg1);
        self.func13(env, arg0.wrapping_add(32), arg1.wrapping_add(32));
    }
    fn func11(&mut self, env: &Env, mut arg0: i32) -> i32 {
        self.func32(env, arg0);
        self.func32(env, arg0.wrapping_add(32));
        arg0
    }
    fn func12(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i64 = 0;
        let mut var2: i64 = 0;
        'label0: loop {
            let var3 = self.func97(env, arg0, 1051416);
            if (var3 == 0) as i32 != 0 {
                break 'label0;
            }
            let var4 = self.memory.load64(arg0 as usize) as i64;
            var1 = var4;
            self.memory.store64(arg0 as usize, 4332616871279656263.wrapping_sub(var1) as u64);
            let var5 = self.memory.load64(arg0 as usize + 8) as i64;
            var2 = var5;
            var1 = var2.wrapping_add((var1 as u64 > 4332616871279656263 as u64) as i32 as u32 as i64);
            self.memory.store64(arg0 as usize + 8, (10917124144477883021).wrapping_sub(var1) as u64);
            var2 = ({ let a = (var1 as u64 > 10917124144477883021 as u64) as i32; let b = ((var1 as u64) < var2 as u64) as i32; if (var1 as u64 >= var2 as u64) as i32 != 0 { a } else { b } }) as u32 as i64;
            let var6 = self.memory.load64(arg0 as usize + 16) as i64;
            var1 = var2.wrapping_add(var6);
            self.memory.store64(arg0 as usize + 16, (13281191951274694749).wrapping_sub(var1) as u64);
            let var7 = self.memory.load64(arg0 as usize + 24) as i64;
            self.memory.store64(arg0 as usize + 24, ({ let a = 18446744073709551615; let b = 0 /* False */; if ({ let a = (var1 as u64 > 13281191951274694749 as u64) as i32; let b = ((var1 as u64) < var2 as u64) as i32; if (var1 as u64 >= var2 as u64) as i32 != 0 { a } else { b } }) != 0 { a } else { b } }).wrapping_sub(var7).wrapping_add(3486998266802970665) as u64);
            break;
        }
    }
    fn func13(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let var8 = self.memory.load64(arg0 as usize) as i64;
        var2 = var8;
        let var9 = self.memory.load64(arg1 as usize) as i64;
        var3 = var2.wrapping_add(var9);
        self.memory.store64(arg0 as usize, var3 as u64);
        var4 = ((var3 as u64) < var2 as u64) as i32 as u32 as i64;
        let var10 = self.memory.load64(arg1 as usize + 8) as i64;
        var2 = var4.wrapping_add(var10);
        let var11 = self.memory.load64(arg0 as usize + 8) as i64;
        var5 = var2.wrapping_add(var11);
        self.memory.store64(arg0 as usize + 8, var5 as u64);
        let var12 = self.memory.load64(arg0 as usize + 16) as i64;
        var6 = var12;
        let var13 = self.memory.load64(arg1 as usize + 16) as i64;
        var7 = var6.wrapping_add(var13);
        var2 = var7.wrapping_add((((var2 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var2 as u64) as i32 as u32 as i64));
        self.memory.store64(arg0 as usize + 16, var2 as u64);
        let var14 = self.memory.load64(arg0 as usize + 24) as i64;
        let var15 = self.memory.load64(arg1 as usize + 24) as i64;
        var7 = var14.wrapping_add(var15).wrapping_add((((var7 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var2 as u64) < var7 as u64) as i32 as u32 as i64));
        self.memory.store64(arg0 as usize + 24, var7 as u64);
        'label0: loop {
            let var16 = self.func99(env, arg0);
            if (var16 == 0) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize, var3.wrapping_add(14114127202429895353) as u64);
            var3 = var5.wrapping_add({ let a = 7529619929231668594; let b = 7529619929231668595; if ((var3 as u64) < 4332616871279656263 as u64) as i32 != 0 { a } else { b } });
            self.memory.store64(arg0 as usize + 8, var3 as u64);
            var3 = { let a = 5165552122434856866; let b = 5165552122434856867; if (var3 as u64 >= var5 as u64) as i32 != 0 { a } else { b } };
            var2 = var3.wrapping_add(var2);
            self.memory.store64(arg0 as usize + 16, var2 as u64);
            self.memory.store64(arg0 as usize + 24, ({ let a = 14959745806906580950; let b = 14959745806906580951; if (var2 as u64 >= var3 as u64) as i32 != 0 { a } else { b } }).wrapping_add(var7) as u64);
            break;
        }
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let var2 = self.func15(env, arg1);
        arg1 = var2;
        let var3 = self.func15(env, arg1.wrapping_add(64));
        var3;
        let var4 = self.func15(env, arg1.wrapping_add(128));
        var4;
        let var5 = self.func121(env, arg0, arg1, 192);
        var5;
    }
    fn func15(&mut self, env: &Env, mut arg0: i32) -> i32 {
        self.func12(env, arg0);
        self.func12(env, arg0.wrapping_add(32));
        arg0
    }
    fn func16(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = 0;
        'label0: loop {
            let var3 = self.func17(env, arg0, arg1);
            if var3 != 0 {
                break 'label0;
            }
            let var4 = self.func17(env, arg0.wrapping_add(32), arg1.wrapping_add(32));
            var2 = var4 ^ 1;
            break;
        }
        var2
    }
    fn func17(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func44(env, arg0, arg1);
        var2 ^ 1
    }
    fn func18(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
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
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let var18 = self.global0;
        var1 = var18.wrapping_sub(1024);
        self.global0 = var1;
        let var19 = self.func121(env, var1.wrapping_add(768), 1048768, 192);
        var19;
        self.func14(env, var1.wrapping_add(384), var1.wrapping_add(768));
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var21 = self.func19(env, 1048576, var1.wrapping_add(384));
                    if var21 != 0 {
                        break 'label2;
                    }
                    let var22 = self.func19(env, 1048640, var1.wrapping_add(448));
                    if var22 != 0 {
                        break 'label2;
                    }
                    let var23 = self.func19(env, 1048704, var1.wrapping_add(512));
                    if (var23 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                let var24 = self.func121(env, var1.wrapping_add(192), arg0, 192);
                var24;
                var2 = arg0.wrapping_add(192);
                self.func20(env, var1.wrapping_add(192), var2);
                let var26 = self.func121(env, var1.wrapping_add(384), var2, 192);
                var26;
                let var27 = self.func121(env, var1.wrapping_add(384).wrapping_add(64), var2, 64);
                var27;
                var3 = arg0.wrapping_add(320);
                let var28 = self.func121(env, var1.wrapping_add(384), var3, 64);
                var28;
                let var29 = self.func121(env, var1, var3, 64);
                var4 = var29;
                let var30 = self.func11(env, var4);
                let var31 = self.func11(env, var30);
                let var32 = self.func11(env, var31);
                var5 = var32;
                var6 = var4.wrapping_add(960).wrapping_add(24);
                let var33 = self.memory.load64(arg0.wrapping_add(376) as usize) as i64;
                let mut slot_var6_0_i64 = var33 as i64;
                var7 = var4.wrapping_add(960).wrapping_add(16);
                let var34 = self.memory.load64(arg0.wrapping_add(368) as usize) as i64;
                let mut slot_var7_0_i64 = var34 as i64;
                var8 = var4.wrapping_add(960).wrapping_add(8);
                let var35 = self.memory.load64(arg0.wrapping_add(360) as usize) as i64;
                let mut slot_var8_0_i64 = var35 as i64;
                let var36 = self.memory.load64(arg0 as usize + 352) as i64;
                let mut slot_var4_960_i64 = var36 as i64;
                self.func12(env, var4.wrapping_add(960));
                self.func13(env, var4.wrapping_add(960), var5);
                self.func13(env, var4.wrapping_add(960), var4.wrapping_add(384));
                var9 = var4.wrapping_add(768).wrapping_add(24);
                let var40 = self.memory.load64(var5.wrapping_add(56) as usize) as i64;
                let mut slot_var9_0_i64 = var40 as i64;
                var10 = var4.wrapping_add(768).wrapping_add(16);
                let var41 = self.memory.load64(var5.wrapping_add(48) as usize) as i64;
                let mut slot_var10_0_i64 = var41 as i64;
                var11 = var4.wrapping_add(768).wrapping_add(8);
                let var42 = self.memory.load64(var5.wrapping_add(40) as usize) as i64;
                let mut slot_var11_0_i64 = var42 as i64;
                let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
                let mut slot_var4_768_i64 = slot_var5_32_i64 as i64;
                self.func13(env, var4.wrapping_add(768), arg0.wrapping_add(352));
                var12 = var4.wrapping_add(992).wrapping_add(24);
                let mut slot_var12_0_i64 = slot_var9_0_i64 as i64;
                var13 = var4.wrapping_add(992).wrapping_add(16);
                let mut slot_var13_0_i64 = slot_var10_0_i64 as i64;
                var14 = var4.wrapping_add(992).wrapping_add(8);
                let mut slot_var14_0_i64 = slot_var11_0_i64 as i64;
                let mut slot_var4_992_i64 = slot_var4_768_i64 as i64;
                self.func13(env, var4.wrapping_add(992), var3);
                var15 = var4.wrapping_add(768).wrapping_add(56);
                let mut slot_var15_0_i64 = slot_var12_0_i64 as i64;
                var16 = var4.wrapping_add(768).wrapping_add(48);
                let mut slot_var16_0_i64 = slot_var13_0_i64 as i64;
                var17 = var4.wrapping_add(768).wrapping_add(40);
                let mut slot_var17_0_i64 = slot_var14_0_i64 as i64;
                slot_var11_0_i64 = slot_var8_0_i64 as i64;
                slot_var10_0_i64 = slot_var7_0_i64 as i64;
                slot_var9_0_i64 = slot_var6_0_i64 as i64;
                let mut slot_var4_800_i64 = slot_var4_992_i64 as i64;
                slot_var4_768_i64 = slot_var4_960_i64 as i64;
                let var45 = self.func121(env, var4.wrapping_add(384), var4.wrapping_add(768), 64);
                var45;
                let var46 = self.func121(env, var4.wrapping_add(384).wrapping_add(128), arg0.wrapping_add(256), 64);
                var46;
                let var47 = self.func121(env, var4.wrapping_add(768), arg0, 192);
                var47;
                self.func20(env, var4.wrapping_add(768), var4.wrapping_add(384));
                let var49 = self.func121(env, var4.wrapping_add(384), var4.wrapping_add(768), 192);
                var49;
                let var50 = self.func121(env, var4.wrapping_add(576), arg0, 192);
                var50;
                self.func21(env, var4.wrapping_add(576), var2);
                self.func21(env, var4.wrapping_add(192), var4.wrapping_add(384));
                let var53 = self.func121(env, var2, var4.wrapping_add(576), 192);
                let var54 = self.func22(env, var53);
                var54;
                let var55 = self.func121(env, arg0, var4.wrapping_add(576), 192);
                var5 = var55;
                let var56 = self.func121(env, var5.wrapping_add(64), var4.wrapping_add(576), 64);
                var56;
                var2 = var4.wrapping_add(576).wrapping_add(128);
                let var57 = self.func121(env, var5, var2, 64);
                var3 = var57;
                let var58 = self.func121(env, var4, var2, 64);
                var4 = var58;
                let var59 = self.func11(env, var4);
                let var60 = self.func11(env, var59);
                let var61 = self.func11(env, var60);
                var5 = var61;
                let var62 = self.memory.load64(var4.wrapping_add(760) as usize) as i64;
                slot_var6_0_i64 = var62 as i64;
                let var63 = self.memory.load64(var4.wrapping_add(752) as usize) as i64;
                slot_var7_0_i64 = var63 as i64;
                let var64 = self.memory.load64(var4.wrapping_add(744) as usize) as i64;
                slot_var8_0_i64 = var64 as i64;
                let mut slot_var4_736_i64 = self.memory.load64(var4 as usize + 736) as i64;
                slot_var4_960_i64 = slot_var4_736_i64 as i64;
                self.func12(env, var4.wrapping_add(960));
                self.func13(env, var4.wrapping_add(960), var5);
                self.func13(env, var4.wrapping_add(960), var3);
                let var68 = self.memory.load64(var5.wrapping_add(56) as usize) as i64;
                slot_var9_0_i64 = var68 as i64;
                let var69 = self.memory.load64(var5.wrapping_add(48) as usize) as i64;
                slot_var10_0_i64 = var69 as i64;
                let var70 = self.memory.load64(var5.wrapping_add(40) as usize) as i64;
                slot_var11_0_i64 = var70 as i64;
                slot_var4_768_i64 = slot_var5_32_i64 as i64;
                self.func13(env, var4.wrapping_add(768), var4.wrapping_add(736));
                slot_var12_0_i64 = slot_var9_0_i64 as i64;
                slot_var13_0_i64 = slot_var10_0_i64 as i64;
                slot_var14_0_i64 = slot_var11_0_i64 as i64;
                slot_var4_992_i64 = slot_var4_768_i64 as i64;
                self.func13(env, var4.wrapping_add(992), var2);
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var17_0_i64 = slot_var14_0_i64 as i64;
                slot_var11_0_i64 = slot_var8_0_i64 as i64;
                slot_var10_0_i64 = slot_var7_0_i64 as i64;
                slot_var9_0_i64 = slot_var6_0_i64 as i64;
                slot_var4_800_i64 = slot_var4_992_i64 as i64;
                slot_var4_768_i64 = slot_var4_960_i64 as i64;
                let var73 = self.func121(env, var3, var4.wrapping_add(768), 64);
                var5 = var73;
                let var74 = self.func121(env, var5.wrapping_add(128), var4.wrapping_add(576).wrapping_add(64), 64);
                var74;
                self.func23(env, var5, var4.wrapping_add(192));
                self.func23(env, var5, var4.wrapping_add(576));
                break 'label0;
                break;
            }
            let var77 = self.func121(env, var1, arg0, 192);
            var4 = var77;
            let var78 = self.func121(env, var4.wrapping_add(384), arg0, 192);
            var78;
            var5 = arg0.wrapping_add(192);
            self.func20(env, var4.wrapping_add(384), var5);
            let var80 = self.func121(env, var4.wrapping_add(768), var5, 192);
            var80;
            self.func23(env, arg0, var4.wrapping_add(768));
            self.func21(env, arg0, var4.wrapping_add(384));
            let var83 = self.func22(env, var5);
            self.func21(env, var83, var4);
            break;
        }
        self.global0 = var1.wrapping_add(1024);
        arg0
    }
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func16(env, arg0, arg1);
        var2 ^ 1
    }
    fn func20(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func9(env, arg0, arg1);
        self.func9(env, arg0.wrapping_add(64), arg1.wrapping_add(64));
        self.func9(env, arg0.wrapping_add(128), arg1.wrapping_add(128));
    }
    fn func21(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
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
        let mut var15: i32 = 0;
        let var16 = self.global0;
        var2 = var16.wrapping_sub(896);
        self.global0 = var2;
        let var17 = self.func121(env, var2, arg0, 64);
        var2 = var17;
        let var18 = self.func121(env, var2.wrapping_add(832), arg0, 64);
        var18;
        self.func5(env, var2.wrapping_add(832), arg1);
        let var20 = self.func121(env, var2.wrapping_add(64), var2.wrapping_add(832), 64);
        var20;
        var3 = arg0.wrapping_add(64);
        let var21 = self.func121(env, var2.wrapping_add(832), var3, 64);
        var21;
        var4 = arg1.wrapping_add(64);
        self.func5(env, var2.wrapping_add(832), var4);
        let var23 = self.func121(env, var2.wrapping_add(128), var2.wrapping_add(832), 64);
        var23;
        var5 = arg0.wrapping_add(128);
        let var24 = self.func121(env, var2.wrapping_add(832), var5, 64);
        var24;
        var6 = arg1.wrapping_add(128);
        self.func5(env, var2.wrapping_add(832), var6);
        let var26 = self.func121(env, var2.wrapping_add(192), var2.wrapping_add(832), 64);
        var26;
        let var27 = self.func121(env, var2.wrapping_add(832), var3, 64);
        var27;
        self.func10(env, var2.wrapping_add(832), var5);
        let var29 = self.func121(env, var2.wrapping_add(256), var2.wrapping_add(832), 64);
        var29;
        let var30 = self.func121(env, var2.wrapping_add(832), var4, 64);
        var30;
        self.func10(env, var2.wrapping_add(832), var6);
        let var32 = self.func121(env, var2.wrapping_add(736), var2.wrapping_add(832), 64);
        var32;
        self.func5(env, var2.wrapping_add(256), var2.wrapping_add(736));
        self.func9(env, var2.wrapping_add(256), var2.wrapping_add(128));
        self.func9(env, var2.wrapping_add(256), var2.wrapping_add(192));
        let var36 = self.func121(env, var2.wrapping_add(832), arg0, 64);
        var36;
        self.func10(env, var2.wrapping_add(832), var3);
        let var38 = self.func121(env, var2.wrapping_add(384), var2.wrapping_add(832), 64);
        var38;
        let var39 = self.func121(env, var2.wrapping_add(832), arg1, 64);
        var39;
        self.func10(env, var2.wrapping_add(832), var4);
        let var41 = self.func121(env, var2.wrapping_add(736), var2.wrapping_add(832), 64);
        var41;
        self.func5(env, var2.wrapping_add(384), var2.wrapping_add(736));
        self.func9(env, var2.wrapping_add(384), var2.wrapping_add(64));
        self.func9(env, var2.wrapping_add(384), var2.wrapping_add(128));
        let var45 = self.func121(env, var2.wrapping_add(320), var2.wrapping_add(384), 64);
        var45;
        self.func10(env, var2, var5);
        let var47 = self.func121(env, var2.wrapping_add(512), var2, 64);
        var47;
        let var48 = self.func121(env, var2.wrapping_add(832), arg1, 64);
        var48;
        self.func10(env, var2.wrapping_add(832), var6);
        let var50 = self.func121(env, var2.wrapping_add(736), var2.wrapping_add(832), 64);
        var50;
        self.func5(env, var2.wrapping_add(512), var2.wrapping_add(736));
        self.func9(env, var2.wrapping_add(512), var2.wrapping_add(64));
        let var53 = self.func121(env, var2.wrapping_add(832), var2.wrapping_add(512), 64);
        var53;
        self.func10(env, var2.wrapping_add(832), var2.wrapping_add(128));
        let var55 = self.func121(env, var2.wrapping_add(448), var2.wrapping_add(832), 64);
        var55;
        self.func9(env, var2.wrapping_add(448), var2.wrapping_add(192));
        let var57 = self.func121(env, var2.wrapping_add(576), var2.wrapping_add(64), 64);
        var57;
        let var58 = self.func121(env, var2.wrapping_add(736), var2.wrapping_add(256), 64);
        var58;
        let var59 = self.func11(env, var2.wrapping_add(736));
        let var60 = self.func11(env, var59);
        let var61 = self.func11(env, var60);
        arg1 = var61;
        var7 = var2.wrapping_add(704).wrapping_add(24);
        let var62 = self.memory.load64(var2.wrapping_add(256).wrapping_add(56) as usize) as i64;
        let mut slot_var7_0_i64 = var62 as i64;
        var8 = var2.wrapping_add(704).wrapping_add(16);
        let var63 = self.memory.load64(var2.wrapping_add(256).wrapping_add(48) as usize) as i64;
        let mut slot_var8_0_i64 = var63 as i64;
        var9 = var2.wrapping_add(704).wrapping_add(8);
        let var64 = self.memory.load64(var2.wrapping_add(256).wrapping_add(40) as usize) as i64;
        let mut slot_var9_0_i64 = var64 as i64;
        let mut slot_var2_288_i64 = self.memory.load64(var2 as usize + 288) as i64;
        let mut slot_var2_704_i64 = slot_var2_288_i64 as i64;
        self.func12(env, var2.wrapping_add(704));
        self.func13(env, var2.wrapping_add(704), arg1);
        self.func13(env, var2.wrapping_add(704), var2.wrapping_add(256));
        var4 = var2.wrapping_add(832).wrapping_add(24);
        let var68 = self.memory.load64(arg1.wrapping_add(56) as usize) as i64;
        let mut slot_var4_0_i64 = var68 as i64;
        var6 = var2.wrapping_add(832).wrapping_add(16);
        let var69 = self.memory.load64(arg1.wrapping_add(48) as usize) as i64;
        let mut slot_var6_0_i64 = var69 as i64;
        var10 = var2.wrapping_add(832).wrapping_add(8);
        let var70 = self.memory.load64(arg1.wrapping_add(40) as usize) as i64;
        let mut slot_var10_0_i64 = var70 as i64;
        let var71 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_832_i64 = var71 as i64;
        self.func13(env, var2.wrapping_add(832), var2.wrapping_add(256).wrapping_add(32));
        arg1 = var2.wrapping_add(800).wrapping_add(24);
        self.memory.store64(arg1 as usize, slot_var4_0_i64 as u64);
        var11 = var2.wrapping_add(800).wrapping_add(16);
        let mut slot_var11_0_i64 = slot_var6_0_i64 as i64;
        var12 = var2.wrapping_add(800).wrapping_add(8);
        let mut slot_var12_0_i64 = slot_var10_0_i64 as i64;
        let mut slot_var2_800_i64 = slot_var2_832_i64 as i64;
        self.func13(env, var2.wrapping_add(800), var2.wrapping_add(256));
        var13 = var2.wrapping_add(832).wrapping_add(56);
        let var74 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var13_0_i64 = var74 as i64;
        var14 = var2.wrapping_add(832).wrapping_add(48);
        let mut slot_var14_0_i64 = slot_var11_0_i64 as i64;
        var15 = var2.wrapping_add(832).wrapping_add(40);
        let mut slot_var15_0_i64 = slot_var12_0_i64 as i64;
        slot_var10_0_i64 = slot_var9_0_i64 as i64;
        slot_var6_0_i64 = slot_var8_0_i64 as i64;
        slot_var4_0_i64 = slot_var7_0_i64 as i64;
        let mut slot_var2_864_i64 = slot_var2_800_i64 as i64;
        slot_var2_832_i64 = slot_var2_704_i64 as i64;
        let var75 = self.func121(env, var2.wrapping_add(256), var2.wrapping_add(832), 64);
        var75;
        self.func10(env, var2.wrapping_add(576), var2.wrapping_add(256));
        let var77 = self.func121(env, var2.wrapping_add(832), var2.wrapping_add(576), 64);
        var77;
        let var78 = self.func121(env, arg0, var2.wrapping_add(832), 64);
        var78;
        let var79 = self.func121(env, var2.wrapping_add(736), var2.wrapping_add(192), 64);
        var79;
        let var80 = self.func11(env, var2.wrapping_add(736));
        let var81 = self.func11(env, var80);
        let var82 = self.func11(env, var81);
        arg0 = var82;
        let var83 = self.memory.load64(var2.wrapping_add(192).wrapping_add(56) as usize) as i64;
        self.memory.store64(arg1 as usize, var83 as u64);
        let var84 = self.memory.load64(var2.wrapping_add(192).wrapping_add(48) as usize) as i64;
        slot_var11_0_i64 = var84 as i64;
        let var85 = self.memory.load64(var2.wrapping_add(192).wrapping_add(40) as usize) as i64;
        slot_var12_0_i64 = var85 as i64;
        let mut slot_var2_224_i64 = self.memory.load64(var2 as usize + 224) as i64;
        slot_var2_800_i64 = slot_var2_224_i64 as i64;
        self.func12(env, var2.wrapping_add(800));
        self.func13(env, var2.wrapping_add(800), arg0);
        self.func13(env, var2.wrapping_add(800), var2.wrapping_add(192));
        let var89 = self.memory.load64(arg0.wrapping_add(56) as usize) as i64;
        slot_var4_0_i64 = var89 as i64;
        let var90 = self.memory.load64(arg0.wrapping_add(48) as usize) as i64;
        slot_var6_0_i64 = var90 as i64;
        let var91 = self.memory.load64(arg0.wrapping_add(40) as usize) as i64;
        slot_var10_0_i64 = var91 as i64;
        let var92 = self.memory.load64(arg0 as usize + 32) as i64;
        slot_var2_832_i64 = var92 as i64;
        self.func13(env, var2.wrapping_add(832), var2.wrapping_add(192).wrapping_add(32));
        arg0 = var2.wrapping_add(576).wrapping_add(24);
        self.memory.store64(arg0 as usize, slot_var4_0_i64 as u64);
        var7 = var2.wrapping_add(576).wrapping_add(16);
        slot_var7_0_i64 = slot_var6_0_i64 as i64;
        var8 = var2.wrapping_add(576).wrapping_add(8);
        slot_var8_0_i64 = slot_var10_0_i64 as i64;
        let mut slot_var2_576_i64 = slot_var2_832_i64 as i64;
        self.func13(env, var2.wrapping_add(576), var2.wrapping_add(192));
        let var95 = self.memory.load64(arg0 as usize) as i64;
        slot_var13_0_i64 = var95 as i64;
        slot_var14_0_i64 = slot_var7_0_i64 as i64;
        slot_var15_0_i64 = slot_var8_0_i64 as i64;
        slot_var10_0_i64 = slot_var12_0_i64 as i64;
        slot_var6_0_i64 = slot_var11_0_i64 as i64;
        let var96 = self.memory.load64(arg1 as usize) as i64;
        slot_var4_0_i64 = var96 as i64;
        slot_var2_864_i64 = slot_var2_576_i64 as i64;
        slot_var2_832_i64 = slot_var2_800_i64 as i64;
        let var97 = self.func121(env, var2.wrapping_add(640), var2.wrapping_add(832), 64);
        var97;
        self.func10(env, var2.wrapping_add(320), var2.wrapping_add(640));
        let var99 = self.func121(env, var3, var2.wrapping_add(320), 64);
        var99;
        let var100 = self.func121(env, var5, var2.wrapping_add(448), 64);
        var100;
        self.global0 = var2.wrapping_add(896);
    }
    fn func22(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func11(env, arg0);
        arg0 = var1;
        let var2 = self.func11(env, arg0.wrapping_add(64));
        var2;
        let var3 = self.func11(env, arg0.wrapping_add(128));
        var3;
        arg0
    }
    fn func23(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func10(env, arg0, arg1);
        self.func10(env, arg0.wrapping_add(64), arg1.wrapping_add(64));
        self.func10(env, arg0.wrapping_add(128), arg1.wrapping_add(128));
    }
    fn func24(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        self.func3(env, arg0, arg1);
        var2 = arg0.wrapping_add(192);
        self.func3(env, var2, arg1);
        arg1 = arg1.wrapping_shl(6 as u32).wrapping_add(1049792);
        self.func5(env, var2, arg1);
        self.func5(env, arg0.wrapping_add(256), arg1);
        self.func5(env, arg0.wrapping_add(320), arg1);
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func12(env, arg1);
        self.func12(env, arg1.wrapping_add(32));
        let var4 = self.func121(env, arg0, arg1, 64);
        var4;
    }
    fn func26(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        var1 = 0;
        'label0: loop {
            let var2 = self.func27(env, arg0);
            if (var2 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.func27(env, arg0.wrapping_add(64));
            if (var3 == 0) as i32 != 0 {
                break 'label0;
            }
            let var4 = self.func27(env, arg0.wrapping_add(128));
            var1 = var4;
            break;
        }
        var1
    }
    fn func27(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        var1 = 0;
        'label0: loop {
            let var2 = self.func29(env, arg0);
            if (var2 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.func29(env, arg0.wrapping_add(32));
            var1 = var3;
            break;
        }
        var1
    }
    fn func28(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        var1 = 0;
        'label0: loop {
            let var2 = self.func26(env, arg0);
            if (var2 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.func26(env, arg0.wrapping_add(192));
            var1 = var3;
            break;
        }
        var1
    }
    fn func29(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func44(env, arg0, 1051416);
        var1
    }
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
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
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let mut var20: i32 = 0;
        let mut var21: i32 = 0;
        let mut var22: i32 = 0;
        let mut var23: i32 = 0;
        let var24 = self.global0;
        var2 = var24.wrapping_sub(1760);
        self.global0 = var2;
        var3 = arg1.wrapping_add(152);
        var4 = arg1.wrapping_add(217);
        var5 = arg1.wrapping_add(88);
        var6 = arg1.wrapping_add(73);
        var7 = arg1.wrapping_add(8);
        var8 = var2.wrapping_add(424);
        let var25 = self.memory.load32(arg1 as usize + 80) as i32;
        var9 = var25;
        let var26 = self.memory.load32(arg1 as usize) as i32;
        var10 = var26;
        var11 = var2.wrapping_add(728).wrapping_add(64);
        var12 = var2.wrapping_add(864).wrapping_add(128);
        var13 = var2.wrapping_add(864).wrapping_add(64);
        var14 = var2.wrapping_add(864).wrapping_add(129);
        var15 = var2.wrapping_add(264).wrapping_add(128);
        var16 = var2.wrapping_add(264).wrapping_add(64);
        var17 = var2.wrapping_add(864).wrapping_add(8);
        var18 = var2.wrapping_add(72).wrapping_add(129);
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        let var27 = self.memory.load32(arg1 as usize + 4) as i32;
                                        if (var27 == var10) as i32 != 0 {
                                            break 'label7;
                                        }
                                        var10 = 1;
                                        self.memory.store32(arg1 as usize, 1 as u32);
                                        let var28 = self.func121(env, var2.wrapping_add(864), var7, 64);
                                        var28;
                                        let mut slot_var6_0_i32 = self.memory.load32(var6 as usize) as i32;
                                        let mut slot_var2_264_i32 = slot_var6_0_i32 as i32;
                                        let var29 = self.memory.load32(var6.wrapping_add(3) as usize) as i32;
                                        let mut slot_var2_267_i32 = var29 as i32;
                                        let var30 = self.memory.load8(arg1 as usize + 72) as i32;
                                        var19 = var30;
                                        let var31 = self.memory.load32(arg1 as usize + 84) as i32;
                                        if (var31 != var9) as i32 != 0 {
                                            break 'label6;
                                        }
                                        if (var19 & 255 != 2) as i32 != 0 {
                                            break 'label5;
                                        }
                                        break 'label1;
                                        break;
                                    }
                                    var19 = 2;
                                    let var32 = self.memory.load32(arg1 as usize + 84) as i32;
                                    if (var32 == var9) as i32 != 0 {
                                        break 'label1;
                                    }
                                    break;
                                }
                                self.memory.store32(arg1 as usize + 80, 1 as u32);
                                let var33 = self.memory.load8(arg1 as usize + 216) as i32;
                                var20 = var33;
                                'label8: loop {
                                    if (var19 & 255 == 2) as i32 != 0 {
                                        break 'label8;
                                    }
                                    if (var20 & 255 == 2) as i32 != 0 {
                                        break 'label5;
                                    }
                                    let var34 = self.func121(env, var2.wrapping_add(8), var2.wrapping_add(864), 64);
                                    var34;
                                    let mut slot_var2_211_i32 = slot_var2_267_i32 as i32;
                                    let mut slot_var2_208_i32 = slot_var2_264_i32 as i32;
                                    let var35 = self.func121(env, var2.wrapping_add(72), var5, 128);
                                    var35;
                                    let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
                                    let mut slot_var18_0_i32 = slot_var4_0_i32 as i32;
                                    var21 = var4.wrapping_add(3);
                                    let mut slot_var21_0_i32 = self.memory.load32(var21 as usize) as i32;
                                    self.memory.store32(var18.wrapping_add(3) as usize, slot_var21_0_i32 as u32);
                                    self.memory.store8(var2 as usize + 200, var20 as u8);
                                    if (var20 & 1 == 0) as i32 != 0 {
                                        break 'label4;
                                    }
                                    var20 = 0;
                                    var9 = 8;
                                    break 'label3;
                                    break;
                                }
                                if (var20 & 255 == 2) as i32 != 0 {
                                    break 'label1;
                                }
                                break;
                            }
                            self.func31(env, 65);
                            unreachable!();
                            break;
                        }
                        var9 = var2.wrapping_add(264).wrapping_add(24);
                        let var37 = self.memory.load64(0 as usize + 1051248) as i64;
                        let mut slot_var9_0_i64 = var37 as i64;
                        var22 = var2.wrapping_add(264).wrapping_add(16);
                        let var38 = self.memory.load64(0 as usize + 1051240) as i64;
                        let mut slot_var22_0_i64 = var38 as i64;
                        var23 = var2.wrapping_add(264).wrapping_add(8);
                        let var39 = self.memory.load64(0 as usize + 1051232) as i64;
                        let mut slot_var23_0_i64 = var39 as i64;
                        let var40 = self.memory.load64(0 as usize + 1051224) as i64;
                        let mut slot_var2_264_i64 = var40 as i64;
                        self.func32(env, var2.wrapping_add(264));
                        self.memory.store64(var2.wrapping_add(728).wrapping_add(24) as usize, slot_var9_0_i64 as u64);
                        self.memory.store64(var2.wrapping_add(728).wrapping_add(16) as usize, slot_var22_0_i64 as u64);
                        self.memory.store64(var2.wrapping_add(728).wrapping_add(8) as usize, slot_var23_0_i64 as u64);
                        let mut slot_var2_728_i64 = slot_var2_264_i64 as i64;
                        self.func33(env, var2.wrapping_add(864), var2.wrapping_add(728));
                        'label9: loop {
                            'label10: loop {
                                let mut slot_var2_864_i32 = self.memory.load32(var2 as usize + 864) as i32;
                                if (slot_var2_864_i32 == 0) as i32 != 0 {
                                    break 'label10;
                                }
                                let var43 = self.memory.load64(var17.wrapping_add(24) as usize) as i64;
                                self.memory.store64(var2.wrapping_add(216).wrapping_add(24) as usize, var43 as u64);
                                let var44 = self.memory.load64(var17.wrapping_add(16) as usize) as i64;
                                self.memory.store64(var2.wrapping_add(216).wrapping_add(16) as usize, var44 as u64);
                                let var45 = self.memory.load64(var17.wrapping_add(8) as usize) as i64;
                                self.memory.store64(var2.wrapping_add(216).wrapping_add(8) as usize, var45 as u64);
                                let mut slot_var17_0_i64 = self.memory.load64(var17 as usize) as i64;
                                let mut slot_var2_216_i64 = slot_var17_0_i64 as i64;
                                var22 = 0;
                                let mut slot_var2_260_i32 = 0 as i32;
                                let mut slot_var2_252_i64 = 34359738368 as i64;
                                let var46 = self.func121(env, var2.wrapping_add(264), var5, 64);
                                var46;
                                let var47 = self.func121(env, var16, var3, 64);
                                var9 = var47;
                                self.memory.store64(var8.wrapping_add(24) as usize, 0 /* False */ as u64);
                                self.memory.store64(var8.wrapping_add(16) as usize, 0 /* False */ as u64);
                                self.memory.store64(var8.wrapping_add(8) as usize, 0 /* False */ as u64);
                                let mut slot_var8_0_i64 = 0 /* False */ as i64;
                                let var48 = self.memory.load64(0 as usize + 1051224) as i64;
                                let mut slot_var15_0_i64 = var48 as i64;
                                let var49 = self.memory.load64(0 as usize + 1051232) as i64;
                                self.memory.store64(var15.wrapping_add(8) as usize, var49 as u64);
                                let var50 = self.memory.load64(0 as usize + 1051240) as i64;
                                self.memory.store64(var15.wrapping_add(16) as usize, var50 as u64);
                                let var51 = self.memory.load64(0 as usize + 1051248) as i64;
                                self.memory.store64(var15.wrapping_add(24) as usize, var51 as u64);
                                let var52 = self.func121(env, var2.wrapping_add(864), var5, 128);
                                var52;
                                let mut slot_var14_0_i32 = slot_var4_0_i32 as i32;
                                self.memory.store32(var14.wrapping_add(3) as usize, slot_var21_0_i32 as u32);
                                self.memory.store8(var2 as usize + 992, var20 as u8);
                                let var53 = self.func15(env, var13);
                                var53;
                                let var54 = self.func121(env, var2.wrapping_add(456), var2.wrapping_add(864), 136);
                                var54;
                                var23 = 1;
                                var20 = 1050625;
                                'label11: loop {
                                    'label12: loop {
                                        'label13: loop {
                                            if (var22 & 1 == 0) as i32 != 0 {
                                                break 'label13;
                                            }
                                            if (var20 != 1050560) as i32 != 0 {
                                                break 'label12;
                                            }
                                            break 'label9;
                                            break;
                                        }
                                        if (var23 as u32 >= var20.wrapping_sub(1050560) as u32) as i32 != 0 {
                                            break 'label9;
                                        }
                                        var20 = var20.wrapping_sub(var23);
                                        break;
                                    }
                                    let var55 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(264), 64);
                                    var55;
                                    self.func5(env, var2.wrapping_add(728), var9);
                                    let var57 = self.func121(env, var2.wrapping_add(1056), var2.wrapping_add(728), 64);
                                    var57;
                                    self.func34(env, var2.wrapping_add(1056), var2.wrapping_add(216));
                                    self.func7(env, var2.wrapping_add(1120), var9);
                                    self.func7(env, var2.wrapping_add(1184), var15);
                                    self.func8(env, var2.wrapping_add(728), var2.wrapping_add(1184));
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1184));
                                    let var63 = self.func121(env, var2.wrapping_add(592), var2.wrapping_add(728), 64);
                                    var63;
                                    let var64 = self.func121(env, var2.wrapping_add(728), 1050688, 64);
                                    var64;
                                    self.func5(env, var2.wrapping_add(728), var2.wrapping_add(592));
                                    let var66 = self.func121(env, var2.wrapping_add(1248), var2.wrapping_add(728), 64);
                                    var66;
                                    self.func8(env, var2.wrapping_add(728), var2.wrapping_add(1248));
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1248));
                                    let var69 = self.func121(env, var2.wrapping_add(1312), var2.wrapping_add(728), 64);
                                    var69;
                                    let var70 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1120), 64);
                                    var70;
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1312));
                                    let var72 = self.func121(env, var2.wrapping_add(1376), var2.wrapping_add(728), 64);
                                    var72;
                                    self.func34(env, var2.wrapping_add(1376), var2.wrapping_add(216));
                                    let var74 = self.func121(env, var2.wrapping_add(728), var9, 64);
                                    var74;
                                    self.func10(env, var2.wrapping_add(728), var15);
                                    let var76 = self.func121(env, var2.wrapping_add(1696), var2.wrapping_add(728), 64);
                                    var76;
                                    self.func7(env, var2.wrapping_add(1632), var2.wrapping_add(1696));
                                    let var78 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1120), 64);
                                    var78;
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1184));
                                    let var80 = self.func121(env, var2.wrapping_add(592), var2.wrapping_add(728), 64);
                                    var80;
                                    self.func9(env, var2.wrapping_add(1632), var2.wrapping_add(592));
                                    let var82 = self.func121(env, var2.wrapping_add(1440), var2.wrapping_add(1632), 64);
                                    var82;
                                    let var83 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1248), 64);
                                    var83;
                                    self.func9(env, var2.wrapping_add(728), var2.wrapping_add(1120));
                                    let var85 = self.func121(env, var2.wrapping_add(1504), var2.wrapping_add(728), 64);
                                    var85;
                                    self.func7(env, var2.wrapping_add(1568), var2.wrapping_add(264));
                                    self.func7(env, var2.wrapping_add(1632), var2.wrapping_add(1248));
                                    let var88 = self.func121(env, var2.wrapping_add(1696), var2.wrapping_add(1056), 64);
                                    var88;
                                    let var89 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1120), 64);
                                    var89;
                                    self.func9(env, var2.wrapping_add(728), var2.wrapping_add(1312));
                                    let var91 = self.func121(env, var2.wrapping_add(592), var2.wrapping_add(728), 64);
                                    var91;
                                    self.func5(env, var2.wrapping_add(1696), var2.wrapping_add(592));
                                    let var93 = self.func121(env, var2.wrapping_add(264), var2.wrapping_add(1696), 64);
                                    var93;
                                    self.func7(env, var2.wrapping_add(1696), var2.wrapping_add(1376));
                                    self.func8(env, var2.wrapping_add(728), var2.wrapping_add(1632));
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1632));
                                    let var97 = self.func121(env, var2.wrapping_add(592), var2.wrapping_add(728), 64);
                                    var97;
                                    self.func9(env, var2.wrapping_add(1696), var2.wrapping_add(592));
                                    let var99 = self.func121(env, var9, var2.wrapping_add(1696), 64);
                                    var99;
                                    let var100 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1120), 64);
                                    var100;
                                    self.func5(env, var2.wrapping_add(728), var2.wrapping_add(1440));
                                    let var102 = self.func121(env, var15, var2.wrapping_add(728), 64);
                                    var102;
                                    let var103 = self.func121(env, var2.wrapping_add(728), var2.wrapping_add(1440), 64);
                                    var103;
                                    self.func25(env, var2.wrapping_add(864), var2.wrapping_add(728));
                                    self.func8(env, var2.wrapping_add(728), var2.wrapping_add(1568));
                                    self.func10(env, var2.wrapping_add(728), var2.wrapping_add(1568));
                                    let var107 = self.func121(env, var13, var2.wrapping_add(728), 64);
                                    var107;
                                    let var108 = self.func121(env, var12, var2.wrapping_add(1504), 64);
                                    var108;
                                    var20 = var20.wrapping_add(-1);
                                    let var109 = self.memory.load8(var20 as usize) as i32;
                                    var21 = var109;
                                    self.func35(env, var2.wrapping_add(252), var2.wrapping_add(864));
                                    var23 = 0;
                                    var22 = 1;
                                    'label14: loop {
                                        if (var21 == 255) as i32 != 0 {
                                            break 'label14;
                                        }
                                        if (var21 != 1) as i32 != 0 {
                                            continue 'label11;
                                        }
                                        self.func36(env, var2.wrapping_add(864), var2.wrapping_add(264), var2.wrapping_add(72));
                                        self.func35(env, var2.wrapping_add(252), var2.wrapping_add(864));
                                        continue 'label11;
                                        break;
                                    }
                                    self.func36(env, var2.wrapping_add(864), var2.wrapping_add(264), var2.wrapping_add(456));
                                    self.func35(env, var2.wrapping_add(252), var2.wrapping_add(864));
                                    continue 'label11;
                                    break;
                                }
                                break;
                            }
                            self.func37(env);
                            unreachable!();
                            break;
                        }
                        self.func38(env, var2.wrapping_add(592), var2.wrapping_add(72));
                        self.func38(env, var2.wrapping_add(728), var2.wrapping_add(592));
                        let var118 = self.func121(env, var2.wrapping_add(864), var11, 64);
                        var118;
                        self.func25(env, var11, var2.wrapping_add(864));
                        self.func36(env, var2.wrapping_add(864), var2.wrapping_add(264), var2.wrapping_add(592));
                        self.func35(env, var2.wrapping_add(252), var2.wrapping_add(864));
                        self.func36(env, var2.wrapping_add(864), var2.wrapping_add(264), var2.wrapping_add(728));
                        self.func35(env, var2.wrapping_add(252), var2.wrapping_add(864));
                        var20 = slot_var2_252_i64;
                        let mut slot_var2_256_i32 = self.memory.load32(var2 as usize + 256) as i32;
                        var9 = slot_var2_256_i32;
                        if var19 & 1 != 0 {
                            break 'label3;
                        }
                        var21 = slot_var2_260_i32;
                        self.memory.store32(arg0 as usize + 65, slot_var2_208_i32 as u32);
                        self.memory.store32(arg0.wrapping_add(68) as usize, slot_var2_211_i32 as u32);
                        let var124 = self.func121(env, arg0, var2.wrapping_add(8), 64);
                        var15 = var124;
                        let mut slot_var15_84_i32 = var9.wrapping_add(var21.wrapping_mul(192)) as i32;
                        let mut slot_var15_80_i32 = var20 as i32;
                        let mut slot_var15_76_i32 = var9 as i32;
                        let mut slot_var15_72_i32 = var9 as i32;
                        self.memory.store8(var15 as usize + 64, var19 as u8);
                        break 'label0;
                        break;
                    }
                    self.func39(env, var20, var9);
                    var9 = 1;
                    continue 'label2;
                    break;
                }
                break;
            }
            self.memory.store8(arg0 as usize + 64, 2 as u8);
            break;
        }
        self.global0 = var2.wrapping_add(1760);
    }
    fn func31(&mut self, env: &Env, mut arg0: i32) {
        self.func96(env);
        unreachable!();
    }
    fn func32(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        var1 = 0 /* False */;
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 32) as i32 != 0 {
                    break 'label0;
                }
                var3 = arg0.wrapping_add(var2);
                let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                var4 = slot_var3_0_i64;
                slot_var3_0_i64 = (var4.wrapping_shl(1 /* True */ as u32) | var1) as i64;
                var2 = var2.wrapping_add(8);
                var1 = (var4 as u64).wrapping_shr(63 as u32) as i64;
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            let var5 = self.func99(env, arg0);
            if (var5 == 0) as i32 != 0 {
                break 'label2;
            }
            let var6 = self.memory.load64(arg0 as usize) as i64;
            var1 = var6;
            self.memory.store64(arg0 as usize, var1.wrapping_add(14114127202429895353) as u64);
            let var7 = self.memory.load64(arg0 as usize + 8) as i64;
            var4 = var7;
            var1 = var4.wrapping_add({ let a = 7529619929231668594; let b = 7529619929231668595; if ((var1 as u64) < 4332616871279656263 as u64) as i32 != 0 { a } else { b } });
            self.memory.store64(arg0 as usize + 8, var1 as u64);
            var1 = { let a = 5165552122434856866; let b = 5165552122434856867; if (var1 as u64 >= var4 as u64) as i32 != 0 { a } else { b } };
            let var8 = self.memory.load64(arg0 as usize + 16) as i64;
            var4 = var1.wrapping_add(var8);
            self.memory.store64(arg0 as usize + 16, var4 as u64);
            let var9 = self.memory.load64(arg0 as usize + 24) as i64;
            self.memory.store64(arg0 as usize + 24, ({ let a = 14959745806906580950; let b = 14959745806906580951; if (var4 as u64 >= var1 as u64) as i32 != 0 { a } else { b } }).wrapping_add(var9) as u64);
            break;
        }
    }
    fn func33(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(160);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                let var4 = self.func29(env, arg1);
                if var4 != 0 {
                    break 'label1;
                }
                self.memory.store64(var2.wrapping_add(24) as usize, 0 /* False */ as u64);
                self.memory.store64(var2.wrapping_add(16) as usize, 0 /* False */ as u64);
                let mut slot_var2_8_i64 = 0 /* False */ as i64;
                let mut slot_var2_0_i64 = 1 /* True */ as i64;
                let var5 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
                self.memory.store64(var2.wrapping_add(32).wrapping_add(24) as usize, var5 as u64);
                let var6 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
                self.memory.store64(var2.wrapping_add(32).wrapping_add(16) as usize, var6 as u64);
                let var7 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
                self.memory.store64(var2.wrapping_add(32).wrapping_add(8) as usize, var7 as u64);
                let var8 = self.memory.load64(arg1 as usize) as i64;
                let mut slot_var2_32_i64 = var8 as i64;
                let var9 = self.memory.load64(0 as usize + 1051408) as i64;
                self.memory.store64(var2.wrapping_add(64).wrapping_add(24) as usize, var9 as u64);
                let var10 = self.memory.load64(0 as usize + 1051400) as i64;
                self.memory.store64(var2.wrapping_add(64).wrapping_add(16) as usize, var10 as u64);
                let var11 = self.memory.load64(0 as usize + 1051392) as i64;
                self.memory.store64(var2.wrapping_add(64).wrapping_add(8) as usize, var11 as u64);
                let var12 = self.memory.load64(0 as usize + 1051384) as i64;
                let mut slot_var2_64_i64 = var12 as i64;
                let var13 = self.memory.load64(0 as usize + 1050776) as i64;
                self.memory.store64(var2.wrapping_add(96).wrapping_add(24) as usize, var13 as u64);
                let var14 = self.memory.load64(0 as usize + 1050768) as i64;
                self.memory.store64(var2.wrapping_add(96).wrapping_add(16) as usize, var14 as u64);
                let var15 = self.memory.load64(0 as usize + 1050760) as i64;
                self.memory.store64(var2.wrapping_add(96).wrapping_add(8) as usize, var15 as u64);
                let var16 = self.memory.load64(0 as usize + 1050752) as i64;
                let mut slot_var2_96_i64 = var16 as i64;
                self.memory.store64(var2.wrapping_add(128).wrapping_add(24) as usize, 0 /* False */ as u64);
                self.memory.store64(var2.wrapping_add(128).wrapping_add(16) as usize, 0 /* False */ as u64);
                self.memory.store64(var2.wrapping_add(128).wrapping_add(8) as usize, 0 /* False */ as u64);
                let mut slot_var2_128_i64 = 0 /* False */ as i64;
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var17 = self.func45(env, var2.wrapping_add(32), var2);
                            if (var17 == 0) as i32 != 0 {
                                break 'label4;
                            }
                            let var18 = self.func45(env, var2.wrapping_add(64), var2);
                            if var18 != 0 {
                                break 'label3;
                            }
                            break;
                        }
                        let var19 = self.func46(env, var2.wrapping_add(32), var2);
                        arg1 = var19;
                        self.memory.store64(arg0 as usize, 1 /* True */ as u64);
                        arg1 = { let a = var2.wrapping_add(96); let b = var2.wrapping_add(128); if arg1 != 0 { a } else { b } };
                        let var20 = self.memory.load64(arg1 as usize) as i64;
                        self.memory.store64(arg0 as usize + 8, var20 as u64);
                        let var21 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
                        self.memory.store64(arg0.wrapping_add(32) as usize, var21 as u64);
                        let var22 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
                        self.memory.store64(arg0.wrapping_add(24) as usize, var22 as u64);
                        let var23 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
                        self.memory.store64(arg0.wrapping_add(16) as usize, var23 as u64);
                        break 'label0;
                        break;
                    }
                    'label5: loop {
                        'label6: loop {
                            let var24 = self.memory.load8(var2 as usize + 32) as i32;
                            if var24 & 1 != 0 {
                                break 'label5;
                            }
                            self.func77(env, var2.wrapping_add(32));
                            'label7: loop {
                                let var26 = self.memory.load8(var2 as usize + 96) as i32;
                                if (var26 & 1 == 0) as i32 != 0 {
                                    break 'label7;
                                }
                                self.func78(env, var2.wrapping_add(96));
                                break;
                            }
                            self.func77(env, var2.wrapping_add(96));
                            continue 'label6;
                            break;
                        }
                        break;
                    }
                    'label8: loop {
                        'label9: loop {
                            let var29 = self.memory.load8(var2 as usize + 64) as i32;
                            if (var29 & 1 == 0) as i32 != 0 {
                                break 'label9;
                            }
                            'label10: loop {
                                let var30 = self.func75(env, var2.wrapping_add(64), var2.wrapping_add(32));
                                if (var30.wrapping_shl(24 as u32).wrapping_shr(24 as u32) < 0) as i32 != 0 {
                                    break 'label10;
                                }
                                self.func79(env, var2.wrapping_add(64), var2.wrapping_add(32));
                                self.func43(env, var2.wrapping_add(128), var2.wrapping_add(96));
                                continue 'label2;
                                break;
                            }
                            self.func79(env, var2.wrapping_add(32), var2.wrapping_add(64));
                            self.func43(env, var2.wrapping_add(96), var2.wrapping_add(128));
                            continue 'label2;
                            break;
                        }
                        self.func77(env, var2.wrapping_add(64));
                        'label11: loop {
                            let var36 = self.memory.load8(var2 as usize + 128) as i32;
                            if (var36 & 1 == 0) as i32 != 0 {
                                break 'label11;
                            }
                            self.func78(env, var2.wrapping_add(128));
                            break;
                        }
                        self.func77(env, var2.wrapping_add(128));
                        continue 'label8;
                        break;
                    }
                    break;
                }
                break;
            }
            self.memory.store64(arg0 as usize, 0 /* False */ as u64);
            break;
        }
        self.global0 = var2.wrapping_add(160);
    }
    fn func34(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func70(env, arg0, arg1);
        self.func70(env, arg0.wrapping_add(32), arg1);
    }
    fn func35(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        'label0: loop {
            let var3 = self.memory.load32(arg0 as usize + 8) as i32;
            var2 = var3;
            let var4 = self.memory.load32(arg0 as usize) as i32;
            if (var2 != var4) as i32 != 0 {
                break 'label0;
            }
            self.func57(env, arg0);
            break;
        }
        let var6 = self.memory.load32(arg0 as usize + 4) as i32;
        let var7 = self.func121(env, var6.wrapping_add(var2.wrapping_mul(192)), arg1, 192);
        var7;
        self.memory.store32(arg0 as usize + 8, var2.wrapping_add(1) as u32);
    }
    fn func36(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var3 = var7.wrapping_sub(832);
        self.global0 = var3;
        var4 = arg1.wrapping_add(64);
        let var8 = self.func121(env, var3, var4, 64);
        var3 = var8;
        var5 = arg2.wrapping_add(64);
        let var9 = self.func121(env, var3.wrapping_add(768), var5, 64);
        var9;
        var6 = arg1.wrapping_add(128);
        self.func5(env, var3.wrapping_add(768), var6);
        let var11 = self.func121(env, var3.wrapping_add(704), var3.wrapping_add(768), 64);
        var11;
        self.func9(env, var3, var3.wrapping_add(704));
        let var13 = self.func121(env, var3.wrapping_add(64), arg1, 64);
        var13;
        let var14 = self.func121(env, var3.wrapping_add(768), arg2, 64);
        var14;
        self.func5(env, var3.wrapping_add(768), var6);
        let var16 = self.func121(env, var3.wrapping_add(704), var3.wrapping_add(768), 64);
        var16;
        self.func9(env, var3.wrapping_add(64), var3.wrapping_add(704));
        self.func7(env, var3.wrapping_add(128), var3);
        self.func7(env, var3.wrapping_add(192), var3.wrapping_add(64));
        let var20 = self.func121(env, var3.wrapping_add(256), var3.wrapping_add(64), 64);
        var20;
        self.func5(env, var3.wrapping_add(256), var3.wrapping_add(192));
        let var22 = self.func121(env, var3.wrapping_add(320), var6, 64);
        var22;
        self.func5(env, var3.wrapping_add(320), var3.wrapping_add(128));
        let var24 = self.func121(env, var3.wrapping_add(384), arg1, 64);
        var24;
        self.func5(env, var3.wrapping_add(384), var3.wrapping_add(192));
        let var26 = self.func121(env, var3.wrapping_add(768), var3.wrapping_add(256), 64);
        var26;
        self.func10(env, var3.wrapping_add(768), var3.wrapping_add(320));
        let var28 = self.func121(env, var3.wrapping_add(704), var3.wrapping_add(768), 64);
        var28;
        self.func8(env, var3.wrapping_add(768), var3.wrapping_add(384));
        self.func9(env, var3.wrapping_add(704), var3.wrapping_add(768));
        let var31 = self.func121(env, var3.wrapping_add(448), var3.wrapping_add(704), 64);
        var31;
        let var32 = self.func121(env, var3.wrapping_add(512), var3.wrapping_add(64), 64);
        var32;
        self.func5(env, var3.wrapping_add(512), var3.wrapping_add(448));
        let var34 = self.func121(env, arg1, var3.wrapping_add(512), 64);
        var34;
        let var35 = self.func121(env, var3.wrapping_add(576), var3, 64);
        var35;
        self.func9(env, var3.wrapping_add(384), var3.wrapping_add(448));
        self.func5(env, var3.wrapping_add(576), var3.wrapping_add(384));
        let var38 = self.func121(env, var3.wrapping_add(768), var3.wrapping_add(256), 64);
        var38;
        self.func5(env, var3.wrapping_add(768), var4);
        let var40 = self.func121(env, var3.wrapping_add(704), var3.wrapping_add(768), 64);
        var40;
        self.func9(env, var3.wrapping_add(576), var3.wrapping_add(704));
        let var42 = self.func121(env, var4, var3.wrapping_add(576), 64);
        var42;
        self.func5(env, var6, var3.wrapping_add(256));
        let var44 = self.func121(env, var3.wrapping_add(640), var3, 64);
        var44;
        self.func5(env, var3.wrapping_add(640), arg2);
        let var46 = self.func121(env, var3.wrapping_add(768), var3.wrapping_add(64), 64);
        var46;
        self.func5(env, var3.wrapping_add(768), var5);
        let var48 = self.func121(env, var3.wrapping_add(704), var3.wrapping_add(768), 64);
        var48;
        self.func9(env, var3.wrapping_add(640), var3.wrapping_add(704));
        let var50 = self.func121(env, arg0, var3.wrapping_add(64), 64);
        arg1 = var50;
        self.func25(env, arg1.wrapping_add(64), var3);
        let var52 = self.func121(env, arg1.wrapping_add(128), var3.wrapping_add(640), 64);
        var52;
        self.global0 = var3.wrapping_add(832);
    }
    fn func37(&mut self, env: &Env) {
        self.func31(env, 43);
        unreachable!();
    }
    fn func38(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(144);
        self.global0 = var2;
        let var4 = self.func121(env, var2.wrapping_add(8), arg1, 136);
        var4;
        self.func70(env, var2.wrapping_add(40), 1051352);
        self.func5(env, var2.wrapping_add(8), 1051464);
        self.func70(env, var2.wrapping_add(104), 1051352);
        self.func5(env, var2.wrapping_add(72), 1051528);
        let var9 = self.func121(env, arg0, var2.wrapping_add(8), 136);
        var9;
        self.global0 = var2.wrapping_add(144);
    }
    fn func39(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func47(env, arg0, arg1, 8, 192);
    }
    fn func40(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        'label0: loop {
            let var4 = self.memory.load8(arg0 as usize) as i32;
            var1 = var4;
            var2 = var1 & 64;
            var3 = var1.wrapping_shl(24 as u32).wrapping_shr(24 as u32);
            var1 = { let a = { let a = -127; let b = -128; if var2 != 0 { a } else { b } }; let b = var2; if (var3 < 0) as i32 != 0 { a } else { b } };
            if (var1 == -127) as i32 != 0 {
                break 'label0;
            }
            self.memory.store8(arg0 as usize, (var3 & (var1 ^ -1)) as u8);
            break;
        }
        var1
    }
    fn func41(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i32 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i32 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let var18 = self.global0;
        var3 = var18.wrapping_sub(32);
        self.global0 = var3;
        var4 = 0;
        var5 = 0 /* False */;
        var6 = 0 /* False */;
        var7 = 0 /* False */;
        var8 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                if (var4 == 4) as i32 != 0 {
                    break 'label0;
                }
                var9 = 0 /* False */;
                var10 = 0;
                var11 = 0 /* False */;
                'label2: loop {
                    'label3: loop {
                        if (var10 == 64) as i32 != 0 {
                            break 'label2;
                        }
                        var12 = var11.wrapping_add(var9);
                        let var19 = var11;
                        var13 = arg2.wrapping_add(var10);
                        let var20 = self.memory.load64(var13.wrapping_add(24) as usize) as i64;
                        var14 = var20;
                        var15 = var14 & 4294967295;
                        let var21 = self.memory.load64(arg1.wrapping_add(var10) as usize) as i64;
                        var16 = var21;
                        var11 = var16 & 4294967295;
                        var17 = var15.wrapping_mul(var11);
                        var9 = var17.wrapping_add(var8);
                        let var22 = var9;
                        var14 = (var14 as u64).wrapping_shr(32 as u32) as i64;
                        var8 = (var16 as u64).wrapping_shr(32 as u32) as i64;
                        var16 = var14.wrapping_mul(var11);
                        var14 = var16.wrapping_add(var15.wrapping_mul(var8));
                        var14 = var9.wrapping_add(var14.wrapping_shl(32 as u32));
                        let var23 = self.memory.load64(var13.wrapping_add(16) as usize) as i64;
                        var15 = var23;
                        var16 = var15 & 4294967295;
                        var17 = var16.wrapping_mul(var11);
                        var9 = var17.wrapping_add(var7);
                        let var24 = var9;
                        var15 = (var15 as u64).wrapping_shr(32 as u32) as i64;
                        var7 = var15.wrapping_mul(var11);
                        var15 = var7.wrapping_add(var16.wrapping_mul(var8));
                        var15 = var9.wrapping_add(var15.wrapping_shl(32 as u32));
                        let var25 = self.memory.load64(var13.wrapping_add(8) as usize) as i64;
                        var7 = var25;
                        var16 = var7 & 4294967295;
                        var17 = var16.wrapping_mul(var11);
                        var9 = var17.wrapping_add(var6);
                        let var26 = var9;
                        var7 = (var7 as u64).wrapping_shr(32 as u32) as i64;
                        var6 = var7.wrapping_mul(var11);
                        var7 = var6.wrapping_add(var16.wrapping_mul(var8));
                        var7 = var9.wrapping_add(var7.wrapping_shl(32 as u32));
                        let mut slot_var13_0_i64 = self.memory.load64(var13 as usize) as i64;
                        var6 = slot_var13_0_i64;
                        var16 = var6 & 4294967295;
                        var17 = var16.wrapping_mul(var11);
                        var9 = var17.wrapping_add(var5);
                        var5 = (var6 as u64).wrapping_shr(32 as u32) as i64;
                        var5 = var5.wrapping_mul(var11);
                        var11 = var5.wrapping_add(var16.wrapping_mul(var8));
                        var5 = var9.wrapping_add(var11.wrapping_shl(32 as u32));
                        var6 = var7.wrapping_add((((var9 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_mul(var8)).wrapping_add((((var11 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var5 as u64) < var9 as u64) as i32 as u32 as i64));
                        var7 = var15.wrapping_add((((var26 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var7.wrapping_mul(var8)).wrapping_add((((var7 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var6 as u64) < var7 as u64) as i32 as u32 as i64));
                        var8 = var14.wrapping_add((((var24 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var15.wrapping_mul(var8)).wrapping_add((((var15 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var15 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var15 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var7 as u64) < var15 as u64) as i32 as u32 as i64));
                        var9 = var12.wrapping_add((((var22 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var14.wrapping_mul(var8)).wrapping_add((((var14 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var14 as u64) as i32 as u32 as i64));
                        var11 = (((var12 as u64) < var19 as u64) as i32 as u32 as i64).wrapping_add(((var9 as u64) < var12 as u64) as i32 as u32 as i64);
                        var10 = var10.wrapping_add(32);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var27 = var11;
                var9 = var5.wrapping_mul(9786893198990664585);
                var11 = var9 & 4294967295;
                var14 = var11.wrapping_mul(3778125865);
                var12 = var14.wrapping_add(var8);
                let var28 = var12;
                var8 = (var9 as u64).wrapping_shr(32 as u32) as i64;
                var14 = var11.wrapping_mul(811880050);
                var9 = var14.wrapping_add(var8.wrapping_mul(3778125865));
                var9 = var12.wrapping_add(var9.wrapping_shl(32 as u32));
                var14 = var11.wrapping_mul(2172737629);
                var12 = var14.wrapping_add(var7);
                let var29 = var12;
                var15 = var11.wrapping_mul(3092268470);
                var14 = var15.wrapping_add(var8.wrapping_mul(2172737629));
                var14 = var12.wrapping_add(var14.wrapping_shl(32 as u32));
                var15 = var11.wrapping_mul(1752287885);
                var12 = var15.wrapping_add(var6);
                let var30 = var12;
                var7 = var11.wrapping_mul(2541841041);
                var15 = var7.wrapping_add(var8.wrapping_mul(1752287885));
                var15 = var12.wrapping_add(var15.wrapping_shl(32 as u32));
                var7 = var11.wrapping_mul(3632069959);
                var12 = var7.wrapping_add(var5);
                var7 = var11.wrapping_mul(1008765974);
                var11 = var7.wrapping_add(var8.wrapping_mul(3632069959));
                var5 = var15.wrapping_add((((var12 as u64) < var7 as u64) as i32 as u32 as i64 | var8.wrapping_mul(1008765974)).wrapping_add((((var11 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var12.wrapping_add(var11.wrapping_shl(32 as u32)) as u64) < var12 as u64) as i32 as u32 as i64));
                var6 = var14.wrapping_add((((var30 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_mul(2541841041)).wrapping_add((((var15 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var15 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var15 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var15 as u64) as i32 as u32 as i64));
                var7 = var9.wrapping_add((((var29 as u64) < var14 as u64) as i32 as u32 as i64 | var8.wrapping_mul(3092268470)).wrapping_add((((var14 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(((var6 as u64) < var14 as u64) as i32 as u32 as i64));
                var8 = var27.wrapping_add(var9).wrapping_add((((var28 as u64) < var14 as u64) as i32 as u32 as i64 | var8.wrapping_mul(811880050)).wrapping_add((((var9 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var9 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var9 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(((var7 as u64) < var9 as u64) as i32 as u32 as i64));
                arg1 = arg1.wrapping_add(8);
                var4 = var4.wrapping_add(1);
                continue 'label1;
                break;
            }
            break;
        }
        let mut slot_var3_24_i64 = var8 as i64;
        let mut slot_var3_16_i64 = var7 as i64;
        let mut slot_var3_8_i64 = var6 as i64;
        let mut slot_var3_0_i64 = var5 as i64;
        'label4: loop {
            let var31 = self.func99(env, var3);
            if (var31 == 0) as i32 != 0 {
                break 'label4;
            }
            slot_var3_0_i64 = var5.wrapping_add(14114127202429895353) as i64;
            var11 = var6.wrapping_add({ let a = 7529619929231668594; let b = 7529619929231668595; if ((var5 as u64) < 4332616871279656263 as u64) as i32 != 0 { a } else { b } });
            slot_var3_8_i64 = var11 as i64;
            var11 = { let a = 5165552122434856866; let b = 5165552122434856867; if (var11 as u64 >= var6 as u64) as i32 != 0 { a } else { b } };
            var12 = var11.wrapping_add(var7);
            slot_var3_16_i64 = var12 as i64;
            slot_var3_24_i64 = ({ let a = 14959745806906580950; let b = 14959745806906580951; if (var12 as u64 >= var11 as u64) as i32 != 0 { a } else { b } }).wrapping_add(var8) as i64;
            break;
        }
        self.memory.store64(arg0 as usize, slot_var3_0_i64 as u64);
        let var32 = self.memory.load64(var3.wrapping_add(24) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(24) as usize, var32 as u64);
        let var33 = self.memory.load64(var3.wrapping_add(16) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(16) as usize, var33 as u64);
        let var34 = self.memory.load64(var3.wrapping_add(8) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(8) as usize, var34 as u64);
        self.global0 = var3.wrapping_add(32);
    }
    fn func42(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let var12 = self.global0;
        var2 = var12.wrapping_sub(832);
        self.global0 = var2;
        let var13 = self.func121(env, var2, arg0, 192);
        var2 = var13;
        self.func21(env, var2, arg1);
        var3 = arg0.wrapping_add(192);
        let var15 = self.func121(env, var2.wrapping_add(192), var3, 192);
        var15;
        var4 = arg1.wrapping_add(192);
        self.func21(env, var2.wrapping_add(192), var4);
        self.func23(env, var3, arg0);
        let var18 = self.func121(env, var2.wrapping_add(576), arg1, 192);
        var18;
        self.func23(env, var2.wrapping_add(576), var4);
        let var20 = self.func121(env, var2.wrapping_add(384), var2.wrapping_add(576), 192);
        var20;
        self.func21(env, var3, var2.wrapping_add(384));
        self.func20(env, var3, var2);
        self.func20(env, var3, var2.wrapping_add(192));
        let var24 = self.func121(env, arg0, var2.wrapping_add(192), 192);
        arg0 = var24;
        let var25 = self.func121(env, arg0.wrapping_add(64), var2.wrapping_add(192), 64);
        var25;
        var3 = var2.wrapping_add(192).wrapping_add(128);
        let var26 = self.func121(env, arg0, var3, 64);
        arg1 = var26;
        let var27 = self.func121(env, var2.wrapping_add(384), var3, 64);
        var27;
        let var28 = self.func11(env, var2.wrapping_add(384));
        let var29 = self.func11(env, var28);
        let var30 = self.func11(env, var29);
        arg0 = var30;
        var5 = var2.wrapping_add(768).wrapping_add(24);
        let var31 = self.memory.load64(var2.wrapping_add(376) as usize) as i64;
        let mut slot_var5_0_i64 = var31 as i64;
        var6 = var2.wrapping_add(768).wrapping_add(16);
        let var32 = self.memory.load64(var2.wrapping_add(368) as usize) as i64;
        let mut slot_var6_0_i64 = var32 as i64;
        var7 = var2.wrapping_add(768).wrapping_add(8);
        let var33 = self.memory.load64(var2.wrapping_add(360) as usize) as i64;
        let mut slot_var7_0_i64 = var33 as i64;
        let mut slot_var2_352_i64 = self.memory.load64(var2 as usize + 352) as i64;
        let mut slot_var2_768_i64 = slot_var2_352_i64 as i64;
        self.func12(env, var2.wrapping_add(768));
        self.func13(env, var2.wrapping_add(768), arg0);
        self.func13(env, var2.wrapping_add(768), arg1);
        var4 = var2.wrapping_add(576).wrapping_add(24);
        let var37 = self.memory.load64(arg0.wrapping_add(56) as usize) as i64;
        let mut slot_var4_0_i64 = var37 as i64;
        var8 = var2.wrapping_add(576).wrapping_add(16);
        let var38 = self.memory.load64(arg0.wrapping_add(48) as usize) as i64;
        let mut slot_var8_0_i64 = var38 as i64;
        var9 = var2.wrapping_add(576).wrapping_add(8);
        let var39 = self.memory.load64(arg0.wrapping_add(40) as usize) as i64;
        let mut slot_var9_0_i64 = var39 as i64;
        let var40 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var2_576_i64 = var40 as i64;
        self.func13(env, var2.wrapping_add(576), var2.wrapping_add(352));
        arg0 = var2.wrapping_add(800).wrapping_add(24);
        self.memory.store64(arg0 as usize, slot_var4_0_i64 as u64);
        var10 = var2.wrapping_add(800).wrapping_add(16);
        let mut slot_var10_0_i64 = slot_var8_0_i64 as i64;
        var11 = var2.wrapping_add(800).wrapping_add(8);
        let mut slot_var11_0_i64 = slot_var9_0_i64 as i64;
        let mut slot_var2_800_i64 = slot_var2_576_i64 as i64;
        self.func13(env, var2.wrapping_add(800), var3);
        let var43 = self.memory.load64(arg0 as usize) as i64;
        self.memory.store64(var2.wrapping_add(576).wrapping_add(56) as usize, var43 as u64);
        self.memory.store64(var2.wrapping_add(576).wrapping_add(48) as usize, slot_var10_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(576).wrapping_add(40) as usize, slot_var11_0_i64 as u64);
        slot_var9_0_i64 = slot_var7_0_i64 as i64;
        slot_var8_0_i64 = slot_var6_0_i64 as i64;
        slot_var4_0_i64 = slot_var5_0_i64 as i64;
        let mut slot_var2_608_i64 = slot_var2_800_i64 as i64;
        slot_var2_576_i64 = slot_var2_768_i64 as i64;
        let var44 = self.func121(env, arg1, var2.wrapping_add(576), 64);
        arg0 = var44;
        let var45 = self.func121(env, arg0.wrapping_add(128), var2.wrapping_add(192).wrapping_add(64), 64);
        var45;
        self.func23(env, arg0, var2);
        self.global0 = var2.wrapping_add(832);
    }
    fn func43(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let var11 = self.func100(env, arg1, arg0);
        var2 = var11;
        let var12 = self.memory.load64(arg0 as usize) as i64;
        var3 = var12;
        'label0: loop {
            'label1: loop {
                if (var2.wrapping_shl(24 as u32).wrapping_shr(24 as u32) > 0) as i32 != 0 {
                    break 'label1;
                }
                let var13 = self.memory.load64(arg0 as usize + 24) as i64;
                var4 = var13;
                let var14 = self.memory.load64(arg0 as usize + 16) as i64;
                var5 = var14;
                let var15 = self.memory.load64(arg0 as usize + 8) as i64;
                var6 = var15;
                break 'label0;
                break;
            }
            let var16 = self.memory.load64(arg0 as usize + 24) as i64;
            let var17 = self.memory.load64(arg0 as usize + 16) as i64;
            var5 = var17;
            let var18 = self.memory.load64(arg0 as usize + 8) as i64;
            var4 = var18;
            var7 = var3.wrapping_add(4332616871279656263);
            var3 = var4.wrapping_add(((var7 as u64) < var3 as u64) as i32 as u32 as i64);
            var6 = var3.wrapping_add(10917124144477883021);
            var3 = var5.wrapping_add((((var3 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var6 as u64) < var3 as u64) as i32 as u32 as i64));
            var5 = var3.wrapping_add(13281191951274694749);
            var4 = var16.wrapping_add((((var3 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var3 as u64) as i32 as u32 as i64)).wrapping_add(3486998266802970665);
            var3 = var7;
            break;
        }
        let var19 = self.memory.load64(arg1 as usize) as i64;
        var7 = var19;
        self.memory.store64(arg0 as usize, var3.wrapping_sub(var7) as u64);
        var3 = { let a = 18446744073709551615; let b = 0 /* False */; if ((var3 as u64) < var7 as u64) as i32 != 0 { a } else { b } };
        let var20 = self.memory.load64(arg1 as usize + 8) as i64;
        var7 = var20;
        var8 = var3.wrapping_sub(var7);
        var6 = var8.wrapping_add(var6);
        self.memory.store64(arg0 as usize + 8, var6 as u64);
        let var21 = self.memory.load64(arg1 as usize + 16) as i64;
        var9 = var21;
        var10 = var5.wrapping_sub(var9);
        var3 = (var3.wrapping_sub(((var3 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(((var6 as u64) < var8 as u64) as i32 as u32 as i64) == 18446744073709551615) as i32 as u32 as i64;
        self.memory.store64(arg0 as usize + 16, var10.wrapping_sub(var3) as u64);
        let var22 = self.memory.load64(arg1 as usize + 24) as i64;
        self.memory.store64(arg0 as usize + 24, var4.wrapping_sub(var22).wrapping_sub(((((var5 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var10 as u64) < var3 as u64) as i32 as u32 as i64) == 1 /* True */) as i32 as u32 as i64) as u64);
    }
    fn func44(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func45(env, arg0, arg1);
        var2 ^ 1
    }
    fn func45(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func46(env, arg0, arg1);
        var2 ^ 1
    }
    fn func46(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func118(env, arg0, arg1, 32);
        (var2 == 0) as i32
    }
    fn func47(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(16);
        'label0: loop {
            'label1: loop {
                if arg0 != 0 {
                    break 'label1;
                }
                arg0 = 0;
                var4 = var4.wrapping_add(12);
                break 'label0;
                break;
            }
            let mut slot_var4_12_i32 = arg2 as i32;
            arg0 = arg0.wrapping_mul(arg3);
            var4 = var4.wrapping_add(8);
            break;
        }
        let mut slot_var4_0_i32 = arg0 as i32;
    }
    fn func48(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        'label0: loop {
            if (arg0 & 255 != 1) as i32 != 0 {
                break 'label0;
            }
            let var2 = self.memory.load32(arg1 as usize + 4) as i32;
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            arg0 = slot_var2_0_i32;
            if (arg0 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.memory.load32(arg1 as usize) as i32;
            /* TODO: call_indirect */ 0;
            break;
        }
    }
    fn func49(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.memory.load32(arg0 as usize + 12) as i32;
        let var6 = self.memory.load32(arg0 as usize + 8) as i32;
        var1 = var6;
        var2 = var5.wrapping_sub(var1);
        let var7 = self.memory.load32(arg0 as usize + 4) as i32;
        let var8 = self.memory.load32(arg0 as usize) as i32;
        var3 = var8;
        var4 = (var7.wrapping_sub(var3) as u32).wrapping_shr(3 as u32) as i32;
        var2 = { let a = var2; let b = var4; if ((var2 as u32) < var4 as u32) as i32 != 0 { a } else { b } };
        let var9 = var1;
        let var10 = self.memory.load32(arg0 as usize + 48) as i32;
        var1 = var10.wrapping_shl(3 as u32);
        arg0 = var9.wrapping_shl(3 as u32).wrapping_add(var1).wrapping_add(arg0).wrapping_add(16);
        var1 = var3.wrapping_add(var1);
        'label0: loop {
            'label1: loop {
                if (var2 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var11 = self.memory.load64(arg0 as usize) as i64;
                let mut slot_var1_0_i64 = var11 as i64;
                var2 = var2.wrapping_add(-1);
                var1 = var1.wrapping_add(8);
                arg0 = arg0.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
    }
    fn func50(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        'label0: loop {
            if (arg1 & 255 == 129) as i32 != 0 {
                break 'label0;
            }
            self.memory.store8(arg0 as usize, 5 as u8);
            self.memory.store8(arg0 as usize + 1, arg1 as u8);
            let var3 = self.memory.load8(arg2 as usize) as i32;
            let var4 = self.memory.load32(arg2 as usize + 4) as i32;
            self.func48(env, var3, var4);
            return;
            break;
        }
        let var6 = self.memory.load64(arg2 as usize) as i64;
        self.memory.store64(arg0 as usize, var6 as u64);
    }
    fn func51(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        'label0: loop {
            let var3 = self.memory.load8(arg1 as usize + 32) as i32;
            if (var3 == 129) as i32 != 0 {
                break 'label0;
            }
            let var4 = self.func121(env, arg0, arg1, 40);
            var4;
            let var5 = self.memory.load8(arg2 as usize) as i32;
            let var6 = self.memory.load32(arg2 as usize + 4) as i32;
            self.func48(env, var5, var6);
            return;
            break;
        }
        self.memory.store8(arg0 as usize + 32, 129 as u8);
        let var8 = self.memory.load64(arg2 as usize) as i64;
        self.memory.store64(arg0 as usize, var8 as u64);
    }
    fn func52(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        var3 = 1;
        'label0: loop {
            'label1: loop {
                let var4 = self.memory.load32(arg1 as usize) as i32;
                if (var4 != 1) as i32 != 0 {
                    break 'label1;
                }
                let var5 = self.memory.load64(arg1 as usize + 8) as i64;
                self.memory.store64(arg0 as usize + 8, var5 as u64);
                let var6 = self.memory.load64(arg1.wrapping_add(32) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(32) as usize, var6 as u64);
                let var7 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(24) as usize, var7 as u64);
                let var8 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(16) as usize, var8 as u64);
                let var9 = self.memory.load8(arg2 as usize) as i32;
                let var10 = self.memory.load32(arg2 as usize + 4) as i32;
                self.func48(env, var9, var10);
                var3 = 0;
                break 'label0;
                break;
            }
            let var12 = self.memory.load64(arg2 as usize) as i64;
            self.memory.store64(arg0 as usize + 4, var12 as u64);
            break;
        }
        self.memory.store32(arg0 as usize, var3 as u32);
    }
    fn func53(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(16);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var6 = self.memory.load32(arg1 as usize + 4) as i32;
                    var4 = var6;
                    if (var4 as u32 > 7 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    self.func54(env, var3.wrapping_add(8), 1, 27);
                    let mut slot_var3_8_i32 = self.memory.load32(var3 as usize + 8) as i32;
                    arg1 = slot_var3_8_i32;
                    if (arg1 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    let var8 = self.memory.load32(0 as usize + 1050648) as i32;
                    self.memory.store32(arg1.wrapping_add(23) as usize, var8 as u32);
                    let var9 = self.memory.load64(0 as usize + 1050641) as i64;
                    self.memory.store64(arg1.wrapping_add(16) as usize, var9 as u64);
                    let var10 = self.memory.load64(0 as usize + 1050633) as i64;
                    self.memory.store64(arg1.wrapping_add(8) as usize, var10 as u64);
                    let var11 = self.memory.load64(0 as usize + 1050625) as i64;
                    self.memory.store64(arg1 as usize, var11 as u64);
                    let var12 = self.func55(env);
                    arg2 = var12;
                    self.memory.store32(arg2 as usize + 8, 27 as u32);
                    self.memory.store32(arg2 as usize + 4, arg1 as u32);
                    self.memory.store32(arg2 as usize, 27 as u32);
                    let var13 = self.func55(env);
                    arg1 = var13;
                    self.memory.store8(arg1 as usize + 8, 17 as u8);
                    self.memory.store32(arg1 as usize + 4, 1051608 as u32);
                    self.memory.store32(arg1 as usize, arg2 as u32);
                    self.memory.store32(arg0 as usize + 4, arg1 as u32);
                    self.memory.store8(arg0 as usize, 1 as u8);
                    break 'label1;
                    break;
                }
                self.memory.store8(arg0 as usize, 2 as u8);
                self.memory.store32(arg1 as usize + 4, var4.wrapping_add(-8) as u32);
                let var14 = self.memory.load32(arg1 as usize) as i32;
                arg0 = var14;
                let var15 = self.memory.load64(arg0 as usize) as i64;
                self.memory.store64(arg2 as usize, var15 as u64);
                self.memory.store32(arg1 as usize, arg0.wrapping_add(8) as u32);
                break;
            }
            self.global0 = var3.wrapping_add(16);
            return;
            break;
        }
        self.func56(env, 1, 27);
        unreachable!();
    }
    fn func54(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let var3 = self.func92(env, arg2, arg1);
        arg1 = var3;
        self.memory.store32(arg0 as usize + 4, arg2 as u32);
        self.memory.store32(arg0 as usize, arg1 as u32);
    }
    fn func55(&mut self, env: &Env) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func54(env, var0.wrapping_add(8), 4, 12);
        'label0: loop {
            let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
            var1 = slot_var0_8_i32;
            if var1 != 0 {
                break 'label0;
            }
            self.func95(env, 12);
            unreachable!();
            break;
        }
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func56(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        'label0: loop {
            if (arg0 == 0) as i32 != 0 {
                break 'label0;
            }
            self.func95(env, arg1);
            unreachable!();
            break;
        }
        self.func73(env);
        unreachable!();
    }
    fn func57(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        let var3 = self.memory.load32(arg0 as usize) as i32;
        self.func58(env, var1.wrapping_add(8), arg0, var3, 8, 192);
        'label0: loop {
            let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
            arg0 = slot_var1_8_i32;
            if (arg0 == -2147483647) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
            self.func56(env, arg0, slot_var1_12_i32);
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(16);
    }
    fn func58(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i64 = 0;
        let mut var10: i32 = 0;
        let var11 = self.global0;
        var5 = var11.wrapping_sub(32);
        self.global0 = var5;
        var6 = 0;
        'label0: loop {
            'label1: loop {
                arg2 = arg2.wrapping_add(1);
                let var12 = self.memory.load32(arg1 as usize) as i32;
                var7 = var12;
                var8 = var7.wrapping_shl(1 as u32);
                arg2 = { let a = arg2; let b = var8; if (arg2 as u32 > var8 as u32) as i32 != 0 { a } else { b } };
                var8 = { let a = 8; let b = 4; if (arg4 == 1) as i32 != 0 { a } else { b } };
                var8 = { let a = arg2; let b = var8; if (arg2 as u32 > var8 as u32) as i32 != 0 { a } else { b } };
                var9 = ((arg3.wrapping_add(arg4).wrapping_add(-1) & 0.wrapping_sub(arg3)) as u32 as i64).wrapping_mul(var8 as u32 as i64);
                if (var9 as u64).wrapping_shr(32 as u32) as i64 as i32 != 0 {
                    break 'label1;
                }
                arg2 = var9 as i32;
                if (arg2 as u32 > (-2147483648).wrapping_sub(arg3) as u32) as i32 != 0 {
                    break 'label0;
                }
                'label2: loop {
                    'label3: loop {
                        if var7 != 0 {
                            break 'label3;
                        }
                        arg4 = 0;
                        var6 = var5.wrapping_add(28);
                        break 'label2;
                        break;
                    }
                    let mut slot_var5_28_i32 = arg3 as i32;
                    arg4 = var7.wrapping_mul(arg4);
                    let var13 = self.memory.load32(arg1 as usize + 4) as i32;
                    var7 = var13;
                    var6 = var5.wrapping_add(24);
                    break;
                }
                let mut slot_var6_0_i32 = arg4 as i32;
                'label4: loop {
                    'label5: loop {
                        'label6: loop {
                            if (slot_var5_28_i32 == 0) as i32 != 0 {
                                break 'label6;
                            }
                            'label7: loop {
                                let mut slot_var5_24_i32 = self.memory.load32(var5 as usize + 24) as i32;
                                var10 = slot_var5_24_i32;
                                if var10 != 0 {
                                    break 'label7;
                                }
                                self.func113(env, var5.wrapping_add(8), arg3, arg2, 0);
                                let mut slot_var5_8_i32 = self.memory.load32(var5 as usize + 8) as i32;
                                arg4 = slot_var5_8_i32;
                                break 'label5;
                                break;
                            }
                            self.func114(env);
                            let var16 = self.memory.load32(0 as usize + 1053988) as i32;
                            self.func115(env, var5.wrapping_add(16), var16, arg3);
                            let mut slot_var5_16_i32 = self.memory.load32(var5 as usize + 16) as i32;
                            if (slot_var5_16_i32 & 1 == 0) as i32 != 0 {
                                break 'label4;
                            }
                            let mut slot_var5_20_i32 = self.memory.load32(var5 as usize + 20) as i32;
                            arg4 = slot_var5_20_i32;
                            var6 = arg4.wrapping_add(arg2);
                            if ((var6 as u32) < arg4 as u32) as i32 != 0 {
                                break 'label4;
                            }
                            'label8: loop {
                                'label9: loop {
                                    let var18 = self.memory.load32(0 as usize + 1053992) as i32;
                                    if (var6 as u32 <= var18 as u32) as i32 != 0 {
                                        break 'label9;
                                    }
                                    let var19 = self.func116(env, arg2, arg3);
                                    arg4 = var19;
                                    break 'label8;
                                    break;
                                }
                                self.memory.store32(0 as usize + 1053988, var6 as u32);
                                break;
                            }
                            if (arg4 == 0) as i32 != 0 {
                                break 'label5;
                            }
                            let var20 = self.func121(env, arg4, var7, var10);
                            var20;
                            break 'label5;
                            break;
                        }
                        self.func117(env, var5, arg3, arg2);
                        let mut slot_var5_0_i32 = self.memory.load32(var5 as usize) as i32;
                        arg4 = slot_var5_0_i32;
                        break;
                    }
                    var6 = arg3;
                    if (arg4 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    self.memory.store32(arg1 as usize, var8 as u32);
                    self.memory.store32(arg1 as usize + 4, arg4 as u32);
                    var6 = -2147483647;
                    break 'label1;
                    break;
                }
                unreachable!();
                break;
            }
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg2 as u32);
        self.memory.store32(arg0 as usize, var6 as u32);
        self.global0 = var5.wrapping_add(32);
    }
    fn func59(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i64 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var2 = var17.wrapping_sub(1344);
        self.global0 = var2;
        let var18 = self.func121(env, var2, arg1, 384);
        var3 = var18;
        'label0: loop {
            let var19 = self.func28(env, arg1);
            if var19 != 0 {
                break 'label0;
            }
            var4 = 0;
            self.func60(env, var3.wrapping_add(768), 0);
            let mut slot_var3_772_i32 = self.memory.load32(var3 as usize + 772) as i32;
            var5 = slot_var3_772_i32;
            'label1: loop {
                let mut slot_var3_768_i32 = self.memory.load32(var3 as usize + 768) as i32;
                if (slot_var3_768_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var3_776_i32 = self.memory.load32(var3 as usize + 776) as i32;
                var6 = slot_var3_776_i32;
                let mut slot_var6_0_i64 = 4965661367192848881 as i64;
                let mut slot_var3_392_i32 = 0 as i32;
                let mut slot_var3_384_i64 = 4294967296 as i64;
                var7 = 1;
                'label2: loop {
                    var2 = 0;
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                if (var2 == 8) as i32 != 0 {
                                    break 'label4;
                                }
                                var8 = var6.wrapping_add(var2);
                                var2 = var2.wrapping_add(8);
                                let mut slot_var8_0_i64 = self.memory.load64(var8 as usize) as i64;
                                if (slot_var8_0_i64 == 0) as i32 != 0 {
                                    continue 'label5;
                                }
                                break;
                            }
                            var9 = 0;
                            var10 = slot_var6_0_i64;
                            if (var10 & 1 /* True */ == 0) as i32 != 0 {
                                break 'label3;
                            }
                            'label6: loop {
                                var2 = var10 as i32 & 3;
                                if (var2 != 3) as i32 != 0 {
                                    break 'label6;
                                }
                                self.func61(env, var3.wrapping_add(768));
                                var11 = slot_var3_772_i32;
                                var2 = slot_var3_776_i32;
                                let var22 = self.func62(env, var11, var2);
                                var9 = var22;
                                let mut slot_var9_0_i64 = 1 /* True */ as i64;
                                var12 = var2.wrapping_shl(3 as u32);
                                var10 = 0 /* False */;
                                var2 = 0;
                                var13 = slot_var3_768_i32;
                                'label7: loop {
                                    'label8: loop {
                                        if (var2 == 8) as i32 != 0 {
                                            break 'label7;
                                        }
                                        if (var12 == var2) as i32 != 0 {
                                            break 'label7;
                                        }
                                        var8 = var6.wrapping_add(var2);
                                        let var23 = self.memory.load64(var11.wrapping_add(var2) as usize) as i64;
                                        var14 = var23;
                                        var10 = var14.wrapping_add(var10);
                                        var15 = var10.wrapping_add(slot_var8_0_i64);
                                        slot_var8_0_i64 = var15 as i64;
                                        var10 = (((var10 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var15 as u64) < var10 as u64) as i32 as u32 as i64);
                                        var2 = var2.wrapping_add(8);
                                        continue 'label8;
                                        break;
                                    }
                                    break;
                                }
                                self.func63(env, var9, var13);
                                var9 = 255;
                                break 'label3;
                                break;
                            }
                            self.func61(env, var3.wrapping_add(768));
                            var11 = slot_var3_772_i32;
                            var8 = slot_var3_776_i32;
                            let var26 = self.func62(env, var11, var8);
                            var13 = var26;
                            var9 = 2.wrapping_sub(var2);
                            let mut slot_var13_0_i64 = var9 as i64 as i64;
                            var12 = var8.wrapping_shl(3 as u32);
                            var10 = 0 /* False */;
                            var2 = 0;
                            var16 = slot_var3_768_i32;
                            'label9: loop {
                                'label10: loop {
                                    if (var2 == 8) as i32 != 0 {
                                        break 'label9;
                                    }
                                    if (var12 == var2) as i32 != 0 {
                                        break 'label9;
                                    }
                                    var8 = var6.wrapping_add(var2);
                                    var14 = slot_var8_0_i64;
                                    let var27 = self.memory.load64(var11.wrapping_add(var2) as usize) as i64;
                                    var15 = var27;
                                    var10 = var15.wrapping_add(var10);
                                    slot_var8_0_i64 = var14.wrapping_sub(var10) as i64;
                                    var10 = ((((var10 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var10 as u64) as i32 as u32 as i64) == 1 /* True */) as i32 as u32 as i64;
                                    var2 = var2.wrapping_add(8);
                                    continue 'label10;
                                    break;
                                }
                                break;
                            }
                            self.func63(env, var13, var16);
                            break 'label3;
                            break;
                        }
                        let mut slot_var3_388_i32 = self.memory.load32(var3 as usize + 388) as i32;
                        var8 = slot_var3_388_i32;
                        var11 = slot_var3_384_i64;
                        self.func64(env, var5, var6);
                        let var30 = self.func121(env, var3.wrapping_add(768), var3, 384);
                        var30;
                        'label11: loop {
                            let var31 = self.func65(env, var3.wrapping_add(768));
                            var2 = var31;
                            if (var2 == 0) as i32 != 0 {
                                break 'label11;
                            }
                            let var32 = self.func121(env, var3.wrapping_add(384), var2, 384);
                            var32;
                            let var33 = self.func121(env, var3.wrapping_add(1152), 1048960, 64);
                            var33;
                            let var34 = self.func122(env, var3.wrapping_add(1152).wrapping_add(64), 0, 128);
                            var34;
                            let var35 = self.func122(env, var3.wrapping_add(768).wrapping_add(192), 0, 192);
                            var35;
                            let var36 = self.func121(env, var3.wrapping_add(768), var3.wrapping_add(1152), 192);
                            var36;
                            var2 = var8.wrapping_add(var4);
                            var6 = 0;
                            'label12: loop {
                                var12 = var6 & 1;
                                'label13: loop {
                                    'label14: loop {
                                        'label15: loop {
                                            'label16: loop {
                                                if (var2 == var8) as i32 != 0 {
                                                    break 'label16;
                                                }
                                                var2 = var2.wrapping_add(-1);
                                                let var37 = self.memory.load8(var2 as usize) as i8 as i32;
                                                var6 = var37;
                                                if var12 != 0 {
                                                    break 'label15;
                                                }
                                                break 'label14;
                                                break;
                                            }
                                            self.func47(env, var11, var8, 1, 1);
                                            let var39 = self.func121(env, var3, var3.wrapping_add(768), 384);
                                            var39;
                                            break 'label0;
                                            break;
                                        }
                                        let var40 = self.func66(env, var3.wrapping_add(768));
                                        var40;
                                        break;
                                    }
                                    if (var6 == 0) as i32 != 0 {
                                        continue 'label13;
                                    }
                                    break;
                                }
                                self.func42(env, var3.wrapping_add(768), { let a = var3; let b = var3.wrapping_add(384); if (var6 > 0) as i32 != 0 { a } else { b } });
                                var6 = 1;
                                continue 'label12;
                                break;
                            }
                            break;
                        }
                        self.func37(env);
                        unreachable!();
                        break;
                    }
                    'label17: loop {
                        if (var4 != slot_var3_384_i64) as i32 != 0 {
                            break 'label17;
                        }
                        self.func67(env, var3.wrapping_add(384));
                        var7 = slot_var3_388_i32;
                        break;
                    }
                    self.memory.store8(var7.wrapping_add(var4) as usize, var9 as u8);
                    var4 = var4.wrapping_add(1);
                    slot_var3_392_i32 = var4 as i32;
                    var10 = 0 /* False */;
                    var2 = 0;
                    'label18: loop {
                        if (var2 == -8) as i32 != 0 {
                            continue 'label2;
                        }
                        var8 = var6.wrapping_add(var2);
                        var14 = slot_var8_0_i64;
                        slot_var8_0_i64 = ((var14 as u64).wrapping_shr(1 /* True */ as u32) as i64 | var10) as i64;
                        var2 = var2.wrapping_add(-8);
                        var10 = var14.wrapping_shl(63 as u32);
                        continue 'label18;
                        break;
                    }
                    break;
                }
                break;
            }
            self.func56(env, var5, slot_var3_776_i32);
            unreachable!();
            break;
        }
        let var45 = self.func121(env, var3.wrapping_add(768), var3, 384);
        var45;
        let var46 = self.func121(env, arg1, var3.wrapping_add(768), 384);
        var2 = var46;
        let var47 = self.func65(env, var2);
        var47;
        let var48 = self.func121(env, arg0, var2, 384);
        var48;
        self.global0 = var3.wrapping_add(1344);
    }
    fn func60(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (arg1 == 0) as i32 != 0 {
                    break 'label1;
                }
                self.func113(env, var2, 8, 8, 1);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                arg1 = slot_var2_0_i32;
                break 'label0;
                break;
            }
            self.func117(env, var2.wrapping_add(8), 8, 8);
            let mut slot_var2_8_i32 = self.memory.load32(var2 as usize + 8) as i32;
            arg1 = slot_var2_8_i32;
            break;
        }
        self.memory.store32(arg0 as usize + 8, ({ let a = arg1; let b = 8; if arg1 != 0 { a } else { b } }) as u32);
        self.memory.store32(arg0 as usize + 4, ({ let a = 1; let b = 8; if arg1 != 0 { a } else { b } }) as u32);
        self.memory.store32(arg0 as usize, (arg1 == 0) as i32 as u32);
        self.global0 = var2.wrapping_add(16);
    }
    fn func61(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16);
        self.global0 = var1;
        self.func60(env, var1.wrapping_add(4), 1);
        let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
        var2 = slot_var1_8_i32;
        'label0: loop {
            let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
            if (slot_var1_4_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
            self.func56(env, var2, slot_var1_12_i32);
            unreachable!();
            break;
        }
        var3 = slot_var1_12_i32;
        self.memory.store32(arg0 as usize + 8, 1 as u32);
        self.memory.store32(arg0 as usize + 4, var3 as u32);
        self.memory.store32(arg0 as usize, var2 as u32);
        self.global0 = var1.wrapping_add(16);
    }
    fn func62(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        'label0: loop {
            if arg1 != 0 {
                break 'label0;
            }
            self.func91(env, 0, 0);
            unreachable!();
            break;
        }
        arg0
    }
    fn func63(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func64(env, arg1, arg0);
    }
    fn func64(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func47(env, arg0, arg1, 8, 8);
    }
    fn func65(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(384);
        self.global0 = var1;
        var2 = 0;
        'label0: loop {
            let var4 = self.func28(env, arg0);
            if var4 != 0 {
                break 'label0;
            }
            var2 = arg0.wrapping_add(192);
            let var5 = self.func121(env, var1.wrapping_add(192), var2, 192);
            var5;
            self.func14(env, var1, var1.wrapping_add(192));
            let var7 = self.func121(env, var2, var1, 192);
            var7;
            var2 = arg0;
            break;
        }
        self.global0 = var1.wrapping_add(384);
        var2
    }
    fn func66(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let mut var20: i32 = 0;
        let mut var21: i32 = 0;
        let mut var22: i32 = 0;
        let mut var23: i32 = 0;
        let mut var24: i32 = 0;
        let mut var25: i32 = 0;
        let mut var26: i32 = 0;
        let mut var27: i32 = 0;
        let mut var28: i32 = 0;
        let var29 = self.global0;
        var1 = var29.wrapping_sub(944);
        self.global0 = var1;
        var2 = 0 /* False */;
        var3 = 0;
        var4 = 1051384;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    var5 = 3 /* Error(Contract, #0) */;
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match var3 {
                                    0 => break 'label3,
                                    1 => break 'label4,
                                    2 => break 'label4,
                                    3 => break 'label4,
                                    4 => break 'label5,
                                    _ => break 'label4,
                                }
                                break;
                            }
                            self.func119(env, var1, var2, 0 /* False */, var2, 0 /* False */);
                            let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                            if (slot_var1_8_i64 == 0) as i32 != 0 {
                                break 'label1;
                            }
                            break 'label0;
                            break;
                        }
                        let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
                        var5 = (((slot_var4_0_i64 as u64).wrapping_rem(0 as u64) as i64).wrapping_shl(0 /* Void */ as u32) as u64).wrapping_rem(0 as u64) as i64;
                        break;
                    }
                    var4 = var4.wrapping_add(8);
                    var3 = var3.wrapping_add(1);
                    var5 = var2.wrapping_add(var5);
                    var6 = ((var5 as u64) < var2 as u64) as i32;
                    var2 = var5;
                    if (var6 == 0) as i32 != 0 {
                        continue 'label2;
                    }
                    break 'label0;
                    break;
                }
                break;
            }
            'label6: loop {
                'label7: loop {
                    let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                    if (((slot_var1_0_i64 as u64).wrapping_rem(0 as u64) as i64) == 1 /* True */) as i32 != 0 {
                        break 'label7;
                    }
                    let var31 = self.func18(env, arg0);
                    var31;
                    break 'label6;
                    break;
                }
                let var32 = self.func121(env, var1.wrapping_add(880), arg0, 64);
                var32;
                var7 = arg0.wrapping_add(256);
                self.func5(env, var1.wrapping_add(880), var7);
                let var34 = self.func121(env, var1.wrapping_add(16), var1.wrapping_add(880), 64);
                var34;
                let var35 = self.func121(env, var1.wrapping_add(880), arg0, 64);
                var35;
                self.func10(env, var1.wrapping_add(880), var7);
                let var37 = self.func121(env, var1.wrapping_add(80), var1.wrapping_add(880), 64);
                var37;
                let var38 = self.func121(env, var1.wrapping_add(816), var7, 64);
                var38;
                let var39 = self.func11(env, var1.wrapping_add(816));
                let var40 = self.func11(env, var39);
                let var41 = self.func11(env, var40);
                var8 = var41;
                var9 = var1.wrapping_add(400).wrapping_add(24);
                let var42 = self.memory.load64(arg0.wrapping_add(312) as usize) as i64;
                let mut slot_var9_0_i64 = var42 as i64;
                var10 = var1.wrapping_add(400).wrapping_add(16);
                let var43 = self.memory.load64(arg0.wrapping_add(304) as usize) as i64;
                let mut slot_var10_0_i64 = var43 as i64;
                var11 = var1.wrapping_add(400).wrapping_add(8);
                let var44 = self.memory.load64(arg0.wrapping_add(296) as usize) as i64;
                let mut slot_var11_0_i64 = var44 as i64;
                let var45 = self.memory.load64(arg0 as usize + 288) as i64;
                let mut slot_var1_400_i64 = var45 as i64;
                self.func12(env, var1.wrapping_add(400));
                self.func13(env, var1.wrapping_add(400), var8);
                self.func13(env, var1.wrapping_add(400), var7);
                var3 = var1.wrapping_add(880).wrapping_add(24);
                let var49 = self.memory.load64(var8.wrapping_add(56) as usize) as i64;
                let mut slot_var3_0_i64 = var49 as i64;
                var4 = var1.wrapping_add(880).wrapping_add(16);
                let var50 = self.memory.load64(var8.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var50 as i64;
                var6 = var1.wrapping_add(880).wrapping_add(8);
                let var51 = self.memory.load64(var8.wrapping_add(40) as usize) as i64;
                let mut slot_var6_0_i64 = var51 as i64;
                let mut slot_var8_32_i64 = self.memory.load64(var8 as usize + 32) as i64;
                let mut slot_var1_880_i64 = slot_var8_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), arg0.wrapping_add(288));
                var8 = var1.wrapping_add(464).wrapping_add(24);
                let mut slot_var8_0_i64 = slot_var3_0_i64 as i64;
                var12 = var1.wrapping_add(464).wrapping_add(16);
                let mut slot_var12_0_i64 = slot_var4_0_i64 as i64;
                var13 = var1.wrapping_add(464).wrapping_add(8);
                let mut slot_var13_0_i64 = slot_var6_0_i64 as i64;
                let mut slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var7);
                var14 = var1.wrapping_add(880).wrapping_add(56);
                let mut slot_var14_0_i64 = slot_var8_0_i64 as i64;
                var15 = var1.wrapping_add(880).wrapping_add(48);
                let mut slot_var15_0_i64 = slot_var12_0_i64 as i64;
                var16 = var1.wrapping_add(880).wrapping_add(40);
                let mut slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var11_0_i64 as i64;
                slot_var4_0_i64 = slot_var10_0_i64 as i64;
                slot_var3_0_i64 = slot_var9_0_i64 as i64;
                let mut slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_400_i64 as i64;
                let var54 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var54;
                let var55 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var55;
                self.func10(env, var1.wrapping_add(880), arg0);
                let var57 = self.func121(env, var1.wrapping_add(400), var1.wrapping_add(880), 64);
                var57;
                self.func5(env, var1.wrapping_add(80), var1.wrapping_add(400));
                self.func9(env, var1.wrapping_add(80), var1.wrapping_add(16));
                let var60 = self.func121(env, var1.wrapping_add(816), var1.wrapping_add(16), 64);
                var60;
                let var61 = self.func11(env, var1.wrapping_add(816));
                let var62 = self.func11(env, var61);
                let var63 = self.func11(env, var62);
                var17 = var63;
                var18 = var1.wrapping_add(144).wrapping_add(24);
                var19 = var1.wrapping_add(16).wrapping_add(56);
                let mut slot_var19_0_i64 = self.memory.load64(var19 as usize) as i64;
                let mut slot_var18_0_i64 = slot_var19_0_i64 as i64;
                var20 = var1.wrapping_add(144).wrapping_add(16);
                var21 = var1.wrapping_add(16).wrapping_add(48);
                let mut slot_var21_0_i64 = self.memory.load64(var21 as usize) as i64;
                let mut slot_var20_0_i64 = slot_var21_0_i64 as i64;
                var22 = var1.wrapping_add(144).wrapping_add(8);
                var23 = var1.wrapping_add(16).wrapping_add(40);
                let mut slot_var23_0_i64 = self.memory.load64(var23 as usize) as i64;
                let mut slot_var22_0_i64 = slot_var23_0_i64 as i64;
                let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                let mut slot_var1_144_i64 = slot_var1_48_i64 as i64;
                self.func12(env, var1.wrapping_add(144));
                self.func13(env, var1.wrapping_add(144), var17);
                self.func13(env, var1.wrapping_add(144), var1.wrapping_add(16));
                let var67 = self.memory.load64(var17.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var67 as i64;
                let var68 = self.memory.load64(var17.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var68 as i64;
                let var69 = self.memory.load64(var17.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var69 as i64;
                let mut slot_var17_32_i64 = self.memory.load64(var17 as usize + 32) as i64;
                slot_var1_880_i64 = slot_var17_32_i64 as i64;
                var24 = var1.wrapping_add(16).wrapping_add(32);
                self.func13(env, var1.wrapping_add(880), var24);
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var1.wrapping_add(16));
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var22_0_i64 as i64;
                slot_var4_0_i64 = slot_var20_0_i64 as i64;
                slot_var3_0_i64 = slot_var18_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_144_i64 as i64;
                let var72 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var72;
                let var73 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var73;
                self.func9(env, var1.wrapping_add(80), var1.wrapping_add(880));
                self.func8(env, var1.wrapping_add(144), var1.wrapping_add(16));
                var22 = arg0.wrapping_add(192);
                let var76 = self.func121(env, var1.wrapping_add(880), var22, 64);
                var76;
                var17 = arg0.wrapping_add(128);
                self.func5(env, var1.wrapping_add(880), var17);
                let var78 = self.func121(env, var1.wrapping_add(16), var1.wrapping_add(880), 64);
                var78;
                let var79 = self.func121(env, var1.wrapping_add(880), var22, 64);
                var79;
                self.func10(env, var1.wrapping_add(880), var17);
                let var81 = self.func121(env, var1.wrapping_add(208), var1.wrapping_add(880), 64);
                var81;
                let var82 = self.func121(env, var1.wrapping_add(816), var17, 64);
                var82;
                let var83 = self.func11(env, var1.wrapping_add(816));
                let var84 = self.func11(env, var83);
                let var85 = self.func11(env, var84);
                var18 = var85;
                let var86 = self.memory.load64(arg0.wrapping_add(184) as usize) as i64;
                slot_var9_0_i64 = var86 as i64;
                let var87 = self.memory.load64(arg0.wrapping_add(176) as usize) as i64;
                slot_var10_0_i64 = var87 as i64;
                let var88 = self.memory.load64(arg0.wrapping_add(168) as usize) as i64;
                slot_var11_0_i64 = var88 as i64;
                let var89 = self.memory.load64(arg0 as usize + 160) as i64;
                slot_var1_400_i64 = var89 as i64;
                self.func12(env, var1.wrapping_add(400));
                self.func13(env, var1.wrapping_add(400), var18);
                self.func13(env, var1.wrapping_add(400), var17);
                let var93 = self.memory.load64(var18.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var93 as i64;
                let var94 = self.memory.load64(var18.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var94 as i64;
                let var95 = self.memory.load64(var18.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var95 as i64;
                let mut slot_var18_32_i64 = self.memory.load64(var18 as usize + 32) as i64;
                slot_var1_880_i64 = slot_var18_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), arg0.wrapping_add(160));
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var17);
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var11_0_i64 as i64;
                slot_var4_0_i64 = slot_var10_0_i64 as i64;
                slot_var3_0_i64 = slot_var9_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_400_i64 as i64;
                let var98 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var98;
                let var99 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var99;
                self.func10(env, var1.wrapping_add(880), var22);
                let var101 = self.func121(env, var1.wrapping_add(400), var1.wrapping_add(880), 64);
                var101;
                self.func5(env, var1.wrapping_add(208), var1.wrapping_add(400));
                self.func9(env, var1.wrapping_add(208), var1.wrapping_add(16));
                let var104 = self.func121(env, var1.wrapping_add(816), var1.wrapping_add(16), 64);
                var104;
                let var105 = self.func11(env, var1.wrapping_add(816));
                let var106 = self.func11(env, var105);
                let var107 = self.func11(env, var106);
                var18 = var107;
                var25 = var1.wrapping_add(784).wrapping_add(24);
                let mut slot_var25_0_i64 = slot_var19_0_i64 as i64;
                var26 = var1.wrapping_add(784).wrapping_add(16);
                let mut slot_var26_0_i64 = slot_var21_0_i64 as i64;
                var27 = var1.wrapping_add(784).wrapping_add(8);
                let mut slot_var27_0_i64 = slot_var23_0_i64 as i64;
                let mut slot_var1_784_i64 = slot_var1_48_i64 as i64;
                self.func12(env, var1.wrapping_add(784));
                self.func13(env, var1.wrapping_add(784), var18);
                self.func13(env, var1.wrapping_add(784), var1.wrapping_add(16));
                let var111 = self.memory.load64(var18.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var111 as i64;
                let var112 = self.memory.load64(var18.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var112 as i64;
                let var113 = self.memory.load64(var18.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var113 as i64;
                slot_var1_880_i64 = slot_var18_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), var24);
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var1.wrapping_add(16));
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var27_0_i64 as i64;
                slot_var4_0_i64 = slot_var26_0_i64 as i64;
                slot_var3_0_i64 = slot_var25_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_784_i64 as i64;
                let var116 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var116;
                let var117 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var117;
                self.func9(env, var1.wrapping_add(208), var1.wrapping_add(880));
                self.func8(env, var1.wrapping_add(272), var1.wrapping_add(16));
                var28 = arg0.wrapping_add(64);
                let var120 = self.func121(env, var1.wrapping_add(880), var28, 64);
                var120;
                var18 = arg0.wrapping_add(320);
                self.func5(env, var1.wrapping_add(880), var18);
                let var122 = self.func121(env, var1.wrapping_add(16), var1.wrapping_add(880), 64);
                var122;
                let var123 = self.func121(env, var1.wrapping_add(880), var28, 64);
                var123;
                self.func10(env, var1.wrapping_add(880), var18);
                let var125 = self.func121(env, var1.wrapping_add(336), var1.wrapping_add(880), 64);
                var125;
                let var126 = self.func121(env, var1.wrapping_add(816), var18, 64);
                var126;
                let var127 = self.func11(env, var1.wrapping_add(816));
                let var128 = self.func11(env, var127);
                let var129 = self.func11(env, var128);
                var20 = var129;
                let var130 = self.memory.load64(arg0.wrapping_add(376) as usize) as i64;
                slot_var9_0_i64 = var130 as i64;
                let var131 = self.memory.load64(arg0.wrapping_add(368) as usize) as i64;
                slot_var10_0_i64 = var131 as i64;
                let var132 = self.memory.load64(arg0.wrapping_add(360) as usize) as i64;
                slot_var11_0_i64 = var132 as i64;
                let var133 = self.memory.load64(arg0 as usize + 352) as i64;
                slot_var1_400_i64 = var133 as i64;
                self.func12(env, var1.wrapping_add(400));
                self.func13(env, var1.wrapping_add(400), var20);
                self.func13(env, var1.wrapping_add(400), var18);
                let var137 = self.memory.load64(var20.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var137 as i64;
                let var138 = self.memory.load64(var20.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var138 as i64;
                let var139 = self.memory.load64(var20.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var139 as i64;
                let mut slot_var20_32_i64 = self.memory.load64(var20 as usize + 32) as i64;
                slot_var1_880_i64 = slot_var20_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), arg0.wrapping_add(352));
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var18);
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var11_0_i64 as i64;
                slot_var4_0_i64 = slot_var10_0_i64 as i64;
                slot_var3_0_i64 = slot_var9_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_400_i64 as i64;
                let var142 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var142;
                let var143 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var143;
                self.func10(env, var1.wrapping_add(880), var28);
                let var145 = self.func121(env, var1.wrapping_add(400), var1.wrapping_add(880), 64);
                var145;
                self.func5(env, var1.wrapping_add(336), var1.wrapping_add(400));
                self.func9(env, var1.wrapping_add(336), var1.wrapping_add(16));
                let var148 = self.func121(env, var1.wrapping_add(816), var1.wrapping_add(16), 64);
                var148;
                let var149 = self.func11(env, var1.wrapping_add(816));
                let var150 = self.func11(env, var149);
                let var151 = self.func11(env, var150);
                var20 = var151;
                slot_var25_0_i64 = slot_var19_0_i64 as i64;
                slot_var26_0_i64 = slot_var21_0_i64 as i64;
                slot_var27_0_i64 = slot_var23_0_i64 as i64;
                slot_var1_784_i64 = slot_var1_48_i64 as i64;
                self.func12(env, var1.wrapping_add(784));
                self.func13(env, var1.wrapping_add(784), var20);
                self.func13(env, var1.wrapping_add(784), var1.wrapping_add(16));
                let var155 = self.memory.load64(var20.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var155 as i64;
                let var156 = self.memory.load64(var20.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var156 as i64;
                let var157 = self.memory.load64(var20.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var157 as i64;
                slot_var1_880_i64 = slot_var20_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), var24);
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var1.wrapping_add(16));
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var27_0_i64 as i64;
                slot_var4_0_i64 = slot_var26_0_i64 as i64;
                slot_var3_0_i64 = slot_var25_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_784_i64 as i64;
                let var160 = self.func121(env, var1.wrapping_add(464), var1.wrapping_add(880), 64);
                var160;
                let var161 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(464), 64);
                var161;
                self.func9(env, var1.wrapping_add(336), var1.wrapping_add(880));
                self.func8(env, var1.wrapping_add(528), var1.wrapping_add(16));
                let var164 = self.func121(env, var1.wrapping_add(592), var1.wrapping_add(80), 64);
                var164;
                self.func9(env, var1.wrapping_add(592), arg0);
                let var166 = self.func121(env, arg0, var1.wrapping_add(592), 64);
                let var167 = self.func11(env, var166);
                self.func10(env, var167, var1.wrapping_add(80));
                let var169 = self.func121(env, var1.wrapping_add(880), var1.wrapping_add(144), 64);
                var169;
                self.func10(env, var1.wrapping_add(880), var7);
                let var171 = self.func121(env, var1.wrapping_add(816), var1.wrapping_add(880), 64);
                var171;
                let var172 = self.func121(env, var7, var1.wrapping_add(816), 64);
                let var173 = self.func11(env, var172);
                self.func10(env, var173, var1.wrapping_add(144));
                let var175 = self.func121(env, var1.wrapping_add(816), var1.wrapping_add(528), 64);
                var175;
                let var176 = self.func11(env, var1.wrapping_add(816));
                let var177 = self.func11(env, var176);
                let var178 = self.func11(env, var177);
                var7 = var178;
                let var179 = self.memory.load64(var1.wrapping_add(528).wrapping_add(56) as usize) as i64;
                slot_var9_0_i64 = var179 as i64;
                let var180 = self.memory.load64(var1.wrapping_add(528).wrapping_add(48) as usize) as i64;
                slot_var10_0_i64 = var180 as i64;
                let var181 = self.memory.load64(var1.wrapping_add(528).wrapping_add(40) as usize) as i64;
                slot_var11_0_i64 = var181 as i64;
                let mut slot_var1_560_i64 = self.memory.load64(var1 as usize + 560) as i64;
                slot_var1_400_i64 = slot_var1_560_i64 as i64;
                self.func12(env, var1.wrapping_add(400));
                self.func13(env, var1.wrapping_add(400), var7);
                self.func13(env, var1.wrapping_add(400), var1.wrapping_add(528));
                let var185 = self.memory.load64(var7.wrapping_add(56) as usize) as i64;
                slot_var3_0_i64 = var185 as i64;
                let var186 = self.memory.load64(var7.wrapping_add(48) as usize) as i64;
                slot_var4_0_i64 = var186 as i64;
                let var187 = self.memory.load64(var7.wrapping_add(40) as usize) as i64;
                slot_var6_0_i64 = var187 as i64;
                let mut slot_var7_32_i64 = self.memory.load64(var7 as usize + 32) as i64;
                slot_var1_880_i64 = slot_var7_32_i64 as i64;
                self.func13(env, var1.wrapping_add(880), var1.wrapping_add(528).wrapping_add(32));
                slot_var8_0_i64 = slot_var3_0_i64 as i64;
                slot_var12_0_i64 = slot_var4_0_i64 as i64;
                slot_var13_0_i64 = slot_var6_0_i64 as i64;
                slot_var1_464_i64 = slot_var1_880_i64 as i64;
                self.func13(env, var1.wrapping_add(464), var1.wrapping_add(528));
                slot_var14_0_i64 = slot_var8_0_i64 as i64;
                slot_var15_0_i64 = slot_var12_0_i64 as i64;
                slot_var16_0_i64 = slot_var13_0_i64 as i64;
                slot_var6_0_i64 = slot_var11_0_i64 as i64;
                slot_var4_0_i64 = slot_var10_0_i64 as i64;
                slot_var3_0_i64 = slot_var9_0_i64 as i64;
                slot_var1_912_i64 = slot_var1_464_i64 as i64;
                slot_var1_880_i64 = slot_var1_400_i64 as i64;
                let var190 = self.func121(env, var1.wrapping_add(528), var1.wrapping_add(880), 64);
                var190;
                self.func10(env, var22, var1.wrapping_add(528));
                let var192 = self.func11(env, var22);
                self.func10(env, var192, var1.wrapping_add(528));
                let var194 = self.func121(env, var1.wrapping_add(656), var1.wrapping_add(336), 64);
                var194;
                self.func9(env, var1.wrapping_add(656), var17);
                let var196 = self.func121(env, var17, var1.wrapping_add(656), 64);
                let var197 = self.func11(env, var196);
                self.func10(env, var197, var1.wrapping_add(336));
                let var199 = self.func121(env, var1.wrapping_add(720), var1.wrapping_add(208), 64);
                var199;
                self.func9(env, var1.wrapping_add(720), var28);
                let var201 = self.func121(env, var28, var1.wrapping_add(720), 64);
                let var202 = self.func11(env, var201);
                self.func10(env, var202, var1.wrapping_add(208));
                self.func10(env, var18, var1.wrapping_add(272));
                let var205 = self.func11(env, var18);
                self.func10(env, var205, var1.wrapping_add(272));
                break;
            }
            self.global0 = var1.wrapping_add(944);
            return arg0;
            break;
        }
        self.func73(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func67(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        let var3 = self.memory.load32(arg0 as usize) as i32;
        self.func58(env, var1.wrapping_add(8), arg0, var3, 1, 1);
        'label0: loop {
            let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
            arg0 = slot_var1_8_i32;
            if (arg0 == -2147483647) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
            self.func56(env, arg0, slot_var1_12_i32);
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(16);
    }
    fn func68(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
        let var13 = self.global0;
        var3 = var13.wrapping_sub(1216);
        self.global0 = var3;
        let var14 = self.func121(env, var3, arg1, 64);
        var3 = var14;
        let var15 = self.func121(env, var3.wrapping_add(64), arg1.wrapping_add(64), 64);
        var15;
        self.func34(env, var3, arg2.wrapping_add(32));
        self.func34(env, var3.wrapping_add(64), arg2);
        let var18 = self.func121(env, var3.wrapping_add(960), arg0, 64);
        var18;
        self.func5(env, var3.wrapping_add(960), var3);
        let var20 = self.func121(env, var3.wrapping_add(128), var3.wrapping_add(960), 64);
        var20;
        var4 = arg0.wrapping_add(64);
        let var21 = self.func121(env, var3.wrapping_add(960), var4, 64);
        var21;
        self.func5(env, var3.wrapping_add(960), var3);
        let var23 = self.func121(env, var3.wrapping_add(128).wrapping_add(64), var3.wrapping_add(960), 64);
        var23;
        var5 = arg0.wrapping_add(128);
        let var24 = self.func121(env, var3.wrapping_add(960), var5, 64);
        var24;
        self.func5(env, var3.wrapping_add(960), var3);
        let var26 = self.func121(env, var3.wrapping_add(128).wrapping_add(128), var3.wrapping_add(960), 64);
        var26;
        arg2 = arg0.wrapping_add(192);
        let var27 = self.func121(env, var3.wrapping_add(320), arg2, 192);
        var27;
        arg1 = arg1.wrapping_add(128);
        self.func69(env, var3.wrapping_add(320), var3.wrapping_add(64), arg1);
        let var29 = self.func121(env, var3.wrapping_add(960), var3, 64);
        var29;
        self.func10(env, var3.wrapping_add(960), var3.wrapping_add(64));
        let var31 = self.func121(env, var3.wrapping_add(512), var3.wrapping_add(960), 64);
        var31;
        let var32 = self.func121(env, var3.wrapping_add(960), arg0, 192);
        var32;
        self.func23(env, var3.wrapping_add(960), arg2);
        let var34 = self.func121(env, var3.wrapping_add(576), var3.wrapping_add(960), 192);
        var34;
        self.func69(env, var3.wrapping_add(576), var3.wrapping_add(512), arg1);
        let var36 = self.func121(env, var3.wrapping_add(960), var3.wrapping_add(128), 192);
        var36;
        self.func23(env, var3.wrapping_add(960), var3.wrapping_add(320));
        let var38 = self.func121(env, var3.wrapping_add(768), var3.wrapping_add(960), 192);
        var38;
        self.func20(env, var3.wrapping_add(576), var3.wrapping_add(768));
        let var40 = self.func121(env, arg2, var3.wrapping_add(576), 192);
        var40;
        let var41 = self.func121(env, arg0, var3.wrapping_add(320), 192);
        arg0 = var41;
        let var42 = self.func121(env, var4, var3.wrapping_add(320), 64);
        var42;
        arg1 = var3.wrapping_add(320).wrapping_add(128);
        let var43 = self.func121(env, arg0, arg1, 64);
        arg2 = var43;
        let var44 = self.func121(env, var3.wrapping_add(768), arg1, 64);
        var44;
        let var45 = self.func11(env, var3.wrapping_add(768));
        let var46 = self.func11(env, var45);
        let var47 = self.func11(env, var46);
        arg0 = var47;
        var6 = var3.wrapping_add(1152).wrapping_add(24);
        let var48 = self.memory.load64(var3.wrapping_add(504) as usize) as i64;
        let mut slot_var6_0_i64 = var48 as i64;
        var7 = var3.wrapping_add(1152).wrapping_add(16);
        let var49 = self.memory.load64(var3.wrapping_add(496) as usize) as i64;
        let mut slot_var7_0_i64 = var49 as i64;
        var8 = var3.wrapping_add(1152).wrapping_add(8);
        let var50 = self.memory.load64(var3.wrapping_add(488) as usize) as i64;
        let mut slot_var8_0_i64 = var50 as i64;
        let mut slot_var3_480_i64 = self.memory.load64(var3 as usize + 480) as i64;
        let mut slot_var3_1152_i64 = slot_var3_480_i64 as i64;
        self.func12(env, var3.wrapping_add(1152));
        self.func13(env, var3.wrapping_add(1152), arg0);
        self.func13(env, var3.wrapping_add(1152), arg2);
        var4 = var3.wrapping_add(960).wrapping_add(24);
        let var54 = self.memory.load64(arg0.wrapping_add(56) as usize) as i64;
        let mut slot_var4_0_i64 = var54 as i64;
        var9 = var3.wrapping_add(960).wrapping_add(16);
        let var55 = self.memory.load64(arg0.wrapping_add(48) as usize) as i64;
        let mut slot_var9_0_i64 = var55 as i64;
        var10 = var3.wrapping_add(960).wrapping_add(8);
        let var56 = self.memory.load64(arg0.wrapping_add(40) as usize) as i64;
        let mut slot_var10_0_i64 = var56 as i64;
        let var57 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var3_960_i64 = var57 as i64;
        self.func13(env, var3.wrapping_add(960), var3.wrapping_add(480));
        arg0 = var3.wrapping_add(1184).wrapping_add(24);
        self.memory.store64(arg0 as usize, slot_var4_0_i64 as u64);
        var11 = var3.wrapping_add(1184).wrapping_add(16);
        let mut slot_var11_0_i64 = slot_var9_0_i64 as i64;
        var12 = var3.wrapping_add(1184).wrapping_add(8);
        let mut slot_var12_0_i64 = slot_var10_0_i64 as i64;
        let mut slot_var3_1184_i64 = slot_var3_960_i64 as i64;
        self.func13(env, var3.wrapping_add(1184), arg1);
        let var60 = self.memory.load64(arg0 as usize) as i64;
        self.memory.store64(var3.wrapping_add(960).wrapping_add(56) as usize, var60 as u64);
        self.memory.store64(var3.wrapping_add(960).wrapping_add(48) as usize, slot_var11_0_i64 as u64);
        self.memory.store64(var3.wrapping_add(960).wrapping_add(40) as usize, slot_var12_0_i64 as u64);
        slot_var10_0_i64 = slot_var8_0_i64 as i64;
        slot_var9_0_i64 = slot_var7_0_i64 as i64;
        slot_var4_0_i64 = slot_var6_0_i64 as i64;
        let mut slot_var3_992_i64 = slot_var3_1184_i64 as i64;
        slot_var3_960_i64 = slot_var3_1152_i64 as i64;
        let var61 = self.func121(env, arg2, var3.wrapping_add(960), 64);
        arg0 = var61;
        let var62 = self.func121(env, var5, var3.wrapping_add(320).wrapping_add(64), 64);
        var62;
        self.func23(env, arg0, var3.wrapping_add(128));
        self.global0 = var3.wrapping_add(1216);
    }
    fn func69(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
        var3 = var15.wrapping_sub(448);
        self.global0 = var3;
        let var16 = self.func121(env, var3, arg0, 64);
        var3 = var16;
        var4 = arg0.wrapping_add(64);
        let var17 = self.func121(env, var3.wrapping_add(64), var4, 64);
        var17;
        self.func5(env, var3, arg1);
        self.func5(env, var3.wrapping_add(64), arg2);
        let var20 = self.func121(env, var3.wrapping_add(128), arg2, 64);
        var20;
        let var21 = self.func121(env, var3.wrapping_add(192), var4, 64);
        var21;
        var5 = arg0.wrapping_add(128);
        self.func10(env, var3.wrapping_add(192), var5);
        self.func5(env, var3.wrapping_add(128), var3.wrapping_add(192));
        self.func9(env, var3.wrapping_add(128), var3.wrapping_add(64));
        let var25 = self.func121(env, var3.wrapping_add(256), var3.wrapping_add(128), 64);
        var25;
        let var26 = self.func11(env, var3.wrapping_add(256));
        let var27 = self.func11(env, var26);
        let var28 = self.func11(env, var27);
        var6 = var28;
        var7 = var3.wrapping_add(320).wrapping_add(24);
        let var29 = self.memory.load64(var3.wrapping_add(128).wrapping_add(56) as usize) as i64;
        let mut slot_var7_0_i64 = var29 as i64;
        var8 = var3.wrapping_add(320).wrapping_add(16);
        let var30 = self.memory.load64(var3.wrapping_add(128).wrapping_add(48) as usize) as i64;
        let mut slot_var8_0_i64 = var30 as i64;
        var9 = var3.wrapping_add(320).wrapping_add(8);
        let var31 = self.memory.load64(var3.wrapping_add(128).wrapping_add(40) as usize) as i64;
        let mut slot_var9_0_i64 = var31 as i64;
        let mut slot_var3_160_i64 = self.memory.load64(var3 as usize + 160) as i64;
        let mut slot_var3_320_i64 = slot_var3_160_i64 as i64;
        self.func12(env, var3.wrapping_add(320));
        self.func13(env, var3.wrapping_add(320), var6);
        self.func13(env, var3.wrapping_add(320), var3.wrapping_add(128));
        var10 = var3.wrapping_add(384).wrapping_add(24);
        let var35 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
        let mut slot_var10_0_i64 = var35 as i64;
        var11 = var3.wrapping_add(384).wrapping_add(16);
        let var36 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
        let mut slot_var11_0_i64 = var36 as i64;
        var12 = var3.wrapping_add(384).wrapping_add(8);
        let var37 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
        let mut slot_var12_0_i64 = var37 as i64;
        let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
        let mut slot_var3_384_i64 = slot_var6_32_i64 as i64;
        self.func13(env, var3.wrapping_add(384), var3.wrapping_add(160));
        var6 = var3.wrapping_add(352).wrapping_add(24);
        let mut slot_var6_0_i64 = slot_var10_0_i64 as i64;
        var13 = var3.wrapping_add(352).wrapping_add(16);
        let mut slot_var13_0_i64 = slot_var11_0_i64 as i64;
        var14 = var3.wrapping_add(352).wrapping_add(8);
        let mut slot_var14_0_i64 = slot_var12_0_i64 as i64;
        let mut slot_var3_352_i64 = slot_var3_384_i64 as i64;
        self.func13(env, var3.wrapping_add(352), var3.wrapping_add(128));
        self.memory.store64(var3.wrapping_add(384).wrapping_add(56) as usize, slot_var6_0_i64 as u64);
        self.memory.store64(var3.wrapping_add(384).wrapping_add(48) as usize, slot_var13_0_i64 as u64);
        self.memory.store64(var3.wrapping_add(384).wrapping_add(40) as usize, slot_var14_0_i64 as u64);
        slot_var12_0_i64 = slot_var9_0_i64 as i64;
        slot_var11_0_i64 = slot_var8_0_i64 as i64;
        slot_var10_0_i64 = slot_var7_0_i64 as i64;
        let mut slot_var3_416_i64 = slot_var3_352_i64 as i64;
        slot_var3_384_i64 = slot_var3_320_i64 as i64;
        let var40 = self.func121(env, var3.wrapping_add(128), var3.wrapping_add(384), 64);
        var40;
        self.func10(env, var3.wrapping_add(128), var3);
        let var42 = self.func121(env, var3.wrapping_add(192), arg1, 64);
        var42;
        let var43 = self.func121(env, var3.wrapping_add(384), arg0, 64);
        var43;
        self.func10(env, var3.wrapping_add(384), var5);
        self.func5(env, var3.wrapping_add(192), var3.wrapping_add(384));
        self.func9(env, var3.wrapping_add(192), var3);
        self.func10(env, var3.wrapping_add(192), var3.wrapping_add(64));
        let var48 = self.func121(env, var3.wrapping_add(256), arg1, 64);
        var48;
        self.func10(env, var3.wrapping_add(256), arg2);
        let var50 = self.func121(env, var3.wrapping_add(384), arg0, 64);
        var50;
        self.func10(env, var3.wrapping_add(384), var4);
        self.func5(env, var3.wrapping_add(256), var3.wrapping_add(384));
        self.func9(env, var3.wrapping_add(256), var3);
        self.func9(env, var3.wrapping_add(256), var3.wrapping_add(64));
        let var55 = self.func121(env, arg0, var3.wrapping_add(128), 64);
        var55;
        let var56 = self.func121(env, var4, var3.wrapping_add(256), 64);
        var56;
        let var57 = self.func121(env, var5, var3.wrapping_add(192), 64);
        var57;
        self.global0 = var3.wrapping_add(448);
    }
    fn func70(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
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
        'label0: loop {
            let var19 = self.memory.load64(arg0 as usize + 24) as i64;
            var2 = var19;
            var3 = (var2 as u64).wrapping_shr(32 as u32) as i64;
            let var20 = self.memory.load64(arg1 as usize) as i64;
            var4 = var20;
            var5 = (var4 as u64).wrapping_shr(32 as u32) as i64;
            var6 = var2 & 4294967295;
            var7 = var6.wrapping_mul(var5);
            var2 = var4 & 4294967295;
            var4 = var7.wrapping_add(var3.wrapping_mul(var2));
            var7 = var6.wrapping_mul(var2);
            var8 = var7.wrapping_add(var4.wrapping_shl(32 as u32));
            let var21 = self.memory.load64(arg0 as usize + 16) as i64;
            var7 = var21;
            var4 = (var7 as u64).wrapping_shr(32 as u32) as i64;
            var7 = var7 & 4294967295;
            var9 = var7.wrapping_mul(var5);
            var10 = var9.wrapping_add(var4.wrapping_mul(var2));
            var9 = var7.wrapping_mul(var2);
            var11 = var9.wrapping_add(var10.wrapping_shl(32 as u32));
            let var22 = self.memory.load64(arg0 as usize + 8) as i64;
            var9 = var22;
            var10 = (var9 as u64).wrapping_shr(32 as u32) as i64;
            var9 = var9 & 4294967295;
            var12 = var9.wrapping_mul(var5);
            var13 = var12.wrapping_add(var10.wrapping_mul(var2));
            var12 = var9.wrapping_mul(var2);
            var14 = var12.wrapping_add(var13.wrapping_shl(32 as u32));
            let var23 = var12;
            let var24 = var5;
            let var25 = self.memory.load64(arg0 as usize) as i64;
            var12 = var25;
            var13 = (var12 as u64).wrapping_shr(32 as u32) as i64;
            var12 = var12 & 4294967295;
            var15 = var5.wrapping_mul(var12);
            var5 = var15.wrapping_add(var2.wrapping_mul(var13));
            var2 = var2.wrapping_mul(var12);
            var15 = var2.wrapping_add(var5.wrapping_shl(32 as u32));
            var16 = var14.wrapping_add(var24.wrapping_mul(var13).wrapping_add((((var5 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var15 as u64) < var2 as u64) as i32 as u32 as i64));
            var14 = var11.wrapping_add(var10.wrapping_mul(var5).wrapping_add((((var13 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var13 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var23 as u64) as i32 as u32 as i64).wrapping_add(((var16 as u64) < var14 as u64) as i32 as u32 as i64));
            var11 = var8.wrapping_add(var4.wrapping_mul(var5).wrapping_add((((var10 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var10 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var11 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var11 as u64) as i32 as u32 as i64));
            var17 = var3.wrapping_mul(var5).wrapping_add((((var4 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var4 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64);
            var2 = var15.wrapping_mul(9786893198990664585);
            var5 = (var2 as u64).wrapping_shr(32 as u32) as i64;
            let var26 = var5;
            var2 = var2 & 4294967295;
            var18 = var2.wrapping_mul(811880050);
            var8 = var18.wrapping_add(var5.wrapping_mul(3778125865));
            var18 = var2.wrapping_mul(3778125865);
            var8 = var18.wrapping_add(var8.wrapping_shl(32 as u32));
            var11 = var8.wrapping_add(var11);
            var8 = var14.wrapping_add(var2.wrapping_mul(2172737629));
            let var27 = var8;
            var18 = var2.wrapping_mul(3092268470);
            var14 = var18.wrapping_add(var5.wrapping_mul(2172737629));
            var14 = var8.wrapping_add(var14.wrapping_shl(32 as u32));
            var8 = var16.wrapping_add(var2.wrapping_mul(1752287885));
            let var28 = var8;
            var18 = var2.wrapping_mul(2541841041);
            var16 = var18.wrapping_add(var5.wrapping_mul(1752287885));
            var16 = var8.wrapping_add(var16.wrapping_shl(32 as u32));
            var8 = var15.wrapping_add(var2.wrapping_mul(3632069959));
            var2 = var2.wrapping_mul(1008765974);
            var5 = var2.wrapping_add(var5.wrapping_mul(3632069959));
            var15 = var16.wrapping_add((((var8 as u64) < var15 as u64) as i32 as u32 as i64 | var5.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var8 as u64) as i32 as u32 as i64));
            var8 = var14.wrapping_add((((var28 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_mul(2541841041)).wrapping_add((((var16 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var16 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var15 as u64) < var16 as u64) as i32 as u32 as i64));
            var14 = var11.wrapping_add((((var27 as u64) < var14 as u64) as i32 as u32 as i64 | var5.wrapping_mul(3092268470)).wrapping_add((((var14 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var14 as u64) as i32 as u32 as i64));
            var2 = var17.wrapping_add(var26.wrapping_mul(811880050).wrapping_add((((var8 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var11 as u64) as i32 as u32 as i64));
            if ((var2 as u64) < var17 as u64) as i32 != 0 {
                break 'label0;
            }
            let var29 = self.memory.load64(arg1 as usize + 8) as i64;
            var16 = var29;
            var5 = var16 & 4294967295;
            var17 = var5.wrapping_mul(var6);
            var11 = var17.wrapping_add(var2);
            let var30 = var11;
            var2 = (var16 as u64).wrapping_shr(32 as u32) as i64;
            var17 = var2.wrapping_mul(var6);
            var16 = var17.wrapping_add(var5.wrapping_mul(var3));
            var16 = var11.wrapping_add(var16.wrapping_shl(32 as u32));
            var11 = var14.wrapping_add(var5.wrapping_mul(var7));
            let var31 = var11;
            var17 = var2.wrapping_mul(var7);
            var14 = var17.wrapping_add(var5.wrapping_mul(var4));
            var14 = var11.wrapping_add(var14.wrapping_shl(32 as u32));
            var11 = var8.wrapping_add(var5.wrapping_mul(var9));
            let var32 = var11;
            var17 = var2.wrapping_mul(var9);
            var8 = var17.wrapping_add(var5.wrapping_mul(var10));
            var8 = var11.wrapping_add(var8.wrapping_shl(32 as u32));
            var11 = var15.wrapping_add(var5.wrapping_mul(var12));
            var2 = var2.wrapping_mul(var12);
            var5 = var2.wrapping_add(var5.wrapping_mul(var13));
            var15 = var11.wrapping_add(var5.wrapping_shl(32 as u32));
            var11 = var8.wrapping_add((((var11 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var13)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var15 as u64) < var11 as u64) as i32 as u32 as i64));
            var8 = var14.wrapping_add((((var32 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var10)).wrapping_add((((var8 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64));
            var17 = var16.wrapping_add((((var31 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var4)).wrapping_add((((var14 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var14 as u64) as i32 as u32 as i64));
            var18 = (((var30 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var3)).wrapping_add((((var16 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var16 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(((var17 as u64) < var16 as u64) as i32 as u32 as i64);
            var2 = var15.wrapping_mul(9786893198990664585);
            var5 = (var2 as u64).wrapping_shr(32 as u32) as i64;
            let var33 = var5;
            var2 = var2 & 4294967295;
            var16 = var2.wrapping_mul(811880050);
            var14 = var16.wrapping_add(var5.wrapping_mul(3778125865));
            var16 = var2.wrapping_mul(3778125865);
            var14 = var16.wrapping_add(var14.wrapping_shl(32 as u32));
            var16 = var14.wrapping_add(var17);
            var14 = var8.wrapping_add(var2.wrapping_mul(2172737629));
            let var34 = var14;
            var17 = var2.wrapping_mul(3092268470);
            var8 = var17.wrapping_add(var5.wrapping_mul(2172737629));
            var8 = var14.wrapping_add(var8.wrapping_shl(32 as u32));
            var14 = var11.wrapping_add(var2.wrapping_mul(1752287885));
            let var35 = var14;
            var17 = var2.wrapping_mul(2541841041);
            var11 = var17.wrapping_add(var5.wrapping_mul(1752287885));
            var11 = var14.wrapping_add(var11.wrapping_shl(32 as u32));
            var14 = var15.wrapping_add(var2.wrapping_mul(3632069959));
            var2 = var2.wrapping_mul(1008765974);
            var5 = var2.wrapping_add(var5.wrapping_mul(3632069959));
            var15 = var11.wrapping_add((((var14 as u64) < var15 as u64) as i32 as u32 as i64 | var5.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var14 as u64) as i32 as u32 as i64));
            var11 = var8.wrapping_add((((var35 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_mul(2541841041)).wrapping_add((((var11 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var11 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var15 as u64) < var11 as u64) as i32 as u32 as i64));
            var8 = var16.wrapping_add((((var34 as u64) < var8 as u64) as i32 as u32 as i64 | var5.wrapping_mul(3092268470)).wrapping_add((((var8 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64));
            var2 = var18.wrapping_add(var33.wrapping_mul(811880050).wrapping_add((((var14 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_add(((var16 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var16 as u64) as i32 as u32 as i64));
            if ((var2 as u64) < var18 as u64) as i32 != 0 {
                break 'label0;
            }
            let var36 = self.memory.load64(arg1 as usize + 16) as i64;
            var16 = var36;
            var5 = var16 & 4294967295;
            var17 = var5.wrapping_mul(var6);
            var14 = var17.wrapping_add(var2);
            let var37 = var14;
            var2 = (var16 as u64).wrapping_shr(32 as u32) as i64;
            var17 = var2.wrapping_mul(var6);
            var16 = var17.wrapping_add(var5.wrapping_mul(var3));
            var16 = var14.wrapping_add(var16.wrapping_shl(32 as u32));
            var14 = var8.wrapping_add(var5.wrapping_mul(var7));
            let var38 = var14;
            var17 = var2.wrapping_mul(var7);
            var8 = var17.wrapping_add(var5.wrapping_mul(var4));
            var8 = var14.wrapping_add(var8.wrapping_shl(32 as u32));
            var14 = var11.wrapping_add(var5.wrapping_mul(var9));
            let var39 = var14;
            var17 = var2.wrapping_mul(var9);
            var11 = var17.wrapping_add(var5.wrapping_mul(var10));
            var11 = var14.wrapping_add(var11.wrapping_shl(32 as u32));
            var14 = var15.wrapping_add(var5.wrapping_mul(var12));
            var2 = var2.wrapping_mul(var12);
            var5 = var2.wrapping_add(var5.wrapping_mul(var13));
            var15 = var14.wrapping_add(var5.wrapping_shl(32 as u32));
            var14 = var11.wrapping_add((((var14 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var13)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var15 as u64) < var14 as u64) as i32 as u32 as i64));
            var11 = var8.wrapping_add((((var39 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var10)).wrapping_add((((var11 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var11 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var11 as u64) as i32 as u32 as i64));
            var17 = var16.wrapping_add((((var38 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var4)).wrapping_add((((var8 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64));
            var18 = (((var37 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var3)).wrapping_add((((var16 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var16 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var17 as u64) < var16 as u64) as i32 as u32 as i64);
            var2 = var15.wrapping_mul(9786893198990664585);
            var5 = (var2 as u64).wrapping_shr(32 as u32) as i64;
            let var40 = var5;
            var2 = var2 & 4294967295;
            var16 = var2.wrapping_mul(811880050);
            var8 = var16.wrapping_add(var5.wrapping_mul(3778125865));
            var16 = var2.wrapping_mul(3778125865);
            var8 = var16.wrapping_add(var8.wrapping_shl(32 as u32));
            var16 = var8.wrapping_add(var17);
            var8 = var11.wrapping_add(var2.wrapping_mul(2172737629));
            let var41 = var8;
            var17 = var2.wrapping_mul(3092268470);
            var11 = var17.wrapping_add(var5.wrapping_mul(2172737629));
            var11 = var8.wrapping_add(var11.wrapping_shl(32 as u32));
            var8 = var14.wrapping_add(var2.wrapping_mul(1752287885));
            let var42 = var8;
            var17 = var2.wrapping_mul(2541841041);
            var14 = var17.wrapping_add(var5.wrapping_mul(1752287885));
            var14 = var8.wrapping_add(var14.wrapping_shl(32 as u32));
            var8 = var15.wrapping_add(var2.wrapping_mul(3632069959));
            var2 = var2.wrapping_mul(1008765974);
            var5 = var2.wrapping_add(var5.wrapping_mul(3632069959));
            var15 = var14.wrapping_add((((var8 as u64) < var15 as u64) as i32 as u32 as i64 | var5.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var8 as u64) as i32 as u32 as i64));
            var8 = var11.wrapping_add((((var42 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_mul(2541841041)).wrapping_add((((var14 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var15 as u64) < var14 as u64) as i32 as u32 as i64));
            var11 = var16.wrapping_add((((var41 as u64) < var11 as u64) as i32 as u32 as i64 | var5.wrapping_mul(3092268470)).wrapping_add((((var11 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var11 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var11 as u64) as i32 as u32 as i64));
            var2 = var18.wrapping_add(var40.wrapping_mul(811880050).wrapping_add((((var8 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_add(((var16 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var11 as u64) < var16 as u64) as i32 as u32 as i64));
            if ((var2 as u64) < var18 as u64) as i32 != 0 {
                break 'label0;
            }
            let var43 = self.memory.load64(arg1 as usize + 24) as i64;
            var16 = var43;
            var5 = var16 & 4294967295;
            var17 = var5.wrapping_mul(var6);
            var14 = var17.wrapping_add(var2);
            var2 = (var16 as u64).wrapping_shr(32 as u32) as i64;
            var6 = var2.wrapping_mul(var6);
            var3 = var6.wrapping_add(var5.wrapping_mul(var3));
            var6 = var14.wrapping_add(var3.wrapping_shl(32 as u32));
            var3 = var11.wrapping_add(var5.wrapping_mul(var7));
            let var44 = var3;
            var7 = var2.wrapping_mul(var7);
            var4 = var7.wrapping_add(var5.wrapping_mul(var4));
            var4 = var3.wrapping_add(var4.wrapping_shl(32 as u32));
            var3 = var8.wrapping_add(var5.wrapping_mul(var9));
            let var45 = var3;
            var9 = var2.wrapping_mul(var9);
            var7 = var9.wrapping_add(var5.wrapping_mul(var10));
            var7 = var3.wrapping_add(var7.wrapping_shl(32 as u32));
            var10 = var15.wrapping_add(var5.wrapping_mul(var12));
            var2 = var2.wrapping_mul(var12);
            var5 = var2.wrapping_add(var5.wrapping_mul(var13));
            var3 = var10.wrapping_add(var5.wrapping_shl(32 as u32));
            var10 = var7.wrapping_add((((var10 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var13)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var3 as u64) < var10 as u64) as i32 as u32 as i64));
            var7 = var4.wrapping_add((((var45 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var10)).wrapping_add((((var7 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var10 as u64) < var7 as u64) as i32 as u32 as i64));
            var4 = var6.wrapping_add((((var44 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var4)).wrapping_add((((var4 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var4 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var4 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var7 as u64) < var4 as u64) as i32 as u32 as i64));
            var9 = (((var14 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(var3)).wrapping_add((((var3 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var3 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var4 as u64) < var6 as u64) as i32 as u32 as i64);
            var2 = var3.wrapping_mul(9786893198990664585);
            var5 = (var2 as u64).wrapping_shr(32 as u32) as i64;
            let var46 = var5;
            var2 = var2 & 4294967295;
            var13 = var2.wrapping_mul(811880050);
            var6 = var13.wrapping_add(var5.wrapping_mul(3778125865));
            var13 = var2.wrapping_mul(3778125865);
            var6 = var13.wrapping_add(var6.wrapping_shl(32 as u32));
            var4 = var6.wrapping_add(var4);
            var6 = var7.wrapping_add(var2.wrapping_mul(2172737629));
            let var47 = var6;
            var13 = var2.wrapping_mul(3092268470);
            var7 = var13.wrapping_add(var5.wrapping_mul(2172737629));
            var7 = var6.wrapping_add(var7.wrapping_shl(32 as u32));
            var6 = var10.wrapping_add(var2.wrapping_mul(1752287885));
            let var48 = var6;
            var13 = var2.wrapping_mul(2541841041);
            var10 = var13.wrapping_add(var5.wrapping_mul(1752287885));
            var10 = var6.wrapping_add(var10.wrapping_shl(32 as u32));
            var6 = var3.wrapping_add(var2.wrapping_mul(3632069959));
            var2 = var2.wrapping_mul(1008765974);
            var5 = var2.wrapping_add(var5.wrapping_mul(3632069959));
            var5 = var10.wrapping_add((((var6 as u64) < var3 as u64) as i32 as u32 as i64 | var5.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var6 as u64) as i32 as u32 as i64));
            var2 = var7.wrapping_add((((var48 as u64) < var10 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_mul(2541841041)).wrapping_add((((var10 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var10 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var10 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var10 as u64) as i32 as u32 as i64));
            var3 = var4.wrapping_add((((var47 as u64) < var7 as u64) as i32 as u32 as i64 | var5.wrapping_mul(3092268470)).wrapping_add((((var7 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var2 as u64) < var7 as u64) as i32 as u32 as i64));
            var6 = var9.wrapping_add(var46.wrapping_mul(811880050).wrapping_add((((var6 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var6 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_add(((var4 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var4 as u64) as i32 as u32 as i64));
            if ((var6 as u64) < var9 as u64) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize + 24, var6 as u64);
            self.memory.store64(arg0 as usize + 16, var3 as u64);
            self.memory.store64(arg0 as usize + 8, var2 as u64);
            self.memory.store64(arg0 as usize, var5 as u64);
            'label1: loop {
                let var49 = self.func99(env, arg0);
                if (var49 == 0) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(arg0 as usize, var5.wrapping_add(14114127202429895353) as u64);
                var5 = var2.wrapping_add({ let a = 7529619929231668594; let b = 7529619929231668595; if ((var5 as u64) < 4332616871279656263 as u64) as i32 != 0 { a } else { b } });
                self.memory.store64(arg0 as usize + 8, var5 as u64);
                var5 = { let a = 5165552122434856866; let b = 5165552122434856867; if (var5 as u64 >= var2 as u64) as i32 != 0 { a } else { b } };
                var2 = var5.wrapping_add(var3);
                self.memory.store64(arg0 as usize + 16, var2 as u64);
                self.memory.store64(arg0 as usize + 24, ({ let a = 14959745806906580950; let b = 14959745806906580951; if (var2 as u64 >= var5 as u64) as i32 != 0 { a } else { b } }).wrapping_add(var6) as u64);
                break;
            }
            return;
            break;
        }
        self.func73(env);
        unreachable!();
    }
    fn func71(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 32) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var1.wrapping_add(var2) as usize, 0 /* False */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
        self.memory.store64(arg0 as usize, slot_var1_0_i64 as u64);
        self.memory.store8(arg0 as usize + 32, 0 as u8);
        let var4 = self.memory.load64(var1.wrapping_add(24) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(24) as usize, var4 as u64);
        let var5 = self.memory.load64(var1.wrapping_add(16) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(16) as usize, var5 as u64);
        let var6 = self.memory.load64(var1.wrapping_add(8) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(8) as usize, var6 as u64);
    }
    fn func72(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(384);
        self.global0 = var2;
        let var4 = self.func121(env, var2, arg1, 384);
        arg1 = var4;
        let var5 = self.func66(env, arg1);
        let var6 = self.func121(env, arg0, var5, 384);
        var6;
        self.global0 = arg1.wrapping_add(384);
    }
    fn func73(&mut self, env: &Env) {
        self.func96(env);
        unreachable!();
    }
    fn func74(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func75(env, arg0, 1051384);
        (var1.wrapping_shl(24 as u32).wrapping_shr(24 as u32) > -1) as i32
    }
    fn func75(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        var2 = 24;
        'label0: loop {
            'label1: loop {
                if (var2 != -8) as i32 != 0 {
                    break 'label1;
                }
                return 0;
                break;
            }
            'label2: loop {
                let var5 = self.memory.load64(arg0.wrapping_add(var2) as usize) as i64;
                var3 = var5;
                let var6 = self.memory.load64(arg1.wrapping_add(var2) as usize) as i64;
                var4 = var6;
                if (var3 as u64 >= var4 as u64) as i32 != 0 {
                    break 'label2;
                }
                return 255;
                break;
            }
            var2 = var2.wrapping_add(-8);
            if (var3 as u64 <= var4 as u64) as i32 != 0 {
                continue 'label0;
            }
            break;
        }
        1
    }
    fn func76(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
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
        let var40 = self.global0;
        var2 = var40.wrapping_sub(32);
        self.global0 = var2;
        let var41 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
        var3 = var41;
        self.memory.store64(var2.wrapping_add(24) as usize, var3 as u64);
        let var42 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
        var4 = var42;
        self.memory.store64(var2.wrapping_add(16) as usize, var4 as u64);
        let var43 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
        var5 = var43;
        self.memory.store64(var2.wrapping_add(8) as usize, var5 as u64);
        let var44 = self.memory.load64(arg1 as usize) as i64;
        var6 = var44;
        let mut slot_var2_0_i64 = var6 as i64;
        'label0: loop {
            var7 = var3 & 4294967295;
            var8 = var5 & 4294967295;
            var9 = var7.wrapping_mul(var8);
            var10 = (var3 as u64).wrapping_shr(32 as u32) as i64;
            var11 = var10.wrapping_mul(var8);
            var3 = (var5 as u64).wrapping_shr(32 as u32) as i64;
            var12 = var11.wrapping_add(var7.wrapping_mul(var3));
            var13 = var9.wrapping_add(var12.wrapping_shl(32 as u32));
            var5 = (var6 as u64).wrapping_shr(32 as u32) as i64;
            var6 = var6 & 4294967295;
            var14 = var10.wrapping_mul(var6);
            var15 = var14.wrapping_add(var7.wrapping_mul(var5));
            var16 = var7.wrapping_mul(var6);
            var14 = var16.wrapping_add(var15.wrapping_shl(32 as u32));
            var15 = (var4 as u64).wrapping_shr(32 as u32) as i64;
            var17 = var15.wrapping_mul(var6);
            var4 = var4 & 4294967295;
            var16 = var17.wrapping_add(var4.wrapping_mul(var5));
            var17 = var4.wrapping_mul(var6);
            var16 = var17.wrapping_add(var16.wrapping_shl(32 as u32));
            let var45 = var17;
            var18 = var3.wrapping_mul(var6);
            var17 = var18.wrapping_add(var8.wrapping_mul(var5));
            var18 = var8.wrapping_mul(var6);
            var17 = var18.wrapping_add(var17.wrapping_shl(32 as u32));
            var18 = var16.wrapping_add(var3.wrapping_mul(var5).wrapping_add((((var17 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var17 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var17 as u64) < var18 as u64) as i32 as u32 as i64));
            var19 = var14.wrapping_add(var15.wrapping_mul(var5).wrapping_add((((var16 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var16 as u64) < var45 as u64) as i32 as u32 as i64).wrapping_add(((var18 as u64) < var16 as u64) as i32 as u32 as i64));
            var14 = var13.wrapping_add(var10.wrapping_mul(var5).wrapping_add((((var15 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var15 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var14 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_add(((var19 as u64) < var14 as u64) as i32 as u32 as i64));
            var20 = var15.wrapping_mul(var8);
            var16 = var20.wrapping_add(var4.wrapping_mul(var3));
            var20 = var4.wrapping_mul(var8);
            var16 = var20.wrapping_add(var16.wrapping_shl(32 as u32));
            var19 = var16.wrapping_add(var19);
            var16 = var14.wrapping_add(var15.wrapping_mul(var3).wrapping_add((((var16 as u64) < var20 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var16 as u64) < var20 as u64) as i32 as u32 as i64).wrapping_add(((var19 as u64) < var16 as u64) as i32 as u32 as i64));
            arg1 = ((var16 as u64) < var14 as u64) as i32;
            var12 = var10.wrapping_mul(var3).wrapping_add((((var12 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var12 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var13 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var13 as u64) as i32 as u32 as i64);
            var21 = var12.wrapping_add(arg1 as u32 as i64);
            if (({ let a = arg1; let b = ((var21 as u64) < var12 as u64) as i32; if (var16 as u64 >= var14 as u64) as i32 != 0 { a } else { b } }) == 1) as i32 != 0 {
                break 'label0;
            }
            let var46 = var5;
            var5 = var6.wrapping_mul(var5);
            var6 = var6.wrapping_mul(var6);
            var14 = var6.wrapping_add(var5.wrapping_shl(33 as u32));
            var22 = var46.wrapping_mul(var5).wrapping_add((var5 as u64).wrapping_shr(31 as u32) as i64).wrapping_add(((var14 as u64) < var6 as u64) as i32 as u32 as i64);
            var9 = var22.wrapping_add(var17.wrapping_shl(1 /* True */ as u32));
            var6 = var14.wrapping_mul(9786893198990664585);
            var5 = var6 & 4294967295;
            var11 = var9.wrapping_add(var5.wrapping_mul(1752287885));
            var23 = var5.wrapping_mul(2541841041);
            var6 = (var6 as u64).wrapping_shr(32 as u32) as i64;
            var20 = var23.wrapping_add(var6.wrapping_mul(1752287885));
            var24 = var11.wrapping_add(var20.wrapping_shl(32 as u32));
            var12 = var14.wrapping_add(var5.wrapping_mul(3632069959));
            var13 = var5.wrapping_mul(1008765974);
            var14 = var13.wrapping_add(var6.wrapping_mul(3632069959));
            var13 = var24.wrapping_add((((var12 as u64) < var14 as u64) as i32 as u32 as i64 | var6.wrapping_mul(1008765974)).wrapping_add((((var14 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var12.wrapping_add(var14.wrapping_shl(32 as u32)) as u64) < var12 as u64) as i32 as u32 as i64));
            var12 = var13.wrapping_mul(9786893198990664585);
            var14 = var12 & 4294967295;
            var25 = var14.wrapping_mul(3778125865);
            var26 = var14.wrapping_mul(811880050);
            var12 = (var12 as u64).wrapping_shr(32 as u32) as i64;
            var27 = var26.wrapping_add(var12.wrapping_mul(3778125865));
            var28 = var25.wrapping_add(var27.wrapping_shl(32 as u32));
            var29 = var5.wrapping_mul(811880050);
            var30 = var29.wrapping_add(var6.wrapping_mul(3778125865));
            var29 = var5.wrapping_mul(3778125865);
            var30 = var29.wrapping_add(var30.wrapping_shl(32 as u32));
            let var47 = var29;
            let var48 = var3;
            var3 = var8.wrapping_mul(var3);
            var29 = var8.wrapping_mul(var8);
            var8 = var29.wrapping_add(var3.wrapping_shl(33 as u32));
            var3 = var8.wrapping_add(var18.wrapping_shl(1 /* True */ as u32) | (var17 as u64).wrapping_shr(63 as u32) as i64);
            var17 = var3.wrapping_add(((var9 as u64) < var22 as u64) as i32 as u32 as i64);
            var22 = var48.wrapping_mul(var3).wrapping_add((var3 as u64).wrapping_shr(31 as u32) as i64).wrapping_add(((var8 as u64) < var29 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var17 as u64) < var3 as u64) as i32 as u32 as i64);
            var18 = var22.wrapping_add(var19.wrapping_shl(1 /* True */ as u32) | (var18 as u64).wrapping_shr(63 as u32) as i64);
            var8 = var30.wrapping_add(var18);
            let var49 = var30;
            var30 = var5.wrapping_mul(3092268470);
            var3 = var30.wrapping_add(var6.wrapping_mul(2172737629));
            var5 = var5.wrapping_mul(2172737629);
            var3 = var5.wrapping_add(var3.wrapping_shl(32 as u32));
            var5 = var3.wrapping_add(var17);
            var3 = var5.wrapping_add((((var11 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_mul(2541841041)).wrapping_add((((var20 as u64) < var23 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var20 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var24 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(((var13 as u64) < var24 as u64) as i32 as u32 as i64));
            var5 = var8.wrapping_add(var6.wrapping_mul(3092268470).wrapping_add((((var3 as u64) < var30 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var3 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var3 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var5 as u64) as i32 as u32 as i64));
            var31 = var6.wrapping_mul(811880050).wrapping_add((((var30 as u64) < var29 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var30 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var30 as u64) < var47 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var49 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var8 as u64) as i32 as u32 as i64);
            var32 = var4.wrapping_mul(var4);
            var33 = var4.wrapping_mul(var15);
            var17 = var32.wrapping_add(var33.wrapping_shl(33 as u32));
            var18 = var17.wrapping_add(((var18 as u64) < var22 as u64) as i32 as u32 as i64);
            var34 = var18.wrapping_add(var16.wrapping_shl(1 /* True */ as u32) | (var19 as u64).wrapping_shr(63 as u32) as i64);
            var35 = var31.wrapping_add(var34);
            var19 = var28.wrapping_add(var35);
            var8 = var5.wrapping_add(var14.wrapping_mul(2172737629));
            let var50 = var8;
            var6 = var14.wrapping_mul(3092268470);
            var5 = var6.wrapping_add(var12.wrapping_mul(2172737629));
            var6 = var8.wrapping_add(var5.wrapping_shl(32 as u32));
            var8 = var3.wrapping_add(var14.wrapping_mul(1752287885));
            let var51 = var8;
            var5 = var14.wrapping_mul(2541841041);
            var3 = var5.wrapping_add(var12.wrapping_mul(1752287885));
            var3 = var8.wrapping_add(var3.wrapping_shl(32 as u32));
            var8 = var13.wrapping_add(var14.wrapping_mul(3632069959));
            var14 = var14.wrapping_mul(1008765974);
            var5 = var14.wrapping_add(var12.wrapping_mul(3632069959));
            var5 = var3.wrapping_add((((var8 as u64) < var13 as u64) as i32 as u32 as i64 | var12.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var8 as u64) as i32 as u32 as i64));
            var14 = var6.wrapping_add((((var51 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(var12.wrapping_mul(2541841041)).wrapping_add((((var3 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var3 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var3 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var3 as u64) as i32 as u32 as i64));
            var13 = var19.wrapping_add((((var50 as u64) < var5 as u64) as i32 as u32 as i64 | var12.wrapping_mul(3092268470)).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var6 as u64) as i32 as u32 as i64));
            var3 = var5.wrapping_mul(9786893198990664585);
            var8 = var3 & 4294967295;
            var9 = var13.wrapping_add(var8.wrapping_mul(2172737629));
            var36 = var8.wrapping_mul(3092268470);
            var3 = (var3 as u64).wrapping_shr(32 as u32) as i64;
            var11 = var36.wrapping_add(var3.wrapping_mul(2172737629));
            var20 = var9.wrapping_add(var11.wrapping_shl(32 as u32));
            var6 = var14.wrapping_add(var8.wrapping_mul(1752287885));
            let var52 = var6;
            var24 = var8.wrapping_mul(2541841041);
            var14 = var24.wrapping_add(var3.wrapping_mul(1752287885));
            var24 = var6.wrapping_add(var14.wrapping_shl(32 as u32));
            var6 = var5.wrapping_add(var8.wrapping_mul(3632069959));
            var14 = var8.wrapping_mul(1008765974);
            var5 = var14.wrapping_add(var3.wrapping_mul(3632069959));
            var14 = var24.wrapping_add((((var6 as u64) < var5 as u64) as i32 as u32 as i64 | var3.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var6 as u64) as i32 as u32 as i64));
            var24 = var20.wrapping_add((((var52 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(var3.wrapping_mul(2541841041)).wrapping_add((((var14 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var24 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var24 as u64) as i32 as u32 as i64));
            var6 = var14.wrapping_mul(9786893198990664585);
            var5 = var6 & 4294967295;
            var30 = var24.wrapping_add(var5.wrapping_mul(1752287885));
            var37 = var5.wrapping_mul(2541841041);
            var6 = (var6 as u64).wrapping_shr(32 as u32) as i64;
            var22 = var37.wrapping_add(var6.wrapping_mul(1752287885));
            var23 = var30.wrapping_add(var22.wrapping_shl(32 as u32));
            var29 = var14.wrapping_add(var5.wrapping_mul(3632069959));
            var38 = var5.wrapping_mul(1008765974);
            var14 = var38.wrapping_add(var6.wrapping_mul(3632069959));
            var14 = var23.wrapping_add((((var29 as u64) < var14 as u64) as i32 as u32 as i64 | var6.wrapping_mul(1008765974)).wrapping_add((((var14 as u64) < var38 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var14 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var29.wrapping_add(var14.wrapping_shl(32 as u32)) as u64) < var29 as u64) as i32 as u32 as i64));
            slot_var2_0_i64 = var14 as i64;
            var38 = var8.wrapping_mul(3778125865);
            var39 = var8.wrapping_mul(811880050);
            var8 = var39.wrapping_add(var3.wrapping_mul(3778125865));
            var29 = var38.wrapping_add(var8.wrapping_shl(32 as u32));
            var31 = ((var35 as u64) < var31 as u64) as i32 as u32 as i64;
            var32 = var15.wrapping_mul(var15).wrapping_add((var33 as u64).wrapping_shr(31 as u32) as i64).wrapping_add(((var17 as u64) < var32 as u64) as i32 as u32 as i64).wrapping_add(((var18 as u64) < var17 as u64) as i32 as u32 as i64).wrapping_add(((var34 as u64) < var18 as u64) as i32 as u32 as i64);
            var33 = var7.wrapping_mul(var4);
            var34 = var10.wrapping_mul(var4);
            var4 = var34.wrapping_add(var7.wrapping_mul(var15));
            var17 = var33.wrapping_add(var4.wrapping_shl(32 as u32));
            var18 = var17.wrapping_add(var21);
            var21 = var32.wrapping_add(var18.wrapping_shl(1 /* True */ as u32) | (var16 as u64).wrapping_shr(63 as u32) as i64);
            var16 = var31.wrapping_add(var21);
            var27 = var16.wrapping_add(var12.wrapping_mul(811880050).wrapping_add((((var27 as u64) < var26 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var27 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var28 as u64) < var25 as u64) as i32 as u32 as i64).wrapping_add(((var19 as u64) < var28 as u64) as i32 as u32 as i64).wrapping_add(((var13 as u64) < var19 as u64) as i32 as u32 as i64));
            var12 = var29.wrapping_add(var27);
            var13 = var12.wrapping_add((((var9 as u64) < var13 as u64) as i32 as u32 as i64 | var3.wrapping_mul(3092268470)).wrapping_add((((var11 as u64) < var36 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var11 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var20 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var24 as u64) < var20 as u64) as i32 as u32 as i64));
            var19 = var13.wrapping_add(var5.wrapping_mul(2172737629));
            var28 = var5.wrapping_mul(3092268470);
            var9 = var28.wrapping_add(var6.wrapping_mul(2172737629));
            var11 = var19.wrapping_add(var9.wrapping_shl(32 as u32));
            var20 = var11.wrapping_add((((var30 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_mul(2541841041)).wrapping_add((((var22 as u64) < var37 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var22 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var23 as u64) < var30 as u64) as i32 as u32 as i64).wrapping_add(((var14 as u64) < var23 as u64) as i32 as u32 as i64));
            let mut slot_var2_8_i64 = var20 as i64;
            var30 = var5.wrapping_mul(3778125865);
            var22 = var5.wrapping_mul(811880050);
            var5 = var22.wrapping_add(var6.wrapping_mul(3778125865));
            var24 = var30.wrapping_add(var5.wrapping_shl(32 as u32));
            var16 = (((var16 as u64) < var31 as u64) as i32 as u32 as i64).wrapping_add(((var27 as u64) < var16 as u64) as i32 as u32 as i64);
            var27 = var7.wrapping_mul(var7);
            var23 = var7.wrapping_mul(var10);
            var7 = var27.wrapping_add(var23.wrapping_shl(33 as u32));
            var17 = var10.wrapping_mul(var15).wrapping_add((((var4 as u64) < var34 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var4 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var17 as u64) < var33 as u64) as i32 as u32 as i64).wrapping_add(((var18 as u64) < var17 as u64) as i32 as u32 as i64);
            var15 = var7.wrapping_add(var17.wrapping_shl(1 /* True */ as u32) | (var18 as u64).wrapping_shr(63 as u32) as i64);
            var18 = var15.wrapping_add(((var21 as u64) < var32 as u64) as i32 as u32 as i64);
            var4 = var16.wrapping_add(var18);
            var12 = var4.wrapping_add(var3.wrapping_mul(811880050).wrapping_add((((var8 as u64) < var39 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var29 as u64) < var38 as u64) as i32 as u32 as i64).wrapping_add(((var12 as u64) < var29 as u64) as i32 as u32 as i64).wrapping_add(((var13 as u64) < var12 as u64) as i32 as u32 as i64));
            var8 = var24.wrapping_add(var12);
            var3 = var8.wrapping_add((((var19 as u64) < var13 as u64) as i32 as u32 as i64 | var6.wrapping_mul(3092268470)).wrapping_add((((var9 as u64) < var28 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var9 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var11 as u64) < var19 as u64) as i32 as u32 as i64).wrapping_add(((var20 as u64) < var11 as u64) as i32 as u32 as i64));
            let mut slot_var2_16_i64 = var3 as i64;
            var8 = (((var4 as u64) < var16 as u64) as i32 as u32 as i64).wrapping_add(((var12 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var17 as u64).wrapping_shr(63 as u32) as i64).wrapping_add(var10.wrapping_mul(var10).wrapping_add((var23 as u64).wrapping_shr(31 as u32) as i64).wrapping_add(((var7 as u64) < var27 as u64) as i32 as u32 as i64).wrapping_add(((var15 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(((var18 as u64) < var15 as u64) as i32 as u32 as i64))).wrapping_add(var6.wrapping_mul(811880050).wrapping_add((((var5 as u64) < var22 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var24 as u64) < var30 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var8 as u64) as i32 as u32 as i64));
            let mut slot_var2_24_i64 = var8 as i64;
            'label1: loop {
                let var53 = self.func74(env, var2);
                if (var53 == 0) as i32 != 0 {
                    break 'label1;
                }
                slot_var2_0_i64 = var14.wrapping_add(14114127202429895353) as i64;
                var5 = var20.wrapping_add({ let a = 7529619929231668594; let b = 7529619929231668595; if ((var14 as u64) < 4332616871279656263 as u64) as i32 != 0 { a } else { b } });
                slot_var2_8_i64 = var5 as i64;
                var5 = { let a = 5165552122434856866; let b = 5165552122434856867; if (var5 as u64 >= var20 as u64) as i32 != 0 { a } else { b } };
                var3 = var5.wrapping_add(var3);
                slot_var2_16_i64 = var3 as i64;
                slot_var2_24_i64 = ({ let a = 14959745806906580950; let b = 14959745806906580951; if (var3 as u64 >= var5 as u64) as i32 != 0 { a } else { b } }).wrapping_add(var8) as i64;
                break;
            }
            self.memory.store64(arg0 as usize, slot_var2_0_i64 as u64);
            let var54 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(24) as usize, var54 as u64);
            let var55 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(16) as usize, var55 as u64);
            let var56 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(8) as usize, var56 as u64);
            self.global0 = var2.wrapping_add(32);
            return;
            break;
        }
        self.func73(env);
        unreachable!();
    }
    fn func77(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        var1 = 0 /* False */;
        var2 = 24;
        'label0: loop {
            'label1: loop {
                if (var2 == -8) as i32 != 0 {
                    break 'label0;
                }
                var3 = arg0.wrapping_add(var2);
                let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                var4 = slot_var3_0_i64;
                slot_var3_0_i64 = ((var4 as u64).wrapping_shr(1 /* True */ as u32) as i64 | var1) as i64;
                var2 = var2.wrapping_add(-8);
                var1 = var4.wrapping_shl(63 as u32);
                continue 'label1;
                break;
            }
            break;
        }
    }
    fn func78(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i64 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.memory.load64(arg0 as usize) as i64;
        var1 = var5;
        var2 = var1.wrapping_add(4332616871279656263);
        self.memory.store64(arg0 as usize, var2 as u64);
        let var6 = self.memory.load64(arg0 as usize + 8) as i64;
        var3 = var6;
        var1 = var3.wrapping_add(((var2 as u64) < var1 as u64) as i32 as u32 as i64);
        var2 = var1.wrapping_add(10917124144477883021);
        self.memory.store64(arg0 as usize + 8, var2 as u64);
        let var7 = self.memory.load64(arg0 as usize + 16) as i64;
        var4 = var7;
        var1 = var4.wrapping_add((((var1 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var2 as u64) < var1 as u64) as i32 as u32 as i64));
        var2 = var1.wrapping_add(13281191951274694749);
        self.memory.store64(arg0 as usize + 16, var2 as u64);
        let var8 = self.memory.load64(arg0 as usize + 24) as i64;
        self.memory.store64(arg0 as usize + 24, var8.wrapping_add((((var1 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var2 as u64) < var1 as u64) as i32 as u32 as i64)).wrapping_add(3486998266802970665) as u64);
    }
    fn func79(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let var9 = self.memory.load64(arg0 as usize) as i64;
        var2 = var9;
        let var10 = self.memory.load64(arg1 as usize) as i64;
        var3 = var10;
        self.memory.store64(arg0 as usize, var2.wrapping_sub(var3) as u64);
        var2 = { let a = 18446744073709551615; let b = 0 /* False */; if ((var2 as u64) < var3 as u64) as i32 != 0 { a } else { b } };
        let var11 = self.memory.load64(arg1 as usize + 8) as i64;
        var3 = var11;
        var4 = var2.wrapping_sub(var3);
        let var12 = self.memory.load64(arg0 as usize + 8) as i64;
        var5 = var4.wrapping_add(var12);
        self.memory.store64(arg0 as usize + 8, var5 as u64);
        let var13 = self.memory.load64(arg0 as usize + 16) as i64;
        var6 = var13;
        let var14 = self.memory.load64(arg1 as usize + 16) as i64;
        var7 = var14;
        var8 = var6.wrapping_sub(var7);
        var2 = (var2.wrapping_sub(((var2 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var5 as u64) < var4 as u64) as i32 as u32 as i64) == 18446744073709551615) as i32 as u32 as i64;
        self.memory.store64(arg0 as usize + 16, var8.wrapping_sub(var2) as u64);
        let var15 = self.memory.load64(arg0 as usize + 24) as i64;
        let var16 = self.memory.load64(arg1 as usize + 24) as i64;
        self.memory.store64(arg0 as usize + 24, var15.wrapping_sub(var16).wrapping_sub(((((var6 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var2 as u64) as i32 as u32 as i64) == 1 /* True */) as i32 as u32 as i64) as u64);
    }
    fn func80(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        let var5 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
        self.memory.store64(var2.wrapping_add(24) as usize, var5 as u64);
        let var6 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
        self.memory.store64(var2.wrapping_add(16) as usize, var6 as u64);
        let var7 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
        self.memory.store64(var2.wrapping_add(8) as usize, var7 as u64);
        let var8 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_0_i64 = var8 as i64;
        'label0: loop {
            'label1: loop {
                let var9 = self.func29(env, arg1);
                if var9 != 0 {
                    break 'label1;
                }
                var3 = 0 /* False */;
                let var10 = self.func74(env, var2);
                if var10 != 0 {
                    break 'label0;
                }
                self.func70(env, var2, 1050752);
                arg1 = var2;
                break;
            }
            let var12 = self.memory.load64(arg1 as usize) as i64;
            self.memory.store64(arg0 as usize + 8, var12 as u64);
            let var13 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(32) as usize, var13 as u64);
            let var14 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(24) as usize, var14 as u64);
            let var15 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(16) as usize, var15 as u64);
            var3 = 1 /* True */;
            break;
        }
        self.memory.store64(arg0 as usize, var3 as u64);
        self.global0 = var2.wrapping_add(32);
    }
    fn func81(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
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
        let var25 = self.memory.load64(arg1 as usize + 24) as i64;
        var2 = var25;
        let var26 = self.memory.load64(arg1 as usize) as i64;
        var3 = var26;
        var4 = var3.wrapping_mul(9786893198990664585);
        var5 = var4 & 4294967295;
        var6 = var2.wrapping_add(var5.wrapping_mul(3778125865));
        let var27 = var6;
        var2 = (var4 as u64).wrapping_shr(32 as u32) as i64;
        var7 = var5.wrapping_mul(811880050);
        var4 = var7.wrapping_add(var2.wrapping_mul(3778125865));
        var4 = var6.wrapping_add(var4.wrapping_shl(32 as u32));
        var7 = var5.wrapping_mul(2172737629);
        let var28 = self.memory.load64(arg1 as usize + 16) as i64;
        var6 = var7.wrapping_add(var28);
        let var29 = var6;
        var8 = var5.wrapping_mul(3092268470);
        var7 = var8.wrapping_add(var2.wrapping_mul(2172737629));
        var7 = var6.wrapping_add(var7.wrapping_shl(32 as u32));
        var8 = var5.wrapping_mul(1752287885);
        let var30 = self.memory.load64(arg1 as usize + 8) as i64;
        var6 = var8.wrapping_add(var30);
        let var31 = var6;
        var9 = var5.wrapping_mul(2541841041);
        var8 = var9.wrapping_add(var2.wrapping_mul(1752287885));
        var8 = var6.wrapping_add(var8.wrapping_shl(32 as u32));
        var9 = var5.wrapping_mul(3632069959);
        var6 = var9.wrapping_add(var3);
        var3 = var5.wrapping_mul(1008765974);
        var5 = var3.wrapping_add(var2.wrapping_mul(3632069959));
        var6 = var8.wrapping_add((((var6 as u64) < var9 as u64) as i32 as u32 as i64 | var2.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var6 as u64) as i32 as u32 as i64));
        var8 = var7.wrapping_add((((var31 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(2541841041)).wrapping_add((((var8 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var6 as u64) < var8 as u64) as i32 as u32 as i64));
        var7 = var4.wrapping_add((((var29 as u64) < var7 as u64) as i32 as u32 as i64 | var2.wrapping_mul(3092268470)).wrapping_add((((var7 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var7 as u64) as i32 as u32 as i64));
        var10 = (((var27 as u64) < var2 as u64) as i32 as u32 as i64 | var2.wrapping_mul(811880050)).wrapping_add((((var4 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var4 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var4 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(((var7 as u64) < var4 as u64) as i32 as u32 as i64);
        var2 = var6.wrapping_mul(9786893198990664585);
        var5 = var2 & 4294967295;
        var3 = var10.wrapping_add(var5.wrapping_mul(3778125865));
        var11 = var5.wrapping_mul(811880050);
        var2 = (var2 as u64).wrapping_shr(32 as u32) as i64;
        var9 = var11.wrapping_add(var2.wrapping_mul(3778125865));
        var12 = var3.wrapping_add(var9.wrapping_shl(32 as u32));
        var4 = var7.wrapping_add(var5.wrapping_mul(2172737629));
        let var32 = var4;
        var13 = var5.wrapping_mul(3092268470);
        var7 = var13.wrapping_add(var2.wrapping_mul(2172737629));
        var7 = var4.wrapping_add(var7.wrapping_shl(32 as u32));
        var4 = var8.wrapping_add(var5.wrapping_mul(1752287885));
        let var33 = var4;
        var13 = var5.wrapping_mul(2541841041);
        var8 = var13.wrapping_add(var2.wrapping_mul(1752287885));
        var8 = var4.wrapping_add(var8.wrapping_shl(32 as u32));
        var4 = var6.wrapping_add(var5.wrapping_mul(3632069959));
        var6 = var5.wrapping_mul(1008765974);
        var5 = var6.wrapping_add(var2.wrapping_mul(3632069959));
        var4 = var8.wrapping_add((((var4 as u64) < var6 as u64) as i32 as u32 as i64 | var2.wrapping_mul(1008765974)).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var4.wrapping_add(var5.wrapping_shl(32 as u32)) as u64) < var4 as u64) as i32 as u32 as i64));
        var8 = var7.wrapping_add((((var33 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_mul(2541841041)).wrapping_add((((var8 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var4 as u64) < var8 as u64) as i32 as u32 as i64));
        var13 = var12.wrapping_add((((var32 as u64) < var7 as u64) as i32 as u32 as i64 | var2.wrapping_mul(3092268470)).wrapping_add((((var7 as u64) < var13 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var7 as u64) as i32 as u32 as i64));
        var6 = var4.wrapping_mul(9786893198990664585);
        var5 = var6 & 4294967295;
        var14 = var13.wrapping_add(var5.wrapping_mul(2172737629));
        var15 = var5.wrapping_mul(3092268470);
        var6 = (var6 as u64).wrapping_shr(32 as u32) as i64;
        var16 = var15.wrapping_add(var6.wrapping_mul(2172737629));
        var17 = var14.wrapping_add(var16.wrapping_shl(32 as u32));
        var7 = var8.wrapping_add(var5.wrapping_mul(1752287885));
        let var34 = var7;
        var18 = var5.wrapping_mul(2541841041);
        var8 = var18.wrapping_add(var6.wrapping_mul(1752287885));
        var18 = var7.wrapping_add(var8.wrapping_shl(32 as u32));
        var7 = var4.wrapping_add(var5.wrapping_mul(3632069959));
        var8 = var5.wrapping_mul(1008765974);
        var4 = var8.wrapping_add(var6.wrapping_mul(3632069959));
        var8 = var18.wrapping_add((((var7 as u64) < var4 as u64) as i32 as u32 as i64 | var6.wrapping_mul(1008765974)).wrapping_add((((var4 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var4 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var7.wrapping_add(var4.wrapping_shl(32 as u32)) as u64) < var7 as u64) as i32 as u32 as i64));
        var18 = var17.wrapping_add((((var34 as u64) < var8 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_mul(2541841041)).wrapping_add((((var8 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var18 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(((var8 as u64) < var18 as u64) as i32 as u32 as i64));
        var7 = var8.wrapping_mul(9786893198990664585);
        var4 = var7 & 4294967295;
        var19 = var18.wrapping_add(var4.wrapping_mul(1752287885));
        var20 = var4.wrapping_mul(2541841041);
        var7 = (var7 as u64).wrapping_shr(32 as u32) as i64;
        var21 = var20.wrapping_add(var7.wrapping_mul(1752287885));
        var22 = var19.wrapping_add(var21.wrapping_shl(32 as u32));
        var23 = var8.wrapping_add(var4.wrapping_mul(3632069959));
        var24 = var4.wrapping_mul(1008765974);
        var8 = var24.wrapping_add(var7.wrapping_mul(3632069959));
        var23 = var22.wrapping_add((((var23 as u64) < var8 as u64) as i32 as u32 as i64 | var7.wrapping_mul(1008765974)).wrapping_add((((var8 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var8 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var23.wrapping_add(var8.wrapping_shl(32 as u32)) as u64) < var23 as u64) as i32 as u32 as i64));
        self.memory.store64(arg0 as usize, var23 as u64);
        var10 = (((var3 as u64) < var10 as u64) as i32 as u32 as i64 | var2.wrapping_mul(811880050)).wrapping_add((((var9 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var9 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var12 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(((var13 as u64) < var12 as u64) as i32 as u32 as i64);
        var2 = var10.wrapping_add(var5.wrapping_mul(3778125865));
        var11 = var5.wrapping_mul(811880050);
        var5 = var11.wrapping_add(var6.wrapping_mul(3778125865));
        var8 = var2.wrapping_add(var5.wrapping_shl(32 as u32));
        var3 = var8.wrapping_add((((var14 as u64) < var13 as u64) as i32 as u32 as i64 | var6.wrapping_mul(3092268470)).wrapping_add((((var16 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var16 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var17 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_add(((var18 as u64) < var17 as u64) as i32 as u32 as i64));
        var9 = var3.wrapping_add(var4.wrapping_mul(2172737629));
        var14 = var4.wrapping_mul(3092268470);
        var12 = var14.wrapping_add(var7.wrapping_mul(2172737629));
        var13 = var9.wrapping_add(var12.wrapping_shl(32 as u32));
        var16 = var13.wrapping_add((((var19 as u64) < var18 as u64) as i32 as u32 as i64).wrapping_add(var7.wrapping_mul(2541841041)).wrapping_add((((var21 as u64) < var20 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var21 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var22 as u64) < var19 as u64) as i32 as u32 as i64).wrapping_add(((var23 as u64) < var22 as u64) as i32 as u32 as i64));
        self.memory.store64(arg0 as usize + 8, var16 as u64);
        var8 = (((var2 as u64) < var10 as u64) as i32 as u32 as i64 | var6.wrapping_mul(811880050)).wrapping_add((((var5 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var8 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var8 as u64) as i32 as u32 as i64);
        var5 = var8.wrapping_add(var4.wrapping_mul(3778125865));
        var4 = var4.wrapping_mul(811880050);
        var2 = var4.wrapping_add(var7.wrapping_mul(3778125865));
        var6 = var5.wrapping_add(var2.wrapping_shl(32 as u32));
        var3 = var6.wrapping_add((((var9 as u64) < var3 as u64) as i32 as u32 as i64 | var7.wrapping_mul(3092268470)).wrapping_add((((var12 as u64) < var14 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var12 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var13 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(((var16 as u64) < var13 as u64) as i32 as u32 as i64));
        self.memory.store64(arg0 as usize + 16, var3 as u64);
        self.memory.store64(arg0 as usize + 24, (((var5 as u64) < var8 as u64) as i32 as u32 as i64 | var7.wrapping_mul(811880050)).wrapping_add((((var2 as u64) < var4 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var2 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var6 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_add(((var3 as u64) < var6 as u64) as i32 as u32 as i64) as u64);
    }
    fn func82(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var2 = var9.wrapping_sub(192);
        self.global0 = var2;
        self.func71(env, var2.wrapping_add(15));
        var3 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            if (var3 != 24) as i32 != 0 {
                                break 'label4;
                            }
                            self.func53(env, var2.wrapping_add(128), arg1, var2.wrapping_add(39));
                            'label5: loop {
                                let var12 = self.memory.load8(var2 as usize + 128) as i32;
                                if (var12 == 2) as i32 != 0 {
                                    break 'label5;
                                }
                                let mut slot_var2_128_i64 = self.memory.load64(var2 as usize + 128) as i64;
                                var4 = slot_var2_128_i64;
                                if (var4 & 255 != 0 /* Void */) as i32 != 0 {
                                    break 'label1;
                                }
                                break;
                            }
                            self.func48(env, 4, var2);
                            var4 = 0 /* False */;
                            var3 = var2.wrapping_add(80).wrapping_add(24);
                            let mut slot_var3_0_i64 = 0 /* False */ as i64;
                            arg1 = var2.wrapping_add(80).wrapping_add(16);
                            self.memory.store64(arg1 as usize, 0 /* False */ as u64);
                            var5 = var2.wrapping_add(80).wrapping_add(8);
                            let mut slot_var5_0_i64 = 0 /* False */ as i64;
                            let mut slot_var2_80_i64 = 0 /* False */ as i64;
                            let var14 = self.memory.load64(var2.wrapping_add(15).wrapping_add(24) as usize) as i64;
                            self.memory.store64(var2.wrapping_add(168) as usize, var14 as u64);
                            var6 = var2.wrapping_add(128).wrapping_add(32);
                            let var15 = self.memory.load64(var2.wrapping_add(15).wrapping_add(16) as usize) as i64;
                            let mut slot_var6_0_i64 = var15 as i64;
                            var7 = var2.wrapping_add(128).wrapping_add(24);
                            let var16 = self.memory.load64(var2.wrapping_add(15).wrapping_add(8) as usize) as i64;
                            let mut slot_var7_0_i64 = var16 as i64;
                            let mut slot_var2_15_i64 = self.memory.load64(var2 as usize + 15) as i64;
                            let mut slot_var2_144_i64 = slot_var2_15_i64 as i64;
                            let mut slot_var2_184_i32 = 0 as i32;
                            let mut slot_var2_176_i64 = 0 /* False */ as i64;
                            let mut slot_var2_136_i64 = 17179869184 as i64;
                            var8 = var2.wrapping_add(80).wrapping_add(32);
                            let mut slot_var2_132_i32 = var8 as i32;
                            let mut slot_var2_128_i32 = var2.wrapping_add(80) as i32;
                            self.func49(env, var2.wrapping_add(128));
                            self.memory.store64(var2.wrapping_add(48).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                            let var18 = self.memory.load64(arg1 as usize) as i64;
                            self.memory.store64(var2.wrapping_add(48).wrapping_add(16) as usize, var18 as u64);
                            self.memory.store64(var2.wrapping_add(48).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                            let mut slot_var2_48_i64 = slot_var2_80_i64 as i64;
                            self.func80(env, var2.wrapping_add(128), var2.wrapping_add(48));
                            'label6: loop {
                                if (slot_var2_128_i32 != 1) as i32 != 0 {
                                    break 'label6;
                                }
                                let mut slot_var8_0_i64 = slot_var6_0_i64 as i64;
                                slot_var3_0_i64 = slot_var7_0_i64 as i64;
                                let var20 = self.memory.load64(var2.wrapping_add(128).wrapping_add(16) as usize) as i64;
                                self.memory.store64(arg1 as usize, var20 as u64);
                                let mut slot_var2_88_i64 = slot_var2_136_i64 as i64;
                                var4 = 1 /* True */;
                                break;
                            }
                            slot_var2_80_i64 = var4 as i64;
                            self.memory.store8(var2 as usize + 120, 3 as u8);
                            self.func52(env, var2.wrapping_add(128), var2.wrapping_add(80), var2.wrapping_add(120));
                            if (slot_var2_128_i32 & 1 == 0) as i32 != 0 {
                                break 'label2;
                            }
                            var4 = slot_var2_132_i32;
                            break 'label1;
                            break;
                        }
                        self.func53(env, var2.wrapping_add(128), arg1, var2.wrapping_add(15).wrapping_add(var3));
                        'label7: loop {
                            let var23 = self.memory.load8(var2 as usize + 128) as i32;
                            if (var23 == 2) as i32 != 0 {
                                break 'label7;
                            }
                            var4 = slot_var2_128_i64;
                            if (var4 & 255 != 0 /* Void */) as i32 != 0 {
                                break 'label1;
                            }
                            break;
                        }
                        var3 = var3.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                self.memory.store64(arg0 as usize + 8, slot_var2_136_i64 as u64);
                let var24 = self.memory.load64(var2.wrapping_add(128).wrapping_add(32) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(32) as usize, var24 as u64);
                let var25 = self.memory.load64(var2.wrapping_add(128).wrapping_add(24) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(24) as usize, var25 as u64);
                let var26 = self.memory.load64(var2.wrapping_add(128).wrapping_add(16) as usize) as i64;
                self.memory.store64(arg0.wrapping_add(16) as usize, var26 as u64);
                var3 = 0;
                break 'label0;
                break;
            }
            self.memory.store64(arg0 as usize + 4, var4 as u64);
            var3 = 1;
            break;
        }
        self.memory.store32(arg0 as usize, var3 as u32);
        self.global0 = var2.wrapping_add(192);
    }
    fn func83(&mut self, env: &Env, mut proof: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i64 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let var20 = self.global0;
        var1 = var20.wrapping_sub(9600);
        self.global0 = var1;
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 16) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var1.wrapping_add(1856).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var21 = 0 /* TODO: map_unpack_to_linear_memory */
                var21;
                let mut slot_var1_1856_i64 = self.memory.load64(var1 as usize + 1856) as i64;
                proof = slot_var1_1856_i64;
                if (!(Bytes::try_from_val(env, &val_from_i64(proof)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var22 = Bytes::from_val(env, &val_from_i64(proof)).len() as i64
                if (var22 & 18446744069414584320 != 274877906944) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_1864_i64 = self.memory.load64(var1 as usize + 1864) as i64;
                var3 = slot_var1_1864_i64;
                if (!(Bytes::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var23 = Bytes::from_val(env, &val_from_i64(var3)).len() as i64
                if (var23 & 18446744069414584320 != 549755813888) as i32 != 0 {
                    break 'label3;
                }
                var2 = 0;
                let var24 = self.func122(env, var1.wrapping_add(8), 0, 64);
                var24;
                self.func84(env, proof, var1.wrapping_add(8), 64);
                let mut slot_var1_7300_i32 = 64 as i32;
                let mut slot_var1_7296_i32 = var1.wrapping_add(8) as i32;
                self.func71(env, var1.wrapping_add(8832));
                'label4: loop {
                    'label5: loop {
                        if (var2 != 24) as i32 != 0 {
                            break 'label5;
                        }
                        self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8856));
                        'label6: loop {
                            let var28 = self.memory.load8(var1 as usize + 1856) as i32;
                            if (var28 == 2) as i32 != 0 {
                                break 'label6;
                            }
                            let var29 = self.memory.load8(var1 as usize + 1856) as i64;
                            if (var29 != 0 /* Void */) as i32 != 0 {
                                break 'label2;
                            }
                            break;
                        }
                        self.func48(env, 4, var2);
                        proof = 0 /* False */;
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        let mut slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        let mut slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        let mut slot_var5_0_i64 = 0 /* False */ as i64;
                        let mut slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var31 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var31 as u64);
                        var6 = var1.wrapping_add(1856).wrapping_add(32);
                        let var32 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var6_0_i64 = var32 as i64;
                        var7 = var1.wrapping_add(1856).wrapping_add(24);
                        let var33 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var7_0_i64 = var33 as i64;
                        let mut slot_var1_8832_i64 = self.memory.load64(var1 as usize + 8832) as i64;
                        let mut slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        let mut slot_var1_1912_i32 = 0 as i32;
                        let mut slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        var8 = var1.wrapping_add(1088).wrapping_add(32);
                        let mut slot_var1_1860_i32 = var8 as i32;
                        let mut slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        let mut slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        'label7: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label7;
                            }
                            let mut slot_var8_0_i64 = slot_var6_0_i64 as i64;
                            slot_var2_0_i64 = slot_var7_0_i64 as i64;
                            let var36 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var4_0_i64 = var36 as i64;
                            let mut slot_var1_1096_i64 = slot_var1_1864_i64 as i64;
                            proof = 1 /* True */;
                            break;
                        }
                        slot_var1_1088_i64 = proof as i64;
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func52(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let mut slot_var1_9216_i32 = self.memory.load32(var1 as usize + 9216) as i32;
                        if slot_var1_9216_i32 & 1 != 0 {
                            break 'label2;
                        }
                        let var38 = self.memory.load64(var1.wrapping_add(9236) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8064).wrapping_add(8) as usize, var38 as u64);
                        let var39 = self.memory.load64(var1.wrapping_add(9244) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8080) as usize, var39 as u64);
                        let var40 = self.memory.load32(var1.wrapping_add(9252) as usize) as i32;
                        self.memory.store32(var1.wrapping_add(8064).wrapping_add(24) as usize, var40 as u32);
                        let mut slot_var1_9228_i64 = self.memory.load64(var1 as usize + 9228) as i64;
                        let mut slot_var1_8064_i64 = slot_var1_9228_i64 as i64;
                        let mut slot_var1_9224_i32 = self.memory.load32(var1 as usize + 9224) as i32;
                        var7 = slot_var1_9224_i32;
                        self.func71(env, var1.wrapping_add(8832));
                        var2 = 0;
                        'label8: loop {
                            'label9: loop {
                                'label10: loop {
                                    if (var2 != 24) as i32 != 0 {
                                        break 'label10;
                                    }
                                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8856));
                                    let var43 = self.memory.load8(var1 as usize + 1856) as i32;
                                    if (var43 == 2) as i32 != 0 {
                                        break 'label8;
                                    }
                                    let var44 = self.memory.load8(var1 as usize + 1856) as i64;
                                    if (var44 == 0 /* Void */) as i32 != 0 {
                                        break 'label8;
                                    }
                                    break 'label2;
                                    break;
                                }
                                self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8832).wrapping_add(var2));
                                'label11: loop {
                                    let var46 = self.memory.load8(var1 as usize + 1856) as i32;
                                    if (var46 == 2) as i32 != 0 {
                                        break 'label11;
                                    }
                                    let var47 = self.memory.load8(var1 as usize + 1856) as i64;
                                    if (var47 != 0 /* Void */) as i32 != 0 {
                                        break 'label2;
                                    }
                                    break;
                                }
                                var2 = var2.wrapping_add(8);
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var48 = self.func40(env, var1.wrapping_add(8863));
                        var2 = var48;
                        self.memory.store8(var1 as usize + 1856, 4 as u8);
                        self.func50(env, var1.wrapping_add(1088), var2, var1.wrapping_add(1856));
                        'label12: loop {
                            'label13: loop {
                                let var50 = self.memory.load8(var1 as usize + 1088) as i32;
                                if (var50 != 5) as i32 != 0 {
                                    break 'label13;
                                }
                                let var51 = self.memory.load8(var1 as usize + 1089) as i32;
                                var8 = var51;
                                break 'label12;
                                break;
                            }
                            proof = slot_var1_1088_i64;
                            if (proof & 255 != 0) as i32 != 0 {
                                break 'label2;
                            }
                            var8 = (proof as u64).wrapping_shr(0 as u32) as i64 as i32;
                            break;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var52 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var52 as u64);
                        var9 = var1.wrapping_add(1856).wrapping_add(32);
                        let var53 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var9_0_i64 = var53 as i64;
                        var10 = var1.wrapping_add(1856).wrapping_add(24);
                        let var54 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var10_0_i64 = var54 as i64;
                        slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        slot_var1_1912_i32 = 0 as i32;
                        slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        slot_var1_1860_i32 = var1.wrapping_add(1088).wrapping_add(32) as i32;
                        slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        var6 = 129;
                        'label14: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label14;
                            }
                            slot_var2_0_i64 = slot_var9_0_i64 as i64;
                            slot_var4_0_i64 = slot_var10_0_i64 as i64;
                            let var57 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var5_0_i64 = var57 as i64;
                            slot_var1_1088_i64 = slot_var1_1864_i64 as i64;
                            var6 = var8;
                            break;
                        }
                        self.memory.store8(var1 as usize + 1120, var6 as u8);
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func51(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let var59 = self.memory.load8(var1 as usize + 9248) as i32;
                        var2 = var59;
                        if (var2 == 129) as i32 != 0 {
                            break 'label2;
                        }
                        proof = slot_var1_9216_i32;
                        var4 = var1.wrapping_add(7680).wrapping_add(16);
                        var5 = var1.wrapping_add(9216).wrapping_add(24);
                        slot_var4_0_i64 = slot_var5_0_i64 as i64;
                        var6 = var1.wrapping_add(7680).wrapping_add(8);
                        var8 = var1.wrapping_add(9216).wrapping_add(16);
                        slot_var6_0_i64 = slot_var8_0_i64 as i64;
                        let mut slot_var1_7680_i64 = slot_var1_9224_i32 as i64;
                        'label15: loop {
                            'label16: loop {
                                if (var2 == 64) as i32 != 0 {
                                    break 'label16;
                                }
                                let var60 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(1868) as usize, var60 as u64);
                                let var61 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(1876) as usize, var61 as u64);
                                let var62 = self.memory.load32(var1.wrapping_add(8064).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(1884) as usize, var62 as u32);
                                self.memory.store64(var1.wrapping_add(1904) as usize, slot_var6_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(1912) as usize, slot_var4_0_i64 as u64);
                                slot_var1_1856_i32 = var7 as i32;
                                let mut slot_var1_1860_i64 = slot_var1_8064_i64 as i64;
                                let mut slot_var1_1888_i64 = proof as i64;
                                let mut slot_var1_1896_i64 = slot_var1_7680_i64 as i64;
                                self.memory.store8(var1 as usize + 1920, 0 as u8);
                                self.func76(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                self.func70(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                let var65 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(24) as usize) as i64;
                                slot_var5_0_i64 = var65 as i64;
                                let var66 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(16) as usize) as i64;
                                slot_var8_0_i64 = var66 as i64;
                                let var67 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(9216).wrapping_add(8) as usize, var67 as u64);
                                let mut slot_var1_9216_i64 = slot_var1_1088_i64 as i64;
                                'label17: loop {
                                    let var68 = self.func29(env, 1050656);
                                    if var68 != 0 {
                                        break 'label17;
                                    }
                                    self.func13(env, var1.wrapping_add(9216), 1050656);
                                    break;
                                }
                                var2 = var1.wrapping_add(1888);
                                let var70 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(24) as usize, var70 as u64);
                                let var71 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(16) as usize, var71 as u64);
                                let var72 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8832).wrapping_add(8) as usize, var72 as u64);
                                slot_var1_8832_i64 = slot_var1_9216_i64 as i64;
                                var4 = var1.wrapping_add(1088).wrapping_add(24);
                                slot_var4_0_i64 = 0 /* False */ as i64;
                                var5 = var1.wrapping_add(1088).wrapping_add(16);
                                slot_var5_0_i64 = 0 /* False */ as i64;
                                var6 = var1.wrapping_add(1088).wrapping_add(8);
                                slot_var6_0_i64 = 0 /* False */ as i64;
                                slot_var1_1088_i64 = 0 /* False */ as i64;
                                'label18: loop {
                                    let var73 = self.func29(env, var1.wrapping_add(1088));
                                    if var73 != 0 {
                                        break 'label18;
                                    }
                                    slot_var4_0_i64 = 0 /* False */ as i64;
                                    slot_var5_0_i64 = 0 /* False */ as i64;
                                    slot_var6_0_i64 = 0 /* False */ as i64;
                                    slot_var1_1088_i64 = 0 /* False */ as i64;
                                    self.func13(env, var1.wrapping_add(8832), var1.wrapping_add(1088));
                                    break;
                                }
                                self.func76(env, var1.wrapping_add(1088), var2);
                                let var76 = self.func44(env, var1.wrapping_add(1088), var1.wrapping_add(8832));
                                if (var76 == 0) as i32 != 0 {
                                    break 'label2;
                                }
                                proof = slot_var1_1856_i64;
                                let var77 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1864), 56);
                                var77;
                                var9 = 0;
                                break 'label15;
                                break;
                            }
                            let var78 = self.func122(env, var1.wrapping_add(1088), 0, 56);
                            var78;
                            proof = 0 /* False */;
                            var9 = 1;
                            break;
                        }
                        let var79 = self.func121(env, var1.wrapping_add(80), var1.wrapping_add(1088), 56);
                        var79;
                        let var80 = self.memory.load32(var1.wrapping_add(1859) as usize) as i32;
                        let mut slot_var1_75_i32 = var80 as i32;
                        let mut slot_var1_72_i32 = slot_var1_1856_i32 as i32;
                        let var81 = self.func122(env, var1.wrapping_add(136), 0, 128);
                        var81;
                        self.func84(env, var3, var1.wrapping_add(136), 128);
                        let mut slot_var1_652_i32 = 128 as i32;
                        let mut slot_var1_648_i32 = var1.wrapping_add(136) as i32;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(8);
                        var4 = var1.wrapping_add(1876);
                        slot_var2_0_i64 = slot_var4_0_i64 as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(16);
                        var6 = var1.wrapping_add(1884);
                        slot_var5_0_i64 = slot_var6_0_i64 as i64;
                        var7 = var1.wrapping_add(1088).wrapping_add(24);
                        var8 = var1.wrapping_add(1892);
                        let mut slot_var7_0_i32 = slot_var8_0_i64 as i32;
                        let mut slot_var1_1868_i64 = self.memory.load64(var1 as usize + 1868) as i64;
                        slot_var1_1088_i64 = slot_var1_1868_i64 as i64;
                        var10 = slot_var1_1864_i64;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(264).wrapping_add(8) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(264).wrapping_add(16) as usize, slot_var6_0_i64 as u64);
                        self.memory.store32(var1.wrapping_add(264).wrapping_add(24) as usize, slot_var8_0_i64 as u32);
                        self.memory.store64(var1.wrapping_add(2624).wrapping_add(8) as usize, slot_var2_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(2624).wrapping_add(16) as usize, slot_var5_0_i64 as u64);
                        self.memory.store32(var1.wrapping_add(2624).wrapping_add(24) as usize, slot_var7_0_i32 as u32);
                        let mut slot_var1_264_i64 = slot_var1_1868_i64 as i64;
                        let mut slot_var1_2624_i64 = slot_var1_1088_i64 as i64;
                        var7 = slot_var1_1864_i64;
                        self.func82(env, var1.wrapping_add(1856), var1.wrapping_add(648));
                        if (slot_var1_1856_i32 == 1) as i32 != 0 {
                            break 'label2;
                        }
                        let var86 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8064).wrapping_add(8) as usize, var86 as u64);
                        let var87 = self.memory.load64(var1.wrapping_add(1888) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(8080) as usize, var87 as u64);
                        slot_var1_8064_i64 = slot_var1_1872_i64 as i64;
                        var3 = slot_var1_1864_i64;
                        self.func71(env, var1.wrapping_add(8832));
                        var2 = 0;
                        'label19: loop {
                            'label20: loop {
                                'label21: loop {
                                    'label22: loop {
                                        if (var2 != 24) as i32 != 0 {
                                            break 'label22;
                                        }
                                        self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(648), var1.wrapping_add(8856));
                                        'label23: loop {
                                            let var90 = self.memory.load8(var1 as usize + 1856) as i32;
                                            if (var90 == 2) as i32 != 0 {
                                                break 'label23;
                                            }
                                            let var91 = self.memory.load8(var1 as usize + 1856) as i64;
                                            if (var91 != 0 /* Void */) as i32 != 0 {
                                                break 'label2;
                                            }
                                            break;
                                        }
                                        let var92 = self.func40(env, var1.wrapping_add(8863));
                                        var2 = var92;
                                        self.memory.store8(var1 as usize + 1856, 4 as u8);
                                        self.func50(env, var1.wrapping_add(1088), var2, var1.wrapping_add(1856));
                                        let var94 = self.memory.load8(var1 as usize + 1088) as i32;
                                        if (var94 != 5) as i32 != 0 {
                                            break 'label20;
                                        }
                                        let var95 = self.memory.load8(var1 as usize + 1089) as i32;
                                        var8 = var95;
                                        break 'label19;
                                        break;
                                    }
                                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(648), var1.wrapping_add(8832).wrapping_add(var2));
                                    'label24: loop {
                                        let var97 = self.memory.load8(var1 as usize + 1856) as i32;
                                        if (var97 == 2) as i32 != 0 {
                                            break 'label24;
                                        }
                                        let var98 = self.memory.load8(var1 as usize + 1856) as i64;
                                        if (var98 != 0 /* Void */) as i32 != 0 {
                                            break 'label2;
                                        }
                                        break;
                                    }
                                    var2 = var2.wrapping_add(8);
                                    continue 'label21;
                                    break;
                                }
                                break;
                            }
                            var11 = slot_var1_1088_i64;
                            if (var11 & 255 != 0) as i32 != 0 {
                                break 'label2;
                            }
                            var8 = (var11 as u64).wrapping_shr(0 as u32) as i64 as i32;
                            break;
                        }
                        var2 = var1.wrapping_add(1088).wrapping_add(24);
                        slot_var2_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(1088).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(1088).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_1088_i64 = 0 /* False */ as i64;
                        let var99 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(1896) as usize, var99 as u64);
                        var12 = var1.wrapping_add(1856).wrapping_add(32);
                        let var100 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(16) as usize) as i64;
                        let mut slot_var12_0_i64 = var100 as i64;
                        var13 = var1.wrapping_add(1856).wrapping_add(24);
                        let var101 = self.memory.load64(var1.wrapping_add(8832).wrapping_add(8) as usize) as i64;
                        let mut slot_var13_0_i64 = var101 as i64;
                        slot_var1_1872_i64 = slot_var1_8832_i64 as i64;
                        slot_var1_1912_i32 = 0 as i32;
                        slot_var1_1904_i64 = 0 /* False */ as i64;
                        slot_var1_1864_i64 = 17179869184 as i64;
                        slot_var1_1860_i32 = var1.wrapping_add(1088).wrapping_add(32) as i32;
                        slot_var1_1856_i32 = var1.wrapping_add(1088) as i32;
                        self.func49(env, var1.wrapping_add(1856));
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(8448).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                        slot_var1_8448_i64 = slot_var1_1088_i64 as i64;
                        self.func80(env, var1.wrapping_add(1856), var1.wrapping_add(8448));
                        var6 = 129;
                        'label25: loop {
                            if (slot_var1_1856_i32 != 1) as i32 != 0 {
                                break 'label25;
                            }
                            slot_var2_0_i64 = slot_var12_0_i64 as i64;
                            slot_var4_0_i64 = slot_var13_0_i64 as i64;
                            let var104 = self.memory.load64(var1.wrapping_add(1856).wrapping_add(16) as usize) as i64;
                            slot_var5_0_i64 = var104 as i64;
                            slot_var1_1088_i64 = slot_var1_1864_i64 as i64;
                            var6 = var8;
                            break;
                        }
                        self.memory.store8(var1 as usize + 1120, var6 as u8);
                        self.memory.store8(var1 as usize + 1856, 3 as u8);
                        self.func51(env, var1.wrapping_add(9216), var1.wrapping_add(1088), var1.wrapping_add(1856));
                        let var106 = self.memory.load8(var1 as usize + 9248) as i32;
                        var2 = var106;
                        if (var2 == 129) as i32 != 0 {
                            break 'label2;
                        }
                        var11 = slot_var1_9216_i64;
                        var4 = var1.wrapping_add(656).wrapping_add(16);
                        let var107 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                        slot_var4_0_i64 = var107 as i64;
                        var5 = var1.wrapping_add(656).wrapping_add(8);
                        let var108 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                        slot_var5_0_i64 = var108 as i64;
                        var6 = var1.wrapping_add(680).wrapping_add(8);
                        let var109 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(8) as usize) as i64;
                        slot_var6_0_i64 = var109 as i64;
                        var8 = var1.wrapping_add(680).wrapping_add(16);
                        let var110 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(16) as usize) as i64;
                        slot_var8_0_i64 = var110 as i64;
                        let mut slot_var1_656_i64 = slot_var1_9224_i32 as i64;
                        let mut slot_var1_680_i64 = slot_var1_8064_i64 as i64;
                        'label26: loop {
                            'label27: loop {
                                if (var2 == 64) as i32 != 0 {
                                    break 'label27;
                                }
                                let var111 = self.memory.load64(var1.wrapping_add(2624).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8844) as usize, var111 as u64);
                                let var112 = self.memory.load64(var1.wrapping_add(2624).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8852) as usize, var112 as u64);
                                let var113 = self.memory.load32(var1.wrapping_add(2624).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(8860) as usize, var113 as u32);
                                let var114 = self.memory.load64(var1.wrapping_add(264).wrapping_add(8) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8876) as usize, var114 as u64);
                                let var115 = self.memory.load64(var1.wrapping_add(264).wrapping_add(16) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(8884) as usize, var115 as u64);
                                let var116 = self.memory.load32(var1.wrapping_add(264).wrapping_add(24) as usize) as i32;
                                self.memory.store32(var1.wrapping_add(8892) as usize, var116 as u32);
                                let mut slot_var1_8832_i32 = var10 as i32;
                                let mut slot_var1_8836_i64 = slot_var1_2624_i64 as i64;
                                let mut slot_var1_8864_i32 = var7 as i32;
                                let mut slot_var1_8868_i64 = slot_var1_264_i64 as i64;
                                self.memory.store64(var1.wrapping_add(8912) as usize, slot_var6_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8920) as usize, slot_var8_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8944) as usize, slot_var5_0_i64 as u64);
                                self.memory.store64(var1.wrapping_add(8952) as usize, slot_var4_0_i64 as u64);
                                let mut slot_var1_8896_i64 = var3 as i64;
                                let mut slot_var1_8928_i64 = var11 as i64;
                                self.memory.store8(var1 as usize + 8960, 0 as u8);
                                let mut slot_var1_8904_i64 = slot_var1_680_i64 as i64;
                                let mut slot_var1_8936_i64 = slot_var1_656_i64 as i64;
                                self.func7(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                self.func5(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                let var119 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 64);
                                var119;
                                'label28: loop {
                                    let var120 = self.func27(env, 1050688);
                                    if var120 != 0 {
                                        break 'label28;
                                    }
                                    self.func10(env, var1.wrapping_add(1088), 1050688);
                                    break;
                                }
                                var7 = var1.wrapping_add(8832).wrapping_add(64);
                                let var122 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1088), 64);
                                var122;
                                let var123 = self.func122(env, var1.wrapping_add(1856), 0, 64);
                                var123;
                                'label29: loop {
                                    let var124 = self.func27(env, var1.wrapping_add(1856));
                                    if var124 != 0 {
                                        break 'label29;
                                    }
                                    let var125 = self.func122(env, var1.wrapping_add(1856), 0, 64);
                                    var125;
                                    self.func10(env, var1.wrapping_add(9216), var1.wrapping_add(1856));
                                    break;
                                }
                                self.func7(env, var1.wrapping_add(1856), var7);
                                let var128 = self.func16(env, var1.wrapping_add(1856), var1.wrapping_add(9216));
                                if (var128 == 0) as i32 != 0 {
                                    break 'label2;
                                }
                                self.func85(env, var1.wrapping_add(1856));
                                var8 = var1.wrapping_add(1120);
                                var13 = var1.wrapping_add(1088).wrapping_add(64);
                                var14 = var1.wrapping_add(8448).wrapping_add(64);
                                var12 = var1.wrapping_add(1856).wrapping_add(64);
                                var10 = var1.wrapping_add(1856).wrapping_add(128);
                                var4 = 128;
                                var6 = 0;
                                'label30: loop {
                                    var2 = var4;
                                    'label31: loop {
                                        'label32: loop {
                                            'label33: loop {
                                                'label34: loop {
                                                    if (var2 == 0) as i32 != 0 {
                                                        break 'label33;
                                                    }
                                                    var4 = var2.wrapping_add(-1);
                                                    var5 = (var4 as u32).wrapping_shr(6 as u32) as i32;
                                                    if (var2 as u32 >= 129 as u32) as i32 != 0 {
                                                        break 'label32;
                                                    }
                                                    var2 = var4;
                                                    let var130 = self.memory.load64(var5.wrapping_shl(3 as u32).wrapping_add(1051448) as usize) as i64;
                                                    var5 = (var130 as u64).wrapping_shr((var4 & 63) as u32 as i64 as u32) as i64 as i32;
                                                    if ((var6 | var5) & 1 == 0) as i32 != 0 {
                                                        continue 'label34;
                                                    }
                                                    break;
                                                }
                                                var6 = 1;
                                                let var131 = self.func86(env, var1.wrapping_add(1856));
                                                var2 = var131;
                                                if (var5 & 1 == 0) as i32 != 0 {
                                                    continue 'label30;
                                                }
                                                let var132 = self.func87(env, var2);
                                                if var132 != 0 {
                                                    break 'label31;
                                                }
                                                let var133 = self.func121(env, var1.wrapping_add(3840), var10, 64);
                                                var133;
                                                let var134 = self.func88(env, var1.wrapping_add(3840));
                                                var5 = var134;
                                                let var135 = self.func121(env, var1.wrapping_add(4224), var1.wrapping_add(8832), 64);
                                                var135;
                                                self.func89(env, var1.wrapping_add(4224), var5);
                                                let var137 = self.func121(env, var1.wrapping_add(4608), var10, 64);
                                                var137;
                                                self.func89(env, var1.wrapping_add(4608), var7);
                                                self.func89(env, var1.wrapping_add(4608), var5);
                                                'label35: loop {
                                                    let var140 = self.func90(env, var2, var1.wrapping_add(4224));
                                                    if var140 != 0 {
                                                        break 'label35;
                                                    }
                                                    let var141 = self.func121(env, var1.wrapping_add(4992), var1.wrapping_add(4224), 64);
                                                    var141;
                                                    self.func9(env, var1.wrapping_add(4992), var2);
                                                    let var143 = self.func121(env, var1.wrapping_add(5376), var1.wrapping_add(4992), 64);
                                                    var143;
                                                    let var144 = self.func88(env, var1.wrapping_add(5376));
                                                    let var145 = self.func121(env, var1.wrapping_add(5760), var144, 64);
                                                    var145;
                                                    let var146 = self.func11(env, var1.wrapping_add(5760));
                                                    let var147 = self.func11(env, var146);
                                                    var5 = var147;
                                                    let var148 = self.func121(env, var1.wrapping_add(6144), var1.wrapping_add(4992), 64);
                                                    var148;
                                                    let var149 = self.func15(env, var1.wrapping_add(6144));
                                                    var15 = var149;
                                                    self.func89(env, var15, var5);
                                                    let var151 = self.func121(env, var1.wrapping_add(6528), var1.wrapping_add(4608), 64);
                                                    var151;
                                                    self.func9(env, var1.wrapping_add(6528), var12);
                                                    let var153 = self.func11(env, var1.wrapping_add(6528));
                                                    var16 = var153;
                                                    let var154 = self.func121(env, var1.wrapping_add(6912), var2, 64);
                                                    var154;
                                                    self.func89(env, var1.wrapping_add(6912), var5);
                                                    self.func7(env, var2, var16);
                                                    self.func10(env, var2, var15);
                                                    self.func8(env, var1.wrapping_add(1088), var1.wrapping_add(6912));
                                                    self.func9(env, var2, var1.wrapping_add(1088));
                                                    self.func9(env, var1.wrapping_add(6912), var2);
                                                    let var161 = self.func11(env, var12);
                                                    var5 = var161;
                                                    let var162 = self.func121(env, var1.wrapping_add(8448), var16, 64);
                                                    var162;
                                                    let var163 = self.func121(env, var14, var5, 64);
                                                    var163;
                                                    let var164 = self.func121(env, var13, var15, 64);
                                                    var164;
                                                    let var165 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(6912), 64);
                                                    var165;
                                                    var2 = 0;
                                                    let var166 = self.func122(env, var1.wrapping_add(7296), 0, 64);
                                                    var166;
                                                    'label36: loop {
                                                        'label37: loop {
                                                            if (var2 == 128) as i32 != 0 {
                                                                break 'label36;
                                                            }
                                                            let var167 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(8448).wrapping_add(var2), 64);
                                                            var167;
                                                            self.func89(env, var1.wrapping_add(8064), var1.wrapping_add(1088).wrapping_add(var2));
                                                            let var169 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(8064), 64);
                                                            var169;
                                                            var2 = var2.wrapping_add(64);
                                                            self.func10(env, var1.wrapping_add(7296), var1.wrapping_add(7680));
                                                            continue 'label37;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    let var171 = self.func121(env, var5, var1.wrapping_add(7296), 64);
                                                    var171;
                                                    self.func89(env, var10, var1.wrapping_add(4992));
                                                    let var173 = self.func11(env, var10);
                                                    var173;
                                                    continue 'label30;
                                                    break;
                                                }
                                                'label38: loop {
                                                    let var174 = self.func90(env, var12, var1.wrapping_add(4608));
                                                    if var174 != 0 {
                                                        break 'label38;
                                                    }
                                                    self.func85(env, var2);
                                                    continue 'label30;
                                                    break;
                                                }
                                                let var176 = self.func86(env, var2);
                                                var176;
                                                continue 'label30;
                                                break;
                                            }
                                            let var177 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1856), 192);
                                            var177;
                                            let var178 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 136);
                                            var178;
                                            self.func70(env, var1.wrapping_add(1888), 1051352);
                                            self.func70(env, var1.wrapping_add(1952), 1051352);
                                            self.func89(env, var1.wrapping_add(1856), 1051464);
                                            self.func89(env, var1.wrapping_add(1856).wrapping_add(64), 1051528);
                                            let var183 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(1856), 128);
                                            var183;
                                            let var184 = self.memory.load8(var1 as usize + 1984) as i32;
                                            var2 = var184;
                                            self.func85(env, var1.wrapping_add(1856));
                                            'label39: loop {
                                                'label40: loop {
                                                    if var2 & 1 != 0 {
                                                        break 'label40;
                                                    }
                                                    let var186 = self.func121(env, var1.wrapping_add(1088).wrapping_add(64), var1.wrapping_add(8448).wrapping_add(64), 64);
                                                    var186;
                                                    self.memory.store64(var1.wrapping_add(8120) as usize, 0 /* False */ as u64);
                                                    self.memory.store64(var1.wrapping_add(8112) as usize, 0 /* False */ as u64);
                                                    self.memory.store64(var1.wrapping_add(8104) as usize, 0 /* False */ as u64);
                                                    let var187 = self.memory.load64(0 as usize + 1051232) as i64;
                                                    self.memory.store64(var1.wrapping_add(8072) as usize, var187 as u64);
                                                    let var188 = self.memory.load64(0 as usize + 1051240) as i64;
                                                    self.memory.store64(var1.wrapping_add(8080) as usize, var188 as u64);
                                                    let var189 = self.memory.load64(0 as usize + 1051248) as i64;
                                                    self.memory.store64(var1.wrapping_add(8088) as usize, var189 as u64);
                                                    let mut slot_var1_8096_i64 = 0 /* False */ as i64;
                                                    let var190 = self.memory.load64(0 as usize + 1051224) as i64;
                                                    slot_var1_8064_i64 = var190 as i64;
                                                    let var191 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8448), 64);
                                                    var191;
                                                    let var192 = self.func121(env, var1.wrapping_add(1088).wrapping_add(128), var1.wrapping_add(8064), 64);
                                                    var192;
                                                    break 'label39;
                                                    break;
                                                }
                                                let var193 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 192);
                                                var193;
                                                break;
                                            }
                                            let var194 = self.func87(env, var1.wrapping_add(9216));
                                            var2 = var194;
                                            let var195 = self.func87(env, var1.wrapping_add(1088));
                                            var4 = var195;
                                            'label41: loop {
                                                'label42: loop {
                                                    if var2 != 0 {
                                                        break 'label42;
                                                    }
                                                    if var4 != 0 {
                                                        break 'label42;
                                                    }
                                                    var2 = var1.wrapping_add(9216).wrapping_add(128);
                                                    self.func7(env, var1.wrapping_add(5760), var2);
                                                    var4 = var1.wrapping_add(1088).wrapping_add(128);
                                                    self.func7(env, var1.wrapping_add(6144), var4);
                                                    let var198 = self.func121(env, var1.wrapping_add(6528), var1.wrapping_add(9216), 64);
                                                    var198;
                                                    self.func89(env, var1.wrapping_add(6528), var1.wrapping_add(6144));
                                                    let var200 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1088), 64);
                                                    var200;
                                                    self.func89(env, var1.wrapping_add(6912), var1.wrapping_add(5760));
                                                    let var202 = self.func90(env, var1.wrapping_add(6528), var1.wrapping_add(6912));
                                                    if (var202 == 0) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    let var203 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(9216).wrapping_add(64), 64);
                                                    var203;
                                                    let var204 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(6144), 64);
                                                    var204;
                                                    self.func89(env, var1.wrapping_add(7680), var4);
                                                    self.func89(env, var1.wrapping_add(7296), var1.wrapping_add(7680));
                                                    let var207 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1088).wrapping_add(64), 64);
                                                    var207;
                                                    let var208 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(5760), 64);
                                                    var208;
                                                    self.func89(env, var1.wrapping_add(1856), var2);
                                                    self.func89(env, var1.wrapping_add(8064), var1.wrapping_add(1856));
                                                    let var211 = self.func90(env, var1.wrapping_add(7296), var1.wrapping_add(8064));
                                                    if (var211 == 0) as i32 != 0 {
                                                        break 'label2;
                                                    }
                                                    break 'label41;
                                                    break;
                                                }
                                                if (var2 & var4 == 0) as i32 != 0 {
                                                    break 'label2;
                                                }
                                                break;
                                            }
                                            var3 = slot_var1_8832_i64;
                                            let var212 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8840), 120);
                                            var212;
                                            let var213 = self.memory.load32(var1.wrapping_add(8964) as usize) as i32;
                                            let mut slot_var1_9219_i32 = var213 as i32;
                                            let mut slot_var1_8961_i32 = self.memory.load32(var1 as usize + 8961) as i32;
                                            slot_var1_9216_i32 = slot_var1_8961_i32 as i32;
                                            var2 = 0;
                                            break 'label26;
                                            break;
                                        }
                                        self.func91(env, var5, 2);
                                        unreachable!();
                                        break;
                                    }
                                    let var215 = self.func121(env, var2, var1.wrapping_add(8832), 64);
                                    var215;
                                    let var216 = self.func121(env, var12, var7, 64);
                                    var216;
                                    self.memory.store64(var8.wrapping_add(24) as usize, 0 /* False */ as u64);
                                    self.memory.store64(var8.wrapping_add(16) as usize, 0 /* False */ as u64);
                                    self.memory.store64(var8.wrapping_add(8) as usize, 0 /* False */ as u64);
                                    slot_var8_0_i64 = 0 /* False */ as i64;
                                    let var217 = self.memory.load64(0 as usize + 1051232) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(8) as usize, var217 as u64);
                                    let var218 = self.memory.load64(0 as usize + 1051240) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(16) as usize, var218 as u64);
                                    let var219 = self.memory.load64(0 as usize + 1051248) as i64;
                                    self.memory.store64(var1.wrapping_add(1088).wrapping_add(24) as usize, var219 as u64);
                                    let var220 = self.memory.load64(0 as usize + 1051224) as i64;
                                    slot_var1_1088_i64 = var220 as i64;
                                    let var221 = self.func121(env, var10, var1.wrapping_add(1088), 64);
                                    var221;
                                    continue 'label30;
                                    break;
                                }
                                break;
                            }
                            let var222 = self.func122(env, var1.wrapping_add(1856), 0, 120);
                            var222;
                            var2 = 1;
                            var3 = 0 /* False */;
                            break;
                        }
                        let var223 = self.func121(env, var1.wrapping_add(1184), var1.wrapping_add(1856), 120);
                        var223;
                        self.memory.store32(var1.wrapping_add(1308) as usize, slot_var1_9219_i32 as u32);
                        let mut slot_var1_1305_i32 = slot_var1_9216_i32 as i32;
                        slot_var1_1096_i64 = proof as i64;
                        slot_var1_1088_i64 = 4294967296 as i64;
                        let var224 = self.func121(env, var1.wrapping_add(1104), var1.wrapping_add(80), 56);
                        var224;
                        self.memory.store32(var1.wrapping_add(1164) as usize, slot_var1_75_i32 as u32);
                        self.memory.store8(var1 as usize + 1160, var9 as u8);
                        self.memory.store8(var1 as usize + 1304, var2 as u8);
                        let mut slot_var1_1176_i64 = var3 as i64;
                        let mut slot_var1_1168_i64 = 4294967296 as i64;
                        let mut slot_var1_1161_i32 = slot_var1_72_i32 as i32;
                        self.func30(env, var1.wrapping_add(8832), var1.wrapping_add(1088));
                        var17 = 8;
                        var18 = 0;
                        var12 = 0;
                        'label43: loop {
                            'label44: loop {
                                'label45: loop {
                                    'label46: loop {
                                        let var226 = self.memory.load8(var1 as usize + 8896) as i32;
                                        if (var226 == 2) as i32 != 0 {
                                            break 'label46;
                                        }
                                        let var227 = self.func92(env, 352, 8);
                                        var17 = var227;
                                        if (var17 == 0) as i32 != 0 {
                                            break 'label45;
                                        }
                                        let var228 = self.func121(env, var17, var1.wrapping_add(8832), 88);
                                        var2 = var228;
                                        let mut slot_var1_8456_i32 = 1 as i32;
                                        let mut slot_var1_8452_i32 = var2 as i32;
                                        let mut slot_var1_8448_i32 = 4 as i32;
                                        let var229 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(1088), 224);
                                        var229;
                                        var2 = 88;
                                        var12 = 1;
                                        'label47: loop {
                                            'label48: loop {
                                                self.func30(env, var1.wrapping_add(9216), var1.wrapping_add(1856));
                                                let var231 = self.memory.load8(var1 as usize + 9280) as i32;
                                                if (var231 == 2) as i32 != 0 {
                                                    break 'label47;
                                                }
                                                'label49: loop {
                                                    if (var12 != slot_var1_8448_i32) as i32 != 0 {
                                                        break 'label49;
                                                    }
                                                    self.func58(env, var1, var1.wrapping_add(8448), var12, 8, 88);
                                                    let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                                                    var4 = slot_var1_0_i32;
                                                    if (var4 != -2147483647) as i32 != 0 {
                                                        break 'label44;
                                                    }
                                                    var17 = slot_var1_8452_i32;
                                                    break;
                                                }
                                                let var233 = self.func121(env, var17.wrapping_add(var2), var1.wrapping_add(9216), 88);
                                                var233;
                                                var12 = var12.wrapping_add(1);
                                                slot_var1_8456_i32 = var12 as i32;
                                                var2 = var2.wrapping_add(88);
                                                continue 'label48;
                                                break;
                                            }
                                            break;
                                        }
                                        var18 = slot_var1_8448_i32;
                                        break;
                                    }
                                    let var234 = self.func121(env, var1.wrapping_add(1856), 1048960, 64);
                                    var234;
                                    let var235 = self.func122(env, var1.wrapping_add(1856).wrapping_add(64), 0, 128);
                                    var235;
                                    let var236 = self.func122(env, var1.wrapping_add(8832).wrapping_add(192), 0, 192);
                                    var236;
                                    let var237 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 192);
                                    var237;
                                    var14 = var1.wrapping_add(9216).wrapping_add(64);
                                    var16 = var1.wrapping_add(1856).wrapping_add(384);
                                    var19 = var1.wrapping_add(1088).wrapping_add(384);
                                    var15 = var1.wrapping_add(1856).wrapping_add(192);
                                    var8 = var17;
                                    var10 = var12;
                                    'label50: loop {
                                        'label51: loop {
                                            if (var10 == 0) as i32 != 0 {
                                                break 'label50;
                                            }
                                            let var238 = self.func121(env, var1.wrapping_add(9216), 1048960, 64);
                                            var238;
                                            let var239 = self.func122(env, var14, 0, 128);
                                            var239;
                                            let var240 = self.func122(env, var15, 0, 192);
                                            var240;
                                            let var241 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(9216), 192);
                                            var241;
                                            var2 = { let a = var10; let b = 4; if ((var10 as u32) < 4 as u32) as i32 != 0 { a } else { b } };
                                            var10 = var10.wrapping_sub(var2);
                                            var9 = var2.wrapping_mul(88);
                                            var13 = var8.wrapping_add(var9);
                                            var6 = 65;
                                            'label52: loop {
                                                'label53: loop {
                                                    if ((var6 as u32) < 2 as u32) as i32 != 0 {
                                                        break 'label52;
                                                    }
                                                    'label54: loop {
                                                        var7 = var6.wrapping_add(-1);
                                                        if (var7 == 64) as i32 != 0 {
                                                            break 'label54;
                                                        }
                                                        let var242 = self.func18(env, var1.wrapping_add(1856));
                                                        var242;
                                                        break;
                                                    }
                                                    var4 = var9;
                                                    var2 = var8;
                                                    'label55: loop {
                                                        'label56: loop {
                                                            if (var4 == 0) as i32 != 0 {
                                                                break 'label55;
                                                            }
                                                            let mut slot_var2_76_i32 = self.memory.load32(var2 as usize + 76) as i32;
                                                            var5 = slot_var2_76_i32;
                                                            let mut slot_var2_84_i32 = self.memory.load32(var2 as usize + 84) as i32;
                                                            if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                                break 'label43;
                                                            }
                                                            slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                            let var243 = self.func121(env, var1.wrapping_add(9216), var5, 192);
                                                            var243;
                                                            self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(9216), var2);
                                                            var4 = var4.wrapping_add(-88);
                                                            var2 = var2.wrapping_add(88);
                                                            continue 'label56;
                                                            break;
                                                        }
                                                        break;
                                                    }
                                                    'label57: loop {
                                                        let var245 = self.memory.load8(var6.wrapping_add(1050558) as usize) as i32;
                                                        var2 = var245;
                                                        if (var2 == 255) as i32 != 0 {
                                                            break 'label57;
                                                        }
                                                        var6 = var7;
                                                        if (var2 != 1) as i32 != 0 {
                                                            continue 'label53;
                                                        }
                                                        break;
                                                    }
                                                    var4 = var9;
                                                    var2 = var8;
                                                    'label58: loop {
                                                        'label59: loop {
                                                            if var4 != 0 {
                                                                break 'label59;
                                                            }
                                                            var6 = var7;
                                                            continue 'label53;
                                                            break;
                                                        }
                                                        var5 = slot_var2_76_i32;
                                                        if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                            break 'label43;
                                                        }
                                                        slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                        let var246 = self.func121(env, var1.wrapping_add(9216), var5, 192);
                                                        var246;
                                                        self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(9216), var2);
                                                        var4 = var4.wrapping_add(-88);
                                                        var2 = var2.wrapping_add(88);
                                                        continue 'label58;
                                                        break;
                                                    }
                                                    break;
                                                }
                                                break;
                                            }
                                            let var248 = self.func121(env, var19, var1.wrapping_add(1856), 384);
                                            var2 = var248;
                                            let var249 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 384);
                                            var249;
                                            let var250 = self.func121(env, var16, var2, 384);
                                            var2 = var250;
                                            let var251 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(8832), 384);
                                            var251;
                                            self.func42(env, var1.wrapping_add(9216), var2);
                                            let var253 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(9216), 384);
                                            var253;
                                            var8 = var13;
                                            continue 'label51;
                                            break;
                                        }
                                        break;
                                    }
                                    let var254 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8832), 384);
                                    var254;
                                    var4 = var12.wrapping_mul(88);
                                    var2 = var17;
                                    'label60: loop {
                                        'label61: loop {
                                            'label62: loop {
                                                if var4 != 0 {
                                                    break 'label62;
                                                }
                                                var4 = var12.wrapping_mul(88);
                                                var2 = var17;
                                                'label63: loop {
                                                    if (var4 == 0) as i32 != 0 {
                                                        break 'label60;
                                                    }
                                                    var5 = slot_var2_76_i32;
                                                    if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                        break 'label43;
                                                    }
                                                    slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                                    let var255 = self.func121(env, var1.wrapping_add(1088), var5, 192);
                                                    var255;
                                                    self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(1088), var2);
                                                    var4 = var4.wrapping_add(-88);
                                                    var2 = var2.wrapping_add(88);
                                                    continue 'label63;
                                                    break;
                                                }
                                                break;
                                            }
                                            var5 = slot_var2_76_i32;
                                            if (var5 == slot_var2_84_i32) as i32 != 0 {
                                                break 'label43;
                                            }
                                            slot_var2_76_i32 = var5.wrapping_add(192) as i32;
                                            let var257 = self.func121(env, var1.wrapping_add(1088), var5, 192);
                                            var257;
                                            self.func68(env, var1.wrapping_add(1856), var1.wrapping_add(1088), var2);
                                            var4 = var4.wrapping_add(-88);
                                            var2 = var2.wrapping_add(88);
                                            continue 'label61;
                                            break;
                                        }
                                        break;
                                    }
                                    let var259 = self.func121(env, var1.wrapping_add(704), var1.wrapping_add(1856), 384);
                                    var259;
                                    var2 = var17.wrapping_add(80);
                                    'label64: loop {
                                        'label65: loop {
                                            if (var12 == 0) as i32 != 0 {
                                                break 'label64;
                                            }
                                            let var260 = self.memory.load32(var2.wrapping_add(-8) as usize) as i32;
                                            self.func39(env, slot_var2_0_i64, var260);
                                            var12 = var12.wrapping_add(-1);
                                            var2 = var2.wrapping_add(88);
                                            continue 'label65;
                                            break;
                                        }
                                        break;
                                    }
                                    self.func47(env, var18, var17, 8, 88);
                                    let var263 = self.func121(env, var1.wrapping_add(2624), var1.wrapping_add(704), 384);
                                    var263;
                                    let var264 = self.func65(env, var1.wrapping_add(2624));
                                    var264;
                                    let var265 = self.func28(env, var1.wrapping_add(704));
                                    if var265 != 0 {
                                        break 'label43;
                                    }
                                    var18 = var1.wrapping_add(704).wrapping_add(192);
                                    self.func6(env, var1.wrapping_add(8832), var18);
                                    self.func6(env, var1.wrapping_add(1856), var1.wrapping_add(704));
                                    var2 = var1.wrapping_add(8832).wrapping_add(64);
                                    let var268 = self.func121(env, var1.wrapping_add(8448), var2, 64);
                                    var268;
                                    let var269 = self.func121(env, var2, var1.wrapping_add(8832), 64);
                                    var10 = var269;
                                    var6 = var1.wrapping_add(8960);
                                    let var270 = self.func121(env, var1.wrapping_add(8832), var6, 64);
                                    var270;
                                    let var271 = self.func121(env, var1.wrapping_add(9216), var6, 64);
                                    var271;
                                    let var272 = self.func11(env, var1.wrapping_add(9216));
                                    let var273 = self.func11(env, var272);
                                    let var274 = self.func11(env, var273);
                                    var5 = var274;
                                    var13 = var1.wrapping_add(7680).wrapping_add(24);
                                    let var275 = self.memory.load64(var1.wrapping_add(9016) as usize) as i64;
                                    slot_var13_0_i64 = var275 as i64;
                                    var14 = var1.wrapping_add(7680).wrapping_add(16);
                                    let var276 = self.memory.load64(var1.wrapping_add(9008) as usize) as i64;
                                    let mut slot_var14_0_i64 = var276 as i64;
                                    var8 = var1.wrapping_add(7680).wrapping_add(8);
                                    let var277 = self.memory.load64(var1.wrapping_add(9000) as usize) as i64;
                                    slot_var8_0_i64 = var277 as i64;
                                    let mut slot_var1_8992_i64 = self.memory.load64(var1 as usize + 8992) as i64;
                                    slot_var1_7680_i64 = slot_var1_8992_i64 as i64;
                                    self.func12(env, var1.wrapping_add(7680));
                                    self.func13(env, var1.wrapping_add(7680), var5);
                                    self.func13(env, var1.wrapping_add(7680), var1.wrapping_add(8832));
                                    var2 = var1.wrapping_add(1088).wrapping_add(24);
                                    let var281 = self.memory.load64(var5.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var281 as i64;
                                    var4 = var1.wrapping_add(1088).wrapping_add(16);
                                    let var282 = self.memory.load64(var5.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var282 as i64;
                                    var7 = var1.wrapping_add(1088).wrapping_add(8);
                                    let var283 = self.memory.load64(var5.wrapping_add(40) as usize) as i64;
                                    slot_var7_0_i64 = var283 as i64;
                                    let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
                                    slot_var1_1088_i64 = slot_var5_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(8992));
                                    var5 = var1.wrapping_add(8064).wrapping_add(24);
                                    slot_var5_0_i64 = slot_var2_0_i64 as i64;
                                    var9 = var1.wrapping_add(8064).wrapping_add(16);
                                    slot_var9_0_i64 = slot_var4_0_i64 as i64;
                                    var12 = var1.wrapping_add(8064).wrapping_add(8);
                                    slot_var12_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var1_8064_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(8064), var6);
                                    var15 = var1.wrapping_add(1088).wrapping_add(56);
                                    let mut slot_var15_0_i64 = slot_var5_0_i64 as i64;
                                    var16 = var1.wrapping_add(1088).wrapping_add(48);
                                    let mut slot_var16_0_i64 = slot_var9_0_i64 as i64;
                                    var19 = var1.wrapping_add(1088).wrapping_add(40);
                                    let mut slot_var19_0_i64 = slot_var12_0_i64 as i64;
                                    slot_var7_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    let mut slot_var1_1120_i64 = slot_var1_8064_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_7680_i64 as i64;
                                    let var286 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1088), 64);
                                    var286;
                                    let var287 = self.func121(env, var6, var1.wrapping_add(8448), 64);
                                    var12 = var287;
                                    self.func20(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                    let var289 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 192);
                                    var289;
                                    let var290 = self.func26(env, var1.wrapping_add(8832));
                                    if var290 != 0 {
                                        break 'label43;
                                    }
                                    self.func7(env, var1.wrapping_add(3008), var1.wrapping_add(8832));
                                    self.func7(env, var1.wrapping_add(3072), var10);
                                    self.func7(env, var1.wrapping_add(3136), var12);
                                    let var294 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var294;
                                    self.func5(env, var1.wrapping_add(1088), var10);
                                    let var296 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1088), 64);
                                    var296;
                                    let var297 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var297;
                                    self.func5(env, var1.wrapping_add(1088), var12);
                                    let var299 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(1088), 64);
                                    var299;
                                    let var300 = self.func121(env, var1.wrapping_add(3200), var10, 64);
                                    var300;
                                    self.func5(env, var1.wrapping_add(3200), var12);
                                    let var302 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3200), 64);
                                    var302;
                                    let var303 = self.func11(env, var1.wrapping_add(9216));
                                    let var304 = self.func11(env, var303);
                                    let var305 = self.func11(env, var304);
                                    var6 = var305;
                                    var7 = var1.wrapping_add(6528).wrapping_add(24);
                                    let var306 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(56) as usize) as i64;
                                    slot_var7_0_i64 = var306 as i64;
                                    var8 = var1.wrapping_add(6528).wrapping_add(16);
                                    let var307 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(48) as usize) as i64;
                                    slot_var8_0_i64 = var307 as i64;
                                    var9 = var1.wrapping_add(6528).wrapping_add(8);
                                    let var308 = self.memory.load64(var1.wrapping_add(3200).wrapping_add(40) as usize) as i64;
                                    slot_var9_0_i64 = var308 as i64;
                                    let mut slot_var1_3232_i64 = self.memory.load64(var1 as usize + 3232) as i64;
                                    let mut slot_var1_6528_i64 = slot_var1_3232_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6528));
                                    self.func13(env, var1.wrapping_add(6528), var6);
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(3200));
                                    let var312 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var312 as i64;
                                    let var313 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var313 as i64;
                                    var5 = var1.wrapping_add(1088).wrapping_add(8);
                                    let var314 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var314 as i64;
                                    let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(3200).wrapping_add(32));
                                    slot_var13_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var14_0_i64 = slot_var4_0_i64 as i64;
                                    var6 = var1.wrapping_add(7680).wrapping_add(8);
                                    slot_var6_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_7680_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(7680), var1.wrapping_add(3200));
                                    slot_var15_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var6_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_7680_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6528_i64 as i64;
                                    let var317 = self.func121(env, var1.wrapping_add(3200), var1.wrapping_add(1088), 64);
                                    var317;
                                    let var318 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(3200), 64);
                                    var318;
                                    self.func9(env, var1.wrapping_add(3008), var1.wrapping_add(7680));
                                    let var320 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3136), 64);
                                    var320;
                                    let var321 = self.func11(env, var1.wrapping_add(9216));
                                    let var322 = self.func11(env, var321);
                                    let var323 = self.func11(env, var322);
                                    var6 = var323;
                                    var13 = var1.wrapping_add(6144).wrapping_add(24);
                                    let var324 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(56) as usize) as i64;
                                    slot_var13_0_i64 = var324 as i64;
                                    var14 = var1.wrapping_add(6144).wrapping_add(16);
                                    let var325 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(48) as usize) as i64;
                                    slot_var14_0_i64 = var325 as i64;
                                    var17 = var1.wrapping_add(6144).wrapping_add(8);
                                    let var326 = self.memory.load64(var1.wrapping_add(3136).wrapping_add(40) as usize) as i64;
                                    let mut slot_var17_0_i64 = var326 as i64;
                                    let mut slot_var1_3168_i64 = self.memory.load64(var1 as usize + 3168) as i64;
                                    let mut slot_var1_6144_i64 = slot_var1_3168_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.func13(env, var1.wrapping_add(6144), var6);
                                    self.func13(env, var1.wrapping_add(6144), var1.wrapping_add(3136));
                                    let var330 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var330 as i64;
                                    let var331 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var331 as i64;
                                    let var332 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var332 as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(3136).wrapping_add(32));
                                    slot_var7_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var8_0_i64 = slot_var4_0_i64 as i64;
                                    slot_var9_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(3136));
                                    slot_var15_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var17_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_6528_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6144_i64 as i64;
                                    let var335 = self.func121(env, var1.wrapping_add(3136), var1.wrapping_add(1088), 64);
                                    var335;
                                    self.func9(env, var1.wrapping_add(3136), var1.wrapping_add(6912));
                                    self.func9(env, var1.wrapping_add(3072), var1.wrapping_add(7296));
                                    let var338 = self.func121(env, var1.wrapping_add(1088), var12, 64);
                                    var338;
                                    self.func5(env, var1.wrapping_add(1088), var1.wrapping_add(3136));
                                    let var340 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1088), 64);
                                    var340;
                                    let var341 = self.func121(env, var1.wrapping_add(3264), var10, 64);
                                    var341;
                                    self.func5(env, var1.wrapping_add(3264), var1.wrapping_add(3072));
                                    self.func10(env, var1.wrapping_add(8064), var1.wrapping_add(3264));
                                    let var344 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(8064), 64);
                                    var344;
                                    let var345 = self.func11(env, var1.wrapping_add(9216));
                                    let var346 = self.func11(env, var345);
                                    let var347 = self.func11(env, var346);
                                    var6 = var347;
                                    let var348 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(56) as usize) as i64;
                                    slot_var13_0_i64 = var348 as i64;
                                    let var349 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(48) as usize) as i64;
                                    slot_var14_0_i64 = var349 as i64;
                                    let var350 = self.memory.load64(var1.wrapping_add(8064).wrapping_add(40) as usize) as i64;
                                    slot_var17_0_i64 = var350 as i64;
                                    slot_var1_6144_i64 = slot_var1_8096_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.func13(env, var1.wrapping_add(6144), var6);
                                    self.func13(env, var1.wrapping_add(6144), var1.wrapping_add(8064));
                                    let var354 = self.memory.load64(var6.wrapping_add(56) as usize) as i64;
                                    slot_var2_0_i64 = var354 as i64;
                                    let var355 = self.memory.load64(var6.wrapping_add(48) as usize) as i64;
                                    slot_var4_0_i64 = var355 as i64;
                                    let var356 = self.memory.load64(var6.wrapping_add(40) as usize) as i64;
                                    slot_var5_0_i64 = var356 as i64;
                                    slot_var1_1088_i64 = slot_var6_32_i64 as i64;
                                    self.func13(env, var1.wrapping_add(1088), var1.wrapping_add(8064).wrapping_add(32));
                                    slot_var7_0_i64 = slot_var2_0_i64 as i64;
                                    slot_var8_0_i64 = slot_var4_0_i64 as i64;
                                    slot_var9_0_i64 = slot_var5_0_i64 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    self.func13(env, var1.wrapping_add(6528), var1.wrapping_add(8064));
                                    slot_var15_0_i64 = slot_var7_0_i64 as i64;
                                    slot_var16_0_i64 = slot_var8_0_i64 as i64;
                                    slot_var19_0_i64 = slot_var9_0_i64 as i64;
                                    slot_var5_0_i64 = slot_var17_0_i64 as i64;
                                    slot_var4_0_i64 = slot_var14_0_i64 as i64;
                                    slot_var2_0_i64 = slot_var13_0_i64 as i64;
                                    slot_var1_1120_i64 = slot_var1_6528_i64 as i64;
                                    slot_var1_1088_i64 = slot_var1_6144_i64 as i64;
                                    let var359 = self.func121(env, var1.wrapping_add(3328), var1.wrapping_add(1088), 64);
                                    var359;
                                    let var360 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(8832), 64);
                                    var360;
                                    self.func5(env, var1.wrapping_add(1088), var1.wrapping_add(3008));
                                    let var362 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1088), 64);
                                    var362;
                                    self.func10(env, var1.wrapping_add(9216), var1.wrapping_add(3328));
                                    let var364 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(9216), 64);
                                    var364;
                                    let var365 = self.func27(env, var1.wrapping_add(1088));
                                    if var365 != 0 {
                                        break 'label43;
                                    }
                                    var2 = var1.wrapping_add(1088).wrapping_add(32);
                                    self.func76(env, var1.wrapping_add(5760), var2);
                                    self.func76(env, var1.wrapping_add(9216), var1.wrapping_add(1088));
                                    self.func12(env, var1.wrapping_add(5760));
                                    self.func43(env, var1.wrapping_add(9216), var1.wrapping_add(5760));
                                    let var370 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(24) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(24) as usize, var370 as u64);
                                    let var371 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(16) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(16) as usize, var371 as u64);
                                    let var372 = self.memory.load64(var1.wrapping_add(9216).wrapping_add(8) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(5760).wrapping_add(8) as usize, var372 as u64);
                                    let mut slot_var1_5760_i64 = slot_var1_9216_i64 as i64;
                                    self.func33(env, var1.wrapping_add(9216), var1.wrapping_add(5760));
                                    if (slot_var1_9216_i32 == 0) as i32 != 0 {
                                        break 'label43;
                                    }
                                    var4 = var1.wrapping_add(6528).wrapping_add(24);
                                    let var374 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(24) as usize) as i64;
                                    slot_var4_0_i64 = var374 as i64;
                                    var5 = var1.wrapping_add(6528).wrapping_add(16);
                                    let var375 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(16) as usize) as i64;
                                    slot_var5_0_i64 = var375 as i64;
                                    var6 = var1.wrapping_add(6528).wrapping_add(8);
                                    let var376 = self.memory.load64(var1.wrapping_add(1088).wrapping_add(8) as usize) as i64;
                                    slot_var6_0_i64 = var376 as i64;
                                    slot_var1_6528_i64 = slot_var1_1088_i64 as i64;
                                    var7 = var1.wrapping_add(9216).wrapping_add(8);
                                    self.func70(env, var1.wrapping_add(6528), var7);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(24) as usize, slot_var4_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(16) as usize, slot_var5_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3392).wrapping_add(8) as usize, slot_var6_0_i64 as u64);
                                    let mut slot_var1_3392_i64 = slot_var1_6528_i64 as i64;
                                    let var378 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                                    slot_var4_0_i64 = var378 as i64;
                                    let var379 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                                    slot_var5_0_i64 = var379 as i64;
                                    let var380 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                                    slot_var6_0_i64 = var380 as i64;
                                    slot_var1_6528_i64 = slot_var2_0_i64 as i64;
                                    self.func70(env, var1.wrapping_add(6528), var7);
                                    var2 = var1.wrapping_add(6144).wrapping_add(24);
                                    slot_var2_0_i64 = slot_var4_0_i64 as i64;
                                    var4 = var1.wrapping_add(6144).wrapping_add(16);
                                    slot_var4_0_i64 = slot_var5_0_i64 as i64;
                                    var5 = var1.wrapping_add(6144).wrapping_add(8);
                                    slot_var5_0_i64 = slot_var6_0_i64 as i64;
                                    slot_var1_6144_i64 = slot_var1_6528_i64 as i64;
                                    self.func12(env, var1.wrapping_add(6144));
                                    self.memory.store64(var1.wrapping_add(3448) as usize, slot_var2_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3440) as usize, slot_var4_0_i64 as u64);
                                    self.memory.store64(var1.wrapping_add(3432) as usize, slot_var5_0_i64 as u64);
                                    let mut slot_var1_3424_i64 = slot_var1_6144_i64 as i64;
                                    let var383 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(3392), 64);
                                    var383;
                                    let var384 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(3392), 64);
                                    var384;
                                    self.func5(env, var1.wrapping_add(9216), var1.wrapping_add(3008));
                                    let var386 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(9216), 64);
                                    var386;
                                    self.func5(env, var1.wrapping_add(3392), var1.wrapping_add(3136));
                                    let var388 = self.func121(env, var1.wrapping_add(1088).wrapping_add(64), var1.wrapping_add(3392), 64);
                                    var388;
                                    self.func5(env, var1.wrapping_add(8448), var1.wrapping_add(3072));
                                    let var390 = self.func121(env, var1.wrapping_add(1216), var1.wrapping_add(8448), 64);
                                    var390;
                                    let var391 = self.func121(env, var1.wrapping_add(1856).wrapping_add(8), var1.wrapping_add(1088), 192);
                                    var2 = var391;
                                    slot_var1_1856_i64 = 1 /* True */ as i64;
                                    let var392 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(704), 192);
                                    var392;
                                    self.func21(env, var1.wrapping_add(9216), var2);
                                    let var394 = self.func121(env, var1.wrapping_add(1088), var18, 192);
                                    var394;
                                    self.func21(env, var1.wrapping_add(1088), var2);
                                    self.func14(env, var1.wrapping_add(3456).wrapping_add(192), var1.wrapping_add(1088));
                                    let var397 = self.func121(env, var1.wrapping_add(3456), var1.wrapping_add(9216), 192);
                                    var397;
                                    let var398 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(2624), 384);
                                    var398;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(3456));
                                    let var400 = self.func121(env, var1.wrapping_add(3840), var1.wrapping_add(1856), 384);
                                    var400;
                                    let var401 = self.func121(env, var1.wrapping_add(3456), var1.wrapping_add(3840), 384);
                                    var401;
                                    self.func24(env, var1.wrapping_add(3840), 2);
                                    self.func42(env, var1.wrapping_add(3840), var1.wrapping_add(3456));
                                    let var404 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(3840), 384);
                                    var404;
                                    self.func59(env, var1.wrapping_add(4224), var1.wrapping_add(1856));
                                    self.func72(env, var1.wrapping_add(4608), var1.wrapping_add(4224));
                                    self.func72(env, var1.wrapping_add(4992), var1.wrapping_add(4608));
                                    self.func42(env, var1.wrapping_add(4992), var1.wrapping_add(4608));
                                    let var409 = self.func121(env, var1.wrapping_add(5376), var1.wrapping_add(4992), 384);
                                    var409;
                                    let var410 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(4992), 384);
                                    var410;
                                    self.func59(env, var1.wrapping_add(5760), var1.wrapping_add(1856));
                                    self.func72(env, var1.wrapping_add(6144), var1.wrapping_add(5760));
                                    self.func59(env, var1.wrapping_add(6528), var1.wrapping_add(6144));
                                    let var414 = self.func65(env, var1.wrapping_add(5376));
                                    var414;
                                    let var415 = self.func65(env, var1.wrapping_add(6528));
                                    var415;
                                    let var416 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6528), 384);
                                    var416;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(5760));
                                    let var418 = self.func121(env, var1.wrapping_add(6912), var1.wrapping_add(1856), 384);
                                    var418;
                                    self.func42(env, var1.wrapping_add(6912), var1.wrapping_add(5376));
                                    let var420 = self.func121(env, var1.wrapping_add(7296), var1.wrapping_add(6912), 384);
                                    var420;
                                    let var421 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6912), 384);
                                    var421;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(4608));
                                    let var423 = self.func121(env, var1.wrapping_add(7680), var1.wrapping_add(1856), 384);
                                    var423;
                                    let var424 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(6912), 384);
                                    var424;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(5760));
                                    let var426 = self.func121(env, var1.wrapping_add(8064), var1.wrapping_add(1856), 384);
                                    var426;
                                    self.func42(env, var1.wrapping_add(8064), var1.wrapping_add(3840));
                                    let var428 = self.func121(env, var1.wrapping_add(8448), var1.wrapping_add(7680), 384);
                                    var428;
                                    self.func24(env, var1.wrapping_add(8448), 1);
                                    let var430 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(8448), 384);
                                    var430;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(8064));
                                    let var432 = self.func121(env, var1.wrapping_add(8832), var1.wrapping_add(1856), 384);
                                    var432;
                                    self.func24(env, var1.wrapping_add(7296), 2);
                                    let var434 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(7296), 384);
                                    var434;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(8832));
                                    let var436 = self.func121(env, var1.wrapping_add(9216), var1.wrapping_add(1856), 384);
                                    var436;
                                    let var437 = self.func65(env, var1.wrapping_add(3840));
                                    var437;
                                    let var438 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(3840), 384);
                                    var438;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(7680));
                                    let var440 = self.func121(env, var1.wrapping_add(1088), var1.wrapping_add(1856), 384);
                                    var440;
                                    self.func24(env, var1.wrapping_add(1088), 3);
                                    let var442 = self.func121(env, var1.wrapping_add(1856), var1.wrapping_add(1088), 384);
                                    var442;
                                    self.func42(env, var1.wrapping_add(1856), var1.wrapping_add(9216));
                                    let var444 = self.func121(env, var1.wrapping_add(264), var1.wrapping_add(1856), 384);
                                    var444;
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(616));
                                    self.func81(env, var1.wrapping_add(1856), 1051160);
                                    var2 = 0;
                                    var4 = 0;
                                    'label66: loop {
                                        let var447 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var447 & 255 != 0 {
                                            break 'label66;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(584));
                                        self.func81(env, var1.wrapping_add(1856), 1051128);
                                        let var450 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        var4 = (var450 & 255 == 0) as i32;
                                        break;
                                    }
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(552));
                                    self.func81(env, var1.wrapping_add(1856), 1051096);
                                    'label67: loop {
                                        let var453 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var453 & 255 != 0 {
                                            break 'label67;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(520));
                                        self.func81(env, var1.wrapping_add(1856), 1051064);
                                        let var456 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        var2 = (var456 & 255 == 0) as i32;
                                        break;
                                    }
                                    self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(488));
                                    self.func81(env, var1.wrapping_add(1856), 1051032);
                                    proof = 0 /* False */;
                                    'label68: loop {
                                        let var459 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if var459 & 255 != 0 {
                                            break 'label68;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(264).wrapping_add(192));
                                        self.func81(env, var1.wrapping_add(1856), 1051000);
                                        let var462 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                        if (var4 & var2 & (var462 & 255 == 0) as i32 != 1) as i32 != 0 {
                                            break 'label68;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(424));
                                        self.func81(env, var1.wrapping_add(1856), 1050968);
                                        var4 = 0;
                                        var2 = 0;
                                        'label69: loop {
                                            let var465 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var465 & 255 != 0 {
                                                break 'label69;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(392));
                                            self.func81(env, var1.wrapping_add(1856), 1050936);
                                            let var468 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var2 = (var468 & 255 == 0) as i32;
                                            break;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(360));
                                        self.func81(env, var1.wrapping_add(1856), 1050904);
                                        'label70: loop {
                                            let var471 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var471 & 255 != 0 {
                                                break 'label70;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(328));
                                            self.func81(env, var1.wrapping_add(1856), 1050872);
                                            let var474 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var4 = (var474 & 255 == 0) as i32;
                                            break;
                                        }
                                        self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(296));
                                        self.func81(env, var1.wrapping_add(1856), 1050840);
                                        var5 = 0;
                                        'label71: loop {
                                            let var477 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            if var477 & 255 != 0 {
                                                break 'label71;
                                            }
                                            self.func81(env, var1.wrapping_add(1088), var1.wrapping_add(264));
                                            self.func81(env, var1.wrapping_add(1856), 1050808);
                                            let var480 = self.func75(env, var1.wrapping_add(1088), var1.wrapping_add(1856));
                                            var5 = (var480 & 255 == 0) as i32;
                                            break;
                                        }
                                        if (var2 == 0) as i32 != 0 {
                                            break 'label68;
                                        }
                                        proof = (var4 & var5) as u32 as i64;
                                        break;
                                    }
                                    self.global0 = var1.wrapping_add(9600);
                                    return proof;
                                    break;
                                }
                                self.func56(env, 8, 352);
                                unreachable!();
                                break;
                            }
                            let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
                            self.func56(env, var4, slot_var1_4_i32);
                            unreachable!();
                            break;
                        }
                        self.func37(env);
                        unreachable!();
                        break;
                    }
                    self.func53(env, var1.wrapping_add(1856), var1.wrapping_add(7296), var1.wrapping_add(8832).wrapping_add(var2));
                    'label72: loop {
                        let var485 = self.memory.load8(var1 as usize + 1856) as i32;
                        if (var485 == 2) as i32 != 0 {
                            break 'label72;
                        }
                        let var486 = self.memory.load8(var1 as usize + 1856) as i64;
                        if (var486 != 0 /* Void */) as i32 != 0 {
                            break 'label2;
                        }
                        break;
                    }
                    var2 = var2.wrapping_add(8);
                    continue 'label4;
                    break;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.func73(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func84(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) {
        let var3 = 0 /* TODO: bytes_copy_to_linear_memory */
        var3;
    }
    fn func85(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(128);
        self.global0 = var1;
        self.memory.store64(var1.wrapping_add(56) as usize, 0 /* False */ as u64);
        self.memory.store64(var1.wrapping_add(48) as usize, 0 /* False */ as u64);
        self.memory.store64(var1.wrapping_add(40) as usize, 0 /* False */ as u64);
        let var7 = self.memory.load64(0 as usize + 1051232) as i64;
        var2 = var7;
        self.memory.store64(var1.wrapping_add(8) as usize, var2 as u64);
        let var8 = self.memory.load64(0 as usize + 1051240) as i64;
        var3 = var8;
        self.memory.store64(var1.wrapping_add(16) as usize, var3 as u64);
        let var9 = self.memory.load64(0 as usize + 1051248) as i64;
        var4 = var9;
        self.memory.store64(var1.wrapping_add(24) as usize, var4 as u64);
        let mut slot_var1_32_i64 = 0 /* False */ as i64;
        let var10 = self.memory.load64(0 as usize + 1051224) as i64;
        var5 = var10;
        let mut slot_var1_0_i64 = var5 as i64;
        self.memory.store64(var1.wrapping_add(64).wrapping_add(56) as usize, 0 /* False */ as u64);
        self.memory.store64(var1.wrapping_add(64).wrapping_add(48) as usize, 0 /* False */ as u64);
        self.memory.store64(var1.wrapping_add(64).wrapping_add(40) as usize, 0 /* False */ as u64);
        self.memory.store64(var1.wrapping_add(64).wrapping_add(8) as usize, var2 as u64);
        self.memory.store64(var1.wrapping_add(64).wrapping_add(16) as usize, var3 as u64);
        self.memory.store64(var1.wrapping_add(64).wrapping_add(24) as usize, var4 as u64);
        let mut slot_var1_96_i64 = 0 /* False */ as i64;
        let mut slot_var1_64_i64 = var5 as i64;
        let var11 = self.func121(env, arg0, var1, 64);
        arg0 = var11;
        let var12 = self.func121(env, arg0.wrapping_add(64), var1.wrapping_add(64), 64);
        var12;
        let var13 = self.func122(env, arg0.wrapping_add(128), 0, 64);
        var13;
        self.global0 = var1.wrapping_add(128);
    }
    fn func86(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var1 = var7.wrapping_sub(704);
        self.global0 = var1;
        'label0: loop {
            let var8 = self.func87(env, arg0);
            if var8 != 0 {
                break 'label0;
            }
            'label1: loop {
                let var9 = self.func90(env, 1051256, 1051256);
                if var9 != 0 {
                    break 'label1;
                }
                self.func7(env, var1.wrapping_add(128), arg0);
                var2 = arg0.wrapping_add(64);
                self.func7(env, var1.wrapping_add(192), var2);
                let var12 = self.func121(env, var1.wrapping_add(256), var1.wrapping_add(192), 64);
                var12;
                let var13 = self.func88(env, var1.wrapping_add(256));
                var3 = var13;
                var4 = arg0.wrapping_add(128);
                let var14 = self.func121(env, var1.wrapping_add(320), var4, 64);
                var14;
                let var15 = self.func88(env, var1.wrapping_add(320));
                var5 = var15;
                let var16 = self.func121(env, var1.wrapping_add(640), arg0, 64);
                var16;
                self.func10(env, var1.wrapping_add(640), var1.wrapping_add(192));
                let var18 = self.func121(env, var1.wrapping_add(576), var1.wrapping_add(640), 64);
                var18;
                self.func7(env, var1.wrapping_add(448), var1.wrapping_add(576));
                self.func9(env, var1.wrapping_add(448), var1.wrapping_add(128));
                self.func9(env, var1.wrapping_add(448), var3);
                self.func8(env, var1.wrapping_add(384), var1.wrapping_add(448));
                let var23 = self.func121(env, var1.wrapping_add(512), var1.wrapping_add(128), 64);
                var23;
                let var24 = self.func11(env, var1.wrapping_add(512));
                var6 = var24;
                self.func10(env, var6, var1.wrapping_add(128));
                self.func7(env, var1.wrapping_add(640), var5);
                let var27 = self.func122(env, var1.wrapping_add(576), 0, 64);
                var27;
                self.func10(env, var6, var1.wrapping_add(576));
                let var29 = self.func121(env, arg0, var6, 64);
                let var30 = self.func88(env, var29);
                var5 = var30;
                self.func8(env, var1.wrapping_add(640), var1.wrapping_add(384));
                self.func9(env, var5, var1.wrapping_add(640));
                let var33 = self.func121(env, var1.wrapping_add(640), var2, 64);
                var33;
                self.func89(env, var4, var1.wrapping_add(640));
                let var35 = self.func11(env, var4);
                var35;
                let var36 = self.func121(env, var2, var1.wrapping_add(384), 64);
                var2 = var36;
                self.func9(env, var2, var5);
                self.func89(env, var2, var6);
                let var39 = self.func11(env, var3);
                let var40 = self.func11(env, var39);
                let var41 = self.func11(env, var40);
                self.func9(env, var2, var41);
                break 'label0;
                break;
            }
            let var43 = self.func121(env, var1.wrapping_add(320), arg0, 64);
            var43;
            let var44 = self.func88(env, var1.wrapping_add(320));
            var2 = var44;
            var3 = arg0.wrapping_add(64);
            let var45 = self.func121(env, var1.wrapping_add(512), var3, 64);
            var45;
            let var46 = self.func88(env, var1.wrapping_add(512));
            var6 = var46;
            let var47 = self.func121(env, var1.wrapping_add(576), var6, 64);
            var47;
            let var48 = self.func88(env, var1.wrapping_add(576));
            var4 = var48;
            let var49 = self.func121(env, var1.wrapping_add(640), arg0, 64);
            var49;
            self.func89(env, var1.wrapping_add(640), var6);
            let var51 = self.func11(env, var1.wrapping_add(640));
            let var52 = self.func11(env, var51);
            let var53 = self.func121(env, var1, var52, 64);
            var6 = var53;
            let var54 = self.func121(env, var6.wrapping_add(640), var2, 64);
            var54;
            let var55 = self.func11(env, var2);
            self.func10(env, var6.wrapping_add(640), var55);
            let var57 = self.func121(env, var6.wrapping_add(64), var6.wrapping_add(640), 64);
            var57;
            var2 = arg0.wrapping_add(128);
            self.func89(env, var2, var3);
            let var59 = self.func11(env, var2);
            var59;
            let var60 = self.func121(env, arg0, var6.wrapping_add(64), 64);
            let var61 = self.func88(env, var60);
            var2 = var61;
            self.func8(env, var6.wrapping_add(640), var6);
            self.func9(env, var2, var6.wrapping_add(640));
            let var64 = self.func121(env, var3, var6, 64);
            var3 = var64;
            self.func9(env, var3, var2);
            self.func89(env, var3, var6.wrapping_add(64));
            let var67 = self.func11(env, var4);
            let var68 = self.func11(env, var67);
            let var69 = self.func11(env, var68);
            self.func9(env, var3, var69);
            break;
        }
        self.global0 = var1.wrapping_add(704);
        arg0
    }
    fn func87(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func90(env, arg0.wrapping_add(128), 1051256);
        var1
    }
    fn func88(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
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
        let var13 = self.global0;
        var1 = var13.wrapping_sub(192);
        self.global0 = var1;
        let var14 = self.memory.load64(0 as usize + 1051248) as i64;
        self.memory.store64(var1.wrapping_add(128).wrapping_add(24) as usize, var14 as u64);
        let var15 = self.memory.load64(0 as usize + 1051240) as i64;
        self.memory.store64(var1.wrapping_add(128).wrapping_add(16) as usize, var15 as u64);
        let var16 = self.memory.load64(0 as usize + 1051232) as i64;
        self.memory.store64(var1.wrapping_add(128).wrapping_add(8) as usize, var16 as u64);
        let var17 = self.memory.load64(0 as usize + 1051224) as i64;
        let mut slot_var1_128_i64 = var17 as i64;
        self.func12(env, var1.wrapping_add(128));
        'label0: loop {
            'label1: loop {
                let var19 = self.func98(env, 1051192, var1.wrapping_add(128));
                if var19 != 0 {
                    break 'label1;
                }
                var2 = var1.wrapping_add(96).wrapping_add(24);
                var3 = arg0.wrapping_add(24);
                let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                let mut slot_var2_0_i64 = slot_var3_0_i64 as i64;
                var4 = var1.wrapping_add(96).wrapping_add(16);
                var5 = arg0.wrapping_add(16);
                let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
                let mut slot_var4_0_i64 = slot_var5_0_i64 as i64;
                var6 = var1.wrapping_add(96).wrapping_add(8);
                var7 = arg0.wrapping_add(8);
                let mut slot_var7_0_i64 = self.memory.load64(var7 as usize) as i64;
                let mut slot_var6_0_i64 = slot_var7_0_i64 as i64;
                let var20 = self.memory.load64(arg0 as usize) as i64;
                let mut slot_var1_96_i64 = var20 as i64;
                var8 = arg0.wrapping_add(32);
                self.func43(env, var1.wrapping_add(96), var8);
                self.memory.store64(var1.wrapping_add(24) as usize, slot_var2_0_i64 as u64);
                self.memory.store64(var1.wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                self.memory.store64(var1.wrapping_add(8) as usize, slot_var6_0_i64 as u64);
                let mut slot_var1_0_i64 = slot_var1_96_i64 as i64;
                var9 = arg0.wrapping_add(56);
                let mut slot_var9_0_i64 = self.memory.load64(var9 as usize) as i64;
                self.memory.store64(var1.wrapping_add(32).wrapping_add(24) as usize, slot_var9_0_i64 as u64);
                var10 = arg0.wrapping_add(48);
                let mut slot_var10_0_i64 = self.memory.load64(var10 as usize) as i64;
                self.memory.store64(var1.wrapping_add(32).wrapping_add(16) as usize, slot_var10_0_i64 as u64);
                var11 = arg0.wrapping_add(40);
                let mut slot_var11_0_i64 = self.memory.load64(var11 as usize) as i64;
                self.memory.store64(var1.wrapping_add(32).wrapping_add(8) as usize, slot_var11_0_i64 as u64);
                let var22 = self.memory.load64(arg0 as usize + 32) as i64;
                let mut slot_var1_32_i64 = var22 as i64;
                self.memory.store64(var1.wrapping_add(160).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                self.memory.store64(var1.wrapping_add(160).wrapping_add(16) as usize, slot_var5_0_i64 as u64);
                self.memory.store64(var1.wrapping_add(160).wrapping_add(8) as usize, slot_var7_0_i64 as u64);
                let var23 = self.memory.load64(arg0 as usize) as i64;
                let mut slot_var1_160_i64 = var23 as i64;
                self.func12(env, var1.wrapping_add(32));
                self.func43(env, var1.wrapping_add(160), var1.wrapping_add(32));
                slot_var2_0_i64 = slot_var3_0_i64 as i64;
                slot_var4_0_i64 = slot_var5_0_i64 as i64;
                slot_var6_0_i64 = slot_var7_0_i64 as i64;
                let var26 = self.memory.load64(arg0 as usize) as i64;
                slot_var1_96_i64 = var26 as i64;
                self.func70(env, var1.wrapping_add(96), var8);
                var12 = var1.wrapping_add(64).wrapping_add(24);
                let mut slot_var12_0_i64 = slot_var2_0_i64 as i64;
                var2 = var1.wrapping_add(64).wrapping_add(16);
                slot_var2_0_i64 = slot_var4_0_i64 as i64;
                var4 = var1.wrapping_add(64).wrapping_add(8);
                slot_var4_0_i64 = slot_var6_0_i64 as i64;
                let mut slot_var1_64_i64 = slot_var1_96_i64 as i64;
                self.func70(env, var1, var1.wrapping_add(160));
                slot_var9_0_i64 = slot_var12_0_i64 as i64;
                slot_var10_0_i64 = slot_var2_0_i64 as i64;
                slot_var11_0_i64 = slot_var4_0_i64 as i64;
                self.memory.store64(arg0 as usize + 32, slot_var1_64_i64 as u64);
                self.func32(env, var8);
                slot_var3_0_i64 = slot_var12_0_i64 as i64;
                slot_var5_0_i64 = slot_var2_0_i64 as i64;
                slot_var7_0_i64 = slot_var4_0_i64 as i64;
                self.memory.store64(arg0 as usize, slot_var1_64_i64 as u64);
                self.func12(env, arg0);
                self.func13(env, arg0, var1);
                self.func13(env, arg0, var1.wrapping_add(64));
                break 'label0;
                break;
            }
            var2 = arg0.wrapping_add(24);
            self.memory.store64(var1.wrapping_add(24) as usize, slot_var2_0_i64 as u64);
            var3 = arg0.wrapping_add(16);
            self.memory.store64(var1.wrapping_add(16) as usize, slot_var3_0_i64 as u64);
            var4 = arg0.wrapping_add(8);
            self.memory.store64(var1.wrapping_add(8) as usize, slot_var4_0_i64 as u64);
            let var33 = self.memory.load64(arg0 as usize) as i64;
            slot_var1_0_i64 = var33 as i64;
            self.memory.store64(var1.wrapping_add(32).wrapping_add(24) as usize, slot_var2_0_i64 as u64);
            self.memory.store64(var1.wrapping_add(32).wrapping_add(16) as usize, slot_var3_0_i64 as u64);
            self.memory.store64(var1.wrapping_add(32).wrapping_add(8) as usize, slot_var4_0_i64 as u64);
            let var34 = self.memory.load64(arg0 as usize) as i64;
            slot_var1_32_i64 = var34 as i64;
            var2 = arg0.wrapping_add(32);
            self.func43(env, var1.wrapping_add(32), var2);
            let var36 = self.memory.load64(arg0.wrapping_add(56) as usize) as i64;
            self.memory.store64(var1.wrapping_add(96).wrapping_add(24) as usize, var36 as u64);
            let var37 = self.memory.load64(arg0.wrapping_add(48) as usize) as i64;
            self.memory.store64(var1.wrapping_add(96).wrapping_add(16) as usize, var37 as u64);
            let var38 = self.memory.load64(arg0.wrapping_add(40) as usize) as i64;
            self.memory.store64(var1.wrapping_add(96).wrapping_add(8) as usize, var38 as u64);
            let var39 = self.memory.load64(arg0 as usize + 32) as i64;
            slot_var1_96_i64 = var39 as i64;
            self.func13(env, arg0, var1.wrapping_add(96));
            self.func70(env, arg0, var1.wrapping_add(32));
            self.func32(env, var2);
            self.func70(env, var2, var1);
            break;
        }
        self.global0 = var1.wrapping_add(192);
        arg0
    }
    fn func89(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var2 = var9.wrapping_sub(320);
        self.global0 = var2;
        var3 = arg0.wrapping_add(24);
        let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
        self.memory.store64(var2.wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        var4 = arg0.wrapping_add(16);
        let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
        self.memory.store64(var2.wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        var5 = arg0.wrapping_add(8);
        let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
        self.memory.store64(var2.wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        var6 = arg0.wrapping_add(40);
        let mut slot_var6_0_i64 = self.memory.load64(var6 as usize) as i64;
        self.memory.store64(var2.wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        var7 = arg0.wrapping_add(48);
        let mut slot_var7_0_i64 = self.memory.load64(var7 as usize) as i64;
        self.memory.store64(var2.wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        var8 = arg0.wrapping_add(56);
        let mut slot_var8_0_i64 = self.memory.load64(var8 as usize) as i64;
        self.memory.store64(var2.wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var10 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var2_0_i64 = var10 as i64;
        let var11 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var2_32_i64 = var11 as i64;
        self.func12(env, arg0.wrapping_add(32));
        self.memory.store64(var2.wrapping_add(128).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(128).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var13 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var2_128_i64 = var13 as i64;
        let var14 = self.memory.load64(arg0 as usize + 32) as i64;
        let mut slot_var2_160_i64 = var14 as i64;
        var3 = arg1.wrapping_add(56);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(56) as usize, slot_var3_0_i64 as u64);
        var4 = arg1.wrapping_add(48);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(48) as usize, slot_var4_0_i64 as u64);
        var5 = arg1.wrapping_add(40);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(40) as usize, slot_var5_0_i64 as u64);
        var6 = arg1.wrapping_add(8);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(8) as usize, slot_var6_0_i64 as u64);
        var7 = arg1.wrapping_add(16);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(16) as usize, slot_var7_0_i64 as u64);
        var8 = arg1.wrapping_add(24);
        self.memory.store64(var2.wrapping_add(192).wrapping_add(24) as usize, slot_var8_0_i64 as u64);
        let var15 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_224_i64 = var15 as i64;
        let var16 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_192_i64 = var16 as i64;
        self.func41(env, var2.wrapping_add(64), var2.wrapping_add(128), var2.wrapping_add(192));
        self.memory.store64(var2.wrapping_add(256).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
        self.memory.store64(var2.wrapping_add(256).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
        let var18 = self.memory.load64(arg1 as usize + 32) as i64;
        let mut slot_var2_256_i64 = var18 as i64;
        let var19 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_288_i64 = var19 as i64;
        self.func41(env, var2.wrapping_add(64).wrapping_add(32), var2, var2.wrapping_add(256));
        let var21 = self.func121(env, arg0, var2.wrapping_add(64), 64);
        var21;
        self.global0 = var2.wrapping_add(320);
    }
    fn func90(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = 0;
        'label0: loop {
            let var3 = self.func97(env, arg0, arg1);
            if var3 != 0 {
                break 'label0;
            }
            let var4 = self.func97(env, arg0.wrapping_add(32), arg1.wrapping_add(32));
            var2 = var4 ^ 1;
            break;
        }
        var2
    }
    fn func91(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func96(env);
        unreachable!();
    }
    fn func92(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16);
        self.global0 = var2;
        self.func114(env);
        let var7 = self.memory.load32(0 as usize + 1053988) as i32;
        self.func115(env, var2.wrapping_add(8), var7, arg1);
        'label0: loop {
            let mut slot_var2_8_i32 = self.memory.load32(var2 as usize + 8) as i32;
            if (slot_var2_8_i32 & 1 == 0) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_12_i32 = self.memory.load32(var2 as usize + 12) as i32;
            var3 = slot_var2_12_i32;
            var4 = var3.wrapping_add(arg0);
            if ((var4 as u32) < var3 as u32) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    let var9 = self.memory.load32(0 as usize + 1053992) as i32;
                    if (var4 as u32 <= var9 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    let var10 = self.func116(env, arg0, arg1);
                    var3 = var10;
                    break 'label1;
                    break;
                }
                self.memory.store32(0 as usize + 1053988, var4 as u32);
                break;
            }
            self.global0 = var2.wrapping_add(16);
            return var3;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func93(&mut self, env: &Env, mut arg0: i32) {
        self.func94(env, arg0);
        unreachable!();
    }
    fn func94(&mut self, env: &Env, mut arg0: i32) {
        self.func96(env);
        unreachable!();
    }
    fn func95(&mut self, env: &Env, mut arg0: i32) {
        self.func93(env, arg0);
        unreachable!();
    }
    fn func96(&mut self, env: &Env) {
        unreachable!();
    }
    fn func97(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func98(env, arg0, arg1);
        var2 ^ 1
    }
    fn func98(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.func118(env, arg0, arg1, 32);
        (var2 == 0) as i32
    }
    fn func99(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func100(env, arg0, 1051384);
        (var1.wrapping_shl(24 as u32).wrapping_shr(24 as u32) > -1) as i32
    }
    fn func100(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        var2 = 24;
        'label0: loop {
            'label1: loop {
                if (var2 != -8) as i32 != 0 {
                    break 'label1;
                }
                return 0;
                break;
            }
            'label2: loop {
                let var5 = self.memory.load64(arg0.wrapping_add(var2) as usize) as i64;
                var3 = var5;
                let var6 = self.memory.load64(arg1.wrapping_add(var2) as usize) as i64;
                var4 = var6;
                if (var3 as u64 >= var4 as u64) as i32 != 0 {
                    break 'label2;
                }
                return 255;
                break;
            }
            var2 = var2.wrapping_add(-8);
            if (var3 as u64 <= var4 as u64) as i32 != 0 {
                continue 'label0;
            }
            break;
        }
        1
    }
    fn func101(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        'label0: loop {
            'label1: loop {
                let var3 = self.memory.load32(arg0 as usize) as i32;
                arg0 = var3;
                if arg0 != 0 {
                    break 'label1;
                }
                arg0 = 0;
                var1 = var1.wrapping_add(12);
                break 'label0;
                break;
            }
            let mut slot_var1_12_i32 = 1 as i32;
            var1 = var1.wrapping_add(8);
            break;
        }
        let mut slot_var1_0_i32 = arg0 as i32;
    }
    fn func102(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
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
        var2 = var15.wrapping_sub(16);
        self.global0 = var2;
        let var16 = self.memory.load32(arg0 as usize + 8) as i32;
        var3 = var16;
        let var17 = self.memory.load32(arg0 as usize + 4) as i32;
        var4 = var17;
        var5 = 1;
        'label0: loop {
            let var18 = self.memory.load32(arg1 as usize) as i32;
            var6 = var18;
            let var19 = self.memory.load32(arg1 as usize + 4) as i32;
            var7 = var19;
            let mut slot_var7_16_i32 = self.memory.load32(var7 as usize + 16) as i32;
            var8 = slot_var7_16_i32;
            let var20 = /* TODO: call_indirect */ 0;
            if var20 != 0 {
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    if var3 != 0 {
                        break 'label2;
                    }
                    var3 = 0;
                    arg0 = 0;
                    break 'label1;
                    break;
                }
                var9 = 0;
                var10 = 0;
                var11 = var4;
                var12 = var3;
                'label3: loop {
                    'label4: loop {
                        var13 = var11.wrapping_add(var12);
                        arg0 = 0;
                        'label5: loop {
                            'label6: loop {
                                'label7: loop {
                                    var14 = var11.wrapping_add(arg0);
                                    let var21 = self.memory.load8(var14 as usize) as i32;
                                    arg1 = var21;
                                    if (((arg1.wrapping_add(-127) & 255) as u32) < 161 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if (arg1 == 34) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if (arg1 == 92) as i32 != 0 {
                                        break 'label6;
                                    }
                                    arg0 = arg0.wrapping_add(1);
                                    if (var12 != arg0) as i32 != 0 {
                                        continue 'label7;
                                    }
                                    break;
                                }
                                var10 = var10.wrapping_add(var12);
                                break 'label5;
                                break;
                            }
                            var10 = arg0.wrapping_add(var10);
                            'label8: loop {
                                'label9: loop {
                                    'label10: loop {
                                        let var22 = self.memory.load8(var14 as usize) as i8 as i32;
                                        arg0 = var22;
                                        if (arg0 <= -1) as i32 != 0 {
                                            break 'label10;
                                        }
                                        var11 = var14.wrapping_add(1);
                                        arg0 = arg0 & 255;
                                        break 'label9;
                                        break;
                                    }
                                    let var23 = self.memory.load8(var14 as usize + 1) as i32;
                                    arg1 = var23 & 63;
                                    var12 = arg0 & 31;
                                    'label11: loop {
                                        if (arg0 as u32 > -33 as u32) as i32 != 0 {
                                            break 'label11;
                                        }
                                        arg0 = var12.wrapping_shl(6 as u32) | arg1;
                                        var11 = var14.wrapping_add(2);
                                        break 'label9;
                                        break;
                                    }
                                    let var24 = self.memory.load8(var14 as usize + 2) as i32;
                                    arg1 = arg1.wrapping_shl(6 as u32) | var24 & 63;
                                    'label12: loop {
                                        if (arg0 as u32 >= -16 as u32) as i32 != 0 {
                                            break 'label12;
                                        }
                                        arg0 = arg1 | var12.wrapping_shl(12 as u32);
                                        var11 = var14.wrapping_add(3);
                                        break 'label9;
                                        break;
                                    }
                                    var11 = var14.wrapping_add(4);
                                    let var25 = self.memory.load8(var14 as usize + 3) as i32;
                                    arg0 = arg1.wrapping_shl(6 as u32) | var25 & 63 | var12.wrapping_shl(18 as u32) & 1835008;
                                    if (arg0 == 1114112) as i32 != 0 {
                                        break 'label8;
                                    }
                                    break;
                                }
                                self.func103(env, var2, arg0, 65537);
                                'label13: loop {
                                    let var27 = self.memory.load8(var2 as usize + 13) as i32;
                                    let var28 = self.memory.load8(var2 as usize + 12) as i32;
                                    if (var27.wrapping_sub(var28) & 255 == 1) as i32 != 0 {
                                        break 'label13;
                                    }
                                    if ((var10 as u32) < var9 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    'label14: loop {
                                        if (var9 == 0) as i32 != 0 {
                                            break 'label14;
                                        }
                                        'label15: loop {
                                            if ((var9 as u32) < var3 as u32) as i32 != 0 {
                                                break 'label15;
                                            }
                                            if (var9 != var3) as i32 != 0 {
                                                break 'label3;
                                            }
                                            break 'label14;
                                            break;
                                        }
                                        let var29 = self.memory.load8(var4.wrapping_add(var9) as usize) as i8 as i32;
                                        if (var29 <= -65) as i32 != 0 {
                                            break 'label3;
                                        }
                                        break;
                                    }
                                    'label16: loop {
                                        if (var10 == 0) as i32 != 0 {
                                            break 'label16;
                                        }
                                        'label17: loop {
                                            if ((var10 as u32) < var3 as u32) as i32 != 0 {
                                                break 'label17;
                                            }
                                            if (var10 == var3) as i32 != 0 {
                                                break 'label16;
                                            }
                                            break 'label3;
                                            break;
                                        }
                                        let var30 = self.memory.load8(var4.wrapping_add(var10) as usize) as i8 as i32;
                                        if (var30 <= -65) as i32 != 0 {
                                            break 'label3;
                                        }
                                        break;
                                    }
                                    'label18: loop {
                                        let mut slot_var7_12_i32 = self.memory.load32(var7 as usize + 12) as i32;
                                        arg1 = slot_var7_12_i32;
                                        let var31 = /* TODO: call_indirect */ 0;
                                        if var31 != 0 {
                                            break 'label18;
                                        }
                                        'label19: loop {
                                            'label20: loop {
                                                let var32 = self.memory.load8(var2 as usize + 13) as i32;
                                                var14 = var32;
                                                if ((var14 as u32) < 129 as u32) as i32 != 0 {
                                                    break 'label20;
                                                }
                                                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                                                let var33 = /* TODO: call_indirect */ 0;
                                                if var33 != 0 {
                                                    break 'label18;
                                                }
                                                break 'label19;
                                                break;
                                            }
                                            let var34 = self.memory.load8(var2 as usize + 12) as i32;
                                            var12 = var34;
                                            let var35 = /* TODO: call_indirect */ 0;
                                            if var35 != 0 {
                                                break 'label18;
                                            }
                                            break;
                                        }
                                        'label21: loop {
                                            'label22: loop {
                                                if (arg0 as u32 >= 128 as u32) as i32 != 0 {
                                                    break 'label22;
                                                }
                                                arg1 = 1;
                                                break 'label21;
                                                break;
                                            }
                                            'label23: loop {
                                                if (arg0 as u32 >= 2048 as u32) as i32 != 0 {
                                                    break 'label23;
                                                }
                                                arg1 = 2;
                                                break 'label21;
                                                break;
                                            }
                                            arg1 = { let a = 3; let b = 4; if ((arg0 as u32) < 65536 as u32) as i32 != 0 { a } else { b } };
                                            break;
                                        }
                                        var9 = arg1.wrapping_add(var10);
                                        break 'label13;
                                        break;
                                    }
                                    var5 = 1;
                                    break 'label0;
                                    break;
                                }
                                'label24: loop {
                                    'label25: loop {
                                        if (arg0 as u32 >= 128 as u32) as i32 != 0 {
                                            break 'label25;
                                        }
                                        arg0 = 1;
                                        break 'label24;
                                        break;
                                    }
                                    'label26: loop {
                                        if (arg0 as u32 >= 2048 as u32) as i32 != 0 {
                                            break 'label26;
                                        }
                                        arg0 = 2;
                                        break 'label24;
                                        break;
                                    }
                                    arg0 = { let a = 3; let b = 4; if ((arg0 as u32) < 65536 as u32) as i32 != 0 { a } else { b } };
                                    break;
                                }
                                var10 = arg0.wrapping_add(var10);
                                break;
                            }
                            var12 = var13.wrapping_sub(var11);
                            if var12 != 0 {
                                continue 'label4;
                            }
                            break;
                        }
                        break;
                    }
                    if (var9 as u32 > var10 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    arg0 = 0;
                    'label27: loop {
                        if (var9 == 0) as i32 != 0 {
                            break 'label27;
                        }
                        'label28: loop {
                            if ((var9 as u32) < var3 as u32) as i32 != 0 {
                                break 'label28;
                            }
                            arg0 = var3;
                            if (var9 != var3) as i32 != 0 {
                                break 'label3;
                            }
                            break 'label27;
                            break;
                        }
                        arg0 = var9;
                        let var36 = self.memory.load8(var4.wrapping_add(var9) as usize) as i8 as i32;
                        if (var36 <= -65) as i32 != 0 {
                            break 'label3;
                        }
                        break;
                    }
                    'label29: loop {
                        if var10 != 0 {
                            break 'label29;
                        }
                        var3 = 0;
                        break 'label1;
                        break;
                    }
                    'label30: loop {
                        'label31: loop {
                            if ((var10 as u32) < var3 as u32) as i32 != 0 {
                                break 'label31;
                            }
                            if (var10 == var3) as i32 != 0 {
                                break 'label1;
                            }
                            break 'label30;
                            break;
                        }
                        let var37 = self.memory.load8(var4.wrapping_add(var10) as usize) as i8 as i32;
                        if (var37 <= -65) as i32 != 0 {
                            break 'label30;
                        }
                        var3 = var10;
                        break 'label1;
                        break;
                    }
                    var9 = arg0;
                    break;
                }
                self.func104(env, var4, var3, var9, var10);
                unreachable!();
                break;
            }
            let var39 = /* TODO: call_indirect */ 0;
            if var39 != 0 {
                break 'label0;
            }
            let var40 = /* TODO: call_indirect */ 0;
            var5 = var40;
            break;
        }
        self.global0 = var2.wrapping_add(16);
        var5
    }
    fn func103(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        match arg1 {
                                                                            0 => break 'label13,
                                                                            1 => break 'label14,
                                                                            2 => break 'label14,
                                                                            3 => break 'label14,
                                                                            4 => break 'label14,
                                                                            5 => break 'label14,
                                                                            6 => break 'label14,
                                                                            7 => break 'label14,
                                                                            8 => break 'label14,
                                                                            9 => break 'label12,
                                                                            10 => break 'label10,
                                                                            11 => break 'label14,
                                                                            12 => break 'label14,
                                                                            13 => break 'label11,
                                                                            14 => break 'label14,
                                                                            15 => break 'label14,
                                                                            16 => break 'label14,
                                                                            17 => break 'label14,
                                                                            18 => break 'label14,
                                                                            19 => break 'label14,
                                                                            20 => break 'label14,
                                                                            21 => break 'label14,
                                                                            22 => break 'label14,
                                                                            23 => break 'label14,
                                                                            24 => break 'label14,
                                                                            25 => break 'label14,
                                                                            26 => break 'label14,
                                                                            27 => break 'label14,
                                                                            28 => break 'label14,
                                                                            29 => break 'label14,
                                                                            30 => break 'label14,
                                                                            31 => break 'label14,
                                                                            32 => break 'label14,
                                                                            33 => break 'label14,
                                                                            34 => break 'label6,
                                                                            35 => break 'label14,
                                                                            36 => break 'label14,
                                                                            37 => break 'label14,
                                                                            38 => break 'label14,
                                                                            39 => break 'label8,
                                                                            _ => break 'label15,
                                                                        }
                                                                        break;
                                                                    }
                                                                    if (arg1 == 92) as i32 != 0 {
                                                                        break 'label9;
                                                                    }
                                                                    break;
                                                                }
                                                                'label16: loop {
                                                                    if (arg2 & 1 == 0) as i32 != 0 {
                                                                        break 'label16;
                                                                    }
                                                                    if (arg1 as u32 > 767 as u32) as i32 != 0 {
                                                                        break 'label7;
                                                                    }
                                                                    break;
                                                                }
                                                                if ((arg1 as u32) < 32 as u32) as i32 != 0 {
                                                                    break 'label3;
                                                                }
                                                                if ((arg1 as u32) < 127 as u32) as i32 != 0 {
                                                                    break 'label1;
                                                                }
                                                                break 'label4;
                                                                break;
                                                            }
                                                            self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                                            self.memory.store16(arg0 as usize, 12380 as u16);
                                                            break 'label5;
                                                            break;
                                                        }
                                                        self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                                        self.memory.store16(arg0 as usize, 29788 as u16);
                                                        break 'label5;
                                                        break;
                                                    }
                                                    self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                                    self.memory.store16(arg0 as usize, 29276 as u16);
                                                    break 'label5;
                                                    break;
                                                }
                                                self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                                self.memory.store16(arg0 as usize, 28252 as u16);
                                                break 'label5;
                                                break;
                                            }
                                            self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                            self.memory.store16(arg0 as usize, 23644 as u16);
                                            break 'label5;
                                            break;
                                        }
                                        if (arg2 & 256 == 0) as i32 != 0 {
                                            break 'label1;
                                        }
                                        self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                        self.memory.store16(arg0 as usize, 10076 as u16);
                                        break 'label5;
                                        break;
                                    }
                                    let var6 = self.func109(env, arg1);
                                    if (var6 == 0) as i32 != 0 {
                                        break 'label4;
                                    }
                                    self.memory.store8(var3.wrapping_add(12).wrapping_add(2) as usize, 0 as u8);
                                    self.memory.store16(var3 as usize + 12, 0 as u16);
                                    let var7 = self.memory.load8(((arg1 as u32).wrapping_shr(20 as u32) as i32).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var3 as usize + 15, var7 as u8);
                                    let var8 = self.memory.load8(((arg1 as u32).wrapping_shr(4 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var3 as usize + 19, var8 as u8);
                                    let var9 = self.memory.load8(((arg1 as u32).wrapping_shr(8 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var3 as usize + 18, var9 as u8);
                                    let var10 = self.memory.load8(((arg1 as u32).wrapping_shr(12 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var3 as usize + 17, var10 as u8);
                                    let var11 = self.memory.load8(((arg1 as u32).wrapping_shr(16 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var3 as usize + 16, var11 as u8);
                                    arg2 = ((arg1 | 1).leading_zeros() as i32 as u32).wrapping_shr(2 as u32) as i32;
                                    var4 = var3.wrapping_add(12).wrapping_add(arg2);
                                    self.memory.store8(var4 as usize, 123 as u8);
                                    self.memory.store8(var4.wrapping_add(-1) as usize, 117 as u8);
                                    arg2 = arg2.wrapping_add(-2);
                                    self.memory.store8(var3.wrapping_add(12).wrapping_add(arg2) as usize, 92 as u8);
                                    var4 = var3.wrapping_add(12).wrapping_add(8);
                                    let var12 = self.memory.load8((arg1 & 15).wrapping_add(1052387) as usize) as i32;
                                    self.memory.store8(var4 as usize, var12 as u8);
                                    let mut slot_var3_12_i64 = self.memory.load64(var3 as usize + 12) as i64;
                                    self.memory.store64(arg0 as usize, slot_var3_12_i64 as u64);
                                    self.memory.store8(var3 as usize + 21, 125 as u8);
                                    let var13 = self.memory.load16(var4 as usize) as i32;
                                    self.memory.store16(arg0.wrapping_add(8) as usize, var13 as u16);
                                    break 'label2;
                                    break;
                                }
                                if (((arg2 & 16777215) as u32) < 65536 as u32) as i32 != 0 {
                                    break 'label1;
                                }
                                self.memory.store64(arg0 as usize + 2, 0 /* False */ as u64);
                                self.memory.store16(arg0 as usize, 8796 as u16);
                                break;
                            }
                            arg1 = 2;
                            arg2 = 0;
                            break 'label0;
                            break;
                        }
                        'label17: loop {
                            if ((arg1 as u32) < 65536 as u32) as i32 != 0 {
                                break 'label17;
                            }
                            'label18: loop {
                                if (arg1 as u32 >= 131072 as u32) as i32 != 0 {
                                    break 'label18;
                                }
                                let var14 = self.func110(env, arg1, 1052403, 44, 1052491, 208, 1052699, 486);
                                if (var14 == 0) as i32 != 0 {
                                    break 'label3;
                                }
                                break 'label1;
                                break;
                            }
                            if (arg1 & 2097150 == 178206) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1 & 2097120 == 173792) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-177984) as u32 > -7 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-183984) as u32 > -15 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-191472) as u32 > -16 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-194560) as u32 > -2467 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-196608) as u32 > -1507 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-201552) as u32 > -6 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1.wrapping_add(-917760) as u32 > -712017 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (arg1 as u32 >= 918000 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            break 'label1;
                            break;
                        }
                        let var15 = self.func110(env, arg1, 1053185, 40, 1053265, 290, 1053555, 297);
                        if var15 != 0 {
                            break 'label1;
                        }
                        break;
                    }
                    self.memory.store8(var3.wrapping_add(22).wrapping_add(2) as usize, 0 as u8);
                    self.memory.store16(var3 as usize + 22, 0 as u16);
                    let var16 = self.memory.load8(((arg1 as u32).wrapping_shr(20 as u32) as i32).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var3 as usize + 25, var16 as u8);
                    let var17 = self.memory.load8(((arg1 as u32).wrapping_shr(4 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var3 as usize + 29, var17 as u8);
                    let var18 = self.memory.load8(((arg1 as u32).wrapping_shr(8 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var3 as usize + 28, var18 as u8);
                    let var19 = self.memory.load8(((arg1 as u32).wrapping_shr(12 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var3 as usize + 27, var19 as u8);
                    let var20 = self.memory.load8(((arg1 as u32).wrapping_shr(16 as u32) as i32 & 15).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var3 as usize + 26, var20 as u8);
                    arg2 = ((arg1 | 1).leading_zeros() as i32 as u32).wrapping_shr(2 as u32) as i32;
                    var4 = var3.wrapping_add(22).wrapping_add(arg2);
                    self.memory.store8(var4 as usize, 123 as u8);
                    self.memory.store8(var4.wrapping_add(-1) as usize, 117 as u8);
                    arg2 = arg2.wrapping_add(-2);
                    self.memory.store8(var3.wrapping_add(22).wrapping_add(arg2) as usize, 92 as u8);
                    var4 = var3.wrapping_add(22).wrapping_add(8);
                    let var21 = self.memory.load8((arg1 & 15).wrapping_add(1052387) as usize) as i32;
                    self.memory.store8(var4 as usize, var21 as u8);
                    let mut slot_var3_22_i64 = self.memory.load64(var3 as usize + 22) as i64;
                    self.memory.store64(arg0 as usize, slot_var3_22_i64 as u64);
                    self.memory.store8(var3 as usize + 31, 125 as u8);
                    let var22 = self.memory.load16(var4 as usize) as i32;
                    self.memory.store16(arg0.wrapping_add(8) as usize, var22 as u16);
                    break;
                }
                arg1 = 10;
                break 'label0;
                break;
            }
            self.memory.store32(arg0 as usize, arg1 as u32);
            arg1 = 129;
            arg2 = 128;
            break;
        }
        self.memory.store8(arg0 as usize + 13, arg1 as u8);
        self.memory.store8(arg0 as usize + 12, arg2 as u8);
        self.global0 = var3.wrapping_add(32);
    }
    fn func104(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        self.func112(env, arg0, arg1, arg2, arg3);
        unreachable!();
    }
    fn func105(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let var2 = self.memory.load32(arg0 as usize + 4) as i32;
        let var3 = self.memory.load32(arg0 as usize + 8) as i32;
        let var4 = self.func106(env, arg1, var2, var3);
        var4
    }
    fn func106(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
        'label0: loop {
            'label1: loop {
                let var14 = self.memory.load32(arg0 as usize + 8) as i32;
                var3 = var14;
                if (var3 & 402653184 == 0) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    if (var3 & 268435456 == 0) as i32 != 0 {
                                        break 'label6;
                                    }
                                    let var15 = self.memory.load16(arg0 as usize + 14) as i32;
                                    var4 = var15;
                                    if var4 != 0 {
                                        break 'label5;
                                    }
                                    arg2 = 0;
                                    break 'label4;
                                    break;
                                }
                                'label7: loop {
                                    if ((arg2 as u32) < 16 as u32) as i32 != 0 {
                                        break 'label7;
                                    }
                                    var5 = arg1.wrapping_add(3) & -4;
                                    var4 = arg1.wrapping_sub(var5);
                                    var6 = arg2.wrapping_add(var4);
                                    var7 = var6 & 3;
                                    var8 = 0;
                                    var9 = 0;
                                    'label8: loop {
                                        if (arg1 == var5) as i32 != 0 {
                                            break 'label8;
                                        }
                                        var9 = 0;
                                        var10 = arg1;
                                        'label9: loop {
                                            let var16 = self.memory.load8(var10 as usize) as i8 as i32;
                                            var9 = var9.wrapping_add((var16 > -65) as i32);
                                            var10 = var10.wrapping_add(1);
                                            var4 = var4.wrapping_add(1);
                                            if var4 != 0 {
                                                continue 'label9;
                                            }
                                            break;
                                        }
                                        break;
                                    }
                                    'label10: loop {
                                        if (var7 == 0) as i32 != 0 {
                                            break 'label10;
                                        }
                                        var10 = var5.wrapping_add(var6 & -4);
                                        var8 = 0;
                                        'label11: loop {
                                            let var17 = self.memory.load8(var10 as usize) as i8 as i32;
                                            var8 = var8.wrapping_add((var17 > -65) as i32);
                                            var10 = var10.wrapping_add(1);
                                            var7 = var7.wrapping_add(-1);
                                            if var7 != 0 {
                                                continue 'label11;
                                            }
                                            break;
                                        }
                                        break;
                                    }
                                    var4 = (var6 as u32).wrapping_shr(2 as u32) as i32;
                                    var9 = var8.wrapping_add(var9);
                                    'label12: loop {
                                        var6 = var5;
                                        if (var4 == 0) as i32 != 0 {
                                            break 'label2;
                                        }
                                        var11 = { let a = var4; let b = 192; if ((var4 as u32) < 192 as u32) as i32 != 0 { a } else { b } };
                                        var12 = var11 & 3;
                                        var13 = var11.wrapping_shl(2 as u32);
                                        var8 = 0;
                                        'label13: loop {
                                            if ((var4 as u32) < 4 as u32) as i32 != 0 {
                                                break 'label13;
                                            }
                                            var5 = var6.wrapping_add(var13 & 1008);
                                            var8 = 0;
                                            var10 = var6;
                                            'label14: loop {
                                                let var18 = self.memory.load32(var10.wrapping_add(12) as usize) as i32;
                                                var7 = var18;
                                                let var19 = var7;
                                                let var20 = self.memory.load32(var10.wrapping_add(8) as usize) as i32;
                                                var7 = var20;
                                                let var21 = var7;
                                                let var22 = self.memory.load32(var10.wrapping_add(4) as usize) as i32;
                                                var7 = var22;
                                                let var23 = var7;
                                                let mut slot_var10_0_i32 = self.memory.load32(var10 as usize) as i32;
                                                var7 = slot_var10_0_i32;
                                                var8 = ((((var19 ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (var7 as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add(((((var21 ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (var7 as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add(((((var23 ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (var7 as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add(((((var7 ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (var7 as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add(var8))));
                                                var10 = var10.wrapping_add(16);
                                                if (var10 != var5) as i32 != 0 {
                                                    continue 'label14;
                                                }
                                                break;
                                            }
                                            break;
                                        }
                                        var4 = var4.wrapping_sub(var11);
                                        var5 = var6.wrapping_add(var13);
                                        var9 = ((((var8 as u32).wrapping_shr(8 as u32) as i32 & 16711935).wrapping_add(var8 & 16711935).wrapping_mul(65537) as u32).wrapping_shr(16 as u32) as i32).wrapping_add(var9);
                                        if (var12 == 0) as i32 != 0 {
                                            continue 'label12;
                                        }
                                        break;
                                    }
                                    var7 = var12.wrapping_shl(2 as u32);
                                    var10 = var6.wrapping_add((var11 & 252).wrapping_shl(2 as u32));
                                    var8 = 0;
                                    'label15: loop {
                                        var5 = slot_var10_0_i32;
                                        var8 = ((((var5 ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (var5 as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add(var8);
                                        var10 = var10.wrapping_add(4);
                                        var7 = var7.wrapping_add(-4);
                                        if var7 != 0 {
                                            continue 'label15;
                                        }
                                        break;
                                    }
                                    var9 = ((((var8 as u32).wrapping_shr(8 as u32) as i32 & 16711935).wrapping_add(var8 & 16711935).wrapping_mul(65537) as u32).wrapping_shr(16 as u32) as i32).wrapping_add(var9);
                                    break 'label2;
                                    break;
                                }
                                'label16: loop {
                                    if arg2 != 0 {
                                        break 'label16;
                                    }
                                    arg2 = 0;
                                    var9 = 0;
                                    break 'label2;
                                    break;
                                }
                                var9 = 0;
                                var10 = 0;
                                'label17: loop {
                                    let var24 = self.memory.load8(arg1.wrapping_add(var10) as usize) as i8 as i32;
                                    var9 = var9.wrapping_add((var24 > -65) as i32);
                                    var10 = var10.wrapping_add(1);
                                    if (arg2 != var10) as i32 != 0 {
                                        continue 'label17;
                                    }
                                    break 'label2;
                                    break;
                                }
                                break;
                            }
                            var5 = arg1.wrapping_add(arg2);
                            arg2 = 0;
                            var8 = arg1;
                            var7 = var4;
                            'label18: loop {
                                var10 = var8;
                                if (var10 == var5) as i32 != 0 {
                                    break 'label3;
                                }
                                'label19: loop {
                                    'label20: loop {
                                        let var25 = self.memory.load8(var10 as usize) as i8 as i32;
                                        var8 = var25;
                                        if (var8 <= -1) as i32 != 0 {
                                            break 'label20;
                                        }
                                        var8 = var10.wrapping_add(1);
                                        break 'label19;
                                        break;
                                    }
                                    'label21: loop {
                                        if (var8 as u32 >= -32 as u32) as i32 != 0 {
                                            break 'label21;
                                        }
                                        var8 = var10.wrapping_add(2);
                                        break 'label19;
                                        break;
                                    }
                                    'label22: loop {
                                        if (var8 as u32 >= -16 as u32) as i32 != 0 {
                                            break 'label22;
                                        }
                                        var8 = var10.wrapping_add(3);
                                        break 'label19;
                                        break;
                                    }
                                    var8 = var10.wrapping_add(4);
                                    break;
                                }
                                arg2 = var8.wrapping_sub(var10).wrapping_add(arg2);
                                var7 = var7.wrapping_add(-1);
                                if var7 != 0 {
                                    continue 'label18;
                                }
                                break;
                            }
                            break;
                        }
                        var7 = 0;
                        break;
                    }
                    var9 = var4.wrapping_sub(var7);
                    break;
                }
                let var26 = self.memory.load16(arg0 as usize + 12) as i32;
                var10 = var26;
                if (var9 as u32 >= var10 as u32) as i32 != 0 {
                    break 'label1;
                }
                var6 = var10.wrapping_sub(var9);
                var10 = 0;
                var9 = 0;
                'label23: loop {
                    'label24: loop {
                        'label25: loop {
                            match (var3 as u32).wrapping_shr(29 as u32) as i32 & 3 {
                                0 => break 'label23,
                                1 => break 'label25,
                                2 => break 'label24,
                                3 => break 'label23,
                                _ => break 'label23,
                            }
                            break;
                        }
                        var9 = var6;
                        break 'label23;
                        break;
                    }
                    var9 = ((var6 & 65534) as u32).wrapping_shr(1 as u32) as i32;
                    break;
                }
                var4 = var3 & 2097151;
                let var27 = self.memory.load32(arg0 as usize + 4) as i32;
                var7 = var27;
                let var28 = self.memory.load32(arg0 as usize) as i32;
                var5 = var28;
                'label26: loop {
                    'label27: loop {
                        if ((var10 & 65535) as u32 >= (var9 & 65535) as u32) as i32 != 0 {
                            break 'label26;
                        }
                        var8 = 1;
                        var10 = var10.wrapping_add(1);
                        let mut slot_var7_16_i32 = self.memory.load32(var7 as usize + 16) as i32;
                        let var29 = /* TODO: call_indirect */ 0;
                        if var29 != 0 {
                            break 'label0;
                        }
                        continue 'label27;
                        break;
                    }
                    break;
                }
                var8 = 1;
                let mut slot_var7_12_i32 = self.memory.load32(var7 as usize + 12) as i32;
                let var30 = /* TODO: call_indirect */ 0;
                if var30 != 0 {
                    break 'label0;
                }
                var9 = var6.wrapping_sub(var9) & 65535;
                var10 = 0;
                'label28: loop {
                    'label29: loop {
                        if (((var10 & 65535) as u32) < var9 as u32) as i32 != 0 {
                            break 'label29;
                        }
                        return 0;
                        break;
                    }
                    var8 = 1;
                    var10 = var10.wrapping_add(1);
                    let var31 = /* TODO: call_indirect */ 0;
                    if var31 != 0 {
                        break 'label0;
                    }
                    continue 'label28;
                    break;
                }
                break;
            }
            let var32 = self.memory.load32(arg0 as usize) as i32;
            let var33 = self.memory.load32(arg0 as usize + 4) as i32;
            let mut slot_var33_12_i32 = self.memory.load32(var33 as usize + 12) as i32;
            let var34 = /* TODO: call_indirect */ 0;
            var8 = var34;
            break;
        }
        var8
    }
    fn func107(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.memory.store32(arg0 as usize, 0 as u32);
    }
    fn func108(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func96(env);
        unreachable!();
    }
    fn func109(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        var1 = 0;
        var2 = { let a = 0; let b = 17; if ((arg0 as u32) < 71727 as u32) as i32 != 0 { a } else { b } };
        let var6 = var2;
        var2 = var2 | 8;
        let var7 = var2;
        let var8 = self.memory.load32(var2.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var2 = arg0.wrapping_shl(11 as u32);
        var3 = { let a = var6; let b = var7; if (var8.wrapping_shl(11 as u32) as u32 > var2 as u32) as i32 != 0 { a } else { b } };
        let var9 = var3;
        var3 = var3 | 4;
        let var10 = self.memory.load32(var3.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var3 = { let a = var9; let b = var3; if (var10.wrapping_shl(11 as u32) as u32 > var2 as u32) as i32 != 0 { a } else { b } };
        let var11 = var3;
        var3 = var3 | 2;
        let var12 = self.memory.load32(var3.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var3 = { let a = var11; let b = var3; if (var12.wrapping_shl(11 as u32) as u32 > var2 as u32) as i32 != 0 { a } else { b } };
        let var13 = var3;
        var3 = var3.wrapping_add(1);
        let var14 = self.memory.load32(var3.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var3 = { let a = var13; let b = var3; if (var14.wrapping_shl(11 as u32) as u32 > var2 as u32) as i32 != 0 { a } else { b } };
        let var15 = var3;
        var3 = var3.wrapping_add(1);
        let var16 = self.memory.load32(var3.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var3 = { let a = var15; let b = var3; if (var16.wrapping_shl(11 as u32) as u32 > var2 as u32) as i32 != 0 { a } else { b } };
        let var17 = self.memory.load32(var3.wrapping_shl(2 as u32).wrapping_add(1053852) as usize) as i32;
        var4 = var17.wrapping_shl(11 as u32);
        var3 = ((var4 == var2) as i32).wrapping_add(((var4 as u32) < var2 as u32) as i32).wrapping_add(var3);
        var5 = var3.wrapping_shl(2 as u32).wrapping_add(1053852);
        let mut slot_var5_0_i32 = self.memory.load32(var5 as usize) as i32;
        var2 = (slot_var5_0_i32 as u32).wrapping_shr(21 as u32) as i32;
        var4 = 751;
        'label0: loop {
            'label1: loop {
                if (var3 as u32 > 32 as u32) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var5_4_i32 = self.memory.load32(var5 as usize + 4) as i32;
                var4 = (slot_var5_4_i32 as u32).wrapping_shr(21 as u32) as i32;
                if (var3 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            let var18 = self.memory.load32(var5.wrapping_add(-4) as usize) as i32;
            var1 = var18 & 2097151;
            break;
        }
        'label2: loop {
            if (var4.wrapping_add(var2 ^ -1) == 0) as i32 != 0 {
                break 'label2;
            }
            var3 = arg0.wrapping_sub(var1);
            var4 = var4.wrapping_add(-1);
            arg0 = 0;
            'label3: loop {
                let var19 = self.memory.load8(var2.wrapping_add(1051636) as usize) as i32;
                arg0 = arg0.wrapping_add(var19);
                if (arg0 as u32 > var3 as u32) as i32 != 0 {
                    break 'label2;
                }
                var2 = var2.wrapping_add(1);
                if (var4 != var2) as i32 != 0 {
                    continue 'label3;
                }
                break;
            }
            break;
        }
        var2 & 1
    }
    fn func110(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32, mut arg5: i32, mut arg6: i32) -> i32 {
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        var7 = arg1.wrapping_add(arg2.wrapping_shl(1 as u32));
        var8 = ((arg0 & 65280) as u32).wrapping_shr(8 as u32) as i32;
        var9 = 0;
        var10 = arg0 & 255;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            var11 = arg1.wrapping_add(2);
                            let var13 = self.memory.load8(arg1 as usize + 1) as i32;
                            arg2 = var13;
                            var12 = var9.wrapping_add(arg2);
                            'label5: loop {
                                let var14 = self.memory.load8(arg1 as usize) as i32;
                                arg1 = var14;
                                if (arg1 == var8) as i32 != 0 {
                                    break 'label5;
                                }
                                if (arg1 as u32 > var8 as u32) as i32 != 0 {
                                    break 'label1;
                                }
                                var9 = var12;
                                arg1 = var11;
                                if (var11 != var7) as i32 != 0 {
                                    continue 'label4;
                                }
                                break 'label1;
                                break;
                            }
                            if ((var12 as u32) < var9 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (var12 as u32 > arg4 as u32) as i32 != 0 {
                                break 'label2;
                            }
                            arg1 = arg3.wrapping_add(var9);
                            'label6: loop {
                                'label7: loop {
                                    if arg2 != 0 {
                                        break 'label7;
                                    }
                                    var9 = var12;
                                    arg1 = var11;
                                    if (var11 != var7) as i32 != 0 {
                                        continue 'label4;
                                    }
                                    break 'label1;
                                    break;
                                }
                                arg2 = arg2.wrapping_add(-1);
                                let var15 = self.memory.load8(arg1 as usize) as i32;
                                var9 = var15;
                                arg1 = arg1.wrapping_add(1);
                                if (var9 != var10) as i32 != 0 {
                                    continue 'label6;
                                }
                                break;
                            }
                            break;
                        }
                        arg2 = 0;
                        break 'label0;
                        break;
                    }
                    self.func111(env, var9, var12);
                    unreachable!();
                    break;
                }
                self.func108(env, var12, arg4);
                unreachable!();
                break;
            }
            var9 = arg0 & 65535;
            var12 = arg5.wrapping_add(arg6);
            arg2 = 1;
            'label8: loop {
                var10 = arg5.wrapping_add(1);
                'label9: loop {
                    'label10: loop {
                        let var18 = self.memory.load8(arg5 as usize) as i8 as i32;
                        arg1 = var18;
                        if (arg1 < 0) as i32 != 0 {
                            break 'label10;
                        }
                        arg5 = var10;
                        break 'label9;
                        break;
                    }
                    'label11: loop {
                        if (var10 == var12) as i32 != 0 {
                            break 'label11;
                        }
                        let var19 = self.memory.load8(arg5 as usize + 1) as i32;
                        arg1 = (arg1 & 127).wrapping_shl(8 as u32) | var19;
                        arg5 = arg5.wrapping_add(2);
                        break 'label9;
                        break;
                    }
                    self.func37(env);
                    unreachable!();
                    break;
                }
                var9 = var9.wrapping_sub(arg1);
                if (var9 < 0) as i32 != 0 {
                    break 'label0;
                }
                arg2 = arg2 ^ 1;
                if (arg5 != var12) as i32 != 0 {
                    continue 'label8;
                }
                break;
            }
            break;
        }
        arg2 & 1
    }
    fn func111(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func96(env);
        unreachable!();
    }
    fn func112(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if ((arg1 as u32) < 257 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    let var6 = self.memory.load8(arg0 as usize + 256) as i8 as i32;
                    if (var6 > -65) as i32 != 0 {
                        break 'label2;
                    }
                    let var7 = self.memory.load8(arg0 as usize + 255) as i8 as i32;
                    if (var7 > -65) as i32 != 0 {
                        break 'label2;
                    }
                    let var8 = self.memory.load8(arg0 as usize + 254) as i8 as i32;
                    var4 = { let a = 254; let b = 253; if (var8 > -65) as i32 != 0 { a } else { b } };
                    let var9 = self.memory.load8(arg0.wrapping_add(var4) as usize) as i8 as i32;
                    if (var9 <= -65) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            if (arg2 as u32 > arg1 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            if (arg3 as u32 > arg1 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            if (arg2 as u32 > arg3 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            'label6: loop {
                                if (arg2 == 0) as i32 != 0 {
                                    break 'label6;
                                }
                                if (arg2 as u32 >= arg1 as u32) as i32 != 0 {
                                    break 'label6;
                                }
                                let var10 = self.memory.load8(arg0.wrapping_add(arg2) as usize) as i8 as i32;
                                arg3 = { let a = arg3; let b = arg2; if (var10 > -65) as i32 != 0 { a } else { b } };
                                break;
                            }
                            if (arg3 as u32 >= arg1 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            var4 = arg3.wrapping_add(1);
                            arg2 = arg3.wrapping_add(-3);
                            var5 = { let a = 0; let b = arg2; if (arg2 as u32 > arg3 as u32) as i32 != 0 { a } else { b } };
                            if ((var4 as u32) < var5 as u32) as i32 != 0 {
                                break 'label4;
                            }
                            arg2 = arg0.wrapping_add(arg3);
                            arg3 = var4.wrapping_sub(var5);
                            'label7: loop {
                                'label8: loop {
                                    if (arg3 == 0) as i32 != 0 {
                                        break 'label7;
                                    }
                                    arg3 = arg3.wrapping_add(-1);
                                    let var11 = self.memory.load8(arg2 as usize) as i8 as i32;
                                    var4 = var11;
                                    arg2 = arg2.wrapping_add(-1);
                                    if (var4 < -64) as i32 != 0 {
                                        continue 'label8;
                                    }
                                    break;
                                }
                                break;
                            }
                            'label9: loop {
                                'label10: loop {
                                    arg3 = arg3.wrapping_add(var5);
                                    if arg3 != 0 {
                                        break 'label10;
                                    }
                                    arg2 = arg0;
                                    break 'label9;
                                    break;
                                }
                                'label11: loop {
                                    if ((arg3 as u32) < arg1 as u32) as i32 != 0 {
                                        break 'label11;
                                    }
                                    if (arg3 == arg1) as i32 != 0 {
                                        break 'label3;
                                    }
                                    break 'label0;
                                    break;
                                }
                                arg2 = arg0.wrapping_add(arg3);
                                let var12 = self.memory.load8(arg2 as usize) as i8 as i32;
                                if (var12 <= -65) as i32 != 0 {
                                    break 'label0;
                                }
                                if (arg3 == arg1) as i32 != 0 {
                                    break 'label3;
                                }
                                break;
                            }
                            let var13 = self.memory.load8(arg2 as usize) as i8 as i32;
                            arg3 = var13;
                            if (arg3 > -1) as i32 != 0 {
                                break 'label5;
                            }
                            if ((arg3 as u32) < -32 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            if ((arg3 as u32) < -16 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            let var14 = self.memory.load8(arg2 as usize + 1) as i32;
                            let var15 = self.memory.load8(arg2 as usize + 2) as i32;
                            let var16 = self.memory.load8(arg2 as usize + 3) as i32;
                            if ((var14 & 63).wrapping_shl(12 as u32) | (var15 & 63).wrapping_shl(6 as u32) | var16 & 63 | (arg3 & 255).wrapping_shl(18 as u32) & 1835008 == 1114112) as i32 != 0 {
                                break 'label3;
                            }
                            break;
                        }
                        self.func96(env);
                        unreachable!();
                        break;
                    }
                    self.func111(env, var5, var4);
                    unreachable!();
                    break;
                }
                self.func37(env);
                unreachable!();
                break;
            }
            self.func104(env, arg0, arg1, 0, var4);
            unreachable!();
            break;
        }
        self.func104(env, arg0, arg1, arg3, arg1);
        unreachable!();
    }
    fn func113(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(16);
        self.global0 = var4;
        'label0: loop {
            if (arg2 == 0) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                if arg3 != 0 {
                    break 'label1;
                }
                let var7 = self.func92(env, arg2, arg1);
                arg1 = var7;
                break 'label0;
                break;
            }
            self.func114(env);
            let var9 = self.memory.load32(0 as usize + 1053988) as i32;
            self.func115(env, var4.wrapping_add(8), var9, arg1);
            'label2: loop {
                'label3: loop {
                    let mut slot_var4_8_i32 = self.memory.load32(var4 as usize + 8) as i32;
                    if (slot_var4_8_i32 & 1 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var4_12_i32 = self.memory.load32(var4 as usize + 12) as i32;
                    arg3 = slot_var4_12_i32;
                    var5 = arg3.wrapping_add(arg2);
                    if ((var5 as u32) < arg3 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    'label4: loop {
                        'label5: loop {
                            let var11 = self.memory.load32(0 as usize + 1053992) as i32;
                            if (var5 as u32 <= var11 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            let var12 = self.func116(env, arg2, arg1);
                            arg3 = var12;
                            break 'label4;
                            break;
                        }
                        self.memory.store32(0 as usize + 1053988, var5 as u32);
                        break;
                    }
                    if (arg3 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    let var13 = self.func122(env, arg3, 0, arg2);
                    var13;
                    break 'label2;
                    break;
                }
                unreachable!();
                break;
            }
            arg1 = arg3;
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg2 as u32);
        self.memory.store32(arg0 as usize, arg1 as u32);
        self.global0 = var4.wrapping_add(16);
    }
    fn func114(&mut self, env: &Env) {
        let mut var0: i32 = 0;
        'label0: loop {
            'label1: loop {
                let var1 = self.memory.load32(0 as usize + 1053992) as i32;
                if var1 != 0 {
                    break 'label1;
                }
                let var2 = self.memory.size();
                var0 = var2;
                if (var0 as u32 >= 65536 as u32) as i32 != 0 {
                    break 'label0;
                }
                var0 = var0.wrapping_shl(16 as u32);
                self.memory.store32(0 as usize + 1053992, var0 as u32);
                self.memory.store32(0 as usize + 1053988, var0 as u32);
                break;
            }
            return;
            break;
        }
        unreachable!();
    }
    fn func115(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        'label0: loop {
            'label1: loop {
                var3 = (arg1 as u32).wrapping_rem(arg2 as u32) as i32;
                if var3 != 0 {
                    break 'label1;
                }
                arg2 = 1;
                break 'label0;
                break;
            }
            var3 = arg2.wrapping_sub(var3).wrapping_add(arg1);
            arg2 = (var3 as u32 >= arg1 as u32) as i32;
            arg1 = var3;
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg1 as u32);
        self.memory.store32(arg0 as usize, arg2 as u32);
    }
    fn func116(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(16);
        self.global0 = var2;
        var3 = ((arg0 as u32).wrapping_shr(16 as u32) as i32).wrapping_add((arg0 & 65535 != 0) as i32);
        var4 = var3.wrapping_shl(16 as u32);
        'label0: loop {
            'label1: loop {
                let var8 = self.memory.grow(var3 as usize);
                if (var8 == -1) as i32 != 0 {
                    break 'label0;
                }
                let var9 = self.memory.load32(0 as usize + 1053992) as i32;
                var5 = var9;
                var6 = var5.wrapping_add(var4);
                if ((var6 as u32) < var5 as u32) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store32(0 as usize + 1053992, var6 as u32);
                self.func114(env);
                let var11 = self.memory.load32(0 as usize + 1053988) as i32;
                self.func115(env, var2.wrapping_add(8), var11, arg1);
                let mut slot_var2_8_i32 = self.memory.load32(var2 as usize + 8) as i32;
                if (slot_var2_8_i32 & 1 == 0) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_12_i32 = self.memory.load32(var2 as usize + 12) as i32;
                var5 = slot_var2_12_i32;
                var6 = var5.wrapping_add(arg0);
                if ((var6 as u32) < var5 as u32) as i32 != 0 {
                    break 'label0;
                }
                let var13 = self.memory.load32(0 as usize + 1053992) as i32;
                if (var6 as u32 > var13 as u32) as i32 != 0 {
                    continue 'label1;
                }
                break;
            }
            self.memory.store32(0 as usize + 1053988, var6 as u32);
            self.global0 = var2.wrapping_add(16);
            return var5;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func117(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        self.func113(env, var3.wrapping_add(8), arg1, arg2, 0);
        let mut slot_var3_12_i32 = self.memory.load32(var3 as usize + 12) as i32;
        arg2 = slot_var3_12_i32;
        let mut slot_var3_8_i32 = self.memory.load32(var3 as usize + 8) as i32;
        self.memory.store32(arg0 as usize, slot_var3_8_i32 as u32);
        self.memory.store32(arg0 as usize + 4, arg2 as u32);
        self.global0 = var3.wrapping_add(16);
    }
    fn func118(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
    fn func119(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        var5 = arg3 & 4294967295;
        var6 = arg1 & 4294967295;
        var7 = var5.wrapping_mul(var6);
        var8 = (arg3 as u64).wrapping_shr(32 as u32) as i64;
        var6 = var8.wrapping_mul(var6);
        var9 = (arg1 as u64).wrapping_shr(32 as u32) as i64;
        var5 = var6.wrapping_add(var5.wrapping_mul(var9));
        var10 = var7.wrapping_add(var5.wrapping_shl(32 as u32));
        self.memory.store64(arg0 as usize, var10 as u64);
        self.memory.store64(arg0 as usize + 8, var8.wrapping_mul(var9).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64).wrapping_add(((var10 as u64) < var7 as u64) as i32 as u32 as i64).wrapping_add(arg4.wrapping_mul(arg1).wrapping_add(arg3.wrapping_mul(arg2))) as u64);
    }
    fn func120(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
    fn func121(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let var3 = self.func120(env, arg0, arg1, arg2);
        var3
    }
    fn func122(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
