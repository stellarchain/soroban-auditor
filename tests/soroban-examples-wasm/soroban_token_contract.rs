#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, MuxedAddress, Env, IntoVal, String, TryFromVal, Val, FromVal, Vec, Map, Bytes, BytesN, Symbol};

#[contract]
pub struct TokenContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contractevent(topics = ["set_admin",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SetAdmin { #[topic] pub admin: soroban_sdk::Address, pub new_admin: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceDataKey { pub from: soroban_sdk::Address, pub spender: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceValue { pub amount: i128, pub expiration_ledger: u32, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Allowance(AllowanceDataKey), Balance(soroban_sdk::Address), State(soroban_sdk::Address), Admin, }
#[soroban_sdk::contractevent(topics = ["approve",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Approve { #[topic] pub from: soroban_sdk::Address, #[topic] pub spender: soroban_sdk::Address, pub amount: i128, pub expiration_ledger: u32, }
#[soroban_sdk::contractevent(topics = ["transfer",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TransferWithAmountOnly { #[topic] pub from: soroban_sdk::Address, #[topic] pub to: soroban_sdk::Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["transfer",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Transfer { #[topic] pub from: soroban_sdk::Address, #[topic] pub to: soroban_sdk::Address, pub to_muxed_id: Option<u64>, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["burn",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Burn { #[topic] pub from: soroban_sdk::Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["mint",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MintWithAmountOnly { #[topic] pub to: soroban_sdk::Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["mint",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mint { #[topic] pub to: soroban_sdk::Address, pub to_muxed_id: Option<u64>, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["clawback",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clawback { #[topic] pub from: soroban_sdk::Address, pub amount: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokenMetadata { pub decimal: u32, pub name: soroban_sdk::String, pub symbol: soroban_sdk::String, }

#[contractimpl]
impl TokenContract {
    pub fn ___constructor(&mut self, env: Env, admin: soroban_sdk::Address, decimal: u32, name: soroban_sdk::String, symbol: soroban_sdk::String) {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(admin)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (decimal & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (!(String::try_from_val(env, &val_from_i64(name)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(String::try_from_val(env, &val_from_i64(symbol)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (decimal as u64 >= 81604378624 as u64) as i32 != 0 {
                    break 'label0;
                }
                self.func32(env, admin);
                let mut slot_var4_24_i64 = symbol as i64;
                let mut slot_var4_16_i64 = name as i64;
                let mut slot_var4_8_i64 = (decimal & 31) as i64;
                let var7 = self.func23(env);
                let var8 = self.func27(env, 1048772, 3, var4.wrapping_add(8), 3);
                let var9 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(var8)); 0 } }
                var9;
                self.global0 = var4.wrapping_add(32);
                return 0 /* Void */;
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
    pub fn mint(&mut self, env: Env, to: soroban_sdk::Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
    }
    pub fn set_admin(&mut self, env: Env, new_admin: soroban_sdk::Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    pub fn allowance(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address) -> i128 {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount
    }
    pub fn approve(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let value = AllowanceValue { amount: amount, expiration_ledger: expiration_ledger };
        if amount > 0 {
            env.storage().temporary().set(&key, &value);
        } else {
            env.storage().temporary().remove(&key);
        }
    }
    pub fn balance(&mut self, env: Env, id: soroban_sdk::Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }
    pub fn transfer(&mut self, env: Env, from: soroban_sdk::Address, to_muxed: soroban_sdk::MuxedAddress, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance(to_muxed)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to_muxed), &(to_balance + amount));
    }
    pub fn transfer_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        spender.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount -= amount;
        env.storage().temporary().set(&key, &allowance);
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }
    pub fn burn(&mut self, env: Env, from: soroban_sdk::Address, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(balance - amount));
    }
    pub fn burn_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, amount: i128) {
        spender.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let key = DataKey::Allowance(AllowanceDataKey { from: from, spender: spender });
        let mut allowance: AllowanceValue = env.storage().temporary().get(&key).unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        allowance.amount -= amount;
        env.storage().temporary().set(&key, &allowance);
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(balance - amount));
    }
    pub fn decimals(&mut self, env: Env) -> u32 {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.decimal
    }
    pub fn name(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.name
    }
    pub fn symbol(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.symbol
    }
}

#[allow(dead_code)]
impl TokenContract {
    fn func19(&mut self, env: &Env, mut arg0: i32) {
        self.func20(env, arg0, 1 /* True */, 501120, 518400);
    }
    fn func20(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i32, mut arg3: i32) {
        let var4 = self.func21(env, arg0);
        let var5 = match arg1 { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(var4), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(var4), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, _ => { env.storage().instance().extend_ttl((arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 } }
        var5;
    }
    fn func21(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    let var4 = self.memory.load32(arg0 as usize) as i32;
                                    match var4 {
                                        0 => break 'label6,
                                        1 => break 'label5,
                                        2 => break 'label4,
                                        3 => break 'label3,
                                        _ => break 'label6,
                                    }
                                    break;
                                }
                                self.func26(env, var1, 1048648, 9);
                                let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
                                if slot_var1_0_i32 != 0 {
                                    break 'label1;
                                }
                                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                                var2 = slot_var1_8_i64;
                                let var6 = self.memory.load64(arg0 as usize + 16) as i64;
                                slot_var1_8_i64 = var6 as i64;
                                let var7 = self.memory.load64(arg0 as usize + 8) as i64;
                                let mut slot_var1_0_i64 = var7 as i64;
                                let var8 = self.func27(env, 1048596, 2, var1, 2);
                                self.func28(env, var1, var2, var8);
                                break 'label2;
                                break;
                            }
                            self.func26(env, var1, 1048657, 7);
                            if slot_var1_0_i32 != 0 {
                                break 'label1;
                            }
                            let var11 = self.memory.load64(arg0 as usize + 8) as i64;
                            self.func28(env, var1, slot_var1_8_i64, var11);
                            break 'label2;
                            break;
                        }
                        self.func26(env, var1, 1048664, 5);
                        if slot_var1_0_i32 != 0 {
                            break 'label1;
                        }
                        let var14 = self.memory.load64(arg0 as usize + 8) as i64;
                        self.func28(env, var1, slot_var1_8_i64, var14);
                        break 'label2;
                        break;
                    }
                    self.func26(env, var1, 1048669, 5);
                    if slot_var1_0_i32 != 0 {
                        break 'label1;
                    }
                    slot_var1_0_i64 = slot_var1_8_i64 as i64;
                    let var17 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    var2 = var17;
                    break 'label0;
                    break;
                }
                var2 = slot_var1_8_i64;
                if (slot_var1_0_i64 == 0) as i32 != 0 {
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
    fn func22(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(32);
        self.global0 = var1;
        var2 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var7 = self.func23(env);
                var3 = var7;
                let var8 = self.func24(env, var3, 0 /* Void */);
                if (var8 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var9 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                var2 = var9;
                var4 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var4 == 24) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(8).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.func25(env, var2, 1048772, 3, var1.wrapping_add(8), 3);
                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                var2 = slot_var1_8_i64;
                if (var2 & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                var3 = slot_var1_16_i64;
                if (!(String::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                var5 = slot_var1_24_i64;
                if (!(String::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store32(arg0 as usize + 24, (var2 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32);
                self.memory.store64(arg0 as usize + 16, var5 as u64);
                self.memory.store64(arg0 as usize + 8, var3 as u64);
                var2 = 1 /* True */;
                break;
            }
            self.memory.store64(arg0 as usize, var2 as u64);
            self.global0 = var1.wrapping_add(32);
            return;
            break;
        }
        unreachable!();
    }
    fn func23(&mut self, env: &Env) -> i64 {
        METADATA
    }
    fn func24(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var2 == 1 /* True */) as i32
    }
    fn func25(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
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
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
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
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        let mut slot_var3_8_i64 = arg2 as i64;
        let mut slot_var3_0_i64 = arg1 as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        arg2 = var5;
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg2 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func29(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func30(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        let mut slot_var0_8_i64 = 3 /* Error(Contract, #0) */ as i64;
        'label0: loop {
            'label1: loop {
                let var3 = self.func21(env, var0.wrapping_add(8));
                var1 = var3;
                let var4 = self.func24(env, var1, 0 /* Void */);
                if (var4 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var5 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var1)).unwrap_or(val_from_i64(0))) } }
                var1 = var5;
                if (Address::try_from_val(env, &val_from_i64(var1)).is_ok()) as i32 != 0 {
                    break 'label0;
                }
                unreachable!();
                break;
            }
            self.func31(env);
            unreachable!();
            break;
        }
        self.global0 = var0.wrapping_add(32);
        var1
    }
    fn func31(&mut self, env: &Env) {
        self.func44(env);
        unreachable!();
    }
    fn func32(&mut self, env: &Env, mut arg0: i64) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32);
        self.global0 = var1;
        let mut slot_var1_8_i64 = 3 /* Error(Contract, #0) */ as i64;
        let var3 = self.func21(env, var1.wrapping_add(8));
        let var4 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var3), &val_from_i64(arg0)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var3), &val_from_i64(arg0)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var3), &val_from_i64(arg0)); 0 } }
        var4;
        self.global0 = var1.wrapping_add(32);
    }
    fn func33(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_sub(80);
        self.global0 = var3;
        let mut slot_var3_24_i64 = arg2 as i64;
        let mut slot_var3_16_i64 = arg1 as i64;
        let mut slot_var3_8_i64 = 0 /* False */ as i64;
        'label0: loop {
            'label1: loop {
                let var7 = self.func21(env, var3.wrapping_add(8));
                arg2 = var7;
                let var8 = self.func24(env, arg2, 0 /* False */);
                if (var8 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var9 = match 0 /* False */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg2)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg2)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg2)).unwrap_or(val_from_i64(0))) } }
                arg2 = var9;
                var4 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var4 == 16) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var3.wrapping_add(32).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                        var4 = var4.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                'label4: loop {
                    if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(arg2)).is_ok())) as i32 != 0 {
                        break 'label4;
                    }
                    self.func25(env, arg2, 1048632, 2, var3.wrapping_add(32), 2);
                    let mut slot_var3_32_i64 = self.memory.load64(var3 as usize + 32) as i64;
                    self.func34(env, var3.wrapping_add(48), slot_var3_32_i64);
                    let mut slot_var3_48_i32 = self.memory.load32(var3 as usize + 48) as i32;
                    if (slot_var3_48_i32 == 1) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var3_40_i64 = self.memory.load64(var3 as usize + 40) as i64;
                    arg2 = slot_var3_40_i64;
                    if (arg2 & 255 != 0) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var3_72_i64 = self.memory.load64(var3 as usize + 72) as i64;
                    arg1 = slot_var3_72_i64;
                    let mut slot_var3_64_i64 = self.memory.load64(var3 as usize + 64) as i64;
                    var5 = slot_var3_64_i64;
                    'label5: loop {
                        let var12 = self.func35(env);
                        var4 = (arg2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                        if (var12 as u32 > var4 as u32) as i32 != 0 {
                            break 'label5;
                        }
                        self.memory.store64(arg0 as usize, var5 as u64);
                        self.memory.store32(arg0 as usize + 16, var4 as u32);
                        self.memory.store64(arg0 as usize + 8, arg1 as u64);
                        break 'label0;
                        break;
                    }
                    self.memory.store64(arg0 as usize + 8, 0 /* False */ as u64);
                    self.memory.store64(arg0 as usize, 0 /* False */ as u64);
                    self.memory.store32(arg0 as usize + 16, var4 as u32);
                    break 'label0;
                    break;
                }
                unreachable!();
                break;
            }
            self.memory.store64(arg0 as usize, 0 /* False */ as u64);
            self.memory.store32(arg0.wrapping_add(16) as usize, 0 as u32);
            self.memory.store64(arg0.wrapping_add(8) as usize, 0 /* False */ as u64);
            break;
        }
        self.global0 = var3.wrapping_add(80);
    }
    fn func34(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func35(&mut self, env: &Env) -> i32 {
        let var0 = env.ledger().sequence() as i64
        (var0 as u64).wrapping_shr(32 as u32) as i64 as i32
    }
    fn func36(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i32) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(80);
        self.global0 = var5;
        'label0: loop {
            var6 = (arg3 == 0) as i32;
            if (({ let a = (arg2 != 0 /* False */) as i32; let b = (arg3 > 0 /* False */) as i32; if var6 != 0 { a } else { b } }) == 0) as i32 != 0 {
                break 'label0;
            }
            let var8 = self.func35(env);
            if (arg4 as u32 >= var8 as u32) as i32 != 0 {
                break 'label0;
            }
            self.func37(env);
            unreachable!();
            break;
        }
        let mut slot_var5_16_i64 = arg1 as i64;
        let mut slot_var5_8_i64 = arg0 as i64;
        let mut slot_var5_0_i64 = 0 /* False */ as i64;
        let mut slot_var5_40_i64 = arg1 as i64;
        let mut slot_var5_32_i64 = arg0 as i64;
        let mut slot_var5_24_i64 = 0 /* False */ as i64;
        let var10 = self.func21(env, var5.wrapping_add(24));
        arg1 = var10;
        self.func38(env, var5.wrapping_add(64), arg2, arg3);
        'label1: loop {
            'label2: loop {
                'label3: loop {
                    let mut slot_var5_64_i32 = self.memory.load32(var5 as usize + 64) as i32;
                    if (slot_var5_64_i32 == 1) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var5_72_i64 = self.memory.load64(var5 as usize + 72) as i64;
                    let mut slot_var5_48_i64 = slot_var5_72_i64 as i64;
                    let mut slot_var5_56_i64 = ((arg4 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
                    let var12 = self.func27(env, 1048632, 2, var5.wrapping_add(48), 2);
                    let var13 = match 0 /* False */ { 0 => { env.storage().persistent().set(&val_from_i64(arg1), &val_from_i64(var12)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg1), &val_from_i64(var12)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg1), &val_from_i64(var12)); 0 } }
                    var13;
                    if (({ let a = (arg2 != 0 /* False */) as i32; let b = (arg3 > 0 /* False */) as i32; if var6 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        break 'label1;
                    }
                    let var14 = self.func35(env);
                    var6 = var14;
                    if ((arg4 as u32) < var6 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    arg4 = arg4.wrapping_sub(var6);
                    self.func20(env, var5, 0 /* False */, arg4, arg4);
                    break 'label1;
                    break;
                }
                unreachable!();
                break;
            }
            self.func31(env);
            unreachable!();
            break;
        }
        self.global0 = var5.wrapping_add(80);
    }
    fn func37(&mut self, env: &Env) {
        unreachable!();
    }
    fn func38(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func39(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var4 = var8.wrapping_sub(32);
        self.global0 = var4;
        self.func33(env, var4, arg0, arg1);
        'label0: loop {
            let mut slot_var4_0_i64 = self.memory.load64(var4 as usize) as i64;
            var5 = slot_var4_0_i64;
            var6 = ((var5 as u64) < arg2 as u64) as i32;
            let mut slot_var4_8_i64 = self.memory.load64(var4 as usize + 8) as i64;
            var7 = slot_var4_8_i64;
            if ({ let a = var6; let b = (var7 < arg3) as i32; if (var7 == arg3) as i32 != 0 { a } else { b } }) != 0 {
                break 'label0;
            }
            'label1: loop {
                if (({ let a = (arg2 != 0 /* False */) as i32; let b = (arg3 > 0 /* False */) as i32; if (arg3 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                    break 'label1;
                }
                let mut slot_var4_16_i32 = self.memory.load32(var4 as usize + 16) as i32;
                self.func36(env, arg0, arg1, var5.wrapping_sub(arg2), var7.wrapping_sub(arg3).wrapping_sub(var6 as u32 as i64), slot_var4_16_i32);
                break;
            }
            self.global0 = var4.wrapping_add(32);
            return;
            break;
        }
        self.func37(env);
        unreachable!();
    }
    fn func40(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(64);
        self.global0 = var2;
        let mut slot_var2_8_i64 = 1 /* True */ as i64;
        let mut slot_var2_16_i64 = arg1 as i64;
        arg1 = 0 /* False */;
        var3 = 0 /* False */;
        'label0: loop {
            'label1: loop {
                let var6 = self.func21(env, var2.wrapping_add(8));
                var4 = var6;
                let var7 = self.func24(env, var4, 1 /* True */);
                if (var7 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var8 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) } }
                self.func34(env, var2.wrapping_add(32), var8);
                let mut slot_var2_32_i32 = self.memory.load32(var2 as usize + 32) as i32;
                if (slot_var2_32_i32 == 1) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var2_56_i64 = self.memory.load64(var2 as usize + 56) as i64;
                var3 = slot_var2_56_i64;
                let mut slot_var2_48_i64 = self.memory.load64(var2 as usize + 48) as i64;
                arg1 = slot_var2_48_i64;
                self.func19(env, var2.wrapping_add(8));
                break;
            }
            self.memory.store64(arg0 as usize, arg1 as u64);
            self.memory.store64(arg0 as usize + 8, var3 as u64);
            self.global0 = var2.wrapping_add(64);
            return;
            break;
        }
        unreachable!();
    }
    fn func41(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(32);
        self.global0 = var3;
        let mut slot_var3_8_i64 = 1 /* True */ as i64;
        let mut slot_var3_16_i64 = arg0 as i64;
        let var5 = self.func21(env, var3.wrapping_add(8));
        let var6 = self.func42(env, arg1, arg2);
        let var7 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var5), &val_from_i64(var6)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var5), &val_from_i64(var6)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var5), &val_from_i64(var6)); 0 } }
        var7;
        self.func19(env, var3.wrapping_add(8));
        self.global0 = var3.wrapping_add(32);
    }
    fn func42(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func38(env, var2, arg0, arg1);
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
    fn func43(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(16);
        self.global0 = var3;
        self.func40(env, var3, arg0);
        'label0: loop {
            let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
            var4 = slot_var3_8_i64;
            let var7 = arg2;
            let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
            arg2 = slot_var3_0_i64;
            arg1 = arg2.wrapping_add(arg1);
            arg2 = var4.wrapping_add(arg2).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64);
            if ((var4 ^ var7 ^ 18446744073709551615) & (var4 ^ arg2) < 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            self.func41(env, arg0, arg1, arg2);
            self.global0 = var3.wrapping_add(16);
            return;
            break;
        }
        self.func44(env);
        unreachable!();
    }
    fn func44(&mut self, env: &Env) {
        self.func37(env);
        unreachable!();
    }
    fn func45(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var3 = var7.wrapping_sub(16);
        self.global0 = var3;
        self.func40(env, var3, arg0);
        'label0: loop {
            'label1: loop {
                let mut slot_var3_0_i64 = self.memory.load64(var3 as usize) as i64;
                var4 = slot_var3_0_i64;
                var5 = ((var4 as u64) < arg1 as u64) as i32;
                let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
                var6 = slot_var3_8_i64;
                if ({ let a = var5; let b = (var6 < arg2) as i32; if (var6 == arg2) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label1;
                }
                let var9 = arg2;
                arg2 = var6.wrapping_sub(arg2).wrapping_sub(var5 as u32 as i64);
                if ((var6 ^ var9) & (var6 ^ arg2) >= 0 /* False */) as i32 != 0 {
                    break 'label0;
                }
                self.func44(env);
                unreachable!();
                break;
            }
            self.func37(env);
            unreachable!();
            break;
        }
        self.func41(env, arg0, var4.wrapping_sub(arg1), arg2);
        self.global0 = var3.wrapping_add(16);
    }
    fn func46(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        'label0: loop {
            if (arg1 < 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            return;
            break;
        }
        self.func37(env);
        unreachable!();
    }
    fn func47(&mut self, env: &Env, mut admin: i64, mut decimal: i64, mut name: i64, mut symbol: i64) -> i64 {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                if (!(Address::try_from_val(env, &val_from_i64(admin)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (decimal & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (!(String::try_from_val(env, &val_from_i64(name)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (!(String::try_from_val(env, &val_from_i64(symbol)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                if (decimal as u64 >= 81604378624 as u64) as i32 != 0 {
                    break 'label0;
                }
                self.func32(env, admin);
                let mut slot_var4_24_i64 = symbol as i64;
                let mut slot_var4_16_i64 = name as i64;
                let mut slot_var4_8_i64 = (decimal & 31) as i64;
                let var7 = self.func23(env);
                let var8 = self.func27(env, 1048772, 3, var4.wrapping_add(8), 3);
                let var9 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(var8)); 0 } }
                var9;
                self.global0 = var4.wrapping_add(32);
                return 0 /* Void */;
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
    fn func48(&mut self, env: &Env, mut to: i64, mut amount: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var2, amount);
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if (slot_var2_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
            amount = slot_var2_16_i64;
            let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
            var3 = slot_var2_24_i64;
            self.func46(env, amount, var3);
            let var7 = self.func30(env);
            let var8 = { address_from_i64(env, var7).require_auth(); 0 }
            var8;
            self.func49(env);
            self.func43(env, to, amount, var3);
            let var11 = self.func50(env, 1048744, to);
            let var12 = self.func42(env, amount, var3);
            let var13 = { env.events().publish(val_from_i64(var11), val_from_i64(var12)); 0 }
            var13;
            self.global0 = var2.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func49(&mut self, env: &Env) {
        let var0 = { env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD as u32, INSTANCE_BUMP_AMOUNT as u32); 0 }
        var0;
    }
    fn func50(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        let mut slot_var2_8_i64 = arg1 as i64;
        let var4 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var2_0_i64 = var4 as i64;
        arg0 = 0;
        let var5: i64;
        'label0: loop {
            'label1: loop {
                if (arg0 != 16) as i32 != 0 {
                    break 'label1;
                }
                arg0 = 0;
                'label2: loop {
                    'label3: loop {
                        if (arg0 == 16) as i32 != 0 {
                            break 'label2;
                        }
                        let var6 = self.memory.load64(var2.wrapping_add(arg0) as usize) as i64;
                        self.memory.store64(var2.wrapping_add(16).wrapping_add(arg0) as usize, var6 as u64);
                        arg0 = arg0.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                arg1 = var7;
                self.global0 = var2.wrapping_add(32);
                return arg1;
                break;
            }
            self.memory.store64(var2.wrapping_add(16).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            continue 'label0;
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func51(&mut self, env: &Env, mut new_admin: i64) -> i64 {
        let mut var1: i64 = 0;
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(new_admin)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var2 = self.func30(env);
        var1 = var2;
        let var3 = { address_from_i64(env, var1).require_auth(); 0 }
        var3;
        self.func49(env);
        self.func32(env, new_admin);
        let var6 = self.func50(env, 1048576, var1);
        let var7 = { env.events().publish(val_from_i64(var6), val_from_i64(new_admin)); 0 }
        var7;
        0 /* Void */
    }
    fn func52(&mut self, env: &Env, mut from: i64, mut spender: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(spender)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func49(env);
            self.func33(env, var2, from, spender);
            let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
            let var6 = self.func42(env, slot_var2_0_i64, slot_var2_8_i64);
            from = var6;
            self.global0 = var2.wrapping_add(32);
            return from;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func53(&mut self, env: &Env, mut from: i64, mut spender: i64, mut amount: i64, mut expiration_ledger: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(32);
        self.global0 = var4;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(spender)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var4, amount);
            let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (expiration_ledger & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
            amount = slot_var4_24_i64;
            let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
            var5 = slot_var4_16_i64;
            let var8 = { address_from_i64(env, from).require_auth(); 0 }
            var8;
            self.func46(env, var5, amount);
            self.func49(env);
            self.func36(env, from, spender, var5, amount, (expiration_ledger as u64).wrapping_shr(32 as u32) as i64 as i32);
            slot_var4_16_i64 = spender as i64;
            let mut slot_var4_0_i64 = from as i64;
            let mut slot_var4_8_i32 = 1048680 as i32;
            let var12 = self.func54(env, var4);
            from = var12;
            let var13 = self.func42(env, var5, amount);
            self.func28(env, var4, var13, expiration_ledger & 4294967295);
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let var15 = { env.events().publish(val_from_i64(from), val_from_i64(slot_var4_8_i32)); 0 }
            var15;
            self.global0 = var4.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func54(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(48);
        self.global0 = var1;
        let var4 = self.memory.load64(arg0 as usize + 16) as i64;
        let mut slot_var1_16_i64 = var4 as i64;
        let var5 = self.memory.load64(arg0 as usize) as i64;
        let mut slot_var1_8_i64 = var5 as i64;
        let var6 = self.memory.load32(arg0 as usize + 8) as i32;
        let mut slot_var6_0_i64 = self.memory.load64(var6 as usize) as i64;
        let mut slot_var1_0_i64 = slot_var6_0_i64 as i64;
        arg0 = 0;
        let var7: i64;
        'label0: loop {
            'label1: loop {
                if (arg0 != 24) as i32 != 0 {
                    break 'label1;
                }
                arg0 = 0;
                'label2: loop {
                    'label3: loop {
                        if (arg0 == 24) as i32 != 0 {
                            break 'label2;
                        }
                        let var8 = self.memory.load64(var1.wrapping_add(arg0) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(24).wrapping_add(arg0) as usize, var8 as u64);
                        arg0 = arg0.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var9 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                var2 = var9;
                self.global0 = var1.wrapping_add(48);
                return var2;
                break;
            }
            self.memory.store64(var1.wrapping_add(24).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            continue 'label0;
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func55(&mut self, env: &Env, mut id: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(id)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.func49(env);
        self.func40(env, var1, id);
        let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        let var5 = self.func42(env, slot_var1_0_i64, slot_var1_8_i64);
        id = var5;
        self.global0 = var1.wrapping_add(16);
        id
    }
    fn func56(&mut self, env: &Env, mut from: i64, mut to_muxed: i64, mut amount: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var3 = var8.wrapping_sub(48);
        self.global0 = var3;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            var4 = 1;
            'label1: loop {
                'label2: loop {
                    match (to_muxed as i32 & 255).wrapping_add(-77) {
                        0 => break 'label2,
                        1 => break 'label1,
                        _ => break 'label0,
                    }
                    break;
                }
                var4 = 0;
                break;
            }
            self.func34(env, var3, amount);
            let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
            if (slot_var3_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
            amount = slot_var3_24_i64;
            let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
            var5 = slot_var3_16_i64;
            let var10 = { address_from_i64(env, from).require_auth(); 0 }
            var10;
            self.func46(env, var5, amount);
            self.func49(env);
            self.func45(env, from, var5, amount);
            var6 = to_muxed;
            'label3: loop {
                if (var4 == 0) as i32 != 0 {
                    break 'label3;
                }
                let var14 = val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64(to_muxed)).unwrap().address().into_val(env))
                var6 = var14;
                break;
            }
            self.func43(env, var6, var5, amount);
            'label4: loop {
                'label5: loop {
                    if var4 != 0 {
                        break 'label5;
                    }
                    to_muxed = 0 /* False */;
                    break 'label4;
                    break;
                }
                'label6: loop {
                    let var16 = val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64(to_muxed)).unwrap().id().into_val(env))
                    var7 = var16;
                    var4 = var7 as i32 & 255;
                    if (var4 == 6) as i32 != 0 {
                        break 'label6;
                    }
                    'label7: loop {
                        if (var4 != 64) as i32 != 0 {
                            break 'label7;
                        }
                        to_muxed = 1 /* True */;
                        let var17 = val_from_i64(var7).as_u64().unwrap_or(0) as i64
                        var7 = var17;
                        break 'label4;
                        break;
                    }
                    self.func44(env);
                    unreachable!();
                    break;
                }
                var7 = (var7 as u64).wrapping_shr(0 as u32) as i64;
                to_muxed = 1 /* True */;
                break;
            }
            slot_var3_16_i64 = var5 as i64;
            let mut slot_var3_40_i64 = var6 as i64;
            let mut slot_var3_32_i64 = from as i64;
            let mut slot_var3_8_i64 = var7 as i64;
            let mut slot_var3_0_i64 = to_muxed as i64;
            slot_var3_24_i64 = amount as i64;
            self.func57(env, var3);
            self.global0 = var3.wrapping_add(48);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func57(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(32);
        self.global0 = var1;
        let var6 = self.memory.load64(arg0 as usize + 32) as i64;
        var2 = var6;
        let var7 = self.memory.load64(arg0 as usize + 40) as i64;
        let mut slot_var1_24_i64 = var7 as i64;
        let mut slot_var1_8_i64 = var2 as i64;
        let mut slot_var1_16_i32 = 1048688 as i32;
        let var8 = self.func54(env, var1.wrapping_add(8));
        var3 = var8;
        let var9 = self.memory.load64(arg0 as usize + 16) as i64;
        let var10 = self.memory.load64(arg0 as usize + 24) as i64;
        let var11 = self.func42(env, var9, var10);
        var4 = var11;
        var2 = 0 /* Void */;
        'label0: loop {
            let var12 = self.memory.load32(arg0 as usize) as i32;
            if (var12 != 1) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                let var13 = self.memory.load64(arg0 as usize + 8) as i64;
                var2 = var13;
                if (var2 as u64 > 72057594037927935 as u64) as i32 != 0 {
                    break 'label1;
                }
                var2 = var2.wrapping_shl(0 as u32) | 0;
                break 'label0;
                break;
            }
            let var14 = val_to_i64(Val::from_u64(var2 as u64))
            var2 = var14;
            break;
        }
        let mut slot_var1_16_i64 = var2 as i64;
        slot_var1_8_i64 = var4 as i64;
        let var15 = self.func27(env, 1048716, 2, var1.wrapping_add(8), 2);
        let var16 = { env.events().publish(val_from_i64(var3), val_from_i64(var15)); 0 }
        var16;
        self.global0 = var1.wrapping_add(32);
    }
    fn func58(&mut self, env: &Env, mut spender: i64, mut from: i64, mut to: i64, mut amount: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(48);
        self.global0 = var4;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(spender)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var4, amount);
            let mut slot_var4_0_i32 = self.memory.load32(var4 as usize) as i32;
            if (slot_var4_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var4_24_i64 = self.memory.load64(var4 as usize + 24) as i64;
            amount = slot_var4_24_i64;
            let mut slot_var4_16_i64 = self.memory.load64(var4 as usize + 16) as i64;
            var5 = slot_var4_16_i64;
            let var8 = { address_from_i64(env, spender).require_auth(); 0 }
            var8;
            self.func46(env, var5, amount);
            self.func49(env);
            self.func39(env, from, spender, var5, amount);
            self.func45(env, from, var5, amount);
            self.func43(env, to, var5, amount);
            slot_var4_24_i64 = amount as i64;
            slot_var4_16_i64 = var5 as i64;
            let mut slot_var4_40_i64 = to as i64;
            let mut slot_var4_32_i64 = from as i64;
            let mut slot_var4_0_i64 = 0 /* False */ as i64;
            self.func57(env, var4);
            self.global0 = var4.wrapping_add(48);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func59(&mut self, env: &Env, mut from: i64, mut amount: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var2, amount);
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if (slot_var2_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
            amount = slot_var2_24_i64;
            let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
            var3 = slot_var2_16_i64;
            let var6 = { address_from_i64(env, from).require_auth(); 0 }
            var6;
            self.func46(env, var3, amount);
            self.func49(env);
            self.func45(env, from, var3, amount);
            self.func60(env, var3, amount, from);
            self.global0 = var2.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func60(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let var3 = self.func50(env, 1048736, arg2);
        let var4 = self.func42(env, arg0, arg1);
        let var5 = { env.events().publish(val_from_i64(var3), val_from_i64(var4)); 0 }
        var5;
    }
    fn func61(&mut self, env: &Env, mut spender: i64, mut from: i64, mut amount: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32);
        self.global0 = var3;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(spender)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var3, amount);
            let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
            if (slot_var3_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
            amount = slot_var3_24_i64;
            let mut slot_var3_16_i64 = self.memory.load64(var3 as usize + 16) as i64;
            var4 = slot_var3_16_i64;
            let var7 = { address_from_i64(env, spender).require_auth(); 0 }
            var7;
            self.func46(env, var4, amount);
            self.func49(env);
            self.func39(env, from, spender, var4, amount);
            self.func45(env, from, var4, amount);
            self.func60(env, var4, amount, from);
            self.global0 = var3.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func62(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        self.func22(env, var0);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var4 = self.memory.load32(var0 as usize + 24) as i64;
        var1 = var4;
        self.global0 = var0.wrapping_add(32);
        var1.wrapping_shl(32 as u32) | 0
    }
    fn func63(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        self.func22(env, var0);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var0_8_i64 = self.memory.load64(var0 as usize + 8) as i64;
        var1 = slot_var0_8_i64;
        self.global0 = var0.wrapping_add(32);
        var1
    }
    fn func64(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32);
        self.global0 = var0;
        self.func22(env, var0);
        'label0: loop {
            let mut slot_var0_0_i32 = self.memory.load32(var0 as usize) as i32;
            if slot_var0_0_i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var0_16_i64 = self.memory.load64(var0 as usize + 16) as i64;
        var1 = slot_var0_16_i64;
        self.global0 = var0.wrapping_add(32);
        var1
    }
}
