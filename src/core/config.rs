use super::error::{CodeStatsError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub ignore_paths: Vec<PathBuf>,

    #[serde(default)]
    pub language_mappings: std::collections::HashMap<String, String>,

    #[serde(default)]
    pub exclude_extensions: Vec<String>,

    #[serde(default)]
    pub exclude_files: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = std::env::current_dir()?.join(".codstts.toml");

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(config_path).map_err(CodeStatsError::Io)?;
        toml::from_str(&content).map_err(|e| CodeStatsError::Config(e.to_string()))
    }

    pub(crate) fn should_ignore_path(&self, path: &std::path::Path) -> bool {
        self.ignore_paths.iter().any(|p| path.starts_with(p))
    }

    pub(crate) fn get_language_mapping(&self, extension: &str) -> Option<&String> {
        self.language_mappings.get(extension)
    }

    pub(crate) fn should_ignore_extension(&self, extension: &str) -> bool {
        self.exclude_extensions.iter().any(|e| e == extension)
    }

    pub(crate) fn should_ignore_file(&self, filename: &str) -> bool {
        self.exclude_files.iter().any(|f| f == filename)
    }
}
