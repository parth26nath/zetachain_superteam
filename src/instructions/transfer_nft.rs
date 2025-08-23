use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
};

use crate::{
    state::{NFTMetadata},
    errors::UniversalNFTError,
};

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(
        mut,
        seeds = [b"nft_metadata", nft_mint.key().as_ref()],
        bump = nft_metadata.bump,
        has_one = owner
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(
        mut,
        constraint = nft_mint.key() == nft_metadata.mint
    )]
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = new_owner,
        associated_token::mint = nft_mint,
        associated_token::authority = new_owner,
    )]
    pub new_owner_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(mut)]
    pub new_owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<TransferNFT>,
    _new_owner: Pubkey,
) -> Result<()> {
    let clock = Clock::get()?;
    
    // Transfer NFT from current owner to new owner
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_token_account.to_account_info(),
        to: ctx.accounts.new_owner_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    anchor_spl::token::transfer(cpi_ctx, 1)?;
    
    // Update NFT metadata
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.owner = ctx.accounts.new_owner.key();
    nft_metadata.updated_at = clock.unix_timestamp;
    
    msg!("NFT transferred successfully");
    msg!("From: {}", ctx.accounts.owner.key());
    msg!("To: {}", ctx.accounts.new_owner.key());
    msg!("NFT: {}", ctx.accounts.nft_mint.key());
    
    Ok(())
}
