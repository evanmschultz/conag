use conag::ignore_rules::{IgnoreRules, apply_ignore_rules};
use conag::config::Config;
use std::path::PathBuf;
use std::collections::HashMap;
use tempfile::TempDir;
use conag::file_system_ops::list_files;
use std::fs::{self, File};

fn create_test_config(
    ignore_patterns: Vec<String>,
    project_type: Option<String>,
    project_specific_ignores: HashMap<String, Vec<String>>,
    include_hidden_patterns: Vec<String>,
) -> Config {
    Config {
        input_dir: String::from("/test/input"),
        output_dir: String::from("/test/output"),
        ignore_patterns,
        project_type,
        project_specific_ignores,
        include_hidden_patterns,
    }
}

#[test]
fn test_ignore_rules_creation() {
    let config = create_test_config(
        vec!["*.txt".to_string(), "temp/*".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let ignore_rules = IgnoreRules::new(&config);
    assert_eq!(ignore_rules.rules.len(), 2);
    assert_eq!(ignore_rules.include_hidden.len(), 0);
    assert!(ignore_rules.ignore_hidden.matches(".*"));
}

#[test]
fn test_ignore_hidden_files() {
    let config = create_test_config(
        vec![],
        None,
        HashMap::new(),
        vec![".gitignore".to_string()],
    );
    let ignore_rules = IgnoreRules::new(&config);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from(".hidden"),
        PathBuf::from(".gitignore"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&PathBuf::from("file1.txt")));
    assert!(result.contains(&PathBuf::from(".gitignore")));
}

#[test]
fn test_ignore_rules_with_hidden_patterns() {
    let config = create_test_config(
        vec!["*.log".to_string()],
        None,
        HashMap::new(),
        vec![".env".to_string()],
    );
    let ignore_rules = IgnoreRules::new(&config);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.log"),
        PathBuf::from(".hidden"),
        PathBuf::from(".env"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&PathBuf::from("file1.txt")));
    assert!(result.contains(&PathBuf::from(".env")));
}

#[test]
fn test_ignore_file_extension() {
    let config = create_test_config(
        vec!["*.txt".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let ignore_rules = IgnoreRules::new(&config);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.md"),
        PathBuf::from("file3.txt"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], PathBuf::from("file2.md"));
}

#[test]
fn test_ignore_directory() {
    let config = create_test_config(
        vec!["temp/*".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let ignore_rules = IgnoreRules::new(&config);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("temp/file2.txt"),
        PathBuf::from("temp/subdir/file3.txt"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], PathBuf::from("file1.txt"));
}

#[test]
fn test_project_specific_ignore_rules() {
    let mut project_specific_ignores = HashMap::new();
    project_specific_ignores.insert("rust".to_string(), vec!["target/*".to_string()]);
    
    let config = create_test_config(
        vec!["*.log".to_string()],
        Some("rust".to_string()),
        project_specific_ignores,
        vec![],
    );
    
    let ignore_rules = IgnoreRules::new(&config);
    let files = vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("target/debug/main"),
        PathBuf::from("log/app.log"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], PathBuf::from("src/main.rs"));
}

#[test]
#[cfg(target_os = "macos")]
fn test_list_files_ignores_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create a file structure with a symlink
    fs::create_dir(base_path.join("subdir")).unwrap();
    File::create(base_path.join("file1.txt")).unwrap();
    File::create(base_path.join("subdir").join("file2.txt")).unwrap();
    std::os::unix::fs::symlink("file1.txt", base_path.join("symlink.txt")).unwrap();

    let files = list_files(base_path).unwrap();

    assert_eq!(files.len(), 2);
    assert!(files.contains(&base_path.join("file1.txt")));
    assert!(files.contains(&base_path.join("subdir").join("file2.txt")));
    assert!(!files.contains(&base_path.join("symlink.txt")));
}

#[test]
#[cfg(target_os = "macos")]
fn test_list_files_ignores_directory_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create a file structure with a directory symlink
    fs::create_dir(base_path.join("dir1")).unwrap();
    fs::create_dir(base_path.join("dir2")).unwrap();
    File::create(base_path.join("dir1").join("file1.txt")).unwrap();
    File::create(base_path.join("dir2").join("file2.txt")).unwrap();
    std::os::unix::fs::symlink("dir1", base_path.join("symlink_dir")).unwrap();

    let files = list_files(base_path).unwrap();

    assert_eq!(files.len(), 2);
    assert!(files.contains(&base_path.join("dir1").join("file1.txt")));
    assert!(files.contains(&base_path.join("dir2").join("file2.txt")));
    assert!(!files.iter().any(|p| p.to_str().unwrap().contains("symlink_dir")));
}