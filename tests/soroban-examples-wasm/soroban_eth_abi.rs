#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Bytes, Val, Address, FromVal, IntoVal, Vec, Map, BytesN, String, Symbol};

#[contract]
pub struct EthAbi;

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
pub enum Error { Decode = 1, }

#[contractimpl]
impl EthAbi {
    pub fn exec(&mut self, mut env: Env, input: soroban_sdk::Bytes) -> Result<soroban_sdk::Bytes, soroban_sdk::Error> {
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
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var1 = var16.wrapping_sub(1072);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Bytes::try_from_val(env, &val_from_i64(input)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let var17 = self.func31(env, var1.wrapping_add(32), 0, 128);
                    var17;
                    let var18 = Bytes::from_val(env, &val_from_i64(input)).len() as i64
                    self.func5(env, var1.wrapping_add(24), (var18 as u64).wrapping_shr(32 as u32) as i64 as i32, var1.wrapping_add(32), 128);
                    let mut slot_var1_24_i32 = self.memory.load32(var1 as usize + 24) as i32;
                    var2 = slot_var1_24_i32;
                    let mut slot_var1_28_i32 = self.memory.load32(var1 as usize + 28) as i32;
                    var3 = slot_var1_28_i32;
                    let var20 = Bytes::from_val(env, &val_from_i64(input)).len() as i64
                    if (var3 != (var20 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                        break 'label1;
                    }
                    let var21 = 0 /* TODO: bytes_copy_to_linear_memory */
                    var21;
                    self.memory.store8(var1 as usize + 668, 0 as u8);
                    let mut slot_var1_664_i32 = 0 as i32;
                    let mut slot_var1_660_i32 = var3 as i32;
                    let mut slot_var1_656_i32 = var2 as i32;
                    self.func18(env, var1.wrapping_add(772), var1.wrapping_add(656));
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    let var23 = self.memory.load8(var1 as usize + 772) as i32;
                                    if (var23 != 1) as i32 != 0 {
                                        break 'label6;
                                    }
                                    let var24 = self.memory.load64(var1.wrapping_add(792) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(995) as usize, var24 as u64);
                                    let var25 = self.memory.load64(var1.wrapping_add(784) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(987) as usize, var25 as u64);
                                    let mut slot_var1_776_i64 = self.memory.load64(var1 as usize + 776) as i64;
                                    let mut slot_var1_979_i64 = slot_var1_776_i64 as i64;
                                    break 'label5;
                                    break;
                                }
                                self.func18(env, var1.wrapping_add(808), var1.wrapping_add(656));
                                'label7: loop {
                                    let var27 = self.memory.load8(var1 as usize + 808) as i32;
                                    if (var27 != 1) as i32 != 0 {
                                        break 'label7;
                                    }
                                    let var28 = self.memory.load64(var1.wrapping_add(828) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(995) as usize, var28 as u64);
                                    let var29 = self.memory.load64(var1.wrapping_add(820) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(987) as usize, var29 as u64);
                                    let mut slot_var1_812_i64 = self.memory.load64(var1 as usize + 812) as i64;
                                    slot_var1_979_i64 = slot_var1_812_i64 as i64;
                                    break 'label5;
                                    break;
                                }
                                self.func18(env, var1.wrapping_add(844), var1.wrapping_add(656));
                                let var31 = self.memory.load8(var1 as usize + 844) as i32;
                                if (var31 != 1) as i32 != 0 {
                                    break 'label4;
                                }
                                let var32 = self.memory.load64(var1.wrapping_add(864) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(995) as usize, var32 as u64);
                                let var33 = self.memory.load64(var1.wrapping_add(856) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(987) as usize, var33 as u64);
                                let mut slot_var1_848_i64 = self.memory.load64(var1 as usize + 848) as i64;
                                slot_var1_979_i64 = slot_var1_848_i64 as i64;
                                break;
                            }
                            var2 = var1.wrapping_add(464).wrapping_add(19);
                            let var34 = self.memory.load64(var1.wrapping_add(976).wrapping_add(19) as usize) as i64;
                            let mut slot_var2_0_i64 = var34 as i64;
                            var3 = var1.wrapping_add(464).wrapping_add(11);
                            let var35 = self.memory.load64(var1.wrapping_add(976).wrapping_add(11) as usize) as i64;
                            let mut slot_var3_0_i64 = var35 as i64;
                            input = slot_var1_979_i64;
                            let mut slot_var1_563_i64 = input as i64;
                            let mut slot_var1_467_i64 = input as i64;
                            var4 = var1.wrapping_add(360).wrapping_add(16);
                            let mut slot_var4_0_i64 = slot_var2_0_i64 as i64;
                            var2 = var1.wrapping_add(360).wrapping_add(8);
                            slot_var2_0_i64 = slot_var3_0_i64 as i64;
                            let mut slot_var1_360_i64 = slot_var1_467_i64 as i64;
                            self.memory.store64(var1.wrapping_add(256).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                            self.memory.store64(var1.wrapping_add(256).wrapping_add(8) as usize, slot_var2_0_i64 as u64);
                            let mut slot_var1_256_i64 = slot_var1_360_i64 as i64;
                            self.func10(env, var1.wrapping_add(256));
                            input = 4294967299 /* Error(Contract, #1) */;
                            break 'label3;
                            break;
                        }
                        var3 = var1.wrapping_add(976).wrapping_add(88);
                        let var37 = self.memory.load64(var1.wrapping_add(844).wrapping_add(25) as usize) as i64;
                        slot_var3_0_i64 = var37 as i64;
                        var4 = var1.wrapping_add(976).wrapping_add(80);
                        let var38 = self.memory.load64(var1.wrapping_add(844).wrapping_add(17) as usize) as i64;
                        slot_var4_0_i64 = var38 as i64;
                        var5 = var1.wrapping_add(976).wrapping_add(72);
                        let var39 = self.memory.load64(var1.wrapping_add(844).wrapping_add(9) as usize) as i64;
                        let mut slot_var5_0_i64 = var39 as i64;
                        var2 = var1.wrapping_add(772) | 1;
                        let var40 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(8) as usize, var40 as u64);
                        let var41 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(16) as usize, var41 as u64);
                        let var42 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(24) as usize, var42 as u64);
                        var6 = var1.wrapping_add(976).wrapping_add(40);
                        let var43 = self.memory.load64(var1.wrapping_add(808).wrapping_add(9) as usize) as i64;
                        let mut slot_var6_0_i64 = var43 as i64;
                        var7 = var1.wrapping_add(976).wrapping_add(48);
                        let var44 = self.memory.load64(var1.wrapping_add(808).wrapping_add(17) as usize) as i64;
                        let mut slot_var7_0_i64 = var44 as i64;
                        var8 = var1.wrapping_add(976).wrapping_add(56);
                        let var45 = self.memory.load64(var1.wrapping_add(808).wrapping_add(25) as usize) as i64;
                        let mut slot_var8_0_i64 = var45 as i64;
                        let mut slot_var1_845_i64 = self.memory.load64(var1 as usize + 845) as i64;
                        let mut slot_var1_1040_i64 = slot_var1_845_i64 as i64;
                        let mut slot_var1_809_i64 = self.memory.load64(var1 as usize + 809) as i64;
                        let mut slot_var1_1008_i64 = slot_var1_809_i64 as i64;
                        let mut slot_var1_976_i64 = slot_var2_0_i64 as i64;
                        let var46 = self.func30(env, var1.wrapping_add(880), var1.wrapping_add(976), 96);
                        var46;
                        let var47 = self.func30(env, var1.wrapping_add(672), var1.wrapping_add(880), 96);
                        var47;
                        let var48 = self.func30(env, var1.wrapping_add(560), var1.wrapping_add(672), 96);
                        var48;
                        let var49 = self.func30(env, var1.wrapping_add(464), var1.wrapping_add(560), 96);
                        var49;
                        var2 = var1.wrapping_add(672).wrapping_add(8);
                        let var50 = self.memory.load64(var1.wrapping_add(464).wrapping_add(72) as usize) as i64;
                        slot_var2_0_i64 = var50 as i64;
                        var9 = var1.wrapping_add(672).wrapping_add(16);
                        let var51 = self.memory.load64(var1.wrapping_add(464).wrapping_add(80) as usize) as i64;
                        let mut slot_var9_0_i64 = var51 as i64;
                        var10 = var1.wrapping_add(672).wrapping_add(24);
                        let var52 = self.memory.load64(var1.wrapping_add(464).wrapping_add(88) as usize) as i64;
                        let mut slot_var10_0_i64 = var52 as i64;
                        var11 = var1.wrapping_add(560).wrapping_add(24);
                        let var53 = self.memory.load64(var1.wrapping_add(464).wrapping_add(56) as usize) as i64;
                        let mut slot_var11_0_i64 = var53 as i64;
                        var12 = var1.wrapping_add(560).wrapping_add(16);
                        let var54 = self.memory.load64(var1.wrapping_add(464).wrapping_add(48) as usize) as i64;
                        let mut slot_var12_0_i64 = var54 as i64;
                        var13 = var1.wrapping_add(560).wrapping_add(8);
                        let var55 = self.memory.load64(var1.wrapping_add(464).wrapping_add(40) as usize) as i64;
                        let mut slot_var13_0_i64 = var55 as i64;
                        let mut slot_var1_528_i64 = self.memory.load64(var1 as usize + 528) as i64;
                        let mut slot_var1_672_i64 = slot_var1_528_i64 as i64;
                        let mut slot_var1_496_i64 = self.memory.load64(var1 as usize + 496) as i64;
                        let mut slot_var1_560_i64 = slot_var1_496_i64 as i64;
                        self.func3(env, var1.wrapping_add(976).wrapping_add(32), var1.wrapping_add(560));
                        self.func3(env, var1.wrapping_add(1040), var1.wrapping_add(672));
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(72) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(80) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(88) as usize, slot_var3_0_i64 as u64);
                        var3 = var1.wrapping_add(464).wrapping_add(24);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                        var4 = var1.wrapping_add(464).wrapping_add(16);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        var5 = var1.wrapping_add(464).wrapping_add(8);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        let mut slot_var1_912_i64 = slot_var1_1008_i64 as i64;
                        let mut slot_var1_944_i64 = slot_var1_1040_i64 as i64;
                        let mut slot_var1_464_i64 = self.memory.load64(var1 as usize + 464) as i64;
                        let mut slot_var1_880_i64 = slot_var1_464_i64 as i64;
                        let var58 = self.func30(env, var1.wrapping_add(360).wrapping_add(4), var1.wrapping_add(880), 96);
                        let var59 = self.func30(env, var1.wrapping_add(256).wrapping_add(4), var58, 96);
                        let var60 = self.func30(env, var1.wrapping_add(160), var59, 96);
                        var60;
                        let var61 = self.memory.load64(var1.wrapping_add(160).wrapping_add(8) as usize) as i64;
                        slot_var5_0_i64 = var61 as i64;
                        let var62 = self.memory.load64(var1.wrapping_add(160).wrapping_add(16) as usize) as i64;
                        slot_var4_0_i64 = var62 as i64;
                        let var63 = self.memory.load64(var1.wrapping_add(160).wrapping_add(24) as usize) as i64;
                        slot_var3_0_i64 = var63 as i64;
                        let var64 = self.memory.load64(var1.wrapping_add(160).wrapping_add(40) as usize) as i64;
                        slot_var13_0_i64 = var64 as i64;
                        let var65 = self.memory.load64(var1.wrapping_add(160).wrapping_add(48) as usize) as i64;
                        slot_var12_0_i64 = var65 as i64;
                        let var66 = self.memory.load64(var1.wrapping_add(160).wrapping_add(56) as usize) as i64;
                        slot_var11_0_i64 = var66 as i64;
                        let var67 = self.memory.load64(var1.wrapping_add(160).wrapping_add(72) as usize) as i64;
                        slot_var2_0_i64 = var67 as i64;
                        let var68 = self.memory.load64(var1.wrapping_add(160).wrapping_add(80) as usize) as i64;
                        slot_var9_0_i64 = var68 as i64;
                        let var69 = self.memory.load64(var1.wrapping_add(160).wrapping_add(88) as usize) as i64;
                        slot_var10_0_i64 = var69 as i64;
                        let mut slot_var1_160_i64 = self.memory.load64(var1 as usize + 160) as i64;
                        slot_var1_464_i64 = slot_var1_160_i64 as i64;
                        let mut slot_var1_192_i64 = self.memory.load64(var1 as usize + 192) as i64;
                        slot_var1_560_i64 = slot_var1_192_i64 as i64;
                        let mut slot_var1_224_i64 = self.memory.load64(var1 as usize + 224) as i64;
                        slot_var1_672_i64 = slot_var1_224_i64 as i64;
                        var2 = 0;
                        var3 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var2 == 32) as i32 != 0 {
                                    break 'label8;
                                }
                                var4 = var1.wrapping_add(560).wrapping_add(var2);
                                var14 = slot_var4_0_i64;
                                let var70 = self.memory.load64(var1.wrapping_add(672).wrapping_add(var2) as usize) as i64;
                                input = var14.wrapping_add(var70);
                                var15 = input.wrapping_add(var3 as u32 as i64 & 1 /* True */);
                                slot_var4_0_i64 = var15 as i64;
                                var3 = ((input as u64) < var14 as u64) as i32 | ((var15 as u64) < input as u64) as i32;
                                var2 = var2.wrapping_add(8);
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var71 = self.memory.load64(var1.wrapping_add(560).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(312) as usize, var71 as u64);
                        let var72 = self.memory.load64(var1.wrapping_add(560).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(304) as usize, var72 as u64);
                        let var73 = self.memory.load64(var1.wrapping_add(560).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(296) as usize, var73 as u64);
                        let mut slot_var1_288_i64 = slot_var1_560_i64 as i64;
                        var3 = var1.wrapping_add(360).wrapping_add(24);
                        slot_var3_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(360).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(360).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_360_i64 = 0 /* False */ as i64;
                        self.func4(env, var1.wrapping_add(16), var1.wrapping_add(360), 32);
                        let mut slot_var1_20_i32 = self.memory.load32(var1 as usize + 20) as i32;
                        var2 = slot_var1_20_i32;
                        if (var2 != 32) as i32 != 0 {
                            break 'label0;
                        }
                        let var75 = self.memory.load64(var1.wrapping_add(464).wrapping_add(8) as usize) as i64;
                        input = var75;
                        let var76 = self.memory.load64(var1.wrapping_add(464).wrapping_add(16) as usize) as i64;
                        var14 = var76;
                        var15 = slot_var1_464_i64;
                        let mut slot_var1_16_i32 = self.memory.load32(var1 as usize + 16) as i32;
                        var2 = slot_var1_16_i32;
                        let var77 = self.memory.load64(var1.wrapping_add(464).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var2.wrapping_add(24) as usize, var77 as u64);
                        self.memory.store64(var2.wrapping_add(16) as usize, var14 as u64);
                        self.memory.store64(var2.wrapping_add(8) as usize, input as u64);
                        slot_var2_0_i64 = var15 as i64;
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        slot_var1_880_i64 = slot_var1_360_i64 as i64;
                        var2 = var1.wrapping_add(256).wrapping_add(32);
                        let var78 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(8) as usize, var78 as u64);
                        let var79 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(16) as usize, var79 as u64);
                        let var80 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(24) as usize, var80 as u64);
                        slot_var1_976_i64 = slot_var2_0_i64 as i64;
                        var3 = var1.wrapping_add(976);
                        var2 = 31;
                        'label10: loop {
                            'label11: loop {
                                if (var2 == 15) as i32 != 0 {
                                    break 'label10;
                                }
                                let var81 = self.memory.load8(var3 as usize) as i32;
                                var4 = var81;
                                var5 = var1.wrapping_add(976).wrapping_add(var2);
                                let var82 = self.memory.load8(var5 as usize) as i32;
                                self.memory.store8(var3 as usize, var82 as u8);
                                self.memory.store8(var5 as usize, var4 as u8);
                                var2 = var2.wrapping_add(-1);
                                var3 = var3.wrapping_add(1);
                                continue 'label11;
                                break;
                            }
                            break;
                        }
                        let var83 = self.memory.load64(var1.wrapping_add(880).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(24) as usize, var83 as u64);
                        let var84 = self.memory.load64(var1.wrapping_add(880).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(16) as usize, var84 as u64);
                        let var85 = self.memory.load64(var1.wrapping_add(880).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(8) as usize, var85 as u64);
                        let var86 = self.memory.load64(var1.wrapping_add(976).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(400) as usize, var86 as u64);
                        let var87 = self.memory.load64(var1.wrapping_add(976).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(408) as usize, var87 as u64);
                        let var88 = self.memory.load64(var1.wrapping_add(976).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(416) as usize, var88 as u64);
                        slot_var1_360_i64 = slot_var1_880_i64 as i64;
                        let mut slot_var1_392_i64 = slot_var1_976_i64 as i64;
                        self.func16(env, var1.wrapping_add(8), 2, 1, 32);
                        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                        input = slot_var1_8_i64;
                        self.func16(env, var1, 4, 4, 4);
                        let mut slot_var1_984_i32 = 0 as i32;
                        slot_var1_976_i64 = input as i64;
                        let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
                        var2 = slot_var1_4_i32;
                        let mut slot_var1_992_i32 = var2 as i32;
                        let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                        let mut slot_var1_988_i32 = slot_var1_0_i32 as i32;
                        let mut slot_var2_0_i32 = 64 as i32;
                        let mut slot_var1_996_i32 = 1 as i32;
                        self.func19(env, var1.wrapping_add(360), var1.wrapping_add(976));
                        self.func19(env, var1.wrapping_add(360).wrapping_add(32), var1.wrapping_add(976));
                        let mut slot_var1_980_i32 = self.memory.load32(var1 as usize + 980) as i32;
                        var2 = slot_var1_980_i32;
                        let var93 = self.memory.load32(var1 as usize + 984) as i64;
                        input = var93;
                        var3 = slot_var1_976_i64;
                        self.func8(env, slot_var1_988_i32, slot_var1_992_i32, 4, 4);
                        let var95 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                        input = var95;
                        self.func7(env, var3.wrapping_shl(5 as u32), var2);
                        break;
                    }
                    self.global0 = var1.wrapping_add(1072);
                    return input;
                    break;
                }
                unreachable!();
                break;
            }
            self.func21(env);
            unreachable!();
            break;
        }
        self.func22(env, var2);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
}

