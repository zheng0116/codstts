# Codstts
  ![Codstts](https://img.shields.io/badge/passing-code-brightgreen) ![License](https://img.shields.io/badge/license-MIT-blue) ![Rust Version](https://img.shields.io/badge/rust-1.80.1-blue)

English | [中文](./README_zh.md)
## 🌟 Overview
A code statistics tool written in Rust that analyzes programming language distribution in projects.

## 🚀 Features
- Multiple programming language recognition
- Simple and detailed display modes
- Customizable configuration
- Accurate code, comment, and blank line counting
- Beautiful command-line interface
- Support for .gitattributes language overrides
- Ignores common binary files, documentation, and configuration files

## Installation

```bash
cargo install codstts
```

## Usage

Basic usage:

```bash
# Analyze current directory
codstts

# Use simple mode
codstts -s

# Use detailed mode
codstts -d

# Analyze specific directory
codstts path/to/your/project
```

### Configuration

You can create a `.codstts.toml` file in your project root for custom configuration:

```toml
# Paths to ignore
ignore_paths = ["vendor", "node_modules"]

# Language mappings
[language_mappings]
"jsx" = "React"
"tsx" = "React"

# Extensions to exclude
exclude_extensions = ["log", "tmp"]

# Files to exclude
exclude_files = ["LICENSE", "README.md"]
```

## Output Examples

Simple mode:
```
Languages
Languages
██████████████████████████████████████████████████

● Python 62.1% (15666 bytes)
● JavaScript 16.1% (4046 bytes)
● CSS 13.9% (3504 bytes)
● HTML 7.3% (1836 bytes)
● Shell 0.6% (149 bytes)
```

Detailed mode includes additional information such as code lines, comment lines, etc.

## License

MIT

## Contributing

Issues and Pull Requests are welcome!