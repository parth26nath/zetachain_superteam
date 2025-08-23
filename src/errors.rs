use anchor_lang::prelude::*;

#[error_code]
pub enum UniversalNFTError {
    #[msg("Invalid metadata URI length")]
    InvalidMetadataURILength,
    
    #[msg("Maximum supply exceeded")]
    MaxSupplyExceeded,
    
    #[msg("NFT not found")]
    NFTNotFound,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("Invalid ZetaChain ID")]
    InvalidZetaChainID,
    
    #[msg("Cross-chain transfer already in progress")]
    TransferInProgress,
    
    #[msg("Invalid cross-chain data")]
    InvalidCrossChainData,
    
    #[msg("Ownership verification failed")]
    OwnershipVerificationFailed,
    
    #[msg("Invalid recipient address")]
    InvalidRecipientAddress,
    
    #[msg("Transfer status invalid for operation")]
    InvalidTransferStatus,
    
    #[msg("ZetaChain gateway not configured")]
    GatewayNotConfigured,
    
    #[msg("Unsupported target chain")]
    UnsupportedTargetChain,
    
    #[msg("Invalid proof data")]
    InvalidProofData,
    
    #[msg("NFT already exists")]
    NFTAlreadyExists,
    
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    
    #[msg("Token account creation failed")]
    TokenAccountCreationFailed,
    
    #[msg("Compute budget exceeded")]
    ComputeBudgetExceeded,
    
    #[msg("Rent exemption insufficient")]
    RentExemptionInsufficient,
    
    #[msg("Invalid signer")]
    InvalidSigner,
    
    #[msg("Cross-chain data hash mismatch")]
    CrossChainDataHashMismatch,
    
    #[msg("ZetaChain transaction failed")]
    ZetaChainTransactionFailed,
    
    #[msg("Replay protection failed")]
    ReplayProtectionFailed,
    
    #[msg("TSS verification failed")]
    TSSVerificationFailed,
}
