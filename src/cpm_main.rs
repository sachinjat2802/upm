use clap::{Parser, Subcommand};
use std::path::PathBuf;
use upm::cli::*;

/// CPM — Cross-language Package Manager (alias for UPM)
#[derive(Parser)]
#[command(
    name = "cpm",
    about = "\n  ╭──────────────────────────────────────────────────────╮\n  │  CPM — Cross-language Package Manager                │\n  │  Write in one language. Depend on all of them.       │\n  ╰──────────────────────────────────────────────────────╯\n\n  A polyglot package manager that auto-detects and orchestrates\n  npm, pnpm, pip, uv, cargo, go, and 14 more ecosystems.\n\n  Quick start:\n    cpm init                    Create a new polyglot project\n    cpm detect                  Show detected ecosystems\n    cpm install                 Install all dependencies\n    cpm add pip:requests        Add a foreign dependency\n    cpm bridge call python:math.sqrt '[9]'",
    version = env!("CARGO_PKG_VERSION"),
    after_help = "  Aliases: This binary is also available as `upm`.\n  Docs:    https://github.com/upm-org/upm\n"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🚀 Initialize a new polyglot project
    #[command(visible_alias = "i")]
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(short = 'l', long, value_name = "LANG")]
        base_lang: Option<String>,
        #[arg(short = 'f', long, value_name = "LANGS")]
        foreign_langs: Option<String>,
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// 🔍 Detect language ecosystems
    #[command(visible_alias = "d")]
    Detect {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 📦 Install dependencies across all ecosystems
    #[command(visible_alias = "is")]
    Install {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
        #[arg(short = 'p', long)]
        parallel: bool,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// ➕ Add a foreign package dependency
    #[command(visible_alias = "a")]
    Add {
        package: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// ➖ Remove a foreign package dependency
    #[command(visible_alias = "rm")]
    Remove {
        package: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// 🔄 Update dependencies across all ecosystems
    #[command(visible_alias = "up")]
    Update {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
        #[arg(short = 'p', long)]
        parallel: bool,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// 📋 List outdated dependencies
    Outdated {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// 🛡️ Audit security vulnerabilities
    Audit {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// ▶️ Run a script across ecosystems
    Run {
        script: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 📊 Show project status and ecosystem overview
    Status {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// ⚡ Automated 1-command repository migration
    #[command(visible_alias = "m")]
    Migrate {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🩺 Self-healing environment diagnostics & repair
    #[command(visible_alias = "doc")]
    Doctor {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        fix: bool,
    },
    /// 📝 Generate IDE type definitions (.d.ts & .pyi)
    GenerateStubs {
        #[arg(default_value = "./sdk")]
        out_dir: PathBuf,
    },
    /// 🌉 Cross-language bridge
    Bridge {
        #[command(subcommand)]
        sub: BridgeSubcommands,
    },
}

#[derive(Subcommand)]
enum BridgeSubcommands {
    /// Call a foreign language method
    Call {
        target: String,
        args_json: Option<String>,
    },
    /// Dynamically inspect registered RPC methods on a foreign language host
    Inspect {
        language: String,
    },
    /// Microsecond performance profiling benchmark
    Benchmark {
        #[arg(default_value = "python")]
        language: String,
        #[arg(default_value_t = 100)]
        iterations: usize,
    },
    /// Show transport tiers and host status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path, name, base_lang, foreign_langs, yes } =>
            execute_init(&path, name, base_lang, foreign_langs, yes)?,
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
        Commands::Doctor { path, fix } => execute_doctor(&path, fix)?,
        Commands::GenerateStubs { out_dir } => execute_generate_stubs(&out_dir).await?,
        Commands::Bridge { sub } => match sub {
            BridgeSubcommands::Call { target, args_json } => {
                execute_bridge_call(&target, args_json.as_deref()).await?;
            }
            BridgeSubcommands::Inspect { language } => {
                execute_bridge_inspect(&language).await?;
            }
            BridgeSubcommands::Benchmark { language, iterations } => {
                execute_benchmark(&language, iterations).await?;
            }
            BridgeSubcommands::Status => {
                execute_bridge_status()?;
            }
        },
    }

    Ok(())
}
