use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    state::{ProgramState, ZetaChainGatewayState},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = ProgramState::LEN,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        init,
        payer = authority,
        space = ZetaChainGatewayState::LEN,
        seeds = [b"gateway_state"],
        bump
    )]
    pub gateway_state: Account<'info, ZetaChainGatewayState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Initialize>,
    metadata_uri: String,
    max_supply: u64,
) -> Result<()> {
    // Validate metadata URI length
    if metadata_uri.len() > MAX_METADATA_URI_LENGTH {
        return err!(UniversalNFTError::InvalidMetadataURILength);
    }
    
    // Validate max supply
    if max_supply == 0 {
        return err!(UniversalNFTError::MaxSupplyExceeded);
    }
    
    let clock = Clock::get()?;
    
    // Initialize program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.authority = ctx.accounts.authority.key();
    program_state.total_minted = 0;
    program_state.max_supply = max_supply;
    program_state.next_token_id = TOKEN_ID_OFFSET; // Start with offset for uniqueness
    program_state.bump = *ctx.bumps.get("program_state").unwrap();
    program_state.created_at = clock.unix_timestamp;
    
    // Initialize gateway state with default ZetaChain configuration
    let gateway_state = &mut ctx.accounts.gateway_state;
    gateway_state.gateway_address = DEFAULT_GATEWAY_ADDRESS; // Will be updated via setup_gateway
    gateway_state.supported_chains = vec![
        ZETA_CHAIN_ID_SOLANA,
        ZETA_CHAIN_ID_ETHEREUM,
        ZETA_CHAIN_ID_BSC,
        ZETA_CHAIN_ID_POLYGON,
        ZETA_CHAIN_ID_AVALANCHE,
        ZETA_CHAIN_ID_ARBITRUM,
        ZETA_CHAIN_ID_OPTIMISM,
        ZETA_CHAIN_ID_BASE,
        ZETA_CHAIN_ID_LINEA,
        ZETA_CHAIN_ID_MANTLE,
        ZETA_CHAIN_ID_SCROLL,
        ZETA_CHAIN_ID_BERACHAIN,
        ZETA_CHAIN_ID_BITCOIN,
    ];
    gateway_state.version = GATEWAY_VERSION;
    gateway_state.updated_at = clock.unix_timestamp;
    gateway_state.bump = *ctx.bumps.get("gateway_state").unwrap();
    
    msg!("Universal NFT program initialized successfully");
    msg!("Max supply: {}", max_supply);
    msg!("Next token ID: {}", program_state.next_token_id);
    msg!("Supported chains: {}", gateway_state.supported_chains.len());
    
    Ok(())
}
