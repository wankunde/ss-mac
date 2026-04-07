#!/bin/bash

# ss-mac Automatic Release & Sync Script

# Check for commit message
MSG=${1:-"automated release"}

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 1. Local Build
echo -e "${GREEN}==>${NC} Building locally in release mode..."
cargo build --release
if [ $? -ne 0 ]; then
    echo -e "${RED}Error:${NC} Build failed."
    exit 1
fi

# 2. Versioning
# We'll use current timestamp to generate a unique version tag (v0.1.YYYYMMDD-HHMM)
VERSION="v0.1.$(date +'%Y%m%d-%H%M')"
echo -e "${GREEN}==>${NC} Creating tag: $VERSION"

# 3. Git Push
echo -e "${GREEN}==>${NC} Syncing with GitHub..."
git add .
git commit -m "$MSG"
git tag -a "$VERSION" -m "Release $VERSION"
git push origin main
git push origin "$VERSION"

# 4. Create GitHub Release using 'gh' CLI
echo -e "${GREEN}==>${NC} Creating GitHub Release & Uploading Binary..."
if command -v gh &> /dev/null
then
    gh release create "$VERSION" ./target/release/ss-mac \
        --title "Release $VERSION" \
        --notes "Automatically built and released by local script on $(date)."
    echo -e "${GREEN}==>${NC} Release successfully created on GitHub!"
else
    echo -e "${RED}Warning:${NC} GitHub CLI (gh) not found or not logged in. Tag pushed, but release must be manually created if CI fails."
fi

echo -e "${GREEN}==>${NC} Done! Access your release at https://github.com/wankunde/ss-mac/releases"
