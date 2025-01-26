use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct FileStats {
    pub bytes: usize,
    pub lines: LineStats,
}

#[derive(Debug, Default, Clone)]
pub struct LineStats {
    pub total: usize,
    pub code: usize,
    pub comment: usize,
    pub blank: usize,
}

#[derive(Debug)]
pub struct LanguageStats {
    pub stats: HashMap<String, FileStats>,
    pub total_files: usize,
}

impl LanguageStats {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            total_files: 0,
        }
    }

    pub fn update(&mut self, language: &str, stats: FileStats) {
        let entry = self.stats.entry(language.to_string()).or_default();
        entry.bytes += stats.bytes;
        entry.lines.total += stats.lines.total;
        entry.lines.code += stats.lines.code;
        entry.lines.comment += stats.lines.comment;
        entry.lines.blank += stats.lines.blank;
        self.total_files += 1;
    }
}

pub trait RoundToDecimals {
    fn round_to_decimal(self, decimal_places: i32) -> Self;
}

impl RoundToDecimals for f64 {
    fn round_to_decimal(self, decimal_places: i32) -> Self {
        let factor = 10.0f64.powi(decimal_places);
        (self * factor).round() / factor
    }
}
