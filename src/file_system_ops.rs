use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut files = HashSet::new();
    if dir.is_dir() {
        for entry in walkdir::WalkDir::new(dir).follow_links(false).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();
            if path.is_file() {
                files.insert(path);
            }
        }
    }
    Ok(files)
}

pub fn read_file_contents(file_path: &Path) -> io::Result<String> {
    fs::read_to_string(file_path)
}