/// # `cpm generate-stubs` CLI Subcommand — IDE Type Stub Generator
///
/// Queries foreign bridge hosts via dynamic inspection (`__inspect__`) and
/// exports TypeScript type definitions (`.d.ts`) and Python type stubs (`.pyi`).

use crate::bridge::host::HostSupervisor;
use crate::bridge::value::UpmValue;
use colored::Colorize;
use std::path::Path;

/// Generate IDE type stubs for Python and Node.js bridge hosts.
pub async fn execute_generate_stubs(out_dir: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  📝 {}{}",
        "│".cyan(),
        "IDE Type Stub Generator (.d.ts & .pyi)".bold().white(),
        "             │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    std::fs::create_dir_all(out_dir)?;

    // 1. Generate Python stubs (.pyi)
    let py_host_path = Path::new("hosts/python_host.py");
    if py_host_path.exists() {
        print!("  ▶ Inspecting Python host methods... ");
        if let Ok(host) = HostSupervisor::spawn_host("python", py_host_path).await {
            let res = host.peer.call("__inspect__", vec![]).await?;
            if let UpmValue::Array(items) = res {
                let mut pyi_content = String::from("# CPM Auto-Generated Python Type Stubs (.pyi)\n\n");
                for item in items {
                    if let UpmValue::Map(m) = item {
                        if let Some(UpmValue::String(name)) = m.get("name") {
                            let doc = m.get("description").and_then(|v| match v { UpmValue::String(s) => Some(s.as_str()), _ => None }).unwrap_or("");
                            pyi_content.push_str(&format!("def {}(*args, **kwargs):\n    \"\"\"{}\"\"\"\n    ...\n\n", name.replace('.', "_"), doc));
                        }
                    }
                }
                let pyi_file = out_dir.join("cpm_bridge.pyi");
                std::fs::write(&pyi_file, pyi_content)?;
                println!("{}", "done".green().bold());
                println!("    ✔ Exported {}", pyi_file.display().to_string().cyan());
            }
        }
    }

    // 2. Generate Node.js / TypeScript stubs (.d.ts)
    let node_host_path = Path::new("hosts/node_host.js");
    if node_host_path.exists() {
        print!("  ▶ Inspecting Node.js host methods... ");
        if let Ok(host) = HostSupervisor::spawn_host("node", node_host_path).await {
            let res = host.peer.call("__inspect__", vec![]).await?;
            if let UpmValue::Array(items) = res {
                let mut dts_content = String::from("// CPM Auto-Generated TypeScript Type Definitions (.d.ts)\n\nexport namespace CpmBridge {\n");
                for item in items {
                    if let UpmValue::Map(m) = item {
                        if let Some(UpmValue::String(name)) = m.get("name") {
                            let doc = m.get("description").and_then(|v| match v { UpmValue::String(s) => Some(s.as_str()), _ => None }).unwrap_or("");
                            dts_content.push_str(&format!("  /** {} */\n  export function {}(...args: any[]): Promise<any>;\n", doc, name.replace('.', "_")));
                        }
                    }
                }
                dts_content.push_str("}\n");
                let dts_file = out_dir.join("cpm_bridge.d.ts");
                std::fs::write(&dts_file, dts_content)?;
                println!("{}", "done".green().bold());
                println!("    ✔ Exported {}", dts_file.display().to_string().cyan());
            }
        }
    }

    println!();
    println!("  {} IDE type stubs generated successfully at {}", "✔".green(), out_dir.display().to_string().bold());
    println!();

    Ok(())
}
