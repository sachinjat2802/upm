/// # Ecosystem Adapter
///
/// An `EcosystemAdapter` describes a single package manager and its
/// associated CLI commands. The [`AdapterRegistry`] loads builtin adapters
/// for 15+ ecosystems and allows user-defined extensions.
///
/// ## Scoring fields
///
/// | Field                | Purpose                                         |
/// |----------------------|-------------------------------------------------|
/// | `manifest_file`      | Primary manifest filename (e.g. `package.json`)  |
/// | `lockfile_file`      | Lockfile filename (+100 pts if present)           |
/// | `manifest_marker`    | String marker inside manifest (+80 pts)           |
/// | `glob_pattern`       | Extra file glob (+30 pts)                         |
/// | `directory_indicator` | Directory presence indicator (+20 pts)           |
/// | `default_priority`   | Tie-break priority (0–4 pts)                     |

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a package manager ecosystem and its CLI commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemAdapter {
    /// Internal identifier (e.g. `"pnpm"`, `"cargo"`).
    pub name: String,
    /// Language group for scoring (e.g. `"javascript"`, `"python"`).
    pub language: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Primary manifest filename.
    pub manifest_file: String,
    /// Lockfile filename (if applicable).
    pub lockfile_file: Option<String>,
    /// Marker string expected inside the manifest.
    pub manifest_marker: Option<String>,
    /// Extra glob pattern for additional file detection.
    pub glob_pattern: Option<String>,
    /// Directory whose presence indicates this ecosystem.
    pub directory_indicator: Option<String>,
    /// Tie-break priority (higher = preferred).
    pub default_priority: u8,
    /// CLI command to install all dependencies.
    pub install_cmd: Vec<String>,
    /// CLI command to add a single dependency.
    pub add_cmd: Vec<String>,
    /// CLI command to update dependencies.
    pub update_cmd: Vec<String>,
    /// CLI command to list outdated dependencies.
    pub outdated_cmd: Vec<String>,
    /// CLI command to run a security audit.
    pub audit_cmd: Vec<String>,
    /// CLI command to run a named script.
    pub run_cmd: Vec<String>,
}

