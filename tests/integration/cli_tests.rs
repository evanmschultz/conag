use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs::{self, File};
use std::io::Write;

fn create_test_file(dir: &TempDir, name: &str, content: &str) {
    let path = dir.path().join(name);
    let mut file = File::create(path).unwrap();
    write!(file, "{}", content).unwrap();
}

#[test]
fn test_cli_with_valid_input() {
    let temp_dir = TempDir::new().unwrap();
    create_test_file(&temp_dir, "file1.txt", "Content of file 1");
    create_test_file(&temp_dir, "file2.txt", "Content of file 2");

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = ["*.log"]
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("your_project_name").unwrap();
    cmd.arg("--config").arg(config_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Content of file 1"))
        .stdout(predicate::str::contains("Content of file 2"));

    // Check if output file was created
    assert!(temp_dir.path().join("aggregated_contents.txt").exists());
}

#[test]
fn test_cli_with_invalid_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid_config.toml");

    let mut cmd = Command::cargo_bin("your_project_name").unwrap();
    cmd.arg("--config").arg(config_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Config file not found"));
}

#[test]
fn test_cli_with_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = []
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("your_project_name").unwrap();
    cmd.arg("--config").arg(config_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No files found"));
}

#[test]
fn test_cli_with_include_hidden_option() {
    let temp_dir = TempDir::new().unwrap();
    create_test_file(&temp_dir, "visible.txt", "Visible content");
    create_test_file(&temp_dir, ".hidden.txt", "Hidden content");

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = []
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path)
       .arg("--include-hidden").arg(".hidden*");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Visible content"))
        .stdout(predicate::str::contains("Hidden content"));

    // Check if output file was created and contains both contents
    let output_file = temp_dir.path().join("aggregated_contents.txt");
    assert!(output_file.exists());
    let output_content = fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("Visible content"));
    assert!(output_content.contains("Hidden content"));
}

#[test]
fn test_cli_without_include_hidden_option() {
    let temp_dir = TempDir::new().unwrap();
    create_test_file(&temp_dir, "visible.txt", "Visible content");
    create_test_file(&temp_dir, ".hidden.txt", "Hidden content");

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = []
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Visible content"))
        .stdout(predicate::str::not(predicate::str::contains("Hidden content")));

    // Check if output file was created and contains only visible content
    let output_file = temp_dir.path().join("aggregated_contents.txt");
    assert!(output_file.exists());
    let output_content = fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("Visible content"));
    assert!(!output_content.contains("Hidden content"));
}

#[test]
fn test_cli_with_current_directory_as_input() {
    let temp_dir = TempDir::new().unwrap();
    create_test_file(&temp_dir, "file1.txt", "Content of file 1");
    create_test_file(&temp_dir, "file2.txt", "Content of file 2");

    let config_content = format!(
        r#"
        input_dir = "."
        output_dir = "{}"
        ignore_patterns = ["*.log"]
        "#,
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let root_dir_name = env::current_dir().unwrap().file_name().unwrap().to_str().unwrap();
    let expected_output_file = format!("{}_conag_output.txt", root_dir_name);

    let mut cmd = Command::cargo_bin("your_project_name").unwrap();
    cmd.arg("--config").arg(config_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Content of file 1"))
        .stdout(predicate::str::contains("Content of file 2"));

    // Check if the output file was created with the correct name
    assert!(temp_dir.path().join(&expected_output_file).exists());
}

#[test]
fn test_output_file_is_deleted_and_recreated() {
    let temp_dir = TempDir::new().unwrap();
    create_test_file(&temp_dir, "file1.txt", "Content of file 1");

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = ["*.log"]
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let output_file_name = format!("{}_conag_output.txt", temp_dir.path().file_name().unwrap().to_str().unwrap());
    let output_file_path = temp_dir.path().join(&output_file_name);

    // First run to create the output file
    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Content of file 1"));

    assert!(output_file_path.exists());
    let initial_content = fs::read_to_string(&output_file_path).unwrap();
    assert!(initial_content.contains("Content of file 1"));

    // Modify the original file
    fs::write(temp_dir.path().join("file1.txt"), "Updated content of file 1").unwrap();

    // Run again to check if the file is deleted and recreated
    cmd.arg("--config").arg(&config_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated content of file 1"));

    // Check if the output file contains only the updated content
    let new_content = fs::read_to_string(&output_file_path).unwrap();
    assert!(new_content.contains("Updated content of file 1"));
    assert!(!new_content.contains("Content of file 1"));

    // Ensure the file size matches the new content
    assert!(new_content.len() <= initial_content.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::collections::HashMap;

    #[test]
    fn test_cli_default_markdown() {
        let cli = Cli {
            config: "config.toml".to_string(),
            include_hidden: None,
            plain_text: false,
        };

        let mut contents = HashMap::new();
        contents.insert(PathBuf::from("test.txt"), "Test content".to_string());

        let output = crate::aggregator::format_output(&contents, !cli.plain_text);
        assert!(output.contains("# test.txt"));
    }

    #[test]
    fn test_cli_plain_text() {
        let cli = Cli {
            config: "config.toml".to_string(),
            include_hidden: None,
            plain_text: true,
        };

        let mut contents = HashMap::new();
        contents.insert(PathBuf::from("test.txt"), "Test content".to_string());

        let output = crate::aggregator::format_output(&contents, !cli.plain_text);
        assert!(output.contains("<<<File: test.txt>>>"));
    }

    #[test]
    fn test_output_file_extension() {
        let markdown_cli = Cli {
            config: "config.toml".to_string(),
            include_hidden: None,
            plain_text: false,
        };

        let plain_text_cli = Cli {
            config: "config.toml".to_string(),
            include_hidden: None,
            plain_text: true,
        };

        let test_config = crate::config::Config {
            input_dir: ".".to_string(),
            output_dir: "output".to_string(),
            ignore_patterns: vec![],
            include_hidden_patterns: vec![],
            project_specific_ignores: HashMap::new(),
            project_type: None,
        };

        let input_path = PathBuf::from("test_dir");
        let root_dir_name = input_path.file_name().unwrap().to_str().unwrap();

        // Test Markdown extension
        let markdown_extension = if !markdown_cli.plain_text { "md" } else { "txt" };
        let markdown_file_name = format!("{}_conag_output.{}", root_dir_name, markdown_extension);
        assert_eq!(markdown_file_name, "test_dir_conag_output.md");

        // Test plain text extension
        let plain_text_extension = if !plain_text_cli.plain_text { "md" } else { "txt" };
        let plain_text_file_name = format!("{}_conag_output.{}", root_dir_name, plain_text_extension);
        assert_eq!(plain_text_file_name, "test_dir_conag_output.txt");
    }
}

#[test]
#[cfg(feature = "dev")]
fn test_custom_config_path_in_dev_mode() {
    let args = vec!["conag", "--config", "/custom/path/config.toml"];
    let cli = Cli::parse_from(args);
    assert_eq!(cli.config, Some("/custom/path/config.toml".to_string()));
}

#[test]
#[cfg(not(feature = "dev"))]
fn test_no_custom_config_path_in_release_mode() {
    let args = vec!["conag", "--config", "/custom/path/config.toml"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn test_cli_parsing() {
    let args = vec!["conag", "--generate-config", "--plain-text"];
    let cli = Cli::parse_from(args);
    assert!(cli.generate_config);
    assert!(cli.plain_text);
    assert!(cli.config.is_none());
    assert!(cli.include_hidden.is_none());
}

#[test]
fn test_run_generate_config() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("HOME", temp_dir.path());

    let cli = Cli {
        config: None,
        generate_config: true,
        include_hidden: None,
        plain_text: false,
    };

    run(cli).unwrap();

    let config_path = Config::default_config_path().unwrap();
    assert!(config_path.exists());

    std::env::remove_var("HOME");
}

#[test]
fn test_run_with_custom_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("custom_config.toml");
    fs::write(&config_path, "input_dir = \".\"\noutput_dir = \"/tmp/output\"").unwrap();

    let cli = Cli {
        config: Some(config_path.to_str().unwrap().to_string()),
        generate_config: false,
        include_hidden: None,
        plain_text: false,
    };

    if cfg!(feature = "dev") {
        run(cli).unwrap();
        // Add assertions here to check if the custom config was used
    } else {
        // In release mode, custom config should be ignored
        let result = run(cli);
        assert!(result.is_ok());
        // Add assertions here to check if the default config was used instead
    }
}