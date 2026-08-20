//! # Universal Package Platform (UPM / CPM)
//!
//! `upm` is a cross-language package platform implementation written in Rust based on the
//! **Universal Package Platform · Concept & Architecture** specification.
//!
//! ## Core Architecture Layers
//!
//! - **L2 Acquisition (`acquisition`)**: Provides multi-signal ecosystem scoring across 20 package managers
//!   and 13 language ecosystems, `upm.toml` manifest management, and unified dependency execution.
//! - **L2' Invocation (`bridge`)**: Provides the `upm-bridge/1` cross-language stdio RPC protocol, length-prefixed
//!   framing, value serialization (`$blob`, `$ref`, `$fn`), object handle GC tracking, and language host supervisors.
//! - **CLI Layer (`cli`)**: Provides terminal subcommands (`init`, `detect`, `install`, `add`, `update`, `outdated`, `audit`, `run`, `bridge`).

pub mod acquisition;
pub mod bridge;
pub mod cli;

pub use acquisition::*;
pub use bridge::*;
