use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;

use instructions::*;
use state::*;
use errors::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod zetachain_universal_nft {
    use super::*;

    /// Initialize the universal NFT program
    pub fn initialize(
        ctx: Context<Initialize>,
        metadata_uri: String,
        max_supply: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, metadata_uri, max_supply)
    }

    /// Mint a new NFT on Solana with Universal NFT Protocol support
    pub fn mint_nft(
        ctx: Context<MintNFT>,
        metadata_uri: String,
        zeta_chain_id: u64,
        cross_chain_data: Vec<u8>,
    ) -> Result<()> {
        instructions::mint_nft::handler(ctx, metadata_uri, zeta_chain_id, cross_chain_data)
    }

    /// Transfer NFT ownership locally on Solana
    pub fn transfer_nft(
        ctx: Context<TransferNFT>,
        new_owner: Pubkey,
    ) -> Result<()> {
        instructions::transfer_nft::handler(ctx, new_owner)
    }

    /// Initiate cross-chain transfer to another chain via ZetaChain
    pub fn cross_chain_transfer(
        ctx: Context<CrossChainTransfer>,
        target_chain_id: u64,
        recipient: Vec<u8>,
        zeta_chain_data: Vec<u8>,
    ) -> Result<()> {
        instructions::cross_chain_transfer::handler(ctx, target_chain_id, recipient, zeta_chain_data)
    }

    /// Process incoming NFT from another chain via ZetaChain
    pub fn process_incoming_nft(
        ctx: Context<ProcessIncomingNFT>,
        metadata_uri: String,
        source_chain_id: u64,
        cross_chain_data: Vec<u8>,
        zeta_tx_hash: [u8; 32],
    ) -> Result<()> {
        instructions::process_incoming_nft::handler(ctx, metadata_uri, source_chain_id, cross_chain_data, zeta_tx_hash)
    }

    /// Verify cross-chain ownership using cryptographic proof
    pub fn verify_cross_chain_ownership(
        ctx: Context<VerifyCrossChainOwnership>,
        proof_data: Vec<u8>,
    ) -> Result<()> {
        instructions::verify_cross_chain_ownership::handler(ctx, proof_data)
    }

    /// Update NFT metadata (owner only)
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        new_metadata_uri: String,
    ) -> Result<()> {
        instructions::update_metadata::handler(ctx, new_metadata_uri)
    }

    /// Burn NFT and update program state
    pub fn burn_nft(ctx: Context<BurnNFT>) -> Result<()> {
        instructions::burn_nft::handler(ctx)
    }

    /// Setup ZetaChain gateway configuration (authority only)
    pub fn setup_gateway(
        ctx: Context<SetupGateway>,
        gateway_address: [u8; 20],
        supported_chains: Vec<u64>,
        version: u8,
    ) -> Result<()> {
        instructions::setup_gateway::handler(ctx, gateway_address, supported_chains, version)
    }
}
