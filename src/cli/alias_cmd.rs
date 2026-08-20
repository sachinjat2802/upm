/// # `cpm alias` CLI Subcommand — Alias Management
///
/// Enables developers to create, list, and delete custom CLI aliases for
/// frequent polyglot commands.

use colored::Colorize;
use std::collections::BTreeMap;
use std::path::Path;

/// Manage custom CLI aliases.
pub fn execute_alias(path: &Path, action: Option<&str>, name: Option<&str>, command: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🏷️ {}{}",
        "│".cyan(),
        "CPM Custom Alias Management Engine".bold().white(),
        "             │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let alias_file = path.join(".cpm_aliases.json");
    let mut aliases: BTreeMap<String, String> = if alias_file.exists() {
        let content = std::fs::read_to_string(&alias_file)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        let mut default_map = BTreeMap::new();
        default_map.insert("py-sqrt".into(), "bridge call python:math.sqrt".into());
        default_map.insert("node-sha".into(), "bridge call node:crypto.sha256".into());
        default_map.insert("py-parse".into(), "bridge call python:docling.parse".into());
        default_map
    };

    match action.unwrap_or("list") {
        "add" | "set" => {
            if let (Some(a_name), Some(a_cmd)) = (name, command) {
                aliases.insert(a_name.to_string(), a_cmd.to_string());
                std::fs::write(&alias_file, serde_json::to_string_pretty(&aliases)?)?;
                println!("  {} Added alias '{}' → '{}'", "✔".green(), a_name.bold().yellow(), a_cmd.cyan());
            } else {
                println!("  {} Usage: cpm alias add <name> <command>", "✖".bold().red());
            }
        }
        "remove" | "rm" | "delete" => {
            if let Some(a_name) = name {
                if aliases.remove(a_name).is_some() {
                    std::fs::write(&alias_file, serde_json::to_string_pretty(&aliases)?)?;
                    println!("  {} Removed alias '{}'", "✔".green(), a_name.bold().yellow());
                } else {
                    println!("  {} Alias '{}' not found", "✖".bold().red(), a_name);
                }
            }
        }
        _ => {
            // List aliases
            println!("  {}", "Active Custom Aliases:".bold().yellow());
            if aliases.is_empty() {
                println!("    No custom aliases defined. Add one with: cpm alias add <name> <command>");
            } else {
                for (a_name, a_cmd) in &aliases {
                    println!("    {} {} → {}", "•".cyan(), a_name.bold().magenta(), a_cmd.dimmed());
                }
            }
        }
    }

    println!();
    Ok(())
}
