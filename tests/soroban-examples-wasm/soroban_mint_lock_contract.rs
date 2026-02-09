#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct MintLockContract;

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
pub enum Error { NotAuthorizedMinter = 1, DailyLimitInsufficient = 2, NegativeAmount = 3, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StorageKey { Admin, Minter(soroban_sdk::Address, soroban_sdk::Address), MinterStats(soroban_sdk::Address, soroban_sdk::Address, u32, u32), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinterConfig { pub epoch_length: u32, pub limit: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinterStats { pub consumed_limit: i128, }

#[contractimpl]
impl MintLockContract {
    pub fn ___constructor(&mut self, env: Env, admin: soroban_sdk::Address) {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(admin)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func20(env, admin);
        0 /* Void */
    }
    pub fn set_admin(&mut self, env: Env, new_admin: soroban_sdk::Address) {
    }
    pub fn admin(&mut self, env: Env) -> soroban_sdk::Address {
        let var0 = self.func32(env);
        var0
    }
    pub fn set_minter(&mut self, env: Env, contract: soroban_sdk::Address, minter: soroban_sdk::Address, config: MinterConfig) {
        contract.require_auth_for_args((minter, config).into_val(&env));
    }
    pub fn minter(&mut self, mut env: Env, mut contract: soroban_sdk::Address, minter: soroban_sdk::Address) -> Result<(MinterConfig, u32, MinterStats,), soroban_sdk::Error> {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var2 = var8.wrapping_sub(80);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(minter)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var2_64_i64 = minter as i64;
                let mut slot_var2_56_i64 = contract as i64;
                let mut slot_var2_48_i32 = 1 as i32;
                self.func16(env, var2, var2.wrapping_add(48));
                'label2: loop {
                    'label3: loop {
                        let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                        if (slot_var2_0_i32 & 1 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                        var3 = slot_var2_24_i64;
                        let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                        var4 = slot_var2_16_i64;
                        let mut slot_var2_32_i32 = self.memory.load32(var2 as usize + 32) as i32;
                        var5 = slot_var2_32_i32;
                        let var10 = self.func36(env);
                        var6 = var10;
                        if (var5 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var2_72_i64 = minter as i64;
                        slot_var2_64_i64 = contract as i64;
                        let mut slot_var2_52_i32 = var5 as i32;
                        slot_var2_48_i32 = 2 as i32;
                        var6 = (var6 as u32 / var5 as u32) as i32;
                        let mut slot_var2_56_i32 = var6 as i32;
                        self.func21(env, var2, var2.wrapping_add(48));
                        contract = slot_var2_24_i64;
                        minter = slot_var2_16_i64;
                        var7 = slot_var2_0_i32;
                        self.func26(env, var2.wrapping_add(48), var4, var3, var5);
                        if slot_var2_48_i32 != 0 {
                            break 'label1;
                        }
                        var3 = slot_var2_56_i64;
                        var5 = var7 as i32 & 1;
                        self.func29(env, var2.wrapping_add(48), { let a = minter; let b = 0 /* False */; if var5 != 0 { a } else { b } }, { let a = contract; let b = 0 /* False */; if var5 != 0 { a } else { b } });
                        if slot_var2_48_i32 != 0 {
                            break 'label1;
                        }
                        slot_var2_16_i64 = slot_var2_56_i64 as i64;
                        let mut slot_var2_0_i64 = var3 as i64;
                        let mut slot_var2_8_i64 = ((var6 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
                        let var14 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        contract = var14;
                        break 'label2;
                        break;
                    }
                    contract = 4294967299 /* Error(Contract, #1) */;
                    break;
                }
                self.global0 = var2.wrapping_add(80);
                return contract;
                break;
            }
            unreachable!();
            break;
        }
        self.func37(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn mint(&mut self, env: Env, contract: soroban_sdk::Address, minter: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        contract.require_auth_for_args((minter, to, amount).into_val(&env));
    }
}

#[allow(dead_code)]
impl MintLockContract {
    fn func16(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(48);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var5 = self.func17(env, arg1);
                    var3 = var5;
                    let var6 = self.func18(env, var3, 1 /* True */);
                    if var6 != 0 {
                        break 'label2;
                    }
                    var3 = 0 /* False */;
                    break 'label1;
                    break;
                }
                let var7 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                self.func19(env, var2, var7);
                let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                if slot_var2_0_i32 & 1 != 0 {
                    break 'label0;
                }
                let mut slot_var2_32_i32 = self.memory.load32(var2 as usize + 32) as i32;
                arg1 = slot_var2_32_i32;
                let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                var3 = slot_var2_16_i64;
                let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                self.memory.store64(arg0 as usize + 24, slot_var2_24_i64 as u64);
                self.memory.store64(arg0 as usize + 16, var3 as u64);
                self.memory.store32(arg0 as usize + 32, arg1 as u32);
                var3 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var3 as u64);
            self.memory.store64(arg0 as usize + 8, 0 /* False */ as u64);
            self.global0 = var2.wrapping_add(48);
            return;
            break;
        }
        unreachable!();
    }
    fn func17(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(48);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var7 = self.memory.load32(arg0 as usize) as i32;
                            match var7 {
                                0 => break 'label2,
                                1 => break 'label4,
                                2 => break 'label3,
                                _ => break 'label2,
                            }
                            break;
                        }
                        self.func24(env, var1.wrapping_add(8), 1048581, 6);
                        let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                        if slot_var1_8_i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                        var2 = slot_var1_16_i64;
                        let var9 = self.memory.load64(arg0 as usize + 8) as i64;
                        var3 = var9;
                        let var10 = self.memory.load64(arg0 as usize + 16) as i64;
                        let mut slot_var1_24_i64 = var10 as i64;
                        slot_var1_16_i64 = var3 as i64;
                        let mut slot_var1_8_i64 = var2 as i64;
                        let var11 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        var2 = var11;
                        break 'label1;
                        break;
                    }
                    self.func24(env, var1.wrapping_add(8), 1048587, 11);
                    if slot_var1_8_i32 != 0 {
                        break 'label0;
                    }
                    var2 = slot_var1_16_i64;
                    let var13 = self.memory.load32(arg0 as usize + 4) as i64;
                    var3 = var13;
                    let var14 = self.memory.load32(arg0 as usize + 8) as i64;
                    var4 = var14;
                    let var15 = self.memory.load64(arg0 as usize + 16) as i64;
                    var5 = var15;
                    let var16 = self.memory.load64(arg0 as usize + 24) as i64;
                    slot_var1_24_i64 = var16 as i64;
                    slot_var1_16_i64 = var5 as i64;
                    slot_var1_8_i64 = var2 as i64;
                    let mut slot_var1_40_i64 = (var4.wrapping_shl(32 as u32) | 0) as i64;
                    let mut slot_var1_32_i64 = (var3.wrapping_shl(32 as u32) | 0) as i64;
                    let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    var2 = var17;
                    break 'label1;
                    break;
                }
                self.func24(env, var1.wrapping_add(8), 1048576, 5);
                if slot_var1_8_i32 != 0 {
                    break 'label0;
                }
                slot_var1_8_i64 = slot_var1_16_i64 as i64;
                let var19 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var2 = var19;
                break;
            }
            self.global0 = var1.wrapping_add(48);
            return var2;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func18(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var2 == 1 /* True */) as i32
    }
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(48);
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
        var4 = 0 /* False */;
        var5 = 1 /* True */;
        'label2: loop {
            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            self.func22(env, arg1, 1048616, 2, var2, 2);
            let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
            arg1 = slot_var2_0_i64;
            if (arg1 & 255 != 0) as i32 != 0 {
                break 'label2;
            }
            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
            self.func23(env, var2.wrapping_add(16), slot_var2_8_i64);
            let mut slot_var2_16_i32 = self.memory.load32(var2 as usize + 16) as i32;
            if (slot_var2_16_i32 == 1) as i32 != 0 {
                break 'label2;
            }
            let mut slot_var2_32_i64 = self.memory.load64(var2 as usize + 32) as i64;
            var4 = slot_var2_32_i64;
            let mut slot_var2_40_i64 = self.memory.load64(var2 as usize + 40) as i64;
            self.memory.store64(arg0 as usize + 24, slot_var2_40_i64 as u64);
            self.memory.store64(arg0 as usize + 16, var4 as u64);
            self.memory.store32(arg0 as usize + 32, (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32);
            var5 = 0 /* False */;
            var4 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var5 as u64);
        self.memory.store64(arg0 as usize + 8, var4 as u64);
        self.global0 = var2.wrapping_add(48);
    }
    fn func20(&mut self, env: &Env, mut arg0: i64) {
        let var1 = self.func17(env, 1048656);
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64(arg0)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64(arg0)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64(arg0)); 0 } }
        var2;
    }
    fn func21(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(48);
        self.global0 = var2;
        var3 = 0 /* False */;
        var4 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var7 = self.func17(env, arg1);
                var5 = var7;
                let var8 = self.func18(env, var5, 0 /* False */);
                if (var8 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var9 = match 0 /* False */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) } }
                var3 = var9;
                let mut slot_var2_8_i64 = 0 /* Void */ as i64;
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.func22(env, var3, 1048648, 1, var2.wrapping_add(8), 1);
                self.func23(env, var2.wrapping_add(16), slot_var2_8_i64);
                let mut slot_var2_16_i32 = self.memory.load32(var2 as usize + 16) as i32;
                if (slot_var2_16_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_32_i64 = self.memory.load64(var2 as usize + 32) as i64;
                var3 = slot_var2_32_i64;
                let mut slot_var2_40_i64 = self.memory.load64(var2 as usize + 40) as i64;
                self.memory.store64(arg0 as usize + 24, slot_var2_40_i64 as u64);
                self.memory.store64(arg0 as usize + 16, var3 as u64);
                var4 = 0 /* False */;
                var3 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var3 as u64);
            self.memory.store64(arg0 as usize + 8, var4 as u64);
            self.global0 = var2.wrapping_add(48);
            return;
            break;
        }
        unreachable!();
    }
    fn func22(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
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
    fn func23(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func24(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(16);
        self.global0 = var4;
        self.func27(env, var4, arg1, arg2);
        arg2 = 1 /* True */;
        'label0: loop {
            let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
            if slot_var4_0_i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
            slot_var4_8_i64 = slot_var4_8_i64 as i64;
            let mut slot_var4_0_i64 = ((arg3 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
            let var7 = self.func28(env, 1048616, 2, var4, 2);
            self.memory.store64(arg0 as usize + 8, var7 as u64);
            arg2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, arg2 as u64);
        self.global0 = var4.wrapping_add(16);
    }
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
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
    fn func29(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        self.func27(env, var3, arg1, arg2);
        arg2 = 1 /* True */;
        'label0: loop {
            let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
            if slot_var3_0_i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
            let mut slot_var3_0_i64 = slot_var3_8_i64 as i64;
            let var6 = self.func28(env, 1048648, 1, var3, 1);
            self.memory.store64(arg0 as usize + 8, var6 as u64);
            arg2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, arg2 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func30(&mut self, env: &Env, mut admin: i64) -> i64 {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(admin)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func20(env, admin);
        0 /* Void */
    }
    fn func31(&mut self, env: &Env, mut new_admin: i64) -> i64 {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(new_admin)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var1 = self.func32(env);
        let var2 = { address_from_i64(env, var1).require_auth(); 0 }
        var2;
        self.func20(env, new_admin);
        0 /* Void */
    }
    fn func32(&mut self, env: &Env) -> i64 {
        let mut var0: i64 = 0;
        'label0: loop {
            'label1: loop {
                let var1 = self.func17(env, 1048656);
                var0 = var1;
                let var2 = self.func18(env, var0, 0 /* Void */);
                if (var2 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var3 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var0)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var0)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var0)).unwrap_or(val_from_i64(0))) } }
                var0 = var3;
                if (Address::try_from_val(env, &val_from_i64(var0)).is_ok()) as i32 != 0 {
                    break 'label0;
                }
                unreachable!();
                break;
            }
            self.func40(env);
            unreachable!();
            break;
        }
        var0
    }
    fn func33(&mut self, env: &Env) -> i64 {
        let var0 = self.func32(env);
        var0
    }
    fn func34(&mut self, env: &Env, mut contract: i64, mut minter: i64, mut config: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_sub(64);
        self.global0 = var3;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(minter)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func19(env, var3, config);
            let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
            if slot_var3_0_i32 & 1 != 0 {
                break 'label0;
            }
            let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
            config = slot_var3_24_i64;
            let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
            var4 = slot_var3_16_i64;
            let mut slot_var3_32_i32 = self.memory.load32(var3 as usize + 32) as i32;
            var5 = slot_var3_32_i32;
            let var8 = self.func32(env);
            let var9 = { address_from_i64(env, var8).require_auth(); 0 }
            var9;
            slot_var3_16_i64 = minter as i64;
            let mut slot_var3_8_i64 = contract as i64;
            slot_var3_0_i32 = 1 as i32;
            let var10 = self.func17(env, var3);
            contract = var10;
            self.func26(env, var3.wrapping_add(48), var4, config, var5);
            let mut slot_var3_48_i32 = self.memory.load32(var3 as usize + 48) as i32;
            if (slot_var3_48_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_56_i64 = self.memory.load64(var3 as usize + 56) as i64;
            let var12 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(contract), &val_from_i64(slot_var3_56_i64)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(contract), &val_from_i64(slot_var3_56_i64)); 0 }, _ => { env.storage().instance().set(&val_from_i64(contract), &val_from_i64(slot_var3_56_i64)); 0 } }
            var12;
            self.global0 = var3.wrapping_add(64);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func35(&mut self, env: &Env, mut contract: i64, mut minter: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var2 = var8.wrapping_sub(80);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(minter)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var2_64_i64 = minter as i64;
                let mut slot_var2_56_i64 = contract as i64;
                let mut slot_var2_48_i32 = 1 as i32;
                self.func16(env, var2, var2.wrapping_add(48));
                'label2: loop {
                    'label3: loop {
                        let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                        if (slot_var2_0_i32 & 1 == 0) as i32 != 0 {
                            break 'label3;
                        }
                        let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                        var3 = slot_var2_24_i64;
                        let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                        var4 = slot_var2_16_i64;
                        let mut slot_var2_32_i32 = self.memory.load32(var2 as usize + 32) as i32;
                        var5 = slot_var2_32_i32;
                        let var10 = self.func36(env);
                        var6 = var10;
                        if (var5 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var2_72_i64 = minter as i64;
                        slot_var2_64_i64 = contract as i64;
                        let mut slot_var2_52_i32 = var5 as i32;
                        slot_var2_48_i32 = 2 as i32;
                        var6 = (var6 as u32 / var5 as u32) as i32;
                        let mut slot_var2_56_i32 = var6 as i32;
                        self.func21(env, var2, var2.wrapping_add(48));
                        contract = slot_var2_24_i64;
                        minter = slot_var2_16_i64;
                        var7 = slot_var2_0_i32;
                        self.func26(env, var2.wrapping_add(48), var4, var3, var5);
                        if slot_var2_48_i32 != 0 {
                            break 'label1;
                        }
                        var3 = slot_var2_56_i64;
                        var5 = var7 as i32 & 1;
                        self.func29(env, var2.wrapping_add(48), { let a = minter; let b = 0 /* False */; if var5 != 0 { a } else { b } }, { let a = contract; let b = 0 /* False */; if var5 != 0 { a } else { b } });
                        if slot_var2_48_i32 != 0 {
                            break 'label1;
                        }
                        slot_var2_16_i64 = slot_var2_56_i64 as i64;
                        let mut slot_var2_0_i64 = var3 as i64;
                        let mut slot_var2_8_i64 = ((var6 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
                        let var14 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        contract = var14;
                        break 'label2;
                        break;
                    }
                    contract = 4294967299 /* Error(Contract, #1) */;
                    break;
                }
                self.global0 = var2.wrapping_add(80);
                return contract;
                break;
            }
            unreachable!();
            break;
        }
        self.func37(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func36(&mut self, env: &Env) -> i32 {
        let var0 = env.ledger().sequence() as i64
        (var0 as u64).wrapping_shr(32 as u32) as i64 as i32
    }
    fn func37(&mut self, env: &Env) {
        self.func41(env);
        unreachable!();
    }
    fn func38(&mut self, env: &Env, mut contract: i64, mut minter: i64, mut to: i64, mut amount: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var4 = var13.wrapping_sub(80);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        if (!(Address::try_from_val(env, &val_from_i64(minter)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                            break 'label3;
                        }
                        self.func23(env, var4, amount);
                        let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
                        if (slot_var4_0_i32 == 1) as i32 != 0 {
                            break 'label3;
                        }
                        let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
                        var5 = slot_var4_16_i64;
                        let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
                        amount = slot_var4_24_i64;
                        let var15 = self.func39(env, var5, amount);
                        let mut slot_var4_64_i64 = var15 as i64;
                        let mut slot_var4_56_i64 = to as i64;
                        let mut slot_var4_48_i64 = contract as i64;
                        var6 = 0;
                        'label4: loop {
                            'label5: loop {
                                if (var6 != 24) as i32 != 0 {
                                    break 'label5;
                                }
                                var6 = 0;
                                'label6: loop {
                                    'label7: loop {
                                        if (var6 == 24) as i32 != 0 {
                                            break 'label6;
                                        }
                                        let var16 = self.memory.load64(var4.wrapping_add(48).wrapping_add(var6) as usize) as i64;
                                        self.memory.store64(var4.wrapping_add(var6) as usize, var16 as u64);
                                        var6 = var6.wrapping_add(8);
                                        continue 'label7;
                                        break;
                                    }
                                    break;
                                }
                                let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                let var18 = { address_from_i64(env, minter).require_auth_for_args(val_from_i64(var17)); 0 }
                                var18;
                                'label8: loop {
                                    if (amount >= 0 /* False */) as i32 != 0 {
                                        break 'label8;
                                    }
                                    minter = 12884901891 /* Error(Contract, #3) */;
                                    break 'label1;
                                    break;
                                }
                                let var19 = self.func32(env);
                                let var20 = { let a = val_from_i64(var19); let b = val_from_i64(minter); if a < b { -1 } else if a > b { 1 } else { 0 } }
                                if (var20 == 0) as i32 != 0 {
                                    break 'label2;
                                }
                                slot_var4_64_i64 = minter as i64;
                                slot_var4_56_i64 = contract as i64;
                                let mut slot_var4_48_i32 = 1 as i32;
                                self.func16(env, var4, var4.wrapping_add(48));
                                'label9: loop {
                                    if slot_var4_0_i32 & 1 != 0 {
                                        break 'label9;
                                    }
                                    minter = 4294967299 /* Error(Contract, #1) */;
                                    break 'label1;
                                    break;
                                }
                                var7 = slot_var4_24_i64;
                                var8 = slot_var4_16_i64;
                                let mut slot_var4_32_i32 = self.memory.load32(var4 as usize + 32) as i32;
                                var6 = slot_var4_32_i32;
                                let var22 = self.func36(env);
                                var9 = var22;
                                if (var6 == 0) as i32 != 0 {
                                    break 'label0;
                                }
                                let mut slot_var4_72_i64 = minter as i64;
                                slot_var4_64_i64 = contract as i64;
                                slot_var4_48_i32 = 2 as i32;
                                let mut slot_var4_52_i32 = var6 as i32;
                                var10 = (var9 as u32 / var6 as u32) as i32;
                                let mut slot_var4_56_i32 = var10 as i32;
                                self.func21(env, var4, var4.wrapping_add(48));
                                var9 = slot_var4_0_i32 & 1;
                                var11 = { let a = slot_var4_24_i64; let b = 0 /* False */; if var9 != 0 { a } else { b } };
                                minter = { let a = slot_var4_16_i64; let b = 0 /* False */; if var9 != 0 { a } else { b } };
                                var12 = minter.wrapping_add(var5);
                                minter = var11.wrapping_add(amount).wrapping_add(((var12 as u64) < minter as u64) as i32 as u32 as i64);
                                if ((var11 ^ amount ^ 18446744073709551615) & (var11 ^ minter) < 0 /* False */) as i32 != 0 {
                                    break 'label0;
                                }
                                'label10: loop {
                                    if ({ let a = (var12 as u64 > var8 as u64) as i32; let b = (minter > var7) as i32; if (minter == var7) as i32 != 0 { a } else { b } }) != 0 {
                                        break 'label10;
                                    }
                                    let var24 = self.func17(env, var4.wrapping_add(48));
                                    var11 = var24;
                                    self.func29(env, var4, var12, minter);
                                    if (slot_var4_0_i32 == 1) as i32 != 0 {
                                        break 'label3;
                                    }
                                    let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
                                    let var26 = match 0 /* False */ { 0 => { env.storage().persistent().set(&val_from_i64(var11), &val_from_i64(slot_var4_8_i64)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var11), &val_from_i64(slot_var4_8_i64)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var11), &val_from_i64(slot_var4_8_i64)); 0 } }
                                    var26;
                                    var9 = var10.wrapping_add(1);
                                    if (var9 == 0) as i32 != 0 {
                                        break 'label0;
                                    }
                                    minter = (var6 as u32 as i64).wrapping_mul(var9 as u32 as i64);
                                    if (minter as u64).wrapping_shr(32 as u32) as i64 as i32 != 0 {
                                        break 'label0;
                                    }
                                    var6 = minter as i32;
                                    if (var6 == 0) as i32 != 0 {
                                        break 'label0;
                                    }
                                    let var27 = self.func17(env, var4.wrapping_add(48));
                                    minter = (var6.wrapping_add(-1) as u32 as i64).wrapping_shl(32 as u32) | 0;
                                    let var28 = match 0 /* False */ { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(var27), minter as u32, minter as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(var27), minter as u32, minter as u32); 0 }, _ => { env.storage().instance().extend_ttl(minter as u32, minter as u32); 0 } }
                                    var28;
                                    break 'label2;
                                    break;
                                }
                                minter = 8589934595 /* Error(Contract, #2) */;
                                break 'label1;
                                break;
                            }
                            self.memory.store64(var4.wrapping_add(var6) as usize, 0 /* Void */ as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    unreachable!();
                    break;
                }
                let var29 = self.func39(env, var5, amount);
                slot_var4_56_i64 = var29 as i64;
                slot_var4_48_i64 = to as i64;
                var6 = 0;
                'label11: loop {
                    'label12: loop {
                        if (var6 != 16) as i32 != 0 {
                            break 'label12;
                        }
                        var6 = 0;
                        'label13: loop {
                            'label14: loop {
                                if (var6 == 16) as i32 != 0 {
                                    break 'label13;
                                }
                                let var30 = self.memory.load64(var4.wrapping_add(48).wrapping_add(var6) as usize) as i64;
                                self.memory.store64(var4.wrapping_add(var6) as usize, var30 as u64);
                                var6 = var6.wrapping_add(8);
                                continue 'label14;
                                break;
                            }
                            break;
                        }
                        minter = 0 /* Void */;
                        let var31 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        let var32 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(contract)), &Symbol::from_val(env, &val_from_i64(mint)), Vec::<Val>::from_val(env, &val_from_i64(var31))))
                        if (var32 & 255 == 0 /* Void */) as i32 != 0 {
                            break 'label1;
                        }
                        break 'label0;
                        break;
                    }
                    self.memory.store64(var4.wrapping_add(var6) as usize, 0 /* Void */ as u64);
                    var6 = var6.wrapping_add(8);
                    continue 'label11;
                    break;
                }
                break;
            }
            self.global0 = var4.wrapping_add(80);
            return minter;
            break;
        }
        self.func37(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func39(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func27(env, var2, arg0, arg1);
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
    fn func40(&mut self, env: &Env) {
        self.func37(env);
        unreachable!();
    }
    fn func41(&mut self, env: &Env) {
        unreachable!();
    }
}
