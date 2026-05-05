pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A6q28xDSGNKR6riAoKxMq4pBoAvJ9gkYbc2myghVGzVt");

#[program]
pub mod nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price: u64, max_supply: u32) -> Result<()> {
        initialize::handler(ctx, price, max_supply)
    }

    pub fn whitelist(ctx: Context<InitWhitelist>) -> Result<()> {
        whitelist::handler(ctx)
    }

    pub fn remove_whitelist(ctx: Context<RemoveFromWhitelist>) -> Result<()> {
        remove_from_whitelist::handler(ctx)
    }
}
