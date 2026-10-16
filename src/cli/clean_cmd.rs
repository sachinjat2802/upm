/// # `cpm clean` CLI Subcommand — Universal Build Artifact Cleaner
///
/// Removes target build directories (`node_modules`, `.venv`, `target`, `vendor`, `_build`)
/// across all active ecosystems in the workspace.

use crate::acquisition::{AdapterRegistry, DetectionEngine};
use colored::Colorize;
use std::path::Path;

/// Execute workspace cleaning.
pub fn execute_clean(path: &Path, dry_run: bool) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🧹 {}{}",
        "│".cyan(),
        "CPM Universal Build Artifact Cleaner".bold().white(),
        "        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    let mut cleaned_dirs = 0;
    let mut total_freed_bytes: u64 = 0;

    println!("  ▶ Scanning for build artifacts to clean...");

    for eco in &result.detected_ecosystems {
        if let Some(ref dir_name) = eco.directory_indicator {
            let target_dir = path.join(dir_name);
            if target_dir.exists() && target_dir.is_dir() {
                let size = get_dir_size(&target_dir);
                let size_mb = size as f64 / (1024.0 * 1024.0);

                if dry_run {
                    println!("    {} [dry-run] Would remove {} ({:.1} MB)", "⊘".yellow(), dir_name.bold(), size_mb);
                } else {
                    match std::fs::remove_dir_all(&target_dir) {
                        Ok(_) => {
                            println!("    {} Cleaned {} ({:.1} MB freed)", "✔".green(), dir_name.bold(), size_mb);
                            cleaned_dirs += 1;
                            total_freed_bytes += size;
                        }
                        Err(e) => {
                            println!("    {} Could not remove {}: {}", "✖".bold().red(), dir_name, e);
                        }
                    }
                }
            }
        }
    }

    let total_freed_mb = total_freed_bytes as f64 / (1024.0 * 1024.0);
    println!();
    if dry_run {
        println!("  {} Dry run complete. Found {} artifact directory(s).", "ℹ".blue(), cleaned_dirs);
    } else {
        println!(
            "  {} Cleaned {} artifact directory(s), freed {:.1} MB disk space!",
            "✨".bold().yellow(),
            cleaned_dirs,
            total_freed_mb
        );
    }
    println!();

    Ok(())
}

fn get_dir_size(path: &Path) -> u64 {
    let mut total = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Ok(meta) = p.metadata() {
                    total += meta.len();
                }
            } else if p.is_dir() {
                total += get_dir_size(&p);
            }
        }
    }
    total
}
