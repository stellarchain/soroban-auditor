#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, String, Bytes, BytesN, Val, FromVal, Map, Symbol};

#[contract]
pub struct CBIHT4HVRIT5OMVLSXZ44J2ZAXYBDDGOSCN3LTN2DOC6SWHDS5IP6BK3;

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
impl CBIHT4HVRIT5OMVLSXZ44J2ZAXYBDDGOSCN3LTN2DOC6SWHDS5IP6BK3 {

    pub fn init(
        env: Env,
        owner: Address,
    ) -> Result<(), Error> {
        let a = val_to_i64(String::from_str(env, "owner"));
        let b = Self::call_eq_one(env, a, 0);
        let c: i64;
        if b != 0 {
            c = Error(Storage, ExistingValue);
        } else {
            env.storage().put_contract_data(1048592, 5, owner);
            c = 0;
        }
        return c;
    }

    pub fn change_owner(
        env: Env,
        new_owner: Address,
    ) -> Result<(), Error> {
        let a: i32 = -16;
        Self::require_owner_auth(env, a);
        let c: i64;
        if mload32!(a as usize) != 0 {
            c = mload64!(a.wrapping_add(8) as usize);
        } else {
            env.storage().put_contract_data(1048597, 13, new_owner);
            c = 0;
        }
        return c;
    }

    pub fn accept_ownership(
        env: Env,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -16;
        Self::require_auth_for_key(
            env,
            a,
            1048597,
            13,
        );
        b = mload64!(a.wrapping_add(8) as usize);
        if mload32!(a as usize) == 0 {
            env.storage().put_contract_data(1048592, 5, b);
            let d = val_to_i64(String::from_str(env, "pending-owner"));
            env.storage().del_contract_data(d);
            b = 0;
        }
        b
    }

    pub fn cancel_ownership_transfer(
        env: Env,
    ) -> Result<(), Error> {
        let a: i32 = -16;
        Self::require_owner_auth(env, a);
        let d: i64;
        if mload32!(a as usize) != 0 {
            d = mload64!(a.wrapping_add(8) as usize);
        } else {
            let e = val_to_i64(String::from_str(env, "pending-owner"));
            env.storage().del_contract_data(e);
            d = 0;
        }
        d
    }

    pub fn upgrade(
        env: Env,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), Error> {
        let a: i32 = -16;
        {
            let c = Bytes::from_val(env, &val_from_i64(new_wasm_hash)).len() as i64;
            if c & 18446744069414584320 == 137438953472 {
                Self::require_owner_auth(env, a);
                let d: i64;
                if mload32!(a as usize) != 0 {
                    d = mload64!(a.wrapping_add(8) as usize);
                } else {
                    env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(new_wasm_hash)));
                    d = 0;
                }
                return d;
            }
        }
        unreachable!();
    }

    pub fn get_prices(
        env: Env,
        mut feed_ids: Vec<String>,
        mut payload: Bytes,
    ) -> Result<(u64, Vec<U256>), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -96;
        if !((!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | !(Bytes::try_from_val(env, &val_from_i64(payload)).is_ok())) {
            Self::func59(
                env,
                a,
                feed_ids,
                payload,
            );
            Self::decode_val_or_error(env, a.wrapping_sub(-64), a);
            let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
            payload = loaded_val;
            let e: i64;
            'label1: {
                let a_i32_64_2 = mload32!(a.wrapping_add(64) as usize);
                if a_i32_64_2 == 0 {
                    b = mload64!(a.wrapping_add(80) as usize);
                    let f = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
                    let g = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
                    let error_code = 42949672963 /* Error(Contract, #10) */;
                    if (f ^ g) as u64 >= 4294967296 as u64 {
                        e = error_code;
                        break 'label1;
                    }
                    let h = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
                    let i = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    feed_ids = i;
                    mstore32!(a.wrapping_add(60) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
                    loop {
                        Self::vec_pair_iter(env, a, a.wrapping_add(48));
                        Self::copy_val_if_present(env, a.wrapping_sub(-64), a);
                        let a_i32_64 = mload32!(a.wrapping_add(64) as usize);
                        if a_i32_64 == 1 {
                            let a_part_80 = mload64!(a.wrapping_add(80) as usize);
                            let j = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)); v.push_back(val_from_i64(a_part_80)); val_to_i64(v.into_val(env)) };
                            feed_ids = j;
                        }
                    }
                    Self::write_ok_val(env, a, payload);
                    if mload32!(a as usize) != 0 {
                        unreachable!();
                    }
                    payload = mload64!(a.wrapping_add(8) as usize);
                    let loaded_val = feed_ids as i64;
                    let k = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    e = k;
                    break 'label1;
                }
                e = payload;
            }
            return e;
        }
        42949672963 /* Error(Contract, #10) */
    }

    pub fn write_prices(
        env: Env,
        updater: Address,
        mut feed_ids: Vec<String>,
        mut payload: Bytes,
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
        c = -128;
        if (!(Address::try_from_val(env, &val_from_i64(updater)).is_ok())) as i32 | (!(Vec::<Val>::try_from_val(env, &val_from_i64(feed_ids)).is_ok())) as i32 | Bytes::try_from_val(env, &val_from_i64(payload)).is_ok() {
            updater.require_auth();
            env.storage().instance().extend_ttl(INSTANCE_BUMP_AMOUNT as u32, 181440 as u32);
            let l = mload64!(1048720 as usize);
            let mut sv3_8_i64 = l as i64;
            d = c.wrapping_add(8);
            e = 1;
            while e & 1 != 0 {
                e = 0;
                let n = val_to_i64(Address::from_string_bytes(&Bytes::from_val(env, &String::from_str(env, ""))).into_val(env));
                g = n;
                d = d.wrapping_add(8);
            }
            let mut sv3_96_i64 = g as i64;
            e = 0;
            loop {
                d = e;
                if d == 8 {
                    break;
                }
                if d != 8 {
                    e = d.wrapping_add(8);
                    let o = mload64!(c.wrapping_add(96).wrapping_add(d) as usize);
                    let p = { let a = val_from_i64(o); let b = val_from_i64(updater); if a < b { -1 } else if a > b { 1 } else { 0 } };
                }
            }
            e = c.wrapping_add(8);
            Self::func59(
                env,
                e,
                feed_ids,
                payload,
            );
            Self::decode_val_or_error(env, c.wrapping_add(96), e);
            feed_ids = mload64!(c.wrapping_add(104) as usize);
            let c_i32_96 = mload32!(c.wrapping_add(96) as usize);
            if c_i32_96 == 0 {
                payload = mload64!(c.wrapping_add(112) as usize);
                let q = Self::ledger_timestamp_val(env);
                g = q;
                let r = val_to_i64(Vec::<Val>::new(env).into_val(env));
                vec_builder = r;
                let s = Vec::<Val>::from_val(env, &val_from_i64(payload)).len() as i64;
                h = s;
                mstore32!(c.wrapping_add(68) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
                d = (d == 8) as i32;
                'label2: loop {
                    {
                        e = c.wrapping_add(8);
                        Self::vec_pair_iter(env, e, c.wrapping_add(56));
                        Self::copy_val_if_present(env, c.wrapping_add(72), e);
                        let c_i32_72 = mload32!(c.wrapping_add(72) as usize);
                        if c_i32_72 == 0 {
                            let sv3_96_i64 = REDSTONE as i64;
                            g = 0;
                            d = 1;
                            loop {
                                d -= 1;
                                g = REDSTONE;
                            }
                            unreachable!();
                        }
                        let loaded_val = mload64!(c.wrapping_add(80) as usize);
                        h = loaded_val;
                        i = mload64!(c.wrapping_add(88) as usize);
                        env.storage().get_contract_data(c.wrapping_add(8), h);
                        f = mload32!(c.wrapping_add(8) as usize);
                        let mut value_hi = mload64!(c.wrapping_add(24) as usize);
                        e = f & (value_hi as u64 >= feed_ids as u64) as i32;
                        payload = mload64!(c.wrapping_add(32) as usize);
                        if e != 0 {
                            continue 'label2;
                        }
                        j = payload.wrapping_add(40000);
                        if (payload as u64 > j as u64) as i32 != 0 {
                            payload = 18446744073709551615;
                        } else {
                            payload = j;
                        }
                        if (payload as u64 >= g as u64) as i32 & f != 0 {
                            continue 'label2;
                        }
                        let t = Self::result_unwrap_or_panic(env, c.wrapping_add(96));
                        env.storage().temporary().set(&val_from_i64(h), &val_from_i64(t));
                        let w = env.storage().temporary().extend_ttl(&val_from_i64(h), 34560 as u32, 51840 as u32);
                        let value_hi = g as i64;
                        let mut sv3_16_i64 = feed_ids as i64;
                        let mut sv3_8_i64 = i as i64;
                        let x = Self::result_unwrap_or_panic(env, c.wrapping_add(8));
                        vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(x)); val_to_i64(v.into_val(env)) };
                        continue 'label2;
                    }
                }
                let mut sv3_8_i64 = g as i64;
                e = c.wrapping_add(8);
                let z = val_to_i64(Vec::<Val>::new(env).into_val(env));
                let sv3_16_i64 = updater as i64;
                let sv3_8_i64 = vec_builder as i64;
                let aa = Self::map_new_val(
                    env,
                    1048876,
                    2,
                    e,
                    2,
                );
                let ab = val_to_i64(Bytes::from_val(env, &val_from_i64(aa)).into_val(env));
                env.events().publish(val_from_i64(z), val_from_i64(ab));
                feed_ids = 0;
            }
            return feed_ids;
        }
    }

    pub fn read_prices(
        env: Env,
        mut feed_ids: Vec<String>,
    ) -> Result<Vec<U256>, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut vec_builder: i64 = 0;
        a = -96;
        let g = val_to_i64(Vec::<Val>::new(env).into_val(env));
        vec_builder = g;
        let h = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
        mstore32!(a.wrapping_add(20) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
        b = a.wrapping_add(48);
        c = a.wrapping_add(80);
        'label0: {
            loop {
                d = a.wrapping_sub(-64);
                Self::vec_next_string_flag(env, d, a.wrapping_add(8));
                let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
                Self::guard_nonzero_ptr(
                    env,
                    a.wrapping_add(24),
                    mload64!(a.wrapping_add(64) as usize),
                    loaded_val,
                );
                let a_i32_24 = mload32!(a.wrapping_add(24) as usize);
                if a_i32_24 == 0 {
                    unreachable!();
                }
                env.storage().get_contract_data(d, mload64!(a.wrapping_add(32) as usize));
                let a_i32_64_2 = mload32!(a.wrapping_add(64) as usize);
                if a_i32_64_2 == 1 {
                    let i = mload64!(c.wrapping_add(8) as usize);
                    mstore64!(b.wrapping_add(8) as usize, i as u64);
                    let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
                    Self::check_recent_timestamp(env, d, a.wrapping_add(40));
                    let loaded_val = mload64!(a.wrapping_add(72) as usize);
                    feed_ids = loaded_val;
                    let a_i32_64 = mload32!(a.wrapping_add(64) as usize);
                    if a_i32_64 != 0 {
                        vec_builder = feed_ids;
                        break 'label0;
                    }
                    vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(feed_ids)); val_to_i64(v.into_val(env)) };
                }
            }
            vec_builder = Error(Storage, MissingValue);
        }
        return vec_builder;
    }

    pub fn read_timestamp(
        env: Env,
        feed_id: String,
    ) -> Result<u64, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -64;
        let d: i64;
        {
            b = a.wrapping_add(32);
            env.storage().get_contract_data(b, feed_id);
            let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
            if a_i32_32 != 0 {
                unreachable!();
            }
        }
        d = Error(Storage, MissingValue);
        d
    }

    pub fn read_price_data_for_feed(
        env: Env,
        feed_id: String,
    ) -> Result<PriceData, Error> {
        let a: i32 = -96;
        env.storage().get_contract_data(a.wrapping_add(40), feed_id);
        {
            let a_i32_40 = mload32!(a.wrapping_add(40) as usize);
            if a_i32_40 == 1 {
                let c = mload64!(a.wrapping_sub(-64) as usize);
                mstore64!(a.wrapping_add(88) as usize, c as u64);
                mload64!(a.wrapping_add(56) as usize);
                Self::check_recent_timestamp(env, a.wrapping_add(8), a.wrapping_add(72));
            } else {
                Error(Storage, MissingValue) as i64;
            }
        }
        let d = Self::result_from_val(env, a.wrapping_add(8));
        return d;
    }

    pub fn read_price_data(
        env: Env,
        mut feed_ids: Vec<String>,
    ) -> Result<Vec<PriceData>, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut vec_builder: i64 = 0;
        a = -112;
        'label0: {
            'label1: {
                let i = val_to_i64(Vec::<Val>::new(env).into_val(env));
                vec_builder = i;
                let j = Vec::<Val>::from_val(env, &val_from_i64(feed_ids)).len() as i64;
                mstore32!(a.wrapping_add(12) as usize, (j as u64).wrapping_shr(32 as u32) as i64 as u32);
                c = a.wrapping_add(96);
                b = a.wrapping_add(72);
                d = a.wrapping_add(40);
                loop {
                    e = a.wrapping_add(56);
                    Self::vec_next_string_flag(env, e, a);
                    let loaded_val = mload64!(a.wrapping_add(56) as usize);
                    Self::guard_nonzero_ptr(
                        env,
                        a.wrapping_add(16),
                        loaded_val,
                        mload64!(a.wrapping_add(64) as usize),
                    );
                    let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
                    if a_i32_16 == 0 {
                        unreachable!();
                    }
                    let value_hi = mload64!(a.wrapping_add(24) as usize);
                    env.storage().get_contract_data(e, value_hi);
                    let a_i32_56_2 = mload32!(a.wrapping_add(56) as usize);
                    if a_i32_56_2 != 0 {
                        unreachable!();
                    }
                    break 'label1;
                    mload64!(b as usize);
                    f = b.wrapping_add(8);
                    mstore64!(d.wrapping_add(8) as usize, mload64!(f as usize) as u64);
                    Self::check_recent_timestamp(env, e, a.wrapping_add(32));
                    feed_ids = mload64!(a.wrapping_add(64) as usize);
                    let a_i32_56 = mload32!(a.wrapping_add(56) as usize);
                    if a_i32_56 == 0 {
                        mstore64!(c.wrapping_add(8) as usize, mload64!(f as usize) as u64);
                        let k = Self::result_unwrap_or_panic(env, a.wrapping_add(88));
                        vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(k)); val_to_i64(v.into_val(env)) };
                    }
                }
                vec_builder = feed_ids;
                break 'label0;
                unreachable!();
            }
            vec_builder = Error(Storage, MissingValue);
        }
        vec_builder
    }

    pub fn check_price_data(
        env: Env,
        price_data: PriceData,
    ) -> Result<PriceData, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -64;
        Self::func38(env, a.wrapping_add(32), price_data);
        let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
        if a_i32_32 == 1 {
            unreachable!();
        }
        let d = mload64!(a.wrapping_add(56) as usize);
        mstore64!(a.wrapping_add(24) as usize, d as u64);
        let e = mload64!(a.wrapping_add(48) as usize);
        mstore64!(a.wrapping_add(16) as usize, e as u64);
        b = a.wrapping_add(32);
        Self::check_recent_timestamp(env, b, a.wrapping_add(8));
        let f = Self::result_from_val(env, b);
        f
    }

    pub fn unique_signer_threshold(
        env: Env,
    ) -> u64 {
        let a: i32 = -16;
        Self::write_ok_val(env, a, 3 /* Error(Contract, #0) */);
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        mload64!(a.wrapping_add(8) as usize)
    }
}

impl CBIHT4HVRIT5OMVLSXZ44J2ZAXYBDDGOSCN3LTN2DOC6SWHDS5IP6BK3 {

