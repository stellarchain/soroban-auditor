#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, MuxedAddress, Env, IntoVal, String, Val, FromVal, Vec, Map, Symbol};

#[contract]
pub struct TokenContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}

fn val_to_i64(v: Val) -> i64 {
    (unsafe { core::mem::transmute::<Val, u64>(v) }) as i64
}

fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contractevent(topics = ["set_admin",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SetAdmin { #[topic] pub admin: Address, pub new_admin: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceDataKey { pub from: Address, pub spender: Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceValue { pub amount: i128, pub expiration_ledger: u32, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Allowance(AllowanceDataKey), Balance(Address), State(Address), Admin, }
#[soroban_sdk::contractevent(topics = ["approve",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Approve { #[topic] pub from: Address, #[topic] pub spender: Address, pub amount: i128, pub expiration_ledger: u32, }
#[soroban_sdk::contractevent(topics = ["transfer",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TransferWithAmountOnly { #[topic] pub from: Address, #[topic] pub to: Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["transfer",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Transfer { #[topic] pub from: Address, #[topic] pub to: Address, pub to_muxed_id: Option<u64>, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["burn",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Burn { #[topic] pub from: Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["mint",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MintWithAmountOnly { #[topic] pub to: Address, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["mint",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mint { #[topic] pub to: Address, pub to_muxed_id: Option<u64>, pub amount: i128, }
#[soroban_sdk::contractevent(topics = ["clawback",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clawback { #[topic] pub from: Address, pub amount: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokenMetadata { pub decimal: u32, pub name: String, pub symbol: String, }

#[contractimpl]
impl TokenContract {

    pub fn __constructor(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) {
        let mut a: i32 = 0;
        a = -32;
        if decimal as u64 >= 81604378624 as u64 {
            unreachable!();
        }
        env.storage().put_contract_data(admin);
        let c = Self::metadata_const(env);
        let d = Self::map_new_val(
            env,
            1048772,
            3,
            a.wrapping_add(8),
            3,
        );
        let e = env.storage().persistent().set(&val_from_i64(c), &val_from_i64(d));
    }

    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -32;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        let authorized_addr = env.storage().get_contract_data();
        address_from_i64(&env, authorized_addr).require_auth();
        Self::bump_instance_ttl(env);
        Self::credit_balance(
            env,
            to,
            amount_lo,
            amount_hi,
        );
        let c = Self::event_topic_pair(env, 1048744, to);
        let d = Self::i128_parts_to_val(env, amount_lo, amount_hi);
        env.events().publish(val_from_i64(c), val_from_i64(d));
    }

    pub fn set_admin(
        env: Env,
        new_admin: Address,
    ) {
        let mut authorized_addr: i64 = 0;
        let a = env.storage().get_contract_data();
        authorized_addr = a;
        address_from_i64(&env, authorized_addr).require_auth();
        Self::bump_instance_ttl(env);
        env.storage().put_contract_data(new_admin);
        let b = Self::event_topic_pair(env, 1048576, authorized_addr);
        env.events().publish(val_from_i64(b), val_from_i64(new_admin));
        0 /* Void */
    }

    pub fn allowance(
        env: Env,
        from: Address,
        spender: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        a = -32;
        {
            Self::bump_instance_ttl(env);
            env.storage().get_contract_data(a, from, spender);
            let a_lo = mload64!(a as usize);
            let a_hi = mload64!(a.wrapping_add(8) as usize);
            let c = Self::i128_parts_to_val(env, a_lo, a_hi);
            from = c;
            return from;
        }
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -32;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        from.require_auth();
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        Self::bump_instance_ttl(env);
        env.storage().put_contract_data(from, spender, amount_lo, amount_hi, (expiration_ledger as u64).wrapping_shr(32 as u32) as i64 as i32);
        let c = Self::event_topic_triplet(env, amount_val);
        from = c;
        let d = Self::i128_parts_to_val(env, amount_lo, amount_hi);
        Self::vec_pair_builder(
            env,
            amount_val,
            d,
            expiration_ledger & 4294967295,
        );
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_val_hi = mload64!(amount_val.wrapping_add(8) as usize);
        env.events().publish(val_from_i64(from), val_from_i64(amount_val_hi));
    }

    pub fn balance(
        env: Env,
        id: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        a = -16;
        Self::bump_instance_ttl(env);
        env.storage().get_contract_data(a, id);
        let a_lo = mload64!(a as usize);
        let a_hi = mload64!(a.wrapping_add(8) as usize);
        let c = Self::i128_parts_to_val(env, a_lo, a_hi);
        id = c;
        id
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to_muxed: MuxedAddress,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -48;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        from.require_auth();
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        Self::bump_instance_ttl(env);
        Self::debit_balance(
            env,
            from,
            amount_lo,
            amount_hi,
        );
        Self::credit_balance(
            env,
            to_muxed,
            amount_lo,
            amount_hi,
        );
        Self::publish_transfer_event(env, amount_val);
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -48;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        spender.require_auth();
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        Self::bump_instance_ttl(env);
        Self::spend_allowance(
            env,
            from,
            spender,
            amount_lo,
            amount_hi,
        );
        Self::debit_balance(
            env,
            from,
            amount_lo,
            amount_hi,
        );
        Self::credit_balance(
            env,
            to,
            amount_lo,
            amount_hi,
        );
        Self::publish_transfer_event(env, amount_val);
    }

    pub fn burn(
        env: Env,
        from: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -32;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        from.require_auth();
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        Self::bump_instance_ttl(env);
        Self::debit_balance(
            env,
            from,
            amount_lo,
            amount_hi,
        );
        Self::publish_burn_event(
            env,
            amount_lo,
            amount_hi,
            from,
        );
    }

    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        amount_val = -32;
        Self::decode_i128_parts(env, amount_val, amount);
        if mload32!(amount_val as usize) != 0 {
            unreachable!();
        }
        let amount_lo = mload64!(amount_val.wrapping_add(16) as usize);
        let amount_hi = mload64!(amount_val.wrapping_add(24) as usize);
        spender.require_auth();
        Self::require_nonnegative_i128(env, amount_lo, amount_hi);
        Self::bump_instance_ttl(env);
        Self::spend_allowance(
            env,
            from,
            spender,
            amount_lo,
            amount_hi,
        );
        Self::debit_balance(
            env,
            from,
            amount_lo,
            amount_hi,
        );
        Self::publish_burn_event(
            env,
            amount_lo,
            amount_hi,
            from,
        );
    }

    pub fn decimals(
        env: Env,
    ) -> u32 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -32;
        env.storage().get_contract_data(a);
        if mload32!(a as usize) == 0 {
            unreachable!();
        }
        b = mload32!(a.wrapping_add(24) as usize) as i64;
        b.wrapping_shl(32 as u32) | 0
    }

    pub fn name(
        env: Env,
    ) -> String {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -32;
        env.storage().get_contract_data(a);
        if mload32!(a as usize) == 0 {
            unreachable!();
        }
        b = mload64!(a.wrapping_add(8) as usize);
        b
    }

    pub fn symbol(
        env: Env,
    ) -> String {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -32;
        env.storage().get_contract_data(a);
        if mload32!(a as usize) == 0 {
            unreachable!();
        }
        b = mload64!(a.wrapping_add(16) as usize);
        b
    }
}

