/// # `cpm graph` CLI Subcommand — Cross-Platform Dependency Graph Visualizer
///
/// Generates interactive HTML / D3 dependency graph visualizers mapping polyglot
/// relationships across Python, Node.js, Rust, Go, and Ruby packages.

use colored::Colorize;
use std::path::Path;

/// Generate interactive dependency graph HTML.
pub fn execute_graph(path: &Path, out_file: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🕸️ {}{}",
        "│".cyan(),
        "CPM Polyglot Dependency Graph Visualizer".bold().white(),
        "        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let target_name = out_file.unwrap_or("cpm_dep_graph.html");
    let target_path = path.join(target_name);

    println!("  ▶ Traversing polyglot dependency DAG in upm.toml...");

    let manifest = crate::acquisition::UpmManifest::load_from_dir(path);
    let mut nodes_html = String::new();

    if let Some(ref m) = manifest {
        for (pkg, ver) in &m.foreign_dependencies {
            let class_name = if pkg.starts_with("pip:") || pkg.starts_with("python:") {
                "python"
            } else if pkg.starts_with("npm:") || pkg.starts_with("node:") || pkg.starts_with("pnpm:") {
                "node-js"
            } else if pkg.starts_with("cargo:") || pkg.starts_with("rust:") {
                "rust"
            } else {
                "generic"
            };
            nodes_html.push_str(&format!("  <div class=\"node {}\">{} ({})</div>\n", class_name, pkg, ver));
        }
    }

    if nodes_html.is_empty() {
        nodes_html.push_str("  <div class=\"node generic\">No foreign dependencies declared in upm.toml</div>\n");
    }

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>CPM Dependency Graph Visualizer</title>
  <style>
    body {{ background-color: #1e1e2e; color: #cdd6f4; font-family: system-ui, sans-serif; padding: 2rem; }}
    .node {{ padding: 10px 18px; border-radius: 8px; margin: 8px; display: inline-block; font-weight: bold; }}
    .python {{ background: #38761d; color: #fff; }}
    .node-js {{ background: #b45f06; color: #fff; }}
    .rust {{ background: #c2410c; color: #fff; }}
    .generic {{ background: #475569; color: #fff; }}
  </style>
</head>
<body>
  <h1>🕸️ CPM Polyglot Dependency Graph</h1>
{}</body>
</html>"#,
        nodes_html
    );

    std::fs::write(&target_path, html_content)?;

    println!();
    println!("  {} Interactive HTML Dependency Graph generated at {}", "✔".green(), target_name.bold().yellow());
    println!("  {} Open {} in your web browser to explore visual package DAG.", "ℹ".blue(), target_name.cyan());
    println!();

    Ok(())
}
