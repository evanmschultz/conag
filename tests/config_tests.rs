use conag::config::{read_config, Config};
use std::fs;
use tempfile::NamedTempFile;
use std::collections::HashMap;

#[test]
fn test_read_config_with_include_hidden_patterns() {
    let config_content = r#"
    input_dir = "/path/to/input"
    output_dir = "/path/to/output"
    ignore_patterns = ["*.tmp", "*.log"]
    project_type = "rust"
    include_hidden_patterns = [".gitignore", ".env"]

    [project_specific_ignores]
    rust = ["target/", "Cargo.lock"]
    python = ["__pycache__/", "*.pyc"]
    "#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), config_content).unwrap();

    let config: Config = read_config(temp_file.path().to_str().unwrap()).unwrap();

    assert_eq!(config.input_dir, "/path/to/input");
    assert_eq!(config.output_dir, "/path/to/output");
    assert_eq!(config.ignore_patterns, vec!["*.tmp", "*.log"]);
    assert_eq!(config.project_type, Some("rust".to_string()));
    assert_eq!(config.project_specific_ignores.get("rust").unwrap(), &vec!["target/", "Cargo.lock"]);
    assert_eq!(config.project_specific_ignores.get("python").unwrap(), &vec!["__pycache__/", "*.pyc"]);
    assert_eq!(config.include_hidden_patterns, vec![".gitignore", ".env"]);
}

#[test]
fn test_should_include_hidden() {
    let config = Config {
        input_dir: String::new(),
        output_dir: String::new(),
        ignore_patterns: vec![],
        project_type: None,
        project_specific_ignores: HashMap::new(),
        include_hidden_patterns: vec![".gitignore".to_string(), ".env*".to_string()],
    };

    assert!(config.should_include_hidden(".gitignore"));
    assert!(config.should_include_hidden(".env"));
    assert!(config.should_include_hidden(".env.local"));
    assert!(!config.should_include_hidden(".git"));
    assert!(!config.should_include_hidden("normal_file.txt"));
}

#[test]
fn test_get_ignore_patterns_includes_hidden_files() {
    let config = Config {
        input_dir: String::new(),
        output_dir: String::new(),
        ignore_patterns: vec!["*.tmp".to_string()],
        project_type: None,
        project_specific_ignores: HashMap::new(),
        include_hidden_patterns: vec![],
    };

    let patterns = config.get_ignore_patterns();
    assert!(patterns.contains(&"*.tmp".to_string()));
    assert!(patterns.contains(&".*".to_string()));
}