use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::{Result, Context, bail};
use dirs;

/// Represents the configuration for the application.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub ignore_patterns: Vec<String>,
    pub project_type: Option<String>,
    #[serde(default)]
    pub project_specific_ignores: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub include_hidden_patterns: Vec<String>,
}

impl Config {
    /// Returns a vector of ignore patterns, including project-specific patterns if applicable.
    /// Also adds a default pattern to ignore all hidden files unless explicitly included.
    pub fn get_ignore_patterns(&self) -> Vec<String> {
        let mut patterns = self.ignore_patterns.clone();
        if let Some(project_type) = &self.project_type {
            if let Some(specific_ignores) = self.project_specific_ignores.get(project_type) {
                patterns.extend(specific_ignores.clone());
            }
        }
        // Add default pattern to ignore all hidden files, unless they're explicitly included
        if !self.include_hidden_patterns.contains(&".*".to_string()) {
            patterns.push(".*".to_string());
        }
        patterns
    }

    /// Determines whether a hidden file or directory should be included based on the configured patterns.
    pub fn should_include_hidden(&self, path: &str) -> bool {
        self.include_hidden_patterns.iter().any(|pattern| {
            glob::Pattern::new(pattern).map(|p| p.matches(path)).unwrap_or(false)
        })
    }

    /// Resolves the output directory path, replacing {DESKTOP} with the actual desktop path if present.
    pub fn resolve_output_dir(&mut self) -> Result<()> {
        if self.output_dir.contains("{DESKTOP}") {
            let desktop_path = dirs::desktop_dir()
                .with_context(|| "Failed to get desktop directory")?;
            self.output_dir = self.output_dir.replace(
                "{DESKTOP}",
                desktop_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid desktop path"))?
            );
        }
        Ok(())
    }

    /// Returns the default config file path
    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".config")
            .join("conag");
        Ok(config_dir.join("config.toml"))
    }
}



/// Reads and parses the configuration file.
///
/// This function attempts to read the configuration file from the specified path
/// or from the default location if no path is provided. It then parses the
/// contents of the file into a `Config` struct.
///
/// # Arguments
///
/// * `config_path` - An optional string slice that holds the path to the config file.
///   If None, the default config path will be used.
///
/// # Returns
///
/// Returns a `Result<Config>`, which is `Ok(Config)` if the operation was successful,
/// or an error if the config file couldn't be found, read, or parsed.
///
/// # Errors
///
/// This function will return an error if:
/// - The config file does not exist at the specified or default path.
/// - The config file cannot be read.
/// - The contents of the config file cannot be parsed as valid TOML.
/// - The output directory in the config cannot be resolved.
pub fn read_config(config_path: Option<&str>) -> Result<Config> {
    let config_path = match config_path {
        Some(path) => PathBuf::from(path),
        None => Config::default_config_path()?,
    };

    if !config_path.exists() {
        bail!("Config file not found at {:?}. To generate a default config, run:\nconag --generate-config", config_path);
    }

    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
    let mut config: Config = toml::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;
    config.resolve_output_dir()?;
    Ok(config)
}



/// Generates a default configuration file.
///
/// This function attempts to create a default configuration file at the default
/// config path. If the file doesn't exist, it creates the necessary directories,
/// writes the default configuration content, and prints a success message.
/// If the file already exists, it prints a message indicating so.
///
/// # Returns
///
/// Returns a `Result<()>`, which is `Ok(())` if the operation was successful,
/// or an error if there were issues creating directories, writing the file,
/// or determining the default config path.
///
/// # Errors
///
/// This function will return an error if:
/// - The default config path cannot be determined.
/// - The necessary directories cannot be created.
/// - The default config file cannot be written.
pub fn generate_default_config() -> Result<()> {
    let config_path = Config::default_config_path()?;
    if !config_path.exists() {
        let config_dir = config_path.parent().unwrap();
        fs::create_dir_all(config_dir)?;
        fs::write(&config_path, include_str!("../config/default_config.toml"))?;
        println!("Generated default config file at {:?}", config_path);
    } else {
        println!("Config file already exists at {:?}", config_path);
    }
    Ok(())
}