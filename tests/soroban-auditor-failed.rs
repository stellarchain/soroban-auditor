#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, Val, FromVal, Map, Bytes, String, Symbol};

#[contract]
pub struct SorobanAuditorFailed;

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
pub enum Error { NotInitialized = 1, AlreadyInitialized = 2, Unauthorized = 3, ContractLocked = 4, LowShareCount = 5, InvalidShareTotal = 6, ZeroTransferAmount = 7, TransferAmountAboveBalance = 8, TransferAmountAboveUnusedBalance = 9, ZeroWithdrawalAmount = 10, WithdrawalAmountAboveAllocation = 11, NoSharesToSell = 12, NoActiveListing = 13, InsufficientSharesInListing = 14, InvalidPrice = 15, InvalidShareAmount = 16, CannotBuyOwnShares = 17, NoSharesToTransfer = 18, InsufficientSharesToTransfer = 19, CannotTransferToSelf = 20, Overflow = 21, NegativeShareAmount = 22, DuplicateShareholder = 23, InvalidCommissionRate = 24, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Config, Shareholders, Share(Address), TotalAllocation(Address), Allocation(Address, Address), SaleListing(Address), ActiveListings, Commission, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ShareDataKey { pub share: i128, pub shareholder: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigDataKey { pub admin: Address, pub mutable: bool, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CommissionConfig { pub buy_rate_bps: i128, pub distribution_rate_bps: i128, pub recipient: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SaleListingDataKey { pub payment_token: Address, pub price_per_share: i128, pub seller: Address, pub shares_for_sale: i128, }

#[contractimpl]
impl SorobanAuditorFailed {

