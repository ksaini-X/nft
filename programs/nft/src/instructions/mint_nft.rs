use anchor_lang::{prelude::*, system_program::{self, Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{CreateMetadataAccountsV3, Metadata, create_metadata_accounts_v3, mpl_token_metadata::{instructions::CreateMetadataAccountV3, types::{Data, DataV2}}}, token_2022::{MintTo, spl_token_2022::instruction::mint_to}, token_interface::{self, Mint, TokenAccount, TokenInterface}
};
use crate::{error::NFTMintError, state::{Config, Whitelist}, whitelist,};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

   /// CHECK: Authority
    pub authority : UncheckedAccount<'info>, 

    #[account(
        mut, 
        seeds = [b"config"], 
        bump = config.bump
    )]
    pub config : Account<'info, Config>,

    #[account(
        mut, 
        seeds = [b"whitelist", user.key().as_ref()], 
        bump = whitelist.bump, 
        constraint = whitelist.user == user.key() @ NFTMintError::WhitelistMismatch
    )]
    pub whitelist : Account<'info, Whitelist>, 

    #[account(
        init, 
        payer = user, 
        mint::authority = config,
        mint::decimals = 0, 
        constraint = nft_mint.key() != user.key() @ NFTMintError::WhitelistMismatch
    )]
    pub nft_mint :  InterfaceAccount<'info, Mint>, 

    #[account(
        init, 
        payer = user, 
        associated_token::mint = nft_mint, 
        associated_token::authority = user,
    )]
    pub user_associated_token_account : InterfaceAccount<'info, TokenAccount>, 

    /// The metadata account to be created
    /// CHECK: Validated by seeds constraint to be the correct PDA
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK : METADATA_ACCOUNT
    pub metadata_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>
}

pub fn handler(
    ctx:Context<MintNFT>, 
    name:String, 
    symbol:String, uri:String
)->Result<()>{
    let config = &ctx
    .accounts.config;
    let whitelist = &ctx.accounts.whitelist;

    require!(!whitelist.has_minted, NFTMintError::AlreadyMinted);
    require!(
        config.current_supply < config.max_supply,
        NFTMintError::MaxSupplyReached
    );
    
    system_program::transfer(CpiContext::new(
        *ctx.accounts.system_program.key, 
        Transfer{
                    from:ctx.accounts.user.to_account_info(), 
                    to:ctx.accounts.authority.to_account_info()
                }), 
        config.price)?;
    
    token_interface::mint_to(CpiContext::new_with_signer(*ctx.accounts.token_program.key, 
        MintTo{
                    authority:ctx.accounts.config.to_account_info(), 
                    mint:ctx.accounts.nft_mint.to_account_info(), 
                    to:ctx.accounts.user_associated_token_account.to_account_info()
    }, &[&[
        b"config".as_ref(), 
        &[config.bump]
    ]]), 1)?;
    
    create_metadata_accounts_v3(CpiContext::new(
        *ctx.accounts.token_metadata_program.key, CreateMetadataAccountsV3{
            metadata:ctx.accounts.metadata_account.to_account_info(), 
            mint:ctx.accounts.nft_mint.to_account_info(), 
            mint_authority:ctx.accounts.config.to_account_info(), 
            payer:ctx.accounts.user.to_account_info(), 
            rent:ctx.accounts.rent.to_account_info(), 
            system_program:ctx.accounts.system_program.to_account_info(), 
            update_authority:ctx.accounts.user.to_account_info()
        }), DataV2{
            collection:None, 
            creators: None, 
            name, 
            symbol, 
            uri, 
            seller_fee_basis_points:0, 
            uses:None
        }, 
        false, 
        false, 
        None)?;

        ctx.accounts.whitelist.reload();

        let whitelist = &mut ctx.accounts.whitelist;

        whitelist.has_minted = true;
        config.current_supply.checked_add(1).ok_or(NFTMintError::Overflow)?;

    Ok(())
}