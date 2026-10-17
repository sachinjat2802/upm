# CPM Programmatic API Reference

CPM can be embedded directly as a library inside **Rust**, **Python**, and **Node.js** applications, as well as driven via its CLI.

---

## 🦀 Rust API

Add CPM to your `Cargo.toml`:
```toml
[dependencies]
upm = { path = "../cpm" }
tokio = { version = "1.0", features = ["full"] }
```

### 1. Multi-Signal Ecosystem Detection
```rust
use upm::acquisition::DetectionEngine;
use std::path::Path;

fn main() {
    let engine = DetectionEngine::new();
    let result = engine.detect_dir(Path::new("."));

    for score in result.detected_ecosystems() {
        println!("Detected: {} (score: {})", score.adapter.display_name, score.total_score);
    }
}
```

### 2. Cross-Language Bridge RPC Invocation
```rust
use upm::bridge::{HostSupervisor, UpmValue};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = HostSupervisor::spawn_host("python", Path::new("hosts/python_host.py")).await?;

    // Call Python math.sqrt across stdio RPC
    let response = host.peer.call("math.sqrt", vec![UpmValue::Number(144.0)]).await?;
    println!("Response: {:?}", response); // UpmValue::Number(12.0)

    Ok(())
}
```

---

## 🐍 Python SDK (`upm-bridge`)

Install or link the Python bridge package:
```python
import asyncio
from cpm_bridge import CpmBridge

async def main():
    bridge = CpmBridge.connect()

    # Call a Rust or Node.js native function
    res = await bridge.call("node:crypto.sha256", ["hello world"])
    print("Hash from Node.js:", res)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 📦 Node.js / TypeScript SDK (`@cpm/bridge`)

```typescript
import { CpmBridge } from "@cpm/bridge";

async function main() {
  const bridge = await CpmBridge.connect();

  // Call Python PyTorch / NumPy model from Node.js
  const result = await bridge.call("python:model.predict", [{ text: "input data" }]);
  console.log("Prediction:", result);
}

main();
```

---

## ⚡ Content-Addressable Storage (CAS) API

```rust
use upm::acquisition::cache::{compute_sha256, get_cache_dir, purge_cache};
use std::path::Path;

fn main() {
    // 4-lane parallel content hash
    let hash = compute_sha256(b"dependency binary artifact payload");
    println!("CAS Integrity Hash: {}", hash);

    let cache_path = get_cache_dir();
    println!("High-Capacity Storage Location: {}", cache_path.display());
}
```
