/// # Host Supervisor & Host Process Pool
///
/// Manages the lifecycle of foreign language host processes.
/// Each host is spawned as a child process with stdin/stdout piped
/// for the `upm-bridge/1` framed stdio RPC protocol.
///
/// Supported runtimes: `python`, `node`, and any executable that
/// speaks the 4-byte BE length-prefixed JSON wire format.

use crate::acquisition::runner::AcquisitionRunner;
use crate::bridge::peer::BridgePeer;
use crate::bridge::transport::rpc::StdioRpcTransport;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// A running language host process with its associated bridge peer.
pub struct HostProcess {
    /// The RPC peer used to call methods on the host.
    pub peer: Arc<BridgePeer>,
    /// The child process handle. Dropping this sends SIGKILL on Unix
    /// or TerminateProcess on Windows when combined with `kill_on_drop`.
    _child: Child,
}

/// Supervisor responsible for spawning and managing host processes.
pub struct HostSupervisor;

impl HostSupervisor {
    /// Spawn a language host process.
    pub async fn spawn_host(program: &str, script_path: &Path) -> Result<HostProcess> {
        let resolved = AcquisitionRunner::resolve_executable(program);
        let mut child = Command::new(&resolved)
            .arg(script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)   // ← auto-cleanup on drop
            .spawn()
            .map_err(|e| anyhow!(
                "Failed to spawn {} host ({}): {}.\n  Hint: Is '{}' installed and on PATH?",
                program, script_path.display(), e, program
            ))?;

        let stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to capture stdin of {} process", program))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to capture stdout of {} process", program))?;

        let transport = Arc::new(StdioRpcTransport::new(stdout, stdin));
        let peer = Arc::new(BridgePeer::new(transport));

        let peer_clone = peer.clone();
        tokio::spawn(async move {
            peer_clone.run_listener_loop().await;
        });

        Ok(HostProcess { peer, _child: child })
    }
}

/// Thread-safe Pool for reusing warm host processes across RPC calls (#43, #44).
///
/// Keeps host processes alive in memory to eliminate process startup overhead,
/// dropping round-trip latency from ~150ms to <1ms.
pub struct HostPool {
    pool: Mutex<HashMap<String, Arc<HostProcess>>>,
}

impl Default for HostPool {
    fn default() -> Self {
        Self::new()
    }
}

impl HostPool {
    /// Create a new empty host process pool.
    pub fn new() -> Self {
        Self {
            pool: Mutex::new(HashMap::new()),
        }
    }

    /// Get an existing warm host process or spawn a new one (#43).
    pub async fn get_or_spawn(&self, program: &str, script_path: &Path) -> Result<Arc<HostProcess>> {
        let key = format!("{}:{}", program, script_path.display());
        let mut lock = self.pool.lock().await;

        if let Some(host) = lock.get(&key) {
            return Ok(host.clone());
        }

        // Spawn a fresh host process if not pooled or dead
        let new_host = Arc::new(HostSupervisor::spawn_host(program, script_path).await?);
        lock.insert(key, new_host.clone());
        Ok(new_host)
    }

    /// Clear all pooled host processes.
    pub async fn clear(&self) {
        let mut lock = self.pool.lock().await;
        lock.clear();
    }
}
