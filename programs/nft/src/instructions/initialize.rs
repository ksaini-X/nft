use anchor_lang::prelude::*;
use crate::{error::NFTMintError, event::ConfigInitialized, state::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init, 
        seeds = [b"config"],
        payer = authority, 
        bump, 
        space =  Config::size()
    )]
    pub config: Account<'info, Config>, 

    pub system_program:Program<'info, System>
}

pub fn handler(ctx:Context<Initialize>, price:u64, max_supply:u32)->Result<()>{
    require!(price>0,NFTMintError::InvalidPrice);
    require!(max_supply>0,NFTMintError::InvalidMaxSupply);

    let config = &mut ctx.accounts.config;

    config.authority = ctx.accounts.authority.key();
    config.max_supply = max_supply; 
    config.current_supply = 0; 
    config.price = price; 
    config.bump = ctx.bumps.config;

    emit!(ConfigInitialized{
        authority:ctx.accounts.authority.key(), 
        price, 
        max_supply
    });
    
    Ok(())
}