//! # Global CPM Configuration Manager (`cpm config`)
//!
//! Manages global user settings, default registries, network timeouts,
//! cache quotas, and telemetry flags in `D:\.cpm\config.toml`.
//!
//! Time Complexity: O(1) read/write of config TOML.
//! Space Complexity: O(1) buffer allocation.

use crate::acquisition::cache::get_cpm_home;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub settings: BTreeMap<String, String>,
}

pub fn execute_config(action: Option<&str>, key: Option<&str>, value: Option<&str>) -> Result<()> {
    let act = action.unwrap_or("list");

    match act {
        "list" | "ls" => list_config(),
        "get" => {
            if let Some(k) = key {
                get_config(k)
            } else {
                Err(anyhow!("Missing key: cpm config get <key>"))
            }
        }
        "set" => {
            if let (Some(k), Some(v)) = (key, value) {
                set_config(k, v)
            } else {
                Err(anyhow!("Missing key and value: cpm config set <key> <value>"))
            }
        }
        "reset" => reset_config(),
        other => {
            println!("  {} Unknown config action: '{}'", "!".yellow(), other);
            println!("  Usage: cpm config [list | get <key> | set <key> <value> | reset]");
            Ok(())
        }
    }
}

fn config_path() -> PathBuf {
    get_cpm_home().join("config.toml")
}

fn load_config() -> GlobalConfig {
    let path = config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<GlobalConfig>(&content) {
                return cfg;
            }
        }
    }

    // Default configuration values
    let mut defaults = BTreeMap::new();
    defaults.insert("storage.drive".to_string(), "D:".to_string());
    defaults.insert("cache.enabled".to_string(), "true".to_string());
    defaults.insert("cache.max_size_gb".to_string(), "50".to_string());
    defaults.insert("network.timeout_seconds".to_string(), "30".to_string());
    defaults.insert("registries.npm".to_string(), "https://registry.npmjs.org/".to_string());
    defaults.insert("registries.pypi".to_string(), "https://pypi.org/simple/".to_string());
    defaults.insert("registries.crates".to_string(), "https://crates.io".to_string());
    defaults.insert("ai.model_provider".to_string(), "openrouter".to_string());

    GlobalConfig { settings: defaults }
}

fn save_config(cfg: &GlobalConfig) -> Result<()> {
    let path = config_path();
    let content = toml::to_string_pretty(cfg)?;
    fs::write(path, content)?;
    Ok(())
}

fn list_config() -> Result<()> {
    let cfg = load_config();
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ⚙️ {}", "│".cyan(), "CPM Global Configuration (config.toml)".bold().white());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
    println!("  Config File: {}", config_path().display().to_string().cyan());
    println!();

    for (k, v) in &cfg.settings {
        println!("  • {:<28} = {}", k.bold().yellow(), v.cyan());
    }
    println!();
    println!("  Change any setting: {}", "cpm config set <key> <value>".bold().green());
    println!();
    Ok(())
}

fn get_config(key: &str) -> Result<()> {
    let cfg = load_config();
    if let Some(v) = cfg.settings.get(key) {
        println!("{}", v);
        Ok(())
    } else {
        Err(anyhow!("Setting '{}' not found in configuration", key))
    }
}

fn set_config(key: &str, val: &str) -> Result<()> {
    let mut cfg = load_config();
    cfg.settings.insert(key.to_string(), val.to_string());
    save_config(&cfg)?;
    println!("  {} Set {} = {}", "✔".bold().green(), key.bold().yellow(), val.cyan());
    Ok(())
}

fn reset_config() -> Result<()> {
    let p = config_path();
    if p.exists() {
        let _ = fs::remove_file(p);
    }
    let defaults = load_config();
    save_config(&defaults)?;
    println!("  {} Configuration reset to platform defaults.", "✔".bold().green());
    Ok(())
}
