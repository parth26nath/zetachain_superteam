use anchor_lang::prelude::*;

use crate::{
    state::{NFTMetadata, OwnershipVerificationState, ZetaChainGatewayState},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct VerifyCrossChainOwnership<'info> {
    #[account(
        seeds = [b"gateway_state"],
        bump = gateway_state.bump
    )]
    pub gateway_state: Account<'info, ZetaChainGatewayState>,
    
    #[account(
        seeds = [b"nft_metadata", nft_mint.key().as_ref()],
        bump = nft_metadata.bump
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(
        mut,
        init_if_needed,
        payer = verifier,
        space = OwnershipVerificationState::LEN,
        seeds = [b"ownership_verification", nft_mint.key().as_ref()],
        bump
    )]
    pub verification_state: Account<'info, OwnershipVerificationState>,
    
    #[account(mut)]
    pub nft_mint: Account<'info, anchor_spl::token::Mint>,
    
    #[account(mut)]
    pub verifier: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<VerifyCrossChainOwnership>,
    proof_data: Vec<u8>,
) -> Result<()> {
    // Validate proof data length
    if proof_data.len() == 0 {
        return err!(UniversalNFTError::InvalidProofData);
    }
    
    let clock = Clock::get()?;
    
    // Verify proof data hash matches the stored cross-chain data hash
    let proof_hash = anchor_lang::solana_program::keccak::hash(&proof_data).to_bytes();
    if proof_hash != ctx.accounts.nft_metadata.cross_chain_data_hash {
        return err!(UniversalNFTError::CrossChainDataHashMismatch);
    }
    
    // Update verification state
    let verification_state = &mut ctx.accounts.verification_state;
    verification_state.nft_mint = ctx.accounts.nft_mint.key();
    verification_state.zeta_owner = vec![0u8; 100]; // Placeholder for ZetaChain owner
    verification_state.proof_hash = proof_hash;
    verification_state.verified = true;
    verification_state.verified_at = clock.unix_timestamp;
    verification_state.bump = *ctx.bumps.get("verification_state").unwrap();
    
    msg!("Cross-chain ownership verified successfully");
    msg!("NFT: {}", ctx.accounts.nft_mint.key());
    msg!("Proof hash: {:?}", proof_hash);
    msg!("Verified at: {}", clock.unix_timestamp);
    
    Ok(())
}
