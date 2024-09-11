use std::fs::{self, File};
use std::path::PathBuf;
use std::env;
use std::io::Write;
use clap::Parser;
use anyhow::Result;
use std::collections::HashSet;
use crate::config::{read_config, generate_default_config};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Generate default config file
    #[arg(long)]
    pub generate_config: bool,

    /// Optional list of hidden files or directories to include.
    #[arg(long)]
    pub include_hidden: Option<Vec<String>>,

    /// Flag to use plain text output format instead of Markdown.
    #[arg(long, help = "Use plain text output format instead of Markdown")]
    pub plain_text: bool,
}

/// Executes the main functionality of the CLI application.
///
/// This function handles the following operations:
/// - Generates a default configuration file if requested
/// - Reads the configuration (from a custom path in dev mode, or default location in release mode)
/// - Processes command-line arguments to override configuration settings
/// - Lists and filters files in the input directory
/// - Aggregates contents of the filtered files
/// - Formats the output (as Markdown or plain text)
/// - Writes the formatted output to a file in the specified output directory
///
/// # Arguments
///
/// * `cli` - A `Cli` struct containing parsed command-line arguments
///
/// # Returns
///
/// Returns `Ok(())` if the operation is successful, or an error if any step fails.
pub fn run(cli: Cli) -> Result<()> {
    if cli.generate_config {
        generate_default_config()?;
        return Ok(());
    }

    let mut config = if cfg!(feature = "dev") {
        read_config(cli.config.as_deref())?
    } else {
        if cli.config.is_some() {
            eprintln!("Warning: Custom config path is ignored in release mode. Using default config location.");
        }
        read_config(None)?
    };

    if let Some(include_hidden) = cli.include_hidden {
        config.include_hidden_patterns = include_hidden;
    }

    let input_path = if config.input_dir == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&config.input_dir)
    };
    
    // Get the project name from the root directory
    let project_name = input_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown Project");
    
    let files: HashSet<PathBuf> = crate::file_system_ops::list_files(&input_path)?;
    
    let ignore_rules = crate::ignore_rules::IgnoreRules::new(&config);
    let filtered_files: Vec<PathBuf> = crate::ignore_rules::apply_ignore_rules(&ignore_rules, &files);
    
    let contents = crate::aggregator::aggregate_contents(&filtered_files, &input_path)?;

    // Use Markdown by default, unless --plain-text is specified
    let use_markdown = !cli.plain_text;
    let output = crate::aggregator::format_output(project_name, &contents, use_markdown);

    // Ensure the output directory exists
    let output_dir = PathBuf::from(&config.output_dir);
    fs::create_dir_all(&output_dir)?;

    // Generate the output file name based on the root directory name
    let root_dir_name = input_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    let file_extension = if use_markdown { "md" } else { "txt" };
    let output_file_name = format!("{}_conag_output.{}", root_dir_name, file_extension);
    let output_path = output_dir.join(&output_file_name);

    // Open the file in write mode, which truncates the file if it already exists
    let mut file = File::create(&output_path)?;

    // Write the new content
    file.write_all(output.as_bytes())?;

    println!("Output written to: {:?}", output_path);

    Ok(())
}