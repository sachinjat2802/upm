# Skill: CPM / UPM Polyglot Repository Guide

> **Authoritative guide for developers and AI agents working on the CPM (Cross-language Package Manager) codebase.**

---

## 1. Quick Overview

CPM (Universal Package Platform / UPM) is a Rust-based polyglot package manager and cross-language runtime bridge based on the **Universal Package Platform Concept & Architecture** specification.

### Two Core Engine Layers:
1. **L2 Universal Acquisition Engine (`src/acquisition/`)**:
   - Multi-signal weighted detection scoring across 15+ package managers and 10+ languages (+100 lockfile, +80 marker, +40 manifest, +30 glob, +20 dir, 0-4 priority).
   - Unified dependency orchestration (`install`, `add`, `remove`, `update`, `outdated`, `audit`, `run`).
   - `upm.toml` manifest management & native manifest scaffolding.
2. **L2' Cross-Language Invocation Bridge (`src/bridge/`)**:
   - `upm-bridge/1` stdio RPC protocol with 4-byte Big-Endian uint32 length framing.
   - `UpmValue` codec supporting Base64 binary (`$blob`), opaque object handles (`$ref`), and callbacks (`$fn`).
   - Object/callback handle GC registry (`HandleRegistry`).
   - Language host supervisors (`python_host.py`, `node_host.js`) with `kill_on_drop`.

---

## 2. One-Command Setup & Verification

```bash
# Build the project (creates target/debug/cpm.exe and upm.exe)
cargo build

# Run full test suite (31 unit & integration tests)
cargo test

# Run a CLI command
cargo run --bin cpm -- status
cargo run --bin cpm -- detect
cargo run --bin cpm -- bridge inspect python
cargo run --bin cpm -- bridge call python:math.sqrt '[144.0]'
```

---

## 3. Project Layout Map

```
d:/cpm/
├── Cargo.toml                 # Package manifest & dependencies
├── README.md                  # Quick reference & architecture overview
├── SKILL.md                   # This developer & AI agent guide
├── src/
│   ├── lib.rs                 # Core library re-exports
│   ├── main.rs                # `upm` binary CLI entry point
│   ├── cpm_main.rs            # `cpm` binary CLI entry point
│   ├── acquisition/           # L2 Acquisition Engine
│   │   ├── adapter.rs         # 15+ ecosystem adapter definitions
│   │   ├── scoring.rs         # Multi-signal weighted detection engine
│   │   ├── manifest.rs        # upm.toml & native manifest scaffolding
│   │   └── runner.rs          # Native command runner (supports parallel & filter)
│   ├── bridge/                # L2' Invocation Bridge
│   │   ├── protocol.rs        # MessageEnvelope (Request/Response/Release/Ping/Pong)
│   │   ├── value.rs           # UpmValue codec ($blob, $ref, $fn)
│   │   ├── handles.rs         # HandleRegistry for object/callback GC
│   │   ├── peer.rs            # Bidirectional BridgePeer RPC handler
│   │   ├── host.rs            # HostSupervisor (spawns language hosts with kill_on_drop)
│   │   └── transport/         # StdioRpcTransport (4-byte BE framing + 64MB guard)
│   └── cli/                   # CLI Subcommand Execution Handlers
│       ├── init_cmd.rs        # Interactive & non-interactive scaffolding
│       ├── detect_cmd.rs      # Scored detection report display
│       └── bridge_cmd.rs      # bridge call, inspect, and status handlers
├── hosts/
│   ├── python_host.py         # Python stdio RPC host (upm-bridge/1)
│   └── node_host.js           # Node.js stdio RPC host (upm-bridge/1)
├── docs/
│   ├── setup-guide.html       # Setup & Prerequisites HTML guide
│   ├── getting-started.html   # Getting Started HTML guide
│   ├── architecture.html      # Architecture Specification HTML
│   └── bridge-specification.html # Protocol & Framing Specification HTML
└── tests/                     # 31 Unit & Integration Test Cases
    ├── test_acquisition.rs
    ├── test_acquisition_edge_cases.rs
    ├── test_cli_commands.rs
    ├── test_bridge_protocol.rs
    ├── test_bridge_rpc.rs
    ├── test_value_codec_edge_cases.rs
    ├── test_bridge_host_integration.rs
    └── test_detection.rs
```

---

## 4. Key CLI Commands Cheat Sheet

| Task | Command | Description |
|------|---------|-------------|
| **Init Project** | `cpm init` | Interactive project setup |
| **Init (Fast)** | `cpm init -l python -f node,rust -y` | Non-interactive project creation |
| **Detect Ecosystems** | `cpm detect` | Scored detection analysis |
| **Install All** | `cpm install` (alias: `is`) | Install all detected ecosystems |
| **Install Parallel** | `cpm install --parallel` (`-p`) | Install ecosystems concurrently in parallel |
| **Install Filter** | `cpm install --filter python` (`-f`) | Target specific language ecosystem |
| **Add Dependency** | `cpm add requests` | Smart auto-detects prefix (`pip:requests`) |
| **Add Explicit** | `cpm add npm:express` | Add foreign dependency |
| **Remove Dep** | `cpm remove pip:requests` (alias: `rm`) | Remove dependency from `upm.toml` |
| **Update All** | `cpm update` (alias: `up`) | Update dependencies across ecosystems |
| **Outdated Report** | `cpm outdated` | Check outdated dependencies |
| **Security Audit** | `cpm audit` | Check lockfile pinning & vulnerability audit |
| **Bridge Call** | `cpm bridge call python:math.sqrt '[9]'` | Perform stdio RPC call to host |
| **Bridge Inspect** | `cpm bridge inspect python` | Query host RPC method catalog dynamically |
| **Bridge Status** | `cpm bridge status` | Display transport tiers (`ffi`, `embed`, `rpc`) |

---

## 5. Core Development Guidelines

1. **Security & Process Hardening**:
   - Always use vector arguments (`Command::new(prog).args(&args)`) — NEVER use raw shell strings (`sh -c` or `cmd.exe /c`).
   - Validate target identifiers in bridge calls using `is_valid_identifier()` to prevent path traversal or injection.

2. **Framing & Serialization**:
   - All stdio RPC wire payloads use 4-byte Big-Endian `u32` headers (`to_be_bytes()`).
   - Max payload size is guarded by `MAX_MESSAGE_SIZE` (64 MB).
   - Envelopes use untagged/tagged JSON schemas defined in `src/bridge/protocol.rs`.

3. **Adding New Ecosystem Adapters**:
   - Add new ecosystems to `load_builtin_adapters()` in `src/acquisition/adapter.rs`.
   - Specify `name`, `language`, `manifest_file`, `lockfile_file`, `install_cmd`, `add_cmd`, etc.

4. **Adding New Language Host Methods**:
   - Extend `handle_request()` in `hosts/python_host.py` or `handleMessage()` in `hosts/node_host.js`.
   - Update `__inspect__` method handler to register signature and description for discovery via `cpm bridge inspect`.

5. **Testing Rules**:
   - Run `cargo test` before submitting changes.
   - All tests in `tests/` must pass cleanly without warnings.
