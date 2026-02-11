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
        stability_check_duration: u64,
        space_fee: u32,
        slz_fee: u32,
        slz_fee_destination: Address,
        stellarbucks_contract: Address,
        native_contract: Address,
        space_missions_odds: Map<u32, u64>,
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut d: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        a = global0.wrapping_sub(96);
        global0 = a;
        {
            Self::decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
            let mut value_hi = mload64!(a.wrapping_add(24) as usize);
            f = mload64!(a.wrapping_add(32) as usize);
            let i = Self::storage_has_data_key(env, 1049208);
            if i == 0 {
                let j = val_to_i64(Map::<Val, Val>::new(env).into_val(env));
                let k = Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).len() as i64;
                g = (k as u64).wrapping_shr(32 as u32) as i64;
                b = 0;
                stability_check_duration = 0 /* False */;
                while (stability_check_duration as u64) < g as u64 {
                    let l = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).keys().get_unchecked(b as u32));
                    let n = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(space_missions_odds)).values().get_unchecked(b as u32));
                    d = n;
                    Self::decode_u64_from_val(env, a.wrapping_add(8), d);
                    let a_hi = mload64!(a.wrapping_add(8) as usize);
                    if (a_hi != 0) {
                        unreachable!();
                    }
                    let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                    let o = Self::u64_to_val(env, value_lo);
                    d = o;
                    let p = Self::pack_i128_val(env, 0 /* False */, 0 /* False */);
                    let mut sv8_48_i64 = p as i64;
                    let mut sv8_40_i64 = d as i64;
                    b = b.wrapping_add(4294967296);
                    stability_check_duration = stability_check_duration.wrapping_add(1 /* True */);
                    let q = Self::map_new_val(
                        env,
                        1049192,
                        2,
                        a.wrapping_add(40),
                        2,
                    );
                    let r = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(j)); m.set(val_from_i64(l & 18446744069414584320 | 0), val_from_i64(q)); val_to_i64(m.into_val(env)) };
                }
                let mut sv8_40_i64 = admin as i64;
                mstore32!(a.wrapping_add(92) as usize, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(a.wrapping_add(88) as usize, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
                let mut sv8_48_i64 = f as i64;
                env.storage().put_contract_data(a.wrapping_add(40));
                let s = Self::storage_key_from_str(env, 1048576, 11);
                let t = Self::event_topic_self_pair(env, s);
                env.events().publish(val_from_i64(t), val_from_i64(1 /* True */));
                global0 = a.wrapping_add(96);
            }
            Self::fail_with_error_2(env, 4294967299 /* Error(Contract, #1) */);
        }
    }

    pub fn change_contract_info(
        env: Env,
        admin: Address,
        stability_check_duration: u64,
        space_fee: u32,
        slz_fee: u32,
        slz_fee_destination: Address,
        space_missions_odds: Map<u32, u64>,
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut h: i64 = 0;
        let mut j: i64 = 0;
        a = global0.wrapping_sub(160);
        global0 = a;
        Self::decode_u64_from_val(env, a.wrapping_add(24), stability_check_duration);
        let mut value_hi = mload64!(a.wrapping_add(24) as usize);
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
            address_from_i64(env, authorized_addr).require_auth();
            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
            c = loaded_val;
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
                Self::decode_u64_from_val(env, a.wrapping_add(8), e);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if (a_hi != 0) {
                    unreachable!();
                }
                let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                e = 0 /* False */;
                h = 0 /* False */;
                d = d & 18446744069414584320 | 0;
                let q = if Map::<Val, Val>::from_val(env, &val_from_i64(c)).has(val_from_i64(d)) { 1 } else { 0 };
                if q == 1 /* True */ {
                    let r = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(c)).get(val_from_i64(d)).unwrap_or(val_from_i64(0)));
                    Self::decode_launch_from_storage(env, a.wrapping_add(96), r);
                    let mut sv6_96_i64 = mload64!(a.wrapping_add(96) as usize);
                    if sv6_96_i64 != 0 /* False */ {
                        unreachable!();
                    }
                    h = mload64!(b as usize);
                    e = mload64!(a.wrapping_add(104) as usize);
                }
                let s = Self::u64_to_val(env, value_lo);
                let t = Self::pack_i128_val(env, e, h);
                let mut sv6_96_i64 = s as i64;
                f = f.wrapping_add(4294967296);
                stability_check_duration = stability_check_duration.wrapping_add(1 /* True */);
                let u = Self::map_new_val(
                    env,
                    1049192,
                    2,
                    a.wrapping_add(96),
                    2,
                );
                let v = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(c)); m.set(val_from_i64(d), val_from_i64(u)); val_to_i64(m.into_val(env)) };
                c = v;
            }
            let mut authorized_addr = admin as i64;
            let mut loaded_val = c as i64;
            mstore32!(a.wrapping_add(92) as usize, (slz_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
            mstore32!(a.wrapping_add(88) as usize, (space_fee as u64).wrapping_shr(32 as u32) as i64 as u32);
            env.storage().put_contract_data(a.wrapping_add(40));
            global0 = a.wrapping_add(160);
        }
    }

    pub fn upgrade(
        env: Env,
        hash: BytesN<32>,
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        a = global0.wrapping_add(-64);
        global0 = a;
        {
            let c = Bytes::from_val(env, &val_from_i64(hash)).len() as i64;
            if c & 18446744069414584320 == 137438953472 {
                env.storage().get_contract_data(a);
                let a_lo = mload64!(a as usize);
                if a_lo != 0 /* False */ {
                    unreachable!();
                }
                Self::fail_with_error_2(env, 3 /* Error(Contract, #0) */);
            }
        }
        unreachable!();
        let mut authorized_addr = mload64!(a.wrapping_add(8) as usize);
        address_from_i64(env, authorized_addr).require_auth();
        env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(hash)));
        global0 = a.wrapping_sub(-64);
        0 /* Void */
    }

    pub fn start_space_mission(
        env: Env,
        user: Address,
        funding: i128,
        difficulty: u32,
        min_mission_reward: i128,
    ) -> (bool, i128) {
        let mut global0: i32 = 0;
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
        a = global0.wrapping_sub(160);
        global0 = a;
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
                address_from_i64(env, user).require_auth();
                Self::compute_reward_and_fees(
                    env,
                    a,
                    g,
                    d,
                    10000000,
                    0 /* False */,
                    a.wrapping_add(20),
                );
                let a_i32_20 = mload32!(a.wrapping_add(20) as usize);
                if a_i32_20 == 0 {
                    min_mission_reward = mload64!(a as usize);
                    let q = mload64!(a.wrapping_add(8) as usize);
                    funding = q;
                    let cond_val = if funding == 0 { (min_mission_reward == 0) as i32 } else { (funding < 0 /* False */) as i32 };
                    if cond_val == 0 {
                        h = mload64!(a.wrapping_add(48) as usize);
                        let r = Self::storage_key_from_str(env, 1049280, 4);
                        i = r;
                        let s = Self::pack_i128_val(env, min_mission_reward, funding);
                        let mut sv4_152_i64 = s as i64;
                        let mut sv4_144_i64 = user as i64;
                        while b != 16 {
                            mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, 0 /* Void */ as u64);
                            b = b.wrapping_add(8);
                        }
                        b = 0;
                        loop {
                            let t = mload64!(a.wrapping_add(144).wrapping_add(b) as usize);
                            mstore64!(a.wrapping_add(80).wrapping_add(b) as usize, t as u64);
                            b = b.wrapping_add(8);
                        }
                        let u = val_to_i64(Vec::<Val>::new(env).into_val(env));
                        let _ = env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(h)), &Symbol::from_val(env, &val_from_i64(i)), Vec::<Val>::from_val(env, &val_from_i64(u)));
                    }
                    {
                        h = mload64!(a.wrapping_add(64) as usize);
                        i = difficulty & 18446744069414584320 | 0;
                        let v = if Map::<Val, Val>::from_val(env, &val_from_i64(h)).has(val_from_i64(i)) { 1 } else { 0 };
                        if v == 1 /* True */ {
                            let w = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(h)).get(val_from_i64(i)).unwrap_or(val_from_i64(0)));
                            Self::decode_launch_from_storage(env, a.wrapping_add(80), w);
                            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
                            if loaded_val != 0 {
                                unreachable!();
                            }
                            let x = mload64!(a.wrapping_add(104) as usize);
                            j = x;
                            b = (d == 0) as i32;
                            let cond_val_2 = if b != 0 { (g as u64 > j as u64) as i32 } else { (d > 0 /* False */) as i32 };
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
                            let z = env.prng().gen_range::<u64>(1 /* True */ as u64..=j as u64) as i64;
                            if b != 0 {
                                b = (z as u64 <= g as u64) as i32;
                            } else {
                                b = (d >= 0 /* False */) as i32;
                            }
                            if b != 0 {
                                e = funding;
                            } else {
                                e = 0 /* False */;
                            }
                            if b != 0 {
                                f = min_mission_reward;
                            } else {
                                f = 1 /* True */;
                            }
                            k = funding.wrapping_sub(e).wrapping_sub(((min_mission_reward as u64) < f as u64) as i32 as u32 as i64);
                            if (funding ^ e) & (funding ^ k) < 0 /* False */ {
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
                            let ad = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(h)); m.set(val_from_i64(i), val_from_i64(ac)); val_to_i64(m.into_val(env)) };
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
                            let ai = Self::pack_i128_val(env, f, e);
                            d = b as u32 as i64;
                            let mut sv4_88_i64 = min_mission_reward as i64;
                            let mut loaded_val = (difficulty & 18446744069414584320 | 0) as i64;
                            let aj = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            let mut sv4_152_i64 = aj as i64;
                            let mut sv4_144_i64 = user as i64;
                            let ak = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            env.events().publish(val_from_i64(ag), val_from_i64(ak));
                            let am = Self::pack_i128_val(env, f, e);
                            let mut sv4_88_i64 = am as i64;
                            let mut loaded_val = d as i64;
                            let ao = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            global0 = a.wrapping_add(160);
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
        reward_difficulty: u32,
    ) {
        let mut global0: i32 = 0;
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
        a = global0.wrapping_sub(128);
        global0 = a;
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
                address_from_i64(env, user).require_auth();
                j = reward_difficulty & 18446744069414584320 | 0;
                g = mload64!(a.wrapping_add(48) as usize);
                let s = Map::<Val, Val>::from_val(env, &val_from_i64(g)).len() as i64;
                k = (s as u64).wrapping_shr(32 as u32) as i64;
                l = mload64!(a.wrapping_add(40) as usize);
                c = a.wrapping_add(80);
                funds = 0;
                reward_difficulty = 0 /* False */;
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
                            if (d ^ f ^ 18446744073709551615) & (d ^ e) < 0 /* False */ {
                                break 'label0;
                            }
                            let a_part_88 = mload64!(a.wrapping_add(88) as usize);
                            let w = Self::u64_to_val(env, a_part_88);
                            d = w;
                            let x = Self::pack_i128_val(env, o, e);
                            let mut loaded_val = x as i64;
                            let mut sv3_64_i64 = d as i64;
                            let y = Self::map_new_val(
                                env,
                                1049192,
                                2,
                                a.wrapping_sub(-64),
                                2,
                            );
                            let z = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(g)); m.set(val_from_i64(j), val_from_i64(y)); val_to_i64(m.into_val(env)) };
                            break 'label2;
                        }
                        env.storage().put_contract_data(a.wrapping_add(8));
                        global0 = a.wrapping_add(128);
                    }
                    funds = funds.wrapping_add(4294967296);
                    reward_difficulty = reward_difficulty.wrapping_add(1 /* True */);
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
        launch_index: u64,
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        a = global0.wrapping_sub(416);
        global0 = a;
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
                                    address_from_i64(env, dev).require_auth();
                                    let n = env.ledger().timestamp() as i64;
                                    if max_supply | g == 0 {
                                        Self::fail_with_error_2(env, 442381631491 /* Error(Contract, #103) */);
                                    } else {
                                        'label2: {
                                            let cond_val = if max_supply == 0 { (g < 0 /* False */) as i32 } else { (max_supply > 0 /* False */) as i32 };
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
                                                let mut sv8_248_i64 = 1 /* True */ as i64;
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
                                                let mut sv8_264_i64 = launch_index as i64;
                                                let mut sv8_256_i64 = dev as i64;
                                                let mut sv8_248_i64 = 1 /* True */ as i64;
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
                                                let s = Self::storage_key_from_str(env, 1048587, 10);
                                                mstore64!(a.wrapping_add(240) as usize, n as u64);
                                                mstore64!(a.wrapping_add(232) as usize, launch_index as u64);
                                                let t = Self::launch_snapshot_to_val(env, a.wrapping_add(216));
                                                let u = Self::contract_info_to_val(env, b);
                                                env.events().publish(val_from_i64(t), val_from_i64(u));
                                                global0 = a.wrapping_add(416);
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
        launch_key: (Address, u64),
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut g: i64 = 0;
        let mut i: i64 = 0;
        let mut authorized_addr: i64 = 0;
        a = global0.wrapping_sub(208);
        global0 = a;
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
                        address_from_i64(env, authorized_addr).require_auth();
                        let cond_val = if d == 0 { (value_hi == 0) as i32 } else { (d < 0 /* False */) as i32 };
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
        let mut value_hi = launch_key as i64;
        let mut value_lo = e as i64;
        let mut sv1_8_i64 = 1 /* True */ as i64;
        c = a.wrapping_add(8);
        let q = Self::data_key_to_val(env, c);
        env.storage().del_contract_data(q);
        let r = Self::storage_key_from_str(env, 1048617, 13);
        d = r;
        mstore64!(a.wrapping_add(24) as usize, launch_key as u64);
        let mut value_lo = e as i64;
        let mut sv1_8_i64 = d as i64;
        let s = Self::launch_snapshot_to_val(env, c);
        env.events().publish(val_from_i64(s), val_from_i64(1 /* True */));
        global0 = a.wrapping_add(208);
        0 /* Void */
    }

    pub fn buy(
        env: Env,
        mut user: Address,
        mut launch_key: (Address, u64),
        mut sending: i128,
        min_receive: i128,
    ) {
        let mut global0: i32 = 0;
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
        a = global0.wrapping_sub(576);
        global0 = a;
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
                            let mut sv4_248_i64 = 1 /* True */ as i64;
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
                                            let mut loaded_val = mload64!(a.wrapping_add(80) as usize);
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
                                                address_from_i64(env, user).require_auth();
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
                                                if (launch_key ^ min_receive) & (launch_key ^ x) < 0 /* False */ {
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
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        let ao = mload64!(a.wrapping_add(16) as usize);
                                        b = mload32!(a as usize);
                                        if b != 0 {
                                            g = ao;
                                        } else {
                                            g = 0 /* False */;
                                        }
                                        let a_hi = mload64!(a.wrapping_add(8) as usize);
                                        if b != 0 {
                                            f = a_hi;
                                        } else {
                                            f = 0 /* False */;
                                        }
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
                                        let mut sv5_0_i64 = mload64!(b as usize);
                                        e = sv5_0_i64;
                                        h = mload64!(a.wrapping_add(112) as usize);
                                        k = h.wrapping_add(u);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(e.wrapping_add(o));
                                        if (e ^ o ^ 18446744073709551615) & (e ^ h) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = h as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        h = mload64!(a.wrapping_add(128) as usize);
                                        k = h.wrapping_add(m);
                                        h = (((k as u64) < h as u64) as i32 as u32 as i64).wrapping_add(min_receive.wrapping_add(e));
                                        if (e ^ min_receive ^ 18446744073709551615) & (e ^ h) < 0 /* False */ {
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
                                        let mut sv4_280_i64 = 1 /* True */ as i64;
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
                                        let mut sv4_64_i64 = ar as i64;
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
                                        let mut sv4_296_i64 = u as i64;
                                        let mut sv4_288_i64 = min_receive as i64;
                                        let mut sv4_280_i64 = m as i64;
                                        let mut sv4_256_i64 = j as i64;
                                        let mut sv4_248_i64 = g as i64;
                                        let av = Self::launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let aw = Self::pack_i128_val(env, launch_key, f);
                                        let mut sv4_560_i64 = aw as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ax = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = ax;
                                        let ay = Self::launch_key_event_payload(env, b);
                                        launch_key = ay;
                                        Self::contract_info_to_val(env, c);
                                        let mut sv4_560_i64 = launch_key as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ba = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(av), val_from_i64(ba));
                                        global0 = a.wrapping_add(576);
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
        min_receive: i128,
    ) {
        let mut global0: i32 = 0;
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
        a = global0.wrapping_sub(576);
        global0 = a;
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
                            let mut sv4_248_i64 = 1 /* True */ as i64;
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
                                        address_from_i64(env, user).require_auth();
                                        let mut sv4_304_i64 = user as i64;
                                        let mut sv4_296_i64 = launch_key as i64;
                                        let mut sv4_288_i64 = k as i64;
                                        let mut sv4_280_i64 = 0 /* Void */ as i64;
                                        env.storage().get_contract_data(a, a.wrapping_add(280));
                                        b = mload32!(a as usize);
                                        let a_hi = mload64!(a.wrapping_add(8) as usize);
                                        if b != 0 {
                                            e = a_hi;
                                        } else {
                                            e = 0 /* False */;
                                        }
                                        let z = mload64!(a.wrapping_add(16) as usize);
                                        if b != 0 {
                                            h = z;
                                        } else {
                                            h = 0 /* False */;
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
                                        let mut loaded_val = mload64!(a.wrapping_add(56) as usize);
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
                                        let mut sv5_0_i64 = mload64!(b as usize);
                                        e = sv5_0_i64;
                                        let mut sv4_96_i64 = mload64!(a.wrapping_add(96) as usize);
                                        f = sv4_96_i64;
                                        i = e.wrapping_sub(min_receive).wrapping_sub(((f as u64) < g as u64) as i32 as u32 as i64);
                                        if (e ^ min_receive) & (e ^ i) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_96_i64 = f.wrapping_sub(g) as i64;
                                        b = a.wrapping_add(120);
                                        e = sv5_0_i64;
                                        let mut sv4_112_i64 = mload64!(a.wrapping_add(112) as usize);
                                        f = sv4_112_i64;
                                        i = e.wrapping_sub(sending).wrapping_sub(((f as u64) < m as u64) as i32 as u32 as i64);
                                        if (e ^ sending) & (e ^ i) < 0 /* False */ {
                                            break 'label0;
                                        }
                                        sv5_0_i64 = i as i64;
                                        sv4_112_i64 = f.wrapping_sub(m) as i64;
                                        b = a.wrapping_add(136);
                                        e = sv5_0_i64;
                                        f = mload64!(a.wrapping_add(128) as usize);
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
                                        let mut sv4_64_i64 = mload64!(a.wrapping_add(64) as usize);
                                        let ah = Self::token_transfer_checked(
                                            env,
                                            sv4_64_i64,
                                            q,
                                            o,
                                        );
                                        let mut sv4_64_i64 = ah as i64;
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
                                        let mut sv4_296_i64 = m as i64;
                                        let mut sv4_288_i64 = min_receive as i64;
                                        let mut sv4_280_i64 = g as i64;
                                        let mut sv4_256_i64 = k as i64;
                                        let mut sv4_248_i64 = f as i64;
                                        let ak = Self::launch_snapshot_to_val(env, a.wrapping_add(248));
                                        let al = Self::pack_i128_val(env, h, j);
                                        let mut sv4_560_i64 = al as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let am = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        user = am;
                                        let an = Self::launch_key_event_payload(env, b);
                                        sending = an;
                                        let ao = Self::contract_info_to_val(env, c);
                                        let mut sv4_560_i64 = sending as i64;
                                        let mut sv4_552_i64 = user as i64;
                                        let ap = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                        env.events().publish(val_from_i64(ak), val_from_i64(ap));
                                        global0 = a.wrapping_add(576);
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
        launch_key: (Address, u64),
    ) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        a = global0.wrapping_sub(368);
        global0 = a;
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
                            let mut sv1_168_i64 = 1 /* True */ as i64;
                            b = a.wrapping_add(168);
                            env.storage().put_contract_data(b, a);
                            d = mload64!(a.wrapping_add(152) as usize);
                            let o = Self::storage_key_from_str(env, 1048597, 20);
                            mstore64!(a.wrapping_add(192) as usize, d as u64);
                            mstore64!(a.wrapping_add(184) as usize, launch_key as u64);
                            let mut sv1_176_i64 = c as i64;
                            let mut sv1_168_i64 = o as i64;
                            let p = Self::launch_snapshot_to_val(env, b);
                            let q = Self::contract_info_to_val(env, a);
                            env.events().publish(val_from_i64(p), val_from_i64(q));
                            global0 = a.wrapping_add(368);
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
        launch_key: (Address, u64),
    ) {
        let mut global0: i32 = 0;
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
        a = global0.wrapping_sub(448);
        global0 = a;
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
                    let mut sv2_400_i64 = 1 /* True */ as i64;
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
                            let mut sv2_232_i64 = 0 /* Void */ as i64;
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
                                i = 0 /* False */;
                            }
                            if b != 0 {
                                h = h;
                            } else {
                                h = 0 /* False */;
                            }
                            Self::persist_launch_state(
                                env,
                                mload64!(a.wrapping_add(200) as usize),
                                d,
                                user,
                                i,
                                h,
                            );
                            let mut sv2_256_i64 = user as i64;
                            let mut sv2_248_i64 = launch_key as i64;
                            let mut sv2_240_i64 = f as i64;
                            let mut sv2_232_i64 = 0 /* Void */ as i64;
                            let t = Self::data_key_to_val(env, c);
                            env.storage().del_contract_data(t);
                            b = a.wrapping_add(136);
                            d = mload64!(b as usize);
                            e = mload64!(a.wrapping_add(128) as usize);
                            g = e.wrapping_add(i);
                            e = (((g as u64) < e as u64) as i32 as u32 as i64).wrapping_add(d.wrapping_add(h));
                            if (d ^ h ^ 18446744073709551615) & (d ^ e) >= 0 /* False */ {
                                let mut sv2_248_i64 = launch_key as i64;
                                let mut sv2_240_i64 = f as i64;
                                let mut sv2_232_i64 = 1 /* True */ as i64;
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
                                            let cond_val = if d == 0 { (e == 0) as i32 } else { (d < 0 /* False */) as i32 };
                                            if cond_val == 0 {
                                                let y = Self::storage_key_from_str(env, 1049284, 4);
                                                g = y;
                                                let z = Self::pack_i128_val(env, e, d);
                                                let mut sv2_408_i64 = z as i64;
                                                let mut sv2_400_i64 = user as i64;
                                                b = 0;
                                                while b != 16 {
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, 0 /* Void */ as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                b = 0;
                                                loop {
                                                    let aa = mload64!(a.wrapping_add(400).wrapping_add(b) as usize);
                                                    mstore64!(a.wrapping_add(232).wrapping_add(b) as usize, aa as u64);
                                                    b = b.wrapping_add(8);
                                                }
                                                let ab = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                let _ = env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(j)), &Symbol::from_val(env, &val_from_i64(g)), Vec::<Val>::from_val(env, &val_from_i64(ab)));
                                                break 'label0;
                                            }
                                            j = mload64!(a.wrapping_add(216) as usize);
                                            let ac = Self::storage_key_from_str(env, 1048637, 12);
                                            g = ac;
                                            mstore64!(a.wrapping_add(256) as usize, j as u64);
                                            mstore64!(a.wrapping_add(248) as usize, launch_key as u64);
                                            let mut sv2_240_i64 = f as i64;
                                            let mut sv2_232_i64 = g as i64;
                                            let ad = Self::launch_snapshot_to_val(env, a.wrapping_add(232));
                                            let ae = Self::pack_i128_val(env, i, h);
                                            f = ae;
                                            let af = Self::pack_i128_val(env, e, d);
                                            let mut sv2_408_i64 = af as i64;
                                            let mut sv2_400_i64 = f as i64;
                                            let ag = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            let ah = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                            env.events().publish(val_from_i64(ad), val_from_i64(ah));
                                            global0 = a.wrapping_add(448);
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
        sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut e: i64 = 0;
        a = global0.wrapping_sub(368);
        global0 = a;
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
        global0 = a.wrapping_add(368);
        k
    }

    pub fn calculate_sell(
        env: Env,
        mut launch_key: (Address, u64),
        sending: i128,
    ) -> (i128, i128, i128, i128, i128) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut e: i64 = 0;
        a = global0.wrapping_sub(368);
        global0 = a;
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
        global0 = a.wrapping_add(368);
        k
    }

    pub fn get_launch_data(
        env: Env,
        launch_key: (Address, u64),
    ) -> Launch {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        a = global0.wrapping_sub(368);
        global0 = a;
        Self::write_launch_key(env, a.wrapping_add(168), launch_key);
        'label0: {
            let a_part_168 = mload64!(a.wrapping_add(168) as usize);
            if a_part_168 == 0 {
                launch_key = mload64!(a.wrapping_add(176) as usize);
                let c = mload64!(a.wrapping_add(184) as usize);
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
        global0 = a.wrapping_add(368);
        f
    }

    pub fn get_contract_info(
        env: Env,
    ) -> ContractInfo {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = global0.wrapping_sub(128);
        global0 = a;
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
        global0 = a.wrapping_add(128);
        f
    }

    pub fn get_launch_balance(
        env: Env,
        launch_key: (Address, u64),
        user: Address,
    ) -> i128 {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = global0.wrapping_add(-64);
        global0 = a;
        Self::write_launch_key(env, a.wrapping_add(32), launch_key);
        let a_part_32 = mload64!(a.wrapping_add(32) as usize);
        if (!(a_part_32 == 0)) as i32 | Address::try_from_val(env, &val_from_i64(user)).is_ok() {
            let e = mload64!(a.wrapping_add(48) as usize);
            env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(32));
            let mut value_lo = mload64!(a.wrapping_add(16) as usize);
            b = mload32!(a.wrapping_add(8) as usize);
            let f = mload64!(a.wrapping_add(24) as usize);
            let call_arg1 = if b != 0 { value_lo } else { 0 /* False */ };
            let call_arg2 = if b != 0 { f } else { 0 /* False */ };
            let g = Self::pack_i128_val(env, call_arg1, call_arg2);
            global0 = a.wrapping_sub(-64);
            return g;
        }
    }

    pub fn version(
        env: Env,
    ) -> (u32, u32, u32) {
        let mut global0: i32 = 0;
        let mut a: i32 = 0;
        a = global0.wrapping_sub(32);
        global0 = a;
        let d = val_to_i64(Vec::<Val>::new(env).into_val(env));
        global0 = a.wrapping_add(32);
        d
    }
}