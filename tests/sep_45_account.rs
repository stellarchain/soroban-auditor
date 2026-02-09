#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, IntoVal, BytesN, auth::Context, crypto::Hash, Val, FromVal, Map, Bytes, String, Symbol};

#[contract]
pub struct Sep45Account;

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
pub enum DataKey { Admin, Signer(soroban_sdk::BytesN<32>), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Signature { pub public_key: soroban_sdk::BytesN<32>, pub signature: soroban_sdk::BytesN<64>, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AccountError { UnknownSigner = 1, TooManySignatures = 2, }

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
impl Sep45Account {
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, signatures: soroban_sdk::Vec<Signature>, _auth_context: soroban_sdk::Vec<Context>) -> Result<(), AccountError> {
        for signature in signatures.iter() {
            if !env.storage().instance().has(&DataKey::Signer(signature.public_key.clone())) {
                return Err(AccountError::UnknownSigner);
            }
            env.crypto().ed25519_verify(&signature.public_key, &signature_payload.clone().into(), &signature.signature);
        }
        Ok(())
    }
    pub fn ___constructor(&mut self, env: Env, admin: soroban_sdk::Address, signer: soroban_sdk::BytesN<32>) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Signer(signer), &());
    }
    pub fn upgrade(&mut self, env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

#[allow(dead_code)]
impl Sep45Account {
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
impl Sep45Account {
    fn func12(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        let mut __exit_label0: i32 = 0;
        loop {
            {
                if (arg0 as i32 & 1 == 0) as i32 != 0 {
                    break;
                }
                self.func13(env, var2, 1048617, 6);
                let mut slot_var2_0_i32 = mload32!(var2 as usize) as i32;
                if slot_var2_0_i32 != 0 {
                    __exit_label0 = 1; break 'label1;
                }
                let mut slot_var2_8_i64 = mload64!(var2 as usize + 8) as i64;
                arg0 = slot_var2_8_i64;
                slot_var2_8_i64 = arg1 as i64;
                let mut slot_var2_0_i64 = arg0 as i64;
                arg0 = self.vec_new_val(env);
                break;
            }
            self.func13(env, var2, 1048612, 5);
            if slot_var2_0_i32 != 0 {
                __exit_label0 = 1; break 'label1;
            }
            slot_var2_0_i64 = slot_var2_8_i64 as i64;
            arg0 = self.vec_new_val(env);
        }
        if __exit_label0 == 0 {
        self.global0 = var2.wrapping_add(16);
        return arg0;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func13(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var3 = var9.wrapping_sub(16);
        self.global0 = var3;
        var4 = 0 /* False */;
        var5 = arg2;
        var6 = arg1;
        loop {
            let mut __exit_label0: i32 = 0;
            loop {
                let mut __exit_label2: i32 = 0;
                loop {
                    {
                        if (var5 == 0) as i32 != 0 {
                            break;
                        }
                        var7 = 1;
                        let var10 = mload8!(var6 as usize) as i32;
                        var8 = var10;
                        if (var8 == 95) as i32 != 0 {
                            __exit_label0 = 1; break 'label1;
                        }
                        if (((var8.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                            __exit_label2 = 1; break 'label3;
                        }
                        if (((var8.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                            break;
                        }
                        {
                            if ((var8.wrapping_add(-97) & 255) as u32 >= 26 as u32) as i32 != 0 {
                                break;
                            }
                            var7 = var8.wrapping_add(-59);
                            __exit_label0 = 1; break 'label1;
                        }
                        let mut slot_var3_0_i64 = ((var8 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                        let var11 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                        var4 = var11;
                        break;
                    }
                    var4 = var4.wrapping_shl(0 as u32) | 0 /* Symbol() */;
                    let mut slot_var3_4_i64 = var4 as i64;
                }
                if __exit_label2 == 0 {
                mstore64!(arg0 as usize, 0 /* False */ as u64);
                mstore64!(arg0 as usize + 8, var4 as u64);
                self.global0 = var3.wrapping_add(16);
                return;
                }
                var7 = var8.wrapping_add(-46);
                __exit_label0 = 1; break 'label1;
            }
            if __exit_label0 == 0 {
            var7 = var8.wrapping_add(-53);
            }
            var4 = var4.wrapping_shl(0 as u32) | var7 as u32 as i64 & 255;
            var5 = var5.wrapping_add(-1);
            var6 = var6.wrapping_add(1);
            continue;
        }
    }
    fn func16(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        {
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break;
            }
            let var3 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var3 & 18446744069414584320 != 137438953472) as i32 != 0 {
                break;
            }
            mstore64!(arg0 as usize + 8, arg1 as u64);
            var2 = 0 /* False */;
        }
        mstore64!(arg0 as usize, var2 as u64);
    }
    fn func17(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func20(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn call_unreachable(&mut self, env: &Env) {
        self.call_unreachable_2(env);
        unreachable!();
    }
    fn unreachable_stub(&mut self, _env: &Env) {
        unreachable!();
    }
    fn call_unreachable_2(&mut self, env: &Env) {
        self.unreachable_stub(env);
        unreachable!();
    }
}