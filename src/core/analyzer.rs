use super::config::Config;
use super::detector::LanguageDetector;
use super::error::{CodeStatsError, Result};
use super::stats::{FileStats, LanguageStats, LineStats};
use ignore::Walk;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct ProjectAnalyzer {
    detector: LanguageDetector,
    config: Option<Config>,
}

impl ProjectAnalyzer {
    pub fn new() -> Self {
        Self {
            detector: LanguageDetector::new(),
            config: None,
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn analyze_project(&mut self, path: &str) -> Result<(LanguageStats, Vec<PathBuf>)> {
        let mut stats = LanguageStats::new();
        let mut other_files = Vec::new();
        self.detector.load_overrides(Path::new(path))?;

        let pb = self.create_progress_bar();

        for entry in Walk::new(path) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error accessing entry: {}", e);
                    continue;
                }
            };

            self.process_entry(entry, &mut stats, &mut other_files)?;
        }

        pb.finish_and_clear();
        Ok((stats, other_files))
    }

    fn create_progress_bar(&self) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message("Analyzing...");
        pb
    }

    fn process_entry(
        &self,
        entry: ignore::DirEntry,
        stats: &mut LanguageStats,
        other_files: &mut Vec<PathBuf>,
    ) -> Result<()> {
        let path = entry.path();

        // Check config-based ignores first
        if let Some(config) = &self.config {
            if config.should_ignore_path(path) {
                return Ok(());
            }
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if config.should_ignore_file(filename) {
                    return Ok(());
                }
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if config.should_ignore_extension(ext) {
                    return Ok(());
                }
            }
        }

        if !entry.file_type().map_or(false, |ft| ft.is_file()) || self.detector.should_ignore(path)
        {
            return Ok(());
        }

        // Pass config to detect_language
        match self.detector.detect_language(path, self.config.as_ref()) {
            Ok(language) => {
                if let Ok(file_stats) = self.analyze_file(path) {
                    if language == "Other" {
                        other_files.push(path.to_path_buf());
                    }
                    stats.update(language, file_stats);
                }
            }
            Err(e) => eprintln!("Error detecting language for {}: {}", path.display(), e),
        }

        Ok(())
    }

    fn analyze_file(&self, path: &Path) -> Result<FileStats> {
        let content = fs::read_to_string(path).map_err(CodeStatsError::Io)?;
        let mut stats = FileStats {
            bytes: content.len(),
            lines: LineStats::default(),
        };

        let mut in_block_comment = false;

        for line in content.lines() {
            let line = line.trim();
            stats.lines.total += 1;

            if line.is_empty() {
                stats.lines.blank += 1;
                continue;
            }

            if in_block_comment {
                stats.lines.comment += 1;
                if line.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            if line.starts_with("//") || line.starts_with("#") {
                stats.lines.comment += 1;
            } else if line.starts_with("/*") {
                stats.lines.comment += 1;
                in_block_comment = true;
            } else {
                stats.lines.code += 1;
            }
        }

        Ok(stats)
    }
}
