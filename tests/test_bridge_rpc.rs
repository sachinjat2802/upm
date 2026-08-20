use std::sync::Arc;
use upm::bridge::{HandleRegistry, UpmValue};

#[test]
fn test_upm_value_serialization() {
    let num = UpmValue::Number(42.0);
    let json = serde_json::to_string(&num).unwrap();
    assert_eq!(json, "42.0");

    let blob = UpmValue::blob_from_bytes(b"hello world");
    let json_blob = serde_json::to_string(&blob).unwrap();
    assert!(json_blob.contains("$blob"));

    let obj_ref = UpmValue::object_ref("ref_123", Some("Counter".into()));
    let json_ref = serde_json::to_string(&obj_ref).unwrap();
    assert!(json_ref.contains("$ref"));

    let fn_cb = UpmValue::fn_callback("fn_456");
    let json_fn = serde_json::to_string(&fn_cb).unwrap();
    assert!(json_fn.contains("$fn"));
}

#[test]
fn test_handle_registry() {
    let registry = HandleRegistry::new();
    let cb_id = registry.register_callback(Arc::new(|args| {
        let val = match args.get(0) {
            Some(UpmValue::Number(n)) => n + 1.0,
            _ => 0.0,
        };
        Ok(UpmValue::Number(val))
    }));

    assert!(cb_id.starts_with("fn_"));
    let cb = registry.get_callback(&cb_id).unwrap();
    let res = cb(vec![UpmValue::Number(10.0)]).unwrap();
    assert_eq!(res, UpmValue::Number(11.0));

    registry.release_handles(&[cb_id.clone()]);
    assert!(registry.get_callback(&cb_id).is_none());
}
