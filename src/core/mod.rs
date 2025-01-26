mod analyzer;
mod config;
mod detector;
mod display;
mod error;
mod stats;

pub use analyzer::ProjectAnalyzer;
pub use config::Config;
pub use display::StatsDisplay;
pub use error::{CodeStatsError, Result};
pub use stats::{FileStats, LanguageStats, LineStats};
