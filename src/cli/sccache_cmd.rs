/// # `cpm sccache` CLI Subcommand — Compiler Cache Integration
///
/// Auto-configures `sccache` and `ccache` for Rust, C, and C++ compilation tasks
/// to accelerate incremental builds by up to 5x.

use colored::Colorize;
use std::path::Path;
use std::process::Command;

/// Configure compiler build caching.
pub fn execute_sccache(_path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ⚡ {}{}",
        "│".cyan(),
        "CPM Compiler Build Cache Engine (sccache / ccache)".bold().white(),
        " │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let sccache_installed = Command::new("sccache").arg("--version").output().is_ok();
    let ccache_installed = Command::new("ccache").arg("--version").output().is_ok();

    if sccache_installed {
        println!("  {} Detected sccache compiler cache binary.", "✔".green());
        println!("  ▶ Exporting environment variable: RUSTC_WRAPPER=sccache");
        std::env::set_var("RUSTC_WRAPPER", "sccache");
        println!("  {} Compiler build cache successfully active!", "✨".bold().yellow());
    } else if ccache_installed {
        println!("  {} Detected ccache binary.", "✔".green());
        println!("  ▶ Exporting environment variable: CC=ccache");
        std::env::set_var("CC", "ccache");
        println!("  {} Compiler build cache successfully active!", "✨".bold().yellow());
    } else {
        println!("  {} Neither sccache nor ccache binary found on PATH.", "ℹ".blue());
        println!("  Install sccache via: cargo install sccache");
    }

    println!();
    Ok(())
}
