#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, String, Bytes, BytesN, Val, FromVal, Map, Symbol};

#[contract]
pub struct AnotherRandom;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn err_contract(code: u32) -> i64 {
    ((soroban_sdk::xdr::ScErrorType::Contract as u32 as i64) & 255).wrapping_shl(32 as u32) | (code as i64)
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WritePrices { pub updated_feeds: soroban_sdk::Vec<PriceData>, pub updater: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PriceData { pub package_timestamp: u64, pub price: soroban_sdk::U256, pub write_timestamp: u64, }

macro_rules! mload8 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load8($addr)
        }};
}
macro_rules! mload16 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load16($addr)
        }};
}
macro_rules! mload32 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load32($addr)
        }};
}
macro_rules! mload64 {
    ($addr:expr) => {{
            let Self { memory, .. } = self;
            memory.load64($addr)
        }};
}
macro_rules! mstore8 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store8($addr, $value)
        }};
}
macro_rules! mstore16 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store16($addr, $value)
        }};
}
macro_rules! mstore32 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store32($addr, $value)
        }};
}
macro_rules! mstore64 {
    ($addr:expr, $value:expr) => {{
            let Self { memory, .. } = self;
            memory.store64($addr, $value)
        }};
}
macro_rules! msize {
    () => {{
            let Self { memory, .. } = self;
            memory.size()
        }};
}
macro_rules! mgrow {
    ($pages:expr) => {{
            let Self { memory, .. } = self;
            memory.grow($pages)
        }};
}


