//! # UPM Cross-Language Script & Tool Runner
//!
//! Provides unified multi-language task execution (`upm run <script>`) and
//! local package binary execution (`upm exec <tool>`).

use crate::acquisition::manifest::UpmManifest;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

/// Run a defined script across any ecosystem or from `upm.toml`.
pub fn execute_run_script(path: &Path, script_name: Option<&str>, extra_args: &[String]) -> Result<()> {
    let manifest = UpmManifest::load_from_dir(path);
    let mut available_scripts = BTreeMap::new();

    // 1. Collect from upm.toml
    if let Some(ref m) = manifest {
        for (k, v) in &m.scripts {
            available_scripts.insert(k.clone(), (v.clone(), "upm.toml".to_string()));
        }
    }

    // 2. Collect from package.json
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        if let Ok(content) = std::fs::read_to_string(&pkg_json) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(scripts) = v.get("scripts").and_then(|s| s.as_object()) {
                    for (k, val) in scripts {
                        if let Some(cmd) = val.as_str() {
                            available_scripts
                                .entry(k.clone())
                                .or_insert_with(|| (cmd.to_string(), "package.json".to_string()));
                        }
                    }
                }
            }
        }
    }

    // 3. Collect from pyproject.toml
    let pyproject = path.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(content) = std::fs::read_to_string(&pyproject) {
            if let Ok(val) = toml::from_str::<toml::Value>(&content) {
                if let Some(scripts) = val
                    .get("tool")
                    .and_then(|t| t.get("poetry"))
                    .and_then(|p| p.get("scripts"))
                    .and_then(|s| s.as_table())
                {
                    for (k, v) in scripts {
                        if let Some(s) = v.as_str() {
                            available_scripts
                                .entry(k.clone())
                                .or_insert_with(|| (format!("python -m {}", s), "pyproject.toml".to_string()));
                        }
                    }
                }
            }
        }
    }

    // If no script name provided, list all scripts
    let target = match script_name {
        Some(s) if !s.is_empty() => s,
        _ => {
            println!();
            println!("  {}", "📜 Available Project Scripts (UPM Task Engine)".bold().cyan());
            println!("  {}", "═".repeat(60).dimmed());
            if available_scripts.is_empty() {
                println!("  No scripts defined in upm.toml, package.json, or pyproject.toml.");
                println!("  Add a [scripts] table to upm.toml:");
                println!("    [scripts]");
                println!("    dev = \"python app.py\"");
                println!("    build = \"cargo build --release\"");
            } else {
                for (name, (cmd, source)) in &available_scripts {
                    println!("  {} {} {} {}", "•".cyan(), name.bold().yellow(), "→".dimmed(), cmd);
                    println!("    {}", format!("(source: {})", source).dimmed());
                }
                println!();
                println!("  Run any script with: {}", "upm run <script>".bold().green());
            }
            println!();
            return Ok(());
        }
    };

    if let Some((command_str, source)) = available_scripts.get(target) {
        println!();
        println!(
            "  {} Running script '{}' from {}:",
            "▶".bold().cyan(),
            target.bold().yellow(),
            source.dimmed()
        );
        println!("  {} {}", "$".dimmed(), command_str.bold());
        println!();

        let mut full_cmd = command_str.clone();
        if !extra_args.is_empty() {
            full_cmd.push(' ');
            full_cmd.push_str(&extra_args.join(" "));
        }

        // Apply manifest environment variables if defined
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("powershell");
            c.arg("-NoProfile").arg("-ExecutionPolicy").arg("Bypass").arg("-Command").arg(&full_cmd);
            c
        } else {
            let mut c = Command::new("sh");
            c.arg("-c").arg(&full_cmd);
            c
        };

        cmd.current_dir(path);

        if let Some(ref m) = manifest {
            for (k, v) in &m.env {
                cmd.env(k, v);
            }
        }

        let status = cmd.status().map_err(|e| anyhow!("Failed to execute script: {}", e))?;
        if !status.success() {
            return Err(anyhow!("Script '{}' exited with status {}", target, status));
        }
        Ok(())
    } else {
        println!();
        println!("  {} Script '{}' not found in project.", "✖".bold().red(), target.bold());
        println!("  Run '{}' to list available scripts.", "upm run".cyan());
        println!();
        Err(anyhow!("Script not found: {}", target))
    }
}

/// Execute a local ecosystem binary tool (e.g. `upm exec tsc` or `upm exec pytest`).
pub fn execute_tool(path: &Path, tool_name: &str, args: &[String]) -> Result<()> {
    let mut search_paths = Vec::new();

    // Node bin
    search_paths.push(path.join("node_modules").join(".bin"));
    // Python venv bin
    search_paths.push(path.join(".venv").join("Scripts"));
    search_paths.push(path.join(".venv").join("bin"));
    search_paths.push(path.join("venv").join("Scripts"));
    search_paths.push(path.join("venv").join("bin"));
    // Rust target bin
    search_paths.push(path.join("target").join("debug"));
    search_paths.push(path.join("target").join("release"));
    // Composer bin
    search_paths.push(path.join("vendor").join("bin"));

    let mut executable = None;
    for dir in &search_paths {
        let candidate = dir.join(tool_name);
        let candidate_exe = dir.join(format!("{}.exe", tool_name));
        let candidate_cmd = dir.join(format!("{}.cmd", tool_name));
        let candidate_ps1 = dir.join(format!("{}.ps1", tool_name));

        if candidate_exe.exists() {
            executable = Some(candidate_exe);
            break;
        } else if candidate_cmd.exists() {
            executable = Some(candidate_cmd);
            break;
        } else if candidate_ps1.exists() {
            executable = Some(candidate_ps1);
            break;
        } else if candidate.exists() {
            executable = Some(candidate);
            break;
        }
    }

    let exe_path = executable.unwrap_or_else(|| std::path::PathBuf::from(tool_name));
    println!("  {} Executing tool '{}'...", "▶".cyan(), exe_path.display());

    let mut cmd = if cfg!(target_os = "windows")
        && exe_path.extension().and_then(|s| s.to_str()).map(|ext| ext.eq_ignore_ascii_case("ps1")).unwrap_or(false)
    {
        let mut c = Command::new("powershell");
        c.arg("-NoProfile").arg("-ExecutionPolicy").arg("Bypass").arg("-File").arg(&exe_path);
        c.args(args);
        c
    } else {
        let mut c = Command::new(&exe_path);
        c.args(args);
        c
    };
    cmd.current_dir(path);

    let status = cmd.status().map_err(|e| anyhow!("Failed to execute tool '{}': {}", tool_name, e))?;
    if !status.success() {
        return Err(anyhow!("Tool '{}' exited with status {}", tool_name, status));
    }
    Ok(())
}