impl TokenContract {

    fn func19(
        env: &Env,
        arg0: i32,
    ) {
        Self::func20(
            env,
            arg0,
            1 /* True */,
            501120,
            518400,
        );
    }

    fn func20(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i32,
        arg3: i32,
    ) {
        let a = Self::func21(env, arg0);
        let b = match arg1 { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(a), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(a), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, _ => { env.storage().instance().extend_ttl((arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 } }
    }

    fn func21(
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -16;
        'label0: {
            {
                'label2: {
                    {
                        {
                            {
                                Self::func26(
                                    env,
                                    a,
                                    1048648,
                                    9,
                                );
                                if mload32!(a as usize) != 0 {
                                    unreachable!();
                                }
                                let mut sv1_8_i64 = mload64!(a.wrapping_add(8) as usize);
                                b = sv1_8_i64;
                                let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
                                sv1_8_i64 = arg0_lo as i64;
                                let mut vec_builder = mload64!(arg0.wrapping_add(8) as usize);
                                let d = Self::map_new_val(
                                    env,
                                    1048596,
                                    2,
                                    a,
                                    2,
                                );
                                Self::vec_pair_builder(
                                    env,
                                    a,
                                    b,
                                    d,
                                );
                                break 'label2;
                            }
                            Self::func26(
                                env,
                                a,
                                1048657,
                                7,
                            );
                            if mload32!(a as usize) != 0 {
                                unreachable!();
                            }
                            let mut sv1_8_i64 = mload64!(a.wrapping_add(8) as usize);
                            Self::vec_pair_builder(
                                env,
                                a,
                                sv1_8_i64,
                                mload64!(arg0.wrapping_add(8) as usize),
                            );
                            break 'label2;
                        }
                        Self::func26(
                            env,
                            a,
                            1048664,
                            5,
                        );
                        if mload32!(a as usize) != 0 {
                            unreachable!();
                        }
                        let mut sv1_8_i64 = mload64!(a.wrapping_add(8) as usize);
                        Self::vec_pair_builder(
                            env,
                            a,
                            sv1_8_i64,
                            mload64!(arg0.wrapping_add(8) as usize),
                        );
                        break 'label2;
                    }
                    Self::func26(
                        env,
                        a,
                        1048669,
                        5,
                    );
                    if mload32!(a as usize) != 0 {
                        unreachable!();
                    }
                    let sv1_8_i64 = mload64!(a.wrapping_add(8) as usize);
                    let mut vec_builder = sv1_8_i64 as i64;
                    let e = val_to_i64(vec![&env, val_from_i64(vec_builder), val_from_i64(sv1_8_i64)].into_val(env));
                    b = e;
                    break 'label0;
                }
                b = mload64!(a.wrapping_add(8) as usize);
                let vec_builder = mload64!(a as usize);
                if vec_builder == 0 {
                    unreachable!();
                }
            }
        }
        b
    }