#[contractimpl]
impl AnotherRandom {
    pub fn init(&mut self, env: Env, owner: soroban_sdk::Address) -> Result<(), soroban_sdk::Error> {
        let owner_key = String::from_str(&env, "owner");
        if env.storage().persistent().has(&owner_key) { return Err(soroban_sdk::Error::from_type_and_code(soroban_sdk::xdr::ScErrorType::Storage, soroban_sdk::xdr::ScErrorCode::ExistingValue)); }
        env.storage().persistent().set(&owner_key, &owner);
        Ok(())
    }
    pub fn change_owner(&mut self, env: Env, new_owner: soroban_sdk::Address) -> Result<(), soroban_sdk::Error> {
        let owner_key = String::from_str(&env, "owner");
        let pending_key = String::from_str(&env, "pending-owner");
        let owner: Address = env.storage().persistent().get(&owner_key).ok_or(soroban_sdk::Error::from_type_and_code(soroban_sdk::xdr::ScErrorType::Storage, soroban_sdk::xdr::ScErrorCode::MissingValue))?;
        owner.require_auth();
        env.storage().persistent().set(&pending_key, &new_owner);
        Ok(())
    }
    pub fn accept_ownership(&mut self, env: Env) -> Result<(), soroban_sdk::Error> {
        let owner_key = String::from_str(&env, "owner");
        let pending_key = String::from_str(&env, "pending-owner");
        let pending: Address = env.storage().persistent().get(&pending_key).ok_or(soroban_sdk::Error::from_type_and_code(soroban_sdk::xdr::ScErrorType::Storage, soroban_sdk::xdr::ScErrorCode::MissingValue))?;
        pending.require_auth();
        env.storage().persistent().set(&owner_key, &pending);
        env.storage().persistent().remove(&pending_key);
        Ok(())
    }
    pub fn cancel_ownership_transfer(&mut self, env: Env) -> Result<(), soroban_sdk::Error> {
        let owner_key = String::from_str(&env, "owner");
        let pending_key = String::from_str(&env, "pending-owner");
        let owner: Address = env.storage().persistent().get(&owner_key).ok_or(soroban_sdk::Error::from_type_and_code(soroban_sdk::xdr::ScErrorType::Storage, soroban_sdk::xdr::ScErrorCode::MissingValue))?;
        owner.require_auth();
        env.storage().persistent().remove(&pending_key);
        Ok(())
    }
    pub fn upgrade(&mut self, env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
    pub fn get_prices(&mut self, mut env: Env, mut feed_ids: soroban_sdk::Vec<soroban_sdk::String>, payload: soroban_sdk::Bytes) -> Result<(u64, soroban_sdk::Vec<soroban_sdk::U256>,), soroban_sdk::Error> {
        let mut value: i32 = 0;
        let mut value: i64 = 0;
        let mut value: i64 = 0;
        let var5 = self.global0;
        value = var5.wrapping_sub(96);
        self.global0 = value;
        if (!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | (!(Bytes::try_from_val(env, &val_from_i64(payload)).is_ok())) as i32 != 0 {
            unreachable!();
        }
        else {
            self.entry_decode(env, value, feed_ids, payload);
            self.decode_val_or_error(env, value.wrapping_sub(-64), value);
            let mut slot_var2_72_i64 = mload64!(value as usize + 72) as i64;
            payload = slot_var2_72_i64;
            let var8: i64;
            {
                let mut slot_var2_64_i32 = mload32!(value as usize + 64) as i32;
                if (slot_var2_64_i32 == 0) as i32 != 0 {
                    let mut slot_var2_80_i64 = mload64!(value as usize + 80) as i64;
                    value = slot_var2_80_i64;
                    let var9 = Vec::<Val>::from_val(env, &val_from_i64(value)).len() as i64
                    let var10 = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64
                    let var11 = 42949672963 /* Error(Contract, #10) */;
                    if ((var9 ^ var10) as u64 >= 4294967296 as u64) as i32 != 0 {
                        var8 = var11;
                        self.global0 = value.wrapping_add(96);
                        return var8;
                    }
                    var11;
                    let var12 = Vec::<Val>::from_val(env, &val_from_i64(value)).len() as i64
                    value = var12;
                    let var13 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                    feed_ids = var13;
                    mstore32!(value as usize + 60, (value as u64).wrapping_shr(32 as u32) as i64 as u32);
                    let mut slot_var2_56_i32 = 0 as i32;
                    let mut slot_var2_48_i64 = value as i64;
                    loop {
                        self.vec_pair_iter(env, value, value.wrapping_add(48));
                        self.copy_val_if_present(env, value.wrapping_sub(-64), value);
                        if (slot_var2_64_i32 == 1) as i32 != 0 {
                            let var16 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)); v.push_back(val_from_i64(slot_var2_80_i64)); val_to_i64(v.into_val(env)) }
                            feed_ids = var16;
                            continue;
                        }
                        break;
                    }
                    self.write_ok_val(env, value, payload);
                    let mut slot_var2_0_i32 = mload32!(value as usize) as i32;
                    if slot_var2_0_i32 != 0 {
                        self.global0 = value.wrapping_add(96);
                        return var8;
                    }
                    let mut slot_var2_8_i64 = mload64!(value as usize + 8) as i64;
                    payload = slot_var2_8_i64;
                    slot_var2_72_i64 = feed_ids as i64;
                    let mut slot_var2_64_i64 = payload as i64;
                    let var18 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    var8 = var18;
                    self.global0 = value.wrapping_add(96);
                    return var8;
                }
                var8 = payload;
            }
        }
        42949672963 /* Error(Contract, #10) */
    }
    pub fn write_prices(&mut self, env: Env, updater: soroban_sdk::Address, feed_ids: soroban_sdk::Vec<soroban_sdk::String>, payload: soroban_sdk::Bytes) {
        updater.require_auth_for_args((feed_ids, payload).into_val(&env));
    }
    pub fn read_prices(&mut self, mut env: Env, feed_ids: soroban_sdk::Vec<soroban_sdk::String>) -> Result<soroban_sdk::Vec<soroban_sdk::U256>, soroban_sdk::Error> {
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i64 = 0;
        let mut value: i64 = 0;
        let var7 = self.global0;
        value = var7.wrapping_sub(96);
        self.global0 = value;
        if (Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok()) as i32 != 0 {
            let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env))
            value = var8;
            let var9 = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64
            value = var9;
            let mut slot_var1_16_i32 = 0 as i32;
            let mut slot_var1_8_i64 = feed_ids as i64;
            mstore32!(value as usize + 20, (value as u64).wrapping_shr(32 as u32) as i64 as u32);
            value = value.wrapping_add(48);
            value = value.wrapping_add(80);
            {
                loop {
                    value = value.wrapping_sub(-64);
                    self.vec_next_string_flag(env, value, value.wrapping_add(8));
                    let mut slot_var1_64_i64 = mload64!(value as usize + 64) as i64;
                    let mut slot_var1_72_i64 = mload64!(value as usize + 72) as i64;
                    self.guard_nonzero_ptr(env, value.wrapping_add(24), slot_var1_64_i64, slot_var1_72_i64);
                    let mut slot_var1_24_i32 = mload32!(value as usize + 24) as i32;
                    if (slot_var1_24_i32 == 0) as i32 != 0 {
                        break;
                    }
                    let mut slot_var1_32_i64 = mload64!(value as usize + 32) as i64;
                    self.storage_get_val(env, value, slot_var1_32_i64);
                    if (slot_var1_64_i64 == 1) as i32 != 0 {
                        let mut slot_var3_0_i64 = mload64!(value as usize) as i64;
                        let mut slot_var2_0_i64 = slot_var3_0_i64 as i64;
                        let var13 = mload64!(value.wrapping_add(8) as usize) as i64;
                        mstore64!(value.wrapping_add(8) as usize, var13 as u64);
                        let mut slot_var1_40_i64 = slot_var1_72_i64 as i64;
                        self.check_recent_timestamp(env, value, value.wrapping_add(40));
                        feed_ids = slot_var1_72_i64;
                        if slot_var1_64_i64 != 0 {
                            value = feed_ids;
                            break;
                        }
                        let var15 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(value)); v.push_back(val_from_i64(feed_ids)); val_to_i64(v.into_val(env)) }
                        value = var15;
                        continue;
                    }
                    break;
                }
                value = Error(Storage, MissingValue);
            }
            self.global0 = value.wrapping_add(96);
            return value;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn read_timestamp(&mut self, env: Env, feed_id: soroban_sdk::String) -> Result<u64, soroban_sdk::Error> {
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let var3 = self.global0;
        value = var3.wrapping_add(-64);
        self.global0 = value;
        let var4: i64;
        {
            let mut __exit_label1: i32 = 0;
            '__if_guard0: {
                if (String::try_from_val(env, &val_from_i64(feed_id)).is_ok()) as i32 != 0 {
                    value = value.wrapping_add(32);
                    self.storage_get_val(env, value, feed_id);
                    let mut slot_var1_32_i32 = mload32!(value as usize + 32) as i32;
                    if (slot_var1_32_i32 != 1) as i32 != 0 {
                        break '__if_guard0;
                    }
                    let var6 = mload64!(value.wrapping_add(56) as usize) as i64;
                    mstore64!(value.wrapping_add(24) as usize, var6 as u64);
                    let mut slot_var1_48_i64 = mload64!(value as usize + 48) as i64;
                    let mut slot_var1_16_i64 = slot_var1_48_i64 as i64;
                    let mut slot_var1_40_i64 = mload64!(value as usize + 40) as i64;
                    let mut slot_var1_8_i64 = slot_var1_40_i64 as i64;
                    self.check_recent_timestamp(env, value, value.wrapping_add(8));
                    if slot_var1_32_i32 != 0 {
                        __exit_label1 = 1; break '__if_guard0;
                    }
                    self.write_ok_val(env, value, slot_var1_48_i64);
                    if (slot_var1_32_i32 != 1) as i32 != 0 {
                        __exit_label1 = 1; break '__if_guard0;
                    }
                }
                else {
                    unreachable!();
                }
            }
            if __exit_label1 == 0 {
                var4 = Error(Storage, MissingValue);
            }
            else {
                var4 = slot_var1_40_i64;
            }
        }
        self.global0 = value.wrapping_sub(-64);
        var4
    }
    pub fn read_price_data_for_feed(&mut self, env: Env, feed_id: soroban_sdk::String) -> Result<PriceData, soroban_sdk::Error> {
        let mut value: i32 = 0;
        let var2 = self.global0;
        value = var2.wrapping_sub(96);
        self.global0 = value;
        if (String::try_from_val(env, &val_from_i64(feed_id)).is_ok()) as i32 != 0 {
            self.storage_get_val(env, value.wrapping_add(40), feed_id);
            let mut slot_var1_40_i32 = mload32!(value as usize + 40) as i32;
            if (slot_var1_40_i32 == 1) as i32 != 0 {
                let var4 = mload64!(value.wrapping_sub(-64) as usize) as i64;
                mstore64!(value.wrapping_add(88) as usize, var4 as u64);
                let mut slot_var1_56_i64 = mload64!(value as usize + 56) as i64;
                let mut slot_var1_80_i64 = slot_var1_56_i64 as i64;
                let mut slot_var1_48_i64 = mload64!(value as usize + 48) as i64;
                let mut slot_var1_72_i64 = slot_var1_48_i64 as i64;
                self.check_recent_timestamp(env, value.wrapping_add(8), value.wrapping_add(72));
            }
            else {
                let mut slot_var1_16_i64 = Error(Storage, MissingValue) as i64;
                let mut slot_var1_8_i64 = 1 /* True */ as i64;
            }
            let var6 = self.result_from_val(env, value.wrapping_add(8));
            self.global0 = value.wrapping_add(96);
            return var6;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn read_price_data(&mut self, mut env: Env, feed_ids: soroban_sdk::Vec<soroban_sdk::String>) -> Result<soroban_sdk::Vec<PriceData>, soroban_sdk::Error> {
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let mut value: i64 = 0;
        let mut value: i64 = 0;
        let var9 = self.global0;
        value = var9.wrapping_sub(112);
        self.global0 = value;
        value = Error(Storage, MissingValue);
        {
            if (Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok()) as i32 != 0 {
                let var10 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                value = var10;
                let var11 = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64
                value = var11;
                let mut slot_var1_8_i32 = 0 as i32;
                let mut slot_var1_0_i64 = feed_ids as i64;
                mstore32!(value as usize + 12, (value as u64).wrapping_shr(32 as u32) as i64 as u32);
                value = value.wrapping_add(96);
                value = value.wrapping_add(72);
                value = value.wrapping_add(40);
                loop {
                    value = value.wrapping_add(56);
                    self.vec_next_string_flag(env, value, value);
                    let mut slot_var1_56_i64 = mload64!(value as usize + 56) as i64;
                    let mut slot_var1_64_i64 = mload64!(value as usize + 64) as i64;
                    self.guard_nonzero_ptr(env, value.wrapping_add(16), slot_var1_56_i64, slot_var1_64_i64);
                    let mut slot_var1_16_i32 = mload32!(value as usize + 16) as i32;
                    if (slot_var1_16_i32 == 0) as i32 != 0 {
                        break;
                    }
                    let mut slot_var1_24_i64 = mload64!(value as usize + 24) as i64;
                    self.storage_get_val(env, value, slot_var1_24_i64);
                    if (slot_var1_56_i64 != 1) as i32 != 0 {
                        self.global0 = value.wrapping_add(112);
                        return value;
                    }
                    let mut slot_var2_0_i64 = mload64!(value as usize) as i64;
                    let mut slot_var4_0_i64 = slot_var2_0_i64 as i64;
                    value = value.wrapping_add(8);
                    let mut slot_var6_0_i64 = mload64!(value as usize) as i64;
                    mstore64!(value.wrapping_add(8) as usize, slot_var6_0_i64 as u64);
                    let mut slot_var1_32_i64 = slot_var1_64_i64 as i64;
                    self.check_recent_timestamp(env, value, value.wrapping_add(32));
                    feed_ids = slot_var1_64_i64;
                    if (slot_var1_56_i64 == 0) as i32 != 0 {
                        let mut slot_var3_0_i64 = slot_var2_0_i64 as i64;
                        mstore64!(value.wrapping_add(8) as usize, slot_var6_0_i64 as u64);
                        let mut slot_var1_88_i64 = feed_ids as i64;
                        let var16 = self.result_unwrap_or_panic(env, value.wrapping_add(88));
                        let var17 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(value)); v.push_back(val_from_i64(var16)); val_to_i64(v.into_val(env)) }
                        value = var17;
                        continue;
                    }
                    self.global0 = value.wrapping_add(112);
                    return value;
                }
                value = feed_ids;
                break;
            }
            else {
                unreachable!();
            }
        }
        self.global0 = value.wrapping_add(112);
        value
    }
    pub fn check_price_data(&mut self, env: Env, price_data: PriceData) -> Result<PriceData, soroban_sdk::Error> {
        let mut value: i32 = 0;
        let mut value: i32 = 0;
        let var3 = self.global0;
        value = var3.wrapping_add(-64);
        self.global0 = value;
        self.map_unpack_to_val(env, value.wrapping_add(32), price_data);
        let mut slot_var1_32_i32 = mload32!(value as usize + 32) as i32;
        if (slot_var1_32_i32 == 1) as i32 != 0 {
            unreachable!();
        }
        let var5 = mload64!(value.wrapping_add(56) as usize) as i64;
        mstore64!(value.wrapping_add(24) as usize, var5 as u64);
        let var6 = mload64!(value.wrapping_add(48) as usize) as i64;
        mstore64!(value.wrapping_add(16) as usize, var6 as u64);
        let mut slot_var1_40_i64 = mload64!(value as usize + 40) as i64;
        let mut slot_var1_8_i64 = slot_var1_40_i64 as i64;
        value = value.wrapping_add(32);
        self.check_recent_timestamp(env, value, value.wrapping_add(8));
        let var8 = self.result_from_val(env, value);
        self.global0 = value.wrapping_sub(-64);
        var8
    }
    pub fn unique_signer_threshold(&mut self, env: Env) -> u64 {
        let mut value: i32 = 0;
        let mut value: i64 = 0;
        let var2 = self.global0;
        value = var2.wrapping_sub(16);
        self.global0 = value;
        self.write_ok_val(env, value, 0 /* Void */);
        let mut slot_var0_0_i32 = mload32!(value as usize) as i32;
        if (slot_var0_0_i32 == 1) as i32 != 0 {
            unreachable!();
        }
        let mut slot_var0_8_i64 = mload64!(value as usize + 8) as i64;
        self.global0 = value.wrapping_add(16);
        slot_var0_8_i64
    }
}