/// # `cpm init` / `upm init`
///
/// Interactive project initializer that creates a polyglot workspace with
/// primary and foreign language ecosystems. Produces `upm.toml`, scaffolds
/// native manifests, and runs initial dependency acquisition.

use crate::acquisition::{AcquisitionRunner, AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;

/// Available languages for selection in the interactive init flow.
const SUPPORTED_LANGUAGES: &[(&str, &str, &str)] = &[
    ("javascript", "JavaScript / TypeScript", "📦"),
    ("python",     "Python",                  "🐍"),
    ("rust",       "Rust",                    "🦀"),
    ("go",         "Go",                      "🐹"),
    ("java",       "Java / Kotlin",           "☕"),
    ("php",        "PHP",                     "🐘"),
    ("ruby",       "Ruby",                    "💎"),
    ("csharp",     "C# / .NET",              "🔷"),
    ("dart",       "Dart / Flutter",          "🎯"),
    ("elixir",     "Elixir",                  "💧"),
];

/// Prompt the user with a question and return their trimmed input.
/// If `default` is provided and the user presses Enter, return the default.
fn prompt(question: &str, default: Option<&str>) -> String {
    if let Some(def) = default {
        print!("  {} {} [{}]: ", "?".bold().green(), question, def.dimmed());
    } else {
        print!("  {} {}: ", "?".bold().green(), question);
    }
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let trimmed = input.trim().to_string();

    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed
    }
}

/// Display a numbered menu and let the user pick one option.
fn prompt_select(question: &str, options: &[(&str, &str, &str)], default_idx: usize) -> String {
    println!("  {} {}", "?".bold().green(), question);
    for (i, (id, label, icon)) in options.iter().enumerate() {
        let marker = if i == default_idx {
            "›".bold().cyan().to_string()
        } else {
            " ".to_string()
        };
        println!("   {} {} {} {}", marker, format!("{})", i + 1).dimmed(), icon, label);
        let _ = id; // used for return value
    }
    print!("  {} Choose [{}]: ", "›".bold().cyan(), (default_idx + 1).to_string().dimmed());
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return options[default_idx].0.to_string();
    }

    // Accept number or language name
    if let Ok(num) = trimmed.parse::<usize>() {
        if num >= 1 && num <= options.len() {
            return options[num - 1].0.to_string();
        }
    }

    // Try matching by name
    let lower = trimmed.to_lowercase();
    for (id, label, _) in options {
        if id == &lower || label.to_lowercase().starts_with(&lower) {
            return id.to_string();
        }
    }

    options[default_idx].0.to_string()
}

/// Display a multi-select menu and let the user pick multiple options.
fn prompt_multi_select(question: &str, options: &[(&str, &str, &str)], exclude: &str) -> Vec<String> {
    let filtered: Vec<_> = options.iter().filter(|(id, _, _)| *id != exclude).collect();
    println!("  {} {} {}", "?".bold().green(), question, "(comma-separated numbers, e.g. 1,3)".dimmed());

    for (i, (id, label, icon)) in filtered.iter().enumerate() {
        println!("    {} {} {}", format!("{})", i + 1).dimmed(), icon, label);
        let _ = id;
    }
    print!("  {} Choose: ", "›".bold().cyan());
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return vec![];
    }

    let mut selected = Vec::new();
    for part in trimmed.split(',') {
        let part = part.trim();
        if let Ok(num) = part.parse::<usize>() {
            if num >= 1 && num <= filtered.len() {
                selected.push(filtered[num - 1].0.to_string());
            }
        } else {
            // Try matching by name
            let lower = part.to_lowercase();
            for (id, label, _) in &filtered {
                if *id == lower || label.to_lowercase().starts_with(&lower) {
                    selected.push(id.to_string());
                }
            }
        }
    }
    selected
}

