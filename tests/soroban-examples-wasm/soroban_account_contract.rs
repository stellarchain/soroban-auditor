#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env, Vec, Map, IntoVal, Symbol, BytesN, symbol_short, auth::Context, auth::CustomAccountInterface, crypto::Hash, TryIntoVal, Val, FromVal, Bytes, String};

#[contract]
struct AccountContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[contracttype]
#[derive(Clone)]
pub struct AccSignature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    SignerCnt,
    Signer(BytesN<32>),
    SpendLimit(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccError {
    NotEnoughSigners = 1,
    NegativeAmount = 2,
    BadSignatureOrder = 3,
    UnknownSigner = 4,
}

const TRANSFER_FN: Symbol = symbol_short!("transfer");
const APPROVE_FN: Symbol = symbol_short!("approve");
const BURN_FN: Symbol = symbol_short!("burn");

#[contractimpl]
impl AccountContract {
    pub fn __constructor(env: Env, signers: Vec<BytesN<32>>) {
        for signer in signers.iter() {
            env.storage().instance().set(&DataKey::Signer(signer), &());
        }
        env.storage().instance().set(&DataKey::SignerCnt, &signers.len());
    }
    pub fn add_limit(env: Env, token: Address, limit: i128) {
        env.current_contract_address().require_auth();
        env.storage().instance().set(&DataKey::SpendLimit(token), &limit);
    }
}

#[allow(dead_code)]
impl AccountContract {
    fn func21(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (arg0 & 255 != 0 /* Symbol() */) as i32 != 0 {
                        break 'label2;
                    }
                    if (arg1 & 255 == 0 /* Symbol() */) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                let var6 = { let a = val_from_i64(arg0); let b = val_from_i64(arg1); if a < b { -1 } else if a > b { 1 } else { 0 } }
                var3 = (var6 == 0) as i32;
                break 'label0;
                break;
            }
            let mut slot_var2_8_i64 = (arg1 as u64).wrapping_shr(0 as u32) as i64 as i64;
            let mut slot_var2_0_i64 = (arg0 as u64).wrapping_shr(0 as u32) as i64 as i64;
            'label3: loop {
                'label4: loop {
                    let var7 = self.func22(env, var2);
                    var3 = var7;
                    let var8 = self.func22(env, var2.wrapping_add(8));
                    var4 = var8;
                    if (var3 == 1114112) as i32 != 0 {
                        break 'label3;
                    }
                    if (var3 == var4) as i32 != 0 {
                        continue 'label4;
                    }
                    break;
                }
                var3 = 0;
                break 'label0;
                break;
            }
            var3 = (var4 == 1114112) as i32;
            break;
        }
        self.global0 = var2.wrapping_add(16);
        var3 ^ 1
    }
    fn func22(&mut self, env: &Env, mut arg0: i32) -> i32 {
        let mut var1: i64 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.memory.load64(arg0 as usize) as i64;
        var1 = var4;
        'label0: loop {
            'label1: loop {
                if ((var1 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                return 1114112;
                break;
            }
            'label2: loop {
                'label3: loop {
                    var2 = (var1 as u64).wrapping_shr(48 as u32) as i64 as i32 & 63;
                    if (var2 != 1) as i32 != 0 {
                        break 'label3;
                    }
                    var2 = 95;
                    break 'label2;
                    break;
                }
                'label4: loop {
                    'label5: loop {
                        'label6: loop {
                            if (var2.wrapping_add(-1) as u32 >= 11 as u32) as i32 != 0 {
                                break 'label6;
                            }
                            var3 = 46;
                            break 'label5;
                            break;
                        }
                        'label7: loop {
                            if (var2.wrapping_add(-12) as u32 >= 26 as u32) as i32 != 0 {
                                break 'label7;
                            }
                            var3 = 53;
                            break 'label5;
                            break;
                        }
                        if (var2 as u32 <= 37 as u32) as i32 != 0 {
                            break 'label4;
                        }
                        var3 = 59;
                        break;
                    }
                    var2 = var2.wrapping_add(var3);
                    break 'label2;
                    break;
                }
                var1 = var1.wrapping_shl(0 as u32);
                self.memory.store64(arg0 as usize, var1 as u64);
                continue 'label0;
                break;
            }
            break;
        }
        self.memory.store64(arg0 as usize, var1.wrapping_shl(0 as u32) as u64);
        var2
    }
    fn func23(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match arg0 as i32 {
                                    0 => break 'label5,
                                    1 => break 'label4,
                                    2 => break 'label3,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            self.func24(env, var2, 1048684, 9);
                            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                            if slot_var2_0_i32 != 0 {
                                break 'label1;
                            }
                            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                            let mut slot_var2_0_i64 = slot_var2_8_i64 as i64;
                            let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                            arg0 = var5;
                            break 'label0;
                            break;
                        }
                        self.func24(env, var2, 1048693, 6);
                        if slot_var2_0_i32 != 0 {
                            break 'label1;
                        }
                        self.func26(env, var2, slot_var2_8_i64, arg1);
                        break 'label2;
                        break;
                    }
                    self.func24(env, var2, 1048699, 10);
                    if slot_var2_0_i32 != 0 {
                        break 'label1;
                    }
                    self.func26(env, var2, slot_var2_8_i64, arg1);
                    break;
                }
                arg0 = slot_var2_8_i64;
                if (slot_var2_0_i64 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var2.wrapping_add(16);
        arg0
    }
    fn func24(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        'label0: loop {
            'label1: loop {
                if (arg2 as u32 > 9 as u32) as i32 != 0 {
                    break 'label1;
                }
                var3 = 0 /* False */;
                var4 = arg2;
                var5 = arg1;
                'label2: loop {
                    'label3: loop {
                        if var4 != 0 {
                            break 'label3;
                        }
                        var3 = var3.wrapping_shl(0 as u32) | 0 /* Symbol() */;
                        break 'label0;
                        break;
                    }
                    var6 = 1;
                    'label4: loop {
                        let var8 = self.memory.load8(var5 as usize) as i32;
                        var7 = var8;
                        if (var7 == 95) as i32 != 0 {
                            break 'label4;
                        }
                        'label5: loop {
                            if (((var7.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                break 'label5;
                            }
                            'label6: loop {
                                if (((var7.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                    break 'label6;
                                }
                                if ((var7.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                    break 'label1;
                                }
                                var6 = var7.wrapping_add(-59);
                                break 'label4;
                                break;
                            }
                            var6 = var7.wrapping_add(-53);
                            break 'label4;
                            break;
                        }
                        var6 = var7.wrapping_add(-46);
                        break;
                    }
                    var3 = var3.wrapping_shl(0 as u32) | var6 as u32 as i64 & 255;
                    var4 = var4.wrapping_add(-1);
                    var5 = var5.wrapping_add(1);
                    continue 'label2;
                    break;
                }
                break;
            }
            let var9 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
            var3 = var9;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, var3 as u64);
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(16);
        self.global0 = var3;
        let mut slot_var3_8_i64 = arg2 as i64;
        let mut slot_var3_0_i64 = arg1 as i64;
        let var5 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        arg2 = var5;
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg2 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(32);
        self.global0 = var2;
        var3 = 0;
        'label0: loop {
            'label1: loop {
                if (var3 == 16) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var2.wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        var4 = 1 /* True */;
        'label2: loop {
            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            self.func28(env, arg1, 1048668, 2, var2, 2);
            let mut slot_var2_0_i64 = self.memory.load64(var2 as usize) as i64;
            self.func29(env, var2.wrapping_add(16), slot_var2_0_i64);
            let mut slot_var2_16_i32 = self.memory.load32(var2 as usize + 16) as i32;
            if slot_var2_16_i32 != 0 {
                break 'label2;
            }
            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
            arg1 = slot_var2_8_i64;
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
            var5 = slot_var2_24_i64;
            let var9 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var9 & 18446744069414584320 != 274877906944) as i32 != 0 {
                break 'label2;
            }
            self.memory.store64(arg0 as usize + 16, arg1 as u64);
            self.memory.store64(arg0 as usize + 8, var5 as u64);
            var4 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var4 as u64);
        self.global0 = var2.wrapping_add(32);
    }
    fn func28(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        'label0: loop {
            if (arg2 == arg4) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var5 = 0 /* TODO: map_unpack_to_linear_memory */
        var5;
    }
    fn func29(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        var2 = 1 /* True */;
        'label0: loop {
            if (!(Bytes::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var3 = Bytes::from_val(env, &val_from_i64(arg1)).len() as i64
            if (var3 & 18446744069414584320 != 137438953472) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(arg0 as usize + 8, arg1 as u64);
            var2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var2 as u64);
    }
    fn func30(&mut self, env: &Env, mut signers: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(signers)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var6 = Vec::<Val>::from_val(env, &val_from_i64(signers)).len() as i64
                var2 = (var6 as u64).wrapping_shr(32 as u32) as i64;
                var3 = 0 /* False */;
                var4 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var2 == var3) as i32 != 0 {
                            break 'label2;
                        }
                        let var7 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(signers)).get_unchecked(var4 as u32))
                        self.func29(env, var1, var7);
                        if (var3 == 4294967295) as i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                        if ((slot_var1_0_i64 == 0) as i32 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                        let var9 = self.func23(env, 1 /* True */, slot_var1_8_i64);
                        self.func31(env, var9, 0 /* Void */);
                        var4 = var4.wrapping_add(4294967296);
                        var3 = var3.wrapping_add(1 /* True */);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                let var11 = Vec::<Val>::from_val(env, &val_from_i64(signers)).len() as i64
                var3 = var11;
                let var12 = self.func23(env, 0 /* False */, var3);
                self.func31(env, var12, var3 & 18446744069414584320 | 0);
                self.global0 = var1.wrapping_add(16);
                return 0 /* Void */;
                break;
            }
            unreachable!();
            break;
        }
        self.func32(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func31(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) {
        let var2 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 }, _ => { env.storage().instance().set(&val_from_i64(arg0), &val_from_i64(arg1)); 0 } }
        var2;
    }
    fn func32(&mut self, env: &Env) {
        self.func44(env);
        unreachable!();
    }
    fn func33(&mut self, env: &Env, mut token: i64, mut limit: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(token)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            self.func34(env, var2, limit);
            let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
            if (slot_var2_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
            limit = slot_var2_24_i64;
            let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
            var3 = slot_var2_16_i64;
            let var6 = val_to_i64(env.current_contract_address().into_val(env))
            let var7 = { address_from_i64(env, var6).require_auth(); 0 }
            var7;
            let var8 = self.func23(env, 0 /* Void */, token);
            let var9 = self.func35(env, var3, limit);
            self.func31(env, var8, var9);
            self.global0 = var2.wrapping_add(32);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func34(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        var2 = arg1 as i32 & 255;
                        if (var2 == 69) as i32 != 0 {
                            break 'label3;
                        }
                        if (var2 != 11) as i32 != 0 {
                            break 'label1;
                        }
                        self.memory.store64(arg0 as usize + 24, arg1.wrapping_shr(63 as u32) as u64);
                        self.memory.store64(arg0 as usize + 16, arg1.wrapping_shr(0 as u32) as u64);
                        break 'label2;
                        break;
                    }
                    let var4 = ((val_from_i64(arg1).as_i128().unwrap_or(0) >> 64) as i64)
                    var3 = var4;
                    let var5 = ((val_from_i64(arg1).as_i128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
                    arg1 = var5;
                    self.memory.store64(arg0 as usize + 24, var3 as u64);
                    self.memory.store64(arg0 as usize + 16, arg1 as u64);
                    break;
                }
                arg1 = 0 /* False */;
                break 'label0;
                break;
            }
            self.memory.store64(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
            arg1 = 1 /* True */;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
    }
    fn func35(&mut self, env: &Env, mut arg0: i64, mut arg1: i64) -> i64 {
        'label0: loop {
            if (arg0.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                break 'label0;
            }
            if (arg0 ^ arg0 | arg1 ^ arg0.wrapping_shr(63 as u32) != 0 /* False */) as i32 != 0 {
                break 'label0;
            }
            return arg0.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var2 = val_to_i64(Val::from_i128(((arg1 as i128) << 64) | (arg0 as u64 as i128)))
        var2
    }
    fn func36(&mut self, env: &Env, mut signature_payload: i64, mut signatures: i64, mut auth_context: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var3 = var16.wrapping_sub(64);
        self.global0 = var3;
        self.func29(env, var3.wrapping_add(16), signature_payload);
        'label0: loop {
            let mut slot_var3_16_i32 = self.memory.load32(var3 as usize + 16) as i32;
            if (slot_var3_16_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(signatures)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(auth_context)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_24_i64 = self.memory.load64(var3 as usize + 24) as i64;
            var4 = slot_var3_24_i64;
            let var18 = Vec::<Val>::from_val(env, &val_from_i64(signatures)).len() as i64
            var5 = (var18 as u64).wrapping_shr(32 as u32) as i64;
            var6 = 4294967295;
            signature_payload = 0 /* False */;
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                if (var5 == signature_payload) as i32 != 0 {
                                    break 'label5;
                                }
                                var7 = var6.wrapping_add(4294967296);
                                let var19 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(signatures)).get_unchecked(var7 as u32))
                                self.func27(env, var3.wrapping_add(16), var19);
                                if (slot_var3_16_i32 == 1) as i32 != 0 {
                                    break 'label0;
                                }
                                let mut slot_var3_32_i64 = self.memory.load64(var3 as usize + 32) as i64;
                                var8 = slot_var3_32_i64;
                                var9 = slot_var3_24_i64;
                                if (signature_payload == 0 /* False */) as i32 != 0 {
                                    break 'label4;
                                }
                                let var21 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(signatures)).get_unchecked(var6 as u32))
                                self.func27(env, var3.wrapping_add(16), var21);
                                if (slot_var3_16_i32 == 1) as i32 != 0 {
                                    break 'label0;
                                }
                                let var23 = { let a = val_from_i64(slot_var3_24_i64); let b = val_from_i64(var9); if a < b { -1 } else if a > b { 1 } else { 0 } }
                                if (var23 <= 18446744073709551615) as i32 != 0 {
                                    break 'label4;
                                }
                                var10 = 12884901891 /* Error(Contract, #3) */;
                                break 'label3;
                                break;
                            }
                            'label6: loop {
                                'label7: loop {
                                    let var24 = self.func23(env, 0 /* False */, signature_payload);
                                    signature_payload = var24;
                                    let var25 = self.func37(env, signature_payload);
                                    if (var25 == 0) as i32 != 0 {
                                        break 'label7;
                                    }
                                    let var26 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(signature_payload)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(signature_payload)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(signature_payload)).unwrap_or(val_from_i64(0))) } };
                                    signature_payload = var26;
                                    if (signature_payload & 255 != 0) as i32 != 0 {
                                        break 'label0;
                                    }
                                    let var27 = Vec::<Val>::from_val(env, &val_from_i64(signatures)).len() as i64
                                    var5 = var27 ^ signature_payload;
                                    let var28 = val_to_i64(env.current_contract_address().into_val(env))
                                    var11 = var28;
                                    let var29 = val_to_i64(Map::<Val, Val>::new(env).into_val(env))
                                    var8 = var29;
                                    let var30 = Vec::<Val>::from_val(env, &val_from_i64(auth_context)).len() as i64
                                    signatures = (var30 as u64).wrapping_shr(32 as u32) as i64;
                                    signature_payload = 0 /* False */;
                                    'label8: loop {
                                        'label9: loop {
                                            if (signature_payload != signatures) as i32 != 0 {
                                                break 'label9;
                                            }
                                            var10 = 0 /* Void */;
                                            break 'label3;
                                            break;
                                        }
                                        'label10: loop {
                                            'label11: loop {
                                                'label12: loop {
                                                    'label13: loop {
                                                        let var31 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(auth_context)).get_unchecked(signature_payload.wrapping_shl(32 as u32) | 0 as u32))
                                                        var9 = var31;
                                                        if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                                                            break 'label13;
                                                        }
                                                        var12 = 0;
                                                        let var32 = Vec::<Val>::from_val(env, &val_from_i64(var9)).len() as i64
                                                        var6 = var32;
                                                        let mut slot_var3_8_i32 = 0 as i32;
                                                        let mut slot_var3_0_i64 = var9 as i64;
                                                        self.memory.store32(var3 as usize + 12, (var6 as u64).wrapping_shr(32 as u32) as i64 as u32);
                                                        self.func39(env, var3.wrapping_add(16), var3);
                                                        var13 = 1;
                                                        var9 = slot_var3_16_i32;
                                                        if (var9 == 0 /* Void */) as i32 != 0 {
                                                            break 'label11;
                                                        }
                                                        var13 = 1;
                                                        if var9 as i32 & 1 != 0 {
                                                            break 'label11;
                                                        }
                                                        'label14: loop {
                                                            var4 = slot_var3_24_i64;
                                                            var14 = var4 as i32 & 255;
                                                            if (var14 == 74) as i32 != 0 {
                                                                break 'label14;
                                                            }
                                                            if (var14 != 14) as i32 != 0 {
                                                                break 'label11;
                                                            }
                                                            break;
                                                        }
                                                        'label15: loop {
                                                            'label16: loop {
                                                                'label17: loop {
                                                                    'label18: loop {
                                                                        'label19: loop {
                                                                            'label20: loop {
                                                                                'label21: loop {
                                                                                    let var34 = 0 /* TODO: symbol_index_in_linear_memory */;
                                                                                    match (var34 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                                                        0 => break 'label21,
                                                                                        1 => break 'label20,
                                                                                        2 => break 'label19,
                                                                                        _ => break 'label10,
                                                                                    }
                                                                                    break;
                                                                                }
                                                                                var13 = 1;
                                                                                let mut slot_var3_12_i32 = self.memory.load32(var3 as usize + 12) as i32;
                                                                                let var35 = self.func41(env, slot_var3_8_i32, slot_var3_12_i32);
                                                                                if (var35 as u32 > 1 as u32) as i32 != 0 {
                                                                                    break 'label11;
                                                                                }
                                                                                self.func39(env, var3.wrapping_add(16), var3);
                                                                                var9 = slot_var3_16_i32;
                                                                                if (var9 == 0 /* Void */) as i32 != 0 {
                                                                                    break 'label11;
                                                                                }
                                                                                var13 = 1;
                                                                                if var9 as i32 & 1 != 0 {
                                                                                    break 'label11;
                                                                                }
                                                                                var9 = slot_var3_24_i64;
                                                                                var12 = 0;
                                                                                'label22: loop {
                                                                                    if (var12 == 24) as i32 != 0 {
                                                                                        break 'label18;
                                                                                    }
                                                                                    self.memory.store64(var3.wrapping_add(16).wrapping_add(var12) as usize, 0 /* Void */ as u64);
                                                                                    var12 = var12.wrapping_add(8);
                                                                                    continue 'label22;
                                                                                    break;
                                                                                }
                                                                                break;
                                                                            }
                                                                            var13 = 1;
                                                                            let var37 = self.func41(env, slot_var3_8_i32, slot_var3_12_i32);
                                                                            if (var37 as u32 > 1 as u32) as i32 != 0 {
                                                                                break 'label11;
                                                                            }
                                                                            self.func39(env, var3.wrapping_add(16), var3);
                                                                            var9 = slot_var3_16_i32;
                                                                            if (var9 == 0 /* Void */) as i32 != 0 {
                                                                                break 'label11;
                                                                            }
                                                                            var13 = 1;
                                                                            if var9 as i32 & 1 != 0 {
                                                                                break 'label11;
                                                                            }
                                                                            var9 = slot_var3_24_i64;
                                                                            var12 = 0;
                                                                            'label23: loop {
                                                                                if (var12 == 16) as i32 != 0 {
                                                                                    break 'label17;
                                                                                }
                                                                                self.memory.store64(var3.wrapping_add(48).wrapping_add(var12) as usize, 0 /* Void */ as u64);
                                                                                var12 = var12.wrapping_add(8);
                                                                                continue 'label23;
                                                                                break;
                                                                            }
                                                                            break;
                                                                        }
                                                                        var13 = 1;
                                                                        let var39 = self.func41(env, slot_var3_8_i32, slot_var3_12_i32);
                                                                        if (var39 as u32 > 1 as u32) as i32 != 0 {
                                                                            break 'label11;
                                                                        }
                                                                        self.func39(env, var3.wrapping_add(16), var3);
                                                                        var9 = slot_var3_16_i32;
                                                                        if (var9 == 0 /* Void */) as i32 != 0 {
                                                                            break 'label11;
                                                                        }
                                                                        var13 = 1;
                                                                        if var9 as i32 & 1 != 0 {
                                                                            break 'label11;
                                                                        }
                                                                        var9 = slot_var3_24_i64;
                                                                        var12 = 0;
                                                                        'label24: loop {
                                                                            if (var12 == 24) as i32 != 0 {
                                                                                break 'label16;
                                                                            }
                                                                            self.memory.store64(var3.wrapping_add(16).wrapping_add(var12) as usize, 0 /* Void */ as u64);
                                                                            var12 = var12.wrapping_add(8);
                                                                            continue 'label24;
                                                                            break;
                                                                        }
                                                                        break;
                                                                    }
                                                                    if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                                                                        break 'label12;
                                                                    }
                                                                    self.func28(env, var9, 1048740, 3, var3.wrapping_add(16), 3);
                                                                    var7 = slot_var3_16_i32;
                                                                    if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var7)).is_ok())) as i32 != 0 {
                                                                        break 'label12;
                                                                    }
                                                                    var9 = slot_var3_24_i64;
                                                                    if (!(Address::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                                                                        break 'label12;
                                                                    }
                                                                    'label25: loop {
                                                                        var6 = slot_var3_32_i64;
                                                                        var12 = var6 as i32 & 255;
                                                                        if (var12 == 14) as i32 != 0 {
                                                                            break 'label25;
                                                                        }
                                                                        if (var12 != 74) as i32 != 0 {
                                                                            break 'label12;
                                                                        }
                                                                        break;
                                                                    }
                                                                    var12 = 1;
                                                                    var13 = 0;
                                                                    break 'label10;
                                                                    break;
                                                                }
                                                                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                                                                    break 'label12;
                                                                }
                                                                self.func28(env, var9, 1048780, 2, var3.wrapping_add(48), 2);
                                                                let mut slot_var3_48_i64 = self.memory.load64(var3 as usize + 48) as i64;
                                                                self.func42(env, var3.wrapping_add(16), slot_var3_48_i64);
                                                                if slot_var3_16_i32 != 0 {
                                                                    break 'label12;
                                                                }
                                                                var9 = slot_var3_24_i64;
                                                                let mut slot_var3_56_i64 = self.memory.load64(var3 as usize + 56) as i64;
                                                                self.func29(env, var3.wrapping_add(16), slot_var3_56_i64);
                                                                if (slot_var3_16_i32 == 1) as i32 != 0 {
                                                                    break 'label12;
                                                                }
                                                                var6 = slot_var3_24_i64;
                                                                var13 = 0;
                                                                break 'label15;
                                                                break;
                                                            }
                                                            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var9)).is_ok())) as i32 != 0 {
                                                                break 'label12;
                                                            }
                                                            self.func28(env, var9, 1048812, 3, var3.wrapping_add(16), 3);
                                                            var7 = slot_var3_16_i32;
                                                            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var7)).is_ok())) as i32 != 0 {
                                                                break 'label12;
                                                            }
                                                            self.func42(env, var3.wrapping_add(48), slot_var3_24_i64);
                                                            if slot_var3_48_i64 != 0 {
                                                                break 'label12;
                                                            }
                                                            var9 = slot_var3_56_i64;
                                                            self.func29(env, var3.wrapping_add(48), slot_var3_32_i64);
                                                            if (slot_var3_48_i64 == 1) as i32 != 0 {
                                                                break 'label12;
                                                            }
                                                            var6 = slot_var3_56_i64;
                                                            var13 = 0;
                                                            break;
                                                        }
                                                        var12 = 0;
                                                        break 'label10;
                                                        break;
                                                    }
                                                    (signature_payload != 4294967295) as i32;
                                                    break 'label6;
                                                    break;
                                                }
                                                var12 = 0;
                                                break 'label10;
                                                break;
                                            }
                                            break;
                                        }
                                        if (signature_payload == 4294967295) as i32 != 0 {
                                            break 'label6;
                                        }
                                        if var13 != 0 {
                                            break 'label6;
                                        }
                                        'label26: loop {
                                            if ((var5 as u64) < 4294967296 as u64) as i32 != 0 {
                                                break 'label26;
                                            }
                                            var10 = 4294967299 /* Error(Contract, #1) */;
                                            if (var12 == 0) as i32 != 0 {
                                                break 'label3;
                                            }
                                            let var48 = { let a = val_from_i64(var9); let b = val_from_i64(var11); if a < b { -1 } else if a > b { 1 } else { 0 } }
                                            if (var48 == 0) as i32 != 0 {
                                                break 'label3;
                                            }
                                            'label27: loop {
                                                let var49 = self.func21(env, var6, transfer);
                                                if (var49 == 0) as i32 != 0 {
                                                    break 'label27;
                                                }
                                                let var50 = self.func21(env, var6, approve);
                                                if (var50 == 0) as i32 != 0 {
                                                    break 'label27;
                                                }
                                                let var51 = self.func21(env, var6, burn);
                                                if var51 != 0 {
                                                    break 'label26;
                                                }
                                                break;
                                            }
                                            'label28: loop {
                                                'label29: loop {
                                                    let var52 = if Map::<Val, Val>::from_val(env, &val_from_i64(var8)).has(val_from_i64(var9)) { 1 } else { 0 }
                                                    if (var52 != 1 /* True */) as i32 != 0 {
                                                        break 'label29;
                                                    }
                                                    let var53 = val_to_i64(Map::<Val, Val>::from_val(env, &val_from_i64(var8)).get(val_from_i64(var9)).unwrap_or(val_from_i64(0)))
                                                    self.func34(env, var3.wrapping_add(16), var53);
                                                    if (slot_var3_16_i32 != 1) as i32 != 0 {
                                                        break 'label28;
                                                    }
                                                    break 'label0;
                                                    break;
                                                }
                                                let var55 = self.func23(env, 0 /* Void */, var9);
                                                var6 = var55;
                                                let var56 = self.func37(env, var6);
                                                if (var56 == 0) as i32 != 0 {
                                                    break 'label26;
                                                }
                                                let var57 = match 2 { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var6)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var6)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var6)).unwrap_or(val_from_i64(0))) } };
                                                self.func34(env, var3.wrapping_add(16), var57);
                                                if (slot_var3_16_i32 == 1) as i32 != 0 {
                                                    break 'label0;
                                                }
                                                break;
                                            }
                                            let mut slot_var3_40_i64 = self.memory.load64(var3 as usize + 40) as i64;
                                            var6 = slot_var3_40_i64;
                                            var4 = slot_var3_32_i64;
                                            let var59 = Vec::<Val>::from_val(env, &val_from_i64(var7)).len() as i64
                                            if (var59 as u64 <= 12884901887 as u64) as i32 != 0 {
                                                break 'label7;
                                            }
                                            let var60 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var7)).get_unchecked(2 as u32))
                                            self.func34(env, var3.wrapping_add(16), var60);
                                            if (slot_var3_16_i32 == 1) as i32 != 0 {
                                                break 'label6;
                                            }
                                            'label30: loop {
                                                var7 = slot_var3_40_i64;
                                                if (var7 >= 0 /* False */) as i32 != 0 {
                                                    break 'label30;
                                                }
                                                var10 = 8589934595 /* Error(Contract, #2) */;
                                                break 'label3;
                                                break;
                                            }
                                            var15 = slot_var3_32_i64;
                                            var13 = ((var4 as u64) < var15 as u64) as i32;
                                            if ({ let a = var13; let b = (var6 < var7) as i32; if (var6 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                                break 'label3;
                                            }
                                            let var62 = self.func35(env, var4.wrapping_sub(var15), var6.wrapping_sub(var7).wrapping_sub(var13 as u32 as i64));
                                            let var63 = { let mut m = Map::<Val, Val>::from_val(env, &val_from_i64(var8)); m.set(val_from_i64(var9), val_from_i64(var62)); val_to_i64(m.into_val(env)) }
                                            var8 = var63;
                                            break;
                                        }
                                        signature_payload = signature_payload.wrapping_add(1 /* True */);
                                        continue 'label8;
                                        break;
                                    }
                                    break;
                                }
                                self.func43(env);
                                unreachable!();
                                break;
                            }
                            self.func32(env);
                            unreachable!();
                            break;
                        }
                        let var66 = self.func23(env, 1 /* True */, var9);
                        let var67 = self.func37(env, var66);
                        if var67 != 0 {
                            break 'label2;
                        }
                        var10 = 17179869187 /* Error(Contract, #4) */;
                        break;
                    }
                    self.global0 = var3.wrapping_add(64);
                    return var10;
                    break;
                }
                let var68 = { env.crypto().ed25519_verify(&BytesN::<32>::from_val(env, &val_from_i64(var9)), &BytesN::<32>::from_val(env, &val_from_i64(var4)).into(), &BytesN::<64>::from_val(env, &val_from_i64(var8))); 0 }
                var68;
                signature_payload = signature_payload.wrapping_add(1 /* True */);
                var6 = var7;
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func37(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let var1 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(arg0)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(arg0)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(arg0)) { 1 } else { 0 } } }
        (var1 == 1 /* True */) as i32
    }
    fn func38(&mut self, env: &Env, mut arg0: i64) -> i64 {
        let var1 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(arg0)).unwrap_or(val_from_i64(0))) } }
        var1
    }
    fn func39(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        var2 = 0 /* Void */;
        'label0: loop {
            let var4 = self.memory.load32(arg1 as usize + 8) as i32;
            var3 = var4;
            let var5 = self.memory.load32(arg1 as usize + 12) as i32;
            if (var3 as u32 >= var5 as u32) as i32 != 0 {
                break 'label0;
            }
            let var6 = self.memory.load64(arg1 as usize) as i64;
            let var7 = val_to_i64(Vec::<Val>::from_val(env, &val_from_i64(var6)).get_unchecked((var3 as u32 as i64).wrapping_shl(32 as u32) | 0 as u32))
            self.memory.store64(arg0 as usize + 8, var7 as u64);
            self.memory.store32(arg1 as usize + 8, var3.wrapping_add(1) as u32);
            var2 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var2 as u64);
    }
    fn func40(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) -> i64 {
        let var3 = 0 /* TODO: symbol_index_in_linear_memory */
        var3
    }
    fn func41(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        'label0: loop {
            if ((arg1 as u32) < arg0 as u32) as i32 != 0 {
                break 'label0;
            }
            return arg1.wrapping_sub(arg0);
            break;
        }
        self.func32(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func42(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (Vec::<Val>::try_from_val(env, &val_from_i64(arg1)).is_ok()) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(arg0 as usize, 1 /* True */ as u64);
                break 'label0;
                break;
            }
            let var6 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
            var3 = var6;
            let mut slot_var2_8_i32 = 0 as i32;
            let mut slot_var2_0_i64 = arg1 as i64;
            self.memory.store32(var2 as usize + 12, (var3 as u64).wrapping_shr(32 as u32) as i64 as u32);
            self.func39(env, var2.wrapping_add(16), var2);
            'label2: loop {
                let mut slot_var2_16_i64 = self.memory.load64(var2 as usize + 16) as i64;
                arg1 = slot_var2_16_i64;
                if (arg1 == 0 /* Void */) as i32 != 0 {
                    break 'label2;
                }
                if arg1 as i32 & 1 != 0 {
                    break 'label2;
                }
                'label3: loop {
                    let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                    arg1 = slot_var2_24_i64;
                    var4 = arg1 as i32 & 255;
                    if (var4 == 74) as i32 != 0 {
                        break 'label3;
                    }
                    if (var4 != 14) as i32 != 0 {
                        break 'label2;
                    }
                    break;
                }
                'label4: loop {
                    let var8 = 0 /* TODO: symbol_index_in_linear_memory */;
                    if (var8 as u64 > 4294967295 as u64) as i32 != 0 {
                        break 'label4;
                    }
                    let mut slot_var2_12_i32 = self.memory.load32(var2 as usize + 12) as i32;
                    let var9 = self.func41(env, slot_var2_8_i32, slot_var2_12_i32);
                    if (var9 as u32 > 1 as u32) as i32 != 0 {
                        break 'label4;
                    }
                    self.func39(env, var2.wrapping_add(16), var2);
                    arg1 = slot_var2_16_i64;
                    if (arg1 == 0 /* Void */) as i32 != 0 {
                        break 'label4;
                    }
                    if arg1 as i32 & 1 != 0 {
                        break 'label4;
                    }
                    self.func29(env, var2.wrapping_add(16), slot_var2_24_i64);
                    if slot_var2_16_i64 != 0 {
                        break 'label4;
                    }
                    arg1 = slot_var2_24_i64;
                    self.memory.store64(arg0 as usize, 0 /* False */ as u64);
                    self.memory.store64(arg0 as usize + 8, arg1 as u64);
                    break 'label0;
                    break;
                }
                self.memory.store64(arg0 as usize, 1 /* True */ as u64);
                break 'label0;
                break;
            }
            self.memory.store64(arg0 as usize, 1 /* True */ as u64);
            break;
        }
        self.global0 = var2.wrapping_add(32);
    }
    fn func43(&mut self, env: &Env) {
        self.func32(env);
        unreachable!();
    }
    fn func44(&mut self, env: &Env) {
        unreachable!();
    }
}

