#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct AtomicSwapContract;

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
impl AtomicSwapContract {
    pub fn swap(&mut self, env: Env, a: soroban_sdk::Address, b: soroban_sdk::Address, token_a: soroban_sdk::Address, token_b: soroban_sdk::Address, amount_a: i128, min_b_for_a: i128, amount_b: i128, min_a_for_b: i128) {
        a.require_auth_for_args((token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env));
        b.require_auth_for_args((token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env));
        self.move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        self.move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
    fn move_token(&self, env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: i128, transfer_amount: i128) {
        let token = token::Client::new(env, token);
        let contract_address = env.current_contract_address();
        token.transfer(from, &contract_address, &max_spend_amount);
        token.transfer(&contract_address, to, &transfer_amount);
        token.transfer(&contract_address, from, &(max_spend_amount - transfer_amount));
    }
}

#[allow(dead_code)]
impl AtomicSwapContract {
    fn func7(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(48);
        self.global0 = var5;
        let var8 = self.func8(env, arg3, arg4);
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
        self.func10(env);
        unreachable!();
    }
    fn func8(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
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
    fn func9(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func10(&mut self, env: &Env) {
        self.func15(env);
        unreachable!();
    }
    fn func11(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(64);
        self.global0 = var1;
        let var6 = self.memory.load64(arg0 as usize + 16) as i64;
        var2 = var6;
        let var7 = self.memory.load64(arg0 as usize + 24) as i64;
        var3 = var7;
        let var8 = self.memory.load64(arg0 as usize) as i64;
        let var9 = self.memory.load64(arg0 as usize + 8) as i64;
        let var10 = self.func8(env, var8, var9);
        var4 = var10;
        let var11 = self.memory.load64(arg0 as usize + 32) as i64;
        let var12 = self.memory.load64(arg0 as usize + 40) as i64;
        let var13 = self.func8(env, var11, var12);
        let mut slot_var1_24_i64 = var13 as i64;
        let mut slot_var1_16_i64 = var4 as i64;
        let mut slot_var1_8_i64 = var3 as i64;
        let mut slot_var1_0_i64 = var2 as i64;
        arg0 = 0;
        let var14: i64;
        'label0: loop {
            'label1: loop {
                if (arg0 != 32) as i32 != 0 {
                    break 'label1;
                }
                arg0 = 0;
                'label2: loop {
                    'label3: loop {
                        if (arg0 == 32) as i32 != 0 {
                            break 'label2;
                        }
                        let var15 = self.memory.load64(var1.wrapping_add(arg0) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(32).wrapping_add(arg0) as usize, var15 as u64);
                        arg0 = arg0.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var16 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var2 = var16;
                self.global0 = var1.wrapping_add(64);
                return var2;
                break;
            }
            self.memory.store64(var1.wrapping_add(32).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            continue 'label0;
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func12(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64) {
        let mut var7: i64 = 0;
        let var8 = val_to_i64(env.current_contract_address().into_val(env))
        var7 = var8;
        self.func7(env, arg0, arg1, var7, arg3, arg4);
        self.func7(env, arg0, var7, arg2, arg5, arg6);
        'label0: loop {
            let var11 = arg6;
            arg6 = arg4.wrapping_sub(arg6).wrapping_sub(((arg3 as u64) < arg5 as u64) as i32 as u32 as i64);
            if ((arg4 ^ var11) & (arg4 ^ arg6) < 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            self.func7(env, arg0, var7, arg1, arg3.wrapping_sub(arg5), arg6);
            return;
            break;
        }
        self.func10(env);
        unreachable!();
    }
    fn func13(&mut self, env: &Env, mut a: i64, mut b: i64, mut token_a: i64, mut token_b: i64, mut amount_a: i64, mut min_b_for_a: i64, mut amount_b: i64, mut min_a_for_b: i64) -> i64 {
        let mut var8: i32 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var8 = var13.wrapping_sub(48);
        self.global0 = var8;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(a)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(b)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(token_a)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(token_b)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                self.func14(env, var8, amount_a);
                let mut slot_var8_0_i32 = self.memory.load32(var8 as usize) as i32;
                if (slot_var8_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var8_24_i64 = self.memory.load64(var8 as usize + 24) as i64;
                var9 = slot_var8_24_i64;
                let mut slot_var8_16_i64 = self.memory.load64(var8 as usize + 16) as i64;
                var10 = slot_var8_16_i64;
                self.func14(env, var8, min_b_for_a);
                if (slot_var8_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                amount_a = slot_var8_24_i64;
                var11 = slot_var8_16_i64;
                self.func14(env, var8, amount_b);
                if (slot_var8_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                min_b_for_a = slot_var8_24_i64;
                amount_b = slot_var8_16_i64;
                self.func14(env, var8, min_a_for_b);
                if (slot_var8_0_i32 == 1) as i32 != 0 {
                    break 'label1;
                }
                if ({ let a = ((amount_b as u64) < var11 as u64) as i32; let b = (min_b_for_a < amount_a) as i32; if (min_b_for_a == amount_a) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                var12 = slot_var8_16_i64;
                min_a_for_b = slot_var8_24_i64;
                if ({ let a = ((var10 as u64) < var12 as u64) as i32; let b = (var9 < min_a_for_b) as i32; if (var9 == min_a_for_b) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                let mut slot_var8_32_i64 = var11 as i64;
                let mut slot_var8_0_i64 = var10 as i64;
                slot_var8_24_i64 = token_b as i64;
                slot_var8_16_i64 = token_a as i64;
                let mut slot_var8_40_i64 = amount_a as i64;
                let mut slot_var8_8_i64 = var9 as i64;
                let var18 = self.func11(env, var8);
                let var19 = { address_from_i64(env, a).require_auth_for_args(val_from_i64(var18)); 0 }
                var19;
                slot_var8_40_i64 = min_a_for_b as i64;
                slot_var8_32_i64 = var12 as i64;
                slot_var8_8_i64 = min_b_for_a as i64;
                slot_var8_0_i64 = amount_b as i64;
                slot_var8_24_i64 = token_a as i64;
                slot_var8_16_i64 = token_b as i64;
                let var20 = self.func11(env, var8);
                let var21 = { address_from_i64(env, b).require_auth_for_args(val_from_i64(var20)); 0 }
                var21;
                self.func12(env, token_a, a, b, var10, var9, var12, min_a_for_b);
                self.func12(env, token_b, b, a, amount_b, min_b_for_a, var11, amount_a);
                self.global0 = var8.wrapping_add(48);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func15(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func15(&mut self, env: &Env) {
        unreachable!();
    }
}