    pub fn buy_shares(
        env: Env,
        mut buyer: Address,
        seller: Address,
        mut shares_amount: i128,
    ) -> Result<(), Error> {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i64 = 0;
        let mut m: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i32 = 0;
        amount_val = -272;
        'label0: {
            'label1: {
                'label3: {
                    'label4: {
                        {
                            Self::decode_i128_parts(env, amount_val.wrapping_add(80), shares_amount);
                            let amount_val_i32_80_2 = mload32!(amount_val.wrapping_add(80) as usize);
                            if amount_val_i32_80_2 != 0 {
                                unreachable!();
                            }
                            a = mload64!(amount_val.wrapping_add(96) as usize);
                            shares_amount = mload64!(amount_val.wrapping_add(104) as usize);
                            buyer.require_auth();
                            {
                                let cond_val = if shares_amount != 0 { (a == 0) as i32 } else { (shares_amount < 0) as i32 };
                                if cond_val == 0 {
                                    buyer = 68719476739 /* Error(Contract, #16) */;
                                    break 'label0;
                                }
                            }
                            {
                                let r = Self::func50(env, buyer, seller);
                                if r != 0 {
                                    buyer = 73014444035 /* Error(Contract, #17) */;
                                    break 'label0;
                                }
                            }
                            env.storage().get_contract_data(amount_val.wrapping_add(80), seller);
                            if mload32!(amount_val.wrapping_add(80) as usize) & 1 == 0 {
                                buyer = 55834574851 /* Error(Contract, #13) */;
                                break 'label0;
                            }
                            {
                                b = mload64!(amount_val.wrapping_add(96) as usize);
                                c = ((b as u64) < a as u64) as i32;
                                d = mload64!(amount_val.wrapping_add(104) as usize);
                                let cond_val_2 = if d != shares_amount { c } else { (d < shares_amount) as i32 };
                                if cond_val_2 == 0 {
                                    buyer = 60129542147 /* Error(Contract, #14) */;
                                    break 'label0;
                                }
                            }
                            e = mload64!(amount_val.wrapping_add(120) as usize);
                            f = mload64!(amount_val.wrapping_add(112) as usize);
                            g = mload64!(amount_val.wrapping_add(136) as usize);
                            Self::func113(
                                env,
                                amount_val.wrapping_add(48),
                                a,
                                shares_amount,
                                f,
                                e,
                                amount_val.wrapping_add(76),
                            );
                            {
                                let amount_val_i32_76 = mload32!(amount_val.wrapping_add(76) as usize);
                                if amount_val_i32_76 != 0 {
                                    buyer = 90194313219 /* Error(Contract, #21) */;
                                    break 'label0;
                                }
                            }
                            let loaded_val = mload64!(amount_val.wrapping_add(56) as usize);
                            h = loaded_val;
                            i = mload64!(amount_val.wrapping_add(48) as usize);
                            env.storage().get_contract_data(amount_val.wrapping_add(144));
                            Self::func113(
                                env,
                                amount_val.wrapping_add(16),
                                i,
                                h,
                                mload64!(amount_val.wrapping_add(144) as usize),
                                mload64!(amount_val.wrapping_add(152) as usize),
                                amount_val.wrapping_add(44),
                            );
                            let amount_val_i32_44 = mload32!(amount_val.wrapping_add(44) as usize);
                            if amount_val_i32_44 != 0 {
                                unreachable!();
                            }
                            let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
                            let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
                            Self::func111(
                                env,
                                amount_val,
                                amount_lo,
                                amount_hi,
                                10000,
                                0,
                            );
                            l = mload64!(amount_val.wrapping_add(8) as usize);
                            m = mload64!(amount_val as usize);
                            n = h.wrapping_sub(l).wrapping_sub(((i as u64) < m as u64) as i32 as u32 as i64);
                            if (h ^ l) & (h ^ n) < 0 {
                                break 'label3;
                            }
                            o = i.wrapping_sub(m);
                            let cond_val_3 = if n == 0 { (o != 0) as i32 } else { (n > 0) as i32 };
                            if cond_val_3 != 0 {
                                unreachable!();
                            }
                            break 'label4;
                        }
                        unreachable!();
                        Self::func30(
                            env,
                            g,
                            buyer,
                            seller,
                            o,
                            n,
                        );
                    }
                    {
                        let cond_val_4 = if amount_hi != 0 { (amount_lo as u64 > 9999 as u64) as i32 } else { (amount_hi > 0) as i32 };
                        if cond_val_4 == 0 {
                            Self::func30(
                                env,
                                g,
                                buyer,
                                mload64!(amount_val.wrapping_add(176) as usize),
                                m,
                                l,
                            );
                        }
                    }
                    env.storage().get_contract_data(amount_val.wrapping_add(80), seller);
                    if mload32!(amount_val.wrapping_add(80) as usize) & 1 == 0 {
                        buyer = 51539607555 /* Error(Contract, #12) */;
                        break 'label0;
                    }
                    k = mload64!(amount_val.wrapping_add(104) as usize);
                    l = mload64!(amount_val.wrapping_add(96) as usize);
                    n = k.wrapping_sub(shares_amount).wrapping_sub(((l as u64) < a as u64) as i32 as u32 as i64);
                    if (k ^ shares_amount) & (k ^ n) < 0 {
                        break 'label3;
                    }
                    'label14: {
                        {
                            k = l.wrapping_sub(a);
                            let cond_val_5 = if n == 0 { (k == 0) as i32 } else { (n > 0) as i32 };
                            if cond_val_5 != 0 {
                                Self::func56(env, seller);
                                let s = Self::func58(env);
                                n = s;
                                let t = Vec::<Val>::from_val(env, &val_from_i64(n)).len() as i64;
                                k = t;
                                mstore32!(amount_val.wrapping_add(204) as usize, (k as u64).wrapping_shr(32 as u32) as i64 as u32);
                                loop {
                                    Self::func26(env, amount_val.wrapping_add(80), amount_val.wrapping_add(192));
                                    let amount_val_i32_80 = mload32!(amount_val.wrapping_add(80) as usize);
                                    if amount_val_i32_80 != 0 {
                                        unreachable!();
                                    }
                                    break 'label14;
                                    p = mload32!(amount_val.wrapping_add(88) as usize);
                                    let mut sv3_96_i64 = mload64!(amount_val.wrapping_add(96) as usize);
                                    let u = Self::func50(env, sv3_96_i64, seller);
                                }
                                {
                                    let w = Vec::<Val>::from_val(env, &val_from_i64(n)).len() as i64;
                                    if p as u32 < (w as u64).wrapping_shr(32 as u32) as i64 as i32 as u32 {
                                        let x = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(n)); v.remove_unchecked((p as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); val_to_i64(v.into_val(env)) };
                                        n = x;
                                    }
                                }
                                Self::func53(env, n);
                                break 'label14;
                            }
                        }
                        env.storage().put_contract_data(seller, k, n);
                    }
                    env.storage().get_contract_data(amount_val.wrapping_add(192), buyer);
                    if mload32!(amount_val.wrapping_add(192) as usize) & 1 == 0 {
                        unreachable!();
                    }
                    l = mload64!(amount_val.wrapping_add(216) as usize);
                    k = mload64!(amount_val.wrapping_add(208) as usize);
                    n = k.wrapping_add(a);
                    k = l.wrapping_add(shares_amount).wrapping_add(((n as u64) < k as u64) as i32 as u32 as i64);
                    if (l ^ shares_amount ^ 18446744073709551615) & (l ^ k) >= 0 {
                        break 'label1;
                    }
                }
                Self::call_unreachable(env);
                unreachable!();
                let vec_builder = Self::func58(env);
                vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(buyer)); val_to_i64(v.into_val(env)) };
                Self::func53(env, y);
                n = a;
                k = shares_amount;
            }
            env.storage().put_contract_data(buyer, n, k);
            'label18: {
                {
                    n = b.wrapping_sub(a);
                    d = d.wrapping_sub(shares_amount).wrapping_sub(c as u32 as i64);
                    let cond_val_6 = if d == 0 { (n == 0) as i32 } else { (d > 0) as i32 };
                    if cond_val_6 != 0 {
                        Self::func77(env, seller);
                        break 'label18;
                    }
                }
                env.storage().put_contract_data(seller, n, d, f, e, g);
            }
            let sv3_96_i64 = i as i64;
            let z = Self::func85(env, amount_val.wrapping_add(248));
            let aa = Self::func81(env, amount_val.wrapping_add(80));
            env.events().publish(val_from_i64(z), val_from_i64(aa));
            buyer = 0;
        }
        buyer
    }

    pub fn cancel_listing(
        env: Env,
        seller: Address,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -64;
        seller.require_auth();
        env.storage().get_contract_data(a, seller);
        b = 55834574851 /* Error(Contract, #13) */;
        if mload32!(a as usize) & 1 != 0 {
            Self::func77(env, seller);
            let d = Self::func84(env, canceled, seller);
            env.events().publish(val_from_i64(d), val_from_i64(1));
            b = 0;
        }
        return b;
    }

    pub fn distribute_tokens(
        env: Env,
        token_address: Address,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i32 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut k: i64 = 0;
        let mut l: i32 = 0;
        let mut m: i64 = 0;
        let mut n: i64 = 0;
        let mut o: i64 = 0;
        let mut p: i64 = 0;
        let mut q: i64 = 0;
        a = -256;
        b = 4294967299 /* Error(Contract, #1) */;
        'label1: {
            {
                let s = Self::func63(env);
                if s != 0 {
                    Self::require_owner_auth(env);
                    let t = val_to_i64(env.current_contract_address().into_val(env));
                    Self::func89(
                        env,
                        a.wrapping_add(176),
                        token_address,
                        t,
                    );
                    c = mload64!(a.wrapping_add(176) as usize);
                    d = mload64!(a.wrapping_add(184) as usize);
                    Self::func70(env, a.wrapping_add(176), token_address);
                    e = mload32!(a.wrapping_add(176) as usize) & 1;
                    if e != 0 {
                        f = mload64!(a.wrapping_add(200) as usize);
                    } else {
                        f = 0;
                    }
                    let u = f;
                    if e != 0 {
                        g = mload64!(a.wrapping_add(192) as usize);
                    } else {
                        g = 0;
                    }
                    f = d.wrapping_sub(f).wrapping_sub(((c as u64) < g as u64) as i32 as u32 as i64);
                    if (d ^ u) & (d ^ f) < 0 {
                        break 'label1;
                    }
                    b = 0;
                    d = c.wrapping_sub(g);
                    let cond_val = if f == 0 { (d == 0) as i32 } else { (f < 0) as i32 };
                    if cond_val == 0 {
                        env.storage().get_contract_data(a.wrapping_add(96));
                        Self::func113(
                            env,
                            a.wrapping_add(64),
                            d,
                            f,
                            mload64!(a.wrapping_add(112) as usize),
                            mload64!(a.wrapping_add(120) as usize),
                            a.wrapping_add(92),
                        );
                        let a_i32_92 = mload32!(a.wrapping_add(92) as usize);
                        if a_i32_92 != 0 {
                            unreachable!();
                        }
                        h = mload64!(a.wrapping_add(64) as usize);
                        let loaded_val = mload64!(a.wrapping_add(72) as usize);
                        i = loaded_val;
                        Self::func111(
                            env,
                            a.wrapping_add(48),
                            h,
                            i,
                            10000,
                            0,
                        );
                        c = mload64!(a.wrapping_add(56) as usize);
                        g = mload64!(a.wrapping_add(48) as usize);
                        {
                            let cond_val_2 = if i != 0 { (h as u64 > 9999 as u64) as i32 } else { (i > 0) as i32 };
                            if cond_val_2 == 0 {
                                let v = val_to_i64(env.current_contract_address().into_val(env));
                                i = mload64!(a.wrapping_add(128) as usize);
                                Self::func30(
                                    env,
                                    token_address,
                                    v,
                                    i,
                                    g,
                                    c,
                                );
                                let w = Self::func84(env, dist_com, token_address);
                                let x = Self::func83(
                                    env,
                                    i,
                                    g,
                                    c,
                                );
                                env.events().publish(val_from_i64(w), val_from_i64(x));
                            }
                        }
                        j = f.wrapping_sub(c).wrapping_sub(((d as u64) < g as u64) as i32 as u32 as i64);
                        if (f ^ c) & (f ^ j) < 0 {
                            break 'label1;
                        }
                        k = d.wrapping_sub(g);
                        let cond_val_3 = if j == 0 { (k == 0) as i32 } else { (j < 0) as i32 };
                        if cond_val_3 == 0 {
                            let z = Self::func58(env);
                            f = z;
                            let aa = Vec::<Val>::from_val(env, &val_from_i64(f)).len() as i64;
                            d = aa;
                            mstore32!(a.wrapping_add(156) as usize, (d as u64).wrapping_shr(32 as u32) as i64 as u32);
                            l = 0;
                            g = 0;
                            d = 0;
                            m = 0;
                            h = 0;
                            'label4: {
                                loop {
                                    Self::func27(env, a.wrapping_add(176), a.wrapping_add(144));
                                    Self::func28(
                                        env,
                                        a.wrapping_add(160),
                                        mload64!(a.wrapping_add(176) as usize),
                                        mload64!(a.wrapping_add(184) as usize),
                                    );
                                    let a_i32_160 = mload32!(a.wrapping_add(160) as usize);
                                    if a_i32_160 != 0 {
                                        unreachable!();
                                    }
                                    break 'label4;
                                    f = mload64!(a.wrapping_add(168) as usize);
                                    env.storage().get_contract_data(a.wrapping_add(176), f);
                                    i = mload64!(a.wrapping_add(192) as usize);
                                    c = mload64!(a.wrapping_add(200) as usize);
                                    Self::func113(
                                        env,
                                        a.wrapping_add(16),
                                        k,
                                        j,
                                        i,
                                        c,
                                        a.wrapping_add(44),
                                    );
                                    let a_i32_44 = mload32!(a.wrapping_add(44) as usize);
                                    if a_i32_44 != 0 {
                                        unreachable!();
                                    }
                                    if (c == d) as i32 != 0 {
                                        e = (i as u64 > g as u64) as i32;
                                    } else {
                                        e = (c > d) as i32;
                                    }
                                    if e != 0 {
                                        d = c;
                                    } else {
                                        d = d;
                                    }
                                    if e != 0 {
                                        g = i;
                                    } else {
                                        g = g;
                                    }
                                    if e != 0 {
                                        n = f;
                                    } else {
                                        n = n;
                                    }
                                    l = e | l;
                                    let a_lo = mload64!(a.wrapping_add(16) as usize);
                                    let a_hi = mload64!(a.wrapping_add(24) as usize);
                                    Self::func111(
                                        env,
                                        a,
                                        a_lo,
                                        a_hi,
                                        10000,
                                        0,
                                    );
                                    let cond_val_4 = if a_hi == 0 { (a_lo as u64 > 9999 as u64) as i32 } else { (a_hi > 0) as i32 };
                                    c = mload64!(a.wrapping_add(8) as usize);
                                    i = mload64!(a as usize);
                                    Self::func68(
                                        env,
                                        a.wrapping_add(224),
                                        f,
                                        token_address,
                                    );
                                    e = mload32!(a.wrapping_add(224) as usize) & 1;
                                    let a_part_248 = mload64!(a.wrapping_add(248) as usize);
                                    if e != 0 {
                                        o = a_part_248;
                                    } else {
                                        o = 0;
                                    }
                                    let a_part_240 = mload64!(a.wrapping_add(240) as usize);
                                    if e != 0 {
                                        p = a_part_240;
                                    } else {
                                        p = 0;
                                    }
                                    q = i.wrapping_add(p);
                                    p = o.wrapping_add(c).wrapping_add(((q as u64) < p as u64) as i32 as u32 as i64);
                                    if (c ^ o ^ 18446744073709551615) & (o ^ p) < 0 {
                                        break 'label1;
                                    }
                                    Self::func69(
                                        env,
                                        f,
                                        token_address,
                                        q,
                                        p,
                                    );
                                    o = m.wrapping_add(i);
                                    p = h.wrapping_add(c).wrapping_add(((o as u64) < m as u64) as i32 as u32 as i64);
                                    if (h ^ c ^ 18446744073709551615) & (h ^ p) < 0 {
                                        break 'label1;
                                    }
                                    let ab = Self::func84(env, distrib, f);
                                    let ac = Self::func83(
                                        env,
                                        token_address,
                                        i,
                                        c,
                                    );
                                    env.events().publish(val_from_i64(ab), val_from_i64(ac));
                                    m = o;
                                    h = p;
                                }
                            }
                            f = j.wrapping_sub(h).wrapping_sub(((k as u64) < m as u64) as i32 as u32 as i64);
                            if (j ^ h) & (j ^ f) < 0 {
                                break 'label1;
                            }
                            d = k.wrapping_sub(m);
                            if ((if f == 0 { (d == 0) as i32 } else { (f > 0) as i32 })) & l != 0 {
                                c = m;
                                g = h;
                            } else {
                                Self::func68(
                                    env,
                                    a.wrapping_add(176),
                                    n,
                                    token_address,
                                );
                                e = mload32!(a.wrapping_add(176) as usize) & 1;
                                let a_part_200 = mload64!(a.wrapping_add(200) as usize);
                                if e != 0 {
                                    c = a_part_200;
                                } else {
                                    c = 0;
                                }
                                let a_part_192 = mload64!(a.wrapping_add(192) as usize);
                                if e != 0 {
                                    g = a_part_192;
                                } else {
                                    g = 0;
                                }
                                i = g.wrapping_add(d);
                                g = c.wrapping_add(f).wrapping_add(((i as u64) < g as u64) as i32 as u32 as i64);
                                if (c ^ f ^ 18446744073709551615) & (c ^ g) < 0 {
                                    break 'label1;
                                }
                                Self::func69(
                                    env,
                                    n,
                                    token_address,
                                    i,
                                    g,
                                );
                                c = m.wrapping_add(d);
                                g = h.wrapping_add(f).wrapping_add(((c as u64) < m as u64) as i32 as u32 as i64);
                                if (h ^ f ^ 18446744073709551615) & (h ^ g) < 0 {
                                    break 'label1;
                                }
                                let ae = Self::func84(env, dust, n);
                                let af = Self::func83(
                                    env,
                                    token_address,
                                    d,
                                    f,
                                );
                                env.events().publish(val_from_i64(ae), val_from_i64(af));
                            }
                            let ah = Self::func84(env, dist_all, token_address);
                            let ai = Self::i128_parts_to_val(env, c, g);
                            env.events().publish(val_from_i64(ah), val_from_i64(ai));
                        }
                    }
                }
            }
            return b;
        }
        Self::call_unreachable(env);
        unreachable!();
    }

    pub fn get_allocation(
        env: Env,
        shareholder: Address,
        token: Address,
    ) -> Result<i128, Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        a = -32;
        b = 4294967299 /* Error(Contract, #1) */;
        {
            let e = Self::func63(env);
            if e != 0 {
                Self::func68(
                    env,
                    a,
                    shareholder,
                    token,
                );
                let value_lo = mload64!(a.wrapping_add(16) as usize);
                c = mload32!(a as usize) & 1;
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                Self::pack_i128_val(
                    env,
                    a,
                    (if c != 0 { value_lo } else { 0 }),
                    (if c != 0 { value_hi } else { 0 }),
                );
                if mload32!(a as usize) == 1 {
                    unreachable!();
                }
                b = mload64!(a.wrapping_add(8) as usize);
            }
        }
        return b;
    }

    pub fn get_commission_config(
        env: Env,
    ) -> Result<CommissionConfig, Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -80;
        b = a.wrapping_add(16);
        env.storage().get_contract_data(b);
        Self::func45(env, a.wrapping_add(64), b);
        {
            let a_i32_64 = mload32!(a.wrapping_add(64) as usize);
            if a_i32_64 == 1 {
                unreachable!();
            }
        }
        let loaded_val = mload64!(a.wrapping_add(72) as usize);
        loaded_val
    }

    pub fn get_config(
        env: Env,
    ) -> Result<ConfigDataKey, Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        a = -48;
        b = 4294967299 /* Error(Contract, #1) */;
        'label1: {
            {
                let e = Self::func63(env);
                if e != 0 {
                    Self::bump_instance_ttl(env);
                    env.storage().get_contract_data(a.wrapping_add(8), a.wrapping_add(24));
                    let f = mload8!(a.wrapping_add(16) as usize) as i32;
                    c = f;
                    if c == 2 {
                        break 'label1;
                    }
                    Self::func43(
                        env,
                        a.wrapping_add(24),
                        mload64!(a.wrapping_add(8) as usize),
                        c,
                    );
                    let a_i32_24 = mload32!(a.wrapping_add(24) as usize);
                    if a_i32_24 == 1 {
                        unreachable!();
                    }
                    b = mload64!(a.wrapping_add(32) as usize);
                }
            }
            return b;
        }
        Self::call_unreachable_2(env);
        unreachable!();
    }

    pub fn get_listing(
        env: Env,
        mut seller: Address,
    ) -> Result<Option<SaleListingDataKey>, Error> {
        let a: i32 = -80;
        env.storage().get_contract_data(a, seller);
        'label1: {
            {
                seller = mload64!(a as usize);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if seller ^ 0 | a_hi != 0 {
                    if seller as i32 & 1 == 0 {
                        seller = 0;
                        break 'label1;
                    }
                    Self::func67(env, a.wrapping_add(64), a.wrapping_add(16));
                    let a_i32_64 = mload32!(a.wrapping_add(64) as usize);
                    if a_i32_64 != 0 {
                        unreachable!();
                    }
                    let loaded_val = mload64!(a.wrapping_add(72) as usize);
                    seller = loaded_val;
                    break 'label1;
                }
            }
            seller = (mload32!(a.wrapping_add(16) as usize).wrapping_add(-1) as u32 as i64).wrapping_shl(32 as u32).wrapping_add(4294967299 /* Error(Contract, #1) */);
        }
        return seller;
    }

    pub fn get_share(
        env: Env,
        shareholder: Address,
    ) -> Result<Option<i128>, Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -48;
        b = 4294967299 /* Error(Contract, #1) */;
        'label1: {
            let d = Self::func63(env);
            if d != 0 {
                env.storage().get_contract_data(a, shareholder);
                if mload32!(a as usize) & 1 == 0 {
                    b = 0;
                    break 'label1;
                }
                let value_lo = mload64!(a.wrapping_add(16) as usize);
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                Self::pack_i128_val(
                    env,
                    a,
                    value_lo,
                    value_hi,
                );
                if mload32!(a as usize) == 1 {
                    unreachable!();
                }
                b = mload64!(a.wrapping_add(8) as usize);
            }
        }
        return b;
    }

    pub fn init(
        env: Env,
        mut admin: Address,
        mut shares: Vec<ShareDataKey>,
        mutable: bool,
    ) -> Result<(), Error> {
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        c = -32;
        d = 2;
        e = mutable as i32 & 255;
        e = { let a = 1; let b = { let a = 2; let b = 0; if e != 0 { a } else { b } }; if (e == 1) as i32 != 0 { a } else { b } };
        if e != 2 {
            'label1: {
                {
                    let g = Self::func63(env);
                    if g == 0 {
                        Self::bump_instance_ttl(env);
                        env.storage().put_contract_data(c.wrapping_add(8), admin, e);
                        let h = Self::func46(env, shares);
                        d = h;
                        if d == 0 {
                            Self::func51(env, shares);
                            let i = Vec::<Val>::from_val(env, &val_from_i64(shares)).len() as i64;
                            shares = i;
                            let j = Self::func84(env, init, admin);
                            admin = j;
                            (shares & 18446744069414584320 | 0) as i64;
                            let k = val_to_i64(Vec::<Val>::new(env).into_val(env));
                            env.events().publish(val_from_i64(admin), val_from_i64(k));
                            shares = 0;
                            break 'label1;
                        }
                    }
                }
                shares = (d.wrapping_add(-1) as u32 as i64).wrapping_shl(32 as u32).wrapping_add(4294967299 /* Error(Contract, #1) */);
            }
            return shares;
        }
    }

    pub fn list_all_sales(
        env: Env,
    ) -> Result<Vec<SaleListingDataKey>, Error> {
        let mut a: i32 = 0;
        let mut vec_builder: i64 = 0;
        let mut d: i32 = 0;
        a = -96;
        let f = Self::func76(env);
        let g = val_to_i64(Vec::<Val>::new(env).into_val(env));
        vec_builder = g;
        let h = Vec::<Val>::from_val(env, &val_from_i64(f)).len() as i64;
        mstore32!(a.wrapping_add(12) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
        d = a.wrapping_add(48);
        'label0: {
            loop {
                Self::func27(env, a.wrapping_add(32), a);
                Self::func28(
                    env,
                    a.wrapping_add(16),
                    mload64!(a.wrapping_add(32) as usize),
                    mload64!(a.wrapping_add(40) as usize),
                );
                let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
                if a_i32_16 != 0 {
                    unreachable!();
                }
                break 'label0;
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                env.storage().get_contract_data(a.wrapping_add(32), value_hi);
                let i = Self::func75(env, d);
                vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(i)); val_to_i64(v.into_val(env)) };
            }
        }
        vec_builder
    }

    pub fn list_shares(
        env: Env,
    ) -> Result<Vec<ShareDataKey>, Error> {
        let mut a: i32 = 0;
        let mut vec_builder: i64 = 0;
        a = -80;
        vec_builder = 4294967299 /* Error(Contract, #1) */;
        'label1: {
            let e = Self::func63(env);
            if e != 0 {
                let f = val_to_i64(Vec::<Val>::new(env).into_val(env));
                vec_builder = f;
                let g = Self::func58(env);
                let h = Vec::<Val>::from_val(env, &val_from_i64(g)).len() as i64;
                mstore32!(a.wrapping_add(12) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
                loop {
                    Self::func27(env, a.wrapping_add(32), a);
                    Self::func28(
                        env,
                        a.wrapping_add(16),
                        mload64!(a.wrapping_add(32) as usize),
                        mload64!(a.wrapping_add(40) as usize),
                    );
                    let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
                    if a_i32_16 != 0 {
                        unreachable!();
                    }
                    break 'label1;
                    let value_hi = mload64!(a.wrapping_add(24) as usize);
                    env.storage().get_contract_data(a.wrapping_add(32), value_hi);
                    if mload32!(a.wrapping_add(32) as usize) & 1 == 0 {
                        unreachable!();
                    }
                    let loaded_val = mload64!(a.wrapping_add(56) as usize);
                    let i = Self::func54(
                        env,
                        mload64!(a.wrapping_add(48) as usize),
                        loaded_val,
                        mload64!(a.wrapping_add(64) as usize),
                    );
                    vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(i)); val_to_i64(v.into_val(env)) };
                }
            }
        }
        return vec_builder;
        Self::call_unreachable_2(env);
        unreachable!();
    }

    pub fn list_shares_for_sale(
        env: Env,
        seller: Address,
        mut shares_amount: i128,
        price_per_share: i128,
        payment_token: Address,
    ) -> Result<(), Error> {
        let mut amount_val: i32 = 0;
        let a: i64 = 0;
        let mut c: i64 = 0;
        amount_val = -48;
        'label0: {
            {
                Self::decode_i128_parts(env, amount_val, shares_amount);
                if mload32!(amount_val as usize) != 0 {
                    unreachable!();
                }
                let amount_lo_2 = mload64!(amount_val.wrapping_add(16) as usize);
                let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
                Self::decode_i128_parts(env, amount_val, price_per_share);
                if mload32!(amount_val as usize) != 0 {
                    unreachable!();
                }
                {
                    let cond_val = if amount_hi != 0 { (amount_lo_2 == 0) as i32 } else { (amount_hi < 0) as i32 };
                    if cond_val == 0 {
                        shares_amount = 68719476739 /* Error(Contract, #16) */;
                        break 'label0;
                    }
                }
                {
                    let amount_lo_3 = mload64!(amount_val.wrapping_add(16) as usize);
                    let amount_hi_2 = mload64!(amount_val.wrapping_add(24) as usize);
                    let cond_val_2 = if amount_hi_2 != 0 { (amount_lo_3 == 0) as i32 } else { (amount_hi_2 < 0) as i32 };
                    if cond_val_2 == 0 {
                        shares_amount = 64424509443 /* Error(Contract, #15) */;
                        break 'label0;
                    }
                }
                seller.require_auth();
                env.storage().get_contract_data(amount_val, seller);
                if mload32!(amount_val as usize) & 1 == 0 {
                    unreachable!();
                }
                let mut amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
                c = mload64!(amount_val.wrapping_add(24) as usize);
                let cond_val_3 = if c == shares_amount { ((amount_lo as u64) < a as u64) as i32 } else { (c < shares_amount) as i32 };
                if cond_val_3 != 0 {
                    unreachable!();
                }
                env.storage().put_contract_data(seller, a, shares_amount, amount_lo_3, amount_hi_2, payment_token);
                let amount_lo = amount_lo_3 as i64;
                let e = Self::func84(env, listed, seller);
                let f = Self::func81(env, amount_val);
                env.events().publish(val_from_i64(e), val_from_i64(f));
                shares_amount = 0;
                break 'label0;
            }
            unreachable!();
            shares_amount = 51539607555 /* Error(Contract, #12) */;
        }
        shares_amount
    }

    pub fn lock_contract(
        env: Env,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -48;
        b = 4294967299 /* Error(Contract, #1) */;
        {
            let d = Self::func63(env);
            if d != 0 {
                Self::require_owner_auth(env);
                Self::bump_instance_ttl(env);
                env.storage().get_contract_data(a.wrapping_add(32), a.wrapping_add(8));
                {
                    let e = mload8!(a.wrapping_add(40) as usize) as i32;
                    if e != 2 {
                        env.storage().put_contract_data(a.wrapping_add(8), mload64!(a.wrapping_add(32) as usize), 0);
                    }
                }
                let f = Self::func82(env, locked);
                env.events().publish(val_from_i64(f), val_from_i64(1));
                b = 0;
            }
        }
        b
    }

    pub fn set_buy_commission_rate(
        env: Env,
        mut new_rate_bps: i128,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut authorized_addr: i64 = 0;
        let mut c: i64 = 0;
        a = -128;
        Self::decode_i128_parts(env, a.wrapping_add(48), new_rate_bps);
        {
            let a_i32_48 = mload32!(a.wrapping_add(48) as usize);
            if a_i32_48 != 1 {
                b = mload64!(a.wrapping_add(64) as usize);
                let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
                new_rate_bps = loaded_val;
                env.storage().get_contract_data(a);
                authorized_addr = mload64!(a.wrapping_add(32) as usize);
                address_from_i64(&env, authorized_addr).require_auth();
                c = 103079215107 /* Error(Contract, #24) */;
                {
                    let cond_val = if new_rate_bps == 0 { (b as u64 > 5000 as u64) as i32 } else { (new_rate_bps == 0) as i32 };
                    if cond_val != 0 {
                        let value_hi = mload64!(a.wrapping_add(24) as usize);
                        let loaded_val = value_hi as i64;
                        mload64!(a.wrapping_add(16) as usize);
                        env.storage().put_contract_data(a.wrapping_add(104), a.wrapping_add(48));
                        Self::bump_instance_ttl(env);
                        c = 0;
                    }
                }
                return c;
            }
        }
        unreachable!();
    }

    pub fn set_commission_recipient(
        env: Env,
        new_recipient: Address,
    ) -> Result<(), Error> {
        let a: i32 = -128;
        env.storage().get_contract_data(a);
        let authorized_addr = mload64!(a.wrapping_add(32) as usize);
        address_from_i64(&env, authorized_addr).require_auth();
        mload64!(a.wrapping_add(24) as usize);
        mload64!(a.wrapping_add(16) as usize);
        env.storage().put_contract_data(a.wrapping_add(104), a.wrapping_add(48));
        Self::bump_instance_ttl(env);
        0
    }

    pub fn set_distribution_commission_rate(
        env: Env,
        mut new_rate_bps: i128,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut authorized_addr: i64 = 0;
        let mut c: i64 = 0;
        a = -128;
        Self::decode_i128_parts(env, a.wrapping_add(48), new_rate_bps);
        {
            let a_i32_48 = mload32!(a.wrapping_add(48) as usize);
            if a_i32_48 != 1 {
                b = mload64!(a.wrapping_add(64) as usize);
                let mut loaded_val = mload64!(a.wrapping_add(72) as usize);
                new_rate_bps = loaded_val;
                env.storage().get_contract_data(a);
                authorized_addr = mload64!(a.wrapping_add(32) as usize);
                address_from_i64(&env, authorized_addr).require_auth();
                c = 103079215107 /* Error(Contract, #24) */;
                {
                    let cond_val = if new_rate_bps == 0 { (b as u64 > 5000 as u64) as i32 } else { (new_rate_bps == 0) as i32 };
                    if cond_val != 0 {
                        let loaded_val = new_rate_bps as i64;
                        env.storage().put_contract_data(a.wrapping_add(104), a.wrapping_add(48));
                        Self::bump_instance_ttl(env);
                        c = 0;
                    }
                }
                return c;
            }
        }
        unreachable!();
    }

    pub fn transfer_shares(
        env: Env,
        mut from: Address,
        to: Address,
        mut amount: i128,
    ) -> Result<(), Error> {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        amount_val = -144;
        'label0: {
            'label1: {
                'label3: {
                    {
                        Self::decode_i128_parts(env, amount_val.wrapping_add(48), amount);
                        let amount_val_i32_48 = mload32!(amount_val.wrapping_add(48) as usize);
                        if amount_val_i32_48 != 1 {
                            let mut loaded_val = mload64!(amount_val.wrapping_add(72) as usize);
                            amount = loaded_val;
                            a = mload64!(amount_val.wrapping_add(64) as usize);
                            {
                                let k = Self::func63(env);
                                if k == 0 {
                                    from = 4294967299 /* Error(Contract, #1) */;
                                    break 'label0;
                                }
                            }
                            from.require_auth();
                            {
                                let l = Self::func50(env, from, to);
                                if l != 0 {
                                    from = 85899345923 /* Error(Contract, #20) */;
                                    break 'label0;
                                }
                            }
                            {
                                let cond_val = if amount != 0 { (a == 0) as i32 } else { (amount < 0) as i32 };
                                if cond_val == 0 {
                                    from = 68719476739 /* Error(Contract, #16) */;
                                    break 'label0;
                                }
                            }
                            env.storage().get_contract_data(amount_val, from);
                            if mload32!(amount_val as usize) & 1 == 0 {
                                from = 77309411331 /* Error(Contract, #18) */;
                                break 'label0;
                            }
                            {
                                b = mload64!(amount_val.wrapping_add(16) as usize);
                                c = ((b as u64) < a as u64) as i32;
                                d = mload64!(amount_val.wrapping_add(24) as usize);
                                let cond_val_2 = if d != amount { c } else { (d < amount) as i32 };
                                if cond_val_2 == 0 {
                                    from = 81604378627 /* Error(Contract, #19) */;
                                    break 'label0;
                                }
                            }
                            env.storage().get_contract_data(amount_val.wrapping_add(48), to);
                            e = mload64!(amount_val.wrapping_add(56) as usize);
                            f = a;
                            g = amount;
                            h = mload64!(amount_val.wrapping_add(48) as usize);
                            if h as i32 & 1 != 0 {
                                let loaded_val = mload64!(amount_val.wrapping_add(72) as usize);
                                i = loaded_val;
                                g = mload64!(amount_val.wrapping_add(64) as usize);
                                f = g.wrapping_add(a);
                                g = i.wrapping_add(amount).wrapping_add(((f as u64) < g as u64) as i32 as u32 as i64);
                                if (i ^ amount ^ 18446744073709551615) & (i ^ g) < 0 {
                                    break 'label3;
                                }
                            }
                            if b ^ a | d ^ amount != 0 {
                                unreachable!();
                            }
                            Self::func56(env, from);
                            let m = Self::func58(env);
                            d = m;
                            let n = Vec::<Val>::from_val(env, &val_from_i64(d)).len() as i64;
                            b = n;
                            mstore32!(amount_val.wrapping_add(108) as usize, (b as u64).wrapping_shr(32 as u32) as i64 as u32);
                            loop {
                                Self::func26(env, amount_val.wrapping_add(120), amount_val.wrapping_add(96));
                                let amount_val_i32_120 = mload32!(amount_val.wrapping_add(120) as usize);
                                if amount_val_i32_120 != 0 {
                                    unreachable!();
                                }
                                break 'label1;
                                c = mload32!(amount_val.wrapping_add(128) as usize);
                                let mut sv3_136_i64 = mload64!(amount_val.wrapping_add(136) as usize);
                                let o = Self::func50(env, sv3_136_i64, from);
                            }
                            {
                                let p = Vec::<Val>::from_val(env, &val_from_i64(d)).len() as i64;
                                if c as u32 < (p as u64).wrapping_shr(32 as u32) as i64 as i32 as u32 {
                                    let q = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(d)); v.remove_unchecked((c as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); val_to_i64(v.into_val(env)) };
                                    d = q;
                                }
                            }
                            Self::func53(env, d);
                            break 'label1;
                        }
                    }
                    unreachable!();
                }
                Self::call_unreachable(env);
                unreachable!();
                env.storage().put_contract_data(from, b.wrapping_sub(a), d.wrapping_sub(amount).wrapping_sub(c as u32 as i64));
            }
            env.storage().put_contract_data(to, f, g);
            if h | e == 0 {
                let vec_builder = Self::func58(env);
                vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(to)); val_to_i64(v.into_val(env)) };
                Self::func53(env, r);
            }
            let sv3_136_i64 = to as i64;
            let s = Self::func85(env, amount_val.wrapping_add(120));
            let t = Self::i128_parts_to_val(env, a, amount);
            env.events().publish(val_from_i64(s), val_from_i64(t));
            from = 0;
        }
        from
    }

    pub fn transfer_tokens(
        env: Env,
        mut token_address: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<(), Error> {
        let mut amount_val: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i32 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        amount_val = -32;
        {
            Self::decode_i128_parts(env, amount_val, amount);
            if mload32!(amount_val as usize) != 0 {
                unreachable!();
            }
            let amount_lo_2 = mload64!(amount_val.wrapping_add(16) as usize);
            let amount_hi_2 = mload64!(amount_val.wrapping_add(24) as usize);
            'label1: {
                {
                    let h = Self::func63(env);
                    if h == 0 {
                        token_address = 4294967299 /* Error(Contract, #1) */;
                        break 'label1;
                    }
                }
                Self::require_owner_auth(env);
                let i = val_to_i64(env.current_contract_address().into_val(env));
                Self::func89(
                    env,
                    amount_val,
                    token_address,
                    i,
                );
                b = mload64!(amount_val as usize);
                c = mload64!(amount_val.wrapping_add(8) as usize);
                Self::func70(env, amount_val, token_address);
                {
                    let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
                    d = mload32!(amount_val as usize) & 1;
                    if d != 0 {
                        e = amount_hi;
                    } else {
                        e = 0;
                    }
                    let j = e;
                    let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
                    if d != 0 {
                        f = amount_lo;
                    } else {
                        f = 0;
                    }
                    e = c.wrapping_sub(e).wrapping_sub(((b as u64) < f as u64) as i32 as u32 as i64);
                    if (c ^ j) & (c ^ e) >= 0 {
                        let cond_val = if amount_hi_2 == 0 { (amount_lo_2 == 0) as i32 } else { (amount_hi_2 < 0) as i32 };
                        if cond_val == 0 {
                            unreachable!();
                        }
                        token_address = 30064771075 /* Error(Contract, #7) */;
                        break 'label1;
                    }
                }
                Self::call_unreachable(env);
                unreachable!();
                {
                    let cond_val_2 = if amount_hi_2 != c { (amount_lo_2 as u64 > b as u64) as i32 } else { (amount_hi_2 > c) as i32 };
                    if cond_val_2 == 0 {
                        token_address = 34359738371 /* Error(Contract, #8) */;
                        break 'label1;
                    }
                }
                {
                    let cond_val_3 = if amount_hi_2 != e { (amount_lo_2 as u64 > b.wrapping_sub(f) as u64) as i32 } else { (amount_hi_2 > e) as i32 };
                    if cond_val_3 == 0 {
                        token_address = 38654705667 /* Error(Contract, #9) */;
                        break 'label1;
                    }
                }
                let k = val_to_i64(env.current_contract_address().into_val(env));
                Self::func30(
                    env,
                    token_address,
                    k,
                    recipient,
                    amount_lo_2,
                    amount_hi_2,
                );
                let l = Self::func84(env, transfer, recipient);
                let m = Self::func83(
                    env,
                    token_address,
                    amount_lo_2,
                    amount_hi_2,
                );
                env.events().publish(val_from_i64(l), val_from_i64(m));
                token_address = 0;
            }
            return token_address;
        }
    }

    pub fn update_shares(
        env: Env,
        mut shares: Vec<ShareDataKey>,
    ) -> Result<(), Error> {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -64;
        'label0: {
            'label1: {
                {
                    {
                        let f = Self::func63(env);
                        if f == 0 {
                            b = 1;
                            break 'label1;
                        }
                    }
                    Self::require_owner_auth(env);
                    Self::bump_instance_ttl(env);
                    let mut sv1_40_i64 = 0 as i64;
                    env.storage().get_contract_data(a.wrapping_add(24), a.wrapping_add(40));
                    {
                        let g = mload8!(a.wrapping_add(32) as usize) as i32;
                        if g & 1 == 0 {
                            b = 4;
                            break 'label1;
                        }
                    }
                    let h = Self::func46(env, shares);
                    b = h;
                    if b != 0 {
                        unreachable!();
                    }
                    let i = Self::func58(env);
                    let j = Vec::<Val>::from_val(env, &val_from_i64(i)).len() as i64;
                    mstore32!(a.wrapping_add(20) as usize, (j as u64).wrapping_shr(32 as u32) as i64 as u32);
                    'label5: {
                        loop {
                            Self::func27(env, a.wrapping_add(40), a.wrapping_add(8));
                            let mut sv1_40_i64 = mload64!(a.wrapping_add(40) as usize);
                            Self::func28(
                                env,
                                a.wrapping_add(24),
                                sv1_40_i64,
                                mload64!(a.wrapping_add(48) as usize),
                            );
                            let a_i32_24 = mload32!(a.wrapping_add(24) as usize);
                            if a_i32_24 != 0 {
                                unreachable!();
                            }
                            break 'label5;
                            Self::func56(env, mload64!(a.wrapping_add(32) as usize));
                        }
                    }
                    let sv1_40_i64 = 1 as i64;
                    let k = Self::func34(env, a.wrapping_add(40));
                    env.storage().del_contract_data(k);
                    Self::func51(env, shares);
                    let l = Vec::<Val>::from_val(env, &val_from_i64(shares)).len() as i64;
                    shares = l;
                    let m = Self::func82(env, shares);
                    env.events().publish(val_from_i64(m), val_from_i64(shares & 18446744069414584320 | 0));
                    shares = 0;
                    break 'label0;
                }
                unreachable!();
            }
            shares = (b.wrapping_add(-1) as u32 as i64).wrapping_shl(32 as u32).wrapping_add(4294967299 /* Error(Contract, #1) */);
        }
        shares
    }

    pub fn withdraw_allocation(
        env: Env,
        token_address: Address,
        mut shareholder: Address,
        mut amount: i128,
    ) -> Result<(), Error> {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        amount_val = -64;
        'label0: {
            {
                Self::decode_i128_parts(env, amount_val.wrapping_add(32), amount);
                let amount_val_i32_32 = mload32!(amount_val.wrapping_add(32) as usize);
                if amount_val_i32_32 != 0 {
                    unreachable!();
                }
                let mut loaded_val = mload64!(amount_val.wrapping_add(56) as usize);
                amount = loaded_val;
                a = mload64!(amount_val.wrapping_add(48) as usize);
                'label3: {
                    {
                        let h = Self::func63(env);
                        if h == 0 {
                            shareholder = 4294967299 /* Error(Contract, #1) */;
                            break 'label3;
                        }
                    }
                    shareholder.require_auth();
                    Self::func68(
                        env,
                        amount_val.wrapping_add(32),
                        shareholder,
                        token_address,
                    );
                    if a | amount == 0 {
                        shareholder = 42949672963 /* Error(Contract, #10) */;
                        break 'label3;
                    }
                    {
                        let mut sv3_48_i64 = mload64!(amount_val.wrapping_add(48) as usize);
                        b = mload32!(amount_val.wrapping_add(32) as usize) & 1;
                        if b != 0 {
                            c = sv3_48_i64;
                        } else {
                            c = 0;
                        }
                        let mut loaded_val = mload64!(amount_val.wrapping_add(56) as usize);
                        if b != 0 {
                            d = loaded_val;
                        } else {
                            d = 0;
                        }
                        let cond_val = if amount != d { (a as u64 > c as u64) as i32 } else { (amount > d) as i32 };
                        if cond_val == 0 {
                            shareholder = 47244640259 /* Error(Contract, #11) */;
                            break 'label3;
                        }
                    }
                    e = amount ^ d;
                    if a ^ c | e != 0 {
                        f = d.wrapping_sub(amount).wrapping_sub(((c as u64) < a as u64) as i32 as u32 as i64);
                        if e & (d ^ f) < 0 {
                            break 'label0;
                        }
                        Self::func69(
                            env,
                            shareholder,
                            token_address,
                            c.wrapping_sub(a),
                            f,
                        );
                    } else {
                        Self::func70(env, amount_val, token_address);
                        'label9: {
                            if mload32!(amount_val as usize) & 1 != 0 {
                                let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
                                let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
                                Self::func68(
                                    env,
                                    amount_val.wrapping_add(32),
                                    shareholder,
                                    token_address,
                                );
                                if mload32!(amount_val.wrapping_add(32) as usize) & 1 == 0 {
                                    unreachable!();
                                }
                                let loaded_val = mload64!(amount_val.wrapping_add(56) as usize);
                                e = loaded_val;
                                let i = e;
                                f = mload64!(amount_val.wrapping_add(48) as usize);
                                e = amount_hi.wrapping_sub(e).wrapping_sub(((amount_lo as u64) < f as u64) as i32 as u32 as i64);
                                if (amount_hi ^ i) & (amount_hi ^ e) < 0 {
                                    break 'label0;
                                }
                                d = c.wrapping_sub(f);
                                if d | e == 0 {
                                    Self::func72(env, token_address);
                                    break 'label9;
                                }
                                Self::func71(
                                    env,
                                    token_address,
                                    d,
                                    e,
                                );
                            }
                        }
                        let sv3_48_i64 = token_address as i64;
                        let j = Self::func34(env, amount_val.wrapping_add(32));
                        env.storage().del_contract_data(j);
                    }
                    let k = val_to_i64(env.current_contract_address().into_val(env));
                    Self::func30(
                        env,
                        token_address,
                        k,
                        shareholder,
                        a,
                        amount,
                    );
                    let l = Self::func84(env, withdraw, shareholder);
                    let m = Self::func83(
                        env,
                        token_address,
                        a,
                        amount,
                    );
                    env.events().publish(val_from_i64(l), val_from_i64(m));
                    shareholder = 0;
                }
                return shareholder;
            }
            unreachable!();
            Self::call_unreachable_2(env);
            unreachable!();
        }
        Self::call_unreachable(env);
        unreachable!();
    }
}

