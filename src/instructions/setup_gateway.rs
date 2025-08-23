use anchor_lang::prelude::*;

use crate::{
    state::{ZetaChainGatewayState, ProgramState},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct SetupGateway<'info> {
    #[account(
        mut,
        seeds = [b"program_state"],
        bump = program_state.bump,
        has_one = authority
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        seeds = [b"gateway_state"],
        bump = gateway_state.bump
    )]
    pub gateway_state: Account<'info, ZetaChainGatewayState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SetupGateway>,
    gateway_address: [u8; 20],
    supported_chains: Vec<u64>,
    version: u8,
) -> Result<()> {
    // Validate supported chains count
    if supported_chains.len() > MAX_SUPPORTED_CHAINS {
        return err!(UniversalNFTError::InvalidZetaChainID);
    }
    
    // Validate version
    if version < GATEWAY_VERSION {
        return err!(UniversalNFTError::GatewayNotConfigured);
    }
    
    let clock = Clock::get()?;
    
    // Check minimum update interval
    let gateway_state = &ctx.accounts.gateway_state;
    if clock.unix_timestamp - gateway_state.updated_at < MINIMUM_GATEWAY_UPDATE_INTERVAL {
        return err!(UniversalNFTError::GatewayNotConfigured);
    }
    
    // Update gateway state
    let gateway_state = &mut ctx.accounts.gateway_state;
    gateway_state.gateway_address = gateway_address;
    gateway_state.supported_chains = supported_chains;
    gateway_state.version = version;
    gateway_state.updated_at = clock.unix_timestamp;
    
    msg!("Gateway configuration updated successfully");
    msg!("Gateway address: {:?}", gateway_address);
    msg!("Supported chains: {}", gateway_state.supported_chains.len());
    msg!("Version: {}", version);
    msg!("Updated at: {}", clock.unix_timestamp);
    
    Ok(())
}
