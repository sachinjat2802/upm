# Contributing to UPM (Universal Package Manager)

Thank you for your interest in contributing to **UPM** (Universal Package Manager)!
UPM is designed from the ground up to be the first choice for both human software engineers and autonomous AI coding agents.

---

## 🏗️ Codebase Architecture

The project is structured into three primary layers:
1. **L2 Universal Acquisition Engine (`src/acquisition/`)**: Multi-signal ecosystem detection, `upm.toml` manifest serialization, Content-Addressable Storage (CAS), and package runner adapters.
2. **L2' Cross-Language Bridge (`src/bridge/`)**: Length-prefixed binary transport, wire protocol (`upm-bridge/1`), handle GC tracking, and host supervision.
3. **Universal CLI Operations (`src/cli/`)**: Subcommands dispatching package management, monorepo orchestration, LangGraph/LangChain agentic workflows, MCP server sync, security audits, and deployment.

---

## 🛠️ Development Setup

### Prerequisites
- **Rust Toolchain:** Stable (1.75+) via `rustup`
- **Node.js:** 20+ (optional, for testing Node bridge)
- **Python:** 3.10+ (optional, for testing Python bridge)

### Building & Checking
```bash
# Clone the repository
git clone https://github.com/sachinjat2802/upm.git
cd upm

# Fast compile check
cargo check

# Build debug binary (upm)
cargo build

# Run all test suites
cargo test
```

---

## 📏 Core Design & Algorithmic Principles

When submitting code to CPM, please adhere strictly to these principles:

1. **Time & Space Complexity Awareness**:
   - Prefer single-pass directory walks over multiple recursive passes.
   - Use iterative stacks over deep recursion to prevent stack overflows.
   - Pre-allocate collection capacities where size bounds are known (e.g. `Vec::with_capacity`, `HashMap::with_capacity`).
   - Use in-place sorting (`sort_unstable_by`) unless element order preservation is mandatory.
2. **Deterministic & Content-Addressable**:
   - Cache entries and lockfiles must be keyed by cryptographic hashes (SHA-256).
   - All serialization to TOML/JSON must maintain deterministic key ordering.
3. **Zero-Latency Agent Experience**:
   - AI and human commands should respond instantaneously.
   - Batch subprocess calls and avoid spawning shell processes in tight loops.
4. **Pure Native Branding**:
   - Avoid hardcoding third-party brand names or external fork titles in core modules.

---

## 🧪 Testing Guidelines

Every new CLI command or core engine capability must include unit or integration tests in `tests/`:
- Multi-signal scoring tests in `tests/test_acquisition.rs`
- CLI subcommand tests in `tests/test_cli_commands.rs`
- Benchmark and agentic tests in `tests/test_benchmark_features.rs`

Run the test suite before opening a pull request:
```bash
cargo test
```

---

## 📬 Pull Request Process

1. Fork the repo and create your branch from `master`:
   ```bash
   git checkout -b feature/my-amazing-feature
   ```
2. Write clean, self-documenting code with complexity annotations.
3. Run `cargo check` and `cargo test` to ensure 100% test pass rate.
4. Submit your pull request with a descriptive title and summary of changes.
