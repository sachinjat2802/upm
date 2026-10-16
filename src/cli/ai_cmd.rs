//! # CPM AI Manager: Universal AI Configuration & Skills Manager
//!
//! Manages AI prompts, rules, skills, and subagents with universal cross-IDE auto-conversion
//! across Cursor, Claude, Windsurf, Cline, Warp, and Antigravity.

use crate::acquisition::cache::get_ai_dir;
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AiRuleManifest {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub target_ides: Vec<String>,
    pub rules: Vec<String>,
    pub skills: BTreeMap<String, String>,
}

/// Execute the `cpm ai` command.
pub fn execute_ai(path: &Path, action: Option<&str>, target_ide: Option<&str>, rule_arg: Option<&str>) -> Result<()> {
    match action.unwrap_or("status") {
        "status" | "list" => {
            list_ai_configs(path)?;
        }
        "sync" => {
            sync_ai_rules(path)?;
        }
        "export" => {
            export_ai_rules(path, target_ide.unwrap_or("all"))?;
        }
        "add" | "install" => {
            if let Some(rule) = rule_arg {
                add_ai_rule(path, rule)?;
            } else {
                println!("  {} Specify rule or skill to add: cpm ai add <name>", "!".yellow());
            }
        }
        other => {
            println!("  {} Unknown AI command: '{}'", "!".yellow(), other);
            println!("  Usage: cpm ai [status | list | sync | export | add]");
        }
    }
    Ok(())
}

fn list_ai_configs(path: &Path) -> Result<()> {
    println!();
    println!("  {}", "🤖 CPM Universal AI Manager".bold().cyan());
    println!("  {}", "═".repeat(55).dimmed());

    let mut found_configs = Vec::new();

    let targets = [
        ("Cursor Rules", path.join(".cursorrules")),
        ("Cursor MDC Rules", path.join(".cursor").join("rules")),
        ("Claude Guidelines", path.join("CLAUDE.md")),
        ("Windsurf Rules", path.join(".windsurfrules")),
        ("Cline Rules", path.join(".clinerules")),
        ("Antigravity Rules", path.join(".agent").join("rules")),
        ("Antigravity Skills", path.join(".agents").join("skills")),
    ];

    for (label, p) in &targets {
        if p.exists() {
            found_configs.push((*label, p.clone()));
            println!("  {} {} ({})", "✔".green(), label.bold(), p.display().to_string().cyan());
        }
    }

    if found_configs.is_empty() {
        println!("  {} No AI configuration files detected in workspace.", "ℹ".dimmed());
        println!("  Run '{}' to initialize universal rules.", "cpm ai sync".bold().cyan());
    }

    let global_store = get_ai_dir();
    println!();
    println!("  Global AI Store: {}", global_store.display().to_string().yellow());
    println!();
    Ok(())
}

fn sync_ai_rules(path: &Path) -> Result<()> {
    println!();
    println!("  {} Synchronizing universal AI rules across all IDE formats...", "🔄".cyan());
    println!("  {}", "═".repeat(60).dimmed());

    // 1. Discover any existing rule content
    let mut rule_content = String::new();
    let sources = [
        path.join(".cursorrules"),
        path.join("CLAUDE.md"),
        path.join(".windsurfrules"),
        path.join(".clinerules"),
    ];

    for s in &sources {
        if s.exists() {
            if let Ok(c) = fs::read_to_string(s) {
                if !c.trim().is_empty() {
                    rule_content = c;
                    println!("  {} Sourced master rules from: {}", "✔".green(), s.display());
                    break;
                }
            }
        }
    }

    if rule_content.trim().is_empty() {
        rule_content = format!(
            "# AI Coding Assistant Guidelines\n\n\
             - Project managed by CPM / UPM universal package platform.\n\
             - Storage: High-capacity D: drive utilized for cache and builds.\n\
             - Follow language best practices, modularity, and reproducible dependency specs.\n"
        );
    }

    // 2. Synchronize across all IDE file formats
    let targets: Vec<(&str, PathBuf)> = vec![
        ("Cursor", path.join(".cursorrules")),
        ("Claude", path.join("CLAUDE.md")),
        ("Windsurf", path.join(".windsurfrules")),
        ("Cline / Roo", path.join(".clinerules")),
    ];

    for (ide, file_path) in targets {
        fs::write(&file_path, &rule_content)?;
        println!("  {} Synchronized {} rule -> {}", "✔".green(), ide.bold(), file_path.display());
    }

    // Antigravity rules
    let ag_rules_dir = path.join(".agent").join("rules");
    let _ = fs::create_dir_all(&ag_rules_dir);
    fs::write(ag_rules_dir.join("cpm_rules.md"), &rule_content)?;
    println!("  {} Synchronized Antigravity rule -> {}", "✔".green(), ag_rules_dir.display());

    println!();
    println!("  {} Cross-IDE AI configurations synchronized successfully!", "✔".bold().green());
    println!();
    Ok(())
}

fn export_ai_rules(path: &Path, target: &str) -> Result<()> {
    println!();
    println!("  {} Exporting AI rules for target: {}...", "📤".cyan(), target.bold().yellow());
    sync_ai_rules(path)?;
    Ok(())
}

fn add_ai_rule(path: &Path, rule_name: &str) -> Result<()> {
    println!();
    println!("  {} Adding AI rule '{}' to workspace...", "➕".cyan(), rule_name.bold().yellow());

    let claude_md = path.join("CLAUDE.md");
    let mut current = if claude_md.exists() {
        fs::read_to_string(&claude_md).unwrap_or_default()
    } else {
        String::new()
    };

    current.push_str(&format!("\n## Rule: {}\n- Enforce {} standards across codebase.\n", rule_name, rule_name));
    fs::write(&claude_md, &current)?;

    println!("  {} Added rule to CLAUDE.md. Running sync...", "✔".green());
    sync_ai_rules(path)?;
    Ok(())
}
