use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(list_files(&path)?);
            }
        }
    }
    Ok(files)
}

pub fn read_file_contents(file_path: &Path) -> io::Result<String> {
    fs::read_to_string(file_path)
}