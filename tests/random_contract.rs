#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, IntoVal, String, BytesN, Val, FromVal, Vec, Bytes, Symbol};

#[contract]
pub struct RandomContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}

fn val_to_i64(v: Val) -> i64 {
    (unsafe { core::mem::transmute::<Val, u64>(v) }) as i64
}

fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { ContractInfo, Launch(Address, u64), LaunchBalance(Address, u64, Address), SpaceMission(Address), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractInfo { pub admin: Address, pub native_contract: Address, pub slz_fee: u32, pub slz_fee_destination: Address, pub space_fee: u32, pub space_missions: Map<u32, SpaceMissionData>, pub stability_check_duration: u64, pub stellarbucks_contract: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Launch { pub asset: Address, pub dev: Address, pub funds_claimed: bool, pub funds_recipient: Address, pub info: String, pub max_price: i128, pub max_supply: i128, pub min_price: i128, pub pool_balance: i128, pub stability_check: bool, pub stability_check_end: u64, pub stellarbucks: i128, pub supply: i128, pub timestamp: u64, pub tokens_claimed: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpaceMissionData { pub guaranteed_success_funding: u64, pub reward: i128, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error { NotInitialized = 0, Initialized = 1, NotAuthorized = 2, ZeroMinPrice = 100, ZeroMaxPrice = 101, MinPriceGreaterMax = 102, ZeroMaxSupply = 103, MaxSupplyTooBig = 104, MaxPriceTooBig = 105, LaunchNotFound = 200, LaunchInProgress = 201, LaunchEnded = 202, PriceChanged = 203, InsufficientBalance = 204, LaunchSoldOut = 205, LaunchFundsClaimed = 206, InvalidMissionDifficulty = 300, ExasiveFunding = 301, MissionRewardChanged = 302, MissionRewardZero = 303, }

#[contractimpl]
impl RandomContract {

    pub fn initialize(
        &mut self,
        env: Env,
        admin: Address,
        stability_check_duration: u64,
        space_fee: u32,
        slz_fee: u32,
        slz_fee_destination: Address,
        stellarbucks_contract: Address,
        native_contract: Address,
        space_missions_odds: Map<u32, u64>,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        a = self.global0.wrapping_sub(96);
        self.global0 = a;
        {
            self.decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
            let mut value_hi = mload64!(a as usize + 24) as i64;
            if value_hi as i32 | (space_fee & 255 == 0) as i32 | (slz_fee & 255 != 0) as i32 | (!(Address::try_from_val(env, &val_from_i64(slz_fee_destination)).is_ok())) as i32 | (!(Address::try_from_val(env, &val_from_i64(stellarbucks_contract)).is_ok())) as i32 | (!(Address::try_from_val(env, &val_from_i64(native_contract)).is_ok())) as i32 | !(Map::<Val, Val>::try_from_val(env, &val_from_i64(space_missions_odds)).is_ok()) {
                f = mload64!(a as usize + 32) as i64;
                let i = self.storage_has_data_key(env, 1049208);
                if i == 0 {
                    let j = val_to_i64(Map::<Val, Val>::new(env).into_val(env));
                    c = j;
                    let k = Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).len() as i64;
                    g = (k as u64).wrapping_shr(32 as u32) as i64;
                    b = 0;
                    stability_check_duration = 0 /* False */;
                    while (stability_check_duration as u64) < g as u64 {
                        let l = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).keys().get_unchecked(b as u32));
                        e = l;
                        let n = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).values().get_unchecked(b as u32));
                        d = n;
                        self.decode_u64_from_val(env, a.wrapping_add(8), d);
                        let mut sv8_8_i64 = mload64!(a as usize + 8) as i64;
                        if (sv8_8_i64 != 0) {
                            unreachable!();
                        }
                        let mut value_lo = mload64!(a as usize + 16) as i64;
                        let o = self.u64_to_val(env, value_lo);
                        d = o;
                        let p = self.pack_i128_val(env, 0 /* False */, 0 /* False */);
                        let mut sv8_48_i64 = p as i64;
                        let mut sv8_40_i64 = d as i64;
                        b = b.wrapping_add(4294967296);
                        stability_check_duration = stability_check_duration.wrapping_add(1 /* True */);
                        let q = self.map_new_val(
                            env,
                            1049192,
                            2,
                            a.wrapping_add(40),
                            2,
                        );
                        let r = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(c)); m.set(val_from_i64(e & 18446744069414584320 | 0), val_from_i64(q)); val_to_i64(m.into_val(env)) };
                    }
                    let mut sv8_40_i64 = admin as i64;
                    mstore32!(a as usize + 92, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                    mstore32!(a as usize + 88, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                    let mut sv8_48_i64 = f as i64;
                    env.storage().put_contract_data(a.wrapping_add(40));
                    let s = self.storage_key_from_str(env, 1048576, 11);
                    let t = self.event_topic_self_pair(env, s);
                    env.events().publish(val_from_i64(t), val_from_i64(1 /* True */));
                    self.global0 = a.wrapping_add(96);
                }
                self.fail_with_error_2(env, 4294967299 /* Error(Contract, #1) */);
            }
        }
    }

    pub fn change_contract_info(
        &mut self,
        env: Env,
        admin: Address,
        stability_check_duration: u64,
        space_fee: u32,
        slz_fee: u32,
        slz_fee_destination: Address,
        space_missions_odds: Map<u32, u64>,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut j: i64 = 0;
        a = self.global0.wrapping_sub(160);
        self.global0 = a;
        self.decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
        let mut value_hi = mload64!(a as usize + 24) as i64;
        if value_hi as i32 | (space_fee & 255 == 0) as i32 | (slz_fee & 255 != 0) as i32 | (!(Address::try_from_val(env, &val_from_i64(slz_fee_destination)).is_ok())) as i32 | !(Map::<Val, Val>::try_from_val(env, &val_from_i64(space_missions_odds)).is_ok()) {
            env.storage().get_contract_data(a.wrapping_add(96));
            let mut sv6_96_i64 = mload64!(a as usize + 96) as i64;
            if sv6_96_i64 == 0 {
                self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            } else {
                let l = self.memcpy_like(
                    env,
                    a.wrapping_add(40),
                    a.wrapping_add(104),
                    56,
                );
                let mut authorized_addr = mload64!(a as usize + 40) as i64;
                address_from_i64(env, authorized_addr).require_auth();
                c = mload64!(a as usize + 80) as i64;
                let n = Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).len() as i64;
                j = (n as u64).wrapping_shr(32 as u32) as i64;
                b = a.wrapping_add(112);
                f = 0;
                stability_check_duration = 0 /* False */;
                while (stability_check_duration as u64) < j as u64 {
                    let o = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).keys().get_unchecked(f as u32));
                    d = o;
                    let p = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).values().get_unchecked(f as u32));
                    e = p;
                    self.decode_u64_from_val(env, a.wrapping_add(8), e);
                    let mut sv6_8_i64 = mload64!(a as usize + 8) as i64;
                    if (sv6_8_i64 != 0) {
                        unreachable!();
                    }
                    let mut value_lo = mload64!(a as usize + 16) as i64;
                    e = 0 /* False */;
                    h = 0 /* False */;
                    d = d & 18446744069414584320 | 0;
                    let q = if Map::<Val, Val>::from_val(env, &val_from_i64(c)).has(val_from_i64(d)) { 1 } else { 0 };
                    if q == 1 /* True */ {
                        let r = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(c)).get(val_from_i64(d)).unwrap_or(val_from_i64(0)));
                        self.decode_launch_from_storage(env, a.wrapping_add(96), r);
                        let mut sv6_96_i64 = mload64!(a as usize + 96) as i64;
                        if sv6_96_i64 != 0 /* False */ {
                            unreachable!();
                        }
                        h = mload64!(b as usize) as i64;
                        e = mload64!(a as usize + 104) as i64;
                    }
                    let s = self.u64_to_val(env, value_lo);
                    g = s;
                    let t = self.pack_i128_val(env, e, h);
                    let mut sv6_96_i64 = g as i64;
                    f = f.wrapping_add(4294967296);
                    stability_check_duration = stability_check_duration.wrapping_add(1 /* True */);
                    let u = self.map_new_val(
                        env,
                        1049192,
                        2,
                        a.wrapping_add(96),
                        2,
                    );
                    let v = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(c)); m.set(val_from_i64(d), val_from_i64(u)); val_to_i64(m.into_val(env)) };
                }
                let mut authorized_addr = admin as i64;
                mstore32!(a as usize + 92, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(a as usize + 88, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                env.storage().put_contract_data(a.wrapping_add(40));
                self.global0 = a.wrapping_add(160);
            }
        }
    }

    pub fn upgrade(
        &mut self,
        env: Env,
        hash: BytesN<32>,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        {
            let c = Bytes::from_val(env, &val_from_i64(hash)).len() as i64;
            if c & 18446744069414584320 == 137438953472 {
                env.storage().get_contract_data(a);
                let mut sv1_0_i64 = mload64!(a as usize) as i64;
                if sv1_0_i64 != 0 /* False */ {
                    unreachable!();
                }
                self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            }
        }
        unreachable!();
        let mut authorized_addr = mload64!(a as usize + 8) as i64;
        address_from_i64(env, authorized_addr).require_auth();
        env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(hash)));
        self.global0 = a.wrapping_sub(-64);
        0 /* Void */
    }

    pub fn start_space_mission(
        &mut self,
        env: Env,
        user: Address,
        funding: i128,
        difficulty: u32,
        min_mission_reward: i128,
    ) -> (bool, i128) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        a = self.global0.wrapping_sub(160);
        self.global0 = a;
        self.decode_i128_parts(env, a.wrapping_add(80), funding);
        let mut loaded_val = mload64!(a as usize + 80) as i64;
        if (!(loaded_val == 0)) as i32 | difficulty & 255 == 0 {
            d = mload64!(a.wrapping_add(96) as usize) as i64;
            g = mload64!(a as usize + 88) as i64;
            self.decode_i128_parts(env, a.wrapping_add(80), min_mission_reward);
            let mut loaded_val = mload64!(a as usize + 80) as i64;
            if loaded_val == 0 {
                e = mload64!(a.wrapping_add(96) as usize) as i64;
                f = mload64!(a as usize + 88) as i64;
                env.storage().get_contract_data(a.wrapping_add(80));
                let mut loaded_val = mload64!(a as usize + 80) as i64;
                if loaded_val == 0 {
                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    let n = self.memcpy_like(
                        env,
                        a.wrapping_add(24),
                        a.wrapping_add(88),
                        56,
                    );
                    address_from_i64(env, user).require_auth();
                    let mut sv4_20_i32 = 0 as i32;
                    self.compute_reward_and_fees(
                        env,
                        a,
                        g,
                        d,
                        10000000,
                        0 /* False */,
                        a.wrapping_add(20),
                    );
                    let mut sv4_20_i32 = mload32!(a as usize + 20) as i32;
                    if sv4_20_i32 == 0 {
                        min_mission_reward = mload64!(a as usize) as i64;
                        funding = mload64!(a.wrapping_add(8) as usize) as i64;
                        if ((if funding == 0 { (min_mission_reward == 0) as i32 } else { (funding < 0 /* False */) as i32 })) == 0 {
                            h = mload64!(a as usize + 48) as i64;
                            let o = self.storage_key_from_str(env, 1049280, 4);
                            i = o;
                            let p = self.pack_i128_val(env, min_mission_reward, funding);
                            let mut sv4_152_i64 = p as i64;
                            let mut sv4_144_i64 = user as i64;
                            while b != 16 {
                                mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, 0 /* Void */ as u64);
                                b = b.wrapping_add(8);
                            }
                            b = 0;
                            while b != 16 {
                                let q = mload64!(a.wrapping_add(144).wrapping_add(b) as usize) as i64;
                                mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, q as u64);
                                b = b.wrapping_add(8);
                            }
                            let r = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            let _ = env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(h)), &Symbol::from_val(env, &val_from_i64(i)), Vec::<Val>::from_val(env, &val_from_i64(r)));
                        }
                        {
                            h = mload64!(a as usize + 64) as i64;
                            i = difficulty & 18446744069414584320 | 0;
                            let s = if Map::<Val, Val>::from_val(env, &val_from_i64(h)).has(val_from_i64(i)) { 1 } else { 0 };
                            if s == 1 /* True */ {
                                let t = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(h)).get(val_from_i64(i)).unwrap_or(val_from_i64(0)));
                                self.decode_launch_from_storage(env, a.wrapping_add(80), t);
                                let mut loaded_val = mload64!(a as usize + 80) as i64;
                                if loaded_val != 0 {
                                    unreachable!();
                                }
                                j = mload64!(a.wrapping_add(104) as usize) as i64;
                                b = (d == 0) as i32;
                                if ((if b != 0 { (g as u64 > j as u64) as i32 } else { (d > 0 /* False */) as i32 })) != 0 {
                                    unreachable!();
                                }
                                min_mission_reward = mload64!(a as usize + 88) as i64;
                                funding = mload64!(a.wrapping_add(96) as usize) as i64;
                                if ((if funding == e { (f as u64 > min_mission_reward as u64) as i32 } else { (funding < e) as i32 })) != 0 {
                                    unreachable!();
                                }
                                if funding | min_mission_reward == 0 {
                                    self.fail_with_error_2(env, 1301375090691 /* Error(Contract, #303) */);
                                    unreachable!();
                                }
                                let u = env.prng().gen_range::<u64>(1 /* True */ as u64..=j as u64) as i64;
                                b = (if b != 0 { (u as u64 <= g as u64) as i32 } else { (d >= 0 /* False */) as i32 });
                                e = (if b != 0 { funding } else { 0 /* False */ });
                                f = (if b != 0 { min_mission_reward } else { 1 /* True */ });
                                k = funding.wrapping_sub(e).wrapping_sub(((min_mission_reward as u64) < f as u64) as i32 as u32 as i64);
                                if (funding ^ e) & (funding ^ k) < 0 /* False */ {
                                    unreachable!();
                                }
                                let v = self.u64_to_val(env, j);
                                funding = v;
                                let w = self.pack_i128_val(env, min_mission_reward.wrapping_sub(f), k);
                                let mut sv4_88_i64 = w as i64;
                                let mut loaded_val = funding as i64;
                                c = a.wrapping_add(80);
                                let x = self.map_new_val(
                                    env,
                                    1049192,
                                    2,
                                    c,
                                    2,
                                );
                                let y = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(h)); m.set(val_from_i64(i), val_from_i64(x)); val_to_i64(m.into_val(env)) };
                                env.storage().put_contract_data(a.wrapping_add(24));
                                let z = val_to_i64(env.current_contract_address().into_val(env));
                                funding = z;
                                let mut sv4_56_i64 = mload64!(a as usize + 56) as i64;
                                self.persist_launch_state(
                                    env,
                                    sv4_56_i64,
                                    funding,
                                    user,
                                    f,
                                    e,
                                );
                                let aa = self.storage_key_from_str(env, 1048649, 13);
                                let ab = self.event_topic_self_pair(env, aa);
                                let ac = self.pack_i128_val(env, g, d);
                                min_mission_reward = ac;
                                let ad = self.pack_i128_val(env, f, e);
                                d = b as u32 as i64;
                                let mut sv4_88_i64 = min_mission_reward as i64;
                                let mut loaded_val = (difficulty & 18446744069414584320 | 0) as i64;
                                let ae = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                let mut sv4_152_i64 = ae as i64;
                                let mut sv4_144_i64 = user as i64;
                                let af = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                env.events().publish(val_from_i64(ab), val_from_i64(af));
                                let ah = self.pack_i128_val(env, f, e);
                                let mut sv4_88_i64 = ah as i64;
                                let mut loaded_val = d as i64;
                                let ai = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                self.global0 = a.wrapping_add(160);
                                return ai;
                            }
                            self.fail_with_error_2(env, 1288490188803 /* Error(Contract, #300) */);
                            unreachable!();
                        }
                        self.fail_with_error_2(env, 1292785156099 /* Error(Contract, #301) */);
                        unreachable!();
                        self.fail_with_error_2(env, 1297080123395 /* Error(Contract, #302) */);
                    }
                }
            }
        }
        unreachable!();
    }

    pub fn add_space_missions_reward(
        &mut self,
        env: Env,
        user: Address,
        mut funds: i128,
        reward_difficulty: u32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut o: i64 = 0;
        a = self.global0.wrapping_sub(128);
        self.global0 = a;
        'label0: {
            self.decode_i128_parts(env, a.wrapping_sub(-64), funds);
            let mut sv3_64_i64 = mload64!(a as usize + 64) as i64;
            if (!(sv3_64_i64 == 0)) as i32 | reward_difficulty & 255 == 0 {
                f = mload64!(a.wrapping_add(80) as usize) as i64;
                i = mload64!(a as usize + 72) as i64;
                env.storage().get_contract_data(a.wrapping_sub(-64));
                let mut sv3_64_i64 = mload64!(a as usize + 64) as i64;
                if sv3_64_i64 == 0 {
                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    b = (reward_difficulty as u64).wrapping_shr(32 as u32) as i64 as i32;
                    let q = self.memcpy_like(
                        env,
                        a.wrapping_add(8),
                        a.wrapping_add(72),
                        56,
                    );
                    address_from_i64(env, user).require_auth();
                    j = reward_difficulty & 18446744069414584320 | 0;
                    g = mload64!(a as usize + 48) as i64;
                    let r = Map::<Val, Val>::from_val(env, &val_from_i64(g)).len() as i64;
                    k = (r as u64).wrapping_shr(32 as u32) as i64;
                    l = mload64!(a as usize + 40) as i64;
                    c = a.wrapping_add(80);
                    funds = 0;
                    reward_difficulty = 0 /* False */;
                    h = g;
                    loop {
                        'label2: {
                            if (reward_difficulty as u64) < k as u64 {
                                let s = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(g)).keys().get_unchecked(funds as u32));
                                d = s;
                                let t = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(g)).values().get_unchecked(funds as u32));
                                e = t;
                                self.decode_launch_from_storage(env, a.wrapping_sub(-64), e);
                                let mut sv3_64_i64 = mload64!(a as usize + 64) as i64;
                                if sv3_64_i64 != 0 {
                                    unreachable!();
                                }
                                if ((d as u64).wrapping_shr(32 as u32) as i64 as i32) != b {
                                    break 'label2;
                                }
                                d = mload64!(c as usize) as i64;
                                e = mload64!(a as usize + 72) as i64;
                                let mut sv3_88_i64 = mload64!(a as usize + 88) as i64;
                                let u = val_to_i64(env.current_contract_address().into_val(env));
                                self.persist_launch_state(
                                    env,
                                    l,
                                    user,
                                    u,
                                    i,
                                    f,
                                );
                                o = e.wrapping_add(i);
                                e = ((e as u64 > o as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(f));
                                if (d ^ f ^ 18446744073709551615) & (d ^ e) < 0 /* False */ {
                                    break 'label0;
                                }
                                let v = self.u64_to_val(env, sv3_88_i64);
                                d = v;
                                let w = self.pack_i128_val(env, o, e);
                                let mut sv3_64_i64 = d as i64;
                                let x = self.map_new_val(
                                    env,
                                    1049192,
                                    2,
                                    a.wrapping_sub(-64),
                                    2,
                                );
                                let y = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(h)); m.set(val_from_i64(j), val_from_i64(x)); val_to_i64(m.into_val(env)) };
                                break 'label2;
                            }
                            env.storage().put_contract_data(a.wrapping_add(8));
                            self.global0 = a.wrapping_add(128);
                        }
                        funds = funds.wrapping_add(4294967296);
                        reward_difficulty = reward_difficulty.wrapping_add(1 /* True */);
                    }
                    unreachable!();
                }
            }
        }
    }

    pub fn new_launch(
        &mut self,
        env: Env,
        dev: Address,
        mut funds_recipient: Address,
        info: String,
        asset: Address,
        mut max_supply: i128,
        mut min_price: i128,
        mut max_price: i128,
        launch_index: u64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        a = self.global0.wrapping_sub(416);
        self.global0 = a;
        'label0: {
            if !((!(Address::try_from_val(env, &val_from_i64(dev)).is_ok())) as i32 | (!(Address::try_from_val(env, &val_from_i64(funds_recipient)).is_ok())) as i32 | (!(String::try_from_val(env, &val_from_i64(info)).is_ok())) as i32 | !(Address::try_from_val(env, &val_from_i64(asset)).is_ok())) {
                self.decode_i128_parts(env, a.wrapping_add(248), max_supply);
                let mut sv8_248_i64 = mload64!(a as usize + 248) as i64;
                if sv8_248_i64 == 0 {
                    b = a.wrapping_add(264);
                    max_supply = mload64!(b as usize) as i64;
                    g = mload64!(a as usize + 256) as i64;
                    self.decode_i128_parts(env, a.wrapping_add(248), min_price);
                    let mut sv8_248_i64 = mload64!(a as usize + 248) as i64;
                    if sv8_248_i64 == 0 {
                        h = mload64!(b as usize) as i64;
                        i = mload64!(a as usize + 256) as i64;
                        self.decode_i128_parts(env, a.wrapping_add(248), max_price);
                        let mut sv8_248_i64 = mload64!(a as usize + 248) as i64;
                        if sv8_248_i64 == 0 {
                            min_price = mload64!(a.wrapping_add(264) as usize) as i64;
                            max_price = mload64!(a as usize + 256) as i64;
                            self.decode_u64_from_val(env, a.wrapping_add(32), launch_index);
                            let mut sv8_32_i32 = mload32!(a as usize + 32) as i32;
                            if sv8_32_i32 == 0 {
                                launch_index = mload64!(a as usize + 40) as i64;
                                let l = self.storage_has_data_key(env, 1049208);
                                if l == 0 {
                                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                                } else {
                                    address_from_i64(env, dev).require_auth();
                                    let m = env.ledger().timestamp() as i64;
                                    j = m;
                                    if max_supply | g == 0 {
                                        self.fail_with_error_2(env, 442381631491 /* Error(Contract, #103) */);
                                    } else {
                                        'label2: {
                                            if ((if max_supply == 0 { (g < 0 /* False */) as i32 } else { (max_supply > 0 /* False */) as i32 })) == 0 {
                                                let mut sv8_28_i32 = 0 as i32;
                                                self.compute_reward_and_fees(
                                                    env,
                                                    a.wrapping_add(8),
                                                    g,
                                                    max_supply,
                                                    max_price,
                                                    min_price,
                                                    a.wrapping_add(28),
                                                );
                                                let mut sv8_28_i32 = mload32!(a as usize + 28) as i32;
                                                if sv8_28_i32 != 0 {
                                                    unreachable!();
                                                }
                                                let n = mload64!(a.wrapping_add(16) as usize) as i64;
                                                if n > 4999999 {
                                                    break 'label2;
                                                }
                                                if h | i == 0 {
                                                    self.fail_with_error_2(env, 429496729603 /* Error(Contract, #100) */);
                                                    break 'label0;
                                                }
                                                if min_price | max_price == 0 {
                                                    self.fail_with_error_2(env, 433791696899 /* Error(Contract, #101) */);
                                                    break 'label0;
                                                }
                                                if ((if min_price == h { ((max_price as u64) < i as u64) as i32 } else { (min_price < h) as i32 })) != 0 {
                                                    unreachable!();
                                                }
                                                let mut sv8_264_i64 = launch_index as i64;
                                                let mut sv8_256_i64 = dev as i64;
                                                let mut sv8_248_i64 = 1 /* True */ as i64;
                                                let o = self.storage_has_data_key(env, a.wrapping_add(248));
                                                if o != 0 {
                                                    self.fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                                                    break 'label0;
                                                }
                                                let p = val_to_i64(env.current_contract_address().into_val(env));
                                                self.persist_launch_state(
                                                    env,
                                                    asset,
                                                    dev,
                                                    p,
                                                    g,
                                                    max_supply,
                                                );
                                                c = a.wrapping_add(184);
                                                let mut sv10_0_i64 = asset as i64;
                                                d = a.wrapping_add(176);
                                                let mut sv11_0_i64 = info as i64;
                                                e = a.wrapping_add(192);
                                                let mut sv12_0_i64 = 0 /* False */ as i64;
                                                let mut sv8_168_i64 = funds_recipient as i64;
                                                mstore16!(a as usize + 208, 0 as u16);
                                                self.verify_launch_window(env, a.wrapping_sub(-64), 64);
                                                mstore64!(a.wrapping_add(152) as usize, min_price as u64);
                                                mstore64!(a.wrapping_add(136) as usize, h as u64);
                                                let mut sv8_264_i64 = launch_index as i64;
                                                let mut sv8_256_i64 = dev as i64;
                                                let mut sv8_248_i64 = 1 /* True */ as i64;
                                                b = a.wrapping_add(248);
                                                f = a.wrapping_add(48);
                                                env.storage().put_contract_data(b, f);
                                                let q = self.memcpy_like(
                                                    env,
                                                    b,
                                                    f,
                                                    112,
                                                );
                                                let mut sv12_0_i64 = mload64!(e as usize) as i64;
                                                mstore64!(a.wrapping_add(392) as usize, sv12_0_i64 as u64);
                                                let mut sv10_0_i64 = mload64!(c as usize) as i64;
                                                mstore64!(a.wrapping_add(384) as usize, sv10_0_i64 as u64);
                                                let mut sv11_0_i64 = mload64!(d as usize) as i64;
                                                mstore64!(a.wrapping_add(376) as usize, sv11_0_i64 as u64);
                                                let mut sv8_168_i64 = mload64!(a as usize + 168) as i64;
                                                funds_recipient = mload64!(a as usize + 208) as i64;
                                                let r = self.storage_key_from_str(env, 1048587, 10);
                                                mstore64!(a.wrapping_add(240) as usize, j as u64);
                                                mstore64!(a.wrapping_add(232) as usize, launch_index as u64);
                                                let s = self.launch_snapshot_to_val(env, a.wrapping_add(216));
                                                let t = self.contract_info_to_val(env, b);
                                                env.events().publish(val_from_i64(s), val_from_i64(t));
                                                self.global0 = a.wrapping_add(416);
                                            }
                                            self.fail_with_error_2(env, 446676598787 /* Error(Contract, #104) */);
                                            break 'label0;
                                        }
                                        self.fail_with_error_2(env, 450971566083 /* Error(Contract, #105) */);
                                        break 'label0;
                                        self.fail_with_error_2(env, 438086664195 /* Error(Contract, #102) */);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn cancel_launch(
        &mut self,
        env: Env,
        launch_key: (Address, u64),
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut i: i64 = 0;
        let mut authorized_addr: i64 = 0;
        a = self.global0.wrapping_sub(208);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(8), launch_key);
        {
            let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
            if sv1_8_i64 == 0 {
                launch_key = mload64!(a.wrapping_add(24) as usize) as i64;
                e = mload64!(a as usize + 16) as i64;
                let l = self.storage_has_data_key(env, 1049208);
                if l == 0 {
                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(176));
                    let m = mload8!(a as usize + 169) as i32;
                    if m == 2 {
                        self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        f = mload64!(a.wrapping_add(16) as usize) as i64;
                        b = a.wrapping_add(32);
                        d = mload64!(b as usize) as i64;
                        g = mload64!(a as usize + 8) as i64;
                        i = mload64!(a as usize + 144) as i64;
                        let mut value_hi = mload64!(a as usize + 24) as i64;
                        authorized_addr = mload64!(a as usize + 120) as i64;
                        address_from_i64(env, authorized_addr).require_auth();
                        if ((if d == 0 { (value_hi == 0) as i32 } else { (d < 0 /* False */) as i32 })) != 0 {
                            unreachable!();
                        }
                        self.fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                    }
                }
            }
        }
        unreachable!();
        let n = val_to_i64(env.current_contract_address().into_val(env));
        self.persist_launch_state(
            env,
            i,
            n,
            authorized_addr,
            g,
            f,
        );
        let mut value_hi = launch_key as i64;
        let mut value_lo = e as i64;
        let mut sv1_8_i64 = 1 /* True */ as i64;
        c = a.wrapping_add(8);
        let o = self.data_key_to_val(env, c);
        env.storage().del_contract_data(o);
        let p = self.storage_key_from_str(env, 1048617, 13);
        d = p;
        mstore64!(a.wrapping_add(24) as usize, launch_key as u64);
        let mut value_lo = e as i64;
        let mut sv1_8_i64 = d as i64;
        let q = self.launch_snapshot_to_val(env, c);
        env.events().publish(val_from_i64(q), val_from_i64(1 /* True */));
        self.global0 = a.wrapping_add(208);
        0 /* Void */
    }

    pub fn buy(
        &mut self,
        env: Env,
        mut user: Address,
        mut launch_key: (Address, u64),
        mut sending: i128,
        min_receive: i128,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut m: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut s: i64 = 0;
        let mut t: i64 = 0;
        let mut u: i64 = 0;
        let mut v: i64 = 0;
        let mut w: i64 = 0;
        let mut x: i64 = 0;
        a = self.global0.wrapping_sub(576);
        self.global0 = a;
        'label0: {
            self.write_launch_key(env, a.wrapping_add(280), launch_key);
            let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
            if sv4_280_i64 == 0 {
                b = a.wrapping_add(296);
                i = mload64!(b as usize) as i64;
                j = mload64!(a as usize + 288) as i64;
                self.decode_i128_parts(env, a.wrapping_add(280), sending);
                let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                if sv4_280_i64 == 0 {
                    launch_key = mload64!(b as usize) as i64;
                    f = mload64!(a as usize + 288) as i64;
                    self.decode_i128_parts(env, a.wrapping_add(280), min_receive);
                    let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                    if sv4_280_i64 == 0 {
                        g = mload64!(a.wrapping_add(296) as usize) as i64;
                        let mut sv4_288_i64 = mload64!(a as usize + 288) as i64;
                        env.storage().get_contract_data(a.wrapping_add(280));
                        let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                        if sv4_280_i64 == 0 {
                            self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let z = self.memcpy_like(
                                env,
                                a.wrapping_add(24),
                                a.wrapping_add(288),
                                56,
                            );
                            let mut sv4_256_i64 = j as i64;
                            let mut sv4_248_i64 = 1 /* True */ as i64;
                            env.storage().get_contract_data(a.wrapping_add(280), a.wrapping_add(248));
                            let aa = mload8!(a as usize + 441) as i32;
                            if aa == 2 {
                                self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                            } else {
                                let ab = self.memcpy_like(
                                    env,
                                    a.wrapping_add(80),
                                    a.wrapping_add(280),
                                    168,
                                );
                                if launch_key | f == 0 {
                                    self.fail_with_error_2(env, 8589934595 /* Error(Contract, #2) */);
                                } else {
                                    v = mload64!(a as usize + 80) as i64;
                                    l = mload64!(a as usize + 96) as i64;
                                    w = mload64!(a.wrapping_add(88) as usize) as i64;
                                    e = mload64!(a.wrapping_add(104) as usize) as i64;
                                    if v ^ l | w ^ e != 0 {
                                        let ac = self.check_launch_state(env, a.wrapping_add(80));
                                        if ac != 0 {
                                            unreachable!();
                                        }
                                        address_from_i64(env, user).require_auth();
                                        let mut sv4_72_i32 = mload32!(a as usize + 72) as i32;
                                        let mut sv4_76_i32 = mload32!(a as usize + 76) as i32;
                                        self.buy_flow_impl(
                                            env,
                                            a.wrapping_add(280),
                                            f,
                                            launch_key,
                                            a.wrapping_add(80),
                                            sv4_72_i32,
                                            sv4_76_i32,
                                        );
                                        q = mload64!(a.wrapping_add(352) as usize) as i64;
                                        r = mload64!(a.wrapping_add(336) as usize) as i64;
                                        sending = mload64!(a.wrapping_add(320) as usize) as i64;
                                        o = mload64!(a.wrapping_add(304) as usize) as i64;
                                        min_receive = mload64!(a.wrapping_add(288) as usize) as i64;
                                        s = mload64!(a as usize + 344) as i64;
                                        t = mload64!(a as usize + 328) as i64;
                                        n = mload64!(a as usize + 312) as i64;
                                        u = mload64!(a as usize + 296) as i64;
                                        m = mload64!(a as usize + 280) as i64;
                                        let ad = val_to_i64(env.current_contract_address().into_val(env));
                                        p = ad;
                                        h = mload64!(a as usize + 56) as i64;
                                        self.persist_launch_state(
                                            env,
                                            h,
                                            user,
                                            p,
                                            f,
                                            launch_key,
                                        );
                                        if ((if launch_key == min_receive { (f as u64 > m as u64) as i32 } else { (launch_key > min_receive) as i32 })) == 0 {
                                            unreachable!();
                                        }
                                        let ae = val_to_i64(env.current_contract_address().into_val(env));
                                        p = ae;
                                        x = launch_key.wrapping_sub(min_receive).wrapping_sub(((f as u64) < m as u64) as i32 as u32 as i64);
                                        if (launch_key ^ min_receive) & (launch_key ^ x) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        self.persist_launch_state(
                                            env,
                                            h,
                                            p,
                                            user,
                                            f.wrapping_sub(m),
                                            x,
                                        );
                                    } else {
                                        self.fail_with_error_2(env, 880468295683 /* Error(Contract, #205) */);
                                        break 'label0;
                                        self.fail_with_error_2(env, 867583393795 /* Error(Contract, #202) */);
                                        break 'label0;
                                    }
                                    let af = val_to_i64(env.current_contract_address().into_val(env));
                                    let mut sv4_40_i64 = mload64!(a as usize + 40) as i64;
                                    self.persist_launch_state(
                                        env,
                                        h,
                                        af,
                                        sv4_40_i64,
                                        s,
                                        q,
                                    );
                                    if ((if sending == g { (sv4_288_i64 as u64 > n as u64) as i32 } else { (sending < g) as i32 })) == 0 {
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        let ag = mload64!(a.wrapping_add(16) as usize) as i64;
                                        b = mload32!(a as usize) as i32;
                                        g = (if b != 0 { ag } else { 0 /* False */ });
                                        let mut sv4_8_i64 = mload64!(a as usize + 8) as i64;
                                        f = (if b != 0 { sv4_8_i64 } else { 0 /* False */ });
                                        launch_key = f.wrapping_add(n);
                                        f = (((launch_key as u64) < f as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(g));
                                        if (g ^ sending ^ 18446744073709551615) & (g ^ f) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        env.storage().put_contract_data(a.wrapping_add(280), launch_key, f);
                                        g = l.wrapping_add(n);
                                        l = (((g as u64) < l as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(e));
                                        if (sending ^ e ^ 18446744073709551615) & (e ^ l) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        mstore64!(a.wrapping_add(104) as usize, l as u64);
                                        b = a.wrapping_add(120);
                                        let mut sv5_0_i64 = mload64!(b as usize) as i64;
                                        e = sv5_0_i64;
                                        h = mload64!(a as usize + 112) as i64;
                                        k = h.wrapping_add(u);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(e.wrapping_add(o));
                                        if (e ^ o ^ 18446744073709551615) & (e ^ h) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = h as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        h = mload64!(a as usize + 128) as i64;
                                        k = h.wrapping_add(m);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(min_receive.wrapping_add(e));
                                        if (e ^ min_receive ^ 18446744073709551615) & (e ^ h) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        if g ^ v | l ^ w == 0 {
                                            let ah = mload8!(a as usize + 240) as i32;
                                            if ah == 0 {
                                                mstore8!(a as usize + 240, 1 as u8);
                                                let ai = env.ledger().timestamp() as i64;
                                                e = ai;
                                                let mut sv4_32_i64 = mload64!(a as usize + 32) as i64;
                                                g = e.wrapping_add(sv4_32_i64);
                                                if (g as u64) < e as u64 {
                                                    break 'label0;
                                                }
                                            }
                                        }
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 1 /* True */ as i64;
                                        b = a.wrapping_add(280);
                                        c = a.wrapping_add(80);
                                        env.storage().put_contract_data(b, c);
                                        let mut sv4_64_i64 = mload64!(a as usize + 64) as i64;
                                        let aj = self.token_transfer_checked(
                                            env,
                                            sv4_64_i64,
                                            t,
                                            r,
                                        );
                                        let mut sv4_64_i64 = aj as i64;
                                        env.storage().put_contract_data(a.wrapping_add(24));
                                        e = mload64!(a as usize + 232) as i64;
                                        let ak = self.memcpy_like(
                                            env,
                                            a.wrapping_add(384),
                                            c,
                                            168,
                                        );
                                        c = ak;
                                        let al = self.storage_key_from_str(env, 1048630, 3);
                                        g = al;
                                        mstore64!(a.wrapping_add(376) as usize, f as u64);
                                        mstore64!(a.wrapping_add(368) as usize, launch_key as u64);
                                        mstore64!(a.wrapping_add(352) as usize, q as u64);
                                        mstore64!(a.wrapping_add(336) as usize, r as u64);
                                        mstore64!(a.wrapping_add(320) as usize, sending as u64);
                                        mstore64!(a.wrapping_add(304) as usize, o as u64);
                                        mstore64!(a.wrapping_add(272) as usize, e as u64);
                                        mstore64!(a.wrapping_add(264) as usize, i as u64);
                                        let mut sv4_296_i64 = u as i64;
                                        let mut sv4_288_i64 = min_receive as i64;
                                        let mut sv4_280_i64 = m as i64;
                                        let mut sv4_256_i64 = j as i64;
                                        let mut sv4_248_i64 = g as i64;
                                        let am = self.launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let an = self.pack_i128_val(env, launch_key, f);
                                        let mut sv4_560_i64 = an as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ao = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = ao;
                                        let ap = self.launch_key_event_payload(env, b);
                                        launch_key = ap;
                                        self.contract_info_to_val(env, c);
                                        let mut sv4_560_i64 = launch_key as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ar = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(am), val_from_i64(ar));
                                        self.global0 = a.wrapping_add(576);
                                    }
                                    self.fail_with_error_2(env, 871878361091 /* Error(Contract, #203) */);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn sell(
        &mut self,
        env: Env,
        mut user: Address,
        mut launch_key: (Address, u64),
        mut sending: i128,
        min_receive: i128,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut m: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut s: i64 = 0;
        a = self.global0.wrapping_sub(576);
        self.global0 = a;
        'label0: {
            self.write_launch_key(env, a.wrapping_add(280), launch_key);
            let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
            if sv4_280_i64 == 0 {
                b = a.wrapping_add(296);
                launch_key = mload64!(b as usize) as i64;
                k = mload64!(a as usize + 288) as i64;
                self.decode_i128_parts(env, a.wrapping_add(280), sending);
                let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                if sv4_280_i64 == 0 {
                    sending = mload64!(b as usize) as i64;
                    g = mload64!(a as usize + 288) as i64;
                    self.decode_i128_parts(env, a.wrapping_add(280), min_receive);
                    let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                    if sv4_280_i64 == 0 {
                        j = mload64!(a.wrapping_add(296) as usize) as i64;
                        f = mload64!(a as usize + 288) as i64;
                        env.storage().get_contract_data(a.wrapping_add(280));
                        let mut sv4_280_i64 = mload64!(a as usize + 280) as i64;
                        if sv4_280_i64 == 0 {
                            self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let u = self.memcpy_like(
                                env,
                                a.wrapping_add(24),
                                a.wrapping_add(288),
                                56,
                            );
                            let mut sv4_256_i64 = k as i64;
                            let mut sv4_248_i64 = 1 /* True */ as i64;
                            env.storage().get_contract_data(a.wrapping_add(280), a.wrapping_add(248));
                            let v = mload8!(a as usize + 441) as i32;
                            if v == 2 {
                                self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                            } else {
                                b = a.wrapping_add(80);
                                let w = self.memcpy_like(
                                    env,
                                    b,
                                    a.wrapping_add(280),
                                    168,
                                );
                                {
                                    let x = self.check_launch_state(env, b);
                                    if x == 0 {
                                        if sending | g == 0 {
                                            self.fail_with_error_2(env, 8589934595 /* Error(Contract, #2) */);
                                            break 'label0;
                                        }
                                        address_from_i64(env, user).require_auth();
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        let mut sv4_8_i64 = mload64!(a as usize + 8) as i64;
                                        b = mload32!(a as usize) as i32;
                                        e = (if b != 0 { sv4_8_i64 } else { 0 /* False */ });
                                        let y = mload64!(a.wrapping_add(16) as usize) as i64;
                                        h = (if b != 0 { y } else { 0 /* False */ });
                                        if ((if sending == h { ((e as u64) < g as u64) as i32 } else { (h < sending) as i32 })) != 0 {
                                            unreachable!();
                                        }
                                        let mut sv4_72_i32 = mload32!(a as usize + 72) as i32;
                                        let mut sv4_76_i32 = mload32!(a as usize + 76) as i32;
                                        self.sell_flow_impl(
                                            env,
                                            a.wrapping_add(280),
                                            g,
                                            sending,
                                            a.wrapping_add(80),
                                            sv4_72_i32,
                                            sv4_76_i32,
                                        );
                                        o = mload64!(a.wrapping_add(336) as usize) as i64;
                                        sending = mload64!(a.wrapping_add(304) as usize) as i64;
                                        min_receive = mload64!(a.wrapping_add(288) as usize) as i64;
                                        l = mload64!(a.wrapping_add(320) as usize) as i64;
                                        p = mload64!(a.wrapping_add(352) as usize) as i64;
                                        q = mload64!(a as usize + 328) as i64;
                                        m = mload64!(a as usize + 296) as i64;
                                        g = mload64!(a as usize + 280) as i64;
                                        n = mload64!(a as usize + 312) as i64;
                                        r = mload64!(a as usize + 344) as i64;
                                        let z = val_to_i64(env.current_contract_address().into_val(env));
                                        i = z;
                                        s = mload64!(a as usize + 56) as i64;
                                        let mut sv4_40_i64 = mload64!(a as usize + 40) as i64;
                                        self.persist_launch_state(
                                            env,
                                            s,
                                            i,
                                            sv4_40_i64,
                                            r,
                                            p,
                                        );
                                        if ((if j == l { (f as u64 > n as u64) as i32 } else { (j > l) as i32 })) != 0 {
                                            unreachable!();
                                        }
                                        let aa = val_to_i64(env.current_contract_address().into_val(env));
                                        self.persist_launch_state(
                                            env,
                                            s,
                                            aa,
                                            user,
                                            n,
                                            l,
                                        );
                                        j = h.wrapping_sub(min_receive).wrapping_sub(((e as u64) < g as u64) as i32 as u32 as i64);
                                        if (min_receive ^ h) & (h ^ j) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        h = e.wrapping_sub(g);
                                        env.storage().put_contract_data(a.wrapping_add(280), h, j);
                                        b = a.wrapping_add(104);
                                        let mut sv5_0_i64 = mload64!(b as usize) as i64;
                                        e = sv5_0_i64;
                                        let mut sv4_96_i64 = mload64!(a as usize + 96) as i64;
                                        f = sv4_96_i64;
                                        i = e.wrapping_sub(min_receive).wrapping_sub(((f as u64) < g as u64) as i32 as u32 as i64);
                                        if (e ^ min_receive) & (e ^ i) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_96_i64 = f.wrapping_sub(g) as i64;
                                        b = a.wrapping_add(120);
                                        e = sv5_0_i64;
                                        let mut sv4_112_i64 = mload64!(a as usize + 112) as i64;
                                        f = sv4_112_i64;
                                        i = e.wrapping_sub(sending).wrapping_sub(((f as u64) < m as u64) as i32 as u32 as i64);
                                        if (e ^ sending) & (e ^ i) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_112_i64 = f.wrapping_sub(m) as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        f = mload64!(a as usize + 128) as i64;
                                        i = f.wrapping_add(m);
                                        f = (((i as u64) < f as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(e));
                                        if (e ^ sending ^ 18446744073709551615) & (e ^ f) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 1 /* True */ as i64;
                                        b = a.wrapping_add(280);
                                        c = a.wrapping_add(80);
                                        env.storage().put_contract_data(b, c);
                                        let mut sv4_64_i64 = mload64!(a as usize + 64) as i64;
                                        let ab = self.token_transfer_checked(
                                            env,
                                            sv4_64_i64,
                                            q,
                                            o,
                                        );
                                        let mut sv4_64_i64 = ab as i64;
                                        env.storage().put_contract_data(a.wrapping_add(24));
                                        e = mload64!(a as usize + 232) as i64;
                                        let ac = self.memcpy_like(
                                            env,
                                            a.wrapping_add(384),
                                            c,
                                            168,
                                        );
                                        c = ac;
                                        let ad = self.storage_key_from_str(env, 1048633, 4);
                                        f = ad;
                                        mstore64!(a.wrapping_add(376) as usize, j as u64);
                                        mstore64!(a.wrapping_add(368) as usize, h as u64);
                                        mstore64!(a.wrapping_add(352) as usize, p as u64);
                                        mstore64!(a.wrapping_add(336) as usize, o as u64);
                                        mstore64!(a.wrapping_add(320) as usize, l as u64);
                                        mstore64!(a.wrapping_add(304) as usize, sending as u64);
                                        mstore64!(a.wrapping_add(272) as usize, e as u64);
                                        mstore64!(a.wrapping_add(264) as usize, launch_key as u64);
                                        let mut sv4_296_i64 = m as i64;
                                        let mut sv4_288_i64 = min_receive as i64;
                                        let mut sv4_280_i64 = g as i64;
                                        let mut sv4_256_i64 = k as i64;
                                        let mut sv4_248_i64 = f as i64;
                                        let ae = self.launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let af = self.pack_i128_val(env, h, j);
                                        let mut sv4_560_i64 = af as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ag = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = ag;
                                        let ah = self.launch_key_event_payload(env, b);
                                        sending = ah;
                                        let ai = self.contract_info_to_val(env, c);
                                        let mut sv4_560_i64 = sending as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let aj = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(ae), val_from_i64(aj));
                                        self.global0 = a.wrapping_add(576);
                                    }
                                    self.fail_with_error_2(env, 867583393795 /* Error(Contract, #202) */);
                                    break 'label0;
                                }
                                self.fail_with_error_2(env, 876173328387 /* Error(Contract, #204) */);
                                break 'label0;
                                self.fail_with_error_2(env, 871878361091 /* Error(Contract, #203) */);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn claim_launch_funds(
        &mut self,
        env: Env,
        launch_key: (Address, u64),
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        a = self.global0.wrapping_sub(368);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            let mut sv1_168_i64 = mload64!(a as usize + 168) as i64;
            if sv1_168_i64 == 0 {
                launch_key = mload64!(a.wrapping_add(184) as usize) as i64;
                c = mload64!(a as usize + 176) as i64;
                env.storage().get_contract_data(a.wrapping_add(168));
                let mut sv1_168_i64 = mload64!(a as usize + 168) as i64;
                if sv1_168_i64 == 0 {
                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    d = mload64!(a.wrapping_add(208) as usize) as i64;
                    env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                    let g = mload8!(a as usize + 329) as i32;
                    if g == 2 {
                        self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        let h = self.memcpy_like(
                            env,
                            a,
                            a.wrapping_add(168),
                            168,
                        );
                        a = h;
                        let i = mload8!(a as usize + 161) as i32;
                        if i == 0 {
                            let j = self.check_launch_state(env, a);
                            if j == 0 {
                                self.fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                                break 'label0;
                            }
                            let k = val_to_i64(env.current_contract_address().into_val(env));
                            let mut sv1_120_i64 = mload64!(a as usize + 120) as i64;
                            let mut sv1_32_i64 = mload64!(a as usize + 32) as i64;
                            let l = mload64!(a.wrapping_add(40) as usize) as i64;
                            self.persist_launch_state(
                                env,
                                d,
                                k,
                                sv1_120_i64,
                                sv1_32_i64,
                                l,
                            );
                            mstore8!(a as usize + 161, 1 as u8);
                            let mut sv1_176_i64 = c as i64;
                            let mut sv1_168_i64 = 1 /* True */ as i64;
                            b = a.wrapping_add(168);
                            env.storage().put_contract_data(b, a);
                            d = mload64!(a as usize + 152) as i64;
                            let m = self.storage_key_from_str(env, 1048597, 20);
                            e = m;
                            mstore64!(a.wrapping_add(192) as usize, d as u64);
                            mstore64!(a.wrapping_add(184) as usize, launch_key as u64);
                            let mut sv1_176_i64 = c as i64;
                            let mut sv1_168_i64 = e as i64;
                            let n = self.launch_snapshot_to_val(env, b);
                            let o = self.contract_info_to_val(env, a);
                            env.events().publish(val_from_i64(n), val_from_i64(o));
                            self.global0 = a.wrapping_add(368);
                        }
                        self.fail_with_error_2(env, 884763262979 /* Error(Contract, #206) */);
                    }
                }
            }
        }
    }

    pub fn claim_launch_balance(
        &mut self,
        env: Env,
        user: Address,
        launch_key: (Address, u64),
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        a = self.global0.wrapping_sub(448);
        self.global0 = a;
        'label0: {
            self.write_launch_key(env, a.wrapping_add(232), launch_key);
            let mut sv2_232_i64 = mload64!(a as usize + 232) as i64;
            if sv2_232_i64 == 0 {
                launch_key = mload64!(a.wrapping_add(248) as usize) as i64;
                f = mload64!(a as usize + 240) as i64;
                env.storage().get_contract_data(a.wrapping_add(232));
                let mut sv2_232_i64 = mload64!(a as usize + 232) as i64;
                if sv2_232_i64 == 0 {
                    self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    j = mload64!(a.wrapping_add(264) as usize) as i64;
                    let mut sv2_408_i64 = f as i64;
                    let mut sv2_400_i64 = 1 /* True */ as i64;
                    env.storage().get_contract_data(a.wrapping_add(232), a.wrapping_add(400));
                    let m = mload8!(a as usize + 393) as i32;
                    if m == 2 {
                        self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        b = a.wrapping_sub(-64);
                        let n = self.memcpy_like(
                            env,
                            b,
                            a.wrapping_add(232),
                            168,
                        );
                        let o = self.check_launch_state(env, b);
                        if o == 0 {
                            self.fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                        } else {
                            let mut sv2_256_i64 = user as i64;
                            let mut sv2_248_i64 = launch_key as i64;
                            let mut sv2_240_i64 = f as i64;
                            let mut sv2_232_i64 = 0 /* Void */ as i64;
                            c = a.wrapping_add(232);
                            env.storage().get_contract_data(a.wrapping_add(40), c);
                            h = mload64!(a.wrapping_add(56) as usize) as i64;
                            b = mload32!(a as usize + 40) as i32;
                            i = mload64!(a as usize + 48) as i64;
                            let p = val_to_i64(env.current_contract_address().into_val(env));
                            d = p;
                            let mut sv2_200_i64 = mload64!(a as usize + 200) as i64;
                            i = (if b != 0 { i } else { 0 /* False */ });
                            h = (if b != 0 { h } else { 0 /* False */ });
                            self.persist_launch_state(
                                env,
                                sv2_200_i64,
                                d,
                                user,
                                i,
                                h,
                            );
                            let mut sv2_256_i64 = user as i64;
                            let mut sv2_248_i64 = launch_key as i64;
                            let mut sv2_240_i64 = f as i64;
                            let mut sv2_232_i64 = 0 /* Void */ as i64;
                            let q = self.data_key_to_val(env, c);
                            env.storage().del_contract_data(q);
                            b = a.wrapping_add(136);
                            d = mload64!(b as usize) as i64;
                            e = mload64!(a as usize + 128) as i64;
                            g = e.wrapping_add(i);
                            e = (((g as u64) < e as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(h));
                            if (d ^ h ^ 18446744073709551615) & (d ^ e) >= 0 /* False */ {
                                let mut sv2_248_i64 = launch_key as i64;
                                let mut sv2_240_i64 = f as i64;
                                let mut sv2_232_i64 = 1 /* True */ as i64;
                                let mut sv2_36_i32 = 0 as i32;
                                env.storage().put_contract_data(a.wrapping_add(232), a.wrapping_sub(-64));
                                let mut sv2_112_i64 = mload64!(a as usize + 112) as i64;
                                let r = mload64!(a.wrapping_add(120) as usize) as i64;
                                self.compute_reward_and_fees(
                                    env,
                                    a.wrapping_add(16),
                                    i,
                                    h,
                                    sv2_112_i64,
                                    r,
                                    a.wrapping_add(36),
                                );
                                let mut sv2_36_i32 = mload32!(a as usize + 36) as i32;
                                if sv2_36_i32 == 0 {
                                    d = mload64!(a as usize + 64) as i64;
                                    e = mload64!(a.wrapping_add(72) as usize) as i64;
                                    if d | e != 0 {
                                        g = mload64!(a as usize + 16) as i64;
                                        k = mload64!(a.wrapping_add(24) as usize) as i64;
                                        if (g | k ^ 9223372036854775808 != 0) as i32 & d & e == 18446744073709551615 {
                                            self.claim_launch_balance_impl(
                                                env,
                                                a,
                                                g,
                                                k,
                                                d,
                                                e,
                                            );
                                            e = mload64!(a as usize) as i64;
                                            d = mload64!(a.wrapping_add(8) as usize) as i64;
                                            if ((if d == 0 { (e == 0) as i32 } else { (d < 0 /* False */) as i32 })) == 0 {
                                                let s = self.storage_key_from_str(env, 1049284, 4);
                                                g = s;
                                                let t = self.pack_i128_val(env, e, d);
                                                let mut sv2_408_i64 = t as i64;
                                                let mut sv2_400_i64 = user as i64;
                                                b = 0;
                                                while b != 16 {
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, 0 /* Void */ as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                b = 0;
                                                while b != 16 {
                                                    let u = mload64!(a.wrapping_add(400).wrapping_add(b) as usize) as i64;
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, u as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                let v = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                let _ = env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(j)), &Symbol::from_val(env, &val_from_i64(g)), Vec::<Val>::from_val(env, &val_from_i64(v)));
                                                break 'label0;
                                            }
                                            j = mload64!(a as usize + 216) as i64;
                                            let w = self.storage_key_from_str(env, 1048637, 12);
                                            g = w;
                                            mstore64!(a.wrapping_add(256) as usize, j as u64);
                                            mstore64!(a.wrapping_add(248) as usize, launch_key as u64);
                                            let mut sv2_240_i64 = f as i64;
                                            let mut sv2_232_i64 = g as i64;
                                            let x = self.launch_snapshot_to_val(env, a.wrapping_add(232));
                                            let y = self.pack_i128_val(env, i, h);
                                            f = y;
                                            let z = self.pack_i128_val(env, e, d);
                                            let mut sv2_408_i64 = z as i64;
                                            let mut sv2_400_i64 = f as i64;
                                            let aa = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            let ab = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            env.events().publish(val_from_i64(x), val_from_i64(ab));
                                            self.global0 = a.wrapping_add(448);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn calculate_buy(
        &mut self,
        env: Env,
        mut launch_key: (Address, u64),
        sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut e: i64 = 0;
        a = self.global0.wrapping_sub(368);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            {
                let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                if sv2_168_i64 == 0 {
                    b = a.wrapping_add(184);
                    launch_key = mload64!(b as usize) as i64;
                    self.decode_i128_parts(env, a.wrapping_add(168), sending);
                    let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                    if sv2_168_i64 == 0 {
                        sending = mload64!(b as usize) as i64;
                        e = mload64!(a as usize + 176) as i64;
                        env.storage().get_contract_data(a.wrapping_add(168));
                        let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                        if sv2_168_i64 == 0 {
                            self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            b = mload32!(a.wrapping_add(228) as usize) as i32;
                            c = mload32!(a.wrapping_add(224) as usize) as i32;
                            env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                            let g = mload8!(a as usize + 329) as i32;
                            if g != 2 {
                                break 'label0;
                            }
                            self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                        }
                    }
                }
            }
            unreachable!();
        }
        let h = self.memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = h;
        self.buy_flow_impl(
            env,
            a.wrapping_add(168),
            e,
            sending,
            a,
            c,
            b,
        );
        let i = self.launch_key_event_payload(env, a.wrapping_add(168));
        self.global0 = a.wrapping_add(368);
        i
    }

    pub fn calculate_sell(
        &mut self,
        env: Env,
        mut launch_key: (Address, u64),
        sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut e: i64 = 0;
        a = self.global0.wrapping_sub(368);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            {
                let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                if sv2_168_i64 == 0 {
                    b = a.wrapping_add(184);
                    launch_key = mload64!(b as usize) as i64;
                    self.decode_i128_parts(env, a.wrapping_add(168), sending);
                    let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                    if sv2_168_i64 == 0 {
                        sending = mload64!(b as usize) as i64;
                        e = mload64!(a as usize + 176) as i64;
                        env.storage().get_contract_data(a.wrapping_add(168));
                        let mut sv2_168_i64 = mload64!(a as usize + 168) as i64;
                        if sv2_168_i64 == 0 {
                            self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            b = mload32!(a.wrapping_add(228) as usize) as i32;
                            c = mload32!(a.wrapping_add(224) as usize) as i32;
                            env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                            let g = mload8!(a as usize + 329) as i32;
                            if g != 2 {
                                break 'label0;
                            }
                            self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                        }
                    }
                }
            }
            unreachable!();
        }
        let h = self.memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = h;
        self.sell_flow_impl(
            env,
            a.wrapping_add(168),
            e,
            sending,
            a,
            c,
            b,
        );
        let i = self.launch_key_event_payload(env, a.wrapping_add(168));
        self.global0 = a.wrapping_add(368);
        i
    }

    pub fn get_launch_data(
        &mut self,
        env: Env,
        launch_key: (Address, u64),
    ) -> Launch {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(368);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            let mut sv1_168_i64 = mload64!(a as usize + 168) as i64;
            if sv1_168_i64 == 0 {
                launch_key = mload64!(a as usize + 176) as i64;
                let c = mload64!(a.wrapping_add(184) as usize) as i64;
                env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                let d = mload8!(a as usize + 329) as i32;
                if d != 2 {
                    break 'label0;
                }
                self.fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
            }
            unreachable!();
        }
        let e = self.memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = e;
        let f = self.contract_info_to_val(env, a);
        self.global0 = a.wrapping_add(368);
        f
    }

    pub fn get_contract_info(
        &mut self,
        env: Env,
    ) -> ContractInfo {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(128);
        self.global0 = a;
        env.storage().get_contract_data(a.wrapping_sub(-64));
        let mut sv0_64_i64 = mload64!(a as usize + 64) as i64;
        if sv0_64_i64 == 0 {
            self.fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            unreachable!();
        }
        b = a.wrapping_add(8);
        let e = self.memcpy_like(
            env,
            b,
            a.wrapping_add(72),
            56,
        );
        let f = self.launch_record_to_val(env, b);
        self.global0 = a.wrapping_add(128);
        f
    }

    pub fn get_launch_balance(
        &mut self,
        env: Env,
        launch_key: (Address, u64),
        user: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        self.write_launch_key(env, a.wrapping_add(32), launch_key);
        let mut sv2_32_i64 = mload64!(a as usize + 32) as i64;
        if (!(sv2_32_i64 == 0)) as i32 | Address::try_from_val(env, &val_from_i64(user)).is_ok() {
            launch_key = mload64!(a.wrapping_add(48) as usize) as i64;
            env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(32));
            let mut value_lo = mload64!(a as usize + 16) as i64;
            b = mload32!(a as usize + 8) as i32;
            let e = mload64!(a.wrapping_add(24) as usize) as i64;
            let f = self.pack_i128_val(env, (if b != 0 { value_lo } else { 0 /* False */ }), (if b != 0 { e } else { 0 /* False */ }));
            self.global0 = a.wrapping_sub(-64);
            return f;
        }
    }

    pub fn version(
        &mut self,
        env: Env,
    ) -> (u32, u32, u32) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        let d = val_to_i64(Vec::<Val>::new(env).into_val(env));
        self.global0 = a.wrapping_add(32);
        d
    }
}