use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeStatsError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse file content: {0}")]
    ParseError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Language detection failed: {0}")]
    LanguageDetectionError(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

pub type Result<T> = std::result::Result<T, CodeStatsError>;