#[contractimpl(contracttrait)]
impl CustomAccountInterface for AccountContract {
    type Signature = Vec<AccSignature>;
    type Error = AccError;
    #[allow(non_snake_case)]
    fn __check_auth(env: Env, signature_payload: Hash<32>, signatures: Self::Signature, auth_context: Vec<Context>) -> Result<(), AccError> {
        authenticate(&env, &signature_payload, &signatures)?;
        let tot_signers: u32 = env.storage().instance().get::<_, u32>(&DataKey::SignerCnt).unwrap();
        let all_signed = tot_signers == signatures.len();
        let curr_contract = env.current_contract_address();
        let mut spend_left_per_token = Map::<Address, i128>::new(&env);
        for context in auth_context.iter() {
            verify_authorization_policy(&env, &context, &curr_contract, all_signed, &mut spend_left_per_token)?;
        }
        Ok(())
    }
}

fn authenticate(env: &Env, signature_payload: &Hash<32>, signatures: &Vec<AccSignature>) -> Result<(), AccError> {
    for i in 0..signatures.len() {
        let signature = signatures.get_unchecked(i);
        if i > 0 {
            let prev_signature = signatures.get_unchecked(i - 1);
            if prev_signature.public_key >= signature.public_key {
                return Err(AccError::BadSignatureOrder);
            }
        }
        if !env.storage().instance().has(&DataKey::Signer(signature.public_key.clone())) {
            return Err(AccError::UnknownSigner);
        }
        env.crypto().ed25519_verify(&signature.public_key, &signature_payload.clone().into(), &signature.signature);
    }
    Ok(())
}

