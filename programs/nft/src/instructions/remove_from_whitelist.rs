use anchor_lang::prelude::*;
use crate::{error::NFTMintError::Unauthorized, state::{Whitelist, Config}};
#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info>{

    #[account(mut)]
    pub authority :Signer<'info>, 

    #[account(
        mut, 
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority @ Unauthorized
        
    )]
    pub config: Account<'info, Config>, 
    
    #[account(
        mut, 
        seeds = [b"whitelist", whitelist.user.as_ref()], 
        close = authority,
        bump = whitelist.bump
    )]
    pub whitelist : Account<'info, Whitelist>,

    pub system_program: Program<'info, System>


}

pub fn handler(_ctx:Context<RemoveFromWhitelist>)->Result<()>{
    Ok(())
}