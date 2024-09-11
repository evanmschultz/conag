use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Lists all files in the given directory and its subdirectories.
///
/// This function recursively traverses the directory structure starting from the given path,
/// collecting all file paths into a HashSet. It does not follow symbolic links.
///
/// # Arguments
///
/// * `dir` - A reference to a Path representing the directory to start the search from.
///
/// # Returns
///
/// Returns an `io::Result` containing a `HashSet` of `PathBuf`s, each representing a file path.
/// The function returns an empty set if the given path is not a directory.
///
/// # Errors
///
/// This function will return an error if there are issues accessing the file system or
/// if there are permission problems.
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

/// Reads the contents of a file and returns it as a String.
///
/// This function takes a file path and attempts to read its entire contents into a String.
///
/// # Arguments
///
/// * `file_path` - A reference to a Path representing the file to read.
///
/// # Returns
///
/// Returns an `io::Result<String>` which is Ok with the file's contents if successful,
/// or an error if the file couldn't be read.
///
/// # Errors
///
/// This function will return an error if the file cannot be read. This can happen due to
/// various reasons such as the file not existing, lack of permissions, or I/O errors.
pub fn read_file_contents(file_path: &Path) -> io::Result<String> {
    fs::read_to_string(file_path)
}