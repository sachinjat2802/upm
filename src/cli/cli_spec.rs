/// # Shared CLI Specification
///
/// Defines the shared command-line structure, subcommands, and execution dispatcher
/// for both `upm` and `cpm` binaries.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::control;

use crate::cli::*;

/// ╭──────────────────────────────────────────────────────╮
/// │  CPM / UPM — Universal Package Platform              │
/// │  Write in one language. Depend on all of them.       │
/// ╰──────────────────────────────────────────────────────╯
#[derive(Parser)]
#[command(
    name = "cpm",
    about = "\n  ╭──────────────────────────────────────────────────────╮\n  │  CPM / UPM — Universal Package Platform              │\n  │  Write in one language. Depend on all of them.       │\n  ╰──────────────────────────────────────────────────────╯\n\n  A polyglot package manager that auto-detects and orchestrates\n  npm, pnpm, pip, uv, cargo, go, and 14 more ecosystems.\n\n  Quick start:\n    cpm init                    Create a new polyglot project\n    cpm detect                  Show detected ecosystems\n    cpm install                 Install all dependencies\n    cpm add pip:requests        Add a foreign dependency\n    cpm bridge call python:math.sqrt '[9]'",
    version = env!("CARGO_PKG_VERSION"),
    after_help = "  Aliases: Available as both `cpm` and `upm`.\n  Docs:    https://github.com/sachinjat2802/cpm\n"
)]
pub struct Cli {
    /// Format all command output as JSON (#2)
    #[arg(long, global = true)]
    pub json: bool,

