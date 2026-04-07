#!/bin/bash

# ss-mac Sync & Build Script

# Check for a commit message, default to "update"
MSG=${1:-"update"}

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}==>${NC} Syncing with GitHub..."
git add .
git commit -m "$MSG"
git push origin main

echo -e "${GREEN}==>${NC} Compiling the project..."
cargo build --release

echo -e "${GREEN}==>${NC} Done! Run './target/release/ss-mac' to test the new binary."
