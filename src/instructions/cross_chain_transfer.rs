use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
};

use crate::{
    state::{ProgramState, NFTMetadata, CrossChainTransferState, ZetaChainGatewayState, TransferStatus, NFTOrigin},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct CrossChainTransfer<'info> {
    #[account(
        mut,
        seeds = [b"program_state"],
        bump = program_state.bump
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        seeds = [b"gateway_state"],
        bump = gateway_state.bump
    )]
    pub gateway_state: Account<'info, ZetaChainGatewayState>,
    
    #[account(
        mut,
        seeds = [b"nft_metadata", nft_mint.key().as_ref()],
        bump = nft_metadata.bump,
        has_one = owner
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(
        seeds = [TOKEN_ID_SEED, &nft_metadata.token_id.to_le_bytes()],
        bump = nft_origin.bump
    )]
    pub nft_origin: Account<'info, NFTOrigin>,
    
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
        init,
        payer = owner,
        space = CrossChainTransferState::LEN,
        seeds = [b"cross_chain_transfer", nft_mint.key().as_ref()],
        bump
    )]
    pub transfer_state: Account<'info, CrossChainTransferState>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CrossChainTransfer>,
    target_chain_id: u64,
    recipient: Vec<u8>,
    zeta_chain_data: Vec<u8>,
) -> Result<()> {
    // Validate target chain ID
    if !ctx.accounts.gateway_state.supported_chains.contains(&target_chain_id) {
        return err!(UniversalNFTError::UnsupportedTargetChain);
    }
    
    // Cannot transfer to the same chain
    if target_chain_id == ZETA_CHAIN_ID_SOLANA {
        return err!(UniversalNFTError::InvalidZetaChainID);
    }
    
    // Validate recipient address length
    if recipient.len() > MAX_RECIPIENT_ADDRESS_LENGTH {
        return err!(UniversalNFTError::InvalidRecipientAddress);
    }
    
    // Validate cross-chain data length
    if zeta_chain_data.len() > MAX_CROSS_CHAIN_DATA_LENGTH {
        return err!(UniversalNFTError::InvalidCrossChainData);
    }
    
    // Check if transfer is already in progress
    if ctx.accounts.transfer_state.status == TransferStatus::InProgress {
        return err!(UniversalNFTError::TransferInProgress);
    }
    
    let clock = Clock::get()?;
    
    // Get the token ID from NFT origin for cross-chain message
    let token_id = ctx.accounts.nft_metadata.token_id;
    
    // Transfer NFT from owner to program (burning it on Solana)
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_token_account.to_account_info(),
        to: ctx.accounts.nft_mint.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    anchor_spl::token::transfer(cpi_ctx, 1)?;
    
    // Burn the NFT by setting supply to 0
    let cpi_accounts = anchor_spl::token::Burn {
        mint: ctx.accounts.nft_mint.to_account_info(),
        from: ctx.accounts.owner_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    anchor_spl::token::burn(cpi_ctx, 1)?;
    
    // Initialize cross-chain transfer state
    let transfer_state = &mut ctx.accounts.transfer_state;
    transfer_state.nft_mint = ctx.accounts.nft_mint.key();
    transfer_state.token_id = token_id; // Set the Universal NFT token ID
    transfer_state.source_chain_id = ZETA_CHAIN_ID_SOLANA;
    transfer_state.target_chain_id = target_chain_id;
    transfer_state.recipient = recipient;
    transfer_state.status = TransferStatus::InProgress;
    transfer_state.zeta_tx_hash = [0u8; 32]; // Will be updated when ZetaChain confirms
    transfer_state.created_at = clock.unix_timestamp;
    transfer_state.bump = *ctx.bumps.get("transfer_state").unwrap();
    
    // Update NFT metadata to reflect transfer
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.owner = Pubkey::default(); // Clear owner during transfer
    nft_metadata.updated_at = clock.unix_timestamp;
    
    // Update program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted -= 1;
    
    msg!("Cross-chain transfer initiated");
    msg!("NFT: {}", ctx.accounts.nft_mint.key());
    msg!("Token ID: {}", token_id);
    msg!("From: Solana (Chain ID: {})", ZETA_CHAIN_ID_SOLANA);
    msg!("To: Chain ID: {}", target_chain_id);
    msg!("Recipient: {:?}", recipient);
    msg!("Status: In Progress");
    
    // TODO: Integrate with ZetaChain gateway contract to initiate actual cross-chain transfer
    // The token ID should be included in the cross-chain message to identify the NFT on the target chain
    // This would involve calling the gateway contract with the transfer parameters including the token ID
    
    Ok(())
}
