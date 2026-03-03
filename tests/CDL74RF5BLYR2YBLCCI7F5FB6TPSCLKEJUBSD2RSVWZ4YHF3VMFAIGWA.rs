#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, Val, IntoVal, Bytes, BytesN, FromVal, Map, Symbol};

#[contract]
pub struct SorobanContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}

fn val_to_i64(v: Val) -> i64 {
    (unsafe { core::mem::transmute::<Val, u64>(v) }) as i64
}

fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Errors { HomesteadExists = 1, HomesteadMissing = 2, FarmBlockMissing = 3, FarmPaused = 4, FarmNotPaused = 5, PlantAmountTooLow = 6, ZeroCountTooLow = 7, PailExists = 8, PailMissing = 9, WorkMissing = 10, BlockMissing = 11, BlockInvalid = 12, HashInvalid = 13, HarvestNotReady = 14, GapCountTooLow = 15, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Block { pub entropy: BytesN<32>, pub max_gap: u32, pub max_stake: i128, pub max_zeros: u32, pub min_gap: u32, pub min_stake: i128, pub min_zeros: u32, pub normalized_total: i128, pub staked_total: i128, pub timestamp: u64, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pail { pub gap: Option<u32>, pub sequence: u32, pub stake: i128, pub zeros: Option<u32>, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Storage { Homesteader, HomesteadAsset, FarmIndex, FarmBlock, FarmPaused, Block(u32), Pail(Address, u32), }

#[contractimpl]
impl SorobanContract {

    pub fn plant(
        env: Env,
        farmer: Address,
        mut amount: i128,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut amount_val: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        a = -336;
        'label0: {
            {
                'label4: {
                    'label7: {
                        {
                            amount_val = a.wrapping_add(208);
                            Self::func68(env, amount_val, amount);
                            let mut sv2_208_i32 = mload32!(a.wrapping_add(208) as usize);
                            if sv2_208_i32 != 0 {
                                unreachable!();
                            }
                            let j = mload64!(a.wrapping_add(232) as usize);
                            amount = j;
                            e = mload64!(a.wrapping_add(224) as usize);
                            farmer.require_auth();
                            if amount < 0 {
                                break 'label7;
                            }
                            let k = env.storage().get_contract_data();
                            if k != 0 {
                                unreachable!();
                            }
                            let l = env.storage().get_contract_data(1048624);
                            g = l;
                            let m = env.storage().get_contract_data();
                            b = m;
                            env.storage().get_contract_data(amount_val);
                            let mut sv2_208_i32 = mload32!(a.wrapping_add(208) as usize);
                            if sv2_208_i32 & 1 == 0 {
                                unreachable!();
                            }
                            Self::memcpy_like_4(env, a.wrapping_add(16), a.wrapping_add(224));
                            let n = Self::func49(env);
                            let loaded_val = mload64!(a.wrapping_add(80) as usize);
                            f = loaded_val;
                            h = f.wrapping_add(300);
                            if (h as u64) < f as u64 {
                                unreachable!();
                            }
                            if n as u64 >= h as u64 {
                                break 'label4;
                            }
                            env.storage().get_contract_data(amount_val, b);
                            let mut sv2_208_i32 = mload32!(a.wrapping_add(208) as usize);
                            if sv2_208_i32 & 1 != 0 {
                                unreachable!();
                            }
                            if b != 0 {
                                env.storage().put_contract_data(a.wrapping_add(12));
                            }
                            Self::func51(env, a.wrapping_add(112), a.wrapping_add(16));
                            break 'label7;
                        }
                        unreachable!();
                    }
                    Self::fail_with_error_2(env, 25769803779 /* Error(Contract, #6) */);
                    unreachable!();
                    Self::fail_with_error_2(env, 12884901891 /* Error(Contract, #3) */);
                    unreachable!();
                    Self::fail_with_error_2(env, 17179869187 /* Error(Contract, #4) */);
                    unreachable!();
                }
                amount_val = a.wrapping_add(16);
                Self::func51(env, a.wrapping_add(112), amount_val);
                env.storage().get_contract_data(a.wrapping_add(208), b);
                Self::func48(env, amount_val);
                env.storage().put_contract_data(a.wrapping_add(12));
                amount_val = mload32!(a.wrapping_add(12) as usize);
                let mut sv2_216_i64 = farmer as i64;
                let sv2_208_i32 = 6 as i32;
                let o = Self::func40(env, a.wrapping_add(208));
                let p = Self::call_eq_one(env, o, 0);
                if p != 0 {
                    unreachable!();
                }
                let q = mload64!(a.wrapping_add(152) as usize);
                d = q;
                f = mload64!(a.wrapping_add(144) as usize);
                h = f.wrapping_add(e);
                f = (((h as u64) < f as u64) as i32 as u32 as i64).wrapping_add(amount.wrapping_add(d));
                if (d ^ amount ^ 18446744073709551615) & (d ^ f) < 0 {
                    unreachable!();
                }
                if amount | e != 0 {
                    Self::func71(env, e, amount);
                    b = 0;
                    while b != 16 {
                        mstore64!(a.wrapping_add(208).wrapping_add(b) as usize, 0 as u64);
                        b = b.wrapping_add(8);
                    }
                    b = 0;
                    loop {
                        let s = mload64!(a.wrapping_add(320).wrapping_add(b) as usize);
                        mstore64!(a.wrapping_add(208).wrapping_add(b) as usize, s as u64);
                        b = b.wrapping_add(8);
                    }
                    let t = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    Self::func72(
                        env,
                        g,
                        burn,
                        t,
                    );
                    break 'label0;
                }
                b = a.wrapping_add(40);
                d = mload64!(b as usize);
                let mut sv2_32_i64 = mload64!(a.wrapping_add(32) as usize);
                g = sv2_32_i64;
                if (amount == d) as i32 != 0 {
                    b = (g as u64 > e as u64) as i32;
                } else {
                    b = (amount < d) as i32;
                }
                let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                d = value_lo;
                let v = d;
                let w = d;
                let mut value_hi = mload64!(a.wrapping_add(24) as usize);
                d = value_hi;
                if (amount == d) as i32 != 0 {
                    c = ((w as u64) < e as u64) as i32;
                } else {
                    c = (d < amount) as i32;
                }
                value_lo = ((if c != 0 { v } else { e })) as i64;
                value_hi = ((if c != 0 { d } else { amount })) as i64;
                sv2_32_i64 = ((if b != 0 { g } else { e })) as i64;
                Self::ledger_sequence_u32(env);
                let sv2_216_i64 = amount as i64;
                env.storage().put_contract_data(farmer, amount_val, a.wrapping_add(208));
                Self::func65(env, amount_val, a.wrapping_add(112));
                Self::func61(env, a.wrapping_add(16));
                Self::func55(env);
            }
        }
        Self::fail_with_error_2(env, 34359738371 /* Error(Contract, #8) */);
    }

    pub fn work(
        env: Env,
        farmer: Address,
        mut hash: BytesN<32>,
        mut nonce: u64,
    ) -> u32 {
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut m: i32 = 0;
        let mut n: i32 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut s: i64 = 0;
        let mut t: i64 = 0;
        let mut u: i64 = 0;
        c = -400;
        'label0: {
            {
                'label2: {
                    'label5: {
                        'label6: {
                            {
                                f = c.wrapping_add(208);
                                Self::func74(env, f, hash);
                                let c_i32_208 = mload32!(c.wrapping_add(208) as usize);
                                if c_i32_208 != 0 {
                                    unreachable!();
                                }
                                p = mload64!(c.wrapping_add(216) as usize);
                                Self::val_to_i64_checked(env, f, nonce);
                                let c_i32_208_2 = mload32!(c.wrapping_add(208) as usize);
                                if c_i32_208_2 != 0 {
                                    unreachable!();
                                }
                                hash = mload64!(c.wrapping_add(216) as usize);
                                let w = env.storage().get_contract_data();
                                h = w;
                                env.storage().get_contract_data(f);
                                if mload32!(c.wrapping_add(208) as usize) & 1 == 0 {
                                    unreachable!();
                                }
                                d = c.wrapping_add(224);
                                Self::memcpy_like_4(env, c.wrapping_add(16), d);
                                env.storage().get_contract_data(f, h);
                                if mload32!(c.wrapping_add(208) as usize) & 1 == 0 {
                                    unreachable!();
                                }
                                Self::memcpy_like_4(env, c.wrapping_add(112), d);
                                env.storage().get_contract_data(f, farmer, h);
                                let mut sv3_224_i32 = mload32!(c.wrapping_add(224) as usize);
                                if sv3_224_i32 == 2 {
                                    break 'label6;
                                }
                                let x = mload32!(c.wrapping_add(252) as usize);
                                mstore32!(c.wrapping_add(328) as usize, x as u32);
                                nonce = mload64!(c.wrapping_add(216) as usize);
                                s = mload64!(c.wrapping_add(208) as usize);
                                let mut sv3_240_i32 = mload32!(c.wrapping_add(240) as usize);
                                j = sv3_240_i32;
                                let mut sv3_236_i32 = mload32!(c.wrapping_add(236) as usize);
                                k = sv3_236_i32;
                                let mut sv3_232_i32 = mload32!(c.wrapping_add(232) as usize);
                                d = f;
                                g = 0.wrapping_sub(d) & 3;
                                e = d.wrapping_add(g);
                                if d as u32 < e as u32 {
                                    if g != 0 {
                                        i = g;
                                        loop {
                                            mstore8!(d as usize, 0 as u8);
                                            d += 1;
                                            i -= 1;
                                        }
                                    }
                                    if (g.wrapping_sub(1) as u32) >= 7 as u32 {
                                        loop {
                                            mstore8!(d as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(7) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(6) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(5) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(4) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(3) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(2) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(1) as usize, 0 as u8);
                                            d = d.wrapping_add(8);
                                        }
                                    }
                                }
                                g = 76.wrapping_sub(g);
                                d = e.wrapping_add(g & -4);
                                if d as u32 > e as u32 {
                                    loop {
                                        e = e.wrapping_add(4);
                                    }
                                }
                                g = g & 3;
                                i = g.wrapping_add(d);
                                if d as u32 < i as u32 {
                                    e = g;
                                    if e != 0 {
                                        loop {
                                            mstore8!(d as usize, 0 as u8);
                                            d += 1;
                                            e -= 1;
                                        }
                                    }
                                    if (g.wrapping_sub(1) as u32) >= 7 as u32 {
                                        loop {
                                            mstore8!(d as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(7) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(6) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(5) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(4) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(3) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(2) as usize, 0 as u8);
                                            mstore8!(d.wrapping_add(1) as usize, 0 as u8);
                                            d = d.wrapping_add(8);
                                        }
                                    }
                                }
                                d = c.wrapping_add(360);
                                let mut sv4_0_i64 = 0 as i64;
                                e = c.wrapping_add(352);
                                g = c.wrapping_add(344);
                                let y = val_to_i64(Bytes::from_val(env, &val_from_i64(farmer)).into_val(env));
                                o = y;
                                let z = Bytes::from_val(env, &val_from_i64(o)).len() as i64;
                                t = z;
                                if (t as u64) < 137438953472 as u64 {
                                    unreachable!();
                                }
                                let aa = Self::func75(env, o, ((t as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_sub(32));
                                o = aa;
                                let ab = Bytes::from_val(env, &val_from_i64(o)).len() as i64;
                                if ab & 18446744069414584320 != 137438953472 {
                                    unreachable!();
                                }
                                Self::func76(env, o, c.wrapping_add(336));
                                (h.wrapping_shl(24 as u32) | (h & 65280).wrapping_shl(8 as u32) | (h as u32).wrapping_shr(8 as u32) as i32 & 65280 | (h as u32).wrapping_shr(24 as u32) as i32) as i32;
                                i = c.wrapping_add(392);
                                m = c.wrapping_add(384);
                                n = c.wrapping_add(376);
                                Self::func76(env, mload64!(c.wrapping_add(184) as usize), c.wrapping_add(368));
                                mstore64!(c.wrapping_add(244) as usize, mload64!(i as usize) as u64);
                                mstore64!(c.wrapping_add(236) as usize, mload64!(m as usize) as u64);
                                mstore64!(c.wrapping_add(228) as usize, mload64!(n as usize) as u64);
                                mstore64!(c.wrapping_add(260) as usize, mload64!(g as usize) as u64);
                                mstore64!(c.wrapping_add(268) as usize, mload64!(e as usize) as u64);
                                let mut sv4_0_i64 = mload64!(d as usize);
                                mstore64!(c.wrapping_add(276) as usize, sv4_0_i64 as u64);
                                let ac = val_to_i64(Bytes::new(env).into_val(env));
                                let ad = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(ac))).into());
                                t = ad;
                                let ae = Self::ledger_sequence_u32(env);
                                d = ae;
                                if (d as u32) < j as u32 {
                                    unreachable!();
                                }
                                if d == j {
                                    break 'label5;
                                }
                                let af = { let a = val_from_i64(p); let b = val_from_i64(t); if a < b { -1 } else if a > b { 1 } else { 0 } };
                                if af != 0 {
                                    unreachable!();
                                }
                                f = d.wrapping_sub(j);
                                let mut sv3_208_i64 = p as i64;
                                d = 0;
                                loop {
                                    Self::func77(env, c.wrapping_add(8), c.wrapping_add(208));
                                    let ag = mload8!(c.wrapping_add(8) as usize) as i32;
                                    if ag == 0 {
                                        e = d;
                                        break 'label2;
                                    }
                                    let ah = mload8!(c.wrapping_add(9) as usize) as i32;
                                    e = ah;
                                    if e == 0 {
                                        let ai = d;
                                        d = d.wrapping_add(2);
                                        if ai as u32 > d as u32 {
                                            unreachable!();
                                        }
                                    }
                                }
                                e = d.wrapping_add((e.wrapping_shl(24 as u32).leading_zeros() as i32 as u32).wrapping_shr(2 as u32) as i32);
                                if e as u32 >= d as u32 {
                                    break 'label2;
                                }
                                unreachable!();
                            }
                            unreachable!();
                            Self::fail_with_error_2(env, 12884901891 /* Error(Contract, #3) */);
                            unreachable!();
                            Self::fail_with_error_2(env, 47244640259 /* Error(Contract, #11) */);
                            unreachable!();
                        }
                        Self::fail_with_error_2(env, 38654705667 /* Error(Contract, #9) */);
                        unreachable!();
                    }
                    Self::fail_with_error_2(env, 64424509443 /* Error(Contract, #15) */);
                    unreachable!();
                    Self::fail_with_error_2(env, 55834574851 /* Error(Contract, #13) */);
                    unreachable!();
                }
                Self::func52(
                    env,
                    c.wrapping_add(208),
                    c.wrapping_add(112),
                    f,
                    s,
                    nonce,
                    e,
                );
                p = mload64!(c.wrapping_add(216) as usize);
                let aj = mload64!(c.wrapping_add(232) as usize);
                hash = aj;
                let ak = hash;
                q = mload64!(c.wrapping_add(208) as usize);
                let c_part_224_2 = mload64!(c.wrapping_add(224) as usize);
                o = q.wrapping_add(c_part_224_2);
                hash = (((o as u64) < q as u64) as i32 as u32 as i64).wrapping_add(hash.wrapping_add(p));
                if (p ^ ak ^ 18446744073709551615) & (p ^ hash) < 0 {
                    unreachable!();
                }
                let al = mload64!(c.wrapping_add(248) as usize);
                p = al;
                let c_part_240_2 = mload64!(c.wrapping_add(240) as usize);
                r = o.wrapping_add(c_part_240_2);
                o = (((r as u64) < o as u64) as i32 as u32 as i64).wrapping_add(hash.wrapping_add(p));
                if (hash ^ p ^ 18446744073709551615) & (hash ^ o) < 0 {
                    unreachable!();
                }
                let am = mload64!(c.wrapping_add(168) as usize);
                q = am;
                hash = mload64!(c.wrapping_add(160) as usize);
                p = hash.wrapping_add(r);
                hash = (((p as u64) < hash as u64) as i32 as u32 as i64).wrapping_add(o.wrapping_add(q));
                if (q ^ o ^ 18446744073709551615) & (q ^ hash) < 0 {
                    unreachable!();
                }
                if sv3_232_i32 & 1 == 0 {
                    let an = mload64!(c.wrapping_add(152) as usize);
                    hash = an;
                    p = mload64!(c.wrapping_add(144) as usize);
                    o = hash.wrapping_sub(nonce).wrapping_sub(((p as u64) < s as u64) as i32 as u32 as i64);
                    if (hash ^ nonce) & (hash ^ o) < 0 {
                        unreachable!();
                    }
                    hash = p.wrapping_sub(s);
                    d = c.wrapping_add(144);
                    break 'label0;
                }
                if e as u32 > k as u32 {
                    Self::func52(
                        env,
                        c.wrapping_add(208),
                        c.wrapping_add(112),
                        f,
                        s,
                        nonce,
                        k,
                    );
                    q = mload64!(c.wrapping_add(216) as usize);
                    let ao = mload64!(c.wrapping_add(232) as usize);
                    o = ao;
                    let ap = o;
                    u = mload64!(c.wrapping_add(208) as usize);
                    let c_part_224 = mload64!(c.wrapping_add(224) as usize);
                    r = u.wrapping_add(c_part_224);
                    o = (((r as u64) < u as u64) as i32 as u32 as i64).wrapping_add(o.wrapping_add(q));
                    if (q ^ ap ^ 18446744073709551615) & (q ^ o) < 0 {
                        unreachable!();
                    }
                    let aq = mload64!(c.wrapping_add(248) as usize);
                    u = aq;
                    let c_part_240 = mload64!(c.wrapping_add(240) as usize);
                    q = r.wrapping_add(c_part_240);
                    r = (((q as u64) < r as u64) as i32 as u32 as i64).wrapping_add(o.wrapping_add(u));
                    if (o ^ u ^ 18446744073709551615) & (o ^ r) < 0 {
                        unreachable!();
                    }
                    o = hash.wrapping_sub(r).wrapping_sub(((p as u64) < q as u64) as i32 as u32 as i64);
                    if (hash ^ r) & (hash ^ o) < 0 {
                        unreachable!();
                    }
                    d = c.wrapping_add(160);
                    hash = p.wrapping_sub(q);
                    break 'label0;
                }
                Self::fail_with_error_2(env, 30064771075 /* Error(Contract, #7) */);
                unreachable!();
            }
        }
        let sv4_0_i64 = hash as i64;
        let mut sv3_104_i32 = mload32!(c.wrapping_add(104) as usize);
        d = sv3_104_i32;
        sv3_104_i32 = ((if ((d as u32) < f as u32) as i32 != 0 { f } else { d })) as i32;
        let mut sv3_96_i32 = mload32!(c.wrapping_add(96) as usize);
        d = sv3_96_i32;
        sv3_96_i32 = ((if (d as u32 > f as u32) as i32 != 0 { f } else { d })) as i32;
        let mut sv3_108_i32 = mload32!(c.wrapping_add(108) as usize);
        d = sv3_108_i32;
        sv3_108_i32 = ((if ((d as u32) < e as u32) as i32 != 0 { e } else { d })) as i32;
        let mut sv3_100_i32 = mload32!(c.wrapping_add(100) as usize);
        d = sv3_100_i32;
        sv3_100_i32 = ((if (d as u32 > e as u32) as i32 != 0 { e } else { d })) as i32;
        let ar = mload32!(c.wrapping_add(328) as usize);
        mstore32!(c.wrapping_add(252) as usize, ar as u32);
        let sv3_208_i64 = s as i64;
        let sv3_240_i32 = j as i32;
        let sv3_236_i32 = e as i32;
        let sv3_232_i32 = 1 as i32;
        let sv3_224_i32 = 1 as i32;
        mload64!(c.wrapping_add(320) as usize);
        env.storage().put_contract_data(farmer, h, c.wrapping_add(208));
        Self::func65(env, h, c.wrapping_add(112));
        Self::func61(env, c.wrapping_add(16));
        Self::func55(env);
        (f as u32 as i64).wrapping_shl(32 as u32) | 0
    }

    pub fn harvest(
        env: Env,
        farmer: Address,
        mut index: u32,
    ) -> i128 {
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
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        a = -240;
        {
            'label1: {
                if (!(Address::try_from_val(env, &val_from_i64(farmer)).is_ok())) as i32 | !(index & 255 != 0) {
                    let r = env.storage().get_contract_data(1048624);
                    let s = env.storage().get_contract_data();
                    d = s;
                    b = a.wrapping_add(112);
                    c = (index as u64).wrapping_shr(32 as u32) as i64 as i32;
                    env.storage().get_contract_data(b, c);
                    let mut sv2_112_i32 = mload32!(a.wrapping_add(112) as usize);
                    if sv2_112_i32 & 1 != 0 {
                        e = a.wrapping_add(16);
                        Self::memcpy_like_4(env, e, a.wrapping_add(128));
                        env.storage().get_contract_data(b, farmer, c);
                        f = mload32!(a.wrapping_add(128) as usize);
                        if f != 2 {
                            if (c as u32) < d as u32 {
                                if f == 0 {
                                    unreachable!();
                                }
                                d = mload32!(a.wrapping_add(136) as usize);
                                if d == 0 {
                                    unreachable!();
                                }
                                if d & 1 != 0 {
                                    n = mload64!(a.wrapping_add(112) as usize);
                                    l = mload64!(a.wrapping_add(120) as usize);
                                    Self::func52(
                                        env,
                                        b,
                                        e,
                                        mload32!(a.wrapping_add(132) as usize),
                                        n,
                                        l,
                                        mload32!(a.wrapping_add(140) as usize),
                                    );
                                    b = c.wrapping_sub(30558);
                                    b = 0.wrapping_sub((((if (b as u32 <= c as u32) as i32 != 0 { b } else { 0 })) as u32 / 8640 as u32) as i32);
                                    let t = mload64!(a.wrapping_add(152) as usize);
                                    i = t;
                                    let u = mload64!(a.wrapping_add(136) as usize);
                                    g = u;
                                    o = mload64!(a.wrapping_add(144) as usize);
                                    p = mload64!(a.wrapping_add(128) as usize);
                                    h = mload64!(a.wrapping_add(120) as usize);
                                    k = mload64!(a.wrapping_add(112) as usize);
                                    index = 1000000000000;
                                    while b != 0 {
                                        Self::func53(
                                            env,
                                            a.wrapping_add(112),
                                            index,
                                            j,
                                            950000000000,
                                            0,
                                            1000000000000,
                                            0,
                                        );
                                        b += 1;
                                        j = mload64!(a.wrapping_add(120) as usize);
                                        index = mload64!(a.wrapping_add(112) as usize);
                                    }
                                    Self::func53(
                                        env,
                                        a.wrapping_add(112),
                                        25050000000,
                                        0,
                                        index,
                                        j,
                                        1000000000000,
                                        0,
                                    );
                                    j = k.wrapping_add(p);
                                    index = ((k as u64 > j as u64) as i32 as u32 as i64).wrapping_add(g.wrapping_add(h));
                                    if (g ^ h ^ 18446744073709551615) & (h ^ index) >= 0 {
                                        let w = i;
                                        h = j.wrapping_add(o);
                                        i = (((h as u64) < j as u64) as i32 as u32 as i64).wrapping_add(index.wrapping_add(i));
                                        if (index ^ w ^ 18446744073709551615) & (index ^ i) >= 0 {
                                            index = mload64!(a.wrapping_add(120) as usize);
                                            let x = mload64!(a.wrapping_add(56) as usize);
                                            g = x;
                                            let y = g;
                                            k = mload64!(a.wrapping_add(112) as usize);
                                            let a_part_48 = mload64!(a.wrapping_add(48) as usize);
                                            j = k.wrapping_add(a_part_48);
                                            g = (((j as u64) < k as u64) as i32 as u32 as i64).wrapping_add(index.wrapping_add(g));
                                            if (index ^ y ^ 18446744073709551615) & (index ^ g) >= 0 {
                                                index = mload64!(a.wrapping_add(64) as usize);
                                                let z = index;
                                                let aa = index;
                                                let ab = mload64!(a.wrapping_add(72) as usize);
                                                index = ab;
                                                if (index == 0) as i32 != 0 {
                                                    b = (aa as u64 > 1 as u64) as i32;
                                                } else {
                                                    b = (index > 0) as i32;
                                                }
                                                Self::func53(
                                                    env,
                                                    a,
                                                    h,
                                                    i,
                                                    j,
                                                    g,
                                                    (if b != 0 { z } else { 1 }),
                                                    (if b != 0 { index } else { 0 }),
                                                );
                                                let vec_builder = mload64!(a.wrapping_add(8) as usize);
                                                index = vec_builder;
                                                let sv2_0_i64 = mload64!(a as usize);
                                                i = sv2_0_i64;
                                                g = i.wrapping_add(n);
                                                h = (((g as u64) < i as u64) as i32 as u32 as i64).wrapping_add(index.wrapping_add(l));
                                                if (index ^ l ^ 18446744073709551615) & (index ^ h) >= 0 {
                                                    let cond_val = if h == 0 { (g != 0) as i32 } else { (h > 0) as i32 };
                                                    if cond_val == 0 {
                                                        unreachable!();
                                                    }
                                                    Self::func71(env, g, h);
                                                    b = 0;
                                                    while b != 16 {
                                                        mstore64!(a.wrapping_add(112).wrapping_add(b) as usize, 0 as u64);
                                                        b = b.wrapping_add(8);
                                                    }
                                                    b = 0;
                                                    loop {
                                                        let ad = mload64!(a.wrapping_add(224).wrapping_add(b) as usize);
                                                        mstore64!(a.wrapping_add(112).wrapping_add(b) as usize, ad as u64);
                                                        b = b.wrapping_add(8);
                                                    }
                                                    let ae = val_to_i64(vec![&env, val_from_i64(sv2_0_i64), val_from_i64(vec_builder)].into_val(env));
                                                    Self::func72(
                                                        env,
                                                        r,
                                                        mint,
                                                        ae,
                                                    );
                                                    break 'label1;
                                                    unreachable!();
                                                }
                                            }
                                        }
                                    }
                                }
                                unreachable!();
                            }
                            Self::fail_with_error_2(env, 60129542147 /* Error(Contract, #14) */);
                            unreachable!();
                        }
                        Self::fail_with_error_2(env, 38654705667 /* Error(Contract, #9) */);
                        unreachable!();
                    }
                    Self::fail_with_error_2(env, 47244640259 /* Error(Contract, #11) */);
                    unreachable!();
                }
                unreachable!();
            }
            let sv2_112_i32 = 6 as i32;
            let af = Self::func40(env, a.wrapping_add(112));
            env.storage().del_contract_data(af);
            Self::func55(env);
            let ag = Self::func71(env, i, index);
            return ag;
        }
        Self::fail_with_error_2(env, 42949672963 /* Error(Contract, #10) */);
        unreachable!();
    }

    pub fn __constructor(
        env: Env,
        farmer: Address,
        asset: Address,
    ) {
        let a: i32 = -96;
        if (!(Address::try_from_val(env, &val_from_i64(farmer)).is_ok())) as i32 | Address::try_from_val(env, &val_from_i64(asset)).is_ok() {
            farmer.require_auth();
            let c = Self::func40(env, 1048608);
            let d = Self::call_eq_one(env, c, 0);
            if d != 0 {
                unreachable!();
            }
            env.storage().put_contract_data(1048608, farmer);
            env.storage().put_contract_data(1048624, asset);
            Self::func48(env, a);
            Self::func61(env, a);
            Self::func55(env);
        }
        unreachable!();
        Self::fail_with_error_2(env, 4294967299 /* Error(Contract, #1) */);
    }

    pub fn upgrade(
        env: Env,
        hash: BytesN<32>,
    ) {
        let a: i32 = -16;
        Self::func74(env, a, hash);
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        let authorized_addr = env.storage().get_contract_data(1048608);
        address_from_i64(&env, authorized_addr).require_auth();
        let a_hi = mload64!(a.wrapping_add(8) as usize);
        env.deployer().update_current_contract_wasm(BytesN::<32>::from_val(env, &val_from_i64(a_hi)));
        Self::func55(env);
    }

    pub fn pause(
        env: Env,
    ) {
        let authorized_addr = env.storage().get_contract_data(1048608);
        address_from_i64(&env, authorized_addr).require_auth();
        let a = env.storage().get_contract_data();
        if a != 0 {
            Self::fail_with_error_2(env, 17179869187 /* Error(Contract, #4) */);
            unreachable!();
        }
        env.storage().put_contract_data(1);
    }

    pub fn unpause(
        env: Env,
    ) {
        let authorized_addr = env.storage().get_contract_data(1048608);
        address_from_i64(&env, authorized_addr).require_auth();
        let a = env.storage().get_contract_data();
        if a != 0 {
            Self::fail_with_error_2(env, 21474836483 /* Error(Contract, #5) */);
            unreachable!();
        }
        env.storage().put_contract_data(0);
        Self::func55(env);
    }

    pub fn remove_block(
        env: Env,
        index: u32,
    ) {
        let a: i32 = -16;
        let authorized_addr = env.storage().get_contract_data(1048608);
        address_from_i64(&env, authorized_addr).require_auth();
        mstore32!(a.wrapping_add(4) as usize, (index as u64).wrapping_shr(32 as u32) as i64 as u32);
        let c = Self::func40(env, a);
        env.storage().del_contract_data(c);
    }

    pub fn __check_auth(
        env: Env,
        _signature_payload: BytesN<32>,
        _signatures: Option<Vec<Val>>,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Errors> {
        let a: i32 = -16;
        Self::func74(env, a, __signature_payload);
        if (__signatures != 0) as i32 & (!(Vec::<Val>::try_from_val(env, &val_from_i64(__signatures)).is_ok())) as i32 | (mload32!(a as usize) == 1) as i32 | Vec::<Val>::try_from_val(env, &val_from_i64(__auth_contexts)).is_ok() {
            let authorized_addr = env.storage().get_contract_data(1048608);
            let c = val_to_i64(Vec::<Val>::new(env).into_val(env));
            address_from_i64(&env, authorized_addr).require_auth_for_args(val_from_i64(c));
            return 0;
        }
    }
}

