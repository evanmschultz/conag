#!/bin/bash

# Print current working directory
echo "Current working directory: $PWD"

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found in the current directory."
    exit 1
fi

# Check if Cargo.toml contains the project name
if ! grep -q 'name = "conag"' Cargo.toml; then
    echo "Error: Cargo.toml does not contain 'name = \"conag\"'."
    echo "Contents of Cargo.toml:"
    cat Cargo.toml
    exit 1
fi

# Define the path to the default config file in the project
DEFAULT_CONFIG="$PWD/config/default_config.toml"

if [ ! -f "$DEFAULT_CONFIG" ]; then
    echo "Error: Default config file not found at $DEFAULT_CONFIG"
    echo "Contents of $PWD/config:"
    ls -la "$PWD/config"
    exit 1
fi

# Build the application in dev mode
echo "Building conag in dev mode..."
cargo build --features dev

# Run conag on its own directory
echo "Running conag on its own project directory..."
echo -e "\n"
cargo run --features dev -- --config "$DEFAULT_CONFIG"

# Determine the output file locations
PROJECT_NAME=$(basename "$PWD")
OUTPUT_DIR="$HOME/Desktop/conag_output"
MD_OUTPUT_FILE="$OUTPUT_DIR/${PROJECT_NAME}_conag_output.md"

# Function to process output file
process_output_file() {
    local file=$1
    if [ -f "$file" ]; then
        echo "Conag output has been generated successfully."
        echo -e "\n"
        echo "Output file: $file"
        echo -e "\n"
        
        echo "The output file is available at: $file"
        echo -e "\n"

        # Ask if the user wants to delete the output file
        read -p "Do you want to delete this output file? (yes/no): " delete_response

        if [ "$delete_response" = "yes" ]; then
            if rm "$file"; then
                echo -e "\n"
                echo "Output file has been deleted."
                echo -e "\n"
            else
                echo -e "\n"
                echo "Error: Failed to delete the output file."
                echo -e "\n"
            fi
        else
            echo -e "\n"
            echo "Output file has been kept at: $file"
        fi
    else
        echo "Note: Expected output file not found at $file"
    fi
}

# Process Markdown output file
process_output_file "$MD_OUTPUT_FILE"

# Check if the output directory exists
if [ ! -d "$OUTPUT_DIR" ]; then
    echo "Note: The output directory $OUTPUT_DIR does not exist."
    echo "Make sure your config file specifies the correct output directory."
    echo -e "\n"
fi

echo "Script execution completed."