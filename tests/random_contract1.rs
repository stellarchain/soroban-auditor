#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, String, Bytes, BytesN, Val, FromVal, Map, Symbol};

#[contract]
pub struct RandomContract1;

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
pub struct WritePrices { pub updated_feeds: Vec<PriceData>, pub updater: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PriceData { pub package_timestamp: u64, pub price: U256, pub write_timestamp: u64, }

#[contractimpl]
impl RandomContract1 {

    pub fn init(
        &mut self,
        env: Env,
        owner: Address,
    ) -> Result<(), Error> {
        if Address::try_from_val(env, &val_from_i64(owner)).is_ok() {
            let a = val_to_i64(String::from_str(env, "owner"));
            let b = self.call_eq_one(env, a, 0 /* Void */);
            let c: i64;
            if b != 0 {
                c = Error(Storage, ExistingValue);
            } else {
                env.storage().put_contract_data(1048592, 5, owner);
                c = 0 /* Void */;
            }
            return c;
        }
    }

    pub fn change_owner(
        &mut self,
        env: Env,
        new_owner: Address,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        if Address::try_from_val(env, &val_from_i64(new_owner)).is_ok() {
            self.require_owner_auth(env, a);
            let c: i64;
            {
                let mut sv1_0_i32 = mload32!(a as usize) as i32;
                if sv1_0_i32 != 0 {
                    c = mload64!(a as usize + 8) as i64;
                } else {
                    env.storage().put_contract_data(1048597, 13, new_owner);
                    c = 0 /* Void */;
                }
            }
            self.global0 = a.wrapping_add(16);
            return c;
        }
    }

    pub fn accept_ownership(
        &mut self,
        env: Env,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.require_auth_for_key(
            env,
            a,
            1048597,
            13,
        );
        b = mload64!(a as usize + 8) as i64;
        let mut sv0_0_i32 = mload32!(a as usize) as i32;
        if sv0_0_i32 == 0 {
            env.storage().put_contract_data(1048592, 5, b);
            let d = val_to_i64(String::from_str(env, "pending-owner"));
            env.storage().del_contract_data(d);
            b = 0 /* Void */;
        }
        self.global0 = a.wrapping_add(16);
        b
    }

    pub fn cancel_ownership_transfer(
        &mut self,
        env: Env,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.require_owner_auth(env, a);
        let d: i64;
        {
            let mut sv0_0_i32 = mload32!(a as usize) as i32;
            if sv0_0_i32 != 0 {
                d = mload64!(a as usize + 8) as i64;
            } else {
                let e = val_to_i64(String::from_str(env, "pending-owner"));
                env.storage().del_contract_data(e);
                d = 0 /* Void */;
            }
        }
        self.global0 = a.wrapping_add(16);
        d
    }

    pub fn upgrade(
        &mut self,
        env: Env,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        if Bytes::try_from_val(env, &val_from_i64(new_wasm_hash)).is_ok() {
            let c = Bytes::from_val(env, &val_from_i64(new_wasm_hash)).len() as i64;
            if c & 18446744069414584320 == 137438953472 {
                self.require_owner_auth(env, a);
                let d: i64;
                {
                    let mut sv1_0_i32 = mload32!(a as usize) as i32;
                    if sv1_0_i32 != 0 {
                        d = mload64!(a as usize + 8) as i64;
                    } else {
                        env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(new_wasm_hash)));
                        d = 0 /* Void */;
                    }
                }
                self.global0 = a.wrapping_add(16);
                return d;
            }
        }
        unreachable!();
    }

    pub fn get_prices(
        &mut self,
        env: Env,
        mut feed_ids: Vec<String>,
        payload: Bytes,
    ) -> Result<(u64, Vec<U256>), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        a = self.global0.wrapping_sub(96);
        self.global0 = a;
        if !((!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | !(Bytes::try_from_val(env, &val_from_i64(payload)).is_ok())) {
            self.entry_decode(
                env,
                a,
                feed_ids,
                payload,
            );
            self.decode_val_or_error(env, a.wrapping_sub(-64), a);
            payload = mload64!(a as usize + 72) as i64;
            let e: i64;
            'label1: {
                let mut sv2_64_i32 = mload32!(a as usize + 64) as i32;
                if sv2_64_i32 == 0 {
                    b = mload64!(a as usize + 80) as i64;
                    let f = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
                    let g = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
                    let error_code = 42949672963 /* Error(Contract, #10) */;
                    if (f ^ g) as u64 >= 4294967296 as u64 {
                        e = error_code;
                        break 'label1;
                    }
                    let h = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
                    c = h;
                    let i = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    feed_ids = i;
                    mstore32!(a as usize + 60, (c as u64).wrapping_shr(32 as u32) as i64 as u32);
                    loop {
                        self.vec_pair_iter(env, a, a.wrapping_add(48));
                        self.copy_val_if_present(env, a.wrapping_sub(-64), a);
                        let mut sv2_64_i32 = mload32!(a as usize + 64) as i32;
                        if sv2_64_i32 == 1 {
                            let mut loaded_val = mload64!(a as usize + 80) as i64;
                            let j = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)); v.push_back(val_from_i64(loaded_val)); val_to_i64(v.into_val(env)) };
                        }
                    }
                    self.write_ok_val(env, a, payload);
                    let mut sv2_0_i32 = mload32!(a as usize) as i32;
                    if sv2_0_i32 != 0 {
                        unreachable!();
                    }
                    payload = mload64!(a as usize + 8) as i64;
                    let k = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    e = k;
                    break 'label1;
                }
                e = payload;
            }
            self.global0 = a.wrapping_add(96);
            return e;
        }
        42949672963 /* Error(Contract, #10) */
    }

    pub fn write_prices(
        &mut self,
        env: Env,
        updater: Address,
        mut feed_ids: Vec<String>,
        payload: Bytes,
    ) -> Result<(), Error> {
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut vec_builder: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        c = self.global0.wrapping_sub(144);
        self.global0 = c;
        if (!(Address::try_from_val(env, &val_from_i64(updater)).is_ok())) as i32 | (!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | Bytes::try_from_val(env, &val_from_i64(payload)).is_ok() {
            address_from_i64(env, updater).require_auth();
            env.storage().instance().extend_ttl(INSTANCE_BUMP_AMOUNT as u32, 181440 as u32);
            while d != 16 {
                let l = mload32!(d.wrapping_add(1048776) as usize) as i32;
                let m = mload32!(d.wrapping_add(1048780) as usize) as i32;
                let o = val_to_i64(Address::from_string_bytes(&Bytes::from_val(env, &String::from_str(env, ""))).into_val(env));
                mstore64!(c.wrapping_add(120).wrapping_add(d) as usize, o as u64);
                d = d.wrapping_add(8);
            }
            loop {
                d = e;
                if d == 16 {
                    break;
                }
                if d != 16 {
                    e = d.wrapping_add(8);
                    let p = mload64!(c.wrapping_add(120).wrapping_add(d) as usize) as i64;
                    let q = { let a = val_from_i64(p); let b = val_from_i64(updater); if a < b { -1 } else if a > b { 1 } else { 0 } };
                }
            }
            e = c.wrapping_add(8);
            self.entry_decode(
                env,
                e,
                feed_ids,
                payload,
            );
            self.decode_val_or_error(env, c.wrapping_add(96), e);
            payload = mload64!(c as usize + 104) as i64;
            let mut sv3_96_i32 = mload32!(c as usize + 96) as i32;
            if sv3_96_i32 == 0 {
                feed_ids = mload64!(c as usize + 112) as i64;
                let r = self.ledger_timestamp_val(env);
                h = r;
                let s = val_to_i64(Vec::<Val>::new(env).into_val(env));
                vec_builder = s;
                let t = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
                g = t;
                mstore32!(c as usize + 68, (g as u64).wrapping_shr(32 as u32) as i64 as u32);
                e = (d == 16) as i32;
                'label2: loop {
                    {
                        d = c.wrapping_add(8);
                        self.vec_pair_iter(env, d, c.wrapping_add(56));
                        self.copy_val_if_present(env, c.wrapping_add(72), d);
                        let mut sv3_72_i32 = mload32!(c as usize + 72) as i32;
                        if sv3_72_i32 == 0 {
                            feed_ids = 0 /* Void */;
                            d = 1;
                            loop {
                                if d == 0 {
                                    unreachable!();
                                }
                                d -= 1;
                                feed_ids = REDSTONE;
                            }
                            unreachable!();
                        }
                        g = mload64!(c as usize + 80) as i64;
                        i = mload64!(c as usize + 88) as i64;
                        env.storage().get_contract_data(c.wrapping_add(8), g);
                        f = mload32!(c as usize + 8) as i32;
                        let mut value_hi = mload64!(c as usize + 24) as i64;
                        d = f & (value_hi as u64 >= payload as u64) as i32;
                        feed_ids = mload64!(c as usize + 32) as i64;
                        if e == 0 {
                            if d != 0 {
                                continue 'label2;
                            }
                        } else {
                            if d != 0 {
                                continue 'label2;
                            }
                            j = feed_ids.wrapping_add(40000);
                            feed_ids = (if (feed_ids as u64 > j as u64) as i32 != 0 { 18446744073709551615 } else { j });
                        }
                        if (feed_ids as u64 >= h as u64) as i32 & f != 0 {
                            continue 'label2;
                        }
                        let u = self.result_unwrap_or_panic(env, c.wrapping_add(96));
                        let w = env.storage().temporary().set(&val_from_i64(g), &val_from_i64(u));
                        let x = env.storage().temporary().extend_ttl(&val_from_i64(g), 34560 as u32, 51840 as u32);
                        let mut value_hi = h as i64;
                        let mut sv3_16_i64 = payload as i64;
                        let mut sv3_8_i64 = i as i64;
                        let y = self.result_unwrap_or_panic(env, c.wrapping_add(8));
                        vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(y)); val_to_i64(v.into_val(env)) };
                        continue 'label2;
                    }
                }
                let mut sv3_8_i64 = feed_ids as i64;
                d = c.wrapping_add(8);
                let aa = val_to_i64(Vec::<Val>::new(env).into_val(env));
                let mut sv3_16_i64 = updater as i64;
                let mut sv3_8_i64 = vec_builder as i64;
                let ab = self.map_new_val(
                    env,
                    1048884,
                    2,
                    d,
                    2,
                );
                let ac = val_to_i64(Bytes::from_val(env, &val_from_i64(ab)).into_val(env));
                env.events().publish(val_from_i64(aa), val_from_i64(ac));
                payload = 0 /* Void */;
            }
            self.global0 = c.wrapping_add(144);
            return payload;
            unreachable!();
        }
        unreachable!();
    }

    pub fn read_prices(
        &mut self,
        env: Env,
        feed_ids: Vec<String>,
    ) -> Result<Vec<U256>, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut vec_builder: i64 = 0;
        let mut e: i64 = 0;
        a = self.global0.wrapping_sub(96);
        self.global0 = a;
        if Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok() {
            let g = val_to_i64(Vec::<Val>::new(env).into_val(env));
            vec_builder = g;
            let h = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
            e = h;
            mstore32!(a as usize + 20, (e as u64).wrapping_shr(32 as u32) as i64 as u32);
            b = a.wrapping_add(48);
            c = a.wrapping_add(80);
            'label0: {
                loop {
                    d = a.wrapping_sub(-64);
                    self.vec_next_string_flag(env, d, a.wrapping_add(8));
                    let mut sv1_64_i64 = mload64!(a as usize + 64) as i64;
                    let mut loaded_val = mload64!(a as usize + 72) as i64;
                    self.guard_nonzero_ptr(
                        env,
                        a.wrapping_add(24),
                        sv1_64_i64,
                        loaded_val,
                    );
                    let mut sv1_24_i32 = mload32!(a as usize + 24) as i32;
                    if sv1_24_i32 == 0 {
                        unreachable!();
                    }
                    let mut sv1_32_i64 = mload64!(a as usize + 32) as i64;
                    env.storage().get_contract_data(d, sv1_32_i64);
                    let mut sv1_64_i32 = mload32!(a as usize + 64) as i32;
                    if sv1_64_i32 == 1 {
                        let i = mload64!(c.wrapping_add(8) as usize) as i64;
                        mstore64!(b.wrapping_add(8) as usize, i as u64);
                        let mut loaded_val = mload64!(a as usize + 72) as i64;
                        self.check_recent_timestamp(env, d, a.wrapping_add(40));
                        feed_ids = mload64!(a as usize + 72) as i64;
                        let mut sv1_64_i32 = mload32!(a as usize + 64) as i32;
                        if sv1_64_i32 != 0 {
                            vec_builder = feed_ids;
                            break 'label0;
                        }
                        vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(feed_ids)); val_to_i64(v.into_val(env)) };
                    }
                }
                vec_builder = Error(Storage, MissingValue);
            }
            self.global0 = a.wrapping_add(96);
            return vec_builder;
        }
    }

    pub fn read_timestamp(
        &mut self,
        env: Env,
        feed_id: String,
    ) -> Result<u64, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        let d: i64;
        'label0: {
            'label1: {
                'label2: {
                    if String::try_from_val(env, &val_from_i64(feed_id)).is_ok() {
                        b = a.wrapping_add(32);
                        env.storage().get_contract_data(b, feed_id);
                        let mut sv1_32_i32 = mload32!(a as usize + 32) as i32;
                        if sv1_32_i32 != 0 {
                            unreachable!();
                        }
                        break 'label2;
                        let e = mload64!(a.wrapping_add(56) as usize) as i64;
                        mstore64!(a.wrapping_add(24) as usize, e as u64);
                        let mut sv1_48_i64 = mload64!(a as usize + 48) as i64;
                        self.check_recent_timestamp(env, b, a.wrapping_add(8));
                        let mut sv1_32_i32 = mload32!(a as usize + 32) as i32;
                        if sv1_32_i32 != 0 {
                            unreachable!();
                        }
                        let mut sv1_48_i64 = mload64!(a as usize + 48) as i64;
                        self.write_ok_val(env, b, sv1_48_i64);
                        let mut sv1_32_i32 = mload32!(a as usize + 32) as i32;
                        if sv1_32_i32 != 0 {
                            unreachable!();
                        }
                        break 'label1;
                    }
                    unreachable!();
                }
                d = Error(Storage, MissingValue);
                break 'label0;
            }
            d = mload64!(a as usize + 40) as i64;
        }
        self.global0 = a.wrapping_sub(-64);
        d
    }

    pub fn read_price_data_for_feed(
        &mut self,
        env: Env,
        feed_id: String,
    ) -> Result<PriceData, Error> {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(96);
        self.global0 = a;
        if String::try_from_val(env, &val_from_i64(feed_id)).is_ok() {
            env.storage().get_contract_data(a.wrapping_add(40), feed_id);
            {
                let mut sv1_40_i32 = mload32!(a as usize + 40) as i32;
                if sv1_40_i32 == 1 {
                    let c = mload64!(a.wrapping_sub(-64) as usize) as i64;
                    mstore64!(a.wrapping_add(88) as usize, c as u64);
                    self.check_recent_timestamp(env, a.wrapping_add(8), a.wrapping_add(72));
                } else {
                    let mut error_code = Error(Storage, MissingValue) as i64;
                }
            }
            let d = self.result_from_val(env, a.wrapping_add(8));
            self.global0 = a.wrapping_add(96);
            return d;
        }
    }

    pub fn read_price_data(
        &mut self,
        env: Env,
        feed_ids: Vec<String>,
    ) -> Result<Vec<PriceData>, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut vec_builder: i64 = 0;
        let mut g: i64 = 0;
        a = self.global0.wrapping_sub(112);
        self.global0 = a;
        'label0: {
            'label1: {
                if Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok() {
                    let i = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    vec_builder = i;
                    let j = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
                    g = j;
                    mstore32!(a as usize + 12, (g as u64).wrapping_shr(32 as u32) as i64 as u32);
                    c = a.wrapping_add(96);
                    b = a.wrapping_add(72);
                    d = a.wrapping_add(40);
                    loop {
                        e = a.wrapping_add(56);
                        self.vec_next_string_flag(env, e, a);
                        let mut loaded_val = mload64!(a as usize + 56) as i64;
                        let mut sv1_64_i64 = mload64!(a as usize + 64) as i64;
                        self.guard_nonzero_ptr(
                            env,
                            a.wrapping_add(16),
                            loaded_val,
                            sv1_64_i64,
                        );
                        let mut sv1_16_i32 = mload32!(a as usize + 16) as i32;
                        if sv1_16_i32 == 0 {
                            unreachable!();
                        }
                        let mut value_hi = mload64!(a as usize + 24) as i64;
                        env.storage().get_contract_data(e, value_hi);
                        let mut sv1_56_i32 = mload32!(a as usize + 56) as i32;
                        if sv1_56_i32 != 0 {
                            unreachable!();
                        }
                        break 'label1;
                        let mut sv2_0_i64 = mload64!(b as usize) as i64;
                        f = b.wrapping_add(8);
                        let mut sv6_0_i64 = mload64!(f as usize) as i64;
                        mstore64!(d.wrapping_add(8) as usize, sv6_0_i64 as u64);
                        let mut sv1_64_i64 = mload64!(a as usize + 64) as i64;
                        self.check_recent_timestamp(env, e, a.wrapping_add(32));
                        feed_ids = mload64!(a as usize + 64) as i64;
                        let mut sv1_56_i32 = mload32!(a as usize + 56) as i32;
                        if sv1_56_i32 == 0 {
                            let mut sv2_0_i64 = mload64!(b as usize) as i64;
                            let mut sv6_0_i64 = mload64!(f as usize) as i64;
                            mstore64!(c.wrapping_add(8) as usize, sv6_0_i64 as u64);
                            let k = self.result_unwrap_or_panic(env, a.wrapping_add(88));
                            vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(k)); val_to_i64(v.into_val(env)) };
                        }
                    }
                    vec_builder = feed_ids;
                    break 'label0;
                }
                unreachable!();
            }
            vec_builder = Error(Storage, MissingValue);
        }
        self.global0 = a.wrapping_add(112);
        vec_builder
    }

    pub fn check_price_data(
        &mut self,
        env: Env,
        price_data: PriceData,
    ) -> Result<PriceData, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        self.map_unpack_to_val(env, a.wrapping_add(32), price_data);
        let mut sv1_32_i32 = mload32!(a as usize + 32) as i32;
        if sv1_32_i32 == 1 {
            unreachable!();
        }
        let d = mload64!(a.wrapping_add(56) as usize) as i64;
        mstore64!(a.wrapping_add(24) as usize, d as u64);
        let e = mload64!(a.wrapping_add(48) as usize) as i64;
        mstore64!(a.wrapping_add(16) as usize, e as u64);
        b = a.wrapping_add(32);
        self.check_recent_timestamp(env, b, a.wrapping_add(8));
        let f = self.result_from_val(env, b);
        self.global0 = a.wrapping_sub(-64);
        f
    }

    pub fn unique_signer_threshold(
        &mut self,
        env: Env,
    ) -> u64 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.write_ok_val(env, a, 0 /* Void */);
        let mut sv0_0_i32 = mload32!(a as usize) as i32;
        if sv0_0_i32 == 1 {
            unreachable!();
        }
        let mut sv0_8_i64 = mload64!(a as usize + 8) as i64;
        self.global0 = a.wrapping_add(16);
        sv0_8_i64
    }
}

impl RandomContract1 {

