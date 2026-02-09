#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, String, Val, FromVal, Vec, Map, Bytes, BytesN, Symbol};

#[contract]
pub struct SecondAnotherRandom;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn err_contract(code: u32) -> i64 {
    ((soroban_sdk::xdr::ScErrorType::Contract as u32 as i64) & 255).wrapping_shl(32 as u32) | (code as i64)
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Admin, Minter, Balance(soroban_sdk::Address), Allowance(soroban_sdk::Address, soroban_sdk::Address), TotalSupply, Name, Symbol, Decimals, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokenMetadata { pub decimal: u32, pub name: soroban_sdk::String, pub symbol: soroban_sdk::String, }

macro_rules! mload8 {
    ($addr:expr) => {{
        let Self { memory, .. } = self;
        memory.load8($addr)
    }};
}
macro_rules! mload16 {
    ($addr:expr) => {{
        let Self { memory, .. } = self;
        memory.load16($addr)
    }};
}
macro_rules! mload32 {
    ($addr:expr) => {{
        let Self { memory, .. } = self;
        memory.load32($addr)
    }};
}
macro_rules! mload64 {
    ($addr:expr) => {{
        let Self { memory, .. } = self;
        memory.load64($addr)
    }};
}
macro_rules! mstore8 {
    ($addr:expr, $value:expr) => {{
        let Self { memory, .. } = self;
        memory.store8($addr, $value)
    }};
}
macro_rules! mstore16 {
    ($addr:expr, $value:expr) => {{
        let Self { memory, .. } = self;
        memory.store16($addr, $value)
    }};
}
macro_rules! mstore32 {
    ($addr:expr, $value:expr) => {{
        let Self { memory, .. } = self;
        memory.store32($addr, $value)
    }};
}
macro_rules! mstore64 {
    ($addr:expr, $value:expr) => {{
        let Self { memory, .. } = self;
        memory.store64($addr, $value)
    }};
}
macro_rules! msize {
    () => {{
        let Self { memory, .. } = self;
        memory.size()
    }};
}
macro_rules! mgrow {
    ($pages:expr) => {{
        let Self { memory, .. } = self;
        memory.grow($pages)
    }};
}


#[contractimpl]
impl SecondAnotherRandom {
    pub fn admin(&mut self, env: Env) -> soroban_sdk::Address {
        let var0 = self.func59(env, 1048656);
        var0
    }
    pub fn allowance(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address) -> i128 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        if ((!(Address::try_from_val(env, &val_from_i64(from)).is_ok())) as i32 | (!(Address::try_from_val(env, &val_from_i64(spender)).is_ok())) as i32 == 0) as i32 != 0 {
            self.func28(env);
            self.func19(env, var2, from, spender);
            let mut slot_var2_0_i64 = mload64!(var2 as usize) as i64;
            let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
            let var6 = self.func29(env, slot_var2_0_i64, slot_var2_8_i64);
            self.global0 = var2.wrapping_add(16);
            return var6;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn approve(&mut self, env: Env, from: soroban_sdk::Address, spender: soroban_sdk::Address, amount: i128, expiration_ledger: u32) {
        from.require_auth_for_args((spender, amount, expiration_ledger).into_val(&env));
    }
    pub fn balance(&mut self, env: Env, id: soroban_sdk::Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }
    pub fn burn(&mut self, env: Env, from: soroban_sdk::Address, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(balance - amount));
    }
    pub fn burn_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, amount: i128) {
        spender.require_auth_for_args((from, amount).into_val(&env));
    }
    pub fn burn_self(&mut self, env: Env, from: soroban_sdk::Address, amount: i128) {
        from.require_auth_for_args((amount).into_val(&env));
    }
    pub fn decimals(&mut self, env: Env) -> u32 {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.decimal
    }
    pub fn initialize(&mut self, env: Env, admin: soroban_sdk::Address, minter: soroban_sdk::Address) {
        if let Some(_) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }
    pub fn mint(&mut self, env: Env, to: soroban_sdk::Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
    }
    pub fn minter(&mut self, env: Env) -> soroban_sdk::Address {
        let var0 = self.func59(env, 1048680);
        var0
    }
    pub fn name(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.name
    }
    pub fn set_minter(&mut self, env: Env, new_minter: soroban_sdk::Address) {
        // TODO: set storage value
    }
    pub fn symbol(&mut self, env: Env) -> soroban_sdk::String {
        let metadata: TokenMetadata = env.storage().instance().get(&DataKey::State(env.current_contract_address())).unwrap();
        metadata.symbol
    }
    pub fn total_supply(&mut self, env: Env) -> i128 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func28(env);
        self.func23(env, var0);
        let mut slot_var0_0_i64 = mload64!(var0 as usize) as i64;
        let mut slot_var0_8_i64 = mload64!(var0 as usize + 8) as i64;
        let var5 = self.func29(env, slot_var0_0_i64, slot_var0_8_i64);
        self.global0 = var0.wrapping_add(16);
        var5
    }
    pub fn transfer(&mut self, env: Env, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        from.require_auth();
        if amount < 0 { panic!("negative amount"); }
        let from_balance: i128 = env.storage().persistent().get(&DataKey::Balance(from)).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&DataKey::Balance(to)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }
    pub fn transfer_from(&mut self, env: Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        spender.require_auth_for_args((from, to, amount).into_val(&env));
    }
}

