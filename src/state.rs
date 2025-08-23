use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

/// Program state for the Universal NFT program
#[account]
pub struct ProgramState {
    pub authority: Pubkey,
    pub total_minted: u64,
    pub max_supply: u64,
    pub next_token_id: u64, // Added: Unique token ID counter
    pub bump: u8,
    pub created_at: i64,
}

/// ZetaChain gateway configuration
#[account]
pub struct ZetaChainGatewayState {
    pub gateway_address: [u8; 20],
    pub supported_chains: Vec<u64>,
    pub version: u8,
    pub updated_at: i64,
    pub bump: u8,
}

/// NFT metadata and cross-chain information
#[account]
pub struct NFTMetadata {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub zeta_chain_id: u64,
    pub cross_chain_data_hash: [u8; 32],
    pub token_id: u64, // Added: Universal token ID
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

/// NFT origin tracking for Universal NFT Protocol
#[account]
pub struct NFTOrigin {
    pub token_id: u64,
    pub original_mint: Pubkey, // Original mint key from source chain
    pub original_metadata_uri: String,
    pub source_chain_id: u64,
    pub created_at: i64,
    pub bump: u8,
}

/// Cross-chain transfer state
#[account]
pub struct CrossChainTransferState {
    pub nft_mint: Pubkey,
    pub token_id: u64, // Added: Universal NFT token ID
    pub source_chain_id: u64,
    pub target_chain_id: u64,
    pub recipient: Vec<u8>,
    pub status: TransferStatus,
    pub zeta_tx_hash: [u8; 32],
    pub created_at: i64,
    pub bump: u8,
}

/// Ownership verification state
#[account]
pub struct OwnershipVerificationState {
    pub nft_mint: Pubkey,
    pub zeta_owner: Vec<u8>,
    pub proof_hash: [u8; 32],
    pub verified: bool,
    pub verified_at: i64,
    pub bump: u8,
}

/// Transfer status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TransferStatus {
    Pending = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}

impl ProgramState {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 + // total_minted
        8 + // max_supply
        8 + // next_token_id
        1 + // bump
        8; // created_at
}

impl ZetaChainGatewayState {
    pub const LEN: usize = 8 + // discriminator
        20 + // gateway_address
        4 + 13 * 8 + // supported_chains (max 13 chains)
        1 + // version
        8 + // updated_at
        1; // bump
}

impl NFTMetadata {
    pub const LEN: usize = 8 + // discriminator
        32 + // mint
        32 + // owner
        4 + 200 + // metadata_uri (max 200 chars)
        8 + // zeta_chain_id
        32 + // cross_chain_data_hash
        8 + // token_id
        8 + // created_at
        8 + // updated_at
        1; // bump
}

impl NFTOrigin {
    pub const LEN: usize = 8 + // discriminator
        8 + // token_id
        32 + // original_mint
        4 + 200 + // original_metadata_uri (max 200 chars)
        8 + // source_chain_id
        8 + // created_at
        1; // bump
}

impl CrossChainTransferState {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        8 + // token_id
        8 + // source_chain_id
        8 + // target_chain_id
        4 + 100 + // recipient (max 100 bytes)
        1 + // status
        32 + // zeta_tx_hash
        8 + // created_at
        1; // bump
}

impl OwnershipVerificationState {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        4 + 100 + // zeta_owner (max 100 bytes)
        32 + // proof_hash
        1 + // verified
        8 + // verified_at
        1; // bump
}
