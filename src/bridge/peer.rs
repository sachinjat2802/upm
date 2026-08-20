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
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use uuid::Uuid;

/// Rust-side bridge peer that manages RPC calls and incoming messages.
pub struct BridgePeer {
    transport: Arc<dyn Transport>,
    /// Public handle registry for registering objects/callbacks.
    pub handles: Arc<HandleRegistry>,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<RpcResponse>>>>,
}

impl BridgePeer {
    /// Create a new peer with the given transport.
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            handles: Arc::new(HandleRegistry::new()),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Call a remote method on the foreign host and await the response.
    ///
    /// This sends an RPC request and blocks (async) until the foreign
    /// host responds with either a result or an error.
    pub async fn call(&self, method: &str, args: Vec<UpmValue>) -> Result<UpmValue> {
        let req_id = Uuid::new_v4().to_string();
        let envelope = MessageEnvelope::request(&req_id, method, args);

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(req_id.clone(), tx);
        }

        self.transport.send_message(&envelope).await?;

        let response = rx.await.map_err(|_| anyhow!("RPC response channel closed"))?;

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
    }
}
