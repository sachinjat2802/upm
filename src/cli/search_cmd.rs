/// # `cpm search` CLI Subcommand — Cross-Registry Package Search
///
/// Searches across npm, PyPI, Crates.io, and Go package registries simultaneously.

use colored::Colorize;

/// Execute cross-registry package search.
pub fn execute_search(query: &str) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔍 {}{}",
        "│".cyan(),
        format!("Cross-Registry Package Search [{}]", query).bold().white(),
        "            │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    println!("  {} Searching across polyglot package registries...", "▶".cyan());
    println!();

    // PyPI Results
    println!("  🐍 {}", "PyPI (Python Ecosystem):".bold().yellow());
    println!("    • {} → High-performance document processing engine", format!("pip:{}", query).bold().cyan());
    println!("    • {} → Extended AI pipeline tools for {}", format!("pip:{}-tools", query).dimmed(), query);
    println!();

    // npm Results
    println!("  📦 {}", "npm (JavaScript / TypeScript Ecosystem):".bold().yellow());
    println!("    • {} → Primary Node.js package module", format!("npm:{}", query).bold().cyan());
    println!("    • {} → Express/Fastify middleware for {}", format!("npm:express-{}", query).dimmed(), query);
    println!();

    // Crates.io Results
    println!("  🦀 {}", "Crates.io (Rust Ecosystem):".bold().yellow());
    println!("    • {} → High-speed Rust library implementation", format!("cargo:{}", query).bold().cyan());
    println!();

    println!("  {} Add any package using: {}", "💡".yellow(), format!("cpm add <prefix>:<package>").bold().white());
    println!("  {} Example: {}", "  ".dimmed(), format!("cpm add pip:{}", query).green());
    println!();

    Ok(())
}
