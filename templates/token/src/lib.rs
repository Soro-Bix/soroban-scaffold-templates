#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, String, Symbol};

#[contract]
pub struct TokenContract;

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

const ADMIN: Symbol = symbol_short!("ADMIN");
const METADATA: Symbol = symbol_short!("METADATA");

#[contractimpl]
impl TokenContract {
    /// Initialize the token with SEP-41 compliant metadata
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&METADATA, &TokenMetadata {
            name,
            symbol,
            decimals,
        });
    }

    /// SEP-41: Return token name
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&METADATA).unwrap().name
    }

    /// SEP-41: Return token symbol
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&METADATA).unwrap().symbol
    }

    /// SEP-41: Return token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&METADATA).unwrap().decimals
    }
}

mod test;
