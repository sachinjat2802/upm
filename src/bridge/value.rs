/// # Bridge Value Codec
///
/// Defines the `UpmValue` enum — the universal value type that flows
/// across the bridge protocol. It extends JSON with three special
/// tagged variants:
///
/// | Tag     | Purpose                              |
/// |---------|--------------------------------------|
/// | `$blob` | Base64-encoded binary data           |
/// | `$ref`  | Opaque object handle reference        |
/// | `$fn`   | Callback function handle              |

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Base64-encoded binary blob value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlobValue {
    /// Unique blob identifier.
    #[serde(rename = "$blob")]
    pub id: String,
    /// Base64-encoded payload data.
    pub data_base64: String,
    /// Original byte length of the payload.
    pub len: usize,
}

/// Opaque object handle reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefValue {
    /// Handle identifier (matches `HandleRegistry` key).
    #[serde(rename = "$ref")]
    pub id: String,
    /// Optional type name for display/debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// Remote callback function handle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FnValue {
    /// Callback handle identifier.
    #[serde(rename = "$fn")]
    pub id: String,
}

/// Universal value type for the `upm-bridge/1` protocol.
///
/// `UpmValue` is deserialized with `#[serde(untagged)]`, meaning the
/// tagged variants (`$blob`, `$ref`, `$fn`) are matched by the presence
/// of their sentinel JSON keys.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpmValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number (f64).
    Number(f64),
    /// JSON string.
    String(String),
    /// Binary blob with Base64 payload.
    Blob(BlobValue),
    /// Opaque object handle reference.
    Ref(RefValue),
    /// Remote callback function handle.
    Fn(FnValue),
    /// JSON array.
    Array(Vec<UpmValue>),
    /// JSON object / map.
    Map(BTreeMap<String, UpmValue>),
}

impl UpmValue {
    /// Create a `$blob` value from raw bytes.
    pub fn blob_from_bytes(data: &[u8]) -> Self {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        UpmValue::Blob(BlobValue {
            id: uuid::Uuid::new_v4().to_string(),
            data_base64: encoded,
            len: data.len(),
        })
    }

    /// Create a `$ref` value from an object handle ID.
    pub fn object_ref(id: impl Into<String>, type_name: Option<String>) -> Self {
        UpmValue::Ref(RefValue {
            id: id.into(),
            type_name,
        })
    }

    /// Create a `$fn` callback value from a handle ID.
    pub fn fn_callback(id: impl Into<String>) -> Self {
        UpmValue::Fn(FnValue { id: id.into() })
    }

    /// Check if this value is a `$ref`.
    pub fn is_ref(&self) -> bool {
        matches!(self, UpmValue::Ref(_))
    }

    /// Check if this value is a `$fn`.
    pub fn is_fn(&self) -> bool {
        matches!(self, UpmValue::Fn(_))
    }

    /// Check if this value is a `$blob`.
    pub fn is_blob(&self) -> bool {
        matches!(self, UpmValue::Blob(_))
    }
}

/// Structured error returned from a foreign language host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpmError {
    /// Error type name (e.g. `"ValueError"`, `"NotFoundError"`).
    pub error_type: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional stack trace from the foreign runtime.
    pub stack_trace: Option<String>,
}

impl UpmError {
    /// Create a new error with the given type and message.
    pub fn new(error_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_type: error_type.into(),
            message: message.into(),
            stack_trace: None,
        }
    }
}