#[allow(dead_code)]
impl SecondAnotherRandom {
    fn copy_bytes_to_linear_memory(&mut self, env: &Env, b: i64, b_pos: i64, lm_pos: i64, len: i64) {
        let b = Bytes::from_val(env, &val_from_i64(b));
        let start = val_from_i64(b_pos).as_u32().unwrap_or(0);
        let len = val_from_i64(len).as_u32().unwrap_or(0);
        let lm_pos = val_from_i64(lm_pos).as_u32().unwrap_or(0);
        let bytes = b.slice(start..start.saturating_add(len));
        let mut buf: Vec<u8> = vec![0; len as usize];
        bytes.copy_into_slice(&mut buf);
        for (i, b) in buf.iter().enumerate() {
            mstore8!(lm_pos as usize + i, *b as u8);
        }
    }
    fn copy_string_to_linear_memory(&mut self, env: &Env, s: i64, s_pos: i64, lm_pos: i64, len: i64) {
        let s = String::from_val(env, &val_from_i64(s));
        let start = val_from_i64(s_pos).as_u32().unwrap_or(0);
        let len = val_from_i64(len).as_u32().unwrap_or(0);
        let lm_pos = val_from_i64(lm_pos).as_u32().unwrap_or(0);
        let bytes = s.to_bytes().slice(start..start.saturating_add(len));
        let mut buf: Vec<u8> = vec![0; len as usize];
        bytes.copy_into_slice(&mut buf);
        for (i, b) in buf.iter().enumerate() {
            mstore8!(lm_pos as usize + i, *b as u8);
        }
    }
    fn for_each_val<F: FnMut(Val)>(&self, env: &Env, vals: &Vec<Val>, mut f: F) {
        let mut i: i64 = 0;
        let len = vals.len() as i64;
        while i < len {
            let val = vals.get_unchecked((i as i64) << 32);
            f(val);
            i = i.wrapping_add(1);
        }
    }
    fn require_len_match(&self, a: &Vec<Val>, b: &Vec<Val>) -> Result<(), i64> {
        let len_a = a.len() as i64;
        let len_b = b.len() as i64;
        if ((len_a ^ len_b) as u64 >= 4294967296 as u64) as i32 != 0 {
            return Err(err_contract(10));
        }
        Ok(())
    }
    fn require_len_match_len(&self, len_a: i64, len_b: i64) -> Result<(), i64> {
        if ((len_a ^ len_b) as u64 >= 4294967296 as u64) as i32 != 0 {
            return Err(err_contract(10));
        }
        Ok(())
    }
    fn vec_new_val(&self, env: &Env) -> i64 {
        val_to_i64(Vec::<Val>::new(env).into_val(env))
    }
    fn vec_push_val(&self, env: &Env, vec_val: i64, val: i64) -> i64 {
        let mut v = Vec::<Val>::from_val(env, &val_from_i64(vec_val));
        v.push_back(val_from_i64(val));
        val_to_i64(v.into_val(env))
    }
    fn for_each_string<F: FnMut(i64)>(&self, env: &Env, vec_ptr: i64, mut f: F) {
        let mut tmp: i32 = self.global0.wrapping_sub(64);
        self.global0 = tmp;
        let mut slot_tmp_16_i32: i32 = 0;
        let mut slot_tmp_8_i64: i64 = vec_ptr as i64;
        loop {
            let next_ptr = tmp.wrapping_sub(-64);
            self.vec_next_string_flag(env, next_ptr, tmp.wrapping_add(8));
            let mut slot_tmp_64_i64 = mload64!(tmp as usize + 64) as i64;
            let mut slot_tmp_72_i64 = mload64!(tmp as usize + 72) as i64;
            self.guard_nonzero_ptr(env, tmp.wrapping_add(24), slot_tmp_64_i64, slot_tmp_72_i64);
            let mut slot_tmp_24_i32 = mload32!(tmp as usize + 24) as i32;
            if (slot_tmp_24_i32 == 0) as i32 != 0 {
                break;
            }
            let mut slot_tmp_32_i64 = mload64!(tmp as usize + 32) as i64;
            self.storage_get_val(env, next_ptr, slot_tmp_32_i64);
            if (slot_tmp_64_i64 == 1) as i32 != 0 {
                self.check_recent_timestamp(env, next_ptr, tmp.wrapping_add(40));
                let val = mload64!(tmp as usize + 72) as i64;
                f(val);
                if slot_tmp_64_i64 != 0 {
                    break;
                }
                continue;
            }
            break;
        }
        self.global0 = tmp.wrapping_add(64);
    }
    fn for_each_string_checked<F: FnMut(i64)>(&mut self, env: &Env, vec_ptr: i64, mut f: F) -> Result<(), i64> {
        let mut tmp: i32 = self.global0.wrapping_sub(64);
        self.global0 = tmp;
        let mut slot_tmp_16_i32: i32 = 0;
        let mut slot_tmp_8_i64: i64 = vec_ptr as i64;
        loop {
            let next_ptr = tmp.wrapping_sub(-64);
            self.vec_next_string_flag(env, next_ptr, tmp.wrapping_add(8));
            let mut slot_tmp_64_i64 = mload64!(tmp as usize + 64) as i64;
            let mut slot_tmp_72_i64 = mload64!(tmp as usize + 72) as i64;
            self.guard_nonzero_ptr(env, tmp.wrapping_add(24), slot_tmp_64_i64, slot_tmp_72_i64);
            let mut slot_tmp_24_i32 = mload32!(tmp as usize + 24) as i32;
            if (slot_tmp_24_i32 == 0) as i32 != 0 {
                break;
            }
            let mut slot_tmp_32_i64 = mload64!(tmp as usize + 32) as i64;
            self.storage_get_val(env, next_ptr, slot_tmp_32_i64);
            if (slot_tmp_64_i64 == 1) as i32 != 0 {
                self.check_recent_timestamp(env, next_ptr, tmp.wrapping_add(40));
                let val = mload64!(tmp as usize + 72) as i64;
                f(val);
                if slot_tmp_64_i64 != 0 {
                    break;
                }
                continue;
            }
            self.global0 = tmp.wrapping_add(64);
            return Err(Error(Storage, MissingValue));
        }
        self.global0 = tmp.wrapping_add(64);
        Ok(())
    }
    fn next_string_checked(&mut self, env: &Env, base: i32, tmp: i32, iter_ptr: i32) -> (i64, i64, i32) {
        self.vec_next_string_flag(env, tmp, iter_ptr);
        let flag = mload64!(base as usize + 64) as i64;
        let val = mload64!(base as usize + 72) as i64;
        self.guard_nonzero_ptr(env, base.wrapping_add(24), flag, val);
        let ok = mload32!(base as usize + 24) as i32;
        (flag, val, ok)
    }
    fn pack_ok_val(&self, val: i64) -> i64 {
        if (val as u64) <= 72057594037927935 {
            val
        } else {
            val_to_i64(Val::from_u64(val as u64))
        }
    }
    fn zero_24_bytes(&self, base: i32) {
        mstore64!(base as usize, 0);
        mstore64!(base.wrapping_add(8) as usize, 0);
        mstore64!(base.wrapping_add(16) as usize, 0);
    }
}