fn verify_authorization_policy(env: &Env, context: &Context, curr_contract: &Address, all_signed: bool, spend_left_per_token: &mut Map<Address, i128>) -> Result<(), AccError> {
    if all_signed {
        return Ok(());
    }
    let contract_context = match context {
        Context::Contract(c) => {
            if &c.contract == curr_contract {
                return Err(AccError::NotEnoughSigners);
            }
            c
        }
        Context::CreateContractHostFn(_) | Context::CreateContractWithCtorHostFn(_) => {
            return Err(AccError::NotEnoughSigners);
        }
    };
    if contract_context.fn_name != TRANSFER_FN && contract_context.fn_name != APPROVE_FN && contract_context.fn_name != BURN_FN {
        return Ok(());
    }
    let spend_left: Option<i128> = if let Some(spend_left) = spend_left_per_token.get(contract_context.contract.clone()) {
        Some(spend_left)
    } else if let Some(limit_left) = env.storage().instance().get::<_, i128>(&DataKey::SpendLimit(contract_context.contract.clone())) {
        Some(limit_left)
    } else {
        None
    };
    if let Some(spend_left) = spend_left {
        let spent: i128 = contract_context.args.get(2).unwrap().try_into_val(env).unwrap();
        if spent < 0 {
            return Err(AccError::NegativeAmount);
        }
        if !all_signed && spent > spend_left {
            return Err(AccError::NotEnoughSigners);
        }
        spend_left_per_token.set(contract_context.contract.clone(), spend_left - spent);
    }
    Ok(())
}
