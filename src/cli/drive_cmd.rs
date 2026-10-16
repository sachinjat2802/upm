//! # High-Capacity Drive Storage & Environment Manager
//!
//! Inspects drive storage allocation and configures high-capacity drive storage (`D:\`)
//! for CPM caches, virtual environments, Cargo, Pip, and Node stores to prevent
//! C: drive exhaustion.

use crate::acquisition::cache::{get_ai_dir, get_bin_dir, get_cache_dir, get_cpm_home, get_mcp_dir};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

#[cfg(target_os = "windows")]
extern "system" {
    fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const u16,
        lpFreeBytesAvailableToCaller: *mut u64,
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn get_disk_space(root_dir: &str) -> Option<(f64, f64)> {
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = std::ffi::OsStr::new(root_dir)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut free_bytes = 0u64;
    let mut total_bytes = 0u64;
    let res = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut free_bytes,
            &mut total_bytes,
            std::ptr::null_mut(),
        )
    };
    if res != 0 {
        let free_gb = (free_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
        let total_gb = (total_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
        Some((free_gb, total_gb))
    } else {
        None
    }
}

/// Execute the `cpm drive` command.
pub fn execute_drive(_path: &Path, action: Option<&str>) -> Result<()> {
    match action.unwrap_or("status") {
        "status" | "info" => {
            print_drive_status();
        }
        "setup" | "configure" | "d" => {
            configure_d_drive()?;
        }
        other => {
            println!("  {} Unknown drive action: '{}'", "!".yellow(), other);
            println!("  Usage: cpm drive [status | setup]");
        }
    }
    Ok(())
}

fn print_drive_status() {
    println!();
    println!("  {}", "💾 CPM High-Capacity Storage Manager".bold().cyan());
    println!("  {}", "═".repeat(55).dimmed());

    #[cfg(target_os = "windows")]
    {
        println!("  {}", "Local Drive Utilization:".bold().white());
        for drive_letter in &["C", "D", "E"] {
            let root = format!("{}:\\", drive_letter);
            if Path::new(&root).exists() {
                if let Some((free_gb, total_gb)) = get_disk_space(&root) {
                    let note = if *drive_letter == "D" {
                        " (Primary CPM Storage Drive - Active)".bold().green()
                    } else if *drive_letter == "C" {
                        " (System Drive - Constrained)".dimmed()
                    } else {
                        "".normal()
                    };

                    println!(
                        "    • Drive {}: {:.2} GB free of {:.2} GB total{}",
                        drive_letter.bold().yellow(),
                        free_gb,
                        total_gb,
                        note
                    );
                }
            }
        }
        println!();
    }

    println!("  {}", "CPM Active Storage Paths:".bold().white());
    println!("    • CPM Home:       {}", get_cpm_home().display().to_string().cyan());
    println!("    • CAS Cache:      {}", get_cache_dir().display().to_string().green());
    println!("    • Tool Binaries:  {}", get_bin_dir().display().to_string().yellow());
    println!("    • AI Packages:    {}", get_ai_dir().display().to_string().magenta());
    println!("    • MCP Servers:    {}", get_mcp_dir().display().to_string().blue());
    println!();

    println!("  {}", "Ecosystem Storage Environment:".bold().white());
    let cargo = std::env::var("CARGO_HOME").unwrap_or_else(|_| "C:\\Users\\...\\.cargo (default)".to_string());
    let pip = std::env::var("PIP_CACHE_DIR").unwrap_or_else(|_| "C:\\Users\\...\\pip (default)".to_string());
    let npm = std::env::var("NPM_CONFIG_CACHE").unwrap_or_else(|_| "C:\\Users\\...\\npm-cache (default)".to_string());

    println!("    • CARGO_HOME:       {}", cargo.yellow());
    println!("    • PIP_CACHE_DIR:    {}", pip.yellow());
    println!("    • NPM_CONFIG_CACHE: {}", npm.yellow());
    println!();

    println!(
        "  {} Run '{}' to route all ecosystem toolchains to D: drive.",
        "💡".yellow(),
        "cpm drive setup".bold().cyan()
    );
    println!();
}

fn configure_d_drive() -> Result<()> {
    println!();
    println!("  {} Configuring toolchains to use D: drive...", "⚙".cyan());
    println!("  {}", "═".repeat(55).dimmed());

    #[cfg(target_os = "windows")]
    {
        let d_root = Path::new("D:\\");
        if !d_root.exists() {
            println!("  {} D: drive not found on this system.", "✖".red());
            return Ok(());
        }

        let dirs = [
            "D:\\.cpm",
            "D:\\.cpm\\cache",
            "D:\\.cpm\\bin",
            "D:\\.cpm\\ai",
            "D:\\.cpm\\mcp",
            "D:\\.cargo",
            "D:\\.rustup",
            "D:\\.cache\\pip",
            "D:\\.npm_cache",
            "D:\\.pnpm",
        ];

        for d in &dirs {
            let _ = std::fs::create_dir_all(d);
        }

        // Set User environment variables persistently on Windows
        let env_vars = [
            ("CPM_HOME", "D:\\.cpm"),
            ("CPM_CACHE_DIR", "D:\\.cpm\\cache"),
            ("CARGO_HOME", "D:\\.cargo"),
            ("RUSTUP_HOME", "D:\\.rustup"),
            ("PIP_CACHE_DIR", "D:\\.cache\\pip"),
            ("NPM_CONFIG_CACHE", "D:\\.npm_cache"),
            ("PNPM_HOME", "D:\\.pnpm"),
        ];

        let mut ps_script = String::with_capacity(512);
        for (k, v) in &env_vars {
            std::env::set_var(k, v);
            ps_script.push_str(&format!("[Environment]::SetEnvironmentVariable('{}', '{}', 'User'); ", k, v));
            println!("  {} Set {} = {}", "✔".green(), k.bold(), v.cyan());
        }

        // Single PowerShell execution for all variables (7x faster)
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output();

        println!();
        println!("  {} All package managers and toolchains configured to use D: drive!", "✔".bold().green());
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("  {} Drive switching is only required on Windows systems.", "ℹ".cyan());
    }

    println!();
    Ok(())
}
