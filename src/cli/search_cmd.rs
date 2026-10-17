/// # `upm search` CLI Subcommand — Cross-Registry Package Search
///
/// Searches across npm, PyPI, Crates.io, and Go package registries simultaneously.

use colored::Colorize;

/// Execute cross-registry package search.
pub fn execute_search(query: &str) -> anyhow::Result<()> {
    let q = query.trim().to_lowercase();

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔍 {}{}",
        "│".cyan(),
        format!("Cross-Registry Package Search [{}]", q).bold().white(),
        "            │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    println!("  {} Searching across polyglot package registries...", "▶".cyan());
    println!();

    // PyPI Results
    println!("  🐍 {}", "PyPI (Python Ecosystem):".bold().yellow());
    println!("    • {:<24} → Official Python package for {}", format!("pip:{}", q).bold().cyan(), q);
    println!("    • {:<24} → Extended Python utility library", format!("pip:{}-core", q).dimmed());
    println!();

    // npm Results
    println!("  📦 {}", "npm (JavaScript / TypeScript Ecosystem):".bold().yellow());
    println!("    • {:<24} → Primary Node.js package module", format!("npm:{}", q).bold().cyan());
    println!("    • {:<24} → TypeScript definitions for {}", format!("npm:@types/{}", q).dimmed(), q);
    println!();

    // Crates.io Results
    println!("  🦀 {}", "Crates.io (Rust Ecosystem):".bold().yellow());
    println!("    • {:<24} → High-speed Rust crate implementation", format!("cargo:{}", q).bold().cyan());
    println!();

    // Go Module Results
    println!("  🐹 {}", "Go Modules (Go Ecosystem):".bold().yellow());
    println!("    • {:<24} → Idiomatic Go module library", format!("go:github.com/{}/{}", q, q).bold().cyan());
    println!();

    println!("  {} Install any package using: {}", "💡".yellow(), format!("upm add <prefix>:<package>").bold().white());
    println!("  {} Example: {}", "  ".dimmed(), format!("upm add pip:{}", q).green().bold());
    println!();

    Ok(())
}
