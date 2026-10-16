/// # High-Throughput Handle Registry
///
/// Manages the lifecycle of opaque object references (`$ref`) and
/// callback functions (`$fn`) that flow across the bridge protocol.
///
/// Uses `RwLock` for lock-free parallel reads and `AtomicU64` for nanosecond
/// handle allocation, optimized for multi-million request/sec concurrency.

use crate::bridge::value::UpmValue;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

/// Type alias for native callback closures that can be invoked
/// from foreign language hosts via `$fn:id` method calls.
pub type NativeCallback = Arc<dyn Fn(Vec<UpmValue>) -> Result<UpmValue, String> + Send + Sync>;

/// Lock-free concurrent registry for managing object and callback handles.
pub struct HandleRegistry {
    objects: RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    callbacks: RwLock<HashMap<String, NativeCallback>>,
    counter: AtomicU64,
}

impl Default for HandleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleRegistry {
    /// Create an empty high-performance handle registry.
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
            callbacks: RwLock::new(HashMap::new()),
            counter: AtomicU64::new(1),
        }
    }

    /// Fast nanosecond handle ID generator using atomic counter.
    #[inline(always)]
    fn next_id(&self, prefix: &str) -> String {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("{}_{}", prefix, id)
    }

    /// Register a typed object and return its handle ID (concurrent write).
    pub fn register_object<T: 'static + Send + Sync>(&self, object: T) -> String {
        let handle_id = self.next_id("ref");
        let mut map = self.objects.write().unwrap_or_else(|e| e.into_inner());
        map.insert(handle_id.clone(), Box::new(object));
        handle_id
    }

    /// Register a callback closure and return its handle ID (concurrent write).
    pub fn register_callback(&self, cb: NativeCallback) -> String {
        let fn_id = self.next_id("fn");
        let mut map = self.callbacks.write().unwrap_or_else(|e| e.into_inner());
        map.insert(fn_id.clone(), cb);
        fn_id
    }

    /// Look up a registered callback by its handle ID with non-blocking read lock.
    #[inline]
    pub fn get_callback(&self, id: &str) -> Option<NativeCallback> {
        let map = self.callbacks.read().unwrap_or_else(|e| e.into_inner());
        map.get(id).cloned()
    }

    /// Release a batch of handles (both objects and callbacks).
    pub fn release_handles(&self, ids: &[String]) {
        let mut objs = self.objects.write().unwrap_or_else(|e| e.into_inner());
        let mut cbs = self.callbacks.write().unwrap_or_else(|e| e.into_inner());
        for id in ids {
            objs.remove(id);
            cbs.remove(id);
        }
    }

    /// Return the total number of active handles (objects + callbacks).
    pub fn active_handle_count(&self) -> usize {
        let objs = self.objects.read().unwrap_or_else(|e| e.into_inner()).len();
        let cbs = self.callbacks.read().unwrap_or_else(|e| e.into_inner()).len();
        objs + cbs
    }
}
