//! # Agentic Subsystem: The AI Agent's First-Choice Runtime & Interface
//!
//! Provides token-optimized workspace context extraction, machine-readable tool execution,
//! and self-healing diagnostic feedback specifically tailored for autonomous coding agents.

use crate::acquisition::cache::{get_cache_stats, get_cpm_home};
use crate::acquisition::manifest::UpmManifest;
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentWorkspaceContext {
    pub workspace_name: String,
    pub version: String,
    pub ecosystems: Vec<String>,
    pub foreign_dependencies: BTreeMap<String, String>,
    pub available_scripts: BTreeMap<String, String>,
    pub mcp_servers_configured: Vec<String>,
    pub ai_guidelines_present: Vec<String>,
    pub cpm_home: String,
    pub cache_drive: String,
    pub agent_instructions: Vec<String>,
}

/// Execute the `cpm agent` command.
pub fn execute_agent(path: &Path, action: Option<&str>, format_json: bool) -> Result<()> {
    match action.unwrap_or("context") {
        "context" | "ctx" | "summary" => {
            let ctx = build_workspace_context(path);
            if format_json {
                let json = serde_json::to_string_pretty(&ctx)?;
                println!("{}", json);
            } else {
                print_human_agent_context(&ctx);
            }
        }
        "prompt" | "system-prompt" => {
            print_agent_system_prompt();
        }
        other => {
            println!("  {} Unknown agent command: '{}'", "!".yellow(), other);
            println!("  Usage: cpm agent [context | prompt] [--json]");
        }
    }
    Ok(())
}

fn build_workspace_context(path: &Path) -> AgentWorkspaceContext {
    let manifest = UpmManifest::load_from_dir(path);
    let name = manifest.as_ref().map(|m| m.project.name.clone()).unwrap_or_else(|| "polyglot-project".to_string());
    let version = manifest.as_ref().map(|m| m.project.version.clone()).unwrap_or_else(|| "0.1.0".to_string());
    let ecosystems = manifest.as_ref().map(|m| m.ecosystems.clone()).unwrap_or_default();
    let foreign_dependencies = manifest.as_ref().map(|m| m.foreign_dependencies.clone()).unwrap_or_default();
    let available_scripts = manifest.as_ref().map(|m| m.scripts.clone()).unwrap_or_default();

    // AI guidelines
    let mut ai_guidelines = Vec::new();
    if path.join("CLAUDE.md").exists() { ai_guidelines.push("CLAUDE.md".to_string()); }
    if path.join(".cursorrules").exists() { ai_guidelines.push(".cursorrules".to_string()); }
    if path.join(".windsurfrules").exists() { ai_guidelines.push(".windsurfrules".to_string()); }
    if path.join(".clinerules").exists() { ai_guidelines.push(".clinerules".to_string()); }
    if path.join(".agent").join("rules").exists() { ai_guidelines.push(".agent/rules".to_string()); }

    // MCP servers
    let mut mcp_servers = Vec::new();
    let mcp_json = path.join("mcp.json");
    if mcp_json.exists() {
        if let Ok(c) = std::fs::read_to_string(&mcp_json) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&c) {
                if let Some(srvs) = val.get("mcpServers").and_then(|s| s.as_object()) {
                    for (k, _) in srvs {
                        mcp_servers.push(k.clone());
                    }
                }
            }
        }
    }

    let stats = get_cache_stats();

    let agent_instructions = vec![
        "Use `cpm install` to install dependencies across all ecosystems simultaneously.".to_string(),
        "Use `cpm add <eco>:<pkg>` (e.g. `cpm add pip:requests` or `cpm add npm:lodash`) to add foreign dependencies.".to_string(),
        "Use `cpm run <script>` to execute workspace tasks consistently.".to_string(),
        "Use `cpm lock --verify` to guarantee reproducible builds and audit hash integrity.".to_string(),
        "All caches and package stores are centralized on high-capacity storage.".to_string(),
    ];

    AgentWorkspaceContext {
        workspace_name: name,
        version,
        ecosystems,
        foreign_dependencies,
        available_scripts,
        mcp_servers_configured: mcp_servers,
        ai_guidelines_present: ai_guidelines,
        cpm_home: get_cpm_home().display().to_string(),
        cache_drive: stats.drive,
        agent_instructions,
    }
}

fn print_human_agent_context(ctx: &AgentWorkspaceContext) {
    println!();
    println!("  {}", "🤖 CPM Agentic Workspace Context (Token-Optimized)".bold().cyan());
    println!("  {}", "═".repeat(65).dimmed());
    println!("  Workspace:      {} (v{})", ctx.workspace_name.bold().yellow(), ctx.version.cyan());
    println!("  Ecosystems:     {}", ctx.ecosystems.join(", ").green().bold());
    println!("  Storage Drive:  {}", ctx.cache_drive.green().bold());
    println!("  CPM Home:       {}", ctx.cpm_home.cyan());

    if !ctx.foreign_dependencies.is_empty() {
        println!();
        println!("  {}", "Foreign Dependencies:".bold().white());
        for (k, v) in &ctx.foreign_dependencies {
            println!("    • {:<20} {}", k.bold().yellow(), v.cyan());
        }
    }

    if !ctx.available_scripts.is_empty() {
        println!();
        println!("  {}", "Executable Scripts:".bold().white());
        for (k, v) in &ctx.available_scripts {
            println!("    • {:<12} → {}", format!("cpm run {}", k).bold().green(), v.dimmed());
        }
    }

    if !ctx.mcp_servers_configured.is_empty() {
        println!();
        println!("  {}", "Active MCP Servers:".bold().white());
        for s in &ctx.mcp_servers_configured {
            println!("    • {}", s.cyan().bold());
        }
    }

    println!();
    println!("  {}", "Autonomous AI Instructions:".bold().white());
    for inst in &ctx.agent_instructions {
        println!("    › {}", inst.dimmed());
    }
    println!();
    println!("  {} For LLM ingestion, run: {}", "💡".yellow(), "cpm agent context --json".bold().cyan());
    println!();
}

fn print_agent_system_prompt() {
    println!();
    println!("  {}", "📋 AI System Prompt Snippet (Copy into your AI Agent / IDE):".bold().cyan());
    println!("  {}", "═".repeat(70).dimmed());
    println!(
        r#"You are an AI coding assistant working in a CPM / UPM universal package workspace.
Always use `cpm` for package management and tasks:
- Run commands: `cpm run <script>` (e.g. `cpm run dev`, `cpm run test`)
- Add dependencies: `cpm add <ecosystem>:<package>` (e.g. `cpm add pip:fastapi`)
- Inspect repo architecture: `cpm agent context --json`
- Manage MCP tools: `cpm mcp list` / `cpm mcp sync`
- Verify integrity: `cpm lock --verify`
All package stores and caches are optimized on high-capacity storage."#
    );
    println!("  {}", "═".repeat(70).dimmed());
    println!();
}