    fn metadata_const(
        env: &Env,
    ) -> i64 {
        METADATA
    }

    fn call_eq_one(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) -> i32 {
        let a = match arg1 { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (a == 1 /* True */) as i32
    }

    fn func25(
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

    fn func26(
        env: &Env,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut f: i32 = 0;
        a = -16;
        b = 0 /* False */;
        c = arg2;
        d = arg1;
        'label0: {
            {
                loop {
                    if c == 0 {
                        unreachable!();
                    }
                    e = 1;
                    'label3: {
                        'label4: {
                            let h = mload8!(d as usize) as i32;
                            f = h;
                            if f != 95 {
                                if ((f.wrapping_add(-48) & 255) as u32) >= 10 as u32 {
                                    if ((f.wrapping_add(-65) & 255) as u32) >= 26 as u32 {
                                        if (f.wrapping_add(-97) & 255) as u32 > 25 as u32 {
                                            break 'label3;
                                        }
                                        e = f.wrapping_add(-59);
                                        break 'label4;
                                    }
                                    e = f.wrapping_add(-53);
                                    break 'label4;
                                }
                                e = f.wrapping_add(-46);
                            }
                        }
                        b = b.wrapping_shl(0 as u32) | e as u32 as i64 & 255;
                        c = c.wrapping_add(-1);
                        d += 1;
                    }
                }
                let i = val_to_i64(Symbol::new(env, ""));
                b = i;
                break 'label0;
            }
            b = b.wrapping_shl(0 as u32) | 0 /* Symbol() */;
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

    fn vec_pair_builder(
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let mut a: i32 = 0;
        a = -16;
        let vec_builder = arg2 as i64;
        let sv3_0_i64 = arg1 as i64;
        let c = val_to_i64(vec![&env, val_from_i64(sv3_0_i64), val_from_i64(vec_builder)].into_val(env));
    }



    fn call_unreachable(
        env: &Env,
    ) {
        Self::call_unreachable_2(env);
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
                    let mut svarg0_16_i64 = arg1.wrapping_shr(0 as u32) as i64;
                } else {
                    let c = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64);
                    let d = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64);
                    arg1 = d;
                    let svarg0_24_i64 = c as i64;
                    let svarg0_16_i64 = arg1 as i64;
                }
                break 'label0;
            }
            let error_code = Error(Value, UnexpectedType) as i64;
        }
    }

    fn ledger_sequence_u32(
        env: &Env,
    ) -> i32 {
        let a = env.ledger().sequence() as i64;
        (a as u64).wrapping_shr(32 as u32) as i64 as i32
    }


    fn unreachable_stub(
        env: &Env,
    ) {
    }

    fn pack_i128_val(
        env: &Env,
        arg0: i32,
        mut arg1: i64,
        arg2: i64,
    ) {
        'label0: {
            if arg1.wrapping_add(36028797018963968) as u64 <= 72057594037927935 as u64 {
                if arg1 ^ arg1 | arg2 ^ arg1.wrapping_shr(63 as u32) == 0 /* False */ {
                    arg1 = arg1.wrapping_shl(0 as u32) | 0;
                    break 'label0;
                }
            }
            let a = val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)));
        }
    }

    fn spend_allowance(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
        arg3: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = -32;
        env.storage().get_contract_data(a, arg0, arg1);
        {
            b = mload64!(a as usize);
            c = ((b as u64) < arg2 as u64) as i32;
            d = mload64!(a.wrapping_add(8) as usize);
            let cond_val = if d == arg3 { c } else { (d < arg3) as i32 };
            if cond_val == 0 {
                {
                    let cond_val_2 = if arg3 == 0 { (arg2 == 0 /* False */) as i32 } else { (arg3 > 0 /* False */) as i32 };
                    if cond_val_2 == 0 {
                        env.storage().put_contract_data(arg0, arg1, b.wrapping_sub(arg2), d.wrapping_sub(arg3).wrapping_sub(c as u32 as i64), mload32!(a.wrapping_add(16) as usize));
                    }
                }
            }
        }
        Self::unreachable_stub(env);
    }



    fn i128_parts_to_val(
        env: &Env,
        arg0: i64,
        mut arg1: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        a = -16;
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

    fn credit_balance(
        env: &Env,
        arg0: i64,
        mut arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -16;
        env.storage().get_contract_data(a, arg0);
        {
            b = mload64!(a.wrapping_add(8) as usize);
            let d = arg2;
            arg2 = mload64!(a as usize);
            arg1 = arg2.wrapping_add(arg1);
            arg2 = b.wrapping_add(arg2).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64);
            if (b ^ d ^ 18446744073709551615) & (b ^ arg2) >= 0 /* False */ {
                env.storage().put_contract_data(arg0, arg1, arg2);
            }
        }
        Self::call_unreachable_2(env);
    }

    fn call_unreachable_2(
        env: &Env,
    ) {
        Self::unreachable_stub(env);
    }

    fn debit_balance(
        env: &Env,
        arg0: i64,
        arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = -16;
        env.storage().get_contract_data(a, arg0);
        'label0: {
            {
                b = mload64!(a as usize);
                c = ((b as u64) < arg1 as u64) as i32;
                d = mload64!(a.wrapping_add(8) as usize);
                let cond_val = if d == arg2 { c } else { (d < arg2) as i32 };
                if cond_val == 0 {
                    let f = arg2;
                    arg2 = d.wrapping_sub(arg2).wrapping_sub(c as u32 as i64);
                    if (d ^ f) & (d ^ arg2) >= 0 /* False */ {
                        break 'label0;
                    }
                    Self::call_unreachable_2(env);
                    unreachable!();
                }
            }
            unreachable!();
        }
        env.storage().put_contract_data(arg0, b.wrapping_sub(arg1), arg2);
    }

    fn require_nonnegative_i128(
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) {
        Self::unreachable_stub(env);
    }



    fn bump_instance_ttl(
        env: &Env,
    ) {
        env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD as u32, INSTANCE_BUMP_AMOUNT as u32);
    }

    fn event_topic_pair(
        env: &Env,
        mut arg0: i32,
        arg1: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        a = -32;
        arg0 = 0;
        let c: i64;
        loop {
            mstore64!(a.wrapping_add(16).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            unreachable!();
        }
    }




    fn event_topic_triplet(
        env: &Env,
        mut arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        a = -48;
        let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
        arg0 = 0;
        let d: i64;
        loop {
            mstore64!(a.wrapping_add(24).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            unreachable!();
        }
    }



    fn publish_transfer_event(
        env: &Env,
        arg0: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = -32;
        b = mload64!(arg0.wrapping_add(32) as usize);
        let mut sv1_8_i64 = b as i64;
        let f = Self::event_topic_triplet(env, a.wrapping_add(8));
        let arg0_lo = mload64!(arg0.wrapping_add(16) as usize);
        let arg0_hi = mload64!(arg0.wrapping_add(24) as usize);
        let g = Self::i128_parts_to_val(env, arg0_lo, arg0_hi);
        b = 0 /* Void */;
        'label0: {
            if mload32!(arg0 as usize) == 1 {
                b = mload64!(arg0.wrapping_add(8) as usize);
                if b as u64 <= 72057594037927935 as u64 {
                    b = b.wrapping_shl(0 as u32) | 0;
                    break 'label0;
                }
                let h = val_to_i64(Val::from_u64(b as u64));
            }
        }
        let sv1_8_i64 = g as i64;
        let i = Self::map_new_val(
            env,
            1048716,
            2,
            a.wrapping_add(8),
            2,
        );
        env.events().publish(val_from_i64(f), val_from_i64(i));
    }



    fn publish_burn_event(
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
    ) {
        let a = Self::event_topic_pair(env, 1048736, arg2);
        let b = Self::i128_parts_to_val(env, arg0, arg1);
        env.events().publish(val_from_i64(a), val_from_i64(b));
    }




}