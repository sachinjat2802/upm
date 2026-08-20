/// # Acquisition Runner
///
/// Executes native package manager commands for install, add, update,
/// outdated, and audit operations across detected ecosystems.
///
/// All methods accept a `dry_run` flag that logs commands without
/// executing them, an optional `filter` to restrict to specific ecosystems,
/// and `parallel` execution for concurrent installations.

use crate::acquisition::adapter::EcosystemAdapter;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

/// Orchestrates native package manager command execution.
pub struct AcquisitionRunner;

impl AcquisitionRunner {
    /// Filter adapters based on an optional filter string (matches language or adapter name).
    pub fn filter_adapters<'a>(adapters: &'a [EcosystemAdapter], filter: Option<&str>) -> Vec<&'a EcosystemAdapter> {
        if let Some(f) = filter {
            let f_lower = f.to_lowercase();
            adapters
                .iter()
                .filter(|a| a.name.to_lowercase().contains(&f_lower) || a.language.to_lowercase().contains(&f_lower))
                .collect()
        } else {
            adapters.iter().collect()
        }
    }

    /// Run the install command for every adapter in the list.
    pub fn run_install(
        path: &Path,
        adapters: &[EcosystemAdapter],
        dry_run: bool,
        parallel: bool,
        filter: Option<&str>,
    ) -> Result<()> {
        let filtered = Self::filter_adapters(adapters, filter);
        if filtered.is_empty() {
            println!("  {} No matching ecosystems found for filter: '{}'", "!".bold().yellow(), filter.unwrap_or(""));
            return Ok(());
        }

        println!(
            "  {} {}",
            "▶".bold().cyan(),
            if parallel {
                "Installing dependencies across ecosystems (PARALLEL)".bold()
            } else {
                "Installing dependencies across ecosystems".bold()
            }
        );
        println!();

        if parallel && !dry_run && filtered.len() > 1 {
            let path_buf = path.to_path_buf();
            let errors = Arc::new(Mutex::new(Vec::new()));
            let mut handles = Vec::new();

            for adapter in filtered {
                let adapter_clone = adapter.clone();
                let path_clone = path_buf.clone();
                let errors_clone = errors.clone();
                let icon = Self::lang_icon(&adapter.language);

                println!(
                    "  {} {} {} {}",
                    icon,
                    adapter.display_name.bold(),
                    "→".dimmed(),
                    adapter.install_cmd.join(" ").yellow()
                );

                let handle = thread::spawn(move || {
                    if let Err(e) = Self::execute_command(&path_clone, &adapter_clone.install_cmd) {
                        let mut errs = errors_clone.lock().unwrap();
                        errs.push((adapter_clone.display_name.clone(), e));
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.join();
            }

            let errs = errors.lock().unwrap();
            if !errs.is_empty() {
                for (name, err) in errs.iter() {
                    println!("  {} Failed installing {}: {}", "✖".bold().red(), name, err);
                }
            }
        } else {
            for adapter in filtered {
                let icon = Self::lang_icon(&adapter.language);
                println!(
                    "  {} {} {} {}",
                    icon,
                    adapter.display_name.bold(),
                    "→".dimmed(),
                    adapter.install_cmd.join(" ").yellow()
                );

                if !dry_run {
                    Self::execute_command(path, &adapter.install_cmd)?;
                } else {
                    println!("    {} dry run — skipped", "⊘".dimmed());
                }
            }
        }

        println!();
        println!("  {} {}", "✔".green().bold(), "All ecosystems installed.".bold());
        Ok(())
    }

    /// Run the add command for a single adapter.
    pub fn run_add(path: &Path, adapter: &EcosystemAdapter, package: &str, dry_run: bool) -> Result<()> {
        let mut cmd = adapter.add_cmd.clone();
        cmd.push(package.to_string());

        let icon = Self::lang_icon(&adapter.language);
        println!(
            "  {} Adding {} to [{}] via {}",
            icon,
            package.bold().yellow(),
            adapter.display_name.bold(),
            cmd.join(" ").magenta()
        );

        if !dry_run {
            Self::execute_command(path, &cmd)?;
        }
        Ok(())
    }

    /// Run the update command for every adapter.
    pub fn run_update(
        path: &Path,
        adapters: &[EcosystemAdapter],
        dry_run: bool,
        parallel: bool,
        filter: Option<&str>,
    ) -> Result<()> {
        let filtered = Self::filter_adapters(adapters, filter);
        if filtered.is_empty() {
            println!("  {} No matching ecosystems found for filter: '{}'", "!".bold().yellow(), filter.unwrap_or(""));
            return Ok(());
        }

        println!(
            "  {} {}",
            "▶".bold().cyan(),
            if parallel {
                "Updating dependencies across ecosystems (PARALLEL)".bold()
            } else {
                "Updating dependencies across ecosystems".bold()
            }
        );
        println!();

        if parallel && !dry_run && filtered.len() > 1 {
            let path_buf = path.to_path_buf();
            let mut handles = Vec::new();

            for adapter in filtered {
                let adapter_clone = adapter.clone();
                let path_clone = path_buf.clone();
                let icon = Self::lang_icon(&adapter.language);

                println!(
                    "  {} {} {} {}",
                    icon,
                    adapter.display_name.bold(),
                    "→".dimmed(),
                    adapter.update_cmd.join(" ").yellow()
                );

                let handle = thread::spawn(move || {
                    let _ = Self::execute_command(&path_clone, &adapter_clone.update_cmd);
                });
                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.join();
            }
        } else {
            for adapter in filtered {
                let icon = Self::lang_icon(&adapter.language);
                println!(
                    "  {} {} {} {}",
                    icon,
                    adapter.display_name.bold(),
                    "→".dimmed(),
                    adapter.update_cmd.join(" ").yellow()
                );

                if !dry_run {
                    Self::execute_command(path, &adapter.update_cmd)?;
                }
            }
        }
        Ok(())
    }

    /// Run the outdated report command for every adapter.
    pub fn run_outdated(path: &Path, adapters: &[EcosystemAdapter], filter: Option<&str>) -> Result<()> {
        let filtered = Self::filter_adapters(adapters, filter);
        println!(
            "  {} {}",
            "▶".bold().cyan(),
            "Checking for outdated dependencies".bold()
        );

        for adapter in filtered {
            println!();
            println!("  {} {}", "─".repeat(3).dimmed(), adapter.display_name.bold());
            let _ = Self::execute_command(path, &adapter.outdated_cmd);
        }
        Ok(())
    }

    /// Run the security audit command for every adapter.
    pub fn run_audit(path: &Path, adapters: &[EcosystemAdapter], filter: Option<&str>) -> Result<()> {
        let filtered = Self::filter_adapters(adapters, filter);
        println!(
            "  {} {}",
            "▶".bold().cyan(),
            "Running security audit across ecosystems".bold()
        );

        for adapter in filtered {
            println!();
            println!("  {} {}", "🛡️", adapter.display_name.bold());
            let _ = Self::execute_command(path, &adapter.audit_cmd);
        }
        Ok(())
    }

    /// Execute a shell command, printing results inline.
    fn execute_command(path: &Path, cmd_args: &[String]) -> Result<()> {
        if cmd_args.is_empty() {
            return Err(anyhow!("Empty command"));
        }

        let program = &cmd_args[0];
        let args = &cmd_args[1..];

        let mut command = Command::new(program);
        command.args(args).current_dir(path);

        match command.status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    println!("    {} exited with {}", "!".yellow(), status);
                    Ok(())
                }
            }
            Err(e) => {
                println!(
                    "    {} '{}' not found: {}",
                    "⚠".dimmed(),
                    program.dimmed(),
                    e.to_string().dimmed()
                );
                Ok(())
            }
        }
    }

    /// Get a display icon for a language.
    fn lang_icon(lang: &str) -> &'static str {
        match lang {
            "javascript" => "📦",
            "python" => "🐍",
            "rust" => "🦀",
            "go" => "🐹",
            "java" => "☕",
            "php" => "🐘",
            "ruby" => "💎",
            "csharp" => "🔷",
            "dart" => "🎯",
            "elixir" => "💧",
            _ => "📦",
        }
    }
}