    /// Enable verbose diagnostic logging (#3)
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output (#3)
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Disable colored terminal output (#4)
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // ── Core Package Operations ──────────────────────────────────────────────
    /// 🚀 Initialize a new polyglot project
    #[command(next_help_heading = "Core Commands", visible_alias = "i")]
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
    /// 🔍 Detect language ecosystems in your workspace
    #[command(next_help_heading = "Core Commands", visible_alias = "d")]
    Detect {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 📦 Install dependencies across all ecosystems
    #[command(next_help_heading = "Core Commands", visible_alias = "is")]
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
    #[command(next_help_heading = "Core Commands", visible_alias = "a")]
    Add {
        package: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// ➖ Remove a foreign package dependency
    #[command(next_help_heading = "Core Commands", visible_alias = "rm")]
    Remove {
        package: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// 🔄 Update dependencies across all ecosystems
    #[command(next_help_heading = "Core Commands", visible_alias = "up")]
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
    #[command(next_help_heading = "Core Commands")]
    Outdated {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// 🛡️ Audit security vulnerabilities
    #[command(next_help_heading = "Core Commands")]
    Audit {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// 🚀 Rapidly scaffold a production agentic AI or polyglot application
    #[command(next_help_heading = "Core Commands", visible_alias = "new")]
    Create {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 't', long, value_name = "TEMPLATE")]
        template: Option<String>,
        #[arg(short = 'n', long, value_name = "NAME")]
        name: Option<String>,
    },
    /// ▶️ Run a script across ecosystems or from upm.toml (Task Engine)
    #[command(next_help_heading = "Core Commands")]
    Run {
        script: Option<String>,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// 🔨 Execute local project binary or package tool
    #[command(next_help_heading = "Core Commands", visible_alias = "x")]
    Exec {
        tool: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// 🔒 Generate or verify deterministic cpm.lock lockfile (CAS Engine)
    #[command(next_help_heading = "Core Commands")]
    Lock {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'v', long)]
        verify: bool,
        #[arg(short = 'u', long)]
        update: bool,
    },
    /// 🔍 Scan source code to guess undeclared dependencies
    #[command(next_help_heading = "Core Commands")]
    Guess {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'a', long)]
        apply: bool,
    },
    /// ℹ️ Display package metadata and registry details
    #[command(next_help_heading = "Core Commands")]
    Info {
        package: String,
    },
    /// 🤖 Universal AI configuration & skills manager
    #[command(next_help_heading = "AI & Agentic Operations")]
    Ai {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
        #[arg(short = 't', long)]
        target: Option<String>,
        #[arg(short = 'r', long)]
        rule: Option<String>,
    },
    /// 🔌 Universal Model Context Protocol manager
    #[command(next_help_heading = "AI & Agentic Operations")]
    Mcp {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
        #[arg(short = 'n', long)]
        name: Option<String>,
        #[arg(short = 'c', long)]
        cmd: Option<String>,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// 🧠 Token-optimized AI agent context, LangGraph workflows, LangChain pipelines, OpenRouter models & nested teams
    #[command(next_help_heading = "AI & Agentic Operations")]
    Agent {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
    },
    /// 💾 High-capacity storage allocation & environment manager
    #[command(next_help_heading = "Diagnostics & Performance")]
    Drive {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
    },
    /// 📊 Show project status and ecosystem overview
    #[command(next_help_heading = "Core Commands")]
    Status {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// ⚡ Automated 1-command repository migration
    #[command(next_help_heading = "Core Commands", visible_alias = "m")]
    Migrate {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 📋 List installed dependencies across ecosystems (#64)
    #[command(next_help_heading = "Core Commands", visible_alias = "ls")]
    List {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },
    /// 🧹 Clean build artifacts across ecosystems (#18)
    #[command(next_help_heading = "Core Commands")]
    Clean {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// ❓ Explain why a dependency is included in the workspace (#16)
    #[command(next_help_heading = "Core Commands")]
    Why {
        package: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🚀 Interactive quickstart tutorial walkthrough (#36)
    #[command(next_help_heading = "Core Commands", visible_alias = "tutorial")]
    Quickstart {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🏷️ Print platform version information (#10)
    #[command(next_help_heading = "Core Commands")]
    Version,

    // ── Security, Governance & Auditing ─────────────────────────────────────
    /// 📜 Open-source license compliance audit
    #[command(next_help_heading = "Security & Governance")]
    Licenses {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🛡️ Secret & API key leak scanner
    #[command(next_help_heading = "Security & Governance")]
    ScanSecrets {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🛡️ Enforce enterprise RBAC security policies
    #[command(next_help_heading = "Security & Governance")]
    Policy {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🔏 Verify Sigstore / GPG cryptographic package signatures
    #[command(next_help_heading = "Security & Governance")]
    VerifySig {
        #[arg(default_value = ".")]
        path: PathBuf,
        package: Option<String>,
    },
    /// 📑 View immutable cryptographic audit trail log
    #[command(next_help_heading = "Security & Governance")]
    AuditLog {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    // ── Bridge & Cross-Language Operations ─────────────────────────────────
    /// 🌉 Cross-language bridge RPC calls and status
    #[command(next_help_heading = "Bridge Protocol")]
    Bridge {
        #[command(subcommand)]
        sub: BridgeSubcommands,
    },
    /// 💬 Interactive Polyglot REPL Shell
    #[command(next_help_heading = "Bridge Protocol", visible_alias = "r")]
    Repl,
    /// 📝 Generate IDE type definitions (.d.ts & .pyi)
    #[command(next_help_heading = "Bridge Protocol")]
    GenerateStubs {
        #[arg(default_value = "./sdk")]
        out_dir: PathBuf,
    },

    // ── Performance, Diagnostics & Profiling ───────────────────────────────
    /// 🩺 Self-healing environment diagnostics & repair
    #[command(next_help_heading = "Diagnostics & Performance", visible_alias = "doc")]
    Doctor {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        fix: bool,
    },
    /// 🔥 Cross-language call stack flamegraph visualizer
    #[command(next_help_heading = "Diagnostics & Performance")]
    Flamegraph {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// 🔭 Export OpenTelemetry OTLP distributed trace spans
    #[command(next_help_heading = "Diagnostics & Performance")]
    Trace {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// ⚡ Configure sccache / ccache compiler build cache
    #[command(next_help_heading = "Diagnostics & Performance")]
    Sccache {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🗄️ Manage global content-addressable package cache
    #[command(next_help_heading = "Diagnostics & Performance")]
    Cache {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
    },
    /// 📜 Cross-host distributed log aggregator
    #[command(next_help_heading = "Diagnostics & Performance")]
    Logs {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        json: bool,
    },

    // ── Advanced Tooling & Utilities ───────────────────────────────────────
    /// 📦 Export an air-gapped offline workspace bundle
    #[command(next_help_heading = "Advanced Utilities")]
    Bundle {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// 🏷️ Manage custom CLI command aliases
    #[command(next_help_heading = "Advanced Utilities")]
    Alias {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
        name: Option<String>,
        command: Option<String>,
    },
    /// 🔍 Interactive package search across registries
    #[command(next_help_heading = "Advanced Utilities", visible_alias = "s")]
    Search {
        query: String,
    },
    /// ⏪ Restore previous workspace manifest/lockfile snapshot
    #[command(next_help_heading = "Advanced Utilities")]
    Rollback {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 📊 Universal dependency diff viewer
    #[command(next_help_heading = "Advanced Utilities")]
    Diff {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🧠 Transitive dependency conflict resolution advisor
    #[command(next_help_heading = "Advanced Utilities")]
    Resolve {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 💰 Cloud cost & memory resource footprint estimator
    #[command(next_help_heading = "Advanced Utilities")]
    Cost {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// 🕸️ Cross-platform polyglot dependency graph web visualizer
    #[command(next_help_heading = "Advanced Utilities")]
    Graph {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// 🐚 Auto-generate shell tab completion scripts
    #[command(next_help_heading = "Advanced Utilities")]
    Completion {
        #[arg(default_value = ".")]
        path: PathBuf,
        shell: Option<String>,
    },

    // ── DevOps & Cloud Native ──────────────────────────────────────────────
    /// 🐳 Auto-generate polyglot Dockerfile
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Dockerfile {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// ☸️ Auto-generate Kubernetes Helm chart manifests
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Helm {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// ☸️ Auto-generate Kubernetes Operator CRD manifest
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Operator {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
    /// 🧪 Run test suites across all detected ecosystems (Multi-Ecosystem Test Runner)
    #[command(next_help_heading = "Core Commands", visible_alias = "t")]
    Test {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'f', long)]
        filter: Option<String>,
        #[arg(short = 'w', long)]
        watch: bool,
        #[arg(short = 'c', long)]
        coverage: bool,
    },
    /// 🔨 Build projects and bundles across all detected ecosystems
    #[command(next_help_heading = "Core Commands", visible_alias = "b")]
    Build {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'r', long)]
        release: bool,
        #[arg(short = 't', long)]
        target: Option<String>,
    },
    /// 🚀 Deploy application to Docker, Vercel, Netlify, Cloud Run, or AWS
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Deploy {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'p', long)]
        provider: Option<String>,
        #[arg(short = 'e', long)]
        env: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// 🔑 Manage multi-environment variables and secrets (.env)
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Env {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
    },
    /// 🏢 Polyglot monorepo & multi-package workspace orchestrator
    #[command(next_help_heading = "Core Commands", visible_alias = "ws")]
    Workspace {
        #[arg(default_value = ".")]
        path: PathBuf,
        action: Option<String>,
        command: Option<String>,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// 📦 Release & publish packages to npm, Crates.io, or PyPI
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Publish {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
        #[arg(short = 'b', long)]
        bump: Option<String>,
        #[arg(short = 't', long)]
        tag: Option<String>,
    },
    /// 🔌 Manage community plugins and extensions
    #[command(next_help_heading = "Advanced Utilities")]
    Plugin {
        action: Option<String>,
        name: Option<String>,
    },
    /// ⚙️ Manage global CPM user configuration (D:\.cpm\config.toml)
    #[command(next_help_heading = "Advanced Utilities")]
    Config {
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
    },
    /// 🚀 Self-update CPM to latest binary release
    #[command(next_help_heading = "Core Commands")]
    Upgrade {
        #[arg(short = 'c', long)]
        check: bool,
    },
    /// 🤖 Auto-generate production CI/CD workflows (GitHub, GitLab, CircleCI)
    #[command(next_help_heading = "DevOps & Cloud Native")]
    Ci {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'p', long)]
        provider: Option<String>,
        #[arg(short = 'o', long)]
        out: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BridgeSubcommands {
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

/// Dispatcher for executing CLI commands.
pub async fn run_cli_app(_bin_name: &str) -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        control::set_override(false);
    }

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
        Commands::Create { path, template, name } => execute_create(&path, template.as_deref(), name.as_deref())?,
        Commands::Run { script, path, args } => execute_run_script(&path, script.as_deref(), &args)?,
        Commands::Exec { tool, path, args } => execute_tool(&path, &tool, &args)?,
        Commands::Lock { path, verify, update } => execute_lock(&path, verify, update)?,
        Commands::Guess { path, apply } => execute_guess(&path, apply)?,
        Commands::Info { package } => execute_info(&package)?,
        Commands::Ai { path, action, target, rule } => execute_ai(&path, action.as_deref(), target.as_deref(), rule.as_deref())?,
        Commands::Mcp { path, action, name, cmd, args } => execute_mcp(&path, action.as_deref(), name.as_deref(), cmd.as_deref(), &args)?,
        Commands::Agent { path, action } => execute_agent(&path, action.as_deref(), cli.json)?,
        Commands::Drive { path, action } => execute_drive(&path, action.as_deref())?,
        Commands::Status { path } => execute_status(&path)?,
        Commands::Migrate { path } => execute_migrate(&path)?,
        Commands::List { path, filter } => execute_list(&path, filter.as_deref())?,
        Commands::Clean { path, dry_run } => execute_clean(&path, dry_run)?,
        Commands::Why { package, path } => execute_why(&path, &package)?,
        Commands::Quickstart { path } => execute_quickstart(&path)?,
        Commands::Version => {
            println!("cpm / upm version {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Doctor { path, fix } => execute_doctor(&path, fix)?,
        Commands::GenerateStubs { out_dir } => execute_generate_stubs(&out_dir).await?,
        Commands::Bundle { path, out } => execute_bundle(&path, out.as_deref())?,
        Commands::Repl => execute_repl().await?,
        Commands::Alias { path, action, name, command } => execute_alias(&path, action.as_deref(), name.as_deref(), command.as_deref())?,
        Commands::Dockerfile { path, out } => execute_dockerfile(&path, out.as_deref())?,
        Commands::Search { query } => execute_search(&query)?,
        Commands::Rollback { path } => execute_rollback(&path)?,
        Commands::Licenses { path } => execute_licenses(&path)?,
        Commands::ScanSecrets { path } => execute_scan_secrets(&path)?,
        Commands::Diff { path } => execute_diff(&path)?,
        Commands::Resolve { path } => execute_resolve(&path)?,
        Commands::Cost { path } => execute_cost(&path)?,
        Commands::Helm { path, out } => execute_helm(&path, out.as_deref())?,
        Commands::Policy { path } => execute_policy(&path)?,
        Commands::Flamegraph { path, out } => execute_flamegraph(&path, out.as_deref())?,
        Commands::Operator { path, out } => execute_operator(&path, out.as_deref())?,
        Commands::Cache { path, action } => execute_cache(&path, action.as_deref())?,
        Commands::Logs { path, json } => execute_logs(&path, json)?,
        Commands::Sccache { path } => execute_sccache(&path)?,
        Commands::VerifySig { path, package } => execute_verify_sig(&path, package.as_deref())?,
        Commands::AuditLog { path } => execute_audit_log(&path)?,
        Commands::Trace { path, out } => execute_trace(&path, out.as_deref())?,
        Commands::Graph { path, out } => execute_graph(&path, out.as_deref())?,
        Commands::Completion { path, shell } => execute_completion(&path, shell.as_deref())?,
        Commands::Test { path, filter, watch, coverage } => execute_test(&path, filter.as_deref(), watch, coverage)?,
        Commands::Build { path, release, target } => execute_build(&path, release, target.as_deref())?,
        Commands::Deploy { path, provider, env, dry_run } => execute_deploy(&path, provider.as_deref(), env.as_deref(), dry_run)?,
        Commands::Env { path, action, key, value } => execute_env(&path, action.as_deref(), key.as_deref(), value.as_deref())?,
        Commands::Workspace { path, action, command, args } => execute_workspace(&path, action.as_deref(), command.as_deref(), &args)?,
        Commands::Publish { path, dry_run, bump, tag } => execute_publish(&path, dry_run, bump.as_deref(), tag.as_deref())?,
        Commands::Plugin { action, name } => execute_plugin(action.as_deref(), name.as_deref())?,
        Commands::Config { action, key, value } => execute_config(action.as_deref(), key.as_deref(), value.as_deref())?,
        Commands::Upgrade { check } => execute_upgrade(check)?,
        Commands::Ci { path, provider, out } => execute_ci(&path, provider.as_deref(), out.as_deref())?,
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
