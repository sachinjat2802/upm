//! # UPM Binary Entry Point
//!
//! Command-line interface driver for `upm` (Universal Package Platform).

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use upm::cli::*;

/// ╭──────────────────────────────────────────────────────╮
/// │  UPM — Universal Package Platform                    │
/// │  Write in one language. Depend on all of them.       │
/// ╰──────────────────────────────────────────────────────╯
#[derive(Parser)]
#[command(
    name = "upm",
    about = "\n  ╭──────────────────────────────────────────────────────╮\n  │  UPM — Universal Package Platform                    │\n  │  Write in one language. Depend on all of them.       │\n  ╰──────────────────────────────────────────────────────╯\n\n  A polyglot package manager that auto-detects and orchestrates\n  npm, pnpm, pip, uv, cargo, go, and 14 more ecosystems.\n\n  Quick start:\n    upm init                    Create a new polyglot project\n    upm detect                  Show detected ecosystems\n    upm install                 Install all dependencies\n    upm add pip:requests        Add a foreign dependency\n    upm bridge call python:math.sqrt '[9]'",
    version = env!("CARGO_PKG_VERSION"),
    after_help = "  Aliases: This binary is also available as `cpm`.\n  Docs:    https://github.com/upm-org/upm\n"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🚀 Initialize a new polyglot project (like `npm init`)
    ///
    /// Creates upm.toml manifest, scaffolds native language manifests
    /// (package.json, pyproject.toml, Cargo.toml, go.mod), and installs
    /// initial dependencies.
    ///
    /// Examples:
    ///   upm init
    ///   upm init --base-lang python --foreign-langs node,rust
    ///   upm init -y
    #[command(visible_alias = "i")]
    Init {
        /// Project directory (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Project name (default: folder name)
        #[arg(long)]
        name: Option<String>,
        /// Base / primary language (javascript, python, rust, go)
        #[arg(short = 'l', long, value_name = "LANG")]
        base_lang: Option<String>,
        /// Supported foreign languages, comma-separated (python,node,rust,go)
        #[arg(short = 'f', long, value_name = "LANGS")]
        foreign_langs: Option<String>,
        /// Accept all defaults without prompting
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// 🔍 Detect language ecosystems in your project
    ///
    /// Scans the workspace and scores each ecosystem using lockfiles,
    /// manifests, markers, and directory indicators. The highest-scoring
    /// package manager per language wins.
    #[command(visible_alias = "d")]
    Detect {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// 📦 Install dependencies across all detected ecosystems
    ///
    /// Runs the native install command for every detected ecosystem
    /// (e.g. `pnpm install`, `uv sync`, `cargo build`).
    #[command(visible_alias = "is")]
    Install {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
        /// Run ecosystem installs in parallel
        #[arg(short = 'p', long)]
        parallel: bool,
        /// Filter ecosystems by language or adapter name (e.g. python, node)
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },

    /// ➕ Add a foreign package dependency
    ///
    /// Format: ecosystem:package (e.g. pip:requests, npm:express, cargo:serde).
    /// If no ecosystem prefix is given, defaults to npm.
    ///
    /// Examples:
    ///   upm add pip:requests
    ///   upm add npm:express
    ///   upm add cargo:serde
    #[command(visible_alias = "a")]
    Add {
        /// Package identifier (ecosystem:package)
        package: String,
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// ➖ Remove a foreign package dependency
    ///
    /// Format: ecosystem:package (e.g. pip:requests, npm:express).
    #[command(visible_alias = "rm")]
    Remove {
        /// Package identifier (ecosystem:package)
        package: String,
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// 🔄 Update dependencies across all detected ecosystems
    #[command(visible_alias = "up")]
    Update {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
        /// Run ecosystem updates in parallel
        #[arg(short = 'p', long)]
        parallel: bool,
        /// Filter ecosystems by language or adapter name
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },

    /// 📋 List outdated dependencies across ecosystems
    Outdated {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Filter ecosystems by language or adapter name
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },

    /// 🛡️ Audit security vulnerabilities across ecosystems
    Audit {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Filter ecosystems by language or adapter name
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },

    /// ▶️ Run a script across detected ecosystems
    Run {
        /// Script name to execute
        script: String,
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// 📊 Show project status and ecosystem overview
    Status {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// ⚡ Automated 1-command repository migration
    #[command(visible_alias = "m")]
    Migrate {
        /// Workspace directory path
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// 🌉 Cross-language bridge: call foreign methods or view transport info
    Bridge {
        #[command(subcommand)]
        sub: BridgeSubcommands,
    },
}

#[derive(Subcommand)]
enum BridgeSubcommands {
    /// Call a foreign language method over stdio RPC
    ///
    /// Examples:
    ///   upm bridge call python:math.sqrt '[144.0]'
    ///   upm bridge call node:sharp.resize '["photo.jpg", 800, 600]'
    Call {
        /// Target: language:method (e.g. python:math.sqrt)
        target: String,
        /// Arguments as a JSON array string
        args_json: Option<String>,
    },
    /// Dynamically inspect registered RPC methods on a foreign language host
    Inspect {
        /// Language host to inspect (e.g. python, node)
        language: String,
    },
    /// Show active transport tiers and language host status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            path,
            name,
            base_lang,
            foreign_langs,
            yes,
        } => execute_init(&path, name, base_lang, foreign_langs, yes)?,
        Commands::Detect { path } => execute_detect(&path)?,
        Commands::Install { path, dry_run, parallel, filter } => execute_install(&path, dry_run, parallel, filter.as_deref())?,
        Commands::Add { package, path, dry_run } => execute_add(&path, &package, dry_run)?,
        Commands::Remove { package, path, dry_run } => execute_remove(&path, &package, dry_run)?,
        Commands::Update { path, dry_run, parallel, filter } => execute_update(&path, dry_run, parallel, filter.as_deref())?,
        Commands::Outdated { path, filter } => execute_outdated(&path, filter.as_deref())?,
        Commands::Audit { path, filter } => execute_audit(&path, filter.as_deref())?,
        Commands::Run { script, path } => execute_run(&path, &script)?,
        Commands::Status { path } => execute_status(&path)?,
        Commands::Migrate { path } => execute_migrate(&path)?,
        Commands::Bridge { sub } => match sub {
            BridgeSubcommands::Call { target, args_json } => {
                execute_bridge_call(&target, args_json.as_deref()).await?;
            }
            BridgeSubcommands::Inspect { language } => {
                execute_bridge_inspect(&language).await?;
            }
            BridgeSubcommands::Status => execute_bridge_status()?,
        },
    }

    Ok(())
}
