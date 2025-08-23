import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZetachainUniversalNft } from "../target/types/zetachain_universal_nft";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  createMintToInstruction,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import { assert } from "chai";

describe("zetachain-universal-nft", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ZetachainUniversalNft as Program<ZetachainUniversalNft>;
  
  // Test accounts
  const authority = Keypair.generate();
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();
  const mint1 = Keypair.generate();
  const mint2 = Keypair.generate();
  
  // PDAs
  let programStatePda: PublicKey;
  let gatewayStatePda: PublicKey;
  let nftMetadata1Pda: PublicKey;
  let nftMetadata2Pda: PublicKey;
  let crossChainTransferPda: PublicKey;
  let ownershipVerificationPda: PublicKey;
  
  // Token accounts
  let user1TokenAccount: PublicKey;
  let user2TokenAccount: PublicKey;
  
  // Test data
  const testMetadataUri = "https://example.com/metadata.json";
  const testMaxSupply = 1000;
  const testZetaChainId = 2; // Ethereum
  const testCrossChainData = new Uint8Array([1, 2, 3, 4, 5]);
  const testRecipient = new Uint8Array([0x74, 0x2d, 0x3b, 0x1a, 0x4c, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xd0, 0xe1, 0xf2, 0x34, 0x56, 0x78, 0x9a]);
  const testGatewayAddress = new Uint8Array([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78]);

  before(async () => {
    // Airdrop SOL to test accounts
    const signature1 = await provider.connection.requestAirdrop(authority.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature1);
    
    const signature2 = await provider.connection.requestAirdrop(user1.publicKey, 5 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature2);
    
    const signature3 = await provider.connection.requestAirdrop(user2.publicKey, 5 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature3);
    
    // Find PDAs
    [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId
    );
    
    [gatewayStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("gateway_state")],
      program.programId
    );
    
    [nftMetadata1Pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_metadata"), mint1.publicKey.toBuffer()],
      program.programId
    );
    
    [nftMetadata2Pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_metadata"), mint2.publicKey.toBuffer()],
      program.programId
    );
    
    [crossChainTransferPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("cross_chain_transfer"), mint1.publicKey.toBuffer()],
      program.programId
    );
    
    [ownershipVerificationPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("ownership_verification"), mint1.publicKey.toBuffer()],
      program.programId
    );
    
    // Get associated token accounts
    user1TokenAccount = await getAssociatedTokenAddress(
      mint1.publicKey,
      user1.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    
    user2TokenAccount = await getAssociatedTokenAddress(
      mint2.publicKey,
      user2.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
  });

  it("Initializes the Universal NFT program", async () => {
    try {
      const tx = await program.methods
        .initialize(testMetadataUri, new anchor.BN(testMaxSupply))
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([authority])
        .rpc();
      
      console.log("Program initialized successfully. Transaction signature:", tx);
      
      // Verify program state
      const programState = await program.account.programState.fetch(programStatePda);
      assert.equal(programState.authority.toString(), authority.publicKey.toString());
      assert.equal(programState.totalMinted.toNumber(), 0);
      assert.equal(programState.maxSupply.toNumber(), testMaxSupply);
      
      // Verify gateway state
      const gatewayState = await program.account.zetaChainGatewayState.fetch(gatewayStatePda);
      assert.equal(gatewayState.version, 1);
      assert.isTrue(gatewayState.supportedChains.length > 0);
      
    } catch (error) {
      console.error("Error initializing program:", error);
      throw error;
    }
  });

  it("Mints a new NFT", async () => {
    try {
      const tx = await program.methods
        .mintNft(
          testMetadataUri,
          new anchor.BN(testZetaChainId),
          testCrossChainData
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          mint: mint1.publicKey,
          mintAta: user1TokenAccount,
          nftMetadata: nftMetadata1Pda,
          payer: user1.publicKey,
          mintAuthority: user1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user1, mint1])
        .rpc();
      
      console.log("NFT minted successfully. Transaction signature:", tx);
      
      // Verify NFT metadata
      const nftMetadata = await program.account.nftMetadata.fetch(nftMetadata1Pda);
      assert.equal(nftMetadata.mint.toString(), mint1.publicKey.toString());
      assert.equal(nftMetadata.owner.toString(), user1.publicKey.toString());
      assert.equal(nftMetadata.metadataUri, testMetadataUri);
      assert.equal(nftMetadata.zetaChainId.toNumber(), testZetaChainId);
      
      // Verify program state
      const programState = await program.account.programState.fetch(programStatePda);
      assert.equal(programState.totalMinted.toNumber(), 1);
      
    } catch (error) {
      console.error("Error minting NFT:", error);
      throw error;
    }
  });

  it("Updates NFT metadata", async () => {
    try {
      const newMetadataUri = "https://example.com/new-metadata.json";
      
      const tx = await program.methods
        .updateMetadata(newMetadataUri)
        .accounts({
          nftMetadata: nftMetadata1Pda,
          nftMint: mint1.publicKey,
          owner: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();
      
      console.log("Metadata updated successfully. Transaction signature:", tx);
      
      // Verify updated metadata
      const nftMetadata = await program.account.nftMetadata.fetch(nftMetadata1Pda);
      assert.equal(nftMetadata.metadataUri, newMetadataUri);
      
    } catch (error) {
      console.error("Error updating metadata:", error);
      throw error;
    }
  });

  it("Initiates cross-chain transfer", async () => {
    try {
      const targetChainId = 3; // BSC
      
      const tx = await program.methods
        .crossChainTransfer(
          new anchor.BN(targetChainId),
          testRecipient,
          testCrossChainData
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          nftMetadata: nftMetadata1Pda,
          nftMint: mint1.publicKey,
          ownerTokenAccount: user1TokenAccount,
          transferState: crossChainTransferPda,
          owner: user1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user1])
        .rpc();
      
      console.log("Cross-chain transfer initiated. Transaction signature:", tx);
      
      // Verify transfer state
      const transferState = await program.account.crossChainTransferState.fetch(crossChainTransferPda);
      assert.equal(transferState.nftMint.toString(), mint1.publicKey.toString());
      assert.equal(transferState.sourceChainId.toNumber(), 1); // Solana
      assert.equal(transferState.targetChainId.toNumber(), targetChainId);
      assert.equal(transferState.status, 1); // InProgress
      
      // Verify NFT is burned (owner cleared)
      const nftMetadata = await program.account.nftMetadata.fetch(nftMetadata1Pda);
      assert.equal(nftMetadata.owner.toString(), "11111111111111111111111111111111");
      
    } catch (error) {
      console.error("Error initiating cross-chain transfer:", error);
      throw error;
    }
  });

  it("Processes incoming NFT from another chain", async () => {
    try {
      const sourceChainId = 2; // Ethereum
      const incomingMetadataUri = "https://example.com/incoming-metadata.json";
      const incomingCrossChainData = new Uint8Array([5, 4, 3, 2, 1]);
      const zetaTxHash = new Uint8Array(32).fill(1);
      
      const tx = await program.methods
        .processIncomingNft(
          incomingMetadataUri,
          new anchor.BN(sourceChainId),
          incomingCrossChainData,
          zetaTxHash
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          transferState: crossChainTransferPda,
          incomingNftMint: mint2.publicKey,
          recipientTokenAccount: user2TokenAccount,
          nftMetadata: nftMetadata2Pda,
          payer: user2.publicKey,
          recipient: user2.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user2, mint2])
        .rpc();
      
      console.log("Incoming NFT processed successfully. Transaction signature:", tx);
      
      // Verify NFT metadata
      const nftMetadata = await program.account.nftMetadata.fetch(nftMetadata2Pda);
      assert.equal(nftMetadata.mint.toString(), mint2.publicKey.toString());
      assert.equal(nftMetadata.owner.toString(), user2.publicKey.toString());
      assert.equal(nftMetadata.metadataUri, incomingMetadataUri);
      assert.equal(nftMetadata.zetaChainId.toNumber(), sourceChainId);
      
      // Verify transfer state updated
      const transferState = await program.account.crossChainTransferState.fetch(crossChainTransferPda);
      assert.equal(transferState.status, 2); // Completed
      
      // Verify program state
      const programState = await program.account.programState.fetch(programStatePda);
      assert.equal(programState.totalMinted.toNumber(), 1);
      
    } catch (error) {
      console.error("Error processing incoming NFT:", error);
      throw error;
    }
  });

  it("Verifies cross-chain ownership", async () => {
    try {
      const proofData = new Uint8Array([1, 2, 3, 4, 5]);
      
      const tx = await program.methods
        .verifyCrossChainOwnership(proofData)
        .accounts({
          gatewayState: gatewayStatePda,
          nftMetadata: nftMetadata2Pda,
          verificationState: ownershipVerificationPda,
          nftMint: mint2.publicKey,
          verifier: user2.publicKey,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user2])
        .rpc();
      
      console.log("Cross-chain ownership verified. Transaction signature:", tx);
      
      // Verify verification state
      const verificationState = await program.account.ownershipVerificationState.fetch(ownershipVerificationPda);
      assert.equal(verificationState.nftMint.toString(), mint2.publicKey.toString());
      assert.isTrue(verificationState.verified);
      
    } catch (error) {
      console.error("Error verifying cross-chain ownership:", error);
      throw error;
    }
  });

  it("Updates gateway configuration", async () => {
    try {
      const newSupportedChains = [1, 2, 3, 4, 5]; // Solana, Ethereum, BSC, Polygon, Avalanche
      const newVersion = 2;
      
      const tx = await program.methods
        .setupGateway(
          testGatewayAddress,
          newSupportedChains,
          newVersion
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
      
      console.log("Gateway configuration updated. Transaction signature:", tx);
      
      // Verify gateway state
      const gatewayState = await program.account.zetaChainGatewayState.fetch(gatewayStatePda);
      assert.deepEqual(gatewayState.gatewayAddress, Array.from(testGatewayAddress));
      assert.deepEqual(gatewayState.supportedChains, newSupportedChains);
      assert.equal(gatewayState.version, newVersion);
      
    } catch (error) {
      console.error("Error updating gateway configuration:", error);
      throw error;
    }
  });

  it("Transfers NFT between users", async () => {
    try {
      // First mint a new NFT for user1
      const mint3 = Keypair.generate();
      const [nftMetadata3Pda] = PublicKey.findProgramAddressSync(
        [Buffer.from("nft_metadata"), mint3.publicKey.toBuffer()],
        program.programId
      );
      
      const user1TokenAccount3 = await getAssociatedTokenAddress(
        mint3.publicKey,
        user1.publicKey,
        false,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      
      const user2TokenAccount3 = await getAssociatedTokenAddress(
        mint3.publicKey,
        user2.publicKey,
        false,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      
      // Mint NFT
      await program.methods
        .mintNft(
          testMetadataUri,
          new anchor.BN(testZetaChainId),
          testCrossChainData
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          mint: mint3.publicKey,
          mintAta: user1TokenAccount3,
          nftMetadata: nftMetadata3Pda,
          payer: user1.publicKey,
          mintAuthority: user1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user1, mint3])
        .rpc();
      
      // Transfer NFT from user1 to user2
      const tx = await program.methods
        .transferNft(user2.publicKey)
        .accounts({
          nftMetadata: nftMetadata3Pda,
          nftMint: mint3.publicKey,
          ownerTokenAccount: user1TokenAccount3,
          newOwnerTokenAccount: user2TokenAccount3,
          owner: user1.publicKey,
          newOwner: user2.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user1, user2])
        .rpc();
      
      console.log("NFT transferred successfully. Transaction signature:", tx);
      
      // Verify transfer
      const nftMetadata = await program.account.nftMetadata.fetch(nftMetadata3Pda);
      assert.equal(nftMetadata.owner.toString(), user2.publicKey.toString());
      
    } catch (error) {
      console.error("Error transferring NFT:", error);
      throw error;
    }
  });

  it("Burns an NFT", async () => {
    try {
      // First mint a new NFT for user1
      const mint4 = Keypair.generate();
      const [nftMetadata4Pda] = PublicKey.findProgramAddressSync(
        [Buffer.from("nft_metadata"), mint4.publicKey.toBuffer()],
        program.programId
      );
      
      const user1TokenAccount4 = await getAssociatedTokenAddress(
        mint4.publicKey,
        user1.publicKey,
        false,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      
      // Mint NFT
      await program.methods
        .mintNft(
          testMetadataUri,
          new anchor.BN(testZetaChainId),
          testCrossChainData
        )
        .accounts({
          programState: programStatePda,
          gatewayState: gatewayStatePda,
          mint: mint4.publicKey,
          mintAta: user1TokenAccount4,
          nftMetadata: nftMetadata4Pda,
          payer: user1.publicKey,
          mintAuthority: user1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user1, mint4])
        .rpc();
      
      // Burn NFT
      const tx = await program.methods
        .burnNft()
        .accounts({
          programState: programStatePda,
          nftMetadata: nftMetadata4Pda,
          nftMint: mint4.publicKey,
          ownerTokenAccount: user1TokenAccount4,
          owner: user1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user1])
        .rpc();
      
      console.log("NFT burned successfully. Transaction signature:", tx);
      
      // Verify program state
      const programState = await program.account.programState.fetch(programStatePda);
      assert.equal(programState.totalMinted.toNumber(), 2); // Should be 2 after minting and burning
      
    } catch (error) {
      console.error("Error burning NFT:", error);
      throw error;
    }
  });
});
