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
        include_file_overrides: vec![],
        include_dir_overrides: vec![],
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
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from(".hidden"));
    files.insert(PathBuf::from(".gitignore"));

    let result = apply_ignore_rules(
        &ignore_rules, 
        &files, 
        &[], 
        &[], 
        &input_dir,
    );

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
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.log"));
    files.insert(PathBuf::from(".hidden"));
    files.insert(PathBuf::from(".env"));

    let result = apply_ignore_rules(
        &ignore_rules,
        &files,
        &[],
        &[],
        &input_dir,
    );

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
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.md"));
    files.insert(PathBuf::from("file3.txt"));
    
    let result = apply_ignore_rules(    
        &ignore_rules, 
        &files, 
        &[], 
        &[], 
        &input_dir,
    );

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
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("temp/file2.txt"));
    files.insert(PathBuf::from("temp/subdir/file3.txt"));
    
    let result = apply_ignore_rules(
        &ignore_rules, 
        &files, 
        &[], 
        &[], 
        &input_dir,
    );

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
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("src/main.rs"));
    files.insert(PathBuf::from("target/debug/main"));
    files.insert(PathBuf::from("log/app.log"));
    
    let result = apply_ignore_rules(
        &ignore_rules, 
        &files, 
        &[], 
        &[], 
        &input_dir,
    );
    assert_eq!(result.len(), 1);
    assert!(result.contains(&PathBuf::from("src/main.rs")));
}

#[test]
fn test_include_directory_override() {
    let config = create_test_config(
        vec!["ignored_dir/*".to_string(), "*.log".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let include_dir_overrides = vec!["included_dir".to_string()];
    let include_file_overrides = vec!["ignored_dir/ignored_file.txt".to_string()];
    let ignore_rules = IgnoreRules::new(&config);
    let input_dir = PathBuf::from("/test/input");
    let mut files = HashSet::new();

    files.insert(input_dir.join("file1.txt")); // Not ignored
    files.insert(input_dir.join("file2.log")); // Ignored via pattern
    files.insert(input_dir.join("ignored_dir/ignored_file.txt")); // Ignored via directory, but included via include_file_overrides
    files.insert(input_dir.join("included_dir/included_file.txt")); // Should be included via include_dir_overrides
    files.insert(input_dir.join("included_dir/ignored_file.log")); // Ignored via pattern even in included_dir

    let result = apply_ignore_rules(
        &ignore_rules,
        &files,
        &include_file_overrides,
        &include_dir_overrides,
        &input_dir,
    );

    assert_eq!(result.len(), 3);
    assert!(result.contains(&input_dir.join("file1.txt")));
    assert!(result.contains(&input_dir.join("included_dir/included_file.txt")));
    assert!(result.contains(&input_dir.join("ignored_dir/ignored_file.txt")));
    // 'file2.log' is ignored via pattern
    // 'included_dir/ignored_file.log' is ignored via pattern even in included_dir
}

#[test]
fn test_include_file_override() {
    let config = create_test_config(
        vec!["*.txt".to_string()],
        None,
        HashMap::new(),
        vec![],
    );
    let include_file_overrides = vec!["file1.txt".to_string()];
    let include_dir_overrides = vec![];
    let ignore_rules = IgnoreRules::new(&config);
    let mut files = HashSet::new();
    let input_dir = PathBuf::from("/test/input");

    files.insert(PathBuf::from("file1.txt"));
    files.insert(PathBuf::from("file2.md"));
    files.insert(PathBuf::from("file3.txt"));

    let result = apply_ignore_rules(
        &ignore_rules,
        &files,
        &include_file_overrides,
        &include_dir_overrides,
        &input_dir,
    );

    assert_eq!(result.len(), 2);
    assert!(result.contains(&PathBuf::from("file1.txt")));
    assert!(result.contains(&PathBuf::from("file2.md")));
}