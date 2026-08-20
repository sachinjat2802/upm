# Migration Guide — Moving Existing Applications to CPM

> **Migrate any existing Next.js, React, FastAPI, NestJS, Express, Rails, or Django codebase to CPM in 5 simple steps with zero breaking changes.**

---

## 📋 Step-by-Step Migration Workflow

### Step 1: Scan Your Existing Repository
Open your existing codebase directory in terminal and run `cpm detect`. CPM scans existing markers, lockfiles, and manifests (`package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`, etc.) without altering any files:

```bash
cd /path/to/my-existing-app
cpm detect
```

### Step 2: Initialize CPM Manifest (`upm.toml`)
Run non-interactive initialization:

```bash
cpm init -y
```

> **Zero Breaking Changes**: CPM creates a clean `upm.toml` manifest. All your existing `package.json`, `pyproject.toml`, `Cargo.toml`, and build scripts continue working 100% as before!

---

### Step 3: Add Foreign Packages (As Needed)
To introduce Python AI libraries into your Next.js app or Rust compute into your FastAPI app, use `cpm add`:

```bash
# Add Python library to Next.js / Express app
cpm add pip:docling

# Add Rust library
cpm add cargo:serde

# Add Node.js library
cpm add npm:express
```

---

### Step 4: Drop In CPM Client SDK

#### In Node.js / TypeScript (Next.js, NestJS, Express):
```typescript
import { CpmBridge } from './sdk/node/cpm_sdk';

const bridge = new CpmBridge();
const result = await bridge.call('python:docling.parse', ['proposal.pdf']);
```

#### In Python (FastAPI, Flask, Django):
```python
from cpm_sdk import CpmBridge

bridge = CpmBridge()
result = bridge.call('node:crypto.sha256', ['hello world'])
```

---

### Step 5: Verify Build & Security Lockfiles

```bash
# Install all detected ecosystems concurrently in parallel
cpm install --parallel

# Audit lockfiles and supply chain security
cpm audit
```
