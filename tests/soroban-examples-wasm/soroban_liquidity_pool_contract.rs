#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct LiquidityPoolContract;

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
pub enum DataKey { TokenA, TokenB, TotalShares, ReserveA, ReserveB, Shares(soroban_sdk::Address), }

#[contractimpl]
impl LiquidityPoolContract {
    pub fn ___constructor(&mut self, env: Env, token_a: soroban_sdk::Address, token_b: soroban_sdk::Address) {
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(token_a)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(token_b)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var2 = { let a = val_from_i64(token_a); let b = val_from_i64(token_b); if a < b { -1 } else if a > b { 1 } else { 0 } }
                if (var2 >= 0 /* False */) as i32 != 0 {
                    break 'label0;
                }
                self.func23(env, 0 /* False */, token_a);
                self.func23(env, 1 /* True */, token_b);
                self.func39(env, 0 /* False */, 0 /* False */);
                self.func40(env, 0 /* False */, 0 /* False */);
                self.func41(env, 0 /* False */, 0 /* False */);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func46(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn balance_shares(&mut self, mut env: Env, user: soroban_sdk::Address) -> i128 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(user)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func37(env, var1, user);
        let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        let var4 = self.func13(env, slot_var1_0_i64, slot_var1_8_i64);
        user = var4;
        self.global0 = var1.wrapping_add(16);
        user
    }
    pub fn deposit(&mut self, env: Env, to: soroban_sdk::Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128) {
        to.require_auth_for_args((desired_a, min_a, desired_b, min_b).into_val(&env));
    }
    pub fn swap(&mut self, env: Env, to: soroban_sdk::Address, buy_a: bool, out: i128, in_max: i128) {
        to.require_auth_for_args((buy_a, out, in_max).into_val(&env));
    }
    pub fn withdraw(&mut self, env: Env, to: soroban_sdk::Address, share_amount: i128, min_a: i128, min_b: i128) {
        to.require_auth_for_args((share_amount, min_a, min_b).into_val(&env));
    }
    pub fn get_rsrvs(&mut self, env: Env) -> (i128, i128,) {
        env.storage().instance().get(&DataKey::TokenA).unwrap()
    }
}

#[allow(dead_code)]
impl LiquidityPoolContract {
    fn func12(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(48);
        self.global0 = var5;
        let var8 = self.func13(env, arg3, arg4);
        let mut slot_var5_16_i64 = var8 as i64;
        let mut slot_var5_8_i64 = arg2 as i64;
        let mut slot_var5_0_i64 = arg1 as i64;
        var6 = 0;
        'label0: loop {
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
                            let var9 = self.memory.load64(var5.wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, var9 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var10 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    let var11 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(transfer)), Vec::<Val>::from_val(env, &val_from_i64(var10))))
                    if (var11 & 255 != 0 /* Void */) as i32 != 0 {
                        break 'label0;
                    }
                    self.global0 = var5.wrapping_add(48);
                    return;
                    break;
                }
                self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        self.func15(env);
        unreachable!();
    }
    fn func13(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func25(env, var2, arg0, arg1);
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
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func15(&mut self, env: &Env) {
        self.func46(env);
        unreachable!();
    }
    fn func16(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let var5 = self.func17(env, arg0, arg1);
        let var6 = self.func13(env, arg2, arg3);
        let var7 = match arg4 { 0 => { env.storage().persistent().set(&val_from_i64(var5), &val_from_i64(var6)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var5), &val_from_i64(var6)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var5), &val_from_i64(var6)); 0 } }
        var7;
    }
    fn func17(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            match arg0 as i32 {
                                                0 => break 'label8,
                                                1 => break 'label7,
                                                2 => break 'label6,
                                                3 => break 'label5,
                                                4 => break 'label4,
                                                5 => break 'label3,
                                                _ => break 'label8,
                                            }
                                            break;
                                        }
                                        self.func26(env, var2, 1048576, 6);
                                        let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                                        if slot_var2_0_i32 != 0 {
                                            break 'label1;
                                        }
                                        let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                                        self.func27(env, var2, slot_var2_8_i64);
                                        break 'label2;
                                        break;
                                    }
                                    self.func26(env, var2, 1048582, 6);
                                    if slot_var2_0_i32 != 0 {
                                        break 'label1;
                                    }
                                    self.func27(env, var2, slot_var2_8_i64);
                                    break 'label2;
                                    break;
                                }
                                self.func26(env, var2, 1048588, 11);
                                if slot_var2_0_i32 != 0 {
                                    break 'label1;
                                }
                                self.func27(env, var2, slot_var2_8_i64);
                                break 'label2;
                                break;
                            }
                            self.func26(env, var2, 1048599, 8);
                            if slot_var2_0_i32 != 0 {
                                break 'label1;
                            }
                            self.func27(env, var2, slot_var2_8_i64);
                            break 'label2;
                            break;
                        }
                        self.func26(env, var2, 1048607, 8);
                        if slot_var2_0_i32 != 0 {
                            break 'label1;
                        }
                        self.func27(env, var2, slot_var2_8_i64);
                        break 'label2;
                        break;
                    }
                    self.func26(env, var2, 1048615, 6);
                    if slot_var2_0_i32 != 0 {
                        break 'label1;
                    }
                    arg0 = slot_var2_8_i64;
                    slot_var2_8_i64 = arg1 as i64;
                    let mut slot_var2_0_i64 = arg0 as i64;
                    let var15 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    arg0 = var15;
                    break 'label0;
                    break;
                }
                arg0 = slot_var2_8_i64;
                if (slot_var2_0_i64 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var2.wrapping_add(16);
        arg0
    }
    fn func18(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var4 = self.func17(env, arg1, arg1);
                    arg1 = var4;
                    let var5 = self.func19(env, arg1, 0 /* Void */);
                    if var5 != 0 {
                        break 'label2;
                    }
                    arg1 = 0 /* False */;
                    break 'label1;
                    break;
                }
                let var6 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } }
                self.func20(env, var2, var6);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                if (slot_var2_0_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                arg1 = slot_var2_16_i64;
                let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                self.memory.store64(arg0 as usize + 24, slot_var2_24_i64 as u64);
                self.memory.store64(arg0 as usize + 16, arg1 as u64);
                arg1 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, arg1 as u64);
            self.memory.store64(arg0 as usize + 8, 0 /* False */ as u64);
            self.global0 = var2.wrapping_add(32);
            return;
            break;
        }
        unreachable!();
    }
    fn func19(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var2 == 1 /* True */) as i32
    }
    fn func20(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func21(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var3 = self.func17(env, arg1, var2);
                arg1 = var3;
                let var4 = self.func19(env, arg1, 0 /* Void */);
                if (var4 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var5 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } }
                var2 = var5;
                if (!(Address::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(arg0 as usize + 8, var2 as u64);
                var2 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var2 as u64);
            return;
            break;
        }
        unreachable!();
    }
    fn func22(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        self.func16(env, arg0, arg2, arg1, arg2, 0 /* Void */);
    }
    fn func23(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = self.func17(env, arg0, arg1);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 } }
        var3;
    }
    fn func24(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32);
        self.global0 = var4;
        self.func25(env, var4.wrapping_add(16), arg0, arg1);
        'label0: loop {
            'label1: loop {
                let mut slot_var4_16_i32 = self.memory.load32(var4 as usize + 16) as i32;
                if slot_var4_16_i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
                arg1 = slot_var4_24_i64;
                self.func25(env, var4.wrapping_add(16), arg2, arg3);
                if (slot_var4_16_i32 != 1) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        let mut slot_var4_8_i64 = slot_var4_24_i64 as i64;
        let mut slot_var4_0_i64 = arg1 as i64;
        let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        arg1 = var8;
        self.global0 = var4.wrapping_add(32);
        arg1
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func28(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func21(env, var0, 0 /* False */);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func29(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func29(&mut self, env: &Env) {
        self.func50(env, 43);
        unreachable!();
    }
    fn func30(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func21(env, var0, 1 /* True */);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            self.func29(env);
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func31(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        self.func18(env, var1, 0 /* Void */);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if slot_var1_0_i32 & 1 != 0 {
                break 'label0;
            }
            self.func29(env);
            unreachable!();
            break;
        }
        let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
        var2 = slot_var1_24_i64;
        let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
        self.memory.store64(arg0 as usize, slot_var1_16_i64 as u64);
        self.memory.store64(arg0 as usize + 8, var2 as u64);
        self.global0 = var1.wrapping_add(32);
    }
    fn func32(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        self.func18(env, var1, 3 /* Error(Contract, #0) */);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if slot_var1_0_i32 & 1 != 0 {
                break 'label0;
            }
            self.func29(env);
            unreachable!();
            break;
        }
        let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
        var2 = slot_var1_24_i64;
        let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
        self.memory.store64(arg0 as usize, slot_var1_16_i64 as u64);
        self.memory.store64(arg0 as usize + 8, var2 as u64);
        self.global0 = var1.wrapping_add(32);
    }
    fn func33(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        self.func18(env, var1, 0);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if slot_var1_0_i32 & 1 != 0 {
                break 'label0;
            }
            self.func29(env);
            unreachable!();
            break;
        }
        let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
        var2 = slot_var1_24_i64;
        let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
        self.memory.store64(arg0 as usize, slot_var1_16_i64 as u64);
        self.memory.store64(arg0 as usize + 8, var2 as u64);
        self.global0 = var1.wrapping_add(32);
    }
    fn func34(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        let var4 = val_to_i64(env.current_contract_address().into_val(env))
        let mut slot_var2_0_i64 = var4 as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        let var6 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg1)), &Symbol::from_val(env, &val_from_i64(balance)), Vec::<Val>::from_val(env, &val_from_i64(var5))))
        self.func20(env, var2, var6);
        'label0: loop {
            if (slot_var2_0_i64 != 1) as i32 != 0 {
                break 'label0;
            }
            self.func15(env);
            unreachable!();
            break;
        }
        let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
        arg1 = slot_var2_16_i64;
        let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
        self.memory.store64(arg0 as usize + 8, slot_var2_24_i64 as u64);
        self.memory.store64(arg0 as usize, arg1 as u64);
        self.global0 = var2.wrapping_add(32);
    }
    fn func35(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func28(env);
        self.func34(env, arg0, var1);
    }
    fn func36(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func30(env);
        self.func34(env, arg0, var1);
    }
    fn func37(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32);
        self.global0 = var2;
        var3 = 0 /* False */;
        var4 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var6 = self.func17(env, 0, arg1);
                arg1 = var6;
                let var7 = self.func19(env, arg1, 1 /* True */);
                if (var7 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var8 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg1)).unwrap_or(val_from_i64(0))) } }
                self.func20(env, var2, var8);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                if (slot_var2_0_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                var4 = slot_var2_24_i64;
                let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                var3 = slot_var2_16_i64;
                break;
            }
            self.memory.store64(arg0 as usize, var3 as u64);
            self.memory.store64(arg0 as usize + 8, var4 as u64);
            self.global0 = var2.wrapping_add(32);
            return;
            break;
        }
        unreachable!();
    }
    fn func38(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        self.func16(env, 0, arg0, arg1, arg2, 1 /* True */);
    }
    fn func39(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func22(env, 0 /* Void */, arg0, arg1);
    }
    fn func40(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func22(env, 3 /* Error(Contract, #0) */, arg0, arg1);
    }
    fn func41(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func22(env, 0, arg0, arg1);
    }
    fn func42(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let var4 = val_to_i64(env.current_contract_address().into_val(env))
        self.func12(env, arg0, var4, arg1, arg2, arg3);
    }
    fn func43(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let var3 = self.func28(env);
        self.func42(env, var3, arg0, arg1, arg2);
    }
    fn func44(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let var3 = self.func30(env);
        self.func42(env, var3, arg0, arg1, arg2);
    }
    fn func45(&mut self, env: &Env, mut token_a: i64, mut token_b: i64) -> i64 {
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(token_a)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(token_b)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var2 = { let a = val_from_i64(token_a); let b = val_from_i64(token_b); if a < b { -1 } else if a > b { 1 } else { 0 } }
                if (var2 >= 0 /* False */) as i32 != 0 {
                    break 'label0;
                }
                self.func23(env, 0 /* False */, token_a);
                self.func23(env, 1 /* True */, token_b);
                self.func39(env, 0 /* False */, 0 /* False */);
                self.func40(env, 0 /* False */, 0 /* False */);
                self.func41(env, 0 /* False */, 0 /* False */);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func46(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func46(&mut self, env: &Env) {
        unreachable!();
    }
    fn func47(&mut self, env: &Env, mut user: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(user)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func37(env, var1, user);
        let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        let var4 = self.func13(env, slot_var1_0_i64, slot_var1_8_i64);
        user = var4;
        self.global0 = var1.wrapping_add(16);
        user
    }
    fn func48(&mut self, env: &Env, mut to: i64, mut desired_a: i64, mut min_a: i64, mut desired_b: i64, mut min_b: i64) -> i64 {
        let mut var5: i32 = 0;
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
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var5 = var17.wrapping_sub(272);
        self.global0 = var5;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                                break 'label4;
                            }
                            self.func20(env, var5.wrapping_add(224), desired_a);
                            let mut slot_var5_224_i32 = self.memory.load32(var5 as usize + 224) as i32;
                            if (slot_var5_224_i32 == 1) as i32 != 0 {
                                break 'label4;
                            }
                            let mut slot_var5_248_i64 = self.memory.load64(var5 as usize + 248) as i64;
                            var6 = slot_var5_248_i64;
                            let mut slot_var5_240_i64 = self.memory.load64(var5 as usize + 240) as i64;
                            var7 = slot_var5_240_i64;
                            self.func20(env, var5.wrapping_add(224), min_a);
                            if (slot_var5_224_i32 == 1) as i32 != 0 {
                                break 'label4;
                            }
                            var8 = slot_var5_248_i64;
                            var9 = slot_var5_240_i64;
                            self.func20(env, var5.wrapping_add(224), desired_b);
                            if (slot_var5_224_i32 == 1) as i32 != 0 {
                                break 'label4;
                            }
                            var10 = slot_var5_248_i64;
                            var11 = slot_var5_240_i64;
                            self.func20(env, var5.wrapping_add(224), min_b);
                            if (slot_var5_224_i32 == 1) as i32 != 0 {
                                break 'label4;
                            }
                            var12 = slot_var5_248_i64;
                            var13 = slot_var5_240_i64;
                            let var22 = { address_from_i64(env, to).require_auth(); 0 }
                            var22;
                            self.func32(env, var5.wrapping_add(224));
                            min_a = slot_var5_224_i32;
                            let mut slot_var5_232_i64 = self.memory.load64(var5 as usize + 232) as i64;
                            desired_a = slot_var5_232_i64;
                            self.func33(env, var5.wrapping_add(224));
                            'label5: loop {
                                'label6: loop {
                                    var14 = slot_var5_224_i32;
                                    min_b = slot_var5_232_i64;
                                    if ((min_a | var14 | desired_a | min_b == 0) as i32 == 0) as i32 != 0 {
                                        break 'label6;
                                    }
                                    var15 = var11;
                                    desired_b = var10;
                                    break 'label5;
                                    break;
                                }
                                let mut slot_var5_220_i32 = 0 as i32;
                                self.func61(env, var5.wrapping_add(192), var7, var6, var14, min_b, var5.wrapping_add(220));
                                if slot_var5_220_i32 != 0 {
                                    break 'label1;
                                }
                                if (min_a | desired_a == 0) as i32 != 0 {
                                    break 'label1;
                                }
                                let mut slot_var5_200_i64 = self.memory.load64(var5 as usize + 200) as i64;
                                desired_b = slot_var5_200_i64;
                                let mut slot_var5_192_i64 = self.memory.load64(var5 as usize + 192) as i64;
                                var15 = slot_var5_192_i64;
                                'label7: loop {
                                    if (min_a & desired_a != 18446744073709551615) as i32 != 0 {
                                        break 'label7;
                                    }
                                    if (var15 | desired_b ^ 9223372036854775808 == 0) as i32 != 0 {
                                        break 'label1;
                                    }
                                    break;
                                }
                                self.func60(env, var5.wrapping_add(176), var15, desired_b, min_a, desired_a);
                                'label8: loop {
                                    let mut slot_var5_176_i64 = self.memory.load64(var5 as usize + 176) as i64;
                                    var15 = slot_var5_176_i64;
                                    let mut slot_var5_184_i64 = self.memory.load64(var5 as usize + 184) as i64;
                                    desired_b = slot_var5_184_i64;
                                    if (({ let a = (var15 as u64 > var11 as u64) as i32; let b = (desired_b > var10) as i32; if (desired_b == var10) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                                        break 'label8;
                                    }
                                    let mut slot_var5_172_i32 = 0 as i32;
                                    self.func61(env, var5.wrapping_add(144), var11, var10, min_a, desired_a, var5.wrapping_add(172));
                                    if slot_var5_172_i32 != 0 {
                                        break 'label1;
                                    }
                                    if (var14 | min_b == 0) as i32 != 0 {
                                        break 'label1;
                                    }
                                    'label9: loop {
                                        let mut slot_var5_144_i64 = self.memory.load64(var5 as usize + 144) as i64;
                                        desired_b = slot_var5_144_i64;
                                        let mut slot_var5_152_i64 = self.memory.load64(var5 as usize + 152) as i64;
                                        var15 = slot_var5_152_i64;
                                        if (desired_b | var15 ^ 9223372036854775808 != 0 /* False */) as i32 != 0 {
                                            break 'label9;
                                        }
                                        if (var14 & min_b == 18446744073709551615) as i32 != 0 {
                                            break 'label1;
                                        }
                                        break;
                                    }
                                    self.func60(env, var5.wrapping_add(128), desired_b, var15, var14, min_b);
                                    let mut slot_var5_128_i64 = self.memory.load64(var5 as usize + 128) as i64;
                                    var13 = slot_var5_128_i64;
                                    let mut slot_var5_136_i64 = self.memory.load64(var5 as usize + 136) as i64;
                                    var12 = slot_var5_136_i64;
                                    if ({ let a = (var13 as u64 > var7 as u64) as i32; let b = (var12 > var6) as i32; if (var12 == var6) as i32 != 0 { a } else { b } }) != 0 {
                                        break 'label0;
                                    }
                                    var7 = var13;
                                    var6 = var12;
                                    var15 = var11;
                                    desired_b = var10;
                                    if (({ let a = ((var13 as u64) < var9 as u64) as i32; let b = (var12 < var8) as i32; if (var12 == var8) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                                        break 'label5;
                                    }
                                    break 'label0;
                                    break;
                                }
                                if ({ let a = ((var15 as u64) < var13 as u64) as i32; let b = (desired_b < var12) as i32; if (desired_b == var12) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label0;
                                }
                                break;
                            }
                            if ({ let a = (var7 == 0) as i32; let b = (var6 < 0 /* False */) as i32; if (var6 == 0) as i32 != 0 { a } else { b } }) != 0 {
                                break 'label0;
                            }
                            if (({ let a = (var15 != 0 /* False */) as i32; let b = (desired_b > 0 /* False */) as i32; if (desired_b == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                                break 'label0;
                            }
                            let var29 = self.func28(env);
                            var10 = var29;
                            let var30 = self.func30(env);
                            var11 = var30;
                            let var31 = val_to_i64(env.current_contract_address().into_val(env))
                            self.func12(env, var10, to, var31, var7, var6);
                            let var33 = val_to_i64(env.current_contract_address().into_val(env))
                            self.func12(env, var11, to, var33, var15, desired_b);
                            self.func35(env, var5.wrapping_add(224));
                            var7 = slot_var5_232_i64;
                            var15 = slot_var5_224_i32;
                            self.func36(env, var5.wrapping_add(224));
                            var10 = slot_var5_232_i64;
                            var11 = slot_var5_224_i32;
                            self.func31(env, var5.wrapping_add(224));
                            var6 = slot_var5_232_i64;
                            desired_b = slot_var5_224_i32;
                            'label10: loop {
                                'label11: loop {
                                    if ({ let a = (min_a == 0) as i32; let b = (desired_a < 0 /* False */) as i32; if (desired_a == 0) as i32 != 0 { a } else { b } }) != 0 {
                                        break 'label11;
                                    }
                                    if ({ let a = (var14 != 0 /* False */) as i32; let b = (min_b > 0 /* False */) as i32; if (min_b == 0) as i32 != 0 { a } else { b } }) != 0 {
                                        break 'label10;
                                    }
                                    break;
                                }
                                let mut slot_var5_124_i32 = 0 as i32;
                                self.func61(env, var5.wrapping_add(96), var15, var7, var11, var10, var5.wrapping_add(124));
                                if slot_var5_124_i32 != 0 {
                                    break 'label1;
                                }
                                let mut slot_var5_104_i64 = self.memory.load64(var5 as usize + 104) as i64;
                                desired_a = slot_var5_104_i64;
                                if (desired_a <= 18446744073709551615) as i32 != 0 {
                                    break 'label3;
                                }
                                let mut slot_var5_96_i64 = self.memory.load64(var5 as usize + 96) as i64;
                                self.func49(env, var5.wrapping_add(256), slot_var5_96_i64, desired_a);
                                let mut slot_var5_264_i64 = self.memory.load64(var5 as usize + 264) as i64;
                                min_a = slot_var5_264_i64;
                                let mut slot_var5_256_i64 = self.memory.load64(var5 as usize + 256) as i64;
                                min_b = slot_var5_256_i64;
                                break 'label2;
                                break;
                            }
                            let mut slot_var5_92_i32 = 0 as i32;
                            self.func61(env, var5.wrapping_add(64), var15, var7, desired_b, var6, var5.wrapping_add(92));
                            if slot_var5_92_i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var5_72_i64 = self.memory.load64(var5 as usize + 72) as i64;
                            var12 = slot_var5_72_i64;
                            let mut slot_var5_64_i64 = self.memory.load64(var5 as usize + 64) as i64;
                            var8 = slot_var5_64_i64;
                            let mut slot_var5_60_i32 = 0 as i32;
                            self.func61(env, var5.wrapping_add(32), var11, var10, desired_b, var6, var5.wrapping_add(60));
                            if slot_var5_60_i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var5_40_i64 = self.memory.load64(var5 as usize + 40) as i64;
                            var13 = slot_var5_40_i64;
                            let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
                            var9 = slot_var5_32_i64;
                            self.func60(env, var5.wrapping_add(16), var8, var12, min_a, desired_a);
                            self.func60(env, var5, var9, var13, var14, min_b);
                            let mut slot_var5_8_i64 = self.memory.load64(var5 as usize + 8) as i64;
                            desired_a = slot_var5_8_i64;
                            let mut slot_var5_24_i64 = self.memory.load64(var5 as usize + 24) as i64;
                            min_a = slot_var5_24_i64;
                            let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
                            min_b = slot_var5_0_i64;
                            let mut slot_var5_16_i64 = self.memory.load64(var5 as usize + 16) as i64;
                            var14 = slot_var5_16_i64;
                            var16 = { let a = ((min_b as u64) < var14 as u64) as i32; let b = (desired_a < min_a) as i32; if (desired_a == min_a) as i32 != 0 { a } else { b } };
                            min_a = { let a = desired_a; let b = min_a; if var16 != 0 { a } else { b } };
                            min_b = { let a = min_b; let b = var14; if var16 != 0 { a } else { b } };
                            break 'label2;
                            break;
                        }
                        unreachable!();
                        break;
                    }
                    self.func50(env, 42);
                    unreachable!();
                    break;
                }
                desired_a = min_a.wrapping_sub(var6).wrapping_sub(((min_b as u64) < desired_b as u64) as i32 as u32 as i64);
                if ((min_a ^ var6) & (min_a ^ desired_a) < 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                self.func37(env, var5.wrapping_add(224), to);
                min_a = slot_var5_232_i64;
                var6 = slot_var5_224_i32;
                self.func31(env, var5.wrapping_add(224));
                desired_b = min_b.wrapping_sub(desired_b);
                min_b = var6.wrapping_add(desired_b);
                var14 = min_a.wrapping_add(desired_a).wrapping_add(((min_b as u64) < var6 as u64) as i32 as u32 as i64);
                if ((min_a ^ desired_a ^ 18446744073709551615) & (min_a ^ var14) < 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                min_a = slot_var5_232_i64;
                var6 = slot_var5_224_i32;
                self.func38(env, to, min_b, var14);
                let var48 = desired_a;
                to = var6.wrapping_add(desired_b);
                desired_a = min_a.wrapping_add(desired_a).wrapping_add(((to as u64) < var6 as u64) as i32 as u32 as i64);
                if ((min_a ^ var48 ^ 18446744073709551615) & (min_a ^ desired_a) < 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                self.func39(env, to, desired_a);
                self.func40(env, var15, var7);
                self.func41(env, var11, var10);
                self.global0 = var5.wrapping_add(272);
                return 0 /* Void */;
                break;
            }
            self.func15(env);
            unreachable!();
            break;
        }
        self.func46(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func49(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let var10 = self.global0;
        var3 = var10.wrapping_sub(48);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (arg2 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    self.func49(env, var3.wrapping_add(32), (arg1 as u64).wrapping_shr(0 /* Void */ as u32) as i64 | arg2.wrapping_shl(62 as u32), (arg2 as u64).wrapping_shr(0 /* Void */ as u32) as i64);
                    let mut slot_var3_40_i64 = self.memory.load64(var3 as usize + 40) as i64;
                    let mut slot_var3_32_i64 = self.memory.load64(var3 as usize + 32) as i64;
                    var4 = slot_var3_32_i64;
                    var5 = slot_var3_40_i64.wrapping_shl(1 /* True */ as u32) | (var4 as u64).wrapping_shr(63 as u32) as i64;
                    var6 = var4.wrapping_shl(1 /* True */ as u32);
                    var4 = var6 | 1 /* True */;
                    self.func56(env, var3, var5, 0 /* False */, var4, 0 /* False */);
                    self.func56(env, var3.wrapping_add(16), var4, 0 /* False */, var4, 0 /* False */);
                    let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                    let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                    var7 = slot_var3_24_i64;
                    let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                    var8 = slot_var3_0_i64;
                    var8 = var7.wrapping_add(var8.wrapping_add(var8));
                    if (var5 | slot_var3_8_i64 != 0 /* False */) as i32 | ((var8 as u64) < var7 as u64) as i32 != 0 {
                        break 'label0;
                    }
                    let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
                    var9 = { let a = (slot_var3_16_i64 as u64 > arg1 as u64) as i32; let b = (var8 as u64 > arg2 as u64) as i32; if (var8 == arg2) as i32 != 0 { a } else { b } };
                    var5 = { let a = var5; let b = var5; if var9 != 0 { a } else { b } };
                    arg2 = { let a = var6; let b = var4; if var9 != 0 { a } else { b } };
                    break 'label1;
                    break;
                }
                'label3: loop {
                    'label4: loop {
                        if ((arg1 as u64) < 0 as u64) as i32 != 0 {
                            break 'label4;
                        }
                        var4 = 1 /* True */.wrapping_shl((64 /* U64(obj#0) */.wrapping_sub(arg1.leading_zeros() as i64) as u64).wrapping_shr(1 /* True */ as u32) as i64 as u32);
                        'label5: loop {
                            arg2 = var4;
                            let var14 = self.func55(env, arg1, arg2);
                            var4 = var14;
                            if ((arg2 as u64) < var4 as u64) as i32 != 0 {
                                continue 'label5;
                            }
                            break;
                        }
                        'label6: loop {
                            var5 = var4;
                            if (arg2 as u64 <= var5 as u64) as i32 != 0 {
                                break 'label3;
                            }
                            let var15 = self.func55(env, arg1, var5);
                            var4 = var15;
                            arg2 = var5;
                            continue 'label6;
                            break;
                        }
                        break;
                    }
                    arg2 = (arg1 != 0 /* False */) as i32 as u32 as i64;
                    break;
                }
                var5 = 0 /* False */;
                break;
            }
            self.memory.store64(arg0 as usize, arg2 as u64);
            self.memory.store64(arg0 as usize + 8, var5 as u64);
            self.global0 = var3.wrapping_add(48);
            return;
            break;
        }
        self.func15(env);
        unreachable!();
    }
    fn func50(&mut self, env: &Env, mut arg0: i32) {
        self.func46(env);
        unreachable!();
    }
    fn func51(&mut self, env: &Env, mut to: i64, mut buy_a: i64, mut out: i64, mut in_max: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let mut var13: i32 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let var23 = self.global0;
        var4 = var23.wrapping_sub(336);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    var5 = buy_a as i32 & 255;
                    var5 = { let a = 1; let b = ((var5 != 0) as i32).wrapping_shl(1 as u32); if (var5 == 1) as i32 != 0 { a } else { b } };
                    if (var5 == 2) as i32 != 0 {
                        break 'label2;
                    }
                    self.func20(env, var4.wrapping_add(240), out);
                    let mut slot_var4_240_i32 = self.memory.load32(var4 as usize + 240) as i32;
                    if (slot_var4_240_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var4_264_i64 = self.memory.load64(var4 as usize + 264) as i64;
                    buy_a = slot_var4_264_i64;
                    let mut slot_var4_256_i64 = self.memory.load64(var4 as usize + 256) as i64;
                    out = slot_var4_256_i64;
                    self.func20(env, var4.wrapping_add(240), in_max);
                    if (slot_var4_240_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    var6 = slot_var4_264_i64;
                    var7 = slot_var4_256_i64;
                    let var26 = { address_from_i64(env, to).require_auth(); 0 }
                    var26;
                    self.func32(env, var4.wrapping_add(240));
                    var8 = slot_var4_240_i32;
                    let mut slot_var4_248_i64 = self.memory.load64(var4 as usize + 248) as i64;
                    var9 = slot_var4_248_i64;
                    self.func33(env, var4.wrapping_add(240));
                    var10 = slot_var4_240_i32;
                    var11 = var5 & 1;
                    var12 = { let a = var8; let b = var10; if var11 != 0 { a } else { b } };
                    var13 = ((var12 as u64) < out as u64) as i32;
                    var14 = slot_var4_248_i64;
                    in_max = { let a = var9; let b = var14; if var11 != 0 { a } else { b } };
                    if ({ let a = var13; let b = (in_max < buy_a) as i32; if (in_max == buy_a) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label0;
                    }
                    let mut slot_var4_236_i32 = 0 as i32;
                    var11 = var5 & 1;
                    self.func61(env, var4.wrapping_add(208), { let a = var10; let b = var8; if var11 != 0 { a } else { b } }, { let a = var14; let b = var9; if var11 != 0 { a } else { b } }, out, buy_a, var4.wrapping_add(236));
                    if slot_var4_236_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_216_i64 = self.memory.load64(var4 as usize + 216) as i64;
                    var15 = slot_var4_216_i64;
                    let mut slot_var4_208_i64 = self.memory.load64(var4 as usize + 208) as i64;
                    var16 = slot_var4_208_i64;
                    let mut slot_var4_204_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(176), var16, var15, 1000, 0 /* False */, var4.wrapping_add(204));
                    if slot_var4_204_i32 != 0 {
                        break 'label1;
                    }
                    var17 = in_max.wrapping_sub(buy_a).wrapping_sub(var13 as u32 as i64);
                    if ((in_max ^ buy_a) & (in_max ^ var17) < 0 /* False */) as i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_184_i64 = self.memory.load64(var4 as usize + 184) as i64;
                    var15 = slot_var4_184_i64;
                    let mut slot_var4_176_i64 = self.memory.load64(var4 as usize + 176) as i64;
                    var16 = slot_var4_176_i64;
                    let mut slot_var4_172_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(144), var12.wrapping_sub(out), var17, 997, 0 /* False */, var4.wrapping_add(172));
                    if slot_var4_172_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_144_i64 = self.memory.load64(var4 as usize + 144) as i64;
                    in_max = slot_var4_144_i64;
                    let mut slot_var4_152_i64 = self.memory.load64(var4 as usize + 152) as i64;
                    var12 = slot_var4_152_i64;
                    if (in_max | var12 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    'label3: loop {
                        if (in_max & var12 != 18446744073709551615) as i32 != 0 {
                            break 'label3;
                        }
                        if (var16 | var15 ^ 9223372036854775808 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        break;
                    }
                    self.func60(env, var4.wrapping_add(128), var16, var15, in_max, var12);
                    let mut slot_var4_136_i64 = self.memory.load64(var4 as usize + 136) as i64;
                    var12 = slot_var4_136_i64;
                    let mut slot_var4_128_i64 = self.memory.load64(var4 as usize + 128) as i64;
                    var17 = slot_var4_128_i64.wrapping_add(1 /* True */);
                    in_max = var12.wrapping_add((var17 == 0) as i32 as u32 as i64);
                    if ((var12 ^ 18446744073709551615) & (var12 ^ in_max) < 0 /* False */) as i32 != 0 {
                        break 'label1;
                    }
                    if ({ let a = (var17 as u64 > var7 as u64) as i32; let b = (in_max > var6) as i32; if (in_max == var6) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label0;
                    }
                    'label4: loop {
                        'label5: loop {
                            if var5 & 1 != 0 {
                                break 'label5;
                            }
                            var15 = 0 /* False */;
                            let var33 = self.func28(env);
                            var6 = var33;
                            var7 = out;
                            var16 = buy_a;
                            var12 = 0 /* False */;
                            break 'label4;
                            break;
                        }
                        var7 = 0 /* False */;
                        let var34 = self.func30(env);
                        var6 = var34;
                        var16 = 0 /* False */;
                        var15 = out;
                        var12 = buy_a;
                        break;
                    }
                    let var35 = val_to_i64(env.current_contract_address().into_val(env))
                    self.func12(env, var6, to, var35, var17, in_max);
                    self.func35(env, var4.wrapping_add(240));
                    in_max = slot_var4_248_i64;
                    var17 = slot_var4_240_i32;
                    self.func36(env, var4.wrapping_add(240));
                    let mut slot_var4_296_i64 = 0 /* False */ as i64;
                    let mut slot_var4_288_i64 = 1000 as i64;
                    var6 = slot_var4_248_i64;
                    var18 = slot_var4_240_i32;
                    let mut slot_var4_280_i64 = 0 /* False */ as i64;
                    let mut slot_var4_272_i64 = 997 as i64;
                    let mut slot_var4_312_i64 = 0 /* False */ as i64;
                    let mut slot_var4_304_i64 = 0 /* False */ as i64;
                    let mut slot_var4_332_i32 = var4.wrapping_add(288) as i32;
                    let mut slot_var4_328_i32 = var4.wrapping_add(272) as i32;
                    let mut slot_var4_324_i32 = var4.wrapping_add(304) as i32;
                    self.func52(env, var4.wrapping_add(240), var4.wrapping_add(324), var17, in_max, var8, var9, var15, var12);
                    var19 = slot_var4_248_i64;
                    var20 = slot_var4_240_i32;
                    self.func52(env, var4.wrapping_add(240), var4.wrapping_add(324), var18, var6, var10, var14, var7, var16);
                    let mut slot_var4_124_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(96), var8, var9, 1000, 0 /* False */, var4.wrapping_add(124));
                    if slot_var4_124_i32 != 0 {
                        break 'label1;
                    }
                    var8 = slot_var4_248_i64;
                    var9 = slot_var4_240_i32;
                    let mut slot_var4_104_i64 = self.memory.load64(var4 as usize + 104) as i64;
                    var21 = slot_var4_104_i64;
                    let mut slot_var4_96_i64 = self.memory.load64(var4 as usize + 96) as i64;
                    var22 = slot_var4_96_i64;
                    let mut slot_var4_92_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(64), var10, var14, 1000, 0 /* False */, var4.wrapping_add(92));
                    if slot_var4_92_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_72_i64 = self.memory.load64(var4 as usize + 72) as i64;
                    var10 = slot_var4_72_i64;
                    let mut slot_var4_64_i64 = self.memory.load64(var4 as usize + 64) as i64;
                    var14 = slot_var4_64_i64;
                    let mut slot_var4_60_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(32), var20, var19, var9, var8, var4.wrapping_add(60));
                    if slot_var4_60_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_40_i64 = self.memory.load64(var4 as usize + 40) as i64;
                    var8 = slot_var4_40_i64;
                    let mut slot_var4_32_i64 = self.memory.load64(var4 as usize + 32) as i64;
                    var9 = slot_var4_32_i64;
                    let mut slot_var4_28_i32 = 0 as i32;
                    self.func61(env, var4, var22, var21, var14, var10, var4.wrapping_add(28));
                    if slot_var4_28_i32 != 0 {
                        break 'label1;
                    }
                    let var45 = var9;
                    let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
                    let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
                    var9 = slot_var4_8_i64;
                    if ({ let a = ((var45 as u64) < slot_var4_0_i64 as u64) as i32; let b = (var8 < var9) as i32; if (var8 == var9) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label0;
                    }
                    'label6: loop {
                        'label7: loop {
                            if var5 & 1 != 0 {
                                break 'label7;
                            }
                            self.func44(env, to, out, buy_a);
                            break 'label6;
                            break;
                        }
                        self.func43(env, to, out, buy_a);
                        break;
                    }
                    buy_a = in_max.wrapping_sub(var12).wrapping_sub(((var17 as u64) < var15 as u64) as i32 as u32 as i64);
                    if ((in_max ^ var12) & (in_max ^ buy_a) < 0 /* False */) as i32 != 0 {
                        break 'label1;
                    }
                    out = var6.wrapping_sub(var16).wrapping_sub(((var18 as u64) < var7 as u64) as i32 as u32 as i64);
                    if ((var6 ^ var16) & (var6 ^ out) < 0 /* False */) as i32 != 0 {
                        break 'label1;
                    }
                    in_max = var17.wrapping_sub(var15);
                    if ({ let a = (in_max == 0) as i32; let b = (buy_a < 0 /* False */) as i32; if (buy_a == 0) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label0;
                    }
                    to = var18.wrapping_sub(var7);
                    if ({ let a = (to == 0) as i32; let b = (out < 0 /* False */) as i32; if (out == 0) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label0;
                    }
                    self.func40(env, in_max, buy_a);
                    self.func41(env, to, out);
                    self.global0 = var4.wrapping_add(336);
                    return 0 /* Void */;
                    break;
                }
                unreachable!();
                break;
            }
            self.func15(env);
            unreachable!();
            break;
        }
        self.func46(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func52(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64, mut arg7: i64) {
        let mut var8: i32 = 0;
        let mut var9: i64 = 0;
        let mut var10: i32 = 0;
        let var11 = self.global0;
        var8 = var11.wrapping_sub(96);
        self.global0 = var8;
        'label0: loop {
            'label1: loop {
                var9 = arg3.wrapping_sub(arg5).wrapping_sub(((arg2 as u64) < arg4 as u64) as i32 as u32 as i64);
                if ((arg3 ^ arg5) & (arg3 ^ var9) < 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                arg2 = arg2.wrapping_sub(arg4);
                arg3 = var9.wrapping_sub(arg7).wrapping_sub(((arg2 as u64) < arg6 as u64) as i32 as u32 as i64);
                if ((var9 ^ arg7) & (var9 ^ arg3) < 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    'label3: loop {
                        var9 = arg2.wrapping_sub(arg6);
                        let var12 = self.memory.load32(arg1 as usize) as i32;
                        var10 = var12;
                        let mut slot_var10_0_i64 = self.memory.load64(var10 as usize) as i64;
                        let mut slot_var10_8_i64 = self.memory.load64(var10 as usize + 8) as i64;
                        arg2 = slot_var10_8_i64;
                        if ({ let a = (var9 as u64 > slot_var10_0_i64 as u64) as i32; let b = (arg3 > arg2) as i32; if (arg3 == arg2) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label3;
                        }
                        let mut slot_var8_92_i32 = 0 as i32;
                        let var13 = self.memory.load32(arg1 as usize + 8) as i32;
                        arg1 = var13;
                        let var14 = self.memory.load64(arg1 as usize) as i64;
                        arg2 = var14;
                        let var15 = self.memory.load64(arg1 as usize + 8) as i64;
                        arg7 = var15;
                        self.func61(env, var8.wrapping_add(64), arg2, arg7, var9, arg3, var8.wrapping_add(92));
                        if slot_var8_92_i32 != 0 {
                            break 'label1;
                        }
                        let mut slot_var8_72_i64 = self.memory.load64(var8 as usize + 72) as i64;
                        arg3 = slot_var8_72_i64;
                        let mut slot_var8_64_i64 = self.memory.load64(var8 as usize + 64) as i64;
                        var9 = slot_var8_64_i64;
                        break 'label2;
                        break;
                    }
                    let mut slot_var8_60_i32 = 0 as i32;
                    let var17 = self.memory.load32(arg1 as usize + 4) as i32;
                    var10 = var17;
                    self.func61(env, var8.wrapping_add(32), slot_var10_0_i64, slot_var10_8_i64, var9, arg3, var8.wrapping_add(60));
                    if slot_var8_60_i32 != 0 {
                        break 'label1;
                    }
                    let mut slot_var8_40_i64 = self.memory.load64(var8 as usize + 40) as i64;
                    arg3 = slot_var8_40_i64;
                    let mut slot_var8_32_i64 = self.memory.load64(var8 as usize + 32) as i64;
                    var9 = slot_var8_32_i64;
                    let var19 = self.memory.load32(arg1 as usize + 8) as i32;
                    arg1 = var19;
                    let var20 = self.memory.load64(arg1 as usize + 8) as i64;
                    arg7 = var20;
                    let var21 = self.memory.load64(arg1 as usize) as i64;
                    arg2 = var21;
                    break;
                }
                let mut slot_var8_28_i32 = 0 as i32;
                self.func61(env, var8, arg2, arg7, arg4, arg5, var8.wrapping_add(28));
                if slot_var8_28_i32 != 0 {
                    break 'label1;
                }
                let mut slot_var8_8_i64 = self.memory.load64(var8 as usize + 8) as i64;
                arg5 = slot_var8_8_i64;
                let var23 = arg3;
                let mut slot_var8_0_i64 = self.memory.load64(var8 as usize) as i64;
                arg3 = slot_var8_0_i64;
                var9 = arg3.wrapping_add(var9);
                arg3 = arg5.wrapping_add(arg3).wrapping_add(((var9 as u64) < arg3 as u64) as i32 as u32 as i64);
                if ((arg5 ^ var23 ^ 18446744073709551615) & (arg5 ^ arg3) >= 0 /* False */) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            self.func15(env);
            unreachable!();
            break;
        }
        self.memory.store64(arg0 as usize, var9 as u64);
        self.memory.store64(arg0 as usize + 8, arg3 as u64);
        self.global0 = var8.wrapping_add(96);
    }
    fn func53(&mut self, env: &Env, mut to: i64, mut share_amount: i64, mut min_a: i64, mut min_b: i64) -> i64 {
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
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i32 = 0;
        let var19 = self.global0;
        var4 = var19.wrapping_sub(128);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    self.func20(env, var4.wrapping_add(96), share_amount);
                    let mut slot_var4_96_i32 = self.memory.load32(var4 as usize + 96) as i32;
                    if (slot_var4_96_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var4_120_i64 = self.memory.load64(var4 as usize + 120) as i64;
                    share_amount = slot_var4_120_i64;
                    let mut slot_var4_112_i64 = self.memory.load64(var4 as usize + 112) as i64;
                    var5 = slot_var4_112_i64;
                    self.func20(env, var4.wrapping_add(96), min_a);
                    if (slot_var4_96_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    var6 = slot_var4_120_i64;
                    var7 = slot_var4_112_i64;
                    self.func20(env, var4.wrapping_add(96), min_b);
                    if (slot_var4_96_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    var8 = slot_var4_120_i64;
                    var9 = slot_var4_112_i64;
                    let var23 = { address_from_i64(env, to).require_auth(); 0 }
                    var23;
                    self.func37(env, var4.wrapping_add(96), to);
                    let mut slot_var4_104_i64 = self.memory.load64(var4 as usize + 104) as i64;
                    min_a = slot_var4_104_i64;
                    if ({ let a = ((slot_var4_96_i32 as u64) < var5 as u64) as i32; let b = (min_a < share_amount) as i32; if (min_a == share_amount) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_92_i32 = 0 as i32;
                    self.func35(env, var4.wrapping_add(96));
                    var10 = slot_var4_96_i32;
                    var11 = slot_var4_104_i64;
                    self.func61(env, var4.wrapping_add(64), var10, var11, var5, share_amount, var4.wrapping_add(92));
                    self.func36(env, var4.wrapping_add(96));
                    var12 = slot_var4_104_i64;
                    var13 = slot_var4_96_i32;
                    self.func31(env, var4.wrapping_add(96));
                    if slot_var4_92_i32 != 0 {
                        break 'label0;
                    }
                    var14 = slot_var4_96_i32;
                    var15 = slot_var4_104_i64;
                    if (var14 | var15 == 0) as i32 != 0 {
                        break 'label0;
                    }
                    'label3: loop {
                        let mut slot_var4_64_i64 = self.memory.load64(var4 as usize + 64) as i64;
                        min_a = slot_var4_64_i64;
                        let mut slot_var4_72_i64 = self.memory.load64(var4 as usize + 72) as i64;
                        min_b = slot_var4_72_i64;
                        if (min_a | min_b ^ 9223372036854775808 != 0 /* False */) as i32 != 0 {
                            break 'label3;
                        }
                        if (var14 & var15 == 18446744073709551615) as i32 != 0 {
                            break 'label0;
                        }
                        break;
                    }
                    self.func60(env, var4.wrapping_add(48), min_a, min_b, var14, var15);
                    let mut slot_var4_44_i32 = 0 as i32;
                    self.func61(env, var4.wrapping_add(16), var13, var12, var5, share_amount, var4.wrapping_add(44));
                    if slot_var4_44_i32 != 0 {
                        break 'label0;
                    }
                    let mut slot_var4_56_i64 = self.memory.load64(var4 as usize + 56) as i64;
                    min_a = slot_var4_56_i64;
                    let mut slot_var4_48_i64 = self.memory.load64(var4 as usize + 48) as i64;
                    min_b = slot_var4_48_i64;
                    'label4: loop {
                        let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
                        var16 = slot_var4_16_i64;
                        let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
                        var17 = slot_var4_24_i64;
                        if (var16 | var17 ^ 9223372036854775808 != 0 /* False */) as i32 != 0 {
                            break 'label4;
                        }
                        if (var14 & var15 == 18446744073709551615) as i32 != 0 {
                            break 'label0;
                        }
                        break;
                    }
                    self.func60(env, var4, var16, var17, var14, var15);
                    if ({ let a = ((min_b as u64) < var7 as u64) as i32; let b = (min_a < var6) as i32; if (min_a == var6) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label1;
                    }
                    let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
                    var6 = slot_var4_0_i64;
                    let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
                    var14 = slot_var4_8_i64;
                    if ({ let a = ((var6 as u64) < var9 as u64) as i32; let b = (var14 < var8) as i32; if (var14 == var8) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label1;
                    }
                    self.func37(env, var4.wrapping_add(96), to);
                    var7 = slot_var4_96_i32;
                    var18 = ((var7 as u64) < var5 as u64) as i32;
                    var15 = slot_var4_104_i64;
                    if ({ let a = var18; let b = (var15 < share_amount) as i32; if (var15 == share_amount) as i32 != 0 { a } else { b } }) != 0 {
                        break 'label1;
                    }
                    self.func31(env, var4.wrapping_add(96));
                    var9 = var15.wrapping_sub(share_amount).wrapping_sub(var18 as u32 as i64);
                    if ((var15 ^ share_amount) & (var15 ^ var9) < 0 /* False */) as i32 != 0 {
                        break 'label0;
                    }
                    var15 = slot_var4_104_i64;
                    var8 = slot_var4_96_i32;
                    self.func38(env, to, var7.wrapping_sub(var5), var9);
                    let var35 = share_amount;
                    share_amount = var15.wrapping_sub(share_amount).wrapping_sub(((var8 as u64) < var5 as u64) as i32 as u32 as i64);
                    if ((var15 ^ var35) & (var15 ^ share_amount) < 0 /* False */) as i32 != 0 {
                        break 'label0;
                    }
                    self.func39(env, var8.wrapping_sub(var5), share_amount);
                    self.func43(env, to, min_b, min_a);
                    self.func44(env, to, var6, var14);
                    share_amount = var11.wrapping_sub(min_a).wrapping_sub(((var10 as u64) < min_b as u64) as i32 as u32 as i64);
                    if ((var11 ^ min_a) & (var11 ^ share_amount) < 0 /* False */) as i32 != 0 {
                        break 'label0;
                    }
                    self.func40(env, var10.wrapping_sub(min_b), share_amount);
                    share_amount = var12.wrapping_sub(var14).wrapping_sub(((var13 as u64) < var6 as u64) as i32 as u32 as i64);
                    if ((var12 ^ var14) & (var12 ^ share_amount) < 0 /* False */) as i32 != 0 {
                        break 'label0;
                    }
                    self.func41(env, var13.wrapping_sub(var6), share_amount);
                    let var41 = self.func24(env, min_b, min_a, var6, var14);
                    share_amount = var41;
                    self.global0 = var4.wrapping_add(128);
                    return share_amount;
                    break;
                }
                unreachable!();
                break;
            }
            self.func46(env);
            unreachable!();
            break;
        }
        self.func15(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func54(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        self.func32(env, var0);
        self.func33(env, var0.wrapping_add(16));
        let mut slot_var0_0_i64 = self.memory.load64(var0 as usize) as i64;
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        let mut slot_var0_16_i64 = self.memory.load64(var0 as usize + 16) as i64;
        let mut slot_var0_24_i64 = self.memory.load64(var0 as usize + 24) as i64;
        let var5 = self.func24(env, slot_var0_0_i64, slot_var0_8_i64, slot_var0_16_i64, slot_var0_24_i64);
        var1 = var5;
        self.global0 = var0.wrapping_add(32);
        var1
    }
    fn func55(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        'label0: loop {
            'label1: loop {
                if (arg1 == 0) as i32 != 0 {
                    break 'label1;
                }
                arg0 = (arg0 as u64 / arg1 as u64) as i64;
                arg1 = arg0.wrapping_add(arg1);
                if (arg1 as u64 >= arg0 as u64) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            self.func15(env);
            unreachable!();
            break;
        }
        (arg1 as u64).wrapping_shr(1 /* True */ as u32) as i64
    }
    fn func56(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
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
    fn func57(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
        let mut var4: i64 = 0;
        'label0: loop {
            'label1: loop {
                if arg3 & 64 != 0 {
                    break 'label1;
                }
                if (arg3 == 0) as i32 != 0 {
                    break 'label0;
                }
                var4 = (arg3 & 63) as u32 as i64;
                arg1 = arg2.wrapping_shl((0.wrapping_sub(arg3) & 63) as u32 as i64 as u32) | (arg1 as u64).wrapping_shr(var4 as u32) as i64;
                arg2 = (arg2 as u64).wrapping_shr(var4 as u32) as i64;
                break 'label0;
                break;
            }
            arg1 = (arg2 as u64).wrapping_shr((arg3 & 63) as u32 as i64 as u32) as i64;
            arg2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
        self.memory.store64(arg0 as usize + 8, arg2 as u64);
    }
    fn func58(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
        let mut var4: i64 = 0;
        'label0: loop {
            'label1: loop {
                if arg3 & 64 != 0 {
                    break 'label1;
                }
                if (arg3 == 0) as i32 != 0 {
                    break 'label0;
                }
                var4 = (arg3 & 63) as u32 as i64;
                arg2 = arg2.wrapping_shl(var4 as u32) | (arg1 as u64).wrapping_shr((0.wrapping_sub(arg3) & 63) as u32 as i64 as u32) as i64;
                arg1 = arg1.wrapping_shl(var4 as u32);
                break 'label0;
                break;
            }
            arg2 = arg1.wrapping_shl((arg3 & 63) as u32 as i64 as u32);
            arg1 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
        self.memory.store64(arg0 as usize + 8, arg2 as u64);
    }
    fn func59(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let var14 = self.global0;
        var5 = var14.wrapping_sub(176);
        self.global0 = var5;
        var6 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            var7 = ({ let a = arg4.leading_zeros() as i64; let b = (arg3.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */); if (arg4 != 0 /* False */) as i32 != 0 { a } else { b } }) as i32;
                            var8 = ({ let a = arg2.leading_zeros() as i64; let b = (arg1.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */); if (arg2 != 0 /* False */) as i32 != 0 { a } else { b } }) as i32;
                            if (var7 as u32 <= var8 as u32) as i32 != 0 {
                                break 'label4;
                            }
                            if (var8 as u32 > 63 as u32) as i32 != 0 {
                                break 'label3;
                            }
                            if (var7 as u32 > 95 as u32) as i32 != 0 {
                                break 'label2;
                            }
                            if ((var7.wrapping_sub(var8) as u32) < 32 as u32) as i32 != 0 {
                                break 'label1;
                            }
                            var9 = 96.wrapping_sub(var7);
                            self.func57(env, var5.wrapping_add(160), arg3, arg4, var9);
                            let var16 = self.memory.load32(var5 as usize + 160) as i64;
                            var10 = var16.wrapping_add(1 /* True */);
                            var11 = 0 /* False */;
                            var6 = 0 /* False */;
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                var8 = 64.wrapping_sub(var8);
                                                self.func57(env, var5.wrapping_add(144), arg1, arg2, var8);
                                                let mut slot_var5_144_i64 = self.memory.load64(var5 as usize + 144) as i64;
                                                var12 = slot_var5_144_i64;
                                                'label10: loop {
                                                    if (var8 as u32 >= var9 as u32) as i32 != 0 {
                                                        break 'label10;
                                                    }
                                                    self.func57(env, var5.wrapping_add(80), arg3, arg4, var8);
                                                    'label11: loop {
                                                        'label12: loop {
                                                            let mut slot_var5_80_i64 = self.memory.load64(var5 as usize + 80) as i64;
                                                            var10 = slot_var5_80_i64;
                                                            if ((var10 == 0) as i32 == 0) as i32 != 0 {
                                                                break 'label12;
                                                            }
                                                            break 'label11;
                                                            break;
                                                        }
                                                        var12 = (var12 as u64 / var10 as u64) as i64;
                                                        break;
                                                    }
                                                    self.func56(env, var5.wrapping_add(64), arg3, arg4, var12, 0 /* False */);
                                                    'label13: loop {
                                                        let mut slot_var5_64_i64 = self.memory.load64(var5 as usize + 64) as i64;
                                                        var13 = slot_var5_64_i64;
                                                        var8 = ((arg1 as u64) < var13 as u64) as i32;
                                                        let mut slot_var5_72_i64 = self.memory.load64(var5 as usize + 72) as i64;
                                                        var10 = slot_var5_72_i64;
                                                        if ({ let a = var8; let b = ((arg2 as u64) < var10 as u64) as i32; if (arg2 == var10) as i32 != 0 { a } else { b } }) != 0 {
                                                            break 'label13;
                                                        }
                                                        arg2 = arg2.wrapping_sub(var10).wrapping_sub(var8 as u32 as i64);
                                                        arg1 = arg1.wrapping_sub(var13);
                                                        var12 = var11.wrapping_add(var12);
                                                        var6 = var6.wrapping_add(((var12 as u64) < var11 as u64) as i32 as u32 as i64);
                                                        break 'label0;
                                                        break;
                                                    }
                                                    arg4 = arg1.wrapping_add(arg3);
                                                    arg2 = arg2.wrapping_add(arg4).wrapping_add(((arg4 as u64) < arg1 as u64) as i32 as u32 as i64).wrapping_sub(var10).wrapping_sub(((arg4 as u64) < var13 as u64) as i32 as u32 as i64);
                                                    arg1 = arg4.wrapping_sub(var13);
                                                    var12 = var12.wrapping_add(var11).wrapping_add(18446744073709551615);
                                                    var6 = var6.wrapping_add(((var12 as u64) < var11 as u64) as i32 as u32 as i64);
                                                    break 'label0;
                                                    break;
                                                }
                                                var12 = (var12 as u64 / var10 as u64) as i64;
                                                var8 = var8.wrapping_sub(var9);
                                                self.func58(env, var5.wrapping_add(128), var12, 0 /* False */, var8);
                                                self.func56(env, var5.wrapping_add(112), arg3, arg4, var12, 0 /* False */);
                                                let mut slot_var5_112_i64 = self.memory.load64(var5 as usize + 112) as i64;
                                                let mut slot_var5_120_i64 = self.memory.load64(var5 as usize + 120) as i64;
                                                self.func58(env, var5.wrapping_add(96), slot_var5_112_i64, slot_var5_120_i64, var8);
                                                let mut slot_var5_136_i64 = self.memory.load64(var5 as usize + 136) as i64;
                                                let mut slot_var5_128_i64 = self.memory.load64(var5 as usize + 128) as i64;
                                                var6 = slot_var5_128_i64;
                                                var11 = var6.wrapping_add(var11);
                                                var6 = slot_var5_136_i64.wrapping_add(var6).wrapping_add(((var11 as u64) < var6 as u64) as i32 as u32 as i64);
                                                let mut slot_var5_104_i64 = self.memory.load64(var5 as usize + 104) as i64;
                                                let mut slot_var5_96_i64 = self.memory.load64(var5 as usize + 96) as i64;
                                                var12 = slot_var5_96_i64;
                                                arg2 = arg2.wrapping_sub(slot_var5_104_i64).wrapping_sub(((arg1 as u64) < var12 as u64) as i32 as u32 as i64);
                                                arg1 = arg1.wrapping_sub(var12);
                                                var8 = ({ let a = arg2.leading_zeros() as i64; let b = (arg1.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */); if (arg2 != 0 /* False */) as i32 != 0 { a } else { b } }) as i32;
                                                if (var7 as u32 <= var8 as u32) as i32 != 0 {
                                                    break 'label8;
                                                }
                                                if (var8 as u32 <= 63 as u32) as i32 != 0 {
                                                    continue 'label9;
                                                }
                                                break;
                                            }
                                            if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                                                break 'label7;
                                            }
                                            break 'label6;
                                            break;
                                        }
                                        var8 = ((arg1 as u64) < arg3 as u64) as i32;
                                        if (({ let a = var8; let b = ((arg2 as u64) < arg4 as u64) as i32; if (arg2 == arg4) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                                            break 'label5;
                                        }
                                        var12 = var11;
                                        break 'label0;
                                        break;
                                    }
                                    arg2 = (arg1 as u64 / arg3 as u64) as i64;
                                    break;
                                }
                                arg1 = (arg1 as u64).wrapping_rem(arg3 as u64) as i64;
                                var12 = var11.wrapping_add(arg2);
                                var6 = var6.wrapping_add(((var12 as u64) < var11 as u64) as i32 as u32 as i64);
                                arg2 = 0 /* False */;
                                break 'label0;
                                break;
                            }
                            arg2 = arg2.wrapping_sub(arg4).wrapping_sub(var8 as u32 as i64);
                            arg1 = arg1.wrapping_sub(arg3);
                            var12 = var11.wrapping_add(1 /* True */);
                            var6 = var6.wrapping_add((var12 == 0) as i32 as u32 as i64);
                            break 'label0;
                            break;
                        }
                        var8 = { let a = (arg1 as u64 >= arg3 as u64) as i32; let b = (arg2 as u64 >= arg4 as u64) as i32; if (arg2 == arg4) as i32 != 0 { a } else { b } };
                        arg4 = { let a = arg3; let b = 0 /* False */; if var8 != 0 { a } else { b } };
                        arg2 = arg2.wrapping_sub({ let a = arg4; let b = 0 /* False */; if var8 != 0 { a } else { b } }).wrapping_sub(((arg1 as u64) < arg4 as u64) as i32 as u32 as i64);
                        arg1 = arg1.wrapping_sub(arg4);
                        var12 = var8 as u32 as i64;
                        break 'label0;
                        break;
                    }
                    var12 = (arg1 as u64 / arg3 as u64) as i64;
                    arg1 = arg1.wrapping_sub(var12.wrapping_mul(arg3));
                    var6 = 0 /* False */;
                    arg2 = 0 /* False */;
                    break 'label0;
                    break;
                }
                arg4 = arg3 & 4294967295;
                var6 = (arg2 as u64 / arg4 as u64) as i64;
                var12 = (arg1 as u64).wrapping_shr(32 as u32) as i64;
                arg2 = ((arg2.wrapping_sub(var6.wrapping_mul(arg3)).wrapping_shl(32 as u32) | var12) as u64 / arg4 as u64) as i64;
                arg1 = var12.wrapping_sub(arg2.wrapping_mul(arg3)).wrapping_shl(32 as u32) | arg1 & 4294967295;
                arg3 = (arg1 as u64 / arg4 as u64) as i64;
                var12 = arg2.wrapping_shl(32 as u32) | arg3;
                arg1 = arg1.wrapping_sub(arg3.wrapping_mul(arg4));
                var6 = (arg2 as u64).wrapping_shr(32 as u32) as i64 | var6;
                arg2 = 0 /* False */;
                break 'label0;
                break;
            }
            var8 = 64.wrapping_sub(var8);
            self.func57(env, var5.wrapping_add(48), arg3, arg4, var8);
            self.func57(env, var5.wrapping_add(32), arg1, arg2, var8);
            var6 = 0 /* False */;
            let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
            let mut slot_var5_48_i64 = self.memory.load64(var5 as usize + 48) as i64;
            var12 = (slot_var5_32_i64 as u64 / slot_var5_48_i64 as u64) as i64;
            self.func56(env, var5.wrapping_add(16), arg3, 0 /* False */, var12, 0 /* False */);
            self.func56(env, var5, arg4, 0 /* False */, var12, 0 /* False */);
            let mut slot_var5_16_i64 = self.memory.load64(var5 as usize + 16) as i64;
            var10 = slot_var5_16_i64;
            'label14: loop {
                'label15: loop {
                    let mut slot_var5_8_i64 = self.memory.load64(var5 as usize + 8) as i64;
                    let mut slot_var5_24_i64 = self.memory.load64(var5 as usize + 24) as i64;
                    var13 = slot_var5_24_i64;
                    let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
                    var11 = var13.wrapping_add(slot_var5_0_i64);
                    if (slot_var5_8_i64.wrapping_add(((var11 as u64) < var13 as u64) as i32 as u32 as i64) != 0 /* False */) as i32 != 0 {
                        break 'label15;
                    }
                    var8 = ((arg1 as u64) < var10 as u64) as i32;
                    if (({ let a = var8; let b = ((arg2 as u64) < var11 as u64) as i32; if (arg2 == var11) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        break 'label14;
                    }
                    break;
                }
                arg1 = arg3.wrapping_add(arg1);
                arg2 = arg4.wrapping_add(arg2).wrapping_add(((arg1 as u64) < arg3 as u64) as i32 as u32 as i64).wrapping_sub(var11).wrapping_sub(((arg1 as u64) < var10 as u64) as i32 as u32 as i64);
                var12 = var12.wrapping_add(18446744073709551615);
                arg1 = arg1.wrapping_sub(var10);
                break 'label0;
                break;
            }
            arg2 = arg2.wrapping_sub(var11).wrapping_sub(var8 as u32 as i64);
            arg1 = arg1.wrapping_sub(var10);
            var6 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize + 16, arg1 as u64);
        self.memory.store64(arg0 as usize, var12 as u64);
        self.memory.store64(arg0 as usize + 24, arg2 as u64);
        self.memory.store64(arg0 as usize + 8, var6 as u64);
        self.global0 = var5.wrapping_add(176);
    }
    fn func60(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(32);
        self.global0 = var5;
        var6 = (arg2 < 0 /* False */) as i32;
        var6 = (arg4 < 0 /* False */) as i32;
        self.func59(env, var5, { let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg2; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg4.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg4; if var6 != 0 { a } else { b } });
        let mut slot_var5_8_i64 = self.memory.load64(var5 as usize + 8) as i64;
        arg3 = slot_var5_8_i64;
        let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
        arg1 = slot_var5_0_i64;
        var6 = (arg4 ^ arg2 < 0 /* False */) as i32;
        self.memory.store64(arg0 as usize, ({ let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var6 != 0 { a } else { b } }) as u64);
        self.memory.store64(arg0 as usize + 8, ({ let a = 0 /* False */.wrapping_sub(arg3.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg3; if var6 != 0 { a } else { b } }) as u64);
        self.global0 = var5.wrapping_add(32);
    }
    fn func61(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let var11 = self.global0;
        var6 = var11.wrapping_sub(96);
        self.global0 = var6;
        var7 = 0 /* False */;
        var8 = 0 /* False */;
        var9 = 0;
        'label0: loop {
            if (arg1 | arg2 == 0) as i32 != 0 {
                break 'label0;
            }
            if (arg3 | arg4 == 0) as i32 != 0 {
                break 'label0;
            }
            var9 = (arg4 < 0 /* False */) as i32;
            var7 = { let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var9 != 0 { a } else { b } };
            var10 = (arg2 < 0 /* False */) as i32;
            var8 = { let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var10 != 0 { a } else { b } };
            arg3 = { let a = 0 /* False */.wrapping_sub(arg4.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg4; if var9 != 0 { a } else { b } };
            arg4 = arg4 ^ arg2;
            'label1: loop {
                'label2: loop {
                    arg2 = { let a = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg2; if var10 != 0 { a } else { b } };
                    if (arg2 == 0) as i32 != 0 {
                        break 'label2;
                    }
                    'label3: loop {
                        if (arg3 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        self.func56(env, var6.wrapping_add(80), var7, arg3, var8, arg2);
                        var9 = 1;
                        let mut slot_var6_88_i64 = self.memory.load64(var6 as usize + 88) as i64;
                        arg1 = slot_var6_88_i64;
                        let mut slot_var6_80_i64 = self.memory.load64(var6 as usize + 80) as i64;
                        arg2 = slot_var6_80_i64;
                        break 'label1;
                        break;
                    }
                    self.func56(env, var6.wrapping_add(64), var7, arg3, var8, 0 /* False */);
                    self.func56(env, var6.wrapping_add(48), var7, arg3, arg2, 0 /* False */);
                    let mut slot_var6_72_i64 = self.memory.load64(var6 as usize + 72) as i64;
                    arg2 = slot_var6_72_i64;
                    let mut slot_var6_48_i64 = self.memory.load64(var6 as usize + 48) as i64;
                    arg1 = arg2.wrapping_add(slot_var6_48_i64);
                    let mut slot_var6_56_i64 = self.memory.load64(var6 as usize + 56) as i64;
                    var9 = ((arg1 as u64) < arg2 as u64) as i32 | (slot_var6_56_i64 != 0 /* False */) as i32;
                    let mut slot_var6_64_i64 = self.memory.load64(var6 as usize + 64) as i64;
                    arg2 = slot_var6_64_i64;
                    break 'label1;
                    break;
                }
                'label4: loop {
                    if (arg3 == 0) as i32 != 0 {
                        break 'label4;
                    }
                    self.func56(env, var6.wrapping_add(32), var7, 0 /* False */, var8, arg2);
                    self.func56(env, var6.wrapping_add(16), arg3, 0 /* False */, var8, arg2);
                    let mut slot_var6_40_i64 = self.memory.load64(var6 as usize + 40) as i64;
                    arg2 = slot_var6_40_i64;
                    let mut slot_var6_16_i64 = self.memory.load64(var6 as usize + 16) as i64;
                    arg1 = arg2.wrapping_add(slot_var6_16_i64);
                    let mut slot_var6_24_i64 = self.memory.load64(var6 as usize + 24) as i64;
                    var9 = ((arg1 as u64) < arg2 as u64) as i32 | (slot_var6_24_i64 != 0 /* False */) as i32;
                    let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
                    arg2 = slot_var6_32_i64;
                    break 'label1;
                    break;
                }
                self.func56(env, var6, var7, arg3, var8, arg2);
                var9 = 0;
                let mut slot_var6_8_i64 = self.memory.load64(var6 as usize + 8) as i64;
                arg1 = slot_var6_8_i64;
                let mut slot_var6_0_i64 = self.memory.load64(var6 as usize) as i64;
                arg2 = slot_var6_0_i64;
                break;
            }
            var10 = (arg4 < 0 /* False */) as i32;
            var8 = { let a = 0 /* False */.wrapping_sub(arg2); let b = arg2; if var10 != 0 { a } else { b } };
            var7 = { let a = 0 /* False */.wrapping_sub(arg1.wrapping_add((arg2 != 0 /* False */) as i32 as u32 as i64)); let b = arg1; if var10 != 0 { a } else { b } };
            if (var7 ^ arg4 >= 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            var9 = 1;
            break;
        }
        self.memory.store64(arg0 as usize, var8 as u64);
        self.memory.store32(arg5 as usize, var9 as u32);
        self.memory.store64(arg0 as usize + 8, var7 as u64);
        self.global0 = var6.wrapping_add(96);
    }
}