impl SorobanAuditorFailed {

    fn func26(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -32;
        Self::func27(env, a.wrapping_add(16), arg1);
        let value_lo = mload64!(a.wrapping_add(16) as usize);
        let value_hi = mload64!(a.wrapping_add(24) as usize);
        Self::func28(
            env,
            a,
            value_lo,
            value_hi,
        );
        if mload32!(a as usize) == 1 {
            b = mload32!(arg1.wrapping_add(16) as usize);
            if b == -1 {
                unreachable!();
            }
        }
    }

    fn func27(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i64 = 0;
        let mut b: i32 = 0;
        a = 0;
        {
            let mut svarg1_8_i32 = mload32!(arg1.wrapping_add(8) as usize);
            b = svarg1_8_i32;
            if b as u32 < mload32!(arg1.wrapping_add(12) as usize) as u32 {
                let arg1_lo = mload64!(arg1 as usize);
                let c = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1_lo)).get_unchecked((b as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
                a = c;
                let svarg1_8_i32 = b += 1 as i32;
                a = (!(Address::try_from_val(env, &val_from_i64(a)).is_ok())) as i32 as u32 as i64;
            }
        }
    }

    fn func28(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        if arg1 != 0 {
            if arg1 as i32 & 1 != 0 {
                unreachable!();
            }
        }
    }

