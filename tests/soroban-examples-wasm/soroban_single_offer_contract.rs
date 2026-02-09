#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct SingleOfferContract;

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
pub enum DataKey { Offer, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Offer { pub buy_price: u32, pub buy_token: soroban_sdk::Address, pub sell_price: u32, pub sell_token: soroban_sdk::Address, pub seller: soroban_sdk::Address, }

#[contractimpl]
impl SingleOfferContract {
    pub fn create(&mut self, env: Env, seller: soroban_sdk::Address, sell_token: soroban_sdk::Address, buy_token: soroban_sdk::Address, sell_price: u32, buy_price: u32) {
        seller.require_auth_for_args((sell_token, buy_token, sell_price, buy_price).into_val(&env));
    }
    pub fn trade(&mut self, env: Env, buyer: soroban_sdk::Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        buyer.require_auth_for_args((buy_token_amount, min_sell_token_amount).into_val(&env));
    }
    pub fn withdraw(&mut self, env: Env, token: soroban_sdk::Address, amount: i128) {
        token.require_auth_for_args((amount).into_val(&env));
    }
    pub fn updt_price(&mut self, env: Env, sell_price: u32, buy_price: u32) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (sell_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (buy_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if ((sell_price as u64) < 4294967296 as u64) as i32 != 0 {
                    break 'label0;
                }
                if (buy_price as u64 <= 4294967295 as u64) as i32 != 0 {
                    break 'label0;
                }
                self.func18(env, var2);
                let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
                let var5 = { address_from_i64(env, slot_var2_0_i64).require_auth(); 0 }
                var5;
                self.memory.store32(var2 as usize + 28, (buy_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.memory.store32(var2 as usize + 24, (sell_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func21(env, var2);
                self.global0 = var2.wrapping_add(32);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func23(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn get_offer(&mut self, env: Env) -> Offer {
        env.storage().instance().get(&DataKey::Offer).unwrap()
    }
}

#[allow(dead_code)]
impl SingleOfferContract {
    fn func13(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(48);
        self.global0 = var5;
        'label0: loop {
            'label1: loop {
                if (arg3.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                    break 'label1;
                }
                if (arg3 ^ arg3 | arg4 ^ arg3.wrapping_shr(63 as u32) != 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                arg3 = arg3.wrapping_shl(0 as u32) | 0;
                break 'label0;
                break;
            }
            let var8 = val_to_i64(Val::from_i128(((arg4 as i128) << 64) | (arg3 as u64 as i128)))
            arg3 = var8;
            break;
        }
        let mut slot_var5_16_i64 = arg3 as i64;
        let mut slot_var5_8_i64 = arg2 as i64;
        let mut slot_var5_0_i64 = arg1 as i64;
        var6 = 0;
        'label2: loop {
            'label3: loop {
                'label4: loop {
                    if (var6 != 24) as i32 != 0 {
                        break 'label4;
                    }
                    var6 = 0;
                    'label5: loop {
                        'label6: loop {
                            if (var6 == 24) as i32 != 0 {
                                break 'label5;
                            }
                            let var9 = self.memory.load64(var5.wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, var9 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label6;
                            break;
                        }
                        break;
                    }
                    let var10 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    let var11 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(transfer)), Vec::<Val>::from_val(env, &val_from_i64(var10))))
                    if (var11 & 255 != 0 /* Void */) as i32 != 0 {
                        break 'label2;
                    }
                    self.global0 = var5.wrapping_add(48);
                    return;
                    break;
                }
                self.memory.store64(var5.wrapping_add(24).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label3;
                break;
            }
            break;
        }
        self.func15(env);
        unreachable!();
    }
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func15(&mut self, env: &Env) {
        self.func23(env);
        unreachable!();
    }
    fn func16(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var0 = var5.wrapping_sub(16);
        self.global0 = var0;
        var1 = 0 /* False */;
        var2 = -5;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var2 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var3 = 1;
                    'label3: loop {
                        'label4: loop {
                            let var6 = self.memory.load8(var2.wrapping_add(1048581) as usize) as i32;
                            var4 = var6;
                            if (var4 == 95) as i32 != 0 {
                                break 'label4;
                            }
                            'label5: loop {
                                if (((var4.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                    break 'label5;
                                }
                                'label6: loop {
                                    if (((var4.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if ((var4.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    var3 = var4.wrapping_add(-59);
                                    break 'label4;
                                    break;
                                }
                                var3 = var4.wrapping_add(-53);
                                break 'label4;
                                break;
                            }
                            var3 = var4.wrapping_add(-46);
                            break;
                        }
                        var1 = var1.wrapping_shl(0 as u32) | var3 as u32 as i64 & 255;
                        var2 = var2.wrapping_add(1);
                        continue 'label2;
                        break;
                    }
                    break;
                }
                let mut slot_var0_0_i64 = ((var4 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                let var7 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                var1 = var7;
                break 'label0;
                break;
            }
            var1 = var1.wrapping_shl(0 as u32) | 0 /* Symbol() */;
            let mut slot_var0_4_i64 = var1 as i64;
            break;
        }
        slot_var0_0_i64 = var1 as i64;
        let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        var1 = var8;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func17(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(48);
        self.global0 = var1;
        let var4 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var1_40_i64 = var4 as i64;
        let var5 = self.memory.load64(arg0 as usize + 8) as i64;
        let mut slot_var1_32_i64 = var5 as i64;
        let var6 = self.memory.load64(arg0 as usize + 16) as i64;
        let mut slot_var1_16_i64 = var6 as i64;
        let var7 = self.memory.load32(arg0 as usize + 24) as i64;
        let mut slot_var1_24_i64 = (var7.wrapping_shl(32 as u32) | 0) as i64;
        let var8 = self.memory.load32(arg0 as usize + 28) as i64;
        let mut slot_var1_8_i64 = (var8.wrapping_shl(32 as u32) | 0) as i64;
        let var9 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2 = var9;
        self.global0 = var1.wrapping_add(48);
        var2
    }
    fn func18(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var1 = var8.wrapping_sub(48);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                let var9 = self.func16(env);
                var2 = var9;
                let var10 = self.func19(env, var2);
                if (var10 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var11 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) } }
                var2 = var11;
                var3 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var3 == 40) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(8).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                        var3 = var3.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                'label4: loop {
                    if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                        break 'label4;
                    }
                    let var12 = 0 /* TODO: map_unpack_to_linear_memory */
                    var12;
                    let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                    var2 = slot_var1_8_i64;
                    if (var2 & 255 != 0) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                    var4 = slot_var1_16_i64;
                    if (!(Address::try_from_val(env, &val_from_i64(var4)).is_ok())) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                    var5 = slot_var1_24_i64;
                    if (var5 & 255 != 0) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var1_32_i64 = self.memory.load64(var1 as usize + 32) as i64;
                    var6 = slot_var1_32_i64;
                    if (!(Address::try_from_val(env, &val_from_i64(var6)).is_ok())) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var1_40_i64 = self.memory.load64(var1 as usize + 40) as i64;
                    var7 = slot_var1_40_i64;
                    if (Address::try_from_val(env, &val_from_i64(var7)).is_ok()) as i32 != 0 {
                        break 'label0;
                    }
                    break;
                }
                unreachable!();
                break;
            }
            self.func20(env);
            unreachable!();
            break;
        }
        self.memory.store64(arg0 as usize + 16, var4 as u64);
        self.memory.store64(arg0 as usize + 8, var6 as u64);
        self.memory.store64(arg0 as usize, var7 as u64);
        self.memory.store32(arg0 as usize + 28, (var2 as u64).wrapping_shr(32 as u32) as i64 as u32);
        self.memory.store32(arg0 as usize + 24, (var5 as u64).wrapping_shr(32 as u32) as i64 as u32);
        self.global0 = var1.wrapping_add(48);
    }
    fn func19(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func20(&mut self, env: &Env) {
        self.func15(env);
        unreachable!();
    }
    fn func21(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func16(env);
        let var2 = self.func17(env, arg0);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64(var2)); 0 } }
        var3;
    }
    fn func22(&mut self, env: &Env, mut seller: i64, mut sell_token: i64, mut buy_token: i64, mut sell_price: i64, mut buy_price: i64) -> i64 {
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var5 = var6.wrapping_sub(32);
        self.global0 = var5;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(seller)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(sell_token)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(Address::try_from_val(env, &val_from_i64(buy_token)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (sell_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (buy_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                let var7 = self.func16(env);
                let var8 = self.func19(env, var7);
                if var8 != 0 {
                    break 'label0;
                }
                if ((sell_price as u64) < 4294967296 as u64) as i32 != 0 {
                    break 'label0;
                }
                if ((buy_price as u64) < 4294967296 as u64) as i32 != 0 {
                    break 'label0;
                }
                let var9 = { address_from_i64(env, seller).require_auth(); 0 }
                var9;
                self.memory.store32(var5 as usize + 28, (buy_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.memory.store32(var5 as usize + 24, (sell_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                let mut slot_var5_16_i64 = buy_token as i64;
                let mut slot_var5_8_i64 = sell_token as i64;
                let mut slot_var5_0_i64 = seller as i64;
                self.func21(env, var5);
                self.global0 = var5.wrapping_add(32);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func23(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func23(&mut self, env: &Env) {
        unreachable!();
    }
    fn func24(&mut self, env: &Env, mut buyer: i64, mut buy_token_amount: i64, mut min_sell_token_amount: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let var10 = self.global0;
        var3 = var10.wrapping_sub(80);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (!(Address::try_from_val(env, &val_from_i64(buyer)).is_ok())) as i32 != 0 {
                        break 'label2;
                    }
                    self.func25(env, var3.wrapping_add(48), buy_token_amount);
                    let mut slot_var3_48_i32 = self.memory.load32(var3 as usize + 48) as i32;
                    if (slot_var3_48_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var3_72_i64 = self.memory.load64(var3 as usize + 72) as i64;
                    buy_token_amount = slot_var3_72_i64;
                    let mut slot_var3_64_i64 = self.memory.load64(var3 as usize + 64) as i64;
                    var4 = slot_var3_64_i64;
                    self.func25(env, var3.wrapping_add(48), min_sell_token_amount);
                    if (slot_var3_48_i32 == 1) as i32 != 0 {
                        break 'label2;
                    }
                    min_sell_token_amount = slot_var3_72_i64;
                    var5 = slot_var3_64_i64;
                    let var13 = { address_from_i64(env, buyer).require_auth(); 0 }
                    var13;
                    self.func18(env, var3.wrapping_add(48));
                    let mut slot_var3_44_i32 = 0 as i32;
                    let var15 = self.memory.load32(var3 as usize + 72) as i64;
                    self.func34(env, var3.wrapping_add(16), var4, buy_token_amount, var15, 0 /* False */, var3.wrapping_add(44));
                    if slot_var3_44_i32 != 0 {
                        break 'label2;
                    }
                    let mut slot_var3_76_i32 = self.memory.load32(var3 as usize + 76) as i32;
                    var6 = slot_var3_76_i32;
                    if (var6 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var7 = slot_var3_64_i64;
                    let mut slot_var3_56_i64 = self.memory.load64(var3 as usize + 56) as i64;
                    var8 = slot_var3_56_i64;
                    let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
                    let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
                    self.func33(env, var3, slot_var3_16_i64, slot_var3_24_i64, var6 as u32 as i64, 0 /* False */);
                    let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                    var9 = slot_var3_0_i64;
                    let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                    var5 = slot_var3_8_i64;
                    if (({ let a = (var9 as u64 >= var5 as u64) as i32; let b = (var5 >= min_sell_token_amount) as i32; if (var5 == min_sell_token_amount) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        break 'label0;
                    }
                    let var18 = val_to_i64(env.current_contract_address().into_val(env))
                    min_sell_token_amount = var18;
                    self.func13(env, var7, buyer, min_sell_token_amount, var4, buy_token_amount);
                    self.func13(env, var8, min_sell_token_amount, buyer, var9, var5);
                    self.func13(env, var7, min_sell_token_amount, slot_var3_48_i32, var4, buy_token_amount);
                    self.global0 = var3.wrapping_add(80);
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
        self.func23(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
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
    fn func26(&mut self, env: &Env, mut token: i64, mut amount: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(token)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func25(env, var2, amount);
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if (slot_var2_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
            amount = slot_var2_24_i64;
            let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
            var3 = slot_var2_16_i64;
            self.func18(env, var2);
            var4 = slot_var2_0_i32;
            let var8 = { address_from_i64(env, var4).require_auth(); 0 }
            var8;
            let var9 = val_to_i64(env.current_contract_address().into_val(env))
            self.func13(env, token, var9, var4, var3, amount);
            self.global0 = var2.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func27(&mut self, env: &Env, mut sell_price: i64, mut buy_price: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (sell_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (buy_price & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if ((sell_price as u64) < 4294967296 as u64) as i32 != 0 {
                    break 'label0;
                }
                if (buy_price as u64 <= 4294967295 as u64) as i32 != 0 {
                    break 'label0;
                }
                self.func18(env, var2);
                let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
                let var5 = { address_from_i64(env, slot_var2_0_i64).require_auth(); 0 }
                var5;
                self.memory.store32(var2 as usize + 28, (buy_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.memory.store32(var2 as usize + 24, (sell_price as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func21(env, var2);
                self.global0 = var2.wrapping_add(32);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func23(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func28(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        self.func18(env, var0);
        let var4 = self.func17(env, var0);
        var1 = var4;
        self.global0 = var0.wrapping_add(32);
        var1
    }
    fn func29(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
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
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
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
    fn func31(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
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
    fn func32(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
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
                            self.func29(env, var5.wrapping_add(160), arg3, arg4, var9);
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
                                                self.func29(env, var5.wrapping_add(144), arg1, arg2, var8);
                                                let mut slot_var5_144_i64 = self.memory.load64(var5 as usize + 144) as i64;
                                                var12 = slot_var5_144_i64;
                                                'label10: loop {
                                                    if (var8 as u32 >= var9 as u32) as i32 != 0 {
                                                        break 'label10;
                                                    }
                                                    self.func29(env, var5.wrapping_add(80), arg3, arg4, var8);
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
                                                    self.func30(env, var5.wrapping_add(64), arg3, arg4, var12, 0 /* False */);
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
                                                self.func31(env, var5.wrapping_add(128), var12, 0 /* False */, var8);
                                                self.func30(env, var5.wrapping_add(112), arg3, arg4, var12, 0 /* False */);
                                                let mut slot_var5_112_i64 = self.memory.load64(var5 as usize + 112) as i64;
                                                let mut slot_var5_120_i64 = self.memory.load64(var5 as usize + 120) as i64;
                                                self.func31(env, var5.wrapping_add(96), slot_var5_112_i64, slot_var5_120_i64, var8);
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
            self.func29(env, var5.wrapping_add(48), arg3, arg4, var8);
            self.func29(env, var5.wrapping_add(32), arg1, arg2, var8);
            var6 = 0 /* False */;
            let mut slot_var5_32_i64 = self.memory.load64(var5 as usize + 32) as i64;
            let mut slot_var5_48_i64 = self.memory.load64(var5 as usize + 48) as i64;
            var12 = (slot_var5_32_i64 as u64 / slot_var5_48_i64 as u64) as i64;
            self.func30(env, var5.wrapping_add(16), arg3, 0 /* False */, var12, 0 /* False */);
            self.func30(env, var5, arg4, 0 /* False */, var12, 0 /* False */);
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
    fn func33(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(32);
        self.global0 = var5;
        var6 = (arg2 < 0 /* False */) as i32;
        var6 = (arg4 < 0 /* False */) as i32;
        self.func32(env, var5, { let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg2; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var6 != 0 { a } else { b } }, { let a = 0 /* False */.wrapping_sub(arg4.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg4; if var6 != 0 { a } else { b } });
        let mut slot_var5_8_i64 = self.memory.load64(var5 as usize + 8) as i64;
        arg3 = slot_var5_8_i64;
        let mut slot_var5_0_i64 = self.memory.load64(var5 as usize) as i64;
        arg1 = slot_var5_0_i64;
        var6 = (arg4 ^ arg2 < 0 /* False */) as i32;
        self.memory.store64(arg0 as usize, ({ let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var6 != 0 { a } else { b } }) as u64);
        self.memory.store64(arg0 as usize + 8, ({ let a = 0 /* False */.wrapping_sub(arg3.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg3; if var6 != 0 { a } else { b } }) as u64);
        self.global0 = var5.wrapping_add(32);
    }
    fn func34(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i32) {
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
                        self.func30(env, var6.wrapping_add(80), var7, arg3, var8, arg2);
                        var9 = 1;
                        let mut slot_var6_88_i64 = self.memory.load64(var6 as usize + 88) as i64;
                        arg1 = slot_var6_88_i64;
                        let mut slot_var6_80_i64 = self.memory.load64(var6 as usize + 80) as i64;
                        arg2 = slot_var6_80_i64;
                        break 'label1;
                        break;
                    }
                    self.func30(env, var6.wrapping_add(64), var7, arg3, var8, 0 /* False */);
                    self.func30(env, var6.wrapping_add(48), var7, arg3, arg2, 0 /* False */);
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
                    self.func30(env, var6.wrapping_add(32), var7, 0 /* False */, var8, arg2);
                    self.func30(env, var6.wrapping_add(16), arg3, 0 /* False */, var8, arg2);
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
                self.func30(env, var6, var7, arg3, var8, arg2);
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
