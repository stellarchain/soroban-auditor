#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, MuxedAddress, Env, IntoVal, String, TryFromVal, Val, FromVal, Vec, Map, Symbol};

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
        if !(Address::try_from_val(env, &val_from_i64(admin)).is_ok()) {
            unreachable!();
        }
        if decimal & 255 != 0 {
            unreachable!();
        }
        if !(String::try_from_val(env, &val_from_i64(name)).is_ok()) {
            unreachable!();
        }
        if !(String::try_from_val(env, &val_from_i64(symbol)).is_ok()) {
            unreachable!();
        }
        if decimal as u64 < 81604378624 as u64 {
            env.storage().put_contract_data(admin);
            let c = self.metadata_const(env);
            let d = self.map_new_val(
                env,
                1048772,
                3,
                a.wrapping_add(8),
                3,
            );
            let e = env.storage().persistent().set(&val_from_i64(c), &val_from_i64(d));
            self.global0 = a.wrapping_add(32);
        }
        self.unreachable_stub(env);
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
        if Address::try_from_val(env, &val_from_i64(to)).is_ok() {
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
    }

    pub fn set_admin(
        &mut self,
        env: Env,
        new_admin: Address,
    ) {
        let mut authorized_addr: i64 = 0;
        if !(Address::try_from_val(env, &val_from_i64(new_admin)).is_ok()) {
            unreachable!();
        }
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
        if !(Address::try_from_val(env, &val_from_i64(from)).is_ok()) {
            unreachable!();
        }
        if !(Address::try_from_val(env, &val_from_i64(spender)).is_ok()) {
            unreachable!();
        }
        self.bump_instance_ttl(env);
        env.storage().get_contract_data(a, from, spender);
        let mut sv2_0_i64 = mload64!(a as usize) as i64;
        let mut sv2_8_i64 = mload64!(a as usize + 8) as i64;
        let c = self.i128_parts_to_val(env, sv2_0_i64, sv2_8_i64);
        from = c;
        self.global0 = a.wrapping_add(32);
        return from;
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
        if Address::try_from_val(env, &val_from_i64(from)).is_ok() {
            if Address::try_from_val(env, &val_from_i64(spender)).is_ok() {
                self.decode_i128_parts(env, amount_val, amount);
                let mut sv4_0_i32 = mload32!(amount_val as usize) as i32;
                if sv4_0_i32 != 0 {
                    unreachable!();
                }
                if expiration_ledger & 255 == 0 {
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
            }
        }
    }

    pub fn balance(
        &mut self,
        env: Env,
        id: Address,
    ) -> i128 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        if !(Address::try_from_val(env, &val_from_i64(id)).is_ok()) {
            unreachable!();
        }
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
        if Address::try_from_val(env, &val_from_i64(from)).is_ok() {
            a = 1;
            loop {
                match (to_muxed as i32 & 255).wrapping_add(-77) {
                    0 | 1 | _ => break,
                }
            }
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
        if Address::try_from_val(env, &val_from_i64(spender)).is_ok() {
            if Address::try_from_val(env, &val_from_i64(from)).is_ok() {
                if Address::try_from_val(env, &val_from_i64(to)).is_ok() {
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
            }
        }
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
        if Address::try_from_val(env, &val_from_i64(from)).is_ok() {
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
        if Address::try_from_val(env, &val_from_i64(spender)).is_ok() {
            if Address::try_from_val(env, &val_from_i64(from)).is_ok() {
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
        }
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

impl TokenContract {

    fn func19(
        &mut self,
        env: &Env,
        arg0: i32,
    ) {
        self.func20(
            env,
            arg0,
            1 /* True */,
            501120,
            518400,
        );
    }

    fn func20(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i32,
        arg3: i32,
    ) {
        let a = self.func21(env, arg0);
        let b = match arg1 { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(a), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(a), (arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 }, _ => { env.storage().instance().extend_ttl((arg2 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32, (arg3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32); 0 } }
    }

    fn func21(
        &mut self,
        env: &Env,
        arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        'label0: {
            {
                'label2: {
                    {
                        {
                            {
                                {
                                    let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
                                    loop {
                                        match svarg0_0_i32 {
                                            0 | 1 | 2 | 3 | _ => break,
                                        }
                                    }
                                }
                                self.func26(
                                    env,
                                    a,
                                    1048648,
                                    9,
                                );
                                let mut sv1_0_i32 = mload32!(a as usize) as i32;
                                if sv1_0_i32 != 0 {
                                    unreachable!();
                                }
                                let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
                                b = sv1_8_i64;
                                let mut arg0_lo = mload64!(arg0 as usize + 16) as i64;
                                sv1_8_i64 = arg0_lo as i64;
                                let mut svarg0_8_i64 = mload64!(arg0 as usize + 8) as i64;
                                let mut vec_builder = svarg0_8_i64 as i64;
                                let d = self.map_new_val(
                                    env,
                                    1048596,
                                    2,
                                    a,
                                    2,
                                );
                                self.vec_pair_builder(
                                    env,
                                    a,
                                    b,
                                    d,
                                );
                                break 'label2;
                            }
                            self.func26(
                                env,
                                a,
                                1048657,
                                7,
                            );
                            let mut sv1_0_i32 = mload32!(a as usize) as i32;
                            if sv1_0_i32 != 0 {
                                unreachable!();
                            }
                            let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
                            let mut svarg0_8_i64 = mload64!(arg0 as usize + 8) as i64;
                            self.vec_pair_builder(
                                env,
                                a,
                                sv1_8_i64,
                                svarg0_8_i64,
                            );
                            break 'label2;
                        }
                        self.func26(
                            env,
                            a,
                            1048664,
                            5,
                        );
                        let mut sv1_0_i32 = mload32!(a as usize) as i32;
                        if sv1_0_i32 != 0 {
                            unreachable!();
                        }
                        let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
                        let mut svarg0_8_i64 = mload64!(arg0 as usize + 8) as i64;
                        self.vec_pair_builder(
                            env,
                            a,
                            sv1_8_i64,
                            svarg0_8_i64,
                        );
                        break 'label2;
                    }
                    self.func26(
                        env,
                        a,
                        1048669,
                        5,
                    );
                    let mut sv1_0_i32 = mload32!(a as usize) as i32;
                    if sv1_0_i32 != 0 {
                        unreachable!();
                    }
                    let mut sv1_8_i64 = mload64!(a as usize + 8) as i64;
                    let mut vec_builder = sv1_8_i64 as i64;
                    let e = val_to_i64(vec![&env, val_from_i64(vec_builder), val_from_i64(sv1_8_i64)].into_val(env));
                    b = e;
                    break 'label0;
                }
                b = mload64!(a as usize + 8) as i64;
                let mut vec_builder = mload64!(a as usize) as i64;
                if vec_builder == 0 {
                    unreachable!();
                }
            }
        }
        self.global0 = a.wrapping_add(16);
        b
    }


    fn metadata_const(
        &mut self,
        env: &Env,
    ) -> i64 {
        METADATA
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

    fn func25(
        &mut self,
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
        &mut self,
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
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
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
        self.global0 = a.wrapping_add(16);
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

    fn vec_pair_builder(
        &mut self,
        env: &Env,
        arg0: i32,
        arg1: i64,
        arg2: i64,
    ) {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        let mut vec_builder = arg2 as i64;
        let mut sv3_0_i64 = arg1 as i64;
        let c = val_to_i64(vec![&env, val_from_i64(sv3_0_i64), val_from_i64(vec_builder)].into_val(env));
        self.global0 = a.wrapping_add(16);
    }



    fn call_unreachable(
        &mut self,
        env: &Env,
    ) {
        self.call_unreachable_2(env);
    }



    fn decode_i128_parts(
        &mut self,
        env: &Env,
        arg0: i32,
        mut arg1: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
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
                    b = c;
                    let d = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64);
                    arg1 = d;
                    let mut svarg0_24_i64 = b as i64;
                    let mut svarg0_16_i64 = arg1 as i64;
                }
                break 'label0;
            }
            let mut error_code = Error(Value, UnexpectedType) as i64;
        }
    }

    fn ledger_sequence_u32(
        &mut self,
        env: &Env,
    ) -> i32 {
        let a = env.ledger().sequence() as i64;
        (a as u64).wrapping_shr(32 as u32) as i64 as i32
    }


    fn unreachable_stub(
        &mut self,
        env: &Env,
    ) {
    }

    fn pack_i128_val(
        &mut self,
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
        &mut self,
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
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        env.storage().get_contract_data(a, arg0, arg1);
        b = mload64!(a as usize) as i64;
        c = ((b as u64) < arg2 as u64) as i32;
        d = mload64!(a as usize + 8) as i64;
        if ((if d == arg3 { c } else { (d < arg3) as i32 })) == 0 {
            if ((if arg3 == 0 { (arg2 == 0 /* False */) as i32 } else { (arg3 > 0 /* False */) as i32 })) == 0 {
                let mut sv4_16_i32 = mload32!(a as usize + 16) as i32;
                env.storage().put_contract_data(arg0, arg1, b.wrapping_sub(arg2), d.wrapping_sub(arg3).wrapping_sub(c as u32 as i64), sv4_16_i32);
            }
            self.global0 = a.wrapping_add(32);
        }
        self.unreachable_stub(env);
    }



    fn i128_parts_to_val(
        &mut self,
        env: &Env,
        arg0: i64,
        mut arg1: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        self.pack_i128_val(
            env,
            a,
            arg0,
            arg1,
        );
        {
            let mut sv2_0_i32 = mload32!(a as usize) as i32;
            if sv2_0_i32 == 1 {
                unreachable!();
            }
        }
        arg1 = mload64!(a as usize + 8) as i64;
        self.global0 = a.wrapping_add(16);
        arg1
    }

    fn credit_balance(
        &mut self,
        env: &Env,
        arg0: i64,
        mut arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        env.storage().get_contract_data(a, arg0);
        {
            b = mload64!(a as usize + 8) as i64;
            let d = arg2;
            arg2 = mload64!(a as usize) as i64;
            arg1 = arg2.wrapping_add(arg1);
            arg2 = b.wrapping_add(arg2).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64);
            if (b ^ d ^ 18446744073709551615) & (b ^ arg2) >= 0 /* False */ {
                env.storage().put_contract_data(arg0, arg1, arg2);
                self.global0 = a.wrapping_add(16);
            }
        }
        self.call_unreachable_2(env);
    }

    fn call_unreachable_2(
        &mut self,
        env: &Env,
    ) {
        self.unreachable_stub(env);
    }

    fn debit_balance(
        &mut self,
        env: &Env,
        arg0: i64,
        arg1: i64,
        mut arg2: i64,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i32 = 0;
        let mut d: i64 = 0;
        a = self.global0.wrapping_sub(16);
        self.global0 = a;
        env.storage().get_contract_data(a, arg0);
        'label0: {
            b = mload64!(a as usize) as i64;
            c = ((b as u64) < arg1 as u64) as i32;
            d = mload64!(a as usize + 8) as i64;
            if ((if d == arg2 { c } else { (d < arg2) as i32 })) == 0 {
                let f = arg2;
                arg2 = d.wrapping_sub(arg2).wrapping_sub(c as u32 as i64);
                if (d ^ f) & (d ^ arg2) >= 0 /* False */ {
                    break 'label0;
                }
                self.call_unreachable_2(env);
                unreachable!();
            }
            unreachable!();
        }
        env.storage().put_contract_data(arg0, b.wrapping_sub(arg1), arg2);
        self.global0 = a.wrapping_add(16);
    }

    fn require_nonnegative_i128(
        &mut self,
        env: &Env,
        arg0: i64,
        arg1: i64,
    ) {
        self.unreachable_stub(env);
    }



    fn bump_instance_ttl(
        &mut self,
        env: &Env,
    ) {
        env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD as u32, INSTANCE_BUMP_AMOUNT as u32);
    }

    fn event_topic_pair(
        &mut self,
        env: &Env,
        mut arg0: i32,
        mut arg1: i64,
    ) -> i64 {
        let mut a: i32 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        let mut vec_builder = arg1 as i64;
        let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
        let mut sv2_0_i64 = svarg0_0_i64 as i64;
        arg0 = 0;
        let c: i64;
        'label0: loop {
            if arg0 == 16 {
                arg0 = 0;
                while arg0 != 16 {
                    let d = mload64!(a.wrapping_add(arg0) as usize) as i64;
                    mstore64!(a.wrapping_add(16).wrapping_add(arg0) as usize, d as u64);
                    arg0 = arg0.wrapping_add(8);
                }
                let e = val_to_i64(vec![&env, val_from_i64(sv2_0_i64), val_from_i64(vec_builder)].into_val(env));
                arg1 = e;
                self.global0 = a.wrapping_add(32);
                return arg1;
            }
            mstore64!(a.wrapping_add(16).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            continue 'label0;
            unreachable!();
        }
    }




    fn event_topic_triplet(
        &mut self,
        env: &Env,
        mut arg0: i32,
    ) -> i64 {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        a = self.global0.wrapping_sub(48);
        self.global0 = a;
        let mut arg0_lo = mload64!(arg0 as usize + 16) as i64;
        let mut svarg0_0_i64 = mload64!(arg0 as usize) as i64;
        let mut vec_builder = svarg0_0_i64 as i64;
        let mut svarg0_8_i32 = mload32!(arg0 as usize + 8) as i32;
        let mut svsvarg0_8_i32_0_i64 = mload64!(svarg0_8_i32 as usize) as i64;
        let mut sv1_0_i64 = svsvarg0_8_i32_0_i64 as i64;
        arg0 = 0;
        let d: i64;
        'label0: loop {
            if arg0 == 24 {
                arg0 = 0;
                while arg0 != 24 {
                    let e = mload64!(a.wrapping_add(arg0) as usize) as i64;
                    mstore64!(a.wrapping_add(24).wrapping_add(arg0) as usize, e as u64);
                    arg0 = arg0.wrapping_add(8);
                }
                let f = val_to_i64(vec![&env, val_from_i64(sv1_0_i64), val_from_i64(vec_builder)].into_val(env));
                b = f;
                self.global0 = a.wrapping_add(48);
                return b;
            }
            mstore64!(a.wrapping_add(24).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
            arg0 = arg0.wrapping_add(8);
            continue 'label0;
            unreachable!();
        }
    }



    fn publish_transfer_event(
        &mut self,
        env: &Env,
        arg0: i32,
    ) {
        let mut a: i32 = 0;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        a = self.global0.wrapping_sub(32);
        self.global0 = a;
        b = mload64!(arg0 as usize + 32) as i64;
        let mut sv1_8_i64 = b as i64;
        let f = self.event_topic_triplet(env, a.wrapping_add(8));
        c = f;
        let mut arg0_lo = mload64!(arg0 as usize + 16) as i64;
        let mut arg0_hi = mload64!(arg0 as usize + 24) as i64;
        let g = self.i128_parts_to_val(env, arg0_lo, arg0_hi);
        d = g;
        b = 0 /* Void */;
        'label0: {
            let mut svarg0_0_i32 = mload32!(arg0 as usize) as i32;
            if svarg0_0_i32 == 1 {
                b = mload64!(arg0 as usize + 8) as i64;
                if b as u64 <= 72057594037927935 as u64 {
                    b = b.wrapping_shl(0 as u32) | 0;
                    break 'label0;
                }
                let h = val_to_i64(Val::from_u64(b as u64));
            }
        }
        let mut sv1_8_i64 = d as i64;
        let i = self.map_new_val(
            env,
            1048716,
            2,
            a.wrapping_add(8),
            2,
        );
        env.events().publish(val_from_i64(c), val_from_i64(i));
        self.global0 = a.wrapping_add(32);
    }



    fn publish_burn_event(
        &mut self,
        env: &Env,
        arg0: i64,
        arg1: i64,
        arg2: i64,
    ) {
        let a = self.event_topic_pair(env, 1048736, arg2);
        let b = self.i128_parts_to_val(env, arg0, arg1);
        env.events().publish(val_from_i64(a), val_from_i64(b));
    }




}