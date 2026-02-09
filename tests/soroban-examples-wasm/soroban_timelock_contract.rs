#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, Val, FromVal, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct TimelockContract;

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
pub enum DataKey { Init, Balance, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TimeBoundKind { Before, After, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeBound { pub kind: TimeBoundKind, pub timestamp: u64, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClaimableBalance { pub amount: i128, pub claimants: soroban_sdk::Vec<soroban_sdk::Address>, pub time_bound: TimeBound, pub token: soroban_sdk::Address, }

#[contractimpl]
impl TimelockContract {
    pub fn deposit(&mut self, env: Env, from: soroban_sdk::Address, token: soroban_sdk::Address, amount: i128, claimants: soroban_sdk::Vec<soroban_sdk::Address>, time_bound: TimeBound) {
        from.require_auth_for_args((token, amount, claimants, time_bound).into_val(&env));
    }
    pub fn claim(&mut self, env: Env, claimant: soroban_sdk::Address) {
    }
}

#[allow(dead_code)]
impl TimelockContract {
    fn func21(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(48);
        self.global0 = var5;
        self.func22(env, var5.wrapping_add(24), arg3, arg4);
        'label0: loop {
            let mut slot_var5_24_i32 = self.memory.load32(var5 as usize + 24) as i32;
            if (slot_var5_24_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
            let mut slot_var5_16_i64 = slot_var5_32_i64 as i64;
            let mut slot_var5_8_i64 = arg2 as i64;
            let mut slot_var5_0_i64 = arg1 as i64;
            var6 = 0;
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (var6 != 24) as i32 != 0 {
                            break 'label3;
                        }
                        var6 = 0;
                        'label4: loop {
                            'label5: loop {
                                if (var6 == 24) as i32 != 0 {
                                    break 'label4;
                                }
                                let var9 = self.memory.load64(var5.wrapping_add(var6) as usize) as i64;
                                self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, var9 as u64);
                                var6 = var6.wrapping_add(8);
                                continue 'label5;
                                break;
                            }
                            break;
                        }
                        let var10 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        let var11 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(transfer)), Vec::<Val>::from_val(env, &val_from_i64(var10))))
                        if (var11 & 255 != 0 /* Void */) as i32 != 0 {
                            break 'label1;
                        }
                        self.global0 = var5.wrapping_add(48);
                        return;
                        break;
                    }
                    self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                    var6 = var6.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func24(env);
            unreachable!();
            break;
        }
        unreachable!();
    }
    fn func22(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func23(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func24(&mut self, env: &Env) {
        self.func34(env);
        unreachable!();
    }
    fn func25(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (arg0 & 1 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        self.func26(env, var1, 1048580, 7);
                        let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                        if slot_var1_0_i32 != 0 {
                            break 'label1;
                        }
                        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                        self.func27(env, var1, slot_var1_8_i64);
                        break 'label2;
                        break;
                    }
                    self.func26(env, var1, 1048576, 4);
                    if slot_var1_0_i32 != 0 {
                        break 'label1;
                    }
                    self.func27(env, var1, slot_var1_8_i64);
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
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(16);
        self.global0 = var2;
        var3 = 0;
        'label0: loop {
            'label1: loop {
                if (var3 == 16) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var2.wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        var3 = 2;
        'label2: loop {
            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            var3 = 2;
            self.func29(env, arg1, 1048632, 2, var2, 2);
            let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
            arg1 = slot_var2_0_i64;
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            let var9 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
            var4 = var9;
            if ((var4 as u64) < 4294967296 as u64) as i32 != 0 {
                break 'label2;
            }
            'label3: loop {
                let var10 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1)).get_unchecked(0 as u32))
                arg1 = var10;
                var5 = arg1 as i32 & 255;
                if (var5 == 74) as i32 != 0 {
                    break 'label3;
                }
                if (var5 != 14) as i32 != 0 {
                    break 'label2;
                }
                break;
            }
            var5 = (var4 as u64).wrapping_shr(32 as u32) as i64 as i32;
            'label4: loop {
                'label5: loop {
                    'label6: loop {
                        let var11 = 0 /* TODO: symbol_index_in_linear_memory */
                        match (var11 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                            0 => break 'label6,
                            1 => break 'label5,
                            _ => break 'label2,
                        }
                        break;
                    }
                    let var12 = self.func30(env, 1, var5);
                    if var12 != 0 {
                        break 'label2;
                    }
                    var6 = 0;
                    break 'label4;
                    break;
                }
                var6 = 1;
                let var13 = self.func30(env, 1, var5);
                if var13 != 0 {
                    break 'label2;
                }
                break;
            }
            'label7: loop {
                'label8: loop {
                    let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                    arg1 = slot_var2_8_i64;
                    var5 = arg1 as i32 & 255;
                    if (var5 == 64) as i32 != 0 {
                        break 'label8;
                    }
                    if (var5 != 6) as i32 != 0 {
                        break 'label2;
                    }
                    arg1 = (arg1 as u64).wrapping_shr(0 as u32) as i64;
                    break 'label7;
                    break;
                }
                let var14 = val_from_i64(arg1).as_u64().unwrap_or(0) as i64
                arg1 = var14;
                break;
            }
            self.memory.store64(arg0 as usize, arg1 as u64);
            var3 = var6;
            break;
        }
        self.memory.store8(arg0 as usize + 8, var3 as u8);
        self.global0 = var2.wrapping_add(16);
    }
    fn func29(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
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
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        'label0: loop {
            if ((arg1 as u32) < arg0 as u32) as i32 != 0 {
                break 'label0;
            }
            return arg1.wrapping_sub(arg0);
            break;
        }
        self.func24(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func31(&mut self, env: &Env, mut from: i64, mut token: i64, mut amount: i64, mut claimants: i64, mut time_bound: i64) -> i64 {
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let mut var7: i32 = 0;
        let var8 = self.global0;
        var5 = var8.wrapping_sub(64);
        self.global0 = var5;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(token)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func32(env, var5, amount);
            let mut slot_var5_0_i32 = self.memory.load32(var5 as usize) as i32;
            if (slot_var5_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(claimants)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var5_24_i64 = self.memory.load64(var5 as usize + 24) as i64;
            amount = slot_var5_24_i64;
            let mut slot_var5_16_i64 = self.memory.load64(var5 as usize + 16) as i64;
            var6 = slot_var5_16_i64;
            self.func28(env, var5, time_bound);
            let var11 = self.memory.load8(var5 as usize + 8) as i32;
            var7 = var11;
            if (var7 == 2) as i32 != 0 {
                break 'label0;
            }
            time_bound = slot_var5_0_i32;
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        let var12 = Vec::<Val>::from_val(env, &val_from_i64(claimants)).len() as i64
                        if (var12 as u64 > 47244640255 as u64) as i32 != 0 {
                            break 'label3;
                        }
                        let var13 = self.func25(env, 0);
                        let var14 = self.func33(env, var13);
                        if var14 != 0 {
                            break 'label3;
                        }
                        let var15 = { address_from_i64(env, from).require_auth(); 0 }
                        var15;
                        let var16 = val_to_i64(env.current_contract_address().into_val(env))
                        self.func21(env, token, from, var16, var6, amount);
                        let var18 = self.func25(env, 1);
                        from = var18;
                        self.func22(env, var5.wrapping_add(48), var6, amount);
                        let mut slot_var5_48_i32 = self.memory.load32(var5 as usize + 48) as i32;
                        if slot_var5_48_i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var5_56_i64 = self.memory.load64(var5 as usize + 56) as i64;
                        amount = slot_var5_56_i64;
                        'label4: loop {
                            'label5: loop {
                                if (var7 & 1 == 0) as i32 != 0 {
                                    break 'label5;
                                }
                                self.func26(env, var5.wrapping_add(48), 1048593, 5);
                                if slot_var5_48_i32 != 0 {
                                    break 'label0;
                                }
                                self.func27(env, var5.wrapping_add(48), slot_var5_56_i64);
                                break 'label4;
                                break;
                            }
                            self.func26(env, var5.wrapping_add(48), 1048587, 6);
                            if slot_var5_48_i32 != 0 {
                                break 'label0;
                            }
                            self.func27(env, var5.wrapping_add(48), slot_var5_56_i64);
                            break;
                        }
                        var6 = slot_var5_56_i64;
                        if slot_var5_48_i32 as i32 != 0 {
                            break 'label0;
                        }
                        if (time_bound as u64 > 72057594037927935 as u64) as i32 != 0 {
                            break 'label2;
                        }
                        time_bound = time_bound.wrapping_shl(0 as u32) | 0;
                        break 'label1;
                        break;
                    }
                    self.func34(env);
                    unreachable!();
                    break;
                }
                let var25 = val_to_i64(Val::from_u64(time_bound as u64))
                time_bound = var25;
                break;
            }
            let mut slot_var5_40_i64 = time_bound as i64;
            let mut slot_var5_32_i64 = var6 as i64;
            let var26 = self.func35(env, 1048632, 2, var5.wrapping_add(32), 2);
            var6 = var26;
            slot_var5_24_i64 = token as i64;
            slot_var5_16_i64 = var6 as i64;
            let mut slot_var5_8_i64 = claimants as i64;
            let mut slot_var5_0_i64 = amount as i64;
            let var27 = self.func35(env, 1048680, 4, var5, 4);
            self.func36(env, from, var27);
            let var29 = self.func25(env, 0);
            self.func36(env, var29, 0 /* Void */);
            self.global0 = var5.wrapping_add(64);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func32(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func33(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func34(&mut self, env: &Env) {
        unreachable!();
    }
    fn func35(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
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
    fn func36(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn func37(&mut self, env: &Env, mut claimant: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let var10 = self.global0;
        var1 = var10.wrapping_sub(64);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Address::try_from_val(env, &val_from_i64(claimant)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let var11 = { address_from_i64(env, claimant).require_auth(); 0 }
                    var11;
                    let var12 = self.func25(env, 1);
                    var2 = var12;
                    let var13 = self.func33(env, var2);
                    if (var13 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    let var14 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) } }
                    var2 = var14;
                    var3 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var3 == 32) as i32 != 0 {
                                break 'label3;
                            }
                            self.memory.store64(var1.wrapping_add(var3) as usize, 0 /* Void */ as u64);
                            var3 = var3.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    self.func29(env, var2, 1048680, 4, var1, 4);
                    let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                    self.func32(env, var1.wrapping_add(32), slot_var1_0_i64);
                    let mut slot_var1_32_i32 = self.memory.load32(var1 as usize + 32) as i32;
                    if (slot_var1_32_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                    var4 = slot_var1_8_i64;
                    if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var4)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
                    var5 = slot_var1_56_i64;
                    let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                    var6 = slot_var1_48_i64;
                    let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                    self.func28(env, var1.wrapping_add(32), slot_var1_16_i64);
                    let var18 = self.memory.load8(var1 as usize + 40) as i32;
                    var3 = var18;
                    if (var3 == 2) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                    var7 = slot_var1_24_i64;
                    if (!(Address::try_from_val(env, &val_from_i64(var7)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    var2 = slot_var1_32_i32;
                    'label5: loop {
                        'label6: loop {
                            let var19 = env.ledger().timestamp() as i64
                            var8 = var19;
                            var9 = var8 as i32 & 255;
                            if (var9 == 6) as i32 != 0 {
                                break 'label6;
                            }
                            'label7: loop {
                                if (var9 != 64) as i32 != 0 {
                                    break 'label7;
                                }
                                let var20 = val_from_i64(var8).as_u64().unwrap_or(0) as i64
                                var8 = var20;
                                break 'label5;
                                break;
                            }
                            self.func24(env);
                            unreachable!();
                            break;
                        }
                        var8 = (var8 as u64).wrapping_shr(0 as u32) as i64;
                        break;
                    }
                    if (({ let a = (var8 as u64 >= var2 as u64) as i32; let b = (var8 as u64 <= var2 as u64) as i32; if var3 & 1 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        break 'label0;
                    }
                    let var22 = { let v = Vec::<Val>::from_val(env, &val_from_i64(var4)); match v.first_index_of(val_from_i64(claimant)) { Some(i) => i as i64, None => 0 /* Void */ } }
                    if (var22 == 0 /* Void */) as i32 != 0 {
                        break 'label0;
                    }
                    let var23 = val_to_i64(env.current_contract_address().into_val(env))
                    self.func21(env, var7, var23, claimant, var6, var5);
                    let var25 = self.func25(env, 1);
                    let var26 = match 0 /* Void */ { 0 => { env.storage().persistent().remove(&val_from_i64(var25)); 0 }, 1 => { env.storage().temporary().remove(&val_from_i64(var25)); 0 }, _ => { env.storage().instance().remove(&val_from_i64(var25)); 0 } }
                    var26;
                    self.global0 = var1.wrapping_add(64);
                    return 0 /* Void */;
                    break;
                }
                unreachable!();
                break;
            }
            self.func38(env);
            unreachable!();
            break;
        }
        self.func34(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func38(&mut self, env: &Env) {
        self.func24(env);
        unreachable!();
    }
}
