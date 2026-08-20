# CPM / UPM — Universal Package Platform

> **Write in one language. Depend on all of them.**

CPM (Cross-language Package Manager) is a polyglot package manager that **auto-detects**, **orchestrates**, and **bridges** 15+ package managers across 10+ language ecosystems from a single CLI.

```
╭──────────────────────────────────────────────────────╮
│  CPM — Universal Package Platform                    │
│  Write in one language. Depend on all of them.       │
╰──────────────────────────────────────────────────────╯
```

---

## 🌟 Features at a Glance

- 🔍 **Weighted Ecosystem Auto-Detection**: Scores manifest markers, lockfiles, and directory indicators.
- 📦 **Unified Multi-Ecosystem Orchestration**: Install, update, add, remove, and audit packages across Python, Node.js, Rust, Go, Ruby, Java, C#, PHP, Elixir, and Dart.
- 🌉 **High-Performance Cross-Language Bridge**: `upm-bridge/1` stdio RPC protocol for dynamic cross-language calls with `$blob`, `$ref`, and `$fn` codecs.
- 🛡️ **Enterprise Security & Governance**: Secret scanner (`cpm scan-secrets`), RBAC policy checker (`cpm policy`), open-source license auditor (`cpm licenses`), and keyless signature verifier (`cpm verify-sig`).
- ⚡ **Performance & Diagnostics**: Self-healing runtime doctor (`cpm doctor`), RPC latency benchmarker (`cpm bridge benchmark`), SVG flamegraph generator (`cpm flamegraph`), compiler build cache optimizer (`cpm sccache`), and OTLP trace exporter (`cpm trace`).
- ☁️ **DevOps & Cloud Native**: Kubernetes Helm chart generator (`cpm helm`), Kubernetes CRD Operator manifest generator (`cpm operator`), multi-stage Dockerfile generator (`cpm dockerfile`), and cloud memory/cost estimator (`cpm cost`).
- 🔌 **SDK & Framework Integrations**: Native SDKs and middleware for Rust, Go, Java, Python (FastAPI, Django, AWS Lambda), Node.js (NestJS, Nuxt 3, Express), Ruby (Rails Gem), and Flutter / Dart.
- 🐙 **CI/CD & IDE Extensions**: Pre-built configurations for GitHub Actions, GitLab CI, Bitbucket Pipelines, Turborepo, Bazel, VSCode, and JetBrains.

---

## 🚀 Quick Start

```bash
# Initialize a polyglot project (interactive questionnaire or non-interactive flags)
cpm init
cpm init --base-lang python --foreign-langs node,rust -y

# Auto-detect ecosystems in any workspace
cpm detect

# Install dependencies across all ecosystems concurrently
cpm install

# Add dependencies with auto-inferred ecosystems
cpm add pip:requests
cpm add npm:express
cpm add cargo:serde

# Execute cross-language RPC bridge calls
cpm bridge call python:math.sqrt '[144.0]'
cpm bridge call node:crypto.sha256 '["hello world"]'

# Self-healing runtime diagnostics
cpm doctor
```

---

## 📋 Comprehensive CLI Subcommand Reference

### Core Package Operations
| Command | Description |
| :--- | :--- |
| `cpm init` | 🚀 Initialize a polyglot workspace with interactive wizard or non-interactive flags |
| `cpm detect` | 🔍 Auto-detect ecosystems with weighted signal scoring |
| `cpm install` | 📦 Install dependencies across all ecosystems (`--parallel`, `--filter`) |
| `cpm add <eco:pkg>` | ➕ Add dependencies with ecosystem auto-inference |
| `cpm remove <eco:pkg>` | ➖ Remove package dependencies from manifest and native files |
| `cpm update` | 🔄 Update dependencies (`--parallel`, `--filter`, `--auto-pr`) |
| `cpm outdated` | 📋 Check outdated dependencies across registries |
| `cpm status` | 📊 Display project overview and ecosystem summary |
| `cpm run <script>` | ▶️ Run scripts across ecosystems |

### Security, Governance & Auditing
| Command | Description |
| :--- | :--- |
| `cpm scan-secrets` | 🔑 Scan workspace for exposed API keys, RSA keys, and secrets |
| `cpm policy` | 🛡️ Enforce enterprise RBAC security policies (`.cpm_policy.json`) |
| `cpm licenses` | 📄 Audit open-source package licenses against organizational rules |
| `cpm verify-sig` | 🔏 Verify Sigstore Fulcio and GPG cryptographic package signatures |
| `cpm audit-log` | 📑 View append-only cryptographically verified audit trail (`.cpm_audit.log`) |

