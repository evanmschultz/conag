#!/bin/bash

set -e

# Set variables
REPO="evanmschultz/conag"
ASSET_NAME="conag"
INSTALL_DIR="/usr/local/bin"

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS. This installer currently only supports macOS."
    exit 1
fi

# Construct the download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$ASSET_NAME"

echo "Downloading $ASSET_NAME from $DOWNLOAD_URL..."

# Download the latest release asset
curl -L "$DOWNLOAD_URL" -o "$ASSET_NAME"

# Verify that the file was downloaded
if [ ! -f "$ASSET_NAME" ]; then
    echo "Error: Failed to download $ASSET_NAME"
    exit 1
fi

# Make binary executable
chmod +x "$ASSET_NAME"

# Move binary to install directory
sudo mv "$ASSET_NAME" "$INSTALL_DIR"

echo "$ASSET_NAME has been installed to $INSTALL_DIR"
echo "You may need to restart your terminal or run 'source ~/.bash_profile' (or equivalent) to use $ASSET_NAME from anywhere."