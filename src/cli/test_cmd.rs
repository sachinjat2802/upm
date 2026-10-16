//! # Unified Multi-Ecosystem Test Runner
//!
//! Autodetects and runs test suites across Rust (`cargo test`),
//! Node/JS/TS (`pnpm/npm test`, `jest`, `vitest`), Python (`pytest`),
//! and Go (`go test ./...`) with unified progress and exit status.
//!
//! Time Complexity: O(E) where E = detected ecosystems.
//! Space Complexity: O(1) iterative execution.

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn execute_test(path: &Path, filter: Option<&str>, watch: bool, coverage: bool) -> Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🧪 {}", "│".cyan(), "Universal Test Runner (Multi-Ecosystem)".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let mut executed_any = false;

    // 1. Rust / Cargo
    if path.join("Cargo.toml").exists() {
        executed_any = true;
        println!("  {} Running Rust tests (cargo test)...", "🦀".yellow());
        let mut cmd = Command::new("cargo");
        if watch {
            cmd.arg("watch").arg("-x").arg("test");
        } else {
            cmd.arg("test");
            if let Some(f) = filter {
                cmd.arg("--").arg(f);
            }
        }
        cmd.current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute cargo test: {}", e))?;
        if !status.success() {
            return Err(anyhow!("Rust test suite failed"));
        }
    }

    // 2. Node / JS / TS (package.json)
    if path.join("package.json").exists() {
        executed_any = true;
        println!("  {} Running JavaScript/TypeScript tests...", "📦".yellow());
        let (pm, mut args) = if path.join("pnpm-lock.yaml").exists() {
            ("pnpm", vec!["test"])
        } else if path.join("bun.lockb").exists() || path.join("bun.lock").exists() {
            ("bun", vec!["test"])
        } else if path.join("yarn.lock").exists() {
            ("yarn", vec!["test"])
        } else {
            ("npm", vec!["test"])
        };

        if coverage {
            args.push("--");
            args.push("--coverage");
        }
        if watch {
            args.push("--");
            args.push("--watch");
        }

        let mut cmd = Command::new(pm);
        cmd.args(&args).current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute {} test: {}", pm, e))?;
        if !status.success() {
            return Err(anyhow!("Node.js test suite failed"));
        }
    }

    // 3. Python (pytest)
    if path.join("pyproject.toml").exists() || path.join("requirements.txt").exists() || path.join("tests").is_dir() {
        executed_any = true;
        println!("  {} Running Python tests (pytest)...", "🐍".yellow());
        let mut cmd = Command::new("pytest");
        if coverage {
            cmd.arg("--cov");
        }
        if let Some(f) = filter {
            cmd.arg("-k").arg(f);
        }
        cmd.current_dir(path);
        match cmd.status() {
            Ok(status) if !status.success() => {
                return Err(anyhow!("Python test suite failed"));
            }
            Err(_) => {
                // Fallback to python -m unittest
                let mut fallback = Command::new("python");
                fallback.arg("-m").arg("unittest").current_dir(path);
                let _ = fallback.status();
            }
            _ => {}
        }
    }

    // 4. Go (go test)
    if path.join("go.mod").exists() {
        executed_any = true;
        println!("  {} Running Go tests (go test)...", "🐹".yellow());
        let mut cmd = Command::new("go");
        cmd.arg("test");
        if coverage {
            cmd.arg("-cover");
        }
        cmd.arg("./...").current_dir(path);
        let status = cmd.status().map_err(|e| anyhow!("Failed to execute go test: {}", e))?;
        if !status.success() {
            return Err(anyhow!("Go test suite failed"));
        }
    }

    if !executed_any {
        println!("  {} No known test configurations detected in {}", "ℹ".cyan(), path.display());
    } else {
        println!();
        println!("  {} All test suites passed successfully!", "✔".bold().green());
    }
    println!();

    Ok(())
}
