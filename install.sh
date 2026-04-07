#!/bin/bash

# ss-mac Binary Installation Script for macOS

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}==>${NC} Downloading the latest ss-mac binary from GitHub..."

# The URL to the latest release asset
BINARY_URL="https://github.com/wankunde/ss-mac/releases/latest/download/ss-mac"
TMP_BIN="/tmp/ss-mac"

# Download the binary
curl -L -o "$TMP_BIN" "$BINARY_URL"

if [ $? -ne 0 ]; then
    echo -e "${RED}Error:${NC} Failed to download the binary. Please check your internet connection."
    exit 1
fi

# Make the binary executable
chmod +x "$TMP_BIN"

INSTALL_DIR="/usr/local/bin"

# Create /usr/local/bin if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${GREEN}==>${NC} Creating directory $INSTALL_DIR..."
    sudo mkdir -p "$INSTALL_DIR"
fi

echo -e "${GREEN}==>${NC} Installing to $INSTALL_DIR..."

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_BIN" "$INSTALL_DIR/ss-mac"
    echo -e "${GREEN}==>${NC} Successfully installed ss-mac!"
else
    echo -e "${GREEN}==>${NC} Requesting sudo to install to $INSTALL_DIR..."
    sudo mv "$TMP_BIN" "$INSTALL_DIR/ss-mac"
    echo -e "${GREEN}==>${NC} Successfully installed ss-mac!"
fi

echo -e "${GREEN}==>${NC} You can now run 'ss-mac' from your terminal."
