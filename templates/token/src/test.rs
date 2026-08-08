#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

#[test]
fn test_token_initialization() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &contract_id);

    client.initialize(&admin, &String::from_str(&env, "TestToken"), &String::from_str(&env, "TST"), &7u32);
    assert_eq!(client.name(), String::from_str(&env, "TestToken"));
    assert_eq!(client.symbol(), String::from_str(&env, "TST"));
    assert_eq!(client.decimals(), 7u32);
}
