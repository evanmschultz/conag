use conag::config::read_config; 
use std::fs;

#[test]
fn test_read_config() {
    let config_content = r#"
    input_dir = "/path/to/input"
    output_dir = "/path/to/output"
    ignore_patterns = ["*.tmp", "*.log"]
    project_type = "rust"
    "#;

    fs::write("test_config.toml", config_content).unwrap();

    let config = read_config("test_config.toml").unwrap();

    assert_eq!(config.input_dir, "/path/to/input");
    assert_eq!(config.output_dir, "/path/to/output");
    assert_eq!(config.ignore_patterns, vec!["*.tmp", "*.log"]);
    assert_eq!(config.project_type, Some("rust".to_string()));

    fs::remove_file("test_config.toml").unwrap();
}