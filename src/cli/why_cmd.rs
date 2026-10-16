/// # `cpm why` CLI Subcommand — Dependency Reason Explainer
///
/// Explains why a package is included in the workspace (direct manifest entry,
/// foreign dependency in `upm.toml`, or ecosystem default dependency).

use crate::acquisition::{AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;

/// Execute dependency explanation analysis.
pub fn execute_why(path: &Path, package: &str) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ❓ {}{}",
        "│".cyan(),
        format!("Dependency Reason Explainer [{}]", package).bold().white(),
        "        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);
    let manifest = UpmManifest::load_from_dir(path);

    let mut found = false;

    // Check upm.toml foreign dependencies
    if let Some(ref m) = manifest {
        for (key, ver) in &m.foreign_dependencies {
            if key.contains(package) {
                println!(
                    "  {} Direct foreign dependency in {}:",
                    "✔".green().bold(),
                    "upm.toml".bold()
                );
                println!(
                    "    └── Key: {} | Target version: {}",
                    key.bold().yellow(),
                    ver.dimmed()
                );
                println!();
                found = true;
            }
        }
    }

    // Check native ecosystem manifests
    for eco in &result.detected_ecosystems {
        let manifest_path = path.join(&eco.manifest_file);
        if manifest_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                if content.contains(package) {
                    println!(
                        "  {} Found in native manifest {}:",
                        "✔".green().bold(),
                        eco.manifest_file.bold().cyan()
                    );
                    println!(
                        "    └── Ecosystem: {} ({})",
                        eco.display_name.magenta(),
                        eco.language
                    );
                    println!();
                    found = true;
                }
            }
        }
    }

    if !found {
        println!(
            "  {} Package '{}' was not found as a direct dependency in any manifest.",
            "ℹ".blue(),
            package.bold().yellow()
        );
        println!(
            "  {} It may be a transitive dependency or hasn't been added yet with `cpm add`.",
            "  ".dimmed()
        );
        println!();
    }

    Ok(())
}
