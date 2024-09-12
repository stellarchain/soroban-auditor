#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

pub trait Imports {
    type Memory: Memory;
    fn obj_to_u64(&mut self, context: &mut Context<Self::Memory>, obj: U64Object) -> u64;
    fn get_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType) -> Val;
    fn put_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, v: Val, t: StorageType) -> Void;
    fn vec_unpack_to_linear_memory(&mut self, context: &mut Context<Self::Memory>, vec: VecObject, vals_pos: U32Val, len: U32Val) -> Void;
    fn obj_from_u64(&mut self, context: &mut Context<Self::Memory>, v: u64) -> U64Object;
    fn i256_mul(&mut self, context: &mut Context<Self::Memory>, lhs: I256Val, rhs: I256Val) -> I256Val;
    fn i256_div(&mut self, context: &mut Context<Self::Memory>, lhs: I256Val, rhs: I256Val) -> I256Val;
    fn i256_sub(&mut self, context: &mut Context<Self::Memory>, lhs: I256Val, rhs: I256Val) -> I256Val;
    fn map_len(&mut self, context: &mut Context<Self::Memory>, m: MapObject) -> U32Val;
    fn map_key_by_pos(&mut self, context: &mut Context<Self::Memory>, m: MapObject, i: U32Val) -> Val;
    fn map_val_by_pos(&mut self, context: &mut Context<Self::Memory>, m: MapObject, i: U32Val) -> Val;
    fn map_put(&mut self, context: &mut Context<Self::Memory>, m: MapObject, k: Val, v: Val) -> MapObject;
    fn i256_pow(&mut self, context: &mut Context<Self::Memory>, lhs: I256Val, rhs: U32Val) -> I256Val;
    fn i256_add(&mut self, context: &mut Context<Self::Memory>, lhs: I256Val, rhs: I256Val) -> I256Val;
    fn obj_cmp(&mut self, context: &mut Context<Self::Memory>, a: Val, b: Val) -> i64;
    fn map_new(&mut self, context: &mut Context<Self::Memory>) -> MapObject;
    fn contract_event(&mut self, context: &mut Context<Self::Memory>, topics: VecObject, data: Val) -> Void;
    fn require_auth(&mut self, context: &mut Context<Self::Memory>, address: AddressObject) -> Void;
    fn map_has(&mut self, context: &mut Context<Self::Memory>, m: MapObject, k: Val) -> Bool;
    fn map_get(&mut self, context: &mut Context<Self::Memory>, m: MapObject, k: Val) -> Val;
    fn bytes_len(&mut self, context: &mut Context<Self::Memory>, b: BytesObject) -> U32Val;
    fn update_current_contract_wasm(&mut self, context: &mut Context<Self::Memory>, hash: BytesObject) -> Void;
    fn prng_u64_in_inclusive_range(&mut self, context: &mut Context<Self::Memory>, lo: u64, hi: u64) -> u64;
    fn get_current_contract_address(&mut self, context: &mut Context<Self::Memory>) -> AddressObject;
    fn bytes_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, lm_pos: U32Val, len: U32Val) -> BytesObject;
    fn vec_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, vals_pos: U32Val, len: U32Val) -> VecObject;
    fn obj_to_i128_hi64(&mut self, context: &mut Context<Self::Memory>, obj: I128Object) -> i64;
    fn obj_to_i128_lo64(&mut self, context: &mut Context<Self::Memory>, obj: I128Object) -> u64;
    fn obj_from_i128_pieces(&mut self, context: &mut Context<Self::Memory>, hi: i64, lo: u64) -> I128Object;
    fn symbol_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, lm_pos: U32Val, len: U32Val) -> SymbolObject;
    fn call(&mut self, context: &mut Context<Self::Memory>, contract: AddressObject, func: Symbol, args: VecObject) -> Val;
    fn map_new_from_linear_memory(&mut self, context: &mut Context<Self::Memory>, keys_pos: U32Val, vals_pos: U32Val, len: U32Val) -> MapObject;
    fn map_unpack_to_linear_memory(&mut self, context: &mut Context<Self::Memory>, map: MapObject, keys_pos: U32Val, vals_pos: U32Val, len: U32Val) -> Void;
    fn bytes_front(&mut self, context: &mut Context<Self::Memory>, b: BytesObject) -> U32Val;
    fn bytes_slice(&mut self, context: &mut Context<Self::Memory>, b: BytesObject, start: U32Val, end: U32Val) -> BytesObject;
    fn get_ledger_timestamp(&mut self, context: &mut Context<Self::Memory>) -> U64Val;
    fn has_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType) -> Bool;
    fn bytes_append(&mut self, context: &mut Context<Self::Memory>, b1: BytesObject, b2: BytesObject) -> BytesObject;
    fn i256_val_from_be_bytes(&mut self, context: &mut Context<Self::Memory>, bytes: BytesObject) -> I256Val;
    fn i256_val_to_be_bytes(&mut self, context: &mut Context<Self::Memory>, val: I256Val) -> BytesObject;
    fn fail_with_error(&mut self, context: &mut Context<Self::Memory>, error: Error) -> Void;
    fn del_contract_data(&mut self, context: &mut Context<Self::Memory>, k: Val, t: StorageType) -> Void;
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
    pub const __data_end: i32 = 1049288;
    pub const __heap_base: i32 = 1049296;
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
        instance.context.memory.store_slice(1048576, b"initializednew_launchlaunch_funds_claimedcancel_launchbuysellclaim_tokensspace_missionContractInfoLaunchLaunchBalanceSpaceMissionadminnative_contractslz_feeslz_fee_destinationspace_feespace_missionsstability_check_durationstellarbucks_contract\0\x81\0\0\0\0\0\x86\0\0\0\0\0\x95\0\0\0\0\0\x9C\0\0\0\0\0\xAF\0\0\t\0\0\0\xB8\0\0\0\0\0\xC6\0\0\0\0\0\xDE\0\0\0\0\0assetdevfunds_claimedfunds_recipientinfomax_pricemax_supplymin_pricepool_balancestability_checkstability_check_endstellarbuckssupplytimestamptokens_claimed\04\0\0\0\09\0\0\0\0<\0\r\0\0\0I\0\0\0\0X\0\0\0\0\\\0\t\0\0\0e\0\n\0\0\0o\0\t\0\0\0x\0\0\0\0\x84\0\0\0\0\x93\0\0\0\0\xA6\0\0\0\0\xB2\0\0\0\0\xB8\0\t\0\0\0\xC1\0\0\0\0guaranteed_success_fundingrewardH\0\0\0\0b\0");
        instance.context.memory.store_slice(1049256, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFtransferburnmint");
        instance
    }
    pub fn initialize(&mut self, admin: soroban_sdk :: Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk :: Address, stellarbucks_contract: soroban_sdk :: Address, native_contract: soroban_sdk :: Address, space_missions_odds: soroban_sdk :: Map < u32 , u64 >) -> i64 {
        self.context.func80(&mut self.imports, admin, stability_check_duration, space_fee, slz_fee, slz_fee_destination, stellarbucks_contract, native_contract, space_missions_odds)
    }
    pub fn change_contract_info(&mut self, admin: soroban_sdk :: Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk :: Address, space_missions_odds: soroban_sdk :: Map < u32 , u64 >) -> i64 {
        self.context.func82(&mut self.imports, admin, stability_check_duration, space_fee, slz_fee, slz_fee_destination, space_missions_odds)
    }
    pub fn upgrade(&mut self, hash: soroban_sdk :: BytesN < 32 >) -> i64 {
        self.context.func84(&mut self.imports, hash)
    }
    pub fn start_space_mission(&mut self, user: soroban_sdk :: Address, funding: i128, difficulty: u32, min_mission_reward: i128) -> (bool , i128 ,) {
        self.context.func85(&mut self.imports, user, funding, difficulty, min_mission_reward)
    }
    pub fn add_space_missions_reward(&mut self, user: soroban_sdk :: Address, funds: i128, reward_difficulty: u32) -> i64 {
        self.context.func86(&mut self.imports, user, funds, reward_difficulty)
    }
    pub fn new_launch(&mut self, dev: soroban_sdk :: Address, funds_recipient: soroban_sdk :: Address, info: soroban_sdk :: String, asset: soroban_sdk :: Address, max_supply: i128, min_price: i128, max_price: i128, launch_index: u64) -> i64 {
        self.context.func87(&mut self.imports, dev, funds_recipient, info, asset, max_supply, min_price, max_price, launch_index)
    }
    pub fn cancel_launch(&mut self, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.context.func88(&mut self.imports, launch_key)
    }
    pub fn buy(&mut self, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128, min_receive: i128) -> i64 {
        self.context.func90(&mut self.imports, user, launch_key, sending, min_receive)
    }
    pub fn sell(&mut self, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128, min_receive: i128) -> i64 {
        self.context.func91(&mut self.imports, user, launch_key, sending, min_receive)
    }
    pub fn claim_launch_funds(&mut self, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.context.func92(&mut self.imports, launch_key)
    }
    pub fn claim_launch_balance(&mut self, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.context.func93(&mut self.imports, user, launch_key)
    }
    pub fn calculate_buy(&mut self, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128) -> (i128 , i128 , i128 , i128 , i128 ,) {
        self.context.func94(&mut self.imports, launch_key, sending)
    }
    pub fn calculate_sell(&mut self, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128) -> (i128 , i128 , i128 , i128 , i128 ,) {
        self.context.func95(&mut self.imports, launch_key, sending)
    }
    pub fn get_launch_data(&mut self, launch_key: (soroban_sdk :: Address , u64 ,)) -> Launch {
        self.context.func96(&mut self.imports, launch_key)
    }
    pub fn get_contract_info(&mut self) -> ContractInfo {
        self.context.func97(&mut self.imports)
    }
    pub fn get_launch_balance(&mut self, launch_key: (soroban_sdk :: Address , u64 ,), user: soroban_sdk :: Address) -> i128 {
        self.context.func98(&mut self.imports, launch_key, user)
    }
    pub fn version(&mut self) -> (u32 , u32 , u32 ,) {
        self.context.func99(&mut self.imports)
    }
    pub fn __(&mut self) {
        self.context.func103(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, admin: soroban_sdk :: Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk :: Address, stellarbucks_contract: soroban_sdk :: Address, native_contract: soroban_sdk :: Address, space_missions_odds: soroban_sdk :: Map < u32 , u64 >) -> i64 {
        self.func80(imports, admin, stability_check_duration, space_fee, slz_fee, slz_fee_destination, stellarbucks_contract, native_contract, space_missions_odds)
    }
    pub fn change_contract_info<I: Imports<Memory = M>>(&mut self, imports: &mut I, admin: soroban_sdk :: Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk :: Address, space_missions_odds: soroban_sdk :: Map < u32 , u64 >) -> i64 {
        self.func82(imports, admin, stability_check_duration, space_fee, slz_fee, slz_fee_destination, space_missions_odds)
    }
    pub fn upgrade<I: Imports<Memory = M>>(&mut self, imports: &mut I, hash: soroban_sdk :: BytesN < 32 >) -> i64 {
        self.func84(imports, hash)
    }
    pub fn start_space_mission<I: Imports<Memory = M>>(&mut self, imports: &mut I, user: soroban_sdk :: Address, funding: i128, difficulty: u32, min_mission_reward: i128) -> (bool , i128 ,) {
        self.func85(imports, user, funding, difficulty, min_mission_reward)
    }
    pub fn add_space_missions_reward<I: Imports<Memory = M>>(&mut self, imports: &mut I, user: soroban_sdk :: Address, funds: i128, reward_difficulty: u32) -> i64 {
        self.func86(imports, user, funds, reward_difficulty)
    }
    pub fn new_launch<I: Imports<Memory = M>>(&mut self, imports: &mut I, dev: soroban_sdk :: Address, funds_recipient: soroban_sdk :: Address, info: soroban_sdk :: String, asset: soroban_sdk :: Address, max_supply: i128, min_price: i128, max_price: i128, launch_index: u64) -> i64 {
        self.func87(imports, dev, funds_recipient, info, asset, max_supply, min_price, max_price, launch_index)
    }
    pub fn cancel_launch<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.func88(imports, launch_key)
    }
    pub fn buy<I: Imports<Memory = M>>(&mut self, imports: &mut I, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128, min_receive: i128) -> i64 {
        self.func90(imports, user, launch_key, sending, min_receive)
    }
    pub fn sell<I: Imports<Memory = M>>(&mut self, imports: &mut I, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128, min_receive: i128) -> i64 {
        self.func91(imports, user, launch_key, sending, min_receive)
    }
    pub fn claim_launch_funds<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.func92(imports, launch_key)
    }
    pub fn claim_launch_balance<I: Imports<Memory = M>>(&mut self, imports: &mut I, user: soroban_sdk :: Address, launch_key: (soroban_sdk :: Address , u64 ,)) -> i64 {
        self.func93(imports, user, launch_key)
    }
    pub fn calculate_buy<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128) -> (i128 , i128 , i128 , i128 , i128 ,) {
        self.func94(imports, launch_key, sending)
    }
    pub fn calculate_sell<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,), sending: i128) -> (i128 , i128 , i128 , i128 , i128 ,) {
        self.func95(imports, launch_key, sending)
    }
    pub fn get_launch_data<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,)) -> Launch {
        self.func96(imports, launch_key)
    }
    pub fn get_contract_info<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> ContractInfo {
        self.func97(imports)
    }
    pub fn get_launch_balance<I: Imports<Memory = M>>(&mut self, imports: &mut I, launch_key: (soroban_sdk :: Address , u64 ,), user: soroban_sdk :: Address) -> i128 {
        self.func98(imports, launch_key, user)
    }
    pub fn version<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> (u32 , u32 , u32 ,) {
        self.func99(imports)
    }
    pub fn __<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func103(imports)
    }
    fn func42<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        'label0: loop {
            var2 = arg1 as i32 & 255i32;
            if (var2 != 64i32) as i32 != 0 {
                if (var2 != 6i32) as i32 != 0 {
                    var3 = True;
                    var4 = Error(Value, UnexpectedType);
                    break 'label0;
                }
                var4 = (arg1 as u64).wrapping_shr(Timepoint(0) as u32) as i64;
                break 'label0;
            }
            let var5 = imports.obj_to_u64(self, arg1);
            var4 = var5;
            break;
        }
        self.memory.store64(arg0 as usize + 8, var4 as u64);
        self.memory.store64(arg0 as usize, var3 as u64);
    }
    fn func43<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var2 = var6.wrapping_sub(32i32);
        self.global0 = var2;
        'label0: loop {
            let var7 = self.func44(imports, arg1);
            var3 = var7;
            let var8 = self.func45(imports, var3, True);
            let var9: i64;
            if var8 != 0 {
                let var10 = imports.get_contract_data(self, var3, True);
                self.func46(imports, var2.wrapping_add(8i32), var10);
                let var12 = self.memory.load64(var2 as usize + 8) as i64;
                if ((var12 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var13 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
                var4 = var13;
                let var14 = self.memory.load64(var2 as usize + 16) as i64;
                var5 = var14;
                var9 = True;
            } else {
                var9 = False;
            }
            var3 = var9;
            self.memory.store64(arg0 as usize + 8, var5 as u64);
            self.memory.store64(arg0 as usize, var3 as u64);
            self.memory.store64(arg0.wrapping_add(16i32) as usize, var4 as u64);
            self.global0 = var2.wrapping_add(32i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func44<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
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
                            let var7 = self.memory.load32(arg0 as usize) as i32;
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
                        let var9 = self.memory.load64(arg0 as usize + 8) as i64;
                        var3 = var9;
                        let var10 = self.memory.load64(arg0 as usize + 16) as i64;
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
                    let var14 = self.memory.load64(arg0 as usize + 8) as i64;
                    var3 = var14;
                    let var15 = self.memory.load64(arg0 as usize + 16) as i64;
                    let var16 = self.func61(imports, var15);
                    var4 = var16;
                    let var17 = self.memory.load64(arg0 as usize + 24) as i64;
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
                let var20 = self.memory.load64(arg0 as usize + 8) as i64;
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
    fn func45<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i32 {
        let var2 = imports.has_contract_data(self, arg0, arg1);
        (var2 == True) as i32
    }
    fn func46<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4: i64;
        'label0: loop {
            'label1: loop {
                var2 = arg1 as i32 & 255i32;
                if (var2 != 69i32) as i32 != 0 {
                    if (var2 == 11i32) as i32 != 0 {
                        self.memory.store64(arg0.wrapping_add(16i32) as usize, arg1.wrapping_shr(63i64 as u32) as u64);
                        self.memory.store64(arg0 as usize + 8, arg1.wrapping_shr(Timepoint(0) as u32) as u64);
                        break 'label1;
                    }
                    self.memory.store64(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
                    var4 = True;
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
            var4 = False;
            break;
        }
        self.memory.store64(arg0 as usize, var4 as u64);
    }
    fn func47<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) {
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
                let var26 = self.func44(imports, arg1);
                var5 = var26;
                let var27 = self.func45(imports, var5, True);
                if (var27 == 0) as i32 != 0 {
                    self.memory.store8(arg0 as usize + 161, 2i32 as u8);
                    break 'label1;
                }
                let var28 = imports.get_contract_data(self, var5, True);
                var5 = var28;
                var1 = 0i32;
                'label2: loop {
                    if (arg1 != 120i32) as i32 != 0 {
                        self.memory.store64(var2.wrapping_add(32i32).wrapping_add(arg1) as usize, Void as u64);
                        var1 = arg1.wrapping_add(8i32);
                        continue 'label2;
                    }
                    break;
                }
                if (var5 & 255i64 != Map(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                self.func48(imports, var5, 1049040i32, 15i32, var2.wrapping_add(32i32), 15i32);
                let var30 = self.memory.load64(var2 as usize + 32) as i64;
                var5 = var30;
                if (var5 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var31 = self.memory.load64(var2 as usize + 40) as i64;
                var6 = var31;
                if (var6 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var32 = self.memory.load8(var2 as usize + 48) as i32;
                var1 = var32;
                var1 = { let a = 1i32; let b = ((var1 != 0i32) as i32).wrapping_shl(1i32 as u32); if (arg1 == 1i32) as i32 != 0 { a } else { b } };
                if (var1 == 2i32) as i32 != 0 {
                    break 'label0;
                }
                let var33 = self.memory.load64(var2 as usize + 56) as i64;
                var7 = var33;
                if (var7 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var34 = self.memory.load64(var2 as usize + 64) as i64;
                var8 = var34;
                if (var8 & 255i64 != String(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var35 = self.memory.load64(var2 as usize + 72) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var35);
                let var37 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var37 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var38 = self.memory.load64(var3 as usize) as i64;
                var9 = var38;
                let var39 = self.memory.load64(var2 as usize + 160) as i64;
                var10 = var39;
                let var40 = self.memory.load64(var2 as usize + 80) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var40);
                let var42 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var42 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var43 = self.memory.load64(var3 as usize) as i64;
                var11 = var43;
                let var44 = self.memory.load64(var2 as usize + 160) as i64;
                var12 = var44;
                let var45 = self.memory.load64(var2 as usize + 88) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var45);
                let var47 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var47 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var48 = self.memory.load64(var3 as usize) as i64;
                var13 = var48;
                let var49 = self.memory.load64(var2 as usize + 160) as i64;
                var14 = var49;
                let var50 = self.memory.load64(var2 as usize + 96) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var50);
                let var52 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var52 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var53 = self.memory.load8(var2 as usize + 104) as i32;
                var4 = var53;
                var4 = { let a = 1i32; let b = ((var4 != 0i32) as i32).wrapping_shl(1i32 as u32); if (var4 == 1i32) as i32 != 0 { a } else { b } };
                if (var4 == 2i32) as i32 != 0 {
                    break 'label0;
                }
                let var54 = self.memory.load64(var3 as usize) as i64;
                var15 = var54;
                let var55 = self.memory.load64(var2 as usize + 160) as i64;
                var16 = var55;
                let var56 = self.memory.load64(var2 as usize + 112) as i64;
                self.func42(imports, var2.wrapping_add(16i32), var56);
                let var58 = self.memory.load32(var2 as usize + 16) as i32;
                if var58 != 0 {
                    break 'label0;
                }
                let var59 = self.memory.load64(var2 as usize + 24) as i64;
                var17 = var59;
                let var60 = self.memory.load64(var2 as usize + 120) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var60);
                let var62 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var62 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                var3 = var2.wrapping_add(168i32);
                let var63 = self.memory.load64(var3 as usize) as i64;
                var18 = var63;
                let var64 = self.memory.load64(var2 as usize + 160) as i64;
                var19 = var64;
                let var65 = self.memory.load64(var2 as usize + 128) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var65);
                let var67 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var67 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var68 = self.memory.load64(var3 as usize) as i64;
                var20 = var68;
                let var69 = self.memory.load64(var2 as usize + 160) as i64;
                var21 = var69;
                let var70 = self.memory.load64(var2 as usize + 136) as i64;
                self.func42(imports, var2, var70);
                let var72 = self.memory.load32(var2 as usize) as i32;
                if var72 != 0 {
                    break 'label0;
                }
                let var73 = self.memory.load64(var2 as usize + 8) as i64;
                var22 = var73;
                let var74 = self.memory.load64(var2 as usize + 144) as i64;
                self.func46(imports, var2.wrapping_add(152i32), var74);
                let var76 = self.memory.load64(var2 as usize + 152) as i64;
                if ((var76 == 0) as i32 == 0) as i32 != 0 {
                    break 'label0;
                }
                let var77 = self.memory.load64(var2.wrapping_add(168i32) as usize) as i64;
                var23 = var77;
                let var78 = self.memory.load64(var2 as usize + 160) as i64;
                var24 = var78;
                self.memory.store64(arg0 as usize + 96, var10 as u64);
                self.memory.store64(arg0 as usize + 80, var14 as u64);
                self.memory.store64(arg0 as usize + 64, var24 as u64);
                self.memory.store64(arg0 as usize + 48, var19 as u64);
                self.memory.store64(arg0 as usize + 32, var16 as u64);
                self.memory.store64(arg0 as usize + 16, var21 as u64);
                self.memory.store64(arg0 as usize, var12 as u64);
                self.memory.store8(arg0 as usize + 161, (arg1 & 1i32) as u8);
                self.memory.store8(arg0 as usize + 160, (var4 & 1i32) as u8);
                self.memory.store64(arg0 as usize + 152, var22 as u64);
                self.memory.store64(arg0 as usize + 144, var17 as u64);
                self.memory.store64(arg0 as usize + 136, var5 as u64);
                self.memory.store64(arg0 as usize + 128, var8 as u64);
                self.memory.store64(arg0 as usize + 120, var7 as u64);
                self.memory.store64(arg0 as usize + 112, var6 as u64);
                self.memory.store64(arg0.wrapping_add(104i32) as usize, var9 as u64);
                self.memory.store64(arg0.wrapping_add(88i32) as usize, var13 as u64);
                self.memory.store64(arg0.wrapping_add(72i32) as usize, var23 as u64);
                self.memory.store64(arg0.wrapping_add(56i32) as usize, var18 as u64);
                self.memory.store64(arg0.wrapping_add(40i32) as usize, var15 as u64);
                self.memory.store64(arg0.wrapping_add(24i32) as usize, var20 as u64);
                self.memory.store64(arg0 as usize + 8, var11 as u64);
                break;
            }
            self.global0 = var2.wrapping_add(176i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func48<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i32, mut arg2: i32, mut arg3: i32, mut arg4: i32) {
        if (arg2 != arg4) as i32 != 0 {
            unreachable!();
        }
        let var5 = imports.map_unpack_to_linear_memory(self, arg0, (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg3 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg2 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0));
        var5;
    }
    fn func49<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) {
        let var2 = self.func44(imports, arg0);
        let var3 = self.func50(imports, arg1);
        let var4 = imports.put_contract_data(self, var2, var3, True);
        var4;
    }
    fn func50<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
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
        let var17 = self.memory.load64(arg0 as usize + 136) as i64;
        var2 = var17;
        let var18 = self.memory.load64(arg0 as usize + 112) as i64;
        var3 = var18;
        let var19 = self.memory.load8(arg0 as usize + 161) as i64;
        var4 = var19;
        let var20 = self.memory.load64(arg0 as usize + 120) as i64;
        var5 = var20;
        let var21 = self.memory.load64(arg0 as usize + 128) as i64;
        var6 = var21;
        let var22 = self.memory.load64(arg0 as usize + 96) as i64;
        let var23 = self.memory.load64(arg0.wrapping_add(104i32) as usize) as i64;
        let var24 = self.func52(imports, var22, var23);
        var7 = var24;
        let var25 = self.memory.load64(arg0 as usize) as i64;
        let var26 = self.memory.load64(arg0.wrapping_add(8i32) as usize) as i64;
        let var27 = self.func52(imports, var25, var26);
        var8 = var27;
        let var28 = self.memory.load64(arg0 as usize + 80) as i64;
        let var29 = self.memory.load64(arg0.wrapping_add(88i32) as usize) as i64;
        let var30 = self.func52(imports, var28, var29);
        var9 = var30;
        let var31 = self.memory.load64(arg0 as usize + 32) as i64;
        let var32 = self.memory.load64(arg0.wrapping_add(40i32) as usize) as i64;
        let var33 = self.func52(imports, var31, var32);
        var10 = var33;
        let var34 = self.memory.load8(arg0 as usize + 160) as i64;
        var11 = var34;
        let var35 = self.memory.load64(arg0 as usize + 144) as i64;
        let var36 = self.func61(imports, var35);
        var12 = var36;
        let var37 = self.memory.load64(arg0 as usize + 48) as i64;
        let var38 = self.memory.load64(arg0.wrapping_add(56i32) as usize) as i64;
        let var39 = self.func52(imports, var37, var38);
        var13 = var39;
        let var40 = self.memory.load64(arg0 as usize + 16) as i64;
        let var41 = self.memory.load64(arg0.wrapping_add(24i32) as usize) as i64;
        let var42 = self.func52(imports, var40, var41);
        var14 = var42;
        let var43 = self.memory.load64(arg0 as usize + 152) as i64;
        let var44 = self.func61(imports, var43);
        var15 = var44;
        let var45 = self.memory.load64(arg0 as usize + 64) as i64;
        let var46 = self.memory.load64(arg0.wrapping_add(72i32) as usize) as i64;
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
    fn func51<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let var3 = self.func44(imports, arg0);
        let var4 = self.func52(imports, arg1, arg2);
        let var5 = imports.put_contract_data(self, var3, var4, True);
        var5;
    }
    fn func52<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        if ((arg1 ^ arg0.wrapping_shr(63i64 as u32) != False) as i32 | (arg0.wrapping_sub(-36028797018963968i64) as u64 > 72057594037927935i64 as u64) as i32 == 0) as i32 != 0 {
            return arg0.wrapping_shl(Timepoint(0) as u32) | I128(0);
        }
        let var2 = imports.obj_from_i128_pieces(self, arg1, arg0);
        var2
    }
    fn func53<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) {
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
            let var13 = self.func45(imports, var3, Void);
            let var14: i64;
            if var13 != 0 {
                let var15 = imports.get_contract_data(self, var3, Void);
                var3 = var15;
                'label1: loop {
                    if (var2 != 64i32) as i32 != 0 {
                        self.memory.store64(var1.wrapping_add(16i32).wrapping_add(var2) as usize, Void as u64);
                        var2 = var2.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                if (var3 & 255i64 != Map(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                self.func48(imports, var3, 1048820i32, 8i32, var1.wrapping_add(16i32), 8i32);
                let var17 = self.memory.load64(var1 as usize + 16) as i64;
                var3 = var17;
                if (var3 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var18 = self.memory.load64(var1 as usize + 24) as i64;
                var4 = var18;
                if (var4 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var19 = self.memory.load64(var1 as usize + 32) as i64;
                var5 = var19;
                if (var5 & 255i64 != U32(0)) as i32 != 0 {
                    break 'label0;
                }
                let var20 = self.memory.load64(var1 as usize + 40) as i64;
                var6 = var20;
                if (var6 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var21 = self.memory.load64(var1 as usize + 48) as i64;
                var7 = var21;
                if (var7 & 255i64 != U32(0)) as i32 != 0 {
                    break 'label0;
                }
                let var22 = self.memory.load64(var1 as usize + 56) as i64;
                var8 = var22;
                if (var8 & 255i64 != Map(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var23 = self.memory.load64(var1 as usize + 64) as i64;
                self.func42(imports, var1, var23);
                let var25 = self.memory.load32(var1 as usize) as i32;
                if var25 != 0 {
                    break 'label0;
                }
                let var26 = self.memory.load64(var1 as usize + 72) as i64;
                var9 = var26;
                if (var9 & 255i64 != Address(obj#0)) as i32 != 0 {
                    break 'label0;
                }
                let var27 = self.memory.load64(var1 as usize + 8) as i64;
                var10 = var27;
                self.memory.store64(arg0 as usize + 8, var3 as u64);
                self.memory.store32(arg0.wrapping_add(60i32) as usize, (var5 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store32(arg0.wrapping_add(56i32) as usize, (var7 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store64(arg0.wrapping_add(48i32) as usize, var8 as u64);
                self.memory.store64(arg0.wrapping_add(40i32) as usize, var4 as u64);
                self.memory.store64(arg0.wrapping_add(32i32) as usize, var9 as u64);
                self.memory.store64(arg0.wrapping_add(24i32) as usize, var6 as u64);
                self.memory.store64(arg0.wrapping_add(16i32) as usize, var10 as u64);
                var14 = True;
            } else {
                var14 = False;
            }
            self.memory.store64(arg0 as usize, var14 as u64);
            self.global0 = var1.wrapping_add(80i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func54<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i32 {
        let var1 = self.func44(imports, arg0);
        let var2 = self.func45(imports, var1, Void);
        var2
    }
    fn func55<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) {
        let var1 = self.func44(imports, 1049208i32);
        let var2 = self.func56(imports, arg0);
        let var3 = imports.put_contract_data(self, var1, var2, Void);
        var3;
    }
    fn func56<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
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
        let var9 = self.memory.load32(arg0 as usize + 52) as i64;
        var2 = var9;
        let var10 = self.memory.load32(arg0 as usize + 48) as i64;
        var3 = var10;
        let var11 = self.memory.load64(arg0 as usize) as i64;
        var4 = var11;
        let var12 = self.memory.load64(arg0 as usize + 32) as i64;
        var5 = var12;
        let var13 = self.memory.load64(arg0 as usize + 16) as i64;
        var6 = var13;
        let var14 = self.memory.load64(arg0 as usize + 40) as i64;
        var7 = var14;
        let var15 = self.memory.load64(arg0 as usize + 8) as i64;
        let var16 = self.func61(imports, var15);
        self.memory.store64(var1 as usize + 48, var16 as u64);
        self.memory.store64(var1 as usize + 40, var7 as u64);
        self.memory.store64(var1 as usize + 24, var6 as u64);
        self.memory.store64(var1 as usize + 8, var5 as u64);
        self.memory.store64(var1 as usize, var4 as u64);
        let var17 = self.memory.load64(arg0 as usize + 24) as i64;
        self.memory.store64(var1 as usize + 56, var17 as u64);
        self.memory.store64(var1 as usize + 32, (var3.wrapping_shl(32i64 as u32) | U32(0)) as u64);
        self.memory.store64(var1 as usize + 16, (var2.wrapping_shl(32i64 as u32) | U32(0)) as u64);
        let var18 = self.func62(imports, 1048820i32, 8i32, var1, 8i32);
        self.global0 = var1.wrapping_sub(-64i32);
        var18
    }
    fn func57<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(32i32);
        self.global0 = var2;
        'label0: loop {
            'label1: loop {
                if (arg1 & 255i64 == Vec(obj#0)) as i32 != 0 {
                    'label2: loop {
                        if (var3 != 16i32) as i32 != 0 {
                            self.memory.store64(var2.wrapping_add(16i32).wrapping_add(var3) as usize, Void as u64);
                            var3 = var3.wrapping_add(8i32);
                            continue 'label2;
                        }
                        break;
                    }
                    let var6 = imports.vec_unpack_to_linear_memory(self, arg1, (var2.wrapping_add(16i32) as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), U32(2));
                    var6;
                    let var7 = self.memory.load64(var2 as usize + 16) as i64;
                    var4 = var7;
                    if (var4 & 255i64 != Address(obj#0)) as i32 != 0 {
                        break 'label1;
                    }
                    let var8 = self.memory.load64(var2 as usize + 24) as i64;
                    self.func42(imports, var2, var8);
                    let var10 = self.memory.load64(var2 as usize + 8) as i64;
                    var1 = var10;
                    let var11 = self.memory.load32(var2 as usize) as i32;
                    if (var11 == 0) as i32 != 0 {
                        self.memory.store64(arg0 as usize + 8, var4 as u64);
                        self.memory.store64(arg0 as usize, False as u64);
                        self.memory.store64(arg0.wrapping_add(16i32) as usize, arg1 as u64);
                        break 'label0;
                    }
                    self.memory.store64(arg0 as usize, True as u64);
                    self.memory.store64(arg0 as usize + 8, arg1 as u64);
                    break 'label0;
                }
                self.memory.store64(arg0 as usize, True as u64);
                self.memory.store64(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
                break 'label0;
                break;
            }
            self.memory.store64(arg0 as usize, True as u64);
            self.memory.store64(arg0 as usize + 8, Error(Value, UnexpectedType) as u64);
            break;
        }
        self.global0 = var2.wrapping_add(32i32);
    }
    fn func58<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(48i32);
        self.global0 = var1;
        let var7 = self.memory.load64(arg0 as usize) as i64;
        let var8 = self.memory.load64(arg0.wrapping_add(8i32) as usize) as i64;
        let var9 = self.func52(imports, var7, var8);
        var2 = var9;
        let var10 = self.memory.load64(arg0 as usize + 16) as i64;
        let var11 = self.memory.load64(arg0.wrapping_add(24i32) as usize) as i64;
        let var12 = self.func52(imports, var10, var11);
        var3 = var12;
        let var13 = self.memory.load64(arg0 as usize + 32) as i64;
        let var14 = self.memory.load64(arg0.wrapping_add(40i32) as usize) as i64;
        let var15 = self.func52(imports, var13, var14);
        var4 = var15;
        let var16 = self.memory.load64(arg0 as usize + 48) as i64;
        let var17 = self.memory.load64(arg0.wrapping_add(56i32) as usize) as i64;
        let var18 = self.func52(imports, var16, var17);
        var5 = var18;
        let var19 = self.memory.load64(arg0 as usize + 64) as i64;
        let var20 = self.memory.load64(arg0.wrapping_add(72i32) as usize) as i64;
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
    fn func59<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = imports.vec_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0));
        var2
    }
    fn func60<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var1 = var5.wrapping_sub(48i32);
        self.global0 = var1;
        let var6 = self.memory.load64(arg0 as usize) as i64;
        var2 = var6;
        let var7 = self.memory.load64(arg0 as usize + 8) as i64;
        var3 = var7;
        let var8 = self.memory.load64(arg0.wrapping_add(16i32) as usize) as i64;
        let var9 = self.func61(imports, var8);
        var4 = var9;
        let var10 = self.memory.load64(arg0.wrapping_add(24i32) as usize) as i64;
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
            if (arg0 == 16i32) as i32 != 0 {
                var0 = 0i32;
                'label1: loop {
                    if (arg0 != 16i32) as i32 != 0 {
                        let var15 = self.memory.load64(var1.wrapping_add(8i32).wrapping_add(arg0) as usize) as i64;
                        self.memory.store64(var1.wrapping_add(24i32).wrapping_add(arg0) as usize, var15 as u64);
                        var0 = arg0.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
                let var16 = self.func59(imports, var1.wrapping_add(24i32), 2i32);
                self.global0 = var1.wrapping_add(48i32);
                var14 = var16;
            } else {
                self.memory.store64(var1.wrapping_add(24i32).wrapping_add(arg0) as usize, Void as u64);
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
    fn func61<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        if (arg0 as u64 <= 72057594037927935i64 as u64) as i32 != 0 {
            return arg0.wrapping_shl(Timepoint(0) as u32) | U64(0);
        }
        let var1 = imports.obj_from_u64(self, arg0);
        var1
    }
    fn func62<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32) -> i64 {
        if (arg1 != arg3) as i32 != 0 {
            unreachable!();
        }
        let var4 = imports.map_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg2 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0));
        var4
    }
    fn func63<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let var4 = self.global0;
        var1 = var4.wrapping_sub(16i32);
        self.global0 = var1;
        self.memory.store64(var1 as usize, arg0 as u64);
        var3 = Void;
        var2 = 1i32;
        'label0: loop {
            if var2 != 0 {
                var2 = var2.wrapping_sub(1i32);
                var3 = arg0;
                continue 'label0;
            }
            break;
        }
        self.memory.store64(var1 as usize + 8, var3 as u64);
        let var5 = self.func59(imports, var1.wrapping_add(8i32), 1i32);
        self.global0 = var1.wrapping_add(16i32);
        var5
    }
    fn func64<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) -> i64 {
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
                    var5 = var6 as u32 as i64 & 255i64 | var5.wrapping_shl(U64(0) as u32);
                    var3 = var3.wrapping_sub(1i32);
                    var4 = var4.wrapping_add(1i32);
                    continue 'label1;
                }
                break;
            }
            return var5.wrapping_shl(Timepoint(0) as u32) | Symbol();
            break;
        }
        let var9 = imports.symbol_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), (arg1 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0));
        var9
    }
    fn func65<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            let var2 = self.memory.load8(arg0 as usize + 160) as i32;
            if (var2 == 0) as i32 != 0 {
                break 'label0;
            }
            let var3 = self.func66(imports);
            let var4 = self.memory.load64(arg0 as usize + 144) as i64;
            if ((var3 as u64) < var4 as u64) as i32 != 0 {
                break 'label0;
            }
            let var5 = self.memory.load64(arg0 as usize) as i64;
            let var6 = self.memory.load64(arg0 as usize + 16) as i64;
            let var7 = self.memory.load64(arg0.wrapping_add(8i32) as usize) as i64;
            let var8 = self.memory.load64(arg0.wrapping_add(24i32) as usize) as i64;
            var1 = (var5 ^ var6 | var7 ^ var8 == 0) as i32;
            break;
        }
        var1
    }
    fn func66<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i64 = 0;
        let mut var1: i32 = 0;
        let var2 = imports.get_ledger_timestamp(self);
        var0 = var2;
        var1 = var0 as i32 & 255i32;
        if (var1 != 64i32) as i32 != 0 {
            if (var1 == 6i32) as i32 != 0 {
                return (var0 as u64).wrapping_shr(Timepoint(0) as u32) as i64;
            }
            unreachable!();
        }
        let var3 = imports.obj_to_u64(self, var0);
        var3
    }
    fn func67<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64, mut arg7: i64, mut arg8: i64) {
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i64 = 0;
        let var13 = self.global0;
        var9 = var13.wrapping_sub(192i32);
        self.global0 = var9;
        'label0: loop {
            'label1: loop {
                var12 = arg4 ^ arg6;
                if ((var12 | arg3 ^ arg5 == 0) as i32 == 0) as i32 != 0 {
                    if ((arg1 ^ arg7 | arg2 ^ arg8 == 0) as i32 == 0) as i32 != 0 {
                        self.memory.store32(var9 as usize + 188, 0i32 as u32);
                        self.func68(imports, var9.wrapping_add(168i32), arg7, arg8, 20000000i64, False, var9.wrapping_add(188i32));
                        let var15 = self.memory.load32(var9 as usize + 188) as i32;
                        let var16 = self.memory.load64(var9.wrapping_add(176i32) as usize) as i64;
                        var7 = var16;
                        let var17 = self.memory.load64(var9 as usize + 168) as i64;
                        var8 = var17;
                        let var18 = self.func69(imports, arg5, arg6);
                        let var19 = self.func69(imports, arg1, arg2);
                        let var20 = imports.i256_mul(self, var18, var19);
                        let var21 = self.func69(imports, 10000000i64, False);
                        let var22 = imports.i256_div(self, var20, var21);
                        var12 = var22;
                        let var23 = self.func69(imports, arg3, arg4);
                        let var24 = self.func69(imports, arg5, arg6);
                        let var25 = imports.i256_sub(self, var23, var24);
                        let var26 = self.func69(imports, arg1, arg2);
                        let var27 = imports.i256_mul(self, var25, var26);
                        let var28 = self.func69(imports, arg1, arg2);
                        let var29 = imports.i256_mul(self, var27, var28);
                        var1 = var29;
                        if var15 != 0 {
                            break 'label0;
                        }
                        let var30 = self.func69(imports, arg8, arg7);
                        let var31 = imports.i256_div(self, arg1, var30);
                        var1 = var31;
                        self.func70(imports, var9.wrapping_add(144i32), var12);
                        let var33 = self.memory.load64(var9.wrapping_add(160i32) as usize) as i64;
                        let var34 = self.memory.load64(var9 as usize + 152) as i64;
                        var3 = var34;
                        let var35 = self.memory.load32(var9 as usize + 144) as i32;
                        var10 = var35;
                        self.func70(imports, var9.wrapping_add(120i32), arg1);
                        var1 = { let a = var33; let b = False; if var10 != 0 { a } else { b } };
                        let var37 = self.memory.load64(var9.wrapping_add(136i32) as usize) as i64;
                        let var38 = self.memory.load32(var9 as usize + 120) as i32;
                        var11 = var38;
                        var2 = { let a = var37; let b = False; if var11 != 0 { a } else { b } };
                        var3 = { let a = arg3; let b = False; if var10 != 0 { a } else { b } };
                        let var39 = self.memory.load64(var9 as usize + 128) as i64;
                        var4 = var3.wrapping_add({ let a = var39; let b = False; if var11 != 0 { a } else { b } });
                        var3 = (((var4 as u64) < arg3 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg2));
                        if ((var1 ^ var2 ^ -1i64) & (arg1 ^ var3) >= False) as i32 != 0 {
                            break 'label1;
                        }
                        break 'label0;
                    }
                    self.memory.store32(var9 as usize + 116, 0i32 as u32);
                    self.func68(imports, var9.wrapping_add(96i32), arg5, arg6, arg1, arg2, var9.wrapping_add(116i32));
                    let var41 = self.memory.load32(var9 as usize + 116) as i32;
                    if var41 != 0 {
                        break 'label0;
                    }
                    var7 = arg4.wrapping_sub(arg6).wrapping_sub(((arg3 as u64) < arg5 as u64) as i32 as u32 as i64);
                    if (var12 & (arg4 ^ var7) < False) as i32 != 0 {
                        break 'label0;
                    }
                    let var42 = self.memory.load64(var9.wrapping_add(104i32) as usize) as i64;
                    var4 = var42;
                    let var43 = self.memory.load64(var9 as usize + 96) as i64;
                    var6 = var43;
                    self.memory.store32(var9 as usize + 92, 0i32 as u32);
                    self.func68(imports, var9.wrapping_add(72i32), arg3.wrapping_sub(arg5), arg7, arg1, arg2, var9.wrapping_add(92i32));
                    let var45 = self.memory.load32(var9 as usize + 92) as i32;
                    if var45 != 0 {
                        break 'label0;
                    }
                    let var46 = self.memory.load64(var9 as usize + 72) as i64;
                    let var47 = self.memory.load64(var9.wrapping_add(80i32) as usize) as i64;
                    self.func71(imports, var9.wrapping_add(56i32), var46, var47, Void, False);
                    let var49 = self.memory.load64(var9.wrapping_sub(-64i32) as usize) as i64;
                    var1 = var49;
                    let var50 = var1;
                    let var51 = self.memory.load64(var9 as usize + 56) as i64;
                    var2 = arg6.wrapping_add(var51);
                    var1 = (((var2 as u64) < arg6 as u64) as i32 as u32 as i64).wrapping_add(arg1.wrapping_add(arg4));
                    if ((arg4 ^ var50 ^ -1i64) & (arg4 ^ var1) < False) as i32 != 0 {
                        break 'label0;
                    }
                    self.func71(imports, var9.wrapping_add(40i32), arg2, arg1, 10000000i64, False);
                    let var53 = self.memory.load64(var9.wrapping_add(48i32) as usize) as i64;
                    var3 = var53;
                    let var54 = self.memory.load64(var9 as usize + 40) as i64;
                    var4 = var54;
                    break 'label1;
                }
                self.memory.store32(var9 as usize + 36, 0i32 as u32);
                self.func68(imports, var9.wrapping_add(16i32), arg1, arg2, arg3, arg4, var9.wrapping_add(36i32));
                let var56 = self.memory.load32(var9 as usize + 36) as i32;
                if var56 != 0 {
                    break 'label0;
                }
                let var57 = self.memory.load64(var9 as usize + 16) as i64;
                let var58 = self.memory.load64(var9.wrapping_add(24i32) as usize) as i64;
                self.func71(imports, var9, var57, var58, 10000000i64, False);
                let var60 = self.memory.load64(var9.wrapping_add(8i32) as usize) as i64;
                var3 = var60;
                let var61 = self.memory.load64(var9 as usize) as i64;
                var4 = var61;
                break;
            }
            self.memory.store64(arg0 as usize, arg4 as u64);
            self.memory.store64(arg0 as usize + 8, arg3 as u64);
            self.global0 = var9.wrapping_add(192i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func68<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i32) {
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
            if (arg1 | arg2 == 0) as i32 | (arg3 | arg4 == 0) as i32 != 0 {
                var17 = var18;
                break 'label0;
            }
            var18;
            var12 = (arg2 < False) as i32;
            var13 = { let a = False.wrapping_sub(arg1); let b = arg1; if var12 != 0 { a } else { b } };
            var6 = (arg4 < False) as i32;
            var14 = { let a = False.wrapping_sub(arg3); let b = arg3; if var6 != 0 { a } else { b } };
            var3 = { let a = False.wrapping_sub(arg4.wrapping_add((arg3 != False) as i32 as u32 as i64)); let b = arg4; if var6 != 0 { a } else { b } };
            let var19 = self.global0;
            var6 = var19.wrapping_sub(96i32);
            self.global0 = var6;
            var11 = var8.wrapping_add(8i32);
            let var20: i64;
            'label1: loop {
                var1 = { let a = False.wrapping_sub(arg2.wrapping_add((arg1 != False) as i32 as u32 as i64)); let b = arg2; if var12 != 0 { a } else { b } };
                if ((var1 == 0) as i32 == 0) as i32 != 0 {
                    if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                        self.func100(imports, var6.wrapping_add(80i32), var14, arg3, var13, arg1);
                        let var22 = self.memory.load64(var6.wrapping_add(88i32) as usize) as i64;
                        var1 = var22;
                        var10 = 1i32;
                        let var23 = self.memory.load64(var6 as usize + 80) as i64;
                        var20 = var23;
                        break 'label1;
                    }
                    self.func100(imports, var6.wrapping_sub(-64i32), var13, False, var14, arg3);
                    self.func100(imports, var6.wrapping_add(48i32), arg1, False, var14, arg3);
                    let var26 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                    let var27 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                    var3 = var27;
                    let var28 = self.memory.load64(var6 as usize + 48) as i64;
                    var1 = var3.wrapping_add(var28);
                    var10 = (var26 != False) as i32 | ((var1 as u64) < arg3 as u64) as i32;
                    let var29 = self.memory.load64(var6 as usize + 64) as i64;
                    var20 = var29;
                    break 'label1;
                }
                if ((arg3 == 0) as i32 == 0) as i32 != 0 {
                    self.func100(imports, var6.wrapping_add(32i32), var14, False, var13, arg1);
                    self.func100(imports, var6.wrapping_add(16i32), arg3, False, var13, arg1);
                    let var32 = self.memory.load64(var6.wrapping_add(24i32) as usize) as i64;
                    let var33 = self.memory.load64(var6.wrapping_add(40i32) as usize) as i64;
                    var3 = var33;
                    let var34 = self.memory.load64(var6 as usize + 16) as i64;
                    var1 = var3.wrapping_add(var34);
                    var10 = (var32 != False) as i32 | ((var1 as u64) < arg3 as u64) as i32;
                    let var35 = self.memory.load64(var6 as usize + 32) as i64;
                    var20 = var35;
                    break 'label1;
                }
                self.func100(imports, var6, var14, arg3, var13, arg1);
                let var37 = self.memory.load64(var6.wrapping_add(8i32) as usize) as i64;
                var1 = var37;
                let var38 = self.memory.load64(var6 as usize) as i64;
                var20 = var38;
                break;
            }
            self.memory.store64(var11 as usize, var20 as u64);
            self.memory.store8(var11 as usize + 16, var10 as u8);
            self.memory.store64(var11 as usize + 8, arg1 as u64);
            self.global0 = var6.wrapping_add(96i32);
            let var39 = self.memory.load64(var8.wrapping_add(16i32) as usize) as i64;
            var14 = var39;
            let var40 = self.memory.load64(var8 as usize + 8) as i64;
            var13 = var40;
            let var41 = self.memory.load8(var8 as usize + 24) as i32;
            var6 = var41;
            'label2: loop {
                'label3: loop {
                    var2 = arg2 ^ arg4;
                    if (var2 >= False) as i32 != 0 {
                        if (arg2 ^ var14 >= False) as i32 != 0 {
                            break 'label3;
                        }
                        var17 = 1i32;
                        break 'label0;
                    }
                    var1 = False.wrapping_sub(var13);
                    var14 = False.wrapping_sub(var14.wrapping_add((var13 != False) as i32 as u32 as i64));
                    if (arg2 ^ var14 < False) as i32 != 0 {
                        break 'label2;
                    }
                    var13 = arg1;
                    break;
                }
                var17 = var6 & 1i32;
                break 'label0;
                break;
            }
            var13 = arg1;
            var17 = 1i32;
            break;
        }
        var6 = var17;
        self.memory.store64(var9 as usize, var13 as u64);
        self.memory.store8(var9 as usize + 16, var6 as u8);
        self.memory.store64(var9 as usize + 8, var14 as u64);
        self.global0 = var8.wrapping_add(32i32);
        let var42 = self.memory.load64(var7.wrapping_add(16i32) as usize) as i64;
        var1 = var42;
        let var43 = self.memory.load64(var7 as usize + 8) as i64;
        var2 = var43;
        let var44 = self.memory.load8(var7 as usize + 24) as i32;
        self.memory.store32(arg5 as usize, (var44 & 1i32) as u32);
        self.memory.store64(arg0 as usize + 8, arg1 as u64);
        self.memory.store64(arg0 as usize, arg2 as u64);
        self.global0 = var7.wrapping_add(32i32);
    }
    fn func69<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let var3 = self.global0;
        var2 = var3.wrapping_sub(16i32);
        self.global0 = var2;
        self.memory.store64(var2 as usize + 8, (arg0.wrapping_shl(56i64 as u32) | (arg0 & 65280i64).wrapping_shl(40i64 as u32) | (arg0 & 16711680i64).wrapping_shl(24i64 as u32) | (arg0 & 4278190080i64).wrapping_shl(Timepoint(0) as u32) | (arg0 as u64).wrapping_shr(Timepoint(0) as u32) as i64 & 4278190080i64 | (arg0 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (arg0 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (arg0 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
        self.memory.store64(var2 as usize, (arg1.wrapping_shl(56i64 as u32) | (arg1 & 65280i64).wrapping_shl(40i64 as u32) | (arg1 & 16711680i64).wrapping_shl(24i64 as u32) | (arg1 & 4278190080i64).wrapping_shl(Timepoint(0) as u32) | (arg1 as u64).wrapping_shr(Timepoint(0) as u32) as i64 & 4278190080i64 | (arg1 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (arg1 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (arg1 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
        let var4 = self.func102(imports, var2);
        var0 = var4;
        let var5 = self.func102(imports, { let a = 1049256i32; let b = 1049240i32; if (arg1 < False) as i32 != 0 { a } else { b } });
        let var6 = imports.bytes_append(self, var5, arg0);
        let var7 = imports.i256_val_from_be_bytes(self, var6);
        self.global0 = var2.wrapping_add(16i32);
        var7
    }
    fn func70<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i64 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let var7 = self.global0;
        var5 = var7.wrapping_sub(32i32);
        self.global0 = var5;
        let var8 = imports.i256_val_to_be_bytes(self, arg1);
        var1 = var8;
        let var9 = imports.bytes_slice(self, var1, U32(0), U32(16));
        self.func104(imports, var5.wrapping_add(15i32), var9);
        'label0: loop {
            let var11 = self.memory.load8(var5 as usize + 15) as i32;
            if var11 != 0 {
                break 'label0;
            }
            var6 = var5.wrapping_add(24i32);
            let var12 = self.memory.load64(var6 as usize) as i64;
            var3 = var12;
            let var13 = self.memory.load64(var5 as usize + 16) as i64;
            var4 = var13;
            let var14 = imports.bytes_slice(self, arg1, U32(16), U32(32));
            self.func104(imports, var5.wrapping_add(15i32), var14);
            let var16 = self.memory.load8(var5 as usize + 15) as i32;
            if var16 != 0 {
                break 'label0;
            }
            let var17 = self.memory.load64(var6 as usize) as i64;
            var1 = var17;
            let var18 = self.memory.load64(var5 as usize + 16) as i64;
            var2 = var18;
            var2 = var2.wrapping_shl(56i64 as u32) | (var2 & 65280i64).wrapping_shl(40i64 as u32) | (var2 & 16711680i64).wrapping_shl(24i64 as u32) | (var2 & 4278190080i64).wrapping_shl(Timepoint(0) as u32) | (var2 as u64).wrapping_shr(Timepoint(0) as u32) as i64 & 4278190080i64 | (var2 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (var2 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (var2 as u64).wrapping_shr(56i64 as u32) as i64;
            self.memory.store64(arg0.wrapping_add(16i32) as usize, var2 as u64);
            self.memory.store64(arg0 as usize + 8, (arg1.wrapping_shl(56i64 as u32) | (arg1 & 65280i64).wrapping_shl(40i64 as u32) | (arg1 & 16711680i64).wrapping_shl(24i64 as u32) | (arg1 & 4278190080i64).wrapping_shl(Timepoint(0) as u32) | (arg1 as u64).wrapping_shr(Timepoint(0) as u32) as i64 & 4278190080i64 | (arg1 as u64).wrapping_shr(24i64 as u32) as i64 & 16711680i64 | (arg1 as u64).wrapping_shr(40i64 as u32) as i64 & 65280i64 | (arg1 as u64).wrapping_shr(56i64 as u32) as i64) as u64);
            self.memory.store64(arg0 as usize, ((var3 | var4 == 0) as i32 & (var2 >= False) as i32 | (var3 & var4 == -1i64) as i32 & (var2 < False) as i32) as u32 as i64 as u64);
            self.global0 = var5.wrapping_add(32i32);
            return;
            break;
        }
        unreachable!();
    }
    fn func71<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
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
        var13 = (arg2 < False) as i32;
        var5 = { let a = False.wrapping_sub(arg1); let b = arg1; if var13 != 0 { a } else { b } };
        var1 = { let a = False.wrapping_sub(arg2.wrapping_add((arg1 != False) as i32 as u32 as i64)); let b = arg2; if var13 != 0 { a } else { b } };
        var12 = (arg4 < False) as i32;
        var6 = { let a = False.wrapping_sub(arg3); let b = arg3; if var12 != 0 { a } else { b } };
        let var18 = self.global0;
        var13 = var18.wrapping_sub(32i32);
        self.global0 = var13;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            var3 = { let a = False.wrapping_sub(arg4.wrapping_add((arg3 != False) as i32 as u32 as i64)); let b = arg4; if var12 != 0 { a } else { b } };
                            if ((var3 == 0) as i32 == 0) as i32 != 0 {
                                if (arg1 == 0) as i32 | ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = ((arg1 as u64) < arg3 as u64) as i32; if (arg1 == arg3) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label2;
                                }
                                var12 = arg3.leading_zeros() as i64 as i32;
                                var15 = arg1.leading_zeros() as i64 as i32;
                                if ((var12 as u32) < var15 as u32) as i32 != 0 {
                                    break 'label4;
                                }
                                var12 = var12.wrapping_sub(var15);
                                if (var12 as u32 >= 128i32 as u32) as i32 != 0 {
                                    break 'label4;
                                }
                                self.func101(imports, var13.wrapping_add(16i32), var6, arg3, var12);
                                var11 = True.wrapping_shl(var12 as u32 as i64 as u32);
                                let var20 = self.memory.load64(var13.wrapping_add(24i32) as usize) as i64;
                                var8 = var20;
                                let var21 = self.memory.load64(var13 as usize + 16) as i64;
                                var9 = var21;
                                'label5: loop {
                                    var7 = arg1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                    if (var7 >= False) as i32 != 0 {
                                        var10 = var10 | var11;
                                        var5 = var5.wrapping_sub(var9);
                                        if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (arg3 as u64 > var7 as u64) as i32; if (arg3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                            break 'label1;
                                        }
                                        var1 = var7;
                                    }
                                    var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(True as u32) as i64;
                                    var11 = (var11 as u64).wrapping_shr(True as u32) as i64;
                                    var8 = (var8 as u64).wrapping_shr(True as u32) as i64;
                                    continue 'label5;
                                    break;
                                }
                                unreachable!();
                            }
                            'label6: loop {
                                'label7: loop {
                                    'label8: loop {
                                        'label9: loop {
                                            if ((arg1 == 0) as i32 == 0) as i32 != 0 {
                                                if ((arg1 as u64) < var6 as u64) as i32 != 0 {
                                                    break 'label9;
                                                }
                                                if (arg1 == var6) as i32 != 0 {
                                                    break 'label8;
                                                }
                                                var11 = (arg1 as u64 / var6 as u64) as i64;
                                                var7 = arg1.wrapping_sub(var11.wrapping_mul(var6));
                                                if (var6 as u64 >= 4294967296i64 as u64) as i32 != 0 {
                                                    break 'label7;
                                                }
                                                var1 = var7.wrapping_shl(32i64 as u32) | (var5 as u64).wrapping_shr(32i64 as u32) as i64;
                                                let var22 = var1;
                                                var1 = (arg1 as u64 / var6 as u64) as i64;
                                                var3 = var5 & 4294967295i64 | var22.wrapping_sub(var1.wrapping_mul(var6)).wrapping_shl(32i64 as u32);
                                                let var23 = var3;
                                                var3 = (arg3 as u64 / var6 as u64) as i64;
                                                var5 = var23.wrapping_sub(var6.wrapping_mul(var3));
                                                var10 = arg1.wrapping_shl(32i64 as u32) | arg3;
                                                var11 = (arg1 as u64).wrapping_shr(32i64 as u32) as i64 | var11;
                                                var7 = False;
                                                break 'label0;
                                            }
                                            var10 = (var5 as u64 / var6 as u64) as i64;
                                            var5 = var5.wrapping_sub(var10.wrapping_mul(var6));
                                            break 'label3;
                                            break;
                                        }
                                        var15 = arg1.leading_zeros() as i64 as i32;
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
                                        self.func101(imports, var13, var6, arg3, var12);
                                        var7 = True.wrapping_shl(var12 as u32 as i64 as u32);
                                        let var25 = self.memory.load64(var13.wrapping_add(8i32) as usize) as i64;
                                        var8 = var25;
                                        let var26 = self.memory.load64(var13 as usize) as i64;
                                        var9 = var26;
                                        'label10: loop {
                                            var3 = arg1.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                            if (var3 >= False) as i32 != 0 {
                                                var5 = var5.wrapping_sub(var9);
                                                var10 = var7 | var10;
                                                if (arg3 == 0) as i32 != 0 {
                                                    break 'label6;
                                                }
                                                var1 = arg3;
                                            }
                                            var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(True as u32) as i64;
                                            var7 = (var7 as u64).wrapping_shr(True as u32) as i64;
                                            var8 = (var8 as u64).wrapping_shr(True as u32) as i64;
                                            continue 'label10;
                                            break;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    var10 = (var5 as u64 / arg1 as u64) as i64;
                                    var5 = var5.wrapping_sub(var10.wrapping_mul(arg1));
                                    var11 = True;
                                    break 'label0;
                                    break;
                                }
                                if ({ let a = ((var5 as u64) < var6 as u64) as i32; let b = (arg3 as u64 > var7 as u64) as i32; if (arg3 == var7) as i32 != 0 { a } else { b } }) != 0 {
                                    break 'label0;
                                }
                                var8 = arg3.wrapping_shl(63i64 as u32) | (var6 as u64).wrapping_shr(True as u32) as i64;
                                var9 = var6.wrapping_shl(63i64 as u32);
                                var1 = -9223372036854775808i64;
                                'label11: loop {
                                    'label12: loop {
                                        var3 = var7.wrapping_sub(var8).wrapping_sub(((var5 as u64) < var9 as u64) as i32 as u32 as i64);
                                        if (var3 >= False) as i32 != 0 {
                                            var5 = var5.wrapping_sub(var9);
                                            var10 = arg1 | var10;
                                            if (arg3 == 0) as i32 != 0 {
                                                break 'label12;
                                            }
                                            var7 = arg3;
                                        }
                                        var9 = var8.wrapping_shl(63i64 as u32) | (var9 as u64).wrapping_shr(True as u32) as i64;
                                        var1 = (arg1 as u64).wrapping_shr(True as u32) as i64;
                                        var8 = (var8 as u64).wrapping_shr(True as u32) as i64;
                                        continue 'label11;
                                        break;
                                    }
                                    break;
                                }
                                var1 = (var5 as u64 / var6 as u64) as i64;
                                var10 = var1 | var10;
                                var5 = var5.wrapping_sub(arg1.wrapping_mul(var6));
                                var7 = False;
                                break 'label0;
                                break;
                            }
                            var1 = (var5 as u64 / var6 as u64) as i64;
                            var10 = var1 | var10;
                            var5 = var5.wrapping_sub(arg1.wrapping_mul(var6));
                            break 'label3;
                            break;
                        }
                        unreachable!();
                        break;
                    }
                    var7 = False;
                    break 'label1;
                    break;
                }
                var7 = arg1;
                break;
            }
            var11 = False;
            break;
        }
        self.memory.store64(var14 as usize + 16, var5 as u64);
        self.memory.store64(var14 as usize, var10 as u64);
        self.memory.store64(var14.wrapping_add(24i32) as usize, var7 as u64);
        self.memory.store64(var14 as usize + 8, var11 as u64);
        self.global0 = var13.wrapping_add(32i32);
        let var27 = self.memory.load64(var14.wrapping_add(8i32) as usize) as i64;
        var1 = var27;
        let var28 = self.memory.load64(var14 as usize) as i64;
        var3 = var28;
        var13 = (arg2 ^ arg4 < False) as i32;
        self.memory.store64(arg0 as usize, ({ let a = False.wrapping_sub(var3); let b = arg3; if var13 != 0 { a } else { b } }) as u64);
        self.memory.store64(arg0 as usize + 8, ({ let a = False.wrapping_sub(arg1.wrapping_add((arg3 != False) as i32 as u32 as i64)); let b = arg1; if var13 != 0 { a } else { b } }) as u64);
        self.global0 = var14.wrapping_add(32i32);
    }
    fn func72<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) -> i64 {
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
                let var16 = imports.map_len(self, arg0);
                var5 = var16;
                if (var5 as u64 <= 4294967295i64 as u64) as i32 != 0 {
                    break 'label1;
                }
                self.func71(imports, var3.wrapping_add(32i32), arg1, arg2, (var5 as u64).wrapping_shr(32i64 as u32) as i64, False);
                let var18 = imports.map_len(self, arg0);
                var5 = var18;
                self.memory.store32(var3 as usize + 28, 0i32 as u32);
                let var19 = self.memory.load64(var3 as usize + 32) as i64;
                var13 = var19;
                let var20 = self.memory.load64(var3.wrapping_add(40i32) as usize) as i64;
                var9 = var20;
                self.func68(imports, var3.wrapping_add(8i32), var13, var9, (var5 as u64).wrapping_shr(32i64 as u32) as i64, False, var3.wrapping_add(28i32));
                let var22 = self.memory.load32(var3 as usize + 28) as i32;
                if var22 != 0 {
                    break 'label1;
                }
                let var23 = self.memory.load64(var3.wrapping_add(16i32) as usize) as i64;
                var5 = var23;
                let var24 = var5;
                let var25 = var5;
                let var26 = self.memory.load64(var3 as usize + 8) as i64;
                var5 = var26;
                var7 = arg2.wrapping_sub(var25).wrapping_sub(((arg1 as u64) < var5 as u64) as i32 as u32 as i64);
                if ((arg2 ^ var24) & (arg2 ^ var7) < False) as i32 != 0 {
                    break 'label1;
                }
                var8 = arg1.wrapping_sub(var5);
                let var27 = imports.map_len(self, arg0);
                var14 = (var27 as u64).wrapping_shr(32i64 as u32) as i64;
                var4 = var3.wrapping_sub(-64i32);
                var1 = U32(0);
                var2 = False;
                var5 = arg0;
                'label2: loop {
                    if (arg2 == var14) as i32 != 0 {
                        break 'label0;
                    }
                    let var28 = imports.map_key_by_pos(self, arg0, arg1);
                    var10 = var28;
                    let var29 = imports.map_val_by_pos(self, arg0, arg1);
                    var6 = var29;
                    if (arg2 == 4294967295i64) as i32 | (var10 & 255i64 != U32(0)) as i32 != 0 {
                        break 'label1;
                    }
                    self.func73(imports, var3.wrapping_add(48i32), var6);
                    let var31 = self.memory.load64(var3 as usize + 48) as i64;
                    if ((var31 == 0) as i32 == 0) as i32 != 0 {
                        break 'label1;
                    }
                    let var32 = self.memory.load64(var4 as usize) as i64;
                    var11 = var32;
                    let var33 = self.memory.load64(var3 as usize + 56) as i64;
                    var6 = var33;
                    var12 = var6.wrapping_add(var13);
                    var6 = (((var12 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var9.wrapping_add(var11));
                    if ((var11 ^ var9 ^ -1i64) & (var11 ^ var6) < False) as i32 != 0 {
                        break 'label1;
                    }
                    let var34 = var7;
                    var8 = var8.wrapping_add(var12);
                    var7 = (((var8 as u64) < var12 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_add(var7));
                    if ((var6 ^ var34 ^ -1i64) & (var6 ^ var7) < False) as i32 != 0 {
                        break 'label1;
                    }
                    let var35 = self.memory.load64(var3 as usize + 72) as i64;
                    let var36 = self.func61(imports, var35);
                    var6 = var36;
                    let var37 = self.func52(imports, var8, var7);
                    self.memory.store64(var3 as usize + 56, var37 as u64);
                    self.memory.store64(var3 as usize + 48, var6 as u64);
                    var1 = arg1.wrapping_add(4294967296i64);
                    var2 = arg2.wrapping_add(True);
                    var8 = False;
                    let var38 = self.func62(imports, 1049192i32, 2i32, var3.wrapping_add(48i32), 2i32);
                    let var39 = imports.map_put(self, var5, var10 & -4294967296i64 | U32(0), var38);
                    var5 = var39;
                    var7 = False;
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
    fn func73<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64i32);
        self.global0 = var2;
        'label0: loop {
            if (var3 != 16i32) as i32 != 0 {
                self.memory.store64(var2.wrapping_add(24i32).wrapping_add(var3) as usize, Void as u64);
                var3 = var3.wrapping_add(8i32);
                continue 'label0;
            }
            break;
        }
        'label1: loop {
            'label2: loop {
                if (arg1 & 255i64 == Map(obj#0)) as i32 != 0 {
                    self.func48(imports, arg1, 1049192i32, 2i32, var2.wrapping_add(24i32), 2i32);
                    let var7 = self.memory.load64(var2 as usize + 24) as i64;
                    self.func42(imports, var2.wrapping_add(8i32), var7);
                    let var9 = self.memory.load32(var2 as usize + 8) as i32;
                    if var9 != 0 {
                        break 'label2;
                    }
                    let var10 = self.memory.load64(var2 as usize + 16) as i64;
                    var1 = var10;
                    let var11 = self.memory.load64(var2 as usize + 32) as i64;
                    self.func46(imports, var2.wrapping_add(40i32), var11);
                    let var13 = self.memory.load64(var2 as usize + 40) as i64;
                    if (var13 == 0) as i32 != 0 {
                        let var14 = self.memory.load64(var2.wrapping_add(56i32) as usize) as i64;
                        var4 = var14;
                        let var15 = self.memory.load64(var2 as usize + 48) as i64;
                        self.memory.store64(arg0 as usize + 8, var15 as u64);
                        self.memory.store64(arg0 as usize, False as u64);
                        self.memory.store64(arg0.wrapping_add(16i32) as usize, var4 as u64);
                        self.memory.store64(arg0.wrapping_add(24i32) as usize, arg1 as u64);
                        break 'label1;
                    }
                    self.memory.store64(arg0 as usize, True as u64);
                    break 'label1;
                }
                self.memory.store64(arg0 as usize, True as u64);
                break 'label1;
                break;
            }
            self.memory.store64(arg0 as usize, True as u64);
            break;
        }
        self.global0 = var2.wrapping_sub(-64i32);
    }
    fn func74<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i64 = 0;
        let var8 = self.global0;
        var6 = var8.wrapping_sub(48i32);
        self.global0 = var6;
        if (({ let a = (arg3 == 0) as i32; let b = (arg4 < False) as i32; if (arg4 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
            let var9 = self.func75(imports, 1049272i32, 8i32);
            var7 = var9;
            let var10 = self.func52(imports, arg3, arg4);
            self.memory.store64(var6 as usize + 16, var10 as u64);
            self.memory.store64(var6 as usize + 8, arg2 as u64);
            self.memory.store64(var6 as usize, arg1 as u64);
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
                    self.func76(imports, arg0, var7, var12);
                } else {
                    self.memory.store64(var6.wrapping_add(24i32).wrapping_add(var5) as usize, Void as u64);
                    var5 = var5.wrapping_add(8i32);
                    continue 'label0;
                }
                break;
            }
        }
        self.global0 = var6.wrapping_add(48i32);
    }
    fn func75<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) -> i64 {
        let var2 = self.func64(imports, arg0, arg1);
        var2
    }
    fn func76<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) {
        let var3 = imports.call(self, arg0, arg1, arg2);
        if (var3 & 255i64 != Void) as i32 != 0 {
            unreachable!();
        }
    }
    fn func77<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32, mut arg4: i32, mut arg5: i32) {
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
        let var28 = self.memory.load64(arg3 as usize) as i64;
        var15 = var28;
        let var29 = self.memory.load64(arg3.wrapping_add(8i32) as usize) as i64;
        var11 = var29;
        let var30 = self.memory.load64(arg3 as usize + 96) as i64;
        var7 = var30;
        let var31 = self.memory.load64(arg3.wrapping_add(104i32) as usize) as i64;
        var8 = var31;
        let var32 = self.memory.load64(arg3 as usize + 80) as i64;
        var12 = var32;
        let var33 = self.memory.load64(arg3.wrapping_add(88i32) as usize) as i64;
        var19 = var33;
        self.func67(imports, var6.wrapping_add(240i32), var15, var11, var7, var8, var12, var19, var15, var11);
        'label0: loop {
            if (arg1 | arg2 == 0) as i32 != 0 {
                self.func78(imports, arg0, 80i32);
                break 'label0;
            }
            let var36 = self.memory.load64(var6.wrapping_add(248i32) as usize) as i64;
            var9 = var36;
            let var37 = self.memory.load64(var6 as usize + 240) as i64;
            var20 = var37;
            self.func71(imports, var6.wrapping_add(224i32), arg1, arg2, 10000i64, False);
            self.memory.store32(var6 as usize + 220, 0i32 as u32);
            let var39 = self.memory.load64(var6 as usize + 224) as i64;
            var13 = var39;
            let var40 = self.memory.load64(var6.wrapping_add(232i32) as usize) as i64;
            var10 = var40;
            var21 = arg4 as u32 as i64;
            self.func68(imports, var6.wrapping_add(200i32), var13, var10, var21, False, var6.wrapping_add(220i32));
            'label1: loop {
                let var42 = self.memory.load32(var6 as usize + 220) as i32;
                if var42 != 0 {
                    break 'label1;
                }
                let var43 = self.memory.load64(var6.wrapping_add(208i32) as usize) as i64;
                var17 = var43;
                let var44 = self.memory.load64(var6 as usize + 200) as i64;
                var18 = var44;
                self.memory.store32(var6 as usize + 196, 0i32 as u32);
                var26 = arg5 as u32 as i64;
                self.func68(imports, var6.wrapping_add(176i32), var13, var10, var26, False, var6.wrapping_add(196i32));
                let var46 = self.memory.load32(var6 as usize + 196) as i32;
                if var46 != 0 {
                    break 'label1;
                }
                var10 = arg2.wrapping_sub(var17).wrapping_sub(((arg1 as u64) < var18 as u64) as i32 as u32 as i64);
                if ((arg2 ^ var17) & (arg2 ^ var10) < False) as i32 != 0 {
                    break 'label1;
                }
                let var47 = self.memory.load64(var6.wrapping_add(184i32) as usize) as i64;
                var22 = var47;
                var16 = arg1.wrapping_sub(var18);
                let var48 = self.memory.load64(var6 as usize + 176) as i64;
                var23 = var48;
                var13 = var10.wrapping_sub(var22).wrapping_sub(((var16 as u64) < var23 as u64) as i32 as u32 as i64);
                if ((var10 ^ var22) & (var10 ^ var13) < False) as i32 != 0 {
                    break 'label1;
                }
                let var49 = self.memory.load64(arg3.wrapping_add(40i32) as usize) as i64;
                var10 = var49;
                let var50 = self.memory.load64(arg3 as usize + 32) as i64;
                var24 = var50;
                var16 = var16.wrapping_sub(var23);
                var25 = var24.wrapping_add(var16);
                var14 = (((var25 as u64) < var24 as u64) as i32 as u32 as i64).wrapping_add(var10.wrapping_add(var13));
                if ((var10 ^ var13 ^ -1i64) & (var10 ^ var14) < False) as i32 != 0 {
                    break 'label1;
                }
                let var51: i64;
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            if ({ let a = ((var20 as u64) < var25 as u64) as i32; let b = (var9 < var14) as i32; if (var9 == var14) as i32 != 0 { a } else { b } }) != 0 {
                                var13 = var9.wrapping_sub(var10).wrapping_sub(((var20 as u64) < var24 as u64) as i32 as u32 as i64);
                                if ((var9 ^ var10) & (var9 ^ var13) < False) as i32 != 0 {
                                    break 'label1;
                                }
                                self.memory.store32(var6 as usize + 84, 0i32 as u32);
                                var16 = var20.wrapping_sub(var24);
                                self.func68(imports, var6.wrapping_sub(-64i32), var16, var13, 10000i64, False, var6.wrapping_add(84i32));
                                let var53 = self.memory.load32(var6 as usize + 84) as i32;
                                if (var53 == 0) as i32 != 0 {
                                    break 'label4;
                                }
                                break 'label1;
                            }
                            var9 = var8 ^ var19;
                            if ((var9 | var7 ^ var12 == 0) as i32 == 0) as i32 != 0 {
                                var10 = var8.wrapping_sub(var19).wrapping_sub(((var7 as u64) < var12 as u64) as i32 as u32 as i64);
                                if (var9 & (var8 ^ var10) < False) as i32 != 0 {
                                    break 'label1;
                                }
                                let var54 = self.func69(imports, var7.wrapping_sub(var12), var10);
                                var10 = var54;
                                let var55 = self.func69(imports, Void, False);
                                let var56 = self.func69(imports, var12, var19);
                                let var57 = imports.i256_mul(self, var55, var56);
                                let var58 = self.func69(imports, var15, var11);
                                let var59 = imports.i256_mul(self, var57, var58);
                                var12 = var59;
                                let var60 = self.func69(imports, -2i64, -1i64);
                                let var61 = self.func69(imports, var15, var11);
                                let var62 = imports.i256_mul(self, var60, var61);
                                let var63 = self.func69(imports, 10000000i64, False);
                                let var64 = imports.i256_mul(self, var62, var63);
                                let var65 = self.func69(imports, var25, var14);
                                let var66 = imports.i256_mul(self, var64, var65);
                                var7 = var66;
                                let var67 = imports.i256_pow(self, var12, U32(2));
                                let var68 = self.func69(imports, U32(0), False);
                                let var69 = imports.i256_mul(self, var68, var10);
                                let var70 = imports.i256_mul(self, var69, var7);
                                let var71 = imports.i256_sub(self, var67, var70);
                                var9 = var71;
                                self.func70(imports, var6.wrapping_add(152i32), var9);
                                let var73 = self.memory.load32(var6 as usize + 152) as i32;
                                if var73 != 0 {
                                    var7 = I256(0);
                                    let var74 = self.memory.load64(var6.wrapping_add(168i32) as usize) as i64;
                                    if (var74 < False) as i32 != 0 {
                                        break 'label3;
                                    }
                                }
                                let var75 = imports.i256_add(self, var9, I256(1));
                                var14 = var75;
                                var7 = var9;
                                'label5: loop {
                                    'label6: loop {
                                        let var76 = imports.i256_div(self, var14, I256(2));
                                        var8 = var76;
                                        if ((var8 & 255i64 == I256(0)) as i32 & (var7 & 255i64 == I256(0)) as i32 == 0) as i32 != 0 {
                                            let var77 = imports.obj_cmp(self, var8, var7);
                                            if (var77 < False) as i32 != 0 {
                                                break 'label6;
                                            }
                                            break 'label3;
                                        }
                                        if (var8.wrapping_shr(Timepoint(0) as u32) >= var7.wrapping_shr(Timepoint(0) as u32)) as i32 != 0 {
                                            break 'label3;
                                        }
                                        break;
                                    }
                                    let var78 = imports.i256_div(self, var9, var8);
                                    let var79 = imports.i256_add(self, var78, var8);
                                    var14 = var79;
                                    var7 = var8;
                                    continue 'label5;
                                    break;
                                }
                                unreachable!();
                            }
                            self.memory.store32(var6 as usize + 124, 0i32 as u32);
                            self.func68(imports, var6.wrapping_add(104i32), var25, var14, 10000000i64, False, var6.wrapping_add(124i32));
                            let var81 = self.memory.load32(var6 as usize + 124) as i32;
                            if var81 | (var7 | var8 == 0) as i32 != 0 {
                                break 'label1;
                            }
                            let var82 = self.memory.load64(var6 as usize + 104) as i64;
                            var9 = var82;
                            let var83 = self.memory.load64(var6.wrapping_add(112i32) as usize) as i64;
                            var12 = var83;
                            if (var9 | var12 ^ -9223372036854775808i64 == 0) as i32 & (var7 & var8 == -1i64) as i32 != 0 {
                                break 'label1;
                            }
                            self.func71(imports, var6.wrapping_add(88i32), var9, var12, var7, var8);
                            let var85 = self.memory.load64(var6.wrapping_add(96i32) as usize) as i64;
                            var7 = var85;
                            let var86 = self.memory.load64(var6 as usize + 88) as i64;
                            var51 = var86;
                            break 'label2;
                            break;
                        }
                        var2 = var21.wrapping_add(var26);
                        var1 = var2.wrapping_sub(10000i64);
                        var2 = (((arg2 as u64) < var21 as u64) as i32 as u32 as i64).wrapping_add(((arg1 as u64) < arg2 as u64) as i32 as u32 as i64).wrapping_sub(True);
                        if (var1 | var2 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        let var87 = self.memory.load64(var6 as usize + 64) as i64;
                        var7 = var87;
                        let var88 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                        var8 = var88;
                        if (var7 | var8 ^ -9223372036854775808i64 == 0) as i32 & (arg1 & arg2 == -1i64) as i32 != 0 {
                            break 'label1;
                        }
                        self.func71(imports, var6.wrapping_add(48i32), var7, var8, arg1, arg2);
                        let var90 = self.memory.load64(var6 as usize + 48) as i64;
                        var1 = var90;
                        let var91 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                        var2 = var91;
                        if (var1 | var2 ^ -9223372036854775808i64 == 0) as i32 != 0 {
                            break 'label1;
                        }
                        self.func71(imports, var6.wrapping_add(32i32), arg1, arg2, -10000i64, -1i64);
                        self.memory.store32(var6 as usize + 28, 0i32 as u32);
                        let var93 = self.memory.load64(var6 as usize + 32) as i64;
                        let var94 = self.memory.load64(var6.wrapping_add(40i32) as usize) as i64;
                        self.func68(imports, var6.wrapping_add(8i32), var93, var94, var21, False, var6.wrapping_add(28i32));
                        let var96 = self.memory.load32(var6 as usize + 28) as i32;
                        if var96 != 0 {
                            break 'label1;
                        }
                        var2 = False.wrapping_sub(arg2.wrapping_add((arg1 != False) as i32 as u32 as i64));
                        let var97 = self.memory.load64(var6.wrapping_add(16i32) as usize) as i64;
                        var17 = var97;
                        var1 = False.wrapping_sub(arg1);
                        let var98 = self.memory.load64(var6 as usize + 8) as i64;
                        var18 = var98;
                        var7 = arg2.wrapping_sub(var17).wrapping_sub(((var1 as u64) < var18 as u64) as i32 as u32 as i64);
                        if ((var2 ^ var17) & (arg2 ^ var7) < False) as i32 != 0 {
                            break 'label1;
                        }
                        var8 = arg1.wrapping_sub(var18);
                        var22 = var7.wrapping_sub(var13).wrapping_sub(((var8 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((var7 ^ var13) & (var7 ^ var22) < False) as i32 != 0 {
                            break 'label1;
                        }
                        var23 = var8.wrapping_sub(var16);
                        var7 = var11;
                        var51 = var15;
                        break 'label2;
                        break;
                    }
                    let var99 = imports.i256_sub(self, var7, var12);
                    let var100 = self.func69(imports, Void, False);
                    let var101 = imports.i256_mul(self, var100, var10);
                    let var102 = imports.i256_div(self, var99, var101);
                    self.func70(imports, var6.wrapping_add(128i32), var102);
                    let var104 = self.memory.load64(var6.wrapping_add(144i32) as usize) as i64;
                    let var105 = self.memory.load32(var6 as usize + 128) as i32;
                    var4 = var105;
                    var7 = { let a = var104; let b = False; if var4 != 0 { a } else { b } };
                    let var106 = self.memory.load64(var6 as usize + 136) as i64;
                    var51 = { let a = var106; let b = False; if arg4 != 0 { a } else { b } };
                    break;
                }
                var8 = var51;
                var4 = { let a = ((var8 as u64) < var15 as u64) as i32; let b = (var7 < var11) as i32; if (var7 == var11) as i32 != 0 { a } else { b } };
                var7 = { let a = var7; let b = var11; if var4 != 0 { a } else { b } };
                let var107 = self.memory.load64(arg3.wrapping_add(24i32) as usize) as i64;
                var11 = var107;
                let var108 = var11;
                var8 = { let a = var8; let b = var15; if arg4 != 0 { a } else { b } };
                let var109 = self.memory.load64(arg3 as usize + 16) as i64;
                var11 = var109;
                var9 = var7.wrapping_sub(var11).wrapping_sub(((var8 as u64) < var11 as u64) as i32 as u32 as i64);
                if ((var7 ^ var108) & (var7 ^ var9) < False) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(arg0 as usize + 64, var23 as u64);
                self.memory.store64(arg0 as usize + 48, var18 as u64);
                self.memory.store64(arg0 as usize + 32, var8.wrapping_sub(var11) as u64);
                self.memory.store64(arg0 as usize + 16, var16 as u64);
                self.memory.store64(arg0 as usize, arg1 as u64);
                self.memory.store64(arg0.wrapping_add(72i32) as usize, var22 as u64);
                self.memory.store64(arg0.wrapping_add(56i32) as usize, var17 as u64);
                self.memory.store64(arg0.wrapping_add(40i32) as usize, var9 as u64);
                self.memory.store64(arg0.wrapping_add(24i32) as usize, var13 as u64);
                self.memory.store64(arg0 as usize + 8, arg2 as u64);
                break 'label0;
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var6.wrapping_add(256i32);
    }
    fn func78<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        if (arg1 as u32 >= 16i32 as u32) as i32 != 0 {
            var4 = 0i32.wrapping_sub(arg0) & 3i32;
            var2 = arg0.wrapping_add(var4);
            'label0: loop {
                if ((arg0 as u32) < var2 as u32) as i32 != 0 {
                    self.memory.store8(arg0 as usize, 0i32 as u8);
                    var0 = arg0.wrapping_add(1i32);
                    continue 'label0;
                }
                break;
            }
            var0 = 8i32;
            'label1: loop {
                if (arg0 as u32 >= 32i32 as u32) as i32 != 0 {
                    'label2: loop {
                        var1 = arg1.wrapping_sub(var4);
                        var0 = var2.wrapping_add(var1 & -4i32);
                        'label3: loop {
                            if (arg0 as u32 <= var2 as u32) as i32 != 0 {
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
                    var3 = var3.wrapping_shl((arg0 & 24i32) as u32) | var3;
                    var0 = arg0.wrapping_shl(1i32 as u32);
                    continue 'label1;
                }
                break;
            }
            var1 = arg1 & 3i32;
        }
        var1 = arg0.wrapping_add(arg1);
        'label4: loop {
            if ((arg0 as u32) < arg1 as u32) as i32 != 0 {
                self.memory.store8(arg0 as usize, 0i32 as u8);
                var0 = arg0.wrapping_add(1i32);
                continue 'label4;
            }
            break;
        }
    }
    fn func79<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32, mut arg4: i32, mut arg5: i32) {
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
            if (arg1 | arg2 == 0) as i32 != 0 {
                self.func78(imports, arg0, 80i32);
                break 'label0;
            }
            'label1: loop {
                let var18 = self.memory.load64(arg3 as usize + 16) as i64;
                var8 = var18;
                let var19 = self.memory.load64(arg3.wrapping_add(24i32) as usize) as i64;
                var7 = var19;
                if ({ let a = (var8 as u64 > arg1 as u64) as i32; let b = (var7 > arg2) as i32; if (arg2 == var7) as i32 != 0 { a } else { b } }) != 0 {
                    var9 = var7.wrapping_sub(arg2).wrapping_sub((arg1 as u64 > var8 as u64) as i32 as u32 as i64);
                    if ((arg2 ^ var7) & (var7 ^ var9) < False) as i32 != 0 {
                        break 'label1;
                    }
                    let var20 = self.memory.load64(arg3 as usize + 96) as i64;
                    let var21 = self.memory.load64(arg3.wrapping_add(104i32) as usize) as i64;
                    let var22 = self.memory.load64(arg3 as usize + 80) as i64;
                    let var23 = self.memory.load64(arg3.wrapping_add(88i32) as usize) as i64;
                    let var24 = self.memory.load64(arg3 as usize) as i64;
                    let var25 = self.memory.load64(arg3.wrapping_add(8i32) as usize) as i64;
                    self.func67(imports, var6.wrapping_sub(-64i32), var8.wrapping_sub(arg1), var9, var20, var21, var22, var23, var24, var25);
                    let var27 = self.memory.load64(var6.wrapping_add(72i32) as usize) as i64;
                    var10 = var27;
                    let var28 = self.memory.load64(var6 as usize + 64) as i64;
                    var9 = var28;
                }
                let var29 = self.memory.load64(arg3.wrapping_add(40i32) as usize) as i64;
                var8 = var29;
                let var30 = var10;
                let var31 = self.memory.load64(arg3 as usize + 32) as i64;
                var10 = var31;
                var7 = var8.wrapping_sub(var10).wrapping_sub(((var10 as u64) < var9 as u64) as i32 as u32 as i64);
                if ((var8 ^ var30) & (var8 ^ var7) < False) as i32 != 0 {
                    break 'label1;
                }
                var8 = var10.wrapping_sub(var9);
                self.func71(imports, var6.wrapping_add(48i32), var8, var7, 10000i64, False);
                self.memory.store32(var6 as usize + 44, 0i32 as u32);
                let var33 = self.memory.load64(var6 as usize + 48) as i64;
                var11 = var33;
                let var34 = self.memory.load64(var6.wrapping_add(56i32) as usize) as i64;
                var12 = var34;
                self.func68(imports, var6.wrapping_add(24i32), var11, var12, arg4 as u32 as i64, False, var6.wrapping_add(44i32));
                let var36 = self.memory.load32(var6 as usize + 44) as i32;
                if var36 != 0 {
                    break 'label1;
                }
                let var37 = self.memory.load64(var6.wrapping_add(32i32) as usize) as i64;
                var9 = var37;
                let var38 = self.memory.load64(var6 as usize + 24) as i64;
                var10 = var38;
                self.memory.store32(var6 as usize + 20, 0i32 as u32);
                self.func68(imports, var6, var11, var12, arg5 as u32 as i64, False, var6.wrapping_add(20i32));
                let var40 = self.memory.load32(var6 as usize + 20) as i32;
                if var40 != 0 {
                    break 'label1;
                }
                var11 = var7.wrapping_sub(var9).wrapping_sub(((var8 as u64) < var10 as u64) as i32 as u32 as i64);
                if ((var7 ^ var9) & (var7 ^ var11) < False) as i32 != 0 {
                    break 'label1;
                }
                let var41 = self.memory.load64(var6.wrapping_add(8i32) as usize) as i64;
                var12 = var41;
                var14 = var8.wrapping_sub(var10);
                let var42 = self.memory.load64(var6 as usize) as i64;
                var13 = var42;
                var15 = var11.wrapping_sub(var12).wrapping_sub(((var14 as u64) < var13 as u64) as i32 as u32 as i64);
                if ((var11 ^ var12) & (var11 ^ var15) < False) as i32 != 0 {
                    break 'label1;
                }
                self.memory.store64(arg0 as usize + 64, var13 as u64);
                self.memory.store64(arg0 as usize + 48, var10 as u64);
                self.memory.store64(arg0 as usize + 32, var14.wrapping_sub(var13) as u64);
                self.memory.store64(arg0 as usize + 16, var8 as u64);
                self.memory.store64(arg0 as usize, arg1 as u64);
                self.memory.store64(arg0.wrapping_add(72i32) as usize, var12 as u64);
                self.memory.store64(arg0.wrapping_add(56i32) as usize, var9 as u64);
                self.memory.store64(arg0.wrapping_add(40i32) as usize, var15 as u64);
                self.memory.store64(arg0.wrapping_add(24i32) as usize, var7 as u64);
                self.memory.store64(arg0 as usize + 8, arg2 as u64);
                break 'label0;
                break;
            }
            unreachable!();
            break;
        }
        self.global0 = var6.wrapping_add(80i32);
    }
    fn func80<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64, mut arg7: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func42(imports, var8.wrapping_add(24i32), arg1);
            let var17 = self.memory.load64(var8 as usize + 24) as i64;
            if var17 as i32 | (arg2 & 255i64 != U32(0)) as i32 | (arg3 & 255i64 != U32(0)) as i32 | (arg4 & 255i64 != Address(obj#0)) as i32 | (arg5 & 255i64 != Address(obj#0)) as i32 | (arg6 & 255i64 != Address(obj#0)) as i32 | (arg7 & 255i64 != Map(obj#0)) as i32 != 0 {
                break 'label0;
            }
            let var18 = self.memory.load64(var8 as usize + 32) as i64;
            var13 = var18;
            let var19 = self.func54(imports, 1049208i32);
            if (var19 == 0) as i32 != 0 {
                let var20 = imports.map_new(self);
                var10 = var20;
                let var21 = imports.map_len(self, arg7);
                var14 = (var21 as u64).wrapping_shr(32i64 as u32) as i64;
                var9 = U32(0);
                var1 = False;
                'label1: loop {
                    if ((arg1 as u64) < var14 as u64) as i32 != 0 {
                        let var22 = imports.map_key_by_pos(self, arg7, var9);
                        var12 = var22;
                        let var23 = imports.map_val_by_pos(self, arg7, var9);
                        var11 = var23;
                        if (arg1 == 4294967295i64) as i32 | (var12 & 255i64 != U32(0)) as i32 != 0 {
                            break 'label0;
                        }
                        self.func42(imports, var8.wrapping_add(8i32), var11);
                        let var25 = self.memory.load64(var8 as usize + 8) as i64;
                        if var25 as i32 != 0 {
                            break 'label0;
                        }
                        let var26 = self.memory.load64(var8 as usize + 16) as i64;
                        let var27 = self.func61(imports, var26);
                        var11 = var27;
                        let var28 = self.func52(imports, False, False);
                        self.memory.store64(var8 as usize + 48, var28 as u64);
                        self.memory.store64(var8 as usize + 40, var11 as u64);
                        var9 = var9.wrapping_add(4294967296i64);
                        var1 = arg1.wrapping_add(True);
                        let var29 = self.func62(imports, 1049192i32, 2i32, var8.wrapping_add(40i32), 2i32);
                        let var30 = imports.map_put(self, var10, var12 & -4294967296i64 | U32(0), var29);
                        var10 = var30;
                        continue 'label1;
                    }
                    break;
                }
                self.memory.store64(var8 as usize + 40, arg0 as u64);
                self.memory.store64(var8 as usize + 80, var10 as u64);
                self.memory.store64(var8 as usize + 72, arg6 as u64);
                self.memory.store64(var8 as usize + 64, arg5 as u64);
                self.memory.store64(var8 as usize + 56, arg4 as u64);
                self.memory.store32(var8 as usize + 92, (arg3 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store32(var8 as usize + 88, (arg2 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
                self.memory.store64(var8 as usize + 48, var13 as u64);
                self.func55(imports, var8.wrapping_add(40i32));
                let var32 = self.func75(imports, 1048576i32, 11i32);
                let var33 = self.func63(imports, var32);
                let var34 = imports.contract_event(self, var33, True);
                var34;
                self.global0 = var8.wrapping_add(96i32);
                return Void;
            }
            self.func81(imports, Error(Contract, #1));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func81<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) {
        let var1 = imports.fail_with_error(self, arg0);
        var1;
    }
    fn func82<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func42(imports, var6.wrapping_add(24i32), arg1);
            let var18 = self.memory.load64(var6 as usize + 24) as i64;
            if var18 as i32 | (arg2 & 255i64 != U32(0)) as i32 | (arg3 & 255i64 != U32(0)) as i32 | (arg4 & 255i64 != Address(obj#0)) as i32 | (arg5 & 255i64 != Map(obj#0)) as i32 != 0 {
                break 'label0;
            }
            let var19 = self.memory.load64(var6 as usize + 32) as i64;
            var14 = var19;
            self.func53(imports, var6.wrapping_add(96i32));
            let var21 = self.memory.load64(var6 as usize + 96) as i64;
            if (var21 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var23 = self.func83(imports, var6.wrapping_add(40i32), var6.wrapping_add(104i32), 56i32);
            var23;
            let var24 = self.memory.load64(var6 as usize + 40) as i64;
            let var25 = imports.require_auth(self, var24);
            var25;
            let var26 = self.memory.load64(var6 as usize + 80) as i64;
            var8 = var26;
            let var27 = imports.map_len(self, arg5);
            var15 = (var27 as u64).wrapping_shr(32i64 as u32) as i64;
            var7 = var6.wrapping_add(112i32);
            var11 = U32(0);
            var1 = False;
            'label1: loop {
                if ((arg1 as u64) < var15 as u64) as i32 != 0 {
                    let var28 = imports.map_key_by_pos(self, arg5, var11);
                    var9 = var28;
                    let var29 = imports.map_val_by_pos(self, arg5, var11);
                    var10 = var29;
                    if (arg1 == 4294967295i64) as i32 | (var9 & 255i64 != U32(0)) as i32 != 0 {
                        break 'label0;
                    }
                    self.func42(imports, var6.wrapping_add(8i32), var10);
                    let var31 = self.memory.load64(var6 as usize + 8) as i64;
                    if var31 as i32 != 0 {
                        break 'label0;
                    }
                    let var32 = self.memory.load64(var6 as usize + 16) as i64;
                    var10 = False;
                    var13 = False;
                    var9 = var9 & -4294967296i64 | U32(0);
                    let var33 = imports.map_has(self, var8, var9);
                    if (var33 == True) as i32 != 0 {
                        let var34 = imports.map_get(self, var8, var9);
                        self.func73(imports, var6.wrapping_add(96i32), var34);
                        let var36 = self.memory.load64(var6 as usize + 96) as i64;
                        if (var36 != False) as i32 != 0 {
                            break 'label0;
                        }
                        let var37 = self.memory.load64(var7 as usize) as i64;
                        var13 = var37;
                        let var38 = self.memory.load64(var6 as usize + 104) as i64;
                        var10 = var38;
                    }
                    let var39 = self.func61(imports, var32);
                    var12 = var39;
                    let var40 = self.func52(imports, var10, var13);
                    self.memory.store64(var6 as usize + 104, var40 as u64);
                    self.memory.store64(var6 as usize + 96, var12 as u64);
                    var11 = var11.wrapping_add(4294967296i64);
                    var1 = arg1.wrapping_add(True);
                    let var41 = self.func62(imports, 1049192i32, 2i32, var6.wrapping_add(96i32), 2i32);
                    let var42 = imports.map_put(self, var8, var9, var41);
                    var8 = var42;
                    continue 'label1;
                }
                break;
            }
            self.memory.store64(var6 as usize + 40, arg0 as u64);
            self.memory.store64(var6 as usize + 80, var8 as u64);
            self.memory.store64(var6 as usize + 56, arg4 as u64);
            self.memory.store32(var6 as usize + 92, (arg3 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
            self.memory.store32(var6 as usize + 88, (arg2 as u64).wrapping_shr(32i64 as u32) as i64 as u32);
            self.memory.store64(var6 as usize + 48, var14 as u64);
            self.func55(imports, var6.wrapping_add(40i32));
            self.global0 = var6.wrapping_add(160i32);
            return Void;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func83<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        'label0: loop {
            if ((arg2 as u32) < 16i32 as u32) as i32 != 0 {
                var3 = arg0;
                break 'label0;
            }
            var6 = 0i32.wrapping_sub(arg0) & 3i32;
            var4 = arg0.wrapping_add(var6);
            var5 = arg1;
            var3 = arg0;
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
            var8 = arg2.wrapping_sub(var6);
            var9 = var8 & -4i32;
            var3 = var4.wrapping_add(var9);
            'label2: loop {
                var5 = arg1.wrapping_add(var6);
                if (var5 & 3i32 == 0) as i32 != 0 {
                    var1 = var5;
                    'label3: loop {
                        if (var3 as u32 <= var4 as u32) as i32 != 0 {
                            break 'label2;
                        }
                        let var11 = self.memory.load32(arg1 as usize) as i32;
                        self.memory.store32(var4 as usize, var11 as u32);
                        var1 = arg1.wrapping_add(4i32);
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
                let var12 = self.memory.load32(arg2 as usize) as i32;
                var2 = var12;
                'label4: loop {
                    if (var3 as u32 <= var4 as u32) as i32 != 0 {
                        break 'label2;
                    }
                    if var6 != 0 {
                        let arg13 = arg2;
                        let var14 = self.memory.load32(arg1 as usize) as i32;
                        var2 = var14;
                        self.memory.store32(var4 as usize, ((arg13 as u32).wrapping_shr(var6 as u32) as i32 | var2.wrapping_shl(var7 as u32)) as u32);
                        var1 = arg1.wrapping_add(4i32);
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
        var2 = arg2.wrapping_add(var3);
        'label5: loop {
            if (arg2 as u32 > var3 as u32) as i32 != 0 {
                let var15 = self.memory.load8(arg1 as usize) as i32;
                self.memory.store8(var3 as usize, var15 as u8);
                var1 = arg1.wrapping_add(1i32);
                var3 = var3.wrapping_add(1i32);
                continue 'label5;
            }
            break;
        }
        arg0
    }
    fn func84<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_add(-64i32);
        self.global0 = var1;
        'label0: loop {
            'label1: loop {
                if (arg0 & 255i64 != Bytes(obj#0)) as i32 != 0 {
                    break 'label1;
                }
                let var3 = imports.bytes_len(self, arg0);
                if (var3 & -4294967296i64 != 137438953472i64) as i32 != 0 {
                    break 'label1;
                }
                self.func53(imports, var1);
                let var5 = self.memory.load64(var1 as usize) as i64;
                if (var5 != False) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, Error(Contract, #0));
                break;
            }
            unreachable!();
            break;
        }
        let var7 = self.memory.load64(var1 as usize + 8) as i64;
        let var8 = imports.require_auth(self, var7);
        var8;
        let var9 = imports.update_current_contract_wasm(self, arg0);
        var9;
        self.global0 = var1.wrapping_sub(-64i32);
        Void
    }
    fn func85<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var4.wrapping_add(80i32), arg1);
            let var17 = self.memory.load64(var4 as usize + 80) as i64;
            if ((var17 == 0) as i32 == 0) as i32 | (arg2 & 255i64 != U32(0)) as i32 != 0 {
                break 'label0;
            }
            let var18 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
            var7 = var18;
            let var19 = self.memory.load64(var4 as usize + 88) as i64;
            var10 = var19;
            self.func46(imports, var4.wrapping_add(80i32), arg3);
            let var21 = self.memory.load64(var4 as usize + 80) as i64;
            if ((var21 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var22 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
            var8 = var22;
            let var23 = self.memory.load64(var4 as usize + 88) as i64;
            var9 = var23;
            self.func53(imports, var4.wrapping_add(80i32));
            let var25 = self.memory.load64(var4 as usize + 80) as i64;
            if (var25 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var27 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(88i32), 56i32);
            var27;
            let var28 = imports.require_auth(self, arg0);
            var28;
            self.memory.store32(var4 as usize + 20, 0i32 as u32);
            self.func68(imports, var4, var10, var7, 10000000i64, False, var4.wrapping_add(20i32));
            let var30 = self.memory.load32(var4 as usize + 20) as i32;
            if var30 != 0 {
                break 'label0;
            }
            let var31 = self.memory.load64(var4 as usize) as i64;
            var3 = var31;
            let var32 = self.memory.load64(var4.wrapping_add(8i32) as usize) as i64;
            var1 = var32;
            if (({ let a = (var3 == 0) as i32; let b = (var1 < False) as i32; if (arg1 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var33 = self.memory.load64(var4 as usize + 48) as i64;
                var11 = var33;
                let var34 = self.func75(imports, 1049280i32, 4i32);
                var12 = var34;
                let var35 = self.func52(imports, arg3, arg1);
                self.memory.store64(var4 as usize + 152, var35 as u64);
                self.memory.store64(var4 as usize + 144, arg0 as u64);
                'label1: loop {
                    if (var5 == 16i32) as i32 != 0 {
                        var5 = 0i32;
                        'label2: loop {
                            if (var5 != 16i32) as i32 != 0 {
                                let var36 = self.memory.load64(var4.wrapping_add(144i32).wrapping_add(var5) as usize) as i64;
                                self.memory.store64(var4.wrapping_add(80i32).wrapping_add(var5) as usize, var36 as u64);
                                var5 = var5.wrapping_add(8i32);
                                continue 'label2;
                            }
                            break;
                        }
                        let var37 = self.func59(imports, var4.wrapping_add(80i32), 2i32);
                        self.func76(imports, var11, var12, var37);
                    } else {
                        self.memory.store64(var4.wrapping_add(80i32).wrapping_add(var5) as usize, Void as u64);
                        var5 = var5.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
            }
            'label3: loop {
                'label4: loop {
                    let var39 = self.memory.load64(var4 as usize + 64) as i64;
                    var11 = var39;
                    var12 = arg2 & -4294967296i64 | U32(0);
                    let var40 = imports.map_has(self, var11, var12);
                    if (var40 == True) as i32 != 0 {
                        let var41 = imports.map_get(self, var11, var12);
                        self.func73(imports, var4.wrapping_add(80i32), var41);
                        let var43 = self.memory.load64(var4 as usize + 80) as i64;
                        if ((var43 == 0) as i32 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        let var44 = self.memory.load64(var4.wrapping_add(104i32) as usize) as i64;
                        var13 = var44;
                        var5 = (var7 == 0) as i32;
                        if ({ let a = (var10 as u64 > var13 as u64) as i32; let b = (var7 > False) as i32; if var5 != 0 { a } else { b } }) != 0 {
                            break 'label4;
                        }
                        let var45 = self.memory.load64(var4 as usize + 88) as i64;
                        var3 = var45;
                        let var46 = self.memory.load64(var4.wrapping_add(96i32) as usize) as i64;
                        var1 = var46;
                        if ({ let a = (var9 as u64 > var3 as u64) as i32; let b = (var1 < var8) as i32; if (arg1 == var8) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label3;
                        }
                        if (arg1 | arg3 == 0) as i32 != 0 {
                            self.func81(imports, Error(Contract, #303));
                            break 'label0;
                        }
                        let var48 = imports.prng_u64_in_inclusive_range(self, True, var13);
                        var5 = { let a = (var48 as u64 <= var10 as u64) as i32; let b = (var7 >= False) as i32; if var5 != 0 { a } else { b } };
                        var8 = { let a = arg1; let b = False; if var5 != 0 { a } else { b } };
                        var9 = { let a = arg3; let b = True; if var5 != 0 { a } else { b } };
                        var14 = arg1.wrapping_sub(var8).wrapping_sub(((arg3 as u64) < var9 as u64) as i32 as u32 as i64);
                        if ((arg1 ^ var8) & (arg1 ^ var14) < False) as i32 != 0 {
                            break 'label0;
                        }
                        let var49 = self.func61(imports, var13);
                        var1 = var49;
                        let var50 = self.func52(imports, arg3.wrapping_sub(var9), var14);
                        self.memory.store64(var4 as usize + 88, var50 as u64);
                        self.memory.store64(var4 as usize + 80, arg1 as u64);
                        var6 = var4.wrapping_add(80i32);
                        let var51 = self.func62(imports, 1049192i32, 2i32, var6, 2i32);
                        let var52 = imports.map_put(self, var11, var12, var51);
                        self.memory.store64(var4 as usize + 64, var52 as u64);
                        self.func55(imports, var4.wrapping_add(24i32));
                        let var54 = imports.get_current_contract_address(self);
                        var1 = var54;
                        let var55 = self.memory.load64(var4 as usize + 56) as i64;
                        self.func74(imports, var55, arg1, arg0, var9, var8);
                        let var57 = self.func75(imports, 1048649i32, 13i32);
                        let var58 = self.func63(imports, var57);
                        let var59 = self.func52(imports, var10, var7);
                        var3 = var59;
                        let var60 = self.func52(imports, var9, var8);
                        self.memory.store64(var4 as usize + 104, var60 as u64);
                        var7 = var5 as u32 as i64;
                        self.memory.store64(var4 as usize + 96, var7 as u64);
                        self.memory.store64(var4 as usize + 88, arg3 as u64);
                        self.memory.store64(var4 as usize + 80, (arg2 & -4294967296i64 | U32(0)) as u64);
                        let var61 = self.func59(imports, var6, 4i32);
                        self.memory.store64(var4 as usize + 152, var61 as u64);
                        self.memory.store64(var4 as usize + 144, arg0 as u64);
                        let var62 = self.func59(imports, var4.wrapping_add(144i32), 2i32);
                        let var63 = imports.contract_event(self, var58, var62);
                        var63;
                        let var64 = self.func52(imports, var9, var8);
                        self.memory.store64(var4 as usize + 88, var64 as u64);
                        self.memory.store64(var4 as usize + 80, var7 as u64);
                        let var65 = self.func59(imports, var6, 2i32);
                        self.global0 = var4.wrapping_add(160i32);
                        return var65;
                    }
                    self.func81(imports, Error(Contract, #300));
                    break 'label0;
                    break;
                }
                self.func81(imports, Error(Contract, #301));
                break 'label0;
                break;
            }
            self.func81(imports, Error(Contract, #302));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func86<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var3.wrapping_sub(-64i32), arg1);
            let var19 = self.memory.load64(var3 as usize + 64) as i64;
            if ((var19 == 0) as i32 == 0) as i32 | (arg2 & 255i64 != U32(0)) as i32 != 0 {
                break 'label0;
            }
            let var20 = self.memory.load64(var3.wrapping_add(80i32) as usize) as i64;
            var8 = var20;
            let var21 = self.memory.load64(var3 as usize + 72) as i64;
            var11 = var21;
            self.func53(imports, var3.wrapping_sub(-64i32));
            let var23 = self.memory.load64(var3 as usize + 64) as i64;
            if (var23 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            var4 = (arg2 as u64).wrapping_shr(32i64 as u32) as i64 as i32;
            let var25 = self.func83(imports, var3.wrapping_add(8i32), var3.wrapping_add(72i32), 56i32);
            var25;
            let var26 = imports.require_auth(self, arg0);
            var26;
            var12 = arg2 & -4294967296i64 | U32(0);
            let var27 = self.memory.load64(var3 as usize + 48) as i64;
            var9 = var27;
            let var28 = imports.map_len(self, var9);
            var13 = (var28 as u64).wrapping_shr(32i64 as u32) as i64;
            let var29 = self.memory.load64(var3 as usize + 40) as i64;
            var14 = var29;
            var5 = var3.wrapping_add(80i32);
            var1 = U32(0);
            var2 = False;
            var10 = var9;
            'label1: loop {
                'label2: loop {
                    if ((arg2 as u64) < var13 as u64) as i32 != 0 {
                        let var30 = imports.map_key_by_pos(self, var9, arg1);
                        var6 = var30;
                        let var31 = imports.map_val_by_pos(self, var9, arg1);
                        var7 = var31;
                        if (arg2 == 4294967295i64) as i32 | (var6 & 255i64 != U32(0)) as i32 != 0 {
                            break 'label0;
                        }
                        self.func73(imports, var3.wrapping_sub(-64i32), var7);
                        let var33 = self.memory.load64(var3 as usize + 64) as i64;
                        if ((var33 == 0) as i32 == 0) as i32 != 0 {
                            break 'label0;
                        }
                        if (((var6 as u64).wrapping_shr(32i64 as u32) as i64 as i32) != var4) as i32 != 0 {
                            break 'label2;
                        }
                        let var34 = self.memory.load64(var5 as usize) as i64;
                        var6 = var34;
                        let var35 = self.memory.load64(var3 as usize + 72) as i64;
                        var7 = var35;
                        let var36 = self.memory.load64(var3 as usize + 88) as i64;
                        let var37 = imports.get_current_contract_address(self);
                        self.func74(imports, var14, arg0, var37, var11, var8);
                        var16 = var7.wrapping_add(var11);
                        var7 = ((var7 as u64 > var16 as u64) as i32 as u32 as i64).wrapping_add(var6.wrapping_add(var8));
                        if ((var6 ^ var8 ^ -1i64) & (var6 ^ var7) < False) as i32 != 0 {
                            break 'label0;
                        }
                        let var39 = self.func61(imports, var36);
                        var6 = var39;
                        let var40 = self.func52(imports, var16, var7);
                        self.memory.store64(var3 as usize + 72, var40 as u64);
                        self.memory.store64(var3 as usize + 64, var6 as u64);
                        let var41 = self.func62(imports, 1049192i32, 2i32, var3.wrapping_sub(-64i32), 2i32);
                        let var42 = imports.map_put(self, var10, var12, var41);
                        var10 = var42;
                        break 'label2;
                    }
                    self.memory.store64(var3 as usize + 48, var10 as u64);
                    self.func55(imports, var3.wrapping_add(8i32));
                    self.global0 = var3.wrapping_add(128i32);
                    return Void;
                    break;
                }
                var1 = arg1.wrapping_add(4294967296i64);
                var2 = arg2.wrapping_add(True);
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
    fn func87<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64, mut arg5: i64, mut arg6: i64, mut arg7: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 | (arg1 & 255i64 != Address(obj#0)) as i32 | (arg2 & 255i64 != String(obj#0)) as i32 | (arg3 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func46(imports, var8.wrapping_add(248i32), arg4);
            let var20 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var20 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var9 = var8.wrapping_add(264i32);
            let var21 = self.memory.load64(var9 as usize) as i64;
            var4 = var21;
            let var22 = self.memory.load64(var8 as usize + 256) as i64;
            var14 = var22;
            self.func46(imports, var8.wrapping_add(248i32), arg5);
            let var24 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var24 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var25 = self.memory.load64(var9 as usize) as i64;
            var15 = var25;
            let var26 = self.memory.load64(var8 as usize + 256) as i64;
            var16 = var26;
            self.func46(imports, var8.wrapping_add(248i32), arg6);
            let var28 = self.memory.load64(var8 as usize + 248) as i64;
            if ((var28 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var29 = self.memory.load64(var8.wrapping_add(264i32) as usize) as i64;
            var5 = var29;
            let var30 = self.memory.load64(var8 as usize + 256) as i64;
            var6 = var30;
            self.func42(imports, var8.wrapping_add(32i32), arg7);
            let var32 = self.memory.load32(var8 as usize + 32) as i32;
            if var32 != 0 {
                break 'label0;
            }
            let var33 = self.memory.load64(var8 as usize + 40) as i64;
            var7 = var33;
            let var34 = self.func54(imports, 1049208i32);
            if (var34 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var36 = imports.require_auth(self, arg0);
            var36;
            let var37 = self.func66(imports);
            var17 = var37;
            if (arg4 | var14 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #103));
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    if (({ let a = (var14 < False) as i32; let b = (arg4 > False) as i32; if (arg4 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                        self.memory.store32(var8 as usize + 28, 0i32 as u32);
                        self.func68(imports, var8.wrapping_add(8i32), var14, arg4, arg6, arg5, var8.wrapping_add(28i32));
                        let var40 = self.memory.load32(var8 as usize + 28) as i32;
                        if var40 != 0 {
                            break 'label0;
                        }
                        let var41 = self.memory.load64(var8.wrapping_add(16i32) as usize) as i64;
                        if (var41 > 4999999i64) as i32 != 0 {
                            break 'label2;
                        }
                        if (var15 | var16 == 0) as i32 != 0 {
                            self.func81(imports, Error(Contract, #100));
                            break 'label0;
                        }
                        if (arg5 | arg6 == 0) as i32 != 0 {
                            self.func81(imports, Error(Contract, #101));
                            break 'label0;
                        }
                        if ({ let a = ((arg6 as u64) < var16 as u64) as i32; let b = (arg5 < var15) as i32; if (arg5 == var15) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label1;
                        }
                        self.memory.store64(var8 as usize + 264, arg7 as u64);
                        self.memory.store64(var8 as usize + 256, arg0 as u64);
                        self.memory.store64(var8 as usize + 248, True as u64);
                        let var44 = self.func54(imports, var8.wrapping_add(248i32));
                        if var44 != 0 {
                            self.func81(imports, Error(Contract, #201));
                            break 'label0;
                        }
                        let var46 = imports.get_current_contract_address(self);
                        self.func74(imports, arg3, arg0, var46, var14, arg4);
                        var10 = var8.wrapping_add(184i32);
                        self.memory.store64(var10 as usize, arg3 as u64);
                        var11 = var8.wrapping_add(176i32);
                        self.memory.store64(var11 as usize, arg2 as u64);
                        var12 = var8.wrapping_add(192i32);
                        self.memory.store64(var12 as usize, False as u64);
                        self.memory.store64(var8 as usize + 56, arg4 as u64);
                        self.memory.store64(var8 as usize + 48, var14 as u64);
                        self.memory.store64(var8 as usize + 168, arg1 as u64);
                        self.memory.store64(var8 as usize + 160, arg0 as u64);
                        self.memory.store16(var8 as usize + 208, 0i32 as u16);
                        self.func78(imports, var8.wrapping_sub(-64i32), 64i32);
                        self.memory.store64(var8.wrapping_add(152i32) as usize, arg5 as u64);
                        self.memory.store64(var8.wrapping_add(136i32) as usize, var15 as u64);
                        self.memory.store64(var8 as usize + 144, arg6 as u64);
                        self.memory.store64(var8 as usize + 128, var16 as u64);
                        self.memory.store64(var8 as usize + 200, var17 as u64);
                        self.memory.store64(var8 as usize + 264, arg7 as u64);
                        self.memory.store64(var8 as usize + 256, arg0 as u64);
                        self.memory.store64(var8 as usize + 248, True as u64);
                        var9 = var8.wrapping_add(248i32);
                        var13 = var8.wrapping_add(48i32);
                        self.func49(imports, var9, var13);
                        let var50 = self.func83(imports, var9, var13, 112i32);
                        var50;
                        let var51 = self.memory.load64(var12 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(392i32) as usize, var51 as u64);
                        let var52 = self.memory.load64(var10 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(384i32) as usize, var52 as u64);
                        let var53 = self.memory.load64(var11 as usize) as i64;
                        self.memory.store64(var8.wrapping_add(376i32) as usize, var53 as u64);
                        let var54 = self.memory.load64(var8 as usize + 168) as i64;
                        self.memory.store64(var8 as usize + 368, var54 as u64);
                        let var55 = self.memory.load64(var8 as usize + 208) as i64;
                        var1 = var55;
                        let var56 = self.func75(imports, 1048587i32, 10i32);
                        var2 = var56;
                        self.memory.store64(var8.wrapping_add(240i32) as usize, var17 as u64);
                        self.memory.store64(var8.wrapping_add(232i32) as usize, arg7 as u64);
                        self.memory.store64(var8 as usize + 224, arg0 as u64);
                        self.memory.store64(var8 as usize + 216, arg2 as u64);
                        self.memory.store64(var8 as usize + 408, arg1 as u64);
                        self.memory.store64(var8 as usize + 400, var17 as u64);
                        self.memory.store64(var8 as usize + 360, arg0 as u64);
                        let var57 = self.func60(imports, var8.wrapping_add(216i32));
                        let var58 = self.func50(imports, var9);
                        let var59 = imports.contract_event(self, var57, var58);
                        var59;
                        self.global0 = var8.wrapping_add(416i32);
                        return Void;
                    }
                    self.func81(imports, Error(Contract, #104));
                    break 'label0;
                    break;
                }
                self.func81(imports, Error(Contract, #105));
                break 'label0;
                break;
            }
            self.func81(imports, Error(Contract, #102));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func88<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
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
        self.func57(imports, var1.wrapping_add(8i32), arg0);
        'label0: loop {
            'label1: loop {
                let var14 = self.memory.load64(var1 as usize + 8) as i64;
                if ((var14 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var15 = self.memory.load64(var1.wrapping_add(24i32) as usize) as i64;
                var0 = var15;
                let var16 = self.memory.load64(var1 as usize + 16) as i64;
                var5 = var16;
                let var17 = self.func54(imports, 1049208i32);
                if (var17 == 0) as i32 != 0 {
                    self.func81(imports, Error(Contract, #0));
                    break 'label1;
                }
                self.memory.store64(var1 as usize + 192, arg0 as u64);
                self.memory.store64(var1 as usize + 184, var5 as u64);
                self.memory.store64(var1 as usize + 176, True as u64);
                self.func47(imports, var1.wrapping_add(8i32), var1.wrapping_add(176i32));
                let var20 = self.memory.load8(var1 as usize + 169) as i32;
                if (var20 == 2i32) as i32 != 0 {
                    self.func81(imports, Error(Contract, #200));
                    break 'label1;
                }
                let var22 = self.memory.load64(var1.wrapping_add(16i32) as usize) as i64;
                var6 = var22;
                var2 = var1.wrapping_add(32i32);
                let var23 = self.memory.load64(var2 as usize) as i64;
                var4 = var23;
                let var24 = self.memory.load64(var1 as usize + 8) as i64;
                var7 = var24;
                let var25 = self.memory.load64(var1 as usize + 160) as i64;
                var8 = var25;
                let var26 = self.memory.load64(var1 as usize + 144) as i64;
                var9 = var26;
                let var27 = self.memory.load64(var1 as usize + 24) as i64;
                let var28 = self.memory.load64(var1 as usize + 120) as i64;
                var11 = var28;
                let var29 = imports.require_auth(self, var11);
                var29;
                if ({ let a = (var27 == 0) as i32; let b = (var4 < False) as i32; if (var4 == 0) as i32 != 0 { a } else { b } }) != 0 {
                    break 'label0;
                }
                self.func81(imports, Error(Contract, #201));
                break;
            }
            unreachable!();
            break;
        }
        let var31 = imports.get_current_contract_address(self);
        self.func74(imports, var9, var31, var11, var7, var6);
        self.memory.store64(var1 as usize + 24, arg0 as u64);
        self.memory.store64(var1 as usize + 16, var5 as u64);
        self.memory.store64(var1 as usize + 8, True as u64);
        var3 = var1.wrapping_add(8i32);
        let var33 = self.func44(imports, var3);
        self.func89(imports, var33);
        let var35 = self.func75(imports, 1048617i32, 13i32);
        var4 = var35;
        self.memory.store64(var2 as usize, var8 as u64);
        self.memory.store64(var1.wrapping_add(24i32) as usize, arg0 as u64);
        self.memory.store64(var1 as usize + 16, var5 as u64);
        self.memory.store64(var1 as usize + 8, var4 as u64);
        let var36 = self.func60(imports, var3);
        let var37 = imports.contract_event(self, var36, True);
        var37;
        self.global0 = var1.wrapping_add(208i32);
        Void
    }
    fn func89<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) {
        let var1 = imports.del_contract_data(self, arg0, True);
        var1;
    }
    fn func90<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var4.wrapping_add(280i32), arg1);
            let var30 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var30 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var5 = var4.wrapping_add(296i32);
            let var31 = self.memory.load64(var5 as usize) as i64;
            var12 = var31;
            let var32 = self.memory.load64(var4 as usize + 288) as i64;
            var13 = var32;
            self.func46(imports, var4.wrapping_add(280i32), arg2);
            let var34 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var34 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var35 = self.memory.load64(var5 as usize) as i64;
            var1 = var35;
            let var36 = self.memory.load64(var4 as usize + 288) as i64;
            var9 = var36;
            self.func46(imports, var4.wrapping_add(280i32), arg3);
            let var38 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var38 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var39 = self.memory.load64(var4.wrapping_add(296i32) as usize) as i64;
            var10 = var39;
            let var40 = self.memory.load64(var4 as usize + 288) as i64;
            self.func53(imports, var4.wrapping_add(280i32));
            let var42 = self.memory.load64(var4 as usize + 280) as i64;
            if (var42 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var44 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(288i32), 56i32);
            var44;
            self.memory.store64(var4 as usize + 264, var12 as u64);
            self.memory.store64(var4 as usize + 256, var13 as u64);
            self.memory.store64(var4 as usize + 248, True as u64);
            self.func47(imports, var4.wrapping_add(280i32), var4.wrapping_add(248i32));
            let var46 = self.memory.load8(var4 as usize + 441) as i32;
            if (var46 == 2i32) as i32 != 0 {
                self.func81(imports, Error(Contract, #200));
                break 'label0;
            }
            let var48 = self.func83(imports, var4.wrapping_add(80i32), var4.wrapping_add(280i32), 168i32);
            var48;
            if (arg1 | var9 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #2));
                break 'label0;
            }
            'label1: loop {
                'label2: loop {
                    let var50 = self.memory.load64(var4 as usize + 80) as i64;
                    var25 = var50;
                    let var51 = self.memory.load64(var4 as usize + 96) as i64;
                    var15 = var51;
                    let var52 = self.memory.load64(var4.wrapping_add(88i32) as usize) as i64;
                    var26 = var52;
                    let var53 = self.memory.load64(var4.wrapping_add(104i32) as usize) as i64;
                    var8 = var53;
                    if ((var25 ^ var15 | var26 ^ var8 == 0) as i32 == 0) as i32 != 0 {
                        let var54 = self.func65(imports, var4.wrapping_add(80i32));
                        if var54 != 0 {
                            break 'label2;
                        }
                        let var55 = imports.require_auth(self, arg0);
                        var55;
                        let var56 = self.memory.load32(var4 as usize + 72) as i32;
                        let var57 = self.memory.load32(var4 as usize + 76) as i32;
                        self.func77(imports, var4.wrapping_add(280i32), var9, arg1, var4.wrapping_add(80i32), var56, var57);
                        let var59 = self.memory.load64(var4.wrapping_add(352i32) as usize) as i64;
                        var20 = var59;
                        let var60 = self.memory.load64(var4.wrapping_add(336i32) as usize) as i64;
                        var21 = var60;
                        let var61 = self.memory.load64(var4.wrapping_add(320i32) as usize) as i64;
                        var2 = var61;
                        let var62 = self.memory.load64(var4.wrapping_add(304i32) as usize) as i64;
                        var18 = var62;
                        let var63 = self.memory.load64(var4.wrapping_add(288i32) as usize) as i64;
                        var3 = var63;
                        let var64 = self.memory.load64(var4 as usize + 344) as i64;
                        var22 = var64;
                        let var65 = self.memory.load64(var4 as usize + 328) as i64;
                        var23 = var65;
                        let var66 = self.memory.load64(var4 as usize + 312) as i64;
                        var17 = var66;
                        let var67 = self.memory.load64(var4 as usize + 296) as i64;
                        var24 = var67;
                        let var68 = self.memory.load64(var4 as usize + 280) as i64;
                        var16 = var68;
                        let var69 = imports.get_current_contract_address(self);
                        var19 = var69;
                        let var70 = self.memory.load64(var4 as usize + 56) as i64;
                        var11 = var70;
                        self.func74(imports, var11, arg0, var19, var9, arg1);
                        if (({ let a = (var9 as u64 > var16 as u64) as i32; let b = (arg1 > arg3) as i32; if (arg1 == arg3) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                            break 'label1;
                        }
                        let var72 = imports.get_current_contract_address(self);
                        var19 = var72;
                        var27 = arg1.wrapping_sub(arg3).wrapping_sub(((var9 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((arg1 ^ arg3) & (arg1 ^ var27) < False) as i32 != 0 {
                            break 'label0;
                        }
                        self.func74(imports, var11, var19, arg0, var9.wrapping_sub(var16), var27);
                        break 'label1;
                    }
                    self.func81(imports, Error(Contract, #205));
                    break 'label0;
                    break;
                }
                self.func81(imports, Error(Contract, #202));
                break 'label0;
                break;
            }
            let var76 = imports.get_current_contract_address(self);
            let var77 = self.memory.load64(var4 as usize + 40) as i64;
            self.func74(imports, var11, var76, var77, var22, var20);
            if (({ let a = (var40 as u64 > var17 as u64) as i32; let b = (arg2 < var10) as i32; if (arg2 == var10) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                self.memory.store64(var4 as usize + 304, arg0 as u64);
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, Void as u64);
                self.func43(imports, var4, var4.wrapping_add(280i32));
                let var80 = self.memory.load64(var4.wrapping_add(16i32) as usize) as i64;
                let var81 = self.memory.load32(var4 as usize) as i32;
                var5 = var81;
                var10 = { let a = var80; let b = False; if var5 != 0 { a } else { b } };
                let var82 = self.memory.load64(var4 as usize + 8) as i64;
                var9 = { let a = var82; let b = False; if var5 != 0 { a } else { b } };
                var1 = var9.wrapping_add(var17);
                var9 = (((var1 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(arg2.wrapping_add(var10));
                if ((var10 ^ arg2 ^ -1i64) & (var10 ^ var9) < False) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var4 as usize + 304, arg0 as u64);
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, Void as u64);
                self.func51(imports, var4.wrapping_add(280i32), arg1, var9);
                var10 = var15.wrapping_add(var17);
                var15 = (((var10 as u64) < var15 as u64) as i32 as u32 as i64).wrapping_add(arg2.wrapping_add(var8));
                if ((arg2 ^ var8 ^ -1i64) & (var8 ^ var15) < False) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var4.wrapping_add(104i32) as usize, var15 as u64);
                self.memory.store64(var4 as usize + 96, var10 as u64);
                var5 = var4.wrapping_add(120i32);
                let var84 = self.memory.load64(var5 as usize) as i64;
                var8 = var84;
                let var85 = self.memory.load64(var4 as usize + 112) as i64;
                var11 = var85;
                var14 = var11.wrapping_add(var24);
                var11 = (((var14 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_add(var18));
                if ((var8 ^ var18 ^ -1i64) & (var8 ^ var11) < False) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var5 as usize, var11 as u64);
                self.memory.store64(var4 as usize + 112, var14 as u64);
                var5 = var4.wrapping_add(136i32);
                let var86 = self.memory.load64(var5 as usize) as i64;
                var8 = var86;
                let var87 = self.memory.load64(var4 as usize + 128) as i64;
                var11 = var87;
                var14 = var11.wrapping_add(var16);
                var11 = (((var14 as u64) < var11 as u64) as i32 as u32 as i64).wrapping_add(arg3.wrapping_add(var8));
                if ((var8 ^ arg3 ^ -1i64) & (var8 ^ var11) < False) as i32 != 0 {
                    break 'label0;
                }
                self.memory.store64(var5 as usize, var11 as u64);
                self.memory.store64(var4 as usize + 128, var14 as u64);
                'label3: loop {
                    if ((var10 ^ var25 | var15 ^ var26 == 0) as i32 == 0) as i32 != 0 {
                        break 'label3;
                    }
                    let var88 = self.memory.load8(var4 as usize + 240) as i32;
                    if var88 != 0 {
                        break 'label3;
                    }
                    self.memory.store8(var4 as usize + 240, 1i32 as u8);
                    let var89 = self.func66(imports);
                    var8 = var89;
                    let var90 = self.memory.load64(var4 as usize + 32) as i64;
                    var10 = var8.wrapping_add(var90);
                    if ((var10 as u64) < var8 as u64) as i32 != 0 {
                        break 'label0;
                    }
                    self.memory.store64(var4 as usize + 224, var10 as u64);
                    break;
                }
                self.memory.store64(var4 as usize + 296, var12 as u64);
                self.memory.store64(var4 as usize + 288, var13 as u64);
                self.memory.store64(var4 as usize + 280, True as u64);
                var5 = var4.wrapping_add(280i32);
                var6 = var4.wrapping_add(80i32);
                self.func49(imports, var5, var6);
                let var92 = self.memory.load64(var4 as usize + 64) as i64;
                let var93 = self.func72(imports, var92, var23, var21);
                self.memory.store64(var4 as usize + 64, var93 as u64);
                self.func55(imports, var4.wrapping_add(24i32));
                let var95 = self.memory.load64(var4 as usize + 232) as i64;
                var8 = var95;
                let var96 = self.func83(imports, var4.wrapping_add(384i32), var6, 168i32);
                var6 = var96;
                let var97 = self.func75(imports, 1048630i32, 3i32);
                var10 = var97;
                self.memory.store64(var4.wrapping_add(376i32) as usize, var9 as u64);
                self.memory.store64(var4.wrapping_add(368i32) as usize, arg1 as u64);
                self.memory.store64(var4.wrapping_add(352i32) as usize, var20 as u64);
                self.memory.store64(var4.wrapping_add(336i32) as usize, var21 as u64);
                self.memory.store64(var4.wrapping_add(320i32) as usize, arg2 as u64);
                self.memory.store64(var4.wrapping_add(304i32) as usize, var18 as u64);
                self.memory.store64(var4.wrapping_add(272i32) as usize, var8 as u64);
                self.memory.store64(var4.wrapping_add(264i32) as usize, var12 as u64);
                self.memory.store64(var4 as usize + 344, var22 as u64);
                self.memory.store64(var4 as usize + 328, var23 as u64);
                self.memory.store64(var4 as usize + 312, var17 as u64);
                self.memory.store64(var4 as usize + 296, var24 as u64);
                self.memory.store64(var4 as usize + 288, arg3 as u64);
                self.memory.store64(var4 as usize + 280, var16 as u64);
                self.memory.store64(var4 as usize + 256, var13 as u64);
                self.memory.store64(var4 as usize + 248, var10 as u64);
                self.memory.store64(var4 as usize + 360, arg0 as u64);
                let var98 = self.func60(imports, var4.wrapping_add(248i32));
                let var99 = self.func52(imports, arg1, var9);
                self.memory.store64(var4 as usize + 560, var99 as u64);
                self.memory.store64(var4 as usize + 552, arg0 as u64);
                var7 = var4.wrapping_add(552i32);
                let var100 = self.func59(imports, var7, 2i32);
                var0 = var100;
                let var101 = self.func58(imports, var5);
                var1 = var101;
                let var102 = self.func50(imports, var6);
                self.memory.store64(var4 as usize + 568, var102 as u64);
                self.memory.store64(var4 as usize + 560, arg1 as u64);
                self.memory.store64(var4 as usize + 552, arg0 as u64);
                let var103 = self.func59(imports, var7, 3i32);
                let var104 = imports.contract_event(self, var98, var103);
                var104;
                self.global0 = var4.wrapping_add(576i32);
                return Void;
            }
            self.func81(imports, Error(Contract, #203));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func91<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64, mut arg2: i64, mut arg3: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var4.wrapping_add(280i32), arg1);
            let var25 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var25 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            var5 = var4.wrapping_add(296i32);
            let var26 = self.memory.load64(var5 as usize) as i64;
            var1 = var26;
            let var27 = self.memory.load64(var4 as usize + 288) as i64;
            var14 = var27;
            self.func46(imports, var4.wrapping_add(280i32), arg2);
            let var29 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var29 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var30 = self.memory.load64(var5 as usize) as i64;
            var2 = var30;
            let var31 = self.memory.load64(var4 as usize + 288) as i64;
            var10 = var31;
            self.func46(imports, var4.wrapping_add(280i32), arg3);
            let var33 = self.memory.load64(var4 as usize + 280) as i64;
            if ((var33 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var34 = self.memory.load64(var4.wrapping_add(296i32) as usize) as i64;
            var13 = var34;
            let var35 = self.memory.load64(var4 as usize + 288) as i64;
            var9 = var35;
            self.func53(imports, var4.wrapping_add(280i32));
            let var37 = self.memory.load64(var4 as usize + 280) as i64;
            if (var37 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var39 = self.func83(imports, var4.wrapping_add(24i32), var4.wrapping_add(288i32), 56i32);
            var39;
            self.memory.store64(var4 as usize + 264, arg1 as u64);
            self.memory.store64(var4 as usize + 256, var14 as u64);
            self.memory.store64(var4 as usize + 248, True as u64);
            self.func47(imports, var4.wrapping_add(280i32), var4.wrapping_add(248i32));
            let var41 = self.memory.load8(var4 as usize + 441) as i32;
            if (var41 == 2i32) as i32 != 0 {
                self.func81(imports, Error(Contract, #200));
                break 'label0;
            }
            var5 = var4.wrapping_add(80i32);
            let var43 = self.func83(imports, var5, var4.wrapping_add(280i32), 168i32);
            var43;
            'label1: loop {
                'label2: loop {
                    let var44 = self.func65(imports, var5);
                    if (var44 == 0) as i32 != 0 {
                        if (arg2 | var10 == 0) as i32 != 0 {
                            self.func81(imports, Error(Contract, #2));
                            break 'label0;
                        }
                        let var46 = imports.require_auth(self, arg0);
                        var46;
                        self.memory.store64(var4 as usize + 304, arg0 as u64);
                        self.memory.store64(var4 as usize + 296, arg1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, Void as u64);
                        self.func43(imports, var4, var4.wrapping_add(280i32));
                        let var48 = self.memory.load64(var4 as usize + 8) as i64;
                        let var49 = self.memory.load32(var4 as usize) as i32;
                        var5 = var49;
                        var8 = { let a = var48; let b = False; if var5 != 0 { a } else { b } };
                        let var50 = self.memory.load64(var4.wrapping_add(16i32) as usize) as i64;
                        var11 = { let a = var50; let b = False; if var5 != 0 { a } else { b } };
                        if ({ let a = ((var8 as u64) < var10 as u64) as i32; let b = (var11 < arg2) as i32; if (arg2 == var11) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label2;
                        }
                        let var51 = self.memory.load32(var4 as usize + 72) as i32;
                        let var52 = self.memory.load32(var4 as usize + 76) as i32;
                        self.func79(imports, var4.wrapping_add(280i32), var10, arg2, var4.wrapping_add(80i32), var51, var52);
                        let var54 = self.memory.load64(var4.wrapping_add(336i32) as usize) as i64;
                        var18 = var54;
                        let var55 = self.memory.load64(var4.wrapping_add(304i32) as usize) as i64;
                        var2 = var55;
                        let var56 = self.memory.load64(var4.wrapping_add(288i32) as usize) as i64;
                        var3 = var56;
                        let var57 = self.memory.load64(var4.wrapping_add(320i32) as usize) as i64;
                        var15 = var57;
                        let var58 = self.memory.load64(var4.wrapping_add(352i32) as usize) as i64;
                        var19 = var58;
                        let var59 = self.memory.load64(var4 as usize + 328) as i64;
                        var20 = var59;
                        let var60 = self.memory.load64(var4 as usize + 296) as i64;
                        var16 = var60;
                        let var61 = self.memory.load64(var4 as usize + 280) as i64;
                        var10 = var61;
                        let var62 = self.memory.load64(var4 as usize + 312) as i64;
                        var17 = var62;
                        let var63 = self.memory.load64(var4 as usize + 344) as i64;
                        var21 = var63;
                        let var64 = imports.get_current_contract_address(self);
                        var12 = var64;
                        let var65 = self.memory.load64(var4 as usize + 56) as i64;
                        var22 = var65;
                        let var66 = self.memory.load64(var4 as usize + 40) as i64;
                        self.func74(imports, var22, var12, var66, var21, var19);
                        if ({ let a = (var9 as u64 > var17 as u64) as i32; let b = (var13 > var15) as i32; if (var13 == var15) as i32 != 0 { a } else { b } }) != 0 {
                            break 'label1;
                        }
                        let var68 = imports.get_current_contract_address(self);
                        self.func74(imports, var22, var68, arg0, var17, var15);
                        var13 = var11.wrapping_sub(arg3).wrapping_sub(((var8 as u64) < var10 as u64) as i32 as u32 as i64);
                        if ((arg3 ^ var11) & (var11 ^ var13) < False) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var4 as usize + 304, arg0 as u64);
                        self.memory.store64(var4 as usize + 296, arg1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, Void as u64);
                        var11 = var8.wrapping_sub(var10);
                        self.func51(imports, var4.wrapping_add(280i32), var11, var13);
                        var5 = var4.wrapping_add(104i32);
                        let var71 = self.memory.load64(var5 as usize) as i64;
                        var8 = var71;
                        let var72 = self.memory.load64(var4 as usize + 96) as i64;
                        var9 = var72;
                        var12 = var8.wrapping_sub(arg3).wrapping_sub(((var9 as u64) < var10 as u64) as i32 as u32 as i64);
                        if ((var8 ^ arg3) & (var8 ^ var12) < False) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var12 as u64);
                        self.memory.store64(var4 as usize + 96, var9.wrapping_sub(var10) as u64);
                        var5 = var4.wrapping_add(120i32);
                        let var73 = self.memory.load64(var5 as usize) as i64;
                        var8 = var73;
                        let var74 = self.memory.load64(var4 as usize + 112) as i64;
                        var9 = var74;
                        var12 = var8.wrapping_sub(arg2).wrapping_sub(((var9 as u64) < var16 as u64) as i32 as u32 as i64);
                        if ((var8 ^ arg2) & (var8 ^ var12) < False) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var12 as u64);
                        self.memory.store64(var4 as usize + 112, var9.wrapping_sub(var16) as u64);
                        var5 = var4.wrapping_add(136i32);
                        let var75 = self.memory.load64(var5 as usize) as i64;
                        var8 = var75;
                        let var76 = self.memory.load64(var4 as usize + 128) as i64;
                        var9 = var76;
                        var12 = var9.wrapping_add(var16);
                        var9 = (((var12 as u64) < var9 as u64) as i32 as u32 as i64).wrapping_add(arg2.wrapping_add(var8));
                        if ((var8 ^ arg2 ^ -1i64) & (var8 ^ var9) < False) as i32 != 0 {
                            break 'label0;
                        }
                        self.memory.store64(var5 as usize, var9 as u64);
                        self.memory.store64(var4 as usize + 128, var12 as u64);
                        self.memory.store64(var4 as usize + 296, arg1 as u64);
                        self.memory.store64(var4 as usize + 288, var14 as u64);
                        self.memory.store64(var4 as usize + 280, True as u64);
                        var5 = var4.wrapping_add(280i32);
                        var6 = var4.wrapping_add(80i32);
                        self.func49(imports, var5, var6);
                        let var78 = self.memory.load64(var4 as usize + 64) as i64;
                        let var79 = self.func72(imports, var78, var20, var18);
                        self.memory.store64(var4 as usize + 64, var79 as u64);
                        self.func55(imports, var4.wrapping_add(24i32));
                        let var81 = self.memory.load64(var4 as usize + 232) as i64;
                        var8 = var81;
                        let var82 = self.func83(imports, var4.wrapping_add(384i32), var6, 168i32);
                        var6 = var82;
                        let var83 = self.func75(imports, 1048633i32, 4i32);
                        var9 = var83;
                        self.memory.store64(var4.wrapping_add(376i32) as usize, var13 as u64);
                        self.memory.store64(var4.wrapping_add(368i32) as usize, var11 as u64);
                        self.memory.store64(var4.wrapping_add(352i32) as usize, var19 as u64);
                        self.memory.store64(var4.wrapping_add(336i32) as usize, var18 as u64);
                        self.memory.store64(var4.wrapping_add(320i32) as usize, var15 as u64);
                        self.memory.store64(var4.wrapping_add(304i32) as usize, arg2 as u64);
                        self.memory.store64(var4.wrapping_add(272i32) as usize, var8 as u64);
                        self.memory.store64(var4.wrapping_add(264i32) as usize, arg1 as u64);
                        self.memory.store64(var4 as usize + 344, var21 as u64);
                        self.memory.store64(var4 as usize + 328, var20 as u64);
                        self.memory.store64(var4 as usize + 312, var17 as u64);
                        self.memory.store64(var4 as usize + 296, var16 as u64);
                        self.memory.store64(var4 as usize + 288, arg3 as u64);
                        self.memory.store64(var4 as usize + 280, var10 as u64);
                        self.memory.store64(var4 as usize + 256, var14 as u64);
                        self.memory.store64(var4 as usize + 248, var9 as u64);
                        self.memory.store64(var4 as usize + 360, arg0 as u64);
                        let var84 = self.func60(imports, var4.wrapping_add(248i32));
                        let var85 = self.func52(imports, var11, var13);
                        self.memory.store64(var4 as usize + 560, var85 as u64);
                        self.memory.store64(var4 as usize + 552, arg0 as u64);
                        var7 = var4.wrapping_add(552i32);
                        let var86 = self.func59(imports, var7, 2i32);
                        var0 = var86;
                        let var87 = self.func58(imports, var5);
                        var2 = var87;
                        let var88 = self.func50(imports, var6);
                        self.memory.store64(var4 as usize + 568, var88 as u64);
                        self.memory.store64(var4 as usize + 560, arg2 as u64);
                        self.memory.store64(var4 as usize + 552, arg0 as u64);
                        let var89 = self.func59(imports, var7, 3i32);
                        let var90 = imports.contract_event(self, var84, var89);
                        var90;
                        self.global0 = var4.wrapping_add(576i32);
                        return Void;
                    }
                    self.func81(imports, Error(Contract, #202));
                    break 'label0;
                    break;
                }
                self.func81(imports, Error(Contract, #204));
                break 'label0;
                break;
            }
            self.func81(imports, Error(Contract, #203));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func92<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i64 = 0;
        let mut var4: i64 = 0;
        let mut var5: i64 = 0;
        let var6 = self.global0;
        var1 = var6.wrapping_sub(368i32);
        self.global0 = var1;
        self.func57(imports, var1.wrapping_add(168i32), arg0);
        'label0: loop {
            let var8 = self.memory.load64(var1 as usize + 168) as i64;
            if ((var8 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var9 = self.memory.load64(var1.wrapping_add(184i32) as usize) as i64;
            var0 = var9;
            let var10 = self.memory.load64(var1 as usize + 176) as i64;
            var3 = var10;
            self.func53(imports, var1.wrapping_add(168i32));
            let var12 = self.memory.load64(var1 as usize + 168) as i64;
            if (var12 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var14 = self.memory.load64(var1.wrapping_add(208i32) as usize) as i64;
            var4 = var14;
            self.memory.store64(var1 as usize + 352, arg0 as u64);
            self.memory.store64(var1 as usize + 344, var3 as u64);
            self.memory.store64(var1 as usize + 336, True as u64);
            self.func47(imports, var1.wrapping_add(168i32), var1.wrapping_add(336i32));
            let var16 = self.memory.load8(var1 as usize + 329) as i32;
            if (var16 == 2i32) as i32 != 0 {
                self.func81(imports, Error(Contract, #200));
                break 'label0;
            }
            let var18 = self.func83(imports, var1, var1.wrapping_add(168i32), 168i32);
            var1 = var18;
            let var19 = self.memory.load8(var1 as usize + 161) as i32;
            if (var19 == 0) as i32 != 0 {
                let var20 = self.func65(imports, var1);
                if (var20 == 0) as i32 != 0 {
                    self.func81(imports, Error(Contract, #201));
                    break 'label0;
                }
                let var22 = imports.get_current_contract_address(self);
                let var23 = self.memory.load64(var1 as usize + 120) as i64;
                let var24 = self.memory.load64(var1 as usize + 32) as i64;
                let var25 = self.memory.load64(var1.wrapping_add(40i32) as usize) as i64;
                self.func74(imports, var4, var22, var23, var24, var25);
                self.memory.store8(var1 as usize + 161, 1i32 as u8);
                self.memory.store64(var1 as usize + 184, arg0 as u64);
                self.memory.store64(var1 as usize + 176, var3 as u64);
                self.memory.store64(var1 as usize + 168, True as u64);
                var2 = var1.wrapping_add(168i32);
                self.func49(imports, var2, var1);
                let var28 = self.memory.load64(var1 as usize + 152) as i64;
                var4 = var28;
                let var29 = self.func75(imports, 1048597i32, 20i32);
                var5 = var29;
                self.memory.store64(var1.wrapping_add(192i32) as usize, var4 as u64);
                self.memory.store64(var1.wrapping_add(184i32) as usize, arg0 as u64);
                self.memory.store64(var1 as usize + 176, var3 as u64);
                self.memory.store64(var1 as usize + 168, var5 as u64);
                let var30 = self.func60(imports, var2);
                let var31 = self.func50(imports, var1);
                let var32 = imports.contract_event(self, var30, var31);
                var32;
                self.global0 = var1.wrapping_add(368i32);
                return Void;
            }
            self.func81(imports, Error(Contract, #206));
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func93<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
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
            if (arg0 & 255i64 != Address(obj#0)) as i32 != 0 {
                break 'label0;
            }
            self.func57(imports, var2.wrapping_add(232i32), arg1);
            let var15 = self.memory.load64(var2 as usize + 232) as i64;
            if ((var15 == 0) as i32 == 0) as i32 != 0 {
                break 'label0;
            }
            let var16 = self.memory.load64(var2.wrapping_add(248i32) as usize) as i64;
            var1 = var16;
            let var17 = self.memory.load64(var2 as usize + 240) as i64;
            var7 = var17;
            self.func53(imports, var2.wrapping_add(232i32));
            let var19 = self.memory.load64(var2 as usize + 232) as i64;
            if (var19 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #0));
                break 'label0;
            }
            let var21 = self.memory.load64(var2.wrapping_add(264i32) as usize) as i64;
            var11 = var21;
            self.memory.store64(var2 as usize + 416, arg1 as u64);
            self.memory.store64(var2 as usize + 408, var7 as u64);
            self.memory.store64(var2 as usize + 400, True as u64);
            self.func47(imports, var2.wrapping_add(232i32), var2.wrapping_add(400i32));
            let var23 = self.memory.load8(var2 as usize + 393) as i32;
            if (var23 == 2i32) as i32 != 0 {
                self.func81(imports, Error(Contract, #200));
                break 'label0;
            }
            var3 = var2.wrapping_sub(-64i32);
            let var25 = self.func83(imports, var3, var2.wrapping_add(232i32), 168i32);
            var25;
            let var26 = self.func65(imports, var3);
            if (var26 == 0) as i32 != 0 {
                self.func81(imports, Error(Contract, #201));
                break 'label0;
            }
            self.memory.store64(var2 as usize + 256, arg0 as u64);
            self.memory.store64(var2 as usize + 248, arg1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, Void as u64);
            var4 = var2.wrapping_add(232i32);
            self.func43(imports, var2.wrapping_add(40i32), var4);
            let var29 = self.memory.load64(var2.wrapping_add(56i32) as usize) as i64;
            var9 = var29;
            let var30 = self.memory.load32(var2 as usize + 40) as i32;
            var3 = var30;
            let var31 = self.memory.load64(var2 as usize + 48) as i64;
            var10 = var31;
            let var32 = imports.get_current_contract_address(self);
            var5 = var32;
            let var33 = self.memory.load64(var2 as usize + 200) as i64;
            var10 = { let a = var10; let b = False; if var3 != 0 { a } else { b } };
            var9 = { let a = var9; let b = False; if var3 != 0 { a } else { b } };
            self.func74(imports, var33, var5, arg0, var10, var9);
            self.memory.store64(var2 as usize + 256, arg0 as u64);
            self.memory.store64(var2 as usize + 248, arg1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, Void as u64);
            let var35 = self.func44(imports, var4);
            self.func89(imports, var35);
            var3 = var2.wrapping_add(136i32);
            let var37 = self.memory.load64(var3 as usize) as i64;
            var5 = var37;
            let var38 = self.memory.load64(var2 as usize + 128) as i64;
            var6 = var38;
            var8 = var6.wrapping_add(var10);
            var6 = (((var8 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_add(var5.wrapping_add(var9));
            if ((var5 ^ var9 ^ -1i64) & (var5 ^ var6) < False) as i32 != 0 {
                break 'label0;
            }
            self.memory.store64(var3 as usize, var6 as u64);
            self.memory.store64(var2 as usize + 128, var8 as u64);
            self.memory.store64(var2 as usize + 248, arg1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, True as u64);
            self.memory.store32(var2 as usize + 36, 0i32 as u32);
            self.func49(imports, var2.wrapping_add(232i32), var2.wrapping_sub(-64i32));
            let var40 = self.memory.load64(var2 as usize + 112) as i64;
            let var41 = self.memory.load64(var2.wrapping_add(120i32) as usize) as i64;
            self.func68(imports, var2.wrapping_add(16i32), var10, var9, var40, var41, var2.wrapping_add(36i32));
            let var43 = self.memory.load32(var2 as usize + 36) as i32;
            if var43 != 0 {
                break 'label0;
            }
            let var44 = self.memory.load64(var2 as usize + 64) as i64;
            var5 = var44;
            let var45 = self.memory.load64(var2.wrapping_add(72i32) as usize) as i64;
            var6 = var45;
            if (var5 | var6 == 0) as i32 != 0 {
                break 'label0;
            }
            let var46 = self.memory.load64(var2 as usize + 16) as i64;
            var8 = var46;
            let var47 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
            var12 = var47;
            if (var8 | var12 ^ -9223372036854775808i64 == 0) as i32 & (var5 & var6 == -1i64) as i32 != 0 {
                break 'label0;
            }
            self.func71(imports, var2, var8, var12, var5, var6);
            let var49 = self.memory.load64(var2 as usize) as i64;
            var6 = var49;
            let var50 = self.memory.load64(var2.wrapping_add(8i32) as usize) as i64;
            var5 = var50;
            if (({ let a = (var6 == 0) as i32; let b = (var5 < False) as i32; if (var5 == 0) as i32 != 0 { a } else { b } }) == 0) as i32 != 0 {
                let var51 = self.func75(imports, 1049284i32, 4i32);
                var8 = var51;
                let var52 = self.func52(imports, var6, var5);
                self.memory.store64(var2 as usize + 408, var52 as u64);
                self.memory.store64(var2 as usize + 400, arg0 as u64);
                var3 = 0i32;
                'label1: loop {
                    if (var3 == 16i32) as i32 != 0 {
                        var3 = 0i32;
                        'label2: loop {
                            if (var3 != 16i32) as i32 != 0 {
                                let var53 = self.memory.load64(var2.wrapping_add(400i32).wrapping_add(var3) as usize) as i64;
                                self.memory.store64(var2.wrapping_add(232i32).wrapping_add(var3) as usize, var53 as u64);
                                var3 = var3.wrapping_add(8i32);
                                continue 'label2;
                            }
                            break;
                        }
                        let var54 = self.func59(imports, var2.wrapping_add(232i32), 2i32);
                        self.func76(imports, var11, var8, var54);
                    } else {
                        self.memory.store64(var2.wrapping_add(232i32).wrapping_add(var3) as usize, Void as u64);
                        var3 = var3.wrapping_add(8i32);
                        continue 'label1;
                    }
                    break;
                }
            }
            let var56 = self.memory.load64(var2 as usize + 216) as i64;
            var11 = var56;
            let var57 = self.func75(imports, 1048637i32, 12i32);
            var8 = var57;
            self.memory.store64(var2.wrapping_add(256i32) as usize, var11 as u64);
            self.memory.store64(var2.wrapping_add(248i32) as usize, arg1 as u64);
            self.memory.store64(var2 as usize + 240, var7 as u64);
            self.memory.store64(var2 as usize + 232, var8 as u64);
            let var58 = self.func60(imports, var2.wrapping_add(232i32));
            let var59 = self.func52(imports, var10, var9);
            var7 = var59;
            let var60 = self.func52(imports, var6, var5);
            self.memory.store64(var2 as usize + 408, var60 as u64);
            self.memory.store64(var2 as usize + 400, var7 as u64);
            let var61 = self.func59(imports, var2.wrapping_add(400i32), 2i32);
            self.memory.store64(var2 as usize + 440, var61 as u64);
            self.memory.store64(var2 as usize + 432, arg0 as u64);
            let var62 = self.func59(imports, var2.wrapping_add(432i32), 2i32);
            let var63 = imports.contract_event(self, var58, var62);
            var63;
            self.global0 = var2.wrapping_add(448i32);
            return Void;
            break;
        }
        unreachable!();
        // There should've been an expression value here, but this may be unreachable
        unreachable!();
    }
    fn func94<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(368i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(168i32), arg0);
        'label0: loop {
            'label1: loop {
                let var9 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var9 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                var3 = var2.wrapping_add(184i32);
                let var10 = self.memory.load64(var3 as usize) as i64;
                var0 = var10;
                let var11 = self.memory.load64(var2 as usize + 176) as i64;
                var5 = var11;
                self.func46(imports, var2.wrapping_add(168i32), arg1);
                let var13 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var13 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var14 = self.memory.load64(var3 as usize) as i64;
                var1 = var14;
                let var15 = self.memory.load64(var2 as usize + 176) as i64;
                var6 = var15;
                self.func53(imports, var2.wrapping_add(168i32));
                let var17 = self.memory.load64(var2 as usize + 168) as i64;
                if (var17 == 0) as i32 != 0 {
                    self.func81(imports, Error(Contract, #0));
                    break 'label1;
                }
                let var19 = self.memory.load32(var2.wrapping_add(228i32) as usize) as i32;
                var3 = var19;
                let var20 = self.memory.load32(var2.wrapping_add(224i32) as usize) as i32;
                var4 = var20;
                self.memory.store64(var2 as usize + 352, arg0 as u64);
                self.memory.store64(var2 as usize + 344, var5 as u64);
                self.memory.store64(var2 as usize + 336, True as u64);
                self.func47(imports, var2.wrapping_add(168i32), var2.wrapping_add(336i32));
                let var22 = self.memory.load8(var2 as usize + 329) as i32;
                if (var22 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, Error(Contract, #200));
                break;
            }
            unreachable!();
            break;
        }
        let var24 = self.func83(imports, var2, var2.wrapping_add(168i32), 168i32);
        var2 = var24;
        self.func77(imports, var2.wrapping_add(168i32), var6, arg1, var2, var4, var3);
        let var26 = self.func58(imports, var2.wrapping_add(168i32));
        self.global0 = var2.wrapping_add(368i32);
        var26
    }
    fn func95<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let var7 = self.global0;
        var2 = var7.wrapping_sub(368i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(168i32), arg0);
        'label0: loop {
            'label1: loop {
                let var9 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var9 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                var3 = var2.wrapping_add(184i32);
                let var10 = self.memory.load64(var3 as usize) as i64;
                var0 = var10;
                let var11 = self.memory.load64(var2 as usize + 176) as i64;
                var5 = var11;
                self.func46(imports, var2.wrapping_add(168i32), arg1);
                let var13 = self.memory.load64(var2 as usize + 168) as i64;
                if ((var13 == 0) as i32 == 0) as i32 != 0 {
                    break 'label1;
                }
                let var14 = self.memory.load64(var3 as usize) as i64;
                var1 = var14;
                let var15 = self.memory.load64(var2 as usize + 176) as i64;
                var6 = var15;
                self.func53(imports, var2.wrapping_add(168i32));
                let var17 = self.memory.load64(var2 as usize + 168) as i64;
                if (var17 == 0) as i32 != 0 {
                    self.func81(imports, Error(Contract, #0));
                    break 'label1;
                }
                let var19 = self.memory.load32(var2.wrapping_add(228i32) as usize) as i32;
                var3 = var19;
                let var20 = self.memory.load32(var2.wrapping_add(224i32) as usize) as i32;
                var4 = var20;
                self.memory.store64(var2 as usize + 352, arg0 as u64);
                self.memory.store64(var2 as usize + 344, var5 as u64);
                self.memory.store64(var2 as usize + 336, True as u64);
                self.func47(imports, var2.wrapping_add(168i32), var2.wrapping_add(336i32));
                let var22 = self.memory.load8(var2 as usize + 329) as i32;
                if (var22 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, Error(Contract, #200));
                break;
            }
            unreachable!();
            break;
        }
        let var24 = self.func83(imports, var2, var2.wrapping_add(168i32), 168i32);
        var2 = var24;
        self.func79(imports, var2.wrapping_add(168i32), var6, arg1, var2, var4, var3);
        let var26 = self.func58(imports, var2.wrapping_add(168i32));
        self.global0 = var2.wrapping_add(368i32);
        var26
    }
    fn func96<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64) -> i64 {
        let mut var1: i32 = 0;
        let var2 = self.global0;
        var1 = var2.wrapping_sub(368i32);
        self.global0 = var1;
        self.func57(imports, var1.wrapping_add(168i32), arg0);
        'label0: loop {
            let var4 = self.memory.load64(var1 as usize + 168) as i64;
            if (var4 == 0) as i32 != 0 {
                let var5 = self.memory.load64(var1 as usize + 176) as i64;
                var0 = var5;
                let var6 = self.memory.load64(var1.wrapping_add(184i32) as usize) as i64;
                self.memory.store64(var1 as usize + 352, var6 as u64);
                self.memory.store64(var1 as usize + 344, arg0 as u64);
                self.memory.store64(var1 as usize + 336, True as u64);
                self.func47(imports, var1.wrapping_add(168i32), var1.wrapping_add(336i32));
                let var8 = self.memory.load8(var1 as usize + 329) as i32;
                if (var8 != 2i32) as i32 != 0 {
                    break 'label0;
                }
                self.func81(imports, Error(Contract, #200));
            }
            unreachable!();
            break;
        }
        let var10 = self.func83(imports, var1, var1.wrapping_add(168i32), 168i32);
        var1 = var10;
        let var11 = self.func50(imports, var1);
        self.global0 = var1.wrapping_add(368i32);
        var11
    }
    fn func97<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i64 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i64 = 0;
        let var3 = self.global0;
        var0 = var3.wrapping_sub(128i32);
        self.global0 = var0;
        self.func53(imports, var0.wrapping_sub(-64i32));
        let var5 = self.memory.load64(var0 as usize + 64) as i64;
        if (var5 == 0) as i32 != 0 {
            self.func81(imports, Error(Contract, #0));
            unreachable!();
        }
        var1 = var0.wrapping_add(8i32);
        let var7 = self.func83(imports, var1, var0.wrapping_add(72i32), 56i32);
        var7;
        let var8 = self.func56(imports, var1);
        self.global0 = var0.wrapping_add(128i32);
        var8
    }
    fn func98<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i64, mut arg1: i64) -> i64 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_add(-64i32);
        self.global0 = var2;
        self.func57(imports, var2.wrapping_add(32i32), arg0);
        let var7 = self.memory.load64(var2 as usize + 32) as i64;
        if (((var7 == 0) as i32 == 0) as i32 | (arg1 & 255i64 != Address(obj#0)) as i32 == 0) as i32 != 0 {
            let var8 = self.memory.load64(var2.wrapping_add(48i32) as usize) as i64;
            var0 = var8;
            let var9 = self.memory.load64(var2 as usize + 40) as i64;
            var4 = var9;
            self.memory.store64(var2 as usize + 56, arg1 as u64);
            self.memory.store64(var2 as usize + 48, arg0 as u64);
            self.memory.store64(var2 as usize + 40, var4 as u64);
            self.memory.store64(var2 as usize + 32, Void as u64);
            self.func43(imports, var2.wrapping_add(8i32), var2.wrapping_add(32i32));
            let var11 = self.memory.load64(var2 as usize + 16) as i64;
            let var12 = self.memory.load32(var2 as usize + 8) as i32;
            var3 = var12;
            let var13 = self.memory.load64(var2.wrapping_add(24i32) as usize) as i64;
            let var14 = self.func52(imports, { let a = var11; let b = False; if var3 != 0 { a } else { b } }, { let a = var13; let b = False; if var3 != 0 { a } else { b } });
            self.global0 = var2.wrapping_sub(-64i32);
            return var14;
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
        self.memory.store64(var0 as usize + 24, U32(0) as u64);
        self.memory.store64(var0 as usize + 16, U32(0) as u64);
        self.memory.store64(var0 as usize + 8, U32(1) as u64);
        let var3 = self.func59(imports, var0.wrapping_add(8i32), 3i32);
        self.global0 = var0.wrapping_add(32i32);
        var3
    }
    fn func100<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i64, mut arg4: i64) {
        let mut var5: i64 = 0;
        let mut var6: i64 = 0;
        let mut var7: i64 = 0;
        let mut var8: i64 = 0;
        let mut var9: i64 = 0;
        let mut var10: i64 = 0;
        var5 = arg3 & 4294967295i64;
        var6 = arg1 & 4294967295i64;
        var7 = var5.wrapping_mul(var6);
        var8 = (arg3 as u64).wrapping_shr(32i64 as u32) as i64;
        var6 = var6.wrapping_mul(var8);
        var9 = (arg1 as u64).wrapping_shr(32i64 as u32) as i64;
        var5 = var6.wrapping_add(var5.wrapping_mul(var9));
        var10 = var7.wrapping_add(var5.wrapping_shl(32i64 as u32));
        self.memory.store64(arg0 as usize, var10 as u64);
        self.memory.store64(arg0 as usize + 8, ((var7 as u64 > var10 as u64) as i32 as u32 as i64).wrapping_add(var8.wrapping_mul(var9).wrapping_add((((var5 as u64) < var6 as u64) as i32 as u32 as i64).wrapping_shl(32i64 as u32) | (var5 as u64).wrapping_shr(32i64 as u32) as i64)).wrapping_add(arg1.wrapping_mul(arg4).wrapping_add(arg2.wrapping_mul(arg3))) as u64);
    }
    fn func101<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64, mut arg2: i64, mut arg3: i32) {
        let mut var4: i64 = 0;
        'label0: loop {
            if (arg3 & 64i32 == 0) as i32 != 0 {
                if (arg3 == 0) as i32 != 0 {
                    break 'label0;
                }
                var4 = (arg3 & 63i32) as u32 as i64;
                var2 = arg2.wrapping_shl(var4 as u32) | (arg1 as u64).wrapping_shr((0i32.wrapping_sub(arg3) & 63i32) as u32 as i64 as u32) as i64;
                var1 = arg1.wrapping_shl(var4 as u32);
                break 'label0;
            }
            var2 = arg1.wrapping_shl((arg3 & 63i32) as u32 as i64 as u32);
            var1 = False;
            break;
        }
        self.memory.store64(arg0 as usize, arg1 as u64);
        self.memory.store64(arg0 as usize + 8, arg2 as u64);
    }
    fn func102<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32) -> i64 {
        let var1 = imports.bytes_new_from_linear_memory(self, (arg0 as u32 as i64).wrapping_shl(32i64 as u32) | U32(0), U32(16));
        var1
    }
    fn func103<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    fn func104<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut arg0: i32, mut arg1: i64) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i64 = 0;
        let var5 = self.global0;
        var2 = var5.wrapping_sub(16i32);
        self.global0 = var2;
        let var6 = imports.bytes_len(self, arg1);
        let var7: i32;
        if (var6 & -4294967296i64 == 68719476736i64) as i32 != 0 {
            self.memory.store64(var2 as usize + 8, False as u64);
            self.memory.store64(var2 as usize, False as u64);
            'label0: loop {
                'label1: loop {
                    let var8 = imports.bytes_len(self, arg1);
                    if ((var8 as u64) < 4294967296i64 as u64) as i32 != 0 {
                        break 'label0;
                    }
                    let var9 = imports.bytes_front(self, arg1);
                    var4 = var9;
                    let var10 = imports.bytes_len(self, arg1);
                    let var11 = imports.bytes_slice(self, arg1, U32(1), var10 & -4294967296i64 | U32(0));
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
            self.memory.store64(arg0.wrapping_add(9i32) as usize, var13 as u64);
            self.memory.store64(arg0 as usize + 1, arg1 as u64);
            var7 = 0i32;
        } else {
            var7 = 1i32;
        }
        self.memory.store8(arg0 as usize, var7 as u8);
        self.global0 = var2.wrapping_add(16i32);
    }
}
