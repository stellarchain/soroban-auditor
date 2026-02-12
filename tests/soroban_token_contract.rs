#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, MuxedAddress, Env, String, Val, FromVal};

#[contract]
pub struct TokenContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
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
        let a: i32 = -32;
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
        env.storage().persistent().set(&val_from_i64(c), &val_from_i64(d));
    }

    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
    ) {
        let amount_val: i32 = -32;
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
    }

    pub fn allowance(
        env: Env,
        mut from: Address,
        spender: Address,
    ) -> i128 {
        let a: i32 = -32;
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
        mut from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let amount_val: i32 = -32;
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
        mut id: Address,
    ) -> i128 {
        let a: i32 = -16;
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
        let amount_val: i32 = -48;
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
        let amount_val: i32 = -48;
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
        let amount_val: i32 = -32;
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
        let amount_val: i32 = -32;
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