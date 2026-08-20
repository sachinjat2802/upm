/// # Object Handle Registry
///
/// Manages the lifecycle of opaque object references (`$ref`) and
/// callback functions (`$fn`) that flow across the bridge protocol.
///
/// When a host returns a `$ref` or `$fn` value, the handle ID is
/// registered here. The foreign side can invoke `$fn` callbacks or
/// release handles via `ReleaseHandles` messages for GC.

use crate::bridge::value::UpmValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Type alias for native callback closures that can be invoked
/// from foreign language hosts via `$fn:id` method calls.
pub type NativeCallback = Arc<dyn Fn(Vec<UpmValue>) -> Result<UpmValue, String> + Send + Sync>;

/// Thread-safe registry for managing object and callback handles.
///
/// Handles are UUID-prefixed identifiers (`ref_<uuid>` for objects,
/// `fn_<uuid>` for callbacks) stored in `Mutex<HashMap>`s.
pub struct HandleRegistry {
    objects: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    callbacks: Mutex<HashMap<String, NativeCallback>>,
}

impl Default for HandleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleRegistry {
    /// Create an empty handle registry.
    pub fn new() -> Self {
        Self {
            objects: Mutex::new(HashMap::new()),
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    /// Register a typed object and return its handle ID.
    pub fn register_object<T: 'static + Send + Sync>(&self, object: T) -> String {
        let handle_id = format!("ref_{}", Uuid::new_v4());
        let mut map = self.objects.lock().unwrap();
        map.insert(handle_id.clone(), Box::new(object));
        handle_id
    }

    /// Register a callback closure and return its handle ID.
    pub fn register_callback(&self, cb: NativeCallback) -> String {
        let fn_id = format!("fn_{}", Uuid::new_v4());
        let mut map = self.callbacks.lock().unwrap();
        map.insert(fn_id.clone(), cb);
        fn_id
    }

    /// Look up a registered callback by its handle ID.
    pub fn get_callback(&self, id: &str) -> Option<NativeCallback> {
        let map = self.callbacks.lock().unwrap();
        map.get(id).cloned()
    }

    /// Release a batch of handles (both objects and callbacks).
    ///
    /// This is triggered by `ReleaseHandles` messages from the foreign
    /// side, enabling cooperative garbage collection.
    pub fn release_handles(&self, ids: &[String]) {
        let mut objs = self.objects.lock().unwrap();
        let mut cbs = self.callbacks.lock().unwrap();
        for id in ids {
            objs.remove(id);
            cbs.remove(id);
        }
    }

    /// Return the total number of active handles (objects + callbacks).
    pub fn active_handle_count(&self) -> usize {
        let objs = self.objects.lock().unwrap().len();
        let cbs = self.callbacks.lock().unwrap().len();
        objs + cbs
    }
}
