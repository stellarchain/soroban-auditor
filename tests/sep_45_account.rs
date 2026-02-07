#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, BytesN, auth::Context};

#[contract]
pub struct Contract;

#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey { Admin, Signer(soroban_sdk::BytesN<32>), }
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Signature { pub public_key: soroban_sdk::BytesN<32>, pub signature: soroban_sdk::BytesN<64>, }
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AccountError { UnknownSigner = 1, TooManySignatures = 2, }

#[contractimpl]
impl Contract {
    pub fn ___check_auth(&mut self, env: Env, signature_payload: soroban_sdk::BytesN<32>, signatures: soroban_sdk::Vec<Signature>, _auth_context: soroban_sdk::Vec<Context>) -> Result<(), AccountError> {
        for signature in signatures.iter() {
            if !env.storage().instance().has(&DataKey::Signer(signature.public_key.clone())) {
                return Err(AccountError::UnknownSigner);
            }
            env.crypto().ed25519_verify(&signature.public_key, &signature_payload.clone().into(), &signature.signature);
        }
        Ok(())
    }
    pub fn ___constructor(&mut self, env: Env, admin: soroban_sdk::Address, signer: soroban_sdk::BytesN<32>) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Signer(signer), &());
    }
    pub fn upgrade(&mut self, env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
