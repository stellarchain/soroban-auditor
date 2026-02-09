#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Val, Address, FromVal, IntoVal, Vec, Map, Bytes, BytesN, String, Symbol};

#[contract]
pub struct CustomTypesContract;

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
pub struct State { pub count: u32, pub last_incr: u32, }

#[contractimpl]
impl CustomTypesContract {
    pub fn increment(&mut self, env: Env, incr: u32) -> u32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (incr & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                self.func8(env, var1.wrapping_add(8));
                let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                var2 = slot_var1_8_i32;
                var3 = (incr as u64).wrapping_shr(32 as u32) as i64 as i32;
                var4 = var2.wrapping_add(var3);
                if ((var4 as u32) < var2 as u32) as i32 != 0 {
                    break 'label0;
                }
                let var7 = self.func6(env);
                let var8 = self.func5(env, var4, var3);
                let var9 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(var8)); 0 } }
                var9;
                self.global0 = var1.wrapping_add(16);
                return (var4 as u32 as i64).wrapping_shl(32 as u32) | 0;
                break;
            }
            unreachable!();
            break;
        }
        self.func9(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    pub fn get_state(&mut self, env: Env) -> State {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func8(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        let mut slot_var0_12_i32 = self.memory.load32(var0 as usize + 12) as i32;
        let var4 = self.func5(env, slot_var0_8_i32, slot_var0_12_i32);
        var1 = var4;
        self.global0 = var0.wrapping_add(16);
        var1
    }
}

#[allow(dead_code)]
impl CustomTypesContract {
    fn func5(&mut self, env: &Env, mut arg0: i32, mut arg1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(16);
        self.global0 = var2;
        let mut slot_var2_8_i64 = ((arg1 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
        let mut slot_var2_0_i64 = ((arg0 as u32 as i64).wrapping_shl(32 as u32) | 0) as i64;
        let var5 = val_to_i64(Map::<Val, Val>::new(env).into_val(env)) /* TODO: linear memory */
        var3 = var5;
        self.global0 = var2.wrapping_add(16);
        var3
    }
    fn func6(&mut self, env: &Env) -> i64 {
        STATE
    }
    fn func7(&mut self, env: &Env, mut incr: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(16);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (incr & 255 != 0) as i32 != 0 {
                    break 'label1;
                }
                self.func8(env, var1.wrapping_add(8));
                let mut slot_var1_8_i32 = self.memory.load32(var1 as usize + 8) as i32;
                var2 = slot_var1_8_i32;
                var3 = (incr as u64).wrapping_shr(32 as u32) as i64 as i32;
                var4 = var2.wrapping_add(var3);
                if ((var4 as u32) < var2 as u32) as i32 != 0 {
                    break 'label0;
                }
                let var7 = self.func6(env);
                let var8 = self.func5(env, var4, var3);
                let var9 = match 0 /* Void */ { 0 => { env.storage().persistent().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, 1 => { env.storage().temporary().set(&val_from_i64(var7), &val_from_i64(var8)); 0 }, _ => { env.storage().instance().set(&val_from_i64(var7), &val_from_i64(var8)); 0 } }
                var9;
                self.global0 = var1.wrapping_add(16);
                return (var4 as u32 as i64).wrapping_shl(32 as u32) | 0;
                break;
            }
            unreachable!();
            break;
        }
        self.func9(env);
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func8(&mut self, env: &Env, mut arg0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(16);
        self.global0 = var1;
        var2 = 0;
        var3 = 0;
        'label0: loop {
            'label1: loop {
                let var7 = self.func6(env);
                var4 = var7;
                let var8 = match 0 /* Void */ { 0 => { if env.storage().persistent().has(&val_from_i64(var4)) { 1 } else { 0 } }, 1 => { if env.storage().temporary().has(&val_from_i64(var4)) { 1 } else { 0 } }, _ => { if env.storage().instance().has(&val_from_i64(var4)) { 1 } else { 0 } } }
                if (var8 != 1 /* True */) as i32 != 0 {
                    break 'label1;
                }
                let var9 = match 0 /* Void */ { 0 => { val_to_i64(env.storage().persistent().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) }, 1 => { val_to_i64(env.storage().temporary().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) }, _ => { val_to_i64(env.storage().instance().get::<_, Val>(&val_from_i64(var4)).unwrap_or(val_from_i64(0))) } }
                var4 = var9;
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
                if (!(Map::<Val, Val>::try_from_val(env, &val_from_i64(var4)).is_ok())) as i32 != 0 {
                    break 'label0;
                }
                let var10 = 0 /* TODO: map_unpack_to_linear_memory */
                var10;
                let mut slot_var1_0_i64 = self.memory.load64(var1 as usize) as i64;
                var4 = slot_var1_0_i64;
                if (var4 & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                let mut slot_var1_8_i64 = self.memory.load64(var1 as usize + 8) as i64;
                var5 = slot_var1_8_i64;
                if (var5 & 255 != 0) as i32 != 0 {
                    break 'label0;
                }
                var2 = (var4 as u64).wrapping_shr(32 as u32) as i64 as i32;
                var3 = (var5 as u64).wrapping_shr(32 as u32) as i64 as i32;
                break;
            }
            self.memory.store32(arg0 as usize + 4, var3 as u32);
            self.memory.store32(arg0 as usize, var2 as u32);
            self.global0 = var1.wrapping_add(16);
            return;
            break;
        }
        unreachable!();
    }
    fn func9(&mut self, env: &Env) {
        self.func11(env);
        unreachable!();
    }
    fn func10(&mut self, env: &Env) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(16);
        self.global0 = var0;
        self.func8(env, var0.wrapping_add(8));
        let mut slot_var0_8_i32 = self.memory.load32(var0 as usize + 8) as i32;
        let mut slot_var0_12_i32 = self.memory.load32(var0 as usize + 12) as i32;
        let var4 = self.func5(env, slot_var0_8_i32, slot_var0_12_i32);
        var1 = var4;
        self.global0 = var0.wrapping_add(16);
        var1
    }
    fn func11(&mut self, env: &Env) {
        unreachable!();
    }
}
