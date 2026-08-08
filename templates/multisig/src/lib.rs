#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec};

#[contract]
pub struct MultisigContract;

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub description: Symbol,
    pub approvals: Vec<Address>,
    pub executed: bool,
}

const SIGNERS: Symbol = Symbol::new(&b"SIGNERS");
const THRESHOLD: Symbol = Symbol::new(&b"THRESHOLD");
const PROPOSAL_COUNT: Symbol = Symbol::new(&b"PROPOSAL_COUNT");
const PROPOSALS: Symbol = Symbol::new(&b"PROPOSALS");

#[contractimpl]
impl MultisigContract {
    pub fn initialize(env: Env, signers: Vec<Address>, threshold: u32) {
        for signer in signers.iter() {
            signer.require_auth();
        }
        env.storage().instance().set(&SIGNERS, &signers);
        env.storage().instance().set(&THRESHOLD, &threshold);
        env.storage().instance().set(&PROPOSAL_COUNT, &0u64);
    }

    pub fn propose(env: Env, proposer: Address, description: Symbol) -> u64 {
        proposer.require_auth();
        let signers: Vec<Address> = env.storage().instance().get(&SIGNERS).unwrap();
        assert!(signers.contains(&proposer), "Not a signer");

        let mut count: u64 = env.storage().instance().get(&PROPOSAL_COUNT).unwrap();
        count += 1;
        env.storage().instance().set(&PROPOSAL_COUNT, &count);

        let proposal = Proposal {
            id: count,
            description,
            approvals: Vec::new(&env),
            executed: false,
        };
        env.storage().persistent().set(&(PROPOSALS, count), &proposal);
        count
    }

    pub fn approve(env: Env, signer: Address, proposal_id: u64) {
        signer.require_auth();
        let mut proposal: Proposal = env.storage().persistent().get(&(PROPOSALS, proposal_id)).unwrap();
        assert!(!proposal.executed, "Already executed");
        assert!(!proposal.approvals.contains(&signer), "Already approved");
        proposal.approvals.push_back(signer);
        env.storage().persistent().set(&(PROPOSALS, proposal_id), &proposal);
    }

    pub fn get_approvals(env: Env, proposal_id: u64) -> Vec<Address> {
        let proposal: Proposal = env.storage().persistent().get(&(PROPOSALS, proposal_id)).unwrap();
        proposal.approvals
    }
}
