use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};

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

pub fn format_output(project_name: &str, contents: &HashMap<PathBuf, String>, markdown: bool) -> String {
    let mut output = String::new();
    
    // Add project name at the top
    if markdown {
        output.push_str(&format!("# Project: {}\n\n", project_name));
    } else {
        output.push_str(&format!("Project: {}\n{}\n\n", project_name, "=".repeat(40)));
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