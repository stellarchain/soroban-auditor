#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

pub trait Imports {
    type Memory: Memory;
    fn extend_contract_data_ttl(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType, threshold: U32Val, extend_to: U32Val) -> Void;
    fn get_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType) -> Val;
    fn put_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, v: Val, t: StorageType) -> Void;
    fn require_auth(&mut self, context: &mut Context<Self::Memory>, address: AddressObject) -> Void;
    fn contract_event(&mut self, context: &mut Context<Self::Memory>, topics: VecObject, data: Val) -> Void;
    fn vec_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, vals_pos: U32Val, len: U32Val) -> VecObject;
    fn obj_to_i128_hi64(&mut self, context: &mut Context<Self::Memory>, obj: I128Object) -> i64;
    fn obj_to_i128_lo64(&mut self, context: &mut Context<Self::Memory>, obj: I128Object) -> u64;
    fn obj_from_i128_pieces(&mut self, context: &mut Context<Self::Memory>, hi: i64, lo: u64) -> I128Object;
    fn symbol_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, lm_pos: U32Val, len: U32Val) -> SymbolObject;
    fn map_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, keys_pos: U32Val, vals_pos: U32Val, len: U32Val) -> MapObject;
    fn map_unpack_to_linear_memory(&mut self, context: &mut Context<Self::Memory>, map: MapObject, keys_pos: U32Val, vals_pos: U32Val, len: U32Val) -> Void;
    fn get_ledger_sequence(&mut self, context: &mut Context<Self::Memory>) -> U32Val;
    fn has_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType) -> Bool;
    fn extend_current_contract_instance_and_code_ttl(&mut self, context: &mut Context<Self::Memory>, threshold: U32Val, extend_to: U32Val) -> Void;
}

pub trait Memory {
    fn load8(&mut self, addr: usize) -> u8;
    fn load16(&mut self, addr: usize) -> u16;
    fn load32(&mut self, addr: usize) -> u32;
    fn load64(&mut self, addr: usize) -> u64;

    fn store8(&mut self, addr: usize, val: u8);
    fn store16(&mut self, addr: usize, val: u16);
    fn store32(&mut self, addr: usize, val: u32);
    fn store64(&mut self, addr: usize, val: u64);

