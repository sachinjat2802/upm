# CPM / UPM — Universal Package Platform

> **Write in one language. Depend on all of them.**

CPM (Cross-language Package Manager) is a polyglot package manager that **auto-detects** and **orchestrates** 15+ package managers across 10+ language ecosystems from a single CLI.

```
╭──────────────────────────────────────────────────────╮
│  UPM — Universal Package Platform                    │
│  Write in one language. Depend on all of them.       │
╰──────────────────────────────────────────────────────╯
```

## Why CPM?

| Problem | CPM Solution |
|---------|-------------|
| Polyglot projects require learning N package managers | **One CLI** for all ecosystems |
| No cross-language dependency tracking | `upm.toml` unified manifest |
| Calling Python from Node requires manual setup | `cpm bridge call python:math.sqrt '[9]'` |
| Detecting which tools a repo uses is manual | Auto-scoring detection engine |

## Quick Start

```bash
# Initialize a polyglot project (interactive!)
cpm init

# Or non-interactive with flags
cpm init --base-lang python --foreign-langs node,rust -y

# Detect ecosystems in any repo
cpm detect

# Install everything across all ecosystems at once
cpm install

# Add a foreign dependency
cpm add pip:requests
cpm add npm:express
cpm add cargo:serde

# Cross-language RPC call
cpm bridge call python:math.sqrt '[144.0]'
```

## Supported Ecosystems

| Icon | Language | Package Managers |
|------|----------|-----------------|
| 📦 | JavaScript / TypeScript | npm, pnpm, yarn, bun |
| 🐍 | Python | pip, uv, poetry |
| 🦀 | Rust | cargo |
| 🐹 | Go | go mod |
| ☕ | Java / Kotlin | maven, gradle |
| 🐘 | PHP | composer |
| 💎 | Ruby | bundler |
| 🔷 | C# / .NET | nuget (dotnet) |
| 🎯 | Dart / Flutter | pub |
| 💧 | Elixir | mix |

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `cpm init` | `i` | 🚀 Initialize a polyglot project |
| `cpm detect` | `d` | 🔍 Auto-detect ecosystems with scored signals |
| `cpm install` | `is` | 📦 Install deps across all ecosystems (`--parallel`, `--filter`) |
| `cpm add eco:pkg` | `a` | ➕ Add a dependency (smart auto-detects prefix!) |
| `cpm remove eco:pkg` | `rm` | ➖ Remove a package dependency |
| `cpm update` | `up` | 🔄 Update all dependencies (`--parallel`, `--filter`) |
| `cpm outdated` | — | 📋 Show outdated deps (`--filter`) |
| `cpm audit` | — | 🛡️ Security audit across ecosystems (`--filter`) |
| `cpm run <script>` | — | ▶️ Run a script across ecosystems |
| `cpm status` | — | 📊 Project overview & ecosystem summary |
| `cpm bridge inspect` | — | 🔍 Dynamically inspect registered host RPC methods |
| `cpm bridge call` | — | 🌉 Cross-language RPC call |
| `cpm bridge status` | — | 🌉 Transport tier info |

## How Detection Works

CPM uses a **weighted scoring engine** to detect which package managers are active:

| Signal | Weight | Example |
|--------|--------|---------|
| Lockfile present | +100 | `pnpm-lock.yaml` |
| Manifest marker | +80 | `[tool.uv]` inside `pyproject.toml` |
| Manifest file | +40 | `package.json` exists |
| Glob match | +30 | `*.gemspec` files found |
| Directory indicator | +20 | `node_modules/` present |
| Priority tie-break | +0-4 | Adapter-defined |

The **highest-scoring** adapter per language group wins. Multiple languages coexist naturally in polyglot workspaces.

## Cross-Language Bridge

CPM includes `upm-bridge/1` — a stdio RPC protocol for calling foreign language functions:

```bash
# Call Python's math.sqrt from the terminal
cpm bridge call python:math.sqrt '[144.0]'
# → 12.0

# Hash data via Node.js
cpm bridge call node:crypto.sha256 '["hello world"]'

# Check transport tier status
cpm bridge status
```

### Wire Protocol

```
┌───────────────┬──────────────────────┐
│ 4 bytes BE u32│ JSON payload (UTF-8) │
│ (body length) │                      │
└───────────────┴──────────────────────┘
```

### Transport Tiers

| Tier | Mechanism | Latency |
|------|-----------|---------|
| **ffi** | dlopen + C ABI | ~0.88 µs |
| **embed** | CPython/V8 in-process | ~0.56-2.6 µs |
| **rpc** *(default)* | Framed JSON over stdio | ~156 µs |

## Architecture

```
upm/
├── src/
│   ├── acquisition/        # L2 — Detection scoring, manifest, runner
│   │   ├── adapter.rs      # 15+ ecosystem adapter definitions
│   │   ├── scoring.rs      # Weighted detection engine
│   │   ├── manifest.rs     # upm.toml + native manifest scaffolding
│   │   └── runner.rs       # Native command execution
│   ├── bridge/             # L2' — Cross-language invocation
│   │   ├── protocol.rs     # Wire envelope types (Request/Response/Ping/Release)
│   │   ├── transport/      # Framed stdio transport
│   │   ├── value.rs        # UpmValue codec ($blob, $ref, $fn)
│   │   ├── handles.rs      # Object/callback handle GC registry
│   │   ├── peer.rs         # Bidirectional RPC peer
│   │   └── host.rs         # Language host process supervisor
│   ├── cli/                # CLI subcommand handlers
│   │   ├── init_cmd.rs     # Interactive project initialization
│   │   ├── detect_cmd.rs   # Ecosystem detection display
│   │   └── bridge_cmd.rs   # Bridge RPC and status
│   ├── main.rs             # `upm` binary entry point
│   └── cpm_main.rs         # `cpm` binary entry point (same logic)
├── hosts/
│   ├── python_host.py      # Python language host (upm-bridge/1)
│   └── node_host.js        # Node.js language host (upm-bridge/1)
├── tests/
│   ├── test_detection.rs   # Detection + init integration tests
│   └── test_bridge_rpc.rs  # Bridge protocol + value codec tests
└── docs/
    ├── ARCHITECTURE.md     # Detailed design document
    └── BRIDGE_SPECIFICATION.md  # Protocol specification
```

## Building from Source

```bash
# Prerequisites: Rust toolchain (rustup.rs)
cargo build --release

# The binaries are at:
#   target/release/upm.exe
#   target/release/cpm.exe
```

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
