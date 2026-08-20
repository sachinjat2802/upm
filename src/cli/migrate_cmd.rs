/// # `cpm migrate` CLI Subcommand
///
/// Automated 1-command migration tool that scans an existing repository,
/// bootstraps `upm.toml`, injects CPM SDK helpers, and runs initial
/// dependency acquisition and audit.

use crate::acquisition::{AcquisitionRunner, AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;

/// Automated migration execution.
pub fn execute_migrate(path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ⚡ {}{}",
        "│".cyan(),
        "CPM Automated 1-Command Repository Migration".bold().white(),
        "     │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    // Step 1: Detect active ecosystems
    println!("  {} {}", "Step 1/5:".bold().cyan(), "Scanning existing project markers & manifests...".bold());
    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    let detected_names: Vec<String> = result.detected_ecosystems.iter().map(|e| e.name.clone()).collect();
    if detected_names.is_empty() {
        println!("  {} No existing ecosystems detected. Bootstrapping default polyglot workspace.", "!".yellow());
    } else {
        println!("  {} Detected {} ecosystem(s): {}", "✔".green(), detected_names.len(), detected_names.join(", ").bold().yellow());
    }
    println!();

    // Step 2: Bootstrap upm.toml manifest
    println!("  {} {}", "Step 2/5:".bold().cyan(), "Bootstrapping upm.toml manifest (Zero Breaking Changes)...".bold());
    let primary_lang = result.detected_ecosystems.first().map(|e| e.language.as_str()).unwrap_or("javascript");
    let manifest = UpmManifest::new("migrated-app", "0.1.0", Some(primary_lang), detected_names.clone());
    manifest.save_to_dir(path)?;
    println!("  {} upm.toml manifest created.", "✔".green());
    println!();

    // Step 3: Inject CPM SDK helpers
    println!("  {} {}", "Step 3/5:".bold().cyan(), "Injecting CPM Client SDK helpers into ./sdk/...".bold());
    let sdk_dir = path.join("sdk");
    std::fs::create_dir_all(&sdk_dir)?;

    let python_sdk = include_str!("../../sdk/python/cpm_sdk.py");
    let node_sdk = include_str!("../../sdk/node/cpm_sdk.js");
    let node_dts = include_str!("../../sdk/node/cpm_sdk.d.ts");

    std::fs::write(sdk_dir.join("cpm_sdk.py"), python_sdk)?;
    std::fs::write(sdk_dir.join("cpm_sdk.js"), node_sdk)?;
    std::fs::write(sdk_dir.join("cpm_sdk.d.ts"), node_dts)?;
    println!("  {} SDK helpers installed at ./sdk/ (Python, Node.js/TypeScript).", "✔".green());
    println!();

    // Step 4: Run parallel dependency acquisition
    println!("  {} {}", "Step 4/5:".bold().cyan(), "Running parallel dependency acquisition...".bold());
    AcquisitionRunner::run_install(path, &result.detected_ecosystems, false, true, None)?;
    println!();

    // Step 5: Security Lockfile Audit
    println!("  {} {}", "Step 5/5:".bold().cyan(), "Auditing lockfile reproducibility & security...".bold());
    for eco in &result.detected_ecosystems {
        if let Some(ref lockfile) = eco.lockfile_file {
            let lock_path = path.join(lockfile);
            if lock_path.exists() {
                println!("    {} {} → {}", "✔".green(), eco.display_name.bold(), format!("{} present (pinned)", lockfile).dimmed());
            } else {
                println!("    {} {} → {}", "!".yellow(), eco.display_name.bold(), format!("{} missing (unpinned)", lockfile).yellow());
            }
        }
    }
    println!();

    println!("  {}", "╭──────────────────────────────────────────────────────╮".green());
    println!("  {}  ✨ {}{}",
        "│".green(),
        "Migration Complete! Your app is now powered by CPM.".bold().white(),
        "  │".green(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".green());
    println!();
    println!("  {} {}", "Next Steps:".bold(), "Import SDK to call cross-language methods:");
    println!("    TypeScript: {}", "const { CpmBridge } = require('./sdk/cpm_sdk');".yellow());
    println!("    Python:     {}", "from sdk.cpm_sdk import CpmBridge".yellow());
    println!();

    Ok(())
}
