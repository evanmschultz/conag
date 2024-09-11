use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};

/// Aggregates the contents of the given files into a HashMap.
///
/// # Arguments
///
/// * `files` - A slice of PathBuf representing the files to aggregate.
/// * `base_dir` - The base directory path used to calculate relative paths.
///
/// # Returns
///
/// Returns a `Result` containing a `HashMap` where:
/// - Keys are relative `PathBuf`s of the files.
/// - Values are the contents of the files as `String`s.
///
/// If a file cannot be read or contains invalid UTF-8, it is skipped and an error message is printed.
///
/// # Errors
///
/// This function will return an `io::Error` if there are issues reading the files.
pub fn aggregate_contents(files: &[PathBuf], base_dir: &Path) -> io::Result<HashMap<PathBuf, String>> {
    let mut contents = HashMap::new();
    for file in files {
        if let Ok(content) = fs::read_to_string(file) {
            let relative_path = file.strip_prefix(base_dir).unwrap_or(file);
            contents.insert(relative_path.to_path_buf(), content);
        } else {
            eprintln!("Skipping file {:?} due to invalid UTF-8", file);
        }
    }
    Ok(contents)
}

/// Determines the language identifier for a given file based on its extension.
///
/// # Arguments
///
/// * `file` - A `Path` representing the file for which to determine the language.
///
/// # Returns
///
/// Returns a `&str` containing the language identifier. If the file extension is not recognized,
/// it returns "text" as a default value.
fn get_language_identifier(file: &Path) -> &str {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("html") => "html",
        Some("css") => "css",
        Some("json") => "json",
        Some("yaml") | Some("yml") => "yaml",
        Some("md") => "markdown",
        Some("sh") => "bash",
        Some("sql") => "sql",
        Some("c") => "c",
        Some("cpp") | Some("cxx") | Some("cc") => "cpp",
        Some("java") => "java",
        Some("go") => "go",
        Some("rb") => "ruby",
        Some("php") => "php",
        Some("swift") => "swift",
        Some("kt") | Some("kts") => "kotlin",
        Some("scala") => "scala",
        Some("hs") => "haskell",
        Some("lua") => "lua",
        Some("pl") => "perl",
        Some("r") => "r",
        Some("dart") => "dart",
        Some("fs") | Some("fsx") => "fsharp",
        Some("jl") => "julia",
        Some("ex") | Some("exs") => "elixir",
        Some("cs") => "csharp",
        Some("vb") => "vb.net",
        Some("xml") => "xml",
        Some("toml") => "toml",
        Some("dockerfile") => "dockerfile",
        Some("makefile") => "makefile",
        _ => "text",
    }
}

/// Formats the aggregated file contents into a single output string.
///
/// # Arguments
///
/// * `project_name` - The name of the project to be included in the output.
/// * `contents` - A `HashMap` containing file paths and their contents.
/// * `markdown` - A boolean flag indicating whether to format the output as Markdown.
///
/// # Returns
///
/// Returns a `String` containing the formatted output of all files' contents.
///
/// # Format
///
/// The output includes:
/// - Project name at the top
/// - For each file:
///   - File path
///   - File contents in a code block with appropriate language identifier
///
/// If `markdown` is true, the output is formatted for Markdown compatibility.
/// Otherwise, it uses a plain text format with separators.
pub fn format_output(project_name: &str, contents: &HashMap<PathBuf, String>, markdown: bool) -> String {
    let mut output = String::new();
    
    // Add project name at the top
    if markdown {
        output.push_str(&format!("# Project: {}\n\n", project_name));
    } else {
        output.push_str(&format!("# Project: {}\n{}\n\n", project_name, "=".repeat(40)));
    }

    for (file, content) in contents {
        let lang_id = get_language_identifier(file);
        if markdown {
            output.push_str(&format!("## File: {}\n\n```{}\n{}\n```\n\n", file.display(), lang_id, content));
        } else {
            output.push_str(&format!("## File: {}\n{}\n```{}\n{}\n```\n\n", file.display(), "=".repeat(40), lang_id, content));
        }
    }
    output
}