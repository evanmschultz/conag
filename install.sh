#!/bin/bash

set -e

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS. This installer currently only supports macOS."
    exit 1
fi

# Set install directory
INSTALL_DIR="/usr/local/bin"

# Download latest release
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/evanmschultz/conag/releases/latest | grep "browser_download_url.*conag-$OS" | cut -d '"' -f 4)
curl -L $LATEST_RELEASE_URL -o conag

# Make binary executable
chmod +x conag

# Move binary to install directory
sudo mv conag $INSTALL_DIR

echo "conag has been installed to $INSTALL_DIR"
echo "You may need to restart your terminal or run 'source ~/.bash_profile' (or equivalent) to use conag from anywhere."
