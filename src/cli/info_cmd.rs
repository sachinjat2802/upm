//! # Package Information Inspector
//!
//! Inspects package specifications, metadata, versions, and repository links
//! across any ecosystem (inspired by `upm info` and `npm view`).

use anyhow::Result;
use colored::Colorize;

/// Display metadata for a package across ecosystems.
pub fn execute_info(package: &str) -> Result<()> {
    let (eco, pkg_name) = if let Some((e, p)) = package.split_once(':') {
        (e.to_lowercase(), p.to_string())
    } else {
        ("generic".to_string(), package.to_string())
    };

    println!();
    println!("  {} Package Information: {}:{}", "📦".cyan(), eco.cyan().bold(), pkg_name.bold().yellow());
    println!("  {}", "═".repeat(50).dimmed());

    match eco.as_str() {
        "npm" | "pnpm" | "yarn" | "bun" => {
            println!("  Ecosystem:      JavaScript / TypeScript (npm)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://www.npmjs.com/package/{}", pkg_name);
            println!("  Install via:    cpm add npm:{}", pkg_name);
            println!("  Usage Format:   const {} = require('{}');", pkg_name.replace('-', "_"), pkg_name);
        }
        "pip" | "uv" | "poetry" => {
            println!("  Ecosystem:      Python (PyPI)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://pypi.org/project/{}/", pkg_name);
            println!("  Install via:    cpm add pip:{}", pkg_name);
            println!("  Usage Format:   import {}", pkg_name.replace('-', "_"));
        }
        "cargo" | "rust" => {
            println!("  Ecosystem:      Rust (crates.io)");
            println!("  Crate Name:     {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://crates.io/crates/{}", pkg_name);
            println!("  Install via:    cpm add cargo:{}", pkg_name);
            println!("  Usage Format:   use {}::*;", pkg_name.replace('-', "_"));
        }
        "go" => {
            println!("  Ecosystem:      Go (pkg.go.dev)");
            println!("  Module Name:    {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://pkg.go.dev/{}", pkg_name);
            println!("  Install via:    cpm add go:{}", pkg_name);
        }
        _ => {
            println!("  Ecosystem:      {}", eco.cyan());
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Install via:    cpm add {}:{}", eco, pkg_name);
        }
    }

    println!();
    Ok(())
}
