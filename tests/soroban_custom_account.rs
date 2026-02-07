#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, Val, BytesN};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn init(&mut self, env: Env, owners: soroban_sdk::Vec<soroban_sdk::BytesN<32>>) {
        panic!("decompilation pending");
    }
    pub fn add_limit(&mut self, env: Env, token: soroban_sdk::Address, limit: i128) {
        panic!("contract error");
    }
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, signatures: soroban_sdk::Vec<soroban_sdk::Val>, auth_context: soroban_sdk::Vec<soroban_sdk::Val>) -> Result<(), AccError> {
        Ok(())
    }
}
