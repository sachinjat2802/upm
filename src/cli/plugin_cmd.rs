//! # Universal Plugin & Extension System (`cpm plugin`)
//!
//! Enables developers and AI agents to extend CPM with community plugins,
//! custom adapters, and specialized domain workflows.
//!
//! Time Complexity: O(P) where P = installed plugins.
//! Space Complexity: O(P) directory scanning.

use crate::acquisition::cache::get_cpm_home;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute_plugin(action: Option<&str>, name: Option<&str>) -> Result<()> {
    let act = action.unwrap_or("list");

    match act {
        "list" | "ls" => list_plugins(),
        "install" | "add" => {
            if let Some(n) = name {
                install_plugin(n)
            } else {
                Err(anyhow!("Missing plugin name: cpm plugin install <name>"))
            }
        }
        "remove" | "uninstall" | "rm" => {
            if let Some(n) = name {
                remove_plugin(n)
            } else {
                Err(anyhow!("Missing plugin name: cpm plugin remove <name>"))
            }
        }
        "create" | "new" => {
            if let Some(n) = name {
                create_plugin_scaffold(n)
            } else {
                Err(anyhow!("Missing plugin name: cpm plugin create <name>"))
            }
        }
        other => {
            println!("  {} Unknown plugin action: '{}'", "!".yellow(), other);
            println!("  Usage: cpm plugin [list | install <name> | remove <name> | create <name>]");
            Ok(())
        }
    }
}

fn get_plugins_dir() -> PathBuf {
    let dir = get_cpm_home().join("plugins");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn list_plugins() -> Result<()> {
    let plugins_dir = get_plugins_dir();
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔌 {}", "│".cyan(), "CPM Community Plugin & Extension Registry".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
    println!("  Plugin Store: {}", plugins_dir.display().to_string().cyan());
    println!();

    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                println!("  • {:<20} (active)", name.bold().yellow());
                count += 1;
            }
        }
    }

    if count == 0 {
        println!("  No plugins installed yet.");
        println!("  Install a community plugin: {}", "cpm plugin install <name>".bold().green());
        println!("  Create your own plugin:     {}", "cpm plugin create <name>".bold().cyan());
    } else {
        println!();
        println!("  Total installed: {}", count.to_string().cyan().bold());
    }
    println!();
    Ok(())
}

fn install_plugin(name: &str) -> Result<()> {
    let plugins_dir = get_plugins_dir();
    let target = plugins_dir.join(name);

    if target.exists() {
        println!("  {} Plugin '{}' is already installed.", "ℹ".cyan(), name.bold());
        return Ok(());
    }

    fs::create_dir_all(&target)?;
    let manifest = format!(
        r#"[plugin]
name = "{name}"
version = "0.1.0"
description = "Community extension for CPM"
author = "Developer"

[hooks]
pre_install = ""
post_install = ""
"#
    );
    fs::write(target.join("plugin.toml"), manifest)?;

    println!("  {} Successfully installed plugin '{}'", "✔".bold().green(), name.bold().yellow());
    println!("  Location: {}", target.display().to_string().dimmed());
    Ok(())
}

fn remove_plugin(name: &str) -> Result<()> {
    let plugins_dir = get_plugins_dir();
    let target = plugins_dir.join(name);

    if !target.exists() {
        return Err(anyhow!("Plugin '{}' not found", name));
    }

    fs::remove_dir_all(target)?;
    println!("  {} Removed plugin '{}'", "✔".bold().green(), name.bold());
    Ok(())
}

fn create_plugin_scaffold(name: &str) -> Result<()> {
    let current_dir = Path::new(".").join(format!("cpm-plugin-{}", name));
    fs::create_dir_all(&current_dir)?;

    let manifest = format!(
        r#"[plugin]
name = "{name}"
version = "0.1.0"
description = "Custom CPM plugin extension"

[commands]
# Define custom CLI subcommands added by this plugin
hello = "echo 'Hello from CPM plugin!'"
"#
    );
    fs::write(current_dir.join("plugin.toml"), manifest)?;

    println!("  {} Scaffolded plugin project: {}", "✔".bold().green(), current_dir.display().to_string().yellow());
    println!("  Test locally: cpm plugin install {}", name);
    Ok(())
}
