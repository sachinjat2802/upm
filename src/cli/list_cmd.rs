/// # `upm list` / `upm ls` CLI Subcommand — Universal Dependency Lister
///
/// Lists all installed dependencies across active ecosystems and foreign `upm.toml` manifest entries.

use crate::acquisition::{AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;

/// Execute universal dependency listing.
pub fn execute_list(path: &Path, filter: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📋 {}{}",
        "│".cyan(),
        "UPM Universal Dependency Lister".bold().white(),
        "              │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);
    let manifest = UpmManifest::load_from_dir(path);

    let filter_lower = filter.map(|f| f.to_lowercase());

    // 1. Manifest Dependencies
    if let Some(ref m) = manifest {
        println!("  {} {} {}", "📦".cyan(), m.project.name.bold(), format!("v{}", m.project.version).dimmed());
        if !m.foreign_dependencies.is_empty() {
            println!("  {}", "Declared Foreign Dependencies (upm.toml):".bold());
            for (key, version) in &m.foreign_dependencies {
                if let Some(ref f) = filter_lower {
                    if !key.to_lowercase().contains(f) {
                        continue;
                    }
                }
                println!("    ├── {} → {}", key.bold().yellow(), version.dimmed());
            }
            println!();
        }
    }

    // 2. Ecosystem Breakdown
    println!("  {}", "Installed Ecosystem Breakdown:".bold());
    for eco in &result.detected_ecosystems {
        if let Some(ref f) = filter_lower {
            if !eco.name.to_lowercase().contains(f) && !eco.language.to_lowercase().contains(f) {
                continue;
            }
        }
        let icon = super::lang_icon(&eco.language);
        println!("    {} {} {}", icon, eco.display_name.bold().magenta(), format!("({})", eco.manifest_file).dimmed());

        // Check if manifest file exists and display status
        if path.join(&eco.manifest_file).exists() {
            println!("      └── Status: Manifest present & configured");
        }
    }
    println!();

    Ok(())
}
