use super::config::Config;
use super::error::{CodeStatsError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct LanguageDetector {
    vendored_paths: Vec<PathBuf>,
    documentation_paths: Vec<PathBuf>,
    generated_paths: Vec<PathBuf>,
    linguist_overrides: HashMap<PathBuf, String>,
}

impl LanguageDetector {
    pub(crate) fn new() -> Self {
        Self {
            vendored_paths: Vec::new(),
            documentation_paths: Vec::new(),
            generated_paths: Vec::new(),
            linguist_overrides: HashMap::new(),
        }
    }

    pub(crate) fn load_overrides(&mut self, root: &Path) -> Result<()> {
        let gitattributes = root.join(".gitattributes");
        if gitattributes.exists() {
            let content = fs::read_to_string(&gitattributes).map_err(CodeStatsError::Io)?;
            for line in content.lines() {
                if line.contains("linguist-language=") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Some(lang) = parts[1].strip_prefix("linguist-language=") {
                            self.linguist_overrides
                                .insert(root.join(parts[0]), lang.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn detect_language(
        &self,
        path: &Path,
        config: Option<&Config>,
    ) -> Result<&'static str> {
        // Check for linguist override
        if let Some(lang) = self.linguist_overrides.get(path) {
            return Ok(Box::leak(lang.clone().into_boxed_str()));
        }

        // Check config language mappings
        if let Some(config) = config {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(lang) = config.get_language_mapping(ext) {
                    return Ok(Box::leak(lang.clone().into_boxed_str()));
                }
            }
        }

        // Check for Dockerfile
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.eq_ignore_ascii_case("dockerfile")
                || file_name.to_lowercase().ends_with(".dockerfile")
            {
                return Ok("Dockerfile");
            }
        }

        // Fallback to extension
        Ok(path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "py" | "pyi" | "pyx" => "Python",
                "js" | "mjs" => "JavaScript",
                "ts" | "mts" | "cts" => "TypeScript",
                "tsx" | "jsx" => "React",
                "css" | "scss" | "sass" | "less" => "CSS",
                "html" | "htm" | "xhtml" => "HTML",
                "sh" | "bash" | "zsh" => "Shell",
                "rs" => "Rust",
                "go" | "mod" => "Go",
                "java" | "kt" | "kts" => "Java",
                "vue" => "Vue",
                "rb" | "rake" | "gemspec" => "Ruby",
                "php" | "php5" | "phtml" => "PHP",
                "c" | "h" => "C",
                "cpp" | "hpp" | "cc" | "cxx" | "hxx" => "C++",
                "swift" => "Swift",
                "m" | "mm" => "Objective-C",
                "cs" => "C#",
                "pl" | "pm" => "Perl",
                "scala" => "Scala",
                "lua" => "Lua",
                "r" | "R" => "R",
                "dart" => "Dart",
                "ex" | "exs" => "Elixir",
                "hs" => "Haskell",
                "dockerfile" => "Dockerfile",
                _ => "Other",
            })
            .unwrap_or("Other"))
    }

    pub(crate) fn should_ignore(&self, path: &Path) -> bool {
        if self.vendored_paths.iter().any(|p| path.starts_with(p))
            || self.documentation_paths.iter().any(|p| path.starts_with(p))
            || self.generated_paths.iter().any(|p| path.starts_with(p))
        {
            return true;
        }

        // Built-in ignore rules
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            match file_name {
                "Cargo.toml" | "Cargo.lock" | "package.json" | "package-lock.json"
                | "yarn.lock" | "Gemfile" | "Gemfile.lock" | "requirements.txt" | ".gitignore"
                | ".gitattributes" | ".editorconfig" | "composer.json" | "composer.lock"
                | "poetry.lock" | "README.md" | "readme.md" | "README" | "LICENSE"
                | "CHANGELOG.md" => return true,
                _ => {
                    if (file_name.contains("README") || file_name.contains("readme"))
                        && file_name.ends_with(".md")
                    {
                        return true;
                    }
                }
            }
        }

        // Standard ignored extensions
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "exe"
                    | "dll"
                    | "so"
                    | "dylib"
                    | "pyc"
                    | "jar"
                    | "war"
                    | "ear"
                    | "zip"
                    | "tar"
                    | "gz"
                    | "7z"
                    | "rar"
                    | "pdf"
                    | "doc"
                    | "docx"
                    | "xls"
                    | "xlsx"
                    | "png"
                    | "jpg"
                    | "jpeg"
                    | "gif"
                    | "bmp"
                    | "ico"
                    | "svg"
                    | "ttf"
                    | "otf"
                    | "woff"
                    | "woff2"
                    | "eot"
                    | "mp3"
                    | "mp4"
                    | "avi"
                    | "mov"
                    | "wmv"
                    | "wav"
                    | "flac"
                    | "ogg"
                    | "webm"
                    | "lock"
                    | "proto"
                    | "md"
                    | "markdown"
                    | "rst"
                    | "txt"
                    | "adoc"
                    | "asciidoc"
                    | "json"
                    | "yaml"
                    | "yml"
                    | "xml"
                    | "toml"
                    | "ini"
                    | "cfg"
                    | "conf"
                    | "properties"
                    | "prop"
                    | "env"
                    | "bat"
            )
        } else {
            false
        }
    }
}
