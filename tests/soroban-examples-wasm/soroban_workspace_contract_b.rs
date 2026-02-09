#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, vec, Val, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct WorkspaceContractB;

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
impl WorkspaceContractB {
    pub fn add_with(&mut self, mut env: Env, contract: soroban_sdk::Address, x: u32, y: u32) -> u32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32);
        self.global0 = var3;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (x & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            if (y & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_8_i64 = (y & 4294967295) as i64;
            let mut slot_var3_0_i64 = (x & 4294967295) as i64;
            var4 = 0;
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (var4 != 16) as i32 != 0 {
                            break 'label3;
                        }
                        var4 = 0;
                        'label4: loop {
                            'label5: loop {
                                if (var4 == 16) as i32 != 0 {
                                    break 'label4;
                                }
                                let var6 = self.memory.load64(var3.wrapping_add(var4) as usize) as i64;
                                self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, var6 as u64);
                                var4 = var4.wrapping_add(8);
                                continue 'label5;
                                break;
                            }
                            break;
                        }
                        let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
                        let var8 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(contract)), &Symbol::from_val(env, &val_from_i64(add)), Vec::<Val>::from_val(env, &val_from_i64(var7))))
                        contract = var8;
                        if (contract & 255 != 0) as i32 != 0 {
                            break 'label1;
                        }
                        self.global0 = var3.wrapping_add(32);
                        return contract & 4294967295;
                        break;
                    }
                    self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                    var4 = var4.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func3(env);
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
}

#[allow(dead_code)]
impl WorkspaceContractB {
    fn func2(&mut self, env: &Env, mut contract: i64, mut x: i64, mut y: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32);
        self.global0 = var3;
        'label0: loop {
            if (!(Address::try_from_val(env, &val_from_i64(contract)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            if (x & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            if (y & 255 != 0) as i32 != 0 {
                break 'label0;
            }
            let mut slot_var3_8_i64 = (y & 4294967295) as i64;
            let mut slot_var3_0_i64 = (x & 4294967295) as i64;
            var4 = 0;
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        if (var4 != 16) as i32 != 0 {
                            break 'label3;
                        }
                        var4 = 0;
                        'label4: loop {
                            'label5: loop {
                                if (var4 == 16) as i32 != 0 {
                                    break 'label4;
                                }
                                let var6 = self.memory.load64(var3.wrapping_add(var4) as usize) as i64;
                                self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, var6 as u64);
                                var4 = var4.wrapping_add(8);
                                continue 'label5;
                                break;
                            }
                            break;
                        }
                        let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
                        let var8 = val_to_i64(env.invoke_contract::<Val>(&Address::from_val(env, &val_from_i64(contract)), &Symbol::from_val(env, &val_from_i64(add)), Vec::<Val>::from_val(env, &val_from_i64(var7))))
                        contract = var8;
                        if (contract & 255 != 0) as i32 != 0 {
                            break 'label1;
                        }
                        self.global0 = var3.wrapping_add(32);
                        return contract & 4294967295;
                        break;
                    }
                    self.memory.store64(var3.wrapping_add(16).wrapping_add(var4) as usize, 0 /* Void */ as u64);
                    var4 = var4.wrapping_add(8);
                    continue 'label2;
                    break;
                }
                break;
            }
            self.func3(env);
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func3(&mut self, env: &Env) {
        self.func4(env);
        unreachable!();
    }
    fn func4(&mut self, env: &Env) {
        unreachable!();
    }
}
