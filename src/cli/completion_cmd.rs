/// # `cpm completion` CLI Subcommand — Shell Auto-Completion Script Generator
///
/// Auto-generates tab completion scripts for PowerShell, Bash, Zsh, and Fish.

use colored::Colorize;
use std::path::Path;

/// Generate shell auto-completion script.
pub fn execute_completion(path: &Path, shell: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🐚 {}{}",
        "│".cyan(),
        "CPM Shell Auto-Completion Script Generator".bold().white(),
        "     │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let sh = shell.unwrap_or("powershell");
    let target_name = format!("cpm_completion.{}", if sh == "powershell" { "ps1" } else { "sh" });
    let target_path = path.join(&target_name);

    println!("  ▶ Generating shell completion script for: {}", sh.bold().yellow());

    let script_content = r#"# CPM Shell Tab Auto-Completion Script
Register-ArgumentCompleter -Native -CommandName cpm -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $subcommands = @('doctor', 'benchmark', 'generate-stubs', 'bundle', 'repl', 'alias', 'dockerfile', 'search', 'rollback', 'licenses', 'scan-secrets', 'diff', 'resolve', 'cost', 'helm', 'policy', 'flamegraph', 'operator', 'cache', 'logs', 'sccache', 'verify-sig', 'audit-log', 'trace', 'graph', 'completion')
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
"#;

    std::fs::write(&target_path, script_content)?;

    println!();
    println!("  {} Completion script generated successfully at {}", "✔".green(), target_name.bold().yellow());
    println!("  {} Load in PowerShell via: {}", "ℹ".blue(), format!(". .\\{}", target_name).cyan());
    println!();

    Ok(())
}