    fn store_slice(&mut self, addr: usize, val: &'static [u8]);

    fn grow(&mut self, pages: usize) -> i32;
    fn size(&mut self) -> i32;
}

#[contract]
pub struct Contract<I: Imports<Memory = M>, M: Memory> {
    pub imports: I,
    pub context: Context<M>,
}

#[contracttype]
pub struct Context<M: Memory> {
    pub memory: M,
    global0: i32,
}

#[contracttype]
pub mod Consts {
    pub const __data_end: i32 = 1048840;
    pub const __heap_base: i32 = 1048848;
}

#[contractimpl]
impl<I: Imports<Memory = M>, M: Memory> Contract<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        let current_pages = memory.size() as usize;
        if current_pages < 17 {
            memory.grow(17 - current_pages);
            assert_eq!(memory.size(), 17, "Not enough memory pages allocated");
        }
        let mut instance = Self {
            imports,
            context: Context {
                memory,
                global0: 1048576,
            },
        };
        instance.context.memory.store_slice(1048576, b"attempt to subtract with overflow");
        instance.context.memory.store_slice(1048624, b"attempt to add with overflowfromspender\0L\0\0\0\0\0P\0\0\0\0\0amountexpiration_ledger\0h\0\0\0\0\0n\0\0\0\0\0AllowanceBalanceStateAdmincalled `Option::unwrap()` on a `None` valueapprovedecimalnamesymbol\0\0\0\xDC\0\0\0\0\0\xE3\0\0\0\0\0\xE7\0\0");
        instance
    }
    pub fn initialize(&mut self, admin: soroban_sdk :: Address, decimal: u32, name: soroban_sdk :: String, symbol: soroban_sdk :: String) -> i64 {
        self.context.func36(&mut self.imports, admin, decimal, name, symbol)
    }
    pub fn mint(&mut self, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.context.func37(&mut self.imports, to, amount)
    }
    pub fn set_admin(&mut self, new_admin: soroban_sdk :: Address) -> i64 {
        self.context.func40(&mut self.imports, new_admin)
    }
    pub fn allowance(&mut self, from: soroban_sdk :: Address, spender: soroban_sdk :: Address) -> i128 {
        self.context.func42(&mut self.imports, from, spender)
    }
    pub fn approve(&mut self, from: soroban_sdk :: Address, spender: soroban_sdk :: Address, amount: i128, expiration_ledger: u32) -> i64 {
        self.context.func43(&mut self.imports, from, spender, amount, expiration_ledger)
    }
    pub fn balance(&mut self, id: soroban_sdk :: Address) -> i128 {
        self.context.func44(&mut self.imports, id)
    }
    pub fn transfer(&mut self, from: soroban_sdk :: Address, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.context.func45(&mut self.imports, from, to, amount)
    }
    pub fn transfer_from(&mut self, spender: soroban_sdk :: Address, from: soroban_sdk :: Address, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.context.func47(&mut self.imports, spender, from, to, amount)
    }
    pub fn burn(&mut self, from: soroban_sdk :: Address, amount: i128) -> i64 {
        self.context.func48(&mut self.imports, from, amount)
    }
    pub fn burn_from(&mut self, spender: soroban_sdk :: Address, from: soroban_sdk :: Address, amount: i128) -> i64 {
        self.context.func49(&mut self.imports, spender, from, amount)
    }
    pub fn decimals(&mut self) -> u32 {
        self.context.func50(&mut self.imports)
    }
    pub fn name(&mut self) -> soroban_sdk :: String {
        self.context.func52(&mut self.imports)
    }
    pub fn symbol(&mut self) -> soroban_sdk :: String {
        self.context.func53(&mut self.imports)
    }
    pub fn __(&mut self) {
        self.context.func54(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, admin: soroban_sdk :: Address, decimal: u32, name: soroban_sdk :: String, symbol: soroban_sdk :: String) -> i64 {
        self.func36(imports, admin, decimal, name, symbol)
    }
    pub fn mint<I: Imports<Memory = M>>(&mut self, imports: &mut I, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.func37(imports, to, amount)
    }
    pub fn set_admin<I: Imports<Memory = M>>(&mut self, imports: &mut I, new_admin: soroban_sdk :: Address) -> i64 {
        self.func40(imports, new_admin)
    }
    pub fn allowance<I: Imports<Memory = M>>(&mut self, imports: &mut I, from: soroban_sdk :: Address, spender: soroban_sdk :: Address) -> i128 {
        self.func42(imports, from, spender)
    }
    pub fn approve<I: Imports<Memory = M>>(&mut self, imports: &mut I, from: soroban_sdk :: Address, spender: soroban_sdk :: Address, amount: i128, expiration_ledger: u32) -> i64 {
        self.func43(imports, from, spender, amount, expiration_ledger)
    }
    pub fn balance<I: Imports<Memory = M>>(&mut self, imports: &mut I, id: soroban_sdk :: Address) -> i128 {
        self.func44(imports, id)
    }
    pub fn transfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, from: soroban_sdk :: Address, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.func45(imports, from, to, amount)
    }
    pub fn transfer_from<I: Imports<Memory = M>>(&mut self, imports: &mut I, spender: soroban_sdk :: Address, from: soroban_sdk :: Address, to: soroban_sdk :: Address, amount: i128) -> i64 {
        self.func47(imports, spender, from, to, amount)
    }
    pub fn burn<I: Imports<Memory = M>>(&mut self, imports: &mut I, from: soroban_sdk :: Address, amount: i128) -> i64 {
        self.func48(imports, from, amount)
    }
    pub fn burn_from<I: Imports<Memory = M>>(&mut self, imports: &mut I, spender: soroban_sdk :: Address, from: soroban_sdk :: Address, amount: i128) -> i64 {
        self.func49(imports, spender, from, amount)
    }
    pub fn decimals<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> u32 {
        self.func50(imports)
    }
    pub fn name<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> soroban_sdk :: String {
        self.func52(imports)
    }
    pub fn symbol<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> soroban_sdk :: String {
        self.func53(imports)
    }
    pub fn __<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func54(imports)
    }
    fn func15<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) {
        self.func16(imports, arg0, 1i64, 501120i32, 518400i32);
    }
    fn func16<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i32, mut arg3: i32) {
        let var4 = self.func17(imports, arg0);
        let var5 = imports.extend_contract_data_ttl(self, var4, arg1, (arg2 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg3 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var5;
    }
    fn func17<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(16i32);
        self.global0 = var1;
        let var4: i64;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var5 = self.memory.load32(arg0 as usize) as i32;
                            match var5.wrapping_sub(1i32) {
                                0 => break 'label4,
                                1 => break 'label3,
                                2 => break 'label2,
                                _ => break 'label1,
                            }
                            break;
                        }
                        let var6 = self.func18(imports, 1048729i32, 7i32);
                        let var7 = self.memory.load64(arg0 as usize + 8) as i64;
                        let var8 = self.func19(imports, var6, var7);
                        var4 = var8;
                        break 'label0;
                        break;
                    }
                    let var9 = self.func18(imports, 1048736i32, 5i32);
                    let var10 = self.memory.load64(arg0 as usize + 8) as i64;
                    let var11 = self.func19(imports, var9, var10);
                    var4 = var11;
                    break 'label0;
                    break;
                }
                let var12 = self.func18(imports, 1048741i32, 5i32);
                self.memory.store64(var1 as usize, var12 as u64);
                let var13 = self.func20(imports, var1, 1i32);
                var4 = var13;
                break 'label0;
                break;
            }
            let var14 = self.func18(imports, 1048720i32, 9i32);
            let var15 = self.memory.load64(arg0.wrapping_add(16i32) as usize) as i64;
            self.memory.store64(var1 as usize + 8, var15 as u64);
            let var16 = self.memory.load64(arg0 as usize + 8) as i64;
            self.memory.store64(var1 as usize, var16 as u64);
            let var17 = self.func21(imports, 1048664i32, 2i32, var1, 2i32);
            let var18 = self.func19(imports, var14, var17);
            var4 = var18;
            break;
        }
        self.global0 = var1.wrapping_add(16i32);
        var4
    }
    fn func18<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        'label0: loop {
            if (arg1 as u32 > 9i32 as u32) as i32 != 0 {
                break 'label0;
            }
            var3 = arg1;
            var4 = arg0;
            'label1: loop {
                if var3 != 0 {
                    let var6: i32;
                    'label2: loop {
                        let var7 = self.memory.load8(var4 as usize) as i32;
                        var2 = var7;
                        let var8 = 1i32;
                        if (var2 == 95i32) as i32 != 0 {
                            var6 = var8;
                            break 'label2;
                        }
                        var8;
                        if ((var2.wrapping_sub(48i32) & 255i32) as u32 >= 10i32 as u32) as i32 != 0 {
                            if ((var2.wrapping_sub(65i32) & 255i32) as u32 >= 26i32 as u32) as i32 != 0 {
                                if ((var2.wrapping_sub(97i32) & 255i32) as u32 > 25i32 as u32) as i32 != 0 {
                                    break 'label0;
                                }
                                var6 = var2.wrapping_sub(59i32);
                                break 'label2;
                            }
                            var6 = var2.wrapping_sub(53i32);
                            break 'label2;
                        }
                        var6 = var2.wrapping_sub(46i32);
                        break;
                    }
                    var5 = var6 as u32 as i64 & 255i64 | var5.wrapping_shl(6i64 as u32);
                    var3 = var3.wrapping_sub(1i32);
                    var4 = var4.wrapping_add(1i32);
                    continue 'label1;
                }
                break;
            }
            return var5.wrapping_shl(8i64 as u32) | 14i64;
            break;
        }
        let var9 = imports.symbol_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var9
    }
    fn func19<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16i32);
        self.global0 = var2;
        self.memory.store64(var2 as usize + 8, arg1 as u64);
        self.memory.store64(var2 as usize, arg0 as u64);
        let var4 = self.func20(imports, var2, 2i32);
        self.global0 = var2.wrapping_add(16i32);
        var4
    }
    fn func20<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = imports.vec_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var2
    }
    fn func21<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
        if (arg1 != arg3) as i32 != 0 {
            unreachable!();
        }
        let var4 = imports.map_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg2 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var4
    }
    fn func22<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32i32);
        self.global0 = var0;
        self.memory.store64(var0 as usize + 8, 3i64 as u64);
        'label0: loop {
            let var3 = self.func17(imports, var0.wrapping_add(8i32));
            var1 = var3;
            let var4 = self.func23(imports, var1, 2i64);
            if var4 != 0 {
                let var5 = imports.get_contract_data(self, var1, 2i64);
                var1 = var5;
                if (var1 & 255i64 == 77i64) as i32 != 0 {
                    break 'label0;
                }
                unreachable!();
            }
            unreachable!();
            break;
        }
        self.global0 = var0.wrapping_add(32i32);
        var1
    }
    fn func23<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = imports.has_contract_data(self, arg0, arg1);
        (var2 == 1i64) as i32
    }
    fn func24<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32i32);
        self.global0 = var1;
        self.memory.store64(var1 as usize + 8, 3i64 as u64);
        let var3 = self.func17(imports, var1.wrapping_add(8i32));
        let var4 = imports.put_contract_data(self, var3, arg0, 2i64);
        var4;
        self.global0 = var1.wrapping_add(32i32);
    }
    fn func25<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_add(-64i32);
        self.global0 = var3;
        self.memory.store64(var3.wrapping_add(16i32) as usize, arg2 as u64);
        self.memory.store64(var3 as usize + 8, arg1 as u64);
        var2 = 0i64;
        self.memory.store64(var3 as usize, 0i64 as u64);
        'label0: loop {
            'label1: loop {
                let var7 = self.func17(imports, var3);
                var1 = var7;
                let var8 = self.func23(imports, var1, 0i64);
                if (var8 == 0) as i32 != 0 {
                    var1 = 0i64;
                    break 'label1;
                }
                let var9 = imports.get_contract_data(self, arg1, 0i64);
                var1 = var9;
                'label2: loop {
                    if (var4 != 16i32) as i32 != 0 {
                        self.memory.store64(var3.wrapping_add(24i32).wrapping_add(var4) as usize, 2i64 as u64);
                        var4 = var4.wrapping_add(8i32);
                        continue 'label2;
                    }
                    break;
                }
                if (arg1 & 255i64 != 76i64) as i32 != 0 {
                    break 'label0;
                }
                self.func26(imports, arg1, 1048704i32, 2i32, var3.wrapping_add(24i32), 2i32);
                let var11 = self.memory.load64(var3 as usize + 24) as i64;
                self.func27(imports, var3.wrapping_add(40i32), var11);
                let var13 = self.memory.load64(var3 as usize + 40) as i64;
                if ((var13 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var14 = self.memory.load64(var3 as usize + 32) as i64;
                var1 = var14;
                if (var1 & 255i64 != 4i64) as i32 != 0 {
                    break 'label0;
                }
                let var15 = self.memory.load64(var3 as usize + 48) as i64;
                var2 = var15;
                let var16 = self.memory.load64(var3.wrapping_add(56i32) as usize) as i64;
                let var17 = self.func28(imports);
                var4 = (arg1 as u64).wrapping_shr(32i64 as u32) as i64 as i32;
                var5 = (var17 as u32 > var4 as u32) as i32;
                var1 = { let a = 0i64; let b = var16; if var5 != 0 { a } else { b } };
                var2 = { let a = 0i64; let b = arg2; if var5 != 0 { a } else { b } };
                break;
            }
            self.memory.store64(arg0 as usize + 8, arg1 as u64);
            self.memory.store64(arg0 as usize, arg2 as u64);
            self.memory.store32(arg0 as usize + 16, var4 as u32);
            self.global0 = var3.wrapping_sub(-64i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func26<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        if (arg2 != arg4) as i32 != 0 {
            unreachable!();
        }
        let var5 = imports.map_unpack_to_linear_memory(self, arg0, (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg3 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (arg2 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var5;
    }
    fn func27<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        'label0: loop {
            'label1: loop {
                var2 = arg1 as i32 & 255i32;
                if (var2 != 69i32) as i32 != 0 {
                    if (var2 == 11i32) as i32 != 0 {
                        self.memory.store64(arg0.wrapping_add(16i32) as usize, arg1.wrapping_shr(63i64 as u32) as u64);
                        self.memory.store64(arg0 as usize + 8, arg1.wrapping_shr(8i64 as u32) as u64);
                        break 'label1;
                    }
                    self.memory.store64(arg0 as usize + 8, 34359740419i64 as u64);
                    var4 = 1i64;
                    break 'label0;
                }
                let var5 = imports.obj_to_i128_hi64(self, arg1);
                var3 = var5;
                let var6 = imports.obj_to_i128_lo64(self, arg1);
                var1 = var6;
                self.memory.store64(arg0.wrapping_add(16i32) as usize, var3 as u64);
                self.memory.store64(arg0 as usize + 8, arg1 as u64);
                break;
            }
            var4 = 0i64;
            break;
        }
        self.memory.store64(arg0 as usize, var4 as u64);
    }
    fn func28<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = imports.get_ledger_sequence(self);
        (var0 as u64).wrapping_shr(32i64 as u32) as i64 as i32
    }
    fn func29<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i32) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(80i32);
        self.global0 = var5;
        'label0: loop {
            var6 = { let a = (arg2 != 0i64) as i32; let b = (arg3 > 0i64) as i32; if (arg3 == 0) as i32 != 0 { a } else { b } };
            if (var6 == 0) as i32 != 0 {
                break 'label0;
            }
            let var8 = self.func28(imports);
            if (var8 as u32 <= arg4 as u32) as i32 != 0 {
                break 'label0;
            }
            unreachable!();
            break;
        }
        self.memory.store64(var5.wrapping_add(32i32) as usize, arg1 as u64);
        self.memory.store64(var5 as usize + 24, arg0 as u64);
        self.memory.store64(var5 as usize + 16, 0i64 as u64);
        self.memory.store64(var5.wrapping_add(56i32) as usize, arg1 as u64);
        self.memory.store64(var5 as usize + 48, arg0 as u64);
        self.memory.store64(var5 as usize + 40, 0i64 as u64);
        let var9 = self.func17(imports, var5.wrapping_add(40i32));
        self.func30(imports, var5, arg2, arg3);
        self.memory.store64(var5 as usize + 72, ((arg4 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64) as u64);
        let var11 = self.memory.load64(var5 as usize + 8) as i64;
        self.memory.store64(var5 as usize + 64, var11 as u64);
        let var12 = self.func21(imports, 1048704i32, 2i32, var5.wrapping_sub(-64i32), 2i32);
        let var13 = imports.put_contract_data(self, var9, var12, 0i64);
        var13;
        if var6 != 0 {
            let var14 = self.func28(imports);
            var6 = var14;
            if (var6 as u32 > arg4 as u32) as i32 != 0 {
                unreachable!();
            }
            var4 = arg4.wrapping_sub(var6);
            self.func16(imports, var5.wrapping_add(16i32), 0i64, var4, arg4);
        }
        self.global0 = var5.wrapping_add(80i32);
    }
    fn func30<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let var3: i64;
        if (arg2 ^ arg1.wrapping_shr(63i64 as u32) != 0i64) as i32 | (arg1.wrapping_sub(-36028797018963968i64) as u64 > 72057594037927935i64 as u64) as i32 != 0 {
            let var4 = imports.obj_from_i128_pieces(self, arg2, arg1);
            var3 = var4;
        } else {
            var3 = arg1.wrapping_shl(8i64 as u32) | 11i64;
        }
        self.memory.store64(arg0 as usize + 8, var3 as u64);
        self.memory.store64(arg0 as usize, 0i64 as u64);
    }
    fn func31<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(48i32);
        self.global0 = var2;
        self.memory.store64(var2 as usize, 1i64 as u64);
        self.memory.store64(var2 as usize + 8, arg1 as u64);
        var1 = 0i64;
        'label0: loop {
            let var6 = self.func17(imports, var2);
            var3 = var6;
            let var7 = self.func23(imports, var3, 1i64);
            if var7 != 0 {
                let var8 = imports.get_contract_data(self, var3, 1i64);
                self.func27(imports, var2.wrapping_add(24i32), var8);
                let var10 = self.memory.load64(var2 as usize + 24) as i64;
                if ((var10 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var11 = self.memory.load64(var2.wrapping_add(40i32) as usize) as i64;
                var4 = var11;
                let var12 = self.memory.load64(var2 as usize + 32) as i64;
                var1 = var12;
                self.func15(imports, var2);
            }
            self.memory.store64(arg0 as usize + 8, var4 as u64);
            self.memory.store64(arg0 as usize, arg1 as u64);
            self.global0 = var2.wrapping_add(48i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func32<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(48i32);
        self.global0 = var3;
        self.memory.store64(var3 as usize + 24, 1i64 as u64);
        self.memory.store64(var3 as usize + 32, arg0 as u64);
        var4 = var3.wrapping_add(24i32);
        let var6 = self.func17(imports, var4);
        self.func30(imports, var3.wrapping_add(8i32), arg1, arg2);
        let var8 = self.memory.load64(var3 as usize + 16) as i64;
        let var9 = imports.put_contract_data(self, var6, var8, 1i64);
        var9;
        self.func15(imports, var4);
        self.global0 = var3.wrapping_add(48i32);
    }
    fn func33<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var3 = var6.wrapping_sub(16i32);
        self.global0 = var3;
        self.func31(imports, var3, arg0);
        let var8 = self.memory.load64(var3.wrapping_add(8i32) as usize) as i64;
        var4 = var8;
        let arg9 = arg2;
        let var10 = self.memory.load64(var3 as usize) as i64;
        var5 = var10;
        var1 = arg1.wrapping_add(var5);
        var2 = (((var1 as u64) < var5 as u64) as i32 as u32 as i64).wrapping_add(arg2.wrapping_add(var4));
        if ((var4 ^ arg9 ^ -1i64) & (var4 ^ var2) >= 0i64) as i32 != 0 {
            self.func32(imports, arg0, arg1, arg2);
            self.global0 = var3.wrapping_add(16i32);
            return;
        }
        unreachable!();
    }
    fn func34<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var3 = var7.wrapping_sub(16i32);
        self.global0 = var3;
        self.func31(imports, var3, arg0);
        'label0: loop {
            let var9 = self.memory.load64(var3 as usize) as i64;
            var6 = var9;
            var4 = ((var6 as u64) < arg1 as u64) as i32;
            let var10 = self.memory.load64(var3.wrapping_add(8i32) as usize) as i64;
            var5 = var10;
            if (({ let a = var4; let b = (var5 < arg2) as i32; if (arg2 == var5) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let arg11 = arg2;
                var2 = var5.wrapping_sub(arg2).wrapping_sub(var4 as u32 as i64);
                if ((arg11 ^ var5) & (var5 ^ var2) >= 0i64) as i32 != 0 {
                    break 'label0;
                }
                unreachable!();
            }
            unreachable!();
            break;
        }
        self.func32(imports, arg0, var6.wrapping_sub(arg1), arg2);
        self.global0 = var3.wrapping_add(16i32);
    }
    fn func35<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) {
        if (arg1 >= 0i64) as i32 != 0 {
            return;
        }
        unreachable!();
    }
    fn func36<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(32i32);
        self.global0 = var4;
        'label0: loop {
            if ((arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 4i64) as i32 | (arg2 & 255i64 != 73i64) as i32 | (arg3 & 255i64 != 73i64) as i32 == 0) as i32 != 0 {
                self.memory.store64(var4 as usize + 8, 3i64 as u64);
                let var6 = self.func17(imports, var4.wrapping_add(8i32));
                let var7 = self.func23(imports, var6, 2i64);
                if var7 != 0 {
                    break 'label0;
                }
                self.func24(imports, arg0);
                if (arg1 as u64 >= 1099511627776i64 as u64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var4 as usize + 24, arg3 as u64);
                self.memory.store64(var4 as usize + 16, arg2 as u64);
                self.memory.store64(var4 as usize + 8, (arg1 & -4294967296i64 | 4i64) as u64);
                let var9 = self.func21(imports, 1048816i32, 3i32, var4.wrapping_add(8i32), 3i32);
                let var10 = imports.put_contract_data(self, 27311646515383310i64, var9, 2i64);
                var10;
                self.global0 = var4.wrapping_add(32i32);
                return 2i64;
            }
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func37<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(48i32);
        self.global0 = var2;
        'label0: loop {
            if (arg0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func27(imports, var2.wrapping_add(24i32), arg1);
            let var7 = self.memory.load64(var2 as usize + 24) as i64;
            if ((var7 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var8 = self.memory.load64(var2 as usize + 32) as i64;
            var1 = var8;
            let var9 = self.memory.load64(var2.wrapping_add(40i32) as usize) as i64;
            var3 = var9;
            self.func35(imports, var1, var3);
            let var11 = self.func22(imports);
            var4 = var11;
            let var12 = imports.require_auth(self, var4);
            var12;
            self.func38(imports);
            self.func33(imports, arg0, arg1, var3);
            self.memory.store64(var2 as usize + 40, arg0 as u64);
            self.memory.store64(var2 as usize + 32, var4 as u64);
            self.memory.store64(var2 as usize + 24, 3404527886i64 as u64);
            let var15 = self.func39(imports, var2.wrapping_add(24i32));
            self.func30(imports, var2.wrapping_add(8i32), arg1, var3);
            let var17 = self.memory.load64(var2 as usize + 16) as i64;
            let var18 = imports.contract_event(self, var15, var17);
            var18;
            self.global0 = var2.wrapping_add(48i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func38<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = imports.extend_current_contract_instance_and_code_ttl(self, 445302209249284i64, 519519244124164i64);
        var0;
    }
    fn func39<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(48i32);
        self.global0 = var1;
        let var4 = self.memory.load64(arg0 as usize + 16) as i64;
        self.memory.store64(var1 as usize + 16, var4 as u64);
        let var5 = self.memory.load64(arg0 as usize + 8) as i64;
        self.memory.store64(var1 as usize + 8, var5 as u64);
        let var6 = self.memory.load64(arg0 as usize) as i64;
        self.memory.store64(var1 as usize, var6 as u64);
        var0 = 0i32;
        let var7: i64;
        'label0: loop {
            let var8: i64;
            if (arg0 == 24i32) as i32 != 0 {
                var0 = 0i32;
                'label1: loop {
                    if (arg0 != 24i32) as i32 != 0 {
                        let var9 = self.memory.load64(arg0.wrapping_add(var1) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(24i32).wrapping_add(arg0) as usize, var9 as u64);
                        var0 = arg0.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                let var10 = self.func20(imports, var1.wrapping_add(24i32), 3i32);
                self.global0 = var1.wrapping_add(48i32);
                var8 = var10;
            } else {
                self.memory.store64(var1.wrapping_add(24i32).wrapping_add(arg0) as usize, 2i64 as u64);
                var0 = arg0.wrapping_add(8i32);
                continue 'label0;
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
    fn func40<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i64 = 0;
        if (arg0 & 255i64 != 77i64) as i32 != 0 {
            unreachable!();
        }
        let var2 = self.func22(imports);
        var1 = var2;
        let var3 = imports.require_auth(self, var1);
        var3;
        self.func38(imports);
        self.func24(imports, arg0);
        let var6 = self.func41(imports, 4083516257707209486i64, var1);
        let var7 = imports.contract_event(self, var6, arg0);
        var7;
        2i64
    }
    fn func41<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(32i32);
        self.global0 = var3;
        self.memory.store64(var3 as usize + 8, arg1 as u64);
        self.memory.store64(var3 as usize, arg0 as u64);
        let var5: i64;
        'label0: loop {
            let var6: i64;
            if (var2 == 16i32) as i32 != 0 {
                var2 = 0i32;
                'label1: loop {
                    if (var2 != 16i32) as i32 != 0 {
                        let var7 = self.memory.load64(var2.wrapping_add(var3) as usize) as i64;
                        self.memory.store64(var3.wrapping_add(16i32).wrapping_add(var2) as usize, var7 as u64);
                        var2 = var2.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                let var8 = self.func20(imports, var3.wrapping_add(16i32), 2i32);
                self.global0 = var3.wrapping_add(32i32);
                var6 = var8;
            } else {
                self.memory.store64(var3.wrapping_add(16i32).wrapping_add(var2) as usize, 2i64 as u64);
                var2 = var2.wrapping_add(8i32);
                continue 'label0;
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
    fn func42<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(48i32);
        self.global0 = var2;
        if ((arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 77i64) as i32 == 0) as i32 != 0 {
            self.func38(imports);
            self.func25(imports, var2.wrapping_add(24i32), arg0, arg1);
            let var6 = self.memory.load64(var2 as usize + 24) as i64;
            let var7 = self.memory.load64(var2.wrapping_add(32i32) as usize) as i64;
            self.func30(imports, var2.wrapping_add(8i32), var6, var7);
            let var9 = self.memory.load64(var2 as usize + 16) as i64;
            self.global0 = var2.wrapping_add(48i32);
            return var9;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func43<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var4 = var7.wrapping_add(-64i32);
        self.global0 = var4;
        'label0: loop {
            if (arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func27(imports, var4.wrapping_add(24i32), arg2);
            let var9 = self.memory.load64(var4 as usize + 24) as i64;
            if ((var9 == 0) as i32 == 0) as i32 | (arg3 & 255i64 != 4i64) as i32 != 0 {
                break 'label0;
            }
            let var10 = self.memory.load64(var4.wrapping_add(40i32) as usize) as i64;
            var2 = var10;
            let var11 = self.memory.load64(var4 as usize + 32) as i64;
            var5 = var11;
            let var12 = imports.require_auth(self, arg0);
            var12;
            self.func35(imports, var5, arg2);
            self.func38(imports);
            self.func29(imports, arg0, arg1, var5, arg2, (arg3 as u64).wrapping_shr(32i64 as u32) as i64 as i32);
            let var16 = self.func18(imports, 1048789i32, 7i32);
            var6 = var16;
            self.memory.store64(var4 as usize + 40, arg1 as u64);
            self.memory.store64(var4 as usize + 32, arg0 as u64);
            self.memory.store64(var4 as usize + 24, var6 as u64);
            let var17 = self.func39(imports, var4.wrapping_add(24i32));
            self.func30(imports, var4.wrapping_add(8i32), var5, arg2);
            self.memory.store64(var4 as usize + 56, (arg3 & -4294967296i64 | 4i64) as u64);
            let var19 = self.memory.load64(var4 as usize + 16) as i64;
            self.memory.store64(var4 as usize + 48, var19 as u64);
            let var20 = self.func20(imports, var4.wrapping_add(48i32), 2i32);
            let var21 = imports.contract_event(self, var17, var20);
            var21;
            self.global0 = var4.wrapping_sub(-64i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func44<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(32i32);
        self.global0 = var1;
        if (arg0 & 255i64 != 77i64) as i32 != 0 {
            unreachable!();
        }
        self.func38(imports);
        self.func31(imports, var1.wrapping_add(16i32), arg0);
        let var5 = self.memory.load64(var1 as usize + 16) as i64;
        let var6 = self.memory.load64(var1.wrapping_add(24i32) as usize) as i64;
        self.func30(imports, var1, var5, var6);
        let var8 = self.memory.load64(var1 as usize + 8) as i64;
        self.global0 = var1.wrapping_add(32i32);
        var8
    }
    fn func45<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var3 = var5.wrapping_sub(32i32);
        self.global0 = var3;
        'label0: loop {
            if (arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func27(imports, var3.wrapping_add(8i32), arg2);
            let var7 = self.memory.load64(var3 as usize + 8) as i64;
            if ((var7 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var8 = self.memory.load64(var3.wrapping_add(24i32) as usize) as i64;
            var2 = var8;
            let var9 = self.memory.load64(var3 as usize + 16) as i64;
            var4 = var9;
            let var10 = imports.require_auth(self, arg0);
            var10;
            self.func35(imports, var4, arg2);
            self.func38(imports);
            self.func34(imports, arg0, var4, arg2);
            self.func33(imports, arg1, var4, arg2);
            self.func46(imports, arg0, arg1, var4, arg2);
            self.global0 = var3.wrapping_add(32i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func46<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) {
        let mut var4: i32 = 0;
        let var5 = self.global0;
        var4 = var5.wrapping_sub(48i32);
        self.global0 = var4;
        self.memory.store64(var4 as usize + 40, arg1 as u64);
        self.memory.store64(var4 as usize + 32, arg0 as u64);
        self.memory.store64(var4 as usize + 24, 65154533130155790i64 as u64);
        let var6 = self.func39(imports, var4.wrapping_add(24i32));
        self.func30(imports, var4.wrapping_add(8i32), arg2, arg3);
        let var8 = self.memory.load64(var4 as usize + 16) as i64;
        let var9 = imports.contract_event(self, var6, var8);
        var9;
        self.global0 = var4.wrapping_add(48i32);
    }
    fn func47<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let var9 = self.global0;
        var4 = var9.wrapping_sub(48i32);
        self.global0 = var4;
        'label0: loop {
            'label1: loop {
                if (arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 77i64) as i32 | (arg2 & 255i64 != 77i64) as i32 != 0 {
                    break 'label1;
                }
                self.func27(imports, var4.wrapping_add(24i32), arg3);
                let var11 = self.memory.load64(var4 as usize + 24) as i64;
                if ((var11 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var12 = self.memory.load64(var4.wrapping_add(40i32) as usize) as i64;
                var3 = var12;
                let var13 = self.memory.load64(var4 as usize + 32) as i64;
                var6 = var13;
                let var14 = imports.require_auth(self, arg0);
                var14;
                self.func35(imports, var6, arg3);
                self.func38(imports);
                self.func25(imports, var4, arg1, arg0);
                let var18 = self.memory.load64(var4 as usize) as i64;
                var8 = var18;
                var5 = ((var8 as u64) < var6 as u64) as i32;
                let var19 = self.memory.load64(var4.wrapping_add(8i32) as usize) as i64;
                var7 = var19;
                if ({ let a = var5; let b = (var7 < arg3) as i32; if (arg3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                if ({ let a = (var6 != 0i64) as i32; let b = (arg3 > 0i64) as i32; if (arg3 == 0) as i32 != 0 { a } else { b } }) != 0 {
                    let var20 = self.memory.load32(var4 as usize + 16) as i32;
                    self.func29(imports, arg1, arg0, var8.wrapping_sub(var6), var7.wrapping_sub(arg3).wrapping_sub(var5 as u32 as i64), var20);
                }
                self.func34(imports, arg1, var6, arg3);
                self.func33(imports, arg2, var6, arg3);
                self.func46(imports, arg1, arg2, var6, arg3);
                self.global0 = var4.wrapping_add(48i32);
                return 2i64;
                break;
            }
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func48<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var2 = var4.wrapping_sub(48i32);
        self.global0 = var2;
        'label0: loop {
            if (arg0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func27(imports, var2.wrapping_add(24i32), arg1);
            let var6 = self.memory.load64(var2 as usize + 24) as i64;
            if ((var6 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var7 = self.memory.load64(var2.wrapping_add(40i32) as usize) as i64;
            var1 = var7;
            let var8 = self.memory.load64(var2 as usize + 32) as i64;
            var3 = var8;
            let var9 = imports.require_auth(self, arg0);
            var9;
            self.func35(imports, var3, arg1);
            self.func38(imports);
            self.func34(imports, arg0, var3, arg1);
            let var13 = self.func41(imports, 2678977294i64, arg0);
            self.func30(imports, var2.wrapping_add(8i32), var3, arg1);
            let var15 = self.memory.load64(var2 as usize + 16) as i64;
            let var16 = imports.contract_event(self, var13, var15);
            var16;
            self.global0 = var2.wrapping_add(48i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func49<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) -> i64 {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        var3 = var4.wrapping_sub(32i32);
        self.global0 = var3;
        'label0: loop {
            if (arg0 & 255i64 != 77i64) as i32 | (arg1 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func27(imports, var3.wrapping_add(8i32), arg2);
            let var6 = self.memory.load64(var3 as usize + 8) as i64;
            if ((var6 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var7 = self.memory.load64(var3.wrapping_add(24i32) as usize) as i64;
            var1 = var7;
            let var8 = self.memory.load64(var3 as usize + 16) as i64;
            let var9 = imports.require_auth(self, arg0);
            var9;
            self.func35(imports, var8, arg1);
            self.func38(imports);
            self.global0 = var3.wrapping_add(32i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func50<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32i32);
        self.global0 = var0;
        'label0: loop {
            let var3 = self.func23(imports, 27311646515383310i64, 2i64);
            if var3 != 0 {
                let var4 = imports.get_contract_data(self, 27311646515383310i64, 2i64);
                self.func51(imports, var0, var4);
                let var6 = self.memory.load64(var0 as usize) as i64;
                if (var6 == 0) as i32 != 0 {
                    break 'label0;
                }
            }
            unreachable!();
            break;
        }
        let var7 = self.memory.load32(var0.wrapping_add(24i32) as usize) as i64;
        self.global0 = var0.wrapping_add(32i32);
        var7.wrapping_shl(32i64 as u32) | 4i64
    }
    fn func51<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(32i32);
        self.global0 = var2;
        'label0: loop {
            if (var3 != 24i32) as i32 != 0 {
                self.memory.store64(var2.wrapping_add(8i32).wrapping_add(var3) as usize, 2i64 as u64);
                var3 = var3.wrapping_add(8i32);
                continue 'label0;
            }
            break;
        }
        'label1: loop {
            'label2: loop {
                'label3: loop {
                    if (arg1 & 255i64 == 76i64) as i32 != 0 {
                        self.func26(imports, arg1, 1048816i32, 3i32, var2.wrapping_add(8i32), 3i32);
                        let var8 = self.memory.load64(var2 as usize + 8) as i64;
                        var1 = var8;
                        if (var1 & 255i64 != 4i64) as i32 != 0 {
                            break 'label3;
                        }
                        let var9 = self.memory.load64(var2 as usize + 16) as i64;
                        var4 = var9;
                        if (var4 & 255i64 != 73i64) as i32 != 0 {
                            break 'label2;
                        }
                        let var10 = self.memory.load64(var2 as usize + 24) as i64;
                        var5 = var10;
                        if (var5 & 255i64 == 73i64) as i32 != 0 {
                            self.memory.store64(arg0 as usize + 8, var4 as u64);
                            self.memory.store64(arg0 as usize, 0i64 as u64);
                            self.memory.store32(arg0.wrapping_add(24i32) as usize, (arg1 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                            self.memory.store64(arg0.wrapping_add(16i32) as usize, var5 as u64);
                            break 'label1;
                        }
                        self.memory.store64(arg0 as usize, 1i64 as u64);
                        break 'label1;
                    }
                    self.memory.store64(arg0 as usize, 1i64 as u64);
                    break 'label1;
                    break;
                }
                self.memory.store64(arg0 as usize, 1i64 as u64);
                break 'label1;
                break;
            }
            self.memory.store64(arg0 as usize, 1i64 as u64);
            break;
        }
        self.global0 = var2.wrapping_add(32i32);
    }
    fn func52<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32i32);
        self.global0 = var0;
        'label0: loop {
            let var3 = self.func23(imports, 27311646515383310i64, 2i64);
            if var3 != 0 {
                let var4 = imports.get_contract_data(self, 27311646515383310i64, 2i64);
                self.func51(imports, var0, var4);
                let var6 = self.memory.load64(var0 as usize) as i64;
                if (var6 == 0) as i32 != 0 {
                    break 'label0;
                }
            }
            unreachable!();
            break;
        }
        let var7 = self.memory.load64(var0 as usize + 8) as i64;
        self.global0 = var0.wrapping_add(32i32);
        var7
    }
    fn func53<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32i32);
        self.global0 = var0;
        'label0: loop {
            let var3 = self.func23(imports, 27311646515383310i64, 2i64);
            if var3 != 0 {
                let var4 = imports.get_contract_data(self, 27311646515383310i64, 2i64);
                self.func51(imports, var0, var4);
                let var6 = self.memory.load64(var0 as usize) as i64;
                if (var6 == 0) as i32 != 0 {
                    break 'label0;
                }
            }
            unreachable!();
            break;
        }
        let var7 = self.memory.load64(var0.wrapping_add(16i32) as usize) as i64;
        self.global0 = var0.wrapping_add(32i32);
        var7
    }
    fn func54<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
}