#[allow(dead_code)]
impl SecondAnotherRandom {
    fn func14(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64);
        self.global0 = var2;
        let mut slot_var2_8_i64 = 0 /* Void */ as i64;
        let mut slot_var2_16_i64 = arg1 as i64;
        self.func15(env, var2.wrapping_add(32), var2.wrapping_add(8));
        let mut slot_var2_32_i32 = mload32!(var2 as usize + 32) as i32;
        if slot_var2_32_i32 & 1 != 0 {
            let mut slot_var2_56_i64 = mload64!(var2 as usize + 56) as i64;
            var3 = slot_var2_56_i64;
            let mut slot_var2_48_i64 = mload64!(var2 as usize + 48) as i64;
            var4 = slot_var2_48_i64;
            self.func16(env, arg1);
        }
        mstore64!(arg0 as usize, var4 as u64);
        mstore64!(arg0 as usize + 8, var3 as u64);
        self.global0 = var2.wrapping_sub(-64);
    }
    fn func15(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        loop {
            let var5 = self.func21(env, arg1);
            var3 = var5;
            let var6 = self.call_eq_one(env, var3, 1 /* True */);
            let var7: i64;
            if var6 != 0 {
                let var8 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                self.func25(env, var2, var8);
                let mut slot_var2_0_i32 = mload32!(var2 as usize) as i32;
                if (slot_var2_0_i32 == 1) as i32 != 0 {
                    break;
                }
                let mut slot_var2_16_i64 = mload64!(var2 as usize + 16) as i64;
                var3 = slot_var2_16_i64;
                let mut slot_var2_24_i64 = mload64!(var2 as usize + 24) as i64;
                mstore64!(arg0 as usize + 24, slot_var2_24_i64 as u64);
                mstore64!(arg0 as usize + 16, var3 as u64);
                var7 = 1 /* True */;
            }
            else {
                var7 = 0 /* False */;
            }
            mstore64!(arg0 as usize, var7 as u64);
            mstore64!(arg0 as usize + 8, 0 /* False */ as u64);
            self.global0 = var2.wrapping_add(32);
            return;
        }
        unreachable!();
    }
    fn func16(&mut self, env: &Env, mut arg0: i64) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        let mut slot_var1_8_i64 = 0 /* Void */ as i64;
        let mut slot_var1_16_i64 = arg0 as i64;
        var2 = var1.wrapping_add(8);
        let var4 = self.func21(env, var2);
        let var5 = self.call_eq_one(env, var4, 1 /* True */);
        if var5 != 0 {
            self.func22(env, var2);
        }
        self.global0 = var1.wrapping_add(32);
    }
    fn func17(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(32);
        self.global0 = var3;
        let mut slot_var3_8_i64 = 0 /* Void */ as i64;
        let mut slot_var3_16_i64 = arg0 as i64;
        self.func18(env, var3.wrapping_add(8), arg1, arg2);
        self.func16(env, arg0);
        self.global0 = var3.wrapping_add(32);
    }
    fn func18(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        self.func27(env, arg0, arg1, arg2, 1 /* True */);
    }
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_add(-64);
        self.global0 = var3;
        let mut slot_var3_24_i64 = arg2 as i64;
        let mut slot_var3_16_i64 = arg1 as i64;
        let mut slot_var3_8_i64 = err_contract(0) as i64;
        self.func15(env, var3.wrapping_add(32), var3.wrapping_add(8));
        let mut slot_var3_48_i64 = mload64!(var3 as usize + 48) as i64;
        arg1 = slot_var3_48_i64;
        let mut slot_var3_56_i64 = mload64!(var3 as usize + 56) as i64;
        let mut slot_var3_32_i32 = mload32!(var3 as usize + 32) as i32;
        var4 = slot_var3_32_i32 & 1;
        mstore64!(arg0 as usize + 8, ({ let a = slot_var3_56_i64; let b = 0 /* False */; if var4 != 0 { a } else { b } }) as u64);
        mstore64!(arg0 as usize, ({ let a = arg1; let b = 0 /* False */; if var4 != 0 { a } else { b } }) as u64);
        self.global0 = var3.wrapping_sub(-64);
    }
    fn func20(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var4 = var6.wrapping_sub(32);
        self.global0 = var4;
        let mut slot_var4_24_i64 = arg1 as i64;
        let mut slot_var4_16_i64 = arg0 as i64;
        let mut slot_var4_8_i64 = err_contract(0) as i64;
        loop {
            if (({ let a = (arg2 != 0 /* False */) as i32; let b = (arg3 > 0 /* False */) as i32; if (arg3 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var7 = self.func21(env, var4.wrapping_add(8));
                let var8 = match 1 /* True */ { 0 => { env.storage().persistent().remove(&val_from_i64(var7)); 0 }, 1 => { env.storage().temporary().remove(&val_from_i64(var7)); 0 }, _ => { env.storage().instance().remove(&val_from_i64(var7)); 0 } }
                var8;
                break;
            }
            var5 = var4.wrapping_add(8);
            self.func18(env, var5, arg2, arg3);
            self.func22(env, var5);
        }
        self.global0 = var4.wrapping_add(32);
    }
    fn func21(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(32);
        self.global0 = var1;
        let mut __exit_label0: i32 = 0;
        loop {
            let mut __exit_label2: i32 = 0;
            'label3: {
                'label4: {
                    'label5: {
                        'label6: {
                            'label7: {
                                'label8: {
                                    'label9: {
                                        {
                                            let var6 = mload32!(arg0 as usize) as i32;
                                            match var6.wrapping_sub(1) {
                                                0 => break,
                                                1 => break,
                                                2 => break,
                                                3 => break,
                                                4 => break,
                                                5 => break,
                                                6 => break,
                                                _ => break,
                                            }
                                        }
                                        arg0 = var1.wrapping_add(8);
                                        self.func32(env, arg0, 1048576, 5);
                                        __exit_label2 = 1; break 'label3;
                                    }
                                    arg0 = var1.wrapping_add(8);
                                    self.func32(env, arg0, 1048581, 6);
                                    __exit_label2 = 1; break 'label3;
                                }
                                var2 = var1.wrapping_add(8);
                                self.func32(env, var2, 1048587, 7);
                                let mut slot_var1_8_i32 = mload32!(var1 as usize + 8) as i32;
                                if slot_var1_8_i32 != 0 {
                                    break;
                                }
                                let mut slot_var1_16_i64 = mload64!(var1 as usize + 16) as i64;
                                var3 = slot_var1_16_i64;
                                let var10 = mload64!(arg0 as usize + 8) as i64;
                                slot_var1_16_i64 = var10 as i64;
                                let mut slot_var1_8_i64 = var3 as i64;
                                var3 = self.vec_new_val(env);
                                __exit_label0 = 1; break 'label1;
                            }
                            var2 = var1.wrapping_add(8);
                            self.func32(env, var2, 1048594, 9);
                            if slot_var1_8_i32 != 0 {
                                break;
                            }
                            var3 = slot_var1_16_i64;
                            let var13 = mload64!(arg0 as usize + 8) as i64;
                            var4 = var13;
                            let var14 = mload64!(arg0 as usize + 16) as i64;
                            let mut slot_var1_24_i64 = var14 as i64;
                            slot_var1_16_i64 = var4 as i64;
                            slot_var1_8_i64 = var3 as i64;
                            var3 = self.vec_new_val(env);
                            __exit_label0 = 1; break 'label1;
                        }
                        arg0 = var1.wrapping_add(8);
                        self.func32(env, arg0, 1048603, 11);
                        __exit_label2 = 1; break 'label3;
                    }
                    arg0 = var1.wrapping_add(8);
                    self.func32(env, arg0, 1048614, 4);
                    __exit_label2 = 1; break 'label3;
                }
                arg0 = var1.wrapping_add(8);
                self.func32(env, arg0, 1048618, 6);
                __exit_label2 = 1; break 'label3;
            }
            if __exit_label2 == 0 {
            arg0 = var1.wrapping_add(8);
            self.func32(env, arg0, 1048624, 8);
            }
            if slot_var1_8_i32 != 0 {
                break;
            }
            var3 = slot_var1_16_i64;
            let var20 = self.global0;
            var2 = var20.wrapping_sub(16);
            self.global0 = var2;
            let mut slot_var2_8_i64 = var3 as i64;
            var3 = self.vec_new_val(env);
            mstore64!(arg0 as usize, 0 /* False */ as u64);
            mstore64!(arg0 as usize + 8, var3 as u64);
            self.global0 = var2.wrapping_add(16);
            var3 = slot_var1_16_i64;
            if (slot_var1_8_i64 == 0) as i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
        }
        if __exit_label0 == 0 {
        unreachable!();
        }
        self.global0 = var1.wrapping_add(32);
        var3
    }
    fn func22(&mut self, env: &Env, mut arg0: i32) {
        let var1 = self.func21(env, arg0);
        let var2 = match 1 /* True */ { 0 => { env.storage().persistent().extend_ttl(&val_from_i64(var1), INSTANCE_BUMP_AMOUNT as u32, BALANCE_BUMP_AMOUNT as u32); 0 }, 1 => { env.storage().temporary().extend_ttl(&val_from_i64(var1), INSTANCE_BUMP_AMOUNT as u32, BALANCE_BUMP_AMOUNT as u32); 0 }, _ => { env.storage().instance().extend_ttl(INSTANCE_BUMP_AMOUNT as u32, BALANCE_BUMP_AMOUNT as u32); 0 } }
        var2;
    }
    fn func23(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(32);
        self.global0 = var1;
        loop {
            let var5 = self.func21(env, 1048632);
            var2 = var5;
            let var6 = self.call_eq_one(env, var2, 0 /* Void */);
            let var7: i64;
            if var6 != 0 {
                let var8 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var2)).unwrap_or(val_from_i64(0))) } }
                self.func25(env, var1, var8);
                let mut slot_var1_0_i32 = mload32!(var1 as usize) as i32;
                if (slot_var1_0_i32 == 1) as i32 != 0 {
                    break;
                }
                let mut slot_var1_24_i64 = mload64!(var1 as usize + 24) as i64;
                var3 = slot_var1_24_i64;
                let mut slot_var1_16_i64 = mload64!(var1 as usize + 16) as i64;
                var7 = slot_var1_16_i64;
            }
            else {
                var7 = 0 /* False */;
            }
            mstore64!(arg0 as usize, var7 as u64);
            mstore64!(arg0 as usize + 8, var3 as u64);
            self.global0 = var1.wrapping_add(32);
            return;
        }
        unreachable!();
    }
    fn call_eq_one(&mut self, env: &Env, arg0: i64, arg1: i64) -> i32 {
        if self.has_contract_data(env, arg0, arg1) == 1 { 1 } else { 0 }
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        loop {
            let mut __exit_label1: i32 = 0;
            loop {
                var2 = arg1 as i32 & 255;
                if (var2 != 69) as i32 != 0 {
                    if (var2 != 11) as i32 != 0 {
                        __exit_label1 = 1; break 'label2;
                    }
                    mstore64!(arg0 as usize + 24, arg1.wrapping_shr(63 as u32) as u64);
                    mstore64!(arg0 as usize + 16, arg1.wrapping_shr(0 as u32) as u64);
                    break;
                }
                let var5 = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64)
                var3 = var5;
                let var6 = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
                arg1 = var6;
                mstore64!(arg0 as usize + 24, var3 as u64);
                mstore64!(arg0 as usize + 16, arg1 as u64);
            }
            if __exit_label1 == 0 {
            var4 = 0 /* False */;
            break;
            }
            mstore64!(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
            var4 = 1 /* True */;
        }
        mstore64!(arg0 as usize, var4 as u64);
    }
    fn func26(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        self.func27(env, 1048632, arg0, arg1, 0 /* Void */);
    }
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let var4 = self.func21(env, arg0);
        let var5 = self.func29(env, arg1, arg2);
        let var6 = match arg3 { 0 => { env.storage().persistent().set(&val_from_i64(var4), &val_from_i64(var5)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var4), &val_from_i64(var5)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var4), &val_from_i64(var5)); 0 } }
        var6;
    }
    fn func28(&mut self, env: &Env) {
        let var0 = { env.storage().instance().extend_ttl(17280 as u32, INSTANCE_BUMP_AMOUNT as u32); 0 }
        var0;
    }
    fn func29(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        self.func39(env, var2, arg0, arg1);
        let mut slot_var2_0_i32 = mload32!(var2 as usize) as i32;
        if (slot_var2_0_i32 == 1) as i32 != 0 {
            unreachable!();
        }
        let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
        self.global0 = var2.wrapping_add(16);
        slot_var2_8_i64
    }
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        self.func58(env, arg0, arg1, 77 /* Address(obj#0) */);
    }
    fn func31(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let var2 = self.func21(env, arg0);
        let var3 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var2), &val_from_i64(arg1)); 0 } }
        var3;
    }
    fn func32(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        self.func37(env, var3, arg1, arg2);
        let mut slot_var3_0_i32 = mload32!(var3 as usize) as i32;
        let var6: i64;
        if slot_var3_0_i32 != 0 {
            var6 = 1 /* True */;
        } else {
            let mut slot_var3_8_i64 = mload64!(var3 as usize + 8) as i64;
            mstore64!(arg0 as usize + 8, slot_var3_8_i64 as u64);
            var6 = 0 /* False */;
        }
        mstore64!(arg0 as usize, var6 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func37(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let var7: i64;
        let mut __exit_label0: i32 = 0;
        loop {
            if (arg2 as u32 > 9 as u32) as i32 != 0 {
                break;
            }
            var4 = arg2;
            var5 = arg1;
            loop {
                let var8 = var6.wrapping_shl(0 as u32) | 0 /* Symbol() */;
                if (var4 == 0) as i32 != 0 {
                    var7 = var8;
                    __exit_label0 = 1; break 'label1;
                }
                var8;
                let var9: i32;
                loop {
                    let var10 = mload8!(var5 as usize) as i32;
                    var3 = var10;
                    let var11 = 1;
                    if (var3 == 95) as i32 != 0 {
                        var9 = var11;
                        break;
                    }
                    var11;
                    {
                        if ((var3.wrapping_sub(48) & 255) as u32 >= 10 as u32) as i32 != 0 {
                            if (((var3.wrapping_sub(65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                break;
                            }
                            if ((var3.wrapping_sub(97) & 255) as u32 >= 26 as u32) as i32 != 0 {
                                break;
                            }
                            var9 = var3.wrapping_sub(59);
                            break;
                        }
                        var9 = var3.wrapping_sub(46);
                    }
                    var9 = var3.wrapping_sub(53);
                }
                var6 = var9 as u32 as i64 & 255 | var6.wrapping_shl(0 as u32);
                var4 = var4.wrapping_sub(1);
                var5 = var5.wrapping_add(1);
                continue;
            }
            unreachable!();
        }
        if __exit_label0 == 0 {
        let var12 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
        var7 = var12;
        }
        var6 = var7;
        mstore64!(arg0 as usize, 0 /* False */ as u64);
        mstore64!(arg0 as usize + 8, var6 as u64);
    }
    fn func38(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(48);
        self.global0 = var1;
        let var4 = mload64!(arg0 as usize + 16) as i64;
        let mut slot_var1_16_i64 = var4 as i64;
        let var5 = mload64!(arg0 as usize + 8) as i64;
        let mut slot_var1_8_i64 = var5 as i64;
        let var6 = mload64!(arg0 as usize) as i64;
        let mut slot_var1_0_i64 = var6 as i64;
        arg0 = 0;
        let var7: i64;
        loop {
            let var8: i64;
            if (arg0 == 24) as i32 != 0 {
                arg0 = 0;
                    if (arg0 != 24) as i32 != 0 {
                        let var9 = mload64!(arg0.wrapping_add(var1) as usize) as i64;
                        mstore64!(var1.wrapping_add(24).wrapping_add(arg0) as usize, var9 as u64);
                        arg0 = arg0.wrapping_add(8);
                        continue;
                    }
                let var10 = self.vec_new_val(env);
                self.global0 = var1.wrapping_add(48);
                var8 = var10;
            }
            else {
                mstore64!(var1.wrapping_add(24).wrapping_add(arg0) as usize, 0 /* Void */ as u64);
                arg0 = arg0.wrapping_add(8);
                continue;
                // There should've been an expression value here, but this may be unreachable
                unreachable!();
            }
            // There should've been an expression value here, but this may be unreachable
            unreachable!();
            break;
        }
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func39(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let var3: i64;
        if (arg1.wrapping_shr(63 as u32) ^ arg2 != 0 /* False */) as i32 | (arg1.wrapping_sub(18410715276690587648) as u64 > 72057594037927935 as u64) as i32 != 0 {
            let var4 = val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)))
            var3 = var4;
        } else {
            var3 = arg1.wrapping_shl(0 as u32) | 0;
        }
        arg1 = var3;
        mstore64!(arg0 as usize, 0 /* False */ as u64);
        mstore64!(arg0 as usize + 8, arg1 as u64);
    }
    fn func42(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32);
        self.global0 = var4;
        let mut slot_var4_8_i64 = arg0 as i64;
        let mut slot_var4_0_i64 = burn as i64;
        loop {
            if (var3 == 16) as i32 != 0 {
                var3 = 0;
                    if (var3 != 16) as i32 != 0 {
                        let var6 = mload64!(var3.wrapping_add(var4) as usize) as i64;
                        mstore64!(var4.wrapping_add(16).wrapping_add(var3) as usize, var6 as u64);
                        var3 = var3.wrapping_add(8);
                        continue;
                    }
                let var7 = self.vec_new_val(env);
                let var8 = self.func29(env, arg1, arg2);
                let var9 = { env.events().publish(val_from_i64(var7), val_from_i64(var8)); 0 }
                var9;
                self.global0 = var4.wrapping_add(32);
            }
            else {
                mstore64!(var4.wrapping_add(16).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue;
            }
            break;
        }
    }
    fn func55(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32);
        self.global0 = var4;
        let mut slot_var4_24_i64 = arg1 as i64;
        let mut slot_var4_16_i64 = arg0 as i64;
        let mut slot_var4_8_i64 = transfer as i64;
        let var6 = self.func38(env, var4.wrapping_add(8));
        let var7 = self.func29(env, arg2, arg3);
        let var8 = { env.events().publish(val_from_i64(var6), val_from_i64(var7)); 0 }
        var8;
        self.global0 = var4.wrapping_add(32);
    }
    fn func58(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i64) {
        let mut var3: i64 = 0;
        loop {
            let var4 = self.func21(env, arg1);
            var3 = var4;
            let var5 = self.call_eq_one(env, var3, 0 /* Void */);
            let var6: i64;
            if var5 != 0 {
                let var7 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var3)).unwrap_or(val_from_i64(0))) } }
                var3 = var7;
                if (arg2 != var3 & 255) as i32 != 0 {
                    break;
                }
                mstore64!(arg0 as usize + 8, var3 as u64);
                var6 = 1 /* True */;
            }
            else {
                var6 = 0 /* False */;
            }
            mstore64!(arg0 as usize, var6 as u64);
            return;
        }
        unreachable!();
    }
    fn func59(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        self.func28(env);
        self.func30(env, var1, arg0);
        let mut slot_var1_0_i32 = mload32!(var1 as usize) as i32;
        if (slot_var1_0_i32 == 0) as i32 != 0 {
            unreachable!();
        }
        let mut slot_var1_8_i64 = mload64!(var1 as usize + 8) as i64;
        self.global0 = var1.wrapping_add(16);
        slot_var1_8_i64
    }
    fn func60(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        self.func28(env);
        self.func58(env, var1, arg0, 73 /* String(obj#0) */);
        let mut slot_var1_0_i32 = mload32!(var1 as usize) as i32;
        if (slot_var1_0_i32 == 0) as i32 != 0 {
            unreachable!();
        }
        let mut slot_var1_8_i64 = mload64!(var1 as usize + 8) as i64;
        self.global0 = var1.wrapping_add(16);
        slot_var1_8_i64
    }
}