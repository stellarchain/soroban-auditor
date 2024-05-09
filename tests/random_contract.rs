#![allow(
    unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case,
    non_upper_case_globals, unconditional_recursion, path_statements
)]

pub const PAGE_SIZE: usize = 64 << 10;

pub trait Imports {
    type Memory: Memory;
    fn _0(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _1(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn __(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn h(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn __(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn x(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn y(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn w(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _3(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _5(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _6(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _0(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn A(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn v(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _0(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn __(&mut self, context: &mut Context<Self::Memory>) -> i64;
    fn _1(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _0(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _4(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _1(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _8(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _6(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _1(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _7(&mut self, context: &mut Context<Self::Memory>) -> i64;
    fn _3(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn g(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn _8(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _7(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _6(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn j(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn __(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn _9(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn a(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64, var3: i64) -> i64;
    fn b(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn f(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64, var2: i64) -> i64;
    fn _4(&mut self, context: &mut Context<Self::Memory>) -> i64;
    fn _0(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn e(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
    fn h(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn i(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _5(&mut self, context: &mut Context<Self::Memory>, var0: i64) -> i64;
    fn _2(&mut self, context: &mut Context<Self::Memory>, var0: i64, var1: i64) -> i64;
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
    pub const __data_end: i32 = 1049288;
    pub const __heap_base: i32 = 1049296;
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
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
        instance.context.memory.store_slice(1048576, b"initializednew_launchlaunch_funds_claimedcancel_launchbuysellclaim_tokensspace_missionContractInfoLaunchLaunchBalanceSpaceMissionadminnative_contractslz_feeslz_fee_destinationspace_feespace_missionsstability_check_durationstellarbucks_contract\0\x81\0\0\0\0\0\x86\0\0\0\0\0\x95\0\0\0\0\0\x9C\0\0\0\0\0\xAF\0\0\t\0\0\0\xB8\0\0\0\0\0\xC6\0\0\0\0\0\xDE\0\0\0\0\0assetdevfunds_claimedfunds_recipientinfomax_pricemax_supplymin_pricepool_balancestability_checkstability_check_endstellarbuckssupplytimestamptokens_claimed\04\0\0\0\09\0\0\0\0<\0\r\0\0\0I\0\0\0\0X\0\0\0\0\\\0\t\0\0\0e\0\n\0\0\0o\0\t\0\0\0x\0\0\0\0\x84\0\0\0\0\x93\0\0\0\0\xA6\0\0\0\0\xB2\0\0\0\0\xB8\0\t\0\0\0\xC1\0\0\0\0guaranteed_success_fundingrewardH\0\0\0\0b\0");
        instance.context.memory.store_slice(1049256, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFtransferburnmint");
        instance
    }
    pub fn initialize(&mut self, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64, var6: i64, var7: i64) -> i64 {
        self.context.func80(&mut self.imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn change_contract_info(&mut self, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64) -> i64 {
        self.context.func82(&mut self.imports, var0, var1, var2, var3, var4, var5)
    }
    pub fn upgrade(&mut self, var0: i64) -> i64 {
        self.context.func84(&mut self.imports, var0)
    }
    pub fn start_space_mission(&mut self, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.context.func85(&mut self.imports, var0, var1, var2, var3)
    }
    pub fn add_space_missions_reward(&mut self, var0: i64, var1: i64, var2: i64) -> i64 {
        self.context.func86(&mut self.imports, var0, var1, var2)
    }
    pub fn new_launch(&mut self, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64, var6: i64, var7: i64) -> i64 {
        self.context.func87(&mut self.imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn cancel_launch(&mut self, var0: i64) -> i64 {
        self.context.func88(&mut self.imports, var0)
    }
    pub fn buy(&mut self, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.context.func90(&mut self.imports, var0, var1, var2, var3)
    }
    pub fn sell(&mut self, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.context.func91(&mut self.imports, var0, var1, var2, var3)
    }
    pub fn claim_launch_funds(&mut self, var0: i64) -> i64 {
        self.context.func92(&mut self.imports, var0)
    }
    pub fn claim_launch_balance(&mut self, var0: i64, var1: i64) -> i64 {
        self.context.func93(&mut self.imports, var0, var1)
    }
    pub fn calculate_buy(&mut self, var0: i64, var1: i64) -> i64 {
        self.context.func94(&mut self.imports, var0, var1)
    }
    pub fn calculate_sell(&mut self, var0: i64, var1: i64) -> i64 {
        self.context.func95(&mut self.imports, var0, var1)
    }
    pub fn get_launch_data(&mut self, var0: i64) -> i64 {
        self.context.func96(&mut self.imports, var0)
    }
    pub fn get_contract_info(&mut self) -> i64 {
        self.context.func97(&mut self.imports)
    }
    pub fn get_launch_balance(&mut self, var0: i64, var1: i64) -> i64 {
        self.context.func98(&mut self.imports, var0, var1)
    }
    pub fn version(&mut self) -> i64 {
        self.context.func99(&mut self.imports)
    }
    pub fn __(&mut self) {
        self.context.func103(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64, var6: i64, var7: i64) -> i64 {
        self.func80(imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn change_contract_info<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64) -> i64 {
        self.func82(imports, var0, var1, var2, var3, var4, var5)
    }
    pub fn upgrade<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64) -> i64 {
        self.func84(imports, var0)
    }
    pub fn start_space_mission<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.func85(imports, var0, var1, var2, var3)
    }
    pub fn add_space_missions_reward<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64) -> i64 {
        self.func86(imports, var0, var1, var2)
    }
    pub fn new_launch<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64, var4: i64, var5: i64, var6: i64, var7: i64) -> i64 {
        self.func87(imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn cancel_launch<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64) -> i64 {
        self.func88(imports, var0)
    }
    pub fn buy<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.func90(imports, var0, var1, var2, var3)
    }
    pub fn sell<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64, var2: i64, var3: i64) -> i64 {
        self.func91(imports, var0, var1, var2, var3)
    }
    pub fn claim_launch_funds<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64) -> i64 {
        self.func92(imports, var0)
    }
    pub fn claim_launch_balance<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64) -> i64 {
        self.func93(imports, var0, var1)
    }
    pub fn calculate_buy<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64) -> i64 {
        self.func94(imports, var0, var1)
    }
    pub fn calculate_sell<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64) -> i64 {
        self.func95(imports, var0, var1)
    }
    pub fn get_launch_data<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64) -> i64 {
        self.func96(imports, var0)
    }
    pub fn get_contract_info<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        self.func97(imports)
    }
    pub fn get_launch_balance<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i64, var1: i64) -> i64 {
        self.func98(imports, var0, var1)
    }
    pub fn version<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        self.func99(imports)
    }
    pub fn __<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func103(imports)
    }
    fn func42<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        'label0: loop {
            var2 = var1 as i32 & 255i32;
            if (var2 != 64i32) as i32 != 0 {
                if (var2 != 6i32) as i32 != 0 {
                    var3 = 1i64;
                    var4 = 34359740419i64;
                    break 'label0;
                }
                var4 = (var1 as u64).wrapping_shr(8i64 as u32) as i64;
                break 'label0;
            }
            let var5 = imports._0(self, var1);
            var4 = var5;
            break;
        }
        self.memory.store64(var0 as usize + 8, var4 as u64);
        self.memory.store64(var0 as usize, var3 as u64);
    }
    fn func43<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(32i32);
        self.global0 = var2;
        'label0: loop {
            let var7 = self.func44(imports, var1);
            var3 = var7;
            let var8 = self.func45(imports, var3, 1i64);
            let var9: i64;
            if var8 != 0 {
                let var10 = imports._1(self, var3, 1i64);
                self.func46(imports, var2.wrapping_add(8i32), var10);
                let var11 = self.memory.load64(var2 as usize + 8) as i64;
                if ((var11 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var12 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
                var4 = var12;
                let var13 = self.memory.load64(var2 as usize + 16) as i64;
                var5 = var13;
                var9 = 1i64;
            } else {
                var9 = 0i64;
            }
            var3 = var9;
            self.memory.store64(var0 as usize + 8, var5 as u64);
            self.memory.store64(var0 as usize, var3 as u64);
            self.memory.store64(var0.wrapping_add(16i32) as usize, var4 as u64);
            self.global0 = var2.wrapping_add(32i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func44<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(32i32);
        self.global0 = var1;
        let var6: i64;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var7 = self.memory.load32(var0 as usize) as i32;
                            match var7.wrapping_sub(1i32) {
                                0 => break 'label4,
                                1 => break 'label3,
                                2 => break 'label2,
                                _ => break 'label1,
                            }
                            break;
                        }
                        let var8 = self.func64(imports, 1048674i32, 6i32);
                        var2 = var8;
                        let var9 = self.memory.load64(var0 as usize + 8) as i64;
                        var3 = var9;
                        let var10 = self.memory.load64(var0 as usize + 16) as i64;
                        let var11 = self.func61(imports, var10);
                        self.memory.store64(var1 as usize + 16, var11 as u64);
                        self.memory.store64(var1 as usize + 8, var3 as u64);
                        self.memory.store64(var1 as usize, var2 as u64);
                        let var12 = self.func59(imports, var1, 3i32);
                        var6 = var12;
                        break 'label0;
                        break;
                    }
                    let var13 = self.func64(imports, 1048680i32, 13i32);
                    var2 = var13;
                    let var14 = self.memory.load64(var0 as usize + 8) as i64;
                    var3 = var14;
                    let var15 = self.memory.load64(var0 as usize + 16) as i64;
                    let var16 = self.func61(imports, var15);
                    var4 = var16;
                    let var17 = self.memory.load64(var0 as usize + 24) as i64;
                    self.memory.store64(var1 as usize + 24, var17 as u64);
                    self.memory.store64(var1 as usize + 16, var4 as u64);
                    self.memory.store64(var1 as usize + 8, var3 as u64);
                    self.memory.store64(var1 as usize, var2 as u64);
                    let var18 = self.func59(imports, var1, 4i32);
                    var6 = var18;
                    break 'label0;
                    break;
                }
                let var19 = self.func64(imports, 1048693i32, 12i32);
                var2 = var19;
                let var20 = self.memory.load64(var0 as usize + 8) as i64;
                self.memory.store64(var1 as usize + 8, var20 as u64);
                self.memory.store64(var1 as usize, var2 as u64);
                let var21 = self.func59(imports, var1, 2i32);
                var6 = var21;
                break 'label0;
                break;
            }
            let var22 = self.func64(imports, 1048662i32, 12i32);
            self.memory.store64(var1 as usize, var22 as u64);
            let var23 = self.func59(imports, var1, 1i32);
            var6 = var23;
            break;
        }
        self.global0 = var1.wrapping_add(32i32);
        var6
    }
    fn func45<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i32 {
        let var2 = imports._0(self, var0, var1);
        (var2 == 1i64) as i32
    }
    fn func46<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        'label0: loop {
            'label1: loop {
                var2 = var1 as i32 & 255i32;
                if (var2 != 69i32) as i32 != 0 {
                    if (var2 == 11i32) as i32 != 0 {
                        self.memory.store64(var0.wrapping_add(16i32) as usize, var1.wrapping_shr(63i64 as u32) as u64);
                        self.memory.store64(var0 as usize + 8, var1.wrapping_shr(8i64 as u32) as u64);
                        break 'label1;
                    }
                    self.memory.store64(var0 as usize + 8, 34359740419i64 as u64);
                    var4 = 1i64;
                    break 'label0;
                }
                let var5 = imports._8(self, var1);
                var3 = var5;
                let var6 = imports._7(self, var1);
                var1 = var6;
                self.memory.store64(var0.wrapping_add(16i32) as usize, var3 as u64);
                self.memory.store64(var0 as usize + 8, var1 as u64);
                break;
            }
            var4 = 0i64;
            break;
        }
        self.memory.store64(var0 as usize, var4 as u64);
    }
    fn func47<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let var25 = self.global0;
        var2 = var25.wrapping_sub(176i32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                let var26 = self.func44(imports, var1);
                var5 = var26;
                let var27 = self.func45(imports, var5, 1i64);
                if (var27 == 0) as i32 != 0 {
                    self.memory.store8(var0 as usize + 161, 2i32 as u8);
                    break 'label1;
                }
                let var28 = imports._1(self, var5, 1i64);
                var5 = var28;
                var1 = 0i32;
                'label2: loop {
                    if (var1 != 120i32) as i32 != 0 {
                        self.memory.store64(var2.wrapping_add(32i32).wrapping_add(var1) as usize, 2i64 as u64);
                        var1 = var1.wrapping_add(8i32);
                        continue 'label2;
                    }
                    break;
                }
                if (var5 & 255i64 != 76i64) as i32 != 0 {
                    break 'label0;
                }
                self.func48(imports, var5, 1049040i32, 15i32, var2.wrapping_add(32i32), 15i32);
                let var29 = self.memory.load64(var2 as usize + 32) as i64;
                var5 = var29;
                if (var5 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var30 = self.memory.load64(var2 as usize + 40) as i64;
                var6 = var30;
                if (var6 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var31 = self.memory.load8(var2 as usize + 48) as i32;
                var1 = var31;
                var1 = { let a = 1i32; let b = ((var1 != 0i32) as i32).wrapping_shl(1i32 as u32); if (var1 == 1i32) as i32 != 0 { a } else { b } };
                if (var1 == 2i32) as i32 != 0 {
                    break 'label0;
                }
                let var32 = self.memory.load64(var2 as usize + 56) as i64;
                var7 = var32;
                if (var7 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var33 = self.memory.load64(var2 as usize + 64) as i64;
                var8 = var33;
                if (var8 & 255i64 != 73i64) as i32 != 0 {
                    break 'label0;
                }
                let var34 = self.memory.load64(var2 as usize + 72) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var34);
                let var35 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var35 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var36 = self.memory.load64(var3 as usize) as i64;
                var9 = var36;
                let var37 = self.memory.load64(var2 as usize + 160) as i64;
                var10 = var37;
                let var38 = self.memory.load64(var2 as usize + 80) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var38);
                let var39 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var39 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var40 = self.memory.load64(var3 as usize) as i64;
                var11 = var40;
                let var41 = self.memory.load64(var2 as usize + 160) as i64;
                var12 = var41;
                let var42 = self.memory.load64(var2 as usize + 88) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var42);
                let var43 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var43 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var44 = self.memory.load64(var3 as usize) as i64;
                var13 = var44;
                let var45 = self.memory.load64(var2 as usize + 160) as i64;
                var14 = var45;
                let var46 = self.memory.load64(var2 as usize + 96) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var46);
                let var47 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var47 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var48 = self.memory.load8(var2 as usize + 104) as i32;
                var4 = var48;
                var4 = { let a = 1i32; let b = ((var4 != 0i32) as i32).wrapping_shl(1i32 as u32); if (var4 == 1i32) as i32 != 0 { a } else { b } };
                if (var4 == 2i32) as i32 != 0 {
                    break 'label0;
                }
                let var49 = self.memory.load64(var3 as usize) as i64;
                var15 = var49;
                let var50 = self.memory.load64(var2 as usize + 160) as i64;
                var16 = var50;
                let var51 = self.memory.load64(var2 as usize + 112) as i64;
                self.func42(imports, var2.wrapping_add(16i32), var51);
                let var52 = self.memory.load32(var2 as usize + 16) as i32;
                if var52 != 0 {
                    break 'label0;
                }
                let var53 = self.memory.load64(var2 as usize + 24) as i64;
                var17 = var53;
                let var54 = self.memory.load64(var2 as usize + 120) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var54);
                let var55 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var55 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var56 = self.memory.load64(var3 as usize) as i64;
                var18 = var56;
                let var57 = self.memory.load64(var2 as usize + 160) as i64;
                var19 = var57;
                let var58 = self.memory.load64(var2 as usize + 128) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var58);
                let var59 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var59 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var60 = self.memory.load64(var3 as usize) as i64;
                var20 = var60;
                let var61 = self.memory.load64(var2 as usize + 160) as i64;
                var21 = var61;
                let var62 = self.memory.load64(var2 as usize + 136) as i64;
                self.func42(imports, var2, var62);
                let var63 = self.memory.load32(var2 as usize) as i32;
                if var63 != 0 {
                    break 'label0;
                }
                let var64 = self.memory.load64(var2 as usize + 8) as i64;
                var22 = var64;
                let var65 = self.memory.load64(var2 as usize + 144) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var65);
                let var66 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var66 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var67 = self.memory.load64(var2.wrapping_add(168i32) as usize) as i64;
                var23 = var67;
                let var68 = self.memory.load64(var2 as usize + 160) as i64;
                var24 = var68;
                self.memory.store64(var0 as usize + 96, var10 as u64);
                self.memory.store64(var0 as usize + 80, var14 as u64);
                self.memory.store64(var0 as usize + 64, var24 as u64);
                self.memory.store64(var0 as usize + 48, var19 as u64);
                self.memory.store64(var0 as usize + 32, var16 as u64);
                self.memory.store64(var0 as usize + 16, var21 as u64);
                self.memory.store64(var0 as usize, var12 as u64);
                self.memory.store8(var0 as usize + 161, (var1 & 1i32) as u8);
                self.memory.store8(var0 as usize + 160, (var4 & 1i32) as u8);
                self.memory.store64(var0 as usize + 152, var22 as u64);
                self.memory.store64(var0 as usize + 144, var17 as u64);
                self.memory.store64(var0 as usize + 136, var5 as u64);
                self.memory.store64(var0 as usize + 128, var8 as u64);
                self.memory.store64(var0 as usize + 120, var7 as u64);
                self.memory.store64(var0 as usize + 112, var6 as u64);
                self.memory.store64(var0.wrapping_add(104i32) as usize, var9 as u64);
                self.memory.store64(var0.wrapping_add(88i32) as usize, var13 as u64);
                self.memory.store64(var0.wrapping_add(72i32) as usize, var23 as u64);
                self.memory.store64(var0.wrapping_add(56i32) as usize, var18 as u64);
                self.memory.store64(var0.wrapping_add(40i32) as usize, var15 as u64);
                self.memory.store64(var0.wrapping_add(24i32) as usize, var20 as u64);
                self.memory.store64(var0 as usize + 8, var11 as u64);
                break;
            }
            self.global0 = var2.wrapping_add(176i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func48<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) {
        if (var2 != var4) as i32 != 0 {
            unreachable!();
        }
        let var5 = imports.a(self, var0, (var1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var3 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var2 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var5;
    }
    fn func49<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.func44(imports, var0);
        let var3 = self.func50(imports, var1);
        let var4 = imports.__(self, var2, var3, 1i64);
        var4;
    }
    fn func50<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var1 = var16.wrapping_sub(128i32);
        self.global0 = var1;
        let var17 = self.memory.load64(var0 as usize + 136) as i64;
        var2 = var17;
        let var18 = self.memory.load64(var0 as usize + 112) as i64;
        var3 = var18;
        let var19 = self.memory.load8(var0 as usize + 161) as i64;
        var4 = var19;
        let var20 = self.memory.load64(var0 as usize + 120) as i64;
        var5 = var20;
        let var21 = self.memory.load64(var0 as usize + 128) as i64;
        var6 = var21;
        let var22 = self.memory.load64(var0 as usize + 96) as i64;
        let var23 = self.memory.load64(var0.wrapping_add(104i32) as usize) as i64;
        let var24 = self.func52(imports, var22, var23);
        var7 = var24;
        let var25 = self.memory.load64(var0 as usize) as i64;
        let var26 = self.memory.load64(var0.wrapping_add(8i32) as usize) as i64;
        let var27 = self.func52(imports, var25, var26);
        var8 = var27;
        let var28 = self.memory.load64(var0 as usize + 80) as i64;
        let var29 = self.memory.load64(var0.wrapping_add(88i32) as usize) as i64;
        let var30 = self.func52(imports, var28, var29);
        var9 = var30;
        let var31 = self.memory.load64(var0 as usize + 32) as i64;
        let var32 = self.memory.load64(var0.wrapping_add(40i32) as usize) as i64;
        let var33 = self.func52(imports, var31, var32);
        var10 = var33;
        let var34 = self.memory.load8(var0 as usize + 160) as i64;
        var11 = var34;
        let var35 = self.memory.load64(var0 as usize + 144) as i64;
        let var36 = self.func61(imports, var35);
        var12 = var36;
        let var37 = self.memory.load64(var0 as usize + 48) as i64;
        let var38 = self.memory.load64(var0.wrapping_add(56i32) as usize) as i64;
        let var39 = self.func52(imports, var37, var38);
        var13 = var39;
        let var40 = self.memory.load64(var0 as usize + 16) as i64;
        let var41 = self.memory.load64(var0.wrapping_add(24i32) as usize) as i64;
        let var42 = self.func52(imports, var40, var41);
        var14 = var42;
        let var43 = self.memory.load64(var0 as usize + 152) as i64;
        let var44 = self.func61(imports, var43);
        var15 = var44;
        let var45 = self.memory.load64(var0 as usize + 64) as i64;
        let var46 = self.memory.load64(var0.wrapping_add(72i32) as usize) as i64;
        let var47 = self.func52(imports, var45, var46);
        self.memory.store64(var1 as usize + 120, var47 as u64);
        self.memory.store64(var1 as usize + 112, var15 as u64);
        self.memory.store64(var1 as usize + 104, var14 as u64);
        self.memory.store64(var1 as usize + 96, var13 as u64);
        self.memory.store64(var1 as usize + 88, var12 as u64);
        self.memory.store64(var1 as usize + 80, var11 as u64);
        self.memory.store64(var1 as usize + 72, var10 as u64);
        self.memory.store64(var1 as usize + 64, var9 as u64);
        self.memory.store64(var1 as usize + 56, var8 as u64);
        self.memory.store64(var1 as usize + 48, var7 as u64);
        self.memory.store64(var1 as usize + 40, var6 as u64);
        self.memory.store64(var1 as usize + 32, var5 as u64);
        self.memory.store64(var1 as usize + 24, var4 as u64);
        self.memory.store64(var1 as usize + 16, var3 as u64);
        self.memory.store64(var1 as usize + 8, var2 as u64);
        let var48 = self.func62(imports, 1049040i32, 15i32, var1.wrapping_add(8i32), 15i32);
        self.global0 = var1.wrapping_add(128i32);
        var48
    }
    fn func51<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64) {
        let var3 = self.func44(imports, var0);
        let var4 = self.func52(imports, var1, var2);
        let var5 = imports.__(self, var3, var4, 1i64);
        var5;
    }
    fn func52<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        if ((var1 ^ var0.wrapping_shr(63i64 as u32) != 0i64) as i32 | (var0.wrapping_sub(-36028797018963968i64) as u64 > 72057594037927935i64 as u64) as i32 == 0) as i32 != 0 {
            return var0.wrapping_shl(8i64 as u32) | 11i64;
        }
        let var2 = imports._6(self, var1, var0);
        var2
    }
    fn func53<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let var11 = self.global0;
        var1 = var11.wrapping_sub(80i32);
        self.global0 = var1;
        'label0: loop {
            let var12 = self.func44(imports, 1049208i32);
            var3 = var12;
            let var13 = self.func45(imports, var3, 2i64);
            let var14: i64;
            if var13 != 0 {
                let var15 = imports._1(self, var3, 2i64);
                var3 = var15;
                'label1: loop {
                    if (var2 != 64i32) as i32 != 0 {
                        self.memory.store64(var1.wrapping_add(16i32).wrapping_add(var2) as usize, 2i64 as u64);
                        var2 = var2.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                if (var3 & 255i64 != 76i64) as i32 != 0 {
                    break 'label0;
                }
                self.func48(imports, var3, 1048820i32, 8i32, var1.wrapping_add(16i32), 8i32);
                let var16 = self.memory.load64(var1 as usize + 16) as i64;
                var3 = var16;
                if (var3 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var17 = self.memory.load64(var1 as usize + 24) as i64;
                var4 = var17;
                if (var4 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var18 = self.memory.load64(var1 as usize + 32) as i64;
                var5 = var18;
                if (var5 & 255i64 != 4i64) as i32 != 0 {
                    break 'label0;
                }
                let var19 = self.memory.load64(var1 as usize + 40) as i64;
                var6 = var19;
                if (var6 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var20 = self.memory.load64(var1 as usize + 48) as i64;
                var7 = var20;
                if (var7 & 255i64 != 4i64) as i32 != 0 {
                    break 'label0;
                }
                let var21 = self.memory.load64(var1 as usize + 56) as i64;
                var8 = var21;
                if (var8 & 255i64 != 76i64) as i32 != 0 {
                    break 'label0;
                }
                let var22 = self.memory.load64(var1 as usize + 64) as i64;
                self.func42(imports, var1, var22);
                let var23 = self.memory.load32(var1 as usize) as i32;
                if var23 != 0 {
                    break 'label0;
                }
                let var24 = self.memory.load64(var1 as usize + 72) as i64;
                var9 = var24;
                if (var9 & 255i64 != 77i64) as i32 != 0 {
                    break 'label0;
                }
                let var25 = self.memory.load64(var1 as usize + 8) as i64;
                var10 = var25;
                self.memory.store64(var0 as usize + 8, var3 as u64);
                self.memory.store32(var0.wrapping_add(60i32) as usize, (var5 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store32(var0.wrapping_add(56i32) as usize, (var7 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store64(var0.wrapping_add(48i32) as usize, var8 as u64);
                self.memory.store64(var0.wrapping_add(40i32) as usize, var4 as u64);
                self.memory.store64(var0.wrapping_add(32i32) as usize, var9 as u64);
                self.memory.store64(var0.wrapping_add(24i32) as usize, var6 as u64);
                self.memory.store64(var0.wrapping_add(16i32) as usize, var10 as u64);
                var14 = 1i64;
            } else {
                var14 = 0i64;
            }
            self.memory.store64(var0 as usize, var14 as u64);
            self.global0 = var1.wrapping_add(80i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func54<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.func44(imports, var0);
        let var2 = self.func45(imports, var1, 2i64);
        var2
    }
    fn func55<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.func44(imports, 1049208i32);
        let var2 = self.func56(imports, var0);
        let var3 = imports.__(self, var1, var2, 2i64);
        var3;
    }
    fn func56<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var1 = var8.wrapping_add(-64i32);
        self.global0 = var1;
        let var9 = self.memory.load32(var0 as usize + 52) as i64;
        var2 = var9;
        let var10 = self.memory.load32(var0 as usize + 48) as i64;
        var3 = var10;
        let var11 = self.memory.load64(var0 as usize) as i64;
        var4 = var11;
        let var12 = self.memory.load64(var0 as usize + 32) as i64;
        var5 = var12;
        let var13 = self.memory.load64(var0 as usize + 16) as i64;
        var6 = var13;
        let var14 = self.memory.load64(var0 as usize + 40) as i64;
        var7 = var14;
        let var15 = self.memory.load64(var0 as usize + 8) as i64;
        let var16 = self.func61(imports, var15);
        self.memory.store64(var1 as usize + 48, var16 as u64);
        self.memory.store64(var1 as usize + 40, var7 as u64);
        self.memory.store64(var1 as usize + 24, var6 as u64);
        self.memory.store64(var1 as usize + 8, var5 as u64);
        self.memory.store64(var1 as usize, var4 as u64);
        let var17 = self.memory.load64(var0 as usize + 24) as i64;
        self.memory.store64(var1 as usize + 56, var17 as u64);
        self.memory.store64(var1 as usize + 32, (var3.wrapping_shl(32i64 as u32) | 4i64) as u64);
        self.memory.store64(var1 as usize + 16, (var2.wrapping_shl(32i64 as u32) | 4i64) as u64);
        let var18 = self.func62(imports, 1048820i32, 8i32, var1, 8i32);
        self.global0 = var1.wrapping_sub(-64i32);
        var18
    }
    fn func57<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32i32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (var1 & 255i64 == 75i64) as i32 != 0 {
                    'label2: loop {
                        if (var3 != 16i32) as i32 != 0 {
                            self.memory.store64(var2.wrapping_add(16i32).wrapping_add(var3) as usize, 2i64 as u64);
                            var3 = var3.wrapping_add(8i32);
                            continue 'label2;
                        }
                        break;
                    }
                    let var6 = imports.h(self, var1, (var2.wrapping_add(16i32) as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, 8589934596i64);
                    var6;
                    let var7 = self.memory.load64(var2 as usize + 16) as i64;
                    var4 = var7;
                    if (var4 & 255i64 != 77i64) as i32 != 0 {
                        break 'label1;
                    }
                    let var8 = self.memory.load64(var2 as usize + 24) as i64;
                    self.func42(imports, var2, var8);
                    let var9 = self.memory.load64(var2 as usize + 8) as i64;
                    var1 = var9;
                    let var10 = self.memory.load32(var2 as usize) as i32;
                    if (var10 == 0) as i32 != 0 {
                        self.memory.store64(var0 as usize + 8, var4 as u64);
                        self.memory.store64(var0 as usize, 0i64 as u64);
                        self.memory.store64(var0.wrapping_add(16i32) as usize, var1 as u64);
                        break 'label0;
                    }
                    self.memory.store64(var0 as usize, 1i64 as u64);
                    self.memory.store64(var0 as usize + 8, var1 as u64);
                    break 'label0;
                }
                self.memory.store64(var0 as usize, 1i64 as u64);
                self.memory.store64(var0 as usize + 8, 34359740419i64 as u64);
                break 'label0;
                break;
            }
            self.memory.store64(var0 as usize, 1i64 as u64);
            self.memory.store64(var0 as usize + 8, 34359740419i64 as u64);
            break;
        }
        self.global0 = var2.wrapping_add(32i32);
    }
    fn func58<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(48i32);
        self.global0 = var1;
        let var7 = self.memory.load64(var0 as usize) as i64;
        let var8 = self.memory.load64(var0.wrapping_add(8i32) as usize) as i64;
        let var9 = self.func52(imports, var7, var8);
        var2 = var9;
        let var10 = self.memory.load64(var0 as usize + 16) as i64;
        let var11 = self.memory.load64(var0.wrapping_add(24i32) as usize) as i64;
        let var12 = self.func52(imports, var10, var11);
        var3 = var12;
        let var13 = self.memory.load64(var0 as usize + 32) as i64;
        let var14 = self.memory.load64(var0.wrapping_add(40i32) as usize) as i64;
        let var15 = self.func52(imports, var13, var14);
        var4 = var15;
        let var16 = self.memory.load64(var0 as usize + 48) as i64;
        let var17 = self.memory.load64(var0.wrapping_add(56i32) as usize) as i64;
        let var18 = self.func52(imports, var16, var17);
        var5 = var18;
        let var19 = self.memory.load64(var0 as usize + 64) as i64;
        let var20 = self.memory.load64(var0.wrapping_add(72i32) as usize) as i64;
        let var21 = self.func52(imports, var19, var20);
        self.memory.store64(var1 as usize + 40, var21 as u64);
        self.memory.store64(var1 as usize + 32, var5 as u64);
        self.memory.store64(var1 as usize + 24, var4 as u64);
        self.memory.store64(var1 as usize + 16, var3 as u64);
        self.memory.store64(var1 as usize + 8, var2 as u64);
        let var22 = self.func59(imports, var1.wrapping_add(8i32), 5i32);
        self.global0 = var1.wrapping_add(48i32);
        var22
    }
    fn func59<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i64 {
        let var2 = imports.g(self, (var0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var2
    }
    fn func60<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(48i32);
        self.global0 = var1;
        let var6 = self.memory.load64(var0 as usize) as i64;
        var2 = var6;
        let var7 = self.memory.load64(var0 as usize + 8) as i64;
        var3 = var7;
        let var8 = self.memory.load64(var0.wrapping_add(16i32) as usize) as i64;
        let var9 = self.func61(imports, var8);
        var4 = var9;
        let var10 = self.memory.load64(var0.wrapping_add(24i32) as usize) as i64;
        let var11 = self.func61(imports, var10);
        self.memory.store64(var1 as usize + 40, var11 as u64);
        self.memory.store64(var1 as usize + 32, var4 as u64);
        self.memory.store64(var1 as usize + 24, var3 as u64);
        let var12 = self.func59(imports, var1.wrapping_add(24i32), 3i32);
        self.memory.store64(var1 as usize + 16, var12 as u64);
        self.memory.store64(var1 as usize + 8, var2 as u64);
        var0 = 0i32;
        let var13: i64;
        'label0: loop {
            let var14: i64;
            if (var0 == 16i32) as i32 != 0 {
                var0 = 0i32;
                'label1: loop {
                    if (var0 != 16i32) as i32 != 0 {
                        let var15 = self.memory.load64(var1.wrapping_add(8i32).wrapping_add(var0) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(24i32).wrapping_add(var0) as usize, var15 as u64);
                        var0 = var0.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                let var16 = self.func59(imports, var1.wrapping_add(24i32), 2i32);
                self.global0 = var1.wrapping_add(48i32);
                var14 = var16;
            } else {
                self.memory.store64(var1.wrapping_add(24i32).wrapping_add(var0) as usize, 2i64 as u64);
                var0 = var0.wrapping_add(8i32);
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
    fn func61<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        if (var0 as u64 <= 72057594037927935i64 as u64) as i32 != 0 {
            return var0.wrapping_shl(8i64 as u32) | 6i64;
        }
        let var1 = imports.__(self, var0);
        var1
    }
    fn func62<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i64 {
        if (var1 != var3) as i32 != 0 {
            unreachable!();
        }
        let var4 = imports._9(self, (var0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var2 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var4
    }
    fn func63<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16i32);
        self.global0 = var1;
        self.memory.store64(var1 as usize, var0 as u64);
        var3 = 2i64;
        var2 = 1i32;
        'label0: loop {
            if var2 != 0 {
                var2 = var2.wrapping_sub(1i32);
                var3 = var0;
                continue 'label0;
            }
            break;
        }
        self.memory.store64(var1 as usize + 8, var3 as u64);
        let var5 = self.func59(imports, var1.wrapping_add(8i32), 1i32);
        self.global0 = var1.wrapping_add(16i32);
        var5
    }
    fn func64<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        'label0: loop {
            if (var1 as u32 > 9i32 as u32) as i32 != 0 {
                break 'label0;
            }
            var3 = var1;
            var4 = var0;
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
        let var9 = imports.j(self, (var0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, (var1 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64);
        var9
    }
    fn func65<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            let var2 = self.memory.load8(var0 as usize + 160) as i32;
            if (var2 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.func66(imports);
            let var4 = self.memory.load64(var0 as usize + 144) as i64;
            if ((var3 as u64) < var4 as u64) as i32 != 0 {
                break 'label0;
            }
            let var5 = self.memory.load64(var0 as usize) as i64;
            let var6 = self.memory.load64(var0 as usize + 16) as i64;
            let var7 = self.memory.load64(var0.wrapping_add(8i32) as usize) as i64;
            let var8 = self.memory.load64(var0.wrapping_add(24i32) as usize) as i64;
            var1 = (var5 ^ var6 | var7 ^ var8 == 0) as i32;
            break;
        }
        var1
    }
    fn func66<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i64 = 0;
        let mut var1: i32 = 0;
        let var2 = imports._4(self);
        var0 = var2;
        var1 = var0 as i32 & 255i32;
        if (var1 != 64i32) as i32 != 0 {
            if (var1 == 6i32) as i32 != 0 {
                return (var0 as u64).wrapping_shr(8i64 as u32) as i64;
            }
            unreachable!();
        }
        let var3 = imports._0(self, var0);
        var3
    }
    fn func67<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64, mut var5: i64, mut var6: i64, mut var7: i64, mut var8: i64) {
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var9 = var13.wrapping_sub(192i32);
        self.global0 = var9;
        'label0: loop {
            'label1: loop {
                var12 = var4 ^ var6;
                if ((var12 | var3 ^ var5 == 0) as i32 == 0) as i32 != 0 {
                    if ((var1 ^ var7 | var2 ^ var8 == 0) as i32 == 0) as i32 != 0 {
                        self.memory.store32(var9 as usize + 188, 0i32 as u32);
                        self.func68(imports, var9.wrapping_add(168i32), var7, var8, 20000000i64, 0i64, var9.wrapping_add(188i32));
                        let var14 = self.memory.load32(var9 as usize + 188) as i32;
                        let var15 = self.memory.load64(var9.wrapping_add(176i32) as usize) as i64;
                        var7 = var15;
                        let var16 = self.memory.load64(var9 as usize + 168) as i64;
                        var8 = var16;
                        let var17 = self.func69(imports, var5, var6);
                        let var18 = self.func69(imports, var1, var2);
                        let var19 = imports.x(self, var17, var18);
                        let var20 = self.func69(imports, 10000000i64, 0i64);
                        let var21 = imports.y(self, var19, var20);
                        var12 = var21;
                        let var22 = self.func69(imports, var3, var4);
                        let var23 = self.func69(imports, var5, var6);
                        let var24 = imports.w(self, var22, var23);
                        let var25 = self.func69(imports, var1, var2);
                        let var26 = imports.x(self, var24, var25);
                        let var27 = self.func69(imports, var1, var2);
                        let var28 = imports.x(self, var26, var27);
                        var1 = var28;
                        if var14 != 0 {
                            break 'label0;
                        }
                        let var29 = self.func69(imports, var8, var7);
                        let var30 = imports.y(self, var1, var29);
                        var1 = var30;
                        self.func70(imports, var9.wrapping_add(144i32), var12);
                        let var31 = self.memory.load64(var9.wrapping_add(160i32) as usize) as i64;
                        let var32 = self.memory.load64(var9 as usize + 152) as i64;
                        var3 = var32;
                        let var33 = self.memory.load32(var9 as usize + 144) as i32;
                        var10 = var33;
                        self.func70(imports, var9.wrapping_add(120i32), var1);
                        var1 = { let a = var31; let b = 0i64; if var10 != 0 { a } else { b } };
                        let var34 = self.memory.load64(var9.wrapping_add(136i32) as usize) as i64;
                        let var35 = self.memory.load32(var9 as usize + 120) as i32;
                        var11 = var35;
                        var2 = { let a = var34; let b = 0i64; if var11 != 0 { a } else { b } };
                        var3 = { let a = var3; let b = 0i64; if var10 != 0 { a } else { b } };
                        let var36 = self.memory.load64(var9 as usize + 128) as i64;
                        var4 = var3.wrapping_add({ let a = var36; let b = 0i64; if var11 != 0 { a } else { b } });
                        var3 = (((var4 as u64) < var3 as u64) as i32 as u32 as i64).wrapping_add(var1.wrapping_add(var2));
                        if ((var1 ^ var2 ^ -1i64) & (var1 ^ var3) >= 0i64) as i32 != 0 {
                            break 'label1;
                        }
                        break 'label0;
                    }
                    self.memory.store32(var9 as usize + 116, 0i32 as u32);
                    self.func68(imports, var9.wrapping_add(96i32), var5, var6, var1, var2, var9.wrapping_add(116i32));
                    let var37 = self.memory.load32(var9 as usize + 116) as i32;
                    if var37 != 0 {
                        break 'label0;
                    }
                    var7 = var4.wrapping_sub(var6).wrapping_sub(((var3 as u64) < var5 as u64) as i32 as u32 as i64);
                    if (var12 & (var4 ^ var7) < 0i64) as i32 != 0 {
                        break 'label0;
                    }
                    let var38 = self.memory.load64(var9.wrapping_add(104i32) as usize) as i64;
                    var4 = var38;
                    let var39 = self.memory.load64(var9 as usize + 96) as i64;
                    var6 = var39;
                    self.memory.store32(var9 as usize + 92, 0i32 as u32);
                    self.func68(imports, var9.wrapping_add(72i32), var3.wrapping_sub(var5), var7, var1, var2, var9.wrapping_add(92i32));
                    let var40 = self.memory.load32(var9 as usize + 92) as i32;
                    if var40 != 0 {
                        break 'label0;
                    }
                    let var41 = self.memory.load64(var9 as usize + 72) as i64;
                    let var42 = self.memory.load64(var9.wrapping_add(80i32) as usize) as i64;
                    self.func71(imports, var9.wrapping_add(56i32), var41, var42, 2i64, 0i64);
                    let var43 = self.memory.load64(var9.wrapping_sub(-64i32) as usize) as i64;
                    var1 = var43;
                    let var44 = var1;
                    let var45 = self.memory.load64(var9 as usize + 56) as i64;
                    var2 = var6.wrapping_add(var45);
                    var1 = (((var2 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var1.wrapping_add(var4));
                    if ((var4 ^ var44 ^ -1i64) & (var4 ^ var1) < 0i64) as i32 != 0 {
                        break 'label0;
                    }
                    self.func71(imports, var9.wrapping_add(40i32), var2, var1, 10000000i64, 0i64);
                    let var46 = self.memory.load64(var9.wrapping_add(48i32) as usize) as i64;
                    var3 = var46;
                    let var47 = self.memory.load64(var9 as usize + 40) as i64;
                    var4 = var47;
                    break 'label1;
                }
                self.memory.store32(var9 as usize + 36, 0i32 as u32);
                self.func68(imports, var9.wrapping_add(16i32), var1, var2, var3, var4, var9.wrapping_add(36i32));
                let var48 = self.memory.load32(var9 as usize + 36) as i32;
                if var48 != 0 {
                    break 'label0;
                }
                let var49 = self.memory.load64(var9 as usize + 16) as i64;
                let var50 = self.memory.load64(var9.wrapping_add(24i32) as usize) as i64;
                self.func71(imports, var9, var49, var50, 10000000i64, 0i64);
                let var51 = self.memory.load64(var9.wrapping_add(8i32) as usize) as i64;
                var3 = var51;
                let var52 = self.memory.load64(var9 as usize) as i64;
                var4 = var52;
                break;
            }
            self.memory.store64(var0 as usize, var4 as u64);
            self.memory.store64(var0 as usize + 8, var3 as u64);
            self.global0 = var9.wrapping_add(192i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func68<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let var15 = self.global0;
        var7 = var15.wrapping_sub(32i32);
        self.global0 = var7;
        var9 = var7.wrapping_add(8i32);
        let var16 = self.global0;
        var8 = var16.wrapping_sub(32i32);
        self.global0 = var8;
        let var17: i32;
        'label0: loop {
            let var18 = 0i32;
            if (var1 | var2 == 0) as i32 | (var3 | var4 == 0) as i32 != 0 {
                var17 = var18;
                break 'label0;
            }
            var18;
            var12 = (var2 < 0i64) as i32;
            var13 = { let a = 0i64.wrapping_sub(var1); let b = var1; if var12 != 0 { a } else { b } };
            var6 = (var4 < 0i64) as i32;
            var14 = { let a = 0i64.wrapping_sub(var3); let b = var3; if var6 != 0 { a } else { b } };
            var3 = { let a = 0i64.wrapping_sub(var4.wrapping_add((var3 != 0i64) as i32 as u32 as i64)); let b = var4; if var6 != 0 { a } else { b } };
            let var19 = self.global0;
            var6 = var19.wrapping_sub(96i32);
            self.global0 = var6;
            var11 = var8.wrapping_add(8i32);
            let var20: i64;
            'label1: loop {
                var1 = { let a = 0i64.wrapping_sub(var2.wrapping_add((var1 != 0i64) as i32 as u32 as i64)); let b = var2; if var12 != 0 { a } else { b } };
                if ((var1 == 0) as i32 == 0) as i32 != 0 {
                    if ((var3 == 0) as i32 == 0) as i32 != 0 {
                        self.func100(imports, var6.wrapping_add(80i32), var14, var3, var13, var1);
                        let var21 = self.memory.load64(var6.wrapping_add(88i32) as usize) as i64;
                        var1 = var21;
                        var10 = 1i32;
                        let var22 = self.memory.load64(var6 as usize + 80) as i64;
                        var20 = var22;
                        break 'label1;
                    }
                    self.func100(imports, var6.wrapping_sub(-64i32), var13, 0i64, var14, var3);
                    self.func100(imports, var6.wrapping_add(48i32), var1, 0i64, var14, var3);
                    let var23 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                    let var24 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                    var3 = var24;
                    let var25 = self.memory.load64(var6 as usize + 48) as i64;
                    var1 = var3.wrapping_add(var25);
                    var10 = (var23 != 0i64) as i32 | ((var1 as u64) < var3 as u64) as i32;
                    let var26 = self.memory.load64(var6 as usize + 64) as i64;
                    var20 = var26;
                    break 'label1;
                }
                if ((var3 == 0) as i32 == 0) as i32 != 0 {
                    self.func100(imports, var6.wrapping_add(32i32), var14, 0i64, var13, var1);
                    self.func100(imports, var6.wrapping_add(16i32), var3, 0i64, var13, var1);
                    let var27 = self.memory.load64(var6.wrapping_add(24i32) as usize) as i64;
                    let var28 = self.memory.load64(var6.wrapping_add(40i32) as usize) as i64;
                    var3 = var28;
                    let var29 = self.memory.load64(var6 as usize + 16) as i64;
                    var1 = var3.wrapping_add(var29);
                    var10 = (var27 != 0i64) as i32 | ((var1 as u64) < var3 as u64) as i32;
                    let var30 = self.memory.load64(var6 as usize + 32) as i64;
                    var20 = var30;
                    break 'label1;
                }
                self.func100(imports, var6, var14, var3, var13, var1);
                let var31 = self.memory.load64(var6.wrapping_add(8i32) as usize) as i64;
                var1 = var31;
                let var32 = self.memory.load64(var6 as usize) as i64;
                var20 = var32;
                break;
            }
            self.memory.store64(var11 as usize, var20 as u64);
            self.memory.store8(var11 as usize + 16, var10 as u8);
            self.memory.store64(var11 as usize + 8, var1 as u64);
            self.global0 = var6.wrapping_add(96i32);
            let var33 = self.memory.load64(var8.wrapping_add(16i32) as usize) as i64;
            var14 = var33;
            let var34 = self.memory.load64(var8 as usize + 8) as i64;
            var13 = var34;
            let var35 = self.memory.load8(var8 as usize + 24) as i32;
            var6 = var35;
            'label2: loop {
                'label3: loop {
                    var2 = var2 ^ var4;
                    if (var2 >= 0i64) as i32 != 0 {
                        if (var2 ^ var14 >= 0i64) as i32 != 0 {
                            break 'label3;
                        }
                        var17 = 1i32;
                        break 'label0;
                    }
                    var1 = 0i64.wrapping_sub(var13);
                    var14 = 0i64.wrapping_sub(var14.wrapping_add((var13 != 0i64) as i32 as u32 as i64));
                    if (var2 ^ var14 < 0i64) as i32 != 0 {
                        break 'label2;
                    }
                    var13 = var1;
                    break;
                }
                var17 = var6 & 1i32;
                break 'label0;
                break;
            }
            var13 = var1;
            var17 = 1i32;
            break;
        }
        var6 = var17;
        self.memory.store64(var9 as usize, var13 as u64);
        self.memory.store8(var9 as usize + 16, var6 as u8);
        self.memory.store64(var9 as usize + 8, var14 as u64);
        self.global0 = var8.wrapping_add(32i32);
        let var36 = self.memory.load64(var7.wrapping_add(16i32) as usize) as i64;
        var1 = var36;
        let var37 = self.memory.load64(var7 as usize + 8) as i64;
        var2 = var37;
        let var38 = self.memory.load8(var7 as usize + 24) as i32;
        self.memory.store32(var5 as usize, (var38 & 1i32) as u32);
        self.memory.store64(var0 as usize + 8, var1 as u64);
        self.memory.store64(var0 as usize, var2 as u64);
        self.global0 = var7.wrapping_add(32i32);
    }
    fn func69<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16i32);
        self.global0 = var2;
        self.memory.store64(var2 as usize + 8, (var0.wrapping_shl(56i64 as u32) | (var0 & 65280i64).wrapping_shl(40i64 as u32) | (var0 & 16711680i64).wrapping_shl(24i64 as u32) | (var0 & 4278190080i64).wrapping_shl(8i64 as u32) | (var0 as u64).wrapping_shr(8i64 as u32) as i64 & 4278190080i64 | (var0 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (var0 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (var0 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
        self.memory.store64(var2 as usize, (var1.wrapping_shl(56i64 as u32) | (var1 & 65280i64).wrapping_shl(40i64 as u32) | (var1 & 16711680i64).wrapping_shl(24i64 as u32) | (var1 & 4278190080i64).wrapping_shl(8i64 as u32) | (var1 as u64).wrapping_shr(8i64 as u32) as i64 & 4278190080i64 | (var1 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (var1 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (var1 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
        let var4 = self.func102(imports, var2);
        var0 = var4;
        let var5 = self.func102(imports, { let a = 1049256i32; let b = 1049240i32; if (var1 < 0i64) as i32 != 0 { a } else { b } });
        let var6 = imports.e(self, var5, var0);
        let var7 = imports.h(self, var6);
        self.global0 = var2.wrapping_add(16i32);
        var7
    }
    fn func70<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(32i32);
        self.global0 = var5;
        let var8 = imports.i(self, var1);
        var1 = var8;
        let var9 = imports.f(self, var1, 4i64, 68719476740i64);
        self.func104(imports, var5.wrapping_add(15i32), var9);
        'label0: loop {
            let var10 = self.memory.load8(var5 as usize + 15) as i32;
            if var10 != 0 {
                break 'label0;
            }
            var6 = var5.wrapping_add(24i32);
            let var11 = self.memory.load64(var6 as usize) as i64;
            var3 = var11;
            let var12 = self.memory.load64(var5 as usize + 16) as i64;
            var4 = var12;
            let var13 = imports.f(self, var1, 68719476740i64, 137438953476i64);
            self.func104(imports, var5.wrapping_add(15i32), var13);
            let var14 = self.memory.load8(var5 as usize + 15) as i32;
            if var14 != 0 {
                break 'label0;
            }
            let var15 = self.memory.load64(var6 as usize) as i64;
            var1 = var15;
            let var16 = self.memory.load64(var5 as usize + 16) as i64;
            var2 = var16;
            var2 = var2.wrapping_shl(56i64 as u32) | (var2 & 65280i64).wrapping_shl(40i64 as u32) | (var2 & 16711680i64).wrapping_shl(24i64 as u32) | (var2 & 4278190080i64).wrapping_shl(8i64 as u32) | (var2 as u64).wrapping_shr(8i64 as u32) as i64 & 4278190080i64 | (var2 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (var2 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (var2 as u64).wrapping_shr(56i64 as u32) as i64;
            self.memory.store64(var0.wrapping_add(16i32) as usize, var2 as u64);
            self.memory.store64(var0 as usize + 8, (var1.wrapping_shl(56i64 as u32) | (var1 & 65280i64).wrapping_shl(40i64 as u32) | (var1 & 16711680i64).wrapping_shl(24i64 as u32) | (var1 & 4278190080i64).wrapping_shl(8i64 as u32) | (var1 as u64).wrapping_shr(8i64 as u32) as i64 & 4278190080i64 | (var1 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (var1 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (var1 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
            self.memory.store64(var0 as usize, ((var3 | var4 == 0) as i32 & (var2 >= 0i64) as i32 | (var3 & var4 == -1i64) as i32 & (var2 < 0i64) as i32) as u32 as i64 as u64);
            self.global0 = var5.wrapping_add(32i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func71<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64) {
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
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let var17 = self.global0;
        var14 = var17.wrapping_sub(32i32);
        self.global0 = var14;
        var13 = (var2 < 0i64) as i32;
        var5 = { let a = 0i64.wrapping_sub(var1); let b = var1; if var13 != 0 { a } else { b } };
        var1 = { let a = 0i64.wrapping_sub(var2.wrapping_add((var1 != 0i64) as i32 as u32 as i64)); let b = var2; if var13 != 0 { a } else { b } };
        var12 = (var4 < 0i64) as i32;
        var6 = { let a = 0i64.wrapping_sub(var3); let b = var3; if var12 != 0 { a } else { b } };
        let var18 = self.global0;
        var13 = var18.wrapping_sub(32i32);
        self.global0 = var13;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            var3 = { let a = 0i64.wrapping_sub(var4.wrapping_add((var3 != 0i64) as i32 as u32 as i64)); let b = var4; if var12 != 0 { a } else { b } };
                            if ((var3 == 0) as i32 == 0) as i32 != 0 {
                                if (var1 == 0) as i32 | ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = ((var1 as u64) < var3 as u64) as i32; if (var1 == var3) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label2;
                                }
                                var12 = var3.leading_zeros() as i64 as i32;
                                var15 = var1.leading_zeros() as i64 as i32;
                                if ((var12 as u32) < var15 as u32) as i32 != 0 {
                                    break 'label4;
                                }
                                var12 = var12.wrapping_sub(var15);
                                if (var12 as u32 >= 128i32 as u32) as i32 != 0 {
                                    break 'label4;
                                }
                                self.func101(imports, var13.wrapping_add(16i32), var6, var3, var12);
                                var11 = 1i64.wrapping_shl(var12 as u32 as i64 as u32);
                                let var19 = self.memory.load64(var13.wrapping_add(24i32) as usize) as i64;
                                var8 = var19;
                                let var20 = self.memory.load64(var13 as usize + 16) as i64;
                                var9 = var20;
                                'label5: loop {
                                    var7 = var1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                    if (var7 >= 0i64) as i32 != 0 {
                                        var10 = var10 | var11;
                                        var5 = var5.wrapping_sub(var9);
                                        if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (var3 as u64 > var7 as u64) as i32; if (var3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                            break 'label1;
                                        }
                                        var1 = var7;
                                    }
                                    var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(1i64 as u32) as i64;
                                    var11 = (var11 as u64).wrapping_shr(1i64 as u32) as i64;
                                    var8 = (var8 as u64).wrapping_shr(1i64 as u32) as i64;
                                    continue 'label5;
                                    break;
                                }
                                unreachable!();
                            }
                            'label6: loop {
                                'label7: loop {
                                    'label8: loop {
                                        'label9: loop {
                                            if ((var1 == 0) as i32 == 0) as i32 != 0 {
                                                if ((var1 as u64) < var6 as u64) as i32 != 0 {
                                                    break 'label9;
                                                }
                                                if (var1 == var6) as i32 != 0 {
                                                    break 'label8;
                                                }
                                                var11 = (var1 as u64 / var6 as u64) as i64;
                                                var7 = var1.wrapping_sub(var11.wrapping_mul(var6));
                                                if (var6 as u64 >= 4294967296i64 as u64) as i32 != 0 {
                                                    break 'label7;
                                                }
                                                var1 = var7.wrapping_shl(32i64 as u32) | (var5 as u64).wrapping_shr(32i64 as u32) as i64;
                                                let var21 = var1;
                                                var1 = (var1 as u64 / var6 as u64) as i64;
                                                var3 = var5 & 4294967295i64 | var21.wrapping_sub(var1.wrapping_mul(var6)).wrapping_shl(32i64 as u32);
                                                let var22 = var3;
                                                var3 = (var3 as u64 / var6 as u64) as i64;
                                                var5 = var22.wrapping_sub(var6.wrapping_mul(var3));
                                                var10 = var1.wrapping_shl(32i64 as u32) | var3;
                                                var11 = (var1 as u64).wrapping_shr(32i64 as u32) as i64 | var11;
                                                var7 = 0i64;
                                                break 'label0;
                                            }
                                            var10 = (var5 as u64 / var6 as u64) as i64;
                                            var5 = var5.wrapping_sub(var10.wrapping_mul(var6));
                                            break 'label3;
                                            break;
                                        }
                                        var15 = var1.leading_zeros() as i64 as i32;
                                        var16 = var6.leading_zeros() as i64 as i32;
                                        if ((var15 as u32) < var16 as u32) as i32 != 0 {
                                            break 'label4;
                                        }
                                        var12 = 63i32;
                                        if (var15 != var16) as i32 != 0 {
                                            var12 = var15.wrapping_sub(var16);
                                            if (var12 as u32 >= 65i32 as u32) as i32 != 0 {
                                                break 'label4;
                                            }
                                            var12 = 64i32.wrapping_sub(var12);
                                        }
                                        self.func101(imports, var13, var6, var3, var12);
                                        var7 = 1i64.wrapping_shl(var12 as u32 as i64 as u32);
                                        let var23 = self.memory.load64(var13.wrapping_add(8i32) as usize) as i64;
                                        var8 = var23;
                                        let var24 = self.memory.load64(var13 as usize) as i64;
                                        var9 = var24;
                                        'label10: loop {
                                            var3 = var1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                            if (var3 >= 0i64) as i32 != 0 {
                                                var5 = var5.wrapping_sub(var9);
                                                var10 = var7 | var10;
                                                if (var3 == 0) as i32 != 0 {
                                                    break 'label6;
                                                }
                                                var1 = var3;
                                            }
                                            var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(1i64 as u32) as i64;
                                            var7 = (var7 as u64).wrapping_shr(1i64 as u32) as i64;
                                            var8 = (var8 as u64).wrapping_shr(1i64 as u32) as i64;
                                            continue 'label10;
                                            break;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    var10 = (var5 as u64 / var1 as u64) as i64;
                                    var5 = var5.wrapping_sub(var10.wrapping_mul(var1));
                                    var11 = 1i64;
                                    break 'label0;
                                    break;
                                }
                                if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (var3 as u64 > var7 as u64) as i32; if (var3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label0;
                                }
                                var8 = var3.wrapping_shl(63i64 as u32) | (var6 as u64).wrapping_shr(1i64 as u32) as i64;
                                var9 = var6.wrapping_shl(63i64 as u32);
                                var1 = -9223372036854775808i64;
                                'label11: loop {
                                    'label12: loop {
                                        var3 = var7.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                        if (var3 >= 0i64) as i32 != 0 {
                                            var5 = var5.wrapping_sub(var9);
                                            var10 = var1 | var10;
                                            if (var3 == 0) as i32 != 0 {
                                                break 'label12;
                                            }
                                            var7 = var3;
                                        }
                                        var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(1i64 as u32) as i64;
                                        var1 = (var1 as u64).wrapping_shr(1i64 as u32) as i64;
                                        var8 = (var8 as u64).wrapping_shr(1i64 as u32) as i64;
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                var1 = (var5 as u64 / var6 as u64) as i64;
                                var10 = var1 | var10;
                                var5 = var5.wrapping_sub(var1.wrapping_mul(var6));
                                var7 = 0i64;
                                break 'label0;
                                break;
                            }
                            var1 = (var5 as u64 / var6 as u64) as i64;
                            var10 = var1 | var10;
                            var5 = var5.wrapping_sub(var1.wrapping_mul(var6));
                            break 'label3;
                            break;
                        }
                        unreachable!();
                        break;
                    }
                    var7 = 0i64;
                    break 'label1;
                    break;
                }
                var7 = var1;
                break;
            }
            var11 = 0i64;
            break;
        }
        self.memory.store64(var14 as usize + 16, var5 as u64);
        self.memory.store64(var14 as usize, var10 as u64);
        self.memory.store64(var14.wrapping_add(24i32) as usize, var7 as u64);
        self.memory.store64(var14 as usize + 8, var11 as u64);
        self.global0 = var13.wrapping_add(32i32);
        let var25 = self.memory.load64(var14.wrapping_add(8i32) as usize) as i64;
        var1 = var25;
        let var26 = self.memory.load64(var14 as usize) as i64;
        var3 = var26;
        var13 = (var2 ^ var4 < 0i64) as i32;
        self.memory.store64(var0 as usize, ({ let a = 0i64.wrapping_sub(var3); let b = var3; if var13 != 0 { a } else { b } }) as u64);
        self.memory.store64(var0 as usize + 8, ({ let a = 0i64.wrapping_sub(var1.wrapping_add((var3 != 0i64) as i32 as u32 as i64)); let b = var1; if var13 != 0 { a } else { b } }) as u64);
        self.global0 = var14.wrapping_add(32i32);
    }
    fn func72<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let var15 = self.global0;
        var3 = var15.wrapping_sub(80i32);
        self.global0 = var3;
        'label0: loop {
            'label1: loop {
                let var16 = imports._3(self, var0);
                var5 = var16;
                if (var5 as u64 <= 4294967295i64 as u64) as i32 != 0 {
                    break 'label1;
                }
                self.func71(imports, var3.wrapping_add(32i32), var1, var2, (var5 as u64).wrapping_shr(32i64 as u32) as i64, 0i64);
                let var17 = imports._3(self, var0);
                var5 = var17;
                self.memory.store32(var3 as usize + 28, 0i32 as u32);
                let var18 = self.memory.load64(var3 as usize + 32) as i64;
                var13 = var18;
                let var19 = self.memory.load64(var3.wrapping_add(40i32) as usize) as i64;
                var9 = var19;
                self.func68(imports, var3.wrapping_add(8i32), var13, var9, (var5 as u64).wrapping_shr(32i64 as u32) as i64, 0i64, var3.wrapping_add(28i32));
                let var20 = self.memory.load32(var3 as usize + 28) as i32;
                if var20 != 0 {
                    break 'label1;
                }
                let var21 = self.memory.load64(var3.wrapping_add(16i32) as usize) as i64;
                var5 = var21;
                let var22 = var5;
                let var23 = var5;
                let var24 = self.memory.load64(var3 as usize + 8) as i64;
                var5 = var24;
                var7 = var2.wrapping_sub(var23).wrapping_sub(((var1 as u64) < var5 as u64) as i32 as u32 as i64);
                if ((var2 ^ var22) & (var2 ^ var7) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                var8 = var1.wrapping_sub(var5);
                let var25 = imports._3(self, var0);
                var14 = (var25 as u64).wrapping_shr(32i64 as u32) as i64;
                var4 = var3.wrapping_sub(-64i32);
                var1 = 4i64;
                var2 = 0i64;
                var5 = var0;
                'label2: loop {
                    if (var2 == var14) as i32 != 0 {
                        break 'label0;
                    }
                    let var26 = imports._5(self, var0, var1);
                    var10 = var26;
                    let var27 = imports._6(self, var0, var1);
                    var6 = var27;
                    if (var2 == 4294967295i64) as i32 | (var10 & 255i64 != 4i64) as i32 != 0 {
                        break 'label1;
                    }
                    self.func73(imports, var3.wrapping_add(48i32), var6);
                    let var28 = self.memory.load64(var3 as usize + 48) as i64;
                    if ((var28 == 0) as i32 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    let var29 = self.memory.load64(var4 as usize) as i64;
                    var11 = var29;
                    let var30 = self.memory.load64(var3 as usize + 56) as i64;
                    var6 = var30;
                    var12 = var6.wrapping_add(var13);
                    var6 = (((var12 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var9.wrapping_add(var11));
                    if ((var11 ^ var9 ^ -1i64) & (var11 ^ var6) < 0i64) as i32 != 0 {
                        break 'label1;
                    }
                    let var31 = var7;
                    var8 = var8.wrapping_add(var12);
                    var7 = (((var8 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_add(var7));
                    if ((var6 ^ var31 ^ -1i64) & (var6 ^ var7) < 0i64) as i32 != 0 {
                        break 'label1;
                    }
                    let var32 = self.memory.load64(var3 as usize + 72) as i64;
                    let var33 = self.func61(imports, var32);
                    var6 = var33;
                    let var34 = self.func52(imports, var8, var7);
                    self.memory.store64(var3 as usize + 56, var34 as u64);
                    self.memory.store64(var3 as usize + 48, var6 as u64);
                    var1 = var1.wrapping_add(4294967296i64);
                    var2 = var2.wrapping_add(1i64);
                    var8 = 0i64;
                    let var35 = self.func62(imports, 1049192i32, 2i32, var3.wrapping_add(48i32), 2i32);
                    let var36 = imports._0(self, var5, var10 & -4294967296i64 | 4i64, var35);
                    var5 = var36;
                    var7 = 0i64;
                    continue 'label2;
                    break;
                }
                unreachable!();
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var3.wrapping_add(80i32);
        var5
    }
    fn func73<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64i32);
        self.global0 = var2;
        'label0: loop {
            if (var3 != 16i32) as i32 != 0 {
                self.memory.store64(var2.wrapping_add(24i32).wrapping_add(var3) as usize, 2i64 as u64);
                var3 = var3.wrapping_add(8i32);
                continue 'label0;
            }
            break;
        }
        'label1: loop {
            'label2: loop {
                if (var1 & 255i64 == 76i64) as i32 != 0 {
                    self.func48(imports, var1, 1049192i32, 2i32, var2.wrapping_add(24i32), 2i32);
                    let var6 = self.memory.load64(var2 as usize + 24) as i64;
                    self.func42(imports, var2.wrapping_add(8i32), var6);
                    let var7 = self.memory.load32(var2 as usize + 8) as i32;
                    if var7 != 0 {
                        break 'label2;
                    }
                    let var8 = self.memory.load64(var2 as usize + 16) as i64;
                    var1 = var8;
                    let var9 = self.memory.load64(var2 as usize + 32) as i64;
                    self.func46(imports, var2.wrapping_add(40i32), var9);
                    let var10 = self.memory.load64(var2 as usize + 40) as i64;
                    if (var10 == 0) as i32 != 0 {
                        let var11 = self.memory.load64(var2.wrapping_add(56i32) as usize) as i64;
                        var4 = var11;
                        let var12 = self.memory.load64(var2 as usize + 48) as i64;
                        self.memory.store64(var0 as usize + 8, var12 as u64);
                        self.memory.store64(var0 as usize, 0i64 as u64);
                        self.memory.store64(var0.wrapping_add(16i32) as usize, var4 as u64);
                        self.memory.store64(var0.wrapping_add(24i32) as usize, var1 as u64);
                        break 'label1;
                    }
                    self.memory.store64(var0 as usize, 1i64 as u64);
                    break 'label1;
                }
                self.memory.store64(var0 as usize, 1i64 as u64);
                break 'label1;
                break;
            }
            self.memory.store64(var0 as usize, 1i64 as u64);
            break;
        }
        self.global0 = var2.wrapping_sub(-64i32);
    }
    fn func74<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var6 = var8.wrapping_sub(48i32);
        self.global0 = var6;
        if (({ let a = (var3 == 0) as i32; let b = (var4 < 0i64) as i32; if (var4 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
            let var9 = self.func75(imports, 1049272i32, 8i32);
            var7 = var9;
            let var10 = self.func52(imports, var3, var4);
            self.memory.store64(var6 as usize + 16, var10 as u64);
            self.memory.store64(var6 as usize + 8, var2 as u64);
            self.memory.store64(var6 as usize, var1 as u64);
            'label0: loop {
                if (var5 == 24i32) as i32 != 0 {
                    var5 = 0i32;
                    'label1: loop {
                        if (var5 != 24i32) as i32 != 0 {
                            let var11 = self.memory.load64(var5.wrapping_add(var6) as usize) as i64;
                            self.memory.store64(var6.wrapping_add(24i32).wrapping_add(var5) as usize, var11 as u64);
                            var5 = var5.wrapping_add(8i32);
                            continue 'label1;
                        }
                        break;
                    }
                    let var12 = self.func59(imports, var6.wrapping_add(24i32), 3i32);
                    self.func76(imports, var0, var7, var12);
                } else {
                    self.memory.store64(var6.wrapping_add(24i32).wrapping_add(var5) as usize, 2i64 as u64);
                    var5 = var5.wrapping_add(8i32);
                    continue 'label0;
                }
                break;
            }
        }
        self.global0 = var6.wrapping_add(48i32);
    }
    fn func75<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i64 {
        let var2 = self.func64(imports, var0, var1);
        var2
    }
    fn func76<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64) {
        let var3 = imports.__(self, var0, var1, var2);
        if (var3 & 255i64 != 2i64) as i32 != 0 {
            unreachable!();
        }
    }
    fn func77<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i32, mut var4: i32, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let mut var25: i64 = 0;
        let mut var26: i64 = 0;
        let var27 = self.global0;
        var6 = var27.wrapping_sub(256i32);
        self.global0 = var6;
        let var28 = self.memory.load64(var3 as usize) as i64;
        var15 = var28;
        let var29 = self.memory.load64(var3.wrapping_add(8i32) as usize) as i64;
        var11 = var29;
        let var30 = self.memory.load64(var3 as usize + 96) as i64;
        var7 = var30;
        let var31 = self.memory.load64(var3.wrapping_add(104i32) as usize) as i64;
        var8 = var31;
        let var32 = self.memory.load64(var3 as usize + 80) as i64;
        var12 = var32;
        let var33 = self.memory.load64(var3.wrapping_add(88i32) as usize) as i64;
        var19 = var33;
        self.func67(imports, var6.wrapping_add(240i32), var15, var11, var7, var8, var12, var19, var15, var11);
        'label0: loop {
            if (var1 | var2 == 0) as i32 != 0 {
                self.func78(imports, var0, 80i32);
                break 'label0;
            }
            let var34 = self.memory.load64(var6.wrapping_add(248i32) as usize) as i64;
            var9 = var34;
            let var35 = self.memory.load64(var6 as usize + 240) as i64;
            var20 = var35;
            self.func71(imports, var6.wrapping_add(224i32), var1, var2, 10000i64, 0i64);
            self.memory.store32(var6 as usize + 220, 0i32 as u32);
            let var36 = self.memory.load64(var6 as usize + 224) as i64;
            var13 = var36;
            let var37 = self.memory.load64(var6.wrapping_add(232i32) as usize) as i64;
            var10 = var37;
            var21 = var4 as u32 as i64;
            self.func68(imports, var6.wrapping_add(200i32), var13, var10, var21, 0i64, var6.wrapping_add(220i32));
            'label1: loop {
                let var38 = self.memory.load32(var6 as usize + 220) as i32;
                if var38 != 0 {
                    break 'label1;
                }
                let var39 = self.memory.load64(var6.wrapping_add(208i32) as usize) as i64;
                var17 = var39;
                let var40 = self.memory.load64(var6 as usize + 200) as i64;
                var18 = var40;
                self.memory.store32(var6 as usize + 196, 0i32 as u32);
                var26 = var5 as u32 as i64;
                self.func68(imports, var6.wrapping_add(176i32), var13, var10, var26, 0i64, var6.wrapping_add(196i32));
                let var41 = self.memory.load32(var6 as usize + 196) as i32;
                if var41 != 0 {
                    break 'label1;
                }
                var10 = var2.wrapping_sub(var17).wrapping_sub(((var1 as u64) < var18 as u64) as i32 as u32 as i64);
                if ((var2 ^ var17) & (var2 ^ var10) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                let var42 = self.memory.load64(var6.wrapping_add(184i32) as usize) as i64;
                var22 = var42;
                var16 = var1.wrapping_sub(var18);
                let var43 = self.memory.load64(var6 as usize + 176) as i64;
                var23 = var43;
                var13 = var10.wrapping_sub(var22).wrapping_sub(((var16 as u64) < var23 as u64) as i32 as u32 as i64);
                if ((var10 ^ var22) & (var10 ^ var13) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                let var44 = self.memory.load64(var3.wrapping_add(40i32) as usize) as i64;
                var10 = var44;
                let var45 = self.memory.load64(var3 as usize + 32) as i64;
                var24 = var45;
                var16 = var16.wrapping_sub(var23);
                var25 = var24.wrapping_add(var16);
                var14 = (((var25 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_add(var10.wrapping_add(var13));
                if ((var10 ^ var13 ^ -1i64) & (var10 ^ var14) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                let var46: i64;
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            if ({ let a = ((var20 as u64) < var25 as u64) as i32; let b = (var9 < var14) as i32; if (var9 == var14) as i32 != 0 { a } else { b } }) != 0 {
                                var13 = var9.wrapping_sub(var10).wrapping_sub(((var20 as u64) < var24 as u64) as i32 as u32 as i64);
                                if ((var9 ^ var10) & (var9 ^ var13) < 0i64) as i32 != 0 {
                                    break 'label1;
                                }
                                self.memory.store32(var6 as usize + 84, 0i32 as u32);
                                var16 = var20.wrapping_sub(var24);
                                self.func68(imports, var6.wrapping_sub(-64i32), var16, var13, 10000i64, 0i64, var6.wrapping_add(84i32));
                                let var47 = self.memory.load32(var6 as usize + 84) as i32;
                                if (var47 == 0) as i32 != 0 {
                                    break 'label4;
                                }
                                break 'label1;
                            }
                            var9 = var8 ^ var19;
                            if ((var9 | var7 ^ var12 == 0) as i32 == 0) as i32 != 0 {
                                var10 = var8.wrapping_sub(var19).wrapping_sub(((var7 as u64) < var12 as u64) as i32 as u32 as i64);
                                if (var9 & (var8 ^ var10) < 0i64) as i32 != 0 {
                                    break 'label1;
                                }
                                let var48 = self.func69(imports, var7.wrapping_sub(var12), var10);
                                var10 = var48;
                                let var49 = self.func69(imports, 2i64, 0i64);
                                let var50 = self.func69(imports, var12, var19);
                                let var51 = imports.x(self, var49, var50);
                                let var52 = self.func69(imports, var15, var11);
                                let var53 = imports.x(self, var51, var52);
                                var12 = var53;
                                let var54 = self.func69(imports, -2i64, -1i64);
                                let var55 = self.func69(imports, var15, var11);
                                let var56 = imports.x(self, var54, var55);
                                let var57 = self.func69(imports, 10000000i64, 0i64);
                                let var58 = imports.x(self, var56, var57);
                                let var59 = self.func69(imports, var25, var14);
                                let var60 = imports.x(self, var58, var59);
                                var7 = var60;
                                let var61 = imports.A(self, var12, 8589934596i64);
                                let var62 = self.func69(imports, 4i64, 0i64);
                                let var63 = imports.x(self, var62, var10);
                                let var64 = imports.x(self, var63, var7);
                                let var65 = imports.w(self, var61, var64);
                                var9 = var65;
                                self.func70(imports, var6.wrapping_add(152i32), var9);
                                let var66 = self.memory.load32(var6 as usize + 152) as i32;
                                if var66 != 0 {
                                    var7 = 13i64;
                                    let var67 = self.memory.load64(var6.wrapping_add(168i32) as usize) as i64;
                                    if (var67 < 0i64) as i32 != 0 {
                                        break 'label3;
                                    }
                                }
                                let var68 = imports.v(self, var9, 269i64);
                                var14 = var68;
                                var7 = var9;
                                'label5: loop {
                                    'label6: loop {
                                        let var69 = imports.y(self, var14, 525i64);
                                        var8 = var69;
                                        if ((var8 & 255i64 == 13i64) as i32 & (var7 & 255i64 == 13i64) as i32 == 0) as i32 != 0 {
                                            let var70 = imports._0(self, var8, var7);
                                            if (var70 < 0i64) as i32 != 0 {
                                                break 'label6;
                                            }
                                            break 'label3;
                                        }
                                        if (var8.wrapping_shr(8i64 as u32) >= var7.wrapping_shr(8i64 as u32)) as i32 != 0 {
                                            break 'label3;
                                        }
                                        break;
                                    }
                                    let var71 = imports.y(self, var9, var8);
                                    let var72 = imports.v(self, var71, var8);
                                    var14 = var72;
                                    var7 = var8;
                                    continue 'label5;
                                    break;
                                }
                                unreachable!();
                            }
                            self.memory.store32(var6 as usize + 124, 0i32 as u32);
                            self.func68(imports, var6.wrapping_add(104i32), var25, var14, 10000000i64, 0i64, var6.wrapping_add(124i32));
                            let var73 = self.memory.load32(var6 as usize + 124) as i32;
                            if var73 | (var7 | var8 == 0) as i32 != 0 {
                                break 'label1;
                            }
                            let var74 = self.memory.load64(var6 as usize + 104) as i64;
                            var9 = var74;
                            let var75 = self.memory.load64(var6.wrapping_add(112i32) as usize) as i64;
                            var12 = var75;
                            if (var9 | var12 ^ -9223372036854775808i64 == 0) as i32 & (var7 & var8 == -1i64) as i32 != 0 {
                                break 'label1;
                            }
                            self.func71(imports, var6.wrapping_add(88i32), var9, var12, var7, var8);
                            let var76 = self.memory.load64(var6.wrapping_add(96i32) as usize) as i64;
                            var7 = var76;
                            let var77 = self.memory.load64(var6 as usize + 88) as i64;
                            var46 = var77;
                            break 'label2;
                            break;
                        }
                        var2 = var21.wrapping_add(var26);
                        var1 = var2.wrapping_sub(10000i64);
                        var2 = (((var2 as u64) < var21 as u64) as i32 as u32 as i64).wrapping_add(((var1 as u64) < var2 as u64) as i32 as u32 as i64).wrapping_sub(1i64);
                        if (var1 | var2 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        let var78 = self.memory.load64(var6 as usize + 64) as i64;
                        var7 = var78;
                        let var79 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                        var8 = var79;
                        if (var7 | var8 ^ -9223372036854775808i64 == 0) as i32 & (var1 & var2 == -1i64) as i32 != 0 {
                            break 'label1;
                        }
                        self.func71(imports, var6.wrapping_add(48i32), var7, var8, var1, var2);
                        let var80 = self.memory.load64(var6 as usize + 48) as i64;
                        var1 = var80;
                        let var81 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                        var2 = var81;
                        if (var1 | var2 ^ -9223372036854775808i64 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        self.func71(imports, var6.wrapping_add(32i32), var1, var2, -10000i64, -1i64);
                        self.memory.store32(var6 as usize + 28, 0i32 as u32);
                        let var82 = self.memory.load64(var6 as usize + 32) as i64;
                        let var83 = self.memory.load64(var6.wrapping_add(40i32) as usize) as i64;
                        self.func68(imports, var6.wrapping_add(8i32), var82, var83, var21, 0i64, var6.wrapping_add(28i32));
                        let var84 = self.memory.load32(var6 as usize + 28) as i32;
                        if var84 != 0 {
                            break 'label1;
                        }
                        var2 = 0i64.wrapping_sub(var2.wrapping_add((var1 != 0i64) as i32 as u32 as i64));
                        let var85 = self.memory.load64(var6.wrapping_add(16i32) as usize) as i64;
                        var17 = var85;
                        var1 = 0i64.wrapping_sub(var1);
                        let var86 = self.memory.load64(var6 as usize + 8) as i64;
                        var18 = var86;
                        var7 = var2.wrapping_sub(var17).wrapping_sub(((var1 as u64) < var18 as u64) as i32 as u32 as i64);
                        if ((var2 ^ var17) & (var2 ^ var7) < 0i64) as i32 != 0 {
                            break 'label1;
                        }
                        var8 = var1.wrapping_sub(var18);
                        var22 = var7.wrapping_sub(var13).wrapping_sub(((var8 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((var7 ^ var13) & (var7 ^ var22) < 0i64) as i32 != 0 {
                            break 'label1;
                        }
                        var23 = var8.wrapping_sub(var16);
                        var7 = var11;
                        var46 = var15;
                        break 'label2;
                        break;
                    }
                    let var87 = imports.w(self, var7, var12);
                    let var88 = self.func69(imports, 2i64, 0i64);
                    let var89 = imports.x(self, var88, var10);
                    let var90 = imports.y(self, var87, var89);
                    self.func70(imports, var6.wrapping_add(128i32), var90);
                    let var91 = self.memory.load64(var6.wrapping_add(144i32) as usize) as i64;
                    let var92 = self.memory.load32(var6 as usize + 128) as i32;
                    var4 = var92;
                    var7 = { let a = var91; let b = 0i64; if var4 != 0 { a } else { b } };
                    let var93 = self.memory.load64(var6 as usize + 136) as i64;
                    var46 = { let a = var93; let b = 0i64; if var4 != 0 { a } else { b } };
                    break;
                }
                var8 = var46;
                var4 = { let a = ((var8 as u64) < var15 as u64) as i32; let b = (var7 < var11) as i32; if (var7 == var11) as i32 != 0 { a } else { b } };
                var7 = { let a = var7; let b = var11; if var4 != 0 { a } else { b } };
                let var94 = self.memory.load64(var3.wrapping_add(24i32) as usize) as i64;
                var11 = var94;
                let var95 = var11;
                var8 = { let a = var8; let b = var15; if var4 != 0 { a } else { b } };
                let var96 = self.memory.load64(var3 as usize + 16) as i64;
                var11 = var96;
                var9 = var7.wrapping_sub(var11).wrapping_sub(((var8 as u64) < var11 as u64) as i32 as u32 as i64);
                if ((var7 ^ var95) & (var7 ^ var9) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(var0 as usize + 64, var23 as u64);
                self.memory.store64(var0 as usize + 48, var18 as u64);
                self.memory.store64(var0 as usize + 32, var8.wrapping_sub(var11) as u64);
                self.memory.store64(var0 as usize + 16, var16 as u64);
                self.memory.store64(var0 as usize, var1 as u64);
                self.memory.store64(var0.wrapping_add(72i32) as usize, var22 as u64);
                self.memory.store64(var0.wrapping_add(56i32) as usize, var17 as u64);
                self.memory.store64(var0.wrapping_add(40i32) as usize, var9 as u64);
                self.memory.store64(var0.wrapping_add(24i32) as usize, var13 as u64);
                self.memory.store64(var0 as usize + 8, var2 as u64);
                break 'label0;
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var6.wrapping_add(256i32);
    }
    fn func78<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        if (var1 as u32 >= 16i32 as u32) as i32 != 0 {
            var4 = 0i32.wrapping_sub(var0) & 3i32;
            var2 = var0.wrapping_add(var4);
            'label0: loop {
                if ((var0 as u32) < var2 as u32) as i32 != 0 {
                    self.memory.store8(var0 as usize, 0i32 as u8);
                    var0 = var0.wrapping_add(1i32);
                    continue 'label0;
                }
                break;
            }
            var0 = 8i32;
            'label1: loop {
                if (var0 as u32 >= 32i32 as u32) as i32 != 0 {
                    'label2: loop {
                        var1 = var1.wrapping_sub(var4);
                        var0 = var2.wrapping_add(var1 & -4i32);
                        'label3: loop {
                            if (var0 as u32 <= var2 as u32) as i32 != 0 {
                                break 'label2;
                            }
                            self.memory.store32(var2 as usize, var3 as u32);
                            var2 = var2.wrapping_add(4i32);
                            continue 'label3;
                            break;
                        }
                        unreachable!();
                        break;
                    }
                } else {
                    var3 = var3.wrapping_shl((var0 & 24i32) as u32) | var3;
                    var0 = var0.wrapping_shl(1i32 as u32);
                    continue 'label1;
                }
                break;
            }
            var1 = var1 & 3i32;
        }
        var1 = var0.wrapping_add(var1);
        'label4: loop {
            if ((var0 as u32) < var1 as u32) as i32 != 0 {
                self.memory.store8(var0 as usize, 0i32 as u8);
                var0 = var0.wrapping_add(1i32);
                continue 'label4;
            }
            break;
        }
    }
    fn func79<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i32, mut var4: i32, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var6 = var16.wrapping_sub(80i32);
        self.global0 = var6;
        'label0: loop {
            if (var1 | var2 == 0) as i32 != 0 {
                self.func78(imports, var0, 80i32);
                break 'label0;
            }
            'label1: loop {
                let var17 = self.memory.load64(var3 as usize + 16) as i64;
                var8 = var17;
                let var18 = self.memory.load64(var3.wrapping_add(24i32) as usize) as i64;
                var7 = var18;
                if ({ let a = (var8 as u64 > var1 as u64) as i32; let b = (var7 > var2) as i32; if (var2 == var7) as i32 != 0 { a } else { b } }) != 0 {
                    var9 = var7.wrapping_sub(var2).wrapping_sub((var1 as u64 > var8 as u64) as i32 as u32 as i64);
                    if ((var2 ^ var7) & (var7 ^ var9) < 0i64) as i32 != 0 {
                        break 'label1;
                    }
                    let var19 = self.memory.load64(var3 as usize + 96) as i64;
                    let var20 = self.memory.load64(var3.wrapping_add(104i32) as usize) as i64;
                    let var21 = self.memory.load64(var3 as usize + 80) as i64;
                    let var22 = self.memory.load64(var3.wrapping_add(88i32) as usize) as i64;
                    let var23 = self.memory.load64(var3 as usize) as i64;
                    let var24 = self.memory.load64(var3.wrapping_add(8i32) as usize) as i64;
                    self.func67(imports, var6.wrapping_sub(-64i32), var8.wrapping_sub(var1), var9, var19, var20, var21, var22, var23, var24);
                    let var25 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                    var10 = var25;
                    let var26 = self.memory.load64(var6 as usize + 64) as i64;
                    var9 = var26;
                }
                let var27 = self.memory.load64(var3.wrapping_add(40i32) as usize) as i64;
                var8 = var27;
                let var28 = var10;
                let var29 = self.memory.load64(var3 as usize + 32) as i64;
                var10 = var29;
                var7 = var8.wrapping_sub(var10).wrapping_sub(((var10 as u64) < var9 as u64) as i32 as u32 as i64);
                if ((var8 ^ var28) & (var8 ^ var7) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                var8 = var10.wrapping_sub(var9);
                self.func71(imports, var6.wrapping_add(48i32), var8, var7, 10000i64, 0i64);
                self.memory.store32(var6 as usize + 44, 0i32 as u32);
                let var30 = self.memory.load64(var6 as usize + 48) as i64;
                var11 = var30;
                let var31 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                var12 = var31;
                self.func68(imports, var6.wrapping_add(24i32), var11, var12, var4 as u32 as i64, 0i64, var6.wrapping_add(44i32));
                let var32 = self.memory.load32(var6 as usize + 44) as i32;
                if var32 != 0 {
                    break 'label1;
                }
                let var33 = self.memory.load64(var6.wrapping_add(32i32) as usize) as i64;
                var9 = var33;
                let var34 = self.memory.load64(var6 as usize + 24) as i64;
                var10 = var34;
                self.memory.store32(var6 as usize + 20, 0i32 as u32);
                self.func68(imports, var6, var11, var12, var5 as u32 as i64, 0i64, var6.wrapping_add(20i32));
                let var35 = self.memory.load32(var6 as usize + 20) as i32;
                if var35 != 0 {
                    break 'label1;
                }
                var11 = var7.wrapping_sub(var9).wrapping_sub(((var8 as u64) < var10 as u64) as i32 as u32 as i64);
                if ((var7 ^ var9) & (var7 ^ var11) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                let var36 = self.memory.load64(var6.wrapping_add(8i32) as usize) as i64;
                var12 = var36;
                var14 = var8.wrapping_sub(var10);
                let var37 = self.memory.load64(var6 as usize) as i64;
                var13 = var37;
                var15 = var11.wrapping_sub(var12).wrapping_sub(((var14 as u64) < var13 as u64) as i32 as u32 as i64);
                if ((var11 ^ var12) & (var11 ^ var15) < 0i64) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(var0 as usize + 64, var13 as u64);
                self.memory.store64(var0 as usize + 48, var10 as u64);
                self.memory.store64(var0 as usize + 32, var14.wrapping_sub(var13) as u64);
                self.memory.store64(var0 as usize + 16, var8 as u64);
                self.memory.store64(var0 as usize, var1 as u64);
                self.memory.store64(var0.wrapping_add(72i32) as usize, var12 as u64);
                self.memory.store64(var0.wrapping_add(56i32) as usize, var9 as u64);
                self.memory.store64(var0.wrapping_add(40i32) as usize, var15 as u64);
                self.memory.store64(var0.wrapping_add(24i32) as usize, var7 as u64);
                self.memory.store64(var0 as usize + 8, var2 as u64);
                break 'label0;
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var6.wrapping_add(80i32);
    }
    fn func80<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64, mut var5: i64, mut var6: i64, mut var7: i64) -> i64 {
        let mut var8: i32 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let var15 = self.global0;
        var8 = var15.wrapping_sub(96i32);
        self.global0 = var8;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func42(imports, var8.wrapping_add(24i32), var1);
            let var16 = self.memory.load64(var8 as usize + 24) as i64;
            if var16 as i32 | (var2 & 255i64 != 4i64) as i32 | (var3 & 255i64 != 4i64) as i32 | (var4 & 255i64 != 77i64) as i32 | (var5 & 255i64 != 77i64) as i32 | (var6 & 255i64 != 77i64) as i32 | (var7 & 255i64 != 76i64) as i32 != 0 {
                break 'label0;
            }
            let var17 = self.memory.load64(var8 as usize + 32) as i64;
            var13 = var17;
            let var18 = self.func54(imports, 1049208i32);
            if (var18 == 0) as i32 != 0 {
                let var19 = imports.__(self);
                var10 = var19;
                let var20 = imports._3(self, var7);
                var14 = (var20 as u64).wrapping_shr(32i64 as u32) as i64;
                var9 = 4i64;
                var1 = 0i64;
                'label1: loop {
                    if ((var1 as u64) < var14 as u64) as i32 != 0 {
                        let var21 = imports._5(self, var7, var9);
                        var12 = var21;
                        let var22 = imports._6(self, var7, var9);
                        var11 = var22;
                        if (var1 == 4294967295i64) as i32 | (var12 & 255i64 != 4i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.func42(imports, var8.wrapping_add(8i32), var11);
                        let var23 = self.memory.load64(var8 as usize + 8) as i64;
                        if var23 as i32 != 0 {
                            break 'label0;
                        }
                        let var24 = self.memory.load64(var8 as usize + 16) as i64;
                        let var25 = self.func61(imports, var24);
                        var11 = var25;
                        let var26 = self.func52(imports, 0i64, 0i64);
                        self.memory.store64(var8 as usize + 48, var26 as u64);
                        self.memory.store64(var8 as usize + 40, var11 as u64);
                        var9 = var9.wrapping_add(4294967296i64);
                        var1 = var1.wrapping_add(1i64);
                        let var27 = self.func62(imports, 1049192i32, 2i32, var8.wrapping_add(40i32), 2i32);
                        let var28 = imports._0(self, var10, var12 & -4294967296i64 | 4i64, var27);
                        var10 = var28;
                        continue 'label1;
                    }
                    break;
                }
                self.memory.store64(var8 as usize + 40, var0 as u64);
                self.memory.store64(var8 as usize + 80, var10 as u64);
                self.memory.store64(var8 as usize + 72, var6 as u64);
                self.memory.store64(var8 as usize + 64, var5 as u64);
                self.memory.store64(var8 as usize + 56, var4 as u64);
                self.memory.store32(var8 as usize + 92, (var3 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store32(var8 as usize + 88, (var2 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store64(var8 as usize + 48, var13 as u64);
                self.func55(imports, var8.wrapping_add(40i32));
                let var29 = self.func75(imports, 1048576i32, 11i32);
                let var30 = self.func63(imports, var29);
                let var31 = imports._1(self, var30, 1i64);
                var31;
                self.global0 = var8.wrapping_add(96i32);
                return 2i64;
            }
            self.func81(imports, 4294967299i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func81<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) {
        let var1 = imports._5(self, var0);
        var1;
    }
    fn func82<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64, mut var5: i64) -> i64 {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let var16 = self.global0;
        var6 = var16.wrapping_sub(160i32);
        self.global0 = var6;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func42(imports, var6.wrapping_add(24i32), var1);
            let var17 = self.memory.load64(var6 as usize + 24) as i64;
            if var17 as i32 | (var2 & 255i64 != 4i64) as i32 | (var3 & 255i64 != 4i64) as i32 | (var4 & 255i64 != 77i64) as i32 | (var5 & 255i64 != 76i64) as i32 != 0 {
                break 'label0;
            }
            let var18 = self.memory.load64(var6 as usize + 32) as i64;
            var14 = var18;
            self.func53(imports, var6.wrapping_add(96i32));
            let var19 = self.memory.load64(var6 as usize + 96) as i64;
            if (var19 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var20 = self.func83(imports, var6.wrapping_add(40i32), var6.wrapping_add(104i32), 56i32);
            var20;
            let var21 = self.memory.load64(var6 as usize + 40) as i64;
            let var22 = imports._0(self, var21);
            var22;
            let var23 = self.memory.load64(var6 as usize + 80) as i64;
            var8 = var23;
            let var24 = imports._3(self, var5);
            var15 = (var24 as u64).wrapping_shr(32i64 as u32) as i64;
            var7 = var6.wrapping_add(112i32);
            var11 = 4i64;
            var1 = 0i64;
            'label1: loop {
                if ((var1 as u64) < var15 as u64) as i32 != 0 {
                    let var25 = imports._5(self, var5, var11);
                    var9 = var25;
                    let var26 = imports._6(self, var5, var11);
                    var10 = var26;
                    if (var1 == 4294967295i64) as i32 | (var9 & 255i64 != 4i64) as i32 != 0 {
                        break 'label0;
                    }
                    self.func42(imports, var6.wrapping_add(8i32), var10);
                    let var27 = self.memory.load64(var6 as usize + 8) as i64;
                    if var27 as i32 != 0 {
                        break 'label0;
                    }
                    let var28 = self.memory.load64(var6 as usize + 16) as i64;
                    var10 = 0i64;
                    var13 = 0i64;
                    var9 = var9 & -4294967296i64 | 4i64;
                    let var29 = imports._4(self, var8, var9);
                    if (var29 == 1i64) as i32 != 0 {
                        let var30 = imports._1(self, var8, var9);
                        self.func73(imports, var6.wrapping_add(96i32), var30);
                        let var31 = self.memory.load64(var6 as usize + 96) as i64;
                        if (var31 != 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        let var32 = self.memory.load64(var7 as usize) as i64;
                        var13 = var32;
                        let var33 = self.memory.load64(var6 as usize + 104) as i64;
                        var10 = var33;
                    }
                    let var34 = self.func61(imports, var28);
                    var12 = var34;
                    let var35 = self.func52(imports, var10, var13);
                    self.memory.store64(var6 as usize + 104, var35 as u64);
                    self.memory.store64(var6 as usize + 96, var12 as u64);
                    var11 = var11.wrapping_add(4294967296i64);
                    var1 = var1.wrapping_add(1i64);
                    let var36 = self.func62(imports, 1049192i32, 2i32, var6.wrapping_add(96i32), 2i32);
                    let var37 = imports._0(self, var8, var9, var36);
                    var8 = var37;
                    continue 'label1;
                }
                break;
            }
            self.memory.store64(var6 as usize + 40, var0 as u64);
            self.memory.store64(var6 as usize + 80, var8 as u64);
            self.memory.store64(var6 as usize + 56, var4 as u64);
            self.memory.store32(var6 as usize + 92, (var3 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
            self.memory.store32(var6 as usize + 88, (var2 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
            self.memory.store64(var6 as usize + 48, var14 as u64);
            self.func55(imports, var6.wrapping_add(40i32));
            self.global0 = var6.wrapping_add(160i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func83<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        'label0: loop {
            if ((var2 as u32) < 16i32 as u32) as i32 != 0 {
                var3 = var0;
                break 'label0;
            }
            var6 = 0i32.wrapping_sub(var0) & 3i32;
            var4 = var0.wrapping_add(var6);
            var5 = var1;
            var3 = var0;
            'label1: loop {
                if ((var3 as u32) < var4 as u32) as i32 != 0 {
                    let var10 = self.memory.load8(var5 as usize) as i32;
                    self.memory.store8(var3 as usize, var10 as u8);
                    var5 = var5.wrapping_add(1i32);
                    var3 = var3.wrapping_add(1i32);
                    continue 'label1;
                }
                break;
            }
            var8 = var2.wrapping_sub(var6);
            var9 = var8 & -4i32;
            var3 = var4.wrapping_add(var9);
            'label2: loop {
                var5 = var1.wrapping_add(var6);
                if (var5 & 3i32 == 0) as i32 != 0 {
                    var1 = var5;
                    'label3: loop {
                        if (var3 as u32 <= var4 as u32) as i32 != 0 {
                            break 'label2;
                        }
                        let var11 = self.memory.load32(var1 as usize) as i32;
                        self.memory.store32(var4 as usize, var11 as u32);
                        var1 = var1.wrapping_add(4i32);
                        var4 = var4.wrapping_add(4i32);
                        continue 'label3;
                        break;
                    }
                    unreachable!();
                }
                var2 = var5 & -4i32;
                var1 = var2.wrapping_add(4i32);
                var7 = var5.wrapping_shl(3i32 as u32);
                var6 = var7 & 24i32;
                var7 = 0i32.wrapping_sub(var7) & 24i32;
                let var12 = self.memory.load32(var2 as usize) as i32;
                var2 = var12;
                'label4: loop {
                    if (var3 as u32 <= var4 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    if var6 != 0 {
                        let var13 = var2;
                        let var14 = self.memory.load32(var1 as usize) as i32;
                        var2 = var14;
                        self.memory.store32(var4 as usize, ((var13 as u32).wrapping_shr(var6 as u32) as i32 | var2.wrapping_shl(var7 as u32)) as u32);
                        var1 = var1.wrapping_add(4i32);
                        var4 = var4.wrapping_add(4i32);
                        continue 'label4;
                    }
                    break;
                }
                unreachable!();
                break;
            }
            var2 = var8 & 3i32;
            var1 = var5.wrapping_add(var9);
            break;
        }
        var2 = var2.wrapping_add(var3);
        'label5: loop {
            if (var2 as u32 > var3 as u32) as i32 != 0 {
                let var15 = self.memory.load8(var1 as usize) as i32;
                self.memory.store8(var3 as usize, var15 as u8);
                var1 = var1.wrapping_add(1i32);
                var3 = var3.wrapping_add(1i32);
                continue 'label5;
            }
            break;
        }
        var0
    }
    fn func84<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_add(-64i32);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (var0 & 255i64 != 72i64) as i32 != 0 {
                    break 'label1;
                }
                let var3 = imports._8(self, var0);
                if (var3 & -4294967296i64 != 137438953472i64) as i32 != 0 {
                    break 'label1;
                }
                self.func53(imports, var1);
                let var4 = self.memory.load64(var1 as usize) as i64;
                if (var4 != 0i64) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, 3i64);
                break;
            }
            unreachable!();
            break;
        }
        let var5 = self.memory.load64(var1 as usize + 8) as i64;
        let var6 = imports._0(self, var5);
        var6;
        let var7 = imports._6(self, var0);
        var7;
        self.global0 = var1.wrapping_sub(-64i32);
        2i64
    }
    fn func85<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let var15 = self.global0;
        var4 = var15.wrapping_sub(160i32);
        self.global0 = var4;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var4.wrapping_add(80i32), var1);
            let var16 = self.memory.load64(var4 as usize + 80) as i64;
            if ((var16 == 0) as i32 == 0) as i32 | (var2 & 255i64 != 4i64) as i32 != 0 {
                break 'label0;
            }
            let var17 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
            var7 = var17;
            let var18 = self.memory.load64(var4 as usize + 88) as i64;
            var10 = var18;
            self.func46(imports, var4.wrapping_add(80i32), var3);
            let var19 = self.memory.load64(var4 as usize + 80) as i64;
            if ((var19 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var20 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
            var8 = var20;
            let var21 = self.memory.load64(var4 as usize + 88) as i64;
            var9 = var21;
            self.func53(imports, var4.wrapping_add(80i32));
            let var22 = self.memory.load64(var4 as usize + 80) as i64;
            if (var22 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var23 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(88i32), 56i32);
            var23;
            let var24 = imports._0(self, var0);
            var24;
            self.memory.store32(var4 as usize + 20, 0i32 as u32);
            self.func68(imports, var4, var10, var7, 10000000i64, 0i64, var4.wrapping_add(20i32));
            let var25 = self.memory.load32(var4 as usize + 20) as i32;
            if var25 != 0 {
                break 'label0;
            }
            let var26 = self.memory.load64(var4 as usize) as i64;
            var3 = var26;
            let var27 = self.memory.load64(var4.wrapping_add(8i32) as usize) as i64;
            var1 = var27;
            if (({ let a = (var3 == 0) as i32; let b = (var1 < 0i64) as i32; if (var1 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var28 = self.memory.load64(var4 as usize + 48) as i64;
                var11 = var28;
                let var29 = self.func75(imports, 1049280i32, 4i32);
                var12 = var29;
                let var30 = self.func52(imports, var3, var1);
                self.memory.store64(var4 as usize + 152, var30 as u64);
                self.memory.store64(var4 as usize + 144, var0 as u64);
                'label1: loop {
                    if (var5 == 16i32) as i32 != 0 {
                        var5 = 0i32;
                        'label2: loop {
                            if (var5 != 16i32) as i32 != 0 {
                                let var31 = self.memory.load64(var4.wrapping_add(144i32).wrapping_add(var5) as usize) as i64;
                                self.memory.store64(var4.wrapping_add(80i32).wrapping_add(var5) as usize, var31 as u64);
                                var5 = var5.wrapping_add(8i32);
                                continue 'label2;
                            }
                            break;
                        }
                        let var32 = self.func59(imports, var4.wrapping_add(80i32), 2i32);
                        self.func76(imports, var11, var12, var32);
                    } else {
                        self.memory.store64(var4.wrapping_add(80i32).wrapping_add(var5) as usize, 2i64 as u64);
                        var5 = var5.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
            }
            'label3: loop {
                'label4: loop {
                    let var33 = self.memory.load64(var4 as usize + 64) as i64;
                    var11 = var33;
                    var12 = var2 & -4294967296i64 | 4i64;
                    let var34 = imports._4(self, var11, var12);
                    if (var34 == 1i64) as i32 != 0 {
                        let var35 = imports._1(self, var11, var12);
                        self.func73(imports, var4.wrapping_add(80i32), var35);
                        let var36 = self.memory.load64(var4 as usize + 80) as i64;
                        if ((var36 == 0) as i32 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        let var37 = self.memory.load64(var4.wrapping_add(104i32) as usize) as i64;
                        var13 = var37;
                        var5 = (var7 == 0) as i32;
                        if ({ let a = (var10 as u64 > var13 as u64) as i32; let b = (var7 > 0i64) as i32; if var5 != 0 { a } else { b } }) != 0 {
                            break 'label4;
                        }
                        let var38 = self.memory.load64(var4 as usize + 88) as i64;
                        var3 = var38;
                        let var39 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
                        var1 = var39;
                        if ({ let a = (var9 as u64 > var3 as u64) as i32; let b = (var1 < var8) as i32; if (var1 == var8) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label3;
                        }
                        if (var1 | var3 == 0) as i32 != 0 {
                            self.func81(imports, 1301375090691i64);
                            break 'label0;
                        }
                        let var40 = imports._1(self, 1i64, var13);
                        var5 = { let a = (var40 as u64 <= var10 as u64) as i32; let b = (var7 >= 0i64) as i32; if var5 != 0 { a } else { b } };
                        var8 = { let a = var1; let b = 0i64; if var5 != 0 { a } else { b } };
                        var9 = { let a = var3; let b = 1i64; if var5 != 0 { a } else { b } };
                        var14 = var1.wrapping_sub(var8).wrapping_sub(((var3 as u64) < var9 as u64) as i32 as u32 as i64);
                        if ((var1 ^ var8) & (var1 ^ var14) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        let var41 = self.func61(imports, var13);
                        var1 = var41;
                        let var42 = self.func52(imports, var3.wrapping_sub(var9), var14);
                        self.memory.store64(var4 as usize + 88, var42 as u64);
                        self.memory.store64(var4 as usize + 80, var1 as u64);
                        var6 = var4.wrapping_add(80i32);
                        let var43 = self.func62(imports, 1049192i32, 2i32, var6, 2i32);
                        let var44 = imports._0(self, var11, var12, var43);
                        self.memory.store64(var4 as usize + 64, var44 as u64);
                        self.func55(imports, var4.wrapping_add(24i32));
                        let var45 = imports._7(self);
                        var1 = var45;
                        let var46 = self.memory.load64(var4 as usize + 56) as i64;
                        self.func74(imports, var46, var1, var0, var9, var8);
                        let var47 = self.func75(imports, 1048649i32, 13i32);
                        let var48 = self.func63(imports, var47);
                        let var49 = self.func52(imports, var10, var7);
                        var3 = var49;
                        let var50 = self.func52(imports, var9, var8);
                        self.memory.store64(var4 as usize + 104, var50 as u64);
                        var7 = var5 as u32 as i64;
                        self.memory.store64(var4 as usize + 96, var7 as u64);
                        self.memory.store64(var4 as usize + 88, var3 as u64);
                        self.memory.store64(var4 as usize + 80, (var2 & -4294967296i64 | 4i64) as u64);
                        let var51 = self.func59(imports, var6, 4i32);
                        self.memory.store64(var4 as usize + 152, var51 as u64);
                        self.memory.store64(var4 as usize + 144, var0 as u64);
                        let var52 = self.func59(imports, var4.wrapping_add(144i32), 2i32);
                        let var53 = imports._1(self, var48, var52);
                        var53;
                        let var54 = self.func52(imports, var9, var8);
                        self.memory.store64(var4 as usize + 88, var54 as u64);
                        self.memory.store64(var4 as usize + 80, var7 as u64);
                        let var55 = self.func59(imports, var6, 2i32);
                        self.global0 = var4.wrapping_add(160i32);
                        return var55;
                    }
                    self.func81(imports, 1288490188803i64);
                    break 'label0;
                    break;
                }
                self.func81(imports, 1292785156099i64);
                break 'label0;
                break;
            }
            self.func81(imports, 1297080123395i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func86<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64) -> i64 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let var17 = self.global0;
        var3 = var17.wrapping_sub(128i32);
        self.global0 = var3;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var3.wrapping_sub(-64i32), var1);
            let var18 = self.memory.load64(var3 as usize + 64) as i64;
            if ((var18 == 0) as i32 == 0) as i32 | (var2 & 255i64 != 4i64) as i32 != 0 {
                break 'label0;
            }
            let var19 = self.memory.load64(var3.wrapping_add(80i32) as usize) as i64;
            var8 = var19;
            let var20 = self.memory.load64(var3 as usize + 72) as i64;
            var11 = var20;
            self.func53(imports, var3.wrapping_sub(-64i32));
            let var21 = self.memory.load64(var3 as usize + 64) as i64;
            if (var21 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            var4 = (var2 as u64).wrapping_shr(32i64 as u32) as i64 as i32;
            let var22 = self.func83(imports, var3.wrapping_add(8i32), var3.wrapping_add(72i32), 56i32);
            var22;
            let var23 = imports._0(self, var0);
            var23;
            var12 = var2 & -4294967296i64 | 4i64;
            let var24 = self.memory.load64(var3 as usize + 48) as i64;
            var9 = var24;
            let var25 = imports._3(self, var9);
            var13 = (var25 as u64).wrapping_shr(32i64 as u32) as i64;
            let var26 = self.memory.load64(var3 as usize + 40) as i64;
            var14 = var26;
            var5 = var3.wrapping_add(80i32);
            var1 = 4i64;
            var2 = 0i64;
            var10 = var9;
            'label1: loop {
                'label2: loop {
                    if ((var2 as u64) < var13 as u64) as i32 != 0 {
                        let var27 = imports._5(self, var9, var1);
                        var6 = var27;
                        let var28 = imports._6(self, var9, var1);
                        var7 = var28;
                        if (var2 == 4294967295i64) as i32 | (var6 & 255i64 != 4i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.func73(imports, var3.wrapping_sub(-64i32), var7);
                        let var29 = self.memory.load64(var3 as usize + 64) as i64;
                        if ((var29 == 0) as i32 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        if (((var6 as u64).wrapping_shr(32i64 as u32) as i64 as i32) != var4) as i32 != 0 {
                            break 'label2;
                        }
                        let var30 = self.memory.load64(var5 as usize) as i64;
                        var6 = var30;
                        let var31 = self.memory.load64(var3 as usize + 72) as i64;
                        var7 = var31;
                        let var32 = self.memory.load64(var3 as usize + 88) as i64;
                        let var33 = imports._7(self);
                        self.func74(imports, var14, var0, var33, var11, var8);
                        var16 = var7.wrapping_add(var11);
                        var7 = ((var7 as u64 > var16 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_add(var8));
                        if ((var6 ^ var8 ^ -1i64) & (var6 ^ var7) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        let var34 = self.func61(imports, var32);
                        var6 = var34;
                        let var35 = self.func52(imports, var16, var7);
                        self.memory.store64(var3 as usize + 72, var35 as u64);
                        self.memory.store64(var3 as usize + 64, var6 as u64);
                        let var36 = self.func62(imports, 1049192i32, 2i32, var3.wrapping_sub(-64i32), 2i32);
                        let var37 = imports._0(self, var10, var12, var36);
                        var10 = var37;
                        break 'label2;
                    }
                    self.memory.store64(var3 as usize + 48, var10 as u64);
                    self.func55(imports, var3.wrapping_add(8i32));
                    self.global0 = var3.wrapping_add(128i32);
                    return 2i64;
                    break;
                }
                var1 = var1.wrapping_add(4294967296i64);
                var2 = var2.wrapping_add(1i64);
                continue 'label1;
                break;
            }
            unreachable!();
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func87<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64, mut var5: i64, mut var6: i64, mut var7: i64) -> i64 {
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let var18 = self.global0;
        var8 = var18.wrapping_sub(416i32);
        self.global0 = var8;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 | (var1 & 255i64 != 77i64) as i32 | (var2 & 255i64 != 73i64) as i32 | (var3 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var8.wrapping_add(248i32), var4);
            let var19 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var19 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var9 = var8.wrapping_add(264i32);
            let var20 = self.memory.load64(var9 as usize) as i64;
            var4 = var20;
            let var21 = self.memory.load64(var8 as usize + 256) as i64;
            var14 = var21;
            self.func46(imports, var8.wrapping_add(248i32), var5);
            let var22 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var22 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var23 = self.memory.load64(var9 as usize) as i64;
            var15 = var23;
            let var24 = self.memory.load64(var8 as usize + 256) as i64;
            var16 = var24;
            self.func46(imports, var8.wrapping_add(248i32), var6);
            let var25 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var25 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var26 = self.memory.load64(var8.wrapping_add(264i32) as usize) as i64;
            var5 = var26;
            let var27 = self.memory.load64(var8 as usize + 256) as i64;
            var6 = var27;
            self.func42(imports, var8.wrapping_add(32i32), var7);
            let var28 = self.memory.load32(var8 as usize + 32) as i32;
            if var28 != 0 {
                break 'label0;
            }
            let var29 = self.memory.load64(var8 as usize + 40) as i64;
            var7 = var29;
            let var30 = self.func54(imports, 1049208i32);
            if (var30 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var31 = imports._0(self, var0);
            var31;
            let var32 = self.func66(imports);
            var17 = var32;
            if (var4 | var14 == 0) as i32 != 0 {
                self.func81(imports, 442381631491i64);
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    if (({ let a = (var14 < 0i64) as i32; let b = (var4 > 0i64) as i32; if (var4 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        self.memory.store32(var8 as usize + 28, 0i32 as u32);
                        self.func68(imports, var8.wrapping_add(8i32), var14, var4, var6, var5, var8.wrapping_add(28i32));
                        let var33 = self.memory.load32(var8 as usize + 28) as i32;
                        if var33 != 0 {
                            break 'label0;
                        }
                        let var34 = self.memory.load64(var8.wrapping_add(16i32) as usize) as i64;
                        if (var34 > 4999999i64) as i32 != 0 {
                            break 'label2;
                        }
                        if (var15 | var16 == 0) as i32 != 0 {
                            self.func81(imports, 429496729603i64);
                            break 'label0;
                        }
                        if (var5 | var6 == 0) as i32 != 0 {
                            self.func81(imports, 433791696899i64);
                            break 'label0;
                        }
                        if ({ let a = ((var6 as u64) < var16 as u64) as i32; let b = (var5 < var15) as i32; if (var5 == var15) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label1;
                        }
                        self.memory.store64(var8 as usize + 264, var7 as u64);
                        self.memory.store64(var8 as usize + 256, var0 as u64);
                        self.memory.store64(var8 as usize + 248, 1i64 as u64);
                        let var35 = self.func54(imports, var8.wrapping_add(248i32));
                        if var35 != 0 {
                            self.func81(imports, 863288426499i64);
                            break 'label0;
                        }
                        let var36 = imports._7(self);
                        self.func74(imports, var3, var0, var36, var14, var4);
                        var10 = var8.wrapping_add(184i32);
                        self.memory.store64(var10 as usize, var3 as u64);
                        var11 = var8.wrapping_add(176i32);
                        self.memory.store64(var11 as usize, var2 as u64);
                        var12 = var8.wrapping_add(192i32);
                        self.memory.store64(var12 as usize, 0i64 as u64);
                        self.memory.store64(var8 as usize + 56, var4 as u64);
                        self.memory.store64(var8 as usize + 48, var14 as u64);
                        self.memory.store64(var8 as usize + 168, var1 as u64);
                        self.memory.store64(var8 as usize + 160, var0 as u64);
                        self.memory.store16(var8 as usize + 208, 0i32 as u16);
                        self.func78(imports, var8.wrapping_sub(-64i32), 64i32);
                        self.memory.store64(var8.wrapping_add(152i32) as usize, var5 as u64);
                        self.memory.store64(var8.wrapping_add(136i32) as usize, var15 as u64);
                        self.memory.store64(var8 as usize + 144, var6 as u64);
                        self.memory.store64(var8 as usize + 128, var16 as u64);
                        self.memory.store64(var8 as usize + 200, var17 as u64);
                        self.memory.store64(var8 as usize + 264, var7 as u64);
                        self.memory.store64(var8 as usize + 256, var0 as u64);
                        self.memory.store64(var8 as usize + 248, 1i64 as u64);
                        var9 = var8.wrapping_add(248i32);
                        var13 = var8.wrapping_add(48i32);
                        self.func49(imports, var9, var13);
                        let var37 = self.func83(imports, var9, var13, 112i32);
                        var37;
                        let var38 = self.memory.load64(var12 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(392i32) as usize, var38 as u64);
                        let var39 = self.memory.load64(var10 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(384i32) as usize, var39 as u64);
                        let var40 = self.memory.load64(var11 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(376i32) as usize, var40 as u64);
                        let var41 = self.memory.load64(var8 as usize + 168) as i64;
                        self.memory.store64(var8 as usize + 368, var41 as u64);
                        let var42 = self.memory.load64(var8 as usize + 208) as i64;
                        var1 = var42;
                        let var43 = self.func75(imports, 1048587i32, 10i32);
                        var2 = var43;
                        self.memory.store64(var8.wrapping_add(240i32) as usize, var17 as u64);
                        self.memory.store64(var8.wrapping_add(232i32) as usize, var7 as u64);
                        self.memory.store64(var8 as usize + 224, var0 as u64);
                        self.memory.store64(var8 as usize + 216, var2 as u64);
                        self.memory.store64(var8 as usize + 408, var1 as u64);
                        self.memory.store64(var8 as usize + 400, var17 as u64);
                        self.memory.store64(var8 as usize + 360, var0 as u64);
                        let var44 = self.func60(imports, var8.wrapping_add(216i32));
                        let var45 = self.func50(imports, var9);
                        let var46 = imports._1(self, var44, var45);
                        var46;
                        self.global0 = var8.wrapping_add(416i32);
                        return 2i64;
                    }
                    self.func81(imports, 446676598787i64);
                    break 'label0;
                    break;
                }
                self.func81(imports, 450971566083i64);
                break 'label0;
                break;
            }
            self.func81(imports, 438086664195i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func88<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let var12 = self.global0;
        var1 = var12.wrapping_sub(208i32);
        self.global0 = var1;
        self.func57(imports, var1.wrapping_add(8i32), var0);
        'label0: loop {
            'label1: loop {
                let var13 = self.memory.load64(var1 as usize + 8) as i64;
                if ((var13 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var14 = self.memory.load64(var1.wrapping_add(24i32) as usize) as i64;
                var0 = var14;
                let var15 = self.memory.load64(var1 as usize + 16) as i64;
                var5 = var15;
                let var16 = self.func54(imports, 1049208i32);
                if (var16 == 0) as i32 != 0 {
                    self.func81(imports, 3i64);
                    break 'label1;
                }
                self.memory.store64(var1 as usize + 192, var0 as u64);
                self.memory.store64(var1 as usize + 184, var5 as u64);
                self.memory.store64(var1 as usize + 176, 1i64 as u64);
                self.func47(imports, var1.wrapping_add(8i32), var1.wrapping_add(176i32));
                let var17 = self.memory.load8(var1 as usize + 169) as i32;
                if (var17 == 2i32) as i32 != 0 {
                    self.func81(imports, 858993459203i64);
                    break 'label1;
                }
                let var18 = self.memory.load64(var1.wrapping_add(16i32) as usize) as i64;
                var6 = var18;
                var2 = var1.wrapping_add(32i32);
                let var19 = self.memory.load64(var2 as usize) as i64;
                var4 = var19;
                let var20 = self.memory.load64(var1 as usize + 8) as i64;
                var7 = var20;
                let var21 = self.memory.load64(var1 as usize + 160) as i64;
                var8 = var21;
                let var22 = self.memory.load64(var1 as usize + 144) as i64;
                var9 = var22;
                let var23 = self.memory.load64(var1 as usize + 24) as i64;
                let var24 = self.memory.load64(var1 as usize + 120) as i64;
                var11 = var24;
                let var25 = imports._0(self, var11);
                var25;
                if ({ let a = (var23 == 0) as i32; let b = (var4 < 0i64) as i32; if (var4 == 0) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                self.func81(imports, 863288426499i64);
                break;
            }
            unreachable!();
            break;
        }
        let var26 = imports._7(self);
        self.func74(imports, var9, var26, var11, var7, var6);
        self.memory.store64(var1 as usize + 24, var0 as u64);
        self.memory.store64(var1 as usize + 16, var5 as u64);
        self.memory.store64(var1 as usize + 8, 1i64 as u64);
        var3 = var1.wrapping_add(8i32);
        let var27 = self.func44(imports, var3);
        self.func89(imports, var27);
        let var28 = self.func75(imports, 1048617i32, 13i32);
        var4 = var28;
        self.memory.store64(var2 as usize, var8 as u64);
        self.memory.store64(var1.wrapping_add(24i32) as usize, var0 as u64);
        self.memory.store64(var1 as usize + 16, var5 as u64);
        self.memory.store64(var1 as usize + 8, var4 as u64);
        let var29 = self.func60(imports, var3);
        let var30 = imports._1(self, var29, 1i64);
        var30;
        self.global0 = var1.wrapping_add(208i32);
        2i64
    }
    fn func89<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) {
        let var1 = imports._2(self, var0, 1i64);
        var1;
    }
    fn func90<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let mut var23: i64 = 0;
        let mut var24: i64 = 0;
        let mut var25: i64 = 0;
        let mut var26: i64 = 0;
        let mut var27: i64 = 0;
        let var28 = self.global0;
        var4 = var28.wrapping_sub(576i32);
        self.global0 = var4;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var4.wrapping_add(280i32), var1);
            let var29 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var29 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var5 = var4.wrapping_add(296i32);
            let var30 = self.memory.load64(var5 as usize) as i64;
            var12 = var30;
            let var31 = self.memory.load64(var4 as usize + 288) as i64;
            var13 = var31;
            self.func46(imports, var4.wrapping_add(280i32), var2);
            let var32 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var32 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var33 = self.memory.load64(var5 as usize) as i64;
            var1 = var33;
            let var34 = self.memory.load64(var4 as usize + 288) as i64;
            var9 = var34;
            self.func46(imports, var4.wrapping_add(280i32), var3);
            let var35 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var35 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var36 = self.memory.load64(var4.wrapping_add(296i32) as usize) as i64;
            var10 = var36;
            let var37 = self.memory.load64(var4 as usize + 288) as i64;
            self.func53(imports, var4.wrapping_add(280i32));
            let var38 = self.memory.load64(var4 as usize + 280) as i64;
            if (var38 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var39 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(288i32), 56i32);
            var39;
            self.memory.store64(var4 as usize + 264, var12 as u64);
            self.memory.store64(var4 as usize + 256, var13 as u64);
            self.memory.store64(var4 as usize + 248, 1i64 as u64);
            self.func47(imports, var4.wrapping_add(280i32), var4.wrapping_add(248i32));
            let var40 = self.memory.load8(var4 as usize + 441) as i32;
            if (var40 == 2i32) as i32 != 0 {
                self.func81(imports, 858993459203i64);
                break 'label0;
            }
            let var41 = self.func83(imports, var4.wrapping_add(80i32), var4.wrapping_add(280i32), 168i32);
            var41;
            if (var1 | var9 == 0) as i32 != 0 {
                self.func81(imports, 8589934595i64);
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    let var42 = self.memory.load64(var4 as usize + 80) as i64;
                    var25 = var42;
                    let var43 = self.memory.load64(var4 as usize + 96) as i64;
                    var15 = var43;
                    let var44 = self.memory.load64(var4.wrapping_add(88i32) as usize) as i64;
                    var26 = var44;
                    let var45 = self.memory.load64(var4.wrapping_add(104i32) as usize) as i64;
                    var8 = var45;
                    if ((var25 ^ var15 | var26 ^ var8 == 0) as i32 == 0) as i32 != 0 {
                        let var46 = self.func65(imports, var4.wrapping_add(80i32));
                        if var46 != 0 {
                            break 'label2;
                        }
                        let var47 = imports._0(self, var0);
                        var47;
                        let var48 = self.memory.load32(var4 as usize + 72) as i32;
                        let var49 = self.memory.load32(var4 as usize + 76) as i32;
                        self.func77(imports, var4.wrapping_add(280i32), var9, var1, var4.wrapping_add(80i32), var48, var49);
                        let var50 = self.memory.load64(var4.wrapping_add(352i32) as usize) as i64;
                        var20 = var50;
                        let var51 = self.memory.load64(var4.wrapping_add(336i32) as usize) as i64;
                        var21 = var51;
                        let var52 = self.memory.load64(var4.wrapping_add(320i32) as usize) as i64;
                        var2 = var52;
                        let var53 = self.memory.load64(var4.wrapping_add(304i32) as usize) as i64;
                        var18 = var53;
                        let var54 = self.memory.load64(var4.wrapping_add(288i32) as usize) as i64;
                        var3 = var54;
                        let var55 = self.memory.load64(var4 as usize + 344) as i64;
                        var22 = var55;
                        let var56 = self.memory.load64(var4 as usize + 328) as i64;
                        var23 = var56;
                        let var57 = self.memory.load64(var4 as usize + 312) as i64;
                        var17 = var57;
                        let var58 = self.memory.load64(var4 as usize + 296) as i64;
                        var24 = var58;
                        let var59 = self.memory.load64(var4 as usize + 280) as i64;
                        var16 = var59;
                        let var60 = imports._7(self);
                        var19 = var60;
                        let var61 = self.memory.load64(var4 as usize + 56) as i64;
                        var11 = var61;
                        self.func74(imports, var11, var0, var19, var9, var1);
                        if (({ let a = (var9 as u64 > var16 as u64) as i32; let b = (var1 > var3) as i32; if (var1 == var3) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                            break 'label1;
                        }
                        let var62 = imports._7(self);
                        var19 = var62;
                        var27 = var1.wrapping_sub(var3).wrapping_sub(((var9 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((var1 ^ var3) & (var1 ^ var27) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.func74(imports, var11, var19, var0, var9.wrapping_sub(var16), var27);
                        break 'label1;
                    }
                    self.func81(imports, 880468295683i64);
                    break 'label0;
                    break;
                }
                self.func81(imports, 867583393795i64);
                break 'label0;
                break;
            }
            let var63 = imports._7(self);
            let var64 = self.memory.load64(var4 as usize + 40) as i64;
            self.func74(imports, var11, var63, var64, var22, var20);
            if (({ let a = (var37 as u64 > var17 as u64) as i32; let b = (var2 < var10) as i32; if (var2 == var10) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                self.memory.store64(var4 as usize + 304, var0 as u64);
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, 2i64 as u64);
                self.func43(imports, var4, var4.wrapping_add(280i32));
                let var65 = self.memory.load64(var4.wrapping_add(16i32) as usize) as i64;
                let var66 = self.memory.load32(var4 as usize) as i32;
                var5 = var66;
                var10 = { let a = var65; let b = 0i64; if var5 != 0 { a } else { b } };
                let var67 = self.memory.load64(var4 as usize + 8) as i64;
                var9 = { let a = var67; let b = 0i64; if var5 != 0 { a } else { b } };
                var1 = var9.wrapping_add(var17);
                var9 = (((var1 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_add(var10));
                if ((var10 ^ var2 ^ -1i64) & (var10 ^ var9) < 0i64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var4 as usize + 304, var0 as u64);
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, 2i64 as u64);
                self.func51(imports, var4.wrapping_add(280i32), var1, var9);
                var10 = var15.wrapping_add(var17);
                var15 = (((var10 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_add(var8));
                if ((var2 ^ var8 ^ -1i64) & (var8 ^ var15) < 0i64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var4.wrapping_add(104i32) as usize, var15 as u64);
                self.memory.store64(var4 as usize + 96, var10 as u64);
                var5 = var4.wrapping_add(120i32);
                let var68 = self.memory.load64(var5 as usize) as i64;
                var8 = var68;
                let var69 = self.memory.load64(var4 as usize + 112) as i64;
                var11 = var69;
                var14 = var11.wrapping_add(var24);
                var11 = (((var14 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_add(var18));
                if ((var8 ^ var18 ^ -1i64) & (var8 ^ var11) < 0i64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var5 as usize, var11 as u64);
                self.memory.store64(var4 as usize + 112, var14 as u64);
                var5 = var4.wrapping_add(136i32);
                let var70 = self.memory.load64(var5 as usize) as i64;
                var8 = var70;
                let var71 = self.memory.load64(var4 as usize + 128) as i64;
                var11 = var71;
                var14 = var11.wrapping_add(var16);
                var11 = (((var14 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var3.wrapping_add(var8));
                if ((var8 ^ var3 ^ -1i64) & (var8 ^ var11) < 0i64) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var5 as usize, var11 as u64);
                self.memory.store64(var4 as usize + 128, var14 as u64);
                'label3: loop {
                    if ((var10 ^ var25 | var15 ^ var26 == 0) as i32 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    let var72 = self.memory.load8(var4 as usize + 240) as i32;
                    if var72 != 0 {
                        break 'label3;
                    }
                    self.memory.store8(var4 as usize + 240, 1i32 as u8);
                    let var73 = self.func66(imports);
                    var8 = var73;
                    let var74 = self.memory.load64(var4 as usize + 32) as i64;
                    var10 = var8.wrapping_add(var74);
                    if ((var10 as u64) < var8 as u64) as i32 != 0 {
                        break 'label0;
                    }
                    self.memory.store64(var4 as usize + 224, var10 as u64);
                    break;
                }
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, 1i64 as u64);
                var5 = var4.wrapping_add(280i32);
                var6 = var4.wrapping_add(80i32);
                self.func49(imports, var5, var6);
                let var75 = self.memory.load64(var4 as usize + 64) as i64;
                let var76 = self.func72(imports, var75, var23, var21);
                self.memory.store64(var4 as usize + 64, var76 as u64);
                self.func55(imports, var4.wrapping_add(24i32));
                let var77 = self.memory.load64(var4 as usize + 232) as i64;
                var8 = var77;
                let var78 = self.func83(imports, var4.wrapping_add(384i32), var6, 168i32);
                var6 = var78;
                let var79 = self.func75(imports, 1048630i32, 3i32);
                var10 = var79;
                self.memory.store64(var4.wrapping_add(376i32) as usize, var9 as u64);
                self.memory.store64(var4.wrapping_add(368i32) as usize, var1 as u64);
                self.memory.store64(var4.wrapping_add(352i32) as usize, var20 as u64);
                self.memory.store64(var4.wrapping_add(336i32) as usize, var21 as u64);
                self.memory.store64(var4.wrapping_add(320i32) as usize, var2 as u64);
                self.memory.store64(var4.wrapping_add(304i32) as usize, var18 as u64);
                self.memory.store64(var4.wrapping_add(272i32) as usize, var8 as u64);
                self.memory.store64(var4.wrapping_add(264i32) as usize, var12 as u64);
                self.memory.store64(var4 as usize + 344, var22 as u64);
                self.memory.store64(var4 as usize + 328, var23 as u64);
                self.memory.store64(var4 as usize + 312, var17 as u64);
                self.memory.store64(var4 as usize + 296, var24 as u64);
                self.memory.store64(var4 as usize + 288, var3 as u64);
                self.memory.store64(var4 as usize + 280, var16 as u64);
                self.memory.store64(var4 as usize + 256, var13 as u64);
                self.memory.store64(var4 as usize + 248, var10 as u64);
                self.memory.store64(var4 as usize + 360, var0 as u64);
                let var80 = self.func60(imports, var4.wrapping_add(248i32));
                let var81 = self.func52(imports, var1, var9);
                self.memory.store64(var4 as usize + 560, var81 as u64);
                self.memory.store64(var4 as usize + 552, var0 as u64);
                var7 = var4.wrapping_add(552i32);
                let var82 = self.func59(imports, var7, 2i32);
                var0 = var82;
                let var83 = self.func58(imports, var5);
                var1 = var83;
                let var84 = self.func50(imports, var6);
                self.memory.store64(var4 as usize + 568, var84 as u64);
                self.memory.store64(var4 as usize + 560, var1 as u64);
                self.memory.store64(var4 as usize + 552, var0 as u64);
                let var85 = self.func59(imports, var7, 3i32);
                let var86 = imports._1(self, var80, var85);
                var86;
                self.global0 = var4.wrapping_add(576i32);
                return 2i64;
            }
            self.func81(imports, 871878361091i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func91<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64, mut var2: i64, mut var3: i64) -> i64 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let mut var13: i64 = 0;
        let mut var14: i64 = 0;
        let mut var15: i64 = 0;
        let mut var16: i64 = 0;
        let mut var17: i64 = 0;
        let mut var18: i64 = 0;
        let mut var19: i64 = 0;
        let mut var20: i64 = 0;
        let mut var21: i64 = 0;
        let mut var22: i64 = 0;
        let var23 = self.global0;
        var4 = var23.wrapping_sub(576i32);
        self.global0 = var4;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var4.wrapping_add(280i32), var1);
            let var24 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var24 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var5 = var4.wrapping_add(296i32);
            let var25 = self.memory.load64(var5 as usize) as i64;
            var1 = var25;
            let var26 = self.memory.load64(var4 as usize + 288) as i64;
            var14 = var26;
            self.func46(imports, var4.wrapping_add(280i32), var2);
            let var27 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var27 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var28 = self.memory.load64(var5 as usize) as i64;
            var2 = var28;
            let var29 = self.memory.load64(var4 as usize + 288) as i64;
            var10 = var29;
            self.func46(imports, var4.wrapping_add(280i32), var3);
            let var30 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var30 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var31 = self.memory.load64(var4.wrapping_add(296i32) as usize) as i64;
            var13 = var31;
            let var32 = self.memory.load64(var4 as usize + 288) as i64;
            var9 = var32;
            self.func53(imports, var4.wrapping_add(280i32));
            let var33 = self.memory.load64(var4 as usize + 280) as i64;
            if (var33 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var34 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(288i32), 56i32);
            var34;
            self.memory.store64(var4 as usize + 264, var1 as u64);
            self.memory.store64(var4 as usize + 256, var14 as u64);
            self.memory.store64(var4 as usize + 248, 1i64 as u64);
            self.func47(imports, var4.wrapping_add(280i32), var4.wrapping_add(248i32));
            let var35 = self.memory.load8(var4 as usize + 441) as i32;
            if (var35 == 2i32) as i32 != 0 {
                self.func81(imports, 858993459203i64);
                break 'label0;
            }
            var5 = var4.wrapping_add(80i32);
            let var36 = self.func83(imports, var5, var4.wrapping_add(280i32), 168i32);
            var36;
            'label1: loop {
                'label2: loop {
                    let var37 = self.func65(imports, var5);
                    if (var37 == 0) as i32 != 0 {
                        if (var2 | var10 == 0) as i32 != 0 {
                            self.func81(imports, 8589934595i64);
                            break 'label0;
                        }
                        let var38 = imports._0(self, var0);
                        var38;
                        self.memory.store64(var4 as usize + 304, var0 as u64);
                        self.memory.store64(var4 as usize + 296, var1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, 2i64 as u64);
                        self.func43(imports, var4, var4.wrapping_add(280i32));
                        let var39 = self.memory.load64(var4 as usize + 8) as i64;
                        let var40 = self.memory.load32(var4 as usize) as i32;
                        var5 = var40;
                        var8 = { let a = var39; let b = 0i64; if var5 != 0 { a } else { b } };
                        let var41 = self.memory.load64(var4.wrapping_add(16i32) as usize) as i64;
                        var11 = { let a = var41; let b = 0i64; if var5 != 0 { a } else { b } };
                        if ({ let a = ((var8 as u64) < var10 as u64) as i32; let b = (var11 < var2) as i32; if (var2 == var11) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label2;
                        }
                        let var42 = self.memory.load32(var4 as usize + 72) as i32;
                        let var43 = self.memory.load32(var4 as usize + 76) as i32;
                        self.func79(imports, var4.wrapping_add(280i32), var10, var2, var4.wrapping_add(80i32), var42, var43);
                        let var44 = self.memory.load64(var4.wrapping_add(336i32) as usize) as i64;
                        var18 = var44;
                        let var45 = self.memory.load64(var4.wrapping_add(304i32) as usize) as i64;
                        var2 = var45;
                        let var46 = self.memory.load64(var4.wrapping_add(288i32) as usize) as i64;
                        var3 = var46;
                        let var47 = self.memory.load64(var4.wrapping_add(320i32) as usize) as i64;
                        var15 = var47;
                        let var48 = self.memory.load64(var4.wrapping_add(352i32) as usize) as i64;
                        var19 = var48;
                        let var49 = self.memory.load64(var4 as usize + 328) as i64;
                        var20 = var49;
                        let var50 = self.memory.load64(var4 as usize + 296) as i64;
                        var16 = var50;
                        let var51 = self.memory.load64(var4 as usize + 280) as i64;
                        var10 = var51;
                        let var52 = self.memory.load64(var4 as usize + 312) as i64;
                        var17 = var52;
                        let var53 = self.memory.load64(var4 as usize + 344) as i64;
                        var21 = var53;
                        let var54 = imports._7(self);
                        var12 = var54;
                        let var55 = self.memory.load64(var4 as usize + 56) as i64;
                        var22 = var55;
                        let var56 = self.memory.load64(var4 as usize + 40) as i64;
                        self.func74(imports, var22, var12, var56, var21, var19);
                        if ({ let a = (var9 as u64 > var17 as u64) as i32; let b = (var13 > var15) as i32; if (var13 == var15) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label1;
                        }
                        let var57 = imports._7(self);
                        self.func74(imports, var22, var57, var0, var17, var15);
                        var13 = var11.wrapping_sub(var3).wrapping_sub(((var8 as u64) < var10 as u64) as i32 as u32 as i64);
                        if ((var3 ^ var11) & (var11 ^ var13) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var4 as usize + 304, var0 as u64);
                        self.memory.store64(var4 as usize + 296, var1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, 2i64 as u64);
                        var11 = var8.wrapping_sub(var10);
                        self.func51(imports, var4.wrapping_add(280i32), var11, var13);
                        var5 = var4.wrapping_add(104i32);
                        let var58 = self.memory.load64(var5 as usize) as i64;
                        var8 = var58;
                        let var59 = self.memory.load64(var4 as usize + 96) as i64;
                        var9 = var59;
                        var12 = var8.wrapping_sub(var3).wrapping_sub(((var9 as u64) < var10 as u64) as i32 as u32 as i64);
                        if ((var8 ^ var3) & (var8 ^ var12) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var12 as u64);
                        self.memory.store64(var4 as usize + 96, var9.wrapping_sub(var10) as u64);
                        var5 = var4.wrapping_add(120i32);
                        let var60 = self.memory.load64(var5 as usize) as i64;
                        var8 = var60;
                        let var61 = self.memory.load64(var4 as usize + 112) as i64;
                        var9 = var61;
                        var12 = var8.wrapping_sub(var2).wrapping_sub(((var9 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((var8 ^ var2) & (var8 ^ var12) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var12 as u64);
                        self.memory.store64(var4 as usize + 112, var9.wrapping_sub(var16) as u64);
                        var5 = var4.wrapping_add(136i32);
                        let var62 = self.memory.load64(var5 as usize) as i64;
                        var8 = var62;
                        let var63 = self.memory.load64(var4 as usize + 128) as i64;
                        var9 = var63;
                        var12 = var9.wrapping_add(var16);
                        var9 = (((var12 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(var2.wrapping_add(var8));
                        if ((var8 ^ var2 ^ -1i64) & (var8 ^ var9) < 0i64) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var9 as u64);
                        self.memory.store64(var4 as usize + 128, var12 as u64);
                        self.memory.store64(var4 as usize + 296, var1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, 1i64 as u64);
                        var5 = var4.wrapping_add(280i32);
                        var6 = var4.wrapping_add(80i32);
                        self.func49(imports, var5, var6);
                        let var64 = self.memory.load64(var4 as usize + 64) as i64;
                        let var65 = self.func72(imports, var64, var20, var18);
                        self.memory.store64(var4 as usize + 64, var65 as u64);
                        self.func55(imports, var4.wrapping_add(24i32));
                        let var66 = self.memory.load64(var4 as usize + 232) as i64;
                        var8 = var66;
                        let var67 = self.func83(imports, var4.wrapping_add(384i32), var6, 168i32);
                        var6 = var67;
                        let var68 = self.func75(imports, 1048633i32, 4i32);
                        var9 = var68;
                        self.memory.store64(var4.wrapping_add(376i32) as usize, var13 as u64);
                        self.memory.store64(var4.wrapping_add(368i32) as usize, var11 as u64);
                        self.memory.store64(var4.wrapping_add(352i32) as usize, var19 as u64);
                        self.memory.store64(var4.wrapping_add(336i32) as usize, var18 as u64);
                        self.memory.store64(var4.wrapping_add(320i32) as usize, var15 as u64);
                        self.memory.store64(var4.wrapping_add(304i32) as usize, var2 as u64);
                        self.memory.store64(var4.wrapping_add(272i32) as usize, var8 as u64);
                        self.memory.store64(var4.wrapping_add(264i32) as usize, var1 as u64);
                        self.memory.store64(var4 as usize + 344, var21 as u64);
                        self.memory.store64(var4 as usize + 328, var20 as u64);
                        self.memory.store64(var4 as usize + 312, var17 as u64);
                        self.memory.store64(var4 as usize + 296, var16 as u64);
                        self.memory.store64(var4 as usize + 288, var3 as u64);
                        self.memory.store64(var4 as usize + 280, var10 as u64);
                        self.memory.store64(var4 as usize + 256, var14 as u64);
                        self.memory.store64(var4 as usize + 248, var9 as u64);
                        self.memory.store64(var4 as usize + 360, var0 as u64);
                        let var69 = self.func60(imports, var4.wrapping_add(248i32));
                        let var70 = self.func52(imports, var11, var13);
                        self.memory.store64(var4 as usize + 560, var70 as u64);
                        self.memory.store64(var4 as usize + 552, var0 as u64);
                        var7 = var4.wrapping_add(552i32);
                        let var71 = self.func59(imports, var7, 2i32);
                        var0 = var71;
                        let var72 = self.func58(imports, var5);
                        var2 = var72;
                        let var73 = self.func50(imports, var6);
                        self.memory.store64(var4 as usize + 568, var73 as u64);
                        self.memory.store64(var4 as usize + 560, var2 as u64);
                        self.memory.store64(var4 as usize + 552, var0 as u64);
                        let var74 = self.func59(imports, var7, 3i32);
                        let var75 = imports._1(self, var69, var74);
                        var75;
                        self.global0 = var4.wrapping_add(576i32);
                        return 2i64;
                    }
                    self.func81(imports, 867583393795i64);
                    break 'label0;
                    break;
                }
                self.func81(imports, 876173328387i64);
                break 'label0;
                break;
            }
            self.func81(imports, 871878361091i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func92<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(368i32);
        self.global0 = var1;
        self.func57(imports, var1.wrapping_add(168i32), var0);
        'label0: loop {
            let var7 = self.memory.load64(var1 as usize + 168) as i64;
            if ((var7 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var8 = self.memory.load64(var1.wrapping_add(184i32) as usize) as i64;
            var0 = var8;
            let var9 = self.memory.load64(var1 as usize + 176) as i64;
            var3 = var9;
            self.func53(imports, var1.wrapping_add(168i32));
            let var10 = self.memory.load64(var1 as usize + 168) as i64;
            if (var10 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var11 = self.memory.load64(var1.wrapping_add(208i32) as usize) as i64;
            var4 = var11;
            self.memory.store64(var1 as usize + 352, var0 as u64);
            self.memory.store64(var1 as usize + 344, var3 as u64);
            self.memory.store64(var1 as usize + 336, 1i64 as u64);
            self.func47(imports, var1.wrapping_add(168i32), var1.wrapping_add(336i32));
            let var12 = self.memory.load8(var1 as usize + 329) as i32;
            if (var12 == 2i32) as i32 != 0 {
                self.func81(imports, 858993459203i64);
                break 'label0;
            }
            let var13 = self.func83(imports, var1, var1.wrapping_add(168i32), 168i32);
            var1 = var13;
            let var14 = self.memory.load8(var1 as usize + 161) as i32;
            if (var14 == 0) as i32 != 0 {
                let var15 = self.func65(imports, var1);
                if (var15 == 0) as i32 != 0 {
                    self.func81(imports, 863288426499i64);
                    break 'label0;
                }
                let var16 = imports._7(self);
                let var17 = self.memory.load64(var1 as usize + 120) as i64;
                let var18 = self.memory.load64(var1 as usize + 32) as i64;
                let var19 = self.memory.load64(var1.wrapping_add(40i32) as usize) as i64;
                self.func74(imports, var4, var16, var17, var18, var19);
                self.memory.store8(var1 as usize + 161, 1i32 as u8);
                self.memory.store64(var1 as usize + 184, var0 as u64);
                self.memory.store64(var1 as usize + 176, var3 as u64);
                self.memory.store64(var1 as usize + 168, 1i64 as u64);
                var2 = var1.wrapping_add(168i32);
                self.func49(imports, var2, var1);
                let var20 = self.memory.load64(var1 as usize + 152) as i64;
                var4 = var20;
                let var21 = self.func75(imports, 1048597i32, 20i32);
                var5 = var21;
                self.memory.store64(var1.wrapping_add(192i32) as usize, var4 as u64);
                self.memory.store64(var1.wrapping_add(184i32) as usize, var0 as u64);
                self.memory.store64(var1 as usize + 176, var3 as u64);
                self.memory.store64(var1 as usize + 168, var5 as u64);
                let var22 = self.func60(imports, var2);
                let var23 = self.func50(imports, var1);
                let var24 = imports._1(self, var22, var23);
                var24;
                self.global0 = var1.wrapping_add(368i32);
                return 2i64;
            }
            self.func81(imports, 884763262979i64);
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func93<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        let mut var11: i64 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var2 = var13.wrapping_sub(448i32);
        self.global0 = var2;
        'label0: loop {
            if (var0 & 255i64 != 77i64) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var2.wrapping_add(232i32), var1);
            let var14 = self.memory.load64(var2 as usize + 232) as i64;
            if ((var14 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var15 = self.memory.load64(var2.wrapping_add(248i32) as usize) as i64;
            var1 = var15;
            let var16 = self.memory.load64(var2 as usize + 240) as i64;
            var7 = var16;
            self.func53(imports, var2.wrapping_add(232i32));
            let var17 = self.memory.load64(var2 as usize + 232) as i64;
            if (var17 == 0) as i32 != 0 {
                self.func81(imports, 3i64);
                break 'label0;
            }
            let var18 = self.memory.load64(var2.wrapping_add(264i32) as usize) as i64;
            var11 = var18;
            self.memory.store64(var2 as usize + 416, var1 as u64);
            self.memory.store64(var2 as usize + 408, var7 as u64);
            self.memory.store64(var2 as usize + 400, 1i64 as u64);
            self.func47(imports, var2.wrapping_add(232i32), var2.wrapping_add(400i32));
            let var19 = self.memory.load8(var2 as usize + 393) as i32;
            if (var19 == 2i32) as i32 != 0 {
                self.func81(imports, 858993459203i64);
                break 'label0;
            }
            var3 = var2.wrapping_sub(-64i32);
            let var20 = self.func83(imports, var3, var2.wrapping_add(232i32), 168i32);
            var20;
            let var21 = self.func65(imports, var3);
            if (var21 == 0) as i32 != 0 {
                self.func81(imports, 863288426499i64);
                break 'label0;
            }
            self.memory.store64(var2 as usize + 256, var0 as u64);
            self.memory.store64(var2 as usize + 248, var1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, 2i64 as u64);
            var4 = var2.wrapping_add(232i32);
            self.func43(imports, var2.wrapping_add(40i32), var4);
            let var22 = self.memory.load64(var2.wrapping_add(56i32) as usize) as i64;
            var9 = var22;
            let var23 = self.memory.load32(var2 as usize + 40) as i32;
            var3 = var23;
            let var24 = self.memory.load64(var2 as usize + 48) as i64;
            var10 = var24;
            let var25 = imports._7(self);
            var5 = var25;
            let var26 = self.memory.load64(var2 as usize + 200) as i64;
            var10 = { let a = var10; let b = 0i64; if var3 != 0 { a } else { b } };
            var9 = { let a = var9; let b = 0i64; if var3 != 0 { a } else { b } };
            self.func74(imports, var26, var5, var0, var10, var9);
            self.memory.store64(var2 as usize + 256, var0 as u64);
            self.memory.store64(var2 as usize + 248, var1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, 2i64 as u64);
            let var27 = self.func44(imports, var4);
            self.func89(imports, var27);
            var3 = var2.wrapping_add(136i32);
            let var28 = self.memory.load64(var3 as usize) as i64;
            var5 = var28;
            let var29 = self.memory.load64(var2 as usize + 128) as i64;
            var6 = var29;
            var8 = var6.wrapping_add(var10);
            var6 = (((var8 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_add(var9));
            if ((var5 ^ var9 ^ -1i64) & (var5 ^ var6) < 0i64) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(var3 as usize, var6 as u64);
            self.memory.store64(var2 as usize + 128, var8 as u64);
            self.memory.store64(var2 as usize + 248, var1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, 1i64 as u64);
            self.memory.store32(var2 as usize + 36, 0i32 as u32);
            self.func49(imports, var2.wrapping_add(232i32), var2.wrapping_sub(-64i32));
            let var30 = self.memory.load64(var2 as usize + 112) as i64;
            let var31 = self.memory.load64(var2.wrapping_add(120i32) as usize) as i64;
            self.func68(imports, var2.wrapping_add(16i32), var10, var9, var30, var31, var2.wrapping_add(36i32));
            let var32 = self.memory.load32(var2 as usize + 36) as i32;
            if var32 != 0 {
                break 'label0;
            }
            let var33 = self.memory.load64(var2 as usize + 64) as i64;
            var5 = var33;
            let var34 = self.memory.load64(var2.wrapping_add(72i32) as usize) as i64;
            var6 = var34;
            if (var5 | var6 == 0) as i32 != 0 {
                break 'label0;
            }
            let var35 = self.memory.load64(var2 as usize + 16) as i64;
            var8 = var35;
            let var36 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
            var12 = var36;
            if (var8 | var12 ^ -9223372036854775808i64 == 0) as i32 & (var5 & var6 == -1i64) as i32 != 0 {
                break 'label0;
            }
            self.func71(imports, var2, var8, var12, var5, var6);
            let var37 = self.memory.load64(var2 as usize) as i64;
            var6 = var37;
            let var38 = self.memory.load64(var2.wrapping_add(8i32) as usize) as i64;
            var5 = var38;
            if (({ let a = (var6 == 0) as i32; let b = (var5 < 0i64) as i32; if (var5 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var39 = self.func75(imports, 1049284i32, 4i32);
                var8 = var39;
                let var40 = self.func52(imports, var6, var5);
                self.memory.store64(var2 as usize + 408, var40 as u64);
                self.memory.store64(var2 as usize + 400, var0 as u64);
                var3 = 0i32;
                'label1: loop {
                    if (var3 == 16i32) as i32 != 0 {
                        var3 = 0i32;
                        'label2: loop {
                            if (var3 != 16i32) as i32 != 0 {
                                let var41 = self.memory.load64(var2.wrapping_add(400i32).wrapping_add(var3) as usize) as i64;
                                self.memory.store64(var2.wrapping_add(232i32).wrapping_add(var3) as usize, var41 as u64);
                                var3 = var3.wrapping_add(8i32);
                                continue 'label2;
                            }
                            break;
                        }
                        let var42 = self.func59(imports, var2.wrapping_add(232i32), 2i32);
                        self.func76(imports, var11, var8, var42);
                    } else {
                        self.memory.store64(var2.wrapping_add(232i32).wrapping_add(var3) as usize, 2i64 as u64);
                        var3 = var3.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
            }
            let var43 = self.memory.load64(var2 as usize + 216) as i64;
            var11 = var43;
            let var44 = self.func75(imports, 1048637i32, 12i32);
            var8 = var44;
            self.memory.store64(var2.wrapping_add(256i32) as usize, var11 as u64);
            self.memory.store64(var2.wrapping_add(248i32) as usize, var1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, var8 as u64);
            let var45 = self.func60(imports, var2.wrapping_add(232i32));
            let var46 = self.func52(imports, var10, var9);
            var7 = var46;
            let var47 = self.func52(imports, var6, var5);
            self.memory.store64(var2 as usize + 408, var47 as u64);
            self.memory.store64(var2 as usize + 400, var7 as u64);
            let var48 = self.func59(imports, var2.wrapping_add(400i32), 2i32);
            self.memory.store64(var2 as usize + 440, var48 as u64);
            self.memory.store64(var2 as usize + 432, var0 as u64);
            let var49 = self.func59(imports, var2.wrapping_add(432i32), 2i32);
            let var50 = imports._1(self, var45, var49);
            var50;
            self.global0 = var2.wrapping_add(448i32);
            return 2i64;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func94<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(368i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(168i32), var0);
        'label0: loop {
            'label1: loop {
                let var8 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var8 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                var3 = var2.wrapping_add(184i32);
                let var9 = self.memory.load64(var3 as usize) as i64;
                var0 = var9;
                let var10 = self.memory.load64(var2 as usize + 176) as i64;
                var5 = var10;
                self.func46(imports, var2.wrapping_add(168i32), var1);
                let var11 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var11 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var12 = self.memory.load64(var3 as usize) as i64;
                var1 = var12;
                let var13 = self.memory.load64(var2 as usize + 176) as i64;
                var6 = var13;
                self.func53(imports, var2.wrapping_add(168i32));
                let var14 = self.memory.load64(var2 as usize + 168) as i64;
                if (var14 == 0) as i32 != 0 {
                    self.func81(imports, 3i64);
                    break 'label1;
                }
                let var15 = self.memory.load32(var2.wrapping_add(228i32) as usize) as i32;
                var3 = var15;
                let var16 = self.memory.load32(var2.wrapping_add(224i32) as usize) as i32;
                var4 = var16;
                self.memory.store64(var2 as usize + 352, var0 as u64);
                self.memory.store64(var2 as usize + 344, var5 as u64);
                self.memory.store64(var2 as usize + 336, 1i64 as u64);
                self.func47(imports, var2.wrapping_add(168i32), var2.wrapping_add(336i32));
                let var17 = self.memory.load8(var2 as usize + 329) as i32;
                if (var17 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, 858993459203i64);
                break;
            }
            unreachable!();
            break;
        }
        let var18 = self.func83(imports, var2, var2.wrapping_add(168i32), 168i32);
        var2 = var18;
        self.func77(imports, var2.wrapping_add(168i32), var6, var1, var2, var4, var3);
        let var19 = self.func58(imports, var2.wrapping_add(168i32));
        self.global0 = var2.wrapping_add(368i32);
        var19
    }
    fn func95<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(368i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(168i32), var0);
        'label0: loop {
            'label1: loop {
                let var8 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var8 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                var3 = var2.wrapping_add(184i32);
                let var9 = self.memory.load64(var3 as usize) as i64;
                var0 = var9;
                let var10 = self.memory.load64(var2 as usize + 176) as i64;
                var5 = var10;
                self.func46(imports, var2.wrapping_add(168i32), var1);
                let var11 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var11 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var12 = self.memory.load64(var3 as usize) as i64;
                var1 = var12;
                let var13 = self.memory.load64(var2 as usize + 176) as i64;
                var6 = var13;
                self.func53(imports, var2.wrapping_add(168i32));
                let var14 = self.memory.load64(var2 as usize + 168) as i64;
                if (var14 == 0) as i32 != 0 {
                    self.func81(imports, 3i64);
                    break 'label1;
                }
                let var15 = self.memory.load32(var2.wrapping_add(228i32) as usize) as i32;
                var3 = var15;
                let var16 = self.memory.load32(var2.wrapping_add(224i32) as usize) as i32;
                var4 = var16;
                self.memory.store64(var2 as usize + 352, var0 as u64);
                self.memory.store64(var2 as usize + 344, var5 as u64);
                self.memory.store64(var2 as usize + 336, 1i64 as u64);
                self.func47(imports, var2.wrapping_add(168i32), var2.wrapping_add(336i32));
                let var17 = self.memory.load8(var2 as usize + 329) as i32;
                if (var17 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, 858993459203i64);
                break;
            }
            unreachable!();
            break;
        }
        let var18 = self.func83(imports, var2, var2.wrapping_add(168i32), 168i32);
        var2 = var18;
        self.func79(imports, var2.wrapping_add(168i32), var6, var1, var2, var4, var3);
        let var19 = self.func58(imports, var2.wrapping_add(168i32));
        self.global0 = var2.wrapping_add(368i32);
        var19
    }
    fn func96<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(368i32);
        self.global0 = var1;
        self.func57(imports, var1.wrapping_add(168i32), var0);
        'label0: loop {
            let var3 = self.memory.load64(var1 as usize + 168) as i64;
            if (var3 == 0) as i32 != 0 {
                let var4 = self.memory.load64(var1 as usize + 176) as i64;
                var0 = var4;
                let var5 = self.memory.load64(var1.wrapping_add(184i32) as usize) as i64;
                self.memory.store64(var1 as usize + 352, var5 as u64);
                self.memory.store64(var1 as usize + 344, var0 as u64);
                self.memory.store64(var1 as usize + 336, 1i64 as u64);
                self.func47(imports, var1.wrapping_add(168i32), var1.wrapping_add(336i32));
                let var6 = self.memory.load8(var1 as usize + 329) as i32;
                if (var6 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, 858993459203i64);
            }
            unreachable!();
            break;
        }
        let var7 = self.func83(imports, var1, var1.wrapping_add(168i32), 168i32);
        var1 = var7;
        let var8 = self.func50(imports, var1);
        self.global0 = var1.wrapping_add(368i32);
        var8
    }
    fn func97<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(128i32);
        self.global0 = var0;
        self.func53(imports, var0.wrapping_sub(-64i32));
        let var4 = self.memory.load64(var0 as usize + 64) as i64;
        if (var4 == 0) as i32 != 0 {
            self.func81(imports, 3i64);
            unreachable!();
        }
        var1 = var0.wrapping_add(8i32);
        let var5 = self.func83(imports, var1, var0.wrapping_add(72i32), 56i32);
        var5;
        let var6 = self.func56(imports, var1);
        self.global0 = var0.wrapping_add(128i32);
        var6
    }
    fn func98<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i64, mut var1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(32i32), var0);
        let var6 = self.memory.load64(var2 as usize + 32) as i64;
        if (((var6 == 0) as i32 == 0) as i32 | (var1 & 255i64 != 77i64) as i32 == 0) as i32 != 0 {
            let var7 = self.memory.load64(var2.wrapping_add(48i32) as usize) as i64;
            var0 = var7;
            let var8 = self.memory.load64(var2 as usize + 40) as i64;
            var4 = var8;
            self.memory.store64(var2 as usize + 56, var1 as u64);
            self.memory.store64(var2 as usize + 48, var0 as u64);
            self.memory.store64(var2 as usize + 40, var4 as u64);
            self.memory.store64(var2 as usize + 32, 2i64 as u64);
            self.func43(imports, var2.wrapping_add(8i32), var2.wrapping_add(32i32));
            let var9 = self.memory.load64(var2 as usize + 16) as i64;
            let var10 = self.memory.load32(var2 as usize + 8) as i32;
            var3 = var10;
            let var11 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
            let var12 = self.func52(imports, { let a = var9; let b = 0i64; if var3 != 0 { a } else { b } }, { let a = var11; let b = 0i64; if var3 != 0 { a } else { b } });
            self.global0 = var2.wrapping_sub(-64i32);
            return var12;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func99<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i64 = 0;
        let var2 = self.global0;
        var0 = var2.wrapping_sub(32i32);
        self.global0 = var0;
        self.memory.store64(var0 as usize + 24, 4i64 as u64);
        self.memory.store64(var0 as usize + 16, 4i64 as u64);
        self.memory.store64(var0 as usize + 8, 4294967300i64 as u64);
        let var3 = self.func59(imports, var0.wrapping_add(8i32), 3i32);
        self.global0 = var0.wrapping_add(32i32);
        var3
    }
    fn func100<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i64, mut var4: i64) {
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        var5 = var3 & 4294967295i64;
        var6 = var1 & 4294967295i64;
        var7 = var5.wrapping_mul(var6);
        var8 = (var3 as u64).wrapping_shr(32i64 as u32) as i64;
        var6 = var6.wrapping_mul(var8);
        var9 = (var1 as u64).wrapping_shr(32i64 as u32) as i64;
        var5 = var6.wrapping_add(var5.wrapping_mul(var9));
        var10 = var7.wrapping_add(var5.wrapping_shl(32i64 as u32));
        self.memory.store64(var0 as usize, var10 as u64);
        self.memory.store64(var0 as usize + 8, ((var7 as u64 > var10 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_mul(var9).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32i64 as u32) | (var5 as u64).wrapping_shr(32i64 as u32) as i64)).wrapping_add(var1.wrapping_mul(var4).wrapping_add(var2.wrapping_mul(var3))) as u64);
    }
    fn func101<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64, mut var2: i64, mut var3: i32) {
        let mut var4: i64 = 0;
        'label0: loop {
            if (var3 & 64i32 == 0) as i32 != 0 {
                if (var3 == 0) as i32 != 0 {
                    break 'label0;
                }
                var4 = (var3 & 63i32) as u32 as i64;
                var2 = var2.wrapping_shl(var4 as u32) | (var1 as u64).wrapping_shr((0i32.wrapping_sub(var3) & 63i32) as u32 as i64 as u32) as i64;
                var1 = var1.wrapping_shl(var4 as u32);
                break 'label0;
            }
            var2 = var1.wrapping_shl((var3 & 63i32) as u32 as i64 as u32);
            var1 = 0i64;
            break;
        }
        self.memory.store64(var0 as usize, var1 as u64);
        self.memory.store64(var0 as usize + 8, var2 as u64);
    }
    fn func102<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i64 {
        let var1 = imports._3(self, (var0 as u32 as i64).wrapping_shl(32i64 as u32) | 4i64, 68719476740i64);
        var1
    }
    fn func103<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    fn func104<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16i32);
        self.global0 = var2;
        let var6 = imports._8(self, var1);
        let var7: i32;
        if (var6 & -4294967296i64 == 68719476736i64) as i32 != 0 {
            self.memory.store64(var2 as usize + 8, 0i64 as u64);
            self.memory.store64(var2 as usize, 0i64 as u64);
            'label0: loop {
                'label1: loop {
                    let var8 = imports._8(self, var1);
                    if ((var8 as u64) < 4294967296i64 as u64) as i32 != 0 {
                        break 'label0;
                    }
                    let var9 = imports.b(self, var1);
                    var4 = var9;
                    let var10 = imports._8(self, var1);
                    let var11 = imports.f(self, var1, 4294967300i64, var10 & -4294967296i64 | 4i64);
                    var1 = var11;
                    if (var3 != 16i32) as i32 != 0 {
                        self.memory.store8(var2.wrapping_add(var3) as usize, (var4 as u64).wrapping_shr(32i64 as u32) as i64 as u8);
                        var3 = var3.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                unreachable!();
                break;
            }
            let var12 = self.memory.load64(var2 as usize) as i64;
            var1 = var12;
            let var13 = self.memory.load64(var2 as usize + 8) as i64;
            self.memory.store64(var0.wrapping_add(9i32) as usize, var13 as u64);
            self.memory.store64(var0 as usize + 1, var1 as u64);
            var7 = 0i32;
        } else {
            var7 = 1i32;
        }
        self.memory.store8(var0 as usize, var7 as u8);
        self.global0 = var2.wrapping_add(16i32);
    }
}
