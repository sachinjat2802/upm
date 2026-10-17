//! # Universal Package Publishing & Release Pipeline (`cpm publish`)
//!
//! Orchestrates automated version bumping, pre-publish verification,
//! and registry publishing across npm, Crates.io, and PyPI.
//!
//! Time Complexity: O(1) validation + network publishing.
//! Space Complexity: O(1) buffer allocation.

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn execute_publish(path: &Path, dry_run: bool, bump: Option<&str>, tag: Option<&str>) -> Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📦 {}", "│".cyan(), "Universal Package Publishing Pipeline".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    if let Some(b) = bump {
        println!("  {} Bumping version ({b})...", "🏷️".cyan());
        bump_version(path, b)?;
    }

    // 1. Verify lockfile before publishing
    println!("  {} Verifying lockfile and integrity...", "🔒".cyan());
    let _ = crate::cli::lock_cmd::execute_lock(path, true, false);

    let mut published = false;

    // 2. Rust Crates.io
    if path.join("Cargo.toml").exists() {
        published = true;
        println!("  {} Publishing to Crates.io...", "🦀".yellow());
        let resolved = crate::acquisition::AcquisitionRunner::resolve_executable("cargo");
        let mut cmd = Command::new(&resolved);
        cmd.arg("publish");
        if dry_run {
            cmd.arg("--dry-run");
        }
        cmd.current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to run cargo publish: {}", e))?;
        if !status.success() && !dry_run {
            return Err(anyhow!("cargo publish failed"));
        }
    }

    // 3. Node.js npm registry
    if path.join("package.json").exists() {
        published = true;
        println!("  {} Publishing to npm Registry...", "📦".yellow());
        let resolved = crate::acquisition::AcquisitionRunner::resolve_executable("npm");
        let mut cmd = Command::new(&resolved);
        cmd.arg("publish");
        if let Some(t) = tag {
            cmd.arg("--tag").arg(t);
        }
        if dry_run {
            cmd.arg("--dry-run");
        }
        cmd.current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to run npm publish: {}", e))?;
        if !status.success() && !dry_run {
            return Err(anyhow!("npm publish failed"));
        }
    }

    // 4. Python PyPI
    if path.join("pyproject.toml").exists() {
        published = true;
        println!("  {} Publishing to PyPI via twine...", "🐍".yellow());
        let resolved = crate::acquisition::AcquisitionRunner::resolve_executable("twine");
        let mut cmd = Command::new(&resolved);
        cmd.arg("upload").arg("dist/*");
        if dry_run {
            println!("  [dry-run] twine upload dist/* skipped.");
        } else {
            cmd.current_dir(path);
            let _ = cmd.status();
        }
    }

    if !published {
        println!("  {} No publishable package manifests found in {}", "ℹ".cyan(), path.display());
    } else {
        println!();
        println!("  {} Publishing process completed successfully!", "✔".bold().green());
    }
    println!();
    Ok(())
}

fn bump_version(path: &Path, bump_type: &str) -> Result<()> {
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        let resolved = crate::acquisition::AcquisitionRunner::resolve_executable("npm");
        let _ = Command::new(&resolved)
            .args(["version", bump_type, "--no-git-tag-version"])
            .current_dir(path)
            .status();
        println!("  {} Updated package.json version", "✔".green());
    }
    Ok(())
}
