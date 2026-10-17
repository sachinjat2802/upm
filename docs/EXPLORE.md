# CPM Exploration & Deep-Dive Architecture Guide

> A comprehensive, hands-on exploration of the **CPM / UPM Universal Agentic Platform**.

---

## 🌟 Table of Contents
1. [The Agentic Era Manifesto](#1-the-agentic-era-manifesto)
2. [Exploration 1: Scaffolding an Agentic App](#2-exploration-1-scaffolding-an-agentic-app)
3. [Exploration 2: Native AST Dependency Guesser](#3-exploration-2-native-ast-dependency-guesser)
4. [Exploration 3: Content-Addressable Storage (CAS) & Lockfile Verification](#4-exploration-3-content-addressable-storage-cas--lockfile-verification)
5. [Exploration 4: Universal AI Guidelines Cross-IDE Synchronization](#5-exploration-4-universal-ai-guidelines-cross-ide-synchronization)
6. [Exploration 5: Universal MCP Hub](#6-exploration-5-universal-mcp-hub)
7. [Exploration 6: Autonomous AI Agent Interface](#7-exploration-6-autonomous-ai-agent-interface)
8. [Exploration 7: High-Capacity D: Drive Optimization](#8-exploration-7-high-capacity-d-drive-optimization)
9. [Exploration 8: Zero-Latency Cross-Language Bridge](#9-exploration-8-zero-latency-cross-language-bridge)
10. [FAQ & Production Best Practices](#10-faq--production-best-practices)

---

## 1. The Agentic Era Manifesto

In modern software development, humans and AI agents build applications together. However, developer workstations are fragmented:
- **Language Fragmentation:** Python for AI models, TypeScript for frontend, Rust/Go for backends.
- **Rule Fragmentation:** Cursor needs `.cursorrules`, Claude needs `CLAUDE.md`, Windsurf needs `.windsurfrules`, Cline needs `.clinerules`, Antigravity needs `.agent/rules`.
- **Protocol Fragmentation:** MCP servers must be configured separately in Claude Desktop, Cursor, and Zed.
- **Disk Space Fragmentation:** C: drives frequently fill up with duplicated `.venv`, `node_modules`, and package caches.

**CPM unites this entire ecosystem into one clean, high-performance platform.**

---

## 2. Exploration 1: Scaffolding an Agentic App

To reduce app creation complexity to zero, run:

```bash
cpm create ai-assistant --template agentic
cd ai-assistant
```

### What CPM Generates:
1. `upm.toml`: Polyglot manifest declaring ecosystems (`python`, `node`), foreign dependencies (`fastapi`, `pydantic`), and unified scripts (`dev`, `start`, `test`).
2. `server.py`: Production-ready FastAPI microservice with AI agent execution endpoints.
3. `mcp.json`: Model Context Protocol configuration for immediate AI tool integration.
4. `CLAUDE.md`, `.cursorrules`, `.windsurfrules`: Synchronized AI guidelines ready for your favorite editor.

Run the app immediately:
```bash
cpm install
cpm run dev
```

---

## 3. Exploration 2: Native AST Dependency Guesser

CPM inspects your codebase's Abstract Syntax Tree (AST) to detect undeclared dependencies in a single pass:

```bash
# Scan Python, JavaScript, TypeScript, and Rust source files
cpm guess
```

If you wrote `import requests` in `main.py` without declaring it in `upm.toml`, CPM reports:
```text
  🔍 Scanning source code for undeclared dependencies...
  ═══════════════════════════════════════════════════════
  Undeclared foreign dependencies detected:
    • pip:requests  (referenced in: main.py)

  Run 'cpm guess --apply' to automatically add them.
```

Applying the guess automatically installs the package:
```bash
cpm guess --apply
```

---

## 4. Exploration 3: Content-Addressable Storage (CAS) & Lockfile Verification

CPM provides a high-speed Content-Addressable Storage (CAS) cache and deterministic lockfile engine:

### Generating Deterministic Lockfiles:
```bash
cpm lock
```
Generates `cpm.lock`:
```toml
lockfile_version = 1
generated_at = "2026-10-16T17:40:00Z"

[packages."pip:fastapi"]
ecosystem = "pip"
version = "0.110.0"
integrity = "sha256-4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a"
```

### Verifying Integrity:
```bash
cpm lock --verify
```
Guarantees that no dependencies were tampered with in CI/CD or production environments.

### Cache Management:
```bash
# View cache footprint and storage drive
cpm cache status

# Purge cache
cpm cache clean
```

---

## 5. Exploration 4: Universal AI Guidelines Cross-IDE Synchronization

CPM guarantees that your AI instructions remain in sync across all AI coding assistants:

```bash
# Synchronizes rules between Cursor, Claude, Windsurf, Cline, and Antigravity
cpm ai sync

# Add a new architectural rule to the workspace
cpm ai add "No mutable global state"
```

CPM writes the rule once and updates:
- `CLAUDE.md` (for Claude Desktop & Claude Code)
- `.cursorrules` (for Cursor)
- `.windsurfrules` (for Windsurf)
- `.clinerules` (for Cline & Roo-Code)
- `.agent/rules/` (for Antigravity IDE)

---

## 6. Exploration 5: Universal MCP Hub

CPM unifies Model Context Protocol server configuration across all AI editors:

```bash
# List all detected MCP servers across installed AI IDEs
cpm mcp list

# Register a new MCP tool in your project
cpm mcp add database-tool --cmd python -- db_mcp.py

# Auto-sync MCP servers to Claude Desktop, Cursor, and Zed configs
cpm mcp sync
```

---

## 7. Exploration 6: Autonomous AI Agent Interface

AI Coding Agents love CPM because it gives them exact, token-optimized context without wasting prompt tokens:

```bash
# Human-friendly workspace summary
cpm agent context

# Token-optimized JSON for LLM prompts and tools
cpm agent context --json
```

Example JSON response consumed by AI agents:
```json
{
  "workspace_name": "ai-assistant",
  "version": "0.1.0",
  "ecosystems": ["python", "node"],
  "foreign_dependencies": {
    "pip:fastapi": "^0.110.0"
  },
  "available_scripts": {
    "dev": "python -m uvicorn server:app --reload"
  },
  "mcp_servers_configured": ["ai-assistant-server"],
  "ai_guidelines_present": ["CLAUDE.md", ".cursorrules"],
  "cpm_home": "D:\\.cpm",
  "cache_drive": "D:\\"
}
```

---

## 8. Exploration 7: High-Capacity D: Drive Optimization

Developer machines often run out of space on `C:\`. CPM provides automatic and manual high-capacity drive management:

```bash
# Inspect all physical drives and free space
cpm drive status
```
Output:
```text
  💾 CPM High-Capacity Storage Manager
  ═══════════════════════════════════════════════════════
  Local Drive Utilization:
    • Drive C: 15.19 GB free of 103.95 GB total (System Drive - Constrained)
    • Drive D: 806.78 GB free of 124.73 GB total (Primary CPM Storage Drive - Active)

  CPM Active Storage Paths:
    • CPM Home:       D:\.cpm
    • CAS Cache:      D:\.cpm\cache
    • Tool Binaries:  D:\.cpm\bin
    • AI Packages:    D:\.cpm\ai
    • MCP Servers:    D:\.cpm\mcp
```

### Route All Toolchains to D:
```bash
cpm drive setup
```
Automatically configures Windows User Environment variables:
- `CPM_HOME=D:\.cpm`
- `CPM_CACHE_DIR=D:\.cpm\cache`
- `CARGO_HOME=D:\.cargo`
- `RUSTUP_HOME=D:\.rustup`
- `PIP_CACHE_DIR=D:\.cache\pip`
- `NPM_CONFIG_CACHE=D:\.npm_cache`
- `PNPM_HOME=D:\.pnpm`

---

## 9. Exploration 8: Zero-Latency Cross-Language Bridge

Call foreign language functions directly without network latency:

```bash
# Call Python math library from CLI
cpm bridge call python:math.sqrt '[144.0]'

# Call Node.js crypto library from CLI
cpm bridge call node:crypto.sha256 '["hello world"]'

# Benchmark RPC latency across 1,000 iterations
cpm bridge benchmark python 1000
```

---

## 10. FAQ & Production Best Practices

### Q: Can I use `upm` instead of `cpm`?
**A:** Yes. Both binaries are identical aliases. Typing `cpm install` or `upm install` performs the exact same operation.

### Q: How does CPM handle virtual environments?
**A:** CPM manages isolated virtual environments per project (e.g. `.venv`), but deduplicates all package wheels in `D:\.cpm\cache`, saving hundreds of gigabytes across projects.

### Q: Does CPM work on Linux and macOS?
**A:** Yes. CPM is written in pure Rust and compiles natively on Linux, macOS, and Windows. On Linux and macOS, it defaults to `~/.cpm` unless configured with `CPM_HOME`.
