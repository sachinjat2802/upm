//! # Multi-Environment Secret & Profile Manager (`cpm env`)
//!
//! Provides seamless `.env` file management, environment switching
//! (dev, staging, prod), key/value querying, and secret masking.
//!
//! Time Complexity: O(L) where L = lines in .env file.
//! Space Complexity: O(L) string map buffer.

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn execute_env(
    path: &Path,
    action: Option<&str>,
    key: Option<&str>,
    value: Option<&str>,
) -> Result<()> {
    let act = action.unwrap_or("list");

    match act {
        "list" | "ls" => list_env(path),
        "get" => {
            if let Some(k) = key {
                get_env_key(path, k)
            } else {
                Err(anyhow!("Missing key name: cpm env get <KEY>"))
            }
        }
        "set" => {
            if let (Some(k), Some(v)) = (key, value) {
                set_env_key(path, k, v)
            } else {
                Err(anyhow!("Missing key and value: cpm env set <KEY> <VALUE>"))
            }
        }
        "switch" | "use" => {
            if let Some(profile) = key {
                switch_env_profile(path, profile)
            } else {
                Err(anyhow!("Missing profile: cpm env switch <dev|staging|prod>"))
            }
        }
        other => {
            println!("  {} Unknown env action: '{}'", "!".yellow(), other);
            println!("  Usage: cpm env [list | get <KEY> | set <KEY> <VAL> | switch <PROFILE>]");
            Ok(())
        }
    }
}

fn list_env(path: &Path) -> Result<()> {
    let env_file = path.join(".env");
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🔑 Active Environment Configuration (.env)          │", "│".cyan());
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    if !env_file.exists() {
        println!("  {} No .env file found in {}", "ℹ".cyan(), path.display());
        println!("  Initialize one with: {}", "cpm env set KEY VALUE".bold().green());
        println!();
        return Ok(());
    }

    let entries = parse_env_file(&env_file)?;
    if entries.is_empty() {
        println!("  .env file is empty.");
    } else {
        for (k, v) in &entries {
            let masked = mask_secret(v);
            println!("  • {:<24} = {}", k.bold().yellow(), masked.dimmed());
        }
    }
    println!();
    Ok(())
}

fn get_env_key(path: &Path, key: &str) -> Result<()> {
    let env_file = path.join(".env");
    if !env_file.exists() {
        return Err(anyhow!(".env file not found"));
    }
    let entries = parse_env_file(&env_file)?;
    if let Some(v) = entries.get(key) {
        println!("{}", v);
        Ok(())
    } else {
        Err(anyhow!("Key '{}' not found in .env", key))
    }
}

fn set_env_key(path: &Path, key: &str, value: &str) -> Result<()> {
    let env_file = path.join(".env");
    let mut entries = if env_file.exists() {
        parse_env_file(&env_file)?
    } else {
        BTreeMap::new()
    };

    entries.insert(key.to_string(), value.to_string());
    save_env_file(&env_file, &entries)?;

    println!("  {} Set {} in .env", "✔".bold().green(), key.bold().yellow());
    Ok(())
}

fn switch_env_profile(path: &Path, profile: &str) -> Result<()> {
    let target = path.join(format!(".env.{}", profile));
    let active = path.join(".env");

    if !target.exists() {
        return Err(anyhow!("Profile file '{}' does not exist", target.display()));
    }

    fs::copy(&target, &active)?;
    println!(
        "  {} Switched active environment to '{}' (.env updated)",
        "✔".bold().green(),
        profile.bold().cyan()
    );
    Ok(())
}

fn parse_env_file(path: &Path) -> Result<BTreeMap<String, String>> {
    let content = fs::read_to_string(path)?;
    let mut map = BTreeMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }
    Ok(map)
}

fn save_env_file(path: &Path, entries: &BTreeMap<String, String>) -> Result<()> {
    let mut out = String::with_capacity(entries.len() * 32);
    for (k, v) in entries {
        let escaped = v.replace('\\', "\\\\").replace('"', "\\\"");
        out.push_str(&format!("{}=\"{}\"\n", k, escaped));
    }
    fs::write(path, out)?;
    Ok(())
}

fn mask_secret(val: &str) -> String {
    let char_count = val.chars().count();
    if char_count <= 4 {
        "****".to_string()
    } else {
        let prefix: String = val.chars().take(2).collect();
        let suffix: String = val.chars().rev().take(2).collect::<Vec<_>>().into_iter().rev().collect();
        format!("{}...{}", prefix, suffix)
    }
}
