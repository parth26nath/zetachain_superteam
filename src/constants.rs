use anchor_lang::prelude::*;

// Universal NFT Program Constants

// Maximum lengths
pub const MAX_METADATA_URI_LENGTH: usize = 200;
pub const MAX_CROSS_CHAIN_DATA_LENGTH: usize = 1000;
pub const MAX_RECIPIENT_ADDRESS_LENGTH: usize = 100;
pub const MAX_SUPPORTED_CHAINS: usize = 13;

// ZetaChain Network IDs
pub const ZETA_CHAIN_ID_SOLANA: u64 = 1;
pub const ZETA_CHAIN_ID_ETHEREUM: u64 = 2;
pub const ZETA_CHAIN_ID_BSC: u64 = 3;
pub const ZETA_CHAIN_ID_POLYGON: u64 = 4;
pub const ZETA_CHAIN_ID_AVALANCHE: u64 = 5;
pub const ZETA_CHAIN_ID_ARBITRUM: u64 = 6;
pub const ZETA_CHAIN_ID_OPTIMISM: u64 = 7;
pub const ZETA_CHAIN_ID_BASE: u64 = 8;
pub const ZETA_CHAIN_ID_LINEA: u64 = 9;
pub const ZETA_CHAIN_ID_MANTLE: u64 = 10;
pub const ZETA_CHAIN_ID_SCROLL: u64 = 11;
pub const ZETA_CHAIN_ID_BERACHAIN: u64 = 12;
pub const ZETA_CHAIN_ID_BITCOIN: u64 = 13;

// Solana-specific constants
pub const SOLANA_DECIMALS: u8 = 0;
pub const SOLANA_RENT_EXEMPTION: u64 = 2_039_280; // Minimum rent exemption for accounts

// Security constants
pub const REPLAY_PROTECTION_WINDOW: i64 = 300; // 5 minutes in seconds
pub const TSS_TIMEOUT: i64 = 3600; // 1 hour in seconds
pub const MINIMUM_GATEWAY_UPDATE_INTERVAL: i64 = 60; // 1 minute in seconds

// Fee constants
pub const CROSS_CHAIN_TRANSFER_FEE: u64 = 0; // No fee for now
pub const MINT_FEE: u64 = 0; // No fee for now

// Default metadata values
pub const DEFAULT_METADATA_NAME: &str = "Universal NFT";
pub const DEFAULT_METADATA_SYMBOL: &str = "UNFT";
pub const DEFAULT_METADATA_DESCRIPTION: &str = "Cross-chain Universal NFT";

// Gateway configuration
pub const GATEWAY_VERSION: u8 = 1;
pub const DEFAULT_GATEWAY_ADDRESS: [u8; 20] = [0u8; 20];

// Token ID generation constants
pub const TOKEN_ID_SEED: &[u8] = b"nft_origin";
pub const TOKEN_ID_OFFSET: u64 = 1000000; // Offset to ensure uniqueness

// Metaplex constants
pub const METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
pub const MASTER_EDITION_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

// Error message constants
pub const ERROR_INVALID_CHAIN_ID: &str = "Invalid ZetaChain ID";
pub const ERROR_INVALID_METADATA: &str = "Invalid metadata";
pub const ERROR_INVALID_TOKEN_ID: &str = "Invalid token ID";
pub const ERROR_TRANSFER_FAILED: &str = "Cross-chain transfer failed";
pub const ERROR_ORIGIN_NOT_FOUND: &str = "NFT origin not found";
