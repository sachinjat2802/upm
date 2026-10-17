//! # Universal Package Inspector (`cpm info`)
//!
//! Inspects package specifications, metadata, versions, and registry links
//! across any language ecosystem (CPM Universal Package Inspector).

use anyhow::Result;
use colored::Colorize;

/// Display metadata for a package across ecosystems.
pub fn execute_info(package: &str) -> Result<()> {
    let (eco, raw_pkg) = if let Some((e, p)) = package.split_once(':') {
        (e.to_lowercase(), p.to_string())
    } else {
        ("generic".to_string(), package.to_string())
    };

    let (pkg_name, requested_ver) = if let Some((n, v)) = raw_pkg.split_once('@') {
        (n.to_string(), Some(v.to_string()))
    } else if let Some((n, v)) = raw_pkg.split_once("==") {
        (n.to_string(), Some(v.to_string()))
    } else {
        (raw_pkg, None)
    };

    println!();
    println!("  {} Package Information: {}:{}", "📦".cyan(), eco.cyan().bold(), pkg_name.bold().yellow());
    println!("  {}", "═".repeat(50).dimmed());

    if let Some(ref ver) = requested_ver {
        println!("  Requested Ver:  {}", ver.bold().cyan());
    }

    match eco.as_str() {
        "npm" | "pnpm" | "yarn" | "bun" => {
            println!("  Ecosystem:      JavaScript / TypeScript (npm)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://www.npmjs.com/package/{}", pkg_name);
            println!("  Install via:    cpm add npm:{}", if let Some(v) = &requested_ver { format!("{}@{}", pkg_name, v) } else { pkg_name.clone() });
            println!("  Usage Format:   const {} = require('{}');", pkg_name.replace('-', "_"), pkg_name);
        }
        "pip" | "uv" | "poetry" => {
            println!("  Ecosystem:      Python (PyPI)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://pypi.org/project/{}/", pkg_name);
            println!("  Install via:    cpm add pip:{}", if let Some(v) = &requested_ver { format!("{}=={}", pkg_name, v) } else { pkg_name.clone() });
            println!("  Usage Format:   import {}", pkg_name.replace('-', "_"));
        }
        "cargo" | "rust" => {
            println!("  Ecosystem:      Rust (crates.io)");
            println!("  Crate Name:     {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://crates.io/crates/{}", pkg_name);
            println!("  Install via:    cpm add cargo:{}", if let Some(v) = &requested_ver { format!("{}@{}", pkg_name, v) } else { pkg_name.clone() });
            println!("  Usage Format:   use {}::*;", pkg_name.replace('-', "_"));
        }
        "go" => {
            println!("  Ecosystem:      Go (pkg.go.dev)");
            println!("  Module Name:    {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://pkg.go.dev/{}", pkg_name);
            println!("  Install via:    cpm add go:{}", pkg_name);
        }
        "maven" | "gradle" => {
            println!("  Ecosystem:      Java / JVM (Maven Central)");
            println!("  Artifact Name:  {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://central.sonatype.com/artifact/{}", pkg_name);
            println!("  Install via:    cpm add maven:{}", pkg_name);
        }
        "composer" | "php" => {
            println!("  Ecosystem:      PHP (Packagist)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://packagist.org/packages/{}", pkg_name);
            println!("  Install via:    cpm add composer:{}", pkg_name);
        }
        "nuget" | "csharp" | "dotnet" => {
            println!("  Ecosystem:      .NET / C# (NuGet)");
            println!("  Package Name:   {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://www.nuget.org/packages/{}", pkg_name);
            println!("  Install via:    cpm add nuget:{}", pkg_name);
        }
        "rubygems" | "gem" | "ruby" => {
            println!("  Ecosystem:      Ruby (RubyGems)");
            println!("  Gem Name:       {}", pkg_name.yellow().bold());
            println!("  Registry URL:   https://rubygems.org/gems/{}", pkg_name);
            println!("  Install via:    cpm add gem:{}", pkg_name);
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
