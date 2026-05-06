# Solana NFT Mint Program

A Solana NFT minting program built with Anchor, featuring a configurable NFT minting system with admin-controlled whitelisting and supply limits.

## Features

- Global config PDA for protocol configuration
- Admin-controlled whitelist system
- Supply-capped NFT minting
- Per-user whitelist PDAs
- PDA-based authority management
- LiteSVM integration tests
- Anchor account validation and constraints

---

## Architecture

### Config PDA

Singleton PDA derived using:
```rust
[b"config"]
```
Stores:
- Admin authority
- NFT mint price
- Maximum NFT supply
- Current minted supply
- Whitelist PDA

Per-user PDA derived using:
```rust
[b"whitelist", user_pubkey]
```
Stores:
- Whitelisted user
- Mint status tracking

### Instruction Flow
a. Initialize Config  
Creates the global protocol configuration.
  
 - Sets mint price  
 - Sets maximum NFT supply  
 - Assigns admin authority  

d. Initialize Whitelist  
Admin whitelists a user by creating a user-specific PDA.

e. Mint NFT  
Whitelisted users can mint NFTs if:
 - Supply cap is not exceeded  
 - User has not minted already  
 - Required payment is provided  

## Tech Stack
to be added here.
rust, anchor framework, solana, spl token program, litesvm.
Concepts demonstrated include program derived addresses (PDAs), singleton account architecture, per-user PDA state, account discriminators, instruction discriminators, CPI and account validation, anchor constraints, solana transaction construction, litesvm-based testing.
The project uses LiteSVM for integration testing.
to run tests: `cargo test -- --nocapture`
The project structure includes `programs/nft/` with src folder containing instructions, state files, error.rs, event.rs and lib.rs; and a tests folder with test.rs.
