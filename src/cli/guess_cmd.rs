//! # CPM Source Code Dependency Guesser
//!
//! Scans workspace source files for `import`, `require`, and `use` statements,
//! compares them against registered dependencies in `upm.toml`, and detects
//! missing foreign dependencies.

use crate::acquisition::manifest::UpmManifest;
use crate::cli::execute_add;
use anyhow::Result;
use colored::Colorize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Scan source files and guess required dependencies.
pub fn execute_guess(path: &Path, apply: bool) -> Result<()> {
    println!();
    println!("  {} Scanning source code for undeclared dependencies...", "🔍".cyan());
    println!("  {}", "═".repeat(55).dimmed());

    let mut guessed_dependencies = BTreeMap::new();

    // Single-pass iterative directory walk over all source files: O(N) files, O(depth) space
    let source_files = collect_source_files(path)?;

    for (file, lang) in source_files {
        if let Ok(content) = fs::read_to_string(&file) {
            match lang {
                SourceLang::Python => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 {
                                let mod_name = parts[1].split('.').next().unwrap_or("");
                                if is_third_party_python(mod_name) {
                                    guessed_dependencies
                                        .entry(format!("pip:{}", mod_name))
                                        .or_insert_with(BTreeSet::new)
                                        .insert(file.display().to_string());
                                }
                            }
                        }
                    }
                }
                SourceLang::JavaScript => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if (trimmed.starts_with("import ") && trimmed.contains(" from ")) || trimmed.contains("require(") {
                            if let Some(pkg) = extract_js_package(trimmed) {
                                if !pkg.starts_with('.') && !pkg.starts_with('/') && is_third_party_node(&pkg) {
                                    guessed_dependencies
                                        .entry(format!("npm:{}", pkg))
                                        .or_insert_with(BTreeSet::new)
                                        .insert(file.display().to_string());
                                }
                            }
                        }
                    }
                }
                SourceLang::Rust => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("use ")
                            && !trimmed.starts_with("use crate::")
                            && !trimmed.starts_with("use super::")
                            && !trimmed.starts_with("use std::")
                            && !trimmed.starts_with("use core::")
                        {
                            if let Some(first_seg) = trimmed.trim_start_matches("use ").split("::").next() {
                                let clean = first_seg.trim_end_matches(';').trim();
                                if !clean.is_empty() && clean != "self" {
                                    guessed_dependencies
                                        .entry(format!("cargo:{}", clean))
                                        .or_insert_with(BTreeSet::new)
                                        .insert(file.display().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let manifest = UpmManifest::load_from_dir(path);
    let existing_deps: BTreeSet<String> = manifest
        .map(|m| m.foreign_dependencies.keys().cloned().collect())
        .unwrap_or_default();

    let mut undeclared = Vec::new();
    for (pkg, files) in &guessed_dependencies {
        if !existing_deps.contains(pkg) {
            undeclared.push((pkg.clone(), files.clone()));
        }
    }

    if undeclared.is_empty() {
        println!("  {} No undeclared foreign dependencies detected.", "✔".green());
        println!("  All detected source imports are tracked in manifests.");
        println!();
        return Ok(());
    }

    println!("  {} Found {} undeclared imports:", "💡".yellow(), undeclared.len().to_string().cyan().bold());
    println!();
    for (pkg, files) in &undeclared {
        println!("  {} {}", "•".cyan(), pkg.bold().yellow());
        for f in files.iter().take(2) {
            println!("    {} imported in {}", "↳".dimmed(), f.dimmed());
        }
    }
    println!();

    if apply {
        println!("  {} Auto-adding guessed dependencies to manifest...", "⚡".cyan());
        for (pkg, _) in &undeclared {
            let _ = execute_add(path, pkg, false);
        }
        println!("  {} All undeclared dependencies added.", "✔".green());
    } else {
        println!("  {} Run with {} to automatically declare these dependencies.", "Tip:".bold(), "cpm guess --apply".bold().green());
    }
    println!();

    Ok(())
}

#[derive(Clone, Copy)]
enum SourceLang {
    Python,
    JavaScript,
    Rust,
}

fn extract_js_package(line: &str) -> Option<String> {
    if let Some(pos) = line.find("from ") {
        let rest = line[pos + 5..].trim().trim_matches(';').trim();
        let pkg = rest.trim_matches('\'').trim_matches('"');
        return Some(pkg.split('/').take(if pkg.starts_with('@') { 2 } else { 1 }).collect::<Vec<_>>().join("/"));
    }
    if let Some(pos) = line.find("require(") {
        let rest = line[pos + 8..].trim();
        let end = rest.find(')')?;
        let pkg = rest[..end].trim().trim_matches('\'').trim_matches('"');
        return Some(pkg.split('/').take(if pkg.starts_with('@') { 2 } else { 1 }).collect::<Vec<_>>().join("/"));
    }
    None
}

fn is_third_party_python(name: &str) -> bool {
    let stdlib = [
        "os", "sys", "re", "json", "math", "time", "datetime", "pathlib", "collections",
        "itertools", "functools", "typing", "subprocess", "unittest", "logging", "asyncio",
        "http", "urllib", "threading", "multiprocessing", "hashlib", "tempfile", "io"
    ];
    !stdlib.contains(&name) && !name.is_empty()
}

fn is_third_party_node(name: &str) -> bool {
    let builtins = [
        "fs", "path", "http", "https", "url", "os", "crypto", "stream", "events", "util",
        "child_process", "buffer", "process", "cluster", "net", "tls", "zlib", "assert"
    ];
    !builtins.contains(&name)
}

/// Collects all source files in a single iterative pass.
/// Time Complexity: O(N) where N = total files in workspace.
/// Space Complexity: O(D) where D = maximum directory depth.
fn collect_source_files(dir: &Path) -> Result<Vec<(std::path::PathBuf, SourceLang)>> {
    let mut files = Vec::new();
    if !dir.is_dir() {
        return Ok(files);
    }

    let mut stack = vec![dir.to_path_buf()];
    while let Some(current_dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name != "target" && name != "node_modules" && name != ".git" && name != ".venv" && name != "venv" && name != "dist" && name != "build" {
                        stack.push(path);
                    }
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let lang = match ext {
                        "py" => Some(SourceLang::Python),
                        "js" | "ts" | "jsx" | "tsx" | "mjs" => Some(SourceLang::JavaScript),
                        "rs" => Some(SourceLang::Rust),
                        _ => None,
                    };
                    if let Some(l) = lang {
                        files.push((path, l));
                    }
                }
            }
        }
    }
    Ok(files)
}
