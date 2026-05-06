use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    /// Admin Pubkey, handles the config(singleton)
    pub authority: Pubkey,
    /// Price a user has to pay, to create a NFT
    pub price: u64,
    /// Max NFTs that can be minted
    pub max_supply: u32,
    /// NFTs that have been minted
    pub current_supply: u32,
    /// Bump
    pub bump: u8,
}

impl Config {
    pub fn size() -> usize {
        8 + Config::INIT_SPACE
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Whitelist {
    pub user: Pubkey,
    pub has_minted: bool,
    pub bump: u8,
}

impl Whitelist {
    pub fn size() -> usize {
        8 + Whitelist::INIT_SPACE
    }
}
