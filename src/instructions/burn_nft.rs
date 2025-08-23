use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Burn},
};

use crate::{
    state::{NFTMetadata, ProgramState},
    errors::UniversalNFTError,
};

#[derive(Accounts)]
pub struct BurnNFT<'info> {
    #[account(
        mut,
        seeds = [b"program_state"],
        bump = program_state.bump
    )]
    pub program_state: Account<'info, ProgramState>,
    
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
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<BurnNFT>,
) -> Result<()> {
    // Burn the NFT
    let cpi_accounts = Burn {
        mint: ctx.accounts.nft_mint.to_account_info(),
        from: ctx.accounts.owner_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    anchor_spl::token::burn(cpi_ctx, 1)?;
    
    // Update program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted -= 1;
    
    msg!("NFT burned successfully");
    msg!("NFT: {}", ctx.accounts.nft_mint.key());
    msg!("Owner: {}", ctx.accounts.owner.key());
    msg!("Total minted: {}", program_state.total_minted);
    
    Ok(())
}
