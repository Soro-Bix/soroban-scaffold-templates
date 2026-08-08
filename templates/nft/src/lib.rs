#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec};

#[contract]
pub struct NftContract;

#[contracttype]
#[derive(Clone)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image_uri: String,
}

const OWNER: Symbol = Symbol::new(&b"OWNER");
const TOKEN_COUNT: Symbol = Symbol::new(&b"TOKEN_COUNT");
const TOKENS: Symbol = Symbol::new(&b"TOKENS");

#[contractimpl]
impl NftContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&OWNER, &admin);
        env.storage().instance().set(&TOKEN_COUNT, &0u32);
    }

    pub fn mint(env: Env, to: Address, name: String, description: String, image_uri: String) -> u32 {
        let owner: Address = env.storage().instance().get(&OWNER).unwrap();
        owner.require_auth();

        let mut count: u32 = env.storage().instance().get(&TOKEN_COUNT).unwrap();
        count += 1;
        env.storage().instance().set(&TOKEN_COUNT, &count);

        let token_id = count;
        let metadata = NftMetadata { name, description, image_uri };
        env.storage().persistent().set(&(TOKENS, token_id), &(to, metadata));

        token_id
    }

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        let (owner, _): (Address, NftMetadata) = env.storage().persistent().get(&(TOKENS, token_id)).unwrap();
        owner
    }

    pub fn token_metadata(env: Env, token_id: u32) -> NftMetadata {
        let (_, metadata): (Address, NftMetadata) = env.storage().persistent().get(&(TOKENS, token_id)).unwrap();
        metadata
    }

    pub fn total_supply(env: Env) -> u32 {
        env.storage().instance().get(&TOKEN_COUNT).unwrap()
    }
}
