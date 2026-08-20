/// # Transport Layer
///
/// Defines the `Transport` trait for bridge communication.
/// Implementations provide framed, bidirectional message passing
/// between the Rust host and foreign language processes.
///
/// The default implementation is [`rpc::StdioRpcTransport`], which
/// uses 4-byte big-endian length-prefixed JSON frames over stdin/stdout.

pub mod rpc;

use crate::bridge::protocol::MessageEnvelope;
use anyhow::Result;
use async_trait::async_trait;

/// Trait for bridge message transports.
///
/// Implementors must provide framed send and receive operations.
/// The transport is used by [`super::peer::BridgePeer`] for all
/// RPC communication.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message envelope to the foreign side.
    async fn send_message(&self, msg: &MessageEnvelope) -> Result<()>;

    /// Read the next message envelope from the foreign side.
    ///
    /// Returns `Ok(None)` when the connection is closed (EOF).
    async fn read_message(&self) -> Result<Option<MessageEnvelope>>;
}
