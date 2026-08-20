# `upm-bridge/1` Wire Protocol & Value Codec Specification

This document defines the wire framing, envelope JSON schema, value serialization semantics, handle management, and callback dispatch rules for `upm-bridge/1`.

---

## 1. Transport Framing

All messages over standard input/output (stdio) MUST be framed with a **4-byte Big-Endian uint32 length header** preceding the UTF-8 encoded JSON body:

```text
+-----------------------+---------------------------------------+
|  Length (4-byte BE)   |  JSON Envelope Payload (UTF-8 Bytes)  |
+-----------------------+---------------------------------------+
|  0x00 0x00 0x00 0x3E  |  {"type":"request","id":"1", ...}     |
+-----------------------+---------------------------------------+
```

---

## 2. Wire Envelope Message Types

### Request Envelope
```json
{
  "type": "request",
  "id": "req_1001",
  "method": "python:math.sqrt",
  "args": [144.0]
}
```

### Response Envelope (Success)
```json
{
  "type": "response",
  "id": "req_1001",
  "result": 12.0,
  "error": null
}
```

### Response Envelope (Error)
```json
{
  "type": "response",
  "id": "req_1001",
  "result": null,
  "error": {
    "code": "METHOD_NOT_FOUND",
    "message": "Unknown method: python:invalid"
  }
}
```

### Handle Release Envelope
```json
{
  "type": "release_handles",
  "handles": ["ref_1001", "fn_2002"]
}
```

---

## 3. Value Representation Semantics

| Type | JSON Representation | Behavior |
| :--- | :--- | :--- |
| **Primitive** | `123`, `"hello"`, `true`, `[1, 2]`, `{"a": 1}` | Copied directly by value |
| **Out-of-band Blob** | `{"$blob": "b1", "data_base64": "SGVsbG8="}` | Binary data encoded as Base64 |
| **Object Handle** | `{"$ref": "ref_1001", "type": "Counter"}` | Opaque reference handle staying in target runtime |
| **Function Callback** | `{"$fn": "fn_2001"}` | Remote invokable callback handle |