    fn vec_next_string_to_linear(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        b = a.wrapping_add(16);
        self.vec_next_string_flag(env, b, arg1);
        let mut value_lo = mload64!(a as usize + 16) as i64;
        let mut value_hi = mload64!(a as usize + 24) as i64;
        self.guard_nonzero_ptr(
            env,
            a,
            value_lo,
            value_hi,
        );
        'label0: {
            {
                let mut sv2_0_i32 = mload32!(a as usize) as i32;
                let h: i32;
                if sv2_0_i32 == 1 {
                    e = mload64!(a as usize + 8) as i64;
                    arg1 = a.wrapping_add(40);
                    let mut svarg1_0_i64 = 0 /* False */ as i64;
                    c = a.wrapping_add(32);
                    let mut sv4_0_i64 = 0 /* False */ as i64;
                    d = a.wrapping_add(24);
                    let mut sv5_0_i64 = 0 /* False */ as i64;
                    let mut value_lo = 0 /* False */ as i64;
                    let i = String::from_val(env, &val_from_i64(e)).len() as i64;
                    f = i;
                    if f as u64 >= 141733920768 as u64 {
                        unreachable!();
                    }
                    let j = String::from_val(env, &val_from_i64(e)).len() as i64;
                    if (j ^ f) as u64 >= 4294967296 as u64 {
                        break 'label0;
                    }
                    self.copy_string_to_linear_memory(env, e, 0, (b as u32 as i64).wrapping_shl(32 as u32) | 0, f & 270582939648 | 0);
                    let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                    mstore64!(arg0.wrapping_add(25) as usize, svarg1_0_i64 as u64);
                    let mut sv4_0_i64 = mload64!(c as usize) as i64;
                    mstore64!(arg0.wrapping_add(17) as usize, sv4_0_i64 as u64);
                    let mut sv5_0_i64 = mload64!(d as usize) as i64;
                    mstore64!(arg0.wrapping_add(9) as usize, sv5_0_i64 as u64);
                    let mut value_lo = mload64!(a as usize + 16) as i64;
                    h = 1;
                } else {
                    h = 0;
                }
                mstore8!(arg0 as usize, h as u8);
                self.global0 = a.wrapping_add(48);
            }
        }
    }

    fn vec_next_string_flag(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = mload32!(arg1 as usize + 8) as i32;
        let mut svarg1_12_i32 = mload32!(arg1 as usize + 12) as i32;
        if a as u32 >= svarg1_12_i32 as u32 {
            let mut svarg0_0_i64 = 0 /* Void */ as i64;
        }
        let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
        let c = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(svarg1_0_i64)).get_unchecked((a as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
        b = c;
        a += 1;
        if a != 0 {
            let mut svarg0_0_i64 = (!(String::try_from_val(env, &val_from_i64(b)).is_ok())) as i32 as u32 as i64 as i64;
        }
    }

    fn guard_nonzero_ptr(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        let c: i64;
        if arg1 != 0 /* Void */ {
            if arg1 as i32 & 1 != 0 {
                unreachable!();
            }
        }
        self.global0 = a.wrapping_add(16);
    }

    fn span_from_range(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        if arg1 as u32 > arg2 as u32 {
            unreachable!();
        }
        arg1 = arg2.wrapping_sub(arg1);
    }


    fn call_eq_one(
        &mut self,
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let a = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (a == 1 /* True */) as i32
    }

    fn map_unpack_to_val(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        loop {
            if b != 24 {
                mstore64!(a.wrapping_add(8).wrapping_add(b) as usize, 0 /* Void */ as u64);
                b = b.wrapping_add(8);
            }
        }
        'label1: {
            'label2: {
                if Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok() {
                    let f = 0;
                    b = a.wrapping_add(32);
                    let mut sv2_8_i64 = mload64!(a as usize + 8) as i64;
                    self.val_to_i64_checked(env, b, sv2_8_i64);
                    let mut sv2_32_i32 = mload32!(a as usize + 32) as i32;
                    if sv2_32_i32 != 0 {
                        unreachable!();
                    }
                    arg1 = mload64!(a as usize + 40) as i64;
                    d = mload64!(a as usize + 16) as i64;
                    c = d as i32 & 255;
                    if (c != 12) as i32 & c != 70 {
                        break 'label2;
                    }
                    let mut value_hi = mload64!(a as usize + 24) as i64;
                    self.val_to_i64_checked(env, b, value_hi);
                    let mut sv2_32_i32 = mload32!(a as usize + 32) as i32;
                    if sv2_32_i32 == 0 {
                        unreachable!();
                    }
                    break 'label1;
                }
                break 'label1;
                break 'label1;
            }
        }
        self.global0 = a.wrapping_add(48);
    }



    fn decode_error_from_val(
        &mut self,
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = 509;
        'label0: {
            'label1: {
                {
                    {
                        {
                            {
                                {
                                    let d = mload8!(arg0 as usize) as i32;
                                    loop {
                                        match d.wrapping_sub(1) {
                                            0 => break 'label1,
                                            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | _ => break,
                                        }
                                    }
                                }
                                a = 510;
                                break 'label1;
                                a = 0;
                                'label28: {
                                    {
                                        {
                                            let mut svarg0_4_i32 = mload32!(arg0 as usize + 4) as i32;
                                            b = svarg0_4_i32 ^ -2147483648;
                                            loop {
                                                match ((if (b as u32 >= 4 as u32) as i32 != 0 { 1 } else { b })).wrapping_sub(1) {
                                                    0 => break,
                                                    1 => break 'label28,
                                                    2 | _ => break,
                                                }
                                            }
                                        }
                                        let e = mload8!(arg0 as usize + 8) as i32;
                                        a = e;
                                        break 'label28;
                                    }
                                    a = mload32!(arg0 as usize + 12) as i32;
                                    break 'label28;
                                    let mut svarg0_8_i32 = mload32!(arg0 as usize + 8) as i32;
                                    a = svarg0_8_i32;
                                }
                                a = (a & 65535).wrapping_add(700);
                                if a & 65535 == a {
                                    break 'label1;
                                }
                                break 'label0;
                                a = 511;
                                break 'label1;
                                a = 512;
                                break 'label1;
                                a = 520;
                                break 'label1;
                                let f = mload32!(arg0 as usize + 4) as i64;
                                c = f.wrapping_mul(0);
                                if (c as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                    unreachable!();
                                }
                                a = c as i32;
                                b = a.wrapping_add(2000);
                                if (b as u32) < a as u32 {
                                    break 'label0;
                                }
                                a = b.wrapping_add(svarg0_8_i32);
                                if b as u32 > a as u32 {
                                    break 'label0;
                                }
                                break 'label1;
                            }
                            a = 513;
                            break 'label1;
                            a = 514;
                            break 'label1;
                            a = 515;
                            break 'label1;
                            a = 521;
                            break 'label1;
                            a = 516;
                            break 'label1;
                            a = 517;
                            break 'label1;
                            a = 522;
                            break 'label1;
                            a = 523;
                            break 'label1;
                            a = 518;
                            break 'label1;
                            a = 519;
                            break 'label1;
                            a = 1101;
                            break 'label1;
                            a = 1102;
                            break 'label1;
                            a = 1200;
                            break 'label1;
                            a = 1300;
                            break 'label1;
                            a = 1400;
                            break 'label1;
                            a = 1500;
                            break 'label1;
                            let g = mload16!(arg0 as usize + 4) as i32;
                            a = g.wrapping_add(600);
                            if a & 65535 != a {
                                break 'label0;
                            }
                            break 'label1;
                        }
                        let h = mload16!(arg0 as usize + 4) as i32;
                        a = h.wrapping_add(1000);
                        if a & 65535 != a {
                            break 'label0;
                        }
                        break 'label1;
                    }
                    let i = mload16!(arg0 as usize + 4) as i32;
                    a = i.wrapping_add(1050);
                    if a & 65535 != a {
                        break 'label0;
                    }
                    break 'label1;
                }
                let j = mload8!(arg0 as usize + 16) as i32;
                a = j;
            }
            return (a as u32 as i64 & 65535).wrapping_shl(32 as u32) | 3 /* Error(Contract, #0) */;
        }
    }

    fn copy_val_if_present(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        b = mload64!(arg1 as usize) as i64;
        let d: i64;
        if b != 0 /* Void */ {
            if b as i32 & 1 != 0 {
                unreachable!();
            }
            let mut arg1_lo = mload64!(arg1 as usize + 16) as i64;
        }
        self.global0 = a.wrapping_add(16);
    }

    fn decode_val_or_error(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let c: i64;
        {
            let d = mload8!(arg1 as usize) as i32;
            if d == 27 {
                let mut arg1_lo = mload64!(arg1 as usize + 16) as i64;
                c = mload64!(arg1 as usize + 8) as i64;
            } else {
                let e = self.decode_error_from_val(env, arg1);
            }
        }
    }

    fn alloc_copy(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_range(
            env,
            a.wrapping_add(8),
            arg2,
            1,
            1,
        );
        let mut sv3_12_i32 = mload32!(a as usize + 12) as i32;
        let d = self.memmove(
            env,
            sv3_12_i32,
            arg1,
            arg2,
        );
        self.global0 = a.wrapping_add(16);
    }

    fn alloc_range(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_core(
            env,
            a.wrapping_add(4),
            arg1,
            0,
            arg2,
            arg3,
        );
        arg1 = mload32!(a as usize + 8) as i32;
        let mut sv4_4_i32 = mload32!(a as usize + 4) as i32;
        if sv4_4_i32 == 0 {
            let mut sv4_12_i32 = mload32!(a as usize + 12) as i32;
            self.global0 = a.wrapping_add(16);
        }
        self.alloc_trap(env, arg1, sv4_12_i32);
    }

    fn alloc_realloc(
        &mut self,
        env: &Env,
        mut arg0: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
        self.alloc_size_align(
            env,
            a.wrapping_add(8),
            arg0,
            svarg0_0_i32,
            1,
            1,
            64,
        );
        arg0 = mload32!(a as usize + 8) as i32;
        if arg0 != -2147483647 {
            let mut sv1_12_i32 = mload32!(a as usize + 12) as i32;
            self.alloc_trap(env, arg0, sv1_12_i32);
            unreachable!();
        }
        self.global0 = a.wrapping_add(16);
    }

    fn alloc_size_align(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        mut arg2: i32,
        mut arg3: i32,
        mut arg4: i32,
        arg5: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i64 = 0;
        b = self.global0.wrapping_sub(16);
        self.global0 = b;
        'label0: {
            c = arg2.wrapping_add(arg3);
            if arg2 as u32 <= c as u32 {
                e = 0.wrapping_sub(arg4);
                arg3 = arg4 -= 1;
                d = mload32!(arg1 as usize) as i32;
                arg2 = d.wrapping_shl(1 as u32);
                arg2 = (if (arg2 as u32 > c as u32) as i32 != 0 { arg2 } else { c });
                f = (if (arg2 as u32 <= 4 as u32) as i32 != 0 { 4 } else { arg2 });
                g = ((e & arg3.wrapping_add(arg5)) as u32 as i64).wrapping_mul(f as u32 as i64);
                if (g as u64).wrapping_shr(32 as u32) as (i64 == 0) {
                    arg2 = g as i32;
                    if arg2 as u32 <= (-2147483648).wrapping_sub(arg4) as u32 {
                        'label2: {
                            let i: i32;
                            'label3: {
                                if d != 0 {
                                    if arg5 == 0 {
                                        self.func108(
                                            env,
                                            b.wrapping_add(8),
                                            arg4,
                                            arg2,
                                        );
                                        i = mload32!(b as usize + 8) as i32;
                                        break 'label3;
                                    }
                                    c = mload32!(arg1 as usize + 4) as i32;
                                    self.memcpy_like_5(env);
                                    a = mload32!(1049932 as usize) as i32;
                                    arg3 = a.wrapping_add(arg3);
                                    if (arg3 as u32) < a as u32 {
                                        unreachable!();
                                    }
                                    {
                                        arg3 = arg3 & e;
                                        a = arg3.wrapping_add(arg2);
                                        let j = mload32!(1049936 as usize) as i32;
                                        if a as u32 > j as u32 {
                                            let k = self.func110(env, arg2, arg4);
                                            arg3 = k;
                                        } else {
                                            mstore32!(1049932 as usize, a as u32);
                                        }
                                    }
                                    if arg3 == 0 {
                                        unreachable!();
                                    }
                                    let l = self.memmove(
                                        env,
                                        arg3,
                                        c,
                                        arg5.wrapping_mul(d),
                                    );
                                    break 'label2;
                                }
                                self.func108(
                                    env,
                                    b,
                                    arg4,
                                    arg2,
                                );
                                i = mload32!(b as usize) as i32;
                            }
                            arg3 = i;
                            if arg3 == 0 {
                                unreachable!();
                            }
                        }
                        break 'label0;
                    }
                }
            }
        }
        self.global0 = b.wrapping_add(16);
    }

    fn alloc_trap(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        if arg0 == 0 {
            unreachable!();
        }
    }

    fn alloc_core(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        mut arg2: i32,
        mut arg3: i32,
        mut arg4: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        'label0: {
            a = 0.wrapping_sub(arg3);
            c = arg3 -= 1;
            d = ((a & c.wrapping_add(arg4)) as u32 as i64).wrapping_mul(arg1 as u32 as i64);
            if (d as u64).wrapping_shr(32 as u32) as (i64 == 0) {
                arg4 = d as i32;
                if arg4 as u32 <= (-2147483648).wrapping_sub(arg3) as u32 {
                    if arg4 == 0 {
                        let mut svarg0_8_i32 = arg3 as i32;
                        arg3 = 0;
                        let mut svarg0_4_i32 = 0 as i32;
                        break 'label0;
                    }
                    let e = mload8!(1049928 as usize) as i32;
                    {
                        if arg2 != 0 {
                            self.memcpy_like_5(env);
                            arg2 = mload32!(1049932 as usize) as i32;
                            c = arg2.wrapping_add(c);
                            if (c as u32) < arg2 as u32 {
                                unreachable!();
                            }
                            {
                                arg2 = a & c;
                                a = arg2.wrapping_add(arg4);
                                let f = mload32!(1049936 as usize) as i32;
                                if a as u32 > f as u32 {
                                    let g = self.func110(env, arg4, arg3);
                                    arg2 = g;
                                } else {
                                    mstore32!(1049932 as usize, a as u32);
                                }
                            }
                            if arg2 == 0 {
                                unreachable!();
                            }
                            arg3 = arg2;
                            if arg4 as u32 >= 16 as u32 {
                                a = 0.wrapping_sub(arg3) & 3;
                                b = arg3.wrapping_add(a);
                                if b as u32 > arg3 as u32 {
                                    if a != 0 {
                                        c = a;
                                        loop {
                                            mstore8!(arg3 as usize, 0 as u8);
                                            arg3 += 1;
                                            c -= 1;
                                        }
                                    }
                                    if (a.wrapping_sub(1) as u32) >= 7 as u32 {
                                        loop {
                                            mstore8!(arg3 as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(7) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(6) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(5) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(4) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(3) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(2) as usize, 0 as u8);
                                            mstore8!(arg3.wrapping_add(1) as usize, 0 as u8);
                                            arg3 = arg3.wrapping_add(8);
                                        }
                                    }
                                }
                                arg4 = arg4.wrapping_sub(a);
                                arg3 = b.wrapping_add(arg4 & -4);
                                if arg3 as u32 > b as u32 {
                                    loop {
                                        b = b.wrapping_add(4);
                                    }
                                }
                                arg4 = arg4 & 3;
                            }
                            a = arg3.wrapping_add(arg4);
                            if arg3 as u32 < a as u32 {
                                b = arg4 & 7;
                                if b != 0 {
                                    loop {
                                        mstore8!(arg3 as usize, 0 as u8);
                                        arg3 += 1;
                                        b -= 1;
                                    }
                                }
                                if (arg4.wrapping_sub(1) as u32) >= 7 as u32 {
                                    loop {
                                        mstore8!(arg3 as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(7) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(6) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(5) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(4) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(3) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(2) as usize, 0 as u8);
                                        mstore8!(arg3.wrapping_add(1) as usize, 0 as u8);
                                        arg3 = arg3.wrapping_add(8);
                                    }
                                }
                            }
                        } else {
                            let h = self.func111(env, arg4, arg3);
                            arg2 = h;
                            if arg2 == 0 {
                                unreachable!();
                            }
                        }
                        let mut svarg0_8_i32 = arg2 as i32;
                        let mut svarg0_4_i32 = arg1 as i32;
                        break 'label0;
                    }
                    break 'label0;
                }
            }
            break 'label0;
        }
    }

    fn result_unwrap_or_panic(
        &mut self,
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.result_from_val_unchecked(env, a, arg0);
        let mut sv1_0_i32 = mload32!(a as usize) as i32;
        if sv1_0_i32 == 1 {
            unreachable!();
        }
        let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
        self.global0 = a.wrapping_add(16);
        sv1_8_i64
    }

    fn result_from_val_unchecked(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        b = a.wrapping_add(32);
        let mut svarg1_8_i64 = mload64!(arg1 as usize + 8) as i64;
        self.write_ok_val(env, b, svarg1_8_i64);
        'label0: {
            {
                let mut sv2_32_i32 = mload32!(a as usize + 32) as i32;
                if sv2_32_i32 == 0 {
                    let mut arg1_lo = mload64!(arg1 as usize + 16) as i64;
                    self.write_ok_val(env, b, arg1_lo);
                    let mut sv2_32_i32 = mload32!(a as usize + 32) as i32;
                    if sv2_32_i32 == 0 {
                        let f = self.map_new_val(
                            env,
                            1048940,
                            3,
                            a.wrapping_add(8),
                            3,
                        );
                        break 'label0;
                    }
                }
            }
        }
        self.global0 = a.wrapping_add(48);
    }


    fn result_from_val(
        &mut self,
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        b = arg0.wrapping_add(8);
        let e: i64;
        'label0: {
            let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
            if svarg0_0_i32 == 0 {
                self.result_from_val_unchecked(env, a, b);
                let mut sv1_0_i32 = mload32!(a as usize) as i32;
                if sv1_0_i32 == 0 {
                    e = mload64!(a as usize + 8) as i64;
                    break 'label0;
                }
                unreachable!();
            }
            e = mload64!(b as usize) as i64;
        }
        self.global0 = a.wrapping_add(16);
        e
    }

    fn require_owner_auth(
        &mut self,
        env: &Env,
        arg0: i32,
    ) {
        self.require_auth_for_key(
            env,
            arg0,
            1048592,
            5,
        );
    }

    fn require_auth_for_key(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i64 = 0;
        let b: i64;
        {
            let c = val_to_i64(String::from_str(env, ""));
            a = c;
            let d = self.call_eq_one(env, a, 0 /* Void */);
            if d == 0 {
                a = Error(Storage, MissingValue);
            } else {
                let e = val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(a)).unwrap_or(val_from_i64(0)));
                a = e;
                if !(Address::try_from_val(env, &val_from_i64(a)).is_ok()) {
                    unreachable!();
                }
                address_from_i64(env, a).require_auth();
            }
        }
    }


    fn vec_pair_iter(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        b = self.global0.wrapping_sub(16);
        self.global0 = b;
        {
            c = mload32!(arg1 as usize + 8) as i32;
            let mut svarg1_12_i32 = mload32!(arg1 as usize + 12) as i32;
            if c as u32 >= svarg1_12_i32 as u32 {
                let mut svarg0_0_i64 = 0 /* Void */ as i64;
            } else {
                f = 1 /* True */;
                d = Error(Value, UnexpectedType);
                'label1: {
                    let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                    let h = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(svarg1_0_i64)).get_unchecked((c as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
                    e = h;
                    if Vec::<Val>::try_from_val(env, &val_from_i64(e)).is_ok() {
                        loop {
                            if a != 16 {
                                mstore64!(a.wrapping_add(b) as usize, 0 /* Void */ as u64);
                                a = a.wrapping_add(8);
                            }
                        }
                        let i = 0;
                        d = mload64!(b as usize) as i64;
                        if String::try_from_val(env, &val_from_i64(d)).is_ok() {
                            e = mload64!(b as usize + 8) as i64;
                            a = e as i32 & 255;
                            if (a == 12) as i32 & a != 70 {
                                f = 0 /* False */;
                                break 'label1;
                            }
                        }
                        d = Error(Value, UnexpectedType);
                    }
                }
                a = c += 1;
                if a != 0 {
                    let mut svarg0_0_i64 = f as i64;
                } else {
                    unreachable!();
                }
            }
        }
        self.global0 = b.wrapping_add(16);
    }

    fn ledger_timestamp_val(
        &mut self,
        env: &Env,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        let e: i64;
        'label0: {
            let f = env.ledger().timestamp() as i64;
            c = f;
            b = c as i32 & 255;
            if b != 64 {
                let g = (c as u64).wrapping_shr(0 as u32) as i64;
                if b == 6 {
                    e = g;
                    break 'label0;
                }
                unreachable!();
            }
            let h = val_from_i64(c).as_u64().unwrap_or(0) as i64;
            e = h;
        }
        self.func130(
            env,
            (c as u64).wrapping_shr(0 as u32) as i64,
            e,
            1000,
        );
        let mut value_lo = mload64!(a as usize + 16) as i64;
        if value_lo == 0 {
            let mut sv0_8_i64 = mload64!(a as usize + 8) as i64;
            self.global0 = a.wrapping_add(32);
            return sv0_8_i64;
        }
        a.wrapping_add(8)
    }

    fn entry_decode(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;
        let mut n: i32 = 0;
        let mut o: i32 = 0;
        let mut p: i32 = 0;
        let mut q: i32 = 0;
        let mut r: i32 = 0;
        let mut s: i32 = 0;
        let mut t: i32 = 0;
        let mut u: i32 = 0;
        let mut w: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        let mut aa: i32 = 0;
        let mut ab: i32 = 0;
        let mut ac: i32 = 0;
        let mut ad: i32 = 0;
        let mut ae: i32 = 0;
        let mut af: i32 = 0;
        let mut ag: i32 = 0;
        let mut ah: i32 = 0;
        let mut ai: i32 = 0;
        let mut aj: i32 = 0;
        let mut ak: i32 = 0;
        let mut al: i32 = 0;
        let mut am: i32 = 0;
        let mut an: i64 = 0;
        let mut ao: i64 = 0;
        let mut ap: i64 = 0;
        let mut aq: i64 = 0;
        let mut ar: i64 = 0;
        let mut at: i64 = 0;
        let mut au: i64 = 0;
        let mut av: i64 = 0;
        a = self.global0.wrapping_sub(576);
        self.global0 = a;
        let ax = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64;
        ao = ax;
        let mut sv3_240_i32 = 0 as i32;
        let mut sv3_232_i64 = arg1 as i64;
        mstore32!(a as usize + 244, (ao as u64).wrapping_shr(32 as u32) as i64 as u32);
        self.vec_next_string_to_linear(env, a.wrapping_add(480), a.wrapping_add(232));
        let ay: i32;
        {
            let az = mload8!(a as usize + 480) as i32;
            if az == 0 {
                n = 1;
                ay = 0;
            } else {
                b = a.wrapping_add(240);
                let mut sv4_0_i32 = mload32!(b as usize) as i32;
                let mut sv3_244_i32 = mload32!(a as usize + 244) as i32;
                self.span_from_range(
                    env,
                    a.wrapping_add(392),
                    sv4_0_i32,
                    sv3_244_i32,
                );
                let mut sv3_392_i32 = mload32!(a as usize + 392) as i32;
                c = sv3_392_i32 += 1;
                c = (if c != 0 { c } else { -1 });
                self.alloc_range(
                    env,
                    a.wrapping_add(72),
                    (if (c as u32 <= 4 as u32) as i32 != 0 { 4 } else { c }),
                    1,
                    32,
                );
                arg1 = mload64!(a.wrapping_add(489) as usize) as i64;
                ao = mload64!(a.wrapping_add(497) as usize) as i64;
                an = mload64!(a.wrapping_add(505) as usize) as i64;
                c = mload32!(a as usize + 72) as i32;
                n = mload32!(a as usize + 76) as i32;
                mstore64!(n.wrapping_add(24) as usize, an as u64);
                mstore64!(n.wrapping_add(16) as usize, ao as u64);
                mstore64!(n.wrapping_add(8) as usize, arg1 as u64);
                let mut sv3_456_i32 = 1 as i32;
                let mut sv3_452_i32 = n as i32;
                let mut sv3_448_i32 = c as i32;
                let mut sv4_0_i64 = mload64!(b as usize) as i64;
                mstore64!(a.wrapping_add(400) as usize, sv4_0_i64 as u64);
                let mut sv3_232_i64 = mload64!(a as usize + 232) as i64;
                let mut sv3_392_i64 = sv3_232_i64 as i64;
                b = a.wrapping_add(481);
                e = 32;
                l = 1;
                loop {
                    self.vec_next_string_to_linear(env, a.wrapping_add(480), a.wrapping_add(392));
                    let ba = mload8!(a as usize + 480) as i32;
                    if ba == 1 {
                        let mut sv3_448_i32 = mload32!(a as usize + 448) as i32;
                        if sv3_448_i32 == l {
                            let mut sv3_400_i32 = mload32!(a as usize + 400) as i32;
                            let mut sv3_404_i32 = mload32!(a as usize + 404) as i32;
                            self.span_from_range(
                                env,
                                a.wrapping_add(544),
                                sv3_400_i32,
                                sv3_404_i32,
                            );
                            let mut sv3_544_i32 = mload32!(a as usize + 544) as i32;
                            c = sv3_544_i32 += 1;
                            self.require_alloc(
                                env,
                                a.wrapping_add(448),
                                l,
                                (if c != 0 { c } else { -1 }),
                                32,
                            );
                            n = mload32!(a as usize + 452) as i32;
                        }
                        c = e.wrapping_add(n);
                        let mut sv4_0_i64 = mload64!(b as usize) as i64;
                        let mut sv5_0_i64 = sv4_0_i64 as i64;
                        let bb = mload64!(b.wrapping_add(24) as usize) as i64;
                        mstore64!(c.wrapping_add(24) as usize, bb as u64);
                        let bc = mload64!(b.wrapping_add(16) as usize) as i64;
                        mstore64!(c.wrapping_add(16) as usize, bc as u64);
                        let bd = mload64!(b.wrapping_add(8) as usize) as i64;
                        mstore64!(c.wrapping_add(8) as usize, bd as u64);
                        l += 1;
                        let mut sv3_456_i32 = l as i32;
                        e = e.wrapping_add(32);
                    }
                }
                ay = mload32!(a as usize + 448) as i32;
            }
        }
        j = ay;
        let be = self.ledger_timestamp_val(env);
        arg1 = be;
        self.alloc_range(
            env,
            a.wrapping_sub(-64),
            2,
            1,
            32,
        );
        let mut sv3_240_i32 = 0 as i32;
        i = mload32!(a as usize + 68) as i32;
        let mut sv3_236_i32 = i as i32;
        c = mload32!(a as usize + 64) as i32;
        let mut sv3_232_i32 = c as i32;
        b = 0;
        if c as u32 <= 1 as u32 {
            self.require_alloc(
                env,
                a.wrapping_add(232),
                0,
                2,
                32,
            );
            i = mload32!(a as usize + 236) as i32;
            b = mload32!(a as usize + 240) as i32;
        }
        d = b.wrapping_shl(5 as u32);
        e = 1048792;
        loop {
            c = a.wrapping_add(392);
            self.alloc_copy(
                env,
                c,
                e,
                20,
            );
            self.build_entry_from_bytes(env, a.wrapping_add(480), c);
            c = d.wrapping_add(i);
            let bf = mload64!(a.wrapping_add(504) as usize) as i64;
            mstore64!(c.wrapping_add(24) as usize, bf as u64);
            let bg = mload64!(a.wrapping_add(496) as usize) as i64;
            mstore64!(c.wrapping_add(16) as usize, bg as u64);
            let bh = mload64!(a.wrapping_add(488) as usize) as i64;
            mstore64!(c.wrapping_add(8) as usize, bh as u64);
            let mut sv3_480_i64 = mload64!(a as usize + 480) as i64;
            let mut sv5_0_i64 = sv3_480_i64 as i64;
            e = e.wrapping_add(20);
            d = d.wrapping_add(32);
            m += 1;
        }
        e = 2;
        'label3: {
            'label4: {
                'label5: {
                    'label6: {
                        'label7: {
                            'label8: {
                                m = b.wrapping_add(m);
                                if m as u32 >= 2 as u32 {
                                    if m as u32 > 255 as u32 {
                                        arg1 = 0;
                                        ao = 0 /* False */;
                                        an = 255;
                                        b = m;
                                        break 'label5;
                                    }
                                    g = mload32!(a as usize + 232) as i32;
                                    f = i.wrapping_add(m.wrapping_shl(5 as u32));
                                    e = i;
                                    {
                                        loop {
                                            c = e;
                                            if d == 0 {
                                                unreachable!();
                                            }
                                            d = d.wrapping_sub(32);
                                            e = c.wrapping_add(32);
                                            let bi = self.memeq32(env, c, 1049298);
                                        }
                                        an = mload64!(c as usize + 7) as i64;
                                        ao = an & 18446744069414584320;
                                        let bj = mload16!(c as usize) as i32;
                                        let bk = mload8!(c.wrapping_add(2) as usize) as i32;
                                        f = bk;
                                        d = bj | f.wrapping_shl(16 as u32);
                                        e = (d as u32).wrapping_shr(8 as u32) as i32;
                                        let bl = mload8!(c as usize + 31) as i32;
                                        m = bl;
                                        i = mload32!(c as usize + 27) as i32;
                                        g = mload32!(c as usize + 23) as i32;
                                        ap = mload64!(c as usize + 15) as i64;
                                        b = mload32!(c as usize + 3) as i32;
                                        arg1 = 0 /* Symbol() */;
                                        break 'label5;
                                    }
                                    if m == 2 {
                                        break 'label8;
                                    }
                                    b = i;
                                    'label11: loop {
                                        if b == f {
                                            break 'label6;
                                        }
                                        h = b.wrapping_add(32);
                                        c = i;
                                        k += 1;
                                        e = k;
                                        loop {
                                            'label13: {
                                                'label14: {
                                                    if e == 0 {
                                                        d = c;
                                                        if c == f {
                                                            break 'label14;
                                                        }
                                                        break 'label13;
                                                    }
                                                    d = c.wrapping_add(e.wrapping_shl(5 as u32));
                                                    if (f.wrapping_sub(c) as u32).wrapping_shr(5 as u32) as i32 as u32 > e as u32 {
                                                        break 'label13;
                                                    }
                                                }
                                                b = h;
                                                continue 'label11;
                                            }
                                            c = d.wrapping_add(32);
                                            e = 0;
                                            let bm = self.memeq32(env, b, d);
                                        }
                                    }
                                    break 'label7;
                                }
                                arg1 = 0;
                                an = 0 /* False */;
                                ao = 0 /* False */;
                                d = m;
                                break 'label5;
                            }
                            let bn = self.memeq32(env, i, i.wrapping_add(32));
                            if bn == 0 {
                                unreachable!();
                            }
                            b = i;
                        }
                        an = mload64!(b as usize + 7) as i64;
                        ao = an & 18446744069414584320;
                        let bo = mload16!(b as usize) as i32;
                        let bp = mload8!(b.wrapping_add(2) as usize) as i32;
                        f = bp;
                        d = bo | f.wrapping_shl(16 as u32);
                        e = (d as u32).wrapping_shr(8 as u32) as i32;
                        let bq = mload8!(b as usize + 31) as i32;
                        m = bq;
                        i = mload32!(b as usize + 27) as i32;
                        g = mload32!(b as usize + 23) as i32;
                        ap = mload64!(b as usize + 15) as i64;
                        b = mload32!(b as usize + 3) as i32;
                        arg1 = 15;
                        break 'label5;
                    }
                    d = 1;
                    if l == 0 {
                        arg1 = 16;
                        break 'label3;
                    }
                    c = 255;
                    if l as u32 > 255 as u32 {
                        arg1 = 17;
                        b = l;
                        break 'label3;
                    }
                    b = l.wrapping_shl(5 as u32);
                    k = n.wrapping_add(b);
                    c = 0;
                    'label15: {
                        loop {
                            if b == c {
                                break 'label15;
                            }
                            let br = c;
                            c = c.wrapping_add(32);
                            let bs = self.memeq32(env, br.wrapping_add(n), 1049298);
                        }
                        b = c.wrapping_add(n).wrapping_sub(32);
                        arg1 = 18;
                        break 'label4;
                    }
                    'label17: {
                        {
                            loop {
                                match (l & 255).wrapping_sub(1) {
                                    0 => break 'label17,
                                    1 | _ => break,
                                }
                            }
                            let bt = self.memeq32(env, n, n.wrapping_add(32));
                            if bt == 0 {
                                unreachable!();
                            }
                            arg1 = 19;
                            b = n;
                            break 'label4;
                        }
                        b = n;
                        'label20: loop {
                            if b == k {
                                break 'label17;
                            }
                            h = b.wrapping_add(32);
                            c = n;
                            p += 1;
                            f = p;
                            loop {
                                'label22: {
                                    'label23: {
                                        if f == 0 {
                                            e = c;
                                            if k == e {
                                                break 'label23;
                                            }
                                            break 'label22;
                                        }
                                        e = c.wrapping_add(f.wrapping_shl(5 as u32));
                                        if (k.wrapping_sub(c) as u32).wrapping_shr(5 as u32) as i32 as u32 > f as u32 {
                                            break 'label22;
                                        }
                                    }
                                    b = h;
                                    continue 'label20;
                                }
                                c = e.wrapping_add(32);
                                f = 0;
                                let bu = self.memeq32(env, b, e);
                            }
                        }
                        arg1 = 19;
                        break 'label4;
                    }
                    b = (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    e = (arg1 as u64).wrapping_shr(0 as u32) as i64 as i32;
                    f = 0;
                    ap = 180000;
                    c = 180000;
                    d = 0;
                    break 'label3;
                }
                e = d & 255 | f.wrapping_shl(16 as u32) | (e & 255).wrapping_shl(8 as u32);
                f = (ao as u64).wrapping_shr(32 as u32) as i64 as i32;
                c = an as i32;
                d = 1;
                break 'label3;
            }
            let bv = mload16!(b as usize) as i32;
            let bw = mload8!(b.wrapping_add(2) as usize) as i32;
            e = bv | bw.wrapping_shl(16 as u32);
            let bx = mload8!(b as usize + 31) as i32;
            m = bx;
            i = mload32!(b as usize + 27) as i32;
            g = mload32!(b as usize + 23) as i32;
            ap = mload64!(b as usize + 15) as i64;
            f = mload32!(b as usize + 11) as i32;
            c = mload32!(b as usize + 7) as i32;
            b = mload32!(b as usize + 3) as i32;
        }
        arg1 = arg1 & 255 | (b as u32 as i64).wrapping_shl(32 as u32) | (e as u32 as i64 & 16777215).wrapping_shl(0 as u32);
        av = c as u32 as i64 | (f as u32 as i64).wrapping_shl(32 as u32);
        'label24: {
            'label25: {
                'label26: {
                    'label27: {
                        'label28: {
                            'label29: {
                                'label30: {
                                    if d == 0 {
                                        b = a.wrapping_add(480);
                                        let by = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64;
                                        self.alloc_range_one(env, b, (by as u64).wrapping_shr(32 as u32) as i64 as i32);
                                        c = mload32!(a as usize + 484) as i32;
                                        {
                                            d = mload32!(a as usize + 488) as i32;
                                            let bz = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64;
                                            if d == (bz as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                self.copy_to_linear_memory(
                                                    env,
                                                    arg2,
                                                    c,
                                                    d,
                                                );
                                                let ca = mload32!(a.wrapping_add(488) as usize) as i32;
                                                mstore32!(a.wrapping_add(88) as usize, ca as u32);
                                                let mut sv3_480_i64 = mload64!(a as usize + 480) as i64;
                                                h = a.wrapping_add(80);
                                                self.span_take(
                                                    env,
                                                    b,
                                                    h,
                                                    9,
                                                );
                                                e = mload32!(a as usize + 492) as i32;
                                                o = mload32!(a as usize + 488) as i32;
                                                let cb = mload8!(a as usize + 480) as i32;
                                                d = cb;
                                                if d != 27 {
                                                    break 'label30;
                                                }
                                                c = mload32!(a as usize + 484) as i32;
                                                d = 5;
                                                f = 0;
                                                if e != 9 {
                                                    break 'label26;
                                                }
                                                let cc = self.memcmp(
                                                    env,
                                                    o,
                                                    1049394,
                                                    9,
                                                );
                                                if cc != 0 {
                                                    unreachable!();
                                                }
                                                self.func66(
                                                    env,
                                                    b,
                                                    h,
                                                    3,
                                                );
                                                'label32: {
                                                    let cd = mload8!(a as usize + 480) as i32;
                                                    d = cd;
                                                    if d == 27 {
                                                        arg2 = mload64!(a as usize + 488) as i64;
                                                        if arg2 as u64 <= 4294967295 as u64 {
                                                            self.span_take(
                                                                env,
                                                                b,
                                                                h,
                                                                arg2 as i32,
                                                            );
                                                            let ce = mload8!(a as usize + 480) as i32;
                                                            d = ce;
                                                            if d != 27 {
                                                                break 'label27;
                                                            }
                                                            self.func66(
                                                                env,
                                                                b,
                                                                h,
                                                                2,
                                                            );
                                                            let cf = mload8!(a as usize + 480) as i32;
                                                            d = cf;
                                                            if d != 27 {
                                                                break 'label27;
                                                            }
                                                            arg2 = mload64!(a as usize + 488) as i64;
                                                            if arg2 as u64 <= 4294967295 as u64 {
                                                                break 'label32;
                                                            }
                                                        }
                                                        d = 23;
                                                        break 'label25;
                                                    }
                                                    break 'label27;
                                                }
                                                z = arg2 as i32;
                                                self.alloc_range(
                                                    env,
                                                    a.wrapping_add(56),
                                                    z,
                                                    8,
                                                    56,
                                                );
                                                r = a.wrapping_add(513);
                                                s = a.wrapping_add(481);
                                                w = a.wrapping_add(520);
                                                t = a.wrapping_add(260);
                                                x = a.wrapping_add(535);
                                                ab = a.wrapping_add(512);
                                                ac = a.wrapping_add(232) | 1;
                                                ad = a.wrapping_add(492);
                                                ae = a.wrapping_add(245);
                                                af = ae.wrapping_add(7);
                                                u = a.wrapping_add(502);
                                                ag = a.wrapping_add(490);
                                                ah = a.wrapping_add(510);
                                                d = 0;
                                                'label33: loop {
                                                    'label34: {
                                                        let cg: i32;
                                                        'label35: {
                                                            'label36: {
                                                                'label37: {
                                                                    'label38: {
                                                                        'label40: {
                                                                            'label41: {
                                                                                {
                                                                                    if y != z {
                                                                                        b = a.wrapping_add(480);
                                                                                        h = a.wrapping_add(80);
                                                                                        self.span_take(
                                                                                            env,
                                                                                            b,
                                                                                            h,
                                                                                            65,
                                                                                        );
                                                                                        'label43: {
                                                                                            let ch = mload8!(a as usize + 480) as i32;
                                                                                            d = ch;
                                                                                            if d == 27 {
                                                                                                ao = mload64!(a as usize + 488) as i64;
                                                                                                let mut sv3_84_i32 = mload32!(a as usize + 84) as i32;
                                                                                                let mut sv3_88_i32 = mload32!(a as usize + 88) as i32;
                                                                                                self.alloc_copy(
                                                                                                    env,
                                                                                                    a.wrapping_add(212),
                                                                                                    sv3_84_i32,
                                                                                                    sv3_88_i32,
                                                                                                );
                                                                                                self.func66(
                                                                                                    env,
                                                                                                    b,
                                                                                                    h,
                                                                                                    3,
                                                                                                );
                                                                                                let ci = mload8!(a as usize + 480) as i32;
                                                                                                d = ci;
                                                                                                if d != 27 {
                                                                                                    break 'label29;
                                                                                                }
                                                                                                at = mload64!(a as usize + 488) as i64;
                                                                                                self.func66(
                                                                                                    env,
                                                                                                    b,
                                                                                                    h,
                                                                                                    4,
                                                                                                );
                                                                                                let cj = mload8!(a as usize + 480) as i32;
                                                                                                d = cj;
                                                                                                if d != 27 {
                                                                                                    break 'label29;
                                                                                                }
                                                                                                aq = mload64!(a as usize + 488) as i64;
                                                                                                self.func66(
                                                                                                    env,
                                                                                                    b,
                                                                                                    h,
                                                                                                    6,
                                                                                                );
                                                                                                let ck = mload8!(a as usize + 480) as i32;
                                                                                                d = ck;
                                                                                                if d == 27 {
                                                                                                    break 'label43;
                                                                                                }
                                                                                                break 'label29;
                                                                                            }
                                                                                            break 'label29;
                                                                                        }
                                                                                        an = aq.wrapping_add(32);
                                                                                        if (an as u64) < aq as u64 {
                                                                                            unreachable!();
                                                                                        }
                                                                                        au = mload64!(a as usize + 488) as i64;
                                                                                        self.func130(
                                                                                            env,
                                                                                            a.wrapping_add(40),
                                                                                            at,
                                                                                            an,
                                                                                        );
                                                                                        let mut sv3_48_i64 = mload64!(a as usize + 48) as i64;
                                                                                        if sv3_48_i64 != 0 /* False */ {
                                                                                            unreachable!();
                                                                                        }
                                                                                        ar = mload64!(a as usize + 40) as i64;
                                                                                        an = ar.wrapping_add(0);
                                                                                        if (an as u64) < ar as u64 {
                                                                                            unreachable!();
                                                                                        }
                                                                                        ar = an.wrapping_add(0);
                                                                                        if an as u64 > ar as u64 {
                                                                                            unreachable!();
                                                                                        }
                                                                                        an = ar.wrapping_add(3 /* Error(Contract, #0) */);
                                                                                        if ar as u64 > an as u64 {
                                                                                            unreachable!();
                                                                                        }
                                                                                        if an as u64 <= 4294967295 as u64 {
                                                                                            b = a.wrapping_add(480);
                                                                                            self.span_take(
                                                                                                env,
                                                                                                b,
                                                                                                a.wrapping_add(212),
                                                                                                an as i32,
                                                                                            );
                                                                                            let cl = mload8!(a as usize + 480) as i32;
                                                                                            d = cl;
                                                                                            if d != 27 {
                                                                                                break 'label29;
                                                                                            }
                                                                                            an = (ao as u64).wrapping_shr(32 as u32) as i64;
                                                                                            if an == 65 /* I64(obj#0) */ {
                                                                                                an = mload64!(a as usize + 488) as i64;
                                                                                                h = s.wrapping_add(24);
                                                                                                d = ao as i32;
                                                                                                let cm = mload64!(d.wrapping_add(24) as usize) as i64;
                                                                                                let mut sv10_0_i64 = cm as i64;
                                                                                                o = s.wrapping_add(16);
                                                                                                let cn = mload64!(d.wrapping_add(16) as usize) as i64;
                                                                                                let mut sv17_0_i64 = cn as i64;
                                                                                                q = s.wrapping_add(8);
                                                                                                let co = mload64!(d.wrapping_add(8) as usize) as i64;
                                                                                                let mut sv19_0_i64 = co as i64;
                                                                                                let mut sv6_0_i64 = mload64!(d as usize) as i64;
                                                                                                let mut sv21_0_i64 = sv6_0_i64 as i64;
                                                                                                mstore8!(a as usize + 480, 0 as u8);
                                                                                                e = a.wrapping_add(448);
                                                                                                self.entry_copy_if_ok(env, e, b);
                                                                                                let cp = mload64!(d.wrapping_add(56) as usize) as i64;
                                                                                                let mut sv10_0_i64 = cp as i64;
                                                                                                let cq = mload64!(d.wrapping_add(48) as usize) as i64;
                                                                                                let mut sv17_0_i64 = cq as i64;
                                                                                                let cr = mload64!(d.wrapping_add(40) as usize) as i64;
                                                                                                let mut sv19_0_i64 = cr as i64;
                                                                                                let mut sv6_32_i64 = mload64!(d as usize + 32) as i64;
                                                                                                let mut sv21_0_i64 = sv6_32_i64 as i64;
                                                                                                mstore8!(a as usize + 480, 0 as u8);
                                                                                                h = a.wrapping_add(544);
                                                                                                self.entry_copy_if_ok(env, h, b);
                                                                                                let cs = self.memeq32(env, e, 1049298);
                                                                                                b = cs;
                                                                                                let ct = self.memeq32(env, h, 1049298);
                                                                                                o = ct;
                                                                                                {
                                                                                                    let cu = self.memcmp_sign32(env, e, 1049330);
                                                                                                    let cv = self.memcmp_sign32(env, h, 1049362);
                                                                                                    if (((cu & 255) as u32) < 2 as u32) as i32 | b | o | !(cv & 255 == 1) {
                                                                                                        let mut sv3_376_i32 = -2147483644 as i32;
                                                                                                    } else {
                                                                                                        self.func69(
                                                                                                            env,
                                                                                                            a.wrapping_add(376),
                                                                                                            d,
                                                                                                            65,
                                                                                                        );
                                                                                                        e = mload32!(a as usize + 376) as i32;
                                                                                                        if e != -2147483644 {
                                                                                                            break 'label40;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                {
                                                                                                    let cw = mload8!(d as usize + 64) as i32;
                                                                                                    e = cw;
                                                                                                    if (e as u32) < 2 as u32 {
                                                                                                        b = e;
                                                                                                    } else {
                                                                                                        b = e.wrapping_sub(27);
                                                                                                        if b as u32 > 1 as u32 {
                                                                                                            break 'label41;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                let cx = val_to_i64(Bytes::new(env).into_val(env));
                                                                                                let cy = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(cx))).into());
                                                                                                self.entry_from_bytes_val(env, a.wrapping_add(392), cy);
                                                                                                h = a.wrapping_add(480);
                                                                                                self.memmove(
                                                                                                    env,
                                                                                                    h,
                                                                                                    d,
                                                                                                    64,
                                                                                                );
                                                                                                let da = val_to_i64(Bytes::new(env).into_val(env));
                                                                                                ao = da;
                                                                                                let mut sv3_424_i64 = mload64!(a as usize + 424) as i64;
                                                                                                let db = val_to_i64(env.crypto().secp256k1_recover(&Hash::<32>::from_val(env, &val_from_i64(sv3_424_i64)), &BytesN::<64>::from_val(env, &val_from_i64(ao)), (b as u32 as i64 & 255).wrapping_shl(32 as u32) | 0 as u32).into());
                                                                                                an = db;
                                                                                                d = a.wrapping_add(544);
                                                                                                self.alloc_range_one(env, d, 65);
                                                                                                e = mload32!(a as usize + 548) as i32;
                                                                                                b = mload32!(a as usize + 552) as i32;
                                                                                                let dc = Bytes::from_val(env, &val_from_i64(an)).len() as i64;
                                                                                                if b != (dc as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                                                                    unreachable!();
                                                                                                }
                                                                                                self.copy_to_linear_memory(
                                                                                                    env,
                                                                                                    an,
                                                                                                    e,
                                                                                                    b,
                                                                                                );
                                                                                                if b == 0 {
                                                                                                    unreachable!();
                                                                                                }
                                                                                                o = 1;
                                                                                                let dd = val_to_i64(Bytes::new(env).into_val(env));
                                                                                                let de = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(dd))).into());
                                                                                                self.entry_from_bytes_val(env, h, de);
                                                                                                self.alloc_copy(
                                                                                                    env,
                                                                                                    d,
                                                                                                    ad,
                                                                                                    20,
                                                                                                );
                                                                                                self.build_entry_from_bytes(env, ac, d);
                                                                                                let df = mload8!(t.wrapping_add(4) as usize) as i32;
                                                                                                mstore8!(a.wrapping_add(228) as usize, df as u8);
                                                                                                let mut sv32_0_i64 = mload64!(ae as usize) as i64;
                                                                                                let mut sv3_304_i64 = sv32_0_i64 as i64;
                                                                                                let mut sv33_0_i64 = mload64!(af as usize) as i64;
                                                                                                let mut sv3_311_i64 = sv33_0_i64 as i64;
                                                                                                let mut sv22_0_i32 = mload32!(t as usize) as i32;
                                                                                                let mut sv3_224_i32 = sv22_0_i32 as i32;
                                                                                                let dg = mload16!(a as usize + 233) as i32;
                                                                                                let dh = mload8!(a as usize + 235) as i32;
                                                                                                h = dg | dh.wrapping_shl(16 as u32);
                                                                                                ao = mload64!(a as usize + 236) as i64;
                                                                                                let di = mload8!(a as usize + 244) as i32;
                                                                                                cg = di;
                                                                                                break 'label35;
                                                                                            }
                                                                                            mstore32!(a as usize + 240, an as u32);
                                                                                            let mut sv3_236_i32 = -2147483645 as i32;
                                                                                            break 'label36;
                                                                                        }
                                                                                        d = 23;
                                                                                        break 'label34;
                                                                                    }
                                                                                    arg2 = mload64!(a as usize + 120) as i64;
                                                                                    h = mload32!(a as usize + 116) as i32;
                                                                                    c = mload32!(a as usize + 88) as i32;
                                                                                    if c != 0 {
                                                                                        d = 6;
                                                                                        break 'label25;
                                                                                    }
                                                                                    if (arg2 as u64) < 4294967296 as u64 {
                                                                                        d = 2;
                                                                                        break 'label38;
                                                                                    }
                                                                                    d = arg2 as i32;
                                                                                    an = mload64!(d as usize) as i64;
                                                                                    ao = an.wrapping_add(av);
                                                                                    if arg1 as u64 > ((if an as u64 > ao as u64 { 18446744073709551615 } else { ao })) as u64 {
                                                                                        d = 9;
                                                                                        break 'label38;
                                                                                    }
                                                                                    ao = arg1.wrapping_add(ap);
                                                                                    if (((if arg1 as u64 > ao as u64 { 18446744073709551615 } else { ao })) as u64) < an as u64 {
                                                                                        d = 10;
                                                                                        break 'label38;
                                                                                    }
                                                                                    f = ((arg2 as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_mul(56);
                                                                                    r = d.wrapping_add(f);
                                                                                    c = 56;
                                                                                    'label46: {
                                                                                        loop {
                                                                                            if c == f {
                                                                                                break 'label46;
                                                                                            }
                                                                                            b = c.wrapping_add(d);
                                                                                            c = c.wrapping_add(56);
                                                                                            arg1 = mload64!(b as usize) as i64;
                                                                                        }
                                                                                        d = 20;
                                                                                        break 'label38;
                                                                                    }
                                                                                    ao = m as u32 as i64;
                                                                                    arg1 = ao.wrapping_mul(l as u32 as i64);
                                                                                    if (arg1 as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                                                                        unreachable!();
                                                                                    }
                                                                                    b = arg1 as i32;
                                                                                    self.alloc_range_fill(
                                                                                        env,
                                                                                        a.wrapping_add(16),
                                                                                        b,
                                                                                        33,
                                                                                    );
                                                                                    let mut sv3_240_i32 = 0 as i32;
                                                                                    c = mload32!(a as usize + 20) as i32;
                                                                                    let mut sv3_236_i32 = c as i32;
                                                                                    g = mload32!(a as usize + 16) as i32;
                                                                                    let mut sv3_232_i32 = g as i32;
                                                                                    p = 0;
                                                                                    if b as u32 > g as u32 {
                                                                                        self.require_alloc(
                                                                                            env,
                                                                                            a.wrapping_add(232),
                                                                                            0,
                                                                                            b,
                                                                                            33,
                                                                                        );
                                                                                        p = mload32!(a as usize + 240) as i32;
                                                                                        let mut sv3_236_i32 = mload32!(a as usize + 236) as i32;
                                                                                        c = sv3_236_i32;
                                                                                    }
                                                                                    c = c.wrapping_add(p.wrapping_mul(33));
                                                                                    g = (if (b as u32 <= 1 as u32) as i32 != 0 { 1 } else { b });
                                                                                    e = g -= 1;
                                                                                    k = a.wrapping_add(488);
                                                                                    o = a.wrapping_add(496);
                                                                                    j = a.wrapping_add(504);
                                                                                    'label48: {
                                                                                        loop {
                                                                                            if e != 0 {
                                                                                                mstore8!(c as usize, 0 as u8);
                                                                                                let mut sv3_480_i64 = mload64!(a as usize + 480) as i64;
                                                                                                let mut sv5_1_i64 = sv3_480_i64 as i64;
                                                                                                let mut sv13_0_i64 = mload64!(k as usize) as i64;
                                                                                                mstore64!(c.wrapping_add(9) as usize, sv13_0_i64 as u64);
                                                                                                let mut sv17_0_i64 = mload64!(o as usize) as i64;
                                                                                                mstore64!(c.wrapping_add(17) as usize, sv17_0_i64 as u64);
                                                                                                let mut sv12_0_i64 = mload64!(j as usize) as i64;
                                                                                                mstore64!(c.wrapping_add(25) as usize, sv12_0_i64 as u64);
                                                                                                e -= 1;
                                                                                                c = c.wrapping_add(33);
                                                                                            } else {
                                                                                                s = g.wrapping_add(p);
                                                                                                if b == 0 {
                                                                                                    s -= 1;
                                                                                                    break 'label48;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        mstore8!(c as usize, 0 as u8);
                                                                                        let mut sv3_392_i64 = mload64!(a as usize + 392) as i64;
                                                                                        sv5_1_i64 = sv3_392_i64 as i64;
                                                                                        let dj = mload64!(a.wrapping_add(400) as usize) as i64;
                                                                                        mstore64!(c.wrapping_add(9) as usize, dj as u64);
                                                                                        let dk = mload64!(a.wrapping_add(408) as usize) as i64;
                                                                                        mstore64!(c.wrapping_add(17) as usize, dk as u64);
                                                                                        let dl = mload64!(a.wrapping_add(416) as usize) as i64;
                                                                                        mstore64!(c.wrapping_add(25) as usize, dl as u64);
                                                                                    }
                                                                                    x = sv3_236_i32;
                                                                                    let mut sv3_384_i32 = h as i32;
                                                                                    let mut sv3_376_i32 = d as i32;
                                                                                    o = l.wrapping_shl(5 as u32);
                                                                                    q = m.wrapping_shl(5 as u32);
                                                                                    f = a.wrapping_add(512);
                                                                                    p = a.wrapping_add(413);
                                                                                    t = a.wrapping_add(404);
                                                                                    'label51: loop {
                                                                                        'label52: {
                                                                                            if d != r {
                                                                                                k = d.wrapping_add(56);
                                                                                                j = mload32!(d as usize + 8) as i32;
                                                                                                if j != -2147483648 {
                                                                                                    break 'label52;
                                                                                                }
                                                                                            }
                                                                                            sv3_384_i32 = 0 as i32;
                                                                                            let mut sv3_376_i64 = 4294967296 as i64;
                                                                                            q = a.wrapping_add(233);
                                                                                            r = a.wrapping_add(393);
                                                                                            f = a.wrapping_add(481);
                                                                                            t = a.wrapping_add(512);
                                                                                            o = 1;
                                                                                            k = 0;
                                                                                            c = 0;
                                                                                            p = 0;
                                                                                            'label53: loop {
                                                                                                'label54: {
                                                                                                    'label55: {
                                                                                                        'label56: {
                                                                                                            'label57: {
                                                                                                                {
                                                                                                                    let dm: i32;
                                                                                                                    if l as u32 <= p as u32 {
                                                                                                                        e = a.wrapping_add(192);
                                                                                                                        g = c;
                                                                                                                        dm = 0;
                                                                                                                    } else {
                                                                                                                        arg1 = (p as u32 as i64).wrapping_mul(ao);
                                                                                                                        if (arg1 as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                                                                                                            unreachable!();
                                                                                                                        }
                                                                                                                        b = arg1 as i32;
                                                                                                                        d = b.wrapping_add(m);
                                                                                                                        if (d as u32) < b as u32 {
                                                                                                                            unreachable!();
                                                                                                                        }
                                                                                                                        if d as u32 > s as u32 {
                                                                                                                            unreachable!();
                                                                                                                        }
                                                                                                                        g = c += 1;
                                                                                                                        if g == 0 {
                                                                                                                            unreachable!();
                                                                                                                        }
                                                                                                                        p += 1;
                                                                                                                        let mut sv3_192_i32 = x.wrapping_add(b.wrapping_mul(33)) as i32;
                                                                                                                        e = a.wrapping_add(152);
                                                                                                                        w = c;
                                                                                                                        dm = x.wrapping_add(d.wrapping_mul(33));
                                                                                                                    }
                                                                                                                    b = dm;
                                                                                                                    let mut sv7_0_i32 = b as i32;
                                                                                                                    c = sv3_192_i32;
                                                                                                                    if c != 0 {
                                                                                                                        let mut sv3_232_i32 = 0 as i32;
                                                                                                                        self.memcpy_like(env, a.wrapping_add(480), a.wrapping_add(232));
                                                                                                                        let dn = mload8!(a as usize + 480) as i32;
                                                                                                                        if dn == 0 {
                                                                                                                            unreachable!();
                                                                                                                        }
                                                                                                                        e = 32;
                                                                                                                        self.alloc_range_fill(
                                                                                                                            env,
                                                                                                                            a.wrapping_add(8),
                                                                                                                            4,
                                                                                                                            32,
                                                                                                                        );
                                                                                                                        h = f.wrapping_add(8);
                                                                                                                        arg1 = mload64!(h as usize) as i64;
                                                                                                                        j = f.wrapping_add(16);
                                                                                                                        arg2 = mload64!(j as usize) as i64;
                                                                                                                        u = f.wrapping_add(24);
                                                                                                                        ap = mload64!(u as usize) as i64;
                                                                                                                        c = mload32!(a as usize + 8) as i32;
                                                                                                                        b = mload32!(a as usize + 12) as i32;
                                                                                                                        let mut sv8_0_i64 = mload64!(f as usize) as i64;
                                                                                                                        let mut sv4_0_i64 = sv8_0_i64 as i64;
                                                                                                                        mstore64!(b.wrapping_add(24) as usize, ap as u64);
                                                                                                                        mstore64!(b.wrapping_add(16) as usize, arg2 as u64);
                                                                                                                        mstore64!(b.wrapping_add(8) as usize, arg1 as u64);
                                                                                                                        d = 1;
                                                                                                                        let mut sv3_552_i32 = 1 as i32;
                                                                                                                        let mut sv3_544_i32 = c as i32;
                                                                                                                        let do = mload64!(a.wrapping_add(248) as usize) as i64;
                                                                                                                        mstore64!(a.wrapping_add(408) as usize, do as u64);
                                                                                                                        let dp = mload64!(a.wrapping_add(240) as usize) as i64;
                                                                                                                        mstore64!(a.wrapping_add(400) as usize, dp as u64);
                                                                                                                        let mut sv3_232_i64 = mload64!(a as usize + 232) as i64;
                                                                                                                        let mut sv3_392_i64 = sv3_232_i64 as i64;
                                                                                                                        i = 0;
                                                                                                                        loop {
                                                                                                                            self.memcpy_like(env, a.wrapping_add(480), a.wrapping_add(392));
                                                                                                                            let dq = mload8!(a as usize + 480) as i32;
                                                                                                                            if dq == 1 {
                                                                                                                                let mut sv3_544_i32 = mload32!(a as usize + 544) as i32;
                                                                                                                                if sv3_544_i32 == d {
                                                                                                                                    self.require_alloc(
                                                                                                                                        env,
                                                                                                                                        a.wrapping_add(544),
                                                                                                                                        d,
                                                                                                                                        1,
                                                                                                                                        32,
                                                                                                                                    );
                                                                                                                                    b = mload32!(a as usize + 548) as i32;
                                                                                                                                }
                                                                                                                                c = b.wrapping_add(e);
                                                                                                                                let mut sv8_0_i64 = mload64!(f as usize) as i64;
                                                                                                                                let mut sv5_0_i64 = sv8_0_i64 as i64;
                                                                                                                                let mut sv23_0_i64 = mload64!(u as usize) as i64;
                                                                                                                                mstore64!(c.wrapping_add(24) as usize, sv23_0_i64 as u64);
                                                                                                                                let mut sv12_0_i64 = mload64!(j as usize) as i64;
                                                                                                                                mstore64!(c.wrapping_add(16) as usize, sv12_0_i64 as u64);
                                                                                                                                let mut sv10_0_i64 = mload64!(h as usize) as i64;
                                                                                                                                mstore64!(c.wrapping_add(8) as usize, sv10_0_i64 as u64);
                                                                                                                                d += 1;
                                                                                                                                let mut sv3_552_i32 = d as i32;
                                                                                                                                i = i.wrapping_add(32);
                                                                                                                                e = e.wrapping_add(32);
                                                                                                                            }
                                                                                                                        }
                                                                                                                        if (d as u32) < 2 as u32 {
                                                                                                                            break 'label55;
                                                                                                                        }
                                                                                                                        'label62: {
                                                                                                                            {
                                                                                                                                {
                                                                                                                                    match d.wrapping_sub(1) {
                                                                                                                                        0 | 1 | 2 | _ => break,
                                                                                                                                    }
                                                                                                                                    if d as u32 >= 21 as u32 {
                                                                                                                                        h = a.wrapping_add(480);
                                                                                                                                        e = self.global0.wrapping_sub(16);
                                                                                                                                        self.global0 = e;
                                                                                                                                        'label67: {
                                                                                                                                            'label68: {
                                                                                                                                                'label69: {
                                                                                                                                                    let ds = self.memcmp_sign32(env, b.wrapping_add(32), b);
                                                                                                                                                    if ds & 255 != 255 {
                                                                                                                                                        c = b.wrapping_sub(-64);
                                                                                                                                                        i = 2;
                                                                                                                                                        loop {
                                                                                                                                                            if d == i {
                                                                                                                                                                break 'label67;
                                                                                                                                                            }
                                                                                                                                                            let dt = self.memcmp_sign32(env, c, c.wrapping_sub(32));
                                                                                                                                                            if dt & 255 == 255 {
                                                                                                                                                                break 'label69;
                                                                                                                                                            }
                                                                                                                                                            c = c.wrapping_add(32);
                                                                                                                                                            i += 1;
                                                                                                                                                        }
                                                                                                                                                        unreachable!();
                                                                                                                                                    }
                                                                                                                                                    c = b.wrapping_sub(-64);
                                                                                                                                                    i = 2;
                                                                                                                                                    loop {
                                                                                                                                                        if d == i {
                                                                                                                                                            break 'label68;
                                                                                                                                                        }
                                                                                                                                                        let du = self.memcmp_sign32(env, c, c.wrapping_sub(32));
                                                                                                                                                        if du & 255 != 255 {
                                                                                                                                                            break 'label69;
                                                                                                                                                        }
                                                                                                                                                        c = c.wrapping_add(32);
                                                                                                                                                        i += 1;
                                                                                                                                                    }
                                                                                                                                                    unreachable!();
                                                                                                                                                }
                                                                                                                                                self.func114(
                                                                                                                                                    env,
                                                                                                                                                    b,
                                                                                                                                                    d,
                                                                                                                                                    0,
                                                                                                                                                    ((d | 1).leading_zeros() as i32).wrapping_shl(1 as u32) ^ 62,
                                                                                                                                                    h,
                                                                                                                                                );
                                                                                                                                                break 'label67;
                                                                                                                                            }
                                                                                                                                            h = (d as u32).wrapping_shr(1 as u32) as i32;
                                                                                                                                            self.func115(
                                                                                                                                                env,
                                                                                                                                                e.wrapping_add(8),
                                                                                                                                                h,
                                                                                                                                                b,
                                                                                                                                                h,
                                                                                                                                            );
                                                                                                                                            u = mload32!(e as usize + 12) as i32;
                                                                                                                                            i = mload32!(e as usize + 8) as i32;
                                                                                                                                            c = h.wrapping_shl(5 as u32);
                                                                                                                                            self.func115(
                                                                                                                                                env,
                                                                                                                                                e,
                                                                                                                                                h,
                                                                                                                                                b.wrapping_add(d.wrapping_shl(5 as u32)).wrapping_sub(c),
                                                                                                                                                h,
                                                                                                                                            );
                                                                                                                                            let mut sv7_0_i32 = mload32!(e as usize) as i32;
                                                                                                                                            j = c.wrapping_add(sv7_0_i32).wrapping_sub(32);
                                                                                                                                            c = 0;
                                                                                                                                            y = mload32!(e as usize + 4) as i32;
                                                                                                                                            loop {
                                                                                                                                                z = c.wrapping_add(h);
                                                                                                                                                if z == 0 {
                                                                                                                                                    unreachable!();
                                                                                                                                                }
                                                                                                                                                if c.wrapping_add(u) == 0 {
                                                                                                                                                    unreachable!();
                                                                                                                                                }
                                                                                                                                                if y as u32 > z.wrapping_sub(1) as u32 {
                                                                                                                                                    self.memcpy_like_6(env, i, j);
                                                                                                                                                    i = i.wrapping_add(32);
                                                                                                                                                    j = j.wrapping_sub(32);
                                                                                                                                                    c -= 1;
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            unreachable!();
                                                                                                                                        }
                                                                                                                                        self.global0 = e.wrapping_add(16);
                                                                                                                                        break 'label57;
                                                                                                                                    }
                                                                                                                                    c = b.wrapping_add(32);
                                                                                                                                    loop {
                                                                                                                                        if i == 0 {
                                                                                                                                            unreachable!();
                                                                                                                                        }
                                                                                                                                        self.func74(env, b, c);
                                                                                                                                        i = i.wrapping_sub(32);
                                                                                                                                        c = c.wrapping_add(32);
                                                                                                                                    }
                                                                                                                                    unreachable!();
                                                                                                                                    let dv = self.ptr_index32(
                                                                                                                                        env,
                                                                                                                                        b,
                                                                                                                                        1,
                                                                                                                                        0,
                                                                                                                                    );
                                                                                                                                    b = dv;
                                                                                                                                    let dw = mload64!(b.wrapping_add(24) as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(504) as usize, dw as u64);
                                                                                                                                    let dx = mload64!(b.wrapping_add(16) as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(496) as usize, dx as u64);
                                                                                                                                    let dy = mload64!(b.wrapping_add(8) as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(488) as usize, dy as u64);
                                                                                                                                    let mut sv4_0_i64 = mload64!(b as usize) as i64;
                                                                                                                                    let mut sv3_480_i64 = sv4_0_i64 as i64;
                                                                                                                                    break 'label56;
                                                                                                                                }
                                                                                                                                let dz = self.ptr_index32(
                                                                                                                                    env,
                                                                                                                                    b,
                                                                                                                                    2,
                                                                                                                                    0,
                                                                                                                                );
                                                                                                                                let ea = self.ptr_index32(
                                                                                                                                    env,
                                                                                                                                    b,
                                                                                                                                    2,
                                                                                                                                    1,
                                                                                                                                );
                                                                                                                                self.func76(
                                                                                                                                    env,
                                                                                                                                    a.wrapping_add(544),
                                                                                                                                    dz,
                                                                                                                                    ea,
                                                                                                                                );
                                                                                                                                break 'label62;
                                                                                                                            }
                                                                                                                            let eb = self.ptr_index32(
                                                                                                                                env,
                                                                                                                                b,
                                                                                                                                3,
                                                                                                                                0,
                                                                                                                            );
                                                                                                                            let ec = self.ptr_index32(
                                                                                                                                env,
                                                                                                                                b,
                                                                                                                                3,
                                                                                                                                1,
                                                                                                                            );
                                                                                                                            let ed = self.ptr_index32(
                                                                                                                                env,
                                                                                                                                b,
                                                                                                                                3,
                                                                                                                                2,
                                                                                                                            );
                                                                                                                            self.entry_match_copy(
                                                                                                                                env,
                                                                                                                                a.wrapping_add(232),
                                                                                                                                eb,
                                                                                                                                ec,
                                                                                                                                ed,
                                                                                                                            );
                                                                                                                            let ee = mload8!(a as usize + 232) as i32;
                                                                                                                            if ee == 0 {
                                                                                                                                let ef = self.ptr_index32(
                                                                                                                                    env,
                                                                                                                                    b,
                                                                                                                                    3,
                                                                                                                                    1,
                                                                                                                                );
                                                                                                                                let eg = self.ptr_index32(
                                                                                                                                    env,
                                                                                                                                    b,
                                                                                                                                    3,
                                                                                                                                    0,
                                                                                                                                );
                                                                                                                                let eh = self.ptr_index32(
                                                                                                                                    env,
                                                                                                                                    b,
                                                                                                                                    3,
                                                                                                                                    2,
                                                                                                                                );
                                                                                                                                self.entry_match_copy(
                                                                                                                                    env,
                                                                                                                                    a.wrapping_add(392),
                                                                                                                                    ef,
                                                                                                                                    eg,
                                                                                                                                    eh,
                                                                                                                                );
                                                                                                                                let ei = mload8!(a as usize + 392) as i32;
                                                                                                                                if ei == 0 {
                                                                                                                                    let ej = self.ptr_index32(
                                                                                                                                        env,
                                                                                                                                        b,
                                                                                                                                        3,
                                                                                                                                        1,
                                                                                                                                    );
                                                                                                                                    let ek = self.ptr_index32(
                                                                                                                                        env,
                                                                                                                                        b,
                                                                                                                                        3,
                                                                                                                                        2,
                                                                                                                                    );
                                                                                                                                    let el = self.ptr_index32(
                                                                                                                                        env,
                                                                                                                                        b,
                                                                                                                                        3,
                                                                                                                                        0,
                                                                                                                                    );
                                                                                                                                    self.entry_match_copy(
                                                                                                                                        env,
                                                                                                                                        a.wrapping_add(480),
                                                                                                                                        ej,
                                                                                                                                        ek,
                                                                                                                                        el,
                                                                                                                                    );
                                                                                                                                    let em = mload8!(a as usize + 480) as i32;
                                                                                                                                    if em == 0 {
                                                                                                                                        unreachable!();
                                                                                                                                    }
                                                                                                                                    let mut sv23_0_i64 = mload64!(u as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(568) as usize, sv23_0_i64 as u64);
                                                                                                                                    let mut sv12_0_i64 = mload64!(j as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(560) as usize, sv12_0_i64 as u64);
                                                                                                                                    let mut sv10_0_i64 = mload64!(h as usize) as i64;
                                                                                                                                    mstore64!(a.wrapping_add(552) as usize, sv10_0_i64 as u64);
                                                                                                                                    let mut sv8_0_i64 = mload64!(f as usize) as i64;
                                                                                                                                    let mut sv3_544_i64 = sv8_0_i64 as i64;
                                                                                                                                    break 'label62;
                                                                                                                                }
                                                                                                                                let en = mload64!(r.wrapping_add(24) as usize) as i64;
                                                                                                                                mstore64!(a.wrapping_add(568) as usize, en as u64);
                                                                                                                                let eo = mload64!(r.wrapping_add(16) as usize) as i64;
                                                                                                                                mstore64!(a.wrapping_add(560) as usize, eo as u64);
                                                                                                                                let ep = mload64!(r.wrapping_add(8) as usize) as i64;
                                                                                                                                mstore64!(a.wrapping_add(552) as usize, ep as u64);
                                                                                                                                let mut sv20_0_i64 = mload64!(r as usize) as i64;
                                                                                                                                sv3_544_i64 = sv20_0_i64 as i64;
                                                                                                                                break 'label62;
                                                                                                                            }
                                                                                                                            let eq = mload64!(q.wrapping_add(24) as usize) as i64;
                                                                                                                            mstore64!(a.wrapping_add(568) as usize, eq as u64);
                                                                                                                            let er = mload64!(q.wrapping_add(16) as usize) as i64;
                                                                                                                            mstore64!(a.wrapping_add(560) as usize, er as u64);
                                                                                                                            let es = mload64!(q.wrapping_add(8) as usize) as i64;
                                                                                                                            mstore64!(a.wrapping_add(552) as usize, es as u64);
                                                                                                                            let mut sv19_0_i64 = mload64!(q as usize) as i64;
                                                                                                                            sv3_544_i64 = sv19_0_i64 as i64;
                                                                                                                        }
                                                                                                                        let et = mload64!(a.wrapping_add(568) as usize) as i64;
                                                                                                                        mstore64!(a.wrapping_add(504) as usize, et as u64);
                                                                                                                        let eu = mload64!(a.wrapping_add(560) as usize) as i64;
                                                                                                                        mstore64!(a.wrapping_add(496) as usize, eu as u64);
                                                                                                                        let ev = mload64!(a.wrapping_add(552) as usize) as i64;
                                                                                                                        mstore64!(a.wrapping_add(488) as usize, ev as u64);
                                                                                                                        let mut sv3_480_i64 = sv3_544_i64 as i64;
                                                                                                                        break 'label56;
                                                                                                                    }
                                                                                                                    arg2 = mload32!(a.wrapping_add(384) as usize) as i64;
                                                                                                                    arg1 = mload64!(a as usize + 376) as i64;
                                                                                                                    d = 27;
                                                                                                                    break 'label37;
                                                                                                                }
                                                                                                            }
                                                                                                            c = (d as u32).wrapping_shr(1 as u32) as i32;
                                                                                                            if d & 1 == 0 {
                                                                                                                let ew = self.ptr_index32(
                                                                                                                    env,
                                                                                                                    b,
                                                                                                                    d,
                                                                                                                    c.wrapping_sub(1),
                                                                                                                );
                                                                                                                let ex = self.ptr_index32(
                                                                                                                    env,
                                                                                                                    b,
                                                                                                                    d,
                                                                                                                    c,
                                                                                                                );
                                                                                                                self.func76(
                                                                                                                    env,
                                                                                                                    a.wrapping_add(544),
                                                                                                                    ew,
                                                                                                                    ex,
                                                                                                                );
                                                                                                            } else {
                                                                                                                let ey = self.ptr_index32(
                                                                                                                    env,
                                                                                                                    b,
                                                                                                                    d,
                                                                                                                    c,
                                                                                                                );
                                                                                                                b = ey;
                                                                                                                let ez = mload64!(b.wrapping_add(24) as usize) as i64;
                                                                                                                mstore64!(a.wrapping_add(568) as usize, ez as u64);
                                                                                                                let fa = mload64!(b.wrapping_add(16) as usize) as i64;
                                                                                                                mstore64!(a.wrapping_add(560) as usize, fa as u64);
                                                                                                                let fb = mload64!(b.wrapping_add(8) as usize) as i64;
                                                                                                                mstore64!(a.wrapping_add(552) as usize, fb as u64);
                                                                                                                let mut sv4_0_i64 = mload64!(b as usize) as i64;
                                                                                                                let mut sv3_544_i64 = sv4_0_i64 as i64;
                                                                                                            }
                                                                                                            let fc = mload64!(a.wrapping_add(568) as usize) as i64;
                                                                                                            mstore64!(a.wrapping_add(504) as usize, fc as u64);
                                                                                                            let fd = mload64!(a.wrapping_add(560) as usize) as i64;
                                                                                                            mstore64!(a.wrapping_add(496) as usize, fd as u64);
                                                                                                            let fe = mload64!(a.wrapping_add(552) as usize) as i64;
                                                                                                            mstore64!(a.wrapping_add(488) as usize, fe as u64);
                                                                                                            let mut sv3_480_i64 = sv3_544_i64 as i64;
                                                                                                        }
                                                                                                        c = a.wrapping_add(472);
                                                                                                        b = a.wrapping_add(504);
                                                                                                        let mut sv5_0_i64 = sv4_0_i64 as i64;
                                                                                                        d = a.wrapping_add(464);
                                                                                                        i = a.wrapping_add(496);
                                                                                                        let mut sv11_0_i64 = mload64!(i as usize) as i64;
                                                                                                        let mut sv6_0_i64 = sv11_0_i64 as i64;
                                                                                                        e = a.wrapping_add(456);
                                                                                                        j = a.wrapping_add(488);
                                                                                                        let mut sv12_0_i64 = mload64!(j as usize) as i64;
                                                                                                        let mut sv7_0_i64 = sv12_0_i64 as i64;
                                                                                                        let mut sv3_448_i64 = sv3_480_i64 as i64;
                                                                                                        if l as u32 <= w as u32 {
                                                                                                            break 'label54;
                                                                                                        }
                                                                                                        b;
                                                                                                        b = n.wrapping_add(w.wrapping_shl(5 as u32));
                                                                                                        mload64!(b.wrapping_add(24) as usize) as i64;
                                                                                                        mload64!(b.wrapping_add(16) as usize) as i64;
                                                                                                        mload64!(b.wrapping_add(8) as usize) as i64;
                                                                                                        arg1 = sv4_0_i64;
                                                                                                        mstore64!(t.wrapping_add(8) as usize, sv7_0_i64 as u64);
                                                                                                        mstore64!(t.wrapping_add(16) as usize, sv6_0_i64 as u64);
                                                                                                        mstore64!(t.wrapping_add(24) as usize, sv5_0_i64 as u64);
                                                                                                        sv3_480_i64 = arg1 as i64;
                                                                                                        let mut sv3_376_i32 = mload32!(a as usize + 376) as i32;
                                                                                                        if sv3_376_i32 == k {
                                                                                                            self.alloc_realloc(env, a.wrapping_add(376));
                                                                                                            o = mload32!(a as usize + 380) as i32;
                                                                                                        }
                                                                                                        self.memmove(
                                                                                                            env,
                                                                                                            o.wrapping_add(k.wrapping_shl(6 as u32)),
                                                                                                            a.wrapping_add(480),
                                                                                                            64,
                                                                                                        );
                                                                                                        k += 1;
                                                                                                        let mut sv3_384_i32 = k as i32;
                                                                                                    }
                                                                                                    c = g;
                                                                                                    continue 'label53;
                                                                                                }
                                                                                            }
                                                                                            unreachable!();
                                                                                        }
                                                                                        arg1 = mload64!(d as usize) as i64;
                                                                                        let mut sv3_400_i32 = j as i32;
                                                                                        let mut sv3_392_i64 = arg1 as i64;
                                                                                        self.memmove(
                                                                                            env,
                                                                                            t,
                                                                                            d.wrapping_add(12),
                                                                                            44,
                                                                                        );
                                                                                        'label76: {
                                                                                            let fl = mload8!(a as usize + 412) as i32;
                                                                                            b = fl;
                                                                                            if b == 1 {
                                                                                                b = (if b != 0 { p } else { 0 });
                                                                                                g = 0;
                                                                                                c = q;
                                                                                                d = i;
                                                                                                loop {
                                                                                                    if c == 0 {
                                                                                                        unreachable!();
                                                                                                    }
                                                                                                    let fm = self.memeq32(env, d, b);
                                                                                                    if fm == 0 {
                                                                                                        c = c.wrapping_sub(32);
                                                                                                        g += 1;
                                                                                                        d = d.wrapping_add(32);
                                                                                                    }
                                                                                                }
                                                                                                c = mload32!(a as usize + 408) as i32;
                                                                                                b = mload32!(a as usize + 404) as i32;
                                                                                                let mut sv3_456_i32 = j as i32;
                                                                                                let mut sv3_452_i32 = b as i32;
                                                                                                let mut sv3_448_i32 = b as i32;
                                                                                                u = b.wrapping_add(c.wrapping_shl(6 as u32));
                                                                                                while b != u {
                                                                                                    y = a.wrapping_add(552);
                                                                                                    c = b.wrapping_add(40);
                                                                                                    let mut sv5_0_i64 = mload64!(c as usize) as i64;
                                                                                                    let mut sv26_0_i64 = sv5_0_i64 as i64;
                                                                                                    z = a.wrapping_add(560);
                                                                                                    d = b.wrapping_add(48);
                                                                                                    let mut sv6_0_i64 = mload64!(d as usize) as i64;
                                                                                                    let mut sv27_0_i64 = sv6_0_i64 as i64;
                                                                                                    w = a.wrapping_add(568);
                                                                                                    e = b.wrapping_add(56);
                                                                                                    let mut sv7_0_i64 = mload64!(e as usize) as i64;
                                                                                                    let mut sv24_0_i64 = sv7_0_i64 as i64;
                                                                                                    let mut sv4_32_i64 = mload64!(b as usize + 32) as i64;
                                                                                                    let mut sv3_544_i64 = sv4_32_i64 as i64;
                                                                                                    arg1 = mload64!(b as usize + 15) as i64;
                                                                                                    arg2 = mload64!(b as usize + 23) as i64;
                                                                                                    let fo = mload8!(b as usize + 31) as i32;
                                                                                                    j = fo;
                                                                                                    let mut sv8_0_i64 = sv4_32_i64 as i64;
                                                                                                    mstore64!(f.wrapping_add(8) as usize, sv5_0_i64 as u64);
                                                                                                    mstore64!(f.wrapping_add(16) as usize, sv6_0_i64 as u64);
                                                                                                    mstore64!(f.wrapping_add(24) as usize, sv7_0_i64 as u64);
                                                                                                    mstore8!(a as usize + 511, j as u8);
                                                                                                    c = b.wrapping_add(7);
                                                                                                    let mut sv4_0_i64 = mload64!(b as usize) as i64;
                                                                                                    let mut sv3_480_i64 = sv4_0_i64 as i64;
                                                                                                    h = b.wrapping_sub(-64);
                                                                                                    {
                                                                                                        let fp = self.memeq32(env, f, 1049298);
                                                                                                        if fp == 0 {
                                                                                                            mstore8!(a as usize + 263, j as u8);
                                                                                                            let mut sv4_0_i64 = mload64!(b as usize) as i64;
                                                                                                            let mut sv3_232_i64 = sv4_0_i64 as i64;
                                                                                                            let mut sv5_0_i64 = mload64!(c as usize) as i64;
                                                                                                            let mut sv3_239_i64 = sv5_0_i64 as i64;
                                                                                                            e = 0;
                                                                                                            c = o;
                                                                                                            d = n;
                                                                                                            loop {
                                                                                                                if c == 0 {
                                                                                                                    unreachable!();
                                                                                                                }
                                                                                                                let fq = self.memeq32(env, d, a.wrapping_add(232));
                                                                                                                if fq == 0 {
                                                                                                                    c = c.wrapping_sub(32);
                                                                                                                    e += 1;
                                                                                                                    d = d.wrapping_add(32);
                                                                                                                }
                                                                                                            }
                                                                                                            ap = (e as u32 as i64).wrapping_mul(ao);
                                                                                                            if (ap as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                                                                                                unreachable!();
                                                                                                            }
                                                                                                            d = ap as i32;
                                                                                                            c = d.wrapping_add(g);
                                                                                                            if (c as u32) < d as u32 {
                                                                                                                unreachable!();
                                                                                                            }
                                                                                                            if c as u32 >= s as u32 {
                                                                                                                unreachable!();
                                                                                                            }
                                                                                                            c = x.wrapping_add(c.wrapping_mul(33));
                                                                                                            let fr = mload8!(c as usize) as i32;
                                                                                                            if fr != 0 {
                                                                                                                unreachable!();
                                                                                                            }
                                                                                                            mstore8!(c as usize, 1 as u8);
                                                                                                            let mut sv3_544_i64 = mload64!(a as usize + 544) as i64;
                                                                                                            let mut sv5_1_i64 = sv3_544_i64 as i64;
                                                                                                            let mut sv26_0_i64 = mload64!(y as usize) as i64;
                                                                                                            mstore64!(c.wrapping_add(9) as usize, sv26_0_i64 as u64);
                                                                                                            let mut sv27_0_i64 = mload64!(z as usize) as i64;
                                                                                                            mstore64!(c.wrapping_add(17) as usize, sv27_0_i64 as u64);
                                                                                                            let mut sv24_0_i64 = mload64!(w as usize) as i64;
                                                                                                            mstore64!(c.wrapping_add(25) as usize, sv24_0_i64 as u64);
                                                                                                        }
                                                                                                    }
                                                                                                    b = h;
                                                                                                }
                                                                                                sv3_452_i32 = b as i32;
                                                                                                break 'label76;
                                                                                                let mut sv3_452_i32 = h as i32;
                                                                                                let mut sv4_0_i64 = mload64!(b as usize) as i64;
                                                                                                let mut sv3_336_i64 = sv4_0_i64 as i64;
                                                                                                let fs = mload64!(b.wrapping_add(7) as usize) as i64;
                                                                                                let mut sv3_343_i64 = fs as i64;
                                                                                                ao = sv3_336_i64;
                                                                                                let mut sv3_304_i64 = ao as i64;
                                                                                                let mut sv3_311_i64 = sv3_343_i64 as i64;
                                                                                                let ft = mload8!(a as usize + 310) as i64;
                                                                                                let fu = mload16!(a as usize + 308) as i64;
                                                                                                c = ((ao & 4294967295 | ft.wrapping_shl(48 as u32) | fu.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                                                                                an = sv3_311_i64;
                                                                                                f = ao as i32;
                                                                                                d = 11;
                                                                                                break 'label37;
                                                                                            }
                                                                                        }
                                                                                        d = k;
                                                                                        continue 'label51;
                                                                                    }
                                                                                    unreachable!();
                                                                                }
                                                                            }
                                                                            mstore8!(a as usize + 240, e as u8);
                                                                            let mut sv3_236_i32 = -2147483648 as i32;
                                                                            break 'label36;
                                                                        }
                                                                        break 'label36;
                                                                    }
                                                                    c = 0;
                                                                }
                                                                ao = (arg1 as u64).wrapping_shr(32 as u32) as i64;
                                                                if d != 27 {
                                                                    g = (arg1 as u64).wrapping_shr(40 as u32) as i64 as i32;
                                                                    k = ao as i32;
                                                                    m = arg1 as i32;
                                                                    break 'label25;
                                                                }
                                                                l = ao as i32;
                                                                b = l.wrapping_add((arg2 as i32).wrapping_shl(6 as u32));
                                                                let fv = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                                arg1 = fv;
                                                                'label82: loop {
                                                                    if b == l {
                                                                        let mut svarg0_16_i64 = arg1 as i64;
                                                                        let mut svarg0_8_i64 = an as i64;
                                                                        mstore8!(arg0 as usize, 27 as u8);
                                                                        break 'label24;
                                                                    }
                                                                    self.memmove(
                                                                        env,
                                                                        a.wrapping_add(480),
                                                                        l,
                                                                        64,
                                                                    );
                                                                    let fx = mload64!(l.wrapping_add(56) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(256) as usize, fx as u64);
                                                                    let fy = mload64!(l.wrapping_add(48) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(248) as usize, fy as u64);
                                                                    let fz = mload64!(l.wrapping_add(40) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(240) as usize, fz as u64);
                                                                    let mut sv14_32_i64 = mload64!(l as usize + 32) as i64;
                                                                    let mut sv3_232_i64 = sv14_32_i64 as i64;
                                                                    let ga = val_to_i64(Bytes::new(env).into_val(env));
                                                                    let gb = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(ga))).into_val(env));
                                                                    arg2 = gb;
                                                                    let gc = mload64!(a.wrapping_add(504) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(416) as usize, gc as u64);
                                                                    let gd = mload64!(a.wrapping_add(496) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(408) as usize, gd as u64);
                                                                    let ge = mload64!(a.wrapping_add(488) as usize) as i64;
                                                                    mstore64!(a.wrapping_add(400) as usize, ge as u64);
                                                                    let mut sv3_480_i64 = mload64!(a as usize + 480) as i64;
                                                                    let mut sv3_392_i64 = sv3_480_i64 as i64;
                                                                    l = l.wrapping_sub(-64);
                                                                    d = 33;
                                                                    loop {
                                                                        c = 0;
                                                                        if d == 1 {
                                                                            e = 0;
                                                                        } else {
                                                                            e = d -= 1;
                                                                            d = e;
                                                                            let gf = mload8!(a.wrapping_add(392).wrapping_add(d).wrapping_sub(2) as usize) as i32;
                                                                        }
                                                                    }
                                                                    loop {
                                                                        if c == e {
                                                                            c = 0;
                                                                        } else {
                                                                            let gg = mload8!(a.wrapping_add(392).wrapping_add(c) as usize) as i32;
                                                                            if gg == 0 {
                                                                                c += 1;
                                                                            }
                                                                        }
                                                                    }
                                                                    if c as u32 <= e as u32 {
                                                                        d = a.wrapping_add(392);
                                                                        let gh = val_to_i64(String::from_str(env, ""));
                                                                        ao = gh;
                                                                        let mut sv3_392_i64 = ao as i64;
                                                                        let gi = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                                        let gj = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(arg1)); v.push_back(val_from_i64(gi)); val_to_i64(v.into_val(env)) };
                                                                        arg1 = gj;
                                                                        continue 'label82;
                                                                    }
                                                                }
                                                                unreachable!();
                                                            }
                                                            mstore8!(a as usize + 232, 1 as u8);
                                                            o = 0;
                                                            h = g;
                                                            cg = j;
                                                        }
                                                        q = cg;
                                                        d = 23;
                                                        if at as u64 > 4294967295 as u64 {
                                                            break 'label34;
                                                        }
                                                        if aq as u64 > 4294967295 as u64 {
                                                            break 'label28;
                                                        }
                                                        'label87: {
                                                            c = at as i32;
                                                            if (c.wrapping_sub(65536) as u32) < -65535 as u32 {
                                                                d = 4;
                                                            } else {
                                                                y += 1;
                                                                ai = aq as i32;
                                                                self.alloc_range(
                                                                    env,
                                                                    a.wrapping_add(32),
                                                                    c,
                                                                    1,
                                                                    64,
                                                                );
                                                                d = 0;
                                                                p = 0;
                                                                'label89: {
                                                                    'label90: loop {
                                                                        if c == p {
                                                                            break 'label87;
                                                                        }
                                                                        b = a.wrapping_add(480);
                                                                        g = a.wrapping_add(80);
                                                                        self.span_take(
                                                                            env,
                                                                            b,
                                                                            g,
                                                                            ai,
                                                                        );
                                                                        let gk = mload8!(a as usize + 480) as i32;
                                                                        d = gk;
                                                                        if d == 27 {
                                                                            aq = mload64!(a as usize + 488) as i64;
                                                                            aa = mload32!(a as usize + 484) as i32;
                                                                            self.span_take(
                                                                                env,
                                                                                b,
                                                                                g,
                                                                                32,
                                                                            );
                                                                            'label91: {
                                                                                let gl: i32;
                                                                                {
                                                                                    let gm = mload8!(a as usize + 480) as i32;
                                                                                    d = gm;
                                                                                    if d == 27 {
                                                                                        arg2 = mload64!(a as usize + 488) as i64;
                                                                                        g = arg2 as i32;
                                                                                        'label93: {
                                                                                            if arg2 as u64 >= 4294967296 as u64 {
                                                                                                b = (arg2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                                                                e = 0;
                                                                                                'label94: {
                                                                                                    loop {
                                                                                                        if b == e {
                                                                                                            break 'label94;
                                                                                                        }
                                                                                                        j = e.wrapping_add(g);
                                                                                                        let gn = mload8!(j as usize) as i32;
                                                                                                        if gn == 0 {
                                                                                                            e += 1;
                                                                                                        }
                                                                                                    }
                                                                                                    k = g -= 1;
                                                                                                    loop {
                                                                                                        g = b;
                                                                                                        if g == 0 {
                                                                                                            unreachable!();
                                                                                                        }
                                                                                                        b -= 1;
                                                                                                        let go = mload8!(g.wrapping_add(k) as usize) as i32;
                                                                                                    }
                                                                                                    if e as u32 <= g as u32 {
                                                                                                        self.func69(
                                                                                                            env,
                                                                                                            a.wrapping_add(352),
                                                                                                            j,
                                                                                                            g.wrapping_sub(e),
                                                                                                        );
                                                                                                        break 'label93;
                                                                                                    }
                                                                                                    unreachable!();
                                                                                                }
                                                                                                break 'label93;
                                                                                            }
                                                                                            b = mload32!(a as usize + 484) as i32;
                                                                                        }
                                                                                        self.bytes_to_fixed32(env, a.wrapping_add(364), a.wrapping_add(352));
                                                                                        mstore64!(a.wrapping_add(504) as usize, 0 /* False */ as u64);
                                                                                        mstore64!(a.wrapping_add(496) as usize, 0 /* False */ as u64);
                                                                                        mstore64!(a.wrapping_add(488) as usize, 0 /* False */ as u64);
                                                                                        let mut sv3_480_i64 = 0 /* False */ as i64;
                                                                                        b = mload32!(a as usize + 372) as i32;
                                                                                        self.span_set(
                                                                                            env,
                                                                                            a.wrapping_add(24),
                                                                                            a.wrapping_add(480),
                                                                                            b,
                                                                                        );
                                                                                        let mut sv3_24_i32 = mload32!(a as usize + 24) as i32;
                                                                                        let mut sv3_28_i32 = mload32!(a as usize + 28) as i32;
                                                                                        let mut sv3_368_i32 = mload32!(a as usize + 368) as i32;
                                                                                        self.memcpy_checked(
                                                                                            env,
                                                                                            sv3_24_i32,
                                                                                            sv3_28_i32,
                                                                                            sv3_368_i32,
                                                                                            b,
                                                                                        );
                                                                                        let gp = mload16!(a as usize + 500) as i32;
                                                                                        let gq = mload8!(u as usize) as i32;
                                                                                        g = gp | gq.wrapping_shl(16 as u32);
                                                                                        let gr = mload16!(a as usize + 480) as i32;
                                                                                        let gs = mload8!(a as usize + 482) as i32;
                                                                                        f = gr | gs.wrapping_shl(16 as u32);
                                                                                        let gt = mload8!(a as usize + 511) as i32;
                                                                                        j = gt;
                                                                                        arg2 = mload64!(a as usize + 503) as i64;
                                                                                        let gu = mload8!(a as usize + 499) as i32;
                                                                                        k = gu;
                                                                                        e = mload32!(a as usize + 495) as i32;
                                                                                        an = mload64!(a as usize + 487) as i64;
                                                                                        let mut sv3_483_i32 = mload32!(a as usize + 483) as i32;
                                                                                        b = sv3_483_i32;
                                                                                        if d == 27 {
                                                                                            break 'label91;
                                                                                        }
                                                                                        gl = b;
                                                                                    } else {
                                                                                        let mut sv20_0_i64 = mload64!(r as usize) as i64;
                                                                                        let mut sv3_232_i64 = sv20_0_i64 as i64;
                                                                                        let gv = mload64!(r.wrapping_add(7) as usize) as i64;
                                                                                        let mut sv3_239_i64 = gv as i64;
                                                                                        let gw = mload16!(a as usize + 481) as i32;
                                                                                        let gx = mload8!(a as usize + 483) as i32;
                                                                                        f = gw | gx.wrapping_shl(16 as u32);
                                                                                        let gy = mload16!(a as usize + 501) as i32;
                                                                                        let gz = mload8!(a.wrapping_add(503) as usize) as i32;
                                                                                        g = gy | gz.wrapping_shl(16 as u32);
                                                                                        let ha = mload8!(a as usize + 512) as i32;
                                                                                        j = ha;
                                                                                        arg2 = mload64!(a as usize + 504) as i64;
                                                                                        let hb = mload8!(a as usize + 500) as i32;
                                                                                        k = hb;
                                                                                        e = mload32!(a as usize + 496) as i32;
                                                                                        an = mload64!(a as usize + 488) as i64;
                                                                                        gl = mload32!(a as usize + 484) as i32;
                                                                                    }
                                                                                }
                                                                                c = gl;
                                                                                let mut sv3_455_i64 = sv3_239_i64 as i64;
                                                                                let mut sv3_448_i64 = sv3_232_i64 as i64;
                                                                                break 'label89;
                                                                            }
                                                                            mstore16!(a as usize + 500, g as u16);
                                                                            mstore8!(u as usize, (g as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                            mstore8!(a as usize + 511, j as u8);
                                                                            mstore8!(a as usize + 499, k as u8);
                                                                            sv3_483_i32 = b as i32;
                                                                            mstore16!(a as usize + 480, f as u16);
                                                                            mstore8!(a as usize + 482, (f as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                            let mut sv3_232_i32 = aa as i32;
                                                                            self.bytes_to_fixed32_struct(env, ab, a.wrapping_add(232));
                                                                            let mut sv3_480_i32 = mload32!(a as usize + 480) as i32;
                                                                            let mut sv3_296_i32 = sv3_480_i32 as i32;
                                                                            let mut sv3_483_i32 = mload32!(a as usize + 483) as i32;
                                                                            let mut sv3_299_i32 = sv3_483_i32 as i32;
                                                                            let mut sv24_0_i64 = mload64!(w as usize) as i64;
                                                                            let mut sv3_448_i64 = sv24_0_i64 as i64;
                                                                            let hc = mload64!(w.wrapping_add(7) as usize) as i64;
                                                                            let mut sv3_455_i64 = hc as i64;
                                                                            let hd = mload8!(ag as usize) as i32;
                                                                            g = hd;
                                                                            let he = mload8!(ah as usize) as i32;
                                                                            e = he;
                                                                            let hf = mload8!(a as usize + 487) as i32;
                                                                            j = hf;
                                                                            k = mload32!(a as usize + 491) as i32;
                                                                            arg2 = mload64!(a as usize + 495) as i64;
                                                                            f = mload32!(a as usize + 503) as i32;
                                                                            let hg = mload8!(a as usize + 507) as i32;
                                                                            aa = hg;
                                                                            an = mload64!(a as usize + 511) as i64;
                                                                            let hh = mload8!(a as usize + 519) as i32;
                                                                            aj = hh;
                                                                            let hi = mload16!(a as usize + 488) as i32;
                                                                            ak = hi;
                                                                            let hj = mload16!(a as usize + 508) as i32;
                                                                            al = hj;
                                                                            b = a.wrapping_add(400);
                                                                            let hk = mload8!(x.wrapping_add(8) as usize) as i32;
                                                                            mstore8!(b as usize, hk as u8);
                                                                            am = a.wrapping_add(288);
                                                                            let hl = mload8!(b as usize) as i32;
                                                                            mstore8!(am as usize, hl as u8);
                                                                            let mut sv25_0_i64 = mload64!(x as usize) as i64;
                                                                            let mut sv3_392_i64 = sv25_0_i64 as i64;
                                                                            let mut sv3_383_i64 = sv3_455_i64 as i64;
                                                                            let mut sv3_376_i64 = sv3_448_i64 as i64;
                                                                            let mut sv3_280_i64 = sv3_392_i64 as i64;
                                                                            let mut sv3_544_i64 = sv3_376_i64 as i64;
                                                                            let mut sv3_551_i64 = sv3_383_i64 as i64;
                                                                            d = mload32!(a as usize + 276) as i32;
                                                                            let mut sv3_268_i32 = mload32!(a as usize + 268) as i32;
                                                                            if d == sv3_268_i32 {
                                                                                self.alloc_realloc(env, a.wrapping_add(268));
                                                                            }
                                                                            p += 1;
                                                                            let mut sv3_272_i32 = mload32!(a as usize + 272) as i32;
                                                                            b = sv3_272_i32.wrapping_add(d.wrapping_shl(6 as u32));
                                                                            e = al | e.wrapping_shl(16 as u32);
                                                                            mstore16!(b as usize + 28, e as u16);
                                                                            g = ak | g.wrapping_shl(16 as u32);
                                                                            mstore16!(b as usize + 8, g as u16);
                                                                            let mut sv3_296_i32 = mload32!(a as usize + 296) as i32;
                                                                            let mut sv4_0_i32 = sv3_296_i32 as i32;
                                                                            mstore8!(b as usize + 39, aj as u8);
                                                                            mstore8!(b as usize + 27, aa as u8);
                                                                            mstore8!(b as usize + 7, j as u8);
                                                                            let mut sv3_544_i64 = mload64!(a as usize + 544) as i64;
                                                                            mstore8!(b.wrapping_add(30) as usize, (e as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                            mstore8!(b.wrapping_add(10) as usize, (g as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                            let mut sv3_299_i32 = mload32!(a as usize + 299) as i32;
                                                                            mstore32!(b.wrapping_add(3) as usize, sv3_299_i32 as u32);
                                                                            let mut sv3_551_i64 = mload64!(a as usize + 551) as i64;
                                                                            mstore64!(b.wrapping_add(47) as usize, sv3_551_i64 as u64);
                                                                            let hm = mload8!(am as usize) as i32;
                                                                            mstore8!(b.wrapping_add(63) as usize, hm as u8);
                                                                            let mut sv3_280_i64 = mload64!(a as usize + 280) as i64;
                                                                            d += 1;
                                                                            continue 'label90;
                                                                        }
                                                                    }
                                                                    let mut sv3_513_i64 = mload64!(a as usize + 513) as i64;
                                                                    let mut sv3_448_i64 = sv3_513_i64 as i64;
                                                                    let hn = mload64!(a.wrapping_add(520) as usize) as i64;
                                                                    let mut sv3_455_i64 = hn as i64;
                                                                    let ho = mload16!(a as usize + 481) as i32;
                                                                    let hp = mload8!(a as usize + 483) as i32;
                                                                    f = ho | hp.wrapping_shl(16 as u32);
                                                                    let hq = mload16!(a as usize + 501) as i32;
                                                                    let hr = mload8!(a.wrapping_add(503) as usize) as i32;
                                                                    g = hq | hr.wrapping_shl(16 as u32);
                                                                    let hs = mload8!(a as usize + 512) as i32;
                                                                    j = hs;
                                                                    arg2 = mload64!(a as usize + 504) as i64;
                                                                    let ht = mload8!(a as usize + 500) as i32;
                                                                    k = ht;
                                                                    e = mload32!(a as usize + 496) as i32;
                                                                    an = mload64!(a as usize + 488) as i64;
                                                                    c = mload32!(a as usize + 484) as i32;
                                                                }
                                                                let mut sv3_383_i64 = sv3_455_i64 as i64;
                                                                let mut sv3_376_i64 = sv3_448_i64 as i64;
                                                                let mut sv3_336_i64 = sv3_376_i64 as i64;
                                                                let mut sv3_343_i64 = sv3_383_i64 as i64;
                                                            }
                                                            let mut sv3_176_i32 = e as i32;
                                                            let mut sv3_152_i64 = sv3_336_i64 as i64;
                                                            let mut sv3_159_i64 = sv3_343_i64 as i64;
                                                            break 'label28;
                                                        }
                                                        d = a.wrapping_add(188);
                                                        let hu = mload8!(a.wrapping_add(228) as usize) as i32;
                                                        mstore8!(d as usize, hu as u8);
                                                        let mut sv3_304_i64 = mload64!(a as usize + 304) as i64;
                                                        sv3_152_i64 = sv3_304_i64 as i64;
                                                        let mut sv3_311_i64 = mload64!(a as usize + 311) as i64;
                                                        sv3_159_i64 = sv3_311_i64 as i64;
                                                        let mut sv3_224_i32 = mload32!(a as usize + 224) as i32;
                                                        let mut sv3_184_i32 = sv3_224_i32 as i32;
                                                        arg2 = sv3_272_i32;
                                                        let mut sv3_268_i32 = mload32!(a as usize + 268) as i32;
                                                        let mut sv3_168_i32 = sv3_268_i32 as i32;
                                                        an = sv3_168_i32;
                                                        let mut sv3_135_i64 = sv3_159_i64 as i64;
                                                        let mut sv3_128_i64 = sv3_152_i64 as i64;
                                                        g = a.wrapping_add(182);
                                                        let hv = mload8!(a.wrapping_add(151) as usize) as i32;
                                                        mstore8!(g as usize, hv as u8);
                                                        let hw = mload16!(a as usize + 149) as i32;
                                                        mstore16!(a as usize + 180, hw as u16);
                                                        let mut sv3_199_i64 = sv3_135_i64 as i64;
                                                        let mut sv3_192_i64 = sv3_128_i64 as i64;
                                                        e = mload32!(a as usize + 124) as i32;
                                                        let mut sv3_116_i32 = mload32!(a as usize + 116) as i32;
                                                        if e == sv3_116_i32 {
                                                            b = self.global0.wrapping_sub(16);
                                                            self.global0 = b;
                                                            c = a.wrapping_add(116);
                                                            let mut sv5_0_i32 = mload32!(c as usize) as i32;
                                                            self.alloc_size_align(
                                                                env,
                                                                b.wrapping_add(8),
                                                                c,
                                                                sv5_0_i32,
                                                                1,
                                                                8,
                                                                56,
                                                            );
                                                            c = mload32!(b as usize + 8) as i32;
                                                            if c != -2147483647 {
                                                                let mut sv4_12_i32 = mload32!(b as usize + 12) as i32;
                                                                self.alloc_trap(env, c, sv4_12_i32);
                                                                unreachable!();
                                                            }
                                                            self.global0 = b.wrapping_add(16);
                                                        }
                                                        c = (au as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                        f = (au as u64).wrapping_shr(0 as u32) as i64 as i32;
                                                        let mut sv3_120_i32 = mload32!(a as usize + 120) as i32;
                                                        b = sv3_120_i32.wrapping_add(e.wrapping_mul(56));
                                                        mstore16!(b as usize + 21, h as u16);
                                                        mstore8!(b as usize + 32, q as u8);
                                                        mstore8!(b as usize + 20, o as u8);
                                                        mstore32!(b as usize + 16, (arg2 as u64).wrapping_shr(32 as u32) as i64 as u32);
                                                        let mut sv4_0_i64 = au as i64;
                                                        let mut sv3_192_i64 = mload64!(a as usize + 192) as i64;
                                                        let mut sv3_184_i32 = mload32!(a as usize + 184) as i32;
                                                        let hy = mload16!(a as usize + 180) as i32;
                                                        mstore16!(b as usize + 53, hy as u16);
                                                        mstore8!(b.wrapping_add(23) as usize, (h as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                        let mut sv3_199_i64 = mload64!(a as usize + 199) as i64;
                                                        mstore64!(b.wrapping_add(40) as usize, sv3_199_i64 as u64);
                                                        let hz = mload8!(d as usize) as i32;
                                                        mstore8!(b.wrapping_add(52) as usize, hz as u8);
                                                        let ia = mload8!(g as usize) as i32;
                                                        mstore8!(b.wrapping_add(55) as usize, ia as u8);
                                                        d = e += 1;
                                                        g = h;
                                                        j = q;
                                                        k = o;
                                                        continue 'label33;
                                                    }
                                                }
                                                break 'label28;
                                            }
                                        }
                                    }
                                    let mut svarg0_16_i64 = ap as i64;
                                    let mut svarg0_8_i64 = av as i64;
                                    break 'label24;
                                }
                                let mut sv3_513_i64 = mload64!(a as usize + 513) as i64;
                                let ib = mload64!(a.wrapping_add(520) as usize) as i64;
                                let mut sv3_327_i64 = ib as i64;
                                arg1 = mload32!(a as usize + 481) as i64;
                                let ic = mload8!(a as usize + 487) as i64;
                                let id = mload16!(a as usize + 485) as i64;
                                c = ((arg1 | ic.wrapping_shl(48 as u32) | id.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                let ie = mload16!(a as usize + 501) as i32;
                                let ig = mload8!(a.wrapping_add(503) as usize) as i32;
                                g = ie | ig.wrapping_shl(16 as u32);
                                let ih = mload8!(a as usize + 512) as i32;
                                j = ih;
                                let ii = mload8!(a as usize + 500) as i32;
                                k = ii;
                                let mut sv3_496_i32 = mload32!(a as usize + 496) as i32;
                                m = sv3_496_i32;
                                f = arg1 as i32;
                                break 'label26;
                            }
                            let mut sv3_176_i32 = sv3_496_i32 as i32;
                            let mut sv3_152_i64 = sv3_513_i64 as i64;
                            let ij = mload64!(a.wrapping_add(520) as usize) as i64;
                            let mut sv3_159_i64 = ij as i64;
                            let ik = mload16!(a as usize + 481) as i32;
                            let il = mload8!(a as usize + 483) as i32;
                            f = ik | il.wrapping_shl(16 as u32);
                            let im = mload16!(a as usize + 501) as i32;
                            let in = mload8!(a.wrapping_add(503) as usize) as i32;
                            g = im | in.wrapping_shl(16 as u32);
                            let mut sv3_488_i64 = mload64!(a as usize + 488) as i64;
                            an = sv3_488_i64;
                            let mut sv3_484_i32 = mload32!(a as usize + 484) as i32;
                            c = sv3_484_i32;
                            let io = mload8!(a as usize + 500) as i32;
                            k = io;
                            let ip = mload8!(a as usize + 512) as i32;
                            j = ip;
                        }
                        let mut sv3_135_i64 = sv3_159_i64 as i64;
                        let mut sv3_128_i64 = sv3_152_i64 as i64;
                        sv3_327_i64 = sv3_135_i64 as i64;
                        m = sv3_176_i32;
                        break 'label25;
                    }
                    let iq = mload64!(a.wrapping_add(520) as usize) as i64;
                    sv3_327_i64 = iq as i64;
                    let ir = mload16!(a as usize + 481) as i32;
                    let is = mload8!(a as usize + 483) as i32;
                    f = ir | is.wrapping_shl(16 as u32);
                    let it = mload16!(a as usize + 501) as i32;
                    let iu = mload8!(a.wrapping_add(503) as usize) as i32;
                    g = it | iu.wrapping_shl(16 as u32);
                    let iv = mload8!(a as usize + 512) as i32;
                    j = iv;
                    let iw = mload8!(a as usize + 500) as i32;
                    k = iw;
                    m = sv3_496_i32;
                    an = sv3_488_i64;
                    c = sv3_484_i32;
                    break 'label25;
                }
                an = o as u32 as i64 | (e as u32 as i64).wrapping_shl(32 as u32);
            }
            mstore8!(arg0 as usize + 32, j as u8);
            mstore8!(arg0 as usize, d as u8);
            mstore64!(arg0.wrapping_add(40) as usize, sv3_327_i64 as u64);
            mstore8!(arg0.wrapping_add(7) as usize, (c as u32).wrapping_shr(24 as u32) as i32 as u32 as i64 as u8);
            mstore16!(arg0.wrapping_add(5) as usize, (c as u32).wrapping_shr(8 as u32) as i32 as u32 as i64 as u16);
            mstore32!(arg0 as usize + 1, (c.wrapping_shl(24 as u32) as u32 as i64 | f as u32 as i64 & 16777215) as u32);
            svarg0_16_i64 = (m as u32 as i64 | (g as u32 as i64).wrapping_shl(40 as u32) | (k as u32 as i64 & 255).wrapping_shl(32 as u32)) as i64;
        }
        self.global0 = a.wrapping_add(576);
    }

    fn require_alloc(
        &mut self,
        env: &Env,
        mut arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_size_align(
            env,
            a.wrapping_add(8),
            arg0,
            arg1,
            arg2,
            1,
            arg3,
        );
        arg0 = mload32!(a as usize + 8) as i32;
        if arg0 != -2147483647 {
            let mut sv4_12_i32 = mload32!(a as usize + 12) as i32;
            self.alloc_trap(env, arg0, sv4_12_i32);
            unreachable!();
        }
        self.global0 = a.wrapping_add(16);
    }

    fn build_entry_from_bytes(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        self.bytes_to_fixed32(env, a.wrapping_add(20), arg1);
        arg1 = a.wrapping_add(56);
        let mut svarg1_0_i64 = 0 /* False */ as i64;
        b = a.wrapping_add(48);
        let mut sv3_0_i64 = 0 /* False */ as i64;
        c = a.wrapping_add(40);
        let mut sv4_0_i64 = 0 /* False */ as i64;
        let mut sv2_32_i64 = 0 /* False */ as i64;
        d = mload32!(a as usize + 28) as i32;
        self.span_set(
            env,
            a.wrapping_add(8),
            a.wrapping_add(32),
            d,
        );
        let mut sv2_8_i32 = mload32!(a as usize + 8) as i32;
        let mut sv2_12_i32 = mload32!(a as usize + 12) as i32;
        let mut sv2_24_i32 = mload32!(a as usize + 24) as i32;
        self.memcpy_checked(
            env,
            sv2_8_i32,
            sv2_12_i32,
            sv2_24_i32,
            d,
        );
        let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
        mstore64!(arg0.wrapping_add(24) as usize, svarg1_0_i64 as u64);
        let mut sv3_0_i64 = mload64!(b as usize) as i64;
        mstore64!(arg0.wrapping_add(16) as usize, sv3_0_i64 as u64);
        let mut sv4_0_i64 = mload64!(c as usize) as i64;
        mstore64!(arg0.wrapping_add(8) as usize, sv4_0_i64 as u64);
        let mut sv2_32_i64 = mload64!(a as usize + 32) as i64;
        self.global0 = a.wrapping_sub(-64);
    }

    fn memeq32(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = self.memcmp(
            env,
            arg0,
            arg1,
            32,
        );
        (a == 0) as i32
    }

    fn alloc_range_one(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_core(
            env,
            a.wrapping_add(4),
            arg1,
            1,
            1,
            1,
        );
        b = mload32!(a as usize + 8) as i32;
        let mut sv2_4_i32 = mload32!(a as usize + 4) as i32;
        if sv2_4_i32 == 1 {
            let mut sv2_12_i32 = mload32!(a as usize + 12) as i32;
            self.alloc_trap(env, b, sv2_12_i32);
            unreachable!();
        }
        self.global0 = a.wrapping_add(16);
    }

    fn copy_to_linear_memory(
        &mut self,
        env: &Env,
        arg0: i64,
        arg1: i32,
        arg2: i32,
    ) {
        self.copy_bytes_to_linear_memory(env, arg0, 0, (arg1 as u32 as i64).wrapping_shl(32 as u32) | 0, (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0);
    }

    fn span_take(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        'label0: {
            b = mload32!(arg1 as usize + 8) as i32;
            loop {
                match ((if ((arg2 as u32) < b as u32) as i32 != 0 { -1 } else { (b != arg2) as i32 })) & 255 {
                    0 | 1 | _ => break,
                }
            }
            mstore8!(arg0 as usize, 26 as u8);
            break 'label0;
            if arg2 as u32 <= b as u32 {
                self.func125(
                    env,
                    a,
                    arg1,
                    b.wrapping_sub(arg2),
                );
            } else {
                unreachable!();
                let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                svarg1_0_i64 = 4294967296 as i64;
                arg1 = arg1.wrapping_add(8);
                mstore32!(a.wrapping_add(8) as usize, svarg1_0_i64 as u32);
            }
            mstore8!(arg0 as usize, 27 as u8);
            let e = mload32!(a.wrapping_add(8) as usize) as i32;
            mstore32!(arg0.wrapping_add(12) as usize, e as u32);
        }
        self.global0 = a.wrapping_add(16);
    }

    fn func66(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i64 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        self.span_take(
            env,
            a.wrapping_add(16),
            arg1,
            arg2,
        );
        'label0: {
            let h: i32;
            'label1: {
                {
                    let i = mload8!(a as usize + 16) as i32;
                    arg1 = i;
                    if arg1 == 27 {
                        arg1 = mload32!(a as usize + 28) as i32;
                        b = mload32!(a as usize + 24) as i32;
                        let mut sv3_20_i32 = mload32!(a as usize + 20) as i32;
                        arg2 = 0;
                        loop {
                            let j = b;
                            if arg1 == arg2 {
                                h = j;
                                break 'label1;
                            }
                            let k = arg2;
                            arg2 += 1;
                            let l = mload8!(k.wrapping_add(b) as usize) as i32;
                            c = l;
                        }
                        mstore8!(b as usize, c as u8);
                        c = arg2.wrapping_add(b);
                        arg1 = arg1.wrapping_sub(arg2);
                        e = b += 1;
                        arg2 = 0;
                    } else {
                        let m = mload16!(a as usize + 17) as i32;
                        mstore16!(arg0 as usize + 1, m as u16);
                        let n = mload8!(a as usize + 19) as i32;
                        mstore8!(arg0.wrapping_add(3) as usize, n as u8);
                        let o = mload64!(a.wrapping_add(40) as usize) as i64;
                        mstore64!(arg0.wrapping_add(24) as usize, o as u64);
                        let p = mload64!(a.wrapping_add(48) as usize) as i64;
                        mstore64!(arg0.wrapping_add(32) as usize, p as u64);
                        let q = mload64!(a.wrapping_add(56) as usize) as i64;
                        mstore64!(arg0.wrapping_add(40) as usize, q as u64);
                        f = sv3_20_i32;
                        mstore8!(arg0 as usize, arg1 as u8);
                        break 'label0;
                    }
                }
                loop {
                    if arg1 != 0 {
                        let r = mload8!(arg2.wrapping_add(c) as usize) as i32;
                        mstore8!(arg2.wrapping_add(e) as usize, r as u8);
                        arg1 -= 1;
                        arg2 += 1;
                    }
                }
                h = arg2.wrapping_add(b) += 1;
            }
            arg1 = h.wrapping_sub(b);
            if arg1 as u32 > 8 as u32 {
                self.bytes_to_fixed32_struct(env, arg0.wrapping_add(1), a.wrapping_add(16));
                mstore8!(arg0 as usize, 1 as u8);
                break 'label0;
            }
            let mut value_lo = 0 /* False */ as i64;
            self.func123(
                env,
                a.wrapping_add(8),
                8.wrapping_sub(arg1),
                a.wrapping_add(16),
                8,
            );
            let mut sv3_8_i32 = mload32!(a as usize + 8) as i32;
            let mut sv3_12_i32 = mload32!(a as usize + 12) as i32;
            self.memcpy_checked(
                env,
                sv3_8_i32,
                sv3_12_i32,
                b,
                arg1,
            );
            mstore8!(arg0 as usize, 27 as u8);
            f = mload64!(a as usize + 16) as i64;
        }
        self.global0 = a.wrapping_sub(-64);
    }

    fn entry_copy_if_ok(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let c = mload8!(arg1 as usize) as i32;
        if c == 0 {
            let d = mload64!(arg1.wrapping_add(25) as usize) as i64;
            mstore64!(arg0.wrapping_add(24) as usize, d as u64);
            let e = mload64!(arg1.wrapping_add(17) as usize) as i64;
            mstore64!(arg0.wrapping_add(16) as usize, e as u64);
            let f = mload64!(arg1.wrapping_add(9) as usize) as i64;
            mstore64!(arg0.wrapping_add(8) as usize, f as u64);
        }
    }

    fn memcmp_sign32(
        &mut self,
        env: &Env,
        mut arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = self.memcmp(
            env,
            arg0,
            arg1,
            32,
        );
        arg0 = a;
        (if (arg0 < 0) as i32 != 0 { -1 } else { (arg0 != 0) as i32 })
    }

    fn func69(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_range_fill(
            env,
            a.wrapping_add(8),
            arg2,
            1,
        );
        let mut sv3_12_i32 = mload32!(a as usize + 12) as i32;
        let d = self.memmove(
            env,
            sv3_12_i32,
            arg1,
            arg2,
        );
        self.global0 = a.wrapping_add(16);
    }


    fn entry_from_bytes_val(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        b = a.wrapping_add(24);
        let mut sv3_0_i64 = 0 /* False */ as i64;
        c = a.wrapping_add(16);
        let mut sv4_0_i64 = 0 /* False */ as i64;
        d = a.wrapping_add(8);
        let mut sv5_0_i64 = 0 /* False */ as i64;
        let mut sv2_0_i64 = 0 /* False */ as i64;
        self.copy_to_linear_memory(
            env,
            arg1,
            a,
            32,
        );
        let mut sv3_0_i64 = mload64!(b as usize) as i64;
        mstore64!(arg0.wrapping_add(24) as usize, sv3_0_i64 as u64);
        let mut sv4_0_i64 = mload64!(c as usize) as i64;
        mstore64!(arg0.wrapping_add(16) as usize, sv4_0_i64 as u64);
        let mut sv5_0_i64 = mload64!(d as usize) as i64;
        mstore64!(arg0.wrapping_add(8) as usize, sv5_0_i64 as u64);
        let mut sv2_0_i64 = mload64!(a as usize) as i64;
        self.global0 = a.wrapping_add(32);
    }

    fn alloc_range_fill(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.alloc_core(
            env,
            a.wrapping_add(4),
            arg1,
            0,
            1,
            arg2,
        );
        arg1 = mload32!(a as usize + 8) as i32;
        let mut sv3_4_i32 = mload32!(a as usize + 4) as i32;
        if sv3_4_i32 == 0 {
            let mut sv3_12_i32 = mload32!(a as usize + 12) as i32;
            self.global0 = a.wrapping_add(16);
        }
        self.alloc_trap(env, arg1, sv3_12_i32);
    }

    fn memcpy_like(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        b = mload32!(arg1 as usize + 16) as i32;
        c = mload32!(arg1 as usize + 20) as i32;
        let d: i32;
        'label0: {
            loop {
                a = b;
                let e = 0;
                if (a == 0) as i32 | a == c {
                    d = e;
                    break 'label0;
                }
                b = a.wrapping_add(33);
                let f = mload8!(a as usize) as i32;
            }
            let g = mload64!(a.wrapping_add(25) as usize) as i64;
            mstore64!(arg0.wrapping_add(25) as usize, g as u64);
            let h = mload64!(a.wrapping_add(17) as usize) as i64;
            mstore64!(arg0.wrapping_add(17) as usize, h as u64);
            let i = mload64!(a.wrapping_add(9) as usize) as i64;
            mstore64!(arg0.wrapping_add(9) as usize, i as u64);
            d = 1;
        }
        mstore8!(0 as usize, d as u8);
    }

    fn func74(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        b = arg1.wrapping_sub(32);
        let d = self.memcmp_sign32(env, arg1, b);
        if d & 255 == 255 {
            let e = mload64!(arg1.wrapping_add(24) as usize) as i64;
            mstore64!(a.wrapping_add(24) as usize, e as u64);
            let f = mload64!(arg1.wrapping_add(16) as usize) as i64;
            mstore64!(a.wrapping_add(16) as usize, f as u64);
            let g = mload64!(arg1.wrapping_add(8) as usize) as i64;
            mstore64!(a.wrapping_add(8) as usize, g as u64);
            let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
            let mut sv2_0_i64 = svarg1_0_i64 as i64;
            loop {
                {
                    arg1 = b;
                    mstore64!(arg1.wrapping_add(32) as usize, svarg1_0_i64 as u64);
                    let h = mload64!(arg1.wrapping_add(24) as usize) as i64;
                    mstore64!(arg1.wrapping_add(56) as usize, h as u64);
                    let i = mload64!(arg1.wrapping_add(16) as usize) as i64;
                    mstore64!(arg1.wrapping_add(48) as usize, i as u64);
                    let j = mload64!(arg1.wrapping_add(8) as usize) as i64;
                    mstore64!(arg1.wrapping_add(40) as usize, j as u64);
                    if arg0 != arg1 {
                        b = arg1.wrapping_sub(32);
                        let k = self.memcmp_sign32(env, a, b);
                    }
                }
            }
            let mut sv2_0_i64 = mload64!(a as usize) as i64;
            let mut svarg1_0_i64 = sv2_0_i64 as i64;
            let l = mload64!(a.wrapping_add(24) as usize) as i64;
            mstore64!(arg1.wrapping_add(24) as usize, l as u64);
            let m = mload64!(a.wrapping_add(16) as usize) as i64;
            mstore64!(arg1.wrapping_add(16) as usize, m as u64);
            let n = mload64!(a.wrapping_add(8) as usize) as i64;
            mstore64!(arg1.wrapping_add(8) as usize, n as u64);
        }
        self.global0 = a.wrapping_add(32);
    }

    fn ptr_index32(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> i32 {
        if arg1 as u32 <= arg2 as u32 {
            unreachable!();
        }
        arg0.wrapping_add(arg2.wrapping_shl(5 as u32))
    }

    fn func76(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = self.global0.wrapping_sub(128);
        self.global0 = a;
        c = a.wrapping_sub(-64);
        self.memcpy_like_7(env, c, arg1);
        b = a.wrapping_add(96);
        self.memcpy_like_7(env, b, arg2);
        self.memcpy_like_8(
            env,
            a,
            c,
            b,
        );
        mstore64!(a.wrapping_add(87) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(80) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(72) as usize, 0 /* False */ as u64);
        let e = mload8!(arg1 as usize + 31) as i32;
        mstore8!(a as usize + 95, (e & 1) as u8);
        mstore64!(a.wrapping_add(119) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(112) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(104) as usize, 0 /* False */ as u64);
        let f = mload8!(arg2 as usize + 31) as i32;
        mstore8!(a as usize + 127, (f & 1) as u8);
        arg1 = a.wrapping_add(32);
        self.memcpy_like_8(
            env,
            arg1,
            c,
            b,
        );
        self.memcpy_like_7(env, b, arg1);
        self.memcpy_like_8(
            env,
            arg0,
            a,
            b,
        );
        self.global0 = a.wrapping_add(128);
    }

    fn entry_match_copy(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let mut a: i32 = 0;
        {
            {
                let b = self.func121(env, arg2, arg1);
                if b != 0 {
                    let c = self.func122(env, arg2, arg3);
                    if c != 0 {
                        unreachable!();
                    }
                }
                let d = self.func121(env, arg2, arg3);
                if d == 0 {
                    unreachable!();
                }
                let e = self.func122(env, arg2, arg1);
                if e == 0 {
                    unreachable!();
                }
            }
            let f = mload64!(arg2.wrapping_add(24) as usize) as i64;
            mstore64!(arg0.wrapping_add(25) as usize, f as u64);
            let g = mload64!(arg2.wrapping_add(16) as usize) as i64;
            mstore64!(arg0.wrapping_add(17) as usize, g as u64);
            let h = mload64!(arg2.wrapping_add(8) as usize) as i64;
            mstore64!(arg0.wrapping_add(9) as usize, h as u64);
            a = 1;
        }
        mstore8!(arg0 as usize, a as u8);
    }


    fn bytes_to_fixed32(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = mload32!(arg1 as usize + 8) as i32;
        if a as u32 <= 32 as u32 {
            let e = mload32!(arg1.wrapping_add(8) as usize) as i32;
            mstore32!(arg0.wrapping_add(8) as usize, e as u32);
        }
        b = mload32!(arg1 as usize + 4) as i32;
        c = a.wrapping_sub(32);
        a = c;
        loop {
            if a == 0 {
                unreachable!();
            }
            a -= 1;
            let f = mload8!(b as usize) as i32;
            b += 1;
        }
        unreachable!();
        self.func125(
            env,
            arg0,
            arg1,
            c,
        );
    }

    fn span_set(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        if arg2 as u32 >= 33 as u32 {
            unreachable!();
        }
    }

    fn memcpy_checked(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        if arg1 != arg3 {
            unreachable!();
        }
        let a = self.memmove(
            env,
            arg0,
            arg2,
            arg1,
        );
    }

    fn bytes_to_fixed32_struct(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = self.global0.wrapping_add(-64);
        self.global0 = a;
        self.bytes_to_fixed32(env, a.wrapping_add(20), arg1);
        b = a.wrapping_add(56);
        let mut sv3_0_i64 = 0 /* False */ as i64;
        c = a.wrapping_add(48);
        let mut sv4_0_i64 = 0 /* False */ as i64;
        d = a.wrapping_add(40);
        let mut sv5_0_i64 = 0 /* False */ as i64;
        let mut sv2_32_i64 = 0 /* False */ as i64;
        arg1 = mload32!(a as usize + 28) as i32;
        if arg1 as u32 <= 32 as u32 {
            self.func123(
                env,
                a.wrapping_add(8),
                32.wrapping_sub(arg1),
                a.wrapping_add(32),
                32,
            );
            let mut sv2_8_i32 = mload32!(a as usize + 8) as i32;
            let mut sv2_12_i32 = mload32!(a as usize + 12) as i32;
            let mut sv2_24_i32 = mload32!(a as usize + 24) as i32;
            self.memcpy_checked(
                env,
                sv2_8_i32,
                sv2_12_i32,
                sv2_24_i32,
                arg1,
            );
            let mut sv3_0_i64 = mload64!(b as usize) as i64;
            mstore64!(arg0.wrapping_add(24) as usize, sv3_0_i64 as u64);
            let mut sv4_0_i64 = mload64!(c as usize) as i64;
            mstore64!(arg0.wrapping_add(16) as usize, sv4_0_i64 as u64);
            let mut sv5_0_i64 = mload64!(d as usize) as i64;
            mstore64!(arg0.wrapping_add(8) as usize, sv5_0_i64 as u64);
            let mut sv2_32_i64 = mload64!(a as usize + 32) as i64;
            self.global0 = a.wrapping_sub(-64);
        }
    }








    fn write_ok_val(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let a: i64;
        if arg1 as u64 <= 72057594037927935 as u64 {
            a = arg1.wrapping_shl(0 as u32) | 0;
        } else {
            let b = val_to_i64(Val::from_u64(arg1 as u64));
        }
    }


    fn map_new_val(
        &mut self,
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


    fn check_recent_timestamp(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        let f: i64;
        {
            c = mload64!(arg1 as usize + 16) as i64;
            b = c.wrapping_add(108000000);
            b = (if ((b as u64) < c as u64) as i32 != 0 { 18446744073709551615 } else { b });
            let g = self.ledger_timestamp_val(env);
            d = g;
            if b as u64 > d as u64 {
                let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                let mut svarg0_8_i64 = svarg1_0_i64 as i64;
                let h = mload64!(arg1.wrapping_add(16) as usize) as i64;
                mstore64!(arg0.wrapping_add(24) as usize, h as u64);
                let i = mload64!(arg1.wrapping_add(8) as usize) as i64;
                mstore64!(arg0.wrapping_add(16) as usize, i as u64);
            } else {
                mstore8!(a as usize, 25 as u8);
                let j = self.decode_error_from_val(env, a);
                let mut svarg0_8_i64 = j as i64;
            }
        }
        self.global0 = a.wrapping_add(48);
    }






    fn val_to_i64_checked(
        &mut self,
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
                c = (arg1 as u64).wrapping_shr(0 as u32) as i64;
                break 'label0;
            }
            let d = val_from_i64(arg1).as_u64().unwrap_or(0) as i64;
        }
    }

    fn func101(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> i32 {
        if arg2 != 1114112 {
            let mut svarg1_16_i32 = mload32!(arg1 as usize + 16) as i32;
            let a = { let _ = (svarg1_16_i32, arg0, arg2); unimplemented!("call_indirect type 1") };
            if a != 0 {
                return 1;
            }
        }
        if arg3 == 0 {
            return 0;
        }
        let mut svarg1_12_i32 = mload32!(arg1 as usize + 12) as i32;
        let b = { let _ = (svarg1_12_i32, arg0, arg3, 0); unimplemented!("call_indirect type 0") };
        b
    }

    fn func102(
        &mut self,
        env: &Env,
        mut arg0: i32,
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
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        'label0: {
            'label1: {
                'label2: {
                    c = mload32!(arg0 as usize) as i32;
                    b = mload32!(arg0 as usize + 8) as i32;
                    if c | b != 0 {
                        'label3: {
                            if b & 1 != 0 {
                                d = arg1.wrapping_add(arg2);
                                g = mload32!(arg0 as usize + 12) as i32;
                                if g == 0 {
                                    b = arg1;
                                } else {
                                    b = arg1;
                                    loop {
                                        a = b;
                                        if a == d {
                                            break 'label3;
                                        }
                                        let l: i32;
                                        {
                                            let m = mload8!(a as usize) as i8 as i32;
                                            b = m;
                                            let n = a += 1;
                                            if b >= 0 {
                                                l = n;
                                            } else {
                                                let o = a.wrapping_add(2);
                                                if (b as u32) < -32 as u32 {
                                                    l = o;
                                                } else {
                                                    let p = a.wrapping_add(3);
                                                    if (b as u32) < -16 as u32 {
                                                        l = p;
                                                    } else {
                                                        l = a.wrapping_add(4);
                                                    }
                                                }
                                            }
                                        }
                                        b = l;
                                        f = b.wrapping_sub(a).wrapping_add(f);
                                        e += 1;
                                    }
                                }
                                if b == d {
                                    break 'label3;
                                }
                                let q = mload8!(b as usize) as i8 as i32;
                                let r: i32;
                                'label7: {
                                    'label8: {
                                        if f != 0 {
                                            if arg2 as u32 <= f as u32 {
                                                if arg2 == f {
                                                    break 'label8;
                                                }
                                                r = 0;
                                                break 'label7;
                                            }
                                            let s = mload8!(arg1.wrapping_add(f) as usize) as i8 as i32;
                                            if s >= -64 {
                                                break 'label8;
                                            }
                                            r = 0;
                                            break 'label7;
                                        }
                                    }
                                    r = arg1;
                                }
                                b = r;
                                arg2 = (if b != 0 { f } else { arg2 });
                                arg1 = (if b != 0 { b } else { arg1 });
                            }
                        }
                        if c == 0 {
                            unreachable!();
                        }
                        i = mload32!(arg0 as usize + 4) as i32;
                        if arg2 as u32 >= 16 as u32 {
                            f = arg1.wrapping_add(3) & -4;
                            e = arg1.wrapping_sub(f);
                            h = arg2.wrapping_add(e);
                            g = h & 3;
                            c = 0;
                            a = 0;
                            if arg1 != f {
                                if e as u32 <= -4 as u32 {
                                    d = 0;
                                    loop {
                                        b = arg1.wrapping_add(d);
                                        let t = mload8!(b as usize) as i8 as i32;
                                        let u = mload8!(b += 1 as usize) as i8 as i32;
                                        let v = mload8!(b.wrapping_add(2) as usize) as i8 as i32;
                                        let w = mload8!(b.wrapping_add(3) as usize) as i8 as i32;
                                        a = a.wrapping_add((t > -65) as i32).wrapping_add((u > -65) as i32).wrapping_add((v > -65) as i32).wrapping_add((w > -65) as i32);
                                        d = d.wrapping_add(4);
                                    }
                                }
                                b = arg1;
                                loop {
                                    let x = mload8!(b as usize) as i8 as i32;
                                    a = a.wrapping_add((x > -65) as i32);
                                    b += 1;
                                    e += 1;
                                }
                            }
                            if g != 0 {
                                b = f.wrapping_add(h & -4);
                                let y = mload8!(b as usize) as i8 as i32;
                                c = (y > -65) as i32;
                                if g != 1 {
                                    let z = mload8!(b as usize + 1) as i8 as i32;
                                    c = c.wrapping_add((z > -65) as i32);
                                    if g != 2 {
                                        let aa = mload8!(b as usize + 2) as i8 as i32;
                                        c = c.wrapping_add((aa > -65) as i32);
                                    }
                                }
                            }
                            d = (h as u32).wrapping_shr(2 as u32) as i32;
                            c = a.wrapping_add(c);
                            'label12: loop {
                                e = f;
                                if d == 0 {
                                    unreachable!();
                                }
                                g = (if (d as u32 >= 192 as u32) as i32 != 0 { 192 } else { d });
                                h = g & 3;
                                f = g.wrapping_shl(2 as u32);
                                b = 0;
                                if d as u32 >= 4 as u32 {
                                    j = e.wrapping_add(f & 1008);
                                    a = e;
                                    loop {
                                        let ab = b;
                                        k = mload32!(a as usize) as i32;
                                        b = mload32!(a as usize + 4) as i32;
                                        b = mload32!(a as usize + 8) as i32;
                                        b = mload32!(a as usize + 12) as i32;
                                        b = ab.wrapping_add((((k ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (k as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add((((b ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (b as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add((((b ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (b as u32).wrapping_shr(6 as u32) as i32) & 16843009).wrapping_add((((b ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (b as u32).wrapping_shr(6 as u32) as i32) & 16843009);
                                        a = a.wrapping_add(16);
                                    }
                                }
                                d = d.wrapping_sub(g);
                                f = e.wrapping_add(f);
                                c = ((((b as u32).wrapping_shr(8 as u32) as i32 & 16711935).wrapping_add(b & 16711935).wrapping_mul(65537) as u32).wrapping_shr(16 as u32) as i32).wrapping_add(c);
                                if h == 0 {
                                    continue 'label12;
                                }
                            }
                            b = e.wrapping_add((g & 252).wrapping_shl(2 as u32));
                            a = mload32!(b as usize) as i32;
                            a = (((a ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (a as u32).wrapping_shr(6 as u32) as i32) & 16843009;
                            if h == 1 {
                                break 'label2;
                            }
                            e = mload32!(b as usize + 4) as i32;
                            a = a.wrapping_add((((e ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (e as u32).wrapping_shr(6 as u32) as i32) & 16843009);
                            if h == 2 {
                                break 'label2;
                            }
                            b = mload32!(b as usize + 8) as i32;
                            a = a.wrapping_add((((b ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (b as u32).wrapping_shr(6 as u32) as i32) & 16843009);
                            break 'label2;
                        }
                        if arg2 == 0 {
                            c = 0;
                            break 'label1;
                        }
                        b = arg2 & 3;
                        if (arg2 as u32) < 4 as u32 {
                            c = 0;
                            e = 0;
                        } else {
                            c = 0;
                            a = arg1;
                            e = arg2 & 12;
                            f = e;
                            loop {
                                let ac = mload8!(a as usize) as i8 as i32;
                                let ad = mload8!(a += 1 as usize) as i8 as i32;
                                let ae = mload8!(a.wrapping_add(2) as usize) as i8 as i32;
                                let af = mload8!(a.wrapping_add(3) as usize) as i8 as i32;
                                c = c.wrapping_add((ac > -65) as i32).wrapping_add((ad > -65) as i32).wrapping_add((ae > -65) as i32).wrapping_add((af > -65) as i32);
                                a = a.wrapping_add(4);
                                f = f.wrapping_sub(4);
                            }
                        }
                        if b == 0 {
                            unreachable!();
                        }
                        a = arg1.wrapping_add(e);
                        loop {
                            let ag = mload8!(a as usize) as i8 as i32;
                            c = c.wrapping_add((ag > -65) as i32);
                            a += 1;
                            b -= 1;
                        }
                        break 'label1;
                    }
                    break 'label0;
                }
                c = ((((a as u32).wrapping_shr(8 as u32) as i32 & 459007).wrapping_add(a & 16711935).wrapping_mul(65537) as u32).wrapping_shr(16 as u32) as i32).wrapping_add(c);
            }
            if (c as u32) < i as u32 {
                d = i.wrapping_sub(c);
                'label18: {
                    {
                        let ah = mload8!(arg0 as usize + 24) as i32;
                        b = ah;
                        a = (if (b != 3) as i32 != 0 { b } else { 0 });
                        loop {
                            match a.wrapping_sub(1) {
                                0 | 1 => break,
                                _ => break 'label18,
                            }
                        }
                    }
                    a = d;
                    d = 0;
                    break 'label18;
                    a = (d as u32).wrapping_shr(1 as u32) as i32;
                    d = (d += 1 as u32).wrapping_shr(1 as u32) as i32;
                }
                a += 1;
                e = mload32!(arg0 as usize + 16) as i32;
                b = mload32!(arg0 as usize + 32) as i32;
                arg0 = mload32!(arg0 as usize + 28) as i32;
                loop {
                    a -= 1;
                    if a == 0 {
                        unreachable!();
                    }
                    let mut sv4_16_i32 = mload32!(b as usize + 16) as i32;
                    let ai = { let _ = (sv4_16_i32, arg0, e); unimplemented!("call_indirect type 1") };
                }
                return 1;
            }
            break 'label0;
            let mut sv4_12_i32 = mload32!(b as usize + 12) as i32;
            let aj = { let _ = (sv4_12_i32, arg0, arg1, arg2); unimplemented!("call_indirect type 0") };
            if aj != 0 {
                return 1;
            }
            a = 0;
            loop {
                if a == d {
                    return 0;
                }
                a += 1;
                let mut sv4_16_i32 = mload32!(b as usize + 16) as i32;
                let ak = { let _ = (sv4_16_i32, arg0, e); unimplemented!("call_indirect type 1") };
            }
            return ((a.wrapping_sub(1) as u32) < d as u32) as i32;
        }
        let mut svarg0_28_i32 = mload32!(arg0 as usize + 28) as i32;
        let mut svarg0_32_i32 = mload32!(arg0 as usize + 32) as i32;
        let mut svsvarg0_32_i32_12_i32 = mload32!(svarg0_32_i32 as usize + 12) as i32;
        let al = { let _ = (svsvarg0_32_i32_12_i32, svarg0_28_i32, arg1, arg2); unimplemented!("call_indirect type 0") };
        al
    }

    fn memcpy_like_2(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        let mut sv3_44_i32 = arg1 as i32;
        let mut sv3_40_i32 = arg0 as i32;
        mstore8!(a as usize + 36, 3 as u8);
        let mut sv3_20_i32 = 0 as i32;
        let mut sv3_12_i32 = 0 as i32;
        let l: i32;
        'label0: {
            'label1: {
                {
                    h = mload32!(arg2 as usize + 16) as i32;
                    if h == 0 {
                        arg0 = mload32!(arg2 as usize + 12) as i32;
                        if arg0 == 0 {
                            unreachable!();
                        }
                        arg1 = mload32!(arg2 as usize + 8) as i32;
                        b = arg1.wrapping_add(arg0.wrapping_shl(3 as u32));
                        e = (arg0 -= 1 & 536870911) += 1;
                        arg0 = mload32!(arg2 as usize) as i32;
                        loop {
                            c = mload32!(arg0.wrapping_add(4) as usize) as i32;
                            if c == 0 {
                                break;
                            }
                            if c != 0 {
                                let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
                                let mut svsv3_44_i32_12_i32 = mload32!(sv3_44_i32 as usize + 12) as i32;
                                let m = { let _ = (svsv3_44_i32_12_i32, sv3_40_i32, svarg0_0_i32, c); unimplemented!("call_indirect type 0") };
                                if m != 0 {
                                    unreachable!();
                                }
                            }
                            let mut svarg1_0_i32 = mload32!(arg1 as usize) as i32;
                            let n = mload32!(arg1.wrapping_add(4) as usize) as i32;
                            let o = { let _ = (n, svarg1_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 1") };
                            if o != 0 {
                                unreachable!();
                            }
                            arg0 = arg0.wrapping_add(8);
                            arg1 = arg1.wrapping_add(8);
                        }
                    } else {
                        arg0 = mload32!(arg2 as usize + 20) as i32;
                        if arg0 == 0 {
                            unreachable!();
                        }
                        i = arg0.wrapping_shl(5 as u32);
                        e = (arg0 -= 1 & 134217727) += 1;
                        c = mload32!(arg2 as usize + 8) as i32;
                        arg0 = mload32!(arg2 as usize) as i32;
                        loop {
                            arg1 = mload32!(arg0.wrapping_add(4) as usize) as i32;
                            if arg1 == 0 {
                                break;
                            }
                            if arg1 != 0 {
                                let mut sv3_40_i32 = mload32!(a as usize + 40) as i32;
                                let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
                                let mut sv3_44_i32 = mload32!(a as usize + 44) as i32;
                                let mut svsv3_44_i32_12_i32 = mload32!(sv3_44_i32 as usize + 12) as i32;
                                let p = { let _ = (svsv3_44_i32_12_i32, sv3_40_i32, svarg0_0_i32, arg1); unimplemented!("call_indirect type 0") };
                                if p != 0 {
                                    unreachable!();
                                }
                            }
                            arg1 = f.wrapping_add(h);
                            let q = mload32!(arg1.wrapping_add(16) as usize) as i32;
                            let r = mload8!(arg1.wrapping_add(28) as usize) as i32;
                            mstore8!(a as usize + 36, r as u8);
                            let s = mload32!(arg1.wrapping_add(24) as usize) as i32;
                            b = mload32!(arg1.wrapping_add(12) as usize) as i32;
                            g = 0;
                            d = 0;
                            {
                                {
                                    let t = mload32!(arg1.wrapping_add(8) as usize) as i32;
                                    match t.wrapping_sub(1) {
                                        0 | 1 | _ => break,
                                    }
                                }
                                j = b.wrapping_shl(3 as u32).wrapping_add(c);
                                let mut sv12_0_i32 = mload32!(j as usize) as i32;
                                if sv12_0_i32 != 0 {
                                    unreachable!();
                                }
                                b = mload32!(j as usize + 4) as i32;
                            }
                            d = 1;
                            let mut sv3_12_i32 = d as i32;
                            b = mload32!(arg1.wrapping_add(4) as usize) as i32;
                            {
                                {
                                    let mut svarg1_0_i32 = mload32!(arg1 as usize) as i32;
                                    match svarg1_0_i32.wrapping_sub(1) {
                                        0 | 1 | _ => break,
                                    }
                                }
                                d = b.wrapping_shl(3 as u32).wrapping_add(c);
                                let mut sv6_0_i32 = mload32!(d as usize) as i32;
                                if sv6_0_i32 != 0 {
                                    unreachable!();
                                }
                                b = mload32!(d as usize + 4) as i32;
                            }
                            g = 1;
                            let mut sv3_20_i32 = g as i32;
                            let u = mload32!(arg1.wrapping_add(20) as usize) as i32;
                            arg1 = c.wrapping_add(u.wrapping_shl(3 as u32));
                            let v = mload32!(arg1.wrapping_add(4) as usize) as i32;
                            let w = { let _ = (v, svarg1_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 1") };
                            if w != 0 {
                                unreachable!();
                            }
                            arg0 = arg0.wrapping_add(8);
                            f = f.wrapping_add(32);
                        }
                    }
                    let mut svarg2_4_i32 = mload32!(arg2 as usize + 4) as i32;
                    if e as u32 >= svarg2_4_i32 as u32 {
                        break 'label1;
                    }
                    let mut sv3_40_i32 = mload32!(a as usize + 40) as i32;
                    let mut svarg2_0_i32 = mload32!(arg2 as usize) as i32;
                    arg0 = svarg2_0_i32.wrapping_add(e.wrapping_shl(3 as u32));
                    let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
                    let mut svarg0_4_i32 = mload32!(arg0 as usize + 4) as i32;
                    let mut sv3_44_i32 = mload32!(a as usize + 44) as i32;
                    let mut svsv3_44_i32_12_i32 = mload32!(sv3_44_i32 as usize + 12) as i32;
                    let x = { let _ = (svsv3_44_i32_12_i32, sv3_40_i32, svarg0_0_i32, svarg0_4_i32); unimplemented!("call_indirect type 0") };
                    if x == 0 {
                        unreachable!();
                    }
                }
                l = 1;
                break 'label0;
            }
            l = 0;
        }
        self.global0 = a.wrapping_add(48);
        l
    }

    fn memcpy_like_3(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        k = arg1 -= 1;
        h = mload32!(arg0 as usize + 4) as i32;
        i = mload32!(arg0 as usize) as i32;
        j = mload32!(arg0 as usize + 8) as i32;
        'label0: {
            'label1: loop {
                if d != 0 {
                    unreachable!();
                }
                let m: i32;
                'label2: {
                    'label3: {
                        if (arg2 as u32) >= b as u32 {
                            'label4: loop {
                                d = arg1.wrapping_add(b);
                                'label5: {
                                    'label6: {
                                        'label7: {
                                            e = arg2.wrapping_sub(b);
                                            if e as u32 <= 7 as u32 {
                                                if arg2 != b {
                                                    break 'label7;
                                                }
                                                b = arg2;
                                                break 'label3;
                                            }
                                            'label8: {
                                                c = d.wrapping_add(3) & -4;
                                                a = c.wrapping_sub(d);
                                                if a != 0 {
                                                    arg0 = 0;
                                                    loop {
                                                        let n = mload8!(arg0.wrapping_add(d) as usize) as i32;
                                                        if n == 10 {
                                                            break 'label5;
                                                        }
                                                        arg0 += 1;
                                                    }
                                                    arg0 = e.wrapping_sub(8);
                                                    if a as u32 <= arg0 as u32 {
                                                        break 'label8;
                                                    }
                                                    break 'label6;
                                                }
                                                arg0 = e.wrapping_sub(8);
                                            }
                                            loop {
                                                g = mload32!(c as usize) as i32;
                                                let o = g;
                                                let p = g;
                                                g = mload32!(c.wrapping_add(4) as usize) as i32;
                                                if (16843008.wrapping_sub(o ^ 168430090) | p) & (16843008.wrapping_sub(g ^ 168430090) | g) & -2139062144 != -2139062144 {
                                                    break 'label6;
                                                }
                                                c = c.wrapping_add(8);
                                                a = a.wrapping_add(8);
                                            }
                                            break 'label6;
                                        }
                                        arg0 = 0;
                                        loop {
                                            let q = mload8!(arg0.wrapping_add(d) as usize) as i32;
                                            if q == 10 {
                                                break 'label5;
                                            }
                                            arg0 += 1;
                                        }
                                        b = arg2;
                                        break 'label3;
                                    }
                                    if a == e {
                                        b = arg2;
                                        break 'label3;
                                    }
                                    c = a.wrapping_add(d);
                                    e = arg2.wrapping_sub(a).wrapping_sub(b);
                                    arg0 = 0;
                                    'label12: {
                                        loop {
                                            let r = mload8!(arg0.wrapping_add(c) as usize) as i32;
                                            if r == 10 {
                                                break 'label12;
                                            }
                                            arg0 += 1;
                                        }
                                        b = arg2;
                                        break 'label3;
                                    }
                                    arg0 = arg0.wrapping_add(a);
                                }
                                a = arg0.wrapping_add(b);
                                b = a += 1;
                                if arg2 as u32 > a as u32 {
                                    let s = mload8!(arg0.wrapping_add(d) as usize) as i32;
                                    if s == 10 {
                                        a = b;
                                        m = a;
                                        break 'label2;
                                    }
                                }
                                if arg2 as u32 >= b as u32 {
                                    continue 'label4;
                                }
                            }
                        }
                    }
                    if arg2 == f {
                        break 'label0;
                    }
                    m = arg2;
                }
                arg0 = m;
                {
                    let t = mload8!(j as usize) as i32;
                    if t != 0 {
                        let mut sv10_12_i32 = mload32!(h as usize + 12) as i32;
                        let u = { let _ = (sv10_12_i32, i, 1049032, 4); unimplemented!("call_indirect type 0") };
                        if u != 0 {
                            unreachable!();
                        }
                    }
                    c = 0;
                    if arg0 != f {
                        let v = mload8!(arg0.wrapping_add(k) as usize) as i32;
                        c = (v == 10) as i32;
                    }
                    arg0 = arg0.wrapping_sub(f);
                    e = arg1.wrapping_add(f);
                    mstore8!(j as usize, c as u8);
                    let mut sv10_12_i32 = mload32!(h as usize + 12) as i32;
                    let w = { let _ = (sv10_12_i32, i, e, arg0); unimplemented!("call_indirect type 0") };
                    if w == 0 {
                        continue 'label1;
                    }
                }
            }
            l = 1;
        }
        l
    }




    fn func108(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        if arg2 != 0 {
            let a = mload8!(1049928 as usize) as i32;
            let b = self.func111(env, arg2, arg1);
        }
    }

    fn memcpy_like_5(
        &mut self,
        env: &Env,
    ) {
        let mut a: i32 = 0;
        'label0: {
            let b = mload32!(1049936 as usize) as i32;
            if b == 0 {
                let c = msize!();
                a = c;
                if a as u32 > 65535 as u32 {
                    break 'label0;
                }
                a = a.wrapping_shl(16 as u32);
                mstore32!(1049936 as usize, a as u32);
                mstore32!(1049932 as usize, a as u32);
            }
        }
    }

    fn func110(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        d = 0.wrapping_sub(arg1);
        arg1 -= 1;
        a = arg0.wrapping_add(65535);
        e = a & -65536;
        a = (a as u32).wrapping_shr(16 as u32) as i32;
        'label0: {
            loop {
                let f = mgrow!(a as usize);
                if f == -1 {
                    unreachable!();
                }
                let g = mload32!(1049936 as usize) as i32;
                mstore32!(1049936 as usize, g.wrapping_add(e) as u32);
                self.memcpy_like_5(env);
                b = mload32!(1049932 as usize) as i32;
                c = b.wrapping_add(arg1);
                if (c as u32) < b as u32 {
                    break 'label0;
                }
                b = c & d;
                c = b.wrapping_add(arg0);
                let h = mload32!(1049936 as usize) as i32;
            }
            mstore32!(1049932 as usize, c as u32);
            return b;
        }
    }

    fn func111(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        self.memcpy_like_5(env);
        a = mload32!(1049932 as usize) as i32;
        b = arg1.wrapping_add(a) -= 1;
        if b as u32 >= a as u32 {
            a = b & 0.wrapping_sub(arg1);
            b = a.wrapping_add(arg0);
            let c = mload32!(1049936 as usize) as i32;
            if b as u32 > c as u32 {
                let d = self.func110(env, arg0, arg1);
                return d;
            }
            mstore32!(1049932 as usize, b as u32);
            return a;
        }
    }

    fn func112(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
        mut arg3: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        if arg3 & 536870904 != 0 {
            arg3 = (arg3 as u32).wrapping_shr(3 as u32) as i32;
            a = arg3.wrapping_shl(7 as u32);
            b = arg3.wrapping_mul(224);
            let c = self.func112(
                env,
                arg0,
                arg0.wrapping_add(a),
                arg0.wrapping_add(b),
                arg3,
            );
            arg0 = c;
            let d = self.func112(
                env,
                arg1,
                arg1.wrapping_add(a),
                arg1.wrapping_add(b),
                arg3,
            );
            arg1 = d;
            let e = self.func112(
                env,
                arg2,
                arg2.wrapping_add(a),
                arg2.wrapping_add(b),
                arg3,
            );
            arg2 = e;
        }
        let f = self.memcmp_sign32(env, arg0, arg1);
        arg3 = (f & 255 == 255) as i32;
        let g = self.memcmp_sign32(env, arg0, arg2);
        let h: i32;
        if arg3 ^ g & 255 == 255 {
            h = arg0;
        } else {
            let i = self.memcmp_sign32(env, arg1, arg2);
            h = (if arg3 ^ (i & 255 == 255) as i32 != 0 { arg2 } else { arg1 });
        }
        h
    }

    fn func113(
        &mut self,
        env: &Env,
        mut arg0: i32,
        arg1: i32,
    ) {
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let k = self.memcmp_sign32(env, arg0.wrapping_add(32), arg0);
        d = k;
        let l = self.memcmp_sign32(env, arg0.wrapping_add(96), arg0.wrapping_sub(-64));
        e = (l & 255 == 255) as i32;
        c = arg0.wrapping_add((if e != 0 { 96 } else { 64 }));
        e = arg0.wrapping_add((if e != 0 { 64 } else { 96 }));
        f = d & 255;
        d = arg0.wrapping_add(((f != 255) as i32).wrapping_shl(5 as u32));
        arg0 = arg0.wrapping_add(((f == 255) as i32).wrapping_shl(5 as u32));
        let m = self.memcmp_sign32(env, c, arg0);
        f = (m & 255 == 255) as i32;
        let n = self.memcmp_sign32(env, e, d);
        g = (n & 255 == 255) as i32;
        h = { let a = e; let b = { let a = d; let b = c; if f != 0 { a } else { b } }; if g != 0 { a } else { b } };
        i = { let a = arg0; let b = { let a = c; let b = d; if g != 0 { a } else { b } }; if f != 0 { a } else { b } };
        let o = self.memcmp_sign32(env, h, i);
        j = o;
        arg0 = (if f != 0 { c } else { arg0 });
        let p = mload64!(arg0.wrapping_add(24) as usize) as i64;
        mstore64!(arg1.wrapping_add(24) as usize, p as u64);
        let q = mload64!(arg0.wrapping_add(16) as usize) as i64;
        mstore64!(arg1.wrapping_add(16) as usize, q as u64);
        let r = mload64!(arg0.wrapping_add(8) as usize) as i64;
        mstore64!(arg1.wrapping_add(8) as usize, r as u64);
        c = (j & 255 == 255) as i32;
        arg0 = (if c != 0 { h } else { i });
        let s = mload64!(arg0.wrapping_add(24) as usize) as i64;
        mstore64!(arg1.wrapping_add(56) as usize, s as u64);
        let t = mload64!(arg0.wrapping_add(16) as usize) as i64;
        mstore64!(arg1.wrapping_add(48) as usize, t as u64);
        let u = mload64!(arg0.wrapping_add(8) as usize) as i64;
        mstore64!(arg1.wrapping_add(40) as usize, u as u64);
        arg0 = (if c != 0 { i } else { h });
        let v = mload64!(arg0.wrapping_add(24) as usize) as i64;
        mstore64!(arg1.wrapping_add(88) as usize, v as u64);
        let w = mload64!(arg0.wrapping_add(16) as usize) as i64;
        mstore64!(arg1.wrapping_add(80) as usize, w as u64);
        let x = mload64!(arg0.wrapping_add(8) as usize) as i64;
        mstore64!(arg1.wrapping_add(72) as usize, x as u64);
        arg0 = (if g != 0 { d } else { e });
        let y = mload64!(arg0.wrapping_add(8) as usize) as i64;
        mstore64!(arg1.wrapping_add(104) as usize, y as u64);
        let z = mload64!(arg0.wrapping_add(16) as usize) as i64;
        mstore64!(arg1.wrapping_add(112) as usize, z as u64);
        let aa = mload64!(arg0.wrapping_add(24) as usize) as i64;
        mstore64!(arg1.wrapping_add(120) as usize, aa as u64);
    }

    fn func114(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
        mut arg3: i32,
        mut arg4: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i64 = 0;
        let mut m: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        b = self.global0.wrapping_sub(192);
        self.global0 = b;
        'label0: loop {
            'label1: {
                'label2: {
                    'label3: {
                        'label4: {
                            if arg1 as u32 >= 33 as u32 {
                                if arg3 == 0 {
                                    arg2 = arg0;
                                    a = self.global0.wrapping_sub(48);
                                    self.global0 = a;
                                    arg3 = arg1;
                                    arg4 = arg3.wrapping_add((arg1 as u32).wrapping_shr(1 as u32) as i32);
                                    'label5: loop {
                                        if arg4 != 0 {
                                            let s: i32;
                                            arg4 -= 1;
                                            if arg3 as u32 > arg4 as u32 {
                                                arg0 = arg2.wrapping_add(arg4.wrapping_shl(5 as u32));
                                                arg1 = arg0.wrapping_add(8);
                                                let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                                                n = svarg1_0_i64;
                                                c = arg0.wrapping_add(16);
                                                let mut sv7_0_i64 = mload64!(c as usize) as i64;
                                                l = sv7_0_i64;
                                                e = arg0.wrapping_add(24);
                                                let mut sv9_0_i64 = mload64!(e as usize) as i64;
                                                m = sv9_0_i64;
                                                let mut svarg2_0_i64 = mload64!(arg2 as usize) as i64;
                                                o = svarg2_0_i64;
                                                let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                                                svarg2_0_i64 = svarg0_0_i64 as i64;
                                                f = arg2.wrapping_add(24);
                                                let mut sv10_0_i64 = mload64!(f as usize) as i64;
                                                p = sv10_0_i64;
                                                sv10_0_i64 = m as i64;
                                                f = arg2.wrapping_add(16);
                                                m = sv10_0_i64;
                                                sv10_0_i64 = l as i64;
                                                f = arg2.wrapping_add(8);
                                                l = sv10_0_i64;
                                                sv10_0_i64 = n as i64;
                                                sv9_0_i64 = p as i64;
                                                sv7_0_i64 = m as i64;
                                                svarg1_0_i64 = l as i64;
                                                svarg0_0_i64 = o as i64;
                                                s = 0;
                                            } else {
                                                s = arg4.wrapping_sub(arg3);
                                            }
                                            arg1 = s;
                                            self.func115(
                                                env,
                                                a.wrapping_add(8),
                                                (if (arg3 as u32 > arg4 as u32) as i32 != 0 { arg4 } else { arg3 }),
                                                arg2,
                                                arg3,
                                            );
                                            e = mload32!(a as usize + 12) as i32;
                                            c = mload32!(a as usize + 8) as i32;
                                            loop {
                                                f = arg1.wrapping_shl(1 as u32);
                                                arg0 = f | 1;
                                                if arg0 as u32 >= e as u32 {
                                                    continue 'label5;
                                                }
                                                f = f.wrapping_add(2);
                                                if e as u32 > f as u32 {
                                                    let t = self.memcmp_sign32(env, c.wrapping_add(arg0.wrapping_shl(5 as u32)), c.wrapping_add(f.wrapping_shl(5 as u32)));
                                                    arg0 = arg0.wrapping_add((t & 255 == 255) as i32);
                                                }
                                                arg1 = c.wrapping_add(arg1.wrapping_shl(5 as u32));
                                                f = c.wrapping_add(arg0.wrapping_shl(5 as u32));
                                                let u = self.memcmp_sign32(env, arg1, f);
                                                if u & 255 != 255 {
                                                    continue 'label5;
                                                }
                                                self.memcpy_like_6(env, arg1, f);
                                                arg1 = arg0;
                                            }
                                            unreachable!();
                                        }
                                    }
                                    self.global0 = a.wrapping_add(48);
                                    break 'label4;
                                }
                                e = (arg1 as u32).wrapping_shr(3 as u32) as i32;
                                a = arg0.wrapping_add(e.wrapping_mul(224));
                                c = arg0.wrapping_add(e.wrapping_shl(7 as u32));
                                arg3 -= 1;
                                let v: i32;
                                if arg1 as u32 >= 64 as u32 {
                                    let w = self.func112(
                                        env,
                                        arg0,
                                        c,
                                        a,
                                        e,
                                    );
                                    v = w;
                                } else {
                                    let x = self.memcmp_sign32(env, arg0, c);
                                    e = (x & 255 == 255) as i32;
                                    let y = self.memcmp_sign32(env, arg0, a);
                                    let z = arg0;
                                    if e ^ y & 255 == 255 {
                                        v = z;
                                    } else {
                                        let aa = self.memcmp_sign32(env, c, a);
                                        v = (if e ^ (aa & 255 == 255) as i32 != 0 { a } else { c });
                                    }
                                }
                                c = (v.wrapping_sub(arg0) as u32).wrapping_shr(5 as u32) as i32;
                                if arg2 != 0 {
                                    a = arg0.wrapping_add(c.wrapping_shl(5 as u32));
                                    let ab = self.memcmp_sign32(env, arg2, a);
                                    if ab & 255 != 255 {
                                        break 'label2;
                                    }
                                }
                                if arg1 as u32 > c as u32 {
                                    break 'label3;
                                }
                                break 'label1;
                            }
                            c = 0;
                            arg2 = self.global0.wrapping_sub(1552);
                            self.global0 = arg2;
                            'label9: {
                                arg3 = arg1;
                                if (arg3 as u32) >= 2 as u32 {
                                    d = 1;
                                    f = (arg1 as u32).wrapping_shr(1 as u32) as i32;
                                    arg4 = f.wrapping_shl(5 as u32);
                                    arg1 = arg0.wrapping_add(arg4);
                                    arg4 = arg2.wrapping_add(arg4);
                                    if arg3 as u32 >= 8 as u32 {
                                        self.func113(env, arg0, arg2);
                                        self.func113(env, arg1, arg4);
                                        d = 4;
                                    } else {
                                        let ad = mload64!(arg0.wrapping_add(24) as usize) as i64;
                                        mstore64!(arg2.wrapping_add(24) as usize, ad as u64);
                                        let ae = mload64!(arg0.wrapping_add(16) as usize) as i64;
                                        mstore64!(arg2.wrapping_add(16) as usize, ae as u64);
                                        let af = mload64!(arg0.wrapping_add(8) as usize) as i64;
                                        mstore64!(arg2.wrapping_add(8) as usize, af as u64);
                                        let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                                        let mut svarg2_0_i64 = svarg0_0_i64 as i64;
                                        let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                                        let ag = mload64!(arg1.wrapping_add(8) as usize) as i64;
                                        mstore64!(arg4.wrapping_add(8) as usize, ag as u64);
                                        let ah = mload64!(arg1.wrapping_add(16) as usize) as i64;
                                        mstore64!(arg4.wrapping_add(16) as usize, ah as u64);
                                        let ai = mload64!(arg1.wrapping_add(24) as usize) as i64;
                                        mstore64!(arg4.wrapping_add(24) as usize, ai as u64);
                                    }
                                    h = 0.wrapping_sub(d);
                                    i = arg3.wrapping_sub(f);
                                    arg1 = d.wrapping_shl(5 as u32);
                                    j = arg0.wrapping_add(arg1);
                                    k = arg1.wrapping_add(arg2);
                                    while c != 2 {
                                        arg1 = mload32!(arg2.wrapping_add(1536).wrapping_add(c.wrapping_shl(2 as u32)) as usize) as i32;
                                        a = (if arg1 != 0 { i } else { f });
                                        e = h.wrapping_add((if (a as u32 > d as u32) as i32 != 0 { a } else { d }));
                                        g = arg1.wrapping_shl(5 as u32);
                                        arg1 = j.wrapping_add(g);
                                        a = g.wrapping_add(k);
                                        g = arg2.wrapping_add(g);
                                        loop {
                                            if e != 0 {
                                                let mut svarg1_0_i64 = mload64!(arg1 as usize) as i64;
                                                sv5_0_i64 = svarg1_0_i64 as i64;
                                                let aj = mload64!(arg1.wrapping_add(24) as usize) as i64;
                                                mstore64!(a.wrapping_add(24) as usize, aj as u64);
                                                let ak = mload64!(arg1.wrapping_add(16) as usize) as i64;
                                                mstore64!(a.wrapping_add(16) as usize, ak as u64);
                                                let al = mload64!(arg1.wrapping_add(8) as usize) as i64;
                                                mstore64!(a.wrapping_add(8) as usize, al as u64);
                                                self.func74(env, g, a);
                                                e -= 1;
                                                arg1 = arg1.wrapping_add(32);
                                                a = a.wrapping_add(32);
                                            }
                                        }
                                        c += 1;
                                    }
                                    e = arg4.wrapping_sub(32);
                                    arg1 = arg3.wrapping_shl(5 as u32).wrapping_sub(32);
                                    a = arg0.wrapping_add(arg1);
                                    c = arg1.wrapping_add(arg2);
                                    arg1 = arg2;
                                    loop {
                                        if f != 0 {
                                            let am = self.memcmp_sign32(env, arg4, arg1);
                                            g = am & 255;
                                            h = (g == 255) as i32;
                                            d = (if h != 0 { arg4 } else { arg1 });
                                            let mut sv8_0_i64 = mload64!(d as usize) as i64;
                                            let mut svarg0_0_i64 = sv8_0_i64 as i64;
                                            let an = mload64!(d.wrapping_add(24) as usize) as i64;
                                            mstore64!(arg0.wrapping_add(24) as usize, an as u64);
                                            let ao = mload64!(d.wrapping_add(16) as usize) as i64;
                                            mstore64!(arg0.wrapping_add(16) as usize, ao as u64);
                                            let ap = mload64!(d.wrapping_add(8) as usize) as i64;
                                            mstore64!(arg0.wrapping_add(8) as usize, ap as u64);
                                            let aq = self.memcmp_sign32(env, c, e);
                                            i = aq & 255;
                                            j = (i == 255) as i32;
                                            d = (if j != 0 { e } else { c });
                                            let mut sv8_0_i64 = mload64!(d as usize) as i64;
                                            let mut sv5_0_i64 = sv8_0_i64 as i64;
                                            let ar = mload64!(d.wrapping_add(24) as usize) as i64;
                                            mstore64!(a.wrapping_add(24) as usize, ar as u64);
                                            let at = mload64!(d.wrapping_add(16) as usize) as i64;
                                            mstore64!(a.wrapping_add(16) as usize, at as u64);
                                            let au = mload64!(d.wrapping_add(8) as usize) as i64;
                                            mstore64!(a.wrapping_add(8) as usize, au as u64);
                                            f -= 1;
                                            a = a.wrapping_sub(32);
                                            arg0 = arg0.wrapping_add(32);
                                            arg1 = arg1.wrapping_add(((g != 255) as i32).wrapping_shl(5 as u32));
                                            arg4 = arg4.wrapping_add(h.wrapping_shl(5 as u32));
                                            e = e.wrapping_add((if j != 0 { -32 } else { 0 }));
                                            c = c.wrapping_add((if (i != 255) as i32 != 0 { -32 } else { 0 }));
                                        } else {
                                            a = e.wrapping_add(32);
                                            let av: i32;
                                            if arg3 & 1 != 0 {
                                                e = ((arg1 as u32) < a as u32) as i32;
                                                arg3 = (if e != 0 { arg1 } else { arg4 });
                                                let mut svarg3_0_i64 = mload64!(arg3 as usize) as i64;
                                                let mut svarg0_0_i64 = svarg3_0_i64 as i64;
                                                let aw = mload64!(arg3.wrapping_add(24) as usize) as i64;
                                                mstore64!(arg0.wrapping_add(24) as usize, aw as u64);
                                                let ax = mload64!(arg3.wrapping_add(16) as usize) as i64;
                                                mstore64!(arg0.wrapping_add(16) as usize, ax as u64);
                                                let ay = mload64!(arg3.wrapping_add(8) as usize) as i64;
                                                mstore64!(arg0.wrapping_add(8) as usize, ay as u64);
                                                arg4 = arg4.wrapping_add(((arg1 as u32 >= a as u32) as i32).wrapping_shl(5 as u32));
                                                av = arg1.wrapping_add(e.wrapping_shl(5 as u32));
                                            } else {
                                                av = arg1;
                                            }
                                            if (av == a) as i32 & arg4 == c.wrapping_add(32) {
                                                break 'label9;
                                            }
                                            unreachable!();
                                        }
                                        unreachable!();
                                    }
                                    unreachable!();
                                    unreachable!();
                                }
                            }
                            self.global0 = arg2.wrapping_add(1552);
                        }
                        self.global0 = b.wrapping_add(192);
                        return arg0;
                    }
                    a = arg0.wrapping_add(c.wrapping_shl(5 as u32));
                    d = a.wrapping_add(8);
                    let mut sv8_0_i64 = mload64!(d as usize) as i64;
                    n = sv8_0_i64;
                    g = a.wrapping_add(16);
                    let mut sv11_0_i64 = mload64!(g as usize) as i64;
                    l = sv11_0_i64;
                    h = a.wrapping_add(24);
                    let mut sv12_0_i64 = mload64!(h as usize) as i64;
                    m = sv12_0_i64;
                    let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                    o = svarg0_0_i64;
                    let mut sv5_0_i64 = mload64!(a as usize) as i64;
                    svarg0_0_i64 = sv5_0_i64 as i64;
                    c = arg0.wrapping_add(24);
                    let mut sv7_0_i64 = mload64!(c as usize) as i64;
                    p = sv7_0_i64;
                    sv7_0_i64 = m as i64;
                    e = arg0.wrapping_add(16);
                    let mut sv9_0_i64 = mload64!(e as usize) as i64;
                    m = sv9_0_i64;
                    sv9_0_i64 = l as i64;
                    f = arg0.wrapping_add(8);
                    let mut sv10_0_i64 = mload64!(f as usize) as i64;
                    l = sv10_0_i64;
                    sv10_0_i64 = n as i64;
                    sv12_0_i64 = p as i64;
                    sv11_0_i64 = m as i64;
                    sv8_0_i64 = l as i64;
                    sv5_0_i64 = o as i64;
                    d = arg0.wrapping_add(32);
                    let mut sv6_140_i32 = d as i32;
                    let az = mload64!(arg0.wrapping_add(56) as usize) as i64;
                    mstore64!(b.wrapping_add(168) as usize, az as u64);
                    let ba = mload64!(arg0.wrapping_add(48) as usize) as i64;
                    mstore64!(b.wrapping_add(160) as usize, ba as u64);
                    let bb = mload64!(arg0.wrapping_add(40) as usize) as i64;
                    mstore64!(b.wrapping_add(152) as usize, bb as u64);
                    let mut svarg0_32_i64 = mload64!(arg0 as usize + 32) as i64;
                    let mut sv6_144_i64 = svarg0_32_i64 as i64;
                    let mut sv6_188_i32 = 0 as i32;
                    a = arg0.wrapping_sub(-64);
                    let mut sv6_184_i32 = a as i32;
                    let mut sv6_176_i32 = d as i32;
                    d = arg1.wrapping_shl(5 as u32);
                    g = arg0.wrapping_add(d);
                    let mut sv6_180_i32 = b.wrapping_add(144) as i32;
                    while !(a as u32 >= g as u32) {
                        self.func117(
                            env,
                            arg0,
                            b.wrapping_add(140),
                            b.wrapping_add(176),
                        );
                        a = mload32!(b as usize + 184) as i32;
                    }
                    'label15: {
                        d = sv6_140_i32.wrapping_add(d).wrapping_sub(32);
                        loop {
                            if a == d {
                                break 'label15;
                            }
                            self.func117(
                                env,
                                arg0,
                                b.wrapping_add(140),
                                b.wrapping_add(176),
                            );
                            let mut sv6_184_i32 = mload32!(b as usize + 184) as i32;
                            a = sv6_184_i32;
                        }
                        unreachable!();
                    }
                    let mut sv6_180_i32 = mload32!(b as usize + 180) as i32;
                    sv6_184_i32 = sv6_180_i32 as i32;
                    self.func117(
                        env,
                        arg0,
                        b.wrapping_add(140),
                        b.wrapping_add(176),
                    );
                    d = mload32!(b as usize + 188) as i32;
                    if d as u32 >= arg1 as u32 {
                        break 'label1;
                    }
                    a = arg0.wrapping_add(d.wrapping_shl(5 as u32));
                    g = a.wrapping_add(8);
                    let mut sv11_0_i64 = mload64!(g as usize) as i64;
                    n = sv11_0_i64;
                    h = a.wrapping_add(16);
                    let mut sv12_0_i64 = mload64!(h as usize) as i64;
                    l = sv12_0_i64;
                    i = a.wrapping_add(24);
                    m = mload64!(i as usize) as i64;
                    let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                    o = svarg0_0_i64;
                    let mut sv5_0_i64 = mload64!(a as usize) as i64;
                    svarg0_0_i64 = sv5_0_i64 as i64;
                    let mut sv7_0_i64 = mload64!(c as usize) as i64;
                    p = sv7_0_i64;
                    sv7_0_i64 = m as i64;
                    let mut sv9_0_i64 = mload64!(e as usize) as i64;
                    m = sv9_0_i64;
                    sv9_0_i64 = l as i64;
                    let mut sv10_0_i64 = mload64!(f as usize) as i64;
                    l = sv10_0_i64;
                    sv10_0_i64 = n as i64;
                    sv12_0_i64 = m as i64;
                    sv11_0_i64 = l as i64;
                    sv5_0_i64 = o as i64;
                    self.func114(
                        env,
                        arg0,
                        d,
                        arg2,
                        arg3,
                        arg4,
                    );
                    arg1 = arg1.wrapping_add(d ^ -1);
                    arg0 = a.wrapping_add(32);
                    arg2 = a;
                    continue 'label0;
                }
                arg2 = a.wrapping_add(8);
                let mut svarg2_0_i64 = mload64!(arg2 as usize) as i64;
                n = svarg2_0_i64;
                d = a.wrapping_add(16);
                let mut sv8_0_i64 = mload64!(d as usize) as i64;
                l = sv8_0_i64;
                g = a.wrapping_add(24);
                let mut sv11_0_i64 = mload64!(g as usize) as i64;
                m = sv11_0_i64;
                let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                let mut sv5_0_i64 = mload64!(a as usize) as i64;
                svarg0_0_i64 = sv5_0_i64 as i64;
                c = arg0.wrapping_add(24);
                let mut sv7_0_i64 = mload64!(c as usize) as i64;
                p = sv7_0_i64;
                sv7_0_i64 = m as i64;
                e = arg0.wrapping_add(16);
                let mut sv9_0_i64 = mload64!(e as usize) as i64;
                m = sv9_0_i64;
                sv9_0_i64 = l as i64;
                f = arg0.wrapping_add(8);
                let mut sv10_0_i64 = mload64!(f as usize) as i64;
                l = sv10_0_i64;
                sv10_0_i64 = n as i64;
                sv11_0_i64 = p as i64;
                sv8_0_i64 = m as i64;
                svarg2_0_i64 = l as i64;
                arg2 = arg0.wrapping_add(32);
                let mut sv6_140_i32 = arg2 as i32;
                let bc = mload64!(arg0.wrapping_add(56) as usize) as i64;
                mstore64!(b.wrapping_add(168) as usize, bc as u64);
                let bd = mload64!(arg0.wrapping_add(48) as usize) as i64;
                mstore64!(b.wrapping_add(160) as usize, bd as u64);
                let be = mload64!(arg0.wrapping_add(40) as usize) as i64;
                mstore64!(b.wrapping_add(152) as usize, be as u64);
                let mut svarg0_32_i64 = mload64!(arg0 as usize + 32) as i64;
                let mut sv6_144_i64 = svarg0_32_i64 as i64;
                let mut sv6_188_i32 = 0 as i32;
                a = arg0.wrapping_sub(-64);
                let mut sv6_184_i32 = a as i32;
                let mut sv6_176_i32 = arg2 as i32;
                arg2 = arg1.wrapping_shl(5 as u32);
                d = arg0.wrapping_add(arg2);
                let mut sv6_180_i32 = b.wrapping_add(144) as i32;
                while !(a as u32 >= d as u32) {
                    self.func118(
                        env,
                        arg0,
                        b.wrapping_add(140),
                        b.wrapping_add(176),
                    );
                    a = mload32!(b as usize + 184) as i32;
                }
                'label18: {
                    arg2 = sv6_140_i32.wrapping_add(arg2).wrapping_sub(32);
                    loop {
                        if arg2 == a {
                            break 'label18;
                        }
                        self.func118(
                            env,
                            arg0,
                            b.wrapping_add(140),
                            b.wrapping_add(176),
                        );
                        a = mload32!(b as usize + 184) as i32;
                    }
                    unreachable!();
                }
                let mut sv6_180_i32 = mload32!(b as usize + 180) as i32;
                self.func118(
                    env,
                    arg0,
                    b.wrapping_add(140),
                    b.wrapping_add(176),
                );
                a = mload32!(b as usize + 188) as i32;
                if a as u32 >= arg1 as u32 {
                    break 'label1;
                }
                arg2 = arg0.wrapping_add(a.wrapping_shl(5 as u32));
                d = arg2.wrapping_add(8);
                let mut sv8_0_i64 = mload64!(d as usize) as i64;
                g = arg2.wrapping_add(16);
                let mut sv11_0_i64 = mload64!(g as usize) as i64;
                h = arg2.wrapping_add(24);
                let mut sv12_0_i64 = mload64!(h as usize) as i64;
                let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
                let mut svarg2_0_i64 = mload64!(arg2 as usize) as i64;
                let mut sv7_0_i64 = mload64!(c as usize) as i64;
                let mut sv9_0_i64 = mload64!(e as usize) as i64;
                let mut sv10_0_i64 = mload64!(f as usize) as i64;
                arg2 = a += 1;
                arg1 = arg1.wrapping_sub(arg2);
                arg0 = arg0.wrapping_add(arg2.wrapping_shl(5 as u32));
                continue 'label0;
            }
        }
    }

    fn func115(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
    }

    fn memcpy_like_6(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        loop {
            if a != 32 {
                b = arg0.wrapping_add(a);
                let d = mload8!(b as usize) as i32;
                c = d;
                let e = b;
                b = arg1.wrapping_add(a);
                let f = mload8!(b as usize) as i32;
                mstore8!(e as usize, f as u8);
                mstore8!(b as usize, c as u8);
                a += 1;
            }
        }
    }

    fn func117(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
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
        a = mload32!(arg2 as usize + 8) as i32;
        let j = self.memcmp_sign32(env, a, arg0);
        b = j;
        let mut svarg1_0_i32 = mload32!(arg1 as usize) as i32;
        let mut svarg2_12_i32 = mload32!(arg2 as usize + 12) as i32;
        c = svarg2_12_i32;
        arg0 = svarg1_0_i32.wrapping_add(c.wrapping_shl(5 as u32));
        d = arg0.wrapping_add(8);
        g = mload64!(d as usize) as i64;
        e = arg0.wrapping_add(16);
        h = mload64!(e as usize) as i64;
        f = arg0.wrapping_add(24);
        i = mload64!(f as usize) as i64;
        arg1 = mload32!(arg2 as usize) as i32;
        mstore64!(arg1.wrapping_add(24) as usize, i as u64);
        mstore64!(arg1.wrapping_add(16) as usize, h as u64);
        mstore64!(arg1.wrapping_add(8) as usize, g as u64);
        let k = mload64!(a.wrapping_add(24) as usize) as i64;
        let l = mload64!(a.wrapping_add(16) as usize) as i64;
        let m = mload64!(a.wrapping_add(8) as usize) as i64;
        svarg2_12_i32 = c.wrapping_add((b & 255 == 255) as i32) as i32;
    }

    fn func118(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
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
        let j = arg0;
        arg0 = mload32!(arg2 as usize + 8) as i32;
        let k = self.memcmp_sign32(env, j, arg0);
        b = k;
        let mut svarg1_0_i32 = mload32!(arg1 as usize) as i32;
        let mut svarg2_12_i32 = mload32!(arg2 as usize + 12) as i32;
        c = svarg2_12_i32;
        arg1 = svarg1_0_i32.wrapping_add(c.wrapping_shl(5 as u32));
        d = arg1.wrapping_add(8);
        g = mload64!(d as usize) as i64;
        e = arg1.wrapping_add(16);
        h = mload64!(e as usize) as i64;
        f = arg1.wrapping_add(24);
        i = mload64!(f as usize) as i64;
        a = mload32!(arg2 as usize) as i32;
        mstore64!(a.wrapping_add(24) as usize, i as u64);
        mstore64!(a.wrapping_add(16) as usize, h as u64);
        mstore64!(a.wrapping_add(8) as usize, g as u64);
        let l = mload64!(arg0.wrapping_add(24) as usize) as i64;
        let m = mload64!(arg0.wrapping_add(16) as usize) as i64;
        let n = mload64!(arg0.wrapping_add(8) as usize) as i64;
        svarg2_12_i32 = c.wrapping_add((b & 255 != 255) as i32) as i32;
    }

    fn memcpy_like_7(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        a = self.global0.wrapping_sub(32);
        mstore64!(a.wrapping_add(24) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(16) as usize, 0 /* False */ as u64);
        mstore64!(a.wrapping_add(8) as usize, 0 /* False */ as u64);
        loop {
            if b != 32 {
                c = a.wrapping_add(b);
                d = arg1.wrapping_add(b);
                let g = mload8!(d as usize) as i32;
                e = (g as u32).wrapping_shr(1 as u32) as i32;
                mstore8!(c as usize, e as u8);
                if b != 0 {
                    let h = mload8!(d -= 1 as usize) as i32;
                    mstore8!(c as usize, (h.wrapping_shl(7 as u32) | e) as u8);
                }
                b += 1;
            }
        }
        let i = mload64!(a.wrapping_add(24) as usize) as i64;
        mstore64!(arg0.wrapping_add(24) as usize, i as u64);
        let j = mload64!(a.wrapping_add(16) as usize) as i64;
        mstore64!(arg0.wrapping_add(16) as usize, j as u64);
        let k = mload64!(a.wrapping_add(8) as usize) as i64;
        mstore64!(arg0.wrapping_add(8) as usize, k as u64);
    }

    fn memcpy_like_8(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = self.global0.wrapping_sub(32);
        let e = mload64!(arg1.wrapping_add(24) as usize) as i64;
        mstore64!(a.wrapping_add(24) as usize, e as u64);
        let f = mload64!(arg1.wrapping_add(16) as usize) as i64;
        mstore64!(a.wrapping_add(16) as usize, f as u64);
        let g = mload64!(arg1.wrapping_add(8) as usize) as i64;
        mstore64!(a.wrapping_add(8) as usize, g as u64);
        arg1 = 31;
        while arg1 != -1 {
            b = arg1.wrapping_add(a);
            let h = b;
            let i = mload8!(arg1.wrapping_add(arg2) as usize) as i32;
            let j = mload8!(b as usize) as i32;
            b = i.wrapping_add(c.wrapping_add(j));
            mstore8!(h as usize, b as u8);
            c = ((b & 65280) as u32).wrapping_shr(8 as u32) as i32;
            arg1 -= 1;
        }
        let k = mload64!(a.wrapping_add(24) as usize) as i64;
        mstore64!(arg0.wrapping_add(24) as usize, k as u64);
        let l = mload64!(a.wrapping_add(16) as usize) as i64;
        mstore64!(arg0.wrapping_add(16) as usize, l as u64);
        let m = mload64!(a.wrapping_add(8) as usize) as i64;
        mstore64!(arg0.wrapping_add(8) as usize, m as u64);
    }

    fn func121(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = self.memcmp_sign32(env, arg0, arg1);
        (((a & 255) as u32) < 2 as u32) as i32
    }

    fn func122(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = self.memcmp_sign32(env, arg0, arg1);
        (((a.wrapping_sub(3) & 255) as u32) < 254 as u32) as i32
    }

    fn func123(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        if arg1 as u32 > arg3 as u32 {
            unreachable!();
        }
    }


    fn func125(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        b = mload32!(arg1 as usize + 8) as i32;
        if arg2 as u32 <= b as u32 {
            b = b.wrapping_sub(arg2);
            self.alloc_range_fill(
                env,
                a.wrapping_add(8),
                b,
                1,
            );
            let mut sv3_12_i32 = mload32!(a as usize + 12) as i32;
            let mut svarg1_4_i32 = mload32!(arg1 as usize + 4) as i32;
            let f = self.memmove(
                env,
                sv3_12_i32,
                svarg1_4_i32.wrapping_add(arg2),
                b,
            );
            self.global0 = a.wrapping_add(16);
        }
    }



    fn func128(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> i32 {
        let a = self.memcpy_like_2(
            env,
            arg0,
            arg1,
            arg2,
        );
        a
    }

    fn memcmp(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        'label0: {
            if arg2 != 0 {
                loop {
                    let d = mload8!(arg0 as usize) as i32;
                    a = d;
                    let e = mload8!(arg1 as usize) as i32;
                    b = e;
                    if a == b {
                        arg0 += 1;
                        arg1 += 1;
                        arg2 -= 1;
                        break 'label0;
                    }
                }
                c = a.wrapping_sub(b);
            }
        }
        c
    }

    fn func130(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        a = arg2 & 4294967295;
        b = arg1 & 4294967295;
        c = a.wrapping_mul(b);
        arg2 = (arg2 as u64).wrapping_shr(32 as u32) as i64;
        b = b.wrapping_mul(arg2);
        d = (arg1 as u64).wrapping_shr(32 as u32) as i64;
        arg1 = b.wrapping_add(a.wrapping_mul(d));
        a = c.wrapping_add(arg1.wrapping_shl(32 as u32));
    }

    fn memmove(
        &mut self,
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
        let mut h: i32 = 0;
        if (arg2 as u32) < 16 as u32 {
            a = arg0;
        } else {
            d = 0.wrapping_sub(arg0) & 3;
            c = arg0.wrapping_add(d);
            if c as u32 > arg0 as u32 {
                a = arg0;
                b = arg1;
                if d != 0 {
                    e = d;
                    loop {
                        let i = mload8!(b as usize) as i32;
                        mstore8!(a as usize, i as u8);
                        b += 1;
                        a += 1;
                        e -= 1;
                    }
                }
                if (d.wrapping_sub(1) as u32) >= 7 as u32 {
                    loop {
                        let j = mload8!(b as usize) as i32;
                        mstore8!(a as usize, j as u8);
                        let k = mload8!(b += 1 as usize) as i32;
                        mstore8!(a.wrapping_add(1) as usize, k as u8);
                        let l = mload8!(b.wrapping_add(2) as usize) as i32;
                        mstore8!(a.wrapping_add(2) as usize, l as u8);
                        let m = mload8!(b.wrapping_add(3) as usize) as i32;
                        mstore8!(a.wrapping_add(3) as usize, m as u8);
                        let n = mload8!(b.wrapping_add(4) as usize) as i32;
                        mstore8!(a.wrapping_add(4) as usize, n as u8);
                        let o = mload8!(b.wrapping_add(5) as usize) as i32;
                        mstore8!(a.wrapping_add(5) as usize, o as u8);
                        let p = mload8!(b.wrapping_add(6) as usize) as i32;
                        mstore8!(a.wrapping_add(6) as usize, p as u8);
                        let q = mload8!(b.wrapping_add(7) as usize) as i32;
                        mstore8!(a.wrapping_add(7) as usize, q as u8);
                        b = b.wrapping_add(8);
                        a = a.wrapping_add(8);
                    }
                }
            }
            e = arg2.wrapping_sub(d);
            f = e & -4;
            a = c.wrapping_add(f);
            'label4: {
                b = arg1.wrapping_add(d);
                if b & 3 == 0 {
                    if a as u32 <= c as u32 {
                        break 'label4;
                    }
                    arg1 = b;
                    loop {
                        let mut svarg1_0_i32 = mload32!(arg1 as usize) as i32;
                        let mut sv5_0_i32 = svarg1_0_i32 as i32;
                        arg1 = arg1.wrapping_add(4);
                        c = c.wrapping_add(4);
                    }
                    break 'label4;
                }
                if a as u32 <= c as u32 {
                    break 'label4;
                }
                arg2 = b.wrapping_shl(3 as u32);
                d = arg2 & 24;
                g = b & -4;
                arg1 = g.wrapping_add(4);
                h = 0.wrapping_sub(arg2) & 24;
                arg2 = mload32!(g as usize) as i32;
                loop {
                    let r = arg2;
                    arg2 = svarg1_0_i32;
                    sv5_0_i32 = ((r as u32).wrapping_shr(d as u32) as i32 | arg2.wrapping_shl(h as u32)) as i32;
                    arg1 = arg1.wrapping_add(4);
                    c = c.wrapping_add(4);
                }
            }
            arg2 = e & 3;
            arg1 = b.wrapping_add(f);
        }
        d = arg2.wrapping_add(a);
        if a as u32 < d as u32 {
            b = arg2 & 7;
            if b != 0 {
                loop {
                    let s = mload8!(arg1 as usize) as i32;
                    mstore8!(a as usize, s as u8);
                    arg1 += 1;
                    a += 1;
                    b -= 1;
                }
            }
            if (arg2.wrapping_sub(1) as u32) >= 7 as u32 {
                loop {
                    let t = mload8!(arg1 as usize) as i32;
                    mstore8!(a as usize, t as u8);
                    let u = mload8!(arg1 += 1 as usize) as i32;
                    mstore8!(a.wrapping_add(1) as usize, u as u8);
                    let v = mload8!(arg1.wrapping_add(2) as usize) as i32;
                    mstore8!(a.wrapping_add(2) as usize, v as u8);
                    let w = mload8!(arg1.wrapping_add(3) as usize) as i32;
                    mstore8!(a.wrapping_add(3) as usize, w as u8);
                    let x = mload8!(arg1.wrapping_add(4) as usize) as i32;
                    mstore8!(a.wrapping_add(4) as usize, x as u8);
                    let y = mload8!(arg1.wrapping_add(5) as usize) as i32;
                    mstore8!(a.wrapping_add(5) as usize, y as u8);
                    let z = mload8!(arg1.wrapping_add(6) as usize) as i32;
                    mstore8!(a.wrapping_add(6) as usize, z as u8);
                    let aa = mload8!(arg1.wrapping_add(7) as usize) as i32;
                    mstore8!(a.wrapping_add(7) as usize, aa as u8);
                    arg1 = arg1.wrapping_add(8);
                    a = a.wrapping_add(8);
                }
            }
        }
        arg0
    }
}