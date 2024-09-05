use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub ignore_patterns: Vec<String>,
    pub project_type: Option<String>,
}

pub fn read_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}