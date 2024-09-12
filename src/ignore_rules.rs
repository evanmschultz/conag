use std::path::PathBuf;
use glob::Pattern;
use crate::config::Config;
use std::collections::HashSet;

/// Represents a set of rules for ignoring files and directories.
///
/// This struct contains three types of rules:
/// - `rules`: A list of patterns for files and directories to ignore.
/// - `include_hidden`: A list of patterns for hidden files or directories to include, despite being hidden.
/// - `ignore_hidden`: A pattern to match all hidden files and directories.
pub struct IgnoreRules {
    pub rules: Vec<Pattern>,
    pub include_hidden: Vec<Pattern>,
    pub ignore_hidden: Pattern,
}

impl IgnoreRules {
    /// Creates a new `IgnoreRules` instance from the given configuration.
    ///
    /// This method processes the ignore patterns from the configuration and creates
    /// the corresponding `Pattern` instances for rules, include_hidden, and ignore_hidden.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct containing the ignore patterns.
    ///
    /// # Returns
    ///
    /// Returns a new `IgnoreRules` instance with the processed patterns.
    pub fn new(config: &Config) -> Self {
        let patterns = config.get_ignore_patterns();
        let rules: Vec<Pattern> = patterns.into_iter()
            .filter(|p| p != ".*")  // Filter out the ".*" pattern
            .map(|r| Pattern::new(&r).expect("Invalid ignore pattern"))
            .collect();
        let include_hidden: Vec<Pattern> = config.include_hidden_patterns.iter()
            .map(|p| Pattern::new(p).expect("Invalid include hidden pattern"))
            .collect();
        let ignore_hidden = Pattern::new(".*").expect("Invalid ignore hidden pattern");
        IgnoreRules { rules, include_hidden, ignore_hidden }
    }
}

pub fn apply_ignore_rules(ignore_rules: &IgnoreRules, files: &HashSet<PathBuf>, include_overrides: &[String]) -> Vec<PathBuf> {
    files.iter()
        .filter(|file| {
            let file_name = file.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let is_hidden = ignore_rules.ignore_hidden.matches(file_name);
            let should_ignore = ignore_rules.rules.iter().any(|rule| rule.matches_path(file));
            let should_include_hidden = ignore_rules.include_hidden.iter().any(|pattern| pattern.matches(file_name));
            let should_override = include_overrides.iter().any(|override_path| {
                file.to_str().map_or(false, |f| f.contains(override_path))
            });

            should_override || ((!is_hidden || should_include_hidden) && !should_ignore)
        })
        .cloned()
        .collect()
}