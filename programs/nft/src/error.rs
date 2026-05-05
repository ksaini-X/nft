use anchor_lang::prelude::*;

#[error_code]
pub enum NFTMintError {
    #[msg("Invalid Prices")]
    InvalidPrice,

    #[msg("Invalid Max Supply")]
    InvalidMaxSupply,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("WhitelistMismatch")]
    WhitelistMismatch,

    #[msg("AlreadyMinted")]
    AlreadyMinted,

    #[msg("MaxSupplyReached")]
    MaxSupplyReached,

    #[msg("Overflow")]
    Overflow,
}
