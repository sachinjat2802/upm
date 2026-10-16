# Changelog

All notable changes to the CPM (Cross-language Package Manager) / UPM (Universal Package Platform) project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-08-20

### Added
- **Shared Unified CLI Architecture**: Single shared CLI driver for both `cpm` and `upm` binaries with 100% feature parity (#1).
- **Global CLI Flags**: Added `--json`, `--verbose` (`-v`), `--quiet` (`-q`), and `--no-color` global flags (#2, #3, #4).
- **Standalone `cpm version` Command**: Added explicit `cpm version` command alongside `--version` (#10).
- **Subcommand Help Headings**: Organized all subcommands under descriptive categories in `--help` output (#14).
- **Universal Package Lister (`cpm list` / `cpm ls`)**: Displays installed dependencies across all ecosystems and `upm.toml` (#64).
- **Universal Build Artifact Cleaner (`cpm clean`)**: Recursively cleans `node_modules`, `.venv`, `target`, `vendor`, `_build` and reports freed disk space (#18).
- **Dependency Explainer (`cpm why <package>`)**: Explains why a dependency exists in `upm.toml` or native manifests (#16).
- **Interactive Quickstart Walkthrough (`cpm quickstart`)**: Step-by-step interactive tutorial command (#36).
- **Version Constraint Syntax in `cpm add`**: Added support for `@version` syntax (e.g. `cpm add pip:requests@2.31.0` or `requests@^2.31.0`) (#13, #61).
- **Typo Did-You-Mean Suggestions**: Added Levenshtein distance algorithm for auto-suggesting corrections on unknown ecosystems (#12).
- **Automatic Manifest Backup**: Automatically backs up `upm.toml` to `.cpm_backup/upm.toml.bak` before modifications (#66).
- **Dynamic `.gitignore` Generation**: `cpm init` automatically scaffolds a language-aware `.gitignore` file (#37).
- **Dynamic Rollback Snapshot Scanning**: `cpm rollback` dynamically scans `.cpm_backup/` for all backup files and asks for confirmation (#11, #84).

### Performance & High-Throughput Optimizations
- **Lock-Free Concurrency for Handle GC (`HandleRegistry`)**: Replaced `Mutex` with `RwLock` and `AtomicU64` in `src/bridge/handles.rs` for non-blocking concurrent reads and nanosecond handle ID generation (#25).
- **Vectorized RPC Wire Framing (`StdioRpcTransport`)**: Single contiguous buffer allocation combining length header + payload to halve OS write syscalls per RPC frame (#49).
- **Warm Host Process Pooling (`HostPool`)**: Integrated process pooling in `src/bridge/host.rs` to reuse host processes across requests (#43, #44).
- **Sub-Millisecond Workspace Fast-Path**: `detect_workspace()` resolves configured ecosystems directly from `upm.toml` in <1ms without scanning directory globs (#53).

### Real Implementations (Replacing Stubs)
- **Real Secret & API Key Scanner (`cpm scan-secrets`)**: High-performance zero-dependency regex/pattern scanner for AWS, GitHub, OpenAI, Slack, Stripe, and private keys (#71).
- **Real RBAC Security Policy Engine (`cpm policy`)**: Enforces lockfile pinning and disallowed licenses against `.cpm_policy.json` (#76).
- **Real Open-Source License Auditor (`cpm licenses`)**: Audits workspace dependencies for Copyleft/GPL license risks (#70).
- **Real Dependency Diff Viewer (`cpm diff`)**: Compares active lockfile states against manifest targets to detect drift (#72).
- **Real Offline Workspace Bundler (`cpm bundle`)**: Packages all manifests, lockfiles, SDKs, and metadata into a distribution archive (#74).
- **Real Dependency Resolution Advisor (`cpm resolve`)**: Analyzes cross-language dependency graphs for unpinned wildcards and version conflicts (#73).

### Fixed
- **PyPI Policy URL Typo**: Fixed typo in default policy registry URL (`pimple` → `simple`) (#83).
- **Parallel Update Error Handling**: Fixed `run_update()` parallel mode silently discarding errors (#22).
- **Silent Command Execution Errors**: `AcquisitionRunner::execute_command()` now returns proper `Err` on command failure instead of `Ok(())` (#21).
- **Manifest Loading Error Handling**: Added `try_load_from_dir` and error reporting for invalid `upm.toml` files (#23).
- **Mutex Poisoning Protection**: `HandleRegistry` now handles thread panic lock poisoning gracefully (#25).
- **Non-TTY / EOF Stdin Prompts**: CLI prompts handle non-TTY pipes cleanly without panicking (#26).
- **Deterministic Detection Order**: `AdapterRegistry::all()` now returns adapters in deterministic alphabetical order (#57).
- **Host Script GC Handling**: Added `release_handles` message handler in Python and Node.js bridge host scripts (#51).
