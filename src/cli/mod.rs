/// # CLI Command Handlers
///
/// Each public function corresponds to a subcommand of `upm` / `cpm`.
/// Commands delegate to the acquisition engine, adapter registry, and
/// bridge subsystem.

pub mod alias_cmd;
pub mod audit_log_cmd;
pub mod benchmark_cmd;
pub mod bridge_cmd;
pub mod bundle_cmd;
pub mod cache_cmd;
pub mod completion_cmd;
pub mod cost_cmd;
pub mod detect_cmd;
pub mod diff_cmd;
pub mod doctor_cmd;
pub mod dockerfile_cmd;
pub mod flamegraph_cmd;
pub mod graph_cmd;
pub mod helm_cmd;
pub mod init_cmd;
pub mod licenses_cmd;
pub mod logs_cmd;
pub mod migrate_cmd;
pub mod operator_cmd;
pub mod policy_cmd;
pub mod repl_cmd;
pub mod resolve_cmd;
pub mod rollback_cmd;
pub mod sccache_cmd;
pub mod search_cmd;
pub mod secrets_cmd;
pub mod sig_cmd;
pub mod stubs_cmd;
pub mod clean_cmd;
pub mod list_cmd;
pub mod quickstart_cmd;
pub mod why_cmd;
pub mod trace_cmd;
pub mod run_cmd;
pub mod lock_cmd;
pub mod guess_cmd;
pub mod info_cmd;
pub mod drive_cmd;
pub mod ai_cmd;
pub mod mcp_cmd;
pub mod create_cmd;
pub mod agent_cmd;
pub mod test_cmd;
pub mod build_cmd;
pub mod deploy_cmd;
pub mod env_cmd;
pub mod workspace_cmd;
pub mod publish_cmd;
pub mod plugin_cmd;
pub mod config_cmd;
pub mod upgrade_cmd;
pub mod ci_cmd;

use crate::acquisition::{AcquisitionRunner, AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub mod cli_spec;
pub use cli_spec::*;

pub use clean_cmd::execute_clean;
pub use list_cmd::execute_list;
pub use quickstart_cmd::execute_quickstart;
pub use why_cmd::execute_why;
pub use alias_cmd::execute_alias;
pub use audit_log_cmd::execute_audit_log;
pub use benchmark_cmd::execute_benchmark;
pub use bridge_cmd::{execute_bridge_call, execute_bridge_inspect, execute_bridge_status};
pub use bundle_cmd::execute_bundle;
pub use cache_cmd::execute_cache;
pub use completion_cmd::execute_completion;
pub use cost_cmd::execute_cost;
pub use detect_cmd::execute_detect;
pub use diff_cmd::execute_diff;
pub use doctor_cmd::execute_doctor;
pub use dockerfile_cmd::execute_dockerfile;
pub use flamegraph_cmd::execute_flamegraph;
pub use graph_cmd::execute_graph;
pub use helm_cmd::execute_helm;
pub use init_cmd::execute_init;
pub use licenses_cmd::execute_licenses;
pub use logs_cmd::execute_logs;
pub use migrate_cmd::execute_migrate;
pub use operator_cmd::execute_operator;
pub use policy_cmd::execute_policy;
pub use repl_cmd::execute_repl;
pub use resolve_cmd::execute_resolve;
pub use rollback_cmd::execute_rollback;
pub use sccache_cmd::execute_sccache;
pub use search_cmd::execute_search;
pub use secrets_cmd::execute_scan_secrets;
pub use sig_cmd::execute_verify_sig;
pub use stubs_cmd::execute_generate_stubs;
pub use trace_cmd::execute_trace;
pub use run_cmd::{execute_run_script, execute_tool};
pub use lock_cmd::execute_lock;
pub use guess_cmd::execute_guess;
pub use info_cmd::execute_info;
pub use drive_cmd::execute_drive;
pub use ai_cmd::execute_ai;
pub use mcp_cmd::execute_mcp;
pub use create_cmd::execute_create;
pub use agent_cmd::execute_agent;
pub use test_cmd::execute_test;
pub use build_cmd::execute_build;
pub use deploy_cmd::execute_deploy;
pub use env_cmd::execute_env;
pub use workspace_cmd::execute_workspace;
pub use publish_cmd::execute_publish;
pub use plugin_cmd::execute_plugin;
pub use config_cmd::execute_config;
pub use upgrade_cmd::execute_upgrade;
pub use ci_cmd::execute_ci;

// ─── Shared helpers ───────────────────────────────────────────────────────────

/// Build the detection result for a workspace path (reused by many commands).
fn detect_workspace(path: &Path) -> crate::acquisition::DetectionResult {
    // Fast path: If upm.toml exists and declares ecosystems, resolve instantly (<1ms) (#53)
    if let Some(manifest) = UpmManifest::load_from_dir(path) {
        if !manifest.ecosystems.is_empty() {
            let registry = AdapterRegistry::new();
            let mut detected_ecosystems = Vec::new();
            for eco_name in &manifest.ecosystems {
                if let Some(adapter) = registry.get(eco_name) {
                    detected_ecosystems.push(adapter.clone());
                }
            }
            if !detected_ecosystems.is_empty() {
                return crate::acquisition::DetectionResult {
                    root_path: path.to_path_buf(),
                    scores: vec![],
                    detected_ecosystems,
                };
            }
        }
    }

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    engine.detect_dir(path)
}

/// Get a display emoji icon for a language ecosystem.
///
/// This is the **single source of truth** for language-to-icon mapping,
/// shared across all CLI commands to avoid inconsistencies (#6).
pub fn lang_icon(lang: &str) -> &'static str {
    match lang {
        "javascript" | "typescript" | "node" | "js" | "ts" => "📦",
        "python" | "py" => "🐍",
        "rust" => "🦀",
        "go" | "golang" => "🐹",
        "java" | "kotlin" => "☕",
        "php" => "🐘",
        "ruby" | "rb" => "💎",
        "csharp" | "dotnet" => "🔷",
        "dart" | "flutter" => "🎯",
        "elixir" => "💧",
        _ => "📦",
    }
}

/// Print a consistent box-drawing header with dynamic padding (#5).
///
/// Produces output like:
/// ```text
///   ╭──────────────────────────────────────────────────────╮
///   │  🚀 Title Text Here                                  │
///   ╰──────────────────────────────────────────────────────╯
/// ```
pub fn print_header(icon: &str, title: &str) {
    let box_width = 56; // inner width between │ and │
    let content = format!("  {} {}", icon, title);
    let padding = if content.len() < box_width {
        " ".repeat(box_width - content.len())
    } else {
        String::new()
    };
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}{}{}",
        "│".cyan(),
        format!("{}{}", content, padding).bold().white(),
        "│".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
}

// ─── install ──────────────────────────────────────────────────────────────────

/// Install dependencies across all detected ecosystems.
pub fn execute_install(path: &Path, dry_run: bool, parallel: bool, filter: Option<&str>) -> anyhow::Result<()> {
    let result = detect_workspace(path);

    if result.detected_ecosystems.is_empty() {
        println!();
        println!("  {} {}", "!".bold().yellow(), "No ecosystems detected in this directory.".yellow());
        println!("  {} Run {} to set up a polyglot project.", "›".dimmed(), "cpm init".bold().cyan());
        println!();

        // Auto-create upm.toml with sensible defaults
        let manifest = UpmManifest::default_for("polyglot-app", vec!["pnpm".into(), "uv".into(), "cargo".into()]);
        manifest.save_to_dir(path)?;
        println!("  {} Created {} with default ecosystems.", "✔".green(), "upm.toml".bold());
        return Ok(());
    }

    println!();
    AcquisitionRunner::run_install(path, &result.detected_ecosystems, dry_run, parallel, filter)?;
    println!();
    Ok(())
}

// ─── add ──────────────────────────────────────────────────────────────────────

/// Calculate Levenshtein distance between two strings with O(min(|A|, |B|)) space complexity.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();

    // Ensure b is the shorter string for minimum space allocation
    if a_chars.len() < b_chars.len() {
        std::mem::swap(&mut a_chars, &mut b_chars);
    }

    let m = a_chars.len();
    let n = b_chars.len();

    if n == 0 {
        return m;
    }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

/// Find the closest matching string candidate for a typo suggestion (#12).
fn find_closest_match<'a>(input: &str, candidates: &'a [&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .map(|&c| (c, levenshtein_distance(input, c)))
        .filter(|&(_, dist)| dist <= 3)
        .min_by_key(|&(_, dist)| dist)
        .map(|(c, _)| c)
}

/// Backup upm.toml manifest before modifying it (#66).
fn backup_manifest_before_save(path: &Path) {
    let manifest_file = path.join("upm.toml");
    if manifest_file.exists() {
        let backup_dir = path.join(".cpm_backup");
        let _ = std::fs::create_dir_all(&backup_dir);
        let _ = std::fs::copy(&manifest_file, backup_dir.join("upm.toml.bak"));
    }
}

// ─── add ──────────────────────────────────────────────────────────────────────

/// Add a foreign package dependency (format: ecosystem:package@version or package@version).
pub fn execute_add(path: &Path, package: &str, dry_run: bool) -> anyhow::Result<()> {
    let registry = AdapterRegistry::new();
    let known_ecosystems = ["npm", "pnpm", "yarn", "bun", "pip", "uv", "poetry", "cargo", "go", "maven", "gradle", "composer", "bundler", "nuget", "pub", "mix"];

    let (eco_name, raw_pkg) = if let Some((eco, pkg)) = package.split_once(':') {
        (eco.to_string(), pkg.to_string())
    } else {
        // Infer ecosystem from workspace detection if not explicitly prefixed
        let detection = detect_workspace(path);
        let default_eco = if let Some(winner) = detection.detected_ecosystems.first() {
            match winner.language.as_str() {
                "python" => "pip".to_string(),
                "rust" => "cargo".to_string(),
                "go" => "go".to_string(),
                "javascript" => winner.name.clone(),
                "java" => "maven".to_string(),
                "php" => "composer".to_string(),
                "ruby" => "bundler".to_string(),
                "csharp" => "nuget".to_string(),
                "dart" => "pub".to_string(),
                "elixir" => "mix".to_string(),
                _ => "npm".to_string(),
            }
        } else {
            "npm".to_string()
        };
        (default_eco, package.to_string())
    };

    // Support @version syntax (#13)
    let (pkg_name, pkg_version) = if let Some((name, ver)) = raw_pkg.split_once('@') {
        (name.to_string(), if ver.starts_with('^') || ver.starts_with('~') || ver.starts_with('>') || ver.starts_with('=') { ver.to_string() } else { format!("^{}", ver) })
    } else {
        (raw_pkg.clone(), "^latest".to_string()) // #61: Record version specification format
    };

    if let Some(adapter) = registry.get(&eco_name) {
        println!();
        AcquisitionRunner::run_add(path, adapter, &raw_pkg, dry_run)?;

        // Backup manifest before updating (#66)
        backup_manifest_before_save(path);

        // Update upm.toml manifest (#61, #13)
        let mut manifest = UpmManifest::load_from_dir(path)
            .unwrap_or_else(|| UpmManifest::default_for("polyglot-app", vec![eco_name.clone()]));
        manifest
            .foreign_dependencies
            .insert(format!("{}:{}", eco_name, pkg_name), pkg_version.clone());
        manifest.save_to_dir(path)?;
        println!("  {} Updated {} with {}:{} ({}) dependency.", "✔".green(), "upm.toml".bold(), eco_name.cyan(), pkg_name.bold().yellow(), pkg_version.dimmed());
        println!();
    } else {
        println!();
        println!("  {} Unknown ecosystem: '{}'", "✖".bold().red(), eco_name.bold());
        if let Some(suggestion) = find_closest_match(&eco_name, &known_ecosystems) {
            println!("  {} Did you mean '{}'?", "💡".yellow(), suggestion.bold().cyan());
        }
        println!();
        println!("  {} Supported ecosystems:", "?".bold().green());
        for name in &known_ecosystems {
            println!("    {} {}", "·".dimmed(), name.cyan());
        }
        println!();
        println!("  {} Use {} format, e.g. {}", "Tip".bold(), "ecosystem:package@version".bold(), "pip:requests@2.31.0".bold().yellow());
        println!();
        anyhow::bail!("Unknown ecosystem adapter: '{}'", eco_name);
    }
    Ok(())
}

// ─── remove ───────────────────────────────────────────────────────────────────

/// Remove a foreign package dependency.
pub fn execute_remove(path: &Path, package: &str, dry_run: bool) -> anyhow::Result<()> {
    let registry = AdapterRegistry::new();
    let (eco_name, pkg_name) = if let Some((eco, pkg)) = package.split_once(':') {
        (eco.to_string(), pkg.to_string())
    } else {
        let detection = detect_workspace(path);
        let default_eco = if let Some(winner) = detection.detected_ecosystems.first() {
            match winner.language.as_str() {
                "python" => "pip".to_string(),
                "rust" => "cargo".to_string(),
                "go" => "go".to_string(),
                "javascript" => winner.name.clone(),
                "java" => "maven".to_string(),
                "php" => "composer".to_string(),
                "ruby" => "bundler".to_string(),
                "csharp" => "nuget".to_string(),
                "dart" => "pub".to_string(),
                "elixir" => "mix".to_string(),
                _ => "npm".to_string(),
            }
        } else {
            "npm".to_string()
        };
        (default_eco, package.to_string())
    };

    if let Some(adapter) = registry.get(&eco_name) {
        // Build the remove command from the adapter definition (#28)
        let mut remove_cmd = adapter.remove_cmd.clone();
        remove_cmd.push(pkg_name.clone());

        println!();
        println!(
            "  {} Removing {} from [{}] via {}",
            "›".bold().cyan(),
            pkg_name.bold().yellow(),
            adapter.display_name.bold(),
            remove_cmd.join(" ").magenta()
        );

        if !dry_run && !remove_cmd.is_empty() {
            let program = &remove_cmd[0];
            let args = &remove_cmd[1..];
            let resolved = AcquisitionRunner::resolve_executable(program);
            match Command::new(&resolved).args(args).current_dir(path).status() {
                Ok(status) if status.success() => {
                    println!("  {} Successfully removed {}.", "✔".green(), pkg_name.bold());
                }
                Ok(status) => {
                    println!("  {} Command exited with {}", "!".yellow(), status);
                }
                Err(e) => {
                    println!("  {} Could not run '{}': {}", "!".yellow().dimmed(), program.dimmed(), e.to_string().dimmed());
                }
            }
        }

        // Backup manifest before updating (#66)
        backup_manifest_before_save(path);

        // Update upm.toml manifest
        let mut manifest = UpmManifest::load_from_dir(path)
            .unwrap_or_else(|| UpmManifest::default_for("polyglot-app", vec![eco_name.clone()]));
        manifest.foreign_dependencies.remove(&format!("{}:{}", eco_name, pkg_name));
        manifest.save_to_dir(path)?;
        println!("  {} Removed {}:{} from upm.toml.", "✔".green(), eco_name.cyan(), pkg_name.bold());
        println!();
    } else {
        anyhow::bail!("Unknown ecosystem adapter: '{}'. Run `cpm add --help` for supported ecosystems.", eco_name);
    }
    Ok(())
}

// ─── update ───────────────────────────────────────────────────────────────────

/// Update dependencies across all detected ecosystems.
pub fn execute_update(path: &Path, dry_run: bool, parallel: bool, filter: Option<&str>) -> anyhow::Result<()> {
    let result = detect_workspace(path);
    println!();
    AcquisitionRunner::run_update(path, &result.detected_ecosystems, dry_run, parallel, filter)?;
    println!();
    Ok(())
}

// ─── outdated ─────────────────────────────────────────────────────────────────