    fn vec_next_string_to_linear(
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
        a = -48;
        b = a.wrapping_add(16);
        Self::vec_next_string_flag(env, b, arg1);
        let mut value_lo = mload64!(a.wrapping_add(16) as usize);
        let value_hi = mload64!(a.wrapping_add(24) as usize);
        Self::guard_nonzero_ptr(
            env,
            a,
            value_lo,
            value_hi,
        );
        'label0: {
            {
                let h: i32;
                if mload32!(a as usize) == 1 {
                    e = mload64!(a.wrapping_add(8) as usize);
                    arg1 = a.wrapping_add(40);
                    c = a.wrapping_add(32);
                    d = a.wrapping_add(24);
                    let value_lo = 0 as i64;
                    let i = String::from_val(env, &val_from_i64(e)).len() as i64;
                    f = i;
                    if f as u64 >= 141733920768 as u64 {
                        unreachable!();
                    }
                    let j = String::from_val(env, &val_from_i64(e)).len() as i64;
                    if (j ^ f) as u64 >= 4294967296 as u64 {
                        break 'label0;
                    }
                    Self::copy_string_to_linear_memory(env, e, 0, (b as u32 as i64).wrapping_shl(32 as u32) | 0, f & 270582939648 | 0);
                    mstore64!(arg0.wrapping_add(25) as usize, mload64!(arg1 as usize) as u64);
                    mstore64!(arg0.wrapping_add(17) as usize, mload64!(c as usize) as u64);
                    mstore64!(arg0.wrapping_add(9) as usize, mload64!(d as usize) as u64);
                    mload64!(a.wrapping_add(16) as usize);
                    h = 1;
                } else {
                    h = 0;
                }
                mstore8!(arg0 as usize, h as u8);
            }
        }
    }

    fn vec_next_string_flag(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut svarg1_8_i32 = mload32!(arg1.wrapping_add(8) as usize);
        a = svarg1_8_i32;
        let arg1_lo = mload64!(arg1 as usize);
        let c = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1_lo)).get_unchecked((a as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
        a += 1;
        if a != 0 {
            let svarg1_8_i32 = a as i32;
            (!(String::try_from_val(env, &val_from_i64(c)).is_ok())) as i32 as u32 as i64 as i64;
        }
    }

    fn guard_nonzero_ptr(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -16;
        let c: i64;
        if arg1 != 0 {
            if arg1 as i32 & 1 != 0 {
                unreachable!();
            }
        }
    }

    fn span_from_range(
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
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let a = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (a == 1) as i32
    }

    fn func38(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = -48;
        while b != 24 {
            mstore64!(a.wrapping_add(8).wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
        }
        'label1: {
            'label2: {
                {
                    let f = 0;
                    b = a.wrapping_add(32);
                    Self::val_to_i64_checked(env, b, mload64!(a.wrapping_add(8) as usize));
                    let a_i32_32_2 = mload32!(a.wrapping_add(32) as usize);
                    if a_i32_32_2 != 0 {
                        unreachable!();
                    }
                    d = mload64!(a.wrapping_add(16) as usize);
                    c = d as i32 & 255;
                    if (c != 12) as i32 & c != 70 {
                        break 'label2;
                    }
                    let value_hi = mload64!(a.wrapping_add(24) as usize);
                    Self::val_to_i64_checked(env, b, value_hi);
                    let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                    if a_i32_32 == 0 {
                        unreachable!();
                    }
                    break 'label1;
                    break 'label1;
                }
                break 'label1;
            }
        }
    }



    fn decode_error_from_val(
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
                                }
                                a = 510;
                                break 'label1;
                                a = 0;
                                {
                                    b = mload32!(arg0.wrapping_add(4) as usize) ^ -2147483648;
                                    let e = mload8!(arg0.wrapping_add(8) as usize) as i32;
                                    a = e;
                                    break 'label1;
                                }
                                a = mload32!(arg0.wrapping_add(12) as usize);
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
                                let f = mload32!(arg0.wrapping_add(4) as usize) as i64;
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
                            let g = mload16!(arg0.wrapping_add(4) as usize) as i32;
                            a = g.wrapping_add(600);
                            if a & 65535 != a {
                                break 'label0;
                            }
                            break 'label1;
                        }
                        let h = mload16!(arg0.wrapping_add(4) as usize) as i32;
                        a = h.wrapping_add(1000);
                        if a & 65535 != a {
                            break 'label0;
                        }
                        break 'label1;
                    }
                    let i = mload16!(arg0.wrapping_add(4) as usize) as i32;
                    a = i.wrapping_add(1050);
                    if a & 65535 != a {
                        break 'label0;
                    }
                    break 'label1;
                }
                let j = mload8!(arg0.wrapping_add(16) as usize) as i32;
                a = j;
            }
            return (a as u32 as i64 & 65535).wrapping_shl(32 as u32) | 3 /* Error(Contract, #0) */;
        }
    }

    fn copy_val_if_present(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -16;
        b = mload64!(arg1 as usize);
        let d: i64;
        if b != 0 {
            if b as i32 & 1 != 0 {
                unreachable!();
            }
            mload64!(arg1.wrapping_add(16) as usize);
        }
    }

    fn decode_val_or_error(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let c: i64;
        {
            let d = mload8!(arg1 as usize) as i32;
            if d == 27 {
                mload64!(arg1.wrapping_add(16) as usize);
                c = mload64!(arg1.wrapping_add(8) as usize);
            } else {
                Self::decode_error_from_val(env, arg1);
            }
        }
    }

    fn alloc_copy(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let a: i32 = -16;
        Self::alloc_range(
            env,
            a.wrapping_add(8),
            arg2,
            1,
            1,
        );
        let d = Self::memmove(
            env,
            mload32!(a.wrapping_add(12) as usize),
            arg1,
            arg2,
        );
    }

    fn alloc_range(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let a: i32 = -16;
        Self::func49(
            env,
            a.wrapping_add(4),
            arg1,
            0,
            arg2,
            arg3,
        );
        arg1 = mload32!(a.wrapping_add(8) as usize);
        let a_i32_4 = mload32!(a.wrapping_add(4) as usize);
        if a_i32_4 == 0 {
        }
        Self::alloc_trap(env, arg1, mload32!(a.wrapping_add(12) as usize));
    }

    fn alloc_realloc(
        env: &Env,
        mut arg0: i32,
    ) {
        let a: i32 = -16;
        Self::func47(
            env,
            a.wrapping_add(8),
            arg0,
            mload32!(arg0 as usize),
            1,
            1,
            64,
        );
        arg0 = mload32!(a.wrapping_add(8) as usize);
        if arg0 != -2147483647 {
            Self::alloc_trap(env, arg0, mload32!(a.wrapping_add(12) as usize));
            unreachable!();
        }
    }

    fn func47(
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
        b = -16;
        'label0: {
            c = arg2.wrapping_add(arg3);
            if arg2 as u32 <= c as u32 {
                e = 0.wrapping_sub(arg4);
                arg3 = arg4 -= 1;
                let mut svarg1_0_i32 = mload32!(arg1 as usize);
                d = svarg1_0_i32;
                arg2 = d.wrapping_shl(1 as u32);
                if (arg2 as u32 > c as u32) as i32 != 0 {
                    arg2 = arg2;
                } else {
                    arg2 = c;
                }
                if (arg2 as u32 <= 4 as u32) as i32 != 0 {
                    f = 4;
                } else {
                    f = arg2;
                }
                g = ((e & arg3.wrapping_add(arg5)) as u32 as i64).wrapping_mul(f as u32 as i64);
                if (g as u64).wrapping_shr(32 as u32) as (i64 == 0) {
                    arg2 = g as i32;
                    if arg2 as u32 <= (-2147483648).wrapping_sub(arg4) as u32 {
                        {
                            'label2: {
                                let i: i32;
                                'label3: {
                                    if d != 0 {
                                        if arg5 == 0 {
                                            Self::func108(
                                                env,
                                                b.wrapping_add(8),
                                                arg4,
                                                arg2,
                                            );
                                            i = mload32!(b.wrapping_add(8) as usize);
                                            break 'label3;
                                        }
                                        let mut svarg1_4_i32 = mload32!(arg1.wrapping_add(4) as usize);
                                        c = svarg1_4_i32;
                                        Self::memcpy_like_5(env);
                                        let j = mload32!(1049924 as usize);
                                        a = j;
                                        arg3 = a.wrapping_add(arg3);
                                        if (arg3 as u32) < a as u32 {
                                            unreachable!();
                                        }
                                        {
                                            arg3 = arg3 & e;
                                            a = arg3.wrapping_add(arg2);
                                            let k = mload32!(1049928 as usize);
                                            if a as u32 > k as u32 {
                                                let l = Self::func110(env, arg2, arg4);
                                                arg3 = l;
                                            } else {
                                                mstore32!(1049924 as usize, a as u32);
                                            }
                                        }
                                        if arg3 == 0 {
                                            unreachable!();
                                        }
                                        let m = Self::memmove(
                                            env,
                                            arg3,
                                            c,
                                            arg5.wrapping_mul(d),
                                        );
                                        break 'label2;
                                    }
                                    Self::func108(
                                        env,
                                        b,
                                        arg4,
                                        arg2,
                                    );
                                    i = mload32!(b as usize);
                                }
                                arg3 = i;
                                if arg3 == 0 {
                                    unreachable!();
                                }
                            }
                            let svarg1_0_i32 = f as i32;
                            let svarg1_4_i32 = arg3 as i32;
                            break 'label0;
                        }
                    }
                }
            }
        }
    }

    fn alloc_trap(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        if arg0 == 0 {
            unreachable!();
        }
    }

    fn func49(
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
                }
                let e = mload8!(1049920 as usize) as i32;
                {
                    if arg2 != 0 {
                        Self::memcpy_like_5(env);
                        let f = mload32!(1049924 as usize);
                        arg2 = f;
                        c = arg2.wrapping_add(c);
                        if (c as u32) < arg2 as u32 {
                            unreachable!();
                        }
                        {
                            arg2 = a & c;
                            a = arg2.wrapping_add(arg4);
                            let g = mload32!(1049928 as usize);
                            if a as u32 > g as u32 {
                                let h = Self::func110(env, arg4, arg3);
                                arg2 = h;
                            } else {
                                mstore32!(1049924 as usize, a as u32);
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
                        let i = Self::func111(env, arg4, arg3);
                        arg2 = i;
                        if arg2 == 0 {
                            unreachable!();
                        }
                    }
                    let svarg0_8_i32 = arg2 as i32;
                    let svarg0_4_i32 = arg1 as i32;
                }
            }
        }
    }

    fn result_unwrap_or_panic(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let a: i32 = -16;
        Self::func51(env, a, arg0);
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        mload64!(a.wrapping_add(8) as usize)
    }

    fn func51(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -48;
        b = a.wrapping_add(32);
        Self::write_ok_val(env, b, mload64!(arg1.wrapping_add(8) as usize));
        'label0: {
            {
                let a_i32_32_2 = mload32!(a.wrapping_add(32) as usize);
                if a_i32_32_2 == 0 {
                    let arg1_lo = mload64!(arg1.wrapping_add(16) as usize);
                    Self::write_ok_val(env, b, arg1_lo);
                    let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                    if a_i32_32 == 0 {
                        let f = Self::map_new_val(
                            env,
                            1048932,
                            3,
                            a.wrapping_add(8),
                            3,
                        );
                        break 'label0;
                    }
                }
            }
        }
    }


    fn result_from_val(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -16;
        b = arg0.wrapping_add(8);
        let e: i64;
        'label0: {
            if mload32!(arg0 as usize) == 0 {
                Self::func51(env, a, b);
                if mload32!(a as usize) == 0 {
                    e = mload64!(a.wrapping_add(8) as usize);
                    break 'label0;
                }
                unreachable!();
            }
            e = mload64!(b as usize);
        }
        e
    }

    fn require_owner_auth(
        env: &Env,
        arg0: i32,
    ) {
        Self::require_auth_for_key(
            env,
            arg0,
            1048592,
            5,
        );
    }

    fn require_auth_for_key(
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
            let d = Self::call_eq_one(env, a, 0);
            if d == 0 {
                a = Error(Storage, MissingValue);
            } else {
                let e = val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(a)).unwrap_or(val_from_i64(0)));
                a = e;
                address_from_i64(&env, a).require_auth();
            }
        }
    }


    fn vec_pair_iter(
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
        b = -16;
        {
            let mut svarg1_8_i32 = mload32!(arg1.wrapping_add(8) as usize);
            c = svarg1_8_i32;
            if c as u32 >= mload32!(arg1.wrapping_add(12) as usize) as u32 {
                let mut svarg0_0_i64 = 0 as i64;
            } else {
                f = 1;
                d = Error(Value, UnexpectedType);
                {
                    let arg1_lo = mload64!(arg1 as usize);
                    let h = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1_lo)).get_unchecked((c as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
                    e = h;
                    while a != 16 {
                        mstore64!(a.wrapping_add(b) as usize, 0 as u64);
                        a = a.wrapping_add(8);
                    }
                    let i = 0;
                    d = mload64!(b as usize);
                    e = mload64!(b.wrapping_add(8) as usize);
                    a = e as i32 & 255;
                    if (a == 12) as i32 & a != 70 {
                        f = 0;
                    } else {
                        d = Error(Value, UnexpectedType);
                    }
                }
                a = c += 1;
                if a != 0 {
                    let svarg0_0_i64 = f as i64;
                    let svarg1_8_i32 = a as i32;
                } else {
                    unreachable!();
                }
            }
        }
    }

    fn ledger_timestamp_val(
        env: &Env,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = -32;
        let e: i64;
        'label0: {
            let f = env.ledger().timestamp() as i64;
            c = f;
            b = c as i32 & 255;
            if b != 64 {
                let g = (c as u64) as i64;
                if b == 6 {
                    e = g;
                    break 'label0;
                }
                unreachable!();
            }
            let h = val_from_i64(c).as_u64().unwrap_or(0) as i64;
            e = h;
        }
        Self::func130(
            env,
            (c as u64) as i64,
            e,
            1000,
        );
        let value_lo = mload64!(a.wrapping_add(16) as usize);
        if value_lo == 0 {
            return mload64!(a.wrapping_add(8) as usize);
        }
        a.wrapping_add(8)
    }

    fn func59(
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
        let mut am: i32 = 0;
        let mut an: i64 = 0;
        let mut ao: i64 = 0;
        let mut ap: i64 = 0;
        let mut aq: i64 = 0;
        let mut ar: i64 = 0;
        let mut at: i64 = 0;
        let mut au: i64 = 0;
        let mut av: i64 = 0;
        a = -576;
        let ax = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64;
        ao = ax;
        let mut sv3_240_i32 = 0 as i32;
        let mut sv3_232_i64 = arg1 as i64;
        mstore32!(a.wrapping_add(244) as usize, (ao as u64).wrapping_shr(32 as u32) as i64 as u32);
        Self::vec_next_string_to_linear(env, a.wrapping_add(480), a.wrapping_add(232));
        let ay: i32;
        {
            let az = mload8!(a.wrapping_add(480) as usize) as i32;
            if az == 0 {
                n = 1;
                ay = 0;
            } else {
                b = a.wrapping_add(240);
                Self::span_from_range(
                    env,
                    a.wrapping_add(392),
                    mload32!(b as usize),
                    mload32!(a.wrapping_add(244) as usize),
                );
                c = mload32!(a.wrapping_add(392) as usize) += 1;
                if c != 0 {
                    c = c;
                } else {
                    c = -1;
                }
                Self::alloc_range(
                    env,
                    a.wrapping_add(72),
                    (if (c as u32 <= 4 as u32) as i32 != 0 { 4 } else { c }),
                    1,
                    32,
                );
                let ba = mload64!(a.wrapping_add(489) as usize);
                arg1 = ba;
                let bb = mload64!(a.wrapping_add(497) as usize);
                ao = bb;
                let bc = mload64!(a.wrapping_add(505) as usize);
                an = bc;
                c = mload32!(a.wrapping_add(72) as usize);
                n = mload32!(a.wrapping_add(76) as usize);
                mstore64!(n.wrapping_add(24) as usize, an as u64);
                mstore64!(n.wrapping_add(16) as usize, ao as u64);
                mstore64!(n.wrapping_add(8) as usize, arg1 as u64);
                let mut sv3_456_i32 = 1 as i32;
                let mut sv3_452_i32 = n as i32;
                let mut sv3_448_i32 = c as i32;
                let mut sv4_0_i64 = mload64!(b as usize);
                mstore64!(a.wrapping_add(400) as usize, sv4_0_i64 as u64);
                let mut sv3_232_i64 = mload64!(a.wrapping_add(232) as usize);
                let mut sv3_392_i64 = sv3_232_i64 as i64;
                b = a.wrapping_add(481);
                e = 32;
                k = 1;
                loop {
                    Self::vec_next_string_to_linear(env, a.wrapping_add(480), a.wrapping_add(392));
                    let bd = mload8!(a.wrapping_add(480) as usize) as i32;
                    if bd == 1 {
                        let mut sv3_448_i32 = mload32!(a.wrapping_add(448) as usize);
                        if sv3_448_i32 == k {
                            let mut sv3_400_i32 = mload32!(a.wrapping_add(400) as usize);
                            Self::span_from_range(
                                env,
                                a.wrapping_add(544),
                                sv3_400_i32,
                                mload32!(a.wrapping_add(404) as usize),
                            );
                            let mut sv3_544_i32 = mload32!(a.wrapping_add(544) as usize);
                            c = sv3_544_i32 += 1;
                            Self::require_alloc(
                                env,
                                a.wrapping_add(448),
                                k,
                                (if c != 0 { c } else { -1 }),
                                32,
                            );
                            let mut sv3_452_i32 = mload32!(a.wrapping_add(452) as usize);
                            n = sv3_452_i32;
                        }
                        c = e.wrapping_add(n);
                        let mut sv4_0_i64 = mload64!(b as usize);
                        let mut sv5_0_i64 = sv4_0_i64 as i64;
                        let be = mload64!(b.wrapping_add(24) as usize);
                        mstore64!(c.wrapping_add(24) as usize, be as u64);
                        let bf = mload64!(b.wrapping_add(16) as usize);
                        mstore64!(c.wrapping_add(16) as usize, bf as u64);
                        let bg = mload64!(b.wrapping_add(8) as usize);
                        mstore64!(c.wrapping_add(8) as usize, bg as u64);
                        k += 1;
                        let mut sv3_456_i32 = k as i32;
                        e = e.wrapping_add(32);
                    }
                }
                let mut sv3_448_i32 = mload32!(a.wrapping_add(448) as usize);
                ay = sv3_448_i32;
            }
        }
        i = ay;
        let bh = Self::ledger_timestamp_val(env);
        arg1 = bh;
        Self::alloc_range(
            env,
            a.wrapping_sub(-64),
            5,
            1,
            32,
        );
        b = 0;
        let mut sv3_240_i32 = 0 as i32;
        f = mload32!(a.wrapping_add(68) as usize);
        let mut sv3_236_i32 = f as i32;
        c = mload32!(a.wrapping_add(64) as usize);
        let mut sv3_232_i32 = c as i32;
        if c as u32 <= 4 as u32 {
            Self::require_alloc(
                env,
                a.wrapping_add(232),
                0,
                5,
                32,
            );
            let mut sv3_240_i32 = mload32!(a.wrapping_add(240) as usize);
            g = sv3_240_i32;
            let mut sv3_236_i32 = mload32!(a.wrapping_add(236) as usize);
            f = sv3_236_i32;
        }
        d = g.wrapping_shl(5 as u32);
        e = 1048752;
        loop {
            c = a.wrapping_add(392);
            Self::alloc_copy(
                env,
                c,
                e,
                20,
            );
            Self::build_entry_from_bytes(env, a.wrapping_add(480), c);
            c = d.wrapping_add(f);
            let bi = mload64!(a.wrapping_add(504) as usize);
            mstore64!(c.wrapping_add(24) as usize, bi as u64);
            let bj = mload64!(a.wrapping_add(496) as usize);
            mstore64!(c.wrapping_add(16) as usize, bj as u64);
            let bk = mload64!(a.wrapping_add(488) as usize);
            mstore64!(c.wrapping_add(8) as usize, bk as u64);
            let mut sv3_480_i64 = mload64!(a.wrapping_add(480) as usize);
            let mut sv5_0_i64 = sv3_480_i64 as i64;
            e = e.wrapping_add(20);
            d = d.wrapping_add(32);
            b += 1;
        }
        e = 3;
        'label3: {
            'label4: {
                let bl: i64;
                'label5: {
                    r = b.wrapping_add(g);
                    if r as u32 >= 3 as u32 {
                        if r as u32 > 255 as u32 {
                            ao = 0;
                            an = 255;
                            b = r;
                            bl = 0;
                            break 'label5;
                        }
                        let mut sv3_232_i32 = mload32!(a.wrapping_add(232) as usize);
                        h = sv3_232_i32;
                        g = f.wrapping_add(r.wrapping_shl(5 as u32));
                        e = f;
                        {
                            loop {
                                c = e;
                                if d == 0 {
                                    unreachable!();
                                }
                                d = d.wrapping_sub(32);
                                e = c.wrapping_add(32);
                                let bm = Self::memeq32(env, c, 1049290);
                            }
                            an = mload64!(c.wrapping_add(7) as usize);
                            ao = an & 18446744069414584320;
                            let bn = mload16!(c as usize) as i32;
                            let bo = mload8!(c.wrapping_add(2) as usize) as i32;
                            g = bo;
                            d = bn | g.wrapping_shl(16 as u32);
                            e = (d as u32).wrapping_shr(8 as u32) as i32;
                            let bp = mload8!(c.wrapping_add(31) as usize) as i32;
                            r = bp;
                            f = mload32!(c.wrapping_add(27) as usize);
                            h = mload32!(c.wrapping_add(23) as usize);
                            ap = mload64!(c.wrapping_add(15) as usize);
                            b = mload32!(c.wrapping_add(3) as usize);
                            bl = 0 /* Symbol() */;
                            break 'label5;
                        }
                        b = f;
                        'label8: loop {
                            if b == g {
                                break 'label4;
                            }
                            j = b.wrapping_add(32);
                            c = f;
                            l += 1;
                            e = l;
                            loop {
                                'label10: {
                                    'label11: {
                                        if e == 0 {
                                            d = c;
                                            if c == g {
                                                break 'label11;
                                            }
                                            break 'label10;
                                        }
                                        d = c.wrapping_add(e.wrapping_shl(5 as u32));
                                        if (g.wrapping_sub(c) as u32).wrapping_shr(5 as u32) as i32 as u32 > e as u32 {
                                            break 'label10;
                                        }
                                    }
                                    b = j;
                                    continue 'label8;
                                }
                                c = d.wrapping_add(32);
                                e = 0;
                                let bq = Self::memeq32(env, b, d);
                            }
                        }
                        an = mload64!(b.wrapping_add(7) as usize);
                        ao = an & 18446744069414584320;
                        let br = mload16!(b as usize) as i32;
                        let bs = mload8!(b.wrapping_add(2) as usize) as i32;
                        g = bs;
                        d = br | g.wrapping_shl(16 as u32);
                        e = (d as u32).wrapping_shr(8 as u32) as i32;
                        let bt = mload8!(b.wrapping_add(31) as usize) as i32;
                        r = bt;
                        f = mload32!(b.wrapping_add(27) as usize);
                        let mut sv4_23_i32 = mload32!(b.wrapping_add(23) as usize);
                        h = sv4_23_i32;
                        ap = mload64!(b.wrapping_add(15) as usize);
                        b = mload32!(b.wrapping_add(3) as usize);
                        bl = 15;
                        break 'label5;
                    }
                    an = 0;
                    ao = 0;
                    d = r;
                    bl = 0;
                }
                arg1 = bl;
                e = d & 255 | g.wrapping_shl(16 as u32) | (e & 255).wrapping_shl(8 as u32);
                g = (ao as u64).wrapping_shr(32 as u32) as i64 as i32;
                c = an as i32;
                d = 1;
                break 'label3;
            }
            d = 1;
            c = 255;
            if k as u32 > 255 as u32 {
                arg1 = 17;
                b = k;
                break 'label3;
            }
            b = k.wrapping_shl(5 as u32);
            l = n.wrapping_add(b);
            c = 0;
            'label12: {
                'label13: {
                    loop {
                        if b == c {
                            break 'label13;
                        }
                        let bu = c;
                        c = c.wrapping_add(32);
                        let bv = Self::memeq32(env, bu.wrapping_add(n), 1049290);
                    }
                    b = c.wrapping_add(n).wrapping_sub(32);
                    arg1 = 18;
                    break 'label12;
                }
                'label15: {
                    {
                        let bw = Self::memeq32(env, n, n.wrapping_add(32));
                        if bw == 0 {
                            unreachable!();
                        }
                        arg1 = 19;
                        b = n;
                        break 'label12;
                    }
                    b = n;
                    'label18: loop {
                        if b == l {
                            break 'label15;
                        }
                        j = b.wrapping_add(32);
                        c = n;
                        o += 1;
                        g = o;
                        loop {
                            'label20: {
                                'label21: {
                                    if g == 0 {
                                        e = c;
                                        if l == e {
                                            break 'label21;
                                        }
                                        break 'label20;
                                    }
                                    e = c.wrapping_add(g.wrapping_shl(5 as u32));
                                    if (l.wrapping_sub(c) as u32).wrapping_shr(5 as u32) as i32 as u32 > g as u32 {
                                        break 'label20;
                                    }
                                }
                                b = j;
                                continue 'label18;
                            }
                            c = e.wrapping_add(32);
                            g = 0;
                            let bx = Self::memeq32(env, b, e);
                        }
                    }
                    arg1 = 19;
                    break 'label12;
                }
                b = (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32;
                e = (arg1 as u64) as i64 as i32;
                g = 0;
                ap = 180000;
                c = 180000;
                d = 0;
                break 'label3;
            }
            let by = mload16!(b as usize) as i32;
            let bz = mload8!(b.wrapping_add(2) as usize) as i32;
            e = by | bz.wrapping_shl(16 as u32);
            let ca = mload8!(b.wrapping_add(31) as usize) as i32;
            r = ca;
            f = mload32!(b.wrapping_add(27) as usize);
            let mut sv4_23_i32 = mload32!(b.wrapping_add(23) as usize);
            h = sv4_23_i32;
            ap = mload64!(b.wrapping_add(15) as usize);
            let mut sv4_11_i32 = mload32!(b.wrapping_add(11) as usize);
            g = sv4_11_i32;
            c = mload32!(b.wrapping_add(7) as usize);
            b = mload32!(b.wrapping_add(3) as usize);
        }
        arg1 = arg1 & 255 | (b as u32 as i64).wrapping_shl(32 as u32) | (e as u32 as i64 & 16777215);
        av = c as u32 as i64 | (g as u32 as i64).wrapping_shl(32 as u32);
        'label22: {
            'label23: {
                'label24: {
                    'label25: {
                        'label26: {
                            'label27: {
                                'label28: {
                                    b = a.wrapping_add(480);
                                    let cb = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64;
                                    Self::alloc_range_one(env, b, (cb as u64).wrapping_shr(32 as u32) as i64 as i32);
                                    let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                    c = sv3_484_i32;
                                    {
                                        d = mload32!(a.wrapping_add(488) as usize);
                                        let cc = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64;
                                        if d == (cc as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                            Self::copy_to_linear_memory(
                                                env,
                                                arg2,
                                                c,
                                                d,
                                            );
                                            let cd = mload32!(a.wrapping_add(488) as usize);
                                            mstore32!(a.wrapping_add(88) as usize, cd as u32);
                                            let mut sv3_480_i64 = mload64!(a.wrapping_add(480) as usize);
                                            j = a.wrapping_add(80);
                                            Self::span_take(
                                                env,
                                                b,
                                                j,
                                                9,
                                            );
                                            e = mload32!(a.wrapping_add(492) as usize);
                                            m = mload32!(a.wrapping_add(488) as usize);
                                            let ce = mload8!(a.wrapping_add(480) as usize) as i32;
                                            d = ce;
                                            if d != 27 {
                                                break 'label28;
                                            }
                                            let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                            c = sv3_484_i32;
                                            d = 5;
                                            g = 0;
                                            if e != 9 {
                                                break 'label24;
                                            }
                                            let cf = Self::memcmp(
                                                env,
                                                m,
                                                1049386,
                                                9,
                                            );
                                            if cf != 0 {
                                                unreachable!();
                                            }
                                            Self::func66(
                                                env,
                                                b,
                                                j,
                                                3,
                                            );
                                            'label30: {
                                                let cg = mload8!(a.wrapping_add(480) as usize) as i32;
                                                d = cg;
                                                if d == 27 {
                                                    arg2 = mload64!(a.wrapping_add(488) as usize);
                                                    if arg2 as u64 <= 4294967295 as u64 {
                                                        Self::span_take(
                                                            env,
                                                            b,
                                                            j,
                                                            arg2 as i32,
                                                        );
                                                        let ch = mload8!(a.wrapping_add(480) as usize) as i32;
                                                        d = ch;
                                                        if d != 27 {
                                                            break 'label25;
                                                        }
                                                        Self::func66(
                                                            env,
                                                            b,
                                                            j,
                                                            2,
                                                        );
                                                        let ci = mload8!(a.wrapping_add(480) as usize) as i32;
                                                        d = ci;
                                                        if d != 27 {
                                                            break 'label25;
                                                        }
                                                        arg2 = mload64!(a.wrapping_add(488) as usize);
                                                        if arg2 as u64 <= 4294967295 as u64 {
                                                            break 'label30;
                                                        }
                                                    }
                                                    d = 23;
                                                    break 'label23;
                                                }
                                                break 'label25;
                                            }
                                            z = arg2 as i32;
                                            Self::alloc_range(
                                                env,
                                                a.wrapping_add(56),
                                                z,
                                                8,
                                                56,
                                            );
                                            mload64!(a.wrapping_add(56) as usize);
                                            x = a.wrapping_add(513);
                                            s = a.wrapping_add(481);
                                            u = a.wrapping_add(520);
                                            p = a.wrapping_add(260);
                                            w = a.wrapping_add(535);
                                            ab = a.wrapping_add(512);
                                            ac = a.wrapping_add(232) | 1;
                                            ad = a.wrapping_add(492);
                                            ae = a.wrapping_add(245);
                                            af = ae.wrapping_add(7);
                                            t = a.wrapping_add(502);
                                            ag = a.wrapping_add(490);
                                            ah = a.wrapping_add(510);
                                            d = 0;
                                            'label31: loop {
                                                'label32: {
                                                    let cj: i32;
                                                    'label33: {
                                                        'label34: {
                                                            'label35: {
                                                                'label36: {
                                                                    'label38: {
                                                                        'label39: {
                                                                            {
                                                                                if y != z {
                                                                                    b = a.wrapping_add(480);
                                                                                    j = a.wrapping_add(80);
                                                                                    Self::span_take(
                                                                                        env,
                                                                                        b,
                                                                                        j,
                                                                                        65,
                                                                                    );
                                                                                    'label41: {
                                                                                        let ck = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                        d = ck;
                                                                                        if d == 27 {
                                                                                            ao = mload64!(a.wrapping_add(488) as usize);
                                                                                            Self::alloc_copy(
                                                                                                env,
                                                                                                a.wrapping_add(212),
                                                                                                mload32!(a.wrapping_add(84) as usize),
                                                                                                mload32!(a.wrapping_add(88) as usize),
                                                                                            );
                                                                                            Self::func66(
                                                                                                env,
                                                                                                b,
                                                                                                j,
                                                                                                3,
                                                                                            );
                                                                                            let cl = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                            d = cl;
                                                                                            if d != 27 {
                                                                                                break 'label27;
                                                                                            }
                                                                                            at = mload64!(a.wrapping_add(488) as usize);
                                                                                            Self::func66(
                                                                                                env,
                                                                                                b,
                                                                                                j,
                                                                                                4,
                                                                                            );
                                                                                            let cm = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                            d = cm;
                                                                                            if d != 27 {
                                                                                                break 'label27;
                                                                                            }
                                                                                            aq = mload64!(a.wrapping_add(488) as usize);
                                                                                            Self::func66(
                                                                                                env,
                                                                                                b,
                                                                                                j,
                                                                                                6,
                                                                                            );
                                                                                            let cn = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                            d = cn;
                                                                                            if d == 27 {
                                                                                                break 'label41;
                                                                                            }
                                                                                            break 'label27;
                                                                                        }
                                                                                        break 'label27;
                                                                                    }
                                                                                    an = aq.wrapping_add(32);
                                                                                    if (an as u64) < aq as u64 {
                                                                                        unreachable!();
                                                                                    }
                                                                                    au = mload64!(a.wrapping_add(488) as usize);
                                                                                    Self::func130(
                                                                                        env,
                                                                                        a.wrapping_add(40),
                                                                                        at,
                                                                                        an,
                                                                                    );
                                                                                    let a_part_48 = mload64!(a.wrapping_add(48) as usize);
                                                                                    if a_part_48 != 0 {
                                                                                        unreachable!();
                                                                                    }
                                                                                    ar = mload64!(a.wrapping_add(40) as usize);
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
                                                                                        Self::span_take(
                                                                                            env,
                                                                                            b,
                                                                                            a.wrapping_add(212),
                                                                                            an as i32,
                                                                                        );
                                                                                        let co = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                        d = co;
                                                                                        if d != 27 {
                                                                                            break 'label27;
                                                                                        }
                                                                                        an = (ao as u64).wrapping_shr(32 as u32) as i64;
                                                                                        if an == 65 /* I64(obj#0) */ {
                                                                                            an = mload64!(a.wrapping_add(488) as usize);
                                                                                            j = s.wrapping_add(24);
                                                                                            d = ao as i32;
                                                                                            let cp = mload64!(d.wrapping_add(24) as usize);
                                                                                            let mut sv12_0_i64 = cp as i64;
                                                                                            m = s.wrapping_add(16);
                                                                                            let cq = mload64!(d.wrapping_add(16) as usize);
                                                                                            let mut sv15_0_i64 = cq as i64;
                                                                                            q = s.wrapping_add(8);
                                                                                            let cr = mload64!(d.wrapping_add(8) as usize);
                                                                                            let mut sv19_0_i64 = cr as i64;
                                                                                            mstore8!(a.wrapping_add(480) as usize, 0 as u8);
                                                                                            e = a.wrapping_add(448);
                                                                                            Self::entry_copy_if_ok(env, e, b);
                                                                                            let cs = mload64!(d.wrapping_add(56) as usize);
                                                                                            let sv12_0_i64 = cs as i64;
                                                                                            let ct = mload64!(d.wrapping_add(48) as usize);
                                                                                            let sv15_0_i64 = ct as i64;
                                                                                            let cu = mload64!(d.wrapping_add(40) as usize);
                                                                                            let mut sv19_0_i64 = cu as i64;
                                                                                            mload64!(d.wrapping_add(32) as usize);
                                                                                            mstore8!(a.wrapping_add(480) as usize, 0 as u8);
                                                                                            j = a.wrapping_add(544);
                                                                                            Self::entry_copy_if_ok(env, j, b);
                                                                                            let cv = Self::memeq32(env, e, 1049290);
                                                                                            b = cv;
                                                                                            let cw = Self::memeq32(env, j, 1049290);
                                                                                            m = cw;
                                                                                            {
                                                                                                let cx = Self::memcmp_sign32(env, e, 1049322);
                                                                                                let cy = Self::memcmp_sign32(env, j, 1049354);
                                                                                                if (((cx & 255) as u32) < 2 as u32) as i32 | b | m | !(cy & 255 == 1) {
                                                                                                    let mut sv3_376_i32 = -2147483644 as i32;
                                                                                                } else {
                                                                                                    Self::func69(
                                                                                                        env,
                                                                                                        a.wrapping_add(376),
                                                                                                        d,
                                                                                                        65,
                                                                                                    );
                                                                                                    let mut sv3_376_i32 = mload32!(a.wrapping_add(376) as usize);
                                                                                                    e = sv3_376_i32;
                                                                                                    if e != -2147483644 {
                                                                                                        break 'label38;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            {
                                                                                                let cz = mload8!(d.wrapping_add(64) as usize) as i32;
                                                                                                e = cz;
                                                                                                if (e as u32) < 2 as u32 {
                                                                                                    b = e;
                                                                                                } else {
                                                                                                    b = e.wrapping_sub(27);
                                                                                                    if b as u32 > 1 as u32 {
                                                                                                        break 'label39;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            let da = val_to_i64(Bytes::new(env).into_val(env));
                                                                                            let db = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(da))).into());
                                                                                            Self::entry_from_bytes_val(env, a.wrapping_add(392), db);
                                                                                            j = a.wrapping_add(480);
                                                                                            Self::memmove(
                                                                                                env,
                                                                                                j,
                                                                                                d,
                                                                                                64,
                                                                                            );
                                                                                            let dd = val_to_i64(Bytes::new(env).into_val(env));
                                                                                            ao = dd;
                                                                                            let a_part_424 = mload64!(a.wrapping_add(424) as usize);
                                                                                            let de = val_to_i64(env.crypto().secp256k1_recover(&Hash::<32>::from_val(env, &val_from_i64(a_part_424)), &BytesN::<64>::from_val(env, &val_from_i64(ao)), (b as u32 as i64 & 255).wrapping_shl(32 as u32) | 0 as u32).into());
                                                                                            an = de;
                                                                                            d = a.wrapping_add(544);
                                                                                            Self::alloc_range_one(env, d, 65);
                                                                                            let mut sv3_548_i32 = mload32!(a.wrapping_add(548) as usize);
                                                                                            e = sv3_548_i32;
                                                                                            let mut sv3_552_i32 = mload32!(a.wrapping_add(552) as usize);
                                                                                            b = sv3_552_i32;
                                                                                            let df = Bytes::from_val(env, &val_from_i64(an)).len() as i64;
                                                                                            if b != (df as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                                                                unreachable!();
                                                                                            }
                                                                                            Self::copy_to_linear_memory(
                                                                                                env,
                                                                                                an,
                                                                                                e,
                                                                                                b,
                                                                                            );
                                                                                            if b == 0 {
                                                                                                unreachable!();
                                                                                            }
                                                                                            m = 1;
                                                                                            let dg = val_to_i64(Bytes::new(env).into_val(env));
                                                                                            let dh = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(dg))).into());
                                                                                            Self::entry_from_bytes_val(env, j, dh);
                                                                                            Self::alloc_copy(
                                                                                                env,
                                                                                                d,
                                                                                                ad,
                                                                                                20,
                                                                                            );
                                                                                            Self::build_entry_from_bytes(env, ac, d);
                                                                                            let di = mload8!(p.wrapping_add(4) as usize) as i32;
                                                                                            mstore8!(a.wrapping_add(228) as usize, di as u8);
                                                                                            let mut sv3_311_i64 = mload64!(af as usize);
                                                                                            let dj = mload8!(a.wrapping_add(244) as usize) as i32;
                                                                                            j = dj;
                                                                                            ao = mload64!(a.wrapping_add(236) as usize);
                                                                                            let dk = mload16!(a.wrapping_add(233) as usize) as i32;
                                                                                            let dl = mload8!(a.wrapping_add(235) as usize) as i32;
                                                                                            cj = dk | dl.wrapping_shl(16 as u32);
                                                                                            break 'label33;
                                                                                        }
                                                                                        mstore32!(a.wrapping_add(240) as usize, an as u32);
                                                                                        let mut sv3_236_i32 = -2147483645 as i32;
                                                                                        break 'label34;
                                                                                    }
                                                                                    d = 23;
                                                                                    break 'label32;
                                                                                }
                                                                                arg2 = mload64!(a.wrapping_add(120) as usize);
                                                                                j = mload32!(a.wrapping_add(116) as usize);
                                                                                c = mload32!(a.wrapping_add(88) as usize);
                                                                                if c != 0 {
                                                                                    d = 6;
                                                                                    break 'label23;
                                                                                }
                                                                                if (arg2 as u64) < 4294967296 as u64 {
                                                                                    d = 2;
                                                                                    break 'label36;
                                                                                }
                                                                                d = arg2 as i32;
                                                                                an = mload64!(d as usize);
                                                                                ao = an.wrapping_add(av);
                                                                                if arg1 as u64 > ((if an as u64 > ao as u64 { 18446744073709551615 } else { ao })) as u64 {
                                                                                    d = 9;
                                                                                    break 'label36;
                                                                                }
                                                                                ao = arg1.wrapping_add(ap);
                                                                                if (((if arg1 as u64 > ao as u64 { 18446744073709551615 } else { ao })) as u64) < an as u64 {
                                                                                    d = 10;
                                                                                    break 'label36;
                                                                                }
                                                                                g = ((arg2 as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_mul(56);
                                                                                p = d.wrapping_add(g);
                                                                                c = 56;
                                                                                'label44: {
                                                                                    loop {
                                                                                        if c == g {
                                                                                            break 'label44;
                                                                                        }
                                                                                        b = c.wrapping_add(d);
                                                                                        c = c.wrapping_add(56);
                                                                                        arg1 = mload64!(b as usize);
                                                                                    }
                                                                                    d = 20;
                                                                                    break 'label36;
                                                                                }
                                                                                ao = r as u32 as i64;
                                                                                arg1 = ao.wrapping_mul(k as u32 as i64);
                                                                                if (arg1 as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                                                                    unreachable!();
                                                                                }
                                                                                b = arg1 as i32;
                                                                                Self::alloc_range_fill(
                                                                                    env,
                                                                                    a.wrapping_add(16),
                                                                                    b,
                                                                                    33,
                                                                                );
                                                                                let sv3_240_i32 = 0 as i32;
                                                                                c = mload32!(a.wrapping_add(20) as usize);
                                                                                let mut sv3_236_i32 = c as i32;
                                                                                e = mload32!(a.wrapping_add(16) as usize);
                                                                                let mut sv3_232_i32 = e as i32;
                                                                                o = 0;
                                                                                if b as u32 > e as u32 {
                                                                                    Self::require_alloc(
                                                                                        env,
                                                                                        a.wrapping_add(232),
                                                                                        0,
                                                                                        b,
                                                                                        33,
                                                                                    );
                                                                                    o = mload32!(a.wrapping_add(240) as usize);
                                                                                    let mut sv3_236_i32 = mload32!(a.wrapping_add(236) as usize);
                                                                                    c = sv3_236_i32;
                                                                                }
                                                                                c = c.wrapping_add(o.wrapping_mul(33));
                                                                                if (b as u32 <= 1 as u32) as i32 != 0 {
                                                                                    h = 1;
                                                                                } else {
                                                                                    h = b;
                                                                                }
                                                                                e = h -= 1;
                                                                                l = a.wrapping_add(488);
                                                                                m = a.wrapping_add(496);
                                                                                i = a.wrapping_add(504);
                                                                                'label46: {
                                                                                    while e != 0 {
                                                                                        mstore8!(c as usize, 0 as u8);
                                                                                        let mut sv3_480_i64 = mload64!(a.wrapping_add(480) as usize);
                                                                                        mstore64!(c.wrapping_add(9) as usize, mload64!(l as usize) as u64);
                                                                                        mstore64!(c.wrapping_add(17) as usize, mload64!(m as usize) as u64);
                                                                                        mstore64!(c.wrapping_add(25) as usize, mload64!(i as usize) as u64);
                                                                                        e -= 1;
                                                                                        c = c.wrapping_add(33);
                                                                                    } else {
                                                                                        s = h.wrapping_add(o);
                                                                                        if b == 0 {
                                                                                            s -= 1;
                                                                                            break 'label46;
                                                                                        }
                                                                                    }
                                                                                    mstore8!(c as usize, 0 as u8);
                                                                                    let mut sv3_392_i64 = mload64!(a.wrapping_add(392) as usize);
                                                                                    let dm = mload64!(a.wrapping_add(400) as usize);
                                                                                    mstore64!(c.wrapping_add(9) as usize, dm as u64);
                                                                                    let dn = mload64!(a.wrapping_add(408) as usize);
                                                                                    mstore64!(c.wrapping_add(17) as usize, dn as u64);
                                                                                    let do = mload64!(a.wrapping_add(416) as usize);
                                                                                    mstore64!(c.wrapping_add(25) as usize, do as u64);
                                                                                }
                                                                                x = sv3_236_i32;
                                                                                let mut sv3_384_i32 = j as i32;
                                                                                let sv3_376_i32 = d as i32;
                                                                                m = k.wrapping_shl(5 as u32);
                                                                                q = r.wrapping_shl(5 as u32);
                                                                                g = a.wrapping_add(512);
                                                                                o = a.wrapping_add(413);
                                                                                w = a.wrapping_add(404);
                                                                                'label49: loop {
                                                                                    'label50: {
                                                                                        if d != p {
                                                                                            l = d.wrapping_add(56);
                                                                                            i = mload32!(d.wrapping_add(8) as usize);
                                                                                            if i != -2147483648 {
                                                                                                break 'label50;
                                                                                            }
                                                                                        }
                                                                                        sv3_384_i32 = 0 as i32;
                                                                                        j = a.wrapping_add(233);
                                                                                        l = a.wrapping_add(393);
                                                                                        g = a.wrapping_add(481);
                                                                                        q = a.wrapping_add(512);
                                                                                        w = 1;
                                                                                        m = 0;
                                                                                        c = 0;
                                                                                        o = 0;
                                                                                        'label51: loop {
                                                                                            'label52: {
                                                                                                'label53: {
                                                                                                    'label54: {
                                                                                                        'label55: {
                                                                                                            {
                                                                                                                let dp: i32;
                                                                                                                if k as u32 <= o as u32 {
                                                                                                                    e = a.wrapping_add(192);
                                                                                                                    h = c;
                                                                                                                    dp = 0;
                                                                                                                } else {
                                                                                                                    arg1 = (o as u32 as i64).wrapping_mul(ao);
                                                                                                                    if (arg1 as u64).wrapping_shr(32 as u32) as (i64 != 0) {
                                                                                                                        unreachable!();
                                                                                                                    }
                                                                                                                    b = arg1 as i32;
                                                                                                                    d = b.wrapping_add(r);
                                                                                                                    if (d as u32) < b as u32 {
                                                                                                                        unreachable!();
                                                                                                                    }
                                                                                                                    if d as u32 > s as u32 {
                                                                                                                        unreachable!();
                                                                                                                    }
                                                                                                                    h = c += 1;
                                                                                                                    if h == 0 {
                                                                                                                        unreachable!();
                                                                                                                    }
                                                                                                                    o += 1;
                                                                                                                    let sv3_192_i32 = x.wrapping_add(b.wrapping_mul(33)) as i32;
                                                                                                                    e = a.wrapping_add(152);
                                                                                                                    u = c;
                                                                                                                    dp = x.wrapping_add(d.wrapping_mul(33));
                                                                                                                }
                                                                                                                b = dp;
                                                                                                                c = sv3_192_i32;
                                                                                                                if c != 0 {
                                                                                                                    let mut sv3_232_i32 = 0 as i32;
                                                                                                                    Self::memcpy_like(env, a.wrapping_add(480), a.wrapping_add(232));
                                                                                                                    let dq = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                                                    if dq == 0 {
                                                                                                                        unreachable!();
                                                                                                                    }
                                                                                                                    e = 32;
                                                                                                                    Self::alloc_range_fill(
                                                                                                                        env,
                                                                                                                        a.wrapping_add(8),
                                                                                                                        4,
                                                                                                                        32,
                                                                                                                    );
                                                                                                                    p = g.wrapping_add(8);
                                                                                                                    arg1 = mload64!(p as usize);
                                                                                                                    i = g.wrapping_add(16);
                                                                                                                    arg2 = mload64!(i as usize);
                                                                                                                    t = g.wrapping_add(24);
                                                                                                                    ap = mload64!(t as usize);
                                                                                                                    b = mload32!(a.wrapping_add(8) as usize);
                                                                                                                    f = mload32!(a.wrapping_add(12) as usize);
                                                                                                                    let mut sv9_0_i64 = mload64!(g as usize);
                                                                                                                    mstore64!(f.wrapping_add(24) as usize, ap as u64);
                                                                                                                    mstore64!(f.wrapping_add(16) as usize, arg2 as u64);
                                                                                                                    mstore64!(f.wrapping_add(8) as usize, arg1 as u64);
                                                                                                                    d = 1;
                                                                                                                    let mut sv3_552_i32 = 1 as i32;
                                                                                                                    let sv3_548_i32 = f as i32;
                                                                                                                    let sv3_544_i32 = b as i32;
                                                                                                                    let dr = mload64!(a.wrapping_add(248) as usize);
                                                                                                                    mstore64!(a.wrapping_add(408) as usize, dr as u64);
                                                                                                                    let ds = mload64!(a.wrapping_add(240) as usize);
                                                                                                                    mstore64!(a.wrapping_add(400) as usize, ds as u64);
                                                                                                                    let mut sv3_232_i64 = mload64!(a.wrapping_add(232) as usize);
                                                                                                                    let mut sv3_392_i64 = sv3_232_i64 as i64;
                                                                                                                    b = 0;
                                                                                                                    loop {
                                                                                                                        Self::memcpy_like(env, a.wrapping_add(480), a.wrapping_add(392));
                                                                                                                        let dt = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                                                        if dt == 1 {
                                                                                                                            if mload32!(a.wrapping_add(544) as usize) == d {
                                                                                                                                Self::require_alloc(
                                                                                                                                    env,
                                                                                                                                    a.wrapping_add(544),
                                                                                                                                    d,
                                                                                                                                    1,
                                                                                                                                    32,
                                                                                                                                );
                                                                                                                                f = mload32!(a.wrapping_add(548) as usize);
                                                                                                                            }
                                                                                                                            c = e.wrapping_add(f);
                                                                                                                            let mut sv9_0_i64 = mload64!(g as usize);
                                                                                                                            let mut sv5_0_i64 = sv9_0_i64 as i64;
                                                                                                                            mstore64!(c.wrapping_add(24) as usize, mload64!(t as usize) as u64);
                                                                                                                            mstore64!(c.wrapping_add(16) as usize, mload64!(i as usize) as u64);
                                                                                                                            mstore64!(c.wrapping_add(8) as usize, mload64!(p as usize) as u64);
                                                                                                                            d += 1;
                                                                                                                            let sv3_552_i32 = d as i32;
                                                                                                                            b = b.wrapping_add(32);
                                                                                                                            e = e.wrapping_add(32);
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if (d as u32) < 3 as u32 {
                                                                                                                        break 'label53;
                                                                                                                    }
                                                                                                                    'label60: {
                                                                                                                        {
                                                                                                                            {
                                                                                                                                if d as u32 >= 21 as u32 {
                                                                                                                                    i = a.wrapping_add(480);
                                                                                                                                    e = -16;
                                                                                                                                    'label65: {
                                                                                                                                        'label66: {
                                                                                                                                            'label67: {
                                                                                                                                                let dv = Self::memcmp_sign32(env, f.wrapping_add(32), f);
                                                                                                                                                if dv & 255 != 255 {
                                                                                                                                                    b = f.wrapping_sub(-64);
                                                                                                                                                    c = 2;
                                                                                                                                                    loop {
                                                                                                                                                        if c == d {
                                                                                                                                                            break 'label65;
                                                                                                                                                        }
                                                                                                                                                        let dw = Self::memcmp_sign32(env, b, b.wrapping_sub(32));
                                                                                                                                                        if dw & 255 == 255 {
                                                                                                                                                            break 'label67;
                                                                                                                                                        }
                                                                                                                                                        b = b.wrapping_add(32);
                                                                                                                                                        c += 1;
                                                                                                                                                    }
                                                                                                                                                    unreachable!();
                                                                                                                                                }
                                                                                                                                                b = f.wrapping_sub(-64);
                                                                                                                                                c = 2;
                                                                                                                                                loop {
                                                                                                                                                    if c == d {
                                                                                                                                                        break 'label66;
                                                                                                                                                    }
                                                                                                                                                    let dx = Self::memcmp_sign32(env, b, b.wrapping_sub(32));
                                                                                                                                                    if dx & 255 != 255 {
                                                                                                                                                        break 'label67;
                                                                                                                                                    }
                                                                                                                                                    b = b.wrapping_add(32);
                                                                                                                                                    c += 1;
                                                                                                                                                }
                                                                                                                                                unreachable!();
                                                                                                                                            }
                                                                                                                                            Self::func114(
                                                                                                                                                env,
                                                                                                                                                f,
                                                                                                                                                d,
                                                                                                                                                0,
                                                                                                                                                ((d | 1).leading_zeros() as i32).wrapping_shl(1 as u32) ^ 62,
                                                                                                                                                i,
                                                                                                                                            );
                                                                                                                                            break 'label65;
                                                                                                                                        }
                                                                                                                                        i = (d as u32).wrapping_shr(1 as u32) as i32;
                                                                                                                                        Self::func115(
                                                                                                                                            env,
                                                                                                                                            e.wrapping_add(8),
                                                                                                                                            i,
                                                                                                                                            f,
                                                                                                                                            i,
                                                                                                                                        );
                                                                                                                                        t = mload32!(e.wrapping_add(12) as usize);
                                                                                                                                        c = mload32!(e.wrapping_add(8) as usize);
                                                                                                                                        b = i.wrapping_shl(5 as u32);
                                                                                                                                        Self::func115(
                                                                                                                                            env,
                                                                                                                                            e,
                                                                                                                                            i,
                                                                                                                                            f.wrapping_add(d.wrapping_shl(5 as u32)).wrapping_sub(b),
                                                                                                                                            i,
                                                                                                                                        );
                                                                                                                                        p = b.wrapping_add(mload32!(e as usize)).wrapping_sub(32);
                                                                                                                                        b = 0;
                                                                                                                                        y = mload32!(e.wrapping_add(4) as usize);
                                                                                                                                        loop {
                                                                                                                                            z = b.wrapping_add(i);
                                                                                                                                            if z == 0 {
                                                                                                                                                unreachable!();
                                                                                                                                            }
                                                                                                                                            if b.wrapping_add(t) == 0 {
                                                                                                                                                unreachable!();
                                                                                                                                            }
                                                                                                                                            if y as u32 > z.wrapping_sub(1) as u32 {
                                                                                                                                                Self::memcpy_like_6(env, c, p);
                                                                                                                                                c = c.wrapping_add(32);
                                                                                                                                                p = p.wrapping_sub(32);
                                                                                                                                                b -= 1;
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        unreachable!();
                                                                                                                                    }
                                                                                                                                    break 'label55;
                                                                                                                                }
                                                                                                                                c = f.wrapping_add(32);
                                                                                                                                loop {
                                                                                                                                    unreachable!();
                                                                                                                                    Self::func74(env, f, c);
                                                                                                                                    b = b.wrapping_sub(32);
                                                                                                                                    c = c.wrapping_add(32);
                                                                                                                                }
                                                                                                                                unreachable!();
                                                                                                                                let dy = Self::ptr_index32(
                                                                                                                                    env,
                                                                                                                                    f,
                                                                                                                                    1,
                                                                                                                                    0,
                                                                                                                                );
                                                                                                                                b = dy;
                                                                                                                                let dz = mload64!(b.wrapping_add(24) as usize);
                                                                                                                                mstore64!(a.wrapping_add(504) as usize, dz as u64);
                                                                                                                                let ea = mload64!(b.wrapping_add(16) as usize);
                                                                                                                                mstore64!(a.wrapping_add(496) as usize, ea as u64);
                                                                                                                                let eb = mload64!(b.wrapping_add(8) as usize);
                                                                                                                                mstore64!(a.wrapping_add(488) as usize, eb as u64);
                                                                                                                                let mut sv4_0_i64 = mload64!(b as usize);
                                                                                                                                let mut sv3_480_i64 = sv4_0_i64 as i64;
                                                                                                                                break 'label54;
                                                                                                                            }
                                                                                                                            let ec = Self::ptr_index32(
                                                                                                                                env,
                                                                                                                                f,
                                                                                                                                2,
                                                                                                                                0,
                                                                                                                            );
                                                                                                                            let ed = Self::ptr_index32(
                                                                                                                                env,
                                                                                                                                f,
                                                                                                                                2,
                                                                                                                                1,
                                                                                                                            );
                                                                                                                            Self::func76(
                                                                                                                                env,
                                                                                                                                a.wrapping_add(544),
                                                                                                                                ec,
                                                                                                                                ed,
                                                                                                                            );
                                                                                                                            break 'label60;
                                                                                                                        }
                                                                                                                        let ee = Self::ptr_index32(
                                                                                                                            env,
                                                                                                                            f,
                                                                                                                            3,
                                                                                                                            0,
                                                                                                                        );
                                                                                                                        let ef = Self::ptr_index32(
                                                                                                                            env,
                                                                                                                            f,
                                                                                                                            3,
                                                                                                                            1,
                                                                                                                        );
                                                                                                                        let eg = Self::ptr_index32(
                                                                                                                            env,
                                                                                                                            f,
                                                                                                                            3,
                                                                                                                            2,
                                                                                                                        );
                                                                                                                        Self::entry_match_copy(
                                                                                                                            env,
                                                                                                                            a.wrapping_add(232),
                                                                                                                            ee,
                                                                                                                            ef,
                                                                                                                            eg,
                                                                                                                        );
                                                                                                                        let eh = mload8!(a.wrapping_add(232) as usize) as i32;
                                                                                                                        if eh == 0 {
                                                                                                                            let ei = Self::ptr_index32(
                                                                                                                                env,
                                                                                                                                f,
                                                                                                                                3,
                                                                                                                                1,
                                                                                                                            );
                                                                                                                            let ej = Self::ptr_index32(
                                                                                                                                env,
                                                                                                                                f,
                                                                                                                                3,
                                                                                                                                0,
                                                                                                                            );
                                                                                                                            let ek = Self::ptr_index32(
                                                                                                                                env,
                                                                                                                                f,
                                                                                                                                3,
                                                                                                                                2,
                                                                                                                            );
                                                                                                                            Self::entry_match_copy(
                                                                                                                                env,
                                                                                                                                a.wrapping_add(392),
                                                                                                                                ei,
                                                                                                                                ej,
                                                                                                                                ek,
                                                                                                                            );
                                                                                                                            let el = mload8!(a.wrapping_add(392) as usize) as i32;
                                                                                                                            if el == 0 {
                                                                                                                                let em = Self::ptr_index32(
                                                                                                                                    env,
                                                                                                                                    f,
                                                                                                                                    3,
                                                                                                                                    1,
                                                                                                                                );
                                                                                                                                let en = Self::ptr_index32(
                                                                                                                                    env,
                                                                                                                                    f,
                                                                                                                                    3,
                                                                                                                                    2,
                                                                                                                                );
                                                                                                                                let eo = Self::ptr_index32(
                                                                                                                                    env,
                                                                                                                                    f,
                                                                                                                                    3,
                                                                                                                                    0,
                                                                                                                                );
                                                                                                                                Self::entry_match_copy(
                                                                                                                                    env,
                                                                                                                                    a.wrapping_add(480),
                                                                                                                                    em,
                                                                                                                                    en,
                                                                                                                                    eo,
                                                                                                                                );
                                                                                                                                let ep = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                                                                if ep == 0 {
                                                                                                                                    unreachable!();
                                                                                                                                }
                                                                                                                                mstore64!(a.wrapping_add(568) as usize, mload64!(t as usize) as u64);
                                                                                                                                mstore64!(a.wrapping_add(560) as usize, mload64!(i as usize) as u64);
                                                                                                                                mstore64!(a.wrapping_add(552) as usize, mload64!(p as usize) as u64);
                                                                                                                                let mut sv9_0_i64 = mload64!(g as usize);
                                                                                                                                let mut sv3_544_i64 = sv9_0_i64 as i64;
                                                                                                                                break 'label60;
                                                                                                                            }
                                                                                                                            let eq = mload64!(l.wrapping_add(24) as usize);
                                                                                                                            mstore64!(a.wrapping_add(568) as usize, eq as u64);
                                                                                                                            let er = mload64!(l.wrapping_add(16) as usize);
                                                                                                                            mstore64!(a.wrapping_add(560) as usize, er as u64);
                                                                                                                            let es = mload64!(l.wrapping_add(8) as usize);
                                                                                                                            mstore64!(a.wrapping_add(552) as usize, es as u64);
                                                                                                                            sv3_544_i64 = mload64!(l as usize);
                                                                                                                            break 'label60;
                                                                                                                        }
                                                                                                                        let et = mload64!(j.wrapping_add(24) as usize);
                                                                                                                        mstore64!(a.wrapping_add(568) as usize, et as u64);
                                                                                                                        let eu = mload64!(j.wrapping_add(16) as usize);
                                                                                                                        mstore64!(a.wrapping_add(560) as usize, eu as u64);
                                                                                                                        let ev = mload64!(j.wrapping_add(8) as usize);
                                                                                                                        mstore64!(a.wrapping_add(552) as usize, ev as u64);
                                                                                                                        sv3_544_i64 = mload64!(j as usize);
                                                                                                                    }
                                                                                                                    let ew = mload64!(a.wrapping_add(568) as usize);
                                                                                                                    mstore64!(a.wrapping_add(504) as usize, ew as u64);
                                                                                                                    let ex = mload64!(a.wrapping_add(560) as usize);
                                                                                                                    mstore64!(a.wrapping_add(496) as usize, ex as u64);
                                                                                                                    let ey = mload64!(a.wrapping_add(552) as usize);
                                                                                                                    mstore64!(a.wrapping_add(488) as usize, ey as u64);
                                                                                                                    let mut sv3_480_i64 = sv3_544_i64 as i64;
                                                                                                                    break 'label54;
                                                                                                                }
                                                                                                                arg2 = mload32!(a.wrapping_add(384) as usize) as i64;
                                                                                                                arg1 = mload64!(a.wrapping_add(376) as usize);
                                                                                                                d = 27;
                                                                                                                break 'label35;
                                                                                                            }
                                                                                                        }
                                                                                                        b = (d as u32).wrapping_shr(1 as u32) as i32;
                                                                                                        if d & 1 == 0 {
                                                                                                            let ez = Self::ptr_index32(
                                                                                                                env,
                                                                                                                f,
                                                                                                                d,
                                                                                                                b.wrapping_sub(1),
                                                                                                            );
                                                                                                            let fa = Self::ptr_index32(
                                                                                                                env,
                                                                                                                f,
                                                                                                                d,
                                                                                                                b,
                                                                                                            );
                                                                                                            Self::func76(
                                                                                                                env,
                                                                                                                a.wrapping_add(544),
                                                                                                                ez,
                                                                                                                fa,
                                                                                                            );
                                                                                                        } else {
                                                                                                            let fb = Self::ptr_index32(
                                                                                                                env,
                                                                                                                f,
                                                                                                                d,
                                                                                                                b,
                                                                                                            );
                                                                                                            b = fb;
                                                                                                            let fc = mload64!(b.wrapping_add(24) as usize);
                                                                                                            mstore64!(a.wrapping_add(568) as usize, fc as u64);
                                                                                                            let fd = mload64!(b.wrapping_add(16) as usize);
                                                                                                            mstore64!(a.wrapping_add(560) as usize, fd as u64);
                                                                                                            let fe = mload64!(b.wrapping_add(8) as usize);
                                                                                                            mstore64!(a.wrapping_add(552) as usize, fe as u64);
                                                                                                            let mut sv4_0_i64 = mload64!(b as usize);
                                                                                                            let mut sv3_544_i64 = sv4_0_i64 as i64;
                                                                                                        }
                                                                                                        let ff = mload64!(a.wrapping_add(568) as usize);
                                                                                                        mstore64!(a.wrapping_add(504) as usize, ff as u64);
                                                                                                        let fg = mload64!(a.wrapping_add(560) as usize);
                                                                                                        mstore64!(a.wrapping_add(496) as usize, fg as u64);
                                                                                                        let fh = mload64!(a.wrapping_add(552) as usize);
                                                                                                        mstore64!(a.wrapping_add(488) as usize, fh as u64);
                                                                                                        let mut sv3_480_i64 = sv3_544_i64 as i64;
                                                                                                    }
                                                                                                    c = a.wrapping_add(472);
                                                                                                    b = a.wrapping_add(504);
                                                                                                    let sv5_0_i64 = sv4_0_i64 as i64;
                                                                                                    d = a.wrapping_add(464);
                                                                                                    e = a.wrapping_add(496);
                                                                                                    f = a.wrapping_add(456);
                                                                                                    i = a.wrapping_add(488);
                                                                                                    let sv3_448_i64 = sv3_480_i64 as i64;
                                                                                                    if k as u32 <= u as u32 {
                                                                                                        break 'label52;
                                                                                                    }
                                                                                                    b = n.wrapping_add(u.wrapping_shl(5 as u32));
                                                                                                    mload64!(b.wrapping_add(24) as usize);
                                                                                                    mload64!(b.wrapping_add(16) as usize);
                                                                                                    mload64!(b.wrapping_add(8) as usize);
                                                                                                    arg1 = sv4_0_i64;
                                                                                                    let sv19_0_i64 = sv3_448_i64 as i64;
                                                                                                    mstore64!(q.wrapping_add(8) as usize, mload64!(i as usize) as u64);
                                                                                                    mstore64!(q.wrapping_add(16) as usize, mload64!(e as usize) as u64);
                                                                                                    mstore64!(q.wrapping_add(24) as usize, sv5_0_i64 as u64);
                                                                                                    sv3_480_i64 = arg1 as i64;
                                                                                                    if mload32!(a.wrapping_add(376) as usize) == m {
                                                                                                        Self::alloc_realloc(env, a.wrapping_add(376));
                                                                                                        w = mload32!(a.wrapping_add(380) as usize);
                                                                                                    }
                                                                                                    Self::memmove(
                                                                                                        env,
                                                                                                        w.wrapping_add(m.wrapping_shl(6 as u32)),
                                                                                                        a.wrapping_add(480),
                                                                                                        64,
                                                                                                    );
                                                                                                    m += 1;
                                                                                                    let sv3_384_i32 = m as i32;
                                                                                                }
                                                                                                c = h;
                                                                                                continue 'label51;
                                                                                            }
                                                                                        }
                                                                                        unreachable!();
                                                                                    }
                                                                                    arg1 = mload64!(d as usize);
                                                                                    let sv3_400_i32 = i as i32;
                                                                                    let mut sv3_392_i64 = arg1 as i64;
                                                                                    Self::memmove(
                                                                                        env,
                                                                                        w,
                                                                                        d.wrapping_add(12),
                                                                                        44,
                                                                                    );
                                                                                    'label74: {
                                                                                        let fp = mload8!(a.wrapping_add(412) as usize) as i32;
                                                                                        b = fp;
                                                                                        if b == 1 {
                                                                                            if b != 0 {
                                                                                                b = o;
                                                                                            } else {
                                                                                                b = 0;
                                                                                            }
                                                                                            h = 0;
                                                                                            c = q;
                                                                                            d = f;
                                                                                            loop {
                                                                                                if c == 0 {
                                                                                                    unreachable!();
                                                                                                }
                                                                                                let fq = Self::memeq32(env, d, b);
                                                                                                if fq == 0 {
                                                                                                    c = c.wrapping_sub(32);
                                                                                                    h += 1;
                                                                                                    d = d.wrapping_add(32);
                                                                                                }
                                                                                            }
                                                                                            c = mload32!(a.wrapping_add(408) as usize);
                                                                                            b = mload32!(a.wrapping_add(404) as usize);
                                                                                            let sv3_456_i32 = i as i32;
                                                                                            let mut sv3_452_i32 = b as i32;
                                                                                            let sv3_448_i32 = b as i32;
                                                                                            t = b.wrapping_add(c.wrapping_shl(6 as u32));
                                                                                            while b != t {
                                                                                                y = a.wrapping_add(552);
                                                                                                c = b.wrapping_add(40);
                                                                                                z = a.wrapping_add(560);
                                                                                                d = b.wrapping_add(48);
                                                                                                u = a.wrapping_add(568);
                                                                                                e = b.wrapping_add(56);
                                                                                                let sv4_32_i64 = mload64!(b.wrapping_add(32) as usize);
                                                                                                let sv3_544_i64 = sv4_32_i64 as i64;
                                                                                                arg1 = mload64!(b.wrapping_add(15) as usize);
                                                                                                arg2 = mload64!(b.wrapping_add(23) as usize);
                                                                                                let fr = mload8!(b.wrapping_add(31) as usize) as i32;
                                                                                                i = fr;
                                                                                                let sv9_0_i64 = sv4_32_i64 as i64;
                                                                                                mstore64!(g.wrapping_add(8) as usize, mload64!(c as usize) as u64);
                                                                                                mstore64!(g.wrapping_add(16) as usize, mload64!(d as usize) as u64);
                                                                                                mstore64!(g.wrapping_add(24) as usize, mload64!(e as usize) as u64);
                                                                                                mstore8!(a.wrapping_add(511) as usize, i as u8);
                                                                                                c = b.wrapping_add(7);
                                                                                                let mut sv4_0_i64 = mload64!(b as usize);
                                                                                                let mut sv3_480_i64 = sv4_0_i64 as i64;
                                                                                                j = b.wrapping_sub(-64);
                                                                                                {
                                                                                                    let fs = Self::memeq32(env, g, 1049290);
                                                                                                    if fs == 0 {
                                                                                                        mstore8!(a.wrapping_add(263) as usize, i as u8);
                                                                                                        let mut sv4_0_i64 = mload64!(b as usize);
                                                                                                        let sv3_232_i64 = sv4_0_i64 as i64;
                                                                                                        let mut sv3_239_i64 = mload64!(c as usize);
                                                                                                        e = 0;
                                                                                                        c = m;
                                                                                                        d = n;
                                                                                                        loop {
                                                                                                            if c == 0 {
                                                                                                                unreachable!();
                                                                                                            }
                                                                                                            let ft = Self::memeq32(env, d, a.wrapping_add(232));
                                                                                                            if ft == 0 {
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
                                                                                                        c = d.wrapping_add(h);
                                                                                                        if (c as u32) < d as u32 {
                                                                                                            unreachable!();
                                                                                                        }
                                                                                                        if c as u32 >= s as u32 {
                                                                                                            unreachable!();
                                                                                                        }
                                                                                                        c = x.wrapping_add(c.wrapping_mul(33));
                                                                                                        let fu = mload8!(c as usize) as i32;
                                                                                                        if fu != 0 {
                                                                                                            unreachable!();
                                                                                                        }
                                                                                                        mstore8!(c as usize, 1 as u8);
                                                                                                        mload64!(a.wrapping_add(544) as usize);
                                                                                                        mstore64!(c.wrapping_add(9) as usize, mload64!(y as usize) as u64);
                                                                                                        mstore64!(c.wrapping_add(17) as usize, mload64!(z as usize) as u64);
                                                                                                        mstore64!(c.wrapping_add(25) as usize, mload64!(u as usize) as u64);
                                                                                                    }
                                                                                                }
                                                                                                b = j;
                                                                                            }
                                                                                            sv3_452_i32 = b as i32;
                                                                                            break 'label74;
                                                                                            let sv3_452_i32 = j as i32;
                                                                                            let mut sv4_0_i64 = mload64!(b as usize);
                                                                                            let sv3_336_i64 = sv4_0_i64 as i64;
                                                                                            let fv = mload64!(b.wrapping_add(7) as usize);
                                                                                            let mut sv3_343_i64 = fv as i64;
                                                                                            ao = sv3_336_i64;
                                                                                            let mload64!(ae as usize) = ao as i64;
                                                                                            let sv3_311_i64 = sv3_343_i64 as i64;
                                                                                            let fw = mload8!(a.wrapping_add(310) as usize) as i64;
                                                                                            let fx = mload16!(a.wrapping_add(308) as usize) as i64;
                                                                                            c = ((ao & 4294967295 | fw.wrapping_shl(48 as u32) | fx.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                                                                            an = sv3_311_i64;
                                                                                            g = ao as i32;
                                                                                            d = 11;
                                                                                            break 'label35;
                                                                                        }
                                                                                    }
                                                                                    d = l;
                                                                                    continue 'label49;
                                                                                }
                                                                                unreachable!();
                                                                            }
                                                                        }
                                                                        mstore8!(a.wrapping_add(240) as usize, e as u8);
                                                                        let sv3_236_i32 = -2147483648 as i32;
                                                                        break 'label34;
                                                                    }
                                                                    break 'label34;
                                                                }
                                                                c = 0;
                                                            }
                                                            ao = (arg1 as u64).wrapping_shr(32 as u32) as i64;
                                                            if d != 27 {
                                                                h = (arg1 as u64).wrapping_shr(40 as u32) as i64 as i32;
                                                                l = ao as i32;
                                                                k = arg1 as i32;
                                                                break 'label23;
                                                            }
                                                            k = ao as i32;
                                                            b = k.wrapping_add((arg2 as i32).wrapping_shl(6 as u32));
                                                            let fy = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                            arg1 = fy;
                                                            'label80: loop {
                                                                if b == k {
                                                                    let mut svarg0_16_i64 = arg1 as i64;
                                                                    let mut svarg0_8_i64 = an as i64;
                                                                    mstore8!(arg0 as usize, 27 as u8);
                                                                    break 'label22;
                                                                }
                                                                Self::memmove(
                                                                    env,
                                                                    a.wrapping_add(480),
                                                                    k,
                                                                    64,
                                                                );
                                                                let ga = mload64!(k.wrapping_add(56) as usize);
                                                                mstore64!(a.wrapping_add(256) as usize, ga as u64);
                                                                let gb = mload64!(k.wrapping_add(48) as usize);
                                                                mstore64!(a.wrapping_add(248) as usize, gb as u64);
                                                                let gc = mload64!(k.wrapping_add(40) as usize);
                                                                mstore64!(a.wrapping_add(240) as usize, gc as u64);
                                                                mload64!(k.wrapping_add(32) as usize);
                                                                let gd = val_to_i64(Bytes::new(env).into_val(env));
                                                                let ge = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(gd))).into_val(env));
                                                                arg2 = ge;
                                                                let gf = mload64!(a.wrapping_add(504) as usize);
                                                                mstore64!(a.wrapping_add(416) as usize, gf as u64);
                                                                let gg = mload64!(a.wrapping_add(496) as usize);
                                                                mstore64!(a.wrapping_add(408) as usize, gg as u64);
                                                                let gh = mload64!(a.wrapping_add(488) as usize);
                                                                mstore64!(a.wrapping_add(400) as usize, gh as u64);
                                                                let mut sv3_480_i64 = mload64!(a.wrapping_add(480) as usize);
                                                                let mut sv3_392_i64 = sv3_480_i64 as i64;
                                                                k = k.wrapping_sub(-64);
                                                                d = 33;
                                                                loop {
                                                                    {
                                                                        c = 0;
                                                                        e = d -= 1;
                                                                        d = e;
                                                                        let gi = mload8!(a.wrapping_add(392).wrapping_add(d).wrapping_sub(2) as usize) as i32;
                                                                    }
                                                                }
                                                                while c == e {
                                                                    c = 0;
                                                                } else {
                                                                    let gj = mload8!(a.wrapping_add(392).wrapping_add(c) as usize) as i32;
                                                                    if gj == 0 {
                                                                        c += 1;
                                                                    }
                                                                }
                                                                if c as u32 <= e as u32 {
                                                                    d = a.wrapping_add(392);
                                                                    let gk = val_to_i64(String::from_str(env, ""));
                                                                    ao = gk;
                                                                    let sv3_392_i64 = ao as i64;
                                                                    let gl = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                                                    let gm = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(arg1)); v.push_back(val_from_i64(gl)); val_to_i64(v.into_val(env)) };
                                                                    arg1 = gm;
                                                                    continue 'label80;
                                                                }
                                                            }
                                                            unreachable!();
                                                        }
                                                        mstore8!(a.wrapping_add(232) as usize, 1 as u8);
                                                        m = 0;
                                                        j = i;
                                                        cj = h;
                                                    }
                                                    q = cj;
                                                    d = 23;
                                                    if at as u64 > 4294967295 as u64 {
                                                        break 'label32;
                                                    }
                                                    if aq as u64 > 4294967295 as u64 {
                                                        break 'label26;
                                                    }
                                                    'label85: {
                                                        c = at as i32;
                                                        if (c.wrapping_sub(65536) as u32) < -65535 as u32 {
                                                            d = 4;
                                                        } else {
                                                            y += 1;
                                                            ai = aq as i32;
                                                            Self::alloc_range(
                                                                env,
                                                                a.wrapping_add(32),
                                                                c,
                                                                1,
                                                                64,
                                                            );
                                                            d = 0;
                                                            o = 0;
                                                            'label87: {
                                                                'label88: loop {
                                                                    if c == o {
                                                                        break 'label85;
                                                                    }
                                                                    b = a.wrapping_add(480);
                                                                    e = a.wrapping_add(80);
                                                                    Self::span_take(
                                                                        env,
                                                                        b,
                                                                        e,
                                                                        ai,
                                                                    );
                                                                    let gn = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                    d = gn;
                                                                    if d == 27 {
                                                                        aq = mload64!(a.wrapping_add(488) as usize);
                                                                        let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                                                        aa = sv3_484_i32;
                                                                        Self::span_take(
                                                                            env,
                                                                            b,
                                                                            e,
                                                                            32,
                                                                        );
                                                                        'label89: {
                                                                            let go: i32;
                                                                            {
                                                                                let gp = mload8!(a.wrapping_add(480) as usize) as i32;
                                                                                d = gp;
                                                                                if d == 27 {
                                                                                    arg2 = mload64!(a.wrapping_add(488) as usize);
                                                                                    h = arg2 as i32;
                                                                                    'label91: {
                                                                                        if arg2 as u64 >= 4294967296 as u64 {
                                                                                            b = (arg2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                                                            e = 0;
                                                                                            'label92: {
                                                                                                loop {
                                                                                                    if b == e {
                                                                                                        break 'label92;
                                                                                                    }
                                                                                                    i = e.wrapping_add(h);
                                                                                                    let gq = mload8!(i as usize) as i32;
                                                                                                    if gq == 0 {
                                                                                                        e += 1;
                                                                                                    }
                                                                                                }
                                                                                                l = h -= 1;
                                                                                                loop {
                                                                                                    h = b;
                                                                                                    if h == 0 {
                                                                                                        unreachable!();
                                                                                                    }
                                                                                                    b -= 1;
                                                                                                    let gr = mload8!(h.wrapping_add(l) as usize) as i32;
                                                                                                }
                                                                                                if e as u32 <= h as u32 {
                                                                                                    Self::func69(
                                                                                                        env,
                                                                                                        a.wrapping_add(352),
                                                                                                        i,
                                                                                                        h.wrapping_sub(e),
                                                                                                    );
                                                                                                    break 'label91;
                                                                                                }
                                                                                                unreachable!();
                                                                                            }
                                                                                            break 'label91;
                                                                                        }
                                                                                        let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                                                                        b = sv3_484_i32;
                                                                                    }
                                                                                    Self::bytes_to_fixed32(env, a.wrapping_add(364), a.wrapping_add(352));
                                                                                    mstore64!(a.wrapping_add(504) as usize, 0 as u64);
                                                                                    mstore64!(a.wrapping_add(496) as usize, 0 as u64);
                                                                                    mstore64!(a.wrapping_add(488) as usize, 0 as u64);
                                                                                    let sv3_480_i64 = 0 as i64;
                                                                                    b = mload32!(a.wrapping_add(372) as usize);
                                                                                    Self::span_set(
                                                                                        env,
                                                                                        a.wrapping_add(24),
                                                                                        a.wrapping_add(480),
                                                                                        b,
                                                                                    );
                                                                                    Self::memcpy_checked(
                                                                                        env,
                                                                                        mload32!(a.wrapping_add(24) as usize),
                                                                                        mload32!(a.wrapping_add(28) as usize),
                                                                                        mload32!(a.wrapping_add(368) as usize),
                                                                                        b,
                                                                                    );
                                                                                    let gs = mload16!(a.wrapping_add(500) as usize) as i32;
                                                                                    let gt = mload8!(t as usize) as i32;
                                                                                    h = gs | gt.wrapping_shl(16 as u32);
                                                                                    let gu = mload16!(a.wrapping_add(480) as usize) as i32;
                                                                                    let gv = mload8!(a.wrapping_add(482) as usize) as i32;
                                                                                    g = gu | gv.wrapping_shl(16 as u32);
                                                                                    let gw = mload8!(a.wrapping_add(511) as usize) as i32;
                                                                                    i = gw;
                                                                                    arg2 = mload64!(a.wrapping_add(503) as usize);
                                                                                    let gx = mload8!(a.wrapping_add(499) as usize) as i32;
                                                                                    l = gx;
                                                                                    e = mload32!(a.wrapping_add(495) as usize);
                                                                                    an = mload64!(a.wrapping_add(487) as usize);
                                                                                    b = mload32!(a.wrapping_add(483) as usize);
                                                                                    if d == 27 {
                                                                                        break 'label89;
                                                                                    }
                                                                                    go = b;
                                                                                } else {
                                                                                    let gy = mload64!(x.wrapping_add(7) as usize);
                                                                                    let sv3_239_i64 = gy as i64;
                                                                                    let gz = mload16!(a.wrapping_add(481) as usize) as i32;
                                                                                    let ha = mload8!(a.wrapping_add(483) as usize) as i32;
                                                                                    g = gz | ha.wrapping_shl(16 as u32);
                                                                                    let hb = mload16!(a.wrapping_add(501) as usize) as i32;
                                                                                    let hc = mload8!(a.wrapping_add(503) as usize) as i32;
                                                                                    h = hb | hc.wrapping_shl(16 as u32);
                                                                                    let hd = mload8!(a.wrapping_add(512) as usize) as i32;
                                                                                    i = hd;
                                                                                    arg2 = mload64!(a.wrapping_add(504) as usize);
                                                                                    let he = mload8!(a.wrapping_add(500) as usize) as i32;
                                                                                    l = he;
                                                                                    let mut sv3_496_i32 = mload32!(a.wrapping_add(496) as usize);
                                                                                    e = sv3_496_i32;
                                                                                    an = mload64!(a.wrapping_add(488) as usize);
                                                                                    let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                                                                    go = sv3_484_i32;
                                                                                }
                                                                            }
                                                                            c = go;
                                                                            let mut sv3_455_i64 = sv3_239_i64 as i64;
                                                                            mload64!(x as usize);
                                                                            break 'label87;
                                                                        }
                                                                        mstore16!(a.wrapping_add(500) as usize, h as u16);
                                                                        mstore8!(t as usize, (h as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore8!(a.wrapping_add(511) as usize, i as u8);
                                                                        mstore8!(a.wrapping_add(499) as usize, l as u8);
                                                                        mstore16!(a.wrapping_add(480) as usize, g as u16);
                                                                        mstore8!(a.wrapping_add(482) as usize, (g as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        let sv3_232_i32 = aa as i32;
                                                                        Self::bytes_to_fixed32_struct(env, ab, a.wrapping_add(232));
                                                                        let hf = mload64!(u.wrapping_add(7) as usize);
                                                                        let mut sv3_455_i64 = hf as i64;
                                                                        let hg = mload8!(ag as usize) as i32;
                                                                        e = hg;
                                                                        let hh = mload8!(ah as usize) as i32;
                                                                        h = hh;
                                                                        let hi = mload8!(a.wrapping_add(487) as usize) as i32;
                                                                        i = hi;
                                                                        l = mload32!(a.wrapping_add(491) as usize);
                                                                        arg2 = mload64!(a.wrapping_add(495) as usize);
                                                                        g = mload32!(a.wrapping_add(503) as usize);
                                                                        let hj = mload8!(a.wrapping_add(507) as usize) as i32;
                                                                        aa = hj;
                                                                        an = mload64!(a.wrapping_add(511) as usize);
                                                                        let hk = mload8!(a.wrapping_add(519) as usize) as i32;
                                                                        let hl = mload16!(a.wrapping_add(488) as usize) as i32;
                                                                        let hm = mload16!(a.wrapping_add(508) as usize) as i32;
                                                                        b = a.wrapping_add(400);
                                                                        let hn = mload8!(w.wrapping_add(8) as usize) as i32;
                                                                        mstore8!(b as usize, hn as u8);
                                                                        am = a.wrapping_add(288);
                                                                        let ho = mload8!(b as usize) as i32;
                                                                        mstore8!(am as usize, ho as u8);
                                                                        let mut sv3_383_i64 = sv3_455_i64 as i64;
                                                                        mload64!(u as usize);
                                                                        d = mload32!(a.wrapping_add(276) as usize);
                                                                        if d == mload32!(a.wrapping_add(268) as usize) {
                                                                            Self::alloc_realloc(env, a.wrapping_add(268));
                                                                        }
                                                                        o += 1;
                                                                        let sv3_272_i32 = mload32!(a.wrapping_add(272) as usize);
                                                                        b = sv3_272_i32.wrapping_add(d.wrapping_shl(6 as u32));
                                                                        h = hm | h.wrapping_shl(16 as u32);
                                                                        mstore16!(b.wrapping_add(28) as usize, h as u16);
                                                                        e = hl | e.wrapping_shl(16 as u32);
                                                                        mstore16!(b.wrapping_add(8) as usize, e as u16);
                                                                        mload32!(a.wrapping_add(296) as usize);
                                                                        mstore8!(b.wrapping_add(39) as usize, hk as u8);
                                                                        mstore8!(b.wrapping_add(27) as usize, aa as u8);
                                                                        let sv4_23_i32 = g as i32;
                                                                        let sv4_11_i32 = l as i32;
                                                                        mstore8!(b.wrapping_add(7) as usize, i as u8);
                                                                        mstore8!(b.wrapping_add(30) as usize, (h as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore8!(b.wrapping_add(10) as usize, (e as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore32!(b.wrapping_add(3) as usize, mload32!(a.wrapping_add(299) as usize) as u32);
                                                                        mstore64!(b.wrapping_add(47) as usize, mload64!(a.wrapping_add(551) as usize) as u64);
                                                                        let hp = mload8!(am as usize) as i32;
                                                                        mstore8!(b.wrapping_add(63) as usize, hp as u8);
                                                                        d += 1;
                                                                        continue 'label88;
                                                                    }
                                                                }
                                                                let hq = mload64!(a.wrapping_add(520) as usize);
                                                                let sv3_455_i64 = hq as i64;
                                                                let hr = mload16!(a.wrapping_add(481) as usize) as i32;
                                                                let hs = mload8!(a.wrapping_add(483) as usize) as i32;
                                                                g = hr | hs.wrapping_shl(16 as u32);
                                                                let ht = mload16!(a.wrapping_add(501) as usize) as i32;
                                                                let hu = mload8!(a.wrapping_add(503) as usize) as i32;
                                                                h = ht | hu.wrapping_shl(16 as u32);
                                                                let hv = mload8!(a.wrapping_add(512) as usize) as i32;
                                                                i = hv;
                                                                arg2 = mload64!(a.wrapping_add(504) as usize);
                                                                let hw = mload8!(a.wrapping_add(500) as usize) as i32;
                                                                l = hw;
                                                                let mut sv3_496_i32 = mload32!(a.wrapping_add(496) as usize);
                                                                e = sv3_496_i32;
                                                                an = mload64!(a.wrapping_add(488) as usize);
                                                                let mut sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                                                                c = sv3_484_i32;
                                                            }
                                                            let sv3_383_i64 = sv3_455_i64 as i64;
                                                            let sv3_343_i64 = sv3_383_i64 as i64;
                                                        }
                                                        let mut sv3_176_i32 = e as i32;
                                                        let mut sv3_159_i64 = sv3_343_i64 as i64;
                                                        break 'label26;
                                                    }
                                                    d = a.wrapping_add(188);
                                                    let hx = mload8!(a.wrapping_add(228) as usize) as i32;
                                                    mstore8!(d as usize, hx as u8);
                                                    mload64!(a.wrapping_add(513) as usize) = mload64!(a.wrapping_add(304) as usize);
                                                    sv3_159_i64 = mload64!(a.wrapping_add(311) as usize);
                                                    arg2 = sv3_272_i32;
                                                    an = mload32!(a.wrapping_add(268) as usize);
                                                    let mut sv3_135_i64 = sv3_159_i64 as i64;
                                                    h = a.wrapping_add(182);
                                                    let hy = mload8!(a.wrapping_add(151) as usize) as i32;
                                                    mstore8!(h as usize, hy as u8);
                                                    let hz = mload16!(a.wrapping_add(149) as usize) as i32;
                                                    mstore16!(a.wrapping_add(180) as usize, hz as u16);
                                                    e = mload32!(a.wrapping_add(124) as usize);
                                                    if e == mload32!(a.wrapping_add(116) as usize) {
                                                        b = -16;
                                                        c = a.wrapping_add(116);
                                                        Self::func47(
                                                            env,
                                                            b.wrapping_add(8),
                                                            c,
                                                            mload32!(c as usize),
                                                            1,
                                                            8,
                                                            56,
                                                        );
                                                        c = mload32!(b.wrapping_add(8) as usize);
                                                        if c != -2147483647 {
                                                            Self::alloc_trap(env, c, mload32!(b.wrapping_add(12) as usize));
                                                            unreachable!();
                                                        }
                                                    }
                                                    c = (au as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                    g = (au as u64) as i64 as i32;
                                                    b = mload32!(a.wrapping_add(120) as usize).wrapping_add(e.wrapping_mul(56));
                                                    mstore16!(b.wrapping_add(21) as usize, q as u16);
                                                    mstore8!(b.wrapping_add(32) as usize, j as u8);
                                                    mstore8!(b.wrapping_add(20) as usize, m as u8);
                                                    mstore32!(b.wrapping_add(16) as usize, (arg2 as u64).wrapping_shr(32 as u32) as i64 as u32);
                                                    let sv4_0_i64 = au as i64;
                                                    let ib = mload16!(a.wrapping_add(180) as usize) as i32;
                                                    mstore16!(b.wrapping_add(53) as usize, ib as u16);
                                                    mstore8!(b.wrapping_add(23) as usize, (q as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                    mstore64!(b.wrapping_add(40) as usize, mload64!(a.wrapping_add(199) as usize) as u64);
                                                    let ic = mload8!(d as usize) as i32;
                                                    mstore8!(b.wrapping_add(52) as usize, ic as u8);
                                                    let id = mload8!(h as usize) as i32;
                                                    mstore8!(b.wrapping_add(55) as usize, id as u8);
                                                    d = e += 1;
                                                    h = q;
                                                    i = j;
                                                    l = m;
                                                    continue 'label31;
                                                }
                                            }
                                            break 'label26;
                                        }
                                    }
                                    let mut svarg0_16_i64 = ap as i64;
                                    let svarg0_8_i64 = av as i64;
                                    break 'label22;
                                }
                                let ie = mload64!(a.wrapping_add(520) as usize);
                                let mut sv3_327_i64 = ie as i64;
                                arg1 = mload32!(a.wrapping_add(481) as usize) as i64;
                                let ig = mload8!(a.wrapping_add(487) as usize) as i64;
                                let ih = mload16!(a.wrapping_add(485) as usize) as i64;
                                c = ((arg1 | ig.wrapping_shl(48 as u32) | ih.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                let ii = mload16!(a.wrapping_add(501) as usize) as i32;
                                let ij = mload8!(a.wrapping_add(503) as usize) as i32;
                                h = ii | ij.wrapping_shl(16 as u32);
                                let ik = mload8!(a.wrapping_add(512) as usize) as i32;
                                i = ik;
                                let il = mload8!(a.wrapping_add(500) as usize) as i32;
                                l = il;
                                let sv3_496_i32 = mload32!(a.wrapping_add(496) as usize);
                                k = sv3_496_i32;
                                g = arg1 as i32;
                                break 'label24;
                            }
                            let sv3_176_i32 = sv3_496_i32 as i32;
                            let im = mload64!(a.wrapping_add(520) as usize);
                            let sv3_159_i64 = im as i64;
                            let in = mload16!(a.wrapping_add(481) as usize) as i32;
                            let io = mload8!(a.wrapping_add(483) as usize) as i32;
                            g = in | io.wrapping_shl(16 as u32);
                            let ip = mload16!(a.wrapping_add(501) as usize) as i32;
                            let iq = mload8!(a.wrapping_add(503) as usize) as i32;
                            h = ip | iq.wrapping_shl(16 as u32);
                            let sv3_488_i64 = mload64!(a.wrapping_add(488) as usize);
                            an = sv3_488_i64;
                            let sv3_484_i32 = mload32!(a.wrapping_add(484) as usize);
                            c = sv3_484_i32;
                            let ir = mload8!(a.wrapping_add(500) as usize) as i32;
                            l = ir;
                            let is = mload8!(a.wrapping_add(512) as usize) as i32;
                            i = is;
                        }
                        let sv3_135_i64 = sv3_159_i64 as i64;
                        mload64!(a.wrapping_add(513) as usize);
                        sv3_327_i64 = sv3_135_i64 as i64;
                        k = sv3_176_i32;
                        break 'label23;
                    }
                    let it = mload64!(a.wrapping_add(520) as usize);
                    sv3_327_i64 = it as i64;
                    let iu = mload16!(a.wrapping_add(481) as usize) as i32;
                    let iv = mload8!(a.wrapping_add(483) as usize) as i32;
                    g = iu | iv.wrapping_shl(16 as u32);
                    let iw = mload16!(a.wrapping_add(501) as usize) as i32;
                    let ix = mload8!(a.wrapping_add(503) as usize) as i32;
                    h = iw | ix.wrapping_shl(16 as u32);
                    let iy = mload8!(a.wrapping_add(512) as usize) as i32;
                    i = iy;
                    let iz = mload8!(a.wrapping_add(500) as usize) as i32;
                    l = iz;
                    k = sv3_496_i32;
                    an = sv3_488_i64;
                    c = sv3_484_i32;
                    break 'label23;
                }
                an = m as u32 as i64 | (e as u32 as i64).wrapping_shl(32 as u32);
            }
            mstore8!(arg0.wrapping_add(32) as usize, i as u8);
            mstore8!(arg0 as usize, d as u8);
            mstore64!(arg0.wrapping_add(40) as usize, sv3_327_i64 as u64);
            mstore8!(arg0.wrapping_add(7) as usize, (c as u32).wrapping_shr(24 as u32) as i32 as u32 as i64 as u8);
            mstore16!(arg0.wrapping_add(5) as usize, (c as u32).wrapping_shr(8 as u32) as i32 as u32 as i64 as u16);
            mstore32!(arg0.wrapping_add(1) as usize, (c.wrapping_shl(24 as u32) as u32 as i64 | g as u32 as i64 & 16777215) as u32);
            svarg0_16_i64 = (k as u32 as i64 | (h as u32 as i64).wrapping_shl(40 as u32) | (l as u32 as i64 & 255).wrapping_shl(32 as u32)) as i64;
        }
    }

    fn require_alloc(
        env: &Env,
        mut arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let a: i32 = -16;
        Self::func47(
            env,
            a.wrapping_add(8),
            arg0,
            arg1,
            arg2,
            1,
            arg3,
        );
        arg0 = mload32!(a.wrapping_add(8) as usize);
        if arg0 != -2147483647 {
            Self::alloc_trap(env, arg0, mload32!(a.wrapping_add(12) as usize));
            unreachable!();
        }
    }

    fn build_entry_from_bytes(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = -64;
        Self::bytes_to_fixed32(env, a.wrapping_add(20), arg1);
        arg1 = a.wrapping_add(56);
        b = a.wrapping_add(48);
        c = a.wrapping_add(40);
        d = mload32!(a.wrapping_add(28) as usize);
        Self::span_set(
            env,
            a.wrapping_add(8),
            a.wrapping_add(32),
            d,
        );
        Self::memcpy_checked(
            env,
            mload32!(a.wrapping_add(8) as usize),
            mload32!(a.wrapping_add(12) as usize),
            mload32!(a.wrapping_add(24) as usize),
            d,
        );
        mstore64!(arg0.wrapping_add(24) as usize, mload64!(arg1 as usize) as u64);
        mstore64!(arg0.wrapping_add(16) as usize, mload64!(b as usize) as u64);
        mstore64!(arg0.wrapping_add(8) as usize, mload64!(c as usize) as u64);
    }

    fn memeq32(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = Self::memcmp(
            env,
            arg0,
            arg1,
            32,
        );
        (a == 0) as i32
    }

    fn alloc_range_one(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -16;
        Self::func49(
            env,
            a.wrapping_add(4),
            arg1,
            1,
            1,
            1,
        );
        b = mload32!(a.wrapping_add(8) as usize);
        let a_i32_4 = mload32!(a.wrapping_add(4) as usize);
        if a_i32_4 == 1 {
            Self::alloc_trap(env, b, mload32!(a.wrapping_add(12) as usize));
            unreachable!();
        }
    }

    fn copy_to_linear_memory(
        env: &Env,
        arg0: i64,
        arg1: i32,
        arg2: i32,
    ) {
        Self::copy_bytes_to_linear_memory(env, arg0, 0, (arg1 as u32 as i64).wrapping_shl(32 as u32) | 0, (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0);
    }

    fn span_take(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -16;
        b = mload32!(arg1.wrapping_add(8) as usize);
        mstore8!(arg0 as usize, 26 as u8);
    }

    fn func66(
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
        a = -64;
        Self::span_take(
            env,
            a.wrapping_add(16),
            arg1,
            arg2,
        );
        'label0: {
            let h: i32;
            'label1: {
                {
                    let i = mload8!(a.wrapping_add(16) as usize) as i32;
                    arg1 = i;
                    if arg1 == 27 {
                        arg1 = mload32!(a.wrapping_add(28) as usize);
                        b = mload32!(a.wrapping_add(24) as usize);
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
                        let m = mload16!(a.wrapping_add(17) as usize) as i32;
                        mstore16!(arg0.wrapping_add(1) as usize, m as u16);
                        let n = mload8!(a.wrapping_add(19) as usize) as i32;
                        mstore8!(arg0.wrapping_add(3) as usize, n as u8);
                        let o = mload64!(a.wrapping_add(40) as usize);
                        mstore64!(arg0.wrapping_add(24) as usize, o as u64);
                        let p = mload64!(a.wrapping_add(48) as usize);
                        mstore64!(arg0.wrapping_add(32) as usize, p as u64);
                        let q = mload64!(a.wrapping_add(56) as usize);
                        mstore64!(arg0.wrapping_add(40) as usize, q as u64);
                        f = mload32!(a.wrapping_add(20) as usize);
                        mstore8!(arg0 as usize, arg1 as u8);
                        break 'label0;
                    }
                }
                while arg1 != 0 {
                    let r = mload8!(arg2.wrapping_add(c) as usize) as i32;
                    mstore8!(arg2.wrapping_add(e) as usize, r as u8);
                    arg1 -= 1;
                    arg2 += 1;
                }
                h = arg2.wrapping_add(b) += 1;
            }
            arg1 = h.wrapping_sub(b);
            if arg1 as u32 > 8 as u32 {
                Self::bytes_to_fixed32_struct(env, arg0.wrapping_add(1), a.wrapping_add(16));
                mstore8!(arg0 as usize, 1 as u8);
                break 'label0;
            }
            Self::func123(
                env,
                a.wrapping_add(8),
                8.wrapping_sub(arg1),
                a.wrapping_add(16),
                8,
            );
            Self::memcpy_checked(
                env,
                mload32!(a.wrapping_add(8) as usize),
                mload32!(a.wrapping_add(12) as usize),
                b,
                arg1,
            );
            mstore8!(arg0 as usize, 27 as u8);
            f = mload64!(a.wrapping_add(16) as usize);
        }
    }

    fn entry_copy_if_ok(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let a: i32 = -16;
        let c = mload8!(arg1 as usize) as i32;
        if c == 0 {
            let d = mload64!(arg1.wrapping_add(25) as usize);
            mstore64!(arg0.wrapping_add(24) as usize, d as u64);
            let e = mload64!(arg1.wrapping_add(17) as usize);
            mstore64!(arg0.wrapping_add(16) as usize, e as u64);
            let f = mload64!(arg1.wrapping_add(9) as usize);
            mstore64!(arg0.wrapping_add(8) as usize, f as u64);
        }
    }

    fn memcmp_sign32(
        env: &Env,
        mut arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = Self::memcmp(
            env,
            arg0,
            arg1,
            32,
        );
        arg0 = a;
        (if (arg0 < 0) as i32 != 0 { -1 } else { (arg0 != 0) as i32 })
    }

    fn func69(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let a: i32 = -16;
        Self::alloc_range_fill(
            env,
            a.wrapping_add(8),
            arg2,
            1,
        );
        let d = Self::memmove(
            env,
            mload32!(a.wrapping_add(12) as usize),
            arg1,
            arg2,
        );
    }


    fn entry_from_bytes_val(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = -32;
        b = a.wrapping_add(24);
        c = a.wrapping_add(16);
        d = a.wrapping_add(8);
        Self::copy_to_linear_memory(
            env,
            arg1,
            a,
            32,
        );
        mstore64!(arg0.wrapping_add(24) as usize, mload64!(b as usize) as u64);
        mstore64!(arg0.wrapping_add(16) as usize, mload64!(c as usize) as u64);
        mstore64!(arg0.wrapping_add(8) as usize, mload64!(d as usize) as u64);
    }

    fn alloc_range_fill(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let a: i32 = -16;
        Self::func49(
            env,
            a.wrapping_add(4),
            arg1,
            0,
            1,
            arg2,
        );
        arg1 = mload32!(a.wrapping_add(8) as usize);
        let a_i32_4 = mload32!(a.wrapping_add(4) as usize);
        if a_i32_4 == 0 {
        }
        Self::alloc_trap(env, arg1, mload32!(a.wrapping_add(12) as usize));
    }

    fn memcpy_like(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        b = mload32!(arg1.wrapping_add(16) as usize);
        c = mload32!(arg1.wrapping_add(20) as usize);
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
            let g = mload64!(a.wrapping_add(25) as usize);
            mstore64!(arg0.wrapping_add(25) as usize, g as u64);
            let h = mload64!(a.wrapping_add(17) as usize);
            mstore64!(arg0.wrapping_add(17) as usize, h as u64);
            let i = mload64!(a.wrapping_add(9) as usize);
            mstore64!(arg0.wrapping_add(9) as usize, i as u64);
            d = 1;
        }
        mstore8!(0 as usize, d as u8);
    }

    fn func74(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -32;
        b = arg1.wrapping_sub(32);
        let d = Self::memcmp_sign32(env, arg1, b);
        if d & 255 == 255 {
            let e = mload64!(arg1.wrapping_add(24) as usize);
            mstore64!(a.wrapping_add(24) as usize, e as u64);
            let f = mload64!(arg1.wrapping_add(16) as usize);
            mstore64!(a.wrapping_add(16) as usize, f as u64);
            let g = mload64!(arg1.wrapping_add(8) as usize);
            mstore64!(a.wrapping_add(8) as usize, g as u64);
            loop {
                {
                    arg1 = b;
                    mstore64!(arg1.wrapping_add(32) as usize, mload64!(arg1 as usize) as u64);
                    let h = mload64!(arg1.wrapping_add(24) as usize);
                    mstore64!(arg1.wrapping_add(56) as usize, h as u64);
                    let i = mload64!(arg1.wrapping_add(16) as usize);
                    mstore64!(arg1.wrapping_add(48) as usize, i as u64);
                    let j = mload64!(arg1.wrapping_add(8) as usize);
                    mstore64!(arg1.wrapping_add(40) as usize, j as u64);
                    if arg0 != arg1 {
                        b = arg1.wrapping_sub(32);
                        let k = Self::memcmp_sign32(env, a, b);
                    }
                }
            }
            mload64!(a as usize);
            let l = mload64!(a.wrapping_add(24) as usize);
            mstore64!(arg1.wrapping_add(24) as usize, l as u64);
            let m = mload64!(a.wrapping_add(16) as usize);
            mstore64!(arg1.wrapping_add(16) as usize, m as u64);
            let n = mload64!(a.wrapping_add(8) as usize);
            mstore64!(arg1.wrapping_add(8) as usize, n as u64);
        }
    }

    fn ptr_index32(
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
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = -128;
        c = a.wrapping_sub(-64);
        Self::memcpy_like_7(env, c, arg1);
        b = a.wrapping_add(96);
        Self::memcpy_like_7(env, b, arg2);
        Self::memcpy_like_8(
            env,
            a,
            c,
            b,
        );
        mstore64!(a.wrapping_add(87) as usize, 0 as u64);
        mstore64!(a.wrapping_add(80) as usize, 0 as u64);
        mstore64!(a.wrapping_add(72) as usize, 0 as u64);
        let e = mload8!(arg1.wrapping_add(31) as usize) as i32;
        mstore8!(a.wrapping_add(95) as usize, (e & 1) as u8);
        mstore64!(a.wrapping_add(119) as usize, 0 as u64);
        mstore64!(a.wrapping_add(112) as usize, 0 as u64);
        mstore64!(a.wrapping_add(104) as usize, 0 as u64);
        let f = mload8!(arg2.wrapping_add(31) as usize) as i32;
        mstore8!(a.wrapping_add(127) as usize, (f & 1) as u8);
        arg1 = a.wrapping_add(32);
        Self::memcpy_like_8(
            env,
            arg1,
            c,
            b,
        );
        Self::memcpy_like_7(env, b, arg1);
        Self::memcpy_like_8(
            env,
            arg0,
            a,
            b,
        );
    }

    fn entry_match_copy(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        let mut a: i32 = 0;
        {
            {
                let b = Self::func121(env, arg2, arg1);
                if b != 0 {
                    let c = Self::func122(env, arg2, arg3);
                    if c != 0 {
                        unreachable!();
                    }
                }
                let d = Self::func121(env, arg2, arg3);
                if d == 0 {
                    unreachable!();
                }
                let e = Self::func122(env, arg2, arg1);
                if e == 0 {
                    unreachable!();
                }
            }
            let f = mload64!(arg2.wrapping_add(24) as usize);
            mstore64!(arg0.wrapping_add(25) as usize, f as u64);
            let g = mload64!(arg2.wrapping_add(16) as usize);
            mstore64!(arg0.wrapping_add(17) as usize, g as u64);
            let h = mload64!(arg2.wrapping_add(8) as usize);
            mstore64!(arg0.wrapping_add(9) as usize, h as u64);
            a = 1;
        }
        mstore8!(arg0 as usize, a as u8);
    }


    fn bytes_to_fixed32(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = mload32!(arg1.wrapping_add(8) as usize);
        if a as u32 <= 32 as u32 {
            let e = mload32!(arg1.wrapping_add(8) as usize);
            mstore32!(arg0.wrapping_add(8) as usize, e as u32);
        }
        b = mload32!(arg1.wrapping_add(4) as usize);
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
        Self::func125(
            env,
            arg0,
            arg1,
            c,
        );
    }

    fn span_set(
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
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
        if arg1 != arg3 {
            unreachable!();
        }
        let a = Self::memmove(
            env,
            arg0,
            arg2,
            arg1,
        );
    }

    fn bytes_to_fixed32_struct(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        a = -64;
        Self::bytes_to_fixed32(env, a.wrapping_add(20), arg1);
        b = a.wrapping_add(56);
        c = a.wrapping_add(48);
        d = a.wrapping_add(40);
        arg1 = mload32!(a.wrapping_add(28) as usize);
        if arg1 as u32 <= 32 as u32 {
            Self::func123(
                env,
                a.wrapping_add(8),
                32.wrapping_sub(arg1),
                a.wrapping_add(32),
                32,
            );
            Self::memcpy_checked(
                env,
                mload32!(a.wrapping_add(8) as usize),
                mload32!(a.wrapping_add(12) as usize),
                mload32!(a.wrapping_add(24) as usize),
                arg1,
            );
            mstore64!(arg0.wrapping_add(24) as usize, mload64!(b as usize) as u64);
            mstore64!(arg0.wrapping_add(16) as usize, mload64!(c as usize) as u64);
            mstore64!(arg0.wrapping_add(8) as usize, mload64!(d as usize) as u64);
        }
    }








    fn write_ok_val(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let a: i64;
        if arg1 as u64 <= 72057594037927935 as u64 {
            a = arg1 | 0;
        } else {
            val_to_i64(Val::from_u64(arg1 as u64));
        }
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


    fn check_recent_timestamp(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        a = -48;
        let f: i64;
        {
            c = mload64!(arg1.wrapping_add(16) as usize);
            b = c.wrapping_add(108000000);
            if ((b as u64) < c as u64) as i32 != 0 {
                b = 18446744073709551615;
            } else {
                b = b;
            }
            let g = Self::ledger_timestamp_val(env);
            if b as u64 > g as u64 {
                let h = mload64!(arg1.wrapping_add(16) as usize);
                mstore64!(arg0.wrapping_add(24) as usize, h as u64);
                let i = mload64!(arg1.wrapping_add(8) as usize);
                mstore64!(arg0.wrapping_add(16) as usize, i as u64);
            } else {
                mstore8!(a as usize, 25 as u8);
                let j = Self::decode_error_from_val(env, a);
                let mload64!(arg1 as usize) = j as i64;
            }
        }
    }






    fn val_to_i64_checked(
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

    fn func101(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> i32 {
        if arg2 != 1114112 {
            let a = { let _ = (mload32!(arg1.wrapping_add(16) as usize), arg0, arg2); unimplemented!("call_indirect type 1") };
            if a != 0 {
                return 1;
            }
        }
        if arg3 == 0 {
            return 0;
        }
        let b = { let _ = (mload32!(arg1.wrapping_add(12) as usize), arg0, arg3, 0); unimplemented!("call_indirect type 0") };
        b
    }

    fn func102(
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
        'label1: {
            c = mload32!(arg0 as usize);
            b = mload32!(arg0.wrapping_add(8) as usize);
            if c | b != 0 {
                'label3: {
                    if b & 1 != 0 {
                        d = arg1.wrapping_add(arg2);
                        g = mload32!(arg0.wrapping_add(12) as usize);
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
                        if b != 0 {
                            arg2 = f;
                        } else {
                            arg2 = arg2;
                        }
                        if b != 0 {
                            arg1 = b;
                        } else {
                            arg1 = arg1;
                        }
                    }
                }
                if c == 0 {
                    unreachable!();
                }
                i = mload32!(arg0.wrapping_add(4) as usize);
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
                            let z = mload8!(b += 1 as usize) as i8 as i32;
                            c = c.wrapping_add((z > -65) as i32);
                            if g != 2 {
                                let aa = mload8!(b.wrapping_add(2) as usize) as i8 as i32;
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
                        if (d as u32 >= 192 as u32) as i32 != 0 {
                            g = 192;
                        } else {
                            g = d;
                        }
                        h = g & 3;
                        f = g.wrapping_shl(2 as u32);
                        b = 0;
                        if d as u32 >= 4 as u32 {
                            j = e.wrapping_add(f & 1008);
                            a = e;
                            loop {
                                let ab = b;
                                k = mload32!(a as usize);
                                b = mload32!(a.wrapping_add(4) as usize);
                                b = mload32!(a.wrapping_add(8) as usize);
                                b = mload32!(a.wrapping_add(12) as usize);
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
                    a = mload32!(b as usize);
                    a = (((a ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (a as u32).wrapping_shr(6 as u32) as i32) & 16843009;
                    if h == 1 {
                        break 'label1;
                    }
                    e = mload32!(b.wrapping_add(4) as usize);
                    a = a.wrapping_add((((e ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (e as u32).wrapping_shr(6 as u32) as i32) & 16843009);
                    if h == 2 {
                        break 'label1;
                    }
                    b = mload32!(b.wrapping_add(8) as usize);
                    a = a.wrapping_add((((b ^ -1) as u32).wrapping_shr(7 as u32) as i32 | (b as u32).wrapping_shr(6 as u32) as i32) & 16843009);
                    break 'label1;
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
            c = ((((a as u32).wrapping_shr(8 as u32) as i32 & 459007).wrapping_add(a & 16711935).wrapping_mul(65537) as u32).wrapping_shr(16 as u32) as i32).wrapping_add(c);
        }
        if (c as u32) < i as u32 {
            d = i.wrapping_sub(c);
            {
                let ah = mload8!(arg0.wrapping_add(24) as usize) as i32;
                b = ah;
                if (b != 3) as i32 != 0 {
                    a = b;
                } else {
                    a = 0;
                }
            }
            a = d;
            a += 1;
            e = mload32!(arg0.wrapping_add(16) as usize);
            b = mload32!(arg0.wrapping_add(32) as usize);
            arg0 = mload32!(arg0.wrapping_add(28) as usize);
            loop {
                a -= 1;
                if a == 0 {
                    unreachable!();
                }
                let ai = { let _ = (mload32!(b.wrapping_add(16) as usize), arg0, e); unimplemented!("call_indirect type 1") };
            }
            return 1;
        }
        let al = { let _ = (mload32!(mload32!(arg0.wrapping_add(32) as usize).wrapping_add(12) as usize), mload32!(arg0.wrapping_add(28) as usize), arg1, arg2); unimplemented!("call_indirect type 0") };
        al
    }

    fn memcpy_like_2(
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
        a = -48;
        let sv3_44_i32 = arg1 as i32;
        let sv3_40_i32 = arg0 as i32;
        mstore8!(a.wrapping_add(36) as usize, 3 as u8);
        let mut sv3_20_i32 = 0 as i32;
        let mut sv3_12_i32 = 0 as i32;
        let l: i32;
        'label0: {
            'label1: {
                {
                    h = mload32!(arg2.wrapping_add(16) as usize);
                    if h == 0 {
                        arg0 = mload32!(arg2.wrapping_add(12) as usize);
                        if arg0 == 0 {
                            unreachable!();
                        }
                        arg1 = mload32!(arg2.wrapping_add(8) as usize);
                        b = arg1.wrapping_add(arg0.wrapping_shl(3 as u32));
                        e = (arg0 -= 1 & 536870911) += 1;
                        arg0 = mload32!(arg2 as usize);
                        loop {
                            let m = mload32!(arg0.wrapping_add(4) as usize);
                            c = m;
                            if c != 0 {
                                let n = { let _ = (mload32!(sv3_44_i32.wrapping_add(12) as usize), sv3_40_i32, mload32!(arg0 as usize), c); unimplemented!("call_indirect type 0") };
                                if n != 0 {
                                    unreachable!();
                                }
                            }
                            let mut svarg1_0_i32 = mload32!(arg1 as usize);
                            let o = mload32!(arg1.wrapping_add(4) as usize);
                            let p = { let _ = (o, svarg1_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 1") };
                            if p != 0 {
                                unreachable!();
                            }
                            arg0 = arg0.wrapping_add(8);
                            arg1 = arg1.wrapping_add(8);
                        }
                    } else {
                        arg0 = mload32!(arg2.wrapping_add(20) as usize);
                        if arg0 == 0 {
                            unreachable!();
                        }
                        i = arg0.wrapping_shl(5 as u32);
                        e = (arg0 -= 1 & 134217727) += 1;
                        c = mload32!(arg2.wrapping_add(8) as usize);
                        arg0 = mload32!(arg2 as usize);
                        loop {
                            let q = mload32!(arg0.wrapping_add(4) as usize);
                            arg1 = q;
                            if arg1 != 0 {
                                let r = { let _ = (mload32!(mload32!(a.wrapping_add(44) as usize).wrapping_add(12) as usize), mload32!(a.wrapping_add(40) as usize), mload32!(arg0 as usize), arg1); unimplemented!("call_indirect type 0") };
                                if r != 0 {
                                    unreachable!();
                                }
                            }
                            arg1 = f.wrapping_add(h);
                            mload32!(arg1.wrapping_add(16) as usize);
                            let t = mload8!(arg1.wrapping_add(28) as usize) as i32;
                            mstore8!(a.wrapping_add(36) as usize, t as u8);
                            mload32!(arg1.wrapping_add(24) as usize);
                            let v = mload32!(arg1.wrapping_add(12) as usize);
                            b = v;
                            g = 0;
                            d = 0;
                            {
                                let w = mload32!(arg1.wrapping_add(8) as usize);
                            }
                            j = b.wrapping_shl(3 as u32).wrapping_add(c);
                            if mload32!(j as usize) != 0 {
                                unreachable!();
                            }
                            b = mload32!(j.wrapping_add(4) as usize);
                            d = 1;
                            let sv3_12_i32 = d as i32;
                            let x = mload32!(arg1.wrapping_add(4) as usize);
                            b = x;
                            {
                                let svarg1_0_i32 = mload32!(arg1 as usize);
                            }
                            d = b.wrapping_shl(3 as u32).wrapping_add(c);
                            if mload32!(d as usize) != 0 {
                                unreachable!();
                            }
                            b = mload32!(d.wrapping_add(4) as usize);
                            g = 1;
                            let sv3_20_i32 = g as i32;
                            let y = mload32!(arg1.wrapping_add(20) as usize);
                            arg1 = c.wrapping_add(y.wrapping_shl(3 as u32));
                            let z = mload32!(arg1.wrapping_add(4) as usize);
                            let aa = { let _ = (z, svarg1_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 1") };
                            if aa != 0 {
                                unreachable!();
                            }
                            arg0 = arg0.wrapping_add(8);
                            f = f.wrapping_add(32);
                        }
                    }
                    if e as u32 >= mload32!(arg2.wrapping_add(4) as usize) as u32 {
                        break 'label1;
                    }
                    arg0 = mload32!(arg2 as usize).wrapping_add(e.wrapping_shl(3 as u32));
                    let ab = { let _ = (mload32!(mload32!(a.wrapping_add(44) as usize).wrapping_add(12) as usize), mload32!(a.wrapping_add(40) as usize), mload32!(arg0 as usize), mload32!(arg0.wrapping_add(4) as usize)); unimplemented!("call_indirect type 0") };
                    if ab == 0 {
                        unreachable!();
                    }
                }
                l = 1;
                break 'label0;
            }
            l = 0;
        }
        l
    }

    fn memcpy_like_3(
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
        let f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        k = arg1 -= 1;
        h = mload32!(arg0.wrapping_add(4) as usize);
        i = mload32!(arg0 as usize);
        j = mload32!(arg0.wrapping_add(8) as usize);
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
                                                g = mload32!(c as usize);
                                                let o = g;
                                                let p = g;
                                                let q = mload32!(c.wrapping_add(4) as usize);
                                                g = q;
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
                                            let r = mload8!(arg0.wrapping_add(d) as usize) as i32;
                                            if r == 10 {
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
                                            let s = mload8!(arg0.wrapping_add(c) as usize) as i32;
                                            if s == 10 {
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
                                    let t = mload8!(arg0.wrapping_add(d) as usize) as i32;
                                    if t == 10 {
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
                    let u = mload8!(j as usize) as i32;
                    if u != 0 {
                        let v = { let _ = (mload32!(h.wrapping_add(12) as usize), i, 1049024, 4); unimplemented!("call_indirect type 0") };
                        if v != 0 {
                            unreachable!();
                        }
                    }
                    c = 0;
                    if arg0 != f {
                        let w = mload8!(arg0.wrapping_add(k) as usize) as i32;
                        c = (w == 10) as i32;
                    }
                    arg0 = arg0.wrapping_sub(f);
                    e = arg1.wrapping_add(f);
                    mstore8!(j as usize, c as u8);
                    let x = { let _ = (mload32!(h.wrapping_add(12) as usize), i, e, arg0); unimplemented!("call_indirect type 0") };
                    if x == 0 {
                        continue 'label1;
                    }
                }
            }
            l = 1;
        }
        l
    }




    fn func108(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        if arg2 != 0 {
            let a = mload8!(1049920 as usize) as i32;
            Self::func111(env, arg2, arg1);
        }
    }

    fn memcpy_like_5(
        env: &Env,
    ) {
        let mut a: i32 = 0;
        'label0: {
            let b = mload32!(1049928 as usize);
            if b == 0 {
                let c = msize!();
                a = c;
                if a as u32 > 65535 as u32 {
                    break 'label0;
                }
                a = a.wrapping_shl(16 as u32);
                mstore32!(1049928 as usize, a as u32);
                mstore32!(1049924 as usize, a as u32);
            }
        }
    }

    fn func110(
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
                let g = mload32!(1049928 as usize);
                mstore32!(1049928 as usize, g.wrapping_add(e) as u32);
                Self::memcpy_like_5(env);
                let h = mload32!(1049924 as usize);
                b = h;
                c = b.wrapping_add(arg1);
                if (c as u32) < b as u32 {
                    break 'label0;
                }
                b = c & d;
                c = b.wrapping_add(arg0);
                let i = mload32!(1049928 as usize);
            }
            mstore32!(1049924 as usize, c as u32);
            return b;
        }
    }

    fn func111(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        Self::memcpy_like_5(env);
        let c = mload32!(1049924 as usize);
        a = c;
        b = arg1.wrapping_add(a) -= 1;
        if b as u32 >= a as u32 {
            a = b & 0.wrapping_sub(arg1);
            b = a.wrapping_add(arg0);
            let d = mload32!(1049928 as usize);
            if b as u32 > d as u32 {
                let e = Self::func110(env, arg0, arg1);
                return e;
            }
            mstore32!(1049924 as usize, b as u32);
            return a;
        }
    }

    fn func112(
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
            let c = Self::func112(
                env,
                arg0,
                arg0.wrapping_add(a),
                arg0.wrapping_add(b),
                arg3,
            );
            arg0 = c;
            let d = Self::func112(
                env,
                arg1,
                arg1.wrapping_add(a),
                arg1.wrapping_add(b),
                arg3,
            );
            arg1 = d;
            let e = Self::func112(
                env,
                arg2,
                arg2.wrapping_add(a),
                arg2.wrapping_add(b),
                arg3,
            );
            arg2 = e;
        }
        let f = Self::memcmp_sign32(env, arg0, arg1);
        arg3 = (f & 255 == 255) as i32;
        let g = Self::memcmp_sign32(env, arg0, arg2);
        let h: i32;
        if arg3 ^ g & 255 == 255 {
            h = arg0;
        } else {
            let i = Self::memcmp_sign32(env, arg1, arg2);
            if arg3 ^ (i & 255 == 255) as i32 != 0 {
                h = arg2;
            } else {
                h = arg1;
            }
        }
        h
    }

    fn func113(
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
        let k = Self::memcmp_sign32(env, arg0.wrapping_add(32), arg0);
        d = k;
        let l = Self::memcmp_sign32(env, arg0.wrapping_add(96), arg0.wrapping_sub(-64));
        e = (l & 255 == 255) as i32;
        c = arg0.wrapping_add((if e != 0 { 96 } else { 64 }));
        e = arg0.wrapping_add((if e != 0 { 64 } else { 96 }));
        f = d & 255;
        d = arg0.wrapping_add(((f != 255) as i32).wrapping_shl(5 as u32));
        arg0 = arg0.wrapping_add(((f == 255) as i32).wrapping_shl(5 as u32));
        let m = Self::memcmp_sign32(env, c, arg0);
        f = (m & 255 == 255) as i32;
        let n = Self::memcmp_sign32(env, e, d);
        g = (n & 255 == 255) as i32;
        h = { let a = e; let b = { let a = d; let b = c; if f != 0 { a } else { b } }; if g != 0 { a } else { b } };
        i = { let a = arg0; let b = { let a = c; let b = d; if g != 0 { a } else { b } }; if f != 0 { a } else { b } };
        let o = Self::memcmp_sign32(env, h, i);
        if f != 0 {
            arg0 = c;
        } else {
            arg0 = arg0;
        }
        let p = mload64!(arg0.wrapping_add(24) as usize);
        mstore64!(arg1.wrapping_add(24) as usize, p as u64);
        let q = mload64!(arg0.wrapping_add(16) as usize);
        mstore64!(arg1.wrapping_add(16) as usize, q as u64);
        let r = mload64!(arg0.wrapping_add(8) as usize);
        mstore64!(arg1.wrapping_add(8) as usize, r as u64);
        mload64!(arg0 as usize);
        c = (o & 255 == 255) as i32;
        if c != 0 {
            arg0 = h;
        } else {
            arg0 = i;
        }
        let s = mload64!(arg0.wrapping_add(24) as usize);
        mstore64!(arg1.wrapping_add(56) as usize, s as u64);
        let t = mload64!(arg0.wrapping_add(16) as usize);
        mstore64!(arg1.wrapping_add(48) as usize, t as u64);
        let u = mload64!(arg0.wrapping_add(8) as usize);
        mstore64!(arg1.wrapping_add(40) as usize, u as u64);
        if c != 0 {
            arg0 = i;
        } else {
            arg0 = h;
        }
        let v = mload64!(arg0.wrapping_add(24) as usize);
        mstore64!(arg1.wrapping_add(88) as usize, v as u64);
        let w = mload64!(arg0.wrapping_add(16) as usize);
        mstore64!(arg1.wrapping_add(80) as usize, w as u64);
        let x = mload64!(arg0.wrapping_add(8) as usize);
        mstore64!(arg1.wrapping_add(72) as usize, x as u64);
        if g != 0 {
            arg0 = d;
        } else {
            arg0 = e;
        }
        let y = mload64!(arg0.wrapping_add(8) as usize);
        mstore64!(arg1.wrapping_add(104) as usize, y as u64);
        let z = mload64!(arg0.wrapping_add(16) as usize);
        mstore64!(arg1.wrapping_add(112) as usize, z as u64);
        let aa = mload64!(arg0.wrapping_add(24) as usize);
        mstore64!(arg1.wrapping_add(120) as usize, aa as u64);
    }

    fn func114(
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
        b = -192;
        'label0: loop {
            'label1: {
                'label2: {
                    'label3: {
                        'label4: {
                            if arg1 as u32 >= 33 as u32 {
                                if arg3 == 0 {
                                    arg2 = arg0;
                                    a = -48;
                                    arg3 = arg1;
                                    arg4 = arg3.wrapping_add((arg1 as u32).wrapping_shr(1 as u32) as i32);
                                    'label5: loop {
                                        if arg4 != 0 {
                                            let s: i32;
                                            arg4 -= 1;
                                            if arg3 as u32 > arg4 as u32 {
                                                arg0 = arg2.wrapping_add(arg4.wrapping_shl(5 as u32));
                                                arg1 = arg0.wrapping_add(8);
                                                n = mload64!(arg1 as usize);
                                                c = arg0.wrapping_add(16);
                                                let mut sv7_0_i64 = mload64!(c as usize);
                                                l = sv7_0_i64;
                                                e = arg0.wrapping_add(24);
                                                let mut sv9_0_i64 = mload64!(e as usize);
                                                m = sv9_0_i64;
                                                f = arg2.wrapping_add(24);
                                                let mut sv10_0_i64 = m as i64;
                                                f = arg2.wrapping_add(16);
                                                m = sv10_0_i64;
                                                sv10_0_i64 = l as i64;
                                                f = arg2.wrapping_add(8);
                                                l = sv10_0_i64;
                                                sv10_0_i64 = n as i64;
                                                sv9_0_i64 = sv10_0_i64 as i64;
                                                sv7_0_i64 = m as i64;
                                                s = 0;
                                            } else {
                                                s = arg4.wrapping_sub(arg3);
                                            }
                                            arg1 = s;
                                            Self::func115(
                                                env,
                                                a.wrapping_add(8),
                                                (if (arg3 as u32 > arg4 as u32) as i32 != 0 { arg4 } else { arg3 }),
                                                arg2,
                                                arg3,
                                            );
                                            e = mload32!(a.wrapping_add(12) as usize);
                                            c = mload32!(a.wrapping_add(8) as usize);
                                            loop {
                                                f = arg1.wrapping_shl(1 as u32);
                                                arg0 = f | 1;
                                                if arg0 as u32 >= e as u32 {
                                                    continue 'label5;
                                                }
                                                f = f.wrapping_add(2);
                                                if e as u32 > f as u32 {
                                                    let t = Self::memcmp_sign32(env, c.wrapping_add(arg0.wrapping_shl(5 as u32)), c.wrapping_add(f.wrapping_shl(5 as u32)));
                                                    arg0 = arg0.wrapping_add((t & 255 == 255) as i32);
                                                }
                                                arg1 = c.wrapping_add(arg1.wrapping_shl(5 as u32));
                                                f = c.wrapping_add(arg0.wrapping_shl(5 as u32));
                                                let u = Self::memcmp_sign32(env, arg1, f);
                                                if u & 255 != 255 {
                                                    continue 'label5;
                                                }
                                                Self::memcpy_like_6(env, arg1, f);
                                                arg1 = arg0;
                                            }
                                            unreachable!();
                                        }
                                    }
                                    break 'label4;
                                }
                                e = (arg1 as u32).wrapping_shr(3 as u32) as i32;
                                a = arg0.wrapping_add(e.wrapping_mul(224));
                                c = arg0.wrapping_add(e.wrapping_shl(7 as u32));
                                arg3 -= 1;
                                let v: i32;
                                if arg1 as u32 >= 64 as u32 {
                                    let w = Self::func112(
                                        env,
                                        arg0,
                                        c,
                                        a,
                                        e,
                                    );
                                    v = w;
                                } else {
                                    let x = Self::memcmp_sign32(env, arg0, c);
                                    e = (x & 255 == 255) as i32;
                                    let y = Self::memcmp_sign32(env, arg0, a);
                                    let z = arg0;
                                    if e ^ y & 255 == 255 {
                                        v = z;
                                    } else {
                                        let aa = Self::memcmp_sign32(env, c, a);
                                        if e ^ (aa & 255 == 255) as i32 != 0 {
                                            v = a;
                                        } else {
                                            v = c;
                                        }
                                    }
                                }
                                c = (v.wrapping_sub(arg0) as u32).wrapping_shr(5 as u32) as i32;
                                if arg2 != 0 {
                                    a = arg0.wrapping_add(c.wrapping_shl(5 as u32));
                                    let ab = Self::memcmp_sign32(env, arg2, a);
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
                            arg2 = -1552;
                            'label9: {
                                arg3 = arg1;
                                if (arg3 as u32) >= 2 as u32 {
                                    d = 1;
                                    f = (arg1 as u32).wrapping_shr(1 as u32) as i32;
                                    arg4 = f.wrapping_shl(5 as u32);
                                    arg1 = arg0.wrapping_add(arg4);
                                    arg4 = arg2.wrapping_add(arg4);
                                    if arg3 as u32 >= 8 as u32 {
                                        Self::func113(env, arg0, arg2);
                                        Self::func113(env, arg1, arg4);
                                        d = 4;
                                    } else {
                                        let ad = mload64!(arg0.wrapping_add(24) as usize);
                                        mstore64!(arg2.wrapping_add(24) as usize, ad as u64);
                                        let ae = mload64!(arg0.wrapping_add(16) as usize);
                                        mstore64!(arg2.wrapping_add(16) as usize, ae as u64);
                                        let af = mload64!(arg0.wrapping_add(8) as usize);
                                        mstore64!(arg2.wrapping_add(8) as usize, af as u64);
                                        mload64!(arg0 as usize);
                                        mload64!(arg1 as usize);
                                        let ag = mload64!(arg1.wrapping_add(8) as usize);
                                        mstore64!(arg4.wrapping_add(8) as usize, ag as u64);
                                        let ah = mload64!(arg1.wrapping_add(16) as usize);
                                        mstore64!(arg4.wrapping_add(16) as usize, ah as u64);
                                        let ai = mload64!(arg1.wrapping_add(24) as usize);
                                        mstore64!(arg4.wrapping_add(24) as usize, ai as u64);
                                    }
                                    h = 0.wrapping_sub(d);
                                    i = arg3.wrapping_sub(f);
                                    arg1 = d.wrapping_shl(5 as u32);
                                    j = arg0.wrapping_add(arg1);
                                    k = arg1.wrapping_add(arg2);
                                    while c != 2 {
                                        let aj = mload32!(arg2.wrapping_add(1536).wrapping_add(c.wrapping_shl(2 as u32)) as usize);
                                        arg1 = aj;
                                        if arg1 != 0 {
                                            a = i;
                                        } else {
                                            a = f;
                                        }
                                        e = h.wrapping_add((if (a as u32 > d as u32) as i32 != 0 { a } else { d }));
                                        g = arg1.wrapping_shl(5 as u32);
                                        arg1 = j.wrapping_add(g);
                                        a = g.wrapping_add(k);
                                        g = arg2.wrapping_add(g);
                                        while e != 0 {
                                            let ak = mload64!(arg1.wrapping_add(24) as usize);
                                            mstore64!(a.wrapping_add(24) as usize, ak as u64);
                                            let al = mload64!(arg1.wrapping_add(16) as usize);
                                            mstore64!(a.wrapping_add(16) as usize, al as u64);
                                            let am = mload64!(arg1.wrapping_add(8) as usize);
                                            mstore64!(a.wrapping_add(8) as usize, am as u64);
                                            Self::func74(env, g, a);
                                            e -= 1;
                                            arg1 = arg1.wrapping_add(32);
                                            a = a.wrapping_add(32);
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
                                            let an = Self::memcmp_sign32(env, arg4, arg1);
                                            g = an & 255;
                                            h = (g == 255) as i32;
                                            if h != 0 {
                                                d = arg4;
                                            } else {
                                                d = arg1;
                                            }
                                            mload64!(d as usize);
                                            let ao = mload64!(d.wrapping_add(24) as usize);
                                            mstore64!(arg0.wrapping_add(24) as usize, ao as u64);
                                            let ap = mload64!(d.wrapping_add(16) as usize);
                                            mstore64!(arg0.wrapping_add(16) as usize, ap as u64);
                                            let aq = mload64!(d.wrapping_add(8) as usize);
                                            mstore64!(arg0.wrapping_add(8) as usize, aq as u64);
                                            let ar = Self::memcmp_sign32(env, c, e);
                                            i = ar & 255;
                                            j = (i == 255) as i32;
                                            if j != 0 {
                                                d = e;
                                            } else {
                                                d = c;
                                            }
                                            mload64!(d as usize);
                                            let at = mload64!(d.wrapping_add(24) as usize);
                                            mstore64!(a.wrapping_add(24) as usize, at as u64);
                                            let au = mload64!(d.wrapping_add(16) as usize);
                                            mstore64!(a.wrapping_add(16) as usize, au as u64);
                                            let av = mload64!(d.wrapping_add(8) as usize);
                                            mstore64!(a.wrapping_add(8) as usize, av as u64);
                                            f -= 1;
                                            a = a.wrapping_sub(32);
                                            arg0 = arg0.wrapping_add(32);
                                            arg1 = arg1.wrapping_add(((g != 255) as i32).wrapping_shl(5 as u32));
                                            arg4 = arg4.wrapping_add(h.wrapping_shl(5 as u32));
                                            e = e.wrapping_add((if j != 0 { -32 } else { 0 }));
                                            c = c.wrapping_add((if (i != 255) as i32 != 0 { -32 } else { 0 }));
                                        } else {
                                            a = e.wrapping_add(32);
                                            let aw: i32;
                                            if arg3 & 1 != 0 {
                                                e = ((arg1 as u32) < a as u32) as i32;
                                                if e != 0 {
                                                    arg3 = arg1;
                                                } else {
                                                    arg3 = arg4;
                                                }
                                                let ax = mload64!(arg3.wrapping_add(24) as usize);
                                                mstore64!(arg0.wrapping_add(24) as usize, ax as u64);
                                                let ay = mload64!(arg3.wrapping_add(16) as usize);
                                                mstore64!(arg0.wrapping_add(16) as usize, ay as u64);
                                                let az = mload64!(arg3.wrapping_add(8) as usize);
                                                mstore64!(arg0.wrapping_add(8) as usize, az as u64);
                                                arg4 = arg4.wrapping_add(((arg1 as u32 >= a as u32) as i32).wrapping_shl(5 as u32));
                                                aw = arg1.wrapping_add(e.wrapping_shl(5 as u32));
                                            } else {
                                                aw = arg1;
                                            }
                                            if (aw == a) as i32 & arg4 == c.wrapping_add(32) {
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
                        }
                        return arg0;
                    }
                    a = arg0.wrapping_add(c.wrapping_shl(5 as u32));
                    d = a.wrapping_add(8);
                    n = mload64!(d as usize);
                    g = a.wrapping_add(16);
                    l = mload64!(g as usize);
                    h = a.wrapping_add(24);
                    m = mload64!(h as usize);
                    let mload64!(arg3 as usize) = mload64!(a as usize);
                    c = arg0.wrapping_add(24);
                    let mut sv7_0_i64 = m as i64;
                    e = arg0.wrapping_add(16);
                    let mut sv9_0_i64 = mload64!(e as usize);
                    m = sv9_0_i64;
                    sv9_0_i64 = l as i64;
                    f = arg0.wrapping_add(8);
                    l = mload64!(f as usize);
                    d = arg0.wrapping_add(32);
                    let mut sv6_140_i32 = d as i32;
                    let ba = mload64!(arg0.wrapping_add(56) as usize);
                    mstore64!(b.wrapping_add(168) as usize, ba as u64);
                    let bb = mload64!(arg0.wrapping_add(48) as usize);
                    mstore64!(b.wrapping_add(160) as usize, bb as u64);
                    let bc = mload64!(arg0.wrapping_add(40) as usize);
                    mstore64!(b.wrapping_add(152) as usize, bc as u64);
                    let mut sv6_188_i32 = 0 as i32;
                    a = arg0.wrapping_sub(-64);
                    let mut sv6_184_i32 = a as i32;
                    let mut sv6_176_i32 = d as i32;
                    d = arg1.wrapping_shl(5 as u32);
                    g = arg0.wrapping_add(d);
                    while !(a as u32 >= g as u32) {
                        Self::func117(
                            env,
                            arg0,
                            b.wrapping_add(140),
                            b.wrapping_add(176),
                        );
                        let mut sv6_184_i32 = mload32!(b.wrapping_add(184) as usize);
                        a = sv6_184_i32;
                    }
                    'label15: {
                        d = sv6_140_i32.wrapping_add(d).wrapping_sub(32);
                        loop {
                            if a == d {
                                break 'label15;
                            }
                            Self::func117(
                                env,
                                arg0,
                                b.wrapping_add(140),
                                b.wrapping_add(176),
                            );
                            let mut sv6_184_i32 = mload32!(b.wrapping_add(184) as usize);
                            a = sv6_184_i32;
                        }
                        unreachable!();
                    }
                    sv6_184_i32 = mload32!(b.wrapping_add(180) as usize);
                    Self::func117(
                        env,
                        arg0,
                        b.wrapping_add(140),
                        b.wrapping_add(176),
                    );
                    let mut sv6_188_i32 = mload32!(b.wrapping_add(188) as usize);
                    d = sv6_188_i32;
                    if d as u32 >= arg1 as u32 {
                        break 'label1;
                    }
                    a = arg0.wrapping_add(d.wrapping_shl(5 as u32));
                    g = a.wrapping_add(8);
                    mload64!(g as usize);
                    h = a.wrapping_add(16);
                    l = mload64!(h as usize);
                    i = a.wrapping_add(24);
                    m = mload64!(i as usize);
                    mload64!(a as usize);
                    let sv7_0_i64 = m as i64;
                    let sv9_0_i64 = l as i64;
                    mload64!(f as usize);
                    Self::func114(
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
                mload64!(arg2 as usize);
                d = a.wrapping_add(16);
                mload64!(d as usize);
                g = a.wrapping_add(24);
                mload64!(g as usize);
                mload64!(a as usize);
                c = arg0.wrapping_add(24);
                mload64!(c as usize);
                e = arg0.wrapping_add(16);
                mload64!(e as usize);
                f = arg0.wrapping_add(8);
                mload64!(f as usize);
                arg2 = arg0.wrapping_add(32);
                let sv6_140_i32 = arg2 as i32;
                let bd = mload64!(arg0.wrapping_add(56) as usize);
                mstore64!(b.wrapping_add(168) as usize, bd as u64);
                let be = mload64!(arg0.wrapping_add(48) as usize);
                mstore64!(b.wrapping_add(160) as usize, be as u64);
                let bf = mload64!(arg0.wrapping_add(40) as usize);
                mstore64!(b.wrapping_add(152) as usize, bf as u64);
                mload64!(arg0.wrapping_add(32) as usize);
                let sv6_188_i32 = 0 as i32;
                a = arg0.wrapping_sub(-64);
                let mut sv6_184_i32 = a as i32;
                let sv6_176_i32 = arg2 as i32;
                arg2 = arg1.wrapping_shl(5 as u32);
                d = arg0.wrapping_add(arg2);
                b.wrapping_add(144) as i32;
                while !(a as u32 >= d as u32) {
                    Self::func118(
                        env,
                        arg0,
                        b.wrapping_add(140),
                        b.wrapping_add(176),
                    );
                    let mut sv6_184_i32 = mload32!(b.wrapping_add(184) as usize);
                    a = sv6_184_i32;
                }
                'label18: {
                    arg2 = sv6_140_i32.wrapping_add(arg2).wrapping_sub(32);
                    loop {
                        if arg2 == a {
                            break 'label18;
                        }
                        Self::func118(
                            env,
                            arg0,
                            b.wrapping_add(140),
                            b.wrapping_add(176),
                        );
                        let mut sv6_184_i32 = mload32!(b.wrapping_add(184) as usize);
                        a = sv6_184_i32;
                    }
                    unreachable!();
                }
                sv6_184_i32 = mload32!(b.wrapping_add(180) as usize);
                Self::func118(
                    env,
                    arg0,
                    b.wrapping_add(140),
                    b.wrapping_add(176),
                );
                a = mload32!(b.wrapping_add(188) as usize);
                if a as u32 >= arg1 as u32 {
                    break 'label1;
                }
                arg2 = arg0.wrapping_add(a.wrapping_shl(5 as u32));
                d = arg2.wrapping_add(8);
                mload64!(d as usize);
                g = arg2.wrapping_add(16);
                mload64!(g as usize);
                h = arg2.wrapping_add(24);
                mload64!(h as usize);
                mload64!(arg0 as usize);
                mload64!(arg2 as usize);
                mload64!(c as usize);
                mload64!(e as usize);
                mload64!(f as usize);
                arg2 = a += 1;
                arg1 = arg1.wrapping_sub(arg2);
                arg0 = arg0.wrapping_add(arg2.wrapping_shl(5 as u32));
                continue 'label0;
            }
        }
    }

    fn func115(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) {
    }

    fn memcpy_like_6(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        while a != 32 {
            b = arg0.wrapping_add(a);
            let d = mload8!(b as usize) as i32;
            let e = b;
            b = arg1.wrapping_add(a);
            let f = mload8!(b as usize) as i32;
            mstore8!(e as usize, f as u8);
            mstore8!(b as usize, d as u8);
            a += 1;
        }
    }

    fn func117(
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        a = mload32!(arg2.wrapping_add(8) as usize);
        let j = Self::memcmp_sign32(env, a, arg0);
        let mut svarg2_12_i32 = mload32!(arg2.wrapping_add(12) as usize);
        c = svarg2_12_i32;
        arg0 = mload32!(arg1 as usize).wrapping_add(c.wrapping_shl(5 as u32));
        d = arg0.wrapping_add(8);
        e = arg0.wrapping_add(16);
        f = arg0.wrapping_add(24);
        arg1 = mload32!(arg2 as usize);
        mstore64!(arg1.wrapping_add(24) as usize, mload64!(f as usize) as u64);
        mstore64!(arg1.wrapping_add(16) as usize, mload64!(e as usize) as u64);
        mstore64!(arg1.wrapping_add(8) as usize, mload64!(d as usize) as u64);
        mload64!(a.wrapping_add(24) as usize);
        mload64!(a.wrapping_add(16) as usize);
        mload64!(a.wrapping_add(8) as usize);
        mload64!(arg0 as usize) = mload64!(a as usize);
        svarg2_12_i32 = c.wrapping_add((j & 255 == 255) as i32) as i32;
        a.wrapping_add(32) as i32;
    }

    fn func118(
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let j = arg0;
        arg0 = mload32!(arg2.wrapping_add(8) as usize);
        let k = Self::memcmp_sign32(env, j, arg0);
        let mut svarg2_12_i32 = mload32!(arg2.wrapping_add(12) as usize);
        c = svarg2_12_i32;
        arg1 = mload32!(arg1 as usize).wrapping_add(c.wrapping_shl(5 as u32));
        d = arg1.wrapping_add(8);
        e = arg1.wrapping_add(16);
        f = arg1.wrapping_add(24);
        a = mload32!(arg2 as usize);
        mstore64!(a.wrapping_add(24) as usize, mload64!(f as usize) as u64);
        mstore64!(a.wrapping_add(16) as usize, mload64!(e as usize) as u64);
        mstore64!(a.wrapping_add(8) as usize, mload64!(d as usize) as u64);
        mload64!(arg0.wrapping_add(24) as usize);
        mload64!(arg0.wrapping_add(16) as usize);
        mload64!(arg0.wrapping_add(8) as usize);
        svarg2_12_i32 = c.wrapping_add((k & 255 != 255) as i32) as i32;
        arg0.wrapping_add(32) as i32;
    }

    fn memcpy_like_7(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        a = -32;
        mstore64!(a.wrapping_add(24) as usize, 0 as u64);
        mstore64!(a.wrapping_add(16) as usize, 0 as u64);
        mstore64!(a.wrapping_add(8) as usize, 0 as u64);
        while b != 32 {
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
        let i = mload64!(a.wrapping_add(24) as usize);
        mstore64!(arg0.wrapping_add(24) as usize, i as u64);
        let j = mload64!(a.wrapping_add(16) as usize);
        mstore64!(arg0.wrapping_add(16) as usize, j as u64);
        let k = mload64!(a.wrapping_add(8) as usize);
        mstore64!(arg0.wrapping_add(8) as usize, k as u64);
    }

    fn memcpy_like_8(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = -32;
        let e = mload64!(arg1.wrapping_add(24) as usize);
        mstore64!(a.wrapping_add(24) as usize, e as u64);
        let f = mload64!(arg1.wrapping_add(16) as usize);
        mstore64!(a.wrapping_add(16) as usize, f as u64);
        let g = mload64!(arg1.wrapping_add(8) as usize);
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
        let k = mload64!(a.wrapping_add(24) as usize);
        mstore64!(arg0.wrapping_add(24) as usize, k as u64);
        let l = mload64!(a.wrapping_add(16) as usize);
        mstore64!(arg0.wrapping_add(16) as usize, l as u64);
        let m = mload64!(a.wrapping_add(8) as usize);
        mstore64!(arg0.wrapping_add(8) as usize, m as u64);
    }

    fn func121(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = Self::memcmp_sign32(env, arg0, arg1);
        (((a & 255) as u32) < 2 as u32) as i32
    }

    fn func122(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) -> i32 {
        let a = Self::memcmp_sign32(env, arg0, arg1);
        (((a.wrapping_sub(3) & 255) as u32) < 254 as u32) as i32
    }

    fn func123(
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
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -16;
        let mut svarg1_8_i32 = mload32!(arg1.wrapping_add(8) as usize);
        b = svarg1_8_i32;
        if arg2 as u32 <= b as u32 {
            b = b.wrapping_sub(arg2);
            Self::alloc_range_fill(
                env,
                a.wrapping_add(8),
                b,
                1,
            );
            let svarg1_8_i32 = arg2 as i32;
            let f = Self::memmove(
                env,
                mload32!(a.wrapping_add(12) as usize),
                mload32!(arg1.wrapping_add(4) as usize).wrapping_add(arg2),
                b,
            );
        }
    }



    fn func128(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> i32 {
        let a = Self::memcpy_like_2(
            env,
            arg0,
            arg1,
            arg2,
        );
        a
    }

    fn memcmp(
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
                        let svarg1_0_i32 = mload32!(arg1 as usize);
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
                arg2 = mload32!(g as usize);
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