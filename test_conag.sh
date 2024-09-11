#!/bin/bash

# Set up temporary directory on the desktop
DESKTOP_DIR="$HOME/Desktop"
TEMP_DIR="$DESKTOP_DIR/conag_temp"
TEST_PROJECT_DIR="$TEMP_DIR/test_project"
TEST_OUTPUT_DIR="$TEMP_DIR/test_output"
TEST_CONFIG="$TEMP_DIR/test_config.toml"

# Create temporary directories
mkdir -p "$TEST_PROJECT_DIR/src" "$TEST_PROJECT_DIR/docs"
mkdir -p "$TEST_OUTPUT_DIR"

# Create test files
echo "fn main() { println!(\"Hello, World!\"); }" > "$TEST_PROJECT_DIR/src/main.rs"
echo "# Test Documentation" > "$TEST_PROJECT_DIR/docs/readme.md"
echo "secret_key=12345" > "$TEST_PROJECT_DIR/.env"

# Build the application
echo "Building the application..."
cargo build --features dev

# Generate default config
echo "Generating default config..."
cargo run --features dev -- --generate-config

# Create test config
cat > "$TEST_CONFIG" << EOL
input_dir = "$TEST_PROJECT_DIR"
output_dir = "$TEST_OUTPUT_DIR"
ignore_patterns = ["*.tmp"]
include_hidden_patterns = [".env"]
EOL

echo "Contents of test config file:"
cat "$TEST_CONFIG"

# Function to run conag and check for errors
run_conag() {
    echo "Running: $1"
    if ! eval $1; then
        echo "Error occurred. Exiting."
        exit 1
    fi
}

# Test cases
echo "Running with default config..."
run_conag "cargo run --features dev"

echo "Running with custom config..."
run_conag "cargo run --features dev -- --config \"$TEST_CONFIG\""

echo "Running with custom config and include hidden files..."
run_conag "cargo run --features dev -- --config \"$TEST_CONFIG\" --include-hidden .env"

echo "Running with custom config and plain text output..."
run_conag "cargo run --features dev -- --config \"$TEST_CONFIG\" --plain-text"

echo "Test runs completed."
echo "Temporary files and outputs are located in: $TEMP_DIR"
echo "Config file is located at: $TEST_CONFIG"
echo "Output files are located in: $TEST_OUTPUT_DIR"

# Ask user if they want to delete the temporary files
read -p "Are you done reviewing the outputs? (yes/no): " user_response

if [ "$user_response" = "yes" ]; then
    echo "Deleting temporary files and directories..."
    rm -rf "$TEMP_DIR"
    echo "Cleanup completed."
elif [ "$user_response" = "no" ]; then
    echo "Temporary files and directories have been left for your review."
    echo "You can find them at: $TEMP_DIR"
    echo "Remember to delete them manually when you're done."
else
    echo "Invalid response. If you want to delete the temporary files and directories manually, run:"
    echo "rm -rf \"$TEMP_DIR\""
fi

echo "Test script completed."