//! # L2' Cross-Language Invocation Bridge (`upm-bridge/1`)
//!
//! The `bridge` module implements the cross-language stdio RPC protocol, length-prefixed binary
//! framing transport, value codec semantics (`$blob`, `$ref`, `$fn`), object handle GC tracking,
//! and language host supervisors for Python and Node.js.

pub mod handles;
pub mod host;
pub mod peer;
pub mod protocol;
pub mod transport;
pub mod value;

pub use handles::HandleRegistry;
pub use host::{HostProcess, HostSupervisor};
pub use peer::BridgePeer;
pub use protocol::{MessageEnvelope, RpcRequest, RpcResponse, PROTOCOL_VERSION};
pub use value::{BlobValue, FnValue, RefValue, UpmError, UpmValue};
