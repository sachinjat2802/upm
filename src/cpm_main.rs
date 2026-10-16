//! # CPM Binary Entry Point
//!
//! Command-line interface driver for `cpm` (Cross-language Package Manager).

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    upm::cli::run_cli_app("cpm").await
}