    fn call_unreachable(
        env: &Env,
    ) {
        Self::unreachable_stub(env);
    }

    fn func30(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
        arg3: i64,
        arg4: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -48;
        Self::i128_parts_to_val(env, arg3, arg4);
        b = 0;
        loop {
            mstore64!(a.wrapping_add(24).wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
        }
    }

    fn i128_parts_to_val(
        env: &Env,
        arg0: i64,
        mut arg1: i64,
    ) -> i64 {
        let a: i32 = -16;
        Self::pack_i128_val(
            env,
            a,
            arg0,
            arg1,
        );
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        arg1 = mload64!(a.wrapping_add(8) as usize);
        arg1
    }



    fn func34(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        a = -32;
        'label0: {
            {
                'label2: {
                    {
                        {
                            {
                                {
                                    {
                                        {
                                            {
                                                Self::func78(
                                                    env,
                                                    a.wrapping_add(8),
                                                    1048576,
                                                    6,
                                                );
                                                let a_i32_8_8 = mload32!(a.wrapping_add(8) as usize);
                                                if a_i32_8_8 != 0 {
                                                    unreachable!();
                                                }
                                                let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                                                Self::func79(env, a.wrapping_add(8), value_lo);
                                                break 'label2;
                                            }
                                            Self::func78(
                                                env,
                                                a.wrapping_add(8),
                                                1048582,
                                                12,
                                            );
                                            let a_i32_8_6 = mload32!(a.wrapping_add(8) as usize);
                                            if a_i32_8_6 != 0 {
                                                unreachable!();
                                            }
                                            let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                                            Self::func79(env, a.wrapping_add(8), value_lo);
                                            break 'label2;
                                        }
                                        Self::func78(
                                            env,
                                            a.wrapping_add(8),
                                            1048594,
                                            5,
                                        );
                                        let a_i32_8_7 = mload32!(a.wrapping_add(8) as usize);
                                        if a_i32_8_7 != 0 {
                                            unreachable!();
                                        }
                                        let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                                        Self::vec_pair_builder(
                                            env,
                                            a.wrapping_add(8),
                                            value_lo,
                                            mload64!(arg0.wrapping_add(8) as usize),
                                        );
                                        break 'label2;
                                    }
                                    Self::func78(
                                        env,
                                        a.wrapping_add(8),
                                        1048599,
                                        15,
                                    );
                                    let a_i32_8_4 = mload32!(a.wrapping_add(8) as usize);
                                    if a_i32_8_4 != 0 {
                                        unreachable!();
                                    }
                                    let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                                    Self::vec_pair_builder(
                                        env,
                                        a.wrapping_add(8),
                                        value_lo,
                                        mload64!(arg0.wrapping_add(8) as usize),
                                    );
                                    break 'label2;
                                }
                                Self::func78(
                                    env,
                                    a.wrapping_add(8),
                                    1048614,
                                    10,
                                );
                                let a_i32_8_5 = mload32!(a.wrapping_add(8) as usize);
                                if a_i32_8_5 != 0 {
                                    unreachable!();
                                }
                                let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                                b = value_lo;
                                c = mload64!(arg0.wrapping_add(8) as usize);
                                mload64!(arg0.wrapping_add(16) as usize);
                                value_lo = c as i64;
                                let e = val_to_i64(Vec::<Val>::new(env).into_val(env));
                                b = e;
                                break 'label0;
                            }
                            Self::func78(
                                env,
                                a.wrapping_add(8),
                                1048624,
                                11,
                            );
                            let a_i32_8_2 = mload32!(a.wrapping_add(8) as usize);
                            if a_i32_8_2 != 0 {
                                unreachable!();
                            }
                            let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                            Self::vec_pair_builder(
                                env,
                                a.wrapping_add(8),
                                value_lo,
                                mload64!(arg0.wrapping_add(8) as usize),
                            );
                            break 'label2;
                        }
                        Self::func78(
                            env,
                            a.wrapping_add(8),
                            1048635,
                            14,
                        );
                        let a_i32_8_3 = mload32!(a.wrapping_add(8) as usize);
                        if a_i32_8_3 != 0 {
                            unreachable!();
                        }
                        let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                        Self::func79(env, a.wrapping_add(8), value_lo);
                        break 'label2;
                    }
                    Self::func78(
                        env,
                        a.wrapping_add(8),
                        1048649,
                        10,
                    );
                    let a_i32_8 = mload32!(a.wrapping_add(8) as usize);
                    if a_i32_8 != 0 {
                        unreachable!();
                    }
                    let value_lo = mload64!(a.wrapping_add(16) as usize);
                    Self::func79(env, a.wrapping_add(8), value_lo);
                }
                b = mload64!(a.wrapping_add(16) as usize);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if a_hi == 0 {
                    unreachable!();
                }
            }
        }
        b
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
                    let c = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64);
                    let d = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64);
                    arg1 = d;
                    let svarg0_24_i64 = c as i64;
                    let svarg0_16_i64 = arg1 as i64;
                }
                break 'label0;
            }
            Error(Value, UnexpectedType) as i64;
        }
    }





    fn func41(
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


    fn func43(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i32,
    ) {
        let a: i32 = -16;
        let c = Self::map_new_val(
            env,
            1048704,
            2,
            a,
            2,
        );
    }


    fn func45(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let a: i32 = -32;
        Self::pack_i128_val(
            env,
            a.wrapping_add(8),
            mload64!(arg1 as usize),
            mload64!(arg1.wrapping_add(8) as usize),
        );
        {
            let a_i32_8_2 = mload32!(a.wrapping_add(8) as usize);
            if a_i32_8_2 == 0 {
                let arg1_lo = mload64!(arg1.wrapping_add(16) as usize);
                let arg1_hi = mload64!(arg1.wrapping_add(24) as usize);
                Self::pack_i128_val(
                    env,
                    a.wrapping_add(8),
                    arg1_lo,
                    arg1_hi,
                );
                let a_i32_8 = mload32!(a.wrapping_add(8) as usize);
                if a_i32_8 == 0 {
                    mload64!(a.wrapping_add(16) as usize);
                    let e = Self::map_new_val(
                        env,
                        1048820,
                        3,
                        a.wrapping_add(8),
                        3,
                    );
                }
            }
        }
    }

    fn func46(
        env: &Env,
        arg0: i64,
    ) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        a = -128;
        b = 5;
        'label0: {
            let l = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
            if (l as u64) >= 4294967296 as u64 {
                let m = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                c = m;
                mstore32!(a.wrapping_add(20) as usize, (c as u64).wrapping_shr(32 as u32) as i64 as u32);
                d = 0;
                e = 0;
                'label1: {
                    'label2: loop {
                        Self::func47(env, a.wrapping_add(80), a.wrapping_add(8));
                        Self::func48(env, a.wrapping_add(32), a.wrapping_add(80));
                        'label3: {
                            if mload32!(a.wrapping_add(32) as usize) & 1 != 0 {
                                b = mload32!(a.wrapping_add(24) as usize);
                                if b == -1 {
                                    break 'label1;
                                }
                                let loaded_val = mload64!(a.wrapping_add(56) as usize);
                                f = loaded_val;
                                g = mload64!(a.wrapping_add(48) as usize);
                                h = mload64!(a.wrapping_add(64) as usize);
                                if f >= 0 {
                                    break 'label3;
                                }
                                b = 22;
                                break 'label0;
                            }
                            if (d ^ 10000 | e == 0) as i32 != 0 {
                                b = 0;
                            } else {
                                b = 6;
                            }
                            break 'label0;
                        }
                        i = b as u32 as i64;
                        c = i += 1;
                        let n = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                        j = (n as u64).wrapping_shr(32 as u32) as i64;
                        i = i.wrapping_shl(32 as u32) += 1;
                        loop {
                            'label6: {
                                if c as u64 < j as u64 {
                                    let o = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
                                    if c as u64 >= (o as u64).wrapping_shr(32 as u32) as i64 as u64 {
                                        break 'label6;
                                    }
                                    let p = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg0)).get_unchecked(i as u32));
                                    Self::func49(env, a.wrapping_add(80), p);
                                    if mload32!(a.wrapping_add(80) as usize) & 1 == 0 {
                                        unreachable!();
                                    }
                                    unreachable!();
                                }
                                c = d.wrapping_add(g);
                                i = e.wrapping_add(f).wrapping_add(((c as u64) < d as u64) as i32 as u32 as i64);
                                if (e ^ f ^ 18446744073709551615) & (e ^ i) < 0 {
                                    break 'label1;
                                }
                                continue 'label2;
                                let a_part_112 = mload64!(a.wrapping_add(112) as usize);
                                let q = Self::func50(env, h, a_part_112);
                                if q == 0 {
                                    unreachable!();
                                }
                                b = 23;
                                break 'label0;
                            }
                            c += 1;
                            i = i.wrapping_add(4294967296);
                        }
                    }
                }
                Self::call_unreachable(env);
                unreachable!();
            }
        }
        b
    }

    fn func47(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i32 = 0;
        {
            let mut svarg1_8_i32 = mload32!(arg1.wrapping_add(8) as usize);
            a = svarg1_8_i32;
        }
        let arg1_lo = mload64!(arg1 as usize);
        let b = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(arg1_lo)).get_unchecked((a as u32 as i64).wrapping_shl(32 as u32) | 0 as u32));
        Self::func49(env, arg0, b);
        let svarg1_8_i32 = a += 1 as i32;
    }

    fn func48(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let mut a: i64 = 0;
        'label1: {
            {
                a = mload64!(arg1 as usize);
                let arg1_hi_2 = mload64!(arg1.wrapping_add(8) as usize);
                if a ^ 0 | arg1_hi_2 == 0 {
                    a = 0;
                    break 'label1;
                }
            }
            if a as i32 & 1 != 0 {
                unreachable!();
            }
            mload64!(arg1.wrapping_add(24) as usize);
            mload64!(arg1.wrapping_add(16) as usize);
        }
    }

    fn func49(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        a = -48;
        b = 0;
        loop {
            mstore64!(a.wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
        }
        c = 0;
        {
            Self::func41(
                env,
                arg1,
                1048676,
                2,
                a,
                2,
            );
            Self::decode_i128_parts(env, a.wrapping_add(16), mload64!(a as usize));
            let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
            if a_i32_16 != 1 {
                arg1 = mload64!(a.wrapping_add(8) as usize);
                c = mload64!(a.wrapping_add(40) as usize);
            }
        }
    }

    fn func50(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let c = { let a = val_from_i64(arg0); let b = val_from_i64(arg1); if a < b { -1 } else if a > b { 1 } else { 0 } };
        (c == 0) as i32
    }

    fn func51(
        env: &Env,
        mut arg0: i64,
    ) {
        let mut a: i32 = 0;
        let mut vec_builder: i64 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        a = -112;
        let e = val_to_i64(Vec::<Val>::new(env).into_val(env));
        vec_builder = e;
        let f = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64;
        b = f;
        mstore32!(a.wrapping_add(12) as usize, (b as u64).wrapping_shr(32 as u32) as i64 as u32);
        loop {
            Self::func47(env, a.wrapping_add(64), a);
            Self::func48(env, a.wrapping_add(16), a.wrapping_add(64));
            if mload32!(a.wrapping_add(16) as usize) & 1 == 0 {
                unreachable!();
            }
            arg0 = mload64!(a.wrapping_add(40) as usize);
            b = mload64!(a.wrapping_add(32) as usize);
            c = mload64!(a.wrapping_add(48) as usize);
            vec_builder = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_builder)); v.push_back(val_from_i64(c)); val_to_i64(v.into_val(env)) };
            env.storage().put_contract_data(c, b, arg0);
        }
        Self::func53(env, vec_builder);
    }


    fn func53(
        env: &Env,
        arg0: i64,
    ) {
        let a: i32 = -32;
        env.storage().put_contract_data(a.wrapping_add(8), arg0);
        Self::func55(env, a.wrapping_add(8));
    }

    fn func54(
        env: &Env,
        arg0: i64,
        mut arg1: i64,
        arg2: i64,
    ) -> i64 {
        let a: i32 = -32;
        Self::pack_i128_val(
            env,
            a.wrapping_add(16),
            arg0,
            arg1,
        );
        {
            let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
            if a_i32_16 == 1 {
                unreachable!();
            }
        }
        arg1 = mload64!(a.wrapping_add(24) as usize);
        let c = Self::map_new_val(
            env,
            1048676,
            2,
            a,
            2,
        );
        arg1 = c;
        arg1
    }

    fn func55(
        env: &Env,
        arg0: i32,
    ) {
        let balance_val = Self::func34(env, arg0);
        let a = env.storage().temporary().extend_ttl(&val_from_i64(balance_val), BALANCE_LIFETIME_THRESHOLD as u32, BALANCE_BUMP_AMOUNT as u32);
    }

    fn func56(
        env: &Env,
        arg0: i64,
    ) {
        let a: i32 = -32;
        let c = Self::func34(env, a.wrapping_add(8));
        env.storage().del_contract_data(c);
    }


    fn func58(
        env: &Env,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -48;
        env.storage().get_contract_data(a.wrapping_add(32), a.wrapping_add(8));
        'label0: {
            {
                let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                if a_i32_32 == 1 {
                    b = mload64!(a.wrapping_add(40) as usize);
                    Self::func55(env, a.wrapping_add(8));
                    break 'label0;
                }
            }
            let d = val_to_i64(Vec::<Val>::new(env).into_val(env));
            b = d;
        }
        b
    }


    fn require_owner_auth(
        env: &Env,
    ) {
        let a: i32 = -48;
        Self::bump_instance_ttl(env);
        env.storage().get_contract_data(a.wrapping_add(32), a.wrapping_add(8));
        {
            let c = mload8!(a.wrapping_add(40) as usize) as i32;
            if c == 2 {
                Self::call_unreachable_2(env);
                unreachable!();
            }
        }
        let authorized_addr = mload64!(a.wrapping_add(32) as usize);
        address_from_i64(&env, authorized_addr).require_auth();
    }

    fn bump_instance_ttl(
        env: &Env,
    ) {
        env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD as u32, INSTANCE_BUMP_AMOUNT as u32);
    }

    fn call_unreachable_2(
        env: &Env,
    ) {
        Self::call_unreachable(env);
    }

    fn func63(
        env: &Env,
    ) -> i32 {
        let a: i32 = -32;
        Self::bump_instance_ttl(env);
        let d = Self::func34(env, a.wrapping_add(8));
        let e = Self::call_eq_one(env, d, 0);
        e
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


    fn pack_i128_val(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        arg2: i64,
    ) {
        'label0: {
            if arg1.wrapping_add(36028797018963968) as u64 <= 72057594037927935 as u64 {
                if arg1 ^ arg1 | arg2 ^ arg1.wrapping_shr(63 as u32) == 0 {
                    arg1 = arg1 | 0;
                    break 'label0;
                }
            }
            val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)));
        }
    }

    fn func67(
        env: &Env,
        arg0: i32,
        arg1: i32,
    ) {
        let a: i32 = -32;
        let arg1_lo = mload64!(arg1.wrapping_add(16) as usize);
        let arg1_hi = mload64!(arg1.wrapping_add(24) as usize);
        Self::pack_i128_val(
            env,
            a,
            arg1_lo,
            arg1_hi,
        );
        if mload32!(a as usize) == 0 {
            Self::pack_i128_val(
                env,
                a,
                mload64!(arg1 as usize),
                mload64!(arg1.wrapping_add(8) as usize),
            );
            if mload32!(a as usize) == 0 {
                let g = Self::map_new_val(
                    env,
                    1048896,
                    4,
                    a,
                    4,
                );
            }
        }
    }

    fn func68(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -32;
        env.storage().get_contract_data(arg0, a.wrapping_add(8));
        if mload32!(arg0 as usize) & 1 != 0 {
            Self::func55(env, a.wrapping_add(8));
        }
    }

    fn func69(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
        arg3: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut e: i64 = 0;
        a = -32;
        Self::func68(
            env,
            a,
            arg0,
            arg1,
        );
        'label0: {
            'label2: {
                let value_hi = mload64!(a.wrapping_add(24) as usize);
                b = mload32!(a as usize) & 1;
                if b != 0 {
                    c = value_hi;
                } else {
                    c = 0;
                }
                let g = c;
                let mut value_lo = mload64!(a.wrapping_add(16) as usize);
                if b != 0 {
                    d = value_lo;
                } else {
                    d = 0;
                }
                c = arg3.wrapping_sub(c).wrapping_sub(((arg2 as u64) < d as u64) as i32 as u32 as i64);
                if (arg3 ^ g) & (arg3 ^ c) >= 0 {
                    d = arg2.wrapping_sub(d);
                    if d | c == 0 {
                        unreachable!();
                    }
                    Self::func70(env, a, arg1);
                    if mload32!(a as usize) & 1 != 0 {
                        e = mload64!(a.wrapping_add(24) as usize);
                        let h = c;
                        c = mload64!(a.wrapping_add(16) as usize);
                        d = c.wrapping_add(d);
                        c = e.wrapping_add(c).wrapping_add(((d as u64) < c as u64) as i32 as u32 as i64);
                        if (e ^ h ^ 18446744073709551615) & (e ^ c) < 0 {
                            break 'label2;
                        }
                        let cond_val = if c == 0 { (d == 0) as i32 } else { (c < 0) as i32 };
                        if cond_val != 0 {
                            unreachable!();
                        }
                        Self::func71(
                            env,
                            arg1,
                            d,
                            c,
                        );
                        break 'label0;
                    }
                    let cond_val_2 = if c == 0 { (d == 0) as i32 } else { (c < 0) as i32 };
                    if cond_val_2 != 0 {
                        unreachable!();
                    }
                    Self::func71(
                        env,
                        arg1,
                        d,
                        c,
                    );
                    break 'label0;
                }
            }
            Self::call_unreachable(env);
            unreachable!();
            Self::func72(env, arg1);
        }
        let value_lo = arg1 as i64;
        env.storage().put_contract_data(a, arg2, arg3);
        Self::func55(env, a);
    }

    fn func70(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let a: i32 = -32;
        3 /* Error(Contract, #0) */ as i64;
        env.storage().get_contract_data(arg0, a.wrapping_add(8));
        if mload32!(arg0 as usize) & 1 != 0 {
            Self::func55(env, a.wrapping_add(8));
        }
    }

    fn func71(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -32;
        3 /* Error(Contract, #0) */ as i64;
        env.storage().put_contract_data(a.wrapping_add(8), arg1, arg2);
        Self::func55(env, a.wrapping_add(8));
    }

    fn func72(
        env: &Env,
        arg0: i64,
    ) {
        let a: i32 = -32;
        3 /* Error(Contract, #0) */ as i64;
        let c = Self::func34(env, a.wrapping_add(8));
        env.storage().del_contract_data(c);
    }



    fn func75(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -16;
        Self::func67(env, a, arg0);
        if mload32!(a as usize) == 1 {
            unreachable!();
        }
        b = mload64!(a.wrapping_add(8) as usize);
        b
    }

    fn func76(
        env: &Env,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -48;
        env.storage().get_contract_data(a.wrapping_add(32), a.wrapping_add(8));
        'label0: {
            {
                let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                if a_i32_32 == 1 {
                    b = mload64!(a.wrapping_add(40) as usize);
                    Self::func55(env, a.wrapping_add(8));
                    break 'label0;
                }
            }
            let d = val_to_i64(Vec::<Val>::new(env).into_val(env));
            b = d;
        }
        b
    }

    fn func77(
        env: &Env,
        arg0: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut d: i32 = 0;
        a = -80;
        let f = Self::func34(env, a.wrapping_add(8));
        env.storage().del_contract_data(f);
        let g = Self::func76(env);
        b = g;
        let h = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
        mstore32!(a.wrapping_add(44) as usize, (h as u64).wrapping_shr(32 as u32) as i64 as u32);
        'label0: {
            loop {
                Self::func26(env, a.wrapping_add(56), a.wrapping_add(32));
                let a_i32_56 = mload32!(a.wrapping_add(56) as usize);
                if a_i32_56 != 0 {
                    unreachable!();
                }
                break 'label0;
                d = mload32!(a.wrapping_add(64) as usize);
                let loaded_val = mload64!(a.wrapping_add(72) as usize);
                let i = Self::func50(env, loaded_val, arg0);
            }
            {
                let j = Vec::<Val>::from_val(env, &val_from_i64(b)).len() as i64;
                if d as u32 < (j as u64).wrapping_shr(32 as u32) as i64 as i32 as u32 {
                    let k = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(b)); v.remove_unchecked((d as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); val_to_i64(v.into_val(env)) };
                    b = k;
                }
            }
            env.storage().put_contract_data(a.wrapping_add(56), b);
            Self::func55(env, a.wrapping_add(56));
        }
    }

    fn func78(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i64 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        'label0: {
            'label1: {
                if arg2 as u32 <= 9 as u32 {
                    a = 0;
                    b = arg2;
                    c = arg1;
                    loop {
                        if b == 0 {
                            a = a | 0 /* Symbol() */;
                            break 'label0;
                        }
                        d = 1;
                        'label4: {
                            let f = mload8!(c as usize) as i32;
                            e = f;
                            if e != 95 {
                                'label5: {
                                    if ((e.wrapping_add(-48) & 255) as u32) >= 10 as u32 {
                                        if ((e.wrapping_add(-65) & 255) as u32) < 26 as u32 {
                                            break 'label5;
                                        }
                                        if (e.wrapping_add(-97) & 255) as u32 >= 26 as u32 {
                                            break 'label1;
                                        }
                                        d = e.wrapping_add(-59);
                                        break 'label4;
                                    }
                                    d = e.wrapping_add(-46);
                                    break 'label4;
                                }
                                d = e.wrapping_add(-53);
                            }
                        }
                        a = a | d as u32 as i64 & 255;
                        b = b.wrapping_add(-1);
                        c += 1;
                    }
                }
            }
            val_to_i64(Symbol::new(env, ""));
        }
    }

    fn func79(
        env: &Env,
        arg0: i32,
        arg1: i64,
    ) {
        let a: i32 = -16;
        val_to_i64(Vec::<Val>::new(env).into_val(env));
    }

    fn vec_pair_builder(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -16;
        let vec_builder = arg2 as i64;
        let sv3_0_i64 = arg1 as i64;
        val_to_i64(vec![&env, val_from_i64(sv3_0_i64), val_from_i64(vec_builder)].into_val(env));
    }

    fn func81(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -48;
        let vec_builder = mload64!(arg0 as usize);
        let svarg0_8_i64 = mload64!(arg0.wrapping_add(8) as usize);
        Self::pack_i128_val(
            env,
            a.wrapping_add(32),
            vec_builder,
            svarg0_8_i64,
        );
        'label0: {
            {
                let a_i32_32_2 = mload32!(a.wrapping_add(32) as usize);
                if a_i32_32_2 == 0 {
                    b = mload64!(a.wrapping_add(40) as usize);
                    let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
                    let arg0_hi = mload64!(arg0.wrapping_add(24) as usize);
                    Self::pack_i128_val(
                        env,
                        a.wrapping_add(32),
                        arg0_lo,
                        arg0_hi,
                    );
                    let a_i32_32 = mload32!(a.wrapping_add(32) as usize);
                    if a_i32_32 != 0 {
                        unreachable!();
                    }
                    break 'label0;
                }
            }
            unreachable!();
        }
        let d = val_to_i64(vec![&env, val_from_i64(vec_builder), val_from_i64(svarg0_8_i64)].into_val(env));
        b = d;
        b
    }

    fn func82(
        env: &Env,
        mut arg0: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        a = -16;
        let vec_builder = arg0 as i64;
        b = 0;
        c = 1;
        loop {
            c = c.wrapping_add(-1);
            b = arg0;
        }
        let sv1_8_i64 = b as i64;
        let e = val_to_i64(vec![&env, val_from_i64(vec_builder), val_from_i64(sv1_8_i64)].into_val(env));
        arg0 = e;
        arg0
    }

    fn func83(
        env: &Env,
        arg0: i64,
        arg1: i64,
        mut arg2: i64,
    ) -> i64 {
        let a: i32 = -32;
        Self::pack_i128_val(
            env,
            a.wrapping_add(16),
            arg1,
            arg2,
        );
        {
            let a_i32_16 = mload32!(a.wrapping_add(16) as usize);
            if a_i32_16 == 1 {
                unreachable!();
            }
        }
        let value_hi = mload64!(a.wrapping_add(24) as usize);
        let vec_builder = value_hi as i64;
        let sv3_0_i64 = arg0 as i64;
        let c = val_to_i64(vec![&env, val_from_i64(sv3_0_i64), val_from_i64(vec_builder)].into_val(env));
        arg2 = c;
        arg2
    }

    fn func84(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -32;
        b = 0;
        let d: i64;
        loop {
            mstore64!(a.wrapping_add(16).wrapping_add(b) as usize, 0 as u64);
            b = b.wrapping_add(8);
            unreachable!();
        }
    }

    fn func85(
        env: &Env,
        mut arg0: i32,
    ) -> i64 {
        let a: i32 = -48;
        mload64!(arg0.wrapping_add(16) as usize);
        arg0 = 0;
        let d: i64;
        loop {
            mstore64!(a.wrapping_add(24).wrapping_add(arg0) as usize, 0 as u64);
            arg0 = arg0.wrapping_add(8);
            unreachable!();
        }
    }




    fn func89(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let a: i32 = -32;
        let c = val_to_i64(Vec::<Val>::new(env).into_val(env));
        let d = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg1)), &Symbol::from_val(env, &val_from_i64(balance)), Vec::<Val>::from_val(env, &val_from_i64(c))));
        Self::decode_i128_parts(env, a, d);
        if mload32!(a as usize) == 1 {
            Self::call_unreachable(env);
            unreachable!();
        }
        mload64!(a.wrapping_add(24) as usize);
    }


















    fn unreachable_stub(
        env: &Env,
    ) {
    }

    fn func108(
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

    fn func109(
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
        b = d.wrapping_mul(b);
        e = (arg1 as u64).wrapping_shr(32 as u32) as i64;
        a = b.wrapping_add(a.wrapping_mul(e));
    }

    fn func110(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        mut arg3: i64,
        mut arg4: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;
        let mut i: i64 = 0;
        a = -176;
        b = 0;
        {
            c = ((if (arg4 != 0) as i32 != 0 { arg4.leading_zeros() as i64 } else { (arg3.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */) })) as i32;
            d = ((if (arg2 != 0) as i32 != 0 { arg2.leading_zeros() as i64 } else { (arg1.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */) })) as i32;
            if c as u32 > d as u32 {
                e = 96.wrapping_sub(c);
                Self::func108(
                    env,
                    a.wrapping_add(160),
                    arg3,
                    arg4,
                    e,
                );
                let k = mload32!(a.wrapping_add(160) as usize) as i64;
                f = k += 1;
                g = 0;
                b = 0;
                'label6: {
                    {
                        'label8: {
                            loop {
                                d = 64.wrapping_sub(d);
                                Self::func108(
                                    env,
                                    a.wrapping_add(144),
                                    arg1,
                                    arg2,
                                    d,
                                );
                                h = mload64!(a.wrapping_add(144) as usize);
                                if d as u32 < e as u32 {
                                    Self::func108(
                                        env,
                                        a.wrapping_add(80),
                                        arg3,
                                        arg4,
                                        d,
                                    );
                                    {
                                        let loaded_val = mload64!(a.wrapping_add(80) as usize);
                                        f = loaded_val;
                                        if f == 0 {
                                            unreachable!();
                                        }
                                    }
                                    h = (h as u64 / f as u64) as i64;
                                    Self::func109(
                                        env,
                                        a.wrapping_add(64),
                                        arg3,
                                        arg4,
                                        h,
                                        0,
                                    );
                                    {
                                        i = mload64!(a.wrapping_add(64) as usize);
                                        d = ((arg1 as u64) < i as u64) as i32;
                                        f = mload64!(a.wrapping_add(72) as usize);
                                        let cond_val = if arg2 == f { d } else { ((arg2 as u64) < f as u64) as i32 };
                                        if cond_val == 0 {
                                            arg2 = arg2.wrapping_sub(f).wrapping_sub(d as u32 as i64);
                                            arg1 = arg1.wrapping_sub(i);
                                            h = g.wrapping_add(h);
                                            b = b.wrapping_add(((h as u64) < g as u64) as i32 as u32 as i64);
                                        }
                                    }
                                    arg4 = arg1.wrapping_add(arg3);
                                    arg2 = arg2.wrapping_add(arg4).wrapping_add(((arg4 as u64) < arg1 as u64) as i32 as u32 as i64).wrapping_sub(f).wrapping_sub(((arg4 as u64) < i as u64) as i32 as u32 as i64);
                                    arg1 = arg4.wrapping_sub(i);
                                    h = h.wrapping_add(g).wrapping_add(18446744073709551615);
                                    b = b.wrapping_add(((h as u64) < g as u64) as i32 as u32 as i64);
                                }
                                h = (h as u64 / f as u64) as i64;
                                d = d.wrapping_sub(e);
                                Self::func112(
                                    env,
                                    a.wrapping_add(128),
                                    h,
                                    0,
                                    d,
                                );
                                Self::func109(
                                    env,
                                    a.wrapping_add(112),
                                    arg3,
                                    arg4,
                                    h,
                                    0,
                                );
                                Self::func112(
                                    env,
                                    a.wrapping_add(96),
                                    mload64!(a.wrapping_add(112) as usize),
                                    mload64!(a.wrapping_add(120) as usize),
                                    d,
                                );
                                b = mload64!(a.wrapping_add(128) as usize);
                                g = b.wrapping_add(g);
                                b = mload64!(a.wrapping_add(136) as usize).wrapping_add(b).wrapping_add(((g as u64) < b as u64) as i32 as u32 as i64);
                                h = mload64!(a.wrapping_add(96) as usize);
                                let a_part_104 = mload64!(a.wrapping_add(104) as usize);
                                arg2 = arg2.wrapping_sub(a_part_104).wrapping_sub(((arg1 as u64) < h as u64) as i32 as u32 as i64);
                                arg1 = arg1.wrapping_sub(h);
                                d = ((if (arg2 != 0) as i32 != 0 { arg2.leading_zeros() as i64 } else { (arg1.leading_zeros() as i64).wrapping_add(64 /* U64(obj#0) */) })) as i32;
                                if c as u32 <= d as u32 {
                                    break 'label8;
                                }
                            }
                            if arg3 != 0 {
                                unreachable!();
                            }
                            break 'label6;
                        }
                        d = ((arg1 as u64) < arg3 as u64) as i32;
                        let cond_val_2 = if arg2 == arg4 { d } else { ((arg2 as u64) < arg4 as u64) as i32 };
                        if cond_val_2 == 0 {
                            unreachable!();
                        }
                        h = g;
                        break 'label6;
                    }
                    arg2 = (arg1 as u64 / arg3 as u64) as i64;
                }
                arg1 = (arg1 as u64).wrapping_rem(arg3 as u64) as i64;
                h = g.wrapping_add(arg2);
                b = b.wrapping_add(((h as u64) < g as u64) as i32 as u32 as i64);
                arg2 = 0;
                arg2 = arg2.wrapping_sub(arg4).wrapping_sub(d as u32 as i64);
                arg1 = arg1.wrapping_sub(arg3);
                h = g += 1;
                b = b.wrapping_add((h == 0) as i32 as u32 as i64);
            }
            if (arg2 == arg4) as i32 != 0 {
                d = (arg1 as u64 >= arg3 as u64) as i32;
            } else {
                d = (arg2 as u64 >= arg4 as u64) as i32;
            }
            if d != 0 {
                arg4 = arg3;
            } else {
                arg4 = 0;
            }
            arg2 = arg2.wrapping_sub((if d != 0 { arg4 } else { 0 })).wrapping_sub(((arg1 as u64) < arg4 as u64) as i32 as u32 as i64);
            arg1 = arg1.wrapping_sub(arg4);
            h = (arg1 as u64 / arg3 as u64) as i64;
            arg1 = arg1.wrapping_sub(h.wrapping_mul(arg3));
            b = 0;
            arg2 = 0;
            arg4 = arg3 & 4294967295;
            b = (arg2 as u64 / arg4 as u64) as i64;
            h = (arg1 as u64).wrapping_shr(32 as u32) as i64;
            arg2 = ((arg2.wrapping_sub(b.wrapping_mul(arg3)).wrapping_shl(32 as u32) | h) as u64 / arg4 as u64) as i64;
            arg1 = h.wrapping_sub(arg2.wrapping_mul(arg3)).wrapping_shl(32 as u32) | arg1 & 4294967295;
            arg3 = (arg1 as u64 / arg4 as u64) as i64;
            h = arg2.wrapping_shl(32 as u32) | arg3;
            arg1 = arg1.wrapping_sub(arg3.wrapping_mul(arg4));
            b = (arg2 as u64).wrapping_shr(32 as u32) as i64 | b;
            arg2 = 0;
            d = 64.wrapping_sub(d);
            Self::func108(
                env,
                a.wrapping_add(48),
                arg3,
                arg4,
                d,
            );
            Self::func108(
                env,
                a.wrapping_add(32),
                arg1,
                arg2,
                d,
            );
            let a_part_32 = mload64!(a.wrapping_add(32) as usize);
            let a_part_48 = mload64!(a.wrapping_add(48) as usize);
            h = (a_part_32 as u64 / a_part_48 as u64) as i64;
            Self::func109(
                env,
                a.wrapping_add(16),
                arg3,
                0,
                h,
                0,
            );
            Self::func109(
                env,
                a,
                arg4,
                0,
                h,
                0,
            );
            f = mload64!(a.wrapping_add(16) as usize);
            {
                i = mload64!(a.wrapping_add(24) as usize);
                let a_lo = mload64!(a as usize);
                g = i.wrapping_add(a_lo);
                let a_hi = mload64!(a.wrapping_add(8) as usize);
                if a_hi.wrapping_add(((g as u64) < i as u64) as i32 as u32 as i64) == 0 {
                    d = ((arg1 as u64) < f as u64) as i32;
                    let cond_val_3 = if arg2 == g { d } else { ((arg2 as u64) < g as u64) as i32 };
                    if cond_val_3 == 0 {
                        unreachable!();
                    }
                }
            }
            arg1 = arg3.wrapping_add(arg1);
            arg2 = arg4.wrapping_add(arg2).wrapping_add(((arg1 as u64) < arg3 as u64) as i32 as u32 as i64).wrapping_sub(g).wrapping_sub(((arg1 as u64) < f as u64) as i32 as u32 as i64);
            h = h.wrapping_add(18446744073709551615);
            arg1 = arg1.wrapping_sub(f);
        }
    }

    fn func111(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        arg2: i64,
        mut arg3: i64,
        arg4: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        a = -32;
        b = (arg2 < 0) as i32;
        b = (arg4 < 0) as i32;
        Self::func110(
            env,
            a,
            (if b != 0 { 0.wrapping_sub(arg1) } else { arg1 }),
            (if b != 0 { 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64)) } else { arg2 }),
            (if b != 0 { 0.wrapping_sub(arg3) } else { arg3 }),
            (if b != 0 { 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64)) } else { arg4 }),
        );
        arg3 = mload64!(a.wrapping_add(8) as usize);
        arg1 = mload64!(a as usize);
        b = (arg4 ^ arg2 < 0) as i32;
    }

    fn func112(
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

    fn func113(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        mut arg2: i64,
        mut arg3: i64,
        mut arg4: i64,
        arg5: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        a = -96;
        b = 0;
        c = 0;
        d = 0;
        if arg1 | arg2 != 0 {
            if arg3 | arg4 != 0 {
                d = (arg4 < 0) as i32;
                if d != 0 {
                    b = 0.wrapping_sub(arg3);
                } else {
                    b = arg3;
                }
                e = (arg2 < 0) as i32;
                if e != 0 {
                    c = 0.wrapping_sub(arg1);
                } else {
                    c = arg1;
                }
                if d != 0 {
                    arg3 = 0.wrapping_sub(arg4.wrapping_add((arg3 != 0) as i32 as u32 as i64));
                } else {
                    arg3 = arg4;
                }
                arg4 = arg4 ^ arg2;
                'label1: {
                    if e != 0 {
                        arg2 = 0.wrapping_sub(arg2.wrapping_add((arg1 != 0) as i32 as u32 as i64));
                    } else {
                        arg2 = arg2;
                    }
                    if arg2 != 0 {
                        if arg3 != 0 {
                            Self::func109(
                                env,
                                a.wrapping_add(80),
                                b,
                                arg3,
                                c,
                                arg2,
                            );
                            d = 1;
                            arg1 = mload64!(a.wrapping_add(88) as usize);
                            let loaded_val = mload64!(a.wrapping_add(80) as usize);
                            arg2 = loaded_val;
                            break 'label1;
                        }
                        Self::func109(
                            env,
                            a.wrapping_add(64),
                            b,
                            arg3,
                            c,
                            0,
                        );
                        Self::func109(
                            env,
                            a.wrapping_add(48),
                            b,
                            arg3,
                            arg2,
                            0,
                        );
                        arg2 = mload64!(a.wrapping_add(48) as usize);
                        let a_part_72 = mload64!(a.wrapping_add(72) as usize);
                        arg1 = arg2.wrapping_add(a_part_72);
                        let a_part_56 = mload64!(a.wrapping_add(56) as usize);
                        d = ((arg1 as u64) < arg2 as u64) as i32 | (a_part_56 != 0) as i32;
                        arg2 = mload64!(a.wrapping_add(64) as usize);
                        break 'label1;
                    }
                    if arg3 != 0 {
                        Self::func109(
                            env,
                            a.wrapping_add(32),
                            b,
                            0,
                            c,
                            arg2,
                        );
                        Self::func109(
                            env,
                            a.wrapping_add(16),
                            arg3,
                            0,
                            c,
                            arg2,
                        );
                        arg2 = mload64!(a.wrapping_add(16) as usize);
                        let a_part_40 = mload64!(a.wrapping_add(40) as usize);
                        arg1 = arg2.wrapping_add(a_part_40);
                        let value_hi = mload64!(a.wrapping_add(24) as usize);
                        d = ((arg1 as u64) < arg2 as u64) as i32 | (value_hi != 0) as i32;
                        arg2 = mload64!(a.wrapping_add(32) as usize);
                        break 'label1;
                    }
                    Self::func109(
                        env,
                        a,
                        b,
                        arg3,
                        c,
                        arg2,
                    );
                    arg1 = mload64!(a.wrapping_add(8) as usize);
                    arg2 = mload64!(a as usize);
                }
                e = (arg4 < 0) as i32;
                if e != 0 {
                    c = 0.wrapping_sub(arg2);
                }
                if e != 0 {
                    b = 0.wrapping_sub(arg1.wrapping_add((arg2 != 0) as i32 as u32 as i64));
                }
            }
        }
    }
}