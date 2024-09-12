#[cfg(feature = "dev")]
use assert_cmd::Command;
#[cfg(feature = "dev")]
use std::fs::{self, File};
#[cfg(feature = "dev")]
use tempfile::TempDir;

#[cfg(feature = "dev")]
fn create_test_file(dir: &TempDir, name: &str, content: &str) {
    let path = dir.path().join(name);
    fs::write(path, content).unwrap();
}

#[cfg(feature = "dev")]
#[test]
fn test_include_override() {
    let temp_dir = TempDir::new().unwrap();
    
    // Use the create_test_file function
    create_test_file(&temp_dir, "included.txt", "This should be included");
    create_test_file(&temp_dir, "excluded.txt", "This should be excluded");

    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = ["*.txt"]
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path)
       .arg("--include").arg("included.txt");
    
    let output = cmd.output().expect("Failed to execute command");

    println!("Command stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Command stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    // Find the output file in the temp directory
    let output_file = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            file_name_str.ends_with("_conag_output.md")
        });

    let output_file = output_file.expect("Output file not found").path();
    println!("Found output file: {:?}", output_file);

    // Read the content of the output file
    let output_content = fs::read_to_string(&output_file).unwrap();

    // Check if the output file contains the correct content
    assert!(output_content.contains("This should be included"), "Output should contain 'This should be included'");
    assert!(!output_content.contains("This should be excluded"), "Output should not contain 'This should be excluded'");
}