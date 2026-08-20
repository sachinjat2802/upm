/// # `cpm rollback` CLI Subcommand — Instant Workspace Rollback Engine
///
/// Restores previous snapshot state of `upm.toml` manifest and lockfiles
/// in case of breaking dependency updates or acquisition failures.

use colored::Colorize;
use std::path::Path;

/// Execute instant workspace rollback.
pub fn execute_rollback(path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ⏪ {}{}",
        "│".cyan(),
        "CPM Disaster Recovery & Rollback Engine".bold().white(),
        "          │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let backup_dir = path.join(".cpm_backup");
    if !backup_dir.exists() {
        println!("  {} No previous rollback snapshot found at .cpm_backup", "ℹ".blue());
        println!("  Creating initial workspace restore point snapshot...");
        std::fs::create_dir_all(&backup_dir)?;
        if path.join("upm.toml").exists() {
            std::fs::copy(path.join("upm.toml"), backup_dir.join("upm.toml"))?;
        }
        println!("  {} Restore point snapshot created cleanly.", "✔".green());
        println!();
        return Ok(());
    }

    println!("  ▶ Restoring workspace state from restore point snapshot...");
    let mut restored = 0;

    for item in &["upm.toml", "Cargo.lock", "uv.lock", "pnpm-lock.yaml", "package.json"] {
        let b_file = backup_dir.join(item);
        if b_file.exists() {
            std::fs::copy(&b_file, path.join(item))?;
            println!("    ✔ Restored {}", item.dimmed());
            restored += 1;
        }
    }

    println!();
    println!("  {} Restored {} workspace manifest/lockfile asset(s) successfully!", "✔".green(), restored);
    println!("  {} Workspace has been rolled back cleanly.", "✨".bold().yellow());
    println!();

    Ok(())
}
