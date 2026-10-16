# Skill: CPM / UPM Polyglot Repository Guide

> **Authoritative guide for developers and AI agents working on the CPM (Cross-language Package Manager) codebase.**

---

## 1. Quick Overview

CPM (Universal Package Platform / UPM) is a Rust-based polyglot package manager and cross-language runtime bridge based on the **Universal Package Platform Concept & Architecture** specification.

### Two Core Engine Layers:
1. **L2 Universal Acquisition Engine (`src/acquisition/`)**:
   - Multi-signal weighted detection scoring across 15+ package managers and 10+ languages (+100 lockfile, +80 marker, +40 manifest, +30 glob, +20 dir, 0-4 priority).
   - Deterministic tie-breaker sorting and adapter iteration.
   - Unified dependency orchestration (`install`, `add`, `remove`, `update`, `outdated`, `audit`, `run`, `list`, `clean`, `why`).
   - `upm.toml` manifest management, automatic `.cpm_backup/` snapshots, and `.gitignore` generation.
2. **L2' Cross-Language Invocation Bridge (`src/bridge/`)**:
   - `upm-bridge/1` stdio RPC protocol with 4-byte Big-Endian uint32 length framing.
   - `UpmValue` codec supporting Base64 binary (`$blob`), opaque object handles (`$ref`), and callbacks (`$fn`).
   - Thread-safe object/callback handle GC registry (`HandleRegistry`) with Mutex poisoning recovery.
   - Language host supervisors (`python_host.py`, `node_host.js`) with `kill_on_drop` and `release_handles` support.

---

## 2. One-Command Setup & Verification

```bash
# Build the project (creates target/debug/cpm.exe and upm.exe)
cargo build

# Run full test suite (27 unit & integration tests)
cargo test

# Run CLI commands
cargo run --bin cpm -- status
cargo run --bin cpm -- detect
cargo run --bin cpm -- list
cargo run --bin cpm -- quickstart
cargo run --bin cpm -- bridge inspect python
cargo run --bin cpm -- bridge call python:math.sqrt '[144.0]'
```

---

## 3. Project Layout Map

```
d:/cpm/
├── Cargo.toml                 # Package manifest & dependencies
├── CHANGELOG.md               # Version 0.1.0 release changelog
├── README.md                  # Quick reference & architecture overview
├── SKILL.md                   # This developer & AI agent guide
├── install.sh                 # Linux/macOS installer with toolchain & health check
├── install.ps1                # Windows installer with toolchain & health check
├── src/
│   ├── lib.rs                 # Core library re-exports
│   ├── main.rs                # `upm` binary entry point (delegates to shared cli_spec)
│   ├── cpm_main.rs            # `cpm` binary entry point (delegates to shared cli_spec)
│   ├── acquisition/           # L2 Acquisition Engine
│   │   ├── adapter.rs         # 15+ ecosystem adapter definitions & remove_cmd
│   │   ├── scoring.rs         # Deterministic multi-signal weighted detection engine
│   │   ├── manifest.rs        # upm.toml, try_load_from_dir, & manifest scaffolding
│   │   └── runner.rs          # Native command runner (supports parallel & filter)
│   ├── bridge/                # L2' Invocation Bridge
│   │   ├── protocol.rs        # MessageEnvelope (Request/Response/Release/Ping/Pong)
│   │   ├── value.rs           # UpmValue codec ($blob, $ref, $fn)
│   │   ├── handles.rs         # HandleRegistry for object/callback GC (poison-safe)
│   │   ├── peer.rs            # Bidirectional BridgePeer RPC handler
│   │   ├── host.rs            # HostSupervisor (spawns language hosts with kill_on_drop)
│   │   └── transport/         # StdioRpcTransport (4-byte BE framing + 64MB guard)
│   └── cli/                   # CLI Specification & Handlers
│       ├── cli_spec.rs        # Shared Cli struct, Commands enum, & run_cli_app
│       ├── init_cmd.rs        # Scaffolding, auto-detect, & .gitignore generator
│       ├── detect_cmd.rs      # Scored detection report with winning rationale
│       ├── bridge_cmd.rs      # Bridge call, inspect, and status handlers
│       ├── secrets_cmd.rs     # Secret & API key leak scanner
│       ├── policy_cmd.rs      # RBAC security policy enforcement (.cpm_policy.json)
│       ├── licenses_cmd.rs    # Open-source license auditor (GPL/Copyleft risk check)
│       ├── diff_cmd.rs        # Lockfile & manifest dependency drift analyzer
│       ├── bundle_cmd.rs      # Offline air-gapped workspace bundler
│       ├── list_cmd.rs        # Universal dependency lister (cpm list / cpm ls)
│       ├── clean_cmd.rs       # Build artifact cleaner & space estimator
│       ├── why_cmd.rs         # Dependency reason explainer
│       ├── quickstart_cmd.rs  # Interactive polyglot tutorial walkthrough
│       └── rollback_cmd.rs   # Disaster recovery & dynamic restore snapshot scanner
├── hosts/
│   ├── python_host.py         # Python stdio RPC host (handles release_handles)
│   └── node_host.js           # Node.js stdio RPC host (handles release_handles)
└── tests/                     # 27 Unit & Integration Test Cases
```

---

## 4. Key CLI Commands Cheat Sheet

| Task | Command | Description |
|------|---------|-------------|
| **Quickstart** | `cpm quickstart` | Interactive step-by-step tutorial |
| **Init Project** | `cpm init` | Interactive project setup with `.gitignore` generation |
| **Init (Fast)** | `cpm init -l python -f node,rust -y` | Auto-detects base language in `-y` mode |
| **Detect Ecosystems** | `cpm detect` | Scored detection analysis with winning rationale |
| **List Dependencies** | `cpm list` (alias: `ls`) | List all dependencies across ecosystems & `upm.toml` |
| **Clean Artifacts** | `cpm clean` | Clean build artifacts (`node_modules/`, `.venv/`, `target/`) |
| **Why Dependency** | `cpm why <pkg>` | Explain why a package is included in manifest |
| **Install All** | `cpm install` (alias: `is`) | Install all detected ecosystems |
| **Add Dependency** | `cpm add pip:requests@2.31.0` | Supports `@version` syntax & auto-backup |
| **Remove Dependency** | `cpm remove pip:requests` | Removes dependency & backs up `upm.toml` |
| **Scan Secrets** | `cpm scan-secrets` | Scan files for API keys, AWS keys, & tokens |
| **RBAC Policy** | `cpm policy` | Verify compliance against `.cpm_policy.json` |
| **License Check** | `cpm licenses` | Audit open-source licenses for Copyleft/GPL risks |
| **Dependency Diff** | `cpm diff` | Compare lockfiles against manifest targets |
| **Offline Bundle** | `cpm bundle` | Package workspace assets for air-gapped deployment |
| **Rollback** | `cpm rollback` | Restore previous workspace snapshot state |
