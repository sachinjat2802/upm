/// # `cpm bridge benchmark` CLI Subcommand — RPC Performance Profiler
///
/// Runs microsecond-level latency and throughput benchmarking across stdio RPC
/// bridge calls to Python and Node language hosts.

use crate::bridge::host::HostSupervisor;
use crate::bridge::value::UpmValue;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

/// Execute benchmark profiling iterations against a foreign host process.
pub async fn execute_benchmark(language: &str, iterations: usize) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ⚡ {}{}",
        "│".cyan(),
        format!("Bridge Performance Profiler Benchmark [{}]", language).bold().white(),
        "     │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let host_file = match language {
        "python" | "py" => "hosts/python_host.py",
        "node" | "js" => "hosts/node_host.js",
        other => anyhow::bail!("Unsupported benchmark language: {}", other),
    };

    let host_path = Path::new(host_file);
    if !host_path.exists() {
        anyhow::bail!("Host script not found at {}", host_file);
    }

    print!("  ▶ Spawning {} host supervisor... ", language.cyan());
    let host = HostSupervisor::spawn_host(language, host_path).await?;
    println!("{}", "connected".green().bold());
    println!("  ▶ Running {} RPC iterations...", iterations.to_string().bold().yellow());
    println!();

    let mut latencies_micros: Vec<u128> = Vec::with_capacity(iterations);
    let start_total = Instant::now();

    for i in 0..iterations {
        let t0 = Instant::now();
        let _res = host.peer.call("echo", vec![UpmValue::String(format!("benchmark_payload_{}", i))]).await?;
        let micros = t0.elapsed().as_micros();
        latencies_micros.push(micros);
    }

    let total_micros = start_total.elapsed().as_micros();
    let mean_micros = total_micros as f64 / iterations as f64;

    latencies_micros.sort_unstable();
    let min_micros = latencies_micros.first().copied().unwrap_or(0);
    let max_micros = latencies_micros.last().copied().unwrap_or(0);
    let p95_micros = latencies_micros.get((iterations as f64 * 0.95) as usize).copied().unwrap_or(max_micros);

    let ops_per_sec = (iterations as f64 / (total_micros as f64 / 1_000_000.0)).round();

    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📊 Benchmark Results for [{}]", "│".cyan(), language.bold().magenta());
    println!("  {}", "├──────────────────────────────────────────────────────┤".cyan());
    println!("  {}  Iterations:          {}", "│".cyan(), iterations.to_string().bold());
    println!("  {}  Ops / Second:        {} ops/sec", "│".cyan(), ops_per_sec.to_string().bold().green());
    println!("  {}  Mean Latency:        {:.1} µs", "│".cyan(), mean_micros);
    println!("  {}  Min Latency:         {} µs", "│".cyan(), min_micros);
    println!("  {}  Max Latency:         {} µs", "│".cyan(), max_micros);
    println!("  {}  P95 Latency:         {} µs", "│".cyan(), p95_micros);
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    Ok(())
}
