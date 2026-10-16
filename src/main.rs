//! # UPM Binary Entry Point
//!
//! Command-line interface driver for `upm` (Universal Package Platform).

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    upm::cli::run_cli_app("upm").await
}