#[allow(dead_code)]
impl EthAbi {
    fn func3(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(80);
        self.global0 = var2;
        self.func4(env, var2.wrapping_add(8), arg1, 0);
        let mut slot_var2_12_i32 = self.memory.load32(var2 as usize + 12) as i32;
        var3 = slot_var2_12_i32;
        let mut slot_var2_8_i32 = self.memory.load32(var2 as usize + 8) as i32;
        var4 = slot_var2_8_i32;
        'label0: loop {
            'label1: loop {
                if (var3 == 0) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store8(var4 as usize, 0 as u8);
                var3 = var3.wrapping_add(-1);
                var4 = var4.wrapping_add(1);
                continue 'label1;
                break;
            }
            break;
        }
        var3 = 24;
        let var8 = self.memory.load64(arg1.wrapping_add(24) as usize) as i64;
        self.memory.store64(var2.wrapping_add(16).wrapping_add(24) as usize, var8 as u64);
        let var9 = self.memory.load64(arg1.wrapping_add(16) as usize) as i64;
        self.memory.store64(var2.wrapping_add(16).wrapping_add(16) as usize, var9 as u64);
        let var10 = self.memory.load64(arg1.wrapping_add(8) as usize) as i64;
        self.memory.store64(var2.wrapping_add(16).wrapping_add(8) as usize, var10 as u64);
        let var11 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_16_i64 = var11 as i64;
        self.memory.store64(var2.wrapping_add(48).wrapping_add(24) as usize, 0 /* False */ as u64);
        self.memory.store64(var2.wrapping_add(48).wrapping_add(16) as usize, 0 /* False */ as u64);
        self.memory.store64(var2.wrapping_add(48).wrapping_add(8) as usize, 0 /* False */ as u64);
        let mut slot_var2_48_i64 = 0 /* False */ as i64;
        var4 = var2.wrapping_add(48);
        'label2: loop {
            'label3: loop {
                if (var3 == -8) as i32 != 0 {
                    break 'label2;
                }
                let var12 = self.memory.load64(var2.wrapping_add(16).wrapping_add(var3) as usize) as i64;
                var5 = var12;
                let mut slot_var4_0_i64 = (var5.wrapping_shl(56 as u32) | (var5 & 65280).wrapping_shl(40 as u32) | (var5 & 16711680).wrapping_shl(24 as u32) | (var5 & 4278190080).wrapping_shl(0 as u32) | (var5 as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (var5 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (var5 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (var5 as u64).wrapping_shr(56 as u32) as i64) as i64;
                var4 = var4.wrapping_add(8);
                var3 = var3.wrapping_add(-8);
                continue 'label3;
                break;
            }
            break;
        }
        self.memory.store64(arg0 as usize, slot_var2_48_i64 as u64);
        let var13 = self.memory.load64(var2.wrapping_add(48).wrapping_add(24) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(24) as usize, var13 as u64);
        let var14 = self.memory.load64(var2.wrapping_add(48).wrapping_add(16) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(16) as usize, var14 as u64);
        let var15 = self.memory.load64(var2.wrapping_add(48).wrapping_add(8) as usize) as i64;
        self.memory.store64(arg0.wrapping_add(8) as usize, var15 as u64);
        self.global0 = var2.wrapping_add(80);
    }
    fn func4(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        self.func5(env, var3.wrapping_add(8), arg2, arg1, 32);
        let mut slot_var3_12_i32 = self.memory.load32(var3 as usize + 12) as i32;
        arg1 = slot_var3_12_i32;
        let mut slot_var3_8_i32 = self.memory.load32(var3 as usize + 8) as i32;
        self.memory.store32(arg0 as usize, slot_var3_8_i32 as u32);
        self.memory.store32(arg0 as usize + 4, arg1 as u32);
        self.global0 = var3.wrapping_add(16);
    }
    fn func5(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        'label0: loop {
            if (arg1 as u32 <= arg3 as u32) as i32 != 0 {
                break 'label0;
            }
            self.func6(env, arg1, arg3);
            unreachable!();
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg1 as u32);
        self.memory.store32(arg0 as usize, arg2 as u32);
    }
    fn func6(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func27(env);
        unreachable!();
    }
    fn func7(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func8(env, arg0, arg1, 1, 1);
    }
    fn func8(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
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
    fn func9(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        'label0: loop {
            if (arg0 == -2147483648) as i32 != 0 {
                break 'label0;
            }
            self.func7(env, arg0, arg1);
            break;
        }
    }
    fn func10(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        'label0: loop {
            let var3 = self.memory.load32(arg0 as usize) as i32;
            var1 = var3;
            var2 = { let a = var1.wrapping_add(-2147483647); let b = 0; if (var1 < -2147483638) as i32 != 0 { a } else { b } };
            if (var2 as u32 > 9 as u32) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    if 1.wrapping_shl(var2 as u32) & 894 != 0 {
                        break 'label2;
                    }
                    if var2 != 0 {
                        break 'label1;
                    }
                    let var4 = self.memory.load32(arg0 as usize + 12) as i32;
                    let var5 = self.memory.load32(arg0 as usize + 16) as i32;
                    self.func9(env, var4, var5);
                    let var7 = self.memory.load32(arg0 as usize + 4) as i32;
                    self.func7(env, var1, var7);
                    break;
                }
                return;
                break;
            }
            let var9 = self.memory.load32(arg0 as usize + 12) as i32;
            arg0 = var9;
            let var10 = self.memory.load32(arg0 as usize) as i32;
            let var11 = self.memory.load32(arg0.wrapping_add(4) as usize) as i32;
            self.func8(env, var10, var11, 1, 32);
            let var13 = self.memory.load32(arg0 as usize + 16) as i32;
            let var14 = self.memory.load32(arg0 as usize + 20) as i32;
            let var15 = self.memory.load32(arg0 as usize + 12) as i32;
            let mut slot_var15_16_i32 = self.memory.load32(var15 as usize + 16) as i32;
            /* TODO: call_indirect */ 0;
            return var14;
            break;
        }
        let var17 = self.memory.load32(arg0 as usize + 4) as i32;
        let var18 = self.memory.load32(arg0 as usize + 8) as i32;
        self.func9(env, var17, var18);
    }
    fn func11(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let var8 = self.global0;
        var1 = var8.wrapping_sub(32);
        self.global0 = var1;
        let var9 = self.memory.load32(arg0 as usize) as i32;
        var2 = var9;
        var3 = { let a = var2.wrapping_shl(1 as u32); let b = 1; if var2 != 0 { a } else { b } };
        var4 = { let a = var3; let b = 4; if (var3 as u32 > 4 as u32) as i32 != 0 { a } else { b } };
        var5 = var4.wrapping_shl(5 as u32);
        var6 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var3 as u32 > 134217727 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    var3 = 0;
                    if (var5 < 0) as i32 != 0 {
                        break 'label2;
                    }
                    'label3: loop {
                        'label4: loop {
                            if var2 != 0 {
                                break 'label4;
                            }
                            var2 = var1.wrapping_add(28);
                            break 'label3;
                            break;
                        }
                        let mut slot_var1_28_i32 = 1 as i32;
                        var3 = var2.wrapping_shl(5 as u32);
                        let var10 = self.memory.load32(arg0 as usize + 4) as i32;
                        var6 = var10;
                        var2 = var1.wrapping_add(24);
                        break;
                    }
                    let mut slot_var2_0_i32 = var3 as i32;
                    'label5: loop {
                        'label6: loop {
                            if (slot_var1_28_i32 == 0) as i32 != 0 {
                                break 'label6;
                            }
                            'label7: loop {
                                let mut slot_var1_24_i32 = self.memory.load32(var1 as usize + 24) as i32;
                                var7 = slot_var1_24_i32;
                                if var7 != 0 {
                                    break 'label7;
                                }
                                self.func12(env, var1.wrapping_add(16), var5);
                                let mut slot_var1_16_i32 = self.memory.load32(var1 as usize + 16) as i32;
                                var2 = slot_var1_16_i32;
                                break 'label5;
                                break;
                            }
                            self.func13(env);
                            let var13 = self.memory.load32(0 as usize + 1048576) as i32;
                            var2 = var13;
                            var3 = var2.wrapping_add(var5);
                            if ((var3 as u32) < var2 as u32) as i32 != 0 {
                                break 'label0;
                            }
                            'label8: loop {
                                'label9: loop {
                                    let var14 = self.memory.load32(0 as usize + 1048580) as i32;
                                    if (var3 as u32 <= var14 as u32) as i32 != 0 {
                                        break 'label9;
                                    }
                                    let var15 = self.func14(env, var5, 1);
                                    var2 = var15;
                                    break 'label8;
                                    break;
                                }
                                self.memory.store32(0 as usize + 1048576, var3 as u32);
                                break;
                            }
                            if (var2 == 0) as i32 != 0 {
                                break 'label5;
                            }
                            let var16 = self.func30(env, var2, var6, var7);
                            var16;
                            break 'label5;
                            break;
                        }
                        self.func12(env, var1.wrapping_add(8), var5);
                        let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                        var2 = slot_var1_8_i32;
                        break;
                    }
                    if var2 != 0 {
                        break 'label1;
                    }
                    var6 = 1;
                    break;
                }
                self.func15(env, var6, var5);
                unreachable!();
                break;
            }
            self.memory.store32(arg0 as usize, var4 as u32);
            self.memory.store32(arg0 as usize + 4, var2 as u32);
            self.global0 = var1.wrapping_add(32);
            return;
            break;
        }
        unreachable!();
    }
    fn func12(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                if arg1 != 0 {
                    break 'label1;
                }
                var2 = 1;
                break 'label0;
                break;
            }
            let var3 = self.func17(env, arg1, 1);
            var2 = var3;
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg1 as u32);
        self.memory.store32(arg0 as usize, var2 as u32);
    }
    fn func13(&mut self, env: &Env) {
        let mut var0: i32 = 0;
        'label0: loop {
            'label1: loop {
                let var1 = self.memory.load32(0 as usize + 1048580) as i32;
                if var1 != 0 {
                    break 'label1;
                }
                let var2 = self.memory.size();
                var0 = var2;
                if (var0 as u32 >= 65536 as u32) as i32 != 0 {
                    break 'label0;
                }
                var0 = var0.wrapping_shl(16 as u32);
                self.memory.store32(0 as usize + 1048580, var0 as u32);
                self.memory.store32(0 as usize + 1048576, var0 as u32);
                break;
            }
            return;
            break;
        }
        unreachable!();
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
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
                let var9 = self.memory.load32(0 as usize + 1048580) as i32;
                var5 = var9;
                var6 = var5.wrapping_add(var4);
                if ((var6 as u32) < var5 as u32) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store32(0 as usize + 1048580, var6 as u32);
                self.func13(env);
                let var11 = self.memory.load32(0 as usize + 1048576) as i32;
                self.func28(env, var2.wrapping_add(8), var11, arg1);
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
                let var13 = self.memory.load32(0 as usize + 1048580) as i32;
                if (var6 as u32 > var13 as u32) as i32 != 0 {
                    continue 'label1;
                }
                break;
            }
            self.memory.store32(0 as usize + 1048576, var6 as u32);
            self.global0 = var2.wrapping_add(16);
            return var5;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func15(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        'label0: loop {
            if (arg0 == 0) as i32 != 0 {
                break 'label0;
            }
            self.func25(env, arg1);
            unreachable!();
            break;
        }
        self.func26(env);
        unreachable!();
    }
    fn func16(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) {
        let mut var4: i32 = 0;
        'label0: loop {
            var4 = (arg2.wrapping_add(arg3).wrapping_add(-1) & 0.wrapping_sub(arg2)).wrapping_mul(arg1);
            let var5 = self.func17(env, var4, arg2);
            arg3 = var5;
            if arg3 != 0 {
                break 'label0;
            }
            self.func15(env, arg2, var4);
            unreachable!();
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg3 as u32);
        self.memory.store32(arg0 as usize, arg1 as u32);
    }
    fn func17(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16);
        self.global0 = var2;
        self.func13(env);
        let var7 = self.memory.load32(0 as usize + 1048576) as i32;
        self.func28(env, var2.wrapping_add(8), var7, arg1);
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
                    let var9 = self.memory.load32(0 as usize + 1048580) as i32;
                    if (var4 as u32 <= var9 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    let var10 = self.func14(env, arg0, arg1);
                    var3 = var10;
                    break 'label1;
                    break;
                }
                self.memory.store32(0 as usize + 1048576, var4 as u32);
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
    fn func18(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(32);
        self.global0 = var2;
        let var8 = self.memory.load32(arg1 as usize + 8) as i32;
        var3 = var8;
        let mut slot_var2_8_i32 = -2147483648 as i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var3 as u32 >= -32 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    self.func10(env, var2.wrapping_add(8));
                    let var10 = self.memory.load32(arg1 as usize) as i32;
                    var4 = var10;
                    let var11 = self.memory.load32(arg1 as usize + 4) as i32;
                    var5 = var11;
                    slot_var2_8_i32 = -2147483648 as i32;
                    var6 = var3.wrapping_add(32);
                    if (var6 as u32 <= var5 as u32) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                self.memory.store32(arg0 as usize + 4, -2147483648 as u32);
                arg1 = 1;
                break 'label0;
                break;
            }
            self.func10(env, var2.wrapping_add(8));
            self.memory.store32(arg1 as usize + 8, var6 as u32);
            arg1 = var4.wrapping_add(var3);
            let var13 = self.memory.load16(arg1 as usize) as i32;
            self.memory.store16(arg0 as usize + 1, var13 as u16);
            let var14 = self.memory.load64(arg1 as usize + 11) as i64;
            self.memory.store64(arg0 as usize + 12, var14 as u64);
            let var15 = self.memory.load32(arg1 as usize + 27) as i32;
            self.memory.store32(arg0 as usize + 28, var15 as u32);
            let var16 = self.memory.load64(arg1 as usize + 3) as i64;
            self.memory.store64(arg0 as usize + 4, var16 as u64);
            let var17 = self.memory.load8(arg1.wrapping_add(2) as usize) as i32;
            self.memory.store8(arg0.wrapping_add(3) as usize, var17 as u8);
            let var18 = self.memory.load64(arg1.wrapping_add(19) as usize) as i64;
            self.memory.store64(arg0.wrapping_add(20) as usize, var18 as u64);
            let var19 = self.memory.load8(arg1.wrapping_add(31) as usize) as i32;
            self.memory.store8(arg0.wrapping_add(32) as usize, var19 as u8);
            arg1 = 0;
            break;
        }
        self.memory.store8(arg0 as usize, arg1 as u8);
        self.global0 = var2.wrapping_add(32);
    }
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        'label0: loop {
            let var3 = self.memory.load32(arg1 as usize + 8) as i32;
            var2 = var3;
            let var4 = self.memory.load32(arg1 as usize) as i32;
            if (var2 != var4) as i32 != 0 {
                break 'label0;
            }
            self.func11(env, arg1);
            break;
        }
        self.memory.store32(arg1 as usize + 8, var2.wrapping_add(1) as u32);
        let var6 = self.memory.load32(arg1 as usize + 4) as i32;
        arg1 = var6.wrapping_add(var2.wrapping_shl(5 as u32));
        let var7 = self.memory.load64(arg0 as usize) as i64;
        self.memory.store64(arg1 as usize, var7 as u64);
        let var8 = self.memory.load64(arg0.wrapping_add(8) as usize) as i64;
        self.memory.store64(arg1.wrapping_add(8) as usize, var8 as u64);
        let var9 = self.memory.load64(arg0.wrapping_add(16) as usize) as i64;
        self.memory.store64(arg1.wrapping_add(16) as usize, var9 as u64);
        let var10 = self.memory.load64(arg0.wrapping_add(24) as usize) as i64;
        self.memory.store64(arg1.wrapping_add(24) as usize, var10 as u64);
    }
    fn func20(&mut self, env: &Env, mut input: i64) -> i64 {
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
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var1 = var16.wrapping_sub(1072);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Bytes::try_from_val(env, &val_from_i64(input)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let var17 = self.func31(env, var1.wrapping_add(32), 0, 128);
                    var17;
                    let var18 = Bytes::from_val(env, &val_from_i64(input)).len() as i64
                    self.func5(env, var1.wrapping_add(24), (var18 as u64).wrapping_shr(32 as u32) as i64 as i32, var1.wrapping_add(32), 128);
                    let mut slot_var1_24_i32 = self.memory.load32(var1 as usize + 24) as i32;
                    var2 = slot_var1_24_i32;
                    let mut slot_var1_28_i32 = self.memory.load32(var1 as usize + 28) as i32;
                    var3 = slot_var1_28_i32;
                    let var20 = Bytes::from_val(env, &val_from_i64(input)).len() as i64
                    if (var3 != (var20 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                        break 'label1;
                    }
                    let var21 = 0 /* TODO: bytes_copy_to_linear_memory */
                    var21;
                    self.memory.store8(var1 as usize + 668, 0 as u8);
                    let mut slot_var1_664_i32 = 0 as i32;
                    let mut slot_var1_660_i32 = var3 as i32;
                    let mut slot_var1_656_i32 = var2 as i32;
                    self.func18(env, var1.wrapping_add(772), var1.wrapping_add(656));
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    let var23 = self.memory.load8(var1 as usize + 772) as i32;
                                    if (var23 != 1) as i32 != 0 {
                                        break 'label6;
                                    }
                                    let var24 = self.memory.load64(var1.wrapping_add(792) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(995) as usize, var24 as u64);
                                    let var25 = self.memory.load64(var1.wrapping_add(784) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(987) as usize, var25 as u64);
                                    let mut slot_var1_776_i64 = self.memory.load64(var1 as usize + 776) as i64;
                                    let mut slot_var1_979_i64 = slot_var1_776_i64 as i64;
                                    break 'label5;
                                    break;
                                }
                                self.func18(env, var1.wrapping_add(808), var1.wrapping_add(656));
                                'label7: loop {
                                    let var27 = self.memory.load8(var1 as usize + 808) as i32;
                                    if (var27 != 1) as i32 != 0 {
                                        break 'label7;
                                    }
                                    let var28 = self.memory.load64(var1.wrapping_add(828) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(995) as usize, var28 as u64);
                                    let var29 = self.memory.load64(var1.wrapping_add(820) as usize) as i64;
                                    self.memory.store64(var1.wrapping_add(987) as usize, var29 as u64);
                                    let mut slot_var1_812_i64 = self.memory.load64(var1 as usize + 812) as i64;
                                    slot_var1_979_i64 = slot_var1_812_i64 as i64;
                                    break 'label5;
                                    break;
                                }
                                self.func18(env, var1.wrapping_add(844), var1.wrapping_add(656));
                                let var31 = self.memory.load8(var1 as usize + 844) as i32;
                                if (var31 != 1) as i32 != 0 {
                                    break 'label4;
                                }
                                let var32 = self.memory.load64(var1.wrapping_add(864) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(995) as usize, var32 as u64);
                                let var33 = self.memory.load64(var1.wrapping_add(856) as usize) as i64;
                                self.memory.store64(var1.wrapping_add(987) as usize, var33 as u64);
                                let mut slot_var1_848_i64 = self.memory.load64(var1 as usize + 848) as i64;
                                slot_var1_979_i64 = slot_var1_848_i64 as i64;
                                break;
                            }
                            var2 = var1.wrapping_add(464).wrapping_add(19);
                            let var34 = self.memory.load64(var1.wrapping_add(976).wrapping_add(19) as usize) as i64;
                            let mut slot_var2_0_i64 = var34 as i64;
                            var3 = var1.wrapping_add(464).wrapping_add(11);
                            let var35 = self.memory.load64(var1.wrapping_add(976).wrapping_add(11) as usize) as i64;
                            let mut slot_var3_0_i64 = var35 as i64;
                            input = slot_var1_979_i64;
                            let mut slot_var1_563_i64 = input as i64;
                            let mut slot_var1_467_i64 = input as i64;
                            var4 = var1.wrapping_add(360).wrapping_add(16);
                            let mut slot_var4_0_i64 = slot_var2_0_i64 as i64;
                            var2 = var1.wrapping_add(360).wrapping_add(8);
                            slot_var2_0_i64 = slot_var3_0_i64 as i64;
                            let mut slot_var1_360_i64 = slot_var1_467_i64 as i64;
                            self.memory.store64(var1.wrapping_add(256).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                            self.memory.store64(var1.wrapping_add(256).wrapping_add(8) as usize, slot_var2_0_i64 as u64);
                            let mut slot_var1_256_i64 = slot_var1_360_i64 as i64;
                            self.func10(env, var1.wrapping_add(256));
                            input = 4294967299 /* Error(Contract, #1) */;
                            break 'label3;
                            break;
                        }
                        var3 = var1.wrapping_add(976).wrapping_add(88);
                        let var37 = self.memory.load64(var1.wrapping_add(844).wrapping_add(25) as usize) as i64;
                        slot_var3_0_i64 = var37 as i64;
                        var4 = var1.wrapping_add(976).wrapping_add(80);
                        let var38 = self.memory.load64(var1.wrapping_add(844).wrapping_add(17) as usize) as i64;
                        slot_var4_0_i64 = var38 as i64;
                        var5 = var1.wrapping_add(976).wrapping_add(72);
                        let var39 = self.memory.load64(var1.wrapping_add(844).wrapping_add(9) as usize) as i64;
                        let mut slot_var5_0_i64 = var39 as i64;
                        var2 = var1.wrapping_add(772) | 1;
                        let var40 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(8) as usize, var40 as u64);
                        let var41 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(16) as usize, var41 as u64);
                        let var42 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(24) as usize, var42 as u64);
                        var6 = var1.wrapping_add(976).wrapping_add(40);
                        let var43 = self.memory.load64(var1.wrapping_add(808).wrapping_add(9) as usize) as i64;
                        let mut slot_var6_0_i64 = var43 as i64;
                        var7 = var1.wrapping_add(976).wrapping_add(48);
                        let var44 = self.memory.load64(var1.wrapping_add(808).wrapping_add(17) as usize) as i64;
                        let mut slot_var7_0_i64 = var44 as i64;
                        var8 = var1.wrapping_add(976).wrapping_add(56);
                        let var45 = self.memory.load64(var1.wrapping_add(808).wrapping_add(25) as usize) as i64;
                        let mut slot_var8_0_i64 = var45 as i64;
                        let mut slot_var1_845_i64 = self.memory.load64(var1 as usize + 845) as i64;
                        let mut slot_var1_1040_i64 = slot_var1_845_i64 as i64;
                        let mut slot_var1_809_i64 = self.memory.load64(var1 as usize + 809) as i64;
                        let mut slot_var1_1008_i64 = slot_var1_809_i64 as i64;
                        let mut slot_var1_976_i64 = slot_var2_0_i64 as i64;
                        let var46 = self.func30(env, var1.wrapping_add(880), var1.wrapping_add(976), 96);
                        var46;
                        let var47 = self.func30(env, var1.wrapping_add(672), var1.wrapping_add(880), 96);
                        var47;
                        let var48 = self.func30(env, var1.wrapping_add(560), var1.wrapping_add(672), 96);
                        var48;
                        let var49 = self.func30(env, var1.wrapping_add(464), var1.wrapping_add(560), 96);
                        var49;
                        var2 = var1.wrapping_add(672).wrapping_add(8);
                        let var50 = self.memory.load64(var1.wrapping_add(464).wrapping_add(72) as usize) as i64;
                        slot_var2_0_i64 = var50 as i64;
                        var9 = var1.wrapping_add(672).wrapping_add(16);
                        let var51 = self.memory.load64(var1.wrapping_add(464).wrapping_add(80) as usize) as i64;
                        let mut slot_var9_0_i64 = var51 as i64;
                        var10 = var1.wrapping_add(672).wrapping_add(24);
                        let var52 = self.memory.load64(var1.wrapping_add(464).wrapping_add(88) as usize) as i64;
                        let mut slot_var10_0_i64 = var52 as i64;
                        var11 = var1.wrapping_add(560).wrapping_add(24);
                        let var53 = self.memory.load64(var1.wrapping_add(464).wrapping_add(56) as usize) as i64;
                        let mut slot_var11_0_i64 = var53 as i64;
                        var12 = var1.wrapping_add(560).wrapping_add(16);
                        let var54 = self.memory.load64(var1.wrapping_add(464).wrapping_add(48) as usize) as i64;
                        let mut slot_var12_0_i64 = var54 as i64;
                        var13 = var1.wrapping_add(560).wrapping_add(8);
                        let var55 = self.memory.load64(var1.wrapping_add(464).wrapping_add(40) as usize) as i64;
                        let mut slot_var13_0_i64 = var55 as i64;
                        let mut slot_var1_528_i64 = self.memory.load64(var1 as usize + 528) as i64;
                        let mut slot_var1_672_i64 = slot_var1_528_i64 as i64;
                        let mut slot_var1_496_i64 = self.memory.load64(var1 as usize + 496) as i64;
                        let mut slot_var1_560_i64 = slot_var1_496_i64 as i64;
                        self.func3(env, var1.wrapping_add(976).wrapping_add(32), var1.wrapping_add(560));
                        self.func3(env, var1.wrapping_add(1040), var1.wrapping_add(672));
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(40) as usize, slot_var6_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(48) as usize, slot_var7_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(56) as usize, slot_var8_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(72) as usize, slot_var5_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(80) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(88) as usize, slot_var3_0_i64 as u64);
                        var3 = var1.wrapping_add(464).wrapping_add(24);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                        var4 = var1.wrapping_add(464).wrapping_add(16);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        var5 = var1.wrapping_add(464).wrapping_add(8);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        let mut slot_var1_912_i64 = slot_var1_1008_i64 as i64;
                        let mut slot_var1_944_i64 = slot_var1_1040_i64 as i64;
                        let mut slot_var1_464_i64 = self.memory.load64(var1 as usize + 464) as i64;
                        let mut slot_var1_880_i64 = slot_var1_464_i64 as i64;
                        let var58 = self.func30(env, var1.wrapping_add(360).wrapping_add(4), var1.wrapping_add(880), 96);
                        let var59 = self.func30(env, var1.wrapping_add(256).wrapping_add(4), var58, 96);
                        let var60 = self.func30(env, var1.wrapping_add(160), var59, 96);
                        var60;
                        let var61 = self.memory.load64(var1.wrapping_add(160).wrapping_add(8) as usize) as i64;
                        slot_var5_0_i64 = var61 as i64;
                        let var62 = self.memory.load64(var1.wrapping_add(160).wrapping_add(16) as usize) as i64;
                        slot_var4_0_i64 = var62 as i64;
                        let var63 = self.memory.load64(var1.wrapping_add(160).wrapping_add(24) as usize) as i64;
                        slot_var3_0_i64 = var63 as i64;
                        let var64 = self.memory.load64(var1.wrapping_add(160).wrapping_add(40) as usize) as i64;
                        slot_var13_0_i64 = var64 as i64;
                        let var65 = self.memory.load64(var1.wrapping_add(160).wrapping_add(48) as usize) as i64;
                        slot_var12_0_i64 = var65 as i64;
                        let var66 = self.memory.load64(var1.wrapping_add(160).wrapping_add(56) as usize) as i64;
                        slot_var11_0_i64 = var66 as i64;
                        let var67 = self.memory.load64(var1.wrapping_add(160).wrapping_add(72) as usize) as i64;
                        slot_var2_0_i64 = var67 as i64;
                        let var68 = self.memory.load64(var1.wrapping_add(160).wrapping_add(80) as usize) as i64;
                        slot_var9_0_i64 = var68 as i64;
                        let var69 = self.memory.load64(var1.wrapping_add(160).wrapping_add(88) as usize) as i64;
                        slot_var10_0_i64 = var69 as i64;
                        let mut slot_var1_160_i64 = self.memory.load64(var1 as usize + 160) as i64;
                        slot_var1_464_i64 = slot_var1_160_i64 as i64;
                        let mut slot_var1_192_i64 = self.memory.load64(var1 as usize + 192) as i64;
                        slot_var1_560_i64 = slot_var1_192_i64 as i64;
                        let mut slot_var1_224_i64 = self.memory.load64(var1 as usize + 224) as i64;
                        slot_var1_672_i64 = slot_var1_224_i64 as i64;
                        var2 = 0;
                        var3 = 0;
                        'label8: loop {
                            'label9: loop {
                                if (var2 == 32) as i32 != 0 {
                                    break 'label8;
                                }
                                var4 = var1.wrapping_add(560).wrapping_add(var2);
                                var14 = slot_var4_0_i64;
                                let var70 = self.memory.load64(var1.wrapping_add(672).wrapping_add(var2) as usize) as i64;
                                input = var14.wrapping_add(var70);
                                var15 = input.wrapping_add(var3 as u32 as i64 & 1 /* True */);
                                slot_var4_0_i64 = var15 as i64;
                                var3 = ((input as u64) < var14 as u64) as i32 | ((var15 as u64) < input as u64) as i32;
                                var2 = var2.wrapping_add(8);
                                continue 'label9;
                                break;
                            }
                            break;
                        }
                        let var71 = self.memory.load64(var1.wrapping_add(560).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(312) as usize, var71 as u64);
                        let var72 = self.memory.load64(var1.wrapping_add(560).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(304) as usize, var72 as u64);
                        let var73 = self.memory.load64(var1.wrapping_add(560).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(296) as usize, var73 as u64);
                        let mut slot_var1_288_i64 = slot_var1_560_i64 as i64;
                        var3 = var1.wrapping_add(360).wrapping_add(24);
                        slot_var3_0_i64 = 0 /* False */ as i64;
                        var4 = var1.wrapping_add(360).wrapping_add(16);
                        slot_var4_0_i64 = 0 /* False */ as i64;
                        var5 = var1.wrapping_add(360).wrapping_add(8);
                        slot_var5_0_i64 = 0 /* False */ as i64;
                        slot_var1_360_i64 = 0 /* False */ as i64;
                        self.func4(env, var1.wrapping_add(16), var1.wrapping_add(360), 32);
                        let mut slot_var1_20_i32 = self.memory.load32(var1 as usize + 20) as i32;
                        var2 = slot_var1_20_i32;
                        if (var2 != 32) as i32 != 0 {
                            break 'label0;
                        }
                        let var75 = self.memory.load64(var1.wrapping_add(464).wrapping_add(8) as usize) as i64;
                        input = var75;
                        let var76 = self.memory.load64(var1.wrapping_add(464).wrapping_add(16) as usize) as i64;
                        var14 = var76;
                        var15 = slot_var1_464_i64;
                        let mut slot_var1_16_i32 = self.memory.load32(var1 as usize + 16) as i32;
                        var2 = slot_var1_16_i32;
                        let var77 = self.memory.load64(var1.wrapping_add(464).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var2.wrapping_add(24) as usize, var77 as u64);
                        self.memory.store64(var2.wrapping_add(16) as usize, var14 as u64);
                        self.memory.store64(var2.wrapping_add(8) as usize, input as u64);
                        slot_var2_0_i64 = var15 as i64;
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(24) as usize, slot_var3_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(16) as usize, slot_var4_0_i64 as u64);
                        self.memory.store64(var1.wrapping_add(880).wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                        slot_var1_880_i64 = slot_var1_360_i64 as i64;
                        var2 = var1.wrapping_add(256).wrapping_add(32);
                        let var78 = self.memory.load64(var2.wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(8) as usize, var78 as u64);
                        let var79 = self.memory.load64(var2.wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(16) as usize, var79 as u64);
                        let var80 = self.memory.load64(var2.wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(976).wrapping_add(24) as usize, var80 as u64);
                        slot_var1_976_i64 = slot_var2_0_i64 as i64;
                        var3 = var1.wrapping_add(976);
                        var2 = 31;
                        'label10: loop {
                            'label11: loop {
                                if (var2 == 15) as i32 != 0 {
                                    break 'label10;
                                }
                                let var81 = self.memory.load8(var3 as usize) as i32;
                                var4 = var81;
                                var5 = var1.wrapping_add(976).wrapping_add(var2);
                                let var82 = self.memory.load8(var5 as usize) as i32;
                                self.memory.store8(var3 as usize, var82 as u8);
                                self.memory.store8(var5 as usize, var4 as u8);
                                var2 = var2.wrapping_add(-1);
                                var3 = var3.wrapping_add(1);
                                continue 'label11;
                                break;
                            }
                            break;
                        }
                        let var83 = self.memory.load64(var1.wrapping_add(880).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(24) as usize, var83 as u64);
                        let var84 = self.memory.load64(var1.wrapping_add(880).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(16) as usize, var84 as u64);
                        let var85 = self.memory.load64(var1.wrapping_add(880).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(360).wrapping_add(8) as usize, var85 as u64);
                        let var86 = self.memory.load64(var1.wrapping_add(976).wrapping_add(8) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(400) as usize, var86 as u64);
                        let var87 = self.memory.load64(var1.wrapping_add(976).wrapping_add(16) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(408) as usize, var87 as u64);
                        let var88 = self.memory.load64(var1.wrapping_add(976).wrapping_add(24) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(416) as usize, var88 as u64);
                        slot_var1_360_i64 = slot_var1_880_i64 as i64;
                        let mut slot_var1_392_i64 = slot_var1_976_i64 as i64;
                        self.func16(env, var1.wrapping_add(8), 2, 1, 32);
                        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                        input = slot_var1_8_i64;
                        self.func16(env, var1, 4, 4, 4);
                        let mut slot_var1_984_i32 = 0 as i32;
                        slot_var1_976_i64 = input as i64;
                        let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
                        var2 = slot_var1_4_i32;
                        let mut slot_var1_992_i32 = var2 as i32;
                        let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                        let mut slot_var1_988_i32 = slot_var1_0_i32 as i32;
                        let mut slot_var2_0_i32 = 64 as i32;
                        let mut slot_var1_996_i32 = 1 as i32;
                        self.func19(env, var1.wrapping_add(360), var1.wrapping_add(976));
                        self.func19(env, var1.wrapping_add(360).wrapping_add(32), var1.wrapping_add(976));
                        let mut slot_var1_980_i32 = self.memory.load32(var1 as usize + 980) as i32;
                        var2 = slot_var1_980_i32;
                        let var93 = self.memory.load32(var1 as usize + 984) as i64;
                        input = var93;
                        var3 = slot_var1_976_i64;
                        self.func8(env, slot_var1_988_i32, slot_var1_992_i32, 4, 4);
                        let var95 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */
                        input = var95;
                        self.func7(env, var3.wrapping_shl(5 as u32), var2);
                        break;
                    }
                    self.global0 = var1.wrapping_add(1072);
                    return input;
                    break;
                }
                unreachable!();
                break;
            }
            self.func21(env);
            unreachable!();
            break;
        }
        self.func22(env, var2);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func21(&mut self, env: &Env) {
        self.func26(env);
        unreachable!();
    }
    fn func22(&mut self, env: &Env, mut arg0: i32) {
        self.func27(env);
        unreachable!();
    }
    fn func23(&mut self, env: &Env, mut arg0: i32) {
        self.func24(env, arg0);
        unreachable!();
    }
    fn func24(&mut self, env: &Env, mut arg0: i32) {
        self.func27(env);
        unreachable!();
    }
    fn func25(&mut self, env: &Env, mut arg0: i32) {
        self.func23(env, arg0);
        unreachable!();
    }
    fn func26(&mut self, env: &Env) {
        self.func27(env);
        unreachable!();
    }
    fn func27(&mut self, env: &Env) {
        unreachable!();
    }
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
    fn func29(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let var3 = self.func29(env, arg0, arg1, arg2);
        var3
    }
    fn func31(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
