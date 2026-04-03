# test_case
Permissionless Voting dApp on Stellar

A fully decentralized, permissionless voting smart contract built on Stellar using Soroban.

Overview

This project implements a voting protocol where anyone can:

- Create polls
- Define voting rules
- Participate in voting

There are no administrators, no ownership controls, and no centralized restrictions by default. Each poll defines its own constraints at creation time.

Key Features

- Permissionless poll creation
- Open voting system
- Configurable voting constraints per poll
- Deadline-based voting
- Optional vote limits per address
- Extensible staking mechanism (optional)

Tech Stack

- Smart Contracts: Rust (Soroban SDK)
- Blockchain:
- Contract Platform:

Contract Design

Poll Structure

Each poll contains:

- Unique poll ID
- Creator address
- Question
- List of options
- Vote counts per option
- Record of voters
- Deadline timestamp
- Max votes per address (optional)
- Stake requirement (optional)

Core Functions

create_poll

Creates a new poll with user-defined parameters.

vote

Allows any address to vote on a poll, subject to poll constraints.

get_poll

Fetches poll data by ID.

get_poll_count

Returns total number of polls.

Permissionless Architecture

The contract is designed with the following principles:

- No admin roles
- No privileged access
- No function restrictions by default
- All constraints are defined at the poll level

Project Structure

.
├── src
│   └── lib.rs       # Main contract implementation
├── Cargo.toml
└── README.md

Getting Started

Prerequisites

- Rust (latest stable)
- Cargo
- Soroban CLI

Install Soroban CLI

Follow official instructions from Stellar documentation.

Build Contract

cargo build --target wasm32-unknown-unknown --release

Deploy Contract

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<contract_name>.wasm \
  --network testnet

Usage

Create Poll

- Provide question and options
- Set deadline
- Optionally define voting limits and stake

Vote

- Connect wallet
- Select option
- Submit transaction

Fetch Results

- Query poll using poll ID

Constraints & Validation

- Minimum 2 options required
- Voting disabled after deadline
- Invalid option indexes rejected
- Vote limits enforced if specified

Future Improvements

- Token-based staking integration
- Event logging for indexing
- Frontend dashboard (React + Freighter)
- Off-chain indexing support
- Quadratic voting mechanisms

Security Considerations

- No centralized control
- On-chain validation for all actions
- Minimal attack surface due to simple design

License

MIT License

Author

Developed as a permissionless Web3 application on Stellar.

CCYPTFVECAADYNWGX5DRECY46EVLCJB6ERHSD7VIXF7MKOFTLTCOVBEQ

https://stellar.expert/explorer/testnet/contract/CCYPTFVECAADYNWGX5DRECY46EVLCJB6ERHSD7VIXF7MKOFTLTCOVBEQ

<img width="1920" height="1020" alt="image" src="https://github.com/user-attachments/assets/0367ec02-b836-483f-8839-a40c0d6e2057" />
