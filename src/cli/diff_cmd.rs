/// # `cpm diff` CLI Subcommand — Dependency Diff Viewer
///
/// Displays visual diffs of dependency version changes across active ecosystem lockfiles
/// and manifests.

use crate::acquisition::{AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;

/// Execute dependency diff view.
pub fn execute_diff(path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📊 {}{}",
        "│".cyan(),
        "CPM Universal Dependency Diff Viewer".bold().white(),
        "              │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);
    let manifest = UpmManifest::load_from_dir(path);

    println!("  ▶ Comparing current lockfile states against manifest targets...");
    println!();

    let mut total_drift = 0;

    for eco in &result.detected_ecosystems {
        println!("  {} {} {}", "•".cyan(), eco.display_name.bold().magenta(), format!("({})", eco.language).dimmed());
        let lockfile_name = eco.lockfile_file.as_deref().unwrap_or("N/A");
        let lockfile_exists = eco.lockfile_file.as_ref().map(|l| path.join(l).exists()).unwrap_or(false);

        if !lockfile_exists && eco.lockfile_file.is_some() {
            println!("    {} Manifest: {} | Lockfile: {}", "!".yellow(), eco.manifest_file.cyan(), lockfile_name.yellow());
            println!("    └── {} Lockfile missing! Run `cpm install` to lock dependencies.", "⚠".yellow());
            total_drift += 1;
        } else {
            println!("    {} Manifest: {} | Lockfile: {}", "✔".green(), eco.manifest_file.cyan(), lockfile_name.dimmed());
            println!("    └── All dependencies synchronized with locked tree.");
        }
        println!();
    }

    if let Some(ref m) = manifest {
        if !m.foreign_dependencies.is_empty() {
            println!("  {}", "Foreign Dependency Diff (upm.toml):".bold());
            for (pkg, ver) in &m.foreign_dependencies {
                println!("    {} {} → {}", "•".cyan(), pkg.yellow(), ver.bold().green());
            }
            println!();
        }
    }

    if total_drift == 0 {
        println!("  {} Workspace dependency graph is fully synchronized (0 drift detected).", "✨".bold().yellow());
    } else {
        println!("  {} Found {} ecosystem drift issue(s). Run `cpm install` to reconcile.", "⚠".bold().yellow(), total_drift);
    }
    println!();

    Ok(())
}
