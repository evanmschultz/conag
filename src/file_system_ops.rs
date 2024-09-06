use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip directories like .git, target, etc.
            if path.is_dir() && !path.is_symlink() {
                if path.ends_with(".git") || path.ends_with("target") || path.ends_with("node_modules") {
                    eprintln!("Skipping directory: {:?}", path);
                    continue;
                }
                files.extend(list_files(&path)?);
            } else if path.is_file() && !path.is_symlink() {
                files.push(path);
            }
        }
    }
    Ok(files)
}

pub fn read_file_contents(file_path: &Path) -> io::Result<String> {
    fs::read_to_string(file_path)
}