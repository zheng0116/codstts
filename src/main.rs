use clap::Parser;
use codstts::core::{Config, ProjectAnalyzer, StatsDisplay};
use colored::*;

#[derive(Parser)]
#[command(
    author = "zheng0116",
    version,
    about = "A code statistics tool that analyzes programming language distribution in projects"
)]
struct Cli {
    /// Path to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,

    /// Use simple view mode
    #[arg(short = 's', long = "simple", conflicts_with = "detail")]
    simple: bool,

    /// Use detailed view mode (default)
    #[arg(short = 'd', long = "detail", conflicts_with = "simple")]
    detail: bool,

    /// Use config file (defaults to .codstts.toml)
    #[arg(short, long)]
    config: Option<String>,

    /// Show debug information
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let config = Config::load().unwrap_or_default();

    let mut analyzer = ProjectAnalyzer::new();
    analyzer.set_config(config);
    match analyzer.analyze_project(&cli.path) {
        Ok((stats, other_files)) => {
            if cli.simple || (!cli.simple && !cli.detail) {
                StatsDisplay::print_simple_view(&stats);
            } else {
                StatsDisplay::print_detailed_view(&stats);
            }

            if !other_files.is_empty() && (cli.detail || (!cli.simple && !cli.detail)) {
                println!("\n{}", "Unrecognized files:".yellow().bold());
                for file in other_files {
                    if let Some(file_name) = file.file_name().and_then(|n| n.to_str()) {
                        println!("  {}", file_name);
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error analyzing project: {}", e);
            std::process::exit(1);
        }
    }
}
