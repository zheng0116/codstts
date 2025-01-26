//! Codstts - Code Statistics Tool
//!
//! A command-line tool and library for analyzing programming language distribution in projects.
//! This tool helps developers understand their codebase composition by providing detailed
//! statistics about programming languages used, including code lines, comments, and blank lines.
//!
//! # Features
//!
//! * Multiple programming language recognition
//! * Accurate counting of code, comments, and blank lines
//! * Support for `.gitattributes` language overrides
//! * Configurable ignore patterns
//! * Beautiful command-line interface with progress indication
//! * Both simple and detailed display modes
//!
//! # Example
//!
//! ```no_run
//! use codstts::core::{ProjectAnalyzer, StatsDisplay};
//!
//! let mut analyzer = ProjectAnalyzer::new();
//! if let Ok((stats, _)) = analyzer.analyze_project(".") {
//!     StatsDisplay::print_simple_view(&stats);
//! }
//! ```
//!
//! # Configuration
//!
//! The tool can be configured using a `.codstts.toml` file in your project root:
//!
//! ```toml
//! # Paths to ignore
//! ignore_paths = ["vendor", "node_modules"]
//!
//! # Language mappings
//! [language_mappings]
//! "jsx" = "React"
//! "tsx" = "React"
//! ```
//!
//! # Supported Languages
//!
//! The tool automatically recognizes many popular programming languages including:
//! - Rust
//! - Python
//! - JavaScript/TypeScript
//! - Java
//! - C/C++
//! - Go
//! - And many more...
//!
//! # Output Formats
//!
//! The tool provides two output formats:
//!
//! 1. Simple mode: Shows a basic overview with language percentages
//! 2. Detailed mode: Provides comprehensive statistics including:
//!    - Total lines of code
//!    - Comment lines
//!    - Blank lines
//!    - File counts
//!    - Byte sizes

// Copyright and license notice
// codstts - A code statistics tool that analyzes programming language distribution in projects
// Copyright 2024 stackzheng
//
// Licensed under the MIT License

// Allow certain clippy lints that are acceptable in this context
#![allow(clippy::result_large_err)]

// Re-export the core module which contains all public APIs
pub mod core;

// Development-only dependencies
#[cfg(test)]
extern crate criterion;