/// Execute the `init` subcommand.
pub fn execute_init(
    path: &Path,
    name: Option<String>,
    base_lang: Option<String>,
    foreign_langs: Option<String>,
    yes: bool,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(path)?;
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let folder_name = canonical
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-polyglot-app");

    // Header
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🚀 {}{}",
        "│".cyan(),
        "UPM Project Initializer".bold().white(),
        "                         │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    // --- Gather inputs (interactive or flags) ---
    let proj_name = if let Some(n) = name {
        n
    } else if yes {
        folder_name.to_string()
    } else {
        prompt("Project name", Some(folder_name))
    };

    let primary_lang = if let Some(l) = base_lang {
        l.to_lowercase()
    } else if yes {
        "javascript".to_string()
    } else {
        prompt_select(
            "What is your base language?",
            SUPPORTED_LANGUAGES,
            0, // default: javascript
        )
    };

    let foreign_list: Vec<String> = if let Some(fl) = foreign_langs {
        fl.split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    } else if yes {
        vec!["python".to_string()]
    } else {
        prompt_multi_select(
            "Which foreign ecosystems do you want to support?",
            SUPPORTED_LANGUAGES,
            &primary_lang,
        )
    };

    // Summary
    let lang_icon = SUPPORTED_LANGUAGES
        .iter()
        .find(|(id, _, _)| *id == primary_lang)
        .map(|(_, _, icon)| *icon)
        .unwrap_or("📦");

    println!();
    println!("  {} {}", "Project".dimmed(), proj_name.bold().white());
    println!("  {} {} {}", "Base".dimmed(), lang_icon, primary_lang.bold().magenta());
    if !foreign_list.is_empty() {
        let foreign_display: Vec<String> = foreign_list
            .iter()
            .map(|l| {
                let icon = SUPPORTED_LANGUAGES
                    .iter()
                    .find(|(id, _, _)| id == l)
                    .map(|(_, _, i)| *i)
                    .unwrap_or("📦");
                format!("{} {}", icon, l)
            })
            .collect();
        println!("  {} {}", "Foreign".dimmed(), foreign_display.join(", ").bold().blue());
    }
    println!();

    // Build ecosystem list
    let mut all_langs = vec![primary_lang.clone()];
    for fl in &foreign_list {
        if !all_langs.contains(fl) {
            all_langs.push(fl.clone());
        }
    }

    // Step 1: Scaffold native manifests
    print!("  {} Scaffolding native manifests... ", "⠋".cyan());
    io::stdout().flush().unwrap_or(());
    UpmManifest::bootstrap_native_manifests(path, &proj_name, &all_langs)?;
    println!("{}", "done".green().bold());

    // Map language names to default ecosystem package manager adapters
    let mut ecosystem_adapters = Vec::new();
    for lang in &all_langs {
        match lang.to_lowercase().as_str() {
            "javascript" | "typescript" | "node" | "pnpm" => ecosystem_adapters.push("pnpm".to_string()),
            "npm" => ecosystem_adapters.push("npm".to_string()),
            "yarn" => ecosystem_adapters.push("yarn".to_string()),
            "bun" => ecosystem_adapters.push("bun".to_string()),
            "python" | "pip" | "uv" => ecosystem_adapters.push("uv".to_string()),
            "poetry" => ecosystem_adapters.push("poetry".to_string()),
            "rust" | "cargo" => ecosystem_adapters.push("cargo".to_string()),
            "go" => ecosystem_adapters.push("go".to_string()),
            "java" => ecosystem_adapters.push("maven".to_string()),
            "php" => ecosystem_adapters.push("composer".to_string()),
            "ruby" => ecosystem_adapters.push("bundler".to_string()),
            "csharp" => ecosystem_adapters.push("nuget".to_string()),
            "dart" => ecosystem_adapters.push("pub".to_string()),
            "elixir" => ecosystem_adapters.push("mix".to_string()),
            other => ecosystem_adapters.push(other.to_string()),
        }
    }

    // Step 2: Save upm.toml manifest
    print!("  {} Writing upm.toml... ", "⠋".cyan());
    io::stdout().flush().unwrap_or(());
    let manifest = UpmManifest::new(&proj_name, "0.1.0", Some(&primary_lang), ecosystem_adapters);
    manifest.save_to_dir(path)?;
    println!("{}", "done".green().bold());

    // Step 3: Perform dependency acquisition
    print!("  {} Detecting ecosystems... ", "⠋".cyan());
    io::stdout().flush().unwrap_or(());
    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let detection = engine.detect_dir(path);
    println!("{}", format!("{} found", detection.detected_ecosystems.len()).green().bold());

    if !detection.detected_ecosystems.is_empty() {
        println!();
        AcquisitionRunner::run_install(path, &detection.detected_ecosystems, false, false, None)?;
    }

    // Success footer
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".green());
    println!("  {}  ✨ {}{}",
        "│".green(),
        "Project initialized successfully!".bold().white(),
        "               │".green(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".green());
    println!();
    println!("  {} {}", "Next steps:".bold(), "");
    println!("    {} Show ecosystem breakdown", "cpm detect".bold().cyan());
    println!("    {} Add a foreign dependency", "cpm add pip:requests".bold().cyan());
    println!("    {} Test cross-language RPC", "cpm bridge call python:math.sqrt '[9]'".bold().cyan());
    println!();

    Ok(())
}
