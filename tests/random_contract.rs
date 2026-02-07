#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, BytesN};

#[contract]
pub struct Contract;

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { ContractInfo, Launch(soroban_sdk::Address, u64), LaunchBalance(soroban_sdk::Address, u64, soroban_sdk::Address), SpaceMission(soroban_sdk::Address), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractInfo { pub admin: soroban_sdk::Address, pub native_contract: soroban_sdk::Address, pub slz_fee: u32, pub slz_fee_destination: soroban_sdk::Address, pub space_fee: u32, pub space_missions: soroban_sdk::Map<u32, SpaceMissionData>, pub stability_check_duration: u64, pub stellarbucks_contract: soroban_sdk::Address, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Launch { pub asset: soroban_sdk::Address, pub dev: soroban_sdk::Address, pub funds_claimed: bool, pub funds_recipient: soroban_sdk::Address, pub info: soroban_sdk::String, pub max_price: i128, pub max_supply: i128, pub min_price: i128, pub pool_balance: i128, pub stability_check: bool, pub stability_check_end: u64, pub stellarbucks: i128, pub supply: i128, pub timestamp: u64, pub tokens_claimed: i128, }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpaceMissionData { pub guaranteed_success_funding: u64, pub reward: i128, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error { NotInitialized = 0, Initialized = 1, NotAuthorized = 2, ZeroMinPrice = 100, ZeroMaxPrice = 101, MinPriceGreaterMax = 102, ZeroMaxSupply = 103, MaxSupplyTooBig = 104, MaxPriceTooBig = 105, LaunchNotFound = 200, LaunchInProgress = 201, LaunchEnded = 202, PriceChanged = 203, InsufficientBalance = 204, LaunchSoldOut = 205, LaunchFundsClaimed = 206, InvalidMissionDifficulty = 300, ExasiveFunding = 301, MissionRewardChanged = 302, MissionRewardZero = 303, }

#[contractimpl]
impl Contract {
    pub fn initialize(&mut self, env: Env, admin: soroban_sdk::Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk::Address, stellarbucks_contract: soroban_sdk::Address, native_contract: soroban_sdk::Address, space_missions_odds: soroban_sdk::Map<u32, u64>) {
        panic!("decompilation pending");
    }
    pub fn change_contract_info(&mut self, env: Env, admin: soroban_sdk::Address, stability_check_duration: u64, space_fee: u32, slz_fee: u32, slz_fee_destination: soroban_sdk::Address, space_missions_odds: soroban_sdk::Map<u32, u64>) {
        panic!("decompilation pending");
    }
    pub fn upgrade(&mut self, env: Env, hash: soroban_sdk::BytesN<32>) {
        env.deployer().update_current_contract_wasm(hash);
    }
    pub fn start_space_mission(&mut self, env: Env, user: soroban_sdk::Address, funding: i128, difficulty: u32, min_mission_reward: i128) -> (bool, i128,) {
        panic!("decompilation pending");
    }
    pub fn add_space_missions_reward(&mut self, env: Env, user: soroban_sdk::Address, funds: i128, reward_difficulty: u32) {
        panic!("decompilation pending");
    }
    pub fn new_launch(&mut self, env: Env, dev: soroban_sdk::Address, funds_recipient: soroban_sdk::Address, info: soroban_sdk::String, asset: soroban_sdk::Address, max_supply: i128, min_price: i128, max_price: i128, launch_index: u64) {
        panic!("decompilation pending");
    }
    pub fn cancel_launch(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,)) {
        panic!("decompilation pending");
    }
    pub fn buy(&mut self, env: Env, user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,), sending: i128, min_receive: i128) {
        panic!("decompilation pending");
    }
    pub fn sell(&mut self, env: Env, user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,), sending: i128, min_receive: i128) {
        panic!("decompilation pending");
    }
    pub fn claim_launch_funds(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,)) {
        panic!("decompilation pending");
    }
    pub fn claim_launch_balance(&mut self, env: Env, user: soroban_sdk::Address, launch_key: (soroban_sdk::Address, u64,)) {
        panic!("decompilation pending");
    }
    pub fn calculate_buy(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), sending: i128) -> (i128, i128, i128, i128, i128,) {
        panic!("decompilation pending");
    }
    pub fn calculate_sell(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), sending: i128) -> (i128, i128, i128, i128, i128,) {
        panic!("decompilation pending");
    }
    pub fn get_launch_data(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,)) -> Launch {
        panic!("decompilation pending");
    }
    pub fn get_contract_info(&mut self, env: Env) -> ContractInfo {
        panic!("decompilation pending");
    }
    pub fn get_launch_balance(&mut self, env: Env, launch_key: (soroban_sdk::Address, u64,), user: soroban_sdk::Address) -> i128 {
        panic!("decompilation pending");
    }
    pub fn version(&mut self, env: Env) -> (u32, u32, u32,) {
        panic!("decompilation pending");
    }
}
