/// # Stdio RPC Transport
///
/// Implements the `upm-bridge/1` wire protocol: 4-byte big-endian length
/// prefix followed by a UTF-8 JSON payload. Each message is framed
/// independently, enabling multiplexed bidirectional communication over
/// a single stdin/stdout pair.
///
/// ## Wire format
///
/// ```text
/// ┌───────────────┬──────────────────────┐
/// │ 4 bytes BE u32│ JSON payload (UTF-8) │
/// │ (body length) │                      │
/// └───────────────┴──────────────────────┘
/// ```

use crate::bridge::protocol::MessageEnvelope;
use crate::bridge::transport::Transport;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::Mutex;

/// Maximum allowed message size (64 MB). Prevents OOM from malformed headers.
const MAX_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

/// Framed stdio transport for the `upm-bridge/1` protocol.
pub struct StdioRpcTransport<R, W> {
    reader: Mutex<R>,
    writer: Mutex<W>,
}

impl<R, W> StdioRpcTransport<R, W>
where
    R: AsyncRead + Unpin + Send + Sync,
    W: AsyncWrite + Unpin + Send + Sync,
{
    /// Create a new transport from an async reader (stdout of child)
    /// and an async writer (stdin of child).
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
        }
    }
}

#[async_trait]
impl<R, W> Transport for StdioRpcTransport<R, W>
where
    R: AsyncRead + Unpin + Send + Sync,
    W: AsyncWrite + Unpin + Send + Sync,
{
    async fn send_message(&self, msg: &MessageEnvelope) -> Result<()> {
        let json_bytes = serde_json::to_vec(msg)?;
        let len = json_bytes.len() as u32;
        let mut header = [0u8; 4];
        header.copy_from_slice(&len.to_be_bytes());

        let mut writer = self.writer.lock().await;
        writer.write_all(&header).await?;
        writer.write_all(&json_bytes).await?;
        writer.flush().await?;
        Ok(())
    }

    async fn read_message(&self) -> Result<Option<MessageEnvelope>> {
        let mut reader = self.reader.lock().await;
        let mut header = [0u8; 4];

        match reader.read_exact(&mut header).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(anyhow!("Transport read error: {}", e)),
        }

        let len = u32::from_be_bytes(header) as usize;

        // Guard against unreasonably large messages
        if len > MAX_MESSAGE_SIZE {
            return Err(anyhow!(
                "Message size {} bytes exceeds limit of {} bytes. Possible protocol corruption.",
                len,
                MAX_MESSAGE_SIZE
            ));
        }

        let mut body = vec![0u8; len];
        reader.read_exact(&mut body).await?;

        let msg: MessageEnvelope = serde_json::from_slice(&body)?;
        Ok(Some(msg))
    }
}
