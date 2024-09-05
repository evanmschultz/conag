use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn aggregate_contents(files: &[PathBuf]) -> io::Result<HashMap<PathBuf, String>> {
    let mut contents = HashMap::new();
    for file in files {
        let content = fs::read_to_string(file)?;
        contents.insert(file.file_name().unwrap().to_owned().into(), content);
    }
    Ok(contents)
}

pub fn format_output(contents: &HashMap<PathBuf, String>, markdown: bool) -> String {
    let mut output = String::new();
    for (file, content) in contents {
        if markdown {
            output.push_str(&format!("# {}\n\n{}\n\n", file.display(), content));
        } else {
            output.push_str(&format!("File: {}\n{}\n\n{}\n\n", file.display(), "=".repeat(40), content));
        }
    }
    output
}