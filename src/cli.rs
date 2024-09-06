use std::fs::{self, File};
use std::path::PathBuf;
use std::env;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub config: String,

    #[arg(long)]
    pub include_hidden: Option<Vec<String>>,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = crate::config::read_config(cli.config.as_str())?;

    if let Some(include_hidden) = cli.include_hidden {
        config.include_hidden_patterns = include_hidden;
    }

    // Resolve the input directory to an absolute path
    let input_path = if config.input_dir == "." {
        env::current_dir()?  // Get the current directory if input_dir is "."
    } else {
        PathBuf::from(&config.input_dir)
    };

    let files = crate::file_system_ops::list_files(&input_path)?;

    let ignore_rules = crate::ignore_rules::IgnoreRules::new(&config);
    let filtered_files = crate::ignore_rules::apply_ignore_rules(&ignore_rules, &files);

    let contents = crate::aggregator::aggregate_contents(&filtered_files)?;

    if contents.is_empty() {
        println!("No files found");
        return Ok(());
    }

    let output = crate::aggregator::format_output(&contents, false);

    // Ensure the output directory exists
    let output_dir = PathBuf::from(&config.output_dir);
    fs::create_dir_all(&output_dir)?;

    // Generate the output file name based on the root directory name
    let root_dir_name = input_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    let output_file_name = format!("{}_conag_output.txt", root_dir_name);
    let output_path = output_dir.join(&output_file_name);

    // Open the file in write mode, which truncates the file if it already exists
    let mut file = File::create(&output_path)?;

    // Write the new content
    file.write_all(output.as_bytes())?;

    println!("Output written to: {:?}", output_path);

    Ok(())
}