/// List outdated dependencies across ecosystems.
pub fn execute_outdated(path: &Path, filter: Option<&str>) -> anyhow::Result<()> {
    let result = detect_workspace(path);
    println!();
    AcquisitionRunner::run_outdated(path, &result.detected_ecosystems, filter)?;
    println!();
    Ok(())
}

// ─── audit ────────────────────────────────────────────────────────────────────

/// Audit security vulnerabilities and supply-chain lockfile pinning across ecosystems.
pub fn execute_audit(path: &Path, filter: Option<&str>) -> anyhow::Result<()> {
    let result = detect_workspace(path);
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🛡️ {}{}",
        "│".cyan(),
        "Supply Chain Security & Vulnerability Audit".bold().white(),
        "        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    // Check lockfile pinning for supply-chain security
    println!("  {} {}", "🔒".bold(), "Lockfile & Reproducibility Check:".bold());
    let filtered = AcquisitionRunner::filter_adapters(&result.detected_ecosystems, filter);
    for eco in &filtered {
        if let Some(ref lockfile) = eco.lockfile_file {
            let lock_path = path.join(lockfile);
            if lock_path.exists() {
                println!("    {} {} → {}", "✔".green(), eco.display_name.bold(), format!("{} present (pinned)", lockfile).dimmed());
            } else {
                println!("    {} {} → {}", "!".bold().yellow(), eco.display_name.bold(), format!("{} MISSING! Dependencies are unpinned", lockfile).yellow());
            }
        }
    }
    println!();

    AcquisitionRunner::run_audit(path, &result.detected_ecosystems, filter)?;
    println!();
    Ok(())
}

// ─── run ──────────────────────────────────────────────────────────────────────

/// Run a named script across all detected ecosystems or from `upm.toml`.
pub fn execute_run(path: &Path, script: &str) -> anyhow::Result<()> {
    execute_run_script(path, Some(script), &[])
}

// ─── status ───────────────────────────────────────────────────────────────────

/// Show a comprehensive project status overview.
pub fn execute_status(path: &Path) -> anyhow::Result<()> {
    let start = Instant::now();
    let result = detect_workspace(path);
    let manifest = UpmManifest::load_from_dir(path);
    let scan_ms = start.elapsed().as_millis();

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📊 {}{}",
        "│".cyan(),
        "Project Status".bold().white(),
        "                                  │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    // Project info from manifest
    if let Some(ref m) = manifest {
        println!("  {} {}",    "Name".dimmed(),     m.project.name.bold().white());
        println!("  {} {}",    "Version".dimmed(),  m.project.version.bold());
        if let Some(ref lang) = m.project.primary_language {
            println!("  {} {}",  "Language".dimmed(), lang.bold().magenta());
        }
        println!();
    } else {
        println!("  {} {}",    "Manifest".dimmed(), "not found (run upm init)".yellow());
        println!();
    }

    // Detected ecosystems
    if result.detected_ecosystems.is_empty() {
        println!("  {} No ecosystems detected.", "·".dimmed());
    } else {
        println!("  {}", "Detected Ecosystems:".bold());
        for eco in &result.detected_ecosystems {
            let icon = lang_icon(&eco.language);
            println!(
                "    {} {} {} {}",
                "✔".green(),
                icon,
                eco.display_name.bold(),
                format!("({})", eco.manifest_file).dimmed()
            );
        }
        println!();
    }

    // Foreign dependencies
    if let Some(ref m) = manifest {
        if !m.foreign_dependencies.is_empty() {
            println!("  {}", "Foreign Dependencies:".bold());
            for (key, version) in &m.foreign_dependencies {
                println!("    {} {} {}", "·".dimmed(), key.bold().yellow(), version.dimmed());
            }
            println!();
        }
    }

    // Footer
    let active_count = if result.scores.is_empty() {
        result.detected_ecosystems.len()
    } else {
        result.scores.iter().filter(|s| s.total_score > 0).count()
    };

    println!("  {} Scanned in {}ms across {} active ecosystem(s).",
        "⏱".dimmed(),
        scan_ms,
        active_count
    );
    println!();

    Ok(())
}
