#!/bin/bash

# Create a temporary directory for testing
TEST_DIR="/tmp/conag_test_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_DIR"
echo "Created test directory: $TEST_DIR"

# Create a sample project structure
mkdir -p "$TEST_DIR/src" "$TEST_DIR/docs" "$TEST_DIR/.git"
echo "console.log('Hello, world!');" > "$TEST_DIR/src/main.js"
echo "# README" > "$TEST_DIR/README.md"
echo "*.log" > "$TEST_DIR/.gitignore"
echo "Secret key" > "$TEST_DIR/.env"

# Create a test config file
cat << EOF > "$TEST_DIR/test_config.toml"
input_dir = "$TEST_DIR"
output_dir = "$TEST_DIR/output"
ignore_patterns = ["*.log"]
include_hidden_patterns = [".gitignore"]
EOF

# Run conag with the test config
echo "Running conag..."
cargo run -- --config "$TEST_DIR/test_config.toml"

# Check if the output file exists
OUTPUT_FILE="$TEST_DIR/output/$(basename $TEST_DIR)_conag_output.md"
if [ -f "$OUTPUT_FILE" ]; then
    echo "Test passed: Output file created successfully"
    echo "Output file contents:"
    echo "----------------------------------------"
    cat "$OUTPUT_FILE"
    echo "----------------------------------------"
else
    echo "Test failed: Output file not created"
fi

# Ask user if they want to keep the test directory
read -p "Do you want to keep the test directory for inspection? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "Test directory kept at: $TEST_DIR"
    echo "You can inspect the files and manually delete the directory when done."
    echo "To delete the directory, run: rm -rf $TEST_DIR"
else
    rm -rf "$TEST_DIR"
    echo "Test directory removed"
fi