#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, Vec, Map, Address, String
};

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub id: u64,
    pub creator: Address,
    pub question: String,
    pub options: Vec<String>,
    pub votes: Map<u32, u32>,         // option_index -> votes
    pub voters: Map<Address, u32>,    // address -> votes_cast
    pub deadline: u64,
    pub max_votes_per_address: u32,
    pub stake_required: i128,
}

#[contracttype]
pub enum DataKey {
    Poll(u64),
    PollCount,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // =========================
    // CREATE POLL
    // =========================
    pub fn create_poll(
        env: Env,
        creator: Address,
        question: String,
        options: Vec<String>,
        deadline: u64,
        max_votes_per_address: u32,
        stake_required: i128,
    ) -> u64 {

        creator.require_auth();

        if options.len() < 2 {
            panic!("At least 2 options required");
        }

        let mut poll_count: u64 = env.storage().instance()
            .get(&DataKey::PollCount)
            .unwrap_or(0);

        poll_count += 1;

        let mut vote_map: Map<u32, u32> = Map::new(&env);
        for i in 0..options.len() {
            vote_map.set(i as u32, 0);
        }

        let poll = Poll {
            id: poll_count,
            creator,
            question,
            options,
            votes: vote_map,
            voters: Map::new(&env),
            deadline,
            max_votes_per_address,
            stake_required,
        };

        env.storage().instance().set(&DataKey::Poll(poll_count), &poll);
        env.storage().instance().set(&DataKey::PollCount, &poll_count);

        poll_count
    }

    // =========================
    // VOTE
    // =========================
    pub fn vote(
        env: Env,
        voter: Address,
        poll_id: u64,
        option_index: u32,
    ) {

        voter.require_auth();

        let key = DataKey::Poll(poll_id);

        let mut poll: Poll = env.storage().instance()
            .get(&key)
            .expect("Poll not found");

        let current_time = env.ledger().timestamp();

        if current_time > poll.deadline {
            panic!("Poll expired");
        }

        if option_index >= poll.options.len() as u32 {
            panic!("Invalid option");
        }

        let votes_cast = poll.voters.get(voter.clone()).unwrap_or(0);

        if poll.max_votes_per_address > 0 && votes_cast >= poll.max_votes_per_address {
            panic!("Vote limit reached");
        }

        // Optional staking logic placeholder
        if poll.stake_required > 0 {
            // You can integrate token transfer here if needed
            // Keeping it minimal for now
        }

        let current_votes = poll.votes.get(option_index).unwrap_or(0);
        poll.votes.set(option_index, current_votes + 1);

        poll.voters.set(voter, votes_cast + 1);

        env.storage().instance().set(&key, &poll);
    }

    // =========================
    // GET POLL
    // =========================
    pub fn get_poll(env: Env, poll_id: u64) -> Poll {
        env.storage().instance()
            .get(&DataKey::Poll(poll_id))
            .expect("Poll not found")
    }

    // =========================
    // TOTAL POLLS
    // =========================
    pub fn get_poll_count(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::PollCount)
            .unwrap_or(0)
    }
}