use anchor_lang::prelude::*;
use crate::{state::{Whitelist,Config}, error::NFTMintError};

#[derive(Accounts)]
pub struct InitWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK : User whos being whitelisted by admin
    pub user : UncheckedAccount<'info>, 

    #[account(
        mut, 
        seeds = [b"config"],
        bump = config.bump, 
        has_one = authority @ NFTMintError::Unauthorized
    )]
    pub config: Account<'info, Config>, 

    #[account(
        init, 
        seeds = [b"whitelist", user.key().as_ref()],
        bump, 
        payer = authority,
        space = Whitelist::size()
    )]
    pub whitelist: Account<'info, Whitelist>, 

    pub system_program:Program<'info, System>
}


pub fn handler(ctx:Context<InitWhitelist>)->Result<()>{
    let whitelist =&mut  ctx.accounts.whitelist;

    whitelist.bump = ctx.bumps.whitelist;
    whitelist.has_minted = false;
    whitelist.user = ctx.accounts.user.key();

    Ok(())
}