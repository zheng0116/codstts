use super::stats::{LanguageStats, RoundToDecimals};
use colored::*;

pub struct StatsDisplay;

impl StatsDisplay {
    pub(crate) fn get_language_color(language: &str) -> Color {
        match language {
            "Python" => Color::Blue,
            "JavaScript" => Color::Yellow,
            "CSS" => Color::Magenta,
            "HTML" => Color::Red,
            "Shell" => Color::Green,
            "Rust" => Color::BrightRed,
            "Go" => Color::Cyan,
            "Java" => Color::BrightYellow,
            "Vue" => Color::Green,
            "React" => Color::Blue,
            "TypeScript" => Color::Blue,
            "Ruby" => Color::Red,
            "PHP" => Color::Magenta,
            "C" => Color::Blue,
            "C++" => Color::Magenta,
            "Dockerfile" => Color::Cyan,
            _ => Color::White,
        }
    }

    pub fn print_simple_view(stats: &LanguageStats) {
        println!("\n{}", "Languages".bold());

        let total_bytes: usize = stats.stats.values().map(|s| s.bytes).sum();
        let mut langs: Vec<_> = stats.stats.iter().collect();
        langs.sort_by(|a, b| b.1.bytes.cmp(&a.1.bytes));

        let mut percentages: Vec<f64> = langs
            .iter()
            .map(|(_, stats)| (stats.bytes as f64 / total_bytes as f64 * 100.0))
            .collect();

        for percentage in percentages.iter_mut() {
            *percentage = (*percentage * 10.0).round() / 10.0;
        }

        let total_percentage: f64 = percentages.iter().sum();

        if (total_percentage - 100.0).abs() > 1e-10 {
            if let Some(max_percentage) = percentages
                .iter_mut()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
            {
                *max_percentage =
                    (*max_percentage - (total_percentage - 100.0)).round_to_decimal(1);
            }
        }

        let bar_width = 50;
        for ((language, _), percentage) in langs.iter().zip(percentages.iter()) {
            let bar_length = (percentage * bar_width as f64 / 100.0).round() as usize;
            let bar_segment = "█".repeat(bar_length);
            print!("{}", bar_segment.color(Self::get_language_color(language)));
        }
        println!("\n");

        for ((language, stats), percentage) in langs.iter().zip(percentages.iter()) {
            let color = Self::get_language_color(language);
            println!(
                "{} {} {:.1}% ({} bytes)",
                "●".color(color),
                language.white(),
                percentage,
                stats.bytes
            );
        }
        println!();
    }

    pub fn print_detailed_view(stats: &LanguageStats) {
        println!("\n{}", "📊 Code Statistics".bright_green().bold());
        println!(
            "{} {}",
            "Total files analyzed:".bright_white(),
            stats.total_files
        );

        let mut langs: Vec<_> = stats.stats.iter().collect();
        langs.sort_by(|a, b| b.1.bytes.cmp(&a.1.bytes));

        println!("\n{}", "Language Distribution:".bright_yellow().bold());

        for (language, stats) in langs {
            let color = Self::get_language_color(language);
            println!("\n{}", language.color(color).bold());
            println!("  {} {} bytes", "Size:".bright_white(), stats.bytes);
            println!("  {} {}", "Total lines:".bright_white(), stats.lines.total);
            println!("  {} {}", "Code:".bright_green(), stats.lines.code);
            println!("  {} {}", "Comments:".bright_yellow(), stats.lines.comment);
            println!("  {} {}", "Blank:".bright_black(), stats.lines.blank);

            let code_percent =
                (stats.lines.code as f64 / stats.lines.total as f64 * 100.0) as usize;
            let comment_percent =
                (stats.lines.comment as f64 / stats.lines.total as f64 * 100.0) as usize;
            let blank_percent =
                (stats.lines.blank as f64 / stats.lines.total as f64 * 100.0) as usize;

            let bar = format!(
                "  {}{}{}",
                "█".repeat(code_percent / 2).bright_green(),
                "█".repeat(comment_percent / 2).bright_yellow(),
                "█".repeat(blank_percent / 2).bright_black()
            );
            println!("{}", bar);
        }
        println!();
    }
}
