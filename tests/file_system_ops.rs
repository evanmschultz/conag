use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use conag::file_system_ops::{list_files, read_file_contents};

#[test]
fn test_list_files() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create a file structure
    fs::create_dir(base_path.join("subdir")).unwrap();
    File::create(base_path.join("file1.txt")).unwrap();
    File::create(base_path.join("file2.txt")).unwrap();
    File::create(base_path.join("subdir").join("file3.txt")).unwrap();

    let files = list_files(base_path).unwrap();

    assert_eq!(files.len(), 3);
    assert!(files.contains(&base_path.join("file1.txt")));
    assert!(files.contains(&base_path.join("file2.txt")));
    assert!(files.contains(&base_path.join("subdir").join("file3.txt")));
}

#[test]
fn test_read_file_contents() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    let content = "Hello, World!";
    
    let mut file = File::create(&file_path).unwrap();
    write!(file, "{}", content).unwrap();

    let read_content = read_file_contents(&file_path).unwrap();
    assert_eq!(read_content, content);
}

#[test]
fn test_list_files_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let files = list_files(temp_dir.path()).unwrap();
    assert!(files.is_empty());
}

#[test]
fn test_read_file_contents_non_existent_file() {
    let result = read_file_contents(Path::new("/non/existent/file.txt"));
    assert!(result.is_err());
}