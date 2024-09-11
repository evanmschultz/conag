use conag::aggregator::{aggregate_contents, format_output};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let path = dir.path().join(name);
    let mut file = File::create(&path).unwrap();
    write!(file, "{}", content).unwrap();
    path
}

#[test]
fn test_aggregate_contents() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    let file1 = create_test_file(&temp_dir, "file1.txt", "Content of file 1");
    let file2 = create_test_file(&temp_dir, "file2.txt", "Content of file 2");
    
    let files = vec![file1, file2];
    let result = aggregate_contents(&files, base_path).unwrap();
    
    assert_eq!(result.len(), 2);
    assert_eq!(result[&PathBuf::from("file1.txt")], "Content of file 1");
    assert_eq!(result[&PathBuf::from("file2.txt")], "Content of file 2");
}

#[test]
fn test_aggregate_contents_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    let file1 = create_test_file(&temp_dir, "empty.txt", "");
    
    let files = vec![file1];
    let result = aggregate_contents(&files, base_path).unwrap();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[&PathBuf::from("empty.txt")], "");
}

#[test]
fn test_format_output_text() {
    let mut contents = HashMap::new();
    contents.insert(PathBuf::from("file1.txt"), "Content of file 1".to_string());
    contents.insert(PathBuf::from("file2.txt"), "Content of file 2".to_string());
    
    let project_name = "Test Project";
    let result = format_output(project_name, &contents, false);
    
    assert!(result.contains("Project: Test Project"));
    assert!(result.contains("========================================"));
    assert!(result.contains("File: file1.txt"));
    assert!(result.contains("========================================"));
    assert!(result.contains("```text\nContent of file 1\n```"));
    assert!(result.contains("File: file2.txt"));
    assert!(result.contains("========================================"));
    assert!(result.contains("```text\nContent of file 2\n```"));
}

#[test]
fn test_format_output_markdown() {
    let mut contents = HashMap::new();
    contents.insert(PathBuf::from("file1.txt"), "Content of file 1".to_string());
    contents.insert(PathBuf::from("file2.txt"), "Content of file 2".to_string());
    
    let project_name = "Test Project";
    let result = format_output(project_name, &contents, true);
    
    assert!(result.contains("# Project: Test Project"));
    assert!(result.contains("## File: file1.txt"));
    assert!(result.contains("```text\nContent of file 1\n```"));
    assert!(result.contains("## File: file2.txt"));
    assert!(result.contains("```text\nContent of file 2\n```"));
}