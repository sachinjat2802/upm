/// # Host Supervisor
///
/// Manages the lifecycle of foreign language host processes.
/// Each host is spawned as a child process with stdin/stdout piped
/// for the `upm-bridge/1` framed stdio RPC protocol.
///
/// Supported runtimes: `python`, `node`, and any executable that
/// speaks the 4-byte BE length-prefixed JSON wire format.

use crate::bridge::peer::BridgePeer;
use crate::bridge::transport::rpc::StdioRpcTransport;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};

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
    ///
    /// `program` is the executable name (e.g. `"python"`, `"node"`).
    /// `script_path` is the path to the host script file.
    ///
    /// The child process is spawned with `kill_on_drop(true)` so it is
    /// automatically terminated when the `HostProcess` is dropped.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use upm::bridge::host::HostSupervisor;
    /// # async fn example() -> anyhow::Result<()> {
    /// let host = HostSupervisor::spawn_host("python", Path::new("hosts/python_host.py")).await?;
    /// let result = host.peer.call("math.sqrt", vec![]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn spawn_host(program: &str, script_path: &Path) -> Result<HostProcess> {
        let mut child = Command::new(program)
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