### Performance, Diagnostics & Profiling
| Command | Description |
| :--- | :--- |
| `cpm doctor` | 🩺 Run self-healing diagnostics and automatic virtualenv repair |
| `cpm bridge benchmark` | ⚡ Benchmark RPC latency and throughput across worker threads |
| `cpm flamegraph` | 🔥 Export SVG call stack flamegraph performance visualizer (`cpm_flamegraph.svg`) |
| `cpm trace` | 🔭 Export OpenTelemetry OTLP distributed trace spans (`cpm_trace_spans.json`) |
| `cpm sccache` | 🚀 Auto-configure `sccache` or `ccache` build wrappers for 5x build speeds |
| `cpm cache` | 🗄️ Manage global content-addressable package store (`~/.cpm/cache`) |

### Advanced Tooling & Visualizers
| Command | Description |
| :--- | :--- |
| `cpm generate-stubs` | 📝 Generate IDE type stubs (`.d.ts` and `.pyi`) |
| `cpm bundle` | 📦 Package air-gapped offline dependency tarball archives |
| `cpm repl` | 💬 Interactive polyglot REPL shell |
| `cpm alias` | ⚡ Execute custom command aliases from `upm.toml` |
| `cpm search` | 🔎 Search across PyPI, npm, Crates.io, and RubyGems registries |
| `cpm rollback` | ⏪ Rollback lockfiles and manifests to previous disaster recovery states |
| `cpm diff` | 🔀 Display universal dependency version drift and diffs |
| `cpm resolve` | 🧩 Transitive dependency SAT solver advisor |
| `cpm cost` | 💰 Estimate cloud memory footprint and hosting cost |
| `cpm graph` | 🕸️ Generate interactive HTML dependency DAG visualizers (`cpm_dep_graph.html`) |
| `cpm completion` | 🐚 Generate tab completion scripts for PowerShell, Bash, Zsh, and Fish |

### DevOps & Cloud Native
| Command | Description |
| :--- | :--- |
| `cpm dockerfile` | 🐳 Auto-generate multi-stage polyglot Dockerfiles |
| `cpm helm` | ☸️ Auto-generate Kubernetes Helm chart manifests |
| `cpm operator` | ☸️ Auto-generate Kubernetes Custom Resource Definition (CRD) manifests |

---

## 🌉 Cross-Language Bridge Protocol (`upm-bridge/1`)

CPM includes `upm-bridge/1` — a stdio RPC framed protocol for executing foreign language functions:

```bash
# Call Python's math.sqrt from the terminal
cpm bridge call python:math.sqrt '[144.0]'
# → 12.0

# Hash data via Node.js crypto
cpm bridge call node:crypto.sha256 '["hello world"]'

# Inspect foreign RPC bridge hosts
cpm bridge status
cpm bridge inspect
```

### Transport Tiers

| Tier | Mechanism | Latency |
| :--- | :--- | :--- |
| **ffi** | `dlopen` + C ABI | ~0.88 µs |
| **embed** | CPython/V8 in-process | ~0.56-2.6 µs |
| **rpc** *(default)* | Framed JSON over stdio | ~156 µs |

---

## 🛠️ SDK Adapters & Integrations

- **Python**: FastAPI (`sdk/python/fastapi_cpm.py`), Django (`sdk/python/django_cpm.py`), AWS Lambda (`sdk/lambda/cpm_lambda.py`), OpenTelemetry (`sdk/telemetry/opentelemetry_cpm.py`)
- **Node.js**: NestJS (`sdk/node/nestjs_cpm.ts`), Nuxt 3 (`sdk/node/nuxt_cpm.ts`), Express (`sdk/node/cpm_sdk.js`), Prometheus (`sdk/metrics/prometheus_cpm.js`)
- **Languages**: Rust (`sdk/rust/`), Go (`sdk/go/cpm_sdk.go`), Java (`sdk/java/CpmBridge.java`), Ruby (`sdk/ruby/cpm_rails.rb`), Dart/Flutter (`sdk/dart/cpm_sdk.dart`)
- **Monorepos & CI/CD**: Turborepo (`sdk/monorepo/turborepo_cpm.json`), Bazel (`sdk/bazel/rules_cpm.bzl`), VSCode (`sdk/vscode/package.json`), JetBrains (`sdk/jetbrains/plugin.xml`), GitHub Actions (`.github/workflows/cpm-ci.yml`), GitLab (`.gitlab-ci.yml`), Bitbucket (`bitbucket-pipelines.yml`)

---

## 📜 Documentation & Guides

- 📖 **[Migration Guide](file:///d:/cpm/docs/migration-guide.html)**: Comprehensive guide for migrating existing apps to CPM.
- 🌉 **[Bridge Specification](file:///d:/cpm/docs/bridge-specification.html)**: Detailed RPC protocol wire spec.
- 🛠️ **[CPM Setup & Workflow Guide](file:///d:/cpm/.agent/skills/cpm-setup-guide.md)**: Full step-by-step developer tutorial.

---

## ⚖️ License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
