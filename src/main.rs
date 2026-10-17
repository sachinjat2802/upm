//! # CPM CLI Binary Entry Point
//!
//! Command-line interface driver for `cpm` (Cross-Language Package Manager).

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    upm::cli::run_cli_app("upm").await
}
