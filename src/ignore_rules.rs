use std::path::PathBuf;
use glob::Pattern;

pub struct IgnoreRules {
    pub rules: Vec<Pattern>,
}

impl IgnoreRules {
    pub fn new(rules: Vec<String>) -> Self {
        let rules = rules.into_iter()
            .map(|r| Pattern::new(&r).expect("Invalid ignore pattern"))
            .collect();
        IgnoreRules { rules }
    }
}

pub fn apply_ignore_rules(ignore_rules: &IgnoreRules, files: &[PathBuf]) -> Vec<PathBuf> {
    files.iter()
        .filter(|file| !ignore_rules.rules.iter().any(|rule| rule.matches_path(file)))
        .cloned()
        .collect()
}