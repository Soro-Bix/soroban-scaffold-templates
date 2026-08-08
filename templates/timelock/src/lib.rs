#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contract]
pub struct TimelockContract;

#[contracttype]
#[derive(Clone)]
pub struct LockedTransfer {
    pub recipient: Address,
    pub amount: i128,
    pub release_time: u64,
    pub claimed: bool,
}

const ADMIN: Symbol = Symbol::new(&b"ADMIN");
const LOCK_COUNT: Symbol = Symbol::new(&b"LOCK_COUNT");
const LOCKS: Symbol = Symbol::new(&b"LOCKS");

#[contractimpl]
impl TimelockContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&LOCK_COUNT, &0u64);
    }

    pub fn schedule_transfer(env: Env, recipient: Address, amount: i128, release_time: u64) -> u64 {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut count: u64 = env.storage().instance().get(&LOCK_COUNT).unwrap();
        count += 1;
        env.storage().instance().set(&LOCK_COUNT, &count);

        let lock = LockedTransfer {
            recipient,
            amount,
            release_time,
            claimed: false,
        };
        env.storage().persistent().set(&(LOCKS, count), &lock);
        count
    }

    pub fn claim(env: Env, claimer: Address, lock_id: u64) {
        claimer.require_auth();
        let mut lock: LockedTransfer = env.storage().persistent().get(&(LOCKS, lock_id)).unwrap();
        assert!(!lock.claimed, "Already claimed");
        assert!(lock.recipient == claimer, "Not the recipient");
        assert!(env.ledger().timestamp() >= lock.release_time, "Still locked");

        lock.claimed = true;
        env.storage().persistent().set(&(LOCKS, lock_id), &lock);
    }

    pub fn get_lock(env: Env, lock_id: u64) -> LockedTransfer {
        env.storage().persistent().get(&(LOCKS, lock_id)).unwrap()
    }
}
