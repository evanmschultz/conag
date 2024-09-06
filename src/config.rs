use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use anyhow::Result;

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

    pub fn should_include_hidden(&self, path: &str) -> bool {
        self.include_hidden_patterns.iter().any(|pattern| {
            glob::Pattern::new(pattern).map(|p| p.matches(path)).unwrap_or(false)
        })
    }
}

pub fn read_config(config_path: &str) -> Result<Config> {
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}