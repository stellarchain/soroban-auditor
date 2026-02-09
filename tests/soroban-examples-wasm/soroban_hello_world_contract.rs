#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, vec, String, Val, Address, FromVal, IntoVal, Map, Bytes, BytesN, Symbol};

#[contract]
pub struct HelloWorldContract;

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
impl HelloWorldContract {
    pub fn hello(&mut self, env: Env, to: soroban_sdk::String) -> soroban_sdk::Vec<soroban_sdk::String> {
        vec![&env, String::from_str(&env, "String(obj#0)"), to]
    }
}

#[allow(dead_code)]
impl HelloWorldContract {
    fn func2(&mut self, env: &Env, mut to: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(32);
        self.global0 = var1;
        'label0: loop {
            if (!(String::try_from_val(env, &val_from_i64(to)).is_ok())) as i32 != 0 {
                break 'label0;
            }
            let var5 = val_to_i64(String::from_str(env, "")) /* TODO: linear memory */
            var2 = var5;
            let mut slot_var1_8_i64 = to as i64;
            let mut slot_var1_0_i64 = var2 as i64;
            var3 = 0;
            'label1: loop {
                'label2: loop {
                    if (var3 != 16) as i32 != 0 {
                        break 'label2;
                    }
                    var3 = 0;
                    'label3: loop {
                        'label4: loop {
                            if (var3 == 16) as i32 != 0 {
                                break 'label3;
                            }
                            let var6 = self.memory.load64(var1.wrapping_add(var3) as usize) as i64;
                            self.memory.store64(var1.wrapping_add(16).wrapping_add(var3) as usize, var6 as u64);
                            var3 = var3.wrapping_add(8);
                            continue 'label4;
                            break;
                        }
                        break;
                    }
                    let var7 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */
                    to = var7;
                    self.global0 = var1.wrapping_add(32);
                    return to;
                    break;
                }
                self.memory.store64(var1.wrapping_add(16).wrapping_add(var3) as usize, 0 /* Void */ as u64);
                var3 = var3.wrapping_add(8);
                continue 'label1;
                break;
            }
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
}
