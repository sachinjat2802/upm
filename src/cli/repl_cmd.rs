/// # `cpm repl` CLI Subcommand — Interactive Polyglot REPL
///
/// Provides a real-time interactive shell for calling foreign RPC methods,
/// inspecting bridge hosts, and exploring cross-language functionality.

use crate::bridge::host::HostSupervisor;
use crate::bridge::value::UpmValue;
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;

/// Launch the interactive polyglot REPL.
pub async fn execute_repl() -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  💬 {}{}",
        "│".cyan(),
        "CPM Interactive Polyglot REPL Shell".bold().white(),
        "            │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();
    println!("  Type {} to see examples, or {} to quit.", "help".bold().yellow(), "exit".bold().red());
    println!();

    let stdin = io::stdin();

    loop {
        print!("  {} ", "cpm>".bold().magenta());
        io::stdout().flush().unwrap_or(());

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break; // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            "exit" | "quit" | "q" => {
                println!("  Goodbye! 👋");
                println!();
                break;
            }
            "help" | "?" => {
                println!("  {}", "REPL Command Examples:".bold().yellow());
                println!("    {} Call python math.sqrt", "cpm> python:math.sqrt [144]".dimmed());
                println!("    {} Call node sha256", "cpm> node:crypto.sha256 [\"hello\"]".dimmed());
                println!("    {} Inspect registered methods", "cpm> inspect python".dimmed());
                println!("    {} Exit the REPL", "cpm> exit".dimmed());
                println!();
            }
            input if input.starts_with("inspect ") => {
                let lang = input.trim_start_matches("inspect ").trim();
                let host_file = match lang {
                    "python" | "py" => "hosts/python_host.py",
                    "node" | "js" => "hosts/node_host.js",
                    "go" => "hosts/go_host.go",
                    "ruby" | "rb" => "hosts/ruby_host.rb",
                    _ => "",
                };
                if host_file.is_empty() {
                    println!("  {} Unknown language '{}'", "✖".bold().red(), lang);
                } else if let Ok(host) = HostSupervisor::spawn_host(lang, Path::new(host_file)).await {
                    if let Ok(res) = host.peer.call("__inspect__", vec![]).await {
                        println!("  {} Inspect result for [{}]:", "✔".green(), lang.bold());
                        println!("    {}", serde_json::to_string_pretty(&res).unwrap_or_default().white());
                    }
                }
                println!();
            }
            target_expr if target_expr.contains(':') => {
                let parts: Vec<&str> = target_expr.splitn(2, ' ').collect();
                let target = parts[0];
                let json_args_str = if parts.len() > 1 { parts[1] } else { "[]" };

                let target_parts: Vec<&str> = target.splitn(2, ':').collect();
                let lang = target_parts[0];
                let method = target_parts[1];

                let host_file = match lang {
                    "python" | "py" => "hosts/python_host.py",
                    "node" | "js" => "hosts/node_host.js",
                    "go" => "hosts/go_host.go",
                    "ruby" | "rb" => "hosts/ruby_host.rb",
                    _ => "",
                };

                if host_file.is_empty() {
                    println!("  {} Unsupported language '{}'", "✖".bold().red(), lang);
                    continue;
                }

                let args: Vec<UpmValue> = serde_json::from_str(json_args_str).unwrap_or_else(|_| vec![]);

                let t0 = std::time::Instant::now();
                if let Ok(host) = HostSupervisor::spawn_host(lang, Path::new(host_file)).await {
                    match host.peer.call(method, args).await {
                        Ok(res) => {
                            let elapsed = t0.elapsed().as_micros();
                            println!("  {} Result: {}", "✔".green(), serde_json::to_string_pretty(&res).unwrap_or_default().bold().white());
                            println!("  {} ({:.0}µs round-trip)", "⏱".dimmed(), elapsed);
                        }
                        Err(e) => {
                            println!("  {} RPC Error: {}", "✖".bold().red(), e);
                        }
                    }
                } else {
                    println!("  {} Could not spawn {} host", "✖".bold().red(), lang);
                }
                println!();
            }
            other => {
                println!("  {} Unrecognized REPL command: '{}'. Type 'help' for examples.", "✖".bold().red(), other);
                println!();
            }
        }
    }

    Ok(())
}
