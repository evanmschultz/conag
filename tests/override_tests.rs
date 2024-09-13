#[cfg(feature = "dev")]
use assert_cmd::Command;
#[cfg(feature = "dev")]
use std::fs;
#[cfg(feature = "dev")]
use tempfile::TempDir;

#[cfg(feature = "dev")]
fn create_test_file(dir: &std::path::Path, name: &str, content: &str) {
    let path = dir.join(name);
    fs::write(path, content).unwrap();
}

#[cfg(feature = "dev")]
#[test]
fn test_include_file_override() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files in the temporary directory
    create_test_file(temp_dir.path(), "included.txt", "This should be included");
    create_test_file(temp_dir.path(), "excluded.txt", "This should be excluded");

    // Write a config that ignores all .txt files
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

    // Run the command with --include-file (using hyphen)
    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path)
       .arg("--include-file").arg("included.txt");
    
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
        })
        .expect("Output file not found")
        .path();

    println!("Found output file: {:?}", output_file);

    // Read the content of the output file
    let output_content = fs::read_to_string(&output_file).unwrap();

    // Check if the output contains the included file's content and excludes the other
    assert!(
        output_content.contains("This should be included"),
        "Output should contain 'This should be included'"
    );
    assert!(
        !output_content.contains("This should be excluded"),
        "Output should not contain 'This should be excluded'"
    );
}

#[cfg(feature = "dev")]
#[test]
fn test_include_dir_override() {
    let temp_dir = TempDir::new().unwrap();

    // Create an ignored directory with a file inside
    let ignored_dir = temp_dir.path().join("ignored_dir");
    fs::create_dir(&ignored_dir).unwrap();
    create_test_file(&ignored_dir, "file_included.md", "This should be included because the directory is included");

    // Create a file in the root directory that should be excluded
    create_test_file(temp_dir.path(), "excluded.txt", "This should be excluded");

    // Write a config that ignores the directory and all .txt files
    let config_content = format!(
        r#"
        input_dir = "{}"
        output_dir = "{}"
        ignore_patterns = ["ignored_dir/*", "*.txt"]
        "#,
        temp_dir.path().display(),
        temp_dir.path().display()
    );

    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    // Run the command with --include-dir (using hyphen)
    let mut cmd = Command::cargo_bin("conag").unwrap();
    cmd.arg("--config").arg(&config_path)
       .arg("--include-dir").arg("ignored_dir");
    
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
        })
        .expect("Output file not found")
        .path();

    println!("Found output file: {:?}", output_file);

    // Read the content of the output file
    let output_content = fs::read_to_string(&output_file).unwrap();

    // Check if the output contains the included directory's file content and excludes others
    assert!(
        output_content.contains("This should be included because the directory is included"),
        "Output should contain the included content"
    );
    assert!(
        !output_content.contains("This should be excluded"),
        "Output should not contain the excluded content"
    );
}