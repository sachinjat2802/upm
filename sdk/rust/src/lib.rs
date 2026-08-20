//! # CPM Rust Client SDK
//!
//! Native Rust client library for invoking foreign language methods over CPM stdio RPC bridge.
//!
//! ## Example
//!
//! ```no_run
//! use cpm_sdk::CpmBridge;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let bridge = CpmBridge::new(None);
//!     let result = bridge.call("python:math.sqrt", vec![serde_json::json!(144.0)]).await?;
//!     println!("Result: {:?}", result);
//!     Ok(())
//! }
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CpmBridge {
    pub cpm_bin: PathBuf,
}

impl CpmBridge {
    /// Initialize a new CpmBridge instance.
    pub fn new(cpm_bin: Option<PathBuf>) -> Self {
        let bin = cpm_bin.unwrap_or_else(Self::find_cpm_bin);
        Self { cpm_bin: bin }
    }

    fn find_cpm_bin() -> PathBuf {
        let mut curr = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        for _ in 0..5 {
            for rel in &["target/release/cpm.exe", "target/debug/cpm.exe", "target/release/cpm", "target/debug/cpm"] {
                let candidate = curr.join(rel);
                if candidate.exists() {
                    return candidate;
                }
            }
            if let Some(parent) = curr.parent() {
                curr = parent.to_path_buf();
            } else {
                break;
            }
        }
        PathBuf::from("cpm")
    }

    /// Invoke a foreign language RPC method.
    pub async fn call(&self, target: &str, args: Vec<serde_json::Value>) -> anyhow::Result<serde_json::Value> {
        let args_json = serde_json::to_string(&args)?;
        let output = Command::new(&self.cpm_bin)
            .args(&["bridge", "call", target, &args_json])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!("CPM Bridge Call Error: {}", if !stderr.is_empty() { stderr } else { stdout });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.trim().lines().collect();
        let mut capture = false;
        let mut json_lines = Vec::new();

        for line in lines {
            if line.contains("Response received:") {
                capture = true;
                continue;
            }
            if capture && (line.contains("round-trip via stdio RPC") || line.trim().is_empty()) {
                if !json_lines.is_empty() {
                    break;
                }
            }
            if capture {
                json_lines.push(line);
            }
        }

        let raw_json = json_lines.join("\n");
        let parsed: serde_json::Value = serde_json::from_str(&raw_json)?;
        Ok(parsed)
    }
}