/// Registry of all known ecosystem adapters.
///
/// Created via [`AdapterRegistry::new()`] which auto-loads 15+ builtin
/// adapters covering JavaScript, Python, Rust, Go, Java, PHP, Ruby,
/// C#, Dart, and Elixir ecosystems.
pub struct AdapterRegistry {
    adapters: HashMap<String, EcosystemAdapter>,
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterRegistry {
    /// Create a new registry pre-loaded with all builtin adapters.
    pub fn new() -> Self {
        let mut registry = Self {
            adapters: HashMap::new(),
        };
        registry.load_builtin_adapters();
        registry
    }

    /// Register a custom adapter.
    pub fn register(&mut self, adapter: EcosystemAdapter) {
        self.adapters.insert(adapter.name.clone(), adapter);
    }

    /// Look up an adapter by name (e.g. `"npm"`, `"cargo"`).
    pub fn get(&self, name: &str) -> Option<&EcosystemAdapter> {
        self.adapters.get(name)
    }

    /// Return all registered adapters.
    pub fn all(&self) -> Vec<&EcosystemAdapter> {
        self.adapters.values().collect()
    }

    fn load_builtin_adapters(&mut self) {
        let builtins = vec![
            // ── JavaScript / TypeScript ───────────────────────────────────
            EcosystemAdapter {
                name: "pnpm".into(),
                language: "javascript".into(),
                display_name: "pnpm (Node.js)".into(),
                manifest_file: "package.json".into(),
                lockfile_file: Some("pnpm-lock.yaml".into()),
                manifest_marker: Some("pnpm".into()),
                glob_pattern: None,
                directory_indicator: Some("node_modules".into()),
                default_priority: 4,
                install_cmd: vec!["pnpm".into(), "install".into()],
                add_cmd: vec!["pnpm".into(), "add".into()],
                update_cmd: vec!["pnpm".into(), "update".into()],
                outdated_cmd: vec!["pnpm".into(), "outdated".into()],
                audit_cmd: vec!["pnpm".into(), "audit".into()],
                run_cmd: vec!["pnpm".into(), "run".into()],
            },
            EcosystemAdapter {
                name: "npm".into(),
                language: "javascript".into(),
                display_name: "npm (Node.js)".into(),
                manifest_file: "package.json".into(),
                lockfile_file: Some("package-lock.json".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some("node_modules".into()),
                default_priority: 1,
                install_cmd: vec!["npm".into(), "install".into()],
                add_cmd: vec!["npm".into(), "install".into()],
                update_cmd: vec!["npm".into(), "update".into()],
                outdated_cmd: vec!["npm".into(), "outdated".into()],
                audit_cmd: vec!["npm".into(), "audit".into()],
                run_cmd: vec!["npm".into(), "run".into()],
            },
            EcosystemAdapter {
                name: "yarn".into(),
                language: "javascript".into(),
                display_name: "yarn (Node.js)".into(),
                manifest_file: "package.json".into(),
                lockfile_file: Some("yarn.lock".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some("node_modules".into()),
                default_priority: 3,
                install_cmd: vec!["yarn".into(), "install".into()],
                add_cmd: vec!["yarn".into(), "add".into()],
                update_cmd: vec!["yarn".into(), "upgrade".into()],
                outdated_cmd: vec!["yarn".into(), "outdated".into()],
                audit_cmd: vec!["yarn".into(), "audit".into()],
                run_cmd: vec!["yarn".into(), "run".into()],
            },
            EcosystemAdapter {
                name: "bun".into(),
                language: "javascript".into(),
                display_name: "bun (JavaScript/TS)".into(),
                manifest_file: "package.json".into(),
                lockfile_file: Some("bun.lockb".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some("node_modules".into()),
                default_priority: 2,
                install_cmd: vec!["bun".into(), "install".into()],
                add_cmd: vec!["bun".into(), "add".into()],
                update_cmd: vec!["bun".into(), "update".into()],
                outdated_cmd: vec!["bun".into(), "outdated".into()],
                audit_cmd: vec!["bun".into(), "pm".into(), "audit".into()],
                run_cmd: vec!["bun".into(), "run".into()],
            },

            // ── Python ────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "uv".into(),
                language: "python".into(),
                display_name: "uv (Python)".into(),
                manifest_file: "pyproject.toml".into(),
                lockfile_file: Some("uv.lock".into()),
                manifest_marker: Some("[tool.uv]".into()),
                glob_pattern: None,
                directory_indicator: Some(".venv".into()),
                default_priority: 4,
                install_cmd: vec!["uv".into(), "sync".into()],
                add_cmd: vec!["uv".into(), "add".into()],
                update_cmd: vec!["uv".into(), "lock".into(), "--upgrade".into()],
                outdated_cmd: vec!["uv".into(), "tree".into()],
                audit_cmd: vec!["uv".into(), "pip".into(), "audit".into()],
                run_cmd: vec!["uv".into(), "run".into()],
            },
            EcosystemAdapter {
                name: "poetry".into(),
                language: "python".into(),
                display_name: "Poetry (Python)".into(),
                manifest_file: "pyproject.toml".into(),
                lockfile_file: Some("poetry.lock".into()),
                manifest_marker: Some("[tool.poetry]".into()),
                glob_pattern: None,
                directory_indicator: Some(".venv".into()),
                default_priority: 3,
                install_cmd: vec!["poetry".into(), "install".into()],
                add_cmd: vec!["poetry".into(), "add".into()],
                update_cmd: vec!["poetry".into(), "update".into()],
                outdated_cmd: vec!["poetry".into(), "show".into(), "--outdated".into()],
                audit_cmd: vec!["poetry".into(), "run".into(), "pip-audit".into()],
                run_cmd: vec!["poetry".into(), "run".into()],
            },
            EcosystemAdapter {
                name: "pip".into(),
                language: "python".into(),
                display_name: "pip (Python)".into(),
                manifest_file: "requirements.txt".into(),
                lockfile_file: Some("requirements.lock".into()),
                manifest_marker: None,
                glob_pattern: Some("requirements*.txt".into()),
                directory_indicator: Some(".venv".into()),
                default_priority: 1,
                install_cmd: vec!["pip".into(), "install".into(), "-r".into(), "requirements.txt".into()],
                add_cmd: vec!["pip".into(), "install".into()],
                update_cmd: vec!["pip".into(), "install".into(), "--upgrade".into()],
                outdated_cmd: vec!["pip".into(), "list".into(), "--outdated".into()],
                audit_cmd: vec!["pip-audit".into()],
                run_cmd: vec!["python".into()],
            },

            // ── Rust ──────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "cargo".into(),
                language: "rust".into(),
                display_name: "cargo (Rust)".into(),
                manifest_file: "Cargo.toml".into(),
                lockfile_file: Some("Cargo.lock".into()),
                manifest_marker: Some("[package]".into()),
                glob_pattern: None,
                directory_indicator: Some("target".into()),
                default_priority: 4,
                install_cmd: vec!["cargo".into(), "build".into()],
                add_cmd: vec!["cargo".into(), "add".into()],
                update_cmd: vec!["cargo".into(), "update".into()],
                outdated_cmd: vec!["cargo".into(), "outdated".into()],
                audit_cmd: vec!["cargo".into(), "audit".into()],
                run_cmd: vec!["cargo".into(), "run".into(), "--bin".into()],
            },

            // ── Go ────────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "go".into(),
                language: "go".into(),
                display_name: "go mod (Go)".into(),
                manifest_file: "go.mod".into(),
                lockfile_file: Some("go.sum".into()),
                manifest_marker: Some("module".into()),
                glob_pattern: None,
                directory_indicator: Some("vendor".into()),
                default_priority: 4,
                install_cmd: vec!["go".into(), "mod".into(), "download".into()],
                add_cmd: vec!["go".into(), "get".into()],
                update_cmd: vec!["go".into(), "get".into(), "-u".into()],
                outdated_cmd: vec!["go".into(), "list".into(), "-m".into(), "-u".into(), "all".into()],
                audit_cmd: vec!["govulncheck".into(), "./...".into()],
                run_cmd: vec!["go".into(), "run".into()],
            },

            // ── JVM ───────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "maven".into(),
                language: "java".into(),
                display_name: "Maven (Java)".into(),
                manifest_file: "pom.xml".into(),
                lockfile_file: None,
                manifest_marker: Some("<project".into()),
                glob_pattern: None,
                directory_indicator: Some("target".into()),
                default_priority: 2,
                install_cmd: vec!["mvn".into(), "dependency:resolve".into()],
                add_cmd: vec!["mvn".into(), "dependency:get".into()],
                update_cmd: vec!["mvn".into(), "versions:use-latest-versions".into()],
                outdated_cmd: vec!["mvn".into(), "versions:display-dependency-updates".into()],
                audit_cmd: vec!["mvn".into(), "org.owasp:dependency-check-maven:check".into()],
                run_cmd: vec!["mvn".into(), "exec:java".into()],
            },
            EcosystemAdapter {
                name: "gradle".into(),
                language: "java".into(),
                display_name: "Gradle (Java/Kotlin)".into(),
                manifest_file: "build.gradle".into(),
                lockfile_file: Some("gradle.lockfile".into()),
                manifest_marker: None,
                glob_pattern: Some("build.gradle*".into()),
                directory_indicator: Some(".gradle".into()),
                default_priority: 3,
                install_cmd: vec!["gradle".into(), "build".into()],
                add_cmd: vec!["gradle".into(), "addDependency".into()],
                update_cmd: vec!["gradle".into(), "useLatestVersions".into()],
                outdated_cmd: vec!["gradle".into(), "dependencyUpdates".into()],
                audit_cmd: vec!["gradle".into(), "check".into()],
                run_cmd: vec!["gradle".into(), "run".into()],
            },

            // ── PHP ───────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "composer".into(),
                language: "php".into(),
                display_name: "Composer (PHP)".into(),
                manifest_file: "composer.json".into(),
                lockfile_file: Some("composer.lock".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some("vendor".into()),
                default_priority: 4,
                install_cmd: vec!["composer".into(), "install".into()],
                add_cmd: vec!["composer".into(), "require".into()],
                update_cmd: vec!["composer".into(), "update".into()],
                outdated_cmd: vec!["composer".into(), "outdated".into()],
                audit_cmd: vec!["composer".into(), "audit".into()],
                run_cmd: vec!["composer".into(), "run-script".into()],
            },

            // ── Ruby ──────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "bundler".into(),
                language: "ruby".into(),
                display_name: "Bundler (Ruby)".into(),
                manifest_file: "Gemfile".into(),
                lockfile_file: Some("Gemfile.lock".into()),
                manifest_marker: None,
                glob_pattern: Some("*.gemspec".into()),
                directory_indicator: Some("vendor/bundle".into()),
                default_priority: 4,
                install_cmd: vec!["bundle".into(), "install".into()],
                add_cmd: vec!["bundle".into(), "add".into()],
                update_cmd: vec!["bundle".into(), "update".into()],
                outdated_cmd: vec!["bundle".into(), "outdated".into()],
                audit_cmd: vec!["bundle".into(), "exec".into(), "bundle-audit".into()],
                run_cmd: vec!["bundle".into(), "exec".into()],
            },

            // ── .NET ──────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "nuget".into(),
                language: "csharp".into(),
                display_name: "NuGet (.NET)".into(),
                manifest_file: "packages.config".into(),
                lockfile_file: Some("packages.lock.json".into()),
                manifest_marker: None,
                glob_pattern: Some("*.csproj".into()),
                directory_indicator: Some("obj".into()),
                default_priority: 4,
                install_cmd: vec!["dotnet".into(), "restore".into()],
                add_cmd: vec!["dotnet".into(), "add".into(), "package".into()],
                update_cmd: vec!["dotnet".into(), "restore".into()],
                outdated_cmd: vec!["dotnet".into(), "list".into(), "package".into(), "--outdated".into()],
                audit_cmd: vec!["dotnet".into(), "list".into(), "package".into(), "--vulnerable".into()],
                run_cmd: vec!["dotnet".into(), "run".into()],
            },

