#!/bin/bash

set -e

# Function to create or update a file
create_or_update_file() {
    local file_path="$1"
    local content="$2"
    
    if [ -f "$file_path" ]; then
        read -p "File $file_path already exists. Do you want to overwrite it? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "$content" > "$file_path"
            echo "Updated $file_path"
        else
            echo "Skipped updating $file_path"
        fi
    else
        echo "$content" > "$file_path"
        echo "Created $file_path"
    fi
}

# Function to create directory if it doesn't exist
create_dir_if_not_exists() {
    if [ ! -d "$1" ]; then
        mkdir -p "$1"
        echo "Created directory: $1"
    fi
}

# Create .github/workflows directory
create_dir_if_not_exists ".github/workflows"

# Create or update release.yml for GitHub Actions
RELEASE_YML_CONTENT=$(cat << EOF
name: Release

on:
  release:
    types: [created]

jobs:
  build:
    runs-on: macos-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Build
      run: cargo build --release
    - name: Rename binary
      run: mv target/release/conag target/release/conag-macos
    - name: Upload Release Asset
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: \${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: \${{ github.event.release.upload_url }}
        asset_path: ./target/release/conag-macos
        asset_name: conag-macos
        asset_content_type: application/octet-stream
EOF
)

create_or_update_file ".github/workflows/release.yml" "$RELEASE_YML_CONTENT"

# Create or update install.sh in the root directory
INSTALL_SH_CONTENT=$(cat << EOF
#!/bin/bash

set -e

# Detect OS
if [[ "\$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS. This installer currently only supports macOS."
    exit 1
fi

# Set install directory
INSTALL_DIR="/usr/local/bin"

# Download latest release
LATEST_RELEASE_URL=\$(curl -s https://api.github.com/repos/evanmschultz/conag/releases/latest | grep "browser_download_url.*conag-\$OS" | cut -d '"' -f 4)
curl -L \$LATEST_RELEASE_URL -o conag

# Make binary executable
chmod +x conag

# Move binary to install directory
sudo mv conag \$INSTALL_DIR

echo "conag has been installed to \$INSTALL_DIR"
echo "You may need to restart your terminal or run 'source ~/.bash_profile' (or equivalent) to use conag from anywhere."
EOF
)

create_or_update_file "./install.sh" "$INSTALL_SH_CONTENT"

# Make install.sh executable
chmod +x ./install.sh

# Git operations
git add .github/workflows/release.yml ./install.sh

# Check if there are changes to commit
if ! git diff --cached --exit-code; then
    git commit -m "Updates GitHub Actions workflow and installation script"
    echo "Changes committed."
else
    echo "No changes to commit."
fi

# Tag the release
read -p "Enter the version number for this release (e.g., 0.1.0): " VERSION
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo "Tag v$VERSION already exists. Please choose a different version number."
else
    git tag -a "v$VERSION" -m "Release version $VERSION"
    echo "Created tag v$VERSION"
fi

# Push changes and tags to GitHub
read -p "Do you want to push changes and tags to GitHub? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin main
    git push origin "v$VERSION"
    echo "Changes and tags pushed to GitHub."
    echo "Now go to GitHub and create a new release using the tag v$VERSION"
    echo "The GitHub Action will automatically build and attach the binary to the release."
else
    echo "Changes and tags were not pushed to GitHub. You can push them manually when ready."
fi