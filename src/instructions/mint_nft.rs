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
    state::{ProgramState, NFTMetadata, ZetaChainGatewayState, NFTOrigin},
    errors::UniversalNFTError,
    constants::*,
};

#[derive(Accounts)]
pub struct MintNFT<'info> {
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
    
    #[account(
        init,
        payer = payer,
        mint = mint,
        authority = mint_authority,
        decimals = SOLANA_DECIMALS,
        freeze_authority = Some(mint_authority.key()),
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = mint_authority,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = payer,
        space = NFTMetadata::LEN,
        seeds = [b"nft_metadata", mint.key().as_ref()],
        bump
    )]
    pub nft_metadata: Account<'info, NFTMetadata>,
    
    #[account(
        init,
        payer = payer,
        space = NFTOrigin::LEN,
        seeds = [TOKEN_ID_SEED, &program_state.next_token_id.to_le_bytes()],
        bump
    )]
    pub nft_origin: Account<'info, NFTOrigin>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: This is the mint authority for the NFT
    pub mint_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintNFT>,
    metadata_uri: String,
    zeta_chain_id: u64,
    cross_chain_data: Vec<u8>,
) -> Result<()> {
    // Validate metadata URI length
    if metadata_uri.len() > MAX_METADATA_URI_LENGTH {
        return err!(UniversalNFTError::InvalidMetadataURILength);
    }
    
    // Validate ZetaChain ID
    if !ctx.accounts.gateway_state.supported_chains.contains(&zeta_chain_id) {
        return err!(UniversalNFTError::InvalidZetaChainID);
    }
    
    // Check max supply
    let program_state = &mut ctx.accounts.program_state;
    if program_state.total_minted >= program_state.max_supply {
        return err!(UniversalNFTError::MaxSupplyExceeded);
    }
    
    // Validate cross-chain data length
    if cross_chain_data.len() > MAX_CROSS_CHAIN_DATA_LENGTH {
        return err!(UniversalNFTError::InvalidCrossChainData);
    }
    
    let clock = Clock::get()?;
    
    // Generate unique token ID: [mint pubkey + block.number + next_token_id]
    let block_number = clock.slot;
    let token_id = program_state.next_token_id;
    
    // Mint 1 token to the mint authority
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.mint_ata.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    anchor_spl::token::mint_to(cpi_ctx, 1)?;
    
    // Create metadata account
    let metadata_account = &ctx.accounts.nft_metadata;
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        ctx.accounts.mint.key().as_ref(),
    ];
    let metadata_signer = &[&metadata_seeds[..]];
    
    let create_metadata_accounts = CreateMetadataAccountsV3 {
        metadata: metadata_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        mint_authority: ctx.accounts.mint_authority.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        update_authority: ctx.accounts.mint_authority.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: Some(ctx.accounts.rent.to_account_info()),
    };
    
    let data_v2 = DataV2 {
        name: DEFAULT_METADATA_NAME.to_string(),
        symbol: DEFAULT_METADATA_SYMBOL.to_string(),
        uri: metadata_uri.clone(),
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
    nft_metadata.mint = ctx.accounts.mint.key();
    nft_metadata.owner = ctx.accounts.mint_authority.key();
    nft_metadata.metadata_uri = metadata_uri.clone();
    nft_metadata.zeta_chain_id = zeta_chain_id;
    nft_metadata.cross_chain_data_hash = anchor_lang::solana_program::keccak::hash(&cross_chain_data).to_bytes();
    nft_metadata.token_id = token_id;
    nft_metadata.created_at = clock.unix_timestamp;
    nft_metadata.updated_at = clock.unix_timestamp;
    nft_metadata.bump = *ctx.bumps.get("nft_metadata").unwrap();
    
    // Initialize NFT origin tracking
    let nft_origin = &mut ctx.accounts.nft_origin;
    nft_origin.token_id = token_id;
    nft_origin.original_mint = ctx.accounts.mint.key();
    nft_origin.original_metadata_uri = metadata_uri;
    nft_origin.source_chain_id = zeta_chain_id;
    nft_origin.created_at = clock.unix_timestamp;
    nft_origin.bump = *ctx.bumps.get("nft_origin").unwrap();
    
    // Update program state
    program_state.total_minted += 1;
    program_state.next_token_id += 1;
    
    msg!("NFT minted successfully");
    msg!("Mint address: {}", ctx.accounts.mint.key());
    msg!("Owner: {}", ctx.accounts.mint_authority.key());
    msg!("Token ID: {}", token_id);
    msg!("ZetaChain ID: {}", zeta_chain_id);
    msg!("Total minted: {}", program_state.total_minted);
    msg!("Next token ID: {}", program_state.next_token_id);
    
    Ok(())
}
