#!/bin/bash

# ZetaChain Universal NFT Program - Local Network Setup
# This script sets up a local Solana cluster and demonstrates the complete cross-chain NFT flow
# Flow: ZetaChain -> Ethereum -> BNB (BSC) -> Solana -> ZetaChain

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
PROGRAM_NAME="zetachain_universal_nft"
AUTHORITY_KEYPAIR="$HOME/.config/solana/id.json"
LOCALNET_URL="http://127.0.0.1:8899"
LOCALNET_RPC_URL="http://127.0.0.1:8899"
LOCALNET_WS_URL="ws://127.0.0.1:8899"

# Test accounts
TEST_USER1_KEYPAIR="$HOME/.config/solana/test-user1.json"
TEST_USER2_KEYPAIR="$HOME/.config/solana/test-user2.json"
TEST_USER3_KEYPAIR="$HOME/.config/solana/test-user3.json"

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

print_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

print_demo() {
    echo -e "${CYAN}[DEMO]${NC} $1"
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

# Function to start local Solana cluster
start_local_cluster() {
    print_step "Starting local Solana cluster..."
    
    # Kill any existing local validator
    pkill -f "solana-test-validator" || true
    sleep 2
    
    # Start local validator
    solana-test-validator \
        --rpc-port 8899 \
        --ws-port 8900 \
        --faucet-port 9900 \
        --quiet &
    
    # Wait for cluster to be ready
    print_status "Waiting for local cluster to be ready..."
    sleep 10
    
    # Configure Solana CLI for localnet
    solana config set --url "$LOCALNET_URL"
    solana config set --keypair "$AUTHORITY_KEYPAIR"
    
    # Check cluster status
    if solana cluster-version; then
        print_success "Local Solana cluster started successfully"
    else
        print_error "Failed to start local Solana cluster"
        exit 1
    fi
}

# Function to create test accounts
create_test_accounts() {
    print_step "Creating test accounts..."
    
    # Create test user keypairs
    solana-keygen new --no-bip39-passphrase -o "$TEST_USER1_KEYPAIR"
    solana-keygen new --no-bip39-passphrase -o "$TEST_USER2_KEYPAIR"
    solana-keygen new --no-bip39-passphrase -o "$TEST_USER3_KEYPAIR"
    
    # Airdrop SOL to test accounts
    solana airdrop 10 "$(solana-keygen pubkey "$TEST_USER1_KEYPAIR")"
    solana airdrop 10 "$(solana-keygen pubkey "$TEST_USER2_KEYPAIR")"
    solana airdrop 10 "$(solana-keygen pubkey "$TEST_USER3_KEYPAIR")"
    
    print_success "Test accounts created and funded"
}

# Function to build and deploy program
deploy_program() {
    print_step "Building and deploying program..."
    
    # Build program
    if anchor build; then
        print_success "Program built successfully"
    else
        print_error "Failed to build program"
        exit 1
    fi
    
    # Get program ID
    PROGRAM_ID=$(solana address -k target/deploy/${PROGRAM_NAME}-keypair.json)
    print_status "Program ID: $PROGRAM_ID"
    
    # Deploy to localnet
    if anchor deploy --provider.cluster localnet; then
        print_success "Program deployed successfully to localnet"
        
        # Update Anchor.toml with new program ID
        sed -i.bak "s/zetachain_universal_nft = \".*\"/zetachain_universal_nft = \"$PROGRAM_ID\"/" Anchor.toml
        print_status "Updated Anchor.toml with new program ID"
        
    else
        print_error "Failed to deploy program to localnet"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    print_step "Running tests on localnet..."
    
    if anchor test --provider.cluster localnet; then
        print_success "All tests passed on localnet"
    else
        print_warning "Some tests failed. Check the output above for details."
    fi
}

# Function to demonstrate cross-chain NFT flow
demonstrate_cross_chain_flow() {
    print_demo "Demonstrating Complete Cross-Chain NFT Flow"
    echo "=================================================="
    echo ""
    
    print_step "Step 1: Initialize Program on Solana"
    echo "  - Deploy Universal NFT program"
    echo "  - Configure ZetaChain gateway settings"
    echo "  - Set up supported chain IDs"
    echo ""
    
    print_step "Step 2: Mint NFT on ZetaChain (Simulated)"
    echo "  - Create NFT with metadata"
    echo "  - Assign ownership to test user"
    echo "  - Generate cross-chain transfer data"
    echo ""
    
    print_step "Step 3: Transfer NFT from ZetaChain to Ethereum"
    echo "  - Initiate cross-chain transfer"
    echo "  - Burn NFT on ZetaChain"
    echo "  - Mint NFT on Ethereum (simulated)"
    echo "  - Verify ownership transfer"
    echo ""
    
    print_step "Step 4: Transfer NFT from Ethereum to BNB (BSC)"
    echo "  - Initiate transfer from Ethereum to BSC"
    echo "  - Burn NFT on Ethereum (simulated)"
    echo "  - Mint NFT on BSC (simulated)"
    echo "  - Verify cross-chain ownership"
    echo ""
    
    print_step "Step 5: Transfer NFT from BNB to Solana"
    echo "  - Initiate transfer from BSC to Solana"
    echo "  - Burn NFT on BSC (simulated)"
    echo "  - Process incoming NFT on Solana"
    echo "  - Mint NFT with cross-chain metadata"
    echo ""
    
    print_step "Step 6: Transfer NFT back to ZetaChain"
    echo "  - Initiate transfer from Solana to ZetaChain"
    echo "  - Burn NFT on Solana"
    echo "  - Process incoming NFT on ZetaChain (simulated)"
    echo "  - Complete the full circle"
    echo ""
    
    print_warning "Note: Steps 2-6 are simulated for demonstration purposes"
    print_status "Real cross-chain transfers require ZetaChain testnet setup"
}

# Function to run interactive demo
run_interactive_demo() {
    print_demo "Starting Interactive Demo..."
    echo ""
    
    # This would be an interactive session where users can:
    # 1. Initialize the program
    # 2. Mint NFTs
    # 3. Perform cross-chain transfers
    # 4. Verify ownership
    # 5. Test various scenarios
    
    print_status "Interactive demo would include:"
    echo "  - Program initialization"
    echo "  - NFT minting and burning"
    echo "  - Cross-chain transfer simulation"
    echo "  - Ownership verification"
    echo "  - Error handling demonstration"
    echo ""
    
    print_warning "Interactive demo requires additional setup and user interaction"
}

# Function to show localnet information
show_localnet_info() {
    print_status "Local Network Information:"
    echo "  Solana RPC URL: $LOCALNET_RPC_URL"
    echo "  Solana WS URL: $LOCALNET_WS_URL"
    echo "  Program ID: $PROGRAM_ID"
    echo "  Authority: $(solana address --keypair "$AUTHORITY_KEYPAIR")"
    echo "  Test User 1: $(solana-keygen pubkey "$TEST_USER1_KEYPAIR")"
    echo "  Test User 2: $(solana-keygen pubkey "$TEST_USER2_KEYPAIR")"
    echo "  Test User 3: $(solana-keygen pubkey "$TEST_USER3_KEYPAIR")"
    echo ""
}

# Function to cleanup
cleanup() {
    print_status "Cleaning up..."
    
    # Kill local validator
    pkill -f "solana-test-validator" || true
    
    # Remove test keypairs
    rm -f "$TEST_USER1_KEYPAIR"
    rm -f "$TEST_USER2_KEYPAIR"
    rm -f "$TEST_USER3_KEYPAIR"
    
    print_success "Cleanup completed"
}

# Function to show help
show_help() {
    echo "ZetaChain Universal NFT Program - Local Network Setup"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  start       - Start local Solana cluster and deploy program"
    echo "  stop        - Stop local Solana cluster and cleanup"
    echo "  demo        - Run cross-chain NFT flow demonstration"
    echo "  test        - Run tests on localnet"
    echo "  info        - Show localnet information"
    echo "  help        - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 start    - Start localnet and deploy program"
    echo "  $0 demo     - Run demonstration"
    echo "  $0 stop     - Stop and cleanup"
    echo ""
}

# Main function
main() {
    case $1 in
        "start")
            echo "=========================================="
            echo "Starting ZetaChain Universal NFT Localnet"
            echo "=========================================="
            echo ""
            
            check_prerequisites
            start_local_cluster
            create_test_accounts
            deploy_program
            run_tests
            show_localnet_info
            
            echo ""
            print_success "Localnet setup completed successfully!"
            print_status "Next steps:"
            echo "  1. Run '$0 demo' to see cross-chain flow"
            echo "  2. Run '$0 test' to run tests"
            echo "  3. Run '$0 stop' to cleanup"
            echo ""
            ;;
            
        "stop")
            echo "=========================================="
            echo "Stopping ZetaChain Universal NFT Localnet"
            echo "=========================================="
            echo ""
            
            cleanup
            print_success "Localnet stopped and cleaned up"
            ;;
            
        "demo")
            echo "=========================================="
            echo "Cross-Chain NFT Flow Demonstration"
            echo "=========================================="
            echo ""
            
            demonstrate_cross_chain_flow
            run_interactive_demo
            ;;
            
        "test")
            echo "=========================================="
            echo "Running Tests on Localnet"
            echo "=========================================="
            echo ""
            
            run_tests
            ;;
            
        "info")
            echo "=========================================="
            echo "Local Network Information"
            echo "=========================================="
            echo ""
            
            show_localnet_info
            ;;
            
        "help"|"-h"|"--help"|"")
            show_help
            ;;
            
        *)
            print_error "Unknown command: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Handle script interruption
trap cleanup EXIT

# Run main function
main "$@"
