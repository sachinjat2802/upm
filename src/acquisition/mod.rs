//! # L2 Universal Acquisition Engine
//!
//! The acquisition module handles language ecosystem detection using weighted scoring signals,
//! `upm.toml` manifest loading and scaffolding, and execution of package manager workflows
//! (`install`, `add`, `update`, `outdated`, `audit`, `run`).

pub mod adapter;
pub mod manifest;
pub mod runner;
pub mod scoring;

pub use adapter::{AdapterRegistry, EcosystemAdapter};
pub use manifest::UpmManifest;
pub use runner::AcquisitionRunner;
pub use scoring::{DetectionEngine, DetectionResult, EcosystemScore};
