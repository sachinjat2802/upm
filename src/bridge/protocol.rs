//! Wire message envelope definitions for the `upm-bridge/1` stdio RPC protocol.

use crate::bridge::value::{UpmError, UpmValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Target protocol version string (`upm-bridge/1`).
pub const PROTOCOL_VERSION: &str = "upm-bridge/1";

/// RPC invocation request envelope payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    /// Unique correlation request identifier.
    pub id: String,
    /// Target method (e.g. `python:math.sqrt`, `node:sharp.resize`).
    pub method: String,
    /// Positional arguments list.
    pub args: Vec<UpmValue>,
    /// Keyword arguments map.
    #[serde(default)]
    pub kwargs: BTreeMap<String, UpmValue>,
}

/// RPC invocation response envelope payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    /// Request correlation identifier.
    pub id: String,
    /// Returned result value on success.
    pub result: Option<UpmValue>,
    /// Returned error details on failure.
    pub error: Option<UpmError>,
}

/// Batched handle release message payload for garbage collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseHandlesMessage {
    /// List of object or callback handle IDs to release.
    pub handles: Vec<String>,
}

/// Discriminant wire message envelope enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageEnvelope {
    /// Outgoing or incoming RPC request.
    Request(RpcRequest),
    /// Response to an RPC request.
    Response(RpcResponse),
    /// Batched handle garbage collection release.
    ReleaseHandles(ReleaseHandlesMessage),
    /// Transport ping message.
    Ping { id: String },
    /// Transport pong message.
    Pong { id: String },
}

impl MessageEnvelope {
    /// Create a new RPC request envelope.
    pub fn request(id: impl Into<String>, method: impl Into<String>, args: Vec<UpmValue>) -> Self {
        MessageEnvelope::Request(RpcRequest {
            id: id.into(),
            method: method.into(),
            args,
            kwargs: BTreeMap::new(),
        })
    }

    /// Create a success response envelope.
    pub fn success_response(id: impl Into<String>, result: UpmValue) -> Self {
        MessageEnvelope::Response(RpcResponse {
            id: id.into(),
            result: Some(result),
            error: None,
        })
    }

    /// Create an error response envelope.
    pub fn error_response(id: impl Into<String>, err: UpmError) -> Self {
        MessageEnvelope::Response(RpcResponse {
            id: id.into(),
            result: None,
            error: Some(err),
        })
    }

    /// Create a handle release envelope.
    pub fn release_handles(handles: Vec<String>) -> Self {
        MessageEnvelope::ReleaseHandles(ReleaseHandlesMessage { handles })
    }
}
