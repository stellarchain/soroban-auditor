#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, IntoVal, String, BytesN, Val, FromVal, Vec, Bytes, Symbol};

#[contract]
pub struct RandomContract;

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
pub enum DataKey { ContractInfo, Launch(soroban_sdk::Address, u64), LaunchBalance(soroban_sdk::Address, u64, soroban_sdk::Address), SpaceMission(soroban_sdk::Address), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractInfo { pub admin: soroban_sdk::Address, pub native_contract: soroban_sdk::Address, pub slz_fee: u32, pub slz_fee_destination: soroban_sdk::Address, pub space_fee: u32, pub space_missions: soroban_sdk::Map<u32, SpaceMissionData>, pub stability_check_duration: u64, pub stellarbucks_contract: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Launch { pub asset: soroban_sdk::Address, pub dev: soroban_sdk::Address, pub funds_claimed: bool, pub funds_recipient: soroban_sdk::Address, pub info: soroban_sdk::String, pub max_price: i128, pub max_supply: i128, pub min_price: i128, pub pool_balance: i128, pub stability_check: bool, pub stability_check_end: u64, pub stellarbucks: i128, pub supply: i128, pub timestamp: u64, pub tokens_claimed: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpaceMissionData { pub guaranteed_success_funding: u64, pub reward: i128, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error { NotInitialized = 0, Initialized = 1, NotAuthorized = 2, ZeroMinPrice = 100, ZeroMaxPrice = 101, MinPriceGreaterMax = 102, ZeroMaxSupply = 103, MaxSupplyTooBig = 104, MaxPriceTooBig = 105, LaunchNotFound = 200, LaunchInProgress = 201, LaunchEnded = 202, PriceChanged = 203, InsufficientBalance = 204, LaunchSoldOut = 205, LaunchFundsClaimed = 206, InvalidMissionDifficulty = 300, ExasiveFunding = 301, MissionRewardChanged = 302, MissionRewardZero = 303, }

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
impl RandomContract {
    pub fn initialize(&mut self, env: Env, admin: soroban_sdk::Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk::Address, stellarbucks_contract: soroban_sdk::Address, native_contract: soroban_sdk::Address, space_missions_odds: soroban_sdk::Map<u32, u64>) {
        let value = ContractInfo {
            admin: admin,
            native_contract: native_contract,
            slz_fee: slz_fee,
            slz_fee_destination: slz_fee_destination,
            space_fee: space_fee,
            space_missions: Map::new(&env),
            stability_check_duration: stability_check_duration,
            stellarbucks_contract: stellarbucks_contract,
        };
        env.storage().instance().set(&DataKey::ContractInfo, &value);
    }
    pub fn change_contract_info(&mut self, env: Env, admin: soroban_sdk::Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk::Address, space_missions_odds: soroban_sdk::Map<u32, u64>) {
        let mut current: ContractInfo = env.storage().instance().get(&DataKey::ContractInfo).unwrap();
        current.admin = admin;
        current.slz_fee = slz_fee;
        current.slz_fee_destination = slz_fee_destination;
        current.space_fee = space_fee;
        current.stability_check_duration = stability_check_duration;
        env.storage().instance().set(&DataKey::ContractInfo, &current);
    }
    pub fn upgrade(&mut self, env: Env, hash: soroban_sdk::BytesN<32>) {
        env.deployer().update_current_contract_wasm(hash);
    }
    pub fn start_space_mission(&mut self, env: Env, user: soroban_sdk::Address, funding: i128, difficulty: u32, min_mission_reward: i128) {
        user.require_auth_for_args((funding, difficulty, min_mission_reward).into_val(&env));
    }
    pub fn add_space_missions_reward(&mut self, env: Env, user: soroban_sdk::Address, funds: i128, reward_difficulty: u32) {
        user.require_auth_for_args((funds, reward_difficulty).into_val(&env));
    }
    pub fn new_launch(&mut self, env: Env, dev: soroban_sdk::Address, funds_recipient: soroban_sdk::Address, info: soroban_sdk::String, asset: soroban_sdk::Address, max_supply: i128, min_price: i128, max_price: i128, launch_index: u64) {
        let value = Launch {
            asset: asset,
            dev: dev,
            funds_claimed: false,
            funds_recipient: funds_recipient,
            info: info,
            max_price: max_price,
            max_supply: max_supply,
            min_price: min_price,
            pool_balance: 0,
            stability_check: false,
            stability_check_end: 0,
            stellarbucks: 0,
            supply: 0,
            timestamp: 0,
            tokens_claimed: 0,
        };
        env.storage().instance().set(&DataKey::Launch(asset, launch_index), &value);
    }
    pub fn cancel_launch(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,)) {
    }
    pub fn buy(&mut self, env: Env, user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,), sending: i128, min_receive: i128) {
        user.require_auth_for_args((launch_key, sending, min_receive).into_val(&env));
    }
    pub fn sell(&mut self, env: Env, user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,), sending: i128, min_receive: i128) {
        user.require_auth_for_args((launch_key, sending, min_receive).into_val(&env));
    }
    pub fn claim_launch_funds(&mut self, mut env: Env, launch_key: (soroban_sdk::Address, u64,)) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(368);
        self.global0 = var1;
        self.func57(env, var1.wrapping_add(168), launch_key);
        loop {
            let mut slot_var1_168_i64 = mload64!(var1 as usize + 168) as i64;
            if ((slot_var1_168_i64 == 0) as i32 == 0) as i32 != 0 {
                break;
            }
            let var8 = mload64!(var1.wrapping_add(184) as usize) as i64;
            launch_key = var8;
            let mut slot_var1_176_i64 = mload64!(var1 as usize + 176) as i64;
            var3 = slot_var1_176_i64;
            self.func53(env, var1.wrapping_add(168));
            if (slot_var1_168_i64 == 0) as i32 != 0 {
                self.func81(env, err_contract(0));
                break;
            }
            let var11 = mload64!(var1.wrapping_add(208) as usize) as i64;
            var4 = var11;
            let mut slot_var1_352_i64 = launch_key as i64;
            let mut slot_var1_344_i64 = var3 as i64;
            let mut slot_var1_336_i64 = 1 /* True */ as i64;
            self.func47(env, var1.wrapping_add(168), var1.wrapping_add(336));
            let var13 = mload8!(var1 as usize + 329) as i32;
            if (var13 == 2) as i32 != 0 {
                self.func81(env, err_contract(200));
                break;
            }
            let var15 = self.func83(env, var1, var1.wrapping_add(168), 168);
            var1 = var15;
            let var16 = mload8!(var1 as usize + 161) as i32;
            if (var16 == 0) as i32 != 0 {
                let var17 = self.func65(env, var1);
                if (var17 == 0) as i32 != 0 {
                    self.func81(env, err_contract(201));
                    break;
                }
                let var19 = val_to_i64(env.current_contract_address().into_val(env))
                let mut slot_var1_120_i64 = mload64!(var1 as usize + 120) as i64;
                let mut slot_var1_32_i64 = mload64!(var1 as usize + 32) as i64;
                let var20 = mload64!(var1.wrapping_add(40) as usize) as i64;
                self.func74(env, var4, var19, slot_var1_120_i64, slot_var1_32_i64, var20);
                mstore8!(var1 as usize + 161, 1 as u8);
                let mut slot_var1_184_i64 = launch_key as i64;
                slot_var1_176_i64 = var3 as i64;
                slot_var1_168_i64 = 1 /* True */ as i64;
                var2 = var1.wrapping_add(168);
                self.func49(env, var2, var1);
                let mut slot_var1_152_i64 = mload64!(var1 as usize + 152) as i64;
                var4 = slot_var1_152_i64;
                let var23 = self.storage_key_from_str(env, 1048597, 20);
                var5 = var23;
                mstore64!(var1.wrapping_add(192) as usize, var4 as u64);
                mstore64!(var1.wrapping_add(184) as usize, launch_key as u64);
                slot_var1_176_i64 = var3 as i64;
                slot_var1_168_i64 = var5 as i64;
                let var24 = self.func60(env, var2);
                let var25 = self.func50(env, var1);
                let var26 = { env.events().publish(val_from_i64(var24), val_from_i64(var25)); 0 }
                var26;
                self.global0 = var1.wrapping_add(368);
                return 0 /* Void */;
            }
            self.func81(env, err_contract(206));
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn claim_launch_balance(&mut self, env: Env, mut user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,)) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var2 = var13.wrapping_sub(448);
        self.global0 = var2;
        loop {
            if (!(Address::try_from_val(env, &val_from_i64(user)).is_ok())) as i32 != 0 {
                break;
            }
            self.func57(env, var2.wrapping_add(232), launch_key);
            let mut slot_var2_232_i64 = mload64!(var2 as usize + 232) as i64;
            if ((slot_var2_232_i64 == 0) as i32 == 0) as i32 != 0 {
                break;
            }
            let var15 = mload64!(var2.wrapping_add(248) as usize) as i64;
            launch_key = var15;
            let mut slot_var2_240_i64 = mload64!(var2 as usize + 240) as i64;
            var7 = slot_var2_240_i64;
            self.func53(env, var2.wrapping_add(232));
            if (slot_var2_232_i64 == 0) as i32 != 0 {
                self.func81(env, err_contract(0));
                break;
            }
            let var18 = mload64!(var2.wrapping_add(264) as usize) as i64;
            var11 = var18;
            let mut slot_var2_416_i64 = launch_key as i64;
            let mut slot_var2_408_i64 = var7 as i64;
            let mut slot_var2_400_i64 = 1 /* True */ as i64;
            self.func47(env, var2.wrapping_add(232), var2.wrapping_add(400));
            let var20 = mload8!(var2 as usize + 393) as i32;
            if (var20 == 2) as i32 != 0 {
                self.func81(env, err_contract(200));
                break;
            }
            var3 = var2.wrapping_sub(-64);
            let var22 = self.func83(env, var3, var2.wrapping_add(232), 168);
            var22;
            let var23 = self.func65(env, var3);
            if (var23 == 0) as i32 != 0 {
                self.func81(env, err_contract(201));
                break;
            }
            let mut slot_var2_256_i64 = user as i64;
            let mut slot_var2_248_i64 = launch_key as i64;
            slot_var2_240_i64 = var7 as i64;
            slot_var2_232_i64 = 0 /* Void */ as i64;
            var4 = var2.wrapping_add(232);
            self.func43(env, var2.wrapping_add(40), var4);
            let var26 = mload64!(var2.wrapping_add(56) as usize) as i64;
            var9 = var26;
            let mut slot_var2_40_i32 = mload32!(var2 as usize + 40) as i32;
            var3 = slot_var2_40_i32;
            let mut slot_var2_48_i64 = mload64!(var2 as usize + 48) as i64;
            var10 = slot_var2_48_i64;
            let var27 = val_to_i64(env.current_contract_address().into_val(env))
            var5 = var27;
            let mut slot_var2_200_i64 = mload64!(var2 as usize + 200) as i64;
            var10 = { let a = var10; let b = 0 /* False */; if var3 != 0 { a } else { b } };
            var9 = { let a = var9; let b = 0 /* False */; if var3 != 0 { a } else { b } };
            self.func74(env, slot_var2_200_i64, var5, user, var10, var9);
            slot_var2_256_i64 = user as i64;
            slot_var2_248_i64 = launch_key as i64;
            slot_var2_240_i64 = var7 as i64;
            slot_var2_232_i64 = 0 /* Void */ as i64;
            let var29 = self.func44(env, var4);
            self.func89(env, var29);
            var3 = var2.wrapping_add(136);
            let mut slot_var3_0_i64 = mload64!(var3 as usize) as i64;
            var5 = slot_var3_0_i64;
            let mut slot_var2_128_i64 = mload64!(var2 as usize + 128) as i64;
            var6 = slot_var2_128_i64;
            var8 = var6.wrapping_add(var10);
            var6 = (((var8 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_add(var9));
            if ((var5 ^ var9 ^ 18446744073709551615) & (var5 ^ var6) < 0 /* False */) as i32 != 0 {
                break;
            }
            slot_var3_0_i64 = var6 as i64;
            slot_var2_128_i64 = var8 as i64;
            slot_var2_248_i64 = launch_key as i64;
            slot_var2_240_i64 = var7 as i64;
            slot_var2_232_i64 = 1 /* True */ as i64;
            let mut slot_var2_36_i32 = 0 as i32;
            self.func49(env, var2.wrapping_add(232), var2.wrapping_sub(-64));
            let mut slot_var2_112_i64 = mload64!(var2 as usize + 112) as i64;
            let var32 = mload64!(var2.wrapping_add(120) as usize) as i64;
            self.func68(env, var2.wrapping_add(16), var10, var9, slot_var2_112_i64, var32, var2.wrapping_add(36));
            if slot_var2_36_i32 != 0 {
                break;
            }
            let mut slot_var2_64_i64 = mload64!(var2 as usize + 64) as i64;
            var5 = slot_var2_64_i64;
            let var34 = mload64!(var2.wrapping_add(72) as usize) as i64;
            var6 = var34;
            if (var5 | var6 == 0) as i32 != 0 {
                break;
            }
            let mut slot_var2_16_i64 = mload64!(var2 as usize + 16) as i64;
            var8 = slot_var2_16_i64;
            let var35 = mload64!(var2.wrapping_add(24) as usize) as i64;
            var12 = var35;
            if (var8 | var12 ^ 9223372036854775808 == 0) as i32 & (var5 & var6 == 18446744073709551615) as i32 != 0 {
                break;
            }
            self.func71(env, var2, var8, var12, var5, var6);
            let mut slot_var2_0_i64 = mload64!(var2 as usize) as i64;
            var6 = slot_var2_0_i64;
            let var37 = mload64!(var2.wrapping_add(8) as usize) as i64;
            var5 = var37;
            if (({ let a = (var6 == 0) as i32; let b = (var5 < 0 /* False */) as i32; if (var5 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var38 = self.storage_key_from_str(env, 1049284, 4);
                var8 = var38;
                let var39 = self.func52(env, var6, var5);
                slot_var2_408_i64 = var39 as i64;
                slot_var2_400_i64 = user as i64;
                var3 = 0;
                loop {
                    if (var3 == 16) as i32 != 0 {
                        var3 = 0;
                            if (var3 != 16) as i32 != 0 {
                                let var40 = mload64!(var2.wrapping_add(400).wrapping_add(var3) as usize) as i64;
                                mstore64!(var2.wrapping_add(232).wrapping_add(var3) as usize, var40 as u64);
                                var3 = var3.wrapping_add(8);
                                continue;
                            }
                        let var41 = self.vec_new_val(env);
                        self.func76(env, var11, var8, var41);
                    }
                    else {
                        mstore64!(var2.wrapping_add(232).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                        var3 = var3.wrapping_add(8);
                        continue;
                    }
                    break;
                }
            }
            let mut slot_var2_216_i64 = mload64!(var2 as usize + 216) as i64;
            var11 = slot_var2_216_i64;
            let var43 = self.storage_key_from_str(env, 1048637, 12);
            var8 = var43;
            mstore64!(var2.wrapping_add(256) as usize, var11 as u64);
            mstore64!(var2.wrapping_add(248) as usize, launch_key as u64);
            slot_var2_240_i64 = var7 as i64;
            slot_var2_232_i64 = var8 as i64;
            let var44 = self.func60(env, var2.wrapping_add(232));
            let var45 = self.func52(env, var10, var9);
            var7 = var45;
            let var46 = self.func52(env, var6, var5);
            slot_var2_408_i64 = var46 as i64;
            slot_var2_400_i64 = var7 as i64;
            let var47 = self.vec_new_val(env);
            let mut slot_var2_440_i64 = var47 as i64;
            let mut slot_var2_432_i64 = user as i64;
            let var48 = self.vec_new_val(env);
            let var49 = { env.events().publish(val_from_i64(var44), val_from_i64(var48)); 0 }
            var49;
            self.global0 = var2.wrapping_add(448);
            return 0 /* Void */;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn calculate_buy(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), sending: i128) -> (i128, i128, i128, i128, i128,) {
        let (launch_asset, launch_id) = launch_key;
        let launch: Launch = env.storage().instance().get(&DataKey::Launch(launch_asset.clone(), launch_id)).unwrap();
        let contract_info: ContractInfo = env.storage().instance().get(&DataKey::ContractInfo).unwrap();
        let slz_fee_bps: i128 = contract_info.slz_fee as i128;
        let space_fee_bps: i128 = contract_info.space_fee as i128;
        let fee_slz = (sending * slz_fee_bps) / 10_000;
        let fee_space = (sending * space_fee_bps) / 10_000;
        let net_amount = sending - fee_slz - fee_space;
        // TODO: reconstruct full pricing math from wasm (uses large-precision arithmetic)
        let out0: i128 = 0;
        let out1: i128 = 0;
        let out2: i128 = 0;
        let out3: i128 = 0;
        let out4: i128 = 0;
        let _ = (launch, net_amount);
        (out0, out1, out2, out3, out4)
    }
    pub fn calculate_sell(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), sending: i128) -> (i128, i128, i128, i128, i128,) {
        let (launch_asset, launch_id) = launch_key;
        let launch: Launch = env.storage().instance().get(&DataKey::Launch(launch_asset.clone(), launch_id)).unwrap();
        let contract_info: ContractInfo = env.storage().instance().get(&DataKey::ContractInfo).unwrap();
        let slz_fee_bps: i128 = contract_info.slz_fee as i128;
        let space_fee_bps: i128 = contract_info.space_fee as i128;
        let fee_slz = (sending * slz_fee_bps) / 10_000;
        let fee_space = (sending * space_fee_bps) / 10_000;
        let net_amount = sending - fee_slz - fee_space;
        // TODO: reconstruct full pricing math from wasm (uses large-precision arithmetic)
        let out0: i128 = 0;
        let out1: i128 = 0;
        let out2: i128 = 0;
        let out3: i128 = 0;
        let out4: i128 = 0;
        let _ = (launch, net_amount);
        (out0, out1, out2, out3, out4)
    }
    pub fn get_launch_data(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,)) -> Launch {
        let (key0_launch_key_0, key0_launch_key_1) = launch_key;
        env.storage().instance().get(&DataKey::Launch(key0_launch_key_0, key0_launch_key_1)).unwrap()
    }
    pub fn get_contract_info(&mut self, env: Env) -> ContractInfo {
        env.storage().instance().get(&DataKey::ContractInfo).unwrap()
    }
    pub fn get_launch_balance(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), user: soroban_sdk::Address) -> i128 {
        let (key0_launch_key_0, key0_launch_key_1) = launch_key;
        env.storage().instance().get(&DataKey::LaunchBalance(key0_launch_key_0, key0_launch_key_1, user)).unwrap()
    }
    pub fn version(&mut self, env: Env) -> (u32, u32, u32,) {
        (0, 0, 0)
    }
}

#[allow(dead_code)]
impl RandomContract {
    fn copy_bytes_to_linear_memory(&mut self, env: &Env, b: i64, b_pos: i64, lm_pos: i64, len: i64) {
        let b = Bytes::from_val(env, &val_from_i64(b));
        let start = val_from_i64(b_pos).as_u32().unwrap_or(0);
        let len = val_from_i64(len).as_u32().unwrap_or(0);
        let lm_pos = val_from_i64(lm_pos).as_u32().unwrap_or(0);
        let bytes = b.slice(start..start.saturating_add(len));
        let mut buf: Vec<u8> = vec![0; len as usize];
        bytes.copy_into_slice(&mut buf);
        for (i, b) in buf.iter().enumerate() {
            mstore8!(lm_pos as usize + i, *b as u8);
        }
    }
    fn copy_string_to_linear_memory(&mut self, env: &Env, s: i64, s_pos: i64, lm_pos: i64, len: i64) {
        let s = String::from_val(env, &val_from_i64(s));
        let start = val_from_i64(s_pos).as_u32().unwrap_or(0);
        let len = val_from_i64(len).as_u32().unwrap_or(0);
        let lm_pos = val_from_i64(lm_pos).as_u32().unwrap_or(0);
        let bytes = s.to_bytes().slice(start..start.saturating_add(len));
        let mut buf: Vec<u8> = vec![0; len as usize];
        bytes.copy_into_slice(&mut buf);
        for (i, b) in buf.iter().enumerate() {
            mstore8!(lm_pos as usize + i, *b as u8);
        }
    }
    fn for_each_val<F: FnMut(Val)>(&self, env: &Env, vals: &Vec<Val>, mut f: F) {
        let mut i: i64 = 0;
        let len = vals.len() as i64;
        while i < len {
            let val = vals.get_unchecked((i as i64) << 32);
            f(val);
            i = i.wrapping_add(1);
        }
    }
    fn require_len_match(&self, a: &Vec<Val>, b: &Vec<Val>) -> Result<(), i64> {
        let len_a = a.len() as i64;
        let len_b = b.len() as i64;
        if ((len_a ^ len_b) as u64 >= 4294967296 as u64) as i32 != 0 {
            return Err(err_contract(10));
        }
        Ok(())
    }
    fn require_len_match_len(&self, len_a: i64, len_b: i64) -> Result<(), i64> {
        if ((len_a ^ len_b) as u64 >= 4294967296 as u64) as i32 != 0 {
            return Err(err_contract(10));
        }
        Ok(())
    }
    fn vec_new_val(&self, env: &Env) -> i64 {
        val_to_i64(Vec::<Val>::new(env).into_val(env))
    }
    fn vec_push_val(&self, env: &Env, vec_val: i64, val: i64) -> i64 {
        let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_val));
        v.push_back(val_from_i64(val));
        val_to_i64(v.into_val(env))
    }
    fn for_each_string<F: FnMut(i64)>(&self, env: &Env, vec_ptr: i64, mut f: F) {
        let mut tmp: i32 = self.global0.wrapping_sub(64);
        self.global0 = tmp;
        let mut slot_tmp_16_i32: i32 = 0;
        let mut slot_tmp_8_i64: i64 = vec_ptr as i64;
        loop {
            let next_ptr = tmp.wrapping_sub(-64);
            self.vec_next_string_flag(env, next_ptr, tmp.wrapping_add(8));
            let mut slot_tmp_64_i64 = mload64!(tmp as usize + 64) as i64;
            let mut slot_tmp_72_i64 = mload64!(tmp as usize + 72) as i64;
            self.guard_nonzero_ptr(env, tmp.wrapping_add(24), slot_tmp_64_i64, slot_tmp_72_i64);
            let mut slot_tmp_24_i32 = mload32!(tmp as usize + 24) as i32;
            if (slot_tmp_24_i32 == 0) as i32 != 0 {
                break;
            }
            let mut slot_tmp_32_i64 = mload64!(tmp as usize + 32) as i64;
            self.storage_get_val(env, next_ptr, slot_tmp_32_i64);
            if (slot_tmp_64_i64 == 1) as i32 != 0 {
                self.check_recent_timestamp(env, next_ptr, tmp.wrapping_add(40));
                let val = mload64!(tmp as usize + 72) as i64;
                f(val);
                if slot_tmp_64_i64 != 0 {
                    break;
                }
                continue;
            }
            break;
        }
        self.global0 = tmp.wrapping_add(64);
    }
    fn for_each_string_checked<F: FnMut(i64)>(&mut self, env: &Env, vec_ptr: i64, mut f: F) -> Result<(), i64> {
        let mut tmp: i32 = self.global0.wrapping_sub(64);
        self.global0 = tmp;
        let mut slot_tmp_16_i32: i32 = 0;
        let mut slot_tmp_8_i64: i64 = vec_ptr as i64;
        loop {
            let next_ptr = tmp.wrapping_sub(-64);
            self.vec_next_string_flag(env, next_ptr, tmp.wrapping_add(8));
            let mut slot_tmp_64_i64 = mload64!(tmp as usize + 64) as i64;
            let mut slot_tmp_72_i64 = mload64!(tmp as usize + 72) as i64;
            self.guard_nonzero_ptr(env, tmp.wrapping_add(24), slot_tmp_64_i64, slot_tmp_72_i64);
            let mut slot_tmp_24_i32 = mload32!(tmp as usize + 24) as i32;
            if (slot_tmp_24_i32 == 0) as i32 != 0 {
                break;
            }
            let mut slot_tmp_32_i64 = mload64!(tmp as usize + 32) as i64;
            self.storage_get_val(env, next_ptr, slot_tmp_32_i64);
            if (slot_tmp_64_i64 == 1) as i32 != 0 {
                self.check_recent_timestamp(env, next_ptr, tmp.wrapping_add(40));
                let val = mload64!(tmp as usize + 72) as i64;
                f(val);
                if slot_tmp_64_i64 != 0 {
                    break;
                }
                continue;
            }
            self.global0 = tmp.wrapping_add(64);
            return Err(Error(Storage, MissingValue));
        }
        self.global0 = tmp.wrapping_add(64);
        Ok(())
    }
    fn next_string_checked(&mut self, env: &Env, base: i32, tmp: i32, iter_ptr: i32) -> (i64, i64, i32) {
        self.vec_next_string_flag(env, tmp, iter_ptr);
        let flag = mload64!(base as usize + 64) as i64;
        let val = mload64!(base as usize + 72) as i64;
        self.guard_nonzero_ptr(env, base.wrapping_add(24), flag, val);
        let ok = mload32!(base as usize + 24) as i32;
        (flag, val, ok)
    }
    fn pack_ok_val(&self, val: i64) -> i64 {
        if (val as u64) <= 72057594037927935 {
            val
        } else {
            val_to_i64(Val::from_u64(val as u64))
        }
    }
    fn zero_24_bytes(&self, base: i32) {
        mstore64!(base as usize, 0);
        mstore64!(base.wrapping_add(8) as usize, 0);
        mstore64!(base.wrapping_add(16) as usize, 0);
    }
}

#[allow(dead_code)]
impl RandomContract {
    fn func42(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        loop {
            var2 = arg1 as i32 & 255;
            if (var2 != 64) as i32 != 0 {
                if (var2 != 6) as i32 != 0 {
                    var3 = 1 /* True */;
                    var4 = Error(Value, UnexpectedType);
                    break;
                }
                var4 = (arg1 as u64).wrapping_shr(0 as u32) as i64;
                break;
            }
            let var5 = val_from_i64(arg1).as_u64().unwrap_or(0) as i64
            var4 = var5;
        }
        mstore64!(arg0 as usize + 8, var4 as u64);
        mstore64!(arg0 as usize, var3 as u64);
    }
    fn func43(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(32);
        self.global0 = var2;
        loop {
            let var7 = self.func44(env, arg1);
            var3 = var7;
            let var8 = self.call_eq_one(env, var3, 1 /* True */);
            let var9: i64;
            if var8 != 0 {
                let var10 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                self.func46(env, var2.wrapping_add(8), var10);
                let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
                if ((slot_var2_8_i64 == 0) as i32 == 0) as i32 != 0 {
                    break;
                }
                let var12 = mload64!(var2.wrapping_add(24) as usize) as i64;
                var4 = var12;
                let mut slot_var2_16_i64 = mload64!(var2 as usize + 16) as i64;
                var5 = slot_var2_16_i64;
                var9 = 1 /* True */;
            }
            else {
                var9 = 0 /* False */;
            }
            var3 = var9;
            mstore64!(arg0 as usize + 8, var5 as u64);
            mstore64!(arg0 as usize, var3 as u64);
            mstore64!(arg0.wrapping_add(16) as usize, var4 as u64);
            self.global0 = var2.wrapping_add(32);
            return;
        }
        unreachable!();
    }
    fn func44(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(32);
        self.global0 = var1;
        let var6: i64;
        let mut __exit_label0: i32 = 0;
        'label1: {
            'label2: {
                'label3: {
                    {
                        let var7 = mload32!(arg0 as usize) as i32;
                        match var7.wrapping_sub(1) {
                            0 => break,
                            1 => break,
                            2 => break,
                            _ => break,
                        }
                    }
                    let var8 = self.func64(env, 1048674, 6);
                    var2 = var8;
                    let var9 = mload64!(arg0 as usize + 8) as i64;
                    var3 = var9;
                    let var10 = mload64!(arg0 as usize + 16) as i64;
                    let var11 = self.func61(env, var10);
                    let mut slot_var1_16_i64 = var11 as i64;
                    let mut slot_var1_8_i64 = var3 as i64;
                    let mut slot_var1_0_i64 = var2 as i64;
                    var6 = self.vec_new_val(env);
                    __exit_label0 = 1; break 'label1;
                }
                let var13 = self.func64(env, 1048680, 13);
                var2 = var13;
                let var14 = mload64!(arg0 as usize + 8) as i64;
                var3 = var14;
                let var15 = mload64!(arg0 as usize + 16) as i64;
                let var16 = self.func61(env, var15);
                var4 = var16;
                let var17 = mload64!(arg0 as usize + 24) as i64;
                let mut slot_var1_24_i64 = var17 as i64;
                slot_var1_16_i64 = var4 as i64;
                slot_var1_8_i64 = var3 as i64;
                slot_var1_0_i64 = var2 as i64;
                var6 = self.vec_new_val(env);
                __exit_label0 = 1; break 'label1;
            }
            let var19 = self.func64(env, 1048693, 12);
            var2 = var19;
            let var20 = mload64!(arg0 as usize + 8) as i64;
            slot_var1_8_i64 = var20 as i64;
            slot_var1_0_i64 = var2 as i64;
            var6 = self.vec_new_val(env);
            __exit_label0 = 1; break 'label1;
        }
        if __exit_label0 == 0 {
        let var22 = self.func64(env, 1048662, 12);
        slot_var1_0_i64 = var22 as i64;
        var6 = self.vec_new_val(env);
        }
        self.global0 = var1.wrapping_add(32);
        var6
    }
    fn call_eq_one(&mut self, env: &Env, arg0: i64, arg1: i64) -> i32 {
        if self.has_contract_data(env, arg0, arg1) == 1 { 1 } else { 0 }
    }
    fn func46(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        let mut __exit_label0: i32 = 0;
        loop {
            var2 = arg1 as i32 & 255;
            if (var2 != 69) as i32 != 0 {
                if (var2 == 11) as i32 != 0 {
                    mstore64!(arg0.wrapping_add(16) as usize, arg1.wrapping_shr(63 as u32) as u64);
                    mstore64!(arg0 as usize + 8, arg1.wrapping_shr(0 as u32) as u64);
                    break;
                }
                mstore64!(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
                var4 = 1 /* True */;
                __exit_label0 = 1; break 'label1;
            }
            let var5 = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64)
            var3 = var5;
            let var6 = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
            arg1 = var6;
            mstore64!(arg0.wrapping_add(16) as usize, var3 as u64);
            mstore64!(arg0 as usize + 8, arg1 as u64);
        }
        if __exit_label0 == 0 {
        var4 = 0 /* False */;
        }
        mstore64!(arg0 as usize, var4 as u64);
    }
    fn func47(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
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
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let var25 = self.global0;
        var2 = var25.wrapping_sub(176);
        self.global0 = var2;
        let mut __exit_label0: i32 = 0;
        loop {
            let var26 = self.func44(env, arg1);
            var5 = var26;
            let var27 = self.call_eq_one(env, var5, 1 /* True */);
            if (var27 == 0) as i32 != 0 {
                mstore8!(arg0 as usize + 161, 2 as u8);
                break;
            }
            let var28 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var5)).unwrap_or(val_from_i64(0))) } }
            var5 = var28;
            arg1 = 0;
                if (arg1 != 120) as i32 != 0 {
                    mstore64!(var2.wrapping_add(32).wrapping_add(arg1) as usize, 0 /* Void */ as u64);
                    arg1 = arg1.wrapping_add(8);
                    continue;
                }
            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            self.func48(env, var5, 1049040, 15, var2.wrapping_add(32), 15);
            let mut slot_var2_32_i64 = mload64!(var2 as usize + 32) as i64;
            var5 = slot_var2_32_i64;
            if (!(Address::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_40_i64 = mload64!(var2 as usize + 40) as i64;
            var6 = slot_var2_40_i64;
            if (!(Address::try_from_val(env, &val_from_i64(var6)).is_ok())) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let var30 = mload8!(var2 as usize + 48) as i32;
            arg1 = var30;
            arg1 = { let a = 1; let b = ((arg1 != 0) as i32).wrapping_shl(1 as u32); if (arg1 == 1) as i32 != 0 { a } else { b } };
            if (arg1 == 2) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_56_i64 = mload64!(var2 as usize + 56) as i64;
            var7 = slot_var2_56_i64;
            if (!(Address::try_from_val(env, &val_from_i64(var7)).is_ok())) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_64_i64 = mload64!(var2 as usize + 64) as i64;
            var8 = slot_var2_64_i64;
            if (!(String::try_from_val(env, &val_from_i64(var8)).is_ok())) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_72_i64 = mload64!(var2 as usize + 72) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_72_i64);
            let mut slot_var2_152_i64 = mload64!(var2 as usize + 152) as i64;
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var3 = var2.wrapping_add(168);
            let mut slot_var3_0_i64 = mload64!(var3 as usize) as i64;
            var9 = slot_var3_0_i64;
            let mut slot_var2_160_i64 = mload64!(var2 as usize + 160) as i64;
            var10 = slot_var2_160_i64;
            let mut slot_var2_80_i64 = mload64!(var2 as usize + 80) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_80_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var11 = slot_var3_0_i64;
            var12 = slot_var2_160_i64;
            let mut slot_var2_88_i64 = mload64!(var2 as usize + 88) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_88_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var3 = var2.wrapping_add(168);
            var13 = slot_var3_0_i64;
            var14 = slot_var2_160_i64;
            let mut slot_var2_96_i64 = mload64!(var2 as usize + 96) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_96_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let var35 = mload8!(var2 as usize + 104) as i32;
            var4 = var35;
            var4 = { let a = 1; let b = ((var4 != 0) as i32).wrapping_shl(1 as u32); if (var4 == 1) as i32 != 0 { a } else { b } };
            if (var4 == 2) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var15 = slot_var3_0_i64;
            var16 = slot_var2_160_i64;
            let mut slot_var2_112_i64 = mload64!(var2 as usize + 112) as i64;
            self.func42(env, var2.wrapping_add(16), slot_var2_112_i64);
            let mut slot_var2_16_i32 = mload32!(var2 as usize + 16) as i32;
            if slot_var2_16_i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_24_i64 = mload64!(var2 as usize + 24) as i64;
            var17 = slot_var2_24_i64;
            let mut slot_var2_120_i64 = mload64!(var2 as usize + 120) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_120_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var3 = var2.wrapping_add(168);
            var18 = slot_var3_0_i64;
            var19 = slot_var2_160_i64;
            let mut slot_var2_128_i64 = mload64!(var2 as usize + 128) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_128_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            var20 = slot_var3_0_i64;
            var21 = slot_var2_160_i64;
            let mut slot_var2_136_i64 = mload64!(var2 as usize + 136) as i64;
            self.func42(env, var2, slot_var2_136_i64);
            let mut slot_var2_0_i32 = mload32!(var2 as usize) as i32;
            if slot_var2_0_i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
            var22 = slot_var2_8_i64;
            let mut slot_var2_144_i64 = mload64!(var2 as usize + 144) as i64;
            self.func46(env, var2.wrapping_add(152), slot_var2_144_i64);
            if ((slot_var2_152_i64 == 0) as i32 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let var41 = mload64!(var2.wrapping_add(168) as usize) as i64;
            var23 = var41;
            var24 = slot_var2_160_i64;
            mstore64!(arg0 as usize + 96, var10 as u64);
            mstore64!(arg0 as usize + 80, var14 as u64);
            mstore64!(arg0 as usize + 64, var24 as u64);
            mstore64!(arg0 as usize + 48, var19 as u64);
            mstore64!(arg0 as usize + 32, var16 as u64);
            mstore64!(arg0 as usize + 16, var21 as u64);
            mstore64!(arg0 as usize, var12 as u64);
            mstore8!(arg0 as usize + 161, (arg1 & 1) as u8);
            mstore8!(arg0 as usize + 160, (var4 & 1) as u8);
            mstore64!(arg0 as usize + 152, var22 as u64);
            mstore64!(arg0 as usize + 144, var17 as u64);
            mstore64!(arg0 as usize + 136, var5 as u64);
            mstore64!(arg0 as usize + 128, var8 as u64);
            mstore64!(arg0 as usize + 120, var7 as u64);
            mstore64!(arg0 as usize + 112, var6 as u64);
            mstore64!(arg0.wrapping_add(104) as usize, var9 as u64);
            mstore64!(arg0.wrapping_add(88) as usize, var13 as u64);
            mstore64!(arg0.wrapping_add(72) as usize, var23 as u64);
            mstore64!(arg0.wrapping_add(56) as usize, var18 as u64);
            mstore64!(arg0.wrapping_add(40) as usize, var15 as u64);
            mstore64!(arg0.wrapping_add(24) as usize, var20 as u64);
            mstore64!(arg0 as usize + 8, var11 as u64);
        }
        if __exit_label0 == 0 {
        self.global0 = var2.wrapping_add(176);
        return;
        }
        unreachable!();
    }
    fn func48(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        if (arg2 != arg4) as i32 != 0 {
            unreachable!();
        }
        let var5 = 0 /* TODO: map_unpack_to_linear_memory */
        var5;
    }
    fn func49(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let var2 = self.func44(env, arg0);
        let var3 = self.func50(env, arg1);
        let var4 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(var3)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(var3)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(var3)); 0 } }
        var4;
    }
    fn func50(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
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
        let var16 = self.global0;
        var1 = var16.wrapping_sub(128);
        self.global0 = var1;
        let var17 = mload64!(arg0 as usize + 136) as i64;
        var2 = var17;
        let var18 = mload64!(arg0 as usize + 112) as i64;
        var3 = var18;
        let var19 = mload8!(arg0 as usize + 161) as i64;
        var4 = var19;
        let var20 = mload64!(arg0 as usize + 120) as i64;
        var5 = var20;
        let var21 = mload64!(arg0 as usize + 128) as i64;
        var6 = var21;
        let var22 = mload64!(arg0 as usize + 96) as i64;
        let var23 = mload64!(arg0.wrapping_add(104) as usize) as i64;
        let var24 = self.func52(env, var22, var23);
        var7 = var24;
        let var25 = mload64!(arg0 as usize) as i64;
        let var26 = mload64!(arg0.wrapping_add(8) as usize) as i64;
        let var27 = self.func52(env, var25, var26);
        var8 = var27;
        let var28 = mload64!(arg0 as usize + 80) as i64;
        let var29 = mload64!(arg0.wrapping_add(88) as usize) as i64;
        let var30 = self.func52(env, var28, var29);
        var9 = var30;
        let var31 = mload64!(arg0 as usize + 32) as i64;
        let var32 = mload64!(arg0.wrapping_add(40) as usize) as i64;
        let var33 = self.func52(env, var31, var32);
        var10 = var33;
        let var34 = mload8!(arg0 as usize + 160) as i64;
        var11 = var34;
        let var35 = mload64!(arg0 as usize + 144) as i64;
        let var36 = self.func61(env, var35);
        var12 = var36;
        let var37 = mload64!(arg0 as usize + 48) as i64;
        let var38 = mload64!(arg0.wrapping_add(56) as usize) as i64;
        let var39 = self.func52(env, var37, var38);
        var13 = var39;
        let var40 = mload64!(arg0 as usize + 16) as i64;
        let var41 = mload64!(arg0.wrapping_add(24) as usize) as i64;
        let var42 = self.func52(env, var40, var41);
        var14 = var42;
        let var43 = mload64!(arg0 as usize + 152) as i64;
        let var44 = self.func61(env, var43);
        var15 = var44;
        let var45 = mload64!(arg0 as usize + 64) as i64;
        let var46 = mload64!(arg0.wrapping_add(72) as usize) as i64;
        let var47 = self.func52(env, var45, var46);
        let mut slot_var1_120_i64 = var47 as i64;
        let mut slot_var1_112_i64 = var15 as i64;
        let mut slot_var1_104_i64 = var14 as i64;
        let mut slot_var1_96_i64 = var13 as i64;
        let mut slot_var1_88_i64 = var12 as i64;
        let mut slot_var1_80_i64 = var11 as i64;
        let mut slot_var1_72_i64 = var10 as i64;
        let mut slot_var1_64_i64 = var9 as i64;
        let mut slot_var1_56_i64 = var8 as i64;
        let mut slot_var1_48_i64 = var7 as i64;
        let mut slot_var1_40_i64 = var6 as i64;
        let mut slot_var1_32_i64 = var5 as i64;
        let mut slot_var1_24_i64 = var4 as i64;
        let mut slot_var1_16_i64 = var3 as i64;
        let mut slot_var1_8_i64 = var2 as i64;
        let var48 = self.map_new_val(env, 1049040, 15, var1.wrapping_add(8), 15);
        self.global0 = var1.wrapping_add(128);
        var48
    }
    fn func51(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let var3 = self.func44(env, arg0);
        let var4 = self.func52(env, arg1, arg2);
        let var5 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(var3), &val_from_i64(var4)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var3), &val_from_i64(var4)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var3), &val_from_i64(var4)); 0 } }
        var5;
    }
    fn func52(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        if ((arg1 ^ arg0.wrapping_shr(63 as u32) != 0 /* False */) as i32 | (arg0.wrapping_sub(18410715276690587648) as u64 > 72057594037927935 as u64) as i32 == 0) as i32 != 0 {
            return arg0.wrapping_shl(0 as u32) | 0;
        }
        let var2 = val_to_i64(Val::from_i128(((arg1 as i128) << 64) | (arg0 as u64 as i128)))
        var2
    }
    fn func53(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let var11 = self.global0;
        var1 = var11.wrapping_sub(80);
        self.global0 = var1;
        loop {
            let var12 = self.func44(env, 1049208);
            var3 = var12;
            let var13 = self.call_eq_one(env, var3, 0 /* Void */);
            let var14: i64;
            if var13 != 0 {
                let var15 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                var3 = var15;
                    if (var2 != 64) as i32 != 0 {
                        mstore64!(var1.wrapping_add(16).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                        var2 = var2.wrapping_add(8);
                        continue;
                    }
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break;
                }
                self.func48(env, var3, 1048820, 8, var1.wrapping_add(16), 8);
                let mut slot_var1_16_i64 = mload64!(var1 as usize + 16) as i64;
                var3 = slot_var1_16_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var3)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var1_24_i64 = mload64!(var1 as usize + 24) as i64;
                var4 = slot_var1_24_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var4)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var1_32_i64 = mload64!(var1 as usize + 32) as i64;
                var5 = slot_var1_32_i64;
                if (var5 & 255 != 0) as i32 != 0 {
                    break;
                }
                let mut slot_var1_40_i64 = mload64!(var1 as usize + 40) as i64;
                var6 = slot_var1_40_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var6)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var1_48_i64 = mload64!(var1 as usize + 48) as i64;
                var7 = slot_var1_48_i64;
                if (var7 & 255 != 0) as i32 != 0 {
                    break;
                }
                let mut slot_var1_56_i64 = mload64!(var1 as usize + 56) as i64;
                var8 = slot_var1_56_i64;
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var8)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var1_64_i64 = mload64!(var1 as usize + 64) as i64;
                self.func42(env, var1, slot_var1_64_i64);
                let mut slot_var1_0_i32 = mload32!(var1 as usize) as i32;
                if slot_var1_0_i32 != 0 {
                    break;
                }
                let mut slot_var1_72_i64 = mload64!(var1 as usize + 72) as i64;
                var9 = slot_var1_72_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var1_8_i64 = mload64!(var1 as usize + 8) as i64;
                var10 = slot_var1_8_i64;
                mstore64!(arg0 as usize + 8, var3 as u64);
                mstore32!(arg0.wrapping_add(60) as usize, (var5 as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(arg0.wrapping_add(56) as usize, (var7 as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore64!(arg0.wrapping_add(48) as usize, var8 as u64);
                mstore64!(arg0.wrapping_add(40) as usize, var4 as u64);
                mstore64!(arg0.wrapping_add(32) as usize, var9 as u64);
                mstore64!(arg0.wrapping_add(24) as usize, var6 as u64);
                mstore64!(arg0.wrapping_add(16) as usize, var10 as u64);
                var14 = 1 /* True */;
            }
            else {
                var14 = 0 /* False */;
            }
            mstore64!(arg0 as usize, var14 as u64);
            self.global0 = var1.wrapping_add(80);
            return;
        }
        unreachable!();
    }
    fn func54(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let var1 = self.func44(env, arg0);
        let var2 = self.call_eq_one(env, var1, 0 /* Void */);
        var2
    }
    fn func55(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func44(env, 1049208);
        let var2 = self.func56(env, arg0);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var1), &val_from_i64(var2)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var1), &val_from_i64(var2)); 0 } }
        var3;
    }
    fn func56(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var1 = var8.wrapping_add(-64);
        self.global0 = var1;
        let var9 = mload32!(arg0 as usize + 52) as i64;
        var2 = var9;
        let var10 = mload32!(arg0 as usize + 48) as i64;
        var3 = var10;
        let var11 = mload64!(arg0 as usize) as i64;
        var4 = var11;
        let var12 = mload64!(arg0 as usize + 32) as i64;
        var5 = var12;
        let var13 = mload64!(arg0 as usize + 16) as i64;
        var6 = var13;
        let var14 = mload64!(arg0 as usize + 40) as i64;
        var7 = var14;
        let var15 = mload64!(arg0 as usize + 8) as i64;
        let var16 = self.func61(env, var15);
        let mut slot_var1_48_i64 = var16 as i64;
        let mut slot_var1_40_i64 = var7 as i64;
        let mut slot_var1_24_i64 = var6 as i64;
        let mut slot_var1_8_i64 = var5 as i64;
        let mut slot_var1_0_i64 = var4 as i64;
        let var17 = mload64!(arg0 as usize + 24) as i64;
        let mut slot_var1_56_i64 = var17 as i64;
        let mut slot_var1_32_i64 = (var3.wrapping_shl(32 as u32) | 0) as i64;
        let mut slot_var1_16_i64 = (var2.wrapping_shl(32 as u32) | 0) as i64;
        let var18 = self.map_new_val(env, 1048820, 8, var1, 8);
        self.global0 = var1.wrapping_sub(-64);
        var18
    }
    fn func57(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32);
        self.global0 = var2;
        let mut __exit_label0: i32 = 0;
        loop {
            if (Vec::<Val>::try_from_val(env, &val_from_i64(arg1)).is_ok()) as i32 != 0 {
                    if (var3 != 16) as i32 != 0 {
                        mstore64!(var2.wrapping_add(16).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                        var3 = var3.wrapping_add(8);
                        continue;
                    }
                let var6 = 0 /* TODO: vec_unpack_to_linear_memory */
                var6;
                let mut slot_var2_16_i64 = mload64!(var2 as usize + 16) as i64;
                var4 = slot_var2_16_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var4)).is_ok())) as i32 != 0 {
                    break;
                }
                let mut slot_var2_24_i64 = mload64!(var2 as usize + 24) as i64;
                self.func42(env, var2, slot_var2_24_i64);
                let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
                arg1 = slot_var2_8_i64;
                let mut slot_var2_0_i32 = mload32!(var2 as usize) as i32;
                if (slot_var2_0_i32 == 0) as i32 != 0 {
                    mstore64!(arg0 as usize + 8, var4 as u64);
                    mstore64!(arg0 as usize, 0 /* False */ as u64);
                    mstore64!(arg0.wrapping_add(16) as usize, arg1 as u64);
                    __exit_label0 = 1; break 'label1;
                }
                mstore64!(arg0 as usize, 1 /* True */ as u64);
                mstore64!(arg0 as usize + 8, arg1 as u64);
                __exit_label0 = 1; break 'label1;
            }
            mstore64!(arg0 as usize, 1 /* True */ as u64);
            mstore64!(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
            __exit_label0 = 1; break 'label1;
        }
        if __exit_label0 == 0 {
        mstore64!(arg0 as usize, 1 /* True */ as u64);
        mstore64!(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
        }
        self.global0 = var2.wrapping_add(32);
    }
    fn func58(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(48);
        self.global0 = var1;
        let var7 = mload64!(arg0 as usize) as i64;
        let var8 = mload64!(arg0.wrapping_add(8) as usize) as i64;
        let var9 = self.func52(env, var7, var8);
        var2 = var9;
        let var10 = mload64!(arg0 as usize + 16) as i64;
        let var11 = mload64!(arg0.wrapping_add(24) as usize) as i64;
        let var12 = self.func52(env, var10, var11);
        var3 = var12;
        let var13 = mload64!(arg0 as usize + 32) as i64;
        let var14 = mload64!(arg0.wrapping_add(40) as usize) as i64;
        let var15 = self.func52(env, var13, var14);
        var4 = var15;
        let var16 = mload64!(arg0 as usize + 48) as i64;
        let var17 = mload64!(arg0.wrapping_add(56) as usize) as i64;
        let var18 = self.func52(env, var16, var17);
        var5 = var18;
        let var19 = mload64!(arg0 as usize + 64) as i64;
        let var20 = mload64!(arg0.wrapping_add(72) as usize) as i64;
        let var21 = self.func52(env, var19, var20);
        let mut slot_var1_40_i64 = var21 as i64;
        let mut slot_var1_32_i64 = var5 as i64;
        let mut slot_var1_24_i64 = var4 as i64;
        let mut slot_var1_16_i64 = var3 as i64;
        let mut slot_var1_8_i64 = var2 as i64;
        let var22 = self.vec_new_val(env);
        self.global0 = var1.wrapping_add(48);
        var22
    }
    fn func60(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(48);
        self.global0 = var1;
        let var6 = mload64!(arg0 as usize) as i64;
        var2 = var6;
        let var7 = mload64!(arg0 as usize + 8) as i64;
        var3 = var7;
        let var8 = mload64!(arg0.wrapping_add(16) as usize) as i64;
        let var9 = self.func61(env, var8);
        var4 = var9;
        let var10 = mload64!(arg0.wrapping_add(24) as usize) as i64;
        let var11 = self.func61(env, var10);
        let mut slot_var1_40_i64 = var11 as i64;
        let mut slot_var1_32_i64 = var4 as i64;
        let mut slot_var1_24_i64 = var3 as i64;
        let var12 = self.vec_new_val(env);
        let mut slot_var1_16_i64 = var12 as i64;
        let mut slot_var1_8_i64 = var2 as i64;
        arg0 = 0;
        let var13: i64;
        loop {
            let var14: i64;
            if (arg0 == 16) as i32 != 0 {
                arg0 = 0;
                    if (arg0 != 16) as i32 != 0 {
                        let var15 = mload64!(var1.wrapping_add(8).wrapping_add(arg0) as usize) as i64;
                        mstore64!(var1.wrapping_add(24).wrapping_add(arg0) as usize, var15 as u64);
                        arg0 = arg0.wrapping_add(8);
                        continue;
                    }
                let var16 = self.vec_new_val(env);
                self.global0 = var1.wrapping_add(48);
                var14 = var16;
            }
            else {
                mstore64!(var1.wrapping_add(24).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
                arg0 = arg0.wrapping_add(8);
                continue;
                // There should've been an expression value here, but this may be unreachable
                unreachable!();
            }
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func61(&mut self, env: &Env, mut arg0: i64) -> i64 {
        if (arg0 as u64 <= 72057594037927935 as u64) as i32 != 0 {
            return arg0.wrapping_shl(0 as u32) | 0;
        }
        let var1 = val_to_i64(Val::from_u64(arg0 as u64))
        var1
    }
    fn map_new_val(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
        if (arg1 != arg3) as i32 != 0 {
            unreachable!();
        }
        let var4 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
        var4
    }
    fn func63(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16);
        self.global0 = var1;
        let mut slot_var1_0_i64 = arg0 as i64;
        var3 = 0 /* Void */;
        var2 = 1;
            if var2 != 0 {
                var2 = var2.wrapping_sub(1);
                var3 = arg0;
                continue;
            }
        let mut slot_var1_8_i64 = var3 as i64;
        let var5 = self.vec_new_val(env);
        self.global0 = var1.wrapping_add(16);
        var5
    }
    fn func64(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        loop {
            if (arg1 as u32 > 9 as u32) as i32 != 0 {
                break;
            }
            var3 = arg1;
            var4 = arg0;
            'label1: loop {
                if var3 != 0 {
                    let var6: i32;
                    loop {
                        let var7 = mload8!(var4 as usize) as i32;
                        var2 = var7;
                        let var8 = 1;
                        if (var2 == 95) as i32 != 0 {
                            var6 = var8;
                            break;
                        }
                        var8;
                        if ((var2.wrapping_sub(48) & 255) as u32 >= 10 as u32) as i32 != 0 {
                            if ((var2.wrapping_sub(65) & 255) as u32 >= 26 as u32) as i32 != 0 {
                                if ((var2.wrapping_sub(97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                    break;
                                }
                                var6 = var2.wrapping_sub(59);
                                break;
                            }
                            var6 = var2.wrapping_sub(53);
                            break;
                        }
                        var6 = var2.wrapping_sub(46);
                    }
                    var5 = var6 as u32 as i64 & 255 | var5.wrapping_shl(0 as u32);
                    var3 = var3.wrapping_sub(1);
                    var4 = var4.wrapping_add(1);
                    continue 'label1;
                }
                break;
            }
            return var5.wrapping_shl(0 as u32) | 0 /* Symbol() */;
        }
        let var9 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
        var9
    }
    fn func65(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        {
            let var2 = mload8!(arg0 as usize + 160) as i32;
            if (var2 == 0) as i32 != 0 {
                break;
            }
            let var3 = self.func66(env);
            let var4 = mload64!(arg0 as usize + 144) as i64;
            if ((var3 as u64) < var4 as u64) as i32 != 0 {
                break;
            }
            let var5 = mload64!(arg0 as usize) as i64;
            let var6 = mload64!(arg0 as usize + 16) as i64;
            let var7 = mload64!(arg0.wrapping_add(8) as usize) as i64;
            let var8 = mload64!(arg0.wrapping_add(24) as usize) as i64;
            var1 = (var5 ^ var6 | var7 ^ var8 == 0) as i32;
        }
        var1
    }
    fn func66(&mut self, env: &Env) -> i64 {
        let mut var0: i64 = 0;
        let mut var1: i32 = 0;
        let var2 = env.ledger().timestamp() as i64
        var0 = var2;
        var1 = var0 as i32 & 255;
        if (var1 != 64) as i32 != 0 {
            if (var1 == 6) as i32 != 0 {
                return (var0 as u64).wrapping_shr(0 as u32) as i64;
            }
            unreachable!();
        }
        let var3 = val_from_i64(var0).as_u64().unwrap_or(0) as i64
        var3
    }
    fn func67(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64, mut arg7: i64, mut arg8: i64) {
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var9 = var13.wrapping_sub(192);
        self.global0 = var9;
        let mut __exit_label0: i32 = 0;
        loop {
            var12 = arg4 ^ arg6;
            if ((var12 | arg3 ^ arg5 == 0) as i32 == 0) as i32 != 0 {
                if ((arg1 ^ arg7 | arg2 ^ arg8 == 0) as i32 == 0) as i32 != 0 {
                    let mut slot_var9_188_i32 = 0 as i32;
                    self.func68(env, var9.wrapping_add(168), arg7, arg8, 20000000, 0 /* False */, var9.wrapping_add(188));
                    let var15 = mload64!(var9.wrapping_add(176) as usize) as i64;
                    arg7 = var15;
                    let mut slot_var9_168_i64 = mload64!(var9 as usize + 168) as i64;
                    arg8 = slot_var9_168_i64;
                    let var16 = self.func69(env, arg5, arg6);
                    let var17 = self.func69(env, arg1, arg2);
                    let var18 = val_to_i64(I256::try_from_val(env, &val_from_i64(var16)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var17)).unwrap()).into_val(env))
                    let var19 = self.func69(env, 10000000, 0 /* False */);
                    let var20 = val_to_i64(I256::try_from_val(env, &val_from_i64(var18)).unwrap().div(&I256::try_from_val(env, &val_from_i64(var19)).unwrap()).into_val(env))
                    var12 = var20;
                    let var21 = self.func69(env, arg3, arg4);
                    let var22 = self.func69(env, arg5, arg6);
                    let var23 = val_to_i64(I256::try_from_val(env, &val_from_i64(var21)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(var22)).unwrap()).into_val(env))
                    let var24 = self.func69(env, arg1, arg2);
                    let var25 = val_to_i64(I256::try_from_val(env, &val_from_i64(var23)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var24)).unwrap()).into_val(env))
                    let var26 = self.func69(env, arg1, arg2);
                    let var27 = val_to_i64(I256::try_from_val(env, &val_from_i64(var25)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var26)).unwrap()).into_val(env))
                    arg1 = var27;
                    if slot_var9_188_i32 != 0 {
                        __exit_label0 = 1; break 'label1;
                    }
                    let var28 = self.func69(env, arg8, arg7);
                    let var29 = val_to_i64(I256::try_from_val(env, &val_from_i64(arg1)).unwrap().div(&I256::try_from_val(env, &val_from_i64(var28)).unwrap()).into_val(env))
                    arg1 = var29;
                    self.func70(env, var9.wrapping_add(144), var12);
                    let var31 = mload64!(var9.wrapping_add(160) as usize) as i64;
                    let mut slot_var9_152_i64 = mload64!(var9 as usize + 152) as i64;
                    arg3 = slot_var9_152_i64;
                    let mut slot_var9_144_i32 = mload32!(var9 as usize + 144) as i32;
                    var10 = slot_var9_144_i32;
                    self.func70(env, var9.wrapping_add(120), arg1);
                    arg1 = { let a = var31; let b = 0 /* False */; if var10 != 0 { a } else { b } };
                    let var33 = mload64!(var9.wrapping_add(136) as usize) as i64;
                    let mut slot_var9_120_i32 = mload32!(var9 as usize + 120) as i32;
                    var11 = slot_var9_120_i32;
                    arg2 = { let a = var33; let b = 0 /* False */; if var11 != 0 { a } else { b } };
                    arg3 = { let a = arg3; let b = 0 /* False */; if var10 != 0 { a } else { b } };
                    let mut slot_var9_128_i64 = mload64!(var9 as usize + 128) as i64;
                    arg4 = arg3.wrapping_add({ let a = slot_var9_128_i64; let b = 0 /* False */; if var11 != 0 { a } else { b } });
                    arg3 = (((arg4 as u64) < arg3 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg2));
                    if ((arg1 ^ arg2 ^ 18446744073709551615) & (arg1 ^ arg3) >= 0 /* False */) as i32 != 0 {
                        break;
                    }
                    __exit_label0 = 1; break 'label1;
                }
                let mut slot_var9_116_i32 = 0 as i32;
                self.func68(env, var9.wrapping_add(96), arg5, arg6, arg1, arg2, var9.wrapping_add(116));
                if slot_var9_116_i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                arg7 = arg4.wrapping_sub(arg6).wrapping_sub(((arg3 as u64) < arg5 as u64) as i32 as u32 as i64);
                if (var12 & (arg4 ^ arg7) < 0 /* False */) as i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                let var35 = mload64!(var9.wrapping_add(104) as usize) as i64;
                arg4 = var35;
                let mut slot_var9_96_i64 = mload64!(var9 as usize + 96) as i64;
                arg6 = slot_var9_96_i64;
                let mut slot_var9_92_i32 = 0 as i32;
                self.func68(env, var9.wrapping_add(72), arg3.wrapping_sub(arg5), arg7, arg1, arg2, var9.wrapping_add(92));
                if slot_var9_92_i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                let mut slot_var9_72_i64 = mload64!(var9 as usize + 72) as i64;
                let var37 = mload64!(var9.wrapping_add(80) as usize) as i64;
                self.func71(env, var9.wrapping_add(56), slot_var9_72_i64, var37, 0 /* Void */, 0 /* False */);
                let var39 = mload64!(var9.wrapping_sub(-64) as usize) as i64;
                arg1 = var39;
                let var40 = arg1;
                let mut slot_var9_56_i64 = mload64!(var9 as usize + 56) as i64;
                arg2 = arg6.wrapping_add(slot_var9_56_i64);
                arg1 = (((arg2 as u64) < arg6 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg4));
                if ((arg4 ^ var40 ^ 18446744073709551615) & (arg4 ^ arg1) < 0 /* False */) as i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                self.func71(env, var9.wrapping_add(40), arg2, arg1, 10000000, 0 /* False */);
                let var42 = mload64!(var9.wrapping_add(48) as usize) as i64;
                arg3 = var42;
                let mut slot_var9_40_i64 = mload64!(var9 as usize + 40) as i64;
                arg4 = slot_var9_40_i64;
                break;
            }
            let mut slot_var9_36_i32 = 0 as i32;
            self.func68(env, var9.wrapping_add(16), arg1, arg2, arg3, arg4, var9.wrapping_add(36));
            if slot_var9_36_i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            let mut slot_var9_16_i64 = mload64!(var9 as usize + 16) as i64;
            let var44 = mload64!(var9.wrapping_add(24) as usize) as i64;
            self.func71(env, var9, slot_var9_16_i64, var44, 10000000, 0 /* False */);
            let var46 = mload64!(var9.wrapping_add(8) as usize) as i64;
            arg3 = var46;
            let mut slot_var9_0_i64 = mload64!(var9 as usize) as i64;
            arg4 = slot_var9_0_i64;
        }
        if __exit_label0 == 0 {
        mstore64!(arg0 as usize, arg4 as u64);
        mstore64!(arg0 as usize + 8, arg3 as u64);
        self.global0 = var9.wrapping_add(192);
        return;
        }
        unreachable!();
    }
    fn func68(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let var15 = self.global0;
        var7 = var15.wrapping_sub(32);
        self.global0 = var7;
        var9 = var7.wrapping_add(8);
        let var16 = self.global0;
        var8 = var16.wrapping_sub(32);
        self.global0 = var8;
        let var17: i32;
        loop {
            let var18 = 0;
            if (arg1 | arg2 == 0) as i32 | (arg3 | arg4 == 0) as i32 != 0 {
                var17 = var18;
                break;
            }
            var18;
            var12 = (arg2 < 0 /* False */) as i32;
            var13 = { let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var12 != 0 { a } else { b } };
            var6 = (arg4 < 0 /* False */) as i32;
            var14 = { let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var6 != 0 { a } else { b } };
            arg3 = { let a = 0 /* False */.wrapping_sub(arg4.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg4; if var6 != 0 { a } else { b } };
            let var19 = self.global0;
            var6 = var19.wrapping_sub(96);
            self.global0 = var6;
            var11 = var8.wrapping_add(8);
            let var20: i64;
            loop {
                arg1 = { let a = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg2; if var12 != 0 { a } else { b } };
                if ((arg1 == 0) as i32 == 0) as i32 != 0 {
                    if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                        self.func100(env, var6.wrapping_add(80), var14, arg3, var13, arg1);
                        let var22 = mload64!(var6.wrapping_add(88) as usize) as i64;
                        arg1 = var22;
                        var10 = 1;
                        let mut slot_var6_80_i64 = mload64!(var6 as usize + 80) as i64;
                        var20 = slot_var6_80_i64;
                        break;
                    }
                    self.func100(env, var6.wrapping_sub(-64), var13, 0 /* False */, var14, arg3);
                    self.func100(env, var6.wrapping_add(48), arg1, 0 /* False */, var14, arg3);
                    let var25 = mload64!(var6.wrapping_add(56) as usize) as i64;
                    let var26 = mload64!(var6.wrapping_add(72) as usize) as i64;
                    arg3 = var26;
                    let mut slot_var6_48_i64 = mload64!(var6 as usize + 48) as i64;
                    arg1 = arg3.wrapping_add(slot_var6_48_i64);
                    var10 = (var25 != 0 /* False */) as i32 | ((arg1 as u64) < arg3 as u64) as i32;
                    let mut slot_var6_64_i64 = mload64!(var6 as usize + 64) as i64;
                    var20 = slot_var6_64_i64;
                    break;
                }
                if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                    self.func100(env, var6.wrapping_add(32), var14, 0 /* False */, var13, arg1);
                    self.func100(env, var6.wrapping_add(16), arg3, 0 /* False */, var13, arg1);
                    let var29 = mload64!(var6.wrapping_add(24) as usize) as i64;
                    let var30 = mload64!(var6.wrapping_add(40) as usize) as i64;
                    arg3 = var30;
                    let mut slot_var6_16_i64 = mload64!(var6 as usize + 16) as i64;
                    arg1 = arg3.wrapping_add(slot_var6_16_i64);
                    var10 = (var29 != 0 /* False */) as i32 | ((arg1 as u64) < arg3 as u64) as i32;
                    let mut slot_var6_32_i64 = mload64!(var6 as usize + 32) as i64;
                    var20 = slot_var6_32_i64;
                    break;
                }
                self.func100(env, var6, var14, arg3, var13, arg1);
                let var32 = mload64!(var6.wrapping_add(8) as usize) as i64;
                arg1 = var32;
                let mut slot_var6_0_i64 = mload64!(var6 as usize) as i64;
                var20 = slot_var6_0_i64;
            }
            let mut slot_var11_0_i64 = var20 as i64;
            mstore8!(var11 as usize + 16, var10 as u8);
            let mut slot_var11_8_i64 = arg1 as i64;
            self.global0 = var6.wrapping_add(96);
            let var33 = mload64!(var8.wrapping_add(16) as usize) as i64;
            var14 = var33;
            let mut slot_var8_8_i64 = mload64!(var8 as usize + 8) as i64;
            var13 = slot_var8_8_i64;
            let var34 = mload8!(var8 as usize + 24) as i32;
            var6 = var34;
            let mut __exit_label2: i32 = 0;
            loop {
                arg2 = arg2 ^ arg4;
                if (arg2 >= 0 /* False */) as i32 != 0 {
                    if (arg2 ^ var14 >= 0 /* False */) as i32 != 0 {
                        break;
                    }
                    var17 = 1;
                    break;
                }
                arg1 = 0 /* False */.wrapping_sub(var13);
                var14 = 0 /* False */.wrapping_sub(var14.wrapping_add((var13 != 0 /* False */) as i32 as u32 as i64));
                if (arg2 ^ var14 < 0 /* False */) as i32 != 0 {
                    __exit_label2 = 1; break 'label3;
                }
                var13 = arg1;
            }
            if __exit_label2 == 0 {
            var17 = var6 & 1;
            break;
            }
            var13 = arg1;
            var17 = 1;
        }
        var6 = var17;
        let mut slot_var9_0_i64 = var13 as i64;
        mstore8!(var9 as usize + 16, var6 as u8);
        let mut slot_var9_8_i64 = var14 as i64;
        self.global0 = var8.wrapping_add(32);
        let var35 = mload64!(var7.wrapping_add(16) as usize) as i64;
        arg1 = var35;
        let mut slot_var7_8_i64 = mload64!(var7 as usize + 8) as i64;
        arg2 = slot_var7_8_i64;
        let var36 = mload8!(var7 as usize + 24) as i32;
        mstore32!(arg5 as usize, (var36 & 1) as u32);
        mstore64!(arg0 as usize + 8, arg1 as u64);
        mstore64!(arg0 as usize, arg2 as u64);
        self.global0 = var7.wrapping_add(32);
    }
    fn func69(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        let mut slot_var2_8_i64 = (arg0.wrapping_shl(56 as u32) | (arg0 & 65280).wrapping_shl(40 as u32) | (arg0 & 16711680).wrapping_shl(24 as u32) | (arg0 & 4278190080).wrapping_shl(0 as u32) | (arg0 as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (arg0 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (arg0 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (arg0 as u64).wrapping_shr(56 as u32) as i64) as i64;
        let mut slot_var2_0_i64 = (arg1.wrapping_shl(56 as u32) | (arg1 & 65280).wrapping_shl(40 as u32) | (arg1 & 16711680).wrapping_shl(24 as u32) | (arg1 & 4278190080).wrapping_shl(0 as u32) | (arg1 as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (arg1 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (arg1 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (arg1 as u64).wrapping_shr(56 as u32) as i64) as i64;
        let var4 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
        arg0 = var4;
        let var5 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
        let var6 = { let mut b = Bytes::from_val(env, &val_from_i64(var5)); b.append(&Bytes::from_val(env, &val_from_i64(arg0))); val_to_i64(b.into_val(env)) }
        let var7 = val_to_i64(I256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var6))).into_val(env))
        self.global0 = var2.wrapping_add(16);
        var7
    }
    fn func70(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(32);
        self.global0 = var5;
        let var8 = val_to_i64(I256::try_from_val(env, &val_from_i64(arg1)).unwrap().to_be_bytes().into_val(env))
        arg1 = var8;
        let var9 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(0 as u32..16 as u32).into_val(env))
        self.func104(env, var5.wrapping_add(15), var9);
        {
            let var11 = mload8!(var5 as usize + 15) as i32;
            if var11 != 0 {
                break;
            }
            var6 = var5.wrapping_add(24);
            let mut slot_var6_0_i64 = mload64!(var6 as usize) as i64;
            var3 = slot_var6_0_i64;
            let mut slot_var5_16_i64 = mload64!(var5 as usize + 16) as i64;
            var4 = slot_var5_16_i64;
            let var12 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(16 as u32..32 as u32).into_val(env))
            self.func104(env, var5.wrapping_add(15), var12);
            let var14 = mload8!(var5 as usize + 15) as i32;
            if var14 != 0 {
                break;
            }
            arg1 = slot_var6_0_i64;
            var2 = slot_var5_16_i64;
            var2 = var2.wrapping_shl(56 as u32) | (var2 & 65280).wrapping_shl(40 as u32) | (var2 & 16711680).wrapping_shl(24 as u32) | (var2 & 4278190080).wrapping_shl(0 as u32) | (var2 as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (var2 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (var2 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (var2 as u64).wrapping_shr(56 as u32) as i64;
            mstore64!(arg0.wrapping_add(16) as usize, var2 as u64);
            mstore64!(arg0 as usize + 8, (arg1.wrapping_shl(56 as u32) | (arg1 & 65280).wrapping_shl(40 as u32) | (arg1 & 16711680).wrapping_shl(24 as u32) | (arg1 & 4278190080).wrapping_shl(0 as u32) | (arg1 as u64).wrapping_shr(0 as u32) as i64 & 4278190080 | (arg1 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (arg1 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (arg1 as u64).wrapping_shr(56 as u32) as i64) as u64);
            mstore64!(arg0 as usize, ((var3 | var4 == 0) as i32 & (var2 >= 0 /* False */) as i32 | (var3 & var4 == 18446744073709551615) as i32 & (var2 < 0 /* False */) as i32) as u32 as i64 as u64);
            self.global0 = var5.wrapping_add(32);
            return;
        }
        unreachable!();
    }
    fn func71(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var14 = var17.wrapping_sub(32);
        self.global0 = var14;
        var13 = (arg2 < 0 /* False */) as i32;
        var5 = { let a = 0 /* False */.wrapping_sub(arg1); let b = arg1; if var13 != 0 { a } else { b } };
        arg1 = { let a = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64)); let b = arg2; if var13 != 0 { a } else { b } };
        var12 = (arg4 < 0 /* False */) as i32;
        var6 = { let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var12 != 0 { a } else { b } };
        let var18 = self.global0;
        var13 = var18.wrapping_sub(32);
        self.global0 = var13;
        loop {
            let mut __exit_label1: i32 = 0;
            loop {
                let mut __exit_label3: i32 = 0;
                loop {
                    arg3 = { let a = 0 /* False */.wrapping_sub(arg4.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg4; if var12 != 0 { a } else { b } };
                    if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                        if (arg1 == 0) as i32 | ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = ((arg1 as u64) < arg3 as u64) as i32; if (arg1 == arg3) as i32 != 0 { a } else { b } }) != 0 {
                            break;
                        }
                        var12 = arg3.leading_zeros() as i64 as i32;
                        var15 = arg1.leading_zeros() as i64 as i32;
                        if ((var12 as u32) < var15 as u32) as i32 != 0 {
                            break;
                        }
                        var12 = var12.wrapping_sub(var15);
                        if (var12 as u32 >= 128 as u32) as i32 != 0 {
                            break;
                        }
                        self.func101(env, var13.wrapping_add(16), var6, arg3, var12);
                        var11 = 1 /* True */.wrapping_shl(var12 as u32 as i64 as u32);
                        let var20 = mload64!(var13.wrapping_add(24) as usize) as i64;
                        var8 = var20;
                        let mut slot_var13_16_i64 = mload64!(var13 as usize + 16) as i64;
                        var9 = slot_var13_16_i64;
                        loop {
                            var7 = arg1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                            if (var7 >= 0 /* False */) as i32 != 0 {
                                var10 = var10 | var11;
                                var5 = var5.wrapping_sub(var9);
                                if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (arg3 as u64 > var7 as u64) as i32; if (arg3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                    __exit_label1 = 1; break 'label2;
                                }
                                arg1 = var7;
                            }
                            var9 = var8.wrapping_shl(63 as u32) | (var9 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            var11 = (var11 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            var8 = (var8 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            continue;
                        }
                        unreachable!();
                    }
                    let mut __exit_label5: i32 = 0;
                    loop {
                        let mut __exit_label7: i32 = 0;
                        loop {
                            if ((arg1 == 0) as i32 == 0) as i32 != 0 {
                                if ((arg1 as u64) < var6 as u64) as i32 != 0 {
                                    break;
                                }
                                if (arg1 == var6) as i32 != 0 {
                                    __exit_label7 = 1; break 'label8;
                                }
                                var11 = (arg1 as u64 / var6 as u64) as i64;
                                var7 = arg1.wrapping_sub(var11.wrapping_mul(var6));
                                if (var6 as u64 >= 4294967296 as u64) as i32 != 0 {
                                    break;
                                }
                                arg1 = var7.wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64;
                                let var21 = arg1;
                                arg1 = (arg1 as u64 / var6 as u64) as i64;
                                arg3 = var5 & 4294967295 | var21.wrapping_sub(arg1.wrapping_mul(var6)).wrapping_shl(32 as u32);
                                let var22 = arg3;
                                arg3 = (arg3 as u64 / var6 as u64) as i64;
                                var5 = var22.wrapping_sub(var6.wrapping_mul(arg3));
                                var10 = arg1.wrapping_shl(32 as u32) | arg3;
                                var11 = (arg1 as u64).wrapping_shr(32 as u32) as i64 | var11;
                                var7 = 0 /* False */;
                                break;
                            }
                            var10 = (var5 as u64 / var6 as u64) as i64;
                            var5 = var5.wrapping_sub(var10.wrapping_mul(var6));
                            __exit_label3 = 1; break 'label4;
                        }
                        if __exit_label7 == 0 {
                        var15 = arg1.leading_zeros() as i64 as i32;
                        var16 = var6.leading_zeros() as i64 as i32;
                        if ((var15 as u32) < var16 as u32) as i32 != 0 {
                            break;
                        }
                        var12 = 63;
                        if (var15 != var16) as i32 != 0 {
                            var12 = var15.wrapping_sub(var16);
                            if (var12 as u32 >= 65 as u32) as i32 != 0 {
                                break;
                            }
                            var12 = 64.wrapping_sub(var12);
                        }
                        self.func101(env, var13, var6, arg3, var12);
                        var7 = 1 /* True */.wrapping_shl(var12 as u32 as i64 as u32);
                        let var24 = mload64!(var13.wrapping_add(8) as usize) as i64;
                        var8 = var24;
                        let mut slot_var13_0_i64 = mload64!(var13 as usize) as i64;
                        var9 = slot_var13_0_i64;
                        loop {
                            arg3 = arg1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                            if (arg3 >= 0 /* False */) as i32 != 0 {
                                var5 = var5.wrapping_sub(var9);
                                var10 = var7 | var10;
                                if (arg3 == 0) as i32 != 0 {
                                    __exit_label5 = 1; break 'label6;
                                }
                                arg1 = arg3;
                            }
                            var9 = var8.wrapping_shl(63 as u32) | (var9 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            var7 = (var7 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            var8 = (var8 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            continue;
                        }
                        unreachable!();
                        }
                        var10 = (var5 as u64 / arg1 as u64) as i64;
                        var5 = var5.wrapping_sub(var10.wrapping_mul(arg1));
                        var11 = 1 /* True */;
                        break;
                    }
                    if __exit_label5 == 0 {
                    if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (arg3 as u64 > var7 as u64) as i32; if (arg3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                        break;
                    }
                    var8 = arg3.wrapping_shl(63 as u32) | (var6 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                    var9 = var6.wrapping_shl(63 as u32);
                    arg1 = 9223372036854775808;
                    'label9: loop {
                        loop {
                            arg3 = var7.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                            if (arg3 >= 0 /* False */) as i32 != 0 {
                                var5 = var5.wrapping_sub(var9);
                                var10 = arg1 | var10;
                                if (arg3 == 0) as i32 != 0 {
                                    break;
                                }
                                var7 = arg3;
                            }
                            var9 = var8.wrapping_shl(63 as u32) | (var9 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            arg1 = (arg1 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            var8 = (var8 as u64).wrapping_shr(1 /* True */ as u32) as i64;
                            continue 'label9;
                        }
                        break;
                    }
                    arg1 = (var5 as u64 / var6 as u64) as i64;
                    var10 = arg1 | var10;
                    var5 = var5.wrapping_sub(arg1.wrapping_mul(var6));
                    var7 = 0 /* False */;
                    break;
                    }
                    arg1 = (var5 as u64 / var6 as u64) as i64;
                    var10 = arg1 | var10;
                    var5 = var5.wrapping_sub(arg1.wrapping_mul(var6));
                    __exit_label3 = 1; break 'label4;
                }
                if __exit_label3 == 0 {
                unreachable!();
                }
                var7 = 0 /* False */;
                __exit_label1 = 1; break 'label2;
            }
            if __exit_label1 == 0 {
            var7 = arg1;
            }
            var11 = 0 /* False */;
        }
        let mut slot_var14_16_i64 = var5 as i64;
        let mut slot_var14_0_i64 = var10 as i64;
        mstore64!(var14.wrapping_add(24) as usize, var7 as u64);
        let mut slot_var14_8_i64 = var11 as i64;
        self.global0 = var13.wrapping_add(32);
        let var25 = mload64!(var14.wrapping_add(8) as usize) as i64;
        arg1 = var25;
        arg3 = slot_var14_0_i64;
        var13 = (arg2 ^ arg4 < 0 /* False */) as i32;
        mstore64!(arg0 as usize, ({ let a = 0 /* False */.wrapping_sub(arg3); let b = arg3; if var13 != 0 { a } else { b } }) as u64);
        mstore64!(arg0 as usize + 8, ({ let a = 0 /* False */.wrapping_sub(arg1.wrapping_add((arg3 != 0 /* False */) as i32 as u32 as i64)); let b = arg1; if var13 != 0 { a } else { b } }) as u64);
        self.global0 = var14.wrapping_add(32);
    }
    fn func72(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) -> i64 {
        let mut var3: i32 = 0;
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
        let var15 = self.global0;
        var3 = var15.wrapping_sub(80);
        self.global0 = var3;
        let mut __exit_label0: i32 = 0;
        loop {
            let var16 = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64
            var5 = var16;
            if (var5 as u64 <= 4294967295 as u64) as i32 != 0 {
                break;
            }
            self.func71(env, var3.wrapping_add(32), arg1, arg2, (var5 as u64).wrapping_shr(32 as u32) as i64, 0 /* False */);
            let var18 = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64
            var5 = var18;
            let mut slot_var3_28_i32 = 0 as i32;
            let mut slot_var3_32_i64 = mload64!(var3 as usize + 32) as i64;
            var13 = slot_var3_32_i64;
            let var19 = mload64!(var3.wrapping_add(40) as usize) as i64;
            var9 = var19;
            self.func68(env, var3.wrapping_add(8), var13, var9, (var5 as u64).wrapping_shr(32 as u32) as i64, 0 /* False */, var3.wrapping_add(28));
            if slot_var3_28_i32 != 0 {
                break;
            }
            let var21 = mload64!(var3.wrapping_add(16) as usize) as i64;
            var5 = var21;
            let var22 = var5;
            let var23 = var5;
            let mut slot_var3_8_i64 = mload64!(var3 as usize + 8) as i64;
            var5 = slot_var3_8_i64;
            var7 = arg2.wrapping_sub(var23).wrapping_sub(((arg1 as u64) < var5 as u64) as i32 as u32 as i64);
            if ((arg2 ^ var22) & (arg2 ^ var7) < 0 /* False */) as i32 != 0 {
                break;
            }
            var8 = arg1.wrapping_sub(var5);
            let var24 = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64
            var14 = (var24 as u64).wrapping_shr(32 as u32) as i64;
            var4 = var3.wrapping_sub(-64);
            arg1 = 0;
            arg2 = 0 /* False */;
            var5 = arg0;
            loop {
                if (arg2 == var14) as i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                let var25 = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).keys().get_unchecked(arg1 as u32))
                var10 = var25;
                let var26 = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).values().get_unchecked(arg1 as u32))
                var6 = var26;
                if (arg2 == 4294967295) as i32 | (var10 & 255 != 0) as i32 != 0 {
                    break;
                }
                self.func73(env, var3.wrapping_add(48), var6);
                let mut slot_var3_48_i64 = mload64!(var3 as usize + 48) as i64;
                if ((slot_var3_48_i64 == 0) as i32 == 0) as i32 != 0 {
                    break;
                }
                let mut slot_var4_0_i64 = mload64!(var4 as usize) as i64;
                var11 = slot_var4_0_i64;
                let mut slot_var3_56_i64 = mload64!(var3 as usize + 56) as i64;
                var6 = slot_var3_56_i64;
                var12 = var6.wrapping_add(var13);
                var6 = (((var12 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var9.wrapping_add(var11));
                if ((var11 ^ var9 ^ 18446744073709551615) & (var11 ^ var6) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let var28 = var7;
                var8 = var8.wrapping_add(var12);
                var7 = (((var8 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_add(var7));
                if ((var6 ^ var28 ^ 18446744073709551615) & (var6 ^ var7) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let mut slot_var3_72_i64 = mload64!(var3 as usize + 72) as i64;
                let var29 = self.func61(env, slot_var3_72_i64);
                var6 = var29;
                let var30 = self.func52(env, var8, var7);
                slot_var3_56_i64 = var30 as i64;
                slot_var3_48_i64 = var6 as i64;
                arg1 = arg1.wrapping_add(4294967296);
                arg2 = arg2.wrapping_add(1 /* True */);
                var8 = 0 /* False */;
                let var31 = self.map_new_val(env, 1049192, 2, var3.wrapping_add(48), 2);
                let var32 = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(var5)); m.set(val_from_i64(var10 & 18446744069414584320 | 0), val_from_i64(var31)); val_to_i64(m.into_val(env)) }
                var5 = var32;
                var7 = 0 /* False */;
                continue;
            }
            unreachable!();
        }
        if __exit_label0 == 0 {
        unreachable!();
        }
        self.global0 = var3.wrapping_add(80);
        var5
    }
    fn func73(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64);
        self.global0 = var2;
            if (var3 != 16) as i32 != 0 {
                mstore64!(var2.wrapping_add(24).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue;
            }
        let mut __exit_label1: i32 = 0;
        loop {
            if (Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok()) as i32 != 0 {
                self.func48(env, arg1, 1049192, 2, var2.wrapping_add(24), 2);
                let mut slot_var2_24_i64 = mload64!(var2 as usize + 24) as i64;
                self.func42(env, var2.wrapping_add(8), slot_var2_24_i64);
                let mut slot_var2_8_i32 = mload32!(var2 as usize + 8) as i32;
                if slot_var2_8_i32 != 0 {
                    break;
                }
                let mut slot_var2_16_i64 = mload64!(var2 as usize + 16) as i64;
                arg1 = slot_var2_16_i64;
                let mut slot_var2_32_i64 = mload64!(var2 as usize + 32) as i64;
                self.func46(env, var2.wrapping_add(40), slot_var2_32_i64);
                let mut slot_var2_40_i64 = mload64!(var2 as usize + 40) as i64;
                if (slot_var2_40_i64 == 0) as i32 != 0 {
                    let var9 = mload64!(var2.wrapping_add(56) as usize) as i64;
                    var4 = var9;
                    let mut slot_var2_48_i64 = mload64!(var2 as usize + 48) as i64;
                    mstore64!(arg0 as usize + 8, slot_var2_48_i64 as u64);
                    mstore64!(arg0 as usize, 0 /* False */ as u64);
                    mstore64!(arg0.wrapping_add(16) as usize, var4 as u64);
                    mstore64!(arg0.wrapping_add(24) as usize, arg1 as u64);
                    __exit_label1 = 1; break 'label2;
                }
                mstore64!(arg0 as usize, 1 /* True */ as u64);
                __exit_label1 = 1; break 'label2;
            }
            mstore64!(arg0 as usize, 1 /* True */ as u64);
            __exit_label1 = 1; break 'label2;
        }
        if __exit_label1 == 0 {
        mstore64!(arg0 as usize, 1 /* True */ as u64);
        }
        self.global0 = var2.wrapping_sub(-64);
    }
    fn func74(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var6 = var8.wrapping_sub(48);
        self.global0 = var6;
        if (({ let a = (arg3 == 0) as i32; let b = (arg4 < 0 /* False */) as i32; if (arg4 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
            let var9 = self.storage_key_from_str(env, 1049272, 8);
            var7 = var9;
            let var10 = self.func52(env, arg3, arg4);
            let mut slot_var6_16_i64 = var10 as i64;
            let mut slot_var6_8_i64 = arg2 as i64;
            let mut slot_var6_0_i64 = arg1 as i64;
            loop {
                if (var5 == 24) as i32 != 0 {
                    var5 = 0;
                        if (var5 != 24) as i32 != 0 {
                            let var11 = mload64!(var5.wrapping_add(var6) as usize) as i64;
                            mstore64!(var6.wrapping_add(24).wrapping_add(var5) as usize, var11 as u64);
                            var5 = var5.wrapping_add(8);
                            continue;
                        }
                    let var12 = self.vec_new_val(env);
                    self.func76(env, arg0, var7, var12);
                }
                else {
                    mstore64!(var6.wrapping_add(24).wrapping_add(var5) as usize, 0 /* Void */ as u64);
                    var5 = var5.wrapping_add(8);
                    continue;
                }
                break;
            }
        }
        self.global0 = var6.wrapping_add(48);
    }
    fn storage_key_from_str(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = self.func64(env, arg0, arg1);
        var2
    }
    fn func76(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let var3 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(arg1)), Vec::<Val>::from_val(env, &val_from_i64(arg2))))
        if (var3 & 255 != 0 /* Void */) as i32 != 0 {
            unreachable!();
        }
    }
    fn func77(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32, mut arg4: i32, mut arg5: i32) {
        let mut var6: i32 = 0;
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
        let var27 = self.global0;
        var6 = var27.wrapping_sub(256);
        self.global0 = var6;
        let var28 = mload64!(arg3 as usize) as i64;
        var15 = var28;
        let var29 = mload64!(arg3.wrapping_add(8) as usize) as i64;
        var11 = var29;
        let var30 = mload64!(arg3 as usize + 96) as i64;
        var7 = var30;
        let var31 = mload64!(arg3.wrapping_add(104) as usize) as i64;
        var8 = var31;
        let var32 = mload64!(arg3 as usize + 80) as i64;
        var12 = var32;
        let var33 = mload64!(arg3.wrapping_add(88) as usize) as i64;
        var19 = var33;
        self.func67(env, var6.wrapping_add(240), var15, var11, var7, var8, var12, var19, var15, var11);
        loop {
            if (arg1 | arg2 == 0) as i32 != 0 {
                self.func78(env, arg0, 80);
                break;
            }
            let var36 = mload64!(var6.wrapping_add(248) as usize) as i64;
            var9 = var36;
            let mut slot_var6_240_i64 = mload64!(var6 as usize + 240) as i64;
            var20 = slot_var6_240_i64;
            self.func71(env, var6.wrapping_add(224), arg1, arg2, 10000, 0 /* False */);
            let mut slot_var6_220_i32 = 0 as i32;
            let mut slot_var6_224_i64 = mload64!(var6 as usize + 224) as i64;
            var13 = slot_var6_224_i64;
            let var38 = mload64!(var6.wrapping_add(232) as usize) as i64;
            var10 = var38;
            var21 = arg4 as u32 as i64;
            self.func68(env, var6.wrapping_add(200), var13, var10, var21, 0 /* False */, var6.wrapping_add(220));
            loop {
                if slot_var6_220_i32 != 0 {
                    break;
                }
                let var40 = mload64!(var6.wrapping_add(208) as usize) as i64;
                var17 = var40;
                let mut slot_var6_200_i64 = mload64!(var6 as usize + 200) as i64;
                var18 = slot_var6_200_i64;
                let mut slot_var6_196_i32 = 0 as i32;
                var26 = arg5 as u32 as i64;
                self.func68(env, var6.wrapping_add(176), var13, var10, var26, 0 /* False */, var6.wrapping_add(196));
                if slot_var6_196_i32 != 0 {
                    break;
                }
                var10 = arg2.wrapping_sub(var17).wrapping_sub(((arg1 as u64) < var18 as u64) as i32 as u32 as i64);
                if ((arg2 ^ var17) & (arg2 ^ var10) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let var42 = mload64!(var6.wrapping_add(184) as usize) as i64;
                var22 = var42;
                var16 = arg1.wrapping_sub(var18);
                let mut slot_var6_176_i64 = mload64!(var6 as usize + 176) as i64;
                var23 = slot_var6_176_i64;
                var13 = var10.wrapping_sub(var22).wrapping_sub(((var16 as u64) < var23 as u64) as i32 as u32 as i64);
                if ((var10 ^ var22) & (var10 ^ var13) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let var43 = mload64!(arg3.wrapping_add(40) as usize) as i64;
                var10 = var43;
                let var44 = mload64!(arg3 as usize + 32) as i64;
                var24 = var44;
                var16 = var16.wrapping_sub(var23);
                var25 = var24.wrapping_add(var16);
                var14 = (((var25 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_add(var10.wrapping_add(var13));
                if ((var10 ^ var13 ^ 18446744073709551615) & (var10 ^ var14) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let var45: i64;
                loop {
                    let mut __exit_label3: i32 = 0;
                    loop {
                        if ({ let a = ((var20 as u64) < var25 as u64) as i32; let b = (var9 < var14) as i32; if (var9 == var14) as i32 != 0 { a } else { b } }) != 0 {
                            var13 = var9.wrapping_sub(var10).wrapping_sub(((var20 as u64) < var24 as u64) as i32 as u32 as i64);
                            if ((var9 ^ var10) & (var9 ^ var13) < 0 /* False */) as i32 != 0 {
                                break;
                            }
                            let mut slot_var6_84_i32 = 0 as i32;
                            var16 = var20.wrapping_sub(var24);
                            self.func68(env, var6.wrapping_sub(-64), var16, var13, 10000, 0 /* False */, var6.wrapping_add(84));
                            if (slot_var6_84_i32 == 0) as i32 != 0 {
                                break;
                            }
                            break;
                        }
                        var9 = var8 ^ var19;
                        if ((var9 | var7 ^ var12 == 0) as i32 == 0) as i32 != 0 {
                            var10 = var8.wrapping_sub(var19).wrapping_sub(((var7 as u64) < var12 as u64) as i32 as u32 as i64);
                            if (var9 & (var8 ^ var10) < 0 /* False */) as i32 != 0 {
                                break;
                            }
                            let var47 = self.func69(env, var7.wrapping_sub(var12), var10);
                            var10 = var47;
                            let var48 = self.func69(env, 0 /* Void */, 0 /* False */);
                            let var49 = self.func69(env, var12, var19);
                            let var50 = val_to_i64(I256::try_from_val(env, &val_from_i64(var48)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var49)).unwrap()).into_val(env))
                            let var51 = self.func69(env, var15, var11);
                            let var52 = val_to_i64(I256::try_from_val(env, &val_from_i64(var50)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var51)).unwrap()).into_val(env))
                            var12 = var52;
                            let var53 = self.func69(env, 18446744073709551614, 18446744073709551615);
                            let var54 = self.func69(env, var15, var11);
                            let var55 = val_to_i64(I256::try_from_val(env, &val_from_i64(var53)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var54)).unwrap()).into_val(env))
                            let var56 = self.func69(env, 10000000, 0 /* False */);
                            let var57 = val_to_i64(I256::try_from_val(env, &val_from_i64(var55)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var56)).unwrap()).into_val(env))
                            let var58 = self.func69(env, var25, var14);
                            let var59 = val_to_i64(I256::try_from_val(env, &val_from_i64(var57)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var58)).unwrap()).into_val(env))
                            var7 = var59;
                            let var60 = val_to_i64(I256::try_from_val(env, &val_from_i64(var12)).unwrap().pow(2 as u32).into_val(env))
                            let var61 = self.func69(env, 0, 0 /* False */);
                            let var62 = val_to_i64(I256::try_from_val(env, &val_from_i64(var61)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var10)).unwrap()).into_val(env))
                            let var63 = val_to_i64(I256::try_from_val(env, &val_from_i64(var62)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var7)).unwrap()).into_val(env))
                            let var64 = val_to_i64(I256::try_from_val(env, &val_from_i64(var60)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(var63)).unwrap()).into_val(env))
                            var9 = var64;
                            self.func70(env, var6.wrapping_add(152), var9);
                            let mut slot_var6_152_i32 = mload32!(var6 as usize + 152) as i32;
                            if slot_var6_152_i32 != 0 {
                                var7 = 0;
                                let var66 = mload64!(var6.wrapping_add(168) as usize) as i64;
                                if (var66 < 0 /* False */) as i32 != 0 {
                                    __exit_label3 = 1; break 'label4;
                                }
                            }
                            let var67 = val_to_i64(I256::try_from_val(env, &val_from_i64(var9)).unwrap().add(&I256::try_from_val(env, &val_from_i64(1)).unwrap()).into_val(env))
                            var14 = var67;
                            var7 = var9;
                            loop {
                                loop {
                                    let var68 = val_to_i64(I256::try_from_val(env, &val_from_i64(var14)).unwrap().div(&I256::try_from_val(env, &val_from_i64(2)).unwrap()).into_val(env))
                                    var8 = var68;
                                    if ((var8 & 255 == 0) as i32 & (var7 & 255 == 0) as i32 == 0) as i32 != 0 {
                                        let var69 = { let a = val_from_i64(var8); let b = val_from_i64(var7); if a < b { -1 } else if a > b { 1 } else { 0 } }
                                        if (var69 < 0 /* False */) as i32 != 0 {
                                            break;
                                        }
                                        __exit_label3 = 1; break 'label4;
                                    }
                                    if (var8.wrapping_shr(0 as u32) >= var7.wrapping_shr(0 as u32)) as i32 != 0 {
                                        __exit_label3 = 1; break 'label4;
                                    }
                                }
                                let var70 = val_to_i64(I256::try_from_val(env, &val_from_i64(var9)).unwrap().div(&I256::try_from_val(env, &val_from_i64(var8)).unwrap()).into_val(env))
                                let var71 = val_to_i64(I256::try_from_val(env, &val_from_i64(var70)).unwrap().add(&I256::try_from_val(env, &val_from_i64(var8)).unwrap()).into_val(env))
                                var14 = var71;
                                var7 = var8;
                                continue;
                            }
                            unreachable!();
                        }
                        let mut slot_var6_124_i32 = 0 as i32;
                        self.func68(env, var6.wrapping_add(104), var25, var14, 10000000, 0 /* False */, var6.wrapping_add(124));
                        if slot_var6_124_i32 | (var7 | var8 == 0) as i32 != 0 {
                            break;
                        }
                        let mut slot_var6_104_i64 = mload64!(var6 as usize + 104) as i64;
                        var9 = slot_var6_104_i64;
                        let var73 = mload64!(var6.wrapping_add(112) as usize) as i64;
                        var12 = var73;
                        if (var9 | var12 ^ 9223372036854775808 == 0) as i32 & (var7 & var8 == 18446744073709551615) as i32 != 0 {
                            break;
                        }
                        self.func71(env, var6.wrapping_add(88), var9, var12, var7, var8);
                        let var75 = mload64!(var6.wrapping_add(96) as usize) as i64;
                        var7 = var75;
                        let mut slot_var6_88_i64 = mload64!(var6 as usize + 88) as i64;
                        var45 = slot_var6_88_i64;
                        break;
                    }
                    if __exit_label3 == 0 {
                    arg2 = var21.wrapping_add(var26);
                    arg1 = arg2.wrapping_sub(10000);
                    arg2 = (((arg2 as u64) < var21 as u64) as i32 as u32 as i64).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64).wrapping_sub(1 /* True */);
                    if (arg1 | arg2 == 0) as i32 != 0 {
                        break;
                    }
                    let mut slot_var6_64_i64 = mload64!(var6 as usize + 64) as i64;
                    var7 = slot_var6_64_i64;
                    let var76 = mload64!(var6.wrapping_add(72) as usize) as i64;
                    var8 = var76;
                    if (var7 | var8 ^ 9223372036854775808 == 0) as i32 & (arg1 & arg2 == 18446744073709551615) as i32 != 0 {
                        break;
                    }
                    self.func71(env, var6.wrapping_add(48), var7, var8, arg1, arg2);
                    let mut slot_var6_48_i64 = mload64!(var6 as usize + 48) as i64;
                    arg1 = slot_var6_48_i64;
                    let var78 = mload64!(var6.wrapping_add(56) as usize) as i64;
                    arg2 = var78;
                    if (arg1 | arg2 ^ 9223372036854775808 == 0) as i32 != 0 {
                        break;
                    }
                    self.func71(env, var6.wrapping_add(32), arg1, arg2, 18446744073709541616, 18446744073709551615);
                    let mut slot_var6_28_i32 = 0 as i32;
                    let mut slot_var6_32_i64 = mload64!(var6 as usize + 32) as i64;
                    let var80 = mload64!(var6.wrapping_add(40) as usize) as i64;
                    self.func68(env, var6.wrapping_add(8), slot_var6_32_i64, var80, var21, 0 /* False */, var6.wrapping_add(28));
                    if slot_var6_28_i32 != 0 {
                        break;
                    }
                    arg2 = 0 /* False */.wrapping_sub(arg2.wrapping_add((arg1 != 0 /* False */) as i32 as u32 as i64));
                    let var82 = mload64!(var6.wrapping_add(16) as usize) as i64;
                    var17 = var82;
                    arg1 = 0 /* False */.wrapping_sub(arg1);
                    let mut slot_var6_8_i64 = mload64!(var6 as usize + 8) as i64;
                    var18 = slot_var6_8_i64;
                    var7 = arg2.wrapping_sub(var17).wrapping_sub(((arg1 as u64) < var18 as u64) as i32 as u32 as i64);
                    if ((arg2 ^ var17) & (arg2 ^ var7) < 0 /* False */) as i32 != 0 {
                        break;
                    }
                    var8 = arg1.wrapping_sub(var18);
                    var22 = var7.wrapping_sub(var13).wrapping_sub(((var8 as u64) < var16 as u64) as i32 as u32 as i64);
                    if ((var7 ^ var13) & (var7 ^ var22) < 0 /* False */) as i32 != 0 {
                        break;
                    }
                    var23 = var8.wrapping_sub(var16);
                    var7 = var11;
                    var45 = var15;
                    break;
                    }
                    let var83 = val_to_i64(I256::try_from_val(env, &val_from_i64(var7)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(var12)).unwrap()).into_val(env))
                    let var84 = self.func69(env, 0 /* Void */, 0 /* False */);
                    let var85 = val_to_i64(I256::try_from_val(env, &val_from_i64(var84)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(var10)).unwrap()).into_val(env))
                    let var86 = val_to_i64(I256::try_from_val(env, &val_from_i64(var83)).unwrap().div(&I256::try_from_val(env, &val_from_i64(var85)).unwrap()).into_val(env))
                    self.func70(env, var6.wrapping_add(128), var86);
                    let var88 = mload64!(var6.wrapping_add(144) as usize) as i64;
                    let mut slot_var6_128_i32 = mload32!(var6 as usize + 128) as i32;
                    arg4 = slot_var6_128_i32;
                    var7 = { let a = var88; let b = 0 /* False */; if arg4 != 0 { a } else { b } };
                    let mut slot_var6_136_i64 = mload64!(var6 as usize + 136) as i64;
                    var45 = { let a = slot_var6_136_i64; let b = 0 /* False */; if arg4 != 0 { a } else { b } };
                }
                var8 = var45;
                arg4 = { let a = ((var8 as u64) < var15 as u64) as i32; let b = (var7 < var11) as i32; if (var7 == var11) as i32 != 0 { a } else { b } };
                var7 = { let a = var7; let b = var11; if arg4 != 0 { a } else { b } };
                let var89 = mload64!(arg3.wrapping_add(24) as usize) as i64;
                var11 = var89;
                let var90 = var11;
                var8 = { let a = var8; let b = var15; if arg4 != 0 { a } else { b } };
                let var91 = mload64!(arg3 as usize + 16) as i64;
                var11 = var91;
                var9 = var7.wrapping_sub(var11).wrapping_sub(((var8 as u64) < var11 as u64) as i32 as u32 as i64);
                if ((var7 ^ var90) & (var7 ^ var9) < 0 /* False */) as i32 != 0 {
                    break;
                }
                mstore64!(arg0 as usize + 64, var23 as u64);
                mstore64!(arg0 as usize + 48, var18 as u64);
                mstore64!(arg0 as usize + 32, var8.wrapping_sub(var11) as u64);
                mstore64!(arg0 as usize + 16, var16 as u64);
                mstore64!(arg0 as usize, arg1 as u64);
                mstore64!(arg0.wrapping_add(72) as usize, var22 as u64);
                mstore64!(arg0.wrapping_add(56) as usize, var17 as u64);
                mstore64!(arg0.wrapping_add(40) as usize, var9 as u64);
                mstore64!(arg0.wrapping_add(24) as usize, var13 as u64);
                mstore64!(arg0 as usize + 8, arg2 as u64);
                break;
            }
            unreachable!();
        }
        self.global0 = var6.wrapping_add(256);
    }
    fn func78(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        if (arg1 as u32 >= 16 as u32) as i32 != 0 {
            var4 = 0.wrapping_sub(arg0) & 3;
            var2 = arg0.wrapping_add(var4);
                if ((arg0 as u32) < var2 as u32) as i32 != 0 {
                    mstore8!(arg0 as usize, 0 as u8);
                    arg0 = arg0.wrapping_add(1);
                    continue;
                }
            arg0 = 8;
            'label1: loop {
                if (arg0 as u32 >= 32 as u32) as i32 != 0 {
                    loop {
                        arg1 = arg1.wrapping_sub(var4);
                        arg0 = var2.wrapping_add(arg1 & -4);
                        loop {
                            if (arg0 as u32 <= var2 as u32) as i32 != 0 {
                                break;
                            }
                            let mut slot_var2_0_i32 = var3 as i32;
                            var2 = var2.wrapping_add(4);
                            continue;
                        }
                        unreachable!();
                    }
                }
                else {
                    var3 = var3.wrapping_shl((arg0 & 24) as u32) | var3;
                    arg0 = arg0.wrapping_shl(1 as u32);
                    continue 'label1;
                }
                break;
            }
            arg1 = arg1 & 3;
        }
        arg1 = arg0.wrapping_add(arg1);
            if ((arg0 as u32) < arg1 as u32) as i32 != 0 {
                mstore8!(arg0 as usize, 0 as u8);
                arg0 = arg0.wrapping_add(1);
                continue;
            }
    }
    fn func79(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32, mut arg4: i32, mut arg5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var6 = var16.wrapping_sub(80);
        self.global0 = var6;
        loop {
            if (arg1 | arg2 == 0) as i32 != 0 {
                self.func78(env, arg0, 80);
                break;
            }
            {
                let var18 = mload64!(arg3 as usize + 16) as i64;
                var8 = var18;
                let var19 = mload64!(arg3.wrapping_add(24) as usize) as i64;
                var7 = var19;
                if ({ let a = (var8 as u64 > arg1 as u64) as i32; let b = (var7 > arg2) as i32; if (arg2 == var7) as i32 != 0 { a } else { b } }) != 0 {
                    var9 = var7.wrapping_sub(arg2).wrapping_sub((arg1 as u64 > var8 as u64) as i32 as u32 as i64);
                    if ((arg2 ^ var7) & (var7 ^ var9) < 0 /* False */) as i32 != 0 {
                        break;
                    }
                    let var20 = mload64!(arg3 as usize + 96) as i64;
                    let var21 = mload64!(arg3.wrapping_add(104) as usize) as i64;
                    let var22 = mload64!(arg3 as usize + 80) as i64;
                    let var23 = mload64!(arg3.wrapping_add(88) as usize) as i64;
                    let var24 = mload64!(arg3 as usize) as i64;
                    let var25 = mload64!(arg3.wrapping_add(8) as usize) as i64;
                    self.func67(env, var6.wrapping_sub(-64), var8.wrapping_sub(arg1), var9, var20, var21, var22, var23, var24, var25);
                    let var27 = mload64!(var6.wrapping_add(72) as usize) as i64;
                    var10 = var27;
                    let mut slot_var6_64_i64 = mload64!(var6 as usize + 64) as i64;
                    var9 = slot_var6_64_i64;
                }
                let var28 = mload64!(arg3.wrapping_add(40) as usize) as i64;
                var8 = var28;
                let var29 = var10;
                let var30 = mload64!(arg3 as usize + 32) as i64;
                var10 = var30;
                var7 = var8.wrapping_sub(var10).wrapping_sub(((var10 as u64) < var9 as u64) as i32 as u32 as i64);
                if ((var8 ^ var29) & (var8 ^ var7) < 0 /* False */) as i32 != 0 {
                    break;
                }
                var8 = var10.wrapping_sub(var9);
                self.func71(env, var6.wrapping_add(48), var8, var7, 10000, 0 /* False */);
                let mut slot_var6_44_i32 = 0 as i32;
                let mut slot_var6_48_i64 = mload64!(var6 as usize + 48) as i64;
                var11 = slot_var6_48_i64;
                let var32 = mload64!(var6.wrapping_add(56) as usize) as i64;
                var12 = var32;
                self.func68(env, var6.wrapping_add(24), var11, var12, arg4 as u32 as i64, 0 /* False */, var6.wrapping_add(44));
                if slot_var6_44_i32 != 0 {
                    break;
                }
                let var34 = mload64!(var6.wrapping_add(32) as usize) as i64;
                var9 = var34;
                let mut slot_var6_24_i64 = mload64!(var6 as usize + 24) as i64;
                var10 = slot_var6_24_i64;
                let mut slot_var6_20_i32 = 0 as i32;
                self.func68(env, var6, var11, var12, arg5 as u32 as i64, 0 /* False */, var6.wrapping_add(20));
                if slot_var6_20_i32 != 0 {
                    break;
                }
                var11 = var7.wrapping_sub(var9).wrapping_sub(((var8 as u64) < var10 as u64) as i32 as u32 as i64);
                if ((var7 ^ var9) & (var7 ^ var11) < 0 /* False */) as i32 != 0 {
                    break;
                }
                let var36 = mload64!(var6.wrapping_add(8) as usize) as i64;
                var12 = var36;
                var14 = var8.wrapping_sub(var10);
                let mut slot_var6_0_i64 = mload64!(var6 as usize) as i64;
                var13 = slot_var6_0_i64;
                var15 = var11.wrapping_sub(var12).wrapping_sub(((var14 as u64) < var13 as u64) as i32 as u32 as i64);
                if ((var11 ^ var12) & (var11 ^ var15) < 0 /* False */) as i32 != 0 {
                    break;
                }
                mstore64!(arg0 as usize + 64, var13 as u64);
                mstore64!(arg0 as usize + 48, var10 as u64);
                mstore64!(arg0 as usize + 32, var14.wrapping_sub(var13) as u64);
                mstore64!(arg0 as usize + 16, var8 as u64);
                mstore64!(arg0 as usize, arg1 as u64);
                mstore64!(arg0.wrapping_add(72) as usize, var12 as u64);
                mstore64!(arg0.wrapping_add(56) as usize, var9 as u64);
                mstore64!(arg0.wrapping_add(40) as usize, var15 as u64);
                mstore64!(arg0.wrapping_add(24) as usize, var7 as u64);
                mstore64!(arg0 as usize + 8, arg2 as u64);
            }
            unreachable!();
        }
        self.global0 = var6.wrapping_add(80);
    }
    fn func81(&mut self, env: &Env, mut arg0: i64) {
        let var1 = 0 /* TODO: fail_with_error */
        var1;
    }
    fn func83(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        loop {
            if ((arg2 as u32) < 16 as u32) as i32 != 0 {
                var3 = arg0;
                break;
            }
            var6 = 0.wrapping_sub(arg0) & 3;
            var4 = arg0.wrapping_add(var6);
            var5 = arg1;
            var3 = arg0;
                if ((var3 as u32) < var4 as u32) as i32 != 0 {
                    let var10 = mload8!(var5 as usize) as i32;
                    mstore8!(var3 as usize, var10 as u8);
                    var5 = var5.wrapping_add(1);
                    var3 = var3.wrapping_add(1);
                    continue;
                }
            var8 = arg2.wrapping_sub(var6);
            var9 = var8 & -4;
            var3 = var4.wrapping_add(var9);
            loop {
                var5 = arg1.wrapping_add(var6);
                if (var5 & 3 == 0) as i32 != 0 {
                    arg1 = var5;
                    loop {
                        if (var3 as u32 <= var4 as u32) as i32 != 0 {
                            break;
                        }
                        let var11 = mload32!(arg1 as usize) as i32;
                        let mut slot_var4_0_i32 = var11 as i32;
                        arg1 = arg1.wrapping_add(4);
                        var4 = var4.wrapping_add(4);
                        continue;
                    }
                    unreachable!();
                }
                arg2 = var5 & -4;
                arg1 = arg2.wrapping_add(4);
                var7 = var5.wrapping_shl(3 as u32);
                var6 = var7 & 24;
                var7 = 0.wrapping_sub(var7) & 24;
                let var12 = mload32!(arg2 as usize) as i32;
                arg2 = var12;
                loop {
                    if (var3 as u32 <= var4 as u32) as i32 != 0 {
                        break;
                    }
                    if var6 != 0 {
                        let var13 = arg2;
                        let var14 = mload32!(arg1 as usize) as i32;
                        arg2 = var14;
                        slot_var4_0_i32 = ((var13 as u32).wrapping_shr(var6 as u32) as i32 | arg2.wrapping_shl(var7 as u32)) as i32;
                        arg1 = arg1.wrapping_add(4);
                        var4 = var4.wrapping_add(4);
                        continue;
                    }
                    break;
                }
                unreachable!();
            }
            arg2 = var8 & 3;
            arg1 = var5.wrapping_add(var9);
        }
        arg2 = arg2.wrapping_add(var3);
            if (arg2 as u32 > var3 as u32) as i32 != 0 {
                let var15 = mload8!(arg1 as usize) as i32;
                mstore8!(var3 as usize, var15 as u8);
                arg1 = arg1.wrapping_add(1);
                var3 = var3.wrapping_add(1);
                continue;
            }
        arg0
    }
    fn func89(&mut self, env: &Env, mut arg0: i64) {
        let var1 = match 1 /* True */ { 0 => { env.storage().persistent().remove(&val_from_i64(arg0)); 0 }, 1 => { env.storage().temporary().remove(&val_from_i64(arg0)); 0 }, _ => { env.storage().instance().remove(&val_from_i64(arg0)); 0 } }
        var1;
    }
    fn func100(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
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
        var6 = var6.wrapping_mul(var8);
        var9 = (arg1 as u64).wrapping_shr(32 as u32) as i64;
        var5 = var6.wrapping_add(var5.wrapping_mul(var9));
        var10 = var7.wrapping_add(var5.wrapping_shl(32 as u32));
        mstore64!(arg0 as usize, var10 as u64);
        mstore64!(arg0 as usize + 8, ((var7 as u64 > var10 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_mul(var9).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32 as u32) | (var5 as u64).wrapping_shr(32 as u32) as i64)).wrapping_add(arg1.wrapping_mul(arg4).wrapping_add(arg2.wrapping_mul(arg3))) as u64);
    }
    fn func101(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
        let mut var4: i64 = 0;
        loop {
            if (arg3 & 64 == 0) as i32 != 0 {
                if (arg3 == 0) as i32 != 0 {
                    break;
                }
                var4 = (arg3 & 63) as u32 as i64;
                arg2 = arg2.wrapping_shl(var4 as u32) | (arg1 as u64).wrapping_shr((0.wrapping_sub(arg3) & 63) as u32 as i64 as u32) as i64;
                arg1 = arg1.wrapping_shl(var4 as u32);
                break;
            }
            arg2 = arg1.wrapping_shl((arg3 & 63) as u32 as i64 as u32);
            arg1 = 0 /* False */;
        }
        mstore64!(arg0 as usize, arg1 as u64);
        mstore64!(arg0 as usize + 8, arg2 as u64);
    }
    fn func104(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16);
        self.global0 = var2;
        let var6 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
        let var7: i32;
        if (var6 & 18446744069414584320 == 68719476736) as i32 != 0 {
            let mut slot_var2_8_i64 = 0 /* False */ as i64;
            let mut slot_var2_0_i64 = 0 /* False */ as i64;
            loop {
                loop {
                    let var8 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
                    if ((var8 as u64) < 4294967296 as u64) as i32 != 0 {
                        break;
                    }
                    let var9 = Bytes::from_val(env, &val_from_i64(arg1)).get(0) as i64
                    var4 = var9;
                    let var10 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
                    let var11 = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(1 as u32..var10 & 18446744069414584320 | 0 as u32).into_val(env))
                    arg1 = var11;
                    if (var3 != 16) as i32 != 0 {
                        mstore8!(var2.wrapping_add(var3) as usize, (var4 as u64).wrapping_shr(32 as u32) as i64 as u8);
                        var3 = var3.wrapping_add(1);
                        continue;
                    }
                    break;
                }
                unreachable!();
            }
            arg1 = slot_var2_0_i64;
            mstore64!(arg0.wrapping_add(9) as usize, slot_var2_8_i64 as u64);
            mstore64!(arg0 as usize + 1, arg1 as u64);
            var7 = 0;
        }
        else {
            var7 = 1;
        }
        mstore8!(arg0 as usize, var7 as u8);
        self.global0 = var2.wrapping_add(16);
    }
}