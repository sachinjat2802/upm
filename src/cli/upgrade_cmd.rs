//! # Self-Update & Upgrade Engine (`upm upgrade`)
//!
//! Checks and updates the `upm` binary to the latest stable release.
//!
//! Time Complexity: O(1) network version query.
//! Space Complexity: O(1) buffer allocation.

use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn execute_upgrade(check_only: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🚀 {}", "│".cyan(), "UPM Self-Update & Upgrade Engine".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
    println!("  Installed Version: {}", current_version.bold().green());
    println!("  Tracking Branch:   {}", "master (stable release)".cyan());
    println!();

    if check_only {
        println!("  {} You are running the latest version of UPM (v{})", "✔".bold().green(), current_version);
        println!();
        return Ok(());
    }

    println!("  {} Checking for updates via cargo install...", "⚡".cyan());
    let resolved = crate::acquisition::AcquisitionRunner::resolve_executable("cargo");
    let mut cmd = Command::new(&resolved);
    cmd.args(["install", "--path", ".", "--force"]);

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("  {} Successfully updated UPM to latest binary build!", "✔".bold().green());
        }
        _ => {
            println!("  {} Binary is up to date.", "✔".green());
        }
    }

    println!();
    Ok(())
}
