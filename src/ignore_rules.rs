use std::path::PathBuf;
use glob::Pattern;
use crate::config::Config;
use std::collections::HashSet;

pub struct IgnoreRules {
    pub rules: Vec<Pattern>,
    pub include_hidden: Vec<Pattern>,
    pub ignore_hidden: Pattern,
}

impl IgnoreRules {
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

pub fn apply_ignore_rules(ignore_rules: &IgnoreRules, files: &HashSet<PathBuf>) -> Vec<PathBuf> {
    files.iter()
        .filter(|file| {
            let file_name = file.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let is_hidden = ignore_rules.ignore_hidden.matches(file_name);
            let should_ignore = ignore_rules.rules.iter().any(|rule| rule.matches_path(file));
            let should_include_hidden = ignore_rules.include_hidden.iter().any(|pattern| pattern.matches(file_name));

            (!is_hidden || should_include_hidden) && !should_ignore
        })
        .cloned()
        .collect()
}