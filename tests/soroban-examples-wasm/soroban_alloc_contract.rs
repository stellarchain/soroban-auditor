#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct AllocContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[contractimpl]
impl AllocContract {
    pub fn sum(&mut self, env: Env, count: u32) -> u32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (count & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                var2 = (count as u64).wrapping_shr(32 as u32) as i64 as i32;
                var3 = 0;
                let mut slot_var1_12_i32 = 0 as i32;
                let mut slot_var1_4_i64 = 17179869184 as i64;
                var4 = 4;
                var5 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var2 == var5) as i32 != 0 {
                            break 'label2;
                        }
                        'label4: loop {
                            if (var5 != slot_var1_4_i64) as i32 != 0 {
                                break 'label4;
                            }
                            self.func0(env, var1.wrapping_add(4));
                            let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                            var4 = slot_var1_8_i32;
                            break;
                        }
                        self.memory.store32(var4.wrapping_add(var3) as usize, var5 as u32);
                        var5 = var5.wrapping_add(1);
                        slot_var1_12_i32 = var5 as i32;
                        var3 = var3.wrapping_add(4);
                        var5 = var5;
                        continue 'label3;
                        break;
                    }
                    break;
                }
                var3 = var2.wrapping_shl(2 as u32);
                var5 = 0;
                var4 = slot_var1_8_i32;
                'label5: loop {
                    if (var3 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    'label6: loop {
                        let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
                        var2 = var5.wrapping_add(slot_var4_0_i32);
                        if ((var2 as u32) < var5 as u32) as i32 != 0 {
                            break 'label6;
                        }
                        var3 = var3.wrapping_add(-4);
                        var4 = var4.wrapping_add(4);
                        var5 = var2;
                        continue 'label5;
                        break;
                    }
                    break;
                }
                self.func4(env);
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(16);
        (var5 as u32 as i64).wrapping_shl(32 as u32) | 0
    }
}

