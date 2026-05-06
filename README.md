# Solana NFT Mint Program

A Solana NFT minting program built with Anchor, featuring a configurable NFT minting system with admin-controlled whitelisting and supply limits.

## Features

- Global config PDA for protocol configuration
- Admin-controlled whitelist system
- Supply-capped NFT minting
- Per-user whitelist PDAs
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
- NFT mint fee
- Maximum NFT supply
- Current minted supply

Per-user PDA derived using:
```rust
[b"whitelist", user_pubkey]
```
Stores:
- Whitelisted user
- Mint status tracking (has_minted : bool)

### Instruction Flow
a. Initialize Config  
Creates the global protocol configuration.
 - Sets mint price  
 - Sets maximum NFT supply  
 - Assigns admin authority  

b. Initialize Whitelist  
Admin whitelists a user by creating a user-specific whitelist PDA.

c. Mint NFT  
Whitelisted users can mint NFTs if:
 - Supply cap is not exceeded  
 - User has not minted already  
 - Required fee payment is provided  
