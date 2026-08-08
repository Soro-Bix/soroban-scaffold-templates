#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec};

#[contract]
pub struct VestingContract;

#[contracttype]
#[derive(Clone)]
pub struct VestingSchedule {
    pub beneficiary: Address,
    pub total_amount: i128,
    pub claimed_amount: i128,
    pub start_time: u64,
    pub cliff_time: u64,
    pub end_time: u64,
}

const ADMIN: Symbol = Symbol::new(&b"ADMIN");
const SCHEDULE_COUNT: Symbol = Symbol::new(&b"SCHEDULE_COUNT");
const SCHEDULES: Symbol = Symbol::new(&b"SCHEDULES");

#[contractimpl]
impl VestingContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&SCHEDULE_COUNT, &0u64);
    }

    pub fn create_schedule(
        env: Env,
        beneficiary: Address,
        total_amount: i128,
        start_time: u64,
        cliff_time: u64,
        end_time: u64,
    ) -> u64 {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        assert!(cliff_time >= start_time, "Cliff before start");
        assert!(end_time > cliff_time, "End before cliff");

        let mut count: u64 = env.storage().instance().get(&SCHEDULE_COUNT).unwrap();
        count += 1;
        env.storage().instance().set(&SCHEDULE_COUNT, &count);

        let schedule = VestingSchedule {
            beneficiary,
            total_amount,
            claimed_amount: 0,
            start_time,
            cliff_time,
            end_time,
        };
        env.storage().persistent().set(&(SCHEDULES, count), &schedule);
        count
    }

    pub fn claim(env: Env, schedule_id: u64) -> i128 {
        let mut schedule: VestingSchedule = env.storage().persistent().get(&(SCHEDULES, schedule_id)).unwrap();
        schedule.beneficiary.require_auth();

        let now = env.ledger().timestamp();
        assert!(now >= schedule.cliff_time, "Cliff not reached");

        let vested = if now >= schedule.end_time {
            schedule.total_amount
        } else {
            (schedule.total_amount * (now - schedule.start_time) as i128) / (schedule.end_time - schedule.start_time) as i128
        };

        let claimable = vested - schedule.claimed_amount;
        assert!(claimable > 0, "Nothing to claim");

        schedule.claimed_amount += claimable;
        env.storage().persistent().set(&(SCHEDULES, schedule_id), &schedule);
        claimable
    }

    pub fn get_schedule(env: Env, schedule_id: u64) -> VestingSchedule {
        env.storage().persistent().get(&(SCHEDULES, schedule_id)).unwrap()
    }
}
