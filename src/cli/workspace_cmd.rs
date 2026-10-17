//! # Universal Workspace & Monorepo Manager (`upm workspace`)
//!
//! Provides enterprise-grade polyglot monorepo orchestration across
//! Rust, JavaScript/TypeScript, Python, and Go packages.
//!
//! Time Complexity: O(P) where P = workspace package count.
//! Space Complexity: O(P) package manifest registry.

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: PathBuf,
    pub ecosystem: &'static str,
    pub version: String,
}

pub fn execute_workspace(
    path: &Path,
    action: Option<&str>,
    cmd_or_arg: Option<&str>,
    extra_args: &[String],
) -> Result<()> {
    let act = action.unwrap_or("list");

    match act {
        "list" | "ls" => list_workspace_packages(path),
        "run" => {
            if let Some(cmd) = cmd_or_arg {
                run_workspace_command(path, cmd, extra_args)
            } else {
                Err(anyhow!("Missing command: upm workspace run <command>"))
            }
        }
        "exec" => {
            if let Some(tool) = cmd_or_arg {
                exec_workspace_tool(path, tool, extra_args)
            } else {
                Err(anyhow!("Missing tool name: upm workspace exec <tool>"))
            }
        }
        other => {
            println!("  {} Unknown workspace action: '{}'", "!".yellow(), other);
            println!("  Usage: upm workspace [list | run <script> | exec <tool>]");
            Ok(())
        }
    }
}

/// Discover all sub-packages in the workspace directory tree in a single pass.
/// Time Complexity: O(D) directory traversal.
pub fn discover_packages(root: &Path) -> Result<Vec<WorkspacePackage>> {
    let mut packages = Vec::new();
    let search_roots = ["packages", "apps", "services", "libs", "modules", "."];

    for sr in &search_roots {
        let dir = root.join(sr);
        if !dir.is_dir() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist" {
                        continue;
                    }

                    if p.join("package.json").exists() {
                        let pkg_name = read_json_name(&p.join("package.json")).unwrap_or_else(|| name.to_string());
                        packages.push(WorkspacePackage {
                            name: pkg_name,
                            path: p.clone(),
                            ecosystem: "node",
                            version: "0.1.0".to_string(),
                        });
                    } else if p.join("Cargo.toml").exists() {
                        packages.push(WorkspacePackage {
                            name: name.to_string(),
                            path: p.clone(),
                            ecosystem: "rust",
                            version: "0.1.0".to_string(),
                        });
                    } else if p.join("pyproject.toml").exists() {
                        packages.push(WorkspacePackage {
                            name: name.to_string(),
                            path: p.clone(),
                            ecosystem: "python",
                            version: "0.1.0".to_string(),
                        });
                    } else if p.join("go.mod").exists() {
                        packages.push(WorkspacePackage {
                            name: name.to_string(),
                            path: p.clone(),
                            ecosystem: "go",
                            version: "0.1.0".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Sort and deduplicate in case paths were visited from multiple roots
    packages.sort_by(|a, b| a.path.cmp(&b.path));
    packages.dedup_by(|a, b| a.path == b.path);
    Ok(packages)
}

fn list_workspace_packages(root: &Path) -> Result<()> {
    let packages = discover_packages(root)?;

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🏢 {}", "│".cyan(), "Universal Monorepo Workspace Packages".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    if packages.is_empty() {
        println!("  {} No sub-packages found in packages/, apps/, or services/.", "ℹ".cyan());
        println!("  Organize your multi-language code into: packages/<pkg-name>/");
    } else {
        println!("  Found {} packages in workspace:", packages.len().to_string().cyan().bold());
        println!();
        for pkg in &packages {
            let rel = pkg.path.strip_prefix(root).unwrap_or(&pkg.path);
            let icon = match pkg.ecosystem {
                "node" => "📦",
                "rust" => "🦀",
                "python" => "🐍",
                "go" => "🐹",
                _ => "📁",
            };
            println!(
                "  {} {:<22} [{}] {}",
                icon,
                pkg.name.bold().yellow(),
                pkg.ecosystem.cyan(),
                rel.display().to_string().dimmed()
            );
        }
        println!();
        println!("  Run across all packages: {}", "upm workspace run <script>".bold().green());
    }
    println!();
    Ok(())
}

fn run_workspace_command(root: &Path, script: &str, extra_args: &[String]) -> Result<()> {
    let packages = discover_packages(root)?;
    if packages.is_empty() {
        println!("  No workspace packages discovered.");
        return Ok(());
    }

    println!();
    println!("  {} Running script '{}' across {} packages...", "▶".cyan(), script.bold().yellow(), packages.len());
    println!();

    for pkg in &packages {
        println!("  {} Entering {}", "↳".cyan(), pkg.name.bold());
        let _ = crate::cli::run_cmd::execute_run_script(&pkg.path, Some(script), extra_args);
    }

    println!();
    println!("  {} Monorepo task execution completed.", "✔".bold().green());
    println!();
    Ok(())
}

fn exec_workspace_tool(root: &Path, tool: &str, extra_args: &[String]) -> Result<()> {
    let packages = discover_packages(root)?;
    for pkg in &packages {
        println!("  {} Executing {} in {}", "▶".cyan(), tool.yellow(), pkg.name.bold());
        let _ = crate::cli::run_cmd::execute_tool(&pkg.path, tool, extra_args);
    }
    Ok(())
}

fn read_json_name(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&content).ok()?;
    val.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())
}
