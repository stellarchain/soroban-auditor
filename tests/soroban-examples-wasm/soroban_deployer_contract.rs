#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, auth::InvokerContractAuthEntry, Val, FromVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct DeployerContract;

fn val_from_i64(v: i64) -> Val {
    unsafe { core::mem::transmute::<u64, Val>(v as u64) }
}
fn val_to_i64(v: Val) -> i64 {
    unsafe { core::mem::transmute::<Val, u64>(v) } as i64
}
fn address_from_i64(env: &Env, v: i64) -> Address {
    Address::from_val(env, &val_from_i64(v))
}

#[contractimpl]
impl DeployerContract {
    pub fn call_b(&mut self, env: Env, contract_b_address: soroban_sdk::Address, contract_c_address: soroban_sdk::Address) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var2 = var9.wrapping_sub(96);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(contract_b_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(contract_c_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var10 = self.func9(env, 1048584);
            var3 = var10;
            let var11 = val_to_i64(env.current_contract_address().into_val(env))
            var4 = var11;
            let mut slot_var2_72_i64 = var4 as i64;
            var5 = 0 /* Void */;
            var6 = 1;
            'label1: loop {
                'label2: loop {
                    if (var6 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var6 = var6.wrapping_add(-1);
                    var5 = var4;
                    continue 'label2;
                    break;
                }
                break;
            }
            let mut slot_var2_8_i64 = var5 as i64;
            let var12 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            var5 = var12;
            let var13 = val_to_i64(Vec::<Val>::new(env).into_val(env))
            let mut slot_var2_40_i64 = var13 as i64;
            let mut slot_var2_32_i64 = var5 as i64;
            let mut slot_var2_24_i64 = var3 as i64;
            let mut slot_var2_16_i64 = contract_c_address as i64;
            slot_var2_8_i64 = 0 /* False */ as i64;
            let mut slot_var2_48_i64 = 0 /* Void */ as i64;
            var7 = var2.wrapping_add(48);
            var6 = var2.wrapping_add(8);
            var8 = 1;
            'label3: loop {
                'label4: loop {
                    if (var8 & 1 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    self.func11(env, var2.wrapping_add(72), 1048576, 8);
                    if slot_var2_72_i64 != 0 {
                        break 'label0;
                    }
                    let mut slot_var2_80_i64 = self.memory.load64(var2 as usize + 80) as i64;
                    var5 = slot_var2_80_i64;
                    let mut slot_var6_16_i64 = self.memory.load64(var6 as usize + 16) as i64;
                    let mut slot_var2_88_i64 = slot_var6_16_i64 as i64;
                    let mut slot_var6_8_i64 = self.memory.load64(var6 as usize + 8) as i64;
                    slot_var2_80_i64 = slot_var6_8_i64 as i64;
                    let mut slot_var6_24_i64 = self.memory.load64(var6 as usize + 24) as i64;
                    slot_var2_72_i64 = slot_var6_24_i64 as i64;
                    let var15 = self.func12(env, 1048636, 3, var2.wrapping_add(72), 3);
                    let mut slot_var2_56_i64 = var15 as i64;
                    let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
                    let mut slot_var2_64_i64 = slot_var6_32_i64 as i64;
                    let var16 = self.func12(env, 1048684, 2, var2.wrapping_add(56), 2);
                    self.func13(env, var2.wrapping_add(72), var5, var16);
                    if (slot_var2_72_i64 == 1) as i32 != 0 {
                        break 'label0;
                    }
                    slot_var2_48_i64 = slot_var2_80_i64 as i64;
                    var8 = 0;
                    var6 = var7;
                    continue 'label4;
                    break;
                }
                break;
            }
            let var18 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            let var19 = { env.authorize_as_current_contract(Vec::<InvokerContractAuthEntry>::from_val(env, &val_from_i64(var18))); 0 }
            var19;
            let var20 = val_to_i64(env.current_contract_address().into_val(env))
            var5 = var20;
            let var21 = self.func9(env, 1048599);
            var4 = var21;
            slot_var2_80_i64 = contract_c_address as i64;
            slot_var2_72_i64 = var5 as i64;
            var6 = 0;
            'label5: loop {
                'label6: loop {
                    if (var6 != 16) as i32 != 0 {
                        break 'label6;
                    }
                    var6 = 0;
                    'label7: loop {
                        'label8: loop {
                            if (var6 == 16) as i32 != 0 {
                                break 'label7;
                            }
                            let var22 = self.memory.load64(var2.wrapping_add(72).wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var2.wrapping_add(8).wrapping_add(var6) as usize, var22 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    let var23 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func14(env, contract_b_address, var4, var23);
                    self.global0 = var2.wrapping_add(96);
                    return 0 /* Void */;
                    break;
                }
                self.memory.store64(var2.wrapping_add(8).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label5;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn authorized_fn_b(&mut self, env: Env, authorizer: soroban_sdk::Address, contract_c_address: soroban_sdk::Address) {
        authorizer.require_auth_for_args((contract_c_address).into_val(&env));
    }
    pub fn authorized_fn_c(&mut self, env: Env, authorizer: soroban_sdk::Address) {
    }
}

#[allow(dead_code)]
impl DeployerContract {
    fn func8(&mut self, env: &Env, mut contract_b_address: i64, mut contract_c_address: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let var9 = self.global0;
        var2 = var9.wrapping_sub(96);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(contract_b_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(contract_c_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var10 = self.func9(env, 1048584);
            var3 = var10;
            let var11 = val_to_i64(env.current_contract_address().into_val(env))
            var4 = var11;
            let mut slot_var2_72_i64 = var4 as i64;
            var5 = 0 /* Void */;
            var6 = 1;
            'label1: loop {
                'label2: loop {
                    if (var6 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var6 = var6.wrapping_add(-1);
                    var5 = var4;
                    continue 'label2;
                    break;
                }
                break;
            }
            let mut slot_var2_8_i64 = var5 as i64;
            let var12 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            var5 = var12;
            let var13 = val_to_i64(Vec::<Val>::new(env).into_val(env))
            let mut slot_var2_40_i64 = var13 as i64;
            let mut slot_var2_32_i64 = var5 as i64;
            let mut slot_var2_24_i64 = var3 as i64;
            let mut slot_var2_16_i64 = contract_c_address as i64;
            slot_var2_8_i64 = 0 /* False */ as i64;
            let mut slot_var2_48_i64 = 0 /* Void */ as i64;
            var7 = var2.wrapping_add(48);
            var6 = var2.wrapping_add(8);
            var8 = 1;
            'label3: loop {
                'label4: loop {
                    if (var8 & 1 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    self.func11(env, var2.wrapping_add(72), 1048576, 8);
                    if slot_var2_72_i64 != 0 {
                        break 'label0;
                    }
                    let mut slot_var2_80_i64 = self.memory.load64(var2 as usize + 80) as i64;
                    var5 = slot_var2_80_i64;
                    let mut slot_var6_16_i64 = self.memory.load64(var6 as usize + 16) as i64;
                    let mut slot_var2_88_i64 = slot_var6_16_i64 as i64;
                    let mut slot_var6_8_i64 = self.memory.load64(var6 as usize + 8) as i64;
                    slot_var2_80_i64 = slot_var6_8_i64 as i64;
                    let mut slot_var6_24_i64 = self.memory.load64(var6 as usize + 24) as i64;
                    slot_var2_72_i64 = slot_var6_24_i64 as i64;
                    let var15 = self.func12(env, 1048636, 3, var2.wrapping_add(72), 3);
                    let mut slot_var2_56_i64 = var15 as i64;
                    let mut slot_var6_32_i64 = self.memory.load64(var6 as usize + 32) as i64;
                    let mut slot_var2_64_i64 = slot_var6_32_i64 as i64;
                    let var16 = self.func12(env, 1048684, 2, var2.wrapping_add(56), 2);
                    self.func13(env, var2.wrapping_add(72), var5, var16);
                    if (slot_var2_72_i64 == 1) as i32 != 0 {
                        break 'label0;
                    }
                    slot_var2_48_i64 = slot_var2_80_i64 as i64;
                    var8 = 0;
                    var6 = var7;
                    continue 'label4;
                    break;
                }
                break;
            }
            let var18 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            let var19 = { env.authorize_as_current_contract(Vec::<InvokerContractAuthEntry>::from_val(env, &val_from_i64(var18))); 0 }
            var19;
            let var20 = val_to_i64(env.current_contract_address().into_val(env))
            var5 = var20;
            let var21 = self.func9(env, 1048599);
            var4 = var21;
            slot_var2_80_i64 = contract_c_address as i64;
            slot_var2_72_i64 = var5 as i64;
            var6 = 0;
            'label5: loop {
                'label6: loop {
                    if (var6 != 16) as i32 != 0 {
                        break 'label6;
                    }
                    var6 = 0;
                    'label7: loop {
                        'label8: loop {
                            if (var6 == 16) as i32 != 0 {
                                break 'label7;
                            }
                            let var22 = self.memory.load64(var2.wrapping_add(72).wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var2.wrapping_add(8).wrapping_add(var6) as usize, var22 as u64);
                            var6 = var6.wrapping_add(8);
                            continue 'label8;
                            break;
                        }
                        break;
                    }
                    let var23 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                    self.func14(env, contract_b_address, var4, var23);
                    self.global0 = var2.wrapping_add(96);
                    return 0 /* Void */;
                    break;
                }
                self.memory.store64(var2.wrapping_add(8).wrapping_add(var6) as usize, 0 /* Void */ as u64);
                var6 = var6.wrapping_add(8);
                continue 'label5;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func9(&mut self, env: &Env, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16);
        self.global0 = var1;
        self.func19(env, var1, arg0, 15);
        'label0: loop {
            let mut slot_var1_0_i32 = self.memory.load32(var1 as usize) as i32;
            if (slot_var1_0_i32 != 1) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
        var2 = slot_var1_8_i64;
        self.global0 = var1.wrapping_add(16);
        var2
    }
    fn func10(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
        var2
    }
    fn func11(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(16);
        self.global0 = var3;
        self.func19(env, var3, arg1, arg2);
        var4 = 1 /* True */;
        'label0: loop {
            let mut slot_var3_0_i32 = self.memory.load32(var3 as usize) as i32;
            if slot_var3_0_i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_8_i64 = self.memory.load64(var3 as usize + 8) as i64;
            self.memory.store64(arg0 as usize + 8, slot_var3_8_i64 as u64);
            var4 = 0 /* False */;
            break;
        }
        self.memory.store64(arg0 as usize, var4 as u64);
        self.global0 = var3.wrapping_add(16);
    }
    fn func12(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
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
    fn func13(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
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
    fn func14(&mut self, env: &Env, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        'label0: loop {
            let var3 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(arg0)), &Symbol::from_val(env, &val_from_i64(arg1)), Vec::<Val>::from_val(env, &val_from_i64(arg2))))
            if (var3 & 255 == 0 /* Void */) as i32 != 0 {
                break 'label0;
            }
            self.func18(env);
            unreachable!();
            break;
        }
    }
    fn func15(&mut self, env: &Env, mut authorizer: i64, mut contract_c_address: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(16);
        self.global0 = var2;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(authorizer)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (!(Address::try_from_val(env, &val_from_i64(contract_c_address)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var7 = { address_from_i64(env, authorizer).require_auth(); 0 }
            var7;
            let var8 = self.func9(env, 1048584);
            var3 = var8;
            let mut slot_var2_0_i64 = authorizer as i64;
            var4 = 0 /* Void */;
            var5 = 1;
            'label1: loop {
                'label2: loop {
                    if (var5 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    var5 = var5.wrapping_add(-1);
                    var4 = authorizer;
                    continue 'label2;
                    break;
                }
                break;
            }
            let mut slot_var2_8_i64 = var4 as i64;
            let var9 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
            self.func14(env, contract_c_address, var3, var9);
            self.global0 = var2.wrapping_add(16);
            return 0 /* Void */;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func16(&mut self, env: &Env, mut authorizer: i64) -> i64 {
        'label0: loop {
            if (Address::try_from_val(env, &val_from_i64(authorizer)).is_ok()) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        let var1 = { address_from_i64(env, authorizer).require_auth(); 0 }
        var1;
        0 /* Void */
    }
    fn func17(&mut self, env: &Env) {
        unreachable!();
    }
    fn func18(&mut self, env: &Env) {
        self.func17(env);
        unreachable!();
    }
    fn func19(&mut self, env: &Env, mut arg0: i32, mut arg1: i32, mut arg2: i32) {
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
}
