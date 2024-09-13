use conag::config::{read_config, Config, generate_default_config};
use std::fs;
use tempfile::{NamedTempFile, TempDir};
use std::collections::HashMap;
use std::env;

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

    let config: Config = read_config(Some(temp_file.path().to_str().unwrap())).unwrap();

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
        include_file_overrides: vec![],
        include_dir_overrides: vec![],
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
        include_file_overrides: vec![],
        include_dir_overrides: vec![],
    };

    let patterns = config.get_ignore_patterns();
    assert!(patterns.contains(&"*.tmp".to_string()));
    assert!(patterns.contains(&".*".to_string()));
}

#[test]
fn test_resolve_output_dir() {
    let mut config = Config {
        input_dir: String::new(),
        output_dir: "{DESKTOP}/conag_output".to_string(),
        ignore_patterns: vec![],
        project_type: None,
        project_specific_ignores: HashMap::new(),
        include_hidden_patterns: vec![],
        include_file_overrides: vec![],
        include_dir_overrides: vec![],
    };

    config.resolve_output_dir().unwrap();
    assert!(config.output_dir.contains("conag_output"));
    assert!(!config.output_dir.contains("{DESKTOP}"));
}

#[test]
fn test_read_config_with_custom_path() {
    let config_content = r#"
    input_dir = "/custom/input"
    output_dir = "/custom/output"
    ignore_patterns = ["*.tmp"]
    "#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), config_content).unwrap();

    let config = read_config(Some(temp_file.path().to_str().unwrap())).unwrap();

    assert_eq!(config.input_dir, "/custom/input");
    assert_eq!(config.output_dir, "/custom/output");
    assert_eq!(config.ignore_patterns, vec!["*.tmp"]);
}

#[test]
fn test_default_config_path() {
    // Create a temporary directory to act as the home directory
    let temp_dir = TempDir::new().unwrap();
    
    // Get the expected path
    let expected_path = temp_dir.path().join(".config").join("conag").join("config.toml");
    
    // Get the actual path from the Config::default_config_path_with_home() method
    let actual_path = conag::config::Config::default_config_path_with_home(Some(temp_dir.path())).unwrap();

    // Compare the paths
    assert_eq!(actual_path, expected_path);
}

#[test]
fn test_generate_default_config() {
    let temp_dir = TempDir::new().unwrap();
    env::set_var("HOME", temp_dir.path());

    // Ensure the config directory exists
    let config_dir = temp_dir.path().join(".config").join("conag");
    fs::create_dir_all(&config_dir).unwrap();

    let result = generate_default_config();
    assert!(result.is_ok());

    let config_path = Config::default_config_path().unwrap();
    assert!(config_path.exists(), "Config file was not created at {:?}", config_path);

    // Clean up
    env::remove_var("HOME");
}

#[test]
fn test_config_not_found_error() {
    let temp_dir = TempDir::new().unwrap();
    env::set_var("HOME", temp_dir.path());
    
    // Attempt to read the config (which shouldn't exist)
    let result = read_config(None);
    
    assert!(result.is_err(), "Expected an error when config file doesn't exist");
    
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("Config file not found"));
    assert!(error_message.contains("To generate a default config, run:"));
    assert!(error_message.contains("conag --generate-config"));
    
    // Clean up
    env::remove_var("HOME");
}