            // ── Dart / Flutter ────────────────────────────────────────────
            EcosystemAdapter {
                name: "pub".into(),
                language: "dart".into(),
                display_name: "pub (Dart/Flutter)".into(),
                manifest_file: "pubspec.yaml".into(),
                lockfile_file: Some("pubspec.lock".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some(".dart_tool".into()),
                default_priority: 4,
                install_cmd: vec!["dart".into(), "pub".into(), "get".into()],
                add_cmd: vec!["dart".into(), "pub".into(), "add".into()],
                update_cmd: vec!["dart".into(), "pub".into(), "upgrade".into()],
                outdated_cmd: vec!["dart".into(), "pub".into(), "outdated".into()],
                audit_cmd: vec!["dart".into(), "pub".into(), "deps".into()],
                run_cmd: vec!["dart".into(), "run".into()],
            },

            // ── Elixir ────────────────────────────────────────────────────
            EcosystemAdapter {
                name: "mix".into(),
                language: "elixir".into(),
                display_name: "Mix (Elixir)".into(),
                manifest_file: "mix.exs".into(),
                lockfile_file: Some("mix.lock".into()),
                manifest_marker: None,
                glob_pattern: None,
                directory_indicator: Some("_build".into()),
                default_priority: 4,
                install_cmd: vec!["mix".into(), "deps.get".into()],
                add_cmd: vec!["mix".into(), "deps.get".into()],
                update_cmd: vec!["mix".into(), "deps.update".into(), "--all".into()],
                outdated_cmd: vec!["mix".into(), "hex.outdated".into()],
                audit_cmd: vec!["mix".into(), "hex.audit".into()],
                run_cmd: vec!["mix".into(), "run".into()],
            },
        ];

        for adapter in builtins {
            self.register(adapter);
        }
    }
}
