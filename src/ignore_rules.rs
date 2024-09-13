use std::path::{PathBuf, Path};
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

pub fn apply_ignore_rules(
    ignore_rules: &IgnoreRules,
    files: &HashSet<PathBuf>,
    include_file_overrides: &[String],
    include_dir_overrides: &[String],
    input_dir: &Path,
) -> Vec<PathBuf> {
    files
        .iter()
        .filter(|file| {
            let relative_file = file.strip_prefix(input_dir).unwrap_or(file);
            let file_str = relative_file.to_string_lossy();
            let file_name = relative_file.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let is_hidden = ignore_rules.ignore_hidden.matches(file_name);
            let should_include_hidden = ignore_rules.include_hidden.iter().any(|pattern| pattern.matches(file_name));
            let should_include_file = include_file_overrides.iter().any(|override_path| {
                file_str == *override_path
            });
            let should_include_dir = include_dir_overrides.iter().any(|dir| {
                relative_file.starts_with(dir)
            });

            // Adjust should_ignore to skip directory-level ignores if directory is included
            let should_ignore = ignore_rules.rules.iter().any(|rule| {
                let rule_str = rule.as_str();
                let is_dir_pattern = rule_str.ends_with("/*");
                if is_dir_pattern && should_include_dir {
                    // Skip directory-level ignore if directory is included
                    false
                } else {
                    rule.matches_path(relative_file)
                }
            });

            if should_include_file {
                true
            } else {
                (!is_hidden || should_include_hidden) && !should_ignore
            }
        })
        .cloned()
        .collect()
}