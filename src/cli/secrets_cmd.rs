/// # `cpm scan-secrets` CLI Subcommand — Secret & API Key Leak Scanner
///
/// Scans workspace source files for hardcoded API keys, JWT tokens, private SSH keys,
/// and cloud credentials before publishing or committing code.

use colored::Colorize;
use std::path::Path;

/// Known secret patterns to match without external dependencies.
struct SecretDetector {
    name: &'static str,
    checker: fn(&str) -> bool,
}

const DETECTORS: &[SecretDetector] = &[
    SecretDetector {
        name: "AWS Access Key",
        checker: |line| {
            if let Some(idx) = line.find("AKIA") {
                let tail = &line[idx + 4..];
                tail.chars().take(16).all(|c| c.is_ascii_alphanumeric() && c.is_uppercase()) && tail.len() >= 16
            } else {
                false
            }
        },
    },
    SecretDetector {
        name: "GitHub Personal Token",
        checker: |line| line.contains("ghp_") && line.chars().skip_while(|c| *c != 'g').take(40).count() >= 36,
    },
    SecretDetector {
        name: "Generic Private Key",
        checker: |line| line.contains("-----BEGIN") && line.contains("PRIVATE KEY-----"),
    },
    SecretDetector {
        name: "OpenAI API Key",
        checker: |line| line.contains("sk-") && line.len() > 30,
    },
    SecretDetector {
        name: "Slack Token",
        checker: |line| line.contains("xoxb-") || line.contains("xoxp-") || line.contains("xapp-"),
    },
    SecretDetector {
        name: "Google API Key",
        checker: |line| line.contains("AIzaSy"),
    },
    SecretDetector {
        name: "Stripe Secret Key",
        checker: |line| line.contains("sk_live_") || line.contains("rk_live_"),
    },
    SecretDetector {
        name: "JWT Token",
        checker: |line| line.contains("eyJ") && line.split('.').count() >= 3,
    },
    SecretDetector {
        name: "Generic Secret Assignment",
        checker: |line| {
            let lower = line.to_lowercase();
            (lower.contains("api_key") || lower.contains("secret_key") || lower.contains("password=") || lower.contains("access_token"))
                && (line.contains("=\"") || line.contains("='") || line.contains(": \"") || line.contains(": '"))
        },
    },
];

/// File extensions to scan (skip binary, media, and dependency directories).
const SCANNABLE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "jsx", "tsx", "java", "go", "rb", "php",
    "yaml", "yml", "json", "toml", "env", "cfg", "ini", "conf",
    "sh", "bash", "ps1", "bat", "cmd", "txt", "md", "html",
];

/// Directories to skip during scanning.
const SKIP_DIRS: &[&str] = &[
    "node_modules", ".venv", "venv", "target", ".git", ".hg",
    "__pycache__", ".tox", "dist", "build", "vendor", ".gradle",
    "_build", "deps", ".dart_tool", "obj", "bin",
];

struct SecretFinding {
    file: String,
    line_number: usize,
    pattern_name: String,
    preview: String,
}

/// Execute secret leak scan across workspace.
pub fn execute_scan_secrets(path: &Path) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  🛡️ {}{}",
        "│".cyan(),
        "CPM Secret & API Key Leak Security Scanner".bold().white(),
        "        │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    println!("  ▶ Scanning workspace files for hardcoded credentials...");
    println!("    {} Loaded {} secret detection rules", "ℹ".blue(), DETECTORS.len());
    println!();

    let mut findings: Vec<SecretFinding> = Vec::new();
    let mut files_scanned = 0;

    // Recursively walk the workspace
    scan_directory(path, path, &mut findings, &mut files_scanned);

    // Report results
    println!("  {} Scanned {} files across workspace.", "✔".green(), files_scanned.to_string().bold());
    println!();

    if findings.is_empty() {
        println!("  {} Security Scan Passed: 0 hardcoded secrets or API keys detected!", "✔".bold().green());
    } else {
        println!("  {} Found {} potential secret leak(s):", "✖".bold().red(), findings.len().to_string().bold().yellow());
        println!();
        for finding in &findings {
            println!("    {} {} in {}", "⚠".yellow(), finding.pattern_name.bold().red(), finding.file.cyan());
            println!("      Line {}: {}", finding.line_number, finding.preview.dimmed());
        }
        println!();
        println!("  {} Remove hardcoded secrets and use environment variables instead.", "💡".yellow());
        println!("  {} Consider using a secrets manager (e.g., AWS SSM, HashiCorp Vault, Doppler).", "💡".yellow());
    }
    println!();

    Ok(())
}

/// Recursively scan a directory for secret patterns.
fn scan_directory(
    root: &Path,
    dir: &Path,
    findings: &mut Vec<SecretFinding>,
    files_scanned: &mut usize,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if entry_path.is_dir() {
            // Skip known dependency/build directories
            if SKIP_DIRS.contains(&file_name.as_str()) {
                continue;
            }
            scan_directory(root, &entry_path, findings, files_scanned);
        } else if entry_path.is_file() {
            // Check file extension
            let ext = entry_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            // Also scan extensionless dotfiles like .env
            let is_dotenv = file_name.starts_with(".env");
            if !is_dotenv && !SCANNABLE_EXTENSIONS.contains(&ext) {
                continue;
            }

            // Read file and scan lines
            if let Ok(content) = std::fs::read_to_string(&entry_path) {
                *files_scanned += 1;
                let relative = entry_path
                    .strip_prefix(root)
                    .unwrap_or(&entry_path)
                    .display()
                    .to_string();

                for (line_num, line) in content.lines().enumerate() {
                    for detector in DETECTORS {
                        if (detector.checker)(line) {
                            // Truncate the preview for readability
                            let preview = if line.len() > 80 {
                                format!("{}...", &line[..80])
                            } else {
                                line.to_string()
                            };
                            findings.push(SecretFinding {
                                file: relative.clone(),
                                line_number: line_num + 1,
                                pattern_name: detector.name.to_string(),
                                preview: preview.trim().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}
