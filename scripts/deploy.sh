#!/bin/bash

# ZetaChain Universal NFT Program Deployment Script
# This script deploys the program to different networks and demonstrates the cross-chain NFT flow

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROGRAM_NAME="zetachain_universal_nft"
AUTHORITY_KEYPAIR="$HOME/.config/solana/id.json"
NETWORK=${1:-devnet}

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Solana CLI is installed
    if ! command -v solana &> /dev/null; then
        print_error "Solana CLI is not installed. Please install it first."
        exit 1
    fi
    
    # Check if Anchor is installed
    if ! command -v anchor &> /dev/null; then
        print_error "Anchor CLI is not installed. Please install it first."
        exit 1
    fi
    
    # Check if keypair exists
    if [ ! -f "$AUTHORITY_KEYPAIR" ]; then
        print_error "Authority keypair not found at $AUTHORITY_KEYPAIR"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to build the program
build_program() {
    print_status "Building the program..."
    
    if anchor build; then
        print_success "Program built successfully"
    else
        print_error "Failed to build program"
        exit 1
    fi
}

# Function to deploy to specified network
deploy_program() {
    print_status "Deploying to $NETWORK..."
    
    # Set network configuration
    case $NETWORK in
        "localnet")
            NETWORK_URL="http://127.0.0.1:8899"
            ;;
        "devnet")
            NETWORK_URL="https://api.devnet.solana.com"
            ;;
        "testnet")
            NETWORK_URL="https://api.testnet.solana.com"
            ;;
        "mainnet")
            NETWORK_URL="https://api.mainnet-beta.solana.com"
            ;;
        *)
            print_error "Invalid network: $NETWORK"
            print_status "Supported networks: localnet, devnet, testnet, mainnet"
            exit 1
            ;;
    esac
    
    # Configure Solana CLI
    solana config set --url "$NETWORK_URL"
    solana config set --keypair "$AUTHORITY_KEYPAIR"
    
    # Get program ID from build
    PROGRAM_ID=$(solana address -k target/deploy/${PROGRAM_NAME}-keypair.json)
    print_status "Program ID: $PROGRAM_ID"
    
    # Deploy program
    if anchor deploy --provider.cluster "$NETWORK"; then
        print_success "Program deployed successfully to $NETWORK"
        print_status "Program ID: $PROGRAM_ID"
        
        # Update Anchor.toml with new program ID
        sed -i.bak "s/zetachain_universal_nft = \".*\"/zetachain_universal_nft = \"$PROGRAM_ID\"/" Anchor.toml
        print_status "Updated Anchor.toml with new program ID"
        
    else
        print_error "Failed to deploy program to $NETWORK"
        exit 1
    fi
}

# Function to verify deployment
verify_deployment() {
    print_status "Verifying deployment..."
    
    if solana program show "$PROGRAM_ID" --url "$NETWORK_URL"; then
        print_success "Program deployment verified"
    else
        print_error "Program deployment verification failed"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    if anchor test --provider.cluster "$NETWORK"; then
        print_success "All tests passed"
    else
        print_warning "Some tests failed. Check the output above for details."
    fi
}

# Function to demonstrate cross-chain NFT flow
demonstrate_nft_flow() {
    print_status "Demonstrating cross-chain NFT flow..."
    
    # This would typically involve:
    # 1. Minting an NFT on Solana
    # 2. Initiating cross-chain transfer to ZetaChain
    # 3. Transferring to Ethereum
    # 4. Transferring to BNB (BSC)
    # 5. Transferring back to Solana
    
    print_warning "Cross-chain NFT flow demonstration requires ZetaChain testnet setup"
    print_status "Please refer to the README.md for detailed testing instructions"
}

# Function to show deployment info
show_deployment_info() {
    print_status "Deployment Information:"
    echo "  Network: $NETWORK"
    echo "  Network URL: $NETWORK_URL"
    echo "  Program ID: $PROGRAM_ID"
    echo "  Authority: $(solana address --keypair "$AUTHORITY_KEYPAIR")"
    echo "  Solana Version: $(solana --version)"
    echo "  Anchor Version: $(anchor --version)"
}

# Main deployment flow
main() {
    echo "=========================================="
    echo "ZetaChain Universal NFT Program Deployment"
    echo "=========================================="
    echo ""
    
    check_prerequisites
    build_program
    deploy_program
    verify_deployment
    run_tests
    demonstrate_nft_flow
    show_deployment_info
    
    echo ""
    print_success "Deployment completed successfully!"
    print_status "Next steps:"
    echo "  1. Test the program functionality"
    echo "  2. Configure ZetaChain gateway settings"
    echo "  3. Test cross-chain NFT transfers"
    echo "  4. Monitor program performance"
    echo ""
    print_status "For more information, see README.md"
}

# Handle command line arguments
case $1 in
    "help"|"-h"|"--help")
        echo "Usage: $0 [network]"
        echo ""
        echo "Networks:"
        echo "  localnet  - Deploy to local Solana cluster"
        echo "  devnet    - Deploy to Solana devnet (default)"
        echo "  testnet   - Deploy to Solana testnet"
        echo "  mainnet   - Deploy to Solana mainnet"
        echo ""
        echo "Examples:"
        echo "  $0              # Deploy to devnet"
        echo "  $0 localnet     # Deploy to localnet"
        echo "  $0 mainnet      # Deploy to mainnet"
        exit 0
        ;;
esac

# Run main function
main