impl SorobanContract {

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


    fn func40(
        env: &Env,
        mut arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = -32;
        'label0: {
            {
                'label2: {
                    {
                        arg0 = a.wrapping_add(8);
                        Self::func45(
                            env,
                            arg0,
                            1048920,
                            11,
                        );
                        break 'label2;
                        arg0 = a.wrapping_add(8);
                        Self::func45(
                            env,
                            arg0,
                            1048931,
                            14,
                        );
                        break 'label2;
                        arg0 = a.wrapping_add(8);
                        Self::func45(
                            env,
                            arg0,
                            1048945,
                            9,
                        );
                        break 'label2;
                        arg0 = a.wrapping_add(8);
                        Self::func45(
                            env,
                            arg0,
                            1048954,
                            9,
                        );
                        break 'label2;
                        arg0 = a.wrapping_add(8);
                        Self::func45(
                            env,
                            arg0,
                            1048963,
                            10,
                        );
                        break 'label2;
                        b = a.wrapping_add(8);
                        Self::func45(
                            env,
                            b,
                            1048973,
                            5,
                        );
                        let a_i32_8_2 = mload32!(a.wrapping_add(8) as usize);
                        if a_i32_8_2 != 0 {
                            unreachable!();
                        }
                        c = mload32!(arg0.wrapping_add(4) as usize) as i64;
                        let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                        let mut sv1_8_i64 = value_lo as i64;
                        value_lo = (c.wrapping_shl(32 as u32) | 0) as i64;
                        let f = val_to_i64(Vec::<Val>::new(env).into_val(env));
                        c = f;
                        break 'label0;
                    }
                    b = a.wrapping_add(8);
                    Self::func45(
                        env,
                        b,
                        1048978,
                        4,
                    );
                    let a_i32_8_3 = mload32!(a.wrapping_add(8) as usize);
                    if a_i32_8_3 != 0 {
                        unreachable!();
                    }
                    let value_lo = mload64!(a.wrapping_add(16) as usize);
                    c = value_lo;
                    let sv1_8_i64 = c as i64;
                    let g = val_to_i64(Vec::<Val>::new(env).into_val(env));
                    c = g;
                    break 'label0;
                }
                let a_i32_8 = mload32!(a.wrapping_add(8) as usize);
                if a_i32_8 != 0 {
                    unreachable!();
                }
                c = mload64!(a.wrapping_add(16) as usize);
                b = -16;
                let i = val_to_i64(Vec::<Val>::new(env).into_val(env));
                c = i;
                let mload64!(arg0.wrapping_add(8) as usize) = c as i64;
                c = mload64!(a.wrapping_add(16) as usize);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if a_hi == 0 {
                    unreachable!();
                }
            }
        }
        c
    }

    fn func41(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i64;
        if (arg2 ^ arg1.wrapping_shr(63 as u32) != 0) as i32 | arg1.wrapping_sub(18410715276690587648) as u64 > 72057594037927935 as u64 {
            let b = val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)));
        } else {
            b = arg1 | 0;
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

    fn call_eq_one(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let a = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (a == 1) as i32
    }


    fn func45(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let e: i64;
        'label1: {
            if arg2 as u32 <= 9 as u32 {
                b = arg2;
                c = arg1;
                loop {
                    let f = d | 0 /* Symbol() */;
                    if b == 0 {
                        unreachable!();
                    }
                    let g: i32;
                    'label3: {
                        let h = mload8!(c as usize) as i32;
                        a = h;
                        let i = 1;
                        if a == 95 {
                            g = i;
                        } else {
                            if (a.wrapping_sub(48) & 255) as u32 >= 10 as u32 {
                                if (a.wrapping_sub(65) & 255) as u32 >= 26 as u32 {
                                    if (a.wrapping_sub(97) & 255) as u32 > 25 as u32 {
                                        break 'label1;
                                    }
                                    g = a.wrapping_sub(59);
                                    break 'label3;
                                }
                                g = a.wrapping_sub(53);
                                break 'label3;
                            }
                            g = a.wrapping_sub(46);
                        }
                    }
                    d = g as u32 as i64 & 255 | d;
                    b -= 1;
                    c += 1;
                }
                unreachable!();
            }
        }
        val_to_i64(Symbol::new(env, ""));
    }


    fn func47(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let c: i32;
    }

    fn func48(
        env: &Env,
        arg0: i32,
    ) {
        Self::func49(env);
        val_to_i64(Bytes::new(env).into_val(env));
        mstore64!(arg0.wrapping_add(40) as usize, 0 as u64);
        mstore64!(arg0.wrapping_add(48) as usize, 0 as u64);
        mstore64!(arg0.wrapping_add(56) as usize, 0 as u64);
    }

    fn func49(
        env: &Env,
    ) -> i64 {
        let mut a: i64 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        b = -16;
        let e: i64;
        'label0: {
            let f = env.ledger().timestamp() as i64;
            a = f;
            c = a as i32 & 255;
            if c != 64 {
                let g = (a as u64) as i64;
                if c == 6 {
                    e = g;
                    break 'label0;
                }
                unreachable!();
            }
            let h = val_from_i64(a).as_u64().unwrap_or(0) as i64;
            e = h;
        }
        e
    }


    fn func51(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        Self::func49(env);
        mload64!(arg1.wrapping_add(72) as usize);
        mstore64!(arg0.wrapping_add(40) as usize, 0 as u64);
        mstore64!(arg0.wrapping_add(48) as usize, 0 as u64);
        mstore64!(arg0.wrapping_add(56) as usize, 0 as u64);
        a = mload32!(arg1.wrapping_add(80) as usize);
        a = mload32!(arg1.wrapping_add(84) as usize);
        let e = mload64!(arg1.wrapping_add(8) as usize);
        b = e;
        c = mload64!(arg1 as usize);
        a = (c ^ 18446744073709551615 | b ^ 9223372036854775807 == 0) as i32;
        let f = mload64!(arg1.wrapping_add(24) as usize);
        b = f;
        c = mload64!(arg1.wrapping_add(16) as usize);
        arg1 = (c | b ^ 9223372036854775808 == 0) as i32;
    }

    fn func52(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
        mut arg2: i32,
        mut arg3: i64,
        mut arg4: i64,
        arg5: i32,
    ) {
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;
        let mut n: i32 = 0;
        let mut o: i32 = 0;
        k = -16;
        'label0: {
            m = mload32!(arg1.wrapping_add(88) as usize);
            l = mload32!(arg1.wrapping_add(80) as usize);
            if (m as u32) >= l as u32 {
                e = mload64!(arg1.wrapping_add(16) as usize);
                c = mload64!(arg1 as usize);
                o = ((e as u64) < c as u64) as i32;
                let q = mload64!(arg1.wrapping_add(24) as usize);
                b = q;
                let r = mload64!(arg1.wrapping_add(8) as usize);
                a = r;
                let cond_val = if a == b { o } else { (b < a) as i32 };
                if cond_val == 0 {
                    n = mload32!(arg1.wrapping_add(92) as usize);
                    arg1 = mload32!(arg1.wrapping_add(84) as usize);
                    if (n as u32) >= arg1 as u32 {
                        f = b.wrapping_sub(a).wrapping_sub(o as u32 as i64);
                        if (a ^ b) & (b ^ f) < 0 {
                            break 'label0;
                        }
                        if (arg2 as u32 > l as u32) as i32 != 0 {
                            arg2 = arg2;
                        } else {
                            arg2 = l;
                        }
                        let s = l;
                        arg2 = m.wrapping_sub(l);
                        g = ((if (arg2 as u32 <= 1 as u32) as i32 != 0 { 1 } else { arg2 })) as u32 as i64;
                        h = e.wrapping_sub(c);
                        if (f == 0) as i32 != 0 {
                            arg2 = (h as u64 > 1 as u64) as i32;
                        } else {
                            arg2 = (f > 0) as i32;
                        }
                        if arg2 != 0 {
                            h = h;
                        } else {
                            h = 1;
                        }
                        if arg2 != 0 {
                            f = f;
                        } else {
                            f = 0;
                        }
                        arg2 = (f == 0) as i32 & (g as u64 > h as u64) as i32;
                        if arg2 != 0 {
                            d = g;
                        } else {
                            d = h;
                        }
                        let t = d;
                        l = n.wrapping_sub(arg1);
                        i = ((if (l as u32 <= 1 as u32) as i32 != 0 { 1 } else { l })) as u32 as i64;
                        let u = d;
                        if arg2 != 0 {
                            d = 0;
                        } else {
                            d = f;
                        }
                        if (d == 0) as i32 != 0 {
                            arg2 = (u as u64 > i as u64) as i32;
                        } else {
                            arg2 = (d != 0) as i32;
                        }
                        if arg2 != 0 {
                            j = t;
                        } else {
                            j = i;
                        }
                        if arg2 != 0 {
                            d = d;
                        } else {
                            d = 0;
                        }
                        Self::func53(
                            env,
                            k,
                            ((if ((arg2 as u32) < m as u32) as i32 != 0 { arg2 } else { m })).wrapping_sub(s) as u32 as i64,
                            0,
                            j,
                            d,
                            g,
                            0,
                        );
                        if (arg4 == a) as i32 != 0 {
                            arg2 = (arg3 as u64 > c as u64) as i32;
                        } else {
                            arg2 = (arg4 > a) as i32;
                        }
                        if arg2 != 0 {
                            arg4 = arg4;
                        } else {
                            arg4 = a;
                        }
                        if arg2 != 0 {
                            g = arg3;
                        } else {
                            g = c;
                        }
                        if (arg4 == b) as i32 != 0 {
                            arg2 = ((g as u64) < e as u64) as i32;
                        } else {
                            arg2 = (arg4 < b) as i32;
                        }
                        if arg2 != 0 {
                            arg3 = arg4;
                        } else {
                            arg3 = b;
                        }
                        let v = a;
                        if arg2 != 0 {
                            arg4 = g;
                        } else {
                            arg4 = e;
                        }
                        a = arg3.wrapping_sub(a).wrapping_sub(((arg4 as u64) < c as u64) as i32 as u32 as i64);
                        if (arg3 ^ v) & (arg3 ^ a) < 0 {
                            break 'label0;
                        }
                        arg3 = mload64!(k.wrapping_add(8) as usize);
                        b = mload64!(k as usize);
                        Self::func53(
                            env,
                            k,
                            arg4.wrapping_sub(c),
                            a,
                            j,
                            d,
                            h,
                            f,
                        );
                        c = mload64!(k as usize);
                        arg4 = mload64!(k.wrapping_add(8) as usize);
                        if ((arg1 as u32) < arg5 as u32) as i32 != 0 {
                            arg2 = arg5;
                        } else {
                            arg2 = arg1;
                        }
                        Self::func53(
                            env,
                            k,
                            ((if ((arg2 as u32) < n as u32) as i32 != 0 { arg2 } else { n })).wrapping_sub(arg1) as u32 as i64,
                            0,
                            j,
                            d,
                            i,
                            0,
                        );
                        e = mload64!(k as usize);
                        a = mload64!(k.wrapping_add(8) as usize);
                        if (arg4 == 0) as i32 != 0 {
                            arg1 = (c as u64 > 1 as u64) as i32;
                        } else {
                            arg1 = (arg4 > 0) as i32;
                        }
                        if (arg3 == 0) as i32 != 0 {
                            arg1 = (b as u64 > 1 as u64) as i32;
                        } else {
                            arg1 = (arg3 > 0) as i32;
                        }
                        if (a == 0) as i32 != 0 {
                            arg1 = (e as u64 > 1 as u64) as i32;
                        } else {
                            arg1 = (a > 0) as i32;
                        }
                    }
                }
            }
            Self::fail_with_error_2(env, 51539607555 /* Error(Contract, #12) */);
            unreachable!();
        }
    }

    fn func53(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        mut arg3: i64,
        mut arg4: i64,
        mut arg5: i64,
        mut arg6: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        b = -96;
        a = -96;
        if (arg1 | arg2 != 0) as i32 | arg3 | arg4 == 0 {
            c = (arg4 < 0) as i32;
            if c != 0 {
                h = 0.wrapping_sub(arg3);
            } else {
                h = arg3;
            }
            d = (arg2 < 0) as i32;
            if d != 0 {
                i = 0.wrapping_sub(arg1);
            } else {
                i = arg1;
            }
            if c != 0 {
                g = 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64));
            } else {
                g = arg4;
            }
            k = arg2 ^ arg4;
            let n: i64;
            'label1: {
                if d != 0 {
                    j = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
                } else {
                    j = arg2;
                }
                if j != 0 {
                    if g != 0 {
                        Self::func102(
                            env,
                            a.wrapping_add(80),
                            h,
                            g,
                            i,
                            j,
                        );
                        let o = mload64!(a.wrapping_add(88) as usize);
                        g = o;
                        c = 1;
                        let loaded_val = mload64!(a.wrapping_add(80) as usize);
                        n = loaded_val;
                        break 'label1;
                    }
                    Self::func102(
                        env,
                        a.wrapping_sub(-64),
                        i,
                        0,
                        h,
                        g,
                    );
                    Self::func102(
                        env,
                        a.wrapping_add(48),
                        j,
                        0,
                        h,
                        g,
                    );
                    let p = mload64!(a.wrapping_add(56) as usize);
                    let q = mload64!(a.wrapping_add(72) as usize);
                    h = q;
                    let a_part_48 = mload64!(a.wrapping_add(48) as usize);
                    g = h.wrapping_add(a_part_48);
                    c = (p != 0) as i32 | ((g as u64) < h as u64) as i32;
                    n = mload64!(a.wrapping_add(64) as usize);
                    break 'label1;
                }
                if g != 0 {
                    Self::func102(
                        env,
                        a.wrapping_add(32),
                        h,
                        0,
                        i,
                        j,
                    );
                    Self::func102(
                        env,
                        a.wrapping_add(16),
                        g,
                        0,
                        i,
                        j,
                    );
                    let r = mload64!(a.wrapping_add(24) as usize);
                    let s = mload64!(a.wrapping_add(40) as usize);
                    h = s;
                    let value_lo = mload64!(a.wrapping_add(16) as usize);
                    g = h.wrapping_add(value_lo);
                    c = (r != 0) as i32 | ((g as u64) < h as u64) as i32;
                    n = mload64!(a.wrapping_add(32) as usize);
                    break 'label1;
                }
                Self::func102(
                    env,
                    a,
                    h,
                    g,
                    i,
                    j,
                );
                let t = mload64!(a.wrapping_add(8) as usize);
                g = t;
                c = 0;
                n = mload64!(a as usize);
            }
            h = n;
            d = (k < 0) as i32;
            if d != 0 {
                i = 0.wrapping_sub(h);
            } else {
                i = h;
            }
            if d != 0 {
                g = 0.wrapping_sub(g.wrapping_add((h != 0) as i32 as u32 as i64));
            } else {
                g = g;
            }
            if g ^ k < 0 {
                c = 1;
            }
        }
        mstore32!(b.wrapping_add(72) as usize, c as u32);
        'label2: {
            'label4: {
                let b_i32_72 = mload32!(b.wrapping_add(72) as usize);
                if b_i32_72 != 0 {
                    let u = Self::func93(env, arg1, arg2);
                    arg2 = u;
                    let v = Self::func93(env, arg3, arg4);
                    arg3 = v;
                    let w = Self::func93(env, arg5, arg6);
                    arg1 = w;
                    a = b.wrapping_add(78);
                    let x: i64;
                    'label5: {
                        {
                            let y = val_to_i64(I256::try_from_val(env, &val_from_i64(arg2)).unwrap().mul(&I256::try_from_val(env, &val_from_i64(arg3)).unwrap()).into_val(env));
                            arg2 = y;
                            let z = Self::func94(env, arg2);
                            if z == 0 {
                                let aa = Self::func95(env, arg2);
                                if aa != 0 {
                                    let ab = Self::func94(env, arg1);
                                    if ab != 0 {
                                        unreachable!();
                                    }
                                }
                                let ac = val_to_i64(I256::try_from_val(env, &val_from_i64(arg2)).unwrap().div(&I256::try_from_val(env, &val_from_i64(arg1)).unwrap()).into_val(env));
                                x = ac;
                                break 'label5;
                            }
                        }
                        let ad = val_to_i64(I256::try_from_val(env, &val_from_i64(arg2)).unwrap().rem_euclid(&I256::try_from_val(env, &val_from_i64(arg1)).unwrap()).into_val(env));
                        arg3 = ad;
                        let ae = val_to_i64(I256::try_from_val(env, &val_from_i64(arg2)).unwrap().div(&I256::try_from_val(env, &val_from_i64(arg1)).unwrap()).into_val(env));
                        let af = Self::func95(env, arg3);
                        let ag = val_to_i64(I256::try_from_val(env, &val_from_i64(ae)).unwrap().sub(&I256::try_from_val(env, &val_from_i64((if af != 0 { 1 } else { 0 }))).unwrap()).into_val(env));
                        x = ag;
                    }
                    let ah = val_to_i64(I256::try_from_val(env, &val_from_i64(x)).unwrap().to_be_bytes().into_val(env));
                    arg1 = ah;
                    let ai = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(0 as u32..16 as u32).into_val(env));
                    Self::func96(env, a, ai);
                    let aj = mload8!(b.wrapping_add(78) as usize) as i32;
                    if aj == 1 {
                        break 'label2;
                    }
                    c = b.wrapping_add(87);
                    arg2 = mload64!(c as usize);
                    arg3 = mload64!(b.wrapping_add(79) as usize);
                    let ak = val_to_i64(Bytes::from_val(env, &val_from_i64(arg1)).slice(16 as u32..32 as u32).into_val(env));
                    Self::func96(env, a, ak);
                    let al = mload8!(b.wrapping_add(78) as usize) as i32;
                    if al == 1 {
                        break 'label2;
                    }
                    arg1 = mload64!(c as usize);
                    arg5 = arg1.wrapping_shl(56 as u32) | (arg1 & 65280).wrapping_shl(40 as u32) | (arg1 & 16711680).wrapping_shl(24 as u32) | (arg1 & 4278190080) | (arg1 as u64) as i64 & 4278190080 | (arg1 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (arg1 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (arg1 as u64).wrapping_shr(56 as u32) as i64;
                    arg1 = mload64!(b.wrapping_add(79) as usize);
                    arg6 = arg1.wrapping_shl(56 as u32) | (arg1 & 65280).wrapping_shl(40 as u32) | (arg1 & 16711680).wrapping_shl(24 as u32) | (arg1 & 4278190080) | (arg1 as u64) as i64 & 4278190080 | (arg1 as u64).wrapping_shr(24 as u32) as i64 & 16711680 | (arg1 as u64).wrapping_shr(40 as u32) as i64 & 65280 | (arg1 as u64).wrapping_shr(56 as u32) as i64;
                    if (arg2 | arg3 == 0) as i32 & arg6 >= 0 {
                        unreachable!();
                    }
                    if arg2 & arg3 != 18446744073709551615 {
                        unreachable!();
                    }
                    if arg6 < 0 {
                        break 'label4;
                    }
                    unreachable!();
                }
                arg1 = mload64!(b.wrapping_add(56) as usize);
                {
                    {
                        let am = mload64!(b.wrapping_sub(-64) as usize);
                        arg2 = am;
                        if arg2 >= 0 {
                            if (!(arg1 | arg2 == 0)) as i32 & arg6 < 0 {
                                unreachable!();
                            }
                            if arg5 | arg6 == 0 {
                                unreachable!();
                            }
                            Self::func100(
                                env,
                                b.wrapping_add(40),
                                arg1,
                                arg2,
                                arg5,
                                arg6,
                            );
                            let an = mload64!(b.wrapping_add(48) as usize);
                            arg6 = an;
                            arg5 = mload64!(b.wrapping_add(40) as usize);
                            break 'label4;
                        }
                        if arg5 | arg6 == 0 {
                            unreachable!();
                        }
                        a = (arg5 & arg6 == 18446744073709551615) as i32;
                        c = (arg1 | arg2 ^ 9223372036854775808 != 0) as i32;
                        if a & c == 0 {
                            unreachable!();
                        }
                        if c != 0 {
                            unreachable!();
                        }
                        if a != 0 {
                            unreachable!();
                        }
                    }
                    Self::func100(
                        env,
                        b.wrapping_add(24),
                        arg1,
                        arg2,
                        arg5,
                        arg6,
                    );
                    arg4 = mload64!(b.wrapping_add(24) as usize);
                    let ao = mload64!(b.wrapping_add(32) as usize);
                    arg3 = ao;
                    Self::func102(
                        env,
                        b.wrapping_add(8),
                        arg4,
                        arg3,
                        arg5,
                        arg6,
                    );
                    let ap = arg2;
                    let aq = mload64!(b.wrapping_add(16) as usize);
                    arg2 = mload64!(b.wrapping_add(8) as usize);
                    g = ap.wrapping_sub(aq).wrapping_sub(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64);
                    h = g.wrapping_shr(63 as u32);
                    a = (arg6 < 0) as i32;
                    i = h & ((if a != 0 { 0.wrapping_sub(arg5) } else { arg5 }));
                    arg1 = i.wrapping_add(arg1.wrapping_sub(arg2));
                    let ar = arg1;
                    arg1 = (((arg1 as u64) < i as u64) as i32 as u32 as i64).wrapping_add((h & ((if a != 0 { 0.wrapping_sub(arg6.wrapping_add((arg5 != 0) as i32 as u32 as i64)) } else { arg6 }))).wrapping_add(g));
                    arg1 = ((if (arg1 == 0) as i32 != 0 { (ar != 0) as i32 } else { (arg1 > 0) as i32 })) as u32 as i64;
                    arg6 = arg3.wrapping_sub(((arg4 as u64) < arg1 as u64) as i32 as u32 as i64);
                    if arg3 & (arg3 ^ arg6) < 0 {
                        unreachable!();
                    }
                    arg5 = arg4.wrapping_sub(arg1);
                    break 'label4;
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

    fn func55(
        env: &Env,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let c = Self::ledger_sequence_u32(env);
        a = c;
        'label0: {
            let d = env.ledger().max_live_until_ledger() as i64;
            b = (d as u64).wrapping_shr(32 as u32) as i64 as i32;
            if a as u32 <= b as u32 {
                a = b.wrapping_sub(a);
                if a as u32 >= 120960 as u32 {
                    break 'label0;
                }
            }
            unreachable!();
        }
        env.storage().instance().extend_ttl((a.wrapping_sub(120960) as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (a as u32 as i64).wrapping_shl(32 as u32) | 0 as u32);
    }

    fn ledger_sequence_u32(
        env: &Env,
    ) -> i32 {
        let a = env.ledger().sequence() as i64;
        (a as u64).wrapping_shr(32 as u32) as i64 as i32
    }




    fn func60(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut i: i64 = 0;
        a = -112;
        while b != 80 {
            mstore64!(a.wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
        }
        {
            Self::func67(
                env,
                arg1,
                1048784,
                10,
                a,
                10,
            );
            b = a.wrapping_add(80);
            Self::func74(env, b, mload64!(a as usize));
            let a_i32_80_6 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80_6 != 0 {
                unreachable!();
            }
            arg1 = mload64!(a.wrapping_add(8) as usize);
            let value_lo = mload64!(a.wrapping_add(16) as usize);
            Self::func68(env, b, value_lo);
            let a_i32_80_4 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80_4 != 0 {
                unreachable!();
            }
            e = mload64!(a.wrapping_add(24) as usize);
            f = mload64!(a.wrapping_add(32) as usize);
            Self::func68(env, b, mload64!(a.wrapping_add(40) as usize));
            let a_i32_80_5 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80_5 != 0 {
                unreachable!();
            }
            i = mload64!(a.wrapping_add(48) as usize);
            let loaded_val = mload64!(a.wrapping_add(56) as usize);
            Self::func68(env, b, loaded_val);
            let a_i32_80_2 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80_2 != 0 {
                unreachable!();
            }
            Self::func68(env, b, mload64!(a.wrapping_add(64) as usize));
            let a_i32_80_3 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80_3 != 0 {
                unreachable!();
            }
            Self::val_to_i64_checked(env, b, mload64!(a.wrapping_add(72) as usize));
            let a_i32_80 = mload32!(a.wrapping_add(80) as usize);
            if a_i32_80 == 0 {
                mstore32!(arg0.wrapping_add(108) as usize, (e as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(arg0.wrapping_add(104) as usize, (arg1 as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(arg0.wrapping_add(100) as usize, (i as u64).wrapping_shr(32 as u32) as i64 as u32);
                mstore32!(arg0.wrapping_add(96) as usize, (f as u64).wrapping_shr(32 as u32) as i64 as u32);
            }
        }
    }

    fn func61(
        env: &Env,
        arg0: i32,
    ) {
        env.storage().put_contract_data(1048656, arg0, 0);
    }




    fn func65(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let a: i32 = -16;
        env.storage().put_contract_data(a, arg1, 0);
    }


    fn func67(
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

    fn func68(
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
                    if a != 11 {
                        break 'label1;
                    }
                    let mut svarg0_24_i64 = arg1.wrapping_shr(63 as u32) as i64;
                    let mut svarg0_16_i64 = arg1 as i64;
                } else {
                    let d = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64);
                    let e = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64);
                    arg1 = e;
                    let svarg0_24_i64 = d as i64;
                    let svarg0_16_i64 = arg1 as i64;
                }
                break 'label0;
            }
            Error(Value, UnexpectedType) as i64;
        }
    }



    fn func71(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i64 {
        let a: i32 = -16;
        Self::func41(
            env,
            a,
            arg0,
            arg1,
        );
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        mload64!(a.wrapping_add(8) as usize)
    }

    fn func72(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -16;
        val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(arg1)), Vec::<Val>::from_val(env, &val_from_i64(arg2))));
    }


    fn func74(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        Bytes::from_val(env, &val_from_i64(arg1)).len() as i64;
    }

    fn func75(
        env: &Env,
        arg0: i64,
        arg1: i32,
    ) -> i64 {
        let a = Bytes::from_val(env, &val_from_i64(arg0)).len() as i64;
        let b = val_to_i64(Bytes::from_val(env, &val_from_i64(arg0)).slice((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32..a & 18446744069414584320 | 0 as u32).into_val(env));
        b
    }

    fn func76(
        env: &Env,
        arg0: i64,
        arg1: i32,
    ) {
        Self::copy_bytes_to_linear_memory(env, arg0, 0, (arg1 as u32 as i64).wrapping_shl(32 as u32) | 0, 32);
    }

    fn func77(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        a = mload64!(arg1 as usize);
        let d = Bytes::from_val(env, &val_from_i64(a)).len() as i64;
        b = d;
        let e: i32;
        if b as u64 >= 4294967296 as u64 {
            let f = Bytes::from_val(env, &val_from_i64(a)).get(0) as i64;
            Self::func75(env, a, 1);
            e = (f as u64).wrapping_shr(32 as u32) as i64 as i32;
        } else {
            e = 0;
        }
        mstore8!(arg0.wrapping_add(1) as usize, e as u8);
        mstore8!(arg0 as usize, (b as u64 > 4294967295 as u64) as i32 as u8);
    }









    fn func86(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> i32 {
        if arg2 != 1114112 {
            let a = { let _ = (mload32!(arg1.wrapping_add(16) as usize), arg0, arg2); unimplemented!("call_indirect type 2") };
            if a != 0 {
                return 1;
            }
        }
        if arg3 == 0 {
            return 0;
        }
        let b = { let _ = (mload32!(arg1.wrapping_add(12) as usize), arg0, arg3, 0); unimplemented!("call_indirect type 3") };
        b
    }




    fn memcpy_like(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        arg1 = arg1.wrapping_shl(2 as u32);
        mload32!(arg1.wrapping_add(1049568) as usize);
        mload32!(arg1.wrapping_add(1049608) as usize);
    }

    fn memcpy_like_2(
        env: &Env,
        arg0: i32,
        mut arg1: i32,
    ) {
        arg1 = arg1.wrapping_shl(2 as u32);
        mload32!(arg1.wrapping_add(1049648) as usize);
        mload32!(arg1.wrapping_add(1049688) as usize);
    }

    fn memcpy_like_3(
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
        a = -48;
        h = mload32!(arg2.wrapping_add(4) as usize);
        b = mload32!(arg2.wrapping_add(12) as usize);
        let mut svarg2_0_i32 = mload32!(arg2 as usize);
        d = svarg2_0_i32;
        let sv3_44_i32 = arg1 as i32;
        let sv3_40_i32 = arg0 as i32;
        mstore8!(a.wrapping_add(36) as usize, 3 as u8);
        let mut sv3_20_i32 = 0 as i32;
        let mut sv3_12_i32 = 0 as i32;
        let m: i32;
        'label0: {
            'label1: {
                {
                    i = mload32!(arg2.wrapping_add(16) as usize);
                    if i == 0 {
                        if b == 0 {
                            unreachable!();
                        }
                        arg2 = mload32!(arg2.wrapping_add(8) as usize);
                        arg0 = arg2.wrapping_add(b.wrapping_shl(3 as u32));
                        e = (b -= 1 & 536870911) += 1;
                        arg1 = d;
                        loop {
                            let n = mload32!(arg1.wrapping_add(4) as usize);
                            b = n;
                            if b != 0 {
                                let o = { let _ = (mload32!(sv3_44_i32.wrapping_add(12) as usize), sv3_40_i32, mload32!(arg1 as usize), b); unimplemented!("call_indirect type 3") };
                                if o != 0 {
                                    unreachable!();
                                }
                            }
                            let mut svarg2_0_i32 = mload32!(arg2 as usize);
                            let p = mload32!(arg2.wrapping_add(4) as usize);
                            let q = { let _ = (p, svarg2_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 2") };
                            if q != 0 {
                                unreachable!();
                            }
                            arg1 = arg1.wrapping_add(8);
                            arg2 = arg2.wrapping_add(8);
                        }
                    } else {
                        arg0 = mload32!(arg2.wrapping_add(20) as usize);
                        if arg0 == 0 {
                            unreachable!();
                        }
                        j = arg0.wrapping_shl(5 as u32);
                        e = (arg0 -= 1 & 134217727) += 1;
                        f = mload32!(arg2.wrapping_add(8) as usize);
                        arg0 = 0;
                        arg1 = d;
                        loop {
                            let r = mload32!(arg1.wrapping_add(4) as usize);
                            arg2 = r;
                            if arg2 != 0 {
                                let s = { let _ = (mload32!(mload32!(a.wrapping_add(44) as usize).wrapping_add(12) as usize), mload32!(a.wrapping_add(40) as usize), mload32!(arg1 as usize), arg2); unimplemented!("call_indirect type 3") };
                                if s != 0 {
                                    unreachable!();
                                }
                            }
                            arg2 = arg0.wrapping_add(i);
                            mload32!(arg2.wrapping_add(16) as usize);
                            let u = mload8!(arg2.wrapping_add(28) as usize) as i32;
                            mstore8!(a.wrapping_add(36) as usize, u as u8);
                            mload32!(arg2.wrapping_add(24) as usize);
                            let w = mload32!(arg2.wrapping_add(12) as usize);
                            b = w;
                            g = 0;
                            c = 0;
                            {
                                let x = mload32!(arg2.wrapping_add(8) as usize);
                            }
                            k = b.wrapping_shl(3 as u32).wrapping_add(f);
                            if mload32!(k as usize) != 0 {
                                unreachable!();
                            }
                            b = mload32!(k.wrapping_add(4) as usize);
                            c = 1;
                            let sv3_12_i32 = c as i32;
                            let y = mload32!(arg2.wrapping_add(4) as usize);
                            b = y;
                            {
                                let svarg2_0_i32 = mload32!(arg2 as usize);
                            }
                            c = b.wrapping_shl(3 as u32).wrapping_add(f);
                            if mload32!(c as usize) != 0 {
                                unreachable!();
                            }
                            b = mload32!(c.wrapping_add(4) as usize);
                            g = 1;
                            let sv3_20_i32 = g as i32;
                            let z = mload32!(arg2.wrapping_add(20) as usize);
                            arg2 = f.wrapping_add(z.wrapping_shl(3 as u32));
                            let aa = mload32!(arg2.wrapping_add(4) as usize);
                            let ab = { let _ = (aa, svarg2_0_i32, a.wrapping_add(12)); unimplemented!("call_indirect type 2") };
                            if ab != 0 {
                                unreachable!();
                            }
                            arg1 = arg1.wrapping_add(8);
                            arg0 = arg0.wrapping_add(32);
                        }
                    }
                    if e as u32 >= h as u32 {
                        break 'label1;
                    }
                    arg0 = d.wrapping_add(e.wrapping_shl(3 as u32));
                    let ac = { let _ = (mload32!(mload32!(a.wrapping_add(44) as usize).wrapping_add(12) as usize), mload32!(a.wrapping_add(40) as usize), mload32!(arg0 as usize), mload32!(arg0.wrapping_add(4) as usize)); unimplemented!("call_indirect type 3") };
                    if ac == 0 {
                        unreachable!();
                    }
                }
                m = 1;
                break 'label0;
            }
            m = 0;
        }
        m
    }

    fn func93(
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

    fn func94(
        env: &Env,
        arg0: i64,
    ) -> i32 {
        let a = Self::func97(env, arg0);
        (a & 255 == 255) as i32
    }

    fn func95(
        env: &Env,
        arg0: i64,
    ) -> i32 {
        let a = Self::func97(env, arg0);
        (a & 255 == 1) as i32
    }

    fn func96(
        env: &Env,
        mut arg0: i32,
        arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        a = -48;
        let f = arg0;
        let g = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64;
        let h: i32;
        if g & 18446744069414584320 == 68719476736 {
            mstore64!(a.wrapping_add(24) as usize, 0 as u64);
            arg0 += 1;
            loop {
                Self::func77(env, a.wrapping_add(8), a.wrapping_add(32));
                let i = mload8!(a.wrapping_add(8) as usize) as i32;
                if i == 0 {
                    unreachable!();
                }
                b = mload32!(a.wrapping_add(40) as usize);
                c = b += 1;
                if c == 0 {
                    unreachable!();
                }
                let j = mload8!(a.wrapping_add(9) as usize) as i32;
                if b as u32 <= 15 as u32 {
                    mstore8!(a.wrapping_add(16).wrapping_add(b) as usize, j as u8);
                }
            }
            unreachable!();
            mload64!(a.wrapping_add(16) as usize);
            let k = mload64!(a.wrapping_add(24) as usize);
            mstore64!(arg0.wrapping_add(8) as usize, k as u64);
            h = 0;
        } else {
            h = 1;
        }
        mstore8!(f as usize, h as u8);
    }

    fn func97(
        env: &Env,
        mut arg0: i64,
    ) -> i32 {
        let mut c: i32 = 0;
        let e: i32;
        if arg0 & 255 != 0 {
            let f = { let a = val_from_i64(arg0); let b = val_from_i64(0); if a < b { -1 } else if a > b { 1 } else { 0 } };
            arg0 = f;
            c = (arg0 != 0) as i32;
            e = (arg0 as u64).wrapping_shr(63 as u32) as i64 as i32;
        } else {
            arg0 = arg0;
            c = (arg0 != 0) as i32;
            e = (arg0 < 0) as i32;
        }
        (if e != 0 { -1 } else { c })
    }



    fn func100(
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
        k = (arg4 < 0) as i32;
        if k != 0 {
            b = 0.wrapping_sub(arg3);
        } else {
            b = arg3;
        }
        h = -176;
        'label0: {
            'label1: {
                'label2: {
                    'label3: {
                        if k != 0 {
                            c = 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64));
                        } else {
                            c = arg4;
                        }
                        k = ((if (c != 0) as i32 != 0 { c.leading_zeros() as i64 } else { (b.leading_zeros() as i64).wrapping_sub(18446744073709551552) })) as i32;
                        if i != 0 {
                            arg3 = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
                        } else {
                            arg3 = arg2;
                        }
                        i = ((if (arg3 != 0) as i32 != 0 { arg3.leading_zeros() as i64 } else { (a.leading_zeros() as i64).wrapping_sub(18446744073709551552) })) as i32;
                        if k as u32 > i as u32 {
                            if i as u32 > 63 as u32 {
                                break 'label3;
                            }
                            if k as u32 > 95 as u32 {
                                break 'label2;
                            }
                            if (k.wrapping_sub(i) as u32) < 32 as u32 {
                                break 'label1;
                            }
                            l = 96.wrapping_sub(k);
                            Self::func101(
                                env,
                                h.wrapping_add(160),
                                b,
                                c,
                                l,
                            );
                            let o = mload32!(h.wrapping_add(160) as usize) as i64;
                            e = o += 1;
                            'label5: {
                                {
                                    'label7: {
                                        loop {
                                            i = 64.wrapping_sub(i);
                                            Self::func101(
                                                env,
                                                h.wrapping_add(144),
                                                a,
                                                arg3,
                                                i,
                                            );
                                            arg1 = mload64!(h.wrapping_add(144) as usize);
                                            if (i as u32) < l as u32 {
                                                Self::func101(
                                                    env,
                                                    h.wrapping_add(80),
                                                    b,
                                                    c,
                                                    i,
                                                );
                                                let loaded_val = mload64!(h.wrapping_add(80) as usize);
                                                e = loaded_val;
                                                if e != 0 {
                                                    arg1 = (arg1 as u64 / e as u64) as i64;
                                                }
                                                Self::func102(
                                                    env,
                                                    h.wrapping_sub(-64),
                                                    arg1,
                                                    0,
                                                    b,
                                                    c,
                                                );
                                                e = mload64!(h.wrapping_add(64) as usize);
                                                i = ((a as u64) < e as u64) as i32;
                                                let p = mload64!(h.wrapping_add(72) as usize);
                                                g = p;
                                                let cond_val = if arg3 == g { i } else { ((arg3 as u64) < g as u64) as i32 };
                                                if cond_val == 0 {
                                                    arg3 = arg3.wrapping_sub(g).wrapping_sub(i as u32 as i64);
                                                    a = a.wrapping_sub(e);
                                                    arg1 = arg1.wrapping_add(d);
                                                    f = f.wrapping_add(((arg1 as u64) < d as u64) as i32 as u32 as i64);
                                                    break 'label0;
                                                }
                                                b = a.wrapping_add(b);
                                                arg3 = ((a as u64 > b as u64) as i32 as u32 as i64).wrapping_add(arg3.wrapping_add(c)).wrapping_sub(g).wrapping_sub(((b as u64) < e as u64) as i32 as u32 as i64);
                                                a = b.wrapping_sub(e);
                                                arg1 = arg1.wrapping_add(d) -= 1;
                                                f = f.wrapping_add(((arg1 as u64) < d as u64) as i32 as u32 as i64);
                                                break 'label0;
                                            }
                                            arg1 = (arg1 as u64 / e as u64) as i64;
                                            i = i.wrapping_sub(l) & 127;
                                            Self::func103(
                                                env,
                                                h.wrapping_add(128),
                                                arg1,
                                                0,
                                                i,
                                            );
                                            Self::func102(
                                                env,
                                                h.wrapping_add(112),
                                                arg1,
                                                0,
                                                b,
                                                c,
                                            );
                                            let q = mload64!(h.wrapping_add(120) as usize);
                                            Self::func103(
                                                env,
                                                h.wrapping_add(96),
                                                mload64!(h.wrapping_add(112) as usize),
                                                q,
                                                i,
                                            );
                                            arg1 = mload64!(h.wrapping_add(128) as usize);
                                            d = arg1.wrapping_add(d);
                                            let r = mload64!(h.wrapping_add(136) as usize);
                                            f = (((d as u64) < arg1 as u64) as i32 as u32 as i64).wrapping_add(r.wrapping_add(f));
                                            let s = mload64!(h.wrapping_add(104) as usize);
                                            arg1 = mload64!(h.wrapping_add(96) as usize);
                                            arg3 = arg3.wrapping_sub(s).wrapping_sub(((a as u64) < arg1 as u64) as i32 as u32 as i64);
                                            a = a.wrapping_sub(arg1);
                                            i = ((if (arg3 != 0) as i32 != 0 { arg3.leading_zeros() as i64 } else { (a.leading_zeros() as i64).wrapping_sub(18446744073709551552) })) as i32;
                                            if k as u32 <= i as u32 {
                                                break 'label7;
                                            }
                                        }
                                        if b != 0 {
                                            unreachable!();
                                        }
                                        break 'label5;
                                    }
                                    i = ((a as u64) < b as u64) as i32;
                                    let cond_val_2 = if arg3 == c { i } else { ((arg3 as u64) < c as u64) as i32 };
                                    if cond_val_2 == 0 {
                                        unreachable!();
                                    }
                                    arg1 = d;
                                    break 'label0;
                                }
                                arg3 = (a as u64 / b as u64) as i64;
                            }
                            a = (a as u64).wrapping_rem(b as u64) as i64;
                            arg1 = arg3.wrapping_add(d);
                            f = f.wrapping_add(((arg1 as u64) < d as u64) as i32 as u32 as i64);
                            arg3 = 0;
                            break 'label0;
                            arg3 = arg3.wrapping_sub(c).wrapping_sub(i as u32 as i64);
                            a = a.wrapping_sub(b);
                            arg1 = d += 1;
                            f = f.wrapping_add((arg1 == 0) as i32 as u32 as i64);
                            break 'label0;
                        }
                        if (arg3 == c) as i32 != 0 {
                            i = (a as u64 >= b as u64) as i32;
                        } else {
                            i = (arg3 as u64 >= c as u64) as i32;
                        }
                        if i != 0 {
                            arg1 = b;
                        } else {
                            arg1 = 0;
                        }
                        arg3 = arg3.wrapping_sub((if i != 0 { c } else { 0 })).wrapping_sub(((a as u64) < arg1 as u64) as i32 as u32 as i64);
                        a = a.wrapping_sub(arg1);
                        arg1 = i as u32 as i64;
                        break 'label0;
                    }
                    arg1 = (a as u64 / b as u64) as i64;
                    a = a.wrapping_sub(arg1.wrapping_mul(b));
                    arg3 = 0;
                    break 'label0;
                }
                arg1 = (a as u64).wrapping_shr(32 as u32) as i64;
                let t = arg3;
                let u = arg3;
                arg3 = b & 4294967295;
                c = (u as u64 / arg3 as u64) as i64;
                d = ((arg1 | t.wrapping_sub(c.wrapping_mul(b)).wrapping_shl(32 as u32)) as u64 / arg3 as u64) as i64;
                a = a & 4294967295 | arg1.wrapping_sub(b.wrapping_mul(d)).wrapping_shl(32 as u32);
                b = (a as u64 / arg3 as u64) as i64;
                arg1 = d.wrapping_shl(32 as u32) | b;
                a = a.wrapping_sub(arg3.wrapping_mul(b));
                f = (d as u64).wrapping_shr(32 as u32) as i64 | c;
                arg3 = 0;
                break 'label0;
            }
            i = 64.wrapping_sub(i);
            Self::func101(
                env,
                h.wrapping_add(48),
                b,
                c,
                i,
            );
            Self::func101(
                env,
                h.wrapping_add(32),
                a,
                arg3,
                i,
            );
            let h_part_32 = mload64!(h.wrapping_add(32) as usize);
            let h_part_48 = mload64!(h.wrapping_add(48) as usize);
            arg1 = (h_part_32 as u64 / h_part_48 as u64) as i64;
            Self::func102(
                env,
                h.wrapping_add(16),
                b,
                0,
                arg1,
                0,
            );
            Self::func102(
                env,
                h,
                c,
                0,
                arg1,
                0,
            );
            d = mload64!(h.wrapping_add(16) as usize);
            {
                let v = mload64!(h.wrapping_add(8) as usize);
                let w = mload64!(h.wrapping_add(24) as usize);
                g = w;
                let h_lo = mload64!(h as usize);
                e = g.wrapping_add(h_lo);
                if v.wrapping_add(((e as u64) < g as u64) as i32 as u32 as i64) == 0 {
                    i = ((a as u64) < d as u64) as i32;
                    let cond_val_3 = if arg3 == e { i } else { ((arg3 as u64) < e as u64) as i32 };
                    if cond_val_3 == 0 {
                        unreachable!();
                    }
                }
                a = a.wrapping_add(b);
                arg3 = (((a as u64) < b as u64) as i32 as u32 as i64).wrapping_add(arg3.wrapping_add(c)).wrapping_sub(e).wrapping_sub(((a as u64) < d as u64) as i32 as u32 as i64);
                arg1 -= 1;
                a = a.wrapping_sub(d);
                break 'label0;
            }
            arg3 = arg3.wrapping_sub(e).wrapping_sub(i as u32 as i64);
            a = a.wrapping_sub(d);
        }
        h = (arg2 ^ arg4 < 0) as i32;
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
            arg1 = arg2.wrapping_shl((0.wrapping_sub(arg3) & 63) as u32 as i64 as u32) | (arg1 as u64).wrapping_shr(a as u32) as i64;
            arg2 = (arg2 as u64).wrapping_shr(a as u32) as i64;
        } else {
            arg1 = (arg2 as u64).wrapping_shr((arg3 & 63) as u32 as i64 as u32) as i64;
        }
    }

    fn func102(
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

    fn func103(
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

    fn memcpy_like_4(
        env: &Env,
        mut arg0: i32,
        mut arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        let mut g: i32 = 0;
        let mut h: i32 = 0;
        b = 0.wrapping_sub(arg0) & 3;
        c = arg0.wrapping_add(b);
        if c as u32 > arg0 as u32 {
            a = arg0;
            arg0 = arg1;
            if b != 0 {
                d = b;
                loop {
                    let i = mload8!(arg0 as usize) as i32;
                    mstore8!(a as usize, i as u8);
                    arg0 += 1;
                    a += 1;
                    d -= 1;
                }
            }
            if (b.wrapping_sub(1) as u32) >= 7 as u32 {
                loop {
                    let j = mload8!(arg0 as usize) as i32;
                    mstore8!(a as usize, j as u8);
                    let k = mload8!(arg0 += 1 as usize) as i32;
                    mstore8!(a.wrapping_add(1) as usize, k as u8);
                    let l = mload8!(arg0.wrapping_add(2) as usize) as i32;
                    mstore8!(a.wrapping_add(2) as usize, l as u8);
                    let m = mload8!(arg0.wrapping_add(3) as usize) as i32;
                    mstore8!(a.wrapping_add(3) as usize, m as u8);
                    let n = mload8!(arg0.wrapping_add(4) as usize) as i32;
                    mstore8!(a.wrapping_add(4) as usize, n as u8);
                    let o = mload8!(arg0.wrapping_add(5) as usize) as i32;
                    mstore8!(a.wrapping_add(5) as usize, o as u8);
                    let p = mload8!(arg0.wrapping_add(6) as usize) as i32;
                    mstore8!(a.wrapping_add(6) as usize, p as u8);
                    let q = mload8!(arg0.wrapping_add(7) as usize) as i32;
                    mstore8!(a.wrapping_add(7) as usize, q as u8);
                    arg0 = arg0.wrapping_add(8);
                    a = a.wrapping_add(8);
                }
            }
        }
        d = 96.wrapping_sub(b);
        e = d & -4;
        a = c.wrapping_add(e);
        'label3: {
            arg0 = arg1.wrapping_add(b);
            if arg0 & 3 == 0 {
                if a as u32 <= c as u32 {
                    break 'label3;
                }
                arg1 = arg0;
                loop {
                    let svarg1_0_i32 = mload32!(arg1 as usize);
                    let mut sv4_0_i32 = svarg1_0_i32 as i32;
                    arg1 = arg1.wrapping_add(4);
                    c = c.wrapping_add(4);
                }
                break 'label3;
            }
            if a as u32 <= c as u32 {
                break 'label3;
            }
            b = arg0.wrapping_shl(3 as u32);
            f = b & 24;
            g = arg0 & -4;
            arg1 = g.wrapping_add(4);
            h = 0.wrapping_sub(b) & 24;
            b = mload32!(g as usize);
            loop {
                let r = b;
                b = svarg1_0_i32;
                sv4_0_i32 = ((r as u32).wrapping_shr(f as u32) as i32 | b.wrapping_shl(h as u32)) as i32;
                arg1 = arg1.wrapping_add(4);
                c = c.wrapping_add(4);
            }
        }
        arg1 = arg0.wrapping_add(e);
        b = d & 3;
        d = b.wrapping_add(a);
        if a as u32 < d as u32 {
            arg0 = b;
            if arg0 != 0 {
                loop {
                    let s = mload8!(arg1 as usize) as i32;
                    mstore8!(a as usize, s as u8);
                    arg1 += 1;
                    a += 1;
                    arg0 -= 1;
                }
            }
            if (b.wrapping_sub(1) as u32) >= 7 as u32 {
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
    }

}