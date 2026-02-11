#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, MuxedAddress, Env, IntoVal, String, TryFromVal, Val, FromVal};

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
        &mut self,
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        if decimal as u64 >= 81604378624 as u64 {
            unreachable!();
        }
        env.storage().put_contract_data(admin);
        let c = self.metadata_const(env);
        let d = self.map_new_val(
            env,
            1048772,
            3,
            a.wrapping_add(8),
            3,
        );
        env.storage().persistent().set(&val_from_i64(c), &val_from_i64(d));
        self.global0 = a.wrapping_add(32);
    }

    pub fn mint(
        &mut self,
        env: Env,
        to: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        amount_val = self.global0.wrapping_sub(32);
        self.global0 = amount_val;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv2_0_i32 = mload32!(amount_val as usize) as i32;
        if sv2_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 16) as i64;
        a = mload64!(amount_val as usize + 24) as i64;
        self.require_nonnegative_i128(env, amount, a);
        let authorized_addr = env.storage().get_contract_data();
        address_from_i64(env, authorized_addr).require_auth();
        self.bump_instance_ttl(env);
        self.credit_balance(
            env,
            to,
            amount,
            a,
        );
        let c = self.event_topic_pair(env, 1048744, to);
        let d = self.i128_parts_to_val(env, amount, a);
        env.events().publish(val_from_i64(c), val_from_i64(d));
        self.global0 = amount_val.wrapping_add(32);
    }

    pub fn set_admin(
        &mut self,
        env: Env,
        new_admin: Address,
    ) {
        let mut authorized_addr: i64 = 0;
        let a = env.storage().get_contract_data();
        authorized_addr = a;
        address_from_i64(env, authorized_addr).require_auth();
        self.bump_instance_ttl(env);
        env.storage().put_contract_data(new_admin);
        let b = self.event_topic_pair(env, 1048576, authorized_addr);
        env.events().publish(val_from_i64(b), val_from_i64(new_admin));
        0 /* Void */
    }

    pub fn allowance(
        &mut self,
        env: Env,
        from: Address,
        spender: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        {
            self.bump_instance_ttl(env);
            env.storage().get_contract_data(a, from, spender);
            let mut sv2_0_i64 = mload64!(a as usize) as i64;
            let mut sv2_8_i64 = mload64!(a as usize + 8) as i64;
            let c = self.i128_parts_to_val(env, sv2_0_i64, sv2_8_i64);
            from = c;
            self.global0 = a.wrapping_add(32);
            return from;
        }
    }

    pub fn approve(
        &mut self,
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        amount_val = self.global0.wrapping_sub(32);
        self.global0 = amount_val;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv4_0_i32 = mload32!(amount_val as usize) as i32;
        if sv4_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 24) as i64;
        a = mload64!(amount_val as usize + 16) as i64;
        address_from_i64(env, from).require_auth();
        self.require_nonnegative_i128(env, a, amount);
        self.bump_instance_ttl(env);
        env.storage().put_contract_data(from, spender, a, amount, (expiration_ledger as u64).wrapping_shr(32 as u32) as i64 as i32);
        let mut amount_lo = spender as i64;
        let c = self.event_topic_triplet(env, amount_val);
        from = c;
        let d = self.i128_parts_to_val(env, a, amount);
        self.vec_pair_builder(
            env,
            amount_val,
            d,
            expiration_ledger & 4294967295,
        );
        let mut sv4_0_i32 = mload32!(amount_val as usize) as i32;
        if sv4_0_i32 != 0 {
            unreachable!();
        }
        let mut sv4_8_i64 = mload64!(amount_val as usize + 8) as i64;
        env.events().publish(val_from_i64(from), val_from_i64(sv4_8_i64));
        self.global0 = amount_val.wrapping_add(32);
    }

    pub fn balance(
        &mut self,
        env: Env,
        id: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.bump_instance_ttl(env);
        env.storage().get_contract_data(a, id);
        let mut sv1_0_i64 = mload64!(a as usize) as i64;
        let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
        let c = self.i128_parts_to_val(env, sv1_0_i64, sv1_8_i64);
        id = c;
        self.global0 = a.wrapping_add(16);
        id
    }

    pub fn transfer(
        &mut self,
        env: Env,
        from: Address,
        mut to_muxed: MuxedAddress,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        amount_val = self.global0.wrapping_sub(48);
        self.global0 = amount_val;
        a = 0;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv3_0_i32 = mload32!(amount_val as usize) as i32;
        if sv3_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 24) as i64;
        b = mload64!(amount_val as usize + 16) as i64;
        address_from_i64(env, from).require_auth();
        self.require_nonnegative_i128(env, b, amount);
        self.bump_instance_ttl(env);
        self.debit_balance(
            env,
            from,
            b,
            amount,
        );
        c = to_muxed;
        if a != 0 {
            let f = val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64(to_muxed)).unwrap().address().into_val(env));
            c = f;
        }
        self.credit_balance(
            env,
            c,
            b,
            amount,
        );
        'label4: {
            if a == 0 {
                to_muxed = 0 /* False */;
            } else {
                {
                    let g = val_to_i64(MuxedAddress::try_from_val(env, &val_from_i64(to_muxed)).unwrap().id().into_val(env));
                    d = g;
                    a = d as i32 & 255;
                    if a != 6 {
                        if a == 64 {
                            let h = val_from_i64(d).as_u64().unwrap_or(0) as i64;
                            d = h;
                            break 'label4;
                        }
                        self.call_unreachable_2(env);
                        unreachable!();
                    }
                }
                d = (d as u64).wrapping_shr(0 as u32) as i64;
            }
        }
        let mut amount_lo = b as i64;
        let mut amount_hi = amount as i64;
        self.publish_transfer_event(env, amount_val);
        self.global0 = amount_val.wrapping_add(48);
    }

    pub fn transfer_from(
        &mut self,
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        amount_val = self.global0.wrapping_sub(48);
        self.global0 = amount_val;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv4_0_i32 = mload32!(amount_val as usize) as i32;
        if sv4_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 24) as i64;
        a = mload64!(amount_val as usize + 16) as i64;
        address_from_i64(env, spender).require_auth();
        self.require_nonnegative_i128(env, a, amount);
        self.bump_instance_ttl(env);
        self.spend_allowance(
            env,
            from,
            spender,
            a,
            amount,
        );
        self.debit_balance(
            env,
            from,
            a,
            amount,
        );
        self.credit_balance(
            env,
            to,
            a,
            amount,
        );
        let mut amount_hi = amount as i64;
        let mut amount_lo = a as i64;
        self.publish_transfer_event(env, amount_val);
        self.global0 = amount_val.wrapping_add(48);
    }

    pub fn burn(
        &mut self,
        env: Env,
        from: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        amount_val = self.global0.wrapping_sub(32);
        self.global0 = amount_val;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv2_0_i32 = mload32!(amount_val as usize) as i32;
        if sv2_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 24) as i64;
        a = mload64!(amount_val as usize + 16) as i64;
        address_from_i64(env, from).require_auth();
        self.require_nonnegative_i128(env, a, amount);
        self.bump_instance_ttl(env);
        self.debit_balance(
            env,
            from,
            a,
            amount,
        );
        self.publish_burn_event(
            env,
            a,
            amount,
            from,
        );
        self.global0 = amount_val.wrapping_add(32);
    }

    pub fn burn_from(
        &mut self,
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) {
        let mut amount_val: i32 = 0;
        let mut a: i64 = 0;
        amount_val = self.global0.wrapping_sub(32);
        self.global0 = amount_val;
        self.decode_i128_parts(env, amount_val, amount);
        let mut sv3_0_i32 = mload32!(amount_val as usize) as i32;
        if sv3_0_i32 != 0 {
            unreachable!();
        }
        amount = mload64!(amount_val as usize + 24) as i64;
        a = mload64!(amount_val as usize + 16) as i64;
        address_from_i64(env, spender).require_auth();
        self.require_nonnegative_i128(env, a, amount);
        self.bump_instance_ttl(env);
        self.spend_allowance(
            env,
            from,
            spender,
            a,
            amount,
        );
        self.debit_balance(
            env,
            from,
            a,
            amount,
        );
        self.publish_burn_event(
            env,
            a,
            amount,
            from,
        );
        self.global0 = amount_val.wrapping_add(32);
    }

    pub fn decimals(
        &mut self,
        env: Env,
    ) -> u32 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        env.storage().get_contract_data(a);
        let mut sv0_0_i32 = mload32!(a as usize) as i32;
        if sv0_0_i32 == 0 {
            unreachable!();
        }
        b = mload32!(a as usize + 24) as i64;
        self.global0 = a.wrapping_add(32);
        b.wrapping_shl(32 as u32) | 0
    }

    pub fn name(
        &mut self,
        env: Env,
    ) -> String {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        env.storage().get_contract_data(a);
        let mut sv0_0_i32 = mload32!(a as usize) as i32;
        if sv0_0_i32 == 0 {
            unreachable!();
        }
        b = mload64!(a as usize + 8) as i64;
        self.global0 = a.wrapping_add(32);
        b
    }

    pub fn symbol(
        &mut self,
        env: Env,
    ) -> String {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        env.storage().get_contract_data(a);
        let mut sv0_0_i32 = mload32!(a as usize) as i32;
        if sv0_0_i32 == 0 {
            unreachable!();
        }
        b = mload64!(a as usize + 16) as i64;
        self.global0 = a.wrapping_add(32);
        b
    }
}