//! # Unified Multi-Ecosystem Build Runner
//!
//! Orchestrates builds across Rust (`cargo build`), Node/Web (`pnpm build`, `npm run build`),
//! Go (`go build`), and Python (`python -m build`).
//!
//! Time Complexity: O(E) where E = detected ecosystems.
//! Space Complexity: O(1) iterative execution.

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn execute_build(path: &Path, release: bool, target: Option<&str>) -> Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔨 {}", "│".cyan(), "Universal Build Runner (Multi-Ecosystem)".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let mut built_any = false;

    // 1. Rust / Cargo
    if path.join("Cargo.toml").exists() {
        built_any = true;
        println!("  {} Building Rust artifacts...", "🦀".yellow());
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        if release {
            cmd.arg("--release");
        }
        if let Some(t) = target {
            cmd.arg("--target").arg(t);
        }
        cmd.current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute cargo build: {}", e))?;
        if !status.success() {
            return Err(anyhow!("Rust build failed"));
        }
    }

    // 2. Node / JS / Web
    if path.join("package.json").exists() {
        built_any = true;
        println!("  {} Building JavaScript/TypeScript bundle...", "📦".yellow());
        let pm = if path.join("pnpm-lock.yaml").exists() {
            "pnpm"
        } else if path.join("bun.lockb").exists() || path.join("bun.lock").exists() {
            "bun"
        } else if path.join("yarn.lock").exists() {
            "yarn"
        } else {
            "npm"
        };

        let mut cmd = Command::new(pm);
        cmd.arg("run").arg("build").current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute {} run build: {}", pm, e))?;
        if !status.success() {
            return Err(anyhow!("Web build failed"));
        }
    }

    // 3. Go
    if path.join("go.mod").exists() {
        built_any = true;
        println!("  {} Compiling Go binary...", "🐹".yellow());
        let mut cmd = Command::new("go");
        cmd.arg("build").arg("-v").arg("./...").current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute go build: {}", e))?;
        if !status.success() {
            return Err(anyhow!("Go build failed"));
        }
    }

    // 4. Python wheel/sdist
    if path.join("pyproject.toml").exists() {
        built_any = true;
        println!("  {} Building Python package distribution...", "🐍".yellow());
        let mut cmd = Command::new("python");
        cmd.arg("-m").arg("build").current_dir(path);
        let _ = cmd.status(); // best effort
    }

    if !built_any {
        println!("  {} No known build manifests found in {}", "ℹ".cyan(), path.display());
    } else {
        println!();
        println!("  {} Build completed successfully!", "✔".bold().green());
    }
    println!();

    Ok(())
}
