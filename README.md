# ZetaChain Universal NFT Program

A Solana program that enables cross-chain NFT transfers and interactions between ZetaChain and Solana, implementing the Universal NFT standard for seamless interoperability across multiple blockchain networks.

## 🚀 Features

### Core Functionality
- **Cross-Chain NFT Transfers**: Send NFTs from Solana to other chains (Ethereum, BSC, Polygon, etc.) via ZetaChain
- **Incoming NFT Processing**: Mint NFTs on Solana that originated from other chains
- **Universal Metadata**: Support for cross-chain metadata and ownership verification
- **ZetaChain Gateway Integration**: Seamless integration with ZetaChain's cross-chain messaging protocol

### Security Features
- **TSS (Threshold Signature Scheme) Support**: Enhanced security for cross-chain operations
- **Replay Protection**: Prevents duplicate transaction execution
- **Ownership Verification**: Cryptographic proof verification for cross-chain ownership
- **Access Control**: Role-based permissions for administrative functions

### Solana-Specific Optimizations
- **Compute Budget Management**: Efficient instruction execution
- **Rent Exemption Handling**: Proper account sizing and rent management
- **Token Account Creation**: Automatic associated token account management
- **Signer Validation**: Robust signer verification and management

## 🏗️ Architecture

### Program Structure
```
src/
├── lib.rs                 # Main program entry point
├── state.rs              # Account state definitions
├── errors.rs             # Custom error types
├── constants.rs          # Program constants
└── instructions/         # Instruction handlers
    ├── mod.rs
    ├── initialize.rs     # Program initialization
    ├── mint_nft.rs       # NFT minting
    ├── transfer_nft.rs   # Local NFT transfers
    ├── cross_chain_transfer.rs      # Cross-chain transfers
    ├── process_incoming_nft.rs      # Incoming NFT processing
    ├── verify_cross_chain_ownership.rs # Ownership verification
    ├── update_metadata.rs # Metadata updates
    ├── burn_nft.rs       # NFT burning
    └── setup_gateway.rs  # Gateway configuration
```

### Key Components

#### Program State
- **ProgramState**: Global program configuration and statistics
- **ZetaChainGatewayState**: ZetaChain gateway configuration and supported chains
- **NFTMetadata**: Individual NFT metadata and cross-chain information
- **CrossChainTransferState**: Cross-chain transfer status and tracking
- **OwnershipVerificationState**: Cross-chain ownership verification records

#### Supported Chains
- Solana (Chain ID: 1)
- Ethereum (Chain ID: 2)
- BSC (Chain ID: 3)
- Polygon (Chain ID: 4)
- Avalanche (Chain ID: 5)
- Arbitrum (Chain ID: 6)
- Optimism (Chain ID: 7)
- Base (Chain ID: 8)
- Linea (Chain ID: 9)
- Mantle (Chain ID: 10)
- Scroll (Chain ID: 11)
- Berachain (Chain ID: 12)
- Bitcoin (Chain ID: 13)

## 🛠️ Setup Instructions

### Prerequisites
- Rust 1.70+
- Solana CLI 1.17+
- Anchor Framework 0.29+
- Node.js 18+
- Yarn or npm

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd zetachain_superteam
   ```

2. **Install dependencies**
   ```bash
   yarn install
   # or
   npm install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Generate TypeScript types**
   ```bash
   anchor build
   ```

### Configuration

1. **Update Anchor.toml**
   ```toml
   [programs.devnet]
   zetachain_universal_nft = "YOUR_PROGRAM_ID"
   
   [provider]
   cluster = "devnet"
   wallet = "~/.config/solana/id.json"
   ```

2. **Set your Solana keypair**
   ```bash
   solana config set --keypair ~/.config/solana/id.json
   ```

3. **Switch to devnet**
   ```bash
   solana config set --url devnet
   ```

## 🧪 Testing

### Run Tests
```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/universal-nft.ts

# Run with verbose output
anchor test -- --nocapture
```

### Test Coverage
The test suite covers:
- Program initialization
- NFT minting and burning
- Local NFT transfers
- Cross-chain transfer initiation
- Incoming NFT processing
- Cross-chain ownership verification
- Gateway configuration updates
- Metadata management

## 📖 Usage Examples

### 1. Initialize the Program
```typescript
import { Program } from "@coral-xyz/anchor";
import { ZetachainUniversalNft } from "../target/types/zetachain_universal_nft";

const program = anchor.workspace.ZetachainUniversalNft as Program<ZetachainUniversalNft>;

// Initialize program
const tx = await program.methods
  .initialize("https://example.com/metadata.json", new anchor.BN(1000))
  .accounts({
    programState: programStatePda,
    gatewayState: gatewayStatePda,
    authority: authority.publicKey,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .signers([authority])
  .rpc();
```

