#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, Map, Val, IntoVal, String, Symbol, Bytes, BytesN, FromVal};

#[contract]
pub struct OtherCustomTypesContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Test { pub a: u32, pub b: bool, pub c: soroban_sdk::Symbol, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SimpleEnum { First, Second, Third, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RoyalCard { Jack = 11, Queen = 12, King = 13, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TupleStruct(pub Test, pub SimpleEnum) ;
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ComplexEnum { Struct(Test), Tuple(TupleStruct), Enum(SimpleEnum), Asset(soroban_sdk::Address, i128), Void, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ComplexEnum2 { Stellar(soroban_sdk::Address), Other(soroban_sdk::Symbol), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ComplexEnum3 { Some((soroban_sdk::Address, i128,)), None, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ComplexStruct { pub a32: u32, pub a64: u64, pub admin: soroban_sdk::Address, pub assets_vec: soroban_sdk::Vec<ComplexEnum2>, pub b32: u32, pub base_asset: ComplexEnum2, pub c32: u32, pub complex_enum3: ComplexEnum3, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error { NumberMustBeOdd = 1, }
#[soroban_sdk::contractevent(topics = ["auth",], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AuthEvent { #[topic] pub hello: soroban_sdk::Address, pub world: soroban_sdk::Symbol, }

#[contractimpl]
impl OtherCustomTypesContract {
    pub fn hello(&mut self, env: Env, hello: soroban_sdk::Symbol) -> soroban_sdk::Symbol {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = hello as i32 & 255;
            if (var1 == 14) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 74) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        hello
    }
    pub fn auth(&mut self, env: Env, addr: soroban_sdk::Address, world: soroban_sdk::Symbol) {
        addr.require_auth_for_args((world).into_val(&env));
    }
    pub fn get_count(&mut self, env: Env) -> u32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func24(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        var1 = slot_var0_8_i32;
        let var5 = self.memory.load32(var0 as usize + 12) as i64;
        var2 = var5;
        self.global0 = var0.wrapping_add(16);
        { let a = var2.wrapping_shl(32 as u32) | 0; let b = 0; if var1 & 1 != 0 { a } else { b } }
    }
    pub fn inc(&mut self, env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        env.events().publish((symbol_short!("COUNTER"), symbol_short!("increment")), count);
        count
    }
    pub fn val(&mut self, env: Env) -> soroban_sdk::Val {
        0 /* Void */
    }
    pub fn u32_fail_on_even(&mut self, env: Env, u32_: u32) -> Result<u32, soroban_sdk::Error> {
        'label0: loop {
            if (u32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        { let a = 4294967299 /* Error(Contract, #1) */; let b = u32_ & 4294967295; if (u32_ & 4294967296 == 0) as i32 != 0 { a } else { b } }
    }
    pub fn u32_(&mut self, env: Env, u32_: u32) -> u32 {
        'label0: loop {
            if (u32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        u32_ & 4294967295
    }
    pub fn i32_(&mut self, env: Env, i32_: i32) -> i32 {
        'label0: loop {
            if (i32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        i32_ & -1 /* I32(-1) */
    }
    pub fn i64_(&mut self, mut env: Env, i64_: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                var1 = i64_ as i32 & 255;
                if (var1 == 65) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    if (var1 != 7) as i32 != 0 {
                        break 'label2;
                    }
                    i64_ = i64_.wrapping_shr(0 as u32);
                    break 'label0;
                    break;
                }
                unreachable!();
                break;
            }
            let var2 = val_from_i64(i64_).as_i64().unwrap_or(0) as i64
            i64_ = var2;
            break;
        }
        'label3: loop {
            if (i64_.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                break 'label3;
            }
            return i64_.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var3 = val_to_i64(Val::from_i64(i64_ as i64))
        var3
    }
    pub fn strukt_hel(&mut self, mut env: Env, strukt: Test) -> soroban_sdk::Vec<soroban_sdk::Symbol> {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        self.func33(env, var1.wrapping_add(16), strukt);
        'label0: loop {
            let var5 = self.memory.load8(var1 as usize + 28) as i32;
            if (var5 == 2) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_8_i64 = slot_var1_16_i64 as i64;
            let mut slot_var1_0_i64 = Hello as i64;
            var2 = 0;
            'label1: loop {
                'label2: loop {
                    if (var2 != 16) as i32 != 0 {
                        break 'label2;
                    }
                    var2 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var2 == 16) as i32 != 0 {
                                break 'label3;
                            }
                            let var6 = self.memory.load64(var1.wrapping_add(var2) as usize) as i64;
                            self.memory.store64(var1.wrapping_add(16).wrapping_add(var2) as usize, var6 as u64);
                            var2 = var2.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    strukt = var7;
                    self.global0 = var1.wrapping_add(32);
                    return strukt;
                    break;
                }
                self.memory.store64(var1.wrapping_add(16).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn strukt(&mut self, mut env: Env, strukt: Test) -> Test {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32);
        self.global0 = var1;
        self.func33(env, var1.wrapping_add(16), strukt);
        'label0: loop {
            let var4 = self.memory.load8(var1 as usize + 28) as i32;
            if (var4 == 2) as i32 != 0 {
                break 'label0;
            }
            let var5 = self.memory.load64(var1.wrapping_add(16).wrapping_add(8) as usize) as i64;
            self.memory.store64(var1.wrapping_add(8) as usize, var5 as u64);
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_0_i64 = slot_var1_16_i64 as i64;
            self.func35(env, var1.wrapping_add(16), var1);
            if (slot_var1_16_i64 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
            strukt = slot_var1_24_i64;
            self.global0 = var1.wrapping_add(32);
            return strukt;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn simple(&mut self, mut env: Env, simple: SimpleEnum) -> SimpleEnum {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(32);
        self.global0 = var1;
        'label0: loop {
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(simple)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var5 = Vec::<Val>::from_val(env, &val_from_i64(simple)).len() as i64
            var2 = var5;
            let mut slot_var1_8_i32 = 0 as i32;
            let mut slot_var1_0_i64 = simple as i64;
            self.memory.store32(var1 as usize + 12, (var2 as u64).wrapping_shr(32 as u32) as i64 as u32);
            self.func30(env, var1.wrapping_add(16), var1);
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            simple = slot_var1_16_i64;
            if (simple == 0 /* Void */) as i32 != 0 {
                break 'label0;
            }
            if simple as i32 & 1 != 0 {
                break 'label0;
            }
            'label1: loop {
                let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                simple = slot_var1_24_i64;
                var3 = simple as i32 & 255;
                if (var3 == 74) as i32 != 0 {
                    break 'label1;
                }
                if (var3 != 14) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            'label2: loop {
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            'label6: loop {
                                let var7 = 0 /* TODO: symbol_index_in_linear_memory */;
                                match (var7 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                    0 => break 'label5,
                                    1 => break 'label4,
                                    2 => break 'label6,
                                    _ => break 'label0,
                                }
                                break;
                            }
                            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                            let var8 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                            if (var8 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            break 'label0;
                            break;
                        }
                        let var9 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                        if var9 != 0 {
                            break 'label0;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048604, 5);
                        if slot_var1_16_i64 != 0 {
                            break 'label0;
                        }
                        self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                        break 'label2;
                        break;
                    }
                    let var12 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                    if var12 != 0 {
                        break 'label0;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048609, 6);
                    if slot_var1_16_i64 != 0 {
                        break 'label0;
                    }
                    self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                    break 'label2;
                    break;
                }
                self.func28(env, var1.wrapping_add(16), 1048615, 5);
                if slot_var1_16_i64 != 0 {
                    break 'label0;
                }
                self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                break;
            }
            simple = slot_var1_24_i64;
            if ((slot_var1_16_i64 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            self.global0 = var1.wrapping_add(32);
            return simple;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn complex(&mut self, mut env: Env, complex: ComplexEnum) -> ComplexEnum {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let var10 = self.global0;
        var1 = var10.wrapping_sub(64);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(complex)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var11 = Vec::<Val>::from_val(env, &val_from_i64(complex)).len() as i64
                var2 = var11;
                let mut slot_var1_40_i32 = 0 as i32;
                let mut slot_var1_32_i64 = complex as i64;
                self.memory.store32(var1 as usize + 44, (var2 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1, var1.wrapping_add(32));
                let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                complex = slot_var1_0_i64;
                if (complex == 0 /* Void */) as i32 != 0 {
                    break 'label1;
                }
                if complex as i32 & 1 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                    complex = slot_var1_8_i64;
                    var3 = complex as i32 & 255;
                    if (var3 == 74) as i32 != 0 {
                        break 'label2;
                    }
                    if (var3 != 14) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            'label6: loop {
                                'label7: loop {
                                    'label8: loop {
                                        'label9: loop {
                                            'label10: loop {
                                                'label11: loop {
                                                    let var13 = 0 /* TODO: symbol_index_in_linear_memory */;
                                                    match (var13 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                        0 => break 'label11,
                                                        1 => break 'label10,
                                                        2 => break 'label9,
                                                        3 => break 'label8,
                                                        4 => break 'label7,
                                                        _ => break 'label1,
                                                    }
                                                    break;
                                                }
                                                let mut slot_var1_44_i32 = self.memory.load32(var1 as usize + 44) as i32;
                                                let var14 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                                if (var14 as u32 > 1 as u32) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                self.func30(env, var1, var1.wrapping_add(32));
                                                complex = slot_var1_0_i64;
                                                if (complex == 0 /* Void */) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                if complex as i32 & 1 != 0 {
                                                    break 'label1;
                                                }
                                                self.func33(env, var1.wrapping_add(48), slot_var1_8_i64);
                                                let var17 = self.memory.load8(var1 as usize + 60) as i32;
                                                if (var17 == 2) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
                                                complex = slot_var1_56_i64;
                                                if (complex & 1095216660480 == 8589934592) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                                                var2 = slot_var1_48_i64;
                                                let mut slot_var1_24_i64 = 0 /* False */ as i64;
                                                let mut slot_var1_16_i64 = complex as i64;
                                                slot_var1_8_i64 = var2 as i64;
                                                self.memory.store8(var1 as usize, 0 as u8);
                                                self.func28(env, var1.wrapping_add(48), 1048644, 6);
                                                if slot_var1_48_i64 != 0 {
                                                    break 'label1;
                                                }
                                                complex = slot_var1_56_i64;
                                                self.func35(env, var1.wrapping_add(48), var1 | 8);
                                                if slot_var1_48_i64 != 0 {
                                                    break 'label1;
                                                }
                                                self.func51(env, var1.wrapping_add(48), complex, slot_var1_56_i64);
                                                break 'label4;
                                                break;
                                            }
                                            let var21 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                            if (var21 as u32 > 1 as u32) as i32 != 0 {
                                                break 'label1;
                                            }
                                            self.func30(env, var1, var1.wrapping_add(32));
                                            complex = slot_var1_0_i64;
                                            if (complex == 0 /* Void */) as i32 != 0 {
                                                break 'label1;
                                            }
                                            if complex as i32 & 1 != 0 {
                                                break 'label1;
                                            }
                                            complex = slot_var1_8_i64;
                                            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(complex)).is_ok())) as i32 != 0 {
                                                break 'label1;
                                            }
                                            var3 = 0;
                                            'label12: loop {
                                                'label13: loop {
                                                    if (var3 == 16) as i32 != 0 {
                                                        break 'label12;
                                                    }
                                                    self.memory.store64(var1.wrapping_add(48).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                    var3 = var3.wrapping_add(8);
                                                    continue 'label13;
                                                    break;
                                                }
                                                break;
                                            }
                                            self.func52(env, complex, var1.wrapping_add(48));
                                            self.func33(env, var1, slot_var1_48_i64);
                                            let var25 = self.memory.load8(var1 as usize + 12) as i32;
                                            var4 = var25;
                                            if (var4 == 2) as i32 != 0 {
                                                break 'label1;
                                            }
                                            let var26 = self.memory.load8(var1.wrapping_add(15) as usize) as i32;
                                            var5 = var26;
                                            let var27 = self.memory.load16(var1 as usize + 13) as i32;
                                            var6 = var27;
                                            var7 = slot_var1_8_i64;
                                            complex = slot_var1_0_i64;
                                            let var28 = self.func29(env, slot_var1_56_i64);
                                            var3 = var28 & 255;
                                            if (var3 == 3) as i32 != 0 {
                                                break 'label1;
                                            }
                                            slot_var1_8_i64 = complex as i64;
                                            self.memory.store8(var1 as usize, 1 as u8);
                                            slot_var1_24_i64 = var3 as u32 as i64 as i64;
                                            slot_var1_16_i64 = (((var6 | var5.wrapping_shl(16 as u32)) as u32 as i64).wrapping_shl(40 as u32) | var7 as u32 as i64 | (var4 as u32 as i64).wrapping_shl(32 as u32)) as i64;
                                            self.func28(env, var1.wrapping_add(48), 1048650, 5);
                                            if slot_var1_48_i64 != 0 {
                                                break 'label1;
                                            }
                                            complex = slot_var1_56_i64;
                                            self.func35(env, var1.wrapping_add(48), var1 | 8);
                                            if slot_var1_48_i64 != 0 {
                                                break 'label1;
                                            }
                                            var2 = slot_var1_56_i64;
                                            self.func27(env, var1.wrapping_add(48), var3);
                                            if (slot_var1_48_i64 == 1) as i32 != 0 {
                                                break 'label1;
                                            }
                                            let mut slot_var1_40_i64 = slot_var1_56_i64 as i64;
                                            slot_var1_32_i64 = var2 as i64;
                                            let var32 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                            self.func51(env, var1.wrapping_add(48), complex, var32);
                                            break 'label4;
                                            break;
                                        }
                                        let var34 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                        if (var34 as u32 > 1 as u32) as i32 != 0 {
                                            break 'label1;
                                        }
                                        self.func30(env, var1, var1.wrapping_add(32));
                                        complex = slot_var1_0_i64;
                                        if (complex == 0 /* Void */) as i32 != 0 {
                                            break 'label1;
                                        }
                                        if complex as i32 & 1 != 0 {
                                            break 'label1;
                                        }
                                        let var36 = self.func29(env, slot_var1_8_i64);
                                        var3 = var36 & 255;
                                        if (var3 == 3) as i32 != 0 {
                                            break 'label1;
                                        }
                                        self.func28(env, var1, 1048655, 4);
                                        if slot_var1_0_i64 != 0 {
                                            break 'label1;
                                        }
                                        complex = slot_var1_8_i64;
                                        self.func27(env, var1, var3);
                                        if slot_var1_0_i64 != 0 {
                                            break 'label1;
                                        }
                                        self.func51(env, var1, complex, slot_var1_8_i64);
                                        break 'label6;
                                        break;
                                    }
                                    let var40 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                    if (var40 as u32 > 2 as u32) as i32 != 0 {
                                        break 'label1;
                                    }
                                    self.func30(env, var1, var1.wrapping_add(32));
                                    complex = slot_var1_0_i64;
                                    if (complex == 0 /* Void */) as i32 != 0 {
                                        break 'label1;
                                    }
                                    if complex as i32 & 1 != 0 {
                                        break 'label1;
                                    }
                                    var2 = slot_var1_8_i64;
                                    if (!(Address::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                                        break 'label1;
                                    }
                                    self.func30(env, var1.wrapping_add(48), var1.wrapping_add(32));
                                    complex = slot_var1_48_i64;
                                    if (complex == 0 /* Void */) as i32 != 0 {
                                        break 'label1;
                                    }
                                    if complex as i32 & 1 != 0 {
                                        break 'label1;
                                    }
                                    self.func53(env, var1, slot_var1_56_i64);
                                    if (slot_var1_0_i64 != 1) as i32 != 0 {
                                        break 'label5;
                                    }
                                    break 'label1;
                                    break;
                                }
                                let var44 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                if var44 != 0 {
                                    break 'label1;
                                }
                                self.func28(env, var1, 1048664, 4);
                                if slot_var1_0_i64 != 0 {
                                    break 'label1;
                                }
                                self.func25(env, var1, slot_var1_8_i64);
                                break;
                            }
                            complex = slot_var1_8_i64;
                            var2 = slot_var1_0_i64;
                            break 'label3;
                            break;
                        }
                        complex = slot_var1_24_i64;
                        var8 = slot_var1_16_i64;
                        self.func28(env, var1, 1048659, 5);
                        if slot_var1_0_i64 != 0 {
                            break 'label1;
                        }
                        var9 = slot_var1_8_i64;
                        self.func54(env, var1, var8, complex);
                        if slot_var1_0_i64 != 0 {
                            break 'label1;
                        }
                        slot_var1_16_i64 = slot_var1_8_i64 as i64;
                        slot_var1_8_i64 = var2 as i64;
                        slot_var1_0_i64 = var9 as i64;
                        let var49 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        complex = var49;
                        break 'label0;
                        break;
                    }
                    complex = slot_var1_56_i64;
                    var2 = slot_var1_48_i64;
                    break;
                }
                if (var2 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(64);
        complex
    }
    pub fn addresse(&mut self, env: Env, addresse: soroban_sdk::Address) -> soroban_sdk::Address {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(addresse)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        addresse
    }
    pub fn bytes(&mut self, env: Env, bytes: soroban_sdk::Bytes) -> soroban_sdk::Bytes {
        'label0: loop {
            if (Bytes::try_from_val(env, &val_from_i64(bytes)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        bytes
    }
    pub fn bytes_n(&mut self, env: Env, bytes_n: soroban_sdk::BytesN<9>) -> soroban_sdk::BytesN<9> {
        'label0: loop {
            'label1: loop {
                if (!(Bytes::try_from_val(env, &val_from_i64(bytes_n)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var1 = Bytes::from_val(env, &val_from_i64(bytes_n)).len() as i64
                if (var1 & 18446744069414584320 == 38654705664) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        bytes_n
    }
    pub fn card(&mut self, env: Env, card: RoyalCard) -> RoyalCard {
        'label0: loop {
            'label1: loop {
                if (card & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (((card as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_add(-14) as u32 > -4 as u32) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        card & 4294967295
    }
    pub fn boolean(&mut self, env: Env, boolean: bool) -> bool {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = boolean as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        var1 as u32 as i64
    }
    pub fn not(&mut self, env: Env, boolean: bool) -> bool {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = boolean as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        ((var1 ^ -1) & 1) as u32 as i64
    }
    pub fn i128(&mut self, mut env: Env, i128: i128) -> i128 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32);
        self.global0 = var1;
        self.func53(env, var1, i128);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
            self.func54(env, var1, slot_var1_16_i64, slot_var1_24_i64);
            if (slot_var1_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
            i128 = slot_var1_8_i64;
            self.global0 = var1.wrapping_add(32);
            return i128;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn u128(&mut self, mut env: Env, u128: u128) -> u128 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    var1 = u128 as i32 & 255;
                    if (var1 == 68) as i32 != 0 {
                        break 'label2;
                    }
                    'label3: loop {
                        if (var1 != 10) as i32 != 0 {
                            break 'label3;
                        }
                        u128 = (u128 as u64).wrapping_shr(0 as u32) as i64;
                        break 'label1;
                        break;
                    }
                    unreachable!();
                    break;
                }
                let var3 = ((val_from_i64(u128).as_u128().unwrap_or(0) >> 64) as i64)
                var2 = var3;
                let var4 = ((val_from_i64(u128).as_u128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
                u128 = var4;
                if ({ let a = (u128 as u64 > 72057594037927935 as u64) as i32; let b = (var2 != 0 /* False */) as i32; if (var2 == 0) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                break;
            }
            return u128.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var5 = val_to_i64(Val::from_u128(((var2 as u128) << 64) | (u128 as u64 as u128)))
        var5
    }
    pub fn multi_args(&mut self, env: Env, a: u32, b: bool) -> u32 {
        let mut var2: i32 = 0;
        'label0: loop {
            if (a & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            var2 = b as i32 & 255;
            var2 = { let a = 1; let b = ((var2 != 0) as i32).wrapping_shl(1 as u32); if (var2 == 1) as i32 != 0 { a } else { b } };
            if (var2 == 2) as i32 != 0 {
                break 'label0;
            }
            return { let a = a & 4294967295; let b = 0; if var2 & 1 != 0 { a } else { b } };
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn map(&mut self, env: Env, map: soroban_sdk::Map<u32, bool>) -> soroban_sdk::Map<u32, bool> {
        'label0: loop {
            if (Map::<Val, Val>::try_from_val(env, &val_from_i64(map)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        map
    }
    pub fn vec(&mut self, env: Env, vec: soroban_sdk::Vec<u32>) -> soroban_sdk::Vec<u32> {
        'label0: loop {
            if (Vec::<Val>::try_from_val(env, &val_from_i64(vec)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        vec
    }
    pub fn tuple(&mut self, mut env: Env, tuple: (soroban_sdk::Symbol, u32,)) -> (soroban_sdk::Symbol, u32,) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(tuple)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                var2 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var2 == 16) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(var2) as usize, 0 /* Void */ as u64);
                        var2 = var2.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                self.func52(env, tuple, var1);
                'label4: loop {
                    let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                    tuple = slot_var1_0_i64;
                    var2 = tuple as i32 & 255;
                    if (var2 == 14) as i32 != 0 {
                        break 'label4;
                    }
                    if (var2 != 74) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                var3 = slot_var1_8_i64;
                if (var3 & 255 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        slot_var1_0_i64 = tuple as i64;
        slot_var1_8_i64 = (var3 & 4294967295) as i64;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        tuple = var6;
        self.global0 = var1.wrapping_add(16);
        tuple
    }
    pub fn option(&mut self, env: Env, option: Option<u32>) -> Option<u32> {
        let mut var1: i64 = 0;
        var1 = 0 /* Void */;
        'label0: loop {
            'label1: loop {
                if (option == 0 /* Void */) as i32 != 0 {
                    break 'label1;
                }
                if (option & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                var1 = option & 4294967295;
                break;
            }
            return var1;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn u256(&mut self, env: Env, u256: soroban_sdk::U256) -> soroban_sdk::U256 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = u256 as i32 & 255;
            if (var1 == 12) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 70) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        u256
    }
    pub fn i256(&mut self, env: Env, i256: soroban_sdk::I256) -> soroban_sdk::I256 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = i256 as i32 & 255;
            if (var1 == 13) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 71) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        i256
    }
    pub fn string(&mut self, env: Env, string: soroban_sdk::String) -> soroban_sdk::String {
        'label0: loop {
            if (String::try_from_val(env, &val_from_i64(string)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        string
    }
    pub fn tuple_strukt(&mut self, mut env: Env, tuple_strukt: TupleStruct) -> TupleStruct {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var1 = var7.wrapping_sub(64);
        self.global0 = var1;
        'label0: loop {
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(tuple_strukt)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            var2 = 0;
            'label1: loop {
                'label2: loop {
                    if (var2 == 16) as i32 != 0 {
                        break 'label1;
                    }
                    self.memory.store64(var1.wrapping_add(48).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                    var2 = var2.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func52(env, tuple_strukt, var1.wrapping_add(48));
            let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
            self.func33(env, var1, slot_var1_48_i64);
            let var10 = self.memory.load8(var1 as usize + 12) as i32;
            var3 = var10;
            if (var3 == 2) as i32 != 0 {
                break 'label0;
            }
            var4 = var1.wrapping_add(32).wrapping_add(8);
            var5 = var1.wrapping_add(8);
            let mut slot_var5_0_i32 = self.memory.load32(var5 as usize) as i32;
            let mut slot_var4_0_i32 = slot_var5_0_i32 as i32;
            var6 = var1.wrapping_add(15);
            let var11 = self.memory.load8(var6 as usize) as i32;
            self.memory.store8(var1.wrapping_add(28).wrapping_add(2) as usize, var11 as u8);
            let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
            let mut slot_var1_32_i64 = slot_var1_0_i64 as i64;
            let var12 = self.memory.load16(var1 as usize + 13) as i32;
            self.memory.store16(var1 as usize + 28, var12 as u16);
            let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
            let var13 = self.func29(env, slot_var1_56_i64);
            var2 = var13 & 255;
            if (var2 == 3) as i32 != 0 {
                break 'label0;
            }
            slot_var5_0_i32 = slot_var4_0_i32 as i32;
            let var14 = self.memory.load8(var1.wrapping_add(30) as usize) as i32;
            self.memory.store8(var6 as usize, var14 as u8);
            slot_var1_0_i64 = slot_var1_32_i64 as i64;
            let var15 = self.memory.load16(var1 as usize + 28) as i32;
            self.memory.store16(var1 as usize + 13, var15 as u16);
            self.memory.store8(var1 as usize + 16, var2 as u8);
            self.memory.store8(var1 as usize + 12, var3 as u8);
            self.func35(env, var1.wrapping_add(48), var1);
            if slot_var1_48_i64 != 0 {
                break 'label0;
            }
            tuple_strukt = slot_var1_56_i64;
            self.func27(env, var1.wrapping_add(48), var2);
            if (slot_var1_48_i64 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_40_i64 = slot_var1_56_i64 as i64;
            slot_var1_32_i64 = tuple_strukt as i64;
            let var18 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            tuple_strukt = var18;
            self.global0 = var1.wrapping_add(64);
            return tuple_strukt;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn complex_struct(&mut self, mut env: Env, config: ComplexStruct) -> ComplexStruct {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let var14 = self.global0;
        var1 = var14.wrapping_sub(128);
        self.global0 = var1;
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var1.wrapping_add(48).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func34(env, config, 1048812, 8, var1.wrapping_add(48), 8);
                let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                var3 = slot_var1_48_i64;
                if (var3 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                'label4: loop {
                    'label5: loop {
                        let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
                        config = slot_var1_56_i64;
                        var2 = config as i32 & 255;
                        if (var2 == 64) as i32 != 0 {
                            break 'label5;
                        }
                        if (var2 != 6) as i32 != 0 {
                            break 'label3;
                        }
                        var4 = (config as u64).wrapping_shr(0 as u32) as i64;
                        break 'label4;
                        break;
                    }
                    let var16 = val_from_i64(config).as_u64().unwrap_or(0) as i64
                    var4 = var16;
                    break;
                }
                let mut slot_var1_64_i64 = self.memory.load64(var1 as usize + 64) as i64;
                var5 = slot_var1_64_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_72_i64 = self.memory.load64(var1 as usize + 72) as i64;
                var6 = slot_var1_72_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var6)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_80_i64 = self.memory.load64(var1 as usize + 80) as i64;
                var7 = slot_var1_80_i64;
                if (var7 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_88_i64 = self.memory.load64(var1 as usize + 88) as i64;
                config = slot_var1_88_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var17 = Vec::<Val>::from_val(env, &val_from_i64(config)).len() as i64
                var8 = var17;
                let mut slot_var1_120_i32 = 0 as i32;
                let mut slot_var1_112_i64 = config as i64;
                self.memory.store32(var1 as usize + 124, (var8 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                config = slot_var1_16_i64;
                if (config == 0 /* Void */) as i32 != 0 {
                    break 'label3;
                }
                if config as i32 & 1 != 0 {
                    break 'label3;
                }
                'label6: loop {
                    let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                    config = slot_var1_24_i64;
                    var2 = config as i32 & 255;
                    if (var2 == 74) as i32 != 0 {
                        break 'label6;
                    }
                    if (var2 != 14) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                'label7: loop {
                    'label8: loop {
                        'label9: loop {
                            let var19 = 0 /* TODO: symbol_index_in_linear_memory */;
                            match (var19 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                0 => break 'label9,
                                1 => break 'label8,
                                _ => break 'label3,
                            }
                            break;
                        }
                        let mut slot_var1_124_i32 = self.memory.load32(var1 as usize + 124) as i32;
                        let var20 = self.func32(env, slot_var1_120_i32, slot_var1_124_i32);
                        if (var20 as u32 > 1 as u32) as i32 != 0 {
                            break 'label3;
                        }
                        self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                        config = slot_var1_16_i64;
                        if (config == 0 /* Void */) as i32 != 0 {
                            break 'label3;
                        }
                        if config as i32 & 1 != 0 {
                            break 'label3;
                        }
                        var9 = 0;
                        var10 = slot_var1_24_i64;
                        if (Address::try_from_val(env, &val_from_i64(var10)).is_ok()) as i32 != 0 {
                            break 'label7;
                        }
                        break 'label3;
                        break;
                    }
                    let var22 = self.func32(env, slot_var1_120_i32, slot_var1_124_i32);
                    if (var22 as u32 > 1 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                    config = slot_var1_16_i64;
                    if (config == 0 /* Void */) as i32 != 0 {
                        break 'label3;
                    }
                    var9 = 1;
                    if config as i32 & 1 != 0 {
                        break 'label3;
                    }
                    var10 = slot_var1_24_i64;
                    var2 = var10 as i32 & 255;
                    if (var2 == 14) as i32 != 0 {
                        break 'label7;
                    }
                    if (var2 != 74) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                let mut slot_var1_96_i64 = self.memory.load64(var1 as usize + 96) as i64;
                var11 = slot_var1_96_i64;
                if (var11 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_104_i64 = self.memory.load64(var1 as usize + 104) as i64;
                config = slot_var1_104_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var24 = Vec::<Val>::from_val(env, &val_from_i64(config)).len() as i64
                var8 = var24;
                let mut slot_var1_8_i32 = 0 as i32;
                let mut slot_var1_0_i64 = config as i64;
                self.memory.store32(var1 as usize + 12, (var8 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1.wrapping_add(16), var1);
                config = slot_var1_16_i64;
                if (config == 0 /* Void */) as i32 != 0 {
                    break 'label3;
                }
                if config as i32 & 1 != 0 {
                    break 'label3;
                }
                'label10: loop {
                    config = slot_var1_24_i64;
                    var2 = config as i32 & 255;
                    if (var2 == 74) as i32 != 0 {
                        break 'label10;
                    }
                    if (var2 != 14) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                'label11: loop {
                    'label12: loop {
                        'label13: loop {
                            let var26 = 0 /* TODO: symbol_index_in_linear_memory */;
                            match (var26 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                0 => break 'label12,
                                1 => break 'label13,
                                _ => break 'label3,
                            }
                            break;
                        }
                        let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                        let var27 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                        if var27 != 0 {
                            break 'label3;
                        }
                        var2 = 1;
                        break 'label11;
                        break;
                    }
                    let var28 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                    if (var28 as u32 > 1 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    self.func30(env, var1.wrapping_add(16), var1);
                    config = slot_var1_16_i64;
                    if (config == 0 /* Void */) as i32 != 0 {
                        break 'label3;
                    }
                    if config as i32 & 1 != 0 {
                        break 'label3;
                    }
                    config = slot_var1_24_i64;
                    if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                        break 'label3;
                    }
                    var2 = 0;
                    'label14: loop {
                        'label15: loop {
                            if (var2 == 16) as i32 != 0 {
                                break 'label14;
                            }
                            self.memory.store64(var1.wrapping_add(112).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                            var2 = var2.wrapping_add(8);
                            continue 'label15;
                            break;
                        }
                        break;
                    }
                    self.func52(env, config, var1.wrapping_add(112));
                    var8 = slot_var1_112_i64;
                    if (!(Address::try_from_val(env, &val_from_i64(var8)).is_ok())) as i32 != 0 {
                        break 'label3;
                    }
                    self.func53(env, var1.wrapping_add(16), slot_var1_120_i32);
                    if (slot_var1_16_i64 == 1) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var1_40_i64 = self.memory.load64(var1 as usize + 40) as i64;
                    var12 = slot_var1_40_i64;
                    let mut slot_var1_32_i64 = self.memory.load64(var1 as usize + 32) as i64;
                    var13 = slot_var1_32_i64;
                    var2 = 0;
                    break;
                }
                'label16: loop {
                    'label17: loop {
                        if (var4 as u64 > 72057594037927935 as u64) as i32 != 0 {
                            break 'label17;
                        }
                        config = var4.wrapping_shl(0 as u32) | 0;
                        break 'label16;
                        break;
                    }
                    let var32 = val_to_i64(Val::from_u64(var4 as u64))
                    config = var32;
                    break;
                }
                'label18: loop {
                    'label19: loop {
                        if (var9 == 0) as i32 != 0 {
                            break 'label19;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048715, 5);
                        if slot_var1_16_i64 != 0 {
                            break 'label3;
                        }
                        self.func51(env, var1.wrapping_add(16), slot_var1_24_i64, var10);
                        break 'label18;
                        break;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048708, 7);
                    if slot_var1_16_i64 != 0 {
                        break 'label3;
                    }
                    self.func51(env, var1.wrapping_add(16), slot_var1_24_i64, var10);
                    break;
                }
                var4 = slot_var1_24_i64;
                if slot_var1_16_i64 as i32 != 0 {
                    break 'label3;
                }
                'label20: loop {
                    'label21: loop {
                        if (var2 == 0) as i32 != 0 {
                            break 'label21;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048740, 4);
                        if slot_var1_16_i64 != 0 {
                            break 'label3;
                        }
                        self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                        break 'label20;
                        break;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048736, 4);
                    if slot_var1_16_i64 != 0 {
                        break 'label3;
                    }
                    var10 = slot_var1_24_i64;
                    self.func54(env, var1.wrapping_add(16), var13, var12);
                    if (slot_var1_16_i64 == 1) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var1_120_i64 = slot_var1_24_i64 as i64;
                    slot_var1_112_i64 = var8 as i64;
                    let var41 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func51(env, var1.wrapping_add(16), var10, var41);
                    break;
                }
                var8 = slot_var1_24_i64;
                if (slot_var1_16_i64 == 0) as i32 != 0 {
                    break 'label2;
                }
                break;
            }
            unreachable!();
            break;
        }
        slot_var1_104_i64 = var8 as i64;
        slot_var1_88_i64 = var4 as i64;
        slot_var1_72_i64 = var6 as i64;
        slot_var1_64_i64 = var5 as i64;
        slot_var1_56_i64 = config as i64;
        slot_var1_96_i64 = (var11 & 4294967295) as i64;
        slot_var1_80_i64 = (var7 & 4294967295) as i64;
        slot_var1_48_i64 = (var3 & 4294967295) as i64;
        let var43 = self.func36(env, 1048812, 8, var1.wrapping_add(48), 8);
        config = var43;
        self.global0 = var1.wrapping_add(128);
        config
    }
    pub fn woid(&mut self, env: Env) {
        0 /* Void */
    }
}

#[allow(dead_code)]
impl OtherCustomTypesContract {
    fn func24(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    let var4 = match 1 /* True */ { 0 => { if env.storage().persistent().has(&val_from_i64(COUNTER)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(COUNTER)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(COUNTER)) { 1 } else { 0 } } }
                    if (var4 == 1 /* True */) as i32 != 0 {
                        break 'label2;
                    }
                    var1 = 0;
                    break 'label1;
                    break;
                }
                let var5 = match 1 /* True */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(COUNTER)).unwrap_or(val_from_i64(0))) } }
                var2 = var5;
                if (var2 & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = (var2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                var1 = 1;
                break;
            }
            self.memory.store32(arg0 as usize + 4, var3 as u32);
            self.memory.store32(arg0 as usize, var1 as u32);
            return;
            break;
        }
        unreachable!();
    }
    fn func25(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16);
        self.global0 = var2;
        let mut slot_var2_8_i64 = arg1 as i64;
        let var4 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        arg1 = var4;
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
        self.global0 = var2.wrapping_add(16);
    }
    fn func26(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func27(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            match arg1 & 255 {
                                0 => break 'label4,
                                1 => break 'label3,
                                2 => break 'label2,
                                _ => break 'label4,
                            }
                            break;
                        }
                        self.func28(env, var2, 1048604, 5);
                        var3 = 1 /* True */;
                        let mut slot_var2_0_i32 = self.memory.load32(var2 as usize) as i32;
                        if slot_var2_0_i32 != 0 {
                            break 'label0;
                        }
                        let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
                        self.func25(env, var2, slot_var2_8_i64);
                        if (slot_var2_0_i32 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        break 'label0;
                        break;
                    }
                    self.func28(env, var2, 1048609, 6);
                    var3 = 1 /* True */;
                    if slot_var2_0_i32 != 0 {
                        break 'label0;
                    }
                    self.func25(env, var2, slot_var2_8_i64);
                    if (slot_var2_0_i32 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    break 'label0;
                    break;
                }
                self.func28(env, var2, 1048615, 5);
                var3 = 1 /* True */;
                if slot_var2_0_i32 != 0 {
                    break 'label0;
                }
                self.func25(env, var2, slot_var2_8_i64);
                if slot_var2_0_i32 != 0 {
                    break 'label0;
                }
                break;
            }
            self.memory.store64(arg0 as usize + 8, slot_var2_8_i64 as u64);
            var3 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var3 as u64);
        self.global0 = var2.wrapping_add(16);
    }
    fn func28(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    if (var5 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var7 = 1;
                    'label3: loop {
                        'label4: loop {
                            let var10 = self.memory.load8(var6 as usize) as i32;
                            var8 = var10;
                            if (var8 == 95) as i32 != 0 {
                                break 'label4;
                            }
                            'label5: loop {
                                if (((var8.wrapping_add(-48) & 255) as u32) < 10 as u32) as i32 != 0 {
                                    break 'label5;
                                }
                                'label6: loop {
                                    if (((var8.wrapping_add(-65) & 255) as u32) < 26 as u32) as i32 != 0 {
                                        break 'label6;
                                    }
                                    if ((var8.wrapping_add(-97) & 255) as u32 > 25 as u32) as i32 != 0 {
                                        break 'label3;
                                    }
                                    var7 = var8.wrapping_add(-59);
                                    break 'label4;
                                    break;
                                }
                                var7 = var8.wrapping_add(-53);
                                break 'label4;
                                break;
                            }
                            var7 = var8.wrapping_add(-46);
                            break;
                        }
                        var4 = var4.wrapping_shl(0 as u32) | var7 as u32 as i64 & 255;
                        var5 = var5.wrapping_add(-1);
                        var6 = var6.wrapping_add(1);
                        continue 'label2;
                        break;
                    }
                    break;
                }
                let mut slot_var3_0_i64 = ((var8 as u32 as i64).wrapping_shl(0 as u32) | 1 /* True */) as i64;
                let var11 = val_to_i64(Symbol::new(env, "")) /* TODO: linear memory */
                var4 = var11;
                break 'label0;
                break;
            }
            var4 = var4.wrapping_shl(0 as u32) | 0 /* Symbol() */;
            let mut slot_var3_4_i64 = var4 as i64;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, var4 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func29(&mut self, env: &Env, mut arg0: i64) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(32);
        self.global0 = var1;
        var2 = 3;
        'label0: loop {
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(arg0)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var6 = Vec::<Val>::from_val(env, &val_from_i64(arg0)).len() as i64
            var3 = var6;
            let mut slot_var1_8_i32 = 0 as i32;
            let mut slot_var1_0_i64 = arg0 as i64;
            self.memory.store32(var1 as usize + 12, (var3 as u64).wrapping_shr(32 as u32) as i64 as u32);
            self.func30(env, var1.wrapping_add(16), var1);
            var2 = 3;
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            arg0 = slot_var1_16_i64;
            if (arg0 == 0 /* Void */) as i32 != 0 {
                break 'label0;
            }
            if arg0 as i32 & 1 != 0 {
                break 'label0;
            }
            'label1: loop {
                let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                arg0 = slot_var1_24_i64;
                var4 = arg0 as i32 & 255;
                if (var4 == 74) as i32 != 0 {
                    break 'label1;
                }
                if (var4 != 14) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            var2 = 3;
            'label2: loop {
                'label3: loop {
                    'label4: loop {
                        let var8 = 0 /* TODO: symbol_index_in_linear_memory */;
                        match (var8 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                            0 => break 'label4,
                            1 => break 'label3,
                            2 => break 'label2,
                            _ => break 'label0,
                        }
                        break;
                    }
                    let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                    let var9 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                    if var9 != 0 {
                        break 'label0;
                    }
                    var2 = 0;
                    break 'label0;
                    break;
                }
                let var10 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                if var10 != 0 {
                    break 'label0;
                }
                var2 = 1;
                break 'label0;
                break;
            }
            let var11 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
            if var11 != 0 {
                break 'label0;
            }
            var2 = 2;
            break;
        }
        self.global0 = var1.wrapping_add(32);
        var2
    }
    fn func30(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
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
    fn func31(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32) -> i64 {
        let var3 = 0 /* TODO: symbol_index_in_linear_memory */
        var3
    }
    fn func32(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i32 {
        'label0: loop {
            if ((arg1 as u32) < arg0 as u32) as i32 != 0 {
                break 'label0;
            }
            return arg1.wrapping_sub(arg0);
            break;
        }
        self.func41(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func33(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(32);
        self.global0 = var2;
        var3 = 0;
        'label0: loop {
            'label1: loop {
                if (var3 == 24) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var2.wrapping_add(8).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        var3 = 2;
        'label2: loop {
            if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(arg1)).is_ok())) as i32 != 0 {
                break 'label2;
            }
            self.func34(env, arg1, 1048580, 3, var2.wrapping_add(8), 3);
            let mut slot_var2_8_i64 = self.memory.load64(var2 as usize + 8) as i64;
            arg1 = slot_var2_8_i64;
            if (arg1 & 255 != 0) as i32 != 0 {
                break 'label2;
            }
            var3 = 2;
            let var9 = self.memory.load8(var2 as usize + 16) as i32;
            var4 = var9;
            var4 = { let a = 1; let b = ((var4 != 0) as i32).wrapping_shl(1 as u32); if (var4 == 1) as i32 != 0 { a } else { b } };
            if (var4 == 2) as i32 != 0 {
                break 'label2;
            }
            'label3: loop {
                let mut slot_var2_24_i64 = self.memory.load64(var2 as usize + 24) as i64;
                var5 = slot_var2_24_i64;
                var6 = var5 as i32 & 255;
                if (var6 == 74) as i32 != 0 {
                    break 'label3;
                }
                if (var6 != 14) as i32 != 0 {
                    break 'label2;
                }
                break;
            }
            self.memory.store32(arg0 as usize + 8, (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32 as u32);
            self.memory.store64(arg0 as usize, var5 as u64);
            var3 = var4;
            break;
        }
        self.memory.store8(arg0 as usize + 12, var3 as u8);
        self.global0 = var2.wrapping_add(32);
    }
    fn func34(&mut self, env: &Env, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
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
    fn func35(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(32);
        self.global0 = var2;
        let var5 = self.memory.load64(arg1 as usize) as i64;
        let mut slot_var2_24_i64 = var5 as i64;
        let var6 = self.memory.load8(arg1 as usize + 12) as i64;
        let mut slot_var2_16_i64 = var6 as i64;
        let var7 = self.memory.load32(arg1 as usize + 8) as i64;
        let mut slot_var2_8_i64 = (var7.wrapping_shl(32 as u32) | 0) as i64;
        let var8 = self.func36(env, 1048580, 3, var2.wrapping_add(8), 3);
        var3 = var8;
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, var3 as u64);
        self.global0 = var2.wrapping_add(32);
    }
    fn func36(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
        'label0: loop {
            if (arg1 == arg3) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var4 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
        var4
    }
    fn func37(&mut self, env: &Env, mut hello: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = hello as i32 & 255;
            if (var1 == 14) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 74) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        hello
    }
    fn func38(&mut self, env: &Env, mut addr: i64, mut world: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(addr)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            'label1: loop {
                var3 = world as i32 & 255;
                if (var3 == 14) as i32 != 0 {
                    break 'label1;
                }
                if (var3 != 74) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            let var6 = { address_from_i64(env, addr).require_auth(); 0 }
            var6;
            let mut slot_var2_8_i64 = addr as i64;
            let mut slot_var2_0_i64 = auth as i64;
            var3 = 0;
            'label2: loop {
                'label3: loop {
                    if (var3 != 16) as i32 != 0 {
                        break 'label3;
                    }
                    var3 = 0;
                    'label4: loop {
                        'label5: loop {
                            if (var3 == 16) as i32 != 0 {
                                break 'label4;
                            }
                            let var7 = self.memory.load64(var2.wrapping_add(var3) as usize) as i64;
                            self.memory.store64(var2.wrapping_add(16).wrapping_add(var3) as usize, var7 as u64);
                            var3 = var3.wrapping_add(8);
                            continue 'label5;
                            break;
                        }
                        break;
                    }
                    let var8 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    var4 = var8;
                    let mut slot_var2_16_i64 = world as i64;
                    let var9 = self.func36(env, 1048884, 1, var2.wrapping_add(16), 1);
                    let var10 = { env.events().publish(val_from_i64(var4), val_from_i64(var9)); 0 }
                    var10;
                    self.global0 = var2.wrapping_add(32);
                    return addr;
                    break;
                }
                self.memory.store64(var2.wrapping_add(16).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue 'label2;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func39(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func24(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        var1 = slot_var0_8_i32;
        let var5 = self.memory.load32(var0 as usize + 12) as i64;
        var2 = var5;
        self.global0 = var0.wrapping_add(16);
        { let a = var2.wrapping_shl(32 as u32) | 0; let b = 0; if var1 & 1 != 0 { a } else { b } }
    }
    fn func40(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(16);
        self.global0 = var0;
        self.func24(env, var0.wrapping_add(8));
        'label0: loop {
            let mut slot_var0_12_i32 = self.memory.load32(var0 as usize + 12) as i32;
            let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
            var1 = ({ let a = slot_var0_12_i32; let b = 0; if slot_var0_8_i32 & 1 != 0 { a } else { b } }).wrapping_add(1);
            if var1 != 0 {
                break 'label0;
            }
            self.func41(env);
            unreachable!();
            break;
        }
        var2 = (var1 as u32 as i64).wrapping_shl(32 as u32) | 0;
        let var6 = match 1 /* True */ { 0 => { env.storage().persistent().set(&val_from_i64(COUNTER), &val_from_i64(var2)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(COUNTER), &val_from_i64(var2)); 0 }, _ => { env.storage().instance().set(&val_from_i64(COUNTER), &val_from_i64(var2)); 0 } }
        var6;
        self.global0 = var0.wrapping_add(16);
        var2
    }
    fn func41(&mut self, env: &Env) {
        self.func73(env);
        unreachable!();
    }
    fn func42(&mut self, env: &Env) -> i64 {
        0 /* Void */
    }
    fn func43(&mut self, env: &Env, mut u32_: i64) -> i64 {
        'label0: loop {
            if (u32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        { let a = 4294967299 /* Error(Contract, #1) */; let b = u32_ & 4294967295; if (u32_ & 4294967296 == 0) as i32 != 0 { a } else { b } }
    }
    fn func44(&mut self, env: &Env, mut u32_: i64) -> i64 {
        'label0: loop {
            if (u32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        u32_ & 4294967295
    }
    fn func45(&mut self, env: &Env, mut i32_: i64) -> i64 {
        'label0: loop {
            if (i32_ & 255 == 0) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        i32_ & -1 /* I32(-1) */
    }
    fn func46(&mut self, env: &Env, mut i64_: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                var1 = i64_ as i32 & 255;
                if (var1 == 65) as i32 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    if (var1 != 7) as i32 != 0 {
                        break 'label2;
                    }
                    i64_ = i64_.wrapping_shr(0 as u32);
                    break 'label0;
                    break;
                }
                unreachable!();
                break;
            }
            let var2 = val_from_i64(i64_).as_i64().unwrap_or(0) as i64
            i64_ = var2;
            break;
        }
        'label3: loop {
            if (i64_.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                break 'label3;
            }
            return i64_.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var3 = val_to_i64(Val::from_i64(i64_ as i64))
        var3
    }
    fn func47(&mut self, env: &Env, mut strukt: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(32);
        self.global0 = var1;
        self.func33(env, var1.wrapping_add(16), strukt);
        'label0: loop {
            let var5 = self.memory.load8(var1 as usize + 28) as i32;
            if (var5 == 2) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_8_i64 = slot_var1_16_i64 as i64;
            let mut slot_var1_0_i64 = Hello as i64;
            var2 = 0;
            'label1: loop {
                'label2: loop {
                    if (var2 != 16) as i32 != 0 {
                        break 'label2;
                    }
                    var2 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var2 == 16) as i32 != 0 {
                                break 'label3;
                            }
                            let var6 = self.memory.load64(var1.wrapping_add(var2) as usize) as i64;
                            self.memory.store64(var1.wrapping_add(16).wrapping_add(var2) as usize, var6 as u64);
                            var2 = var2.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    strukt = var7;
                    self.global0 = var1.wrapping_add(32);
                    return strukt;
                    break;
                }
                self.memory.store64(var1.wrapping_add(16).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func48(&mut self, env: &Env, mut strukt: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32);
        self.global0 = var1;
        self.func33(env, var1.wrapping_add(16), strukt);
        'label0: loop {
            let var4 = self.memory.load8(var1 as usize + 28) as i32;
            if (var4 == 2) as i32 != 0 {
                break 'label0;
            }
            let var5 = self.memory.load64(var1.wrapping_add(16).wrapping_add(8) as usize) as i64;
            self.memory.store64(var1.wrapping_add(8) as usize, var5 as u64);
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_0_i64 = slot_var1_16_i64 as i64;
            self.func35(env, var1.wrapping_add(16), var1);
            if (slot_var1_16_i64 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
            strukt = slot_var1_24_i64;
            self.global0 = var1.wrapping_add(32);
            return strukt;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func49(&mut self, env: &Env, mut simple: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(32);
        self.global0 = var1;
        'label0: loop {
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(simple)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var5 = Vec::<Val>::from_val(env, &val_from_i64(simple)).len() as i64
            var2 = var5;
            let mut slot_var1_8_i32 = 0 as i32;
            let mut slot_var1_0_i64 = simple as i64;
            self.memory.store32(var1 as usize + 12, (var2 as u64).wrapping_shr(32 as u32) as i64 as u32);
            self.func30(env, var1.wrapping_add(16), var1);
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            simple = slot_var1_16_i64;
            if (simple == 0 /* Void */) as i32 != 0 {
                break 'label0;
            }
            if simple as i32 & 1 != 0 {
                break 'label0;
            }
            'label1: loop {
                let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                simple = slot_var1_24_i64;
                var3 = simple as i32 & 255;
                if (var3 == 74) as i32 != 0 {
                    break 'label1;
                }
                if (var3 != 14) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            'label2: loop {
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            'label6: loop {
                                let var7 = 0 /* TODO: symbol_index_in_linear_memory */;
                                match (var7 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                    0 => break 'label5,
                                    1 => break 'label4,
                                    2 => break 'label6,
                                    _ => break 'label0,
                                }
                                break;
                            }
                            let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                            let var8 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                            if (var8 == 0) as i32 != 0 {
                                break 'label3;
                            }
                            break 'label0;
                            break;
                        }
                        let var9 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                        if var9 != 0 {
                            break 'label0;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048604, 5);
                        if slot_var1_16_i64 != 0 {
                            break 'label0;
                        }
                        self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                        break 'label2;
                        break;
                    }
                    let var12 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                    if var12 != 0 {
                        break 'label0;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048609, 6);
                    if slot_var1_16_i64 != 0 {
                        break 'label0;
                    }
                    self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                    break 'label2;
                    break;
                }
                self.func28(env, var1.wrapping_add(16), 1048615, 5);
                if slot_var1_16_i64 != 0 {
                    break 'label0;
                }
                self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                break;
            }
            simple = slot_var1_24_i64;
            if ((slot_var1_16_i64 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            self.global0 = var1.wrapping_add(32);
            return simple;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func50(&mut self, env: &Env, mut complex: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let var10 = self.global0;
        var1 = var10.wrapping_sub(64);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(complex)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var11 = Vec::<Val>::from_val(env, &val_from_i64(complex)).len() as i64
                var2 = var11;
                let mut slot_var1_40_i32 = 0 as i32;
                let mut slot_var1_32_i64 = complex as i64;
                self.memory.store32(var1 as usize + 44, (var2 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1, var1.wrapping_add(32));
                let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                complex = slot_var1_0_i64;
                if (complex == 0 /* Void */) as i32 != 0 {
                    break 'label1;
                }
                if complex as i32 & 1 != 0 {
                    break 'label1;
                }
                'label2: loop {
                    let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                    complex = slot_var1_8_i64;
                    var3 = complex as i32 & 255;
                    if (var3 == 74) as i32 != 0 {
                        break 'label2;
                    }
                    if (var3 != 14) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                'label3: loop {
                    'label4: loop {
                        'label5: loop {
                            'label6: loop {
                                'label7: loop {
                                    'label8: loop {
                                        'label9: loop {
                                            'label10: loop {
                                                'label11: loop {
                                                    let var13 = 0 /* TODO: symbol_index_in_linear_memory */;
                                                    match (var13 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                                        0 => break 'label11,
                                                        1 => break 'label10,
                                                        2 => break 'label9,
                                                        3 => break 'label8,
                                                        4 => break 'label7,
                                                        _ => break 'label1,
                                                    }
                                                    break;
                                                }
                                                let mut slot_var1_44_i32 = self.memory.load32(var1 as usize + 44) as i32;
                                                let var14 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                                if (var14 as u32 > 1 as u32) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                self.func30(env, var1, var1.wrapping_add(32));
                                                complex = slot_var1_0_i64;
                                                if (complex == 0 /* Void */) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                if complex as i32 & 1 != 0 {
                                                    break 'label1;
                                                }
                                                self.func33(env, var1.wrapping_add(48), slot_var1_8_i64);
                                                let var17 = self.memory.load8(var1 as usize + 60) as i32;
                                                if (var17 == 2) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
                                                complex = slot_var1_56_i64;
                                                if (complex & 1095216660480 == 8589934592) as i32 != 0 {
                                                    break 'label1;
                                                }
                                                let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                                                var2 = slot_var1_48_i64;
                                                let mut slot_var1_24_i64 = 0 /* False */ as i64;
                                                let mut slot_var1_16_i64 = complex as i64;
                                                slot_var1_8_i64 = var2 as i64;
                                                self.memory.store8(var1 as usize, 0 as u8);
                                                self.func28(env, var1.wrapping_add(48), 1048644, 6);
                                                if slot_var1_48_i64 != 0 {
                                                    break 'label1;
                                                }
                                                complex = slot_var1_56_i64;
                                                self.func35(env, var1.wrapping_add(48), var1 | 8);
                                                if slot_var1_48_i64 != 0 {
                                                    break 'label1;
                                                }
                                                self.func51(env, var1.wrapping_add(48), complex, slot_var1_56_i64);
                                                break 'label4;
                                                break;
                                            }
                                            let var21 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                            if (var21 as u32 > 1 as u32) as i32 != 0 {
                                                break 'label1;
                                            }
                                            self.func30(env, var1, var1.wrapping_add(32));
                                            complex = slot_var1_0_i64;
                                            if (complex == 0 /* Void */) as i32 != 0 {
                                                break 'label1;
                                            }
                                            if complex as i32 & 1 != 0 {
                                                break 'label1;
                                            }
                                            complex = slot_var1_8_i64;
                                            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(complex)).is_ok())) as i32 != 0 {
                                                break 'label1;
                                            }
                                            var3 = 0;
                                            'label12: loop {
                                                'label13: loop {
                                                    if (var3 == 16) as i32 != 0 {
                                                        break 'label12;
                                                    }
                                                    self.memory.store64(var1.wrapping_add(48).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                                                    var3 = var3.wrapping_add(8);
                                                    continue 'label13;
                                                    break;
                                                }
                                                break;
                                            }
                                            self.func52(env, complex, var1.wrapping_add(48));
                                            self.func33(env, var1, slot_var1_48_i64);
                                            let var25 = self.memory.load8(var1 as usize + 12) as i32;
                                            var4 = var25;
                                            if (var4 == 2) as i32 != 0 {
                                                break 'label1;
                                            }
                                            let var26 = self.memory.load8(var1.wrapping_add(15) as usize) as i32;
                                            var5 = var26;
                                            let var27 = self.memory.load16(var1 as usize + 13) as i32;
                                            var6 = var27;
                                            var7 = slot_var1_8_i64;
                                            complex = slot_var1_0_i64;
                                            let var28 = self.func29(env, slot_var1_56_i64);
                                            var3 = var28 & 255;
                                            if (var3 == 3) as i32 != 0 {
                                                break 'label1;
                                            }
                                            slot_var1_8_i64 = complex as i64;
                                            self.memory.store8(var1 as usize, 1 as u8);
                                            slot_var1_24_i64 = var3 as u32 as i64 as i64;
                                            slot_var1_16_i64 = (((var6 | var5.wrapping_shl(16 as u32)) as u32 as i64).wrapping_shl(40 as u32) | var7 as u32 as i64 | (var4 as u32 as i64).wrapping_shl(32 as u32)) as i64;
                                            self.func28(env, var1.wrapping_add(48), 1048650, 5);
                                            if slot_var1_48_i64 != 0 {
                                                break 'label1;
                                            }
                                            complex = slot_var1_56_i64;
                                            self.func35(env, var1.wrapping_add(48), var1 | 8);
                                            if slot_var1_48_i64 != 0 {
                                                break 'label1;
                                            }
                                            var2 = slot_var1_56_i64;
                                            self.func27(env, var1.wrapping_add(48), var3);
                                            if (slot_var1_48_i64 == 1) as i32 != 0 {
                                                break 'label1;
                                            }
                                            let mut slot_var1_40_i64 = slot_var1_56_i64 as i64;
                                            slot_var1_32_i64 = var2 as i64;
                                            let var32 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                            self.func51(env, var1.wrapping_add(48), complex, var32);
                                            break 'label4;
                                            break;
                                        }
                                        let var34 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                        if (var34 as u32 > 1 as u32) as i32 != 0 {
                                            break 'label1;
                                        }
                                        self.func30(env, var1, var1.wrapping_add(32));
                                        complex = slot_var1_0_i64;
                                        if (complex == 0 /* Void */) as i32 != 0 {
                                            break 'label1;
                                        }
                                        if complex as i32 & 1 != 0 {
                                            break 'label1;
                                        }
                                        let var36 = self.func29(env, slot_var1_8_i64);
                                        var3 = var36 & 255;
                                        if (var3 == 3) as i32 != 0 {
                                            break 'label1;
                                        }
                                        self.func28(env, var1, 1048655, 4);
                                        if slot_var1_0_i64 != 0 {
                                            break 'label1;
                                        }
                                        complex = slot_var1_8_i64;
                                        self.func27(env, var1, var3);
                                        if slot_var1_0_i64 != 0 {
                                            break 'label1;
                                        }
                                        self.func51(env, var1, complex, slot_var1_8_i64);
                                        break 'label6;
                                        break;
                                    }
                                    let var40 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                    if (var40 as u32 > 2 as u32) as i32 != 0 {
                                        break 'label1;
                                    }
                                    self.func30(env, var1, var1.wrapping_add(32));
                                    complex = slot_var1_0_i64;
                                    if (complex == 0 /* Void */) as i32 != 0 {
                                        break 'label1;
                                    }
                                    if complex as i32 & 1 != 0 {
                                        break 'label1;
                                    }
                                    var2 = slot_var1_8_i64;
                                    if (!(Address::try_from_val(env, &val_from_i64(var2)).is_ok())) as i32 != 0 {
                                        break 'label1;
                                    }
                                    self.func30(env, var1.wrapping_add(48), var1.wrapping_add(32));
                                    complex = slot_var1_48_i64;
                                    if (complex == 0 /* Void */) as i32 != 0 {
                                        break 'label1;
                                    }
                                    if complex as i32 & 1 != 0 {
                                        break 'label1;
                                    }
                                    self.func53(env, var1, slot_var1_56_i64);
                                    if (slot_var1_0_i64 != 1) as i32 != 0 {
                                        break 'label5;
                                    }
                                    break 'label1;
                                    break;
                                }
                                let var44 = self.func32(env, slot_var1_40_i32, slot_var1_44_i32);
                                if var44 != 0 {
                                    break 'label1;
                                }
                                self.func28(env, var1, 1048664, 4);
                                if slot_var1_0_i64 != 0 {
                                    break 'label1;
                                }
                                self.func25(env, var1, slot_var1_8_i64);
                                break;
                            }
                            complex = slot_var1_8_i64;
                            var2 = slot_var1_0_i64;
                            break 'label3;
                            break;
                        }
                        complex = slot_var1_24_i64;
                        var8 = slot_var1_16_i64;
                        self.func28(env, var1, 1048659, 5);
                        if slot_var1_0_i64 != 0 {
                            break 'label1;
                        }
                        var9 = slot_var1_8_i64;
                        self.func54(env, var1, var8, complex);
                        if slot_var1_0_i64 != 0 {
                            break 'label1;
                        }
                        slot_var1_16_i64 = slot_var1_8_i64 as i64;
                        slot_var1_8_i64 = var2 as i64;
                        slot_var1_0_i64 = var9 as i64;
                        let var49 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                        complex = var49;
                        break 'label0;
                        break;
                    }
                    complex = slot_var1_56_i64;
                    var2 = slot_var1_48_i64;
                    break;
                }
                if (var2 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var1.wrapping_add(64);
        complex
    }
    fn func51(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func52(&mut self, env: &Env, mut arg0: i64, mut arg1: i32) {
        let var2 = 0 /* TODO: vec_unpack_to_linear_memory */
        var2;
    }
    fn func53(&mut self, env: &Env, mut arg0: i32, mut arg1: i64) {
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
    fn func54(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        'label0: loop {
            'label1: loop {
                if (arg1.wrapping_add(36028797018963968) as u64 > 72057594037927935 as u64) as i32 != 0 {
                    break 'label1;
                }
                if (arg1 ^ arg1 | arg2 ^ arg1.wrapping_shr(63 as u32) != 0 /* False */) as i32 != 0 {
                    break 'label1;
                }
                arg1 = arg1.wrapping_shl(0 as u32) | 0;
                break 'label0;
                break;
            }
            let var3 = val_to_i64(Val::from_i128(((arg2 as i128) << 64) | (arg1 as u64 as i128)))
            arg1 = var3;
            break;
        }
        self.memory.store64(arg0 as usize, 0 /* False */ as u64);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
    }
    fn func55(&mut self, env: &Env, mut addresse: i64) -> i64 {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(addresse)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        addresse
    }
    fn func56(&mut self, env: &Env, mut bytes: i64) -> i64 {
        'label0: loop {
            if (Bytes::try_from_val(env, &val_from_i64(bytes)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        bytes
    }
    fn func57(&mut self, env: &Env, mut bytes_n: i64) -> i64 {
        'label0: loop {
            'label1: loop {
                if (!(Bytes::try_from_val(env, &val_from_i64(bytes_n)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                let var1 = Bytes::from_val(env, &val_from_i64(bytes_n)).len() as i64
                if (var1 & 18446744069414584320 == 38654705664) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        bytes_n
    }
    fn func58(&mut self, env: &Env, mut card: i64) -> i64 {
        'label0: loop {
            'label1: loop {
                if (card & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                if (((card as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_add(-14) as u32 > -4 as u32) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        card & 4294967295
    }
    fn func59(&mut self, env: &Env, mut boolean: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = boolean as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        var1 as u32 as i64
    }
    fn func60(&mut self, env: &Env, mut boolean: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = boolean as i32 & 255;
            var1 = { let a = 1; let b = ((var1 != 0) as i32).wrapping_shl(1 as u32); if (var1 == 1) as i32 != 0 { a } else { b } };
            if (var1 != 2) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        ((var1 ^ -1) & 1) as u32 as i64
    }
    fn func61(&mut self, env: &Env, mut i128: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32);
        self.global0 = var1;
        self.func53(env, var1, i128);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
            let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
            self.func54(env, var1, slot_var1_16_i64, slot_var1_24_i64);
            if (slot_var1_0_i32 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
            i128 = slot_var1_8_i64;
            self.global0 = var1.wrapping_add(32);
            return i128;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func62(&mut self, env: &Env, mut u128: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    var1 = u128 as i32 & 255;
                    if (var1 == 68) as i32 != 0 {
                        break 'label2;
                    }
                    'label3: loop {
                        if (var1 != 10) as i32 != 0 {
                            break 'label3;
                        }
                        u128 = (u128 as u64).wrapping_shr(0 as u32) as i64;
                        break 'label1;
                        break;
                    }
                    unreachable!();
                    break;
                }
                let var3 = ((val_from_i64(u128).as_u128().unwrap_or(0) >> 64) as i64)
                var2 = var3;
                let var4 = ((val_from_i64(u128).as_u128().unwrap_or(0) & 0xFFFF_FFFF_FFFF_FFFF) as i64)
                u128 = var4;
                if ({ let a = (u128 as u64 > 72057594037927935 as u64) as i32; let b = (var2 != 0 /* False */) as i32; if (var2 == 0) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                break;
            }
            return u128.wrapping_shl(0 as u32) | 0;
            break;
        }
        let var5 = val_to_i64(Val::from_u128(((var2 as u128) << 64) | (u128 as u64 as u128)))
        var5
    }
    fn func63(&mut self, env: &Env, mut a: i64, mut b: i64) -> i64 {
        let mut var2: i32 = 0;
        'label0: loop {
            if (a & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            var2 = b as i32 & 255;
            var2 = { let a = 1; let b = ((var2 != 0) as i32).wrapping_shl(1 as u32); if (var2 == 1) as i32 != 0 { a } else { b } };
            if (var2 == 2) as i32 != 0 {
                break 'label0;
            }
            return { let a = a & 4294967295; let b = 0; if var2 & 1 != 0 { a } else { b } };
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func64(&mut self, env: &Env, mut map: i64) -> i64 {
        'label0: loop {
            if (Map::<Val, Val>::try_from_val(env, &val_from_i64(map)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        map
    }
    fn func65(&mut self, env: &Env, mut vec: i64) -> i64 {
        'label0: loop {
            if (Vec::<Val>::try_from_val(env, &val_from_i64(vec)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        vec
    }
    fn func66(&mut self, env: &Env, mut tuple: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(tuple)).is_ok())) as i32 != 0 {
                    break 'label1;
                }
                var2 = 0;
                'label2: loop {
                    'label3: loop {
                        if (var2 == 16) as i32 != 0 {
                            break 'label2;
                        }
                        self.memory.store64(var1.wrapping_add(var2) as usize, 0 /* Void */ as u64);
                        var2 = var2.wrapping_add(8);
                        continue 'label3;
                        break;
                    }
                    break;
                }
                self.func52(env, tuple, var1);
                'label4: loop {
                    let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                    tuple = slot_var1_0_i64;
                    var2 = tuple as i32 & 255;
                    if (var2 == 14) as i32 != 0 {
                        break 'label4;
                    }
                    if (var2 != 74) as i32 != 0 {
                        break 'label1;
                    }
                    break;
                }
                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                var3 = slot_var1_8_i64;
                if (var3 & 255 == 0) as i32 != 0 {
                    break 'label0;
                }
                break;
            }
            unreachable!();
            break;
        }
        slot_var1_0_i64 = tuple as i64;
        slot_var1_8_i64 = (var3 & 4294967295) as i64;
        let var6 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
        tuple = var6;
        self.global0 = var1.wrapping_add(16);
        tuple
    }
    fn func67(&mut self, env: &Env, mut option: i64) -> i64 {
        let mut var1: i64 = 0;
        var1 = 0 /* Void */;
        'label0: loop {
            'label1: loop {
                if (option == 0 /* Void */) as i32 != 0 {
                    break 'label1;
                }
                if (option & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                var1 = option & 4294967295;
                break;
            }
            return var1;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func68(&mut self, env: &Env, mut u256: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = u256 as i32 & 255;
            if (var1 == 12) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 70) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        u256
    }
    fn func69(&mut self, env: &Env, mut i256: i64) -> i64 {
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = i256 as i32 & 255;
            if (var1 == 13) as i32 != 0 {
                break 'label0;
            }
            if (var1 == 71) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        i256
    }
    fn func70(&mut self, env: &Env, mut string: i64) -> i64 {
        'label0: loop {
            if (String::try_from_val(env, &val_from_i64(string)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        string
    }
    fn func71(&mut self, env: &Env, mut tuple_strukt: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var1 = var7.wrapping_sub(64);
        self.global0 = var1;
        'label0: loop {
            if (!(Vec::<Val>::try_from_val(env, &val_from_i64(tuple_strukt)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            var2 = 0;
            'label1: loop {
                'label2: loop {
                    if (var2 == 16) as i32 != 0 {
                        break 'label1;
                    }
                    self.memory.store64(var1.wrapping_add(48).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                    var2 = var2.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func52(env, tuple_strukt, var1.wrapping_add(48));
            let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
            self.func33(env, var1, slot_var1_48_i64);
            let var10 = self.memory.load8(var1 as usize + 12) as i32;
            var3 = var10;
            if (var3 == 2) as i32 != 0 {
                break 'label0;
            }
            var4 = var1.wrapping_add(32).wrapping_add(8);
            var5 = var1.wrapping_add(8);
            let mut slot_var5_0_i32 = self.memory.load32(var5 as usize) as i32;
            let mut slot_var4_0_i32 = slot_var5_0_i32 as i32;
            var6 = var1.wrapping_add(15);
            let var11 = self.memory.load8(var6 as usize) as i32;
            self.memory.store8(var1.wrapping_add(28).wrapping_add(2) as usize, var11 as u8);
            let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
            let mut slot_var1_32_i64 = slot_var1_0_i64 as i64;
            let var12 = self.memory.load16(var1 as usize + 13) as i32;
            self.memory.store16(var1 as usize + 28, var12 as u16);
            let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
            let var13 = self.func29(env, slot_var1_56_i64);
            var2 = var13 & 255;
            if (var2 == 3) as i32 != 0 {
                break 'label0;
            }
            slot_var5_0_i32 = slot_var4_0_i32 as i32;
            let var14 = self.memory.load8(var1.wrapping_add(30) as usize) as i32;
            self.memory.store8(var6 as usize, var14 as u8);
            slot_var1_0_i64 = slot_var1_32_i64 as i64;
            let var15 = self.memory.load16(var1 as usize + 28) as i32;
            self.memory.store16(var1 as usize + 13, var15 as u16);
            self.memory.store8(var1 as usize + 16, var2 as u8);
            self.memory.store8(var1 as usize + 12, var3 as u8);
            self.func35(env, var1.wrapping_add(48), var1);
            if slot_var1_48_i64 != 0 {
                break 'label0;
            }
            tuple_strukt = slot_var1_56_i64;
            self.func27(env, var1.wrapping_add(48), var2);
            if (slot_var1_48_i64 == 1) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var1_40_i64 = slot_var1_56_i64 as i64;
            slot_var1_32_i64 = tuple_strukt as i64;
            let var18 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            tuple_strukt = var18;
            self.global0 = var1.wrapping_add(64);
            return tuple_strukt;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func72(&mut self, env: &Env, mut config: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i32 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let var14 = self.global0;
        var1 = var14.wrapping_sub(128);
        self.global0 = var1;
        var2 = 0;
        'label0: loop {
            'label1: loop {
                if (var2 == 64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var1.wrapping_add(48).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                var2 = var2.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                self.func34(env, config, 1048812, 8, var1.wrapping_add(48), 8);
                let mut slot_var1_48_i64 = self.memory.load64(var1 as usize + 48) as i64;
                var3 = slot_var1_48_i64;
                if (var3 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                'label4: loop {
                    'label5: loop {
                        let mut slot_var1_56_i64 = self.memory.load64(var1 as usize + 56) as i64;
                        config = slot_var1_56_i64;
                        var2 = config as i32 & 255;
                        if (var2 == 64) as i32 != 0 {
                            break 'label5;
                        }
                        if (var2 != 6) as i32 != 0 {
                            break 'label3;
                        }
                        var4 = (config as u64).wrapping_shr(0 as u32) as i64;
                        break 'label4;
                        break;
                    }
                    let var16 = val_from_i64(config).as_u64().unwrap_or(0) as i64
                    var4 = var16;
                    break;
                }
                let mut slot_var1_64_i64 = self.memory.load64(var1 as usize + 64) as i64;
                var5 = slot_var1_64_i64;
                if (!(Address::try_from_val(env, &val_from_i64(var5)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_72_i64 = self.memory.load64(var1 as usize + 72) as i64;
                var6 = slot_var1_72_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(var6)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_80_i64 = self.memory.load64(var1 as usize + 80) as i64;
                var7 = slot_var1_80_i64;
                if (var7 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_88_i64 = self.memory.load64(var1 as usize + 88) as i64;
                config = slot_var1_88_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var17 = Vec::<Val>::from_val(env, &val_from_i64(config)).len() as i64
                var8 = var17;
                let mut slot_var1_120_i32 = 0 as i32;
                let mut slot_var1_112_i64 = config as i64;
                self.memory.store32(var1 as usize + 124, (var8 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                let mut slot_var1_16_i64 = self.memory.load64(var1 as usize + 16) as i64;
                config = slot_var1_16_i64;
                if (config == 0 /* Void */) as i32 != 0 {
                    break 'label3;
                }
                if config as i32 & 1 != 0 {
                    break 'label3;
                }
                'label6: loop {
                    let mut slot_var1_24_i64 = self.memory.load64(var1 as usize + 24) as i64;
                    config = slot_var1_24_i64;
                    var2 = config as i32 & 255;
                    if (var2 == 74) as i32 != 0 {
                        break 'label6;
                    }
                    if (var2 != 14) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                'label7: loop {
                    'label8: loop {
                        'label9: loop {
                            let var19 = 0 /* TODO: symbol_index_in_linear_memory */;
                            match (var19 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                0 => break 'label9,
                                1 => break 'label8,
                                _ => break 'label3,
                            }
                            break;
                        }
                        let mut slot_var1_124_i32 = self.memory.load32(var1 as usize + 124) as i32;
                        let var20 = self.func32(env, slot_var1_120_i32, slot_var1_124_i32);
                        if (var20 as u32 > 1 as u32) as i32 != 0 {
                            break 'label3;
                        }
                        self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                        config = slot_var1_16_i64;
                        if (config == 0 /* Void */) as i32 != 0 {
                            break 'label3;
                        }
                        if config as i32 & 1 != 0 {
                            break 'label3;
                        }
                        var9 = 0;
                        var10 = slot_var1_24_i64;
                        if (Address::try_from_val(env, &val_from_i64(var10)).is_ok()) as i32 != 0 {
                            break 'label7;
                        }
                        break 'label3;
                        break;
                    }
                    let var22 = self.func32(env, slot_var1_120_i32, slot_var1_124_i32);
                    if (var22 as u32 > 1 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    self.func30(env, var1.wrapping_add(16), var1.wrapping_add(112));
                    config = slot_var1_16_i64;
                    if (config == 0 /* Void */) as i32 != 0 {
                        break 'label3;
                    }
                    var9 = 1;
                    if config as i32 & 1 != 0 {
                        break 'label3;
                    }
                    var10 = slot_var1_24_i64;
                    var2 = var10 as i32 & 255;
                    if (var2 == 14) as i32 != 0 {
                        break 'label7;
                    }
                    if (var2 != 74) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                let mut slot_var1_96_i64 = self.memory.load64(var1 as usize + 96) as i64;
                var11 = slot_var1_96_i64;
                if (var11 & 255 != 0) as i32 != 0 {
                    break 'label3;
                }
                let mut slot_var1_104_i64 = self.memory.load64(var1 as usize + 104) as i64;
                config = slot_var1_104_i64;
                if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                    break 'label3;
                }
                let var24 = Vec::<Val>::from_val(env, &val_from_i64(config)).len() as i64
                var8 = var24;
                let mut slot_var1_8_i32 = 0 as i32;
                let mut slot_var1_0_i64 = config as i64;
                self.memory.store32(var1 as usize + 12, (var8 as u64).wrapping_shr(32 as u32) as i64 as u32);
                self.func30(env, var1.wrapping_add(16), var1);
                config = slot_var1_16_i64;
                if (config == 0 /* Void */) as i32 != 0 {
                    break 'label3;
                }
                if config as i32 & 1 != 0 {
                    break 'label3;
                }
                'label10: loop {
                    config = slot_var1_24_i64;
                    var2 = config as i32 & 255;
                    if (var2 == 74) as i32 != 0 {
                        break 'label10;
                    }
                    if (var2 != 14) as i32 != 0 {
                        break 'label3;
                    }
                    break;
                }
                'label11: loop {
                    'label12: loop {
                        'label13: loop {
                            let var26 = 0 /* TODO: symbol_index_in_linear_memory */;
                            match (var26 as u64).wrapping_shr(32 as u32) as i64 as i32 {
                                0 => break 'label12,
                                1 => break 'label13,
                                _ => break 'label3,
                            }
                            break;
                        }
                        let mut slot_var1_12_i32 = self.memory.load32(var1 as usize + 12) as i32;
                        let var27 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                        if var27 != 0 {
                            break 'label3;
                        }
                        var2 = 1;
                        break 'label11;
                        break;
                    }
                    let var28 = self.func32(env, slot_var1_8_i32, slot_var1_12_i32);
                    if (var28 as u32 > 1 as u32) as i32 != 0 {
                        break 'label3;
                    }
                    self.func30(env, var1.wrapping_add(16), var1);
                    config = slot_var1_16_i64;
                    if (config == 0 /* Void */) as i32 != 0 {
                        break 'label3;
                    }
                    if config as i32 & 1 != 0 {
                        break 'label3;
                    }
                    config = slot_var1_24_i64;
                    if (!(Vec::<Val>::try_from_val(env, &val_from_i64(config)).is_ok())) as i32 != 0 {
                        break 'label3;
                    }
                    var2 = 0;
                    'label14: loop {
                        'label15: loop {
                            if (var2 == 16) as i32 != 0 {
                                break 'label14;
                            }
                            self.memory.store64(var1.wrapping_add(112).wrapping_add(var2) as usize, 0 /* Void */ as u64);
                            var2 = var2.wrapping_add(8);
                            continue 'label15;
                            break;
                        }
                        break;
                    }
                    self.func52(env, config, var1.wrapping_add(112));
                    var8 = slot_var1_112_i64;
                    if (!(Address::try_from_val(env, &val_from_i64(var8)).is_ok())) as i32 != 0 {
                        break 'label3;
                    }
                    self.func53(env, var1.wrapping_add(16), slot_var1_120_i32);
                    if (slot_var1_16_i64 == 1) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var1_40_i64 = self.memory.load64(var1 as usize + 40) as i64;
                    var12 = slot_var1_40_i64;
                    let mut slot_var1_32_i64 = self.memory.load64(var1 as usize + 32) as i64;
                    var13 = slot_var1_32_i64;
                    var2 = 0;
                    break;
                }
                'label16: loop {
                    'label17: loop {
                        if (var4 as u64 > 72057594037927935 as u64) as i32 != 0 {
                            break 'label17;
                        }
                        config = var4.wrapping_shl(0 as u32) | 0;
                        break 'label16;
                        break;
                    }
                    let var32 = val_to_i64(Val::from_u64(var4 as u64))
                    config = var32;
                    break;
                }
                'label18: loop {
                    'label19: loop {
                        if (var9 == 0) as i32 != 0 {
                            break 'label19;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048715, 5);
                        if slot_var1_16_i64 != 0 {
                            break 'label3;
                        }
                        self.func51(env, var1.wrapping_add(16), slot_var1_24_i64, var10);
                        break 'label18;
                        break;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048708, 7);
                    if slot_var1_16_i64 != 0 {
                        break 'label3;
                    }
                    self.func51(env, var1.wrapping_add(16), slot_var1_24_i64, var10);
                    break;
                }
                var4 = slot_var1_24_i64;
                if slot_var1_16_i64 as i32 != 0 {
                    break 'label3;
                }
                'label20: loop {
                    'label21: loop {
                        if (var2 == 0) as i32 != 0 {
                            break 'label21;
                        }
                        self.func28(env, var1.wrapping_add(16), 1048740, 4);
                        if slot_var1_16_i64 != 0 {
                            break 'label3;
                        }
                        self.func25(env, var1.wrapping_add(16), slot_var1_24_i64);
                        break 'label20;
                        break;
                    }
                    self.func28(env, var1.wrapping_add(16), 1048736, 4);
                    if slot_var1_16_i64 != 0 {
                        break 'label3;
                    }
                    var10 = slot_var1_24_i64;
                    self.func54(env, var1.wrapping_add(16), var13, var12);
                    if (slot_var1_16_i64 == 1) as i32 != 0 {
                        break 'label3;
                    }
                    let mut slot_var1_120_i64 = slot_var1_24_i64 as i64;
                    slot_var1_112_i64 = var8 as i64;
                    let var41 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func51(env, var1.wrapping_add(16), var10, var41);
                    break;
                }
                var8 = slot_var1_24_i64;
                if (slot_var1_16_i64 == 0) as i32 != 0 {
                    break 'label2;
                }
                break;
            }
            unreachable!();
            break;
        }
        slot_var1_104_i64 = var8 as i64;
        slot_var1_88_i64 = var4 as i64;
        slot_var1_72_i64 = var6 as i64;
        slot_var1_64_i64 = var5 as i64;
        slot_var1_56_i64 = config as i64;
        slot_var1_96_i64 = (var11 & 4294967295) as i64;
        slot_var1_80_i64 = (var7 & 4294967295) as i64;
        slot_var1_48_i64 = (var3 & 4294967295) as i64;
        let var43 = self.func36(env, 1048812, 8, var1.wrapping_add(48), 8);
        config = var43;
        self.global0 = var1.wrapping_add(128);
        config
    }
    fn func73(&mut self, env: &Env) {
        unreachable!();
    }
}
