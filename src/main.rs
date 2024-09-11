use clap::Parser;
use conag::cli::{Cli, run};

/// The main entry point for the conag application.
///
/// This function parses command-line arguments using the Cli struct,
/// runs the main application logic, and handles any errors that occur.
/// If an error is encountered, it prints the error message to stderr
/// and exits the program with a non-zero status code.
fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}