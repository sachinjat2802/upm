use base64::Engine;
use std::collections::BTreeMap;
use std::sync::Arc;
use upm::bridge::{HandleRegistry, UpmError, UpmValue};

#[test]
fn test_blob_base64_roundtrip() {
    let raw_bytes = b"\x00\x01\x02\x03\xff\xfe\xfd";
    let blob_val = UpmValue::blob_from_bytes(raw_bytes);

    if let UpmValue::Blob(blob) = &blob_val {
        assert_eq!(blob.len, raw_bytes.len());
        let decoded = base64::engine::general_purpose::STANDARD.decode(&blob.data_base64).unwrap();
        assert_eq!(decoded, raw_bytes);
    } else {
        panic!("Expected Blob variant");
    }

    assert!(blob_val.is_blob());
    assert!(!blob_val.is_ref());
    assert!(!blob_val.is_fn());
}

#[test]
fn test_nested_upm_value_data_structures() {
    let mut map = BTreeMap::new();
    map.insert("name".into(), UpmValue::String("CPM".into()));
    map.insert("version".into(), UpmValue::Number(1.0));
    map.insert("flag".into(), UpmValue::Bool(true));
    map.insert("null_val".into(), UpmValue::Null);
    map.insert("ref_val".into(), UpmValue::object_ref("ref_999", Some("Tensor".into())));

    let list = vec![UpmValue::Map(map), UpmValue::fn_callback("fn_777")];
    let outer = UpmValue::Array(list);

    let json = serde_json::to_string(&outer).unwrap();
    assert!(json.contains("CPM"));
    assert!(json.contains("$ref"));
    assert!(json.contains("$fn"));

    let deserialized: UpmValue = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, outer);
}

#[test]
fn test_upm_error_serialization() {
    let mut err = UpmError::new("RuntimeError", "Division by zero");
    err.stack_trace = Some("Traceback (most recent call last):\n  File 'main.py', line 10".into());

    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("RuntimeError"));
    assert!(json.contains("Division by zero"));
    assert!(json.contains("Traceback"));

    let deserialized: UpmError = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.error_type, "RuntimeError");
    assert_eq!(deserialized.message, "Division by zero");
    assert_eq!(deserialized.stack_trace.as_deref(), Some("Traceback (most recent call last):\n  File 'main.py', line 10"));
}

#[test]
fn test_handle_registry_batch_operations() {
    let registry = HandleRegistry::new();
    assert_eq!(registry.active_handle_count(), 0);

    // Register objects
    let ref1 = registry.register_object(42i32);
    let ref2 = registry.register_object("hello".to_string());

    // Register callbacks
    let fn1 = registry.register_callback(Arc::new(|_| Ok(UpmValue::Null)));
    let fn2 = registry.register_callback(Arc::new(|_| Ok(UpmValue::Bool(true))));

    assert_eq!(registry.active_handle_count(), 4);

    // Release batch
    registry.release_handles(&[ref1.clone(), fn1.clone()]);
    assert_eq!(registry.active_handle_count(), 2);
    assert!(registry.get_callback(&fn1).is_none());
    assert!(registry.get_callback(&fn2).is_some());

    registry.release_handles(&[ref2, fn2]);
    assert_eq!(registry.active_handle_count(), 0);
}
