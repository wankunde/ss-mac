#!/bin/bash

# ss-mac Installation Script for macOS

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}==>${NC} Installing ss-mac..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null
then
    echo -e "${RED}Error:${NC} Rust/Cargo is not installed. Please install it from https://rustup.rs/ first."
    exit 1
fi

# Build the project
echo -e "${GREEN}==>${NC} Building project in release mode..."
cargo build --release

# Move to a directory in PATH
# We'll try to use /usr/local/bin but it might need sudo
INSTALL_DIR="/usr/local/bin"

if [ -w "$INSTALL_DIR" ]; then
    cp target/release/ss-mac "$INSTALL_DIR/"
    echo -e "${GREEN}==>${NC} Successfully installed to $INSTALL_DIR/ss-mac"
else
    echo -e "${GREEN}==>${NC} Requesting sudo to install to $INSTALL_DIR..."
    sudo cp target/release/ss-mac "$INSTALL_DIR/"
    echo -e "${GREEN}==>${NC} Successfully installed to $INSTALL_DIR/ss-mac"
fi

echo -e "${GREEN}==>${NC} You can now run 'ss-mac' from your terminal."
