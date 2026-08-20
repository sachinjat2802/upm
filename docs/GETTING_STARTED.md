# CPM Getting Started Guide

> **The complete guide to polyglot development with CPM.**
> Learn how to set up projects in any language and use packages from other languages in the same codebase.

---

## Table of Contents

1. [Installation](#1-installation)
2. [Your First Project](#2-your-first-project)
3. [Language-Specific Setup](#3-language-specific-setup)
   - [JavaScript / TypeScript](#-javascript--typescript)
   - [Python](#-python)
   - [Rust](#-rust)
   - [Go](#-go)
   - [Java / Kotlin](#-java--kotlin)
   - [PHP](#-php)
   - [Ruby](#-ruby)
   - [C# / .NET](#-c--net)
   - [Dart / Flutter](#-dart--flutter)
   - [Elixir](#-elixir)
4. [Importing Foreign Packages](#4-importing-foreign-packages)
5. [Cross-Language Function Calls](#5-cross-language-function-calls-bridge)
6. [Real-World Recipes](#6-real-world-recipes)
7. [Understanding upm.toml](#7-understanding-upmtoml)
8. [Command Reference](#8-command-reference)

---

## 1. Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs) (for building CPM from source)
- The native package managers you plan to use (e.g. `node`/`pnpm`, `python`/`uv`, `go`, etc.)

### Build from Source

```bash
git clone https://github.com/upm-org/upm.git
cd upm
cargo build --release

# Copy binaries to your PATH
cp target/release/cpm.exe ~/.local/bin/   # or wherever you keep binaries
cp target/release/upm.exe ~/.local/bin/
```

### Verify Installation

```bash
cpm --version
# upm 0.1.0

cpm --help
# Shows the full command reference with emoji icons
```

---

## 2. Your First Project

### Interactive Mode (Recommended)

```bash
mkdir my-app && cd my-app
cpm init
```

CPM will ask you three questions:

```
  ╭──────────────────────────────────────────────────────╮
  │  🚀 UPM Project Initializer                         │
  ╰──────────────────────────────────────────────────────╯

  ? Project name [my-app]: my-app
  ? What is your base language?
   › 1) 📦 JavaScript / TypeScript
     2) 🐍 Python
     3) 🦀 Rust
     4) 🐹 Go
     ...
  › Choose [1]: 2

  ? Which foreign ecosystems do you want to support?
    1) 📦 JavaScript / TypeScript
    2) 🦀 Rust
    3) 🐹 Go
    ...
  › Choose: 1,3
```

This creates:

```
my-app/
├── upm.toml              # CPM manifest (tracks all ecosystems)
├── pyproject.toml         # Python manifest (base language)
├── main.py               # Python entry point
├── package.json           # Node.js manifest (foreign)
├── index.js               # Node.js entry point
├── go.mod                 # Go manifest (foreign)
└── main.go                # Go entry point
```

### Non-Interactive Mode

```bash
cpm init --base-lang python --foreign-langs node,rust,go -y
```

### Quick One-Liners

```bash
# JavaScript project with Python support
cpm init -l javascript -f python -y

# Python project with Node.js + Rust
cpm init -l python -f node,rust -y

# Rust project with Python + Go
cpm init -l rust -f python,go -y

# Go project, all defaults
cpm init -l go -y
```

---

## 3. Language-Specific Setup

### 📦 JavaScript / TypeScript

**What CPM creates:**

| File | Purpose |
|------|---------|
| `package.json` | Node.js manifest with project name & version |
| `index.js` | Entry point file |

**Package Manager Detection:**

CPM auto-detects which JS package manager you use by looking at lockfiles:

| Lockfile | Detected Manager | Priority |
|----------|-----------------|----------|
| `pnpm-lock.yaml` | pnpm | ★★★★ (highest) |
| `yarn.lock` | yarn | ★★★ |
| `bun.lockb` | bun | ★★ |
| `package-lock.json` | npm | ★ |

**Getting started:**

```bash
# If you don't have pnpm installed
npm install -g pnpm

# Initialize with JS as base
cpm init -l javascript -y

# Add packages
cpm add npm:express        # adds via detected manager (pnpm/npm/yarn/bun)
cpm add npm:typescript
cpm add npm:@types/node

# Install all dependencies
cpm install

# Run a script defined in package.json
cpm run dev
```

**Example `index.js`:**

```javascript
const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.json({ message: 'Hello from CPM polyglot project!' });
});

app.listen(3000, () => console.log('Server running on :3000'));
```

---

### 🐍 Python

**What CPM creates:**

| File | Purpose |
|------|---------|
| `pyproject.toml` | Python project manifest (PEP 621) |
| `main.py` | Entry point file |

**Package Manager Detection:**

| Lockfile / Marker | Detected Manager | Priority |
|-------------------|-----------------|----------|
| `uv.lock` or `[tool.uv]` | uv | ★★★★ |
| `poetry.lock` or `[tool.poetry]` | poetry | ★★★ |
| `requirements.txt` | pip | ★ |

**Getting started:**

```bash
# Recommended: install uv (fastest Python package manager)
# https://docs.astral.sh/uv/getting-started/installation/

# Initialize with Python as base
cpm init -l python -y

# Add packages
cpm add pip:requests       # HTTP library
cpm add pip:fastapi        # Web framework
cpm add pip:pandas         # Data analysis
cpm add pip:docling        # Document parsing

# Install all
cpm install

# Run your app
cpm run main.py
```

**Example `main.py`:**

```python
import requests
import json

def fetch_data(url):
    """Fetch JSON data from an API endpoint."""
    response = requests.get(url)
    return response.json()

if __name__ == '__main__':
    data = fetch_data('https://api.github.com/repos/python/cpython')
    print(f"Python has {data['stargazers_count']} stars on GitHub!")
```

---

### 🦀 Rust

**What CPM creates:**

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust package manifest |
| `src/main.rs` | Entry point file |

**Getting started:**

```bash
# Initialize with Rust as base
cpm init -l rust -y

# Add packages (uses `cargo add` under the hood)
cpm add cargo:serde        # Serialization framework
cpm add cargo:tokio        # Async runtime
cpm add cargo:reqwest      # HTTP client
cpm add cargo:clap         # CLI argument parsing

# Build and run
cpm install                # runs `cargo build`
cpm run main               # runs `cargo run --bin main`
```

**Example `src/main.rs`:**

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    stargazers_count: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo: Repo = reqwest::get("https://api.github.com/repos/rust-lang/rust")
        .await?
        .json()
        .await?;
    println!("{} has {} stars!", repo.name, repo.stargazers_count);
    Ok(())
}
```

---

### 🐹 Go

**What CPM creates:**

| File | Purpose |
|------|---------|
| `go.mod` | Go module manifest |
| `main.go` | Entry point file |

**Getting started:**

```bash
# Initialize with Go as base
cpm init -l go -y

# Add packages (uses `go get` under the hood)
cpm add go:github.com/gin-gonic/gin       # Web framework
cpm add go:github.com/spf13/cobra         # CLI framework
cpm add go:github.com/rs/zerolog          # Logging

# Install
cpm install                # runs `go mod download`

# Run
cpm run .                  # runs `go run .`
```

**Example `main.go`:**

```go
package main

import (
    "fmt"
    "net/http"
    "github.com/gin-gonic/gin"
)

func main() {
    r := gin.Default()
    r.GET("/", func(c *gin.Context) {
        c.JSON(http.StatusOK, gin.H{"message": "Hello from CPM Go app!"})
    })
    fmt.Println("Starting server on :8080")
    r.Run(":8080")
}
```

---

### ☕ Java / Kotlin

**Package Manager Detection:**

| Marker | Detected Manager | Priority |
|--------|-----------------|----------|
| `build.gradle` / `build.gradle.kts` | Gradle | ★★★ |
| `pom.xml` | Maven | ★★ |

**Getting started:**

```bash
# Add Java dependencies
cpm add maven:com.google.code.gson:gson:2.10
cpm add gradle:org.springframework.boot:spring-boot-starter-web

# Install
cpm install

# Audit for vulnerabilities
cpm audit
```

---

### 🐘 PHP

**Getting started:**

```bash
# Add PHP dependencies (uses composer)
cpm add composer:laravel/framework
cpm add composer:guzzlehttp/guzzle

# Install
cpm install
```

---

### 💎 Ruby

**Getting started:**

```bash
# Add Ruby dependencies (uses bundler)
cpm add bundler:rails
cpm add bundler:sidekiq

# Install
cpm install
```

---

### 🔷 C# / .NET

**Getting started:**

```bash
# Add .NET dependencies (uses dotnet CLI)
cpm add nuget:Newtonsoft.Json
cpm add nuget:Microsoft.AspNetCore.App

# Install
cpm install                # runs `dotnet restore`
```

---

### 🎯 Dart / Flutter

**Getting started:**

```bash
# Add Dart dependencies
cpm add pub:http
cpm add pub:provider

# Install
cpm install                # runs `dart pub get`
```

---

### 💧 Elixir

**Getting started:**

```bash
# Add Elixir dependencies (uses mix)
cpm add mix:phoenix
cpm add mix:ecto

# Install
cpm install                # runs `mix deps.get`
```

---

## 4. Importing Foreign Packages

This is the core superpower of CPM. You can use packages from **any ecosystem** in the same project.

### Step 1: Add the Foreign Ecosystem

```bash
# Your base is Python, but you need Node.js packages too
cpm init -l python -f node -y
```

### Step 2: Add Foreign Dependencies

```bash
# Add Python packages (your base language)
cpm add pip:requests
cpm add pip:pandas

# Add Node.js packages (foreign ecosystem)
cpm add npm:sharp           # Image processing (no Python equivalent this fast)
cpm add npm:puppeteer       # Browser automation
```

### Step 3: Install Everything At Once

```bash
cpm install
```

This runs **both** `uv sync` (Python) **and** `pnpm install` (Node.js) in one command.

### Step 4: Use Foreign Packages via the Bridge

To actually **call** foreign language code from your base language, use the CPM bridge:

```bash
# From your Python project, call Node.js sharp to resize an image
cpm bridge call node:sharp.resize '["photo.jpg", 800, 600]'
```

### What `upm.toml` Looks Like

```toml
[project]
name = "my-ml-app"
version = "0.1.0"
primary_language = "python"

ecosystems = ["uv", "pnpm"]

[foreign_dependencies]
"pip:requests" = "latest"
"pip:pandas" = "latest"
"npm:sharp" = "latest"
"npm:puppeteer" = "latest"

[transports]
default_tier = "rpc"
```

---

## 5. Cross-Language Function Calls (Bridge)

The CPM bridge lets you call functions in foreign languages over stdio RPC.

### How It Works

```
┌─────────────────────┐     stdio      ┌─────────────────────┐
│   Your Rust/CLI     │ ───────────── │  Python Host         │
│   (cpm bridge call) │  upm-bridge/1 │  (hosts/python_host) │
│                     │ ◄───────────── │                      │
└─────────────────────┘                └─────────────────────┘
```

1. CPM spawns the target language's **host process** (e.g. `python hosts/python_host.py`)
2. Sends a JSON-framed RPC request over stdin
3. The host executes the method and returns the result over stdout
4. CPM displays the result

### Python Bridge Examples

```bash
# Math operations
cpm bridge call python:math.sqrt '[144.0]'
# → 12.0

# Hash data
cpm bridge call python:hash.sha256 '["hello world"]'
# → "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"

# Parse a document (docling simulation)
cpm bridge call python:docling.parse '["report.pdf"]'
# → { "filename": "report.pdf", "pages": 28, "sections": [...] }

# Simple echo
cpm bridge call python:echo '["test message"]'
# → "test message"

# Ping/pong health check
cpm bridge call python:ping
# → "pong"
```

### Node.js Bridge Examples

```bash
# Image processing
cpm bridge call node:sharp.resize '["photo.jpg", 800, 600]'
# → { "resized": true, "width": 800, "height": 600, "format": "png" }

# Crypto hashing
cpm bridge call node:crypto.sha256 '["hello world"]'
# → "b94d27b9934d3e08..."

# Echo
cpm bridge call node:echo '["test"]'
# → "test"
```

### Writing Your Own Host Methods

You can extend the Python host by adding methods to `hosts/python_host.py`:

```python
# In hosts/python_host.py, inside handle_request():

elif method == "my_custom.analyze":
    text = args[0]
    word_count = len(text.split())
    result = {
        "text_length": len(text),
        "word_count": word_count,
        "avg_word_length": len(text) / max(word_count, 1)
    }
    send_message({
        "type": "response",
        "id": req_id,
        "result": result,
        "error": None
    })
```

Then call it:

```bash
cpm bridge call python:my_custom.analyze '["Hello world from CPM bridge"]'
# → { "text_length": 28, "word_count": 5, "avg_word_length": 5.6 }
```

Similarly for Node.js — add methods to `hosts/node_host.js`:

```javascript
// In hosts/node_host.js, inside handleMessage():

} else if (method === 'my_custom.transform') {
    const input = args[0];
    const result = {
        upper: input.toUpperCase(),
        reversed: input.split('').reverse().join(''),
        length: input.length
    };
    sendMessage({ type: 'response', id: reqId, result, error: null });
}
```

---

## 6. Real-World Recipes

### Recipe 1: Python ML + Node.js API Server

> Use Python for machine learning and Node.js for the REST API.

```bash
mkdir ml-api && cd ml-api
cpm init -l python -f node -y

# Python dependencies (ML)
cpm add pip:scikit-learn
cpm add pip:pandas
cpm add pip:numpy

# Node.js dependencies (API)
cpm add npm:express
cpm add npm:cors
cpm add npm:helmet

# Install everything
cpm install
```

**Project structure:**

```
ml-api/
├── upm.toml
├── pyproject.toml       # Python: ML model training
├── main.py              # Python: train/predict
├── package.json         # Node.js: API server
├── index.js             # Node.js: Express routes
└── hosts/
    └── python_host.py   # Bridge: call Python ML from Node.js
```

### Recipe 2: Rust CLI + Python Data Processing

> Use Rust for CLI speed and Python for data analysis.

```bash
mkdir data-tool && cd data-tool
cpm init -l rust -f python -y

# Rust dependencies (CLI)
cpm add cargo:clap
cpm add cargo:serde_json
cpm add cargo:indicatif

# Python dependencies (data)
cpm add pip:pandas
cpm add pip:matplotlib

# Install everything
cpm install
```

### Recipe 3: Go Backend + Node.js Frontend Build

> Use Go for the API server and Node.js for the frontend build toolchain.

```bash
mkdir fullstack && cd fullstack
cpm init -l go -f node -y

# Go dependencies
cpm add go:github.com/gin-gonic/gin
cpm add go:github.com/jmoiron/sqlx

# Node.js dependencies (frontend)
cpm add npm:vite
cpm add npm:react
cpm add npm:react-dom

cpm install
```

### Recipe 4: Polyglot Microservices Monorepo

> One repo, four languages, managed by one tool.

```bash
mkdir platform && cd platform
cpm init -l javascript -f python,rust,go -y

# Each ecosystem gets its own packages
cpm add npm:fastify          # JS API gateway
cpm add pip:fastapi          # Python ML service
cpm add cargo:actix-web      # Rust compute service
cpm add go:github.com/gin-gonic/gin  # Go auth service

# One command installs everything
cpm install

# Check all ecosystems at once
cpm status
cpm outdated
cpm audit
```

---

## 7. Understanding `upm.toml`

The `upm.toml` file is the **single source of truth** for your polyglot project. It tracks which ecosystems are active and what foreign dependencies exist.

### Full Example

```toml
# Project metadata
[project]
name = "my-polyglot-app"
version = "0.1.0"
primary_language = "python"

# Active ecosystem adapters (package managers)
ecosystems = ["uv", "pnpm", "cargo"]

# Cross-ecosystem dependency tracking
[foreign_dependencies]
"pip:requests" = "^2.31"
"pip:pandas" = "^2.2"
"npm:express" = "^4.19"
"npm:sharp" = "^0.33"
"cargo:serde" = "^1.0"

# Bridge transport configuration
[transports]
default_tier = "rpc"          # Options: "rpc", "embed", "ffi"
```

### Fields Explained

| Field | Required | Description |
|-------|----------|-------------|
| `project.name` | Yes | Project name |
| `project.version` | Yes | Semantic version |
| `project.primary_language` | No | Your base language |
| `ecosystems` | Yes | List of active package manager adapters |
| `foreign_dependencies` | No | Map of `"eco:package" = "version"` entries |
| `transports.default_tier` | No | Bridge transport tier (`rpc`, `embed`, `ffi`) |

### Ecosystem Adapter Names

Use these names in `upm.toml` and `cpm add`:

| Name | Language | Tool |
|------|----------|------|
| `npm` | JavaScript | npm |
| `pnpm` | JavaScript | pnpm |
| `yarn` | JavaScript | yarn |
| `bun` | JavaScript | bun |
| `pip` | Python | pip |
| `uv` | Python | uv |
| `poetry` | Python | poetry |
| `cargo` | Rust | cargo |
| `go` | Go | go mod |
| `maven` | Java | Maven |
| `gradle` | Java/Kotlin | Gradle |
| `composer` | PHP | Composer |
| `bundler` | Ruby | Bundler |
| `nuget` | C#/.NET | dotnet |
| `pub` | Dart | dart pub |
| `mix` | Elixir | mix |

---

## 8. Command Reference

### Quick Reference Card

```bash
# ── Project Lifecycle ──────────────────────────────
cpm init                        # Interactive setup
cpm init -l python -f node -y   # Non-interactive
cpm status                      # Project overview
cpm detect                      # Show detected ecosystems

# ── Dependency Management ──────────────────────────
cpm install                     # Install all deps (alias: is)
cpm add pip:requests            # Add dependency (alias: a)
cpm remove pip:requests         # Remove dependency (alias: rm)
cpm update                      # Update all deps (alias: up)
cpm outdated                    # Show outdated deps
cpm audit                       # Security vulnerabilities

# ── Running Code ───────────────────────────────────
cpm run dev                     # Run script across ecosystems
cpm run build
cpm run test

# ── Cross-Language Bridge ──────────────────────────
cpm bridge call python:math.sqrt '[9]'       # Call Python
cpm bridge call node:crypto.sha256 '["hi"]'  # Call Node.js
cpm bridge status                             # Transport info

# ── Shortcuts ──────────────────────────────────────
cpm i                           # same as: cpm init
cpm d                           # same as: cpm detect
cpm is                          # same as: cpm install
cpm a pip:flask                 # same as: cpm add pip:flask
cpm rm pip:flask                # same as: cpm remove pip:flask
cpm up                          # same as: cpm update
```

### Flags

| Flag | Commands | Description |
|------|----------|-------------|
| `--dry-run` | install, add, remove, update | Show what would run without executing |
| `-y, --yes` | init | Accept all defaults |
| `-l, --base-lang` | init | Set the base language |
| `-f, --foreign-langs` | init | Comma-separated foreign languages |
| `--name` | init | Set project name |
| `--help` | all | Show help for any command |

---

## Next Steps

- Read the [Architecture Guide](ARCHITECTURE.md) for the scoring engine design
- Read the [Bridge Specification](BRIDGE_SPECIFICATION.md) for the wire protocol details
- Run `cpm --help` for the full command reference

---

*Built with ❤️ by the CPM/UPM team. Write in one language. Depend on all of them.*
