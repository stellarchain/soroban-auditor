#![allow(
    unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case,
    non_upper_case_globals, unconditional_recursion, path_statements
)]

pub const PAGE_SIZE: usize = 64 << 10;

pub trait Imports {
    type Memory: Memory;
    fn g(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
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

pub struct Instance<I: Imports<Memory = M>, M: Memory> {
    pub imports: I,
    pub context: Context<M>,
}

pub struct Context<M: Memory> {
    pub memory: M,
    global0: i32,
}

pub mod consts {
    pub const __data_end: i32 = 1048576;
    pub const __heap_base: i32 = 1048576;
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        let current_pages = memory.size() as usize;
        if current_pages < 16 {
            memory.grow(16 - current_pages);
            assert_eq!(memory.size(), 16, "Not enough memory pages allocated");
        }
        let mut instance = Self {
            imports,
            context: Context {
                memory,
                global0: 1048576,
            },
        };
        instance
    }
    pub fn hello(&mut self) -> i64 {
        self.context.func1(&mut self.imports)
    }
    pub fn hello2(&mut self, var0: i64) -> i64 {
        self.context.func2(&mut self.imports, var0)
    }
    pub fn __(&mut self) {
        self.context.func3(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn hello<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        self.func1(imports)
    }
    pub fn hello2<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64) -> i64 {
        self.func2(imports, var0)
    }
    pub fn __<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func3(imports)
    }
    fn func1<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        84475147278i64
    }
    fn func2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var1 = var3.wrapping_sub(48i32);
        self.global0 = var1;
        self.memory.store64(var1 as usize + 16, var0 as u64);
        let var4 = self.memory.load64(var1.wrapping_add(16i32) as usize) as i64;
        var0 = var4;
        self.memory.store64(var1 as usize + 8, var0 as u64);
        var2 = var0 as i32 & 255i32;
        self.memory.store64(var1 as usize, ((var2 != 74i32) as i32 & (var2 != 14i32) as i32) as u32 as i64 as u64);
        let var5 = self.memory.load64(var1 as usize) as i64;
        if (var5 as i32 == 0) as i32 != 0 {
            let var6 = self.memory.load64(var1 as usize + 8) as i64;
            self.memory.store64(var1 as usize + 32, var6 as u64);
            self.memory.store64(var1 as usize + 24, 84475147278i64 as u64);
            let var7 = imports.g(self, (var1.wrapping_add(24i32) as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, 8589934596i64);
            var0 = var7;
            self.global0 = var1.wrapping_add(48i32);
            return var0;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func3<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
}
