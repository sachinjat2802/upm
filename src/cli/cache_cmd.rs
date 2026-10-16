/// # `cpm cache` CLI Subcommand — Global Content-Addressable Cache Manager
///
/// Inspects, cleans, and prunes the global content-addressable package store (CPM CAS Engine).

use std::path::Path;

/// Manage global content-addressable cache.
pub fn execute_cache(_path: &Path, action: Option<&str>) -> anyhow::Result<()> {
    crate::acquisition::cache::execute_cache(action)
}
