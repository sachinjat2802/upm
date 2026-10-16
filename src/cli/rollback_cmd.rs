/// # `cpm rollback` CLI Subcommand — Instant Workspace Rollback Engine
///
/// Restores previous snapshot state of `upm.toml` manifest and lockfiles
/// in case of breaking dependency updates or acquisition failures.

use colored::Colorize;
use std::io::{self, Write};
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

    // Dynamic scan of all backup files (#84)
    let backup_entries = match std::fs::read_dir(&backup_dir) {
        Ok(entries) => entries.flatten().collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    if backup_entries.is_empty() {
        println!("  {} Backup directory .cpm_backup is empty.", "ℹ".blue());
        println!();
        return Ok(());
    }

    // Confirmation prompt (#11)
    print!("  {} Are you sure you want to rollback workspace files to previous snapshot? [y/N]: ", "?".bold().yellow());
    io::stdout().flush().unwrap_or(());

    let mut answer = String::new();
    if io::stdin().read_line(&mut answer).unwrap_or(0) > 0 {
        let trimmed = answer.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("  {} Rollback cancelled.", "ℹ".blue());
            println!();
            return Ok(());
        }
    }

    println!("  ▶ Restoring workspace state from restore point snapshot...");
    let mut restored = 0;

    for entry in backup_entries {
        let entry_path = entry.path();
        if entry_path.is_file() {
            let file_name = entry.file_name();
            let target_file_name = file_name
                .to_string_lossy()
                .trim_end_matches(".bak")
                .to_string();

            let final_target = path.join(&target_file_name);

            if let Ok(_) = std::fs::copy(&entry_path, &final_target) {
                println!("    ✔ Restored {}", target_file_name.dimmed());
                restored += 1;
            }
        }
    }

    println!();
    println!("  {} Restored {} workspace manifest/lockfile asset(s) successfully!", "✔".bold().green(), restored);
    println!("  {} Workspace has been rolled back cleanly.", "✨".bold().yellow());
    println!();

    Ok(())
}
