#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn swap(&mut self, env: Env, a: soroban_sdk::Address, b: soroban_sdk::Address, token_a: soroban_sdk::Address, token_b: soroban_sdk::Address, amount_a: i128, min_b_for_a: i128, amount_b: i128, min_a_for_b: i128) {
        a.require_auth_for_args((token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env));
        b.require_auth_for_args((token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env));
        self.move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        self.move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
    fn move_token(&self, env: &Env, token: &Address, from: &Address, to: &Address, max_spend_amount: i128, transfer_amount: i128) {
        let token = token::Client::new(env, token);
        let contract_address = env.current_contract_address();
        token.transfer(from, &contract_address, &max_spend_amount);
        token.transfer(&contract_address, to, &transfer_amount);
        token.transfer(&contract_address, from, &(max_spend_amount - transfer_amount));
    }
}
