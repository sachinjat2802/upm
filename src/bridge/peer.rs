/// # Bridge Peer
///
/// The `BridgePeer` is the Rust-side endpoint of a bidirectional
/// `upm-bridge/1` RPC connection. It can:
///
/// - **Call remote methods** on a foreign language host (request → response).
/// - **Handle incoming requests** by dispatching `$fn:` callbacks.
/// - **Process handle releases** for cooperative garbage collection.
/// - **Respond to ping/pong** keepalive messages.

use crate::bridge::handles::HandleRegistry;
use crate::bridge::protocol::{MessageEnvelope, RpcResponse};
use crate::bridge::transport::Transport;
use crate::bridge::value::{UpmError, UpmValue};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

/// Rust-side bridge peer that manages RPC calls and incoming messages.
pub struct BridgePeer {
    transport: Arc<dyn Transport>,
    /// Public handle registry for registering objects/callbacks.
    pub handles: Arc<HandleRegistry>,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<RpcResponse>>>>,
    req_counter: AtomicU64,
}

impl BridgePeer {
    /// Create a new peer with the given transport.
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            handles: Arc::new(HandleRegistry::new()),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            req_counter: AtomicU64::new(1),
        }
    }

    /// Call a remote method on the foreign host and await the response.
    ///
    /// Time Complexity: O(1) dispatch + network/pipe latency.
    /// Space Complexity: O(1) request record in pending map.
    pub async fn call(&self, method: &str, args: Vec<UpmValue>) -> Result<UpmValue> {
        let id_val = self.req_counter.fetch_add(1, Ordering::Relaxed);
        let req_id = format!("req_{:x}", id_val);
        let envelope = MessageEnvelope::request(&req_id, method, args);

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(req_id.clone(), tx);
        }

        self.transport.send_message(&envelope).await?;

        let response = match tokio::time::timeout(std::time::Duration::from_secs(15), rx).await {
            Ok(Ok(resp)) => resp,
            Ok(Err(_)) => {
                return Err(anyhow!("RPC channel closed: host process terminated or disconnected"));
            }
            Err(_) => {
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&req_id);
                return Err(anyhow!("RPC call '{}' timed out after 15s waiting for response from host", method));
            }
        };

        if let Some(err) = response.error {
            Err(anyhow!("[{}] {}", err.error_type, err.message))
        } else if let Some(res) = response.result {
            Ok(res)
        } else {
            Ok(UpmValue::Null)
        }
    }

    /// Handle a single incoming message envelope.
    pub async fn handle_incoming(&self, envelope: MessageEnvelope) -> Result<()> {
        match envelope {
            MessageEnvelope::Response(resp) => {
                let mut pending = self.pending_requests.lock().await;
                if let Some(tx) = pending.remove(&resp.id) {
                    let _ = tx.send(resp);
                }
            }
            MessageEnvelope::Request(req) => {
                let req_id = req.id;
                let method = req.method;
                let args = req.args;

                let response_envelope = if method.starts_with("$fn:") {
                    let fn_id = method.trim_start_matches("$fn:");
                    if let Some(cb) = self.handles.get_callback(fn_id) {
                        match cb(args) {
                            Ok(res) => MessageEnvelope::success_response(req_id, res),
                            Err(e) => MessageEnvelope::error_response(req_id, UpmError::new("CallbackError", e)),
                        }
                    } else {
                        MessageEnvelope::error_response(
                            req_id,
                            UpmError::new("NotFoundError", format!("Callback function {} not found", fn_id)),
                        )
                    }
                } else {
                    MessageEnvelope::error_response(
                        req_id,
                        UpmError::new("UnhandledMethod", format!("Method {} not implemented on host", method)),
                    )
                };

                self.transport.send_message(&response_envelope).await?;
            }
            MessageEnvelope::ReleaseHandles(msg) => {
                self.handles.release_handles(&msg.handles);
            }
            MessageEnvelope::Ping { id } => {
                let pong = MessageEnvelope::Pong { id };
                self.transport.send_message(&pong).await?;
            }
            MessageEnvelope::Pong { .. } => {}
        }
        Ok(())
    }

    /// Run the listener loop, reading and dispatching messages until
    /// the transport connection closes or errors.
    pub async fn run_listener_loop(self: Arc<Self>) {
        loop {
            match self.transport.read_message().await {
                Ok(Some(msg)) => {
                    if let Err(e) = self.handle_incoming(msg).await {
                        eprintln!("[upm-bridge] Error handling incoming message: {}", e);
                    }
                }
                Ok(None) => break, // Connection closed
                Err(e) => {
                    eprintln!("[upm-bridge] Transport read loop exited: {}", e);
                    break;
                }
            }
        }
        // Connection closed or failed: unblock any callers waiting for responses
        let mut pending = self.pending_requests.lock().await;
        pending.clear();
    }
}
