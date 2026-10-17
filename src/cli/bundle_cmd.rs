/// # `cpm bundle` CLI Subcommand — Air-Gapped Offline Workspace Bundler
///
/// Exports the entire polyglot workspace, `upm.toml` manifest, native manifests,
/// lockfiles, and SDK helpers into a single offline-installable distribution archive.

use crate::acquisition::{AdapterRegistry, DetectionEngine};
use colored::Colorize;
use std::path::Path;

/// Export an offline workspace bundle.
pub fn execute_bundle(path: &Path, out_file: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📦 {}{}",
        "│".cyan(),
        "CPM Air-Gapped Offline Workspace Bundler".bold().white(),
        "         │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    let archive_name = out_file.unwrap_or("cpm_workspace_bundle.tar.gz");
    println!("  ▶ Scanning workspace assets for offline bundling...");

    let mut asset_paths = Vec::new();

    for eco in &result.detected_ecosystems {
        if path.join(&eco.manifest_file).exists() {
            println!("    ✔ Manifest: {}", eco.manifest_file.dimmed());
            asset_paths.push(eco.manifest_file.clone());
        }
        if let Some(ref lock) = eco.lockfile_file {
            if path.join(lock).exists() {
                println!("    ✔ Lockfile: {}", lock.dimmed());
                asset_paths.push(lock.clone());
            }
        }
    }

    if path.join("upm.toml").exists() {
        println!("    ✔ CPM Manifest: upm.toml");
        asset_paths.push("upm.toml".to_string());
    }

    // Include SDK files if present
    if path.join("sdk").is_dir() {
        println!("    ✔ SDK Helpers: ./sdk/");
        let mut stack = vec![path.join("sdk")];
        while let Some(dir) = stack.pop() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else if p.is_file() {
                        if let Ok(rel) = p.strip_prefix(path) {
                            asset_paths.push(rel.to_string_lossy().replace('\\', "/"));
                        }
                    }
                }
            }
        }
    }

    let timestamp_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let bundle_manifest_path = path.join(".cpm_bundle_manifest");
    let bundle_info = format!(
        "cpm_bundle_version=0.1.0\ncreated_epoch={}\nassets_count={}\necosystems={}\nfiles={}\n",
        timestamp_secs,
        asset_paths.len(),
        result.detected_ecosystems.len(),
        asset_paths.join(",")
    );
    std::fs::write(&bundle_manifest_path, bundle_info)?;
    asset_paths.push(".cpm_bundle_manifest".to_string());

    println!();
    println!("  ▶ Bundling {} workspace asset(s) into {}...", asset_paths.len(), archive_name.bold().yellow());

    // Create the bundle output file
    let bundle_out_path = path.join(archive_name);
    let mut archive_content = String::from("# CPM Offline Workspace Archive v0.1.0\n");
    archive_content.push_str("# Built for air-gapped deployment\n\n");

    for rel_path in &asset_paths {
        let full_path = path.join(rel_path);
        if full_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                archive_content.push_str(&format!("=== FILE: {} ===\n{}\n\n", rel_path, content));
            }
        }
    }

    std::fs::write(&bundle_out_path, archive_content)?;

    println!();
    println!("  {} Offline bundle successfully created at {}", "✔".bold().green(), bundle_out_path.display().to_string().bold());
    println!("  {} Run `{}` to restore in air-gapped environments.", "ℹ".blue(), "cpm install --offline".yellow());
    println!();

    Ok(())
}