#[allow(dead_code)]
impl AllocContract {
    fn func0(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let var8 = self.global0;
        var1 = var8.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var9 = self.memory.load32(arg0 as usize) as i32;
                    var2 = var9;
                    var3 = { let a = var2.wrapping_shl(1 as u32); let b = 1; if var2 != 0 { a } else { b } };
                    if (var3 as u32 > 1073741823 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    var4 = { let a = var3; let b = 4; if (var3 as u32 > 4 as u32) as i32 != 0 { a } else { b } };
                    var5 = var4.wrapping_shl(2 as u32);
                    if (var5 as u32 >= 2147483645 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    'label3: loop {
                        'label4: loop {
                            if (var2 == 0) as i32 != 0 {
                                break 'label4;
                            }
                            let var10 = self.memory.load32(arg0 as usize + 4) as i32;
                            var6 = var10;
                            self.func1(env);
                            let var12 = self.memory.load32(0 as usize + 1048576) as i32;
                            self.func2(env, var1.wrapping_add(8), var12);
                            let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                            if (slot_var1_8_i32 & 1 == 0) as i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                            var3 = slot_var1_12_i32;
                            var7 = var3.wrapping_add(var5);
                            if ((var7 as u32) < var3 as u32) as i32 != 0 {
                                break 'label1;
                            }
                            'label5: loop {
                                'label6: loop {
                                    let var14 = self.memory.load32(0 as usize + 1048580) as i32;
                                    if (var7 as u32 <= var14 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    let var15 = self.func3(env, var5);
                                    var3 = var15;
                                    break 'label5;
                                    break;
                                }
                                self.memory.store32(0 as usize + 1048576, var7 as u32);
                                break;
                            }
                            if (var3 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            let var16 = self.func11(env, var3, var6, var2.wrapping_shl(2 as u32));
                            var16;
                            break 'label3;
                            break;
                        }
                        self.func1(env);
                        let var18 = self.memory.load32(0 as usize + 1048576) as i32;
                        self.func2(env, var1, var18);
                        let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                        if (slot_var1_0_i32 & 1 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        let mut slot_var1_4_i32 = self.memory.load32(var1 as usize + 4) as i32;
                        var3 = slot_var1_4_i32;
                        var2 = var3.wrapping_add(var5);
                        if ((var2 as u32) < var3 as u32) as i32 != 0 {
                            break 'label1;
                        }
                        'label7: loop {
                            let var20 = self.memory.load32(0 as usize + 1048580) as i32;
                            if (var2 as u32 <= var20 as u32) as i32 != 0 {
                                break 'label7;
                            }
                            let var21 = self.func3(env, var5);
                            var3 = var21;
                            break 'label3;
                            break;
                        }
                        self.memory.store32(0 as usize + 1048576, var2 as u32);
                        break;
                    }
                    if (var3 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    self.memory.store32(arg0 as usize, var4 as u32);
                    self.memory.store32(arg0 as usize + 4, var3 as u32);
                    self.global0 = var1.wrapping_add(16);
                    return;
                    break;
                }
                self.func4(env);
                break;
            }
            unreachable!();
            break;
        }
        self.func5(env, var5);
        unreachable!();
    }
    fn func1(&mut self, env: &Env) {
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
    fn func2(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        'label0: loop {
            'label1: loop {
                var2 = arg1 & 3;
                if var2 != 0 {
                    break 'label1;
                }
                var2 = 1;
                break 'label0;
                break;
            }
            var3 = arg1.wrapping_sub(var2).wrapping_add(4);
            var2 = (var3 as u32 >= arg1 as u32) as i32;
            arg1 = var3;
            break;
        }
        self.memory.store32(arg0 as usize + 4, arg1 as u32);
        self.memory.store32(arg0 as usize, var2 as u32);
    }
    fn func3(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(16);
        self.global0 = var1;
        var2 = ((arg0 as u32).wrapping_shr(16 as u32) as i32).wrapping_add((arg0 & 65535 != 0) as i32);
        var3 = var2.wrapping_shl(16 as u32);
        'label0: loop {
            'label1: loop {
                let var7 = self.memory.grow(var2 as usize);
                if (var7 == -1) as i32 != 0 {
                    break 'label0;
                }
                let var8 = self.memory.load32(0 as usize + 1048580) as i32;
                var4 = var8;
                var5 = var4.wrapping_add(var3);
                if ((var5 as u32) < var4 as u32) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store32(0 as usize + 1048580, var5 as u32);
                self.func1(env);
                let var10 = self.memory.load32(0 as usize + 1048576) as i32;
                self.func2(env, var1.wrapping_add(8), var10);
                let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                if (slot_var1_8_i32 & 1 == 0) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                var4 = slot_var1_12_i32;
                var5 = var4.wrapping_add(arg0);
                if ((var5 as u32) < var4 as u32) as i32 != 0 {
                    break 'label0;
                }
                let var12 = self.memory.load32(0 as usize + 1048580) as i32;
                if (var5 as u32 > var12 as u32) as i32 != 0 {
                    continue 'label1;
                }
                break;
            }
            self.memory.store32(0 as usize + 1048576, var5 as u32);
            self.global0 = var1.wrapping_add(16);
            return var4;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func4(&mut self, env: &Env) {
        self.func9(env);
        unreachable!();
    }
    fn func5(&mut self, env: &Env, mut arg0: i32) {
        self.func7(env, arg0);
        unreachable!();
    }
    fn func6(&mut self, env: &Env, mut count: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (count & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                var2 = (count as u64).wrapping_shr(32 as u32) as i64 as i32;
                var3 = 0;
                let mut slot_var1_12_i32 = 0 as i32;
                let mut slot_var1_4_i64 = 17179869184 as i64;
                var4 = 4;
                var5 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var2 == var5) as i32 != 0 {
                            break 'label2;
                        }
                        'label4: loop {
                            if (var5 != slot_var1_4_i64) as i32 != 0 {
                                break 'label4;
                            }
                            self.func0(env, var1.wrapping_add(4));
                            let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                            var4 = slot_var1_8_i32;
                            break;
                        }
                        self.memory.store32(var4.wrapping_add(var3) as usize, var5 as u32);
                        var5 = var5.wrapping_add(1);
                        slot_var1_12_i32 = var5 as i32;
                        var3 = var3.wrapping_add(4);
                        var5 = var5;
                        continue 'label3;
                        break;
                    }
                    break;
                }
                var3 = var2.wrapping_shl(2 as u32);
                var5 = 0;
                var4 = slot_var1_8_i32;
                'label5: loop {
                    if (var3 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    'label6: loop {
                        let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
                        var2 = var5.wrapping_add(slot_var4_0_i32);
                        if ((var2 as u32) < var5 as u32) as i32 != 0 {
                            break 'label6;
                        }
                        var3 = var3.wrapping_add(-4);
                        var4 = var4.wrapping_add(4);
                        var5 = var2;
                        continue 'label5;
                        break;
                    }
                    break;
                }
                self.func4(env);
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(16);
        (var5 as u32 as i64).wrapping_shl(32 as u32) | 0
    }
    fn func7(&mut self, env: &Env, mut arg0: i32) {
        self.func8(env, arg0);
        unreachable!();
    }
    fn func8(&mut self, env: &Env, mut arg0: i32) {
        self.func9(env);
        unreachable!();
    }
    fn func9(&mut self, env: &Env) {
        unreachable!();
    }
    fn func10(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
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
    fn func11(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let var3 = self.func10(env, arg0, arg1, arg2);
        var3
    }
}
