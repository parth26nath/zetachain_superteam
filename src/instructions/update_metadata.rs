use anchor_lang::prelude::*;

use crate::{
    state::{NFTMetadata},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        seeds = [b"nft_metadata", nft_mint.key().as_ref()],
        bump = nft_metadata.bump,
        has_one = owner
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(mut)]
    pub nft_mint: Account<'info, anchor_spl::token::Mint>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateMetadata>,
    new_metadata_uri: String,
) -> Result<()> {
    // Validate metadata URI length
    if new_metadata_uri.len() > MAX_METADATA_URI_LENGTH {
        return err!(UniversalNFTError::InvalidMetadataURILength);
    }
    
    let clock = Clock::get()?;
    
    // Update NFT metadata
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.metadata_uri = new_metadata_uri.clone();
    nft_metadata.updated_at = clock.unix_timestamp;
    
    msg!("NFT metadata updated successfully");
    msg!("NFT: {}", ctx.accounts.nft_mint.key());
    msg!("New URI: {}", new_metadata_uri);
    msg!("Updated at: {}", clock.unix_timestamp);
    
    Ok(())
}
