use conag::ignore_rules::{IgnoreRules, apply_ignore_rules};
use conag::config::Config;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

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
        include_overrides: vec![],
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
    let mut files = HashSet::new();
    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from(".hidden"));
    files.insert(PathBuf::from(".gitignore"));
    let result = apply_ignore_rules(&ignore_rules, &files, &[]);
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
    let mut files = HashSet::new();
    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.log"));
    files.insert(PathBuf::from(".hidden"));
    files.insert(PathBuf::from(".env"));
    let result = apply_ignore_rules(&ignore_rules, &files, &[]);
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
    let mut files = HashSet::new();
    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.md"));
    files.insert(PathBuf::from("file3.txt"));
    let result = apply_ignore_rules(&ignore_rules, &files, &[]);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&PathBuf::from("file2.md")));
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
    let mut files = HashSet::new();
    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("temp/file2.txt"));
    files.insert(PathBuf::from("temp/subdir/file3.txt"));
    let result = apply_ignore_rules(&ignore_rules, &files, &[]);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&PathBuf::from("file1.txt")));
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
    let mut files = HashSet::new();
    files.insert(PathBuf::from("src/main.rs"));
    files.insert(PathBuf::from("target/debug/main"));
    files.insert(PathBuf::from("log/app.log"));
    let result = apply_ignore_rules(&ignore_rules, &files, &[]);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&PathBuf::from("src/main.rs")));
}

#[test]
fn test_include_override() {
    let config = create_test_config(
        vec!["*.txt".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let ignore_rules = IgnoreRules::new(&config);
    let mut files = HashSet::new();
    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.md"));
    files.insert(PathBuf::from("file3.txt"));
    let result = apply_ignore_rules(&ignore_rules, &files, &["file1.txt".to_string()]);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&PathBuf::from("file1.txt")));
    assert!(result.contains(&PathBuf::from("file2.md")));
}