# conag (Code Aggregator) 📚

conag is a simple rust based, macOS-only CLI tool designed to aggregate the contents of a project directory. It is useful for collecting code and documentation to use as context with UI based Large Language Models (LLMs).

>**NOTE:** conag was largely made as a rust learning experience, and partial personal usage.

## 🚀 Installation

To install conag, you can use the following command in your terminal:

```bash
curl -sSL https://raw.githubusercontent.com/evanmschultz/conag/main/install.sh | bash
```

This script will download the latest release of conag and install it in `/usr/local/bin`.

## 🛠️ Usage

Before using conag, you need to generate a configuration file:

```bash
conag --generate-config
```

This will create a default configuration file at `~/.config/conag/config.toml`.

### To run conag, first cd to the directory you want to aggregate then run:

```bash
conag 
```

conag will use the configuration file at `~/.config/conag/config.toml`. The aggregated output will be saved to the location specified in the configuration file.

### Command-line Options

- `--generate-config`: Generate a default configuration file
- `--plain-text`: Use plain text output format instead of Markdown
- `--include-hidden <patterns>`: Specify patterns for hidden files or directories to include
- `--include-file <files>`: Specify files to include, overriding ignore rules
- `--include-dir <directories>`: Specify directories to include, overriding ignore rules

## ⚙️ Configuration

The configuration file is located at `~/.config/conag/config.toml`. You can edit this file to customize conag's behavior.

Key configuration options:

```toml
# The directory to scan for files (use "." for current directory)
input_dir = "."

# The directory where the output will be saved
# Use "{DESKTOP}" as a placeholder for the user's desktop directory
output_dir = "{DESKTOP}/conag_output"

# Global ignore patterns (applied to all project types)
# Use glob patterns: '*' for any characters, '**' for any subdirectories
ignore_patterns = [
    "**/LICENSE",
    "**/build/**",
    "**/dist/**",
    "**/node_modules/**",
    "**/.git/**",
    "**/.vscode/**",
    "**/*.log",
    # Add more patterns as needed
]

# Patterns for hidden files to include (empty by default)
include_hidden_patterns = [
    # ".gitignore",
    # "**/.github"
]
```

To update the configuration, open the file in a text editor and modify the patterns as needed. Make sure to use the exact syntax shown above, such as `"**/dist/**"` for ignoring all `dist` directories.

## 🌟 Examples

1. Aggregate a project, including hidden `.gitignore` files:

```bash
conag --include-hidden ".gitignore"
```

2. Aggregate a project, overriding ignore rules for a specific file and directory:

```bash
conag --include-file "important.log" --include-dir "special-folder"
```

3. Generate plain text output instead of Markdown:

```bash
conag --plain-text
```

## 🗑️ Uninstallation

To uninstall conag:

1. Remove the executable:

```bash
sudo rm /usr/local/bin/conag
```

2. Remove the configuration file:

```bash
rm ~/.config/conag/config.toml
```

3. (Optional) Remove the configuration directory if it's empty:

```bash
rmdir ~/.config/conag
```

## 🔧 Advanced Usage: Multiple File and Directory Inclusions

conag allows you to include multiple files and directories that would otherwise be ignored. This is useful when you need to aggregate specific files or folders that are typically excluded. Here are some examples:

1. Include multiple specific files:

```bash
conag --include-file .env,.gitignore,package-lock.json
```

This command includes three typically ignored files: the environment file, Git ignore file, and NPM lock file.

2. Include multiple directories:

```bash
conag --include-dir node_modules,build,dist
```

This command includes three directories that are often ignored: the Node.js modules folder and two common build output folders.

3. Combine file and directory inclusions:

```bash
conag --include-file .env,config.json --include-dir .vscode,scripts
```

This command includes two specific files (.env and config.json) and two directories (.vscode and scripts) that might normally be ignored.

4. Include hidden files and specific directories:

```bash
conag --include-hidden ".*" --include-dir test,docs
```

This command includes all hidden files and also ensures that the 'test' and 'docs' directories are included, even if they're listed in the ignore patterns.

Remember, when specifying multiple items, separate them with commas and don't use spaces between the items.

## 🤝 Contributing

Contributions to conag are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).