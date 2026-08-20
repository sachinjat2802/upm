/// # `cpm cost` CLI Subcommand — Cloud Cost & Memory Estimator
///
/// Estimates cloud deployment costs, RAM usage footprint, and container sizes
/// for polyglot microservice deployments.

use crate::acquisition::{AdapterRegistry, DetectionEngine};
use colored::Colorize;
use std::path::Path;

/// Execute cloud cost and RAM estimation.
pub fn execute_cost(path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  💰 {}{}",
        "│".cyan(),
        "CPM Cloud Cost & Resource Footprint Estimator".bold().white(),
        "     │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    println!("  ▶ Analyzing active runtimes for memory footprint estimation...");
    println!();

    let mut total_est_ram_mb = 15; // Base Rust binary

    for eco in &result.detected_ecosystems {
        let ram = match eco.language.as_str() {
            "javascript" | "typescript" => 45,
            "python" => 35,
            "java" => 120,
            "go" => 20,
            "ruby" => 40,
            _ => 30,
        };
        total_est_ram_mb += ram;
        println!("    • {:<20} → Est. RAM: ~{} MB", eco.display_name.bold().magenta(), ram.to_string().bold().yellow());
    }

    let est_monthly_cost = (total_est_ram_mb as f64 / 1024.0) * 8.50;

    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📊 Projected Cloud Deployment Footprint", "│".cyan());
    println!("  {}", "├──────────────────────────────────────────────────────┤".cyan());
    println!("  {}  Total Estimated RAM:   ~{} MB", "│".cyan(), total_est_ram_mb.to_string().bold().cyan());
    println!("  {}  Est. Container Size:   ~185 MB", "│".cyan());
    println!("  {}  Est. Monthly Cost:     ~${:.2} / month (AWS Fargate/GCP)", "│".cyan(), est_monthly_cost);
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    Ok(())
}
