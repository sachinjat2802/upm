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
pub mod trace_cmd;

use crate::acquisition::{AcquisitionRunner, AdapterRegistry, DetectionEngine, UpmManifest};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

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

// ─── Shared helpers ───────────────────────────────────────────────────────────

/// Build the detection result for a workspace path (reused by many commands).
fn detect_workspace(path: &Path) -> crate::acquisition::DetectionResult {
    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    engine.detect_dir(path)
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

/// Add a foreign package dependency (format: ecosystem:package).
pub fn execute_add(path: &Path, package: &str, dry_run: bool) -> anyhow::Result<()> {
    let registry = AdapterRegistry::new();
    let (eco_name, pkg_name) = if let Some((eco, pkg)) = package.split_once(':') {
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

    if let Some(adapter) = registry.get(&eco_name) {
        println!();
        AcquisitionRunner::run_add(path, adapter, &pkg_name, dry_run)?;

        // Update upm.toml manifest
        let mut manifest = UpmManifest::load_from_dir(path)
            .unwrap_or_else(|| UpmManifest::default_for("polyglot-app", vec![eco_name.clone()]));
        manifest
            .foreign_dependencies
            .insert(format!("{}:{}", eco_name, pkg_name), "latest".into());
        manifest.save_to_dir(path)?;
        println!("  {} Updated {} with {}:{} dependency.", "✔".green(), "upm.toml".bold(), eco_name.cyan(), pkg_name.bold().yellow());
        println!();
    } else {
        println!();
        println!("  {} Unknown ecosystem: '{}'", "✖".bold().red(), eco_name.bold());
        println!();
        println!("  {} Supported ecosystems:", "?".bold().green());
        let known = ["npm", "pnpm", "yarn", "bun", "pip", "uv", "poetry", "cargo", "go", "maven", "gradle", "composer", "bundler", "nuget", "pub", "mix"];
        for name in &known {
            println!("    {} {}", "·".dimmed(), name.cyan());
        }
        println!();
        println!("  {} Use {} format, e.g. {}", "Tip".bold(), "ecosystem:package".bold(), "pip:requests".bold().yellow());
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
        // Build the remove command based on ecosystem
        let remove_cmd = match eco_name.as_str() {
            "npm" => vec!["npm".into(), "uninstall".into(), pkg_name.clone()],
            "pnpm" => vec!["pnpm".into(), "remove".into(), pkg_name.clone()],
            "yarn" => vec!["yarn".into(), "remove".into(), pkg_name.clone()],
            "bun" => vec!["bun".into(), "remove".into(), pkg_name.clone()],
            "pip" | "uv" => vec!["uv".into(), "remove".into(), pkg_name.clone()],
            "poetry" => vec!["poetry".into(), "remove".into(), pkg_name.clone()],
            "cargo" => vec!["cargo".into(), "remove".into(), pkg_name.clone()],
            "go" => vec!["go".into(), "get".into(), format!("{}@none", pkg_name)],
            "composer" => vec!["composer".into(), "remove".into(), pkg_name.clone()],
            "bundler" => vec!["bundle".into(), "remove".into(), pkg_name.clone()],
            "nuget" => vec!["dotnet".into(), "remove".into(), "package".into(), pkg_name.clone()],
            "pub" => vec!["dart".into(), "pub".into(), "remove".into(), pkg_name.clone()],
            "mix" => vec!["mix".into(), "deps.clean".into(), pkg_name.clone()],
            _ => vec![],
        };

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
            match Command::new(program).args(args).current_dir(path).status() {
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

/// Run a named script across all detected ecosystems.
pub fn execute_run(path: &Path, script: &str) -> anyhow::Result<()> {
    let result = detect_workspace(path);

    if result.detected_ecosystems.is_empty() {
        println!();
        println!("  {} No ecosystems detected. Nothing to run.", "!".bold().yellow());
        println!("  {} Run {} first.", "›".dimmed(), "cpm init".bold().cyan());
        println!();
        return Ok(());
    }

    println!();
    println!(
        "  {} Running '{}' across {} ecosystem(s)...",
        "▶".bold().cyan(),
        script.bold().magenta(),
        result.detected_ecosystems.len().to_string().bold()
    );
    println!();

    for eco in &result.detected_ecosystems {
        let mut cmd = eco.run_cmd.clone();
        cmd.push(script.to_string());

        println!(
            "  {} [{}] {}",
            "›".bold().cyan(),
            eco.display_name.bold(),
            cmd.join(" ").yellow()
        );

        let program = &cmd[0];
        let args = &cmd[1..];
        match Command::new(program).args(args).current_dir(path).status() {
            Ok(status) if status.success() => {
                println!("  {} [{}] completed successfully.", "✔".green(), eco.display_name);
            }
            Ok(status) => {
                println!("  {} [{}] exited with {}", "!".yellow(), eco.display_name, status);
            }
            Err(e) => {
                println!(
                    "  {} [{}] '{}' not found: {}",
                    "⚠".yellow().dimmed(),
                    eco.display_name.dimmed(),
                    program.dimmed(),
                    e.to_string().dimmed()
                );
            }
        }
        println!();
    }

    Ok(())
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
        println!("  {} {}",    "Manifest".dimmed(), "not found (run cpm init)".yellow());
        println!();
    }

    // Detected ecosystems
    if result.detected_ecosystems.is_empty() {
        println!("  {} No ecosystems detected.", "·".dimmed());
    } else {
        println!("  {}", "Detected Ecosystems:".bold());
        for eco in &result.detected_ecosystems {
            let icon = match eco.language.as_str() {
                "javascript" => "📦",
                "python" => "🐍",
                "rust" => "🦀",
                "go" => "🐹",
                "java" => "☕",
                "php" => "🐘",
                "ruby" => "💎",
                "csharp" => "🔷",
                "dart" => "🎯",
                "elixir" => "💧",
                _ => "📦",
            };
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
    println!("  {} Scanned in {}ms across {} adapter(s).",
        "⏱".dimmed(),
        scan_ms,
        result.scores.iter().filter(|s| s.total_score > 0).count()
    );
    println!();

    Ok(())
}
