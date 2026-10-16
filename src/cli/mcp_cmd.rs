//! # mcpman Engine: Universal Model Context Protocol (MCP) Manager
//!
//! Discovers, configures, and synchronizes MCP servers across Claude Desktop,
//! Cursor, Antigravity IDE, VS Code, and Zed.

use crate::acquisition::cache::get_mcp_dir;
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct McpConfig {
    #[serde(rename = "mcpServers", default)]
    pub mcp_servers: BTreeMap<String, McpServerEntry>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct McpServerEntry {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

/// Execute the `cpm mcp` command.
pub fn execute_mcp(
    path: &Path,
    action: Option<&str>,
    name: Option<&str>,
    cmd: Option<&str>,
    args: &[String],
) -> Result<()> {
    match action.unwrap_or("list") {
        "list" | "ls" | "status" => {
            list_mcp_servers(path)?;
        }
        "add" | "install" => {
            if let (Some(n), Some(c)) = (name, cmd) {
                add_mcp_server(path, n, c, args)?;
            } else {
                println!("  {} Specify server name and command: cpm mcp add <name> --cmd <command>", "!".yellow());
            }
        }
        "remove" | "rm" => {
            if let Some(n) = name {
                remove_mcp_server(path, n)?;
            } else {
                println!("  {} Specify server name to remove: cpm mcp remove <name>", "!".yellow());
            }
        }
        "sync" => {
            sync_mcp_configs(path)?;
        }
        other => {
            println!("  {} Unknown MCP command: '{}'", "!".yellow(), other);
            println!("  Usage: cpm mcp [list | add | remove | sync]");
        }
    }
    Ok(())
}

fn list_mcp_servers(path: &Path) -> Result<()> {
    println!();
    println!("  {}", "🔌 CPM Universal MCP Manager (mcpman Engine)".bold().cyan());
    println!("  {}", "═".repeat(60).dimmed());

    let clients = get_known_mcp_locations(path);
    let mut total_servers = 0;

    for (client_name, config_path) in &clients {
        print!("  {} {}: ", "•".cyan(), client_name.bold());
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(cfg) = serde_json::from_str::<McpConfig>(&content) {
                    println!("{} servers registered ({})", cfg.mcp_servers.len().to_string().green(), config_path.display().to_string().dimmed());
                    for (srv_name, entry) in &cfg.mcp_servers {
                        println!(
                            "      {} {} [{}] args: [{}]",
                            "›".green(),
                            srv_name.bold(),
                            entry.command.yellow(),
                            entry.args.join(", ").dimmed()
                        );
                        total_servers += 1;
                    }
                    continue;
                }
            }
            println!("{}", "Config present, 0 servers".dimmed());
        } else {
            println!("{}", "Not installed / Config not found".dimmed());
        }
    }

    let global_store = get_mcp_dir();
    println!();
    println!("  Global MCP Store: {}", global_store.display().to_string().cyan());
    println!("  Total Detected Servers: {}", total_servers.to_string().green().bold());
    println!();
    Ok(())
}

fn add_mcp_server(path: &Path, name: &str, cmd: &str, args: &[String]) -> Result<()> {
    println!();
    println!("  {} Registering MCP server '{}'...", "➕".cyan(), name.bold().yellow());

    let workspace_mcp = path.join("mcp.json");
    let mut config: McpConfig = if workspace_mcp.exists() {
        let content = fs::read_to_string(&workspace_mcp)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        McpConfig::default()
    };

    config.mcp_servers.insert(
        name.to_string(),
        McpServerEntry {
            command: cmd.to_string(),
            args: args.to_vec(),
            env: BTreeMap::new(),
        },
    );

    let serialized = serde_json::to_string_pretty(&config)?;
    fs::write(&workspace_mcp, serialized)?;
    println!("  {} Saved to {}", "✔".green(), workspace_mcp.display());

    sync_mcp_configs(path)?;
    Ok(())
}

fn remove_mcp_server(path: &Path, name: &str) -> Result<()> {
    println!();
    println!("  {} Removing MCP server '{}'...", "➖".yellow(), name.bold());

    let workspace_mcp = path.join("mcp.json");
    if workspace_mcp.exists() {
        let content = fs::read_to_string(&workspace_mcp)?;
        if let Ok(mut config) = serde_json::from_str::<McpConfig>(&content) {
            if config.mcp_servers.remove(name).is_some() {
                let serialized = serde_json::to_string_pretty(&config)?;
                fs::write(&workspace_mcp, serialized)?;
                println!("  {} Removed from {}", "✔".green(), workspace_mcp.display());
                sync_mcp_configs(path)?;
                return Ok(());
            }
        }
    }

    println!("  {} Server '{}' not found in workspace mcp.json", "!".yellow(), name);
    Ok(())
}

fn sync_mcp_configs(path: &Path) -> Result<()> {
    println!();
    println!("  {} Synchronizing MCP server configurations across all clients...", "🔄".cyan());
    println!("  {}", "═".repeat(65).dimmed());

    let workspace_mcp = path.join("mcp.json");
    let master_config: McpConfig = if workspace_mcp.exists() {
        let content = fs::read_to_string(&workspace_mcp)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        McpConfig::default()
    };

    let clients = get_known_mcp_locations(path);
    let serialized = serde_json::to_string_pretty(&master_config)?;

    for (client_name, config_path) in clients {
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::write(&config_path, &serialized).is_ok() {
            println!("  {} Synced to {} ({})", "✔".green(), client_name.bold(), config_path.display());
        }
    }

    println!();
    println!("  {} MCP configurations synchronized successfully!", "✔".bold().green());
    println!();
    Ok(())
}

fn get_known_mcp_locations(workspace_root: &Path) -> Vec<(&'static str, PathBuf)> {
    let mut locs = Vec::new();

    // Workspace mcp.json
    locs.push(("Workspace (mcp.json)", workspace_root.join("mcp.json")));

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let claude = PathBuf::from(&appdata).join("Claude").join("claude_desktop_config.json");
            locs.push(("Claude Desktop", claude));

            let zed = PathBuf::from(&appdata).join("Zed").join("settings.json");
            locs.push(("Zed", zed));
        }

        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let cursor = PathBuf::from(&userprofile).join(".cursor").join("mcp.json");
            locs.push(("Cursor", cursor));

            let antigravity = PathBuf::from(&userprofile).join(".gemini").join("antigravity-ide").join("mcp_config.json");
            locs.push(("Antigravity IDE", antigravity));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(home) = std::env::var("HOME") {
            let claude = PathBuf::from(&home).join(".config").join("Claude").join("claude_desktop_config.json");
            locs.push(("Claude Desktop", claude));
            let cursor = PathBuf::from(&home).join(".cursor").join("mcp.json");
            locs.push(("Cursor", cursor));
        }
    }

    locs
}