### 2. Mint an NFT
```typescript
// Mint NFT with cross-chain metadata
const tx = await program.methods
  .mintNft(
    "https://example.com/metadata.json",
    new anchor.BN(2), // Ethereum chain ID
    new Uint8Array([1, 2, 3, 4, 5]) // Cross-chain data
  )
  .accounts({
    programState: programStatePda,
    gatewayState: gatewayStatePda,
    mint: mint.publicKey,
    mintAta: userTokenAccount,
    nftMetadata: nftMetadataPda,
    payer: user.publicKey,
    mintAuthority: user.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .signers([user, mint])
  .rpc();
```

### 3. Initiate Cross-Chain Transfer
```typescript
// Transfer NFT to Ethereum
const tx = await program.methods
  .crossChainTransfer(
    new anchor.BN(2), // Ethereum chain ID
    recipientAddress, // Ethereum recipient address
    crossChainData
  )
  .accounts({
    programState: programStatePda,
    gatewayState: gatewayStatePda,
    nftMetadata: nftMetadataPda,
    nftMint: nftMint.publicKey,
    ownerTokenAccount: ownerTokenAccount,
    transferState: transferStatePda,
    owner: owner.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .signers([owner])
  .rpc();
```

### 4. Process Incoming NFT
```typescript
// Process NFT coming from Ethereum
const tx = await program.methods
  .processIncomingNft(
    "https://example.com/incoming-metadata.json",
    new anchor.BN(2), // Ethereum chain ID
    incomingCrossChainData,
    zetaTxHash
  )
  .accounts({
    programState: programStatePda,
    gatewayState: gatewayStatePda,
    transferState: transferStatePda,
    incomingNftMint: incomingMint.publicKey,
    recipientTokenAccount: recipientTokenAccount,
    nftMetadata: nftMetadataPda,
    payer: recipient.publicKey,
    recipient: recipient.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .signers([recipient, incomingMint])
  .rpc();
```

## 🔗 ZetaChain Integration

### Gateway Configuration
The program integrates with ZetaChain's protocol contracts to enable cross-chain operations:

1. **Gateway Contract**: Handles cross-chain message passing
2. **TSS Verification**: Ensures message authenticity
3. **Replay Protection**: Prevents duplicate message processing
4. **Chain ID Validation**: Supports all ZetaChain-connected networks

### Cross-Chain Flow
1. **Outgoing Transfer**: NFT burned on Solana → Message sent to ZetaChain → NFT minted on target chain
2. **Incoming Transfer**: NFT burned on source chain → Message received from ZetaChain → NFT minted on Solana

## 🔒 Security Considerations

### Access Control
- **Program Authority**: Only authorized accounts can update gateway configuration
- **NFT Ownership**: Only NFT owners can transfer or burn their NFTs
- **Metadata Updates**: Only NFT owners can update metadata

### Cross-Chain Security
- **TSS Verification**: All cross-chain messages verified through ZetaChain's TSS
- **Replay Protection**: Timestamp-based replay protection for cross-chain operations
- **Data Validation**: Comprehensive validation of cross-chain data and addresses

### Solana Security
- **Account Validation**: Proper account ownership and derivation verification
- **Signer Verification**: Multi-signer support for complex operations
- **Rent Management**: Proper account sizing and rent exemption handling

## 🚀 Deployment

### Devnet Deployment
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Verify deployment
solana program show <PROGRAM_ID> --url devnet
```

### Mainnet Deployment
```bash
# Deploy to mainnet
anchor deploy --provider.cluster mainnet

# Verify deployment
solana program show <PROGRAM_ID> --url mainnet
```

## 📊 Monitoring and Analytics

### Program Metrics
- Total NFTs minted
- Cross-chain transfer success rate
- Gateway configuration updates
- Error rates and types

### Transaction Tracking
- Cross-chain transfer status
- ZetaChain transaction hashes
- Ownership verification records
- Metadata update history

## 🤝 Contributing

### Development Guidelines
1. Follow Rust and Solana best practices
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure security best practices are followed

### Testing Requirements
- All new features must have corresponding tests
- Cross-chain functionality must be tested thoroughly
- Security features must be validated
- Performance benchmarks for critical operations


**Note**: This program is designed for educational and development purposes. Always test thoroughly on devnet before deploying to mainnet. Cross-chain operations involve multiple networks and should be tested extensively to ensure reliability and security.
