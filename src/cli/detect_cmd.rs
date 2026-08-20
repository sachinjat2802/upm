/// # `cpm detect` / `upm detect`
///
/// Scans a workspace directory and produces a scored detection report
/// showing which package managers are present, their confidence scores,
/// and which signals contributed to each score.

use crate::acquisition::{AdapterRegistry, DetectionEngine};
use colored::Colorize;
use std::path::Path;

/// Execute the `detect` subcommand.
pub fn execute_detect(path: &Path) -> anyhow::Result<()> {
    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔍 {}{}",
        "│".cyan(),
        "Ecosystem Detection Report".bold().white(),
        "                        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
    println!("  {} {}", "Target".dimmed(), path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display().to_string().yellow());
    println!();

    // Table header
    println!(
        "  {:<20} {:<12} {:>6}  {}",
        "ECOSYSTEM".bold(),
        "LANGUAGE".bold(),
        "SCORE".bold(),
        "SIGNALS".bold()
    );
    println!("  {}", "─".repeat(72).dimmed());

    let has_any_score = result.scores.iter().any(|s| s.total_score > 0);

    for score in &result.scores {
        if score.total_score == 0 && has_any_score {
            continue; // Only show zero-scores if nothing matched
        }

        let icon = if score.winner {
            "✔".green().bold()
        } else if score.total_score > 0 {
            "·".yellow()
        } else {
            "·".dimmed()
        };

        let lang_icon = match score.adapter.language.as_str() {
            "javascript" => "📦",
            "python" => "🐍",
            "rust" => "🦀",
            "go" => "🐹",
            "java" => "☕",
            "php" => "🐘",
            "ruby" => "💎",
            "csharp" => "🔷",
            "dart" => "🎯",
            "elixir" => "💧",
            _ => "📦",
        };

        let score_display = if score.winner {
            score.total_score.to_string().green().bold()
        } else if score.total_score > 0 {
            score.total_score.to_string().yellow()
        } else {
            score.total_score.to_string().dimmed()
        };

        let signals_str = score
            .details
            .iter()
            .map(|d| format!("+{} {}", d.weight, d.signal))
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "  {} {:<18} {} {:<9} {:>6}  {}",
            icon,
            score.adapter.display_name,
            lang_icon,
            score.adapter.language,
            score_display,
            signals_str.dimmed()
        );
    }

    println!("  {}", "─".repeat(72).dimmed());

    // Winners summary
    println!();
    if result.detected_ecosystems.is_empty() {
        println!("  {} {}", "!".bold().yellow(), "No package manager signals detected.".yellow());
        println!("  {} Run {} to initialize a polyglot project.", "›".dimmed(), "cpm init".bold().cyan());
    } else {
        println!("  {} {}:", "✔".green().bold(), "Active ecosystem(s)".bold());
        for eco in &result.detected_ecosystems {
            let icon = match eco.language.as_str() {
                "javascript" => "📦",
                "python" => "🐍",
                "rust" => "🦀",
                "go" => "🐹",
                "java" => "☕",
                _ => "📦",
            };
            println!(
                "    {} {} {} → {}",
                icon,
                eco.display_name.bold().green(),
                format!("({})", eco.manifest_file).dimmed(),
                eco.install_cmd.join(" ").cyan()
            );
        }
    }
    println!();

    Ok(())
}
