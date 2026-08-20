//! # Universal Package Platform (UPM / CPM)
//!
//! `upm` / `cpm` is a cross-language package platform implementation written in Rust based on the
//! **Universal Package Platform · Concept & Architecture** specification.
//!
//! ## Core Architecture Subsystems
//!
//! - **L2 Acquisition (`acquisition`)**: Multi-signal ecosystem detection engine, `upm.toml` manifest management,
//!   15+ package manager adapters (npm, pnpm, yarn, bun, pip, uv, poetry, cargo, go mod, maven, gradle, composer, bundler, nuget, pub, mix).
//! - **L2' Cross-Language Bridge (`bridge`)**: Stdio RPC framing protocol (`upm-bridge/1`), length-prefixed transport,
//!   `UpmValue` serialization (`$blob`, `$ref`, `$fn`), object handle GC registry, and language host process supervisors.
//! - **CLI Engine (`cli`)**: Complete suite of 26 subcommands covering installation, detection, security audits,
//!   diagnostics, benchmarking, flamegraphs, Kubernetes Helm/CRD generation, sccache compiler acceleration, and OTLP tracing.

pub mod acquisition;
pub mod bridge;
pub mod cli;

pub use acquisition::*;
pub use bridge::*;
