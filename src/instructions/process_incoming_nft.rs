use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, MintTo},
    metadata::{
        create_metadata_accounts_v3,
        CreateMetadataAccountsV3,
        DataV2,
    },
};
use mpl_token_metadata::instruction::create_metadata_accounts_v3 as mpl_create_metadata;

use crate::{
    state::{ProgramState, NFTMetadata, CrossChainTransferState, ZetaChainGatewayState, TransferStatus, NFTOrigin},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct ProcessIncomingNFT<'info> {
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
        seeds = [b"cross_chain_transfer", incoming_nft_mint.key().as_ref()],
        bump = transfer_state.bump,
        constraint = transfer_state.status == TransferStatus::InProgress
    )]
    pub transfer_state: Account<'info, CrossChainTransferState>,
    
    #[account(
        init,
        payer = payer,
        mint = incoming_nft_mint,
        authority = recipient,
        decimals = SOLANA_DECIMALS,
        freeze_authority = Some(recipient.key()),
    )]
    pub incoming_nft_mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = incoming_nft_mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = payer,
        space = NFTMetadata::LEN,
        seeds = [b"nft_metadata", incoming_nft_mint.key().as_ref()],
        bump
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = NFTOrigin::LEN,
        seeds = [TOKEN_ID_SEED, &transfer_state.token_id.to_le_bytes()],
        bump
    )]
    pub nft_origin: Account<'info, NFTOrigin>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub recipient: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<ProcessIncomingNFT>,
    metadata_uri: String,
    source_chain_id: u64,
    cross_chain_data: Vec<u8>,
    zeta_tx_hash: [u8; 32],
) -> Result<()> {
    // Validate metadata URI length
    if metadata_uri.len() > MAX_METADATA_URI_LENGTH {
        return err!(UniversalNFTError::InvalidMetadataURILength);
    }
    
    // Validate source chain ID
    if !ctx.accounts.gateway_state.supported_chains.contains(&source_chain_id) {
        return err!(UniversalNFTError::InvalidZetaChainID);
    }
    
    // Cannot process from the same chain
    if source_chain_id == ZETA_CHAIN_ID_SOLANA {
        return err!(UniversalNFTError::InvalidZetaChainID);
    }
    
    // Validate cross-chain data length
    if cross_chain_data.len() > MAX_CROSS_CHAIN_DATA_LENGTH {
        return err!(UniversalNFTError::InvalidCrossChainData);
    }
    
    // Verify transfer state matches
    let transfer_state = &mut ctx.accounts.transfer_state;
    if transfer_state.source_chain_id != source_chain_id {
        return err!(UniversalNFTError::InvalidCrossChainData);
    }
    
    let clock = Clock::get()?;
    
    // Get the token ID from the transfer state
    let token_id = transfer_state.token_id;
    
    // Check if this NFT has been minted on Solana before by looking at the NFTOrigin
    let nft_origin = &mut ctx.accounts.nft_origin;
    let is_existing_nft = nft_origin.token_id != 0;
    
    let final_metadata_uri = if is_existing_nft {
        // This NFT was minted on Solana before - use original metadata
        msg!("Processing existing NFT with token ID: {}", token_id);
        msg!("Original mint: {}", nft_origin.original_mint);
        msg!("Original metadata URI: {}", nft_origin.original_metadata_uri);
        
        // Use the original metadata URI instead of the incoming one
        nft_origin.original_metadata_uri.clone()
    } else {
        // This is a new NFT coming to Solana for the first time
        msg!("Processing new NFT with token ID: {}", token_id);
        
        // Initialize NFT origin tracking
        nft_origin.token_id = token_id;
        nft_origin.original_mint = ctx.accounts.incoming_nft_mint.key();
        nft_origin.original_metadata_uri = metadata_uri.clone();
        nft_origin.source_chain_id = source_chain_id;
        nft_origin.created_at = clock.unix_timestamp;
        nft_origin.bump = *ctx.bumps.get("nft_origin").unwrap();
        
        metadata_uri
    };
    
    // Mint 1 token to the recipient
    let cpi_accounts = MintTo {
        mint: ctx.accounts.incoming_nft_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.recipient.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    anchor_spl::token::mint_to(cpi_ctx, 1)?;
    
    // Create metadata account
    let metadata_account = &ctx.accounts.nft_metadata;
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        ctx.accounts.incoming_nft_mint.key().as_ref(),
    ];
    let metadata_signer = &[&metadata_seeds[..]];
    
    let create_metadata_accounts = CreateMetadataAccountsV3 {
        metadata: metadata_account.to_account_info(),
        mint: ctx.accounts.incoming_nft_mint.to_account_info(),
        mint_authority: ctx.accounts.recipient.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        update_authority: ctx.accounts.recipient.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: Some(ctx.accounts.rent.to_account_info()),
    };
    
    let data_v2 = DataV2 {
        name: DEFAULT_METADATA_NAME.to_string(),
        symbol: DEFAULT_METADATA_SYMBOL.to_string(),
        uri: final_metadata_uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };
    
    let instruction = mpl_create_metadata(
        mpl_token_metadata::ID,
        create_metadata_accounts.metadata.key(),
        create_metadata_accounts.mint.key(),
        create_metadata_accounts.mint_authority.key(),
        create_metadata_accounts.payer.key(),
        create_metadata_accounts.update_authority.key(),
        data_v2.name,
        data_v2.symbol,
        data_v2.uri,
        Some(create_metadata_accounts.creators.clone().unwrap_or_default()),
        data_v2.seller_fee_basis_points,
        data_v2.uses.clone(),
        data_v2.collection.clone(),
        data_v2.is_mutable,
        data_v2.collection_details.clone(),
        data_v2.uses.clone(),
    );
    
    let accounts = vec![
        create_metadata_accounts.metadata.to_account_info(),
        create_metadata_accounts.mint.to_account_info(),
        create_metadata_accounts.mint_authority.to_account_info(),
        create_metadata_accounts.payer.to_account_info(),
        create_metadata_accounts.update_authority.to_account_info(),
        create_metadata_accounts.system_program.to_account_info(),
        create_metadata_accounts.rent.unwrap().to_account_info(),
    ];
    
    solana_program::program::invoke_signed(
        &instruction,
        accounts.as_slice(),
        metadata_signer,
    )?;
    
    // Initialize NFT metadata
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.mint = ctx.accounts.incoming_nft_mint.key();
    nft_metadata.owner = ctx.accounts.recipient.key();
    nft_metadata.metadata_uri = final_metadata_uri;
    nft_metadata.zeta_chain_id = source_chain_id;
    nft_metadata.cross_chain_data_hash = anchor_lang::solana_program::keccak::hash(&cross_chain_data).to_bytes();
    nft_metadata.token_id = token_id;
    nft_metadata.created_at = clock.unix_timestamp;
    nft_metadata.updated_at = clock.unix_timestamp;
    nft_metadata.bump = *ctx.bumps.get("nft_metadata").unwrap();
    
    // Update transfer state
    transfer_state.status = TransferStatus::Completed;
    transfer_state.zeta_tx_hash = zeta_tx_hash;
    
    // Update program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted += 1;
    
    msg!("Incoming NFT processed successfully");
    msg!("Mint address: {}", ctx.accounts.incoming_nft_mint.key());
    msg!("Recipient: {}", ctx.accounts.recipient.key());
    msg!("Token ID: {}", token_id);
    msg!("Source chain: {}", source_chain_id);
    msg!("ZetaChain TX: {:?}", zeta_tx_hash);
    msg!("Status: Completed");
    
    Ok(())
}
