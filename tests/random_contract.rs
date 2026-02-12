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
        env: Env,
        admin: Address,
        mut stability_check_duration: u64,
        space_fee: u32,
        slz_fee: u32,
        slz_fee_destination: Address,
        stellarbucks_contract: Address,
        native_contract: Address,
        space_missions_odds: Map<u32, u64>,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut d: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        a = -96;
        {
            Self::decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
            mload64!(a.wrapping_add(24) as usize);
            f = mload64!(a.wrapping_add(32) as usize);
            let i = Self::storage_has_data_key(env, 1049208);
            if i == 0 {
                let j = val_to_i64(Map::<Val, Val>::new(env).into_val(env));
                let k = Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).len() as i64;
                g = (k as u64).wrapping_shr(32 as u32) as i64;
                b = 0;
                stability_check_duration = 0;
                while (stability_check_duration as u64) < g as u64 {
                    let l = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).keys().get_unchecked(b as u32));
                    let n = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).values().get_unchecked(b as u32));
                    d = n;
                    Self::decode_u64_from_val(env, a.wrapping_add(8), d);
                    let a_hi = mload64!(a.wrapping_add(8) as usize);
                    if (a_hi != 0) {
                        unreachable!();
                    }
                    let value_lo = mload64!(a.wrapping_add(16) as usize);
                    let o = Self::u64_to_val(env, value_lo);
                    d = o;
                    let p = Self::pack_i128_val(env, 0, 0);
                    let mut sv8_48_i64 = p as i64;
                    let mut sv8_40_i64 = d as i64;
                    b = b.wrapping_add(4294967296);
                    stability_check_duration += 1;
                    let q = Self::map_new_val(
                        env,
                        1049192,
                        2,
                        a.wrapping_add(40),
                        2,
                    );
                    { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(j)); m.set(val_from_i64(l & 18446744069414584320 | 0), val_from_i64(q)); val_to_i64(m.into_val(env)) };
                }
                let sv8_40_i64 = admin as i64;
                mstore32!(a.wrapping_add(92) as usize, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(a.wrapping_add(88) as usize, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                let sv8_48_i64 = f as i64;
                env.storage().put_contract_data(a.wrapping_add(40));
                let s = Self::storage_key_from_str(env, 1048576, 11);
                let t = Self::event_topic_self_pair(env, s);
                env.events().publish(val_from_i64(t), val_from_i64(1));
            }
            Self::fail_with_error_2(env, 4294967299 /* Error(Contract, #1) */);
        }
    }

    pub fn change_contract_info(
        env: Env,
        admin: Address,
        mut stability_check_duration: u64,
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
        let mut h: i64 = 0;
        let mut j: i64 = 0;
        a = -160;
        Self::decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
        mload64!(a.wrapping_add(24) as usize);
        env.storage().get_contract_data(a.wrapping_add(96));
        let mut sv6_96_i64 = mload64!(a.wrapping_add(96) as usize);
        if sv6_96_i64 == 0 {
            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
        } else {
            let l = Self::memcpy_like(
                env,
                a.wrapping_add(40),
                a.wrapping_add(104),
                56,
            );
            let mut authorized_addr = mload64!(a.wrapping_add(40) as usize);
            address_from_i64(&env, authorized_addr).require_auth();
            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
            c = loaded_val;
            let n = Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).len() as i64;
            j = (n as u64).wrapping_shr(32 as u32) as i64;
            b = a.wrapping_add(112);
            f = 0;
            stability_check_duration = 0;
            while (stability_check_duration as u64) < j as u64 {
                let o = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).keys().get_unchecked(f as u32));
                d = o;
                let p = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).values().get_unchecked(f as u32));
                e = p;
                Self::decode_u64_from_val(env, a.wrapping_add(8), e);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if (a_hi != 0) {
                    unreachable!();
                }
                let value_lo = mload64!(a.wrapping_add(16) as usize);
                e = 0;
                h = 0;
                d = d & 18446744069414584320 | 0;
                let q = if Map::<Val, Val>::from_val(env, &val_from_i64(c)).has(val_from_i64(d)) { 1 } else { 0 };
                if q == 1 {
                    let r = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(c)).get(val_from_i64(d)).unwrap_or(val_from_i64(0)));
                    Self::decode_launch_from_storage(env, a.wrapping_add(96), r);
                    let mut sv6_96_i64 = mload64!(a.wrapping_add(96) as usize);
                    if sv6_96_i64 != 0 {
                        unreachable!();
                    }
                    h = mload64!(b as usize);
                    e = mload64!(a.wrapping_add(104) as usize);
                }
                let s = Self::u64_to_val(env, value_lo);
                Self::pack_i128_val(env, e, h);
                let sv6_96_i64 = s as i64;
                f = f.wrapping_add(4294967296);
                stability_check_duration += 1;
                let u = Self::map_new_val(
                    env,
                    1049192,
                    2,
                    a.wrapping_add(96),
                    2,
                );
                { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(c)); m.set(val_from_i64(d), val_from_i64(u)); val_to_i64(m.into_val(env)) };
                c = v;
            }
            let authorized_addr = admin as i64;
            let loaded_val = c as i64;
            mstore32!(a.wrapping_add(92) as usize, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
            mstore32!(a.wrapping_add(88) as usize, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
            env.storage().put_contract_data(a.wrapping_add(40));
        }
    }

    pub fn upgrade(
        env: Env,
        hash: BytesN<32>,
    ) {
        let a: i32 = -64;
        {
            let c = Bytes::from_val(env, &val_from_i64(hash)).len() as i64;
            if c & 18446744069414584320 == 137438953472 {
                env.storage().get_contract_data(a);
                let a_lo = mload64!(a as usize);
                if a_lo != 0 {
                    unreachable!();
                }
                Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            }
        }
        unreachable!();
        let authorized_addr = mload64!(a.wrapping_add(8) as usize);
        address_from_i64(&env, authorized_addr).require_auth();
        env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(hash)));
    }

    pub fn start_space_mission(
        env: Env,
        user: Address,
        mut funding: i128,
        difficulty: u32,
        mut min_mission_reward: i128,
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
        a = -160;
        Self::decode_i128_parts(env, a.wrapping_add(80), funding);
        let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
        let n = mload64!(a.wrapping_add(96) as usize);
        d = n;
        g = mload64!(a.wrapping_add(88) as usize);
        Self::decode_i128_parts(env, a.wrapping_add(80), min_mission_reward);
        let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
        if loaded_val == 0 {
            let o = mload64!(a.wrapping_add(96) as usize);
            e = o;
            f = mload64!(a.wrapping_add(88) as usize);
            env.storage().get_contract_data(a.wrapping_add(80));
            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
            if loaded_val == 0 {
                Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            } else {
                let p = Self::memcpy_like(
                    env,
                    a.wrapping_add(24),
                    a.wrapping_add(88),
                    56,
                );
                user.require_auth();
                Self::compute_reward_and_fees(
                    env,
                    a,
                    g,
                    d,
                    10000000,
                    0,
                    a.wrapping_add(20),
                );
                let a_i32_20 = mload32!(a.wrapping_add(20) as usize);
                if a_i32_20 == 0 {
                    min_mission_reward = mload64!(a as usize);
                    let q = mload64!(a.wrapping_add(8) as usize);
                    funding = q;
                    let cond_val = if funding == 0 { (min_mission_reward == 0) as i32 } else { (funding < 0) as i32 };
                    if cond_val == 0 {
                        h = mload64!(a.wrapping_add(48) as usize);
                        let r = Self::storage_key_from_str(env, 1049280, 4);
                        i = r;
                        let s = Self::pack_i128_val(env, min_mission_reward, funding);
                        let mut sv4_152_i64 = s as i64;
                        let mut sv4_144_i64 = user as i64;
                        while b != 16 {
                            mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, 0 as u64);
                            b = b.wrapping_add(8);
                        }
                        b = 0;
                        loop {
                            let t = mload64!(a.wrapping_add(144).wrapping_add(b) as usize);
                            mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, t as u64);
                            b = b.wrapping_add(8);
                        }
                        let u = val_to_i64(Vec::<Val>::new(env).into_val(env));
                        env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(h)), &Symbol::from_val(env, &val_from_i64(i)), Vec::<Val>::from_val(env, &val_from_i64(u)));
                    }
                    {
                        h = mload64!(a.wrapping_add(64) as usize);
                        i = difficulty & 18446744069414584320 | 0;
                        let v = if Map::<Val, Val>::from_val(env, &val_from_i64(h)).has(val_from_i64(i)) { 1 } else { 0 };
                        if v == 1 {
                            let w = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(h)).get(val_from_i64(i)).unwrap_or(val_from_i64(0)));
                            Self::decode_launch_from_storage(env, a.wrapping_add(80), w);
                            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
                            if loaded_val != 0 {
                                unreachable!();
                            }
                            let x = mload64!(a.wrapping_add(104) as usize);
                            j = x;
                            b = (d == 0) as i32;
                            let cond_val_2 = if b != 0 { (g as u64 > j as u64) as i32 } else { (d > 0) as i32 };
                            if cond_val_2 != 0 {
                                unreachable!();
                            }
                            min_mission_reward = mload64!(a.wrapping_add(88) as usize);
                            let y = mload64!(a.wrapping_add(96) as usize);
                            funding = y;
                            let cond_val_3 = if funding == e { (f as u64 > min_mission_reward as u64) as i32 } else { (funding < e) as i32 };
                            if cond_val_3 != 0 {
                                unreachable!();
                            }
                            if funding | min_mission_reward == 0 {
                                Self::fail_with_error_2(env, 1301375090691 /* Error(Contract, #303) */);
                                unreachable!();
                            }
                            let z = env.prng().gen_range::<u64>(1 as u64..=j as u64) as i64;
                            if b != 0 {
                                b = (z as u64 <= g as u64) as i32;
                            } else {
                                b = (d >= 0) as i32;
                            }
                            if b != 0 {
                                e = funding;
                            } else {
                                e = 0;
                            }
                            if b != 0 {
                                f = min_mission_reward;
                            } else {
                                f = 1;
                            }
                            k = funding.wrapping_sub(e).wrapping_sub(((min_mission_reward as u64) < f as u64) as i32 as u32 as i64);
                            if (funding ^ e) & (funding ^ k) < 0 {
                                unreachable!();
                            }
                            let aa = Self::u64_to_val(env, j);
                            funding = aa;
                            let ab = Self::pack_i128_val(env, min_mission_reward.wrapping_sub(f), k);
                            let mut sv4_88_i64 = ab as i64;
                            let mut loaded_val = funding as i64;
                            c = a.wrapping_add(80);
                            let ac = Self::map_new_val(
                                env,
                                1049192,
                                2,
                                c,
                                2,
                            );
                            { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(h)); m.set(val_from_i64(i), val_from_i64(ac)); val_to_i64(m.into_val(env)) };
                            env.storage().put_contract_data(a.wrapping_add(24));
                            let ae = val_to_i64(env.current_contract_address().into_val(env));
                            funding = ae;
                            Self::persist_launch_state(
                                env,
                                mload64!(a.wrapping_add(56) as usize),
                                funding,
                                user,
                                f,
                                e,
                            );
                            let af = Self::storage_key_from_str(env, 1048649, 13);
                            let ag = Self::event_topic_self_pair(env, af);
                            let ah = Self::pack_i128_val(env, g, d);
                            min_mission_reward = ah;
                            Self::pack_i128_val(env, f, e);
                            d = b as u32 as i64;
                            let mut sv4_88_i64 = min_mission_reward as i64;
                            let mut loaded_val = (difficulty & 18446744069414584320 | 0) as i64;
                            let aj = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            let sv4_152_i64 = aj as i64;
                            let sv4_144_i64 = user as i64;
                            let ak = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            env.events().publish(val_from_i64(ag), val_from_i64(ak));
                            let am = Self::pack_i128_val(env, f, e);
                            let sv4_88_i64 = am as i64;
                            let loaded_val = d as i64;
                            let ao = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            return ao;
                        }
                        Self::fail_with_error_2(env, 1288490188803 /* Error(Contract, #300) */);
                        unreachable!();
                    }
                    Self::fail_with_error_2(env, 1292785156099 /* Error(Contract, #301) */);
                    unreachable!();
                    Self::fail_with_error_2(env, 1297080123395 /* Error(Contract, #302) */);
                }
            }
        }
        unreachable!();
    }

    pub fn add_space_missions_reward(
        env: Env,
        user: Address,
        mut funds: i128,
        mut reward_difficulty: u32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut o: i64 = 0;
        a = -128;
        'label0: {
            Self::decode_i128_parts(env, a.wrapping_sub(-64), funds);
            let mut sv3_64_i64 = mload64!(a.wrapping_add(64) as usize);
            let q = mload64!(a.wrapping_add(80) as usize);
            f = q;
            let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
            i = loaded_val;
            env.storage().get_contract_data(a.wrapping_sub(-64));
            let mut sv3_64_i64 = mload64!(a.wrapping_add(64) as usize);
            if sv3_64_i64 == 0 {
                Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            } else {
                b = (reward_difficulty as u64).wrapping_shr(32 as u32) as i64 as i32;
                let r = Self::memcpy_like(
                    env,
                    a.wrapping_add(8),
                    a.wrapping_add(72),
                    56,
                );
                user.require_auth();
                j = reward_difficulty & 18446744069414584320 | 0;
                g = mload64!(a.wrapping_add(48) as usize);
                let s = Map::<Val, Val>::from_val(env, &val_from_i64(g)).len() as i64;
                k = (s as u64).wrapping_shr(32 as u32) as i64;
                l = mload64!(a.wrapping_add(40) as usize);
                c = a.wrapping_add(80);
                funds = 0;
                reward_difficulty = 0;
                loop {
                    'label2: {
                        if (reward_difficulty as u64) < k as u64 {
                            let t = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(g)).keys().get_unchecked(funds as u32));
                            d = t;
                            let u = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(g)).values().get_unchecked(funds as u32));
                            e = u;
                            Self::decode_launch_from_storage(env, a.wrapping_sub(-64), e);
                            let mut sv3_64_i64 = mload64!(a.wrapping_add(64) as usize);
                            if sv3_64_i64 != 0 {
                                unreachable!();
                            }
                            if ((d as u64).wrapping_shr(32 as u32) as i64 as i32) != b {
                                break 'label2;
                            }
                            d = mload64!(c as usize);
                            let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
                            e = loaded_val;
                            let v = val_to_i64(env.current_contract_address().into_val(env));
                            Self::persist_launch_state(
                                env,
                                l,
                                user,
                                v,
                                i,
                                f,
                            );
                            o = e.wrapping_add(i);
                            e = ((e as u64 > o as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(f));
                            if (d ^ f ^ 18446744073709551615) & (d ^ e) < 0 {
                                break 'label0;
                            }
                            let a_part_88 = mload64!(a.wrapping_add(88) as usize);
                            let w = Self::u64_to_val(env, a_part_88);
                            d = w;
                            let x = Self::pack_i128_val(env, o, e);
                            let loaded_val = x as i64;
                            let sv3_64_i64 = d as i64;
                            let y = Self::map_new_val(
                                env,
                                1049192,
                                2,
                                a.wrapping_sub(-64),
                                2,
                            );
                            { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(g)); m.set(val_from_i64(j), val_from_i64(y)); val_to_i64(m.into_val(env)) };
                            break 'label2;
                        }
                        env.storage().put_contract_data(a.wrapping_add(8));
                    }
                    funds = funds.wrapping_add(4294967296);
                    reward_difficulty += 1;
                }
                unreachable!();
            }
        }
    }

    pub fn new_launch(
        env: Env,
        dev: Address,
        mut funds_recipient: Address,
        info: String,
        asset: Address,
        mut max_supply: i128,
        mut min_price: i128,
        mut max_price: i128,
        mut launch_index: u64,
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
        a = -416;
        'label0: {
            if !((!(Address::try_from_val(env, &val_from_i64(dev)).is_ok())) as i32 | (!(Address::try_from_val(env, &val_from_i64(funds_recipient)).is_ok())) as i32 | (!(String::try_from_val(env, &val_from_i64(info)).is_ok())) as i32 | !(Address::try_from_val(env, &val_from_i64(asset)).is_ok())) {
                Self::decode_i128_parts(env, a.wrapping_add(248), max_supply);
                let mut sv8_248_i64 = mload64!(a.wrapping_add(248) as usize);
                if sv8_248_i64 == 0 {
                    b = a.wrapping_add(264);
                    max_supply = mload64!(b as usize);
                    g = mload64!(a.wrapping_add(256) as usize);
                    Self::decode_i128_parts(env, a.wrapping_add(248), min_price);
                    let mut sv8_248_i64 = mload64!(a.wrapping_add(248) as usize);
                    if sv8_248_i64 == 0 {
                        h = mload64!(b as usize);
                        i = mload64!(a.wrapping_add(256) as usize);
                        Self::decode_i128_parts(env, a.wrapping_add(248), max_price);
                        let mut sv8_248_i64 = mload64!(a.wrapping_add(248) as usize);
                        if sv8_248_i64 == 0 {
                            let l = mload64!(a.wrapping_add(264) as usize);
                            min_price = l;
                            max_price = mload64!(a.wrapping_add(256) as usize);
                            Self::decode_u64_from_val(env, a.wrapping_add(32), launch_index);
                            let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                            if a_i32_32 == 0 {
                                launch_index = mload64!(a.wrapping_add(40) as usize);
                                let m = Self::storage_has_data_key(env, 1049208);
                                if m == 0 {
                                    Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                                } else {
                                    dev.require_auth();
                                    let n = env.ledger().timestamp() as i64;
                                    if max_supply | g == 0 {
                                        Self::fail_with_error_2(env, 442381631491 /* Error(Contract, #103) */);
                                    } else {
                                        'label2: {
                                            let cond_val = if max_supply == 0 { (g < 0) as i32 } else { (max_supply > 0) as i32 };
                                            if cond_val == 0 {
                                                Self::compute_reward_and_fees(
                                                    env,
                                                    a.wrapping_add(8),
                                                    g,
                                                    max_supply,
                                                    max_price,
                                                    min_price,
                                                    a.wrapping_add(28),
                                                );
                                                let a_i32_28 = mload32!(a.wrapping_add(28) as usize);
                                                if a_i32_28 != 0 {
                                                    unreachable!();
                                                }
                                                let o = mload64!(a.wrapping_add(16) as usize);
                                                if o > 4999999 {
                                                    break 'label2;
                                                }
                                                if h | i == 0 {
                                                    Self::fail_with_error_2(env, 429496729603 /* Error(Contract, #100) */);
                                                    break 'label0;
                                                }
                                                if min_price | max_price == 0 {
                                                    Self::fail_with_error_2(env, 433791696899 /* Error(Contract, #101) */);
                                                    break 'label0;
                                                }
                                                let cond_val_2 = if min_price == h { ((max_price as u64) < i as u64) as i32 } else { (min_price < h) as i32 };
                                                if cond_val_2 != 0 {
                                                    unreachable!();
                                                }
                                                let mut sv8_264_i64 = launch_index as i64;
                                                let mut sv8_256_i64 = dev as i64;
                                                let mut sv8_248_i64 = 1 as i64;
                                                let p = Self::storage_has_data_key(env, a.wrapping_add(248));
                                                if p != 0 {
                                                    Self::fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                                                    break 'label0;
                                                }
                                                let q = val_to_i64(env.current_contract_address().into_val(env));
                                                Self::persist_launch_state(
                                                    env,
                                                    asset,
                                                    dev,
                                                    q,
                                                    g,
                                                    max_supply,
                                                );
                                                c = a.wrapping_add(184);
                                                d = a.wrapping_add(176);
                                                e = a.wrapping_add(192);
                                                mstore16!(a.wrapping_add(208) as usize, 0 as u16);
                                                Self::verify_launch_window(env, a.wrapping_sub(-64), 64);
                                                mstore64!(a.wrapping_add(152) as usize, min_price as u64);
                                                mstore64!(a.wrapping_add(136) as usize, h as u64);
                                                let sv8_264_i64 = launch_index as i64;
                                                let sv8_256_i64 = dev as i64;
                                                let sv8_248_i64 = 1 as i64;
                                                b = a.wrapping_add(248);
                                                f = a.wrapping_add(48);
                                                env.storage().put_contract_data(b, f);
                                                let r = Self::memcpy_like(
                                                    env,
                                                    b,
                                                    f,
                                                    112,
                                                );
                                                mstore64!(a.wrapping_add(392) as usize, mload64!(e as usize) as u64);
                                                mstore64!(a.wrapping_add(384) as usize, mload64!(c as usize) as u64);
                                                mstore64!(a.wrapping_add(376) as usize, mload64!(d as usize) as u64);
                                                funds_recipient = mload64!(a.wrapping_add(208) as usize);
                                                Self::storage_key_from_str(env, 1048587, 10);
                                                mstore64!(a.wrapping_add(240) as usize, n as u64);
                                                mstore64!(a.wrapping_add(232) as usize, launch_index as u64);
                                                let t = Self::launch_snapshot_to_val(env, a.wrapping_add(216));
                                                let u = Self::contract_info_to_val(env, b);
                                                env.events().publish(val_from_i64(t), val_from_i64(u));
                                            }
                                            Self::fail_with_error_2(env, 446676598787 /* Error(Contract, #104) */);
                                            break 'label0;
                                        }
                                        Self::fail_with_error_2(env, 450971566083 /* Error(Contract, #105) */);
                                        break 'label0;
                                        Self::fail_with_error_2(env, 438086664195 /* Error(Contract, #102) */);
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
        env: Env,
        mut launch_key: (Address, u64),
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut g: i64 = 0;
        let mut i: i64 = 0;
        let mut authorized_addr: i64 = 0;
        a = -208;
        Self::write_launch_key(env, a.wrapping_add(8), launch_key);
        {
            let mut sv1_8_i64 = mload64!(a.wrapping_add(8) as usize);
            if sv1_8_i64 == 0 {
                let l = mload64!(a.wrapping_add(24) as usize);
                launch_key = l;
                e = mload64!(a.wrapping_add(16) as usize);
                let m = Self::storage_has_data_key(env, 1049208);
                if m == 0 {
                    Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(176));
                    let n = mload8!(a.wrapping_add(169) as usize) as i32;
                    if n == 2 {
                        Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        let o = mload64!(a.wrapping_add(16) as usize);
                        b = a.wrapping_add(32);
                        d = mload64!(b as usize);
                        g = mload64!(a.wrapping_add(8) as usize);
                        i = mload64!(a.wrapping_add(144) as usize);
                        let mut value_hi = mload64!(a.wrapping_add(24) as usize);
                        authorized_addr = mload64!(a.wrapping_add(120) as usize);
                        address_from_i64(&env, authorized_addr).require_auth();
                        let cond_val = if d == 0 { (value_hi == 0) as i32 } else { (d < 0) as i32 };
                        if cond_val != 0 {
                            unreachable!();
                        }
                        Self::fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                    }
                }
            }
        }
        unreachable!();
        let p = val_to_i64(env.current_contract_address().into_val(env));
        Self::persist_launch_state(
            env,
            i,
            p,
            authorized_addr,
            g,
            o,
        );
        let value_hi = launch_key as i64;
        let mut value_lo = e as i64;
        let mut sv1_8_i64 = 1 as i64;
        c = a.wrapping_add(8);
        let q = Self::data_key_to_val(env, c);
        env.storage().del_contract_data(q);
        let r = Self::storage_key_from_str(env, 1048617, 13);
        d = r;
        mstore64!(a.wrapping_add(24) as usize, launch_key as u64);
        let value_lo = e as i64;
        let sv1_8_i64 = d as i64;
        let s = Self::launch_snapshot_to_val(env, c);
        env.events().publish(val_from_i64(s), val_from_i64(1));
    }

    pub fn buy(
        env: Env,
        mut user: Address,
        mut launch_key: (Address, u64),
        mut sending: i128,
        mut min_receive: i128,
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
        a = -576;
        'label0: {
            Self::write_launch_key(env, a.wrapping_add(280), launch_key);
            let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
            if sv4_280_i64 == 0 {
                b = a.wrapping_add(296);
                i = mload64!(b as usize);
                j = mload64!(a.wrapping_add(288) as usize);
                Self::decode_i128_parts(env, a.wrapping_add(280), sending);
                let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                if sv4_280_i64 == 0 {
                    launch_key = mload64!(b as usize);
                    f = mload64!(a.wrapping_add(288) as usize);
                    Self::decode_i128_parts(env, a.wrapping_add(280), min_receive);
                    let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                    if sv4_280_i64 == 0 {
                        let z = mload64!(a.wrapping_add(296) as usize);
                        g = z;
                        let mut sv4_288_i64 = mload64!(a.wrapping_add(288) as usize);
                        env.storage().get_contract_data(a.wrapping_add(280));
                        let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                        if sv4_280_i64 == 0 {
                            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let aa = Self::memcpy_like(
                                env,
                                a.wrapping_add(24),
                                a.wrapping_add(288),
                                56,
                            );
                            let mut sv4_256_i64 = j as i64;
                            let mut sv4_248_i64 = 1 as i64;
                            env.storage().get_contract_data(a.wrapping_add(280), a.wrapping_add(248));
                            let ab = mload8!(a.wrapping_add(441) as usize) as i32;
                            if ab == 2 {
                                Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                            } else {
                                let ac = Self::memcpy_like(
                                    env,
                                    a.wrapping_add(80),
                                    a.wrapping_add(280),
                                    168,
                                );
                                if launch_key | f == 0 {
                                    Self::fail_with_error_2(env, 8589934595 /* Error(Contract, #2) */);
                                } else {
                                    'label1: {
                                        {
                                            let loaded_val = mload64!(a.wrapping_add(80) as usize);
                                            v = loaded_val;
                                            l = mload64!(a.wrapping_add(96) as usize);
                                            let ad = mload64!(a.wrapping_add(88) as usize);
                                            w = ad;
                                            let ae = mload64!(a.wrapping_add(104) as usize);
                                            e = ae;
                                            if v ^ l | w ^ e != 0 {
                                                let af = Self::check_launch_state(env, a.wrapping_add(80));
                                                if af != 0 {
                                                    unreachable!();
                                                }
                                                user.require_auth();
                                                Self::buy_flow_impl(
                                                    env,
                                                    a.wrapping_add(280),
                                                    f,
                                                    launch_key,
                                                    a.wrapping_add(80),
                                                    mload32!(a.wrapping_add(72) as usize),
                                                    mload32!(a.wrapping_add(76) as usize),
                                                );
                                                let ag = mload64!(a.wrapping_add(352) as usize);
                                                q = ag;
                                                let ah = mload64!(a.wrapping_add(336) as usize);
                                                r = ah;
                                                let ai = mload64!(a.wrapping_add(320) as usize);
                                                sending = ai;
                                                let aj = mload64!(a.wrapping_add(304) as usize);
                                                o = aj;
                                                let ak = mload64!(a.wrapping_add(288) as usize);
                                                min_receive = ak;
                                                s = mload64!(a.wrapping_add(344) as usize);
                                                t = mload64!(a.wrapping_add(328) as usize);
                                                n = mload64!(a.wrapping_add(312) as usize);
                                                u = mload64!(a.wrapping_add(296) as usize);
                                                m = mload64!(a.wrapping_add(280) as usize);
                                                let al = val_to_i64(env.current_contract_address().into_val(env));
                                                p = al;
                                                h = mload64!(a.wrapping_add(56) as usize);
                                                Self::persist_launch_state(
                                                    env,
                                                    h,
                                                    user,
                                                    p,
                                                    f,
                                                    launch_key,
                                                );
                                                let cond_val = if launch_key == min_receive { (f as u64 > m as u64) as i32 } else { (launch_key > min_receive) as i32 };
                                                if cond_val == 0 {
                                                    unreachable!();
                                                }
                                                let am = val_to_i64(env.current_contract_address().into_val(env));
                                                p = am;
                                                x = launch_key.wrapping_sub(min_receive).wrapping_sub(((f as u64) < m as u64) as i32 as u32 as i64);
                                                if (launch_key ^ min_receive) & (launch_key ^ x) < 0 {
                                                    break 'label0;
                                                }
                                                Self::persist_launch_state(
                                                    env,
                                                    h,
                                                    p,
                                                    user,
                                                    f.wrapping_sub(m),
                                                    x,
                                                );
                                                break 'label1;
                                            }
                                            Self::fail_with_error_2(env, 880468295683 /* Error(Contract, #205) */);
                                            break 'label0;
                                        }
                                        Self::fail_with_error_2(env, 867583393795 /* Error(Contract, #202) */);
                                        break 'label0;
                                    }
                                    let an = val_to_i64(env.current_contract_address().into_val(env));
                                    Self::persist_launch_state(
                                        env,
                                        h,
                                        an,
                                        mload64!(a.wrapping_add(40) as usize),
                                        s,
                                        q,
                                    );
                                    let cond_val_2 = if sending == g { (sv4_288_i64 as u64 > n as u64) as i32 } else { (sending < g) as i32 };
                                    if cond_val_2 == 0 {
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 0 as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        let ao = mload64!(a.wrapping_add(16) as usize);
                                        b = mload32!(a as usize);
                                        if b != 0 {
                                            g = ao;
                                        } else {
                                            g = 0;
                                        }
                                        let a_hi = mload64!(a.wrapping_add(8) as usize);
                                        if b != 0 {
                                            f = a_hi;
                                        } else {
                                            f = 0;
                                        }
                                        launch_key = f.wrapping_add(n);
                                        f = (((launch_key as u64) < f as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(g));
                                        if (g ^ sending ^ 18446744073709551615) & (g ^ f) < 0 {
                                            break 'label0;
                                        }
                                        let sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 0 as i64;
                                        env.storage().put_contract_data(a.wrapping_add(280), launch_key, f);
                                        g = l.wrapping_add(n);
                                        l = (((g as u64) < l as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(e));
                                        if (sending ^ e ^ 18446744073709551615) & (e ^ l) < 0 {
                                            break 'label0;
                                        }
                                        mstore64!(a.wrapping_add(104) as usize, l as u64);
                                        b = a.wrapping_add(120);
                                        let mut sv5_0_i64 = mload64!(b as usize);
                                        e = sv5_0_i64;
                                        h = mload64!(a.wrapping_add(112) as usize);
                                        k = h.wrapping_add(u);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(e.wrapping_add(o));
                                        if (e ^ o ^ 18446744073709551615) & (e ^ h) < 0 {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = h as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        h = mload64!(a.wrapping_add(128) as usize);
                                        k = h.wrapping_add(m);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(min_receive.wrapping_add(e));
                                        if (e ^ min_receive ^ 18446744073709551615) & (e ^ h) < 0 {
                                            break 'label0;
                                        }
                                        if g ^ v | l ^ w == 0 {
                                            let ap = mload8!(a.wrapping_add(240) as usize) as i32;
                                            if ap == 0 {
                                                mstore8!(a.wrapping_add(240) as usize, 1 as u8);
                                                let aq = env.ledger().timestamp() as i64;
                                                e = aq;
                                                let a_part_32 = mload64!(a.wrapping_add(32) as usize);
                                                g = e.wrapping_add(a_part_32);
                                                if (g as u64) < e as u64 {
                                                    break 'label0;
                                                }
                                            }
                                        }
                                        let mut sv4_296_i64 = i as i64;
                                        let mut sv4_288_i64 = j as i64;
                                        let mut sv4_280_i64 = 1 as i64;
                                        b = a.wrapping_add(280);
                                        c = a.wrapping_add(80);
                                        env.storage().put_contract_data(b, c);
                                        let mut sv4_64_i64 = mload64!(a.wrapping_add(64) as usize);
                                        let ar = Self::token_transfer_checked(
                                            env,
                                            sv4_64_i64,
                                            t,
                                            r,
                                        );
                                        let sv4_64_i64 = ar as i64;
                                        env.storage().put_contract_data(a.wrapping_add(24));
                                        e = mload64!(a.wrapping_add(232) as usize);
                                        let at = Self::memcpy_like(
                                            env,
                                            a.wrapping_add(384),
                                            c,
                                            168,
                                        );
                                        c = at;
                                        let au = Self::storage_key_from_str(env, 1048630, 3);
                                        g = au;
                                        mstore64!(a.wrapping_add(376) as usize, f as u64);
                                        mstore64!(a.wrapping_add(368) as usize, launch_key as u64);
                                        mstore64!(a.wrapping_add(352) as usize, q as u64);
                                        mstore64!(a.wrapping_add(336) as usize, r as u64);
                                        mstore64!(a.wrapping_add(320) as usize, sending as u64);
                                        mstore64!(a.wrapping_add(304) as usize, o as u64);
                                        mstore64!(a.wrapping_add(272) as usize, e as u64);
                                        mstore64!(a.wrapping_add(264) as usize, i as u64);
                                        let sv4_296_i64 = u as i64;
                                        let sv4_288_i64 = min_receive as i64;
                                        let sv4_280_i64 = m as i64;
                                        let sv4_256_i64 = j as i64;
                                        let sv4_248_i64 = g as i64;
                                        let av = Self::launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let aw = Self::pack_i128_val(env, launch_key, f);
                                        let mut sv4_560_i64 = aw as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ax = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = ax;
                                        let ay = Self::launch_key_event_payload(env, b);
                                        launch_key = ay;
                                        Self::contract_info_to_val(env, c);
                                        let sv4_560_i64 = launch_key as i64;
                                        let sv4_552_i64 = user as i64;
                                        let ba = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(av), val_from_i64(ba));
                                    }
                                    Self::fail_with_error_2(env, 871878361091 /* Error(Contract, #203) */);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn sell(
        env: Env,
        mut user: Address,
        mut launch_key: (Address, u64),
        mut sending: i128,
        mut min_receive: i128,
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
        a = -576;
        'label0: {
            Self::write_launch_key(env, a.wrapping_add(280), launch_key);
            let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
            if sv4_280_i64 == 0 {
                b = a.wrapping_add(296);
                launch_key = mload64!(b as usize);
                k = mload64!(a.wrapping_add(288) as usize);
                Self::decode_i128_parts(env, a.wrapping_add(280), sending);
                let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                if sv4_280_i64 == 0 {
                    sending = mload64!(b as usize);
                    g = mload64!(a.wrapping_add(288) as usize);
                    Self::decode_i128_parts(env, a.wrapping_add(280), min_receive);
                    let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                    if sv4_280_i64 == 0 {
                        let u = mload64!(a.wrapping_add(296) as usize);
                        j = u;
                        f = mload64!(a.wrapping_add(288) as usize);
                        env.storage().get_contract_data(a.wrapping_add(280));
                        let mut sv4_280_i64 = mload64!(a.wrapping_add(280) as usize);
                        if sv4_280_i64 == 0 {
                            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let v = Self::memcpy_like(
                                env,
                                a.wrapping_add(24),
                                a.wrapping_add(288),
                                56,
                            );
                            let mut sv4_256_i64 = k as i64;
                            let mut sv4_248_i64 = 1 as i64;
                            env.storage().get_contract_data(a.wrapping_add(280), a.wrapping_add(248));
                            let w = mload8!(a.wrapping_add(441) as usize) as i32;
                            if w == 2 {
                                Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                            } else {
                                b = a.wrapping_add(80);
                                let x = Self::memcpy_like(
                                    env,
                                    b,
                                    a.wrapping_add(280),
                                    168,
                                );
                                {
                                    let y = Self::check_launch_state(env, b);
                                    if y == 0 {
                                        if sending | g == 0 {
                                            Self::fail_with_error_2(env, 8589934595 /* Error(Contract, #2) */);
                                            break 'label0;
                                        }
                                        user.require_auth();
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 0 as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        b = mload32!(a as usize);
                                        let a_hi = mload64!(a.wrapping_add(8) as usize);
                                        if b != 0 {
                                            e = a_hi;
                                        } else {
                                            e = 0;
                                        }
                                        let z = mload64!(a.wrapping_add(16) as usize);
                                        if b != 0 {
                                            h = z;
                                        } else {
                                            h = 0;
                                        }
                                        let cond_val = if sending == h { ((e as u64) < g as u64) as i32 } else { (h < sending) as i32 };
                                        if cond_val != 0 {
                                            unreachable!();
                                        }
                                        Self::sell_flow_impl(
                                            env,
                                            a.wrapping_add(280),
                                            g,
                                            sending,
                                            a.wrapping_add(80),
                                            mload32!(a.wrapping_add(72) as usize),
                                            mload32!(a.wrapping_add(76) as usize),
                                        );
                                        let aa = mload64!(a.wrapping_add(336) as usize);
                                        o = aa;
                                        let ab = mload64!(a.wrapping_add(304) as usize);
                                        sending = ab;
                                        let ac = mload64!(a.wrapping_add(288) as usize);
                                        min_receive = ac;
                                        let ad = mload64!(a.wrapping_add(320) as usize);
                                        l = ad;
                                        let ae = mload64!(a.wrapping_add(352) as usize);
                                        p = ae;
                                        q = mload64!(a.wrapping_add(328) as usize);
                                        m = mload64!(a.wrapping_add(296) as usize);
                                        g = mload64!(a.wrapping_add(280) as usize);
                                        n = mload64!(a.wrapping_add(312) as usize);
                                        r = mload64!(a.wrapping_add(344) as usize);
                                        let af = val_to_i64(env.current_contract_address().into_val(env));
                                        i = af;
                                        let loaded_val = mload64!(a.wrapping_add(56) as usize);
                                        s = loaded_val;
                                        Self::persist_launch_state(
                                            env,
                                            s,
                                            i,
                                            mload64!(a.wrapping_add(40) as usize),
                                            r,
                                            p,
                                        );
                                        let cond_val_2 = if j == l { (f as u64 > n as u64) as i32 } else { (j > l) as i32 };
                                        if cond_val_2 != 0 {
                                            unreachable!();
                                        }
                                        let ag = val_to_i64(env.current_contract_address().into_val(env));
                                        Self::persist_launch_state(
                                            env,
                                            s,
                                            ag,
                                            user,
                                            n,
                                            l,
                                        );
                                        j = h.wrapping_sub(min_receive).wrapping_sub(((e as u64) < g as u64) as i32 as u32 as i64);
                                        if (min_receive ^ h) & (h ^ j) < 0 {
                                            break 'label0;
                                        }
                                        let sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 0 as i64;
                                        h = e.wrapping_sub(g);
                                        env.storage().put_contract_data(a.wrapping_add(280), h, j);
                                        b = a.wrapping_add(104);
                                        let mut sv5_0_i64 = mload64!(b as usize);
                                        e = sv5_0_i64;
                                        let mut sv4_96_i64 = mload64!(a.wrapping_add(96) as usize);
                                        f = sv4_96_i64;
                                        i = e.wrapping_sub(min_receive).wrapping_sub(((f as u64) < g as u64) as i32 as u32 as i64);
                                        if (e ^ min_receive) & (e ^ i) < 0 {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_96_i64 = f.wrapping_sub(g) as i64;
                                        b = a.wrapping_add(120);
                                        e = sv5_0_i64;
                                        let mut sv4_112_i64 = mload64!(a.wrapping_add(112) as usize);
                                        f = sv4_112_i64;
                                        i = e.wrapping_sub(sending).wrapping_sub(((f as u64) < m as u64) as i32 as u32 as i64);
                                        if (e ^ sending) & (e ^ i) < 0 {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_112_i64 = f.wrapping_sub(m) as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        f = mload64!(a.wrapping_add(128) as usize);
                                        i = f.wrapping_add(m);
                                        f = (((i as u64) < f as u64) as i32 as u32 as i64).wrapping_add(sending.wrapping_add(e));
                                        if (e ^ sending ^ 18446744073709551615) & (e ^ f) < 0 {
                                            break 'label0;
                                        }
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 1 as i64;
                                        b = a.wrapping_add(280);
                                        c = a.wrapping_add(80);
                                        env.storage().put_contract_data(b, c);
                                        let mut sv4_64_i64 = mload64!(a.wrapping_add(64) as usize);
                                        let ah = Self::token_transfer_checked(
                                            env,
                                            sv4_64_i64,
                                            q,
                                            o,
                                        );
                                        let sv4_64_i64 = ah as i64;
                                        env.storage().put_contract_data(a.wrapping_add(24));
                                        e = mload64!(a.wrapping_add(232) as usize);
                                        let ai = Self::memcpy_like(
                                            env,
                                            a.wrapping_add(384),
                                            c,
                                            168,
                                        );
                                        c = ai;
                                        let aj = Self::storage_key_from_str(env, 1048633, 4);
                                        f = aj;
                                        mstore64!(a.wrapping_add(376) as usize, j as u64);
                                        mstore64!(a.wrapping_add(368) as usize, h as u64);
                                        mstore64!(a.wrapping_add(352) as usize, p as u64);
                                        mstore64!(a.wrapping_add(336) as usize, o as u64);
                                        mstore64!(a.wrapping_add(320) as usize, l as u64);
                                        mstore64!(a.wrapping_add(304) as usize, sending as u64);
                                        mstore64!(a.wrapping_add(272) as usize, e as u64);
                                        mstore64!(a.wrapping_add(264) as usize, launch_key as u64);
                                        let sv4_296_i64 = m as i64;
                                        let sv4_288_i64 = min_receive as i64;
                                        let sv4_280_i64 = g as i64;
                                        let sv4_256_i64 = k as i64;
                                        let sv4_248_i64 = f as i64;
                                        let ak = Self::launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let al = Self::pack_i128_val(env, h, j);
                                        let mut sv4_560_i64 = al as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let am = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = am;
                                        let an = Self::launch_key_event_payload(env, b);
                                        sending = an;
                                        Self::contract_info_to_val(env, c);
                                        let sv4_560_i64 = sending as i64;
                                        let sv4_552_i64 = user as i64;
                                        let ap = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(ak), val_from_i64(ap));
                                    }
                                    Self::fail_with_error_2(env, 867583393795 /* Error(Contract, #202) */);
                                    break 'label0;
                                }
                                Self::fail_with_error_2(env, 876173328387 /* Error(Contract, #204) */);
                                break 'label0;
                                Self::fail_with_error_2(env, 871878361091 /* Error(Contract, #203) */);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn claim_launch_funds(
        env: Env,
        mut launch_key: (Address, u64),
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        a = -368;
        Self::write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            let mut sv1_168_i64 = mload64!(a.wrapping_add(168) as usize);
            if sv1_168_i64 == 0 {
                let g = mload64!(a.wrapping_add(184) as usize);
                launch_key = g;
                c = mload64!(a.wrapping_add(176) as usize);
                env.storage().get_contract_data(a.wrapping_add(168));
                let mut sv1_168_i64 = mload64!(a.wrapping_add(168) as usize);
                if sv1_168_i64 == 0 {
                    Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    let h = mload64!(a.wrapping_add(208) as usize);
                    d = h;
                    env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                    let i = mload8!(a.wrapping_add(329) as usize) as i32;
                    if i == 2 {
                        Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        let j = Self::memcpy_like(
                            env,
                            a,
                            a.wrapping_add(168),
                            168,
                        );
                        a = j;
                        let k = mload8!(a.wrapping_add(161) as usize) as i32;
                        if k == 0 {
                            let l = Self::check_launch_state(env, a);
                            if l == 0 {
                                Self::fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                                break 'label0;
                            }
                            let m = val_to_i64(env.current_contract_address().into_val(env));
                            let n = mload64!(a.wrapping_add(40) as usize);
                            Self::persist_launch_state(
                                env,
                                d,
                                m,
                                mload64!(a.wrapping_add(120) as usize),
                                mload64!(a.wrapping_add(32) as usize),
                                n,
                            );
                            mstore8!(a.wrapping_add(161) as usize, 1 as u8);
                            let mut sv1_176_i64 = c as i64;
                            let mut sv1_168_i64 = 1 as i64;
                            b = a.wrapping_add(168);
                            env.storage().put_contract_data(b, a);
                            d = mload64!(a.wrapping_add(152) as usize);
                            let o = Self::storage_key_from_str(env, 1048597, 20);
                            mstore64!(a.wrapping_add(192) as usize, d as u64);
                            mstore64!(a.wrapping_add(184) as usize, launch_key as u64);
                            let sv1_176_i64 = c as i64;
                            let sv1_168_i64 = o as i64;
                            let p = Self::launch_snapshot_to_val(env, b);
                            let q = Self::contract_info_to_val(env, a);
                            env.events().publish(val_from_i64(p), val_from_i64(q));
                        }
                        Self::fail_with_error_2(env, 884763262979 /* Error(Contract, #206) */);
                    }
                }
            }
        }
    }

    pub fn claim_launch_balance(
        env: Env,
        user: Address,
        mut launch_key: (Address, u64),
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
        a = -448;
        'label0: {
            Self::write_launch_key(env, a.wrapping_add(232), launch_key);
            let mut sv2_232_i64 = mload64!(a.wrapping_add(232) as usize);
            if sv2_232_i64 == 0 {
                let m = mload64!(a.wrapping_add(248) as usize);
                launch_key = m;
                f = mload64!(a.wrapping_add(240) as usize);
                env.storage().get_contract_data(a.wrapping_add(232));
                let mut sv2_232_i64 = mload64!(a.wrapping_add(232) as usize);
                if sv2_232_i64 == 0 {
                    Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                } else {
                    let n = mload64!(a.wrapping_add(264) as usize);
                    j = n;
                    let mut sv2_408_i64 = f as i64;
                    let mut sv2_400_i64 = 1 as i64;
                    env.storage().get_contract_data(a.wrapping_add(232), a.wrapping_add(400));
                    let o = mload8!(a.wrapping_add(393) as usize) as i32;
                    if o == 2 {
                        Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                    } else {
                        b = a.wrapping_sub(-64);
                        let p = Self::memcpy_like(
                            env,
                            b,
                            a.wrapping_add(232),
                            168,
                        );
                        let q = Self::check_launch_state(env, b);
                        if q == 0 {
                            Self::fail_with_error_2(env, 863288426499 /* Error(Contract, #201) */);
                        } else {
                            let mut sv2_256_i64 = user as i64;
                            let mut sv2_248_i64 = launch_key as i64;
                            let mut sv2_240_i64 = f as i64;
                            let mut sv2_232_i64 = 0 as i64;
                            c = a.wrapping_add(232);
                            env.storage().get_contract_data(a.wrapping_add(40), c);
                            let r = mload64!(a.wrapping_add(56) as usize);
                            h = r;
                            b = mload32!(a.wrapping_add(40) as usize);
                            i = mload64!(a.wrapping_add(48) as usize);
                            let s = val_to_i64(env.current_contract_address().into_val(env));
                            d = s;
                            if b != 0 {
                                i = i;
                            } else {
                                i = 0;
                            }
                            if b != 0 {
                                h = h;
                            } else {
                                h = 0;
                            }
                            Self::persist_launch_state(
                                env,
                                mload64!(a.wrapping_add(200) as usize),
                                d,
                                user,
                                i,
                                h,
                            );
                            let sv2_256_i64 = user as i64;
                            let mut sv2_248_i64 = launch_key as i64;
                            let mut sv2_240_i64 = f as i64;
                            let mut sv2_232_i64 = 0 as i64;
                            let t = Self::data_key_to_val(env, c);
                            env.storage().del_contract_data(t);
                            b = a.wrapping_add(136);
                            d = mload64!(b as usize);
                            e = mload64!(a.wrapping_add(128) as usize);
                            g = e.wrapping_add(i);
                            e = (((g as u64) < e as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(h));
                            if (d ^ h ^ 18446744073709551615) & (d ^ e) >= 0 {
                                let sv2_248_i64 = launch_key as i64;
                                let mut sv2_240_i64 = f as i64;
                                let mut sv2_232_i64 = 1 as i64;
                                env.storage().put_contract_data(a.wrapping_add(232), a.wrapping_sub(-64));
                                let u = mload64!(a.wrapping_add(120) as usize);
                                Self::compute_reward_and_fees(
                                    env,
                                    a.wrapping_add(16),
                                    i,
                                    h,
                                    mload64!(a.wrapping_add(112) as usize),
                                    u,
                                    a.wrapping_add(36),
                                );
                                let a_i32_36 = mload32!(a.wrapping_add(36) as usize);
                                if a_i32_36 == 0 {
                                    d = mload64!(a.wrapping_add(64) as usize);
                                    let v = mload64!(a.wrapping_add(72) as usize);
                                    e = v;
                                    if d | e != 0 {
                                        g = mload64!(a.wrapping_add(16) as usize);
                                        let w = mload64!(a.wrapping_add(24) as usize);
                                        k = w;
                                        if (g | k ^ 9223372036854775808 != 0) as i32 & d & e == 18446744073709551615 {
                                            Self::claim_launch_balance_impl(
                                                env,
                                                a,
                                                g,
                                                k,
                                                d,
                                                e,
                                            );
                                            e = mload64!(a as usize);
                                            let x = mload64!(a.wrapping_add(8) as usize);
                                            d = x;
                                            let cond_val = if d == 0 { (e == 0) as i32 } else { (d < 0) as i32 };
                                            if cond_val == 0 {
                                                let y = Self::storage_key_from_str(env, 1049284, 4);
                                                g = y;
                                                let z = Self::pack_i128_val(env, e, d);
                                                let mut sv2_408_i64 = z as i64;
                                                let mut sv2_400_i64 = user as i64;
                                                b = 0;
                                                while b != 16 {
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, 0 as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                b = 0;
                                                loop {
                                                    let aa = mload64!(a.wrapping_add(400).wrapping_add(b) as usize);
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, aa as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                let ab = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(j)), &Symbol::from_val(env, &val_from_i64(g)), Vec::<Val>::from_val(env, &val_from_i64(ab)));
                                                break 'label0;
                                            }
                                            j = mload64!(a.wrapping_add(216) as usize);
                                            let ac = Self::storage_key_from_str(env, 1048637, 12);
                                            g = ac;
                                            mstore64!(a.wrapping_add(256) as usize, j as u64);
                                            mstore64!(a.wrapping_add(248) as usize, launch_key as u64);
                                            let sv2_240_i64 = f as i64;
                                            let sv2_232_i64 = g as i64;
                                            let ad = Self::launch_snapshot_to_val(env, a.wrapping_add(232));
                                            let ae = Self::pack_i128_val(env, i, h);
                                            f = ae;
                                            let af = Self::pack_i128_val(env, e, d);
                                            let sv2_408_i64 = af as i64;
                                            let sv2_400_i64 = f as i64;
                                            val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            let ah = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            env.events().publish(val_from_i64(ad), val_from_i64(ah));
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
        env: Env,
        mut launch_key: (Address, u64),
        mut sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut e: i64 = 0;
        a = -368;
        Self::write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            {
                let a_part_168_3 = mload64!(a.wrapping_add(168) as usize);
                if a_part_168_3 == 0 {
                    b = a.wrapping_add(184);
                    launch_key = mload64!(b as usize);
                    Self::decode_i128_parts(env, a.wrapping_add(168), sending);
                    let a_part_168_2 = mload64!(a.wrapping_add(168) as usize);
                    if a_part_168_2 == 0 {
                        sending = mload64!(b as usize);
                        e = mload64!(a.wrapping_add(176) as usize);
                        env.storage().get_contract_data(a.wrapping_add(168));
                        let a_part_168 = mload64!(a.wrapping_add(168) as usize);
                        if a_part_168 == 0 {
                            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let g = mload32!(a.wrapping_add(228) as usize);
                            b = g;
                            let h = mload32!(a.wrapping_add(224) as usize);
                            env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                            let i = mload8!(a.wrapping_add(329) as usize) as i32;
                            if i != 2 {
                                break 'label0;
                            }
                            Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                        }
                    }
                }
            }
            unreachable!();
        }
        let j = Self::memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = j;
        Self::buy_flow_impl(
            env,
            a.wrapping_add(168),
            e,
            sending,
            a,
            h,
            b,
        );
        let k = Self::launch_key_event_payload(env, a.wrapping_add(168));
        k
    }

    pub fn calculate_sell(
        env: Env,
        mut launch_key: (Address, u64),
        mut sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut e: i64 = 0;
        a = -368;
        Self::write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            {
                let a_part_168_3 = mload64!(a.wrapping_add(168) as usize);
                if a_part_168_3 == 0 {
                    b = a.wrapping_add(184);
                    launch_key = mload64!(b as usize);
                    Self::decode_i128_parts(env, a.wrapping_add(168), sending);
                    let a_part_168_2 = mload64!(a.wrapping_add(168) as usize);
                    if a_part_168_2 == 0 {
                        sending = mload64!(b as usize);
                        e = mload64!(a.wrapping_add(176) as usize);
                        env.storage().get_contract_data(a.wrapping_add(168));
                        let a_part_168 = mload64!(a.wrapping_add(168) as usize);
                        if a_part_168 == 0 {
                            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
                        } else {
                            let g = mload32!(a.wrapping_add(228) as usize);
                            b = g;
                            let h = mload32!(a.wrapping_add(224) as usize);
                            env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                            let i = mload8!(a.wrapping_add(329) as usize) as i32;
                            if i != 2 {
                                break 'label0;
                            }
                            Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
                        }
                    }
                }
            }
            unreachable!();
        }
        let j = Self::memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = j;
        Self::sell_flow_impl(
            env,
            a.wrapping_add(168),
            e,
            sending,
            a,
            h,
            b,
        );
        let k = Self::launch_key_event_payload(env, a.wrapping_add(168));
        k
    }

    pub fn get_launch_data(
        env: Env,
        mut launch_key: (Address, u64),
    ) -> Launch {
        let mut a: i32 = -368;
        Self::write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            let a_part_168 = mload64!(a.wrapping_add(168) as usize);
            if a_part_168 == 0 {
                launch_key = mload64!(a.wrapping_add(176) as usize);
                mload64!(a.wrapping_add(184) as usize);
                env.storage().get_contract_data(a.wrapping_add(168), a.wrapping_add(336));
                let d = mload8!(a.wrapping_add(329) as usize) as i32;
                if d != 2 {
                    break 'label0;
                }
                Self::fail_with_error_2(env, 858993459203 /* Error(Contract, #200) */);
            }
            unreachable!();
        }
        let e = Self::memcpy_like(
            env,
            a,
            a.wrapping_add(168),
            168,
        );
        a = e;
        let f = Self::contract_info_to_val(env, a);
        f
    }

    pub fn get_contract_info(
        env: Env,
    ) -> ContractInfo {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -128;
        env.storage().get_contract_data(a.wrapping_sub(-64));
        let a_part_64 = mload64!(a.wrapping_add(64) as usize);
        if a_part_64 == 0 {
            Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            unreachable!();
        }
        b = a.wrapping_add(8);
        let e = Self::memcpy_like(
            env,
            b,
            a.wrapping_add(72),
            56,
        );
        let f = Self::launch_record_to_val(env, b);
        f
    }

    pub fn get_launch_balance(
        env: Env,
        launch_key: (Address, u64),
        user: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -64;
        Self::write_launch_key(env, a.wrapping_add(32), launch_key);
        let a_part_32 = mload64!(a.wrapping_add(32) as usize);
        if (!(a_part_32 == 0)) as i32 | Address::try_from_val(env, &val_from_i64(user)).is_ok() {
            mload64!(a.wrapping_add(48) as usize);
            env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(32));
            let value_lo = mload64!(a.wrapping_add(16) as usize);
            b = mload32!(a.wrapping_add(8) as usize);
            let f = mload64!(a.wrapping_add(24) as usize);
            let call_arg1 = if b != 0 { value_lo } else { 0 };
            let call_arg2 = if b != 0 { f } else { 0 };
            let g = Self::pack_i128_val(env, call_arg1, call_arg2);
            return g;
        }
    }

    pub fn version(
        env: Env,
    ) -> (u32, u32, u32) {
        let a: i32 = -32;
        let d = val_to_i64(Vec::<Val>::new(env).into_val(env));
        d
    }
}

impl RandomContract {

    fn decode_u64_from_val(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let mut a: i32 = 0;
        let c: i64;
        'label0: {
            a = arg1 as i32 & 255;
            if a != 64 {
                if a != 6 {
                    c = Error(Value, UnexpectedType);
                    break 'label0;
                }
                c = (arg1 as u64) as i64;
                break 'label0;
            }
            val_from_i64(arg1).as_u64().unwrap_or(0) as i64;
        }
    }


    fn data_key_to_val(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        a = -32;
        let f: i64;
        'label0: {
            {
                {
                    {
                        let g = Self::func64(env, 1048674, 6);
                        b = g;
                        c = mload64!(arg0.wrapping_add(8) as usize);
                        let mut arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
                        let h = Self::u64_to_val(env, arg0_lo);
                        let mut sv1_16_i64 = h as i64;
                        let mut vec_builder = c as i64;
                        let mut sv1_0_i64 = b as i64;
                        let i = val_to_i64(vec![&env, val_from_i64(sv1_0_i64), val_from_i64(vec_builder)].into_val(env));
                        f = i;
                        break 'label0;
                    }
                    let j = Self::func64(env, 1048680, 13);
                    b = j;
                    c = mload64!(arg0.wrapping_add(8) as usize);
                    let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
                    let k = Self::u64_to_val(env, arg0_lo);
                    mload64!(arg0.wrapping_add(24) as usize);
                    let sv1_16_i64 = k as i64;
                    let mut vec_builder = c as i64;
                    let mut sv1_0_i64 = b as i64;
                    let l = val_to_i64(vec![&env, val_from_i64(sv1_0_i64), val_from_i64(vec_builder)].into_val(env));
                    f = l;
                    break 'label0;
                }
                let m = Self::func64(env, 1048693, 12);
                b = m;
                let vec_builder = mload64!(arg0.wrapping_add(8) as usize);
                let mut sv1_0_i64 = b as i64;
                let n = val_to_i64(vec![&env, val_from_i64(sv1_0_i64), val_from_i64(vec_builder)].into_val(env));
                f = n;
                break 'label0;
            }
            let o = Self::func64(env, 1048662, 12);
            let sv1_0_i64 = o as i64;
            let p = val_to_i64(vec![&env, val_from_i64(sv1_0_i64), val_from_i64(vec_builder)].into_val(env));
            f = p;
        }
        f
    }

    fn call_eq_one(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let a = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (a == 1) as i32
    }

    fn decode_i128_parts(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let c: i64;
        'label0: {
            'label1: {
                a = arg1 as i32 & 255;
                if a != 69 {
                    if a == 11 {
                        mstore64!(arg0.wrapping_add(16) as usize, arg1.wrapping_shr(63 as u32) as u64);
                        let mut svarg0_8_i64 = arg1 as i64;
                        break 'label1;
                    }
                    svarg0_8_i64 = Error(Value, UnexpectedType) as i64;
                    break 'label0;
                }
                let d = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64);
                let e = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64);
                arg1 = e;
                mstore64!(arg0.wrapping_add(16) as usize, d as u64);
                let svarg0_8_i64 = arg1 as i64;
            }
        }
    }


    fn func48(
        env: &Env,
        arg0: i64,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    ) {
        if arg2 != arg4 {
            unreachable!();
        }
        let a = 0;
    }


    fn contract_info_to_val(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let a: i32 = -128;
        mload8!(arg0.wrapping_add(161) as usize) as i64;
        let r = mload64!(arg0.wrapping_add(104) as usize);
        let arg0_part_96 = mload64!(arg0.wrapping_add(96) as usize);
        Self::pack_i128_val(env, arg0_part_96, r);
        let t = mload64!(arg0.wrapping_add(8) as usize);
        let arg0_lo_2 = mload64!(arg0 as usize);
        Self::pack_i128_val(env, arg0_lo_2, t);
        let loaded_val = mload64!(arg0.wrapping_add(80) as usize);
        let v = mload64!(arg0.wrapping_add(88) as usize);
        Self::pack_i128_val(env, loaded_val, v);
        let x = mload64!(arg0.wrapping_add(40) as usize);
        let arg0_part_32 = mload64!(arg0.wrapping_add(32) as usize);
        Self::pack_i128_val(env, arg0_part_32, x);
        mload8!(arg0.wrapping_add(160) as usize) as i64;
        let arg0_part_144 = mload64!(arg0.wrapping_add(144) as usize);
        Self::u64_to_val(env, arg0_part_144);
        let ab = mload64!(arg0.wrapping_add(56) as usize);
        let arg0_part_48 = mload64!(arg0.wrapping_add(48) as usize);
        Self::pack_i128_val(env, arg0_part_48, ab);
        let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
        let ad = mload64!(arg0.wrapping_add(24) as usize);
        Self::pack_i128_val(env, arg0_lo, ad);
        let arg0_part_152 = mload64!(arg0.wrapping_add(152) as usize);
        Self::u64_to_val(env, arg0_part_152);
        let ag = mload64!(arg0.wrapping_add(72) as usize);
        let arg0_part_64 = mload64!(arg0.wrapping_add(64) as usize);
        Self::pack_i128_val(env, arg0_part_64, ag);
        let ai = Self::map_new_val(
            env,
            1049040,
            15,
            a.wrapping_add(8),
            15,
        );
        ai
    }


    fn pack_i128_val(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i64 {
        if (arg1 ^ arg0.wrapping_shr(63 as u32) != 0) as i32 | !(arg0.wrapping_sub(18410715276690587648) as u64 > 72057594037927935 as u64) {
            return arg0 | 0;
        }
        let a = val_to_i64(Val::from_i128(((arg1 as i128) << 64) | (arg0 as u64 as i128)));
        a
    }


    fn storage_has_data_key(
        env: &Env,
        arg0: i32,
    ) -> i32 {
        let a = Self::data_key_to_val(env, arg0);
        let b = Self::call_eq_one(env, a, 0);
        b
    }


    fn launch_record_to_val(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let a: i32 = -64;
        let arg0_hi_2 = mload64!(arg0.wrapping_add(8) as usize);
        Self::u64_to_val(env, arg0_hi_2);
        mload64!(arg0.wrapping_add(24) as usize);
        let j = Self::map_new_val(
            env,
            1048820,
            8,
            a,
            8,
        );
        j
    }

    fn write_launch_key(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = -32;
        'label0: {
            {
                while b != 16 {
                    mstore64!(a.wrapping_add(16).wrapping_add(b) as usize, 0 as u64);
                    b = b.wrapping_add(8);
                }
                let e = 0;
                c = mload64!(a.wrapping_add(16) as usize);
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                Self::decode_u64_from_val(env, a, value_hi);
                arg1 = mload64!(a.wrapping_add(8) as usize);
                if mload32!(a as usize) == 0 {
                    let mut svarg0_8_i64 = c as i64;
                    mstore64!(arg0.wrapping_add(16) as usize, arg1 as u64);
                    break 'label0;
                }
                svarg0_8_i64 = arg1 as i64;
                break 'label0;
                svarg0_8_i64 = Error(Value, UnexpectedType) as i64;
                break 'label0;
            }
            svarg0_8_i64 = Error(Value, UnexpectedType) as i64;
        }
    }

    fn launch_key_event_payload(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let a: i32 = -48;
        let g = mload64!(arg0.wrapping_add(8) as usize);
        let arg0_lo_2 = mload64!(arg0 as usize);
        Self::pack_i128_val(env, arg0_lo_2, g);
        let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
        let i = mload64!(arg0.wrapping_add(24) as usize);
        Self::pack_i128_val(env, arg0_lo, i);
        let k = mload64!(arg0.wrapping_add(40) as usize);
        let arg0_part_32 = mload64!(arg0.wrapping_add(32) as usize);
        Self::pack_i128_val(env, arg0_part_32, k);
        let m = mload64!(arg0.wrapping_add(56) as usize);
        let arg0_part_48 = mload64!(arg0.wrapping_add(48) as usize);
        Self::pack_i128_val(env, arg0_part_48, m);
        let o = mload64!(arg0.wrapping_add(72) as usize);
        let arg0_part_64 = mload64!(arg0.wrapping_add(64) as usize);
        Self::pack_i128_val(env, arg0_part_64, o);
        let q = val_to_i64(Vec::<Val>::new(env).into_val(env));
        q
    }


    fn launch_snapshot_to_val(
        env: &Env,
        mut arg0: i32,
    ) -> i64 {
        let vec_builder: i32 = -48;
        let e = mload64!(arg0.wrapping_add(16) as usize);
        Self::u64_to_val(env, e);
        let g = mload64!(arg0.wrapping_add(24) as usize);
        Self::u64_to_val(env, g);
        let arg0_lo = mload64!(arg0 as usize);
        let arg0_hi = mload64!(arg0.wrapping_add(8) as usize);
        val_to_i64(vec![&env, val_from_i64(arg0_lo), val_from_i64(arg0_hi)].into_val(env));
        arg0 = 0;
        let j: i64;
        loop {
            let k: i64;
            mstore64!(vec_builder.wrapping_add(24).wrapping_add(arg0) as usize, 0 as u64);
            arg0 = arg0.wrapping_add(8);
            unreachable!();
        }
    }

    fn u64_to_val(
        env: &Env,
        arg0: i64,
    ) -> i64 {
        if arg0 as u64 <= 72057594037927935 as u64 {
            return arg0 | 0;
        }
        let a = val_to_i64(Val::from_u64(arg0 as u64));
        a
    }

    fn map_new_val(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> i64 {
        if arg1 != arg3 {
            unreachable!();
        }
        let a = val_to_i64(Map::<Val, Val>::new(env).into_val(env));
        a
    }

    fn event_topic_self_pair(
        env: &Env,
        arg0: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = -16;
        let vec_builder = arg0 as i64;
        c = 0;
        b = 1;
        loop {
            b -= 1;
            c = arg0;
        }
        let sv1_8_i64 = c as i64;
        let e = val_to_i64(vec![&env, val_from_i64(vec_builder), val_from_i64(sv1_8_i64)].into_val(env));
        e
    }

    fn func64(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        'label0: {
            if arg1 as u32 <= 9 as u32 {
                b = arg1;
                c = arg0;
                while b != 0 {
                    let e: i32;
                    'label2: {
                        let f = mload8!(c as usize) as i32;
                        a = f;
                        let g = 1;
                        if a == 95 {
                            e = g;
                        } else {
                            if (a.wrapping_sub(48) & 255) as u32 >= 10 as u32 {
                                if (a.wrapping_sub(65) & 255) as u32 >= 26 as u32 {
                                    if (a.wrapping_sub(97) & 255) as u32 > 25 as u32 {
                                        break 'label0;
                                    }
                                    e = a.wrapping_sub(59);
                                    break 'label2;
                                }
                                e = a.wrapping_sub(53);
                                break 'label2;
                            }
                            e = a.wrapping_sub(46);
                        }
                    }
                    d = e as u32 as i64 & 255 | d;
                    b -= 1;
                    c += 1;
                }
                return d | 0 /* Symbol() */;
            }
        }
        let h = val_to_i64(Symbol::new(env, ""));
        h
    }

    fn check_launch_state(
        env: &Env,
        arg0: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        {
            let b = mload8!(arg0.wrapping_add(160) as usize) as i32;
            if b != 0 {
                let c = env.ledger().timestamp() as i64;
                let arg0_part_144 = mload64!(arg0.wrapping_add(144) as usize);
                if (c as u64) >= arg0_part_144 as u64 {
                    let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
                    let d = mload64!(arg0.wrapping_add(8) as usize);
                    let e = mload64!(arg0.wrapping_add(24) as usize);
                    let arg0_lo_2 = mload64!(arg0 as usize);
                    a = (arg0_lo_2 ^ arg0_lo | d ^ e == 0) as i32;
                }
            }
        }
        a
    }


    fn func67(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        mut arg3: i64,
        mut arg4: i64,
        arg5: i64,
        mut arg6: i64,
        mut arg7: i64,
        mut arg8: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = -192;
        'label0: {
            'label1: {
                d = arg4 ^ arg6;
                if d | arg3 ^ arg5 != 0 {
                    if arg1 ^ arg7 | arg2 ^ arg8 != 0 {
                        Self::compute_reward_and_fees(
                            env,
                            a.wrapping_add(168),
                            arg7,
                            arg8,
                            20000000,
                            0,
                            a.wrapping_add(188),
                        );
                        let f = mload64!(a.wrapping_add(176) as usize);
                        arg7 = f;
                        arg8 = mload64!(a.wrapping_add(168) as usize);
                        let g = Self::func69(env, arg5, arg6);
                        let h = Self::func69(env, arg1, arg2);
                        let i = val_to_i64(I256::try_from_val(env, &val_from_i64(g)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(h)).unwrap()).into_val(env));
                        let j = Self::func69(env, 10000000, 0);
                        let k = val_to_i64(I256::try_from_val(env, &val_from_i64(i)).unwrap().div(&I256::try_from_val(env, &val_from_i64(j)).unwrap()).into_val(env));
                        d = k;
                        let l = Self::func69(env, arg3, arg4);
                        let m = Self::func69(env, arg5, arg6);
                        let n = val_to_i64(I256::try_from_val(env, &val_from_i64(l)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(m)).unwrap()).into_val(env));
                        let o = Self::func69(env, arg1, arg2);
                        let p = val_to_i64(I256::try_from_val(env, &val_from_i64(n)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(o)).unwrap()).into_val(env));
                        let q = Self::func69(env, arg1, arg2);
                        let r = val_to_i64(I256::try_from_val(env, &val_from_i64(p)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(q)).unwrap()).into_val(env));
                        arg1 = r;
                        let a_i32_188 = mload32!(a.wrapping_add(188) as usize);
                        if a_i32_188 != 0 {
                            unreachable!();
                        }
                        let s = Self::func69(env, arg8, arg7);
                        let t = val_to_i64(I256::try_from_val(env, &val_from_i64(arg1)).unwrap().div(&I256::try_from_val(env, &val_from_i64(s)).unwrap()).into_val(env));
                        arg1 = t;
                        Self::func70(env, a.wrapping_add(144), d);
                        let u = mload64!(a.wrapping_add(160) as usize);
                        arg3 = mload64!(a.wrapping_add(152) as usize);
                        b = mload32!(a.wrapping_add(144) as usize);
                        Self::func70(env, a.wrapping_add(120), arg1);
                        if b != 0 {
                            arg1 = u;
                        } else {
                            arg1 = 0;
                        }
                        let v = mload64!(a.wrapping_add(136) as usize);
                        c = mload32!(a.wrapping_add(120) as usize);
                        if c != 0 {
                            arg2 = v;
                        } else {
                            arg2 = 0;
                        }
                        if b != 0 {
                            arg3 = arg3;
                        } else {
                            arg3 = 0;
                        }
                        let a_part_128 = mload64!(a.wrapping_add(128) as usize);
                        arg4 = arg3.wrapping_add((if c != 0 { a_part_128 } else { 0 }));
                        arg3 = (((arg4 as u64) < arg3 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg2));
                        if (arg1 ^ arg2 ^ 18446744073709551615) & (arg1 ^ arg3) >= 0 {
                            break 'label1;
                        }
                        break 'label0;
                    }
                    Self::compute_reward_and_fees(
                        env,
                        a.wrapping_add(96),
                        arg5,
                        arg6,
                        arg1,
                        arg2,
                        a.wrapping_add(116),
                    );
                    let a_i32_116 = mload32!(a.wrapping_add(116) as usize);
                    if a_i32_116 != 0 {
                        unreachable!();
                    }
                    arg7 = arg4.wrapping_sub(arg6).wrapping_sub(((arg3 as u64) < arg5 as u64) as i32 as u32 as i64);
                    if d & (arg4 ^ arg7) < 0 {
                        break 'label0;
                    }
                    let w = mload64!(a.wrapping_add(104) as usize);
                    arg4 = w;
                    arg6 = mload64!(a.wrapping_add(96) as usize);
                    Self::compute_reward_and_fees(
                        env,
                        a.wrapping_add(72),
                        arg3.wrapping_sub(arg5),
                        arg7,
                        arg1,
                        arg2,
                        a.wrapping_add(92),
                    );
                    let a_i32_92 = mload32!(a.wrapping_add(92) as usize);
                    if a_i32_92 != 0 {
                        unreachable!();
                    }
                    let loaded_val = mload64!(a.wrapping_add(72) as usize);
                    let x = mload64!(a.wrapping_add(80) as usize);
                    Self::claim_launch_balance_impl(
                        env,
                        a.wrapping_add(56),
                        loaded_val,
                        x,
                        0,
                        0,
                    );
                    let y = mload64!(a.wrapping_sub(-64) as usize);
                    arg1 = y;
                    let z = arg1;
                    let a_part_56 = mload64!(a.wrapping_add(56) as usize);
                    arg2 = arg6.wrapping_add(a_part_56);
                    arg1 = (((arg2 as u64) < arg6 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg4));
                    if (arg4 ^ z ^ 18446744073709551615) & (arg4 ^ arg1) < 0 {
                        break 'label0;
                    }
                    Self::claim_launch_balance_impl(
                        env,
                        a.wrapping_add(40),
                        arg2,
                        arg1,
                        10000000,
                        0,
                    );
                    let aa = mload64!(a.wrapping_add(48) as usize);
                    arg3 = aa;
                    arg4 = mload64!(a.wrapping_add(40) as usize);
                    break 'label1;
                }
                Self::compute_reward_and_fees(
                    env,
                    a.wrapping_add(16),
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    a.wrapping_add(36),
                );
                let a_i32_36 = mload32!(a.wrapping_add(36) as usize);
                if a_i32_36 != 0 {
                    unreachable!();
                }
                let value_lo = mload64!(a.wrapping_add(16) as usize);
                let ab = mload64!(a.wrapping_add(24) as usize);
                Self::claim_launch_balance_impl(
                    env,
                    a,
                    value_lo,
                    ab,
                    10000000,
                    0,
                );
                mload64!(a.wrapping_add(8) as usize);
                arg4 = mload64!(a as usize);
            }
        }
    }

    fn compute_reward_and_fees(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        mut arg3: i64,
        arg4: i64,
        arg5: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        b = -32;
        d = b.wrapping_add(8);
        c = -32;
        let l: i32;
        'label0: {
            let m = 0;
            if (arg1 | arg2 == 0) as i32 | arg3 | arg4 == 0 {
                l = m;
            } else {
                g = (arg2 < 0) as i32;
                if g != 0 {
                    h = 0.wrapping_sub(arg1);
                } else {
                    h = arg1;
                }
                a = (arg4 < 0) as i32;
                if a != 0 {
                    i = 0.wrapping_sub(arg3);
                } else {
                    i = arg3;
                }
                if a != 0 {
                    arg3 = 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64));
                } else {
                    arg3 = arg4;
                }
                a = -96;
                f = c.wrapping_add(8);
                let o: i64;
                'label1: {
                    if g != 0 {
                        arg1 = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
                    } else {
                        arg1 = arg2;
                    }
                    if arg1 != 0 {
                        if arg3 != 0 {
                            Self::func100(
                                env,
                                a.wrapping_add(80),
                                i,
                                arg3,
                                h,
                                arg1,
                            );
                            let p = mload64!(a.wrapping_add(88) as usize);
                            arg1 = p;
                            e = 1;
                            let loaded_val = mload64!(a.wrapping_add(80) as usize);
                            o = loaded_val;
                            break 'label1;
                        }
                        Self::func100(
                            env,
                            a.wrapping_sub(-64),
                            h,
                            0,
                            i,
                            arg3,
                        );
                        Self::func100(
                            env,
                            a.wrapping_add(48),
                            arg1,
                            0,
                            i,
                            arg3,
                        );
                        let q = mload64!(a.wrapping_add(56) as usize);
                        let r = mload64!(a.wrapping_add(72) as usize);
                        arg3 = r;
                        let a_part_48 = mload64!(a.wrapping_add(48) as usize);
                        arg1 = arg3.wrapping_add(a_part_48);
                        e = (q != 0) as i32 | ((arg1 as u64) < arg3 as u64) as i32;
                        o = mload64!(a.wrapping_add(64) as usize);
                        break 'label1;
                    }
                    if arg3 != 0 {
                        Self::func100(
                            env,
                            a.wrapping_add(32),
                            i,
                            0,
                            h,
                            arg1,
                        );
                        Self::func100(
                            env,
                            a.wrapping_add(16),
                            arg3,
                            0,
                            h,
                            arg1,
                        );
                        let s = mload64!(a.wrapping_add(24) as usize);
                        let t = mload64!(a.wrapping_add(40) as usize);
                        arg3 = t;
                        let value_lo = mload64!(a.wrapping_add(16) as usize);
                        arg1 = arg3.wrapping_add(value_lo);
                        e = (s != 0) as i32 | ((arg1 as u64) < arg3 as u64) as i32;
                        o = mload64!(a.wrapping_add(32) as usize);
                        break 'label1;
                    }
                    Self::func100(
                        env,
                        a,
                        i,
                        arg3,
                        h,
                        arg1,
                    );
                    let u = mload64!(a.wrapping_add(8) as usize);
                    arg1 = u;
                    o = mload64!(a as usize);
                }
                mstore8!(f.wrapping_add(16) as usize, e as u8);
                let v = mload64!(c.wrapping_add(16) as usize);
                i = v;
                h = mload64!(c.wrapping_add(8) as usize);
                let w = mload8!(c.wrapping_add(24) as usize) as i32;
                a = w;
                'label2: {
                    'label3: {
                        arg2 = arg2 ^ arg4;
                        if arg2 >= 0 {
                            if arg2 ^ i >= 0 {
                                break 'label3;
                            }
                            l = 1;
                            break 'label0;
                        }
                        arg1 = 0.wrapping_sub(h);
                        i = 0.wrapping_sub(i.wrapping_add((h != 0) as i32 as u32 as i64));
                        if arg2 ^ i < 0 {
                            break 'label2;
                        }
                    }
                    l = a & 1;
                    break 'label0;
                }
                l = 1;
            }
        }
        a = l;
        mstore8!(d.wrapping_add(16) as usize, a as u8);
        mload64!(b.wrapping_add(16) as usize);
        arg2 = mload64!(b.wrapping_add(8) as usize);
        mload8!(b.wrapping_add(24) as usize) as i32;
    }

    fn func69(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i64 {
        let a: i32 = -16;
        let d = val_to_i64(Bytes::new(env).into_val(env));
        let e = val_to_i64(Bytes::new(env).into_val(env));
        let f = { let mut b = Bytes::from_val(env, &val_from_i64(e)); b.append(&Bytes::from_val(env, &val_from_i64(d))); val_to_i64(b.into_val(env)) };
        let g = val_to_i64(I256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(f))).into_val(env));
        g
    }

    fn func70(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i64 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        d = -32;
        let g = val_to_i64(I256::try_from_val(env, &val_from_i64(arg1)).unwrap().to_be_bytes().into_val(env));
        arg1 = g;
        let h = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(0 as u32..16 as u32).into_val(env));
        Self::func104(env, d.wrapping_add(15), h);
        let i = mload8!(d.wrapping_add(15) as usize) as i32;
        if i == 0 {
            e = d.wrapping_add(24);
            let j = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(16 as u32..32 as u32).into_val(env));
            Self::func104(env, d.wrapping_add(15), j);
            let k = mload8!(d.wrapping_add(15) as usize) as i32;
            if k == 0 {
                arg1 = mload64!(e as usize);
                a = mload64!(d.wrapping_add(16) as usize);
                a = a.wrapping_shl(56 as u32) | (a & 65280).wrapping_shl(40 as u32) | (a & 16711680).wrapping_shl(24 as u32) | (a & 4278190080) | (a as u64) as i64 & 4278190080 | (a as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (a as u64).wrapping_shr(40 as u32) as i64 & 65280 | (a as u64).wrapping_shr(56 as u32) as i64;
                mstore64!(arg0.wrapping_add(16) as usize, a as u64);
            }
        }
    }

    fn claim_launch_balance_impl(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        arg2: i64,
        mut arg3: i64,
        arg4: i64,
    ) {
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        j = -32;
        i = (arg2 < 0) as i32;
        if i != 0 {
            a = 0.wrapping_sub(arg1);
        } else {
            a = arg1;
        }
        if i != 0 {
            arg1 = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
        } else {
            arg1 = arg2;
        }
        h = (arg4 < 0) as i32;
        if h != 0 {
            b = 0.wrapping_sub(arg3);
        } else {
            b = arg3;
        }
        i = -32;
        'label0: {
            if h != 0 {
                arg3 = 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64));
            } else {
                arg3 = arg4;
            }
            if arg3 != 0 {
                if (arg1 == 0) as i32 | ((if arg1 == arg3 { ((a as u64) < b as u64) as i32 } else { ((arg1 as u64) < arg3 as u64) as i32 })) != 0 {
                    unreachable!();
                }
                h = arg3.leading_zeros() as i64 as i32;
                k = arg1.leading_zeros() as i64 as i32;
                if (h as u32) < k as u32 {
                    unreachable!();
                }
                h = h.wrapping_sub(k);
                if h as u32 >= 128 as u32 {
                    unreachable!();
                }
                Self::func101(
                    env,
                    i.wrapping_add(16),
                    b,
                    arg3,
                    h,
                );
                g = 1.wrapping_shl(h as u32 as i64 as u32);
                let o = mload64!(i.wrapping_add(24) as usize);
                d = o;
                e = mload64!(i.wrapping_add(16) as usize);
                loop {
                    c = arg1.wrapping_sub(d).wrapping_sub(((a as u64) < e as u64) as i32 as u32 as i64);
                    if c >= 0 {
                        f = f | g;
                        a = a.wrapping_sub(e);
                        let cond_val = if arg3 == c { ((a as u64) < b as u64) as i32 } else { (arg3 as u64 > c as u64) as i32 };
                        if cond_val != 0 {
                            unreachable!();
                        }
                        arg1 = c;
                    }
                    e = d.wrapping_shl(63 as u32) | (e as u64).wrapping_shr(1 as u32) as i64;
                    g = (g as u64).wrapping_shr(1 as u32) as i64;
                    d = (d as u64).wrapping_shr(1 as u32) as i64;
                }
                unreachable!();
            }
            {
                'label7: {
                    'label8: {
                        if arg1 != 0 {
                            if (arg1 as u64) < b as u64 {
                                break 'label8;
                            }
                            if arg1 == b {
                                break 'label8;
                            }
                            g = (arg1 as u64 / b as u64) as i64;
                            c = arg1.wrapping_sub(g.wrapping_mul(b));
                            if b as u64 >= 4294967296 as u64 {
                                break 'label7;
                            }
                            arg1 = c.wrapping_shl(32 as u32) | (a as u64).wrapping_shr(32 as u32) as i64;
                            let p = arg1;
                            arg1 = (arg1 as u64 / b as u64) as i64;
                            arg3 = a & 4294967295 | p.wrapping_sub(arg1.wrapping_mul(b)).wrapping_shl(32 as u32);
                            let q = arg3;
                            arg3 = (arg3 as u64 / b as u64) as i64;
                            a = q.wrapping_sub(b.wrapping_mul(arg3));
                            f = arg1.wrapping_shl(32 as u32) | arg3;
                            g = (arg1 as u64).wrapping_shr(32 as u32) as i64 | g;
                            c = 0;
                            break 'label0;
                        }
                        f = (a as u64 / b as u64) as i64;
                        a = a.wrapping_sub(f.wrapping_mul(b));
                        k = arg1.leading_zeros() as i64 as i32;
                        l = b.leading_zeros() as i64 as i32;
                        if (k as u32) < l as u32 {
                            unreachable!();
                        }
                        h = 63;
                        if k != l {
                            h = k.wrapping_sub(l);
                            if h as u32 >= 65 as u32 {
                                unreachable!();
                            }
                            h = 64.wrapping_sub(h);
                        }
                        Self::func101(
                            env,
                            i,
                            b,
                            arg3,
                            h,
                        );
                        c = 1.wrapping_shl(h as u32 as i64 as u32);
                        let r = mload64!(i.wrapping_add(8) as usize);
                        d = r;
                        e = mload64!(i as usize);
                        loop {
                            arg3 = arg1.wrapping_sub(d).wrapping_sub(((a as u64) < e as u64) as i32 as u32 as i64);
                            if arg3 >= 0 {
                                a = a.wrapping_sub(e);
                                f = c | f;
                                if arg3 == 0 {
                                    unreachable!();
                                }
                                arg1 = arg3;
                            }
                            e = d.wrapping_shl(63 as u32) | (e as u64).wrapping_shr(1 as u32) as i64;
                            c = (c as u64).wrapping_shr(1 as u32) as i64;
                            d = (d as u64).wrapping_shr(1 as u32) as i64;
                        }
                        unreachable!();
                    }
                    f = (a as u64 / arg1 as u64) as i64;
                    a = a.wrapping_sub(f.wrapping_mul(arg1));
                    break 'label0;
                }
                let cond_val_2 = if arg3 == c { ((a as u64) < b as u64) as i32 } else { (arg3 as u64 > c as u64) as i32 };
                if cond_val_2 != 0 {
                    unreachable!();
                }
                d = arg3.wrapping_shl(63 as u32) | (b as u64).wrapping_shr(1 as u32) as i64;
                e = b.wrapping_shl(63 as u32);
                arg1 = 9223372036854775808;
                loop {
                    arg3 = c.wrapping_sub(d).wrapping_sub(((a as u64) < e as u64) as i32 as u32 as i64);
                    if arg3 >= 0 {
                        a = a.wrapping_sub(e);
                        f = arg1 | f;
                        if arg3 == 0 {
                            unreachable!();
                        }
                        c = arg3;
                    }
                    e = d.wrapping_shl(63 as u32) | (e as u64).wrapping_shr(1 as u32) as i64;
                    arg1 = (arg1 as u64).wrapping_shr(1 as u32) as i64;
                    d = (d as u64).wrapping_shr(1 as u32) as i64;
                }
                arg1 = (a as u64 / b as u64) as i64;
                f = arg1 | f;
                a = a.wrapping_sub(arg1.wrapping_mul(b));
                c = 0;
                break 'label0;
            }
            arg1 = (a as u64 / b as u64) as i64;
            f = arg1 | f;
            a = a.wrapping_sub(arg1.wrapping_mul(b));
            c = 0;
        }
        mstore64!(j.wrapping_add(24) as usize, c as u64);
        mload64!(j.wrapping_add(8) as usize);
        i = (arg2 ^ arg4 < 0) as i32;
    }

    fn token_transfer_checked(
        env: &Env,
        arg0: i64,
        mut arg1: i64,
        mut arg2: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        a = -80;
        'label0: {
            {
                let o = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                c = o;
                if c as u64 > 4294967295 as u64 {
                    Self::claim_launch_balance_impl(
                        env,
                        a.wrapping_add(32),
                        arg1,
                        arg2,
                        (c as u64).wrapping_shr(32 as u32) as i64,
                        0,
                    );
                    let p = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                    c = p;
                    k = mload64!(a.wrapping_add(32) as usize);
                    let q = mload64!(a.wrapping_add(40) as usize);
                    g = q;
                    Self::compute_reward_and_fees(
                        env,
                        a.wrapping_add(8),
                        k,
                        g,
                        (c as u64).wrapping_shr(32 as u32) as i64,
                        0,
                        a.wrapping_add(28),
                    );
                    let a_i32_28 = mload32!(a.wrapping_add(28) as usize);
                    if a_i32_28 == 0 {
                        let r = mload64!(a.wrapping_add(16) as usize);
                        c = r;
                        let s = c;
                        let t = c;
                        c = mload64!(a.wrapping_add(8) as usize);
                        e = arg2.wrapping_sub(t).wrapping_sub(((arg1 as u64) < c as u64) as i32 as u32 as i64);
                        if (arg2 ^ s) & (arg2 ^ e) >= 0 {
                            f = arg1.wrapping_sub(c);
                            let u = Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                            l = (u as u64).wrapping_shr(32 as u32) as i64;
                            b = a.wrapping_sub(-64);
                            arg1 = 0;
                            arg2 = 0;
                            c = arg0;
                            loop {
                                if arg2 == l {
                                    break 'label0;
                                }
                                let v = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).keys().get_unchecked(arg1 as u32));
                                let w = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(arg0)).values().get_unchecked(arg1 as u32));
                                d = w;
                                Self::decode_launch_from_storage(env, a.wrapping_add(48), d);
                                let mut sv3_48_i64 = mload64!(a.wrapping_add(48) as usize);
                                if sv3_48_i64 != 0 {
                                    unreachable!();
                                }
                                i = mload64!(b as usize);
                                let mut loaded_val = mload64!(a.wrapping_add(56) as usize);
                                d = loaded_val;
                                j = d.wrapping_add(k);
                                d = (((j as u64) < d as u64) as i32 as u32 as i64).wrapping_add(g.wrapping_add(i));
                                if (i ^ g ^ 18446744073709551615) & (i ^ d) < 0 {
                                    unreachable!();
                                }
                                let x = e;
                                f = f.wrapping_add(j);
                                e = (((f as u64) < j as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(e));
                                if (d ^ x ^ 18446744073709551615) & (d ^ e) < 0 {
                                    unreachable!();
                                }
                                let a_part_72 = mload64!(a.wrapping_add(72) as usize);
                                let y = Self::u64_to_val(env, a_part_72);
                                d = y;
                                let z = Self::pack_i128_val(env, f, e);
                                let loaded_val = z as i64;
                                let sv3_48_i64 = d as i64;
                                arg1 = arg1.wrapping_add(4294967296);
                                arg2 += 1;
                                let aa = Self::map_new_val(
                                    env,
                                    1049192,
                                    2,
                                    a.wrapping_add(48),
                                    2,
                                );
                                { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(c)); m.set(val_from_i64(v & 18446744069414584320 | 0), val_from_i64(aa)); val_to_i64(m.into_val(env)) };
                                c = ab;
                            }
                            unreachable!();
                        }
                    }
                }
            }
        }
        c
    }

    fn decode_launch_from_storage(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -64;
        while b != 16 {
            mstore64!(a.wrapping_add(24).wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
        }
        'label1: {
            {
                Self::func48(
                    env,
                    arg1,
                    1049192,
                    2,
                    a.wrapping_add(24),
                    2,
                );
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                Self::decode_u64_from_val(env, a.wrapping_add(8), value_hi);
                let a_i32_8 = mload32!(a.wrapping_add(8) as usize);
                if a_i32_8 != 0 {
                    unreachable!();
                }
                arg1 = mload64!(a.wrapping_add(16) as usize);
                Self::decode_i128_parts(env, a.wrapping_add(40), mload64!(a.wrapping_add(32) as usize));
                let a_part_40 = mload64!(a.wrapping_add(40) as usize);
                if a_part_40 == 0 {
                    let e = mload64!(a.wrapping_add(56) as usize);
                    mstore64!(arg0.wrapping_add(16) as usize, e as u64);
                    mstore64!(arg0.wrapping_add(24) as usize, arg1 as u64);
                    break 'label1;
                }
                break 'label1;
                break 'label1;
            }
        }
    }

    fn persist_launch_state(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
        arg3: i64,
        arg4: i64,
    ) {
        let mut a: i32 = 0;
        let b: i32 = -48;
        let cond_val = if arg4 == 0 { (arg3 == 0) as i32 } else { (arg4 < 0) as i32 };
        if cond_val == 0 {
            let e = Self::storage_key_from_str(env, 1049272, 8);
            Self::pack_i128_val(env, arg3, arg4);
            let vec_builder = arg2 as i64;
            let sv6_0_i64 = arg1 as i64;
            while a != 24 {
                mstore64!(b.wrapping_add(24).wrapping_add(a) as usize, 0 as u64);
                a = a.wrapping_add(8);
            }
            a = 0;
            loop {
                let g = mload64!(a.wrapping_add(b) as usize);
                mstore64!(b.wrapping_add(24).wrapping_add(a) as usize, g as u64);
                a = a.wrapping_add(8);
            }
            let h = val_to_i64(vec![&env, val_from_i64(sv6_0_i64), val_from_i64(vec_builder)].into_val(env));
            env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(e)), Vec::<Val>::from_val(env, &val_from_i64(h)));
        }
    }

    fn storage_key_from_str(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i64 {
        let a = Self::func64(env, arg0, arg1);
        a
    }


    fn buy_flow_impl(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        arg3: i32,
        mut arg4: i32,
        arg5: i32,
    ) {
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
        c = -256;
        l = mload64!(arg3 as usize);
        let y = mload64!(arg3.wrapping_add(8) as usize);
        h = y;
        d = mload64!(arg3.wrapping_add(96) as usize);
        let z = mload64!(arg3.wrapping_add(104) as usize);
        e = z;
        let loaded_val = mload64!(arg3.wrapping_add(80) as usize);
        i = loaded_val;
        let aa = mload64!(arg3.wrapping_add(88) as usize);
        p = aa;
        Self::func67(
            env,
            c.wrapping_add(240),
            l,
            h,
            d,
            e,
            i,
            p,
            l,
            h,
        );
        'label0: {
            if arg1 | arg2 == 0 {
                Self::verify_launch_window(env, arg0, 80);
            } else {
                let ab = mload64!(c.wrapping_add(248) as usize);
                f = ab;
                q = mload64!(c.wrapping_add(240) as usize);
                Self::claim_launch_balance_impl(
                    env,
                    c.wrapping_add(224),
                    arg1,
                    arg2,
                    10000,
                    0,
                );
                j = mload64!(c.wrapping_add(224) as usize);
                let ac = mload64!(c.wrapping_add(232) as usize);
                g = ac;
                r = arg4 as u32 as i64;
                Self::compute_reward_and_fees(
                    env,
                    c.wrapping_add(200),
                    j,
                    g,
                    r,
                    0,
                    c.wrapping_add(220),
                );
                {
                    let c_i32_220 = mload32!(c.wrapping_add(220) as usize);
                    if c_i32_220 == 0 {
                        let ad = mload64!(c.wrapping_add(208) as usize);
                        n = ad;
                        o = mload64!(c.wrapping_add(200) as usize);
                        w = arg5 as u32 as i64;
                        Self::compute_reward_and_fees(
                            env,
                            c.wrapping_add(176),
                            j,
                            g,
                            w,
                            0,
                            c.wrapping_add(196),
                        );
                        let c_i32_196 = mload32!(c.wrapping_add(196) as usize);
                        if c_i32_196 == 0 {
                            g = arg2.wrapping_sub(n).wrapping_sub(((arg1 as u64) < o as u64) as i32 as u32 as i64);
                            if (arg2 ^ n) & (arg2 ^ g) >= 0 {
                                let ae = mload64!(c.wrapping_add(184) as usize);
                                s = ae;
                                m = arg1.wrapping_sub(o);
                                t = mload64!(c.wrapping_add(176) as usize);
                                j = g.wrapping_sub(s).wrapping_sub(((m as u64) < t as u64) as i32 as u32 as i64);
                                if (g ^ s) & (g ^ j) >= 0 {
                                    let af = mload64!(arg3.wrapping_add(40) as usize);
                                    g = af;
                                    u = mload64!(arg3.wrapping_add(32) as usize);
                                    m = m.wrapping_sub(t);
                                    v = u.wrapping_add(m);
                                    k = (((v as u64) < u as u64) as i32 as u32 as i64).wrapping_add(g.wrapping_add(j));
                                    if (g ^ j ^ 18446744073709551615) & (g ^ k) >= 0 {
                                        let ag: i64;
                                        'label2: {
                                            'label3: {
                                                {
                                                    let cond_val = if f == k { ((q as u64) < v as u64) as i32 } else { (f < k) as i32 };
                                                    if cond_val != 0 {
                                                        j = f.wrapping_sub(g).wrapping_sub(((q as u64) < u as u64) as i32 as u32 as i64);
                                                        if (f ^ g) & (f ^ j) < 0 {
                                                            unreachable!();
                                                        }
                                                        m = q.wrapping_sub(u);
                                                        Self::compute_reward_and_fees(
                                                            env,
                                                            c.wrapping_sub(-64),
                                                            m,
                                                            j,
                                                            10000,
                                                            0,
                                                            c.wrapping_add(84),
                                                        );
                                                        let c_i32_84 = mload32!(c.wrapping_add(84) as usize);
                                                        if c_i32_84 == 0 {
                                                            unreachable!();
                                                        }
                                                        unreachable!();
                                                    }
                                                    f = e ^ p;
                                                    if f | d ^ i != 0 {
                                                        g = e.wrapping_sub(p).wrapping_sub(((d as u64) < i as u64) as i32 as u32 as i64);
                                                        if f & (e ^ g) < 0 {
                                                            unreachable!();
                                                        }
                                                        let ah = Self::func69(env, d.wrapping_sub(i), g);
                                                        g = ah;
                                                        let ai = Self::func69(env, 0, 0);
                                                        let aj = Self::func69(env, i, p);
                                                        let ak = val_to_i64(I256::try_from_val(env, &val_from_i64(ai)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(aj)).unwrap()).into_val(env));
                                                        let al = Self::func69(env, l, h);
                                                        let am = val_to_i64(I256::try_from_val(env, &val_from_i64(ak)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(al)).unwrap()).into_val(env));
                                                        i = am;
                                                        let an = Self::func69(env, 18446744073709551614, 18446744073709551615);
                                                        let ao = Self::func69(env, l, h);
                                                        let ap = val_to_i64(I256::try_from_val(env, &val_from_i64(an)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(ao)).unwrap()).into_val(env));
                                                        let aq = Self::func69(env, 10000000, 0);
                                                        let ar = val_to_i64(I256::try_from_val(env, &val_from_i64(ap)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(aq)).unwrap()).into_val(env));
                                                        let at = Self::func69(env, v, k);
                                                        let au = val_to_i64(I256::try_from_val(env, &val_from_i64(ar)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(at)).unwrap()).into_val(env));
                                                        d = au;
                                                        let av = val_to_i64(I256::try_from_val(env, &val_from_i64(i)).unwrap().pow(2 as u32).into_val(env));
                                                        let aw = Self::func69(env, 0, 0);
                                                        let ax = val_to_i64(I256::try_from_val(env, &val_from_i64(aw)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(g)).unwrap()).into_val(env));
                                                        let ay = val_to_i64(I256::try_from_val(env, &val_from_i64(ax)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(d)).unwrap()).into_val(env));
                                                        let az = val_to_i64(I256::try_from_val(env, &val_from_i64(av)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(ay)).unwrap()).into_val(env));
                                                        f = az;
                                                        Self::func70(env, c.wrapping_add(152), f);
                                                        let c_i32_152 = mload32!(c.wrapping_add(152) as usize);
                                                        if c_i32_152 != 0 {
                                                            d = 0;
                                                            let ba = mload64!(c.wrapping_add(168) as usize);
                                                            if ba < 0 {
                                                                break 'label3;
                                                            }
                                                        }
                                                        let bb = val_to_i64(I256::try_from_val(env, &val_from_i64(f)).unwrap().add(&I256::try_from_val(env, &val_from_i64(1)).unwrap()).into_val(env));
                                                        k = bb;
                                                        d = f;
                                                        loop {
                                                            {
                                                                let bc = val_to_i64(I256::try_from_val(env, &val_from_i64(k)).unwrap().div(&I256::try_from_val(env, &val_from_i64(2)).unwrap()).into_val(env));
                                                                e = bc;
                                                                let bd = { let a = val_from_i64(e); let b = val_from_i64(d); if a < b { -1 } else if a > b { 1 } else { 0 } };
                                                                if bd >= 0 {
                                                                    break 'label3;
                                                                    if e >= d {
                                                                        break 'label3;
                                                                    }
                                                                }
                                                            }
                                                            let be = val_to_i64(I256::try_from_val(env, &val_from_i64(f)).unwrap().div(&I256::try_from_val(env, &val_from_i64(e)).unwrap()).into_val(env));
                                                            let bf = val_to_i64(I256::try_from_val(env, &val_from_i64(be)).unwrap().add(&I256::try_from_val(env, &val_from_i64(e)).unwrap()).into_val(env));
                                                            k = bf;
                                                            d = e;
                                                        }
                                                        unreachable!();
                                                    }
                                                    Self::compute_reward_and_fees(
                                                        env,
                                                        c.wrapping_add(104),
                                                        v,
                                                        k,
                                                        10000000,
                                                        0,
                                                        c.wrapping_add(124),
                                                    );
                                                    if mload32!(c.wrapping_add(124) as usize) | d | e == 0 {
                                                        unreachable!();
                                                    }
                                                    f = mload64!(c.wrapping_add(104) as usize);
                                                    let bg = mload64!(c.wrapping_add(112) as usize);
                                                    i = bg;
                                                    if (f | i ^ 9223372036854775808 == 0) as i32 & d & e == 18446744073709551615 {
                                                        unreachable!();
                                                    }
                                                    Self::claim_launch_balance_impl(
                                                        env,
                                                        c.wrapping_add(88),
                                                        f,
                                                        i,
                                                        d,
                                                        e,
                                                    );
                                                    let bh = mload64!(c.wrapping_add(96) as usize);
                                                    d = bh;
                                                    ag = mload64!(c.wrapping_add(88) as usize);
                                                    break 'label2;
                                                }
                                                arg2 = r.wrapping_add(w);
                                                arg1 = arg2.wrapping_sub(10000);
                                                arg2 = (((arg2 as u64) < r as u64) as i32 as u32 as i64).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64) -= 1;
                                                if arg1 | arg2 == 0 {
                                                    unreachable!();
                                                }
                                                d = mload64!(c.wrapping_add(64) as usize);
                                                let bi = mload64!(c.wrapping_add(72) as usize);
                                                e = bi;
                                                if (d | e ^ 9223372036854775808 == 0) as i32 & arg1 & arg2 == 18446744073709551615 {
                                                    unreachable!();
                                                }
                                                Self::claim_launch_balance_impl(
                                                    env,
                                                    c.wrapping_add(48),
                                                    d,
                                                    e,
                                                    arg1,
                                                    arg2,
                                                );
                                                arg1 = mload64!(c.wrapping_add(48) as usize);
                                                let bj = mload64!(c.wrapping_add(56) as usize);
                                                arg2 = bj;
                                                if arg1 | arg2 ^ 9223372036854775808 == 0 {
                                                    unreachable!();
                                                }
                                                Self::claim_launch_balance_impl(
                                                    env,
                                                    c.wrapping_add(32),
                                                    arg1,
                                                    arg2,
                                                    18446744073709541616,
                                                    18446744073709551615,
                                                );
                                                let bk = mload64!(c.wrapping_add(40) as usize);
                                                Self::compute_reward_and_fees(
                                                    env,
                                                    c.wrapping_add(8),
                                                    mload64!(c.wrapping_add(32) as usize),
                                                    bk,
                                                    r,
                                                    0,
                                                    c.wrapping_add(28),
                                                );
                                                let c_i32_28 = mload32!(c.wrapping_add(28) as usize);
                                                if c_i32_28 != 0 {
                                                    unreachable!();
                                                }
                                                arg2 = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
                                                let bl = mload64!(c.wrapping_add(16) as usize);
                                                n = bl;
                                                arg1 = 0.wrapping_sub(arg1);
                                                o = mload64!(c.wrapping_add(8) as usize);
                                                d = arg2.wrapping_sub(n).wrapping_sub(((arg1 as u64) < o as u64) as i32 as u32 as i64);
                                                if (arg2 ^ n) & (arg2 ^ d) < 0 {
                                                    unreachable!();
                                                }
                                                e = arg1.wrapping_sub(o);
                                                s = d.wrapping_sub(j).wrapping_sub(((e as u64) < m as u64) as i32 as u32 as i64);
                                                if (d ^ j) & (d ^ s) < 0 {
                                                    unreachable!();
                                                }
                                                t = e.wrapping_sub(m);
                                                d = h;
                                                ag = l;
                                                break 'label2;
                                            }
                                            let bm = val_to_i64(I256::try_from_val(env, &val_from_i64(d)).unwrap().sub(&I256::try_from_val(env, &val_from_i64(i)).unwrap()).into_val(env));
                                            let bn = Self::func69(env, 0, 0);
                                            let bo = val_to_i64(I256::try_from_val(env, &val_from_i64(bn)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(g)).unwrap()).into_val(env));
                                            let bp = val_to_i64(I256::try_from_val(env, &val_from_i64(bm)).unwrap().div(&I256::try_from_val(env, &val_from_i64(bo)).unwrap()).into_val(env));
                                            Self::func70(env, c.wrapping_add(128), bp);
                                            let bq = mload64!(c.wrapping_add(144) as usize);
                                            arg4 = mload32!(c.wrapping_add(128) as usize);
                                            if arg4 != 0 {
                                                d = bq;
                                            } else {
                                                d = 0;
                                            }
                                            let c_part_136 = mload64!(c.wrapping_add(136) as usize);
                                            if arg4 != 0 {
                                                ag = c_part_136;
                                            } else {
                                                ag = 0;
                                            }
                                        }
                                        e = ag;
                                        if (d == h) as i32 != 0 {
                                            arg4 = ((e as u64) < l as u64) as i32;
                                        } else {
                                            arg4 = (d < h) as i32;
                                        }
                                        if arg4 != 0 {
                                            d = d;
                                        } else {
                                            d = h;
                                        }
                                        let br = mload64!(arg3.wrapping_add(24) as usize);
                                        h = br;
                                        let bs = h;
                                        if arg4 != 0 {
                                            e = e;
                                        } else {
                                            e = l;
                                        }
                                        h = mload64!(arg3.wrapping_add(16) as usize);
                                        f = d.wrapping_sub(h).wrapping_sub(((e as u64) < h as u64) as i32 as u32 as i64);
                                        if (d ^ bs) & (d ^ f) < 0 {
                                            unreachable!();
                                        }
                                        mstore64!(arg0.wrapping_add(72) as usize, s as u64);
                                        mstore64!(arg0.wrapping_add(56) as usize, n as u64);
                                        mstore64!(arg0.wrapping_add(40) as usize, f as u64);
                                        mstore64!(arg0.wrapping_add(24) as usize, j as u64);
                                        break 'label0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn verify_launch_window(
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        if arg1 as u32 >= 16 as u32 {
            c = 0.wrapping_sub(arg0) & 3;
            a = arg0.wrapping_add(c);
            while (arg0 as u32) < a as u32 {
                mstore8!(arg0 as usize, 0 as u8);
                arg0 += 1;
            }
            arg0 = 8;
            while !(arg0 as u32 >= 32 as u32) {
                b = b.wrapping_shl((arg0 & 24) as u32) | b;
                arg0 = arg0.wrapping_shl(1 as u32);
            }
            'label2: {
                arg1 = arg1.wrapping_sub(c);
                arg0 = a.wrapping_add(arg1 & -4);
                loop {
                    if arg0 as u32 <= a as u32 {
                        break 'label2;
                    }
                    a = a.wrapping_add(4);
                }
                unreachable!();
            }
            arg1 = arg1 & 3;
        }
        arg1 = arg0.wrapping_add(arg1);
        while (arg0 as u32) < arg1 as u32 {
            mstore8!(arg0 as usize, 0 as u8);
            arg0 += 1;
        }
    }

    fn sell_flow_impl(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        a = -80;
        'label0: {
            if arg1 | arg2 == 0 {
                Self::verify_launch_window(env, arg0, 80);
            } else {
                {
                    c = mload64!(arg3.wrapping_add(16) as usize);
                    let l = mload64!(arg3.wrapping_add(24) as usize);
                    b = l;
                    let cond_val = if arg2 == b { (c as u64 > arg1 as u64) as i32 } else { (b > arg2) as i32 };
                    if cond_val != 0 {
                        d = b.wrapping_sub(arg2).wrapping_sub((arg1 as u64 > c as u64) as i32 as u32 as i64);
                        if (arg2 ^ b) & (b ^ d) < 0 {
                            unreachable!();
                        }
                        let m = mload64!(arg3.wrapping_add(104) as usize);
                        let loaded_val = mload64!(arg3.wrapping_add(80) as usize);
                        let n = mload64!(arg3.wrapping_add(88) as usize);
                        let o = mload64!(arg3.wrapping_add(8) as usize);
                        Self::func67(
                            env,
                            a.wrapping_sub(-64),
                            c.wrapping_sub(arg1),
                            d,
                            mload64!(arg3.wrapping_add(96) as usize),
                            m,
                            loaded_val,
                            n,
                            mload64!(arg3 as usize),
                            o,
                        );
                        let p = mload64!(a.wrapping_add(72) as usize);
                        e = p;
                        d = mload64!(a.wrapping_add(64) as usize);
                    }
                    let q = mload64!(arg3.wrapping_add(40) as usize);
                    c = q;
                    let r = e;
                    e = mload64!(arg3.wrapping_add(32) as usize);
                    b = c.wrapping_sub(e).wrapping_sub(((e as u64) < d as u64) as i32 as u32 as i64);
                    if (c ^ r) & (c ^ b) < 0 {
                        unreachable!();
                    }
                    c = e.wrapping_sub(d);
                    Self::claim_launch_balance_impl(
                        env,
                        a.wrapping_add(48),
                        c,
                        b,
                        10000,
                        0,
                    );
                    f = mload64!(a.wrapping_add(48) as usize);
                    let s = mload64!(a.wrapping_add(56) as usize);
                    g = s;
                    Self::compute_reward_and_fees(
                        env,
                        a.wrapping_add(24),
                        f,
                        g,
                        arg4 as u32 as i64,
                        0,
                        a.wrapping_add(44),
                    );
                    let a_i32_44 = mload32!(a.wrapping_add(44) as usize);
                    if a_i32_44 != 0 {
                        unreachable!();
                    }
                    let t = mload64!(a.wrapping_add(32) as usize);
                    d = t;
                    e = mload64!(a.wrapping_add(24) as usize);
                    Self::compute_reward_and_fees(
                        env,
                        a,
                        f,
                        g,
                        arg5 as u32 as i64,
                        0,
                        a.wrapping_add(20),
                    );
                    let a_i32_20 = mload32!(a.wrapping_add(20) as usize);
                    if a_i32_20 != 0 {
                        unreachable!();
                    }
                    f = b.wrapping_sub(d).wrapping_sub(((c as u64) < e as u64) as i32 as u32 as i64);
                    if (b ^ d) & (b ^ f) < 0 {
                        unreachable!();
                    }
                    let u = mload64!(a.wrapping_add(8) as usize);
                    g = u;
                    i = c.wrapping_sub(e);
                    h = mload64!(a as usize);
                    j = f.wrapping_sub(g).wrapping_sub(((i as u64) < h as u64) as i32 as u32 as i64);
                    if (f ^ g) & (f ^ j) < 0 {
                        unreachable!();
                    }
                    mstore64!(arg0.wrapping_add(72) as usize, g as u64);
                    mstore64!(arg0.wrapping_add(56) as usize, d as u64);
                    mstore64!(arg0.wrapping_add(40) as usize, j as u64);
                    mstore64!(arg0.wrapping_add(24) as usize, b as u64);
                    break 'label0;
                }
            }
        }
    }


    fn fail_with_error_2(
        env: &Env,
        arg0: i64,
    ) {
        let a = 0 /* TODO: fail_with_error */;
    }


    fn memcpy_like(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        if (arg2 as u32) < 16 as u32 {
            a = arg0;
        } else {
            d = 0.wrapping_sub(arg0) & 3;
            b = arg0.wrapping_add(d);
            c = arg1;
            a = arg0;
            while (a as u32) < b as u32 {
                let h = mload8!(c as usize) as i32;
                mstore8!(a as usize, h as u8);
                c += 1;
                a += 1;
            }
            f = arg2.wrapping_sub(d);
            g = f & -4;
            a = b.wrapping_add(g);
            'label2: {
                c = arg1.wrapping_add(d);
                if c & 3 == 0 {
                    arg1 = c;
                    loop {
                        if a as u32 <= b as u32 {
                            break 'label2;
                        }
                        let svarg1_0_i32 = mload32!(arg1 as usize);
                        let mut sv4_0_i32 = svarg1_0_i32 as i32;
                        arg1 = arg1.wrapping_add(4);
                        b = b.wrapping_add(4);
                    }
                    unreachable!();
                }
                arg2 = c & -4;
                arg1 = arg2.wrapping_add(4);
                e = c.wrapping_shl(3 as u32);
                d = e & 24;
                e = 0.wrapping_sub(e) & 24;
                arg2 = mload32!(arg2 as usize);
                loop {
                    if a as u32 <= b as u32 {
                        break 'label2;
                    }
                    if d != 0 {
                        let i = arg2;
                        arg2 = svarg1_0_i32;
                        sv4_0_i32 = ((i as u32).wrapping_shr(d as u32) as i32 | arg2.wrapping_shl(e as u32)) as i32;
                        arg1 = arg1.wrapping_add(4);
                        b = b.wrapping_add(4);
                    }
                }
                unreachable!();
            }
            arg2 = f & 3;
            arg1 = c.wrapping_add(g);
        }
        arg2 = arg2.wrapping_add(a);
        while arg2 as u32 > a as u32 {
            let j = mload8!(arg1 as usize) as i32;
            mstore8!(a as usize, j as u8);
            arg1 += 1;
            a += 1;
        }
        arg0
    }

















    fn func100(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
        arg3: i64,
        arg4: i64,
    ) {
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        a = arg3 & 4294967295;
        b = arg1 & 4294967295;
        d = (arg3 as u64).wrapping_shr(32 as u32) as i64;
        b = b.wrapping_mul(d);
        e = (arg1 as u64).wrapping_shr(32 as u32) as i64;
        a = b.wrapping_add(a.wrapping_mul(e));
    }

    fn func101(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        arg3: i32,
    ) {
        let mut a: i64 = 0;
        if arg3 & 64 == 0 {
            if arg3 == 0 {
                unreachable!();
            }
            a = (arg3 & 63) as u32 as i64;
            arg2 = arg2.wrapping_shl(a as u32) | (arg1 as u64).wrapping_shr((0.wrapping_sub(arg3) & 63) as u32 as i64 as u32) as i64;
            arg1 = arg1.wrapping_shl(a as u32);
        } else {
            arg2 = arg1.wrapping_shl((arg3 & 63) as u32 as i64 as u32);
        }
    }



    fn func104(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -16;
        let e = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64;
        let f: i32;
        if e & 18446744069414584320 == 68719476736 {
            'label0: {
                loop {
                    let g = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64;
                    if (g as u64) < 4294967296 as u64 {
                        break 'label0;
                    }
                    let h = Bytes::from_val(env, &val_from_i64(arg1)).get(0) as i64;
                    let i = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64;
                    let j = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(1 as u32..i & 18446744069414584320 | 0 as u32).into_val(env));
                    arg1 = j;
                    if b != 16 {
                        mstore8!(a.wrapping_add(b) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u8);
                        b += 1;
                    }
                }
                unreachable!();
            }
            arg1 = mload64!(a as usize);
            mstore64!(arg0.wrapping_add(9) as usize, mload64!(a.wrapping_add(8) as usize) as u64);
            f = 0;
        } else {
            f = 1;
        }
        mstore8!(arg0 as usize, f as u8);
    }
}