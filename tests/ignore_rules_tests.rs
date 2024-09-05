use conag::ignore_rules::{IgnoreRules, apply_ignore_rules};
use std::path::PathBuf;

#[test]
fn test_ignore_rules_creation() {
    let rules = vec!["*.txt".to_string(), "temp/*".to_string()];
    let ignore_rules = IgnoreRules::new(rules);
    assert_eq!(ignore_rules.rules.len(), 2);
}

#[test]
fn test_ignore_file_extension() {
    let rules = vec!["*.txt".to_string()];
    let ignore_rules = IgnoreRules::new(rules);
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
    let rules = vec!["temp/*".to_string()];
    let ignore_rules = IgnoreRules::new(rules);
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
fn test_multiple_ignore_rules() {
    let rules = vec!["*.txt".to_string(), "temp/*".to_string()];
    let ignore_rules = IgnoreRules::new(rules);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.md"),
        PathBuf::from("temp/file3.md"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], PathBuf::from("file2.md"));
}

#[test]
fn test_no_ignore_rules() {
    let ignore_rules = IgnoreRules::new(vec![]);
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.md"),
    ];
    let result = apply_ignore_rules(&ignore_rules, &files);
    assert_eq!(result.len(), 2);
}