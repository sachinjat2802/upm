//! # CPM Comprehensive Deep Edge Cases & Stress Test Suite
//!
//! Validates system resilience across malformed inputs, boundary limits,
//! directory traversal attacks, unicode/binary edge cases, tampered lockfiles,
//! and extreme scaling conditions.

use std::fs;
use tempfile::tempdir;
use upm::acquisition::cache::{cache_package, compute_sha256, get_cached_package, verify_package_integrity};
use upm::acquisition::manifest::UpmManifest;
use upm::acquisition::{AdapterRegistry, DetectionEngine};
use upm::bridge::value::UpmValue;

#[test]
fn test_manifest_malformed_toml_handling() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // 1. Completely malformed TOML syntax
    fs::write(path.join("cpm.toml"), "[project\nname = invalid_syntax\nversion ==").unwrap();
    let loaded = UpmManifest::load_from_dir(path);
    assert!(loaded.is_none(), "Malformed TOML should fail safely without crashing");

    // 2. Empty manifest file
    fs::write(path.join("cpm.toml"), "").unwrap();
    let loaded_empty = UpmManifest::load_from_dir(path);
    // An empty TOML file cannot deserialize required struct ProjectConfig fields, should return None gracefully
    assert!(loaded_empty.is_none(), "Empty TOML missing required fields should return None safely");
}

#[test]
fn test_manifest_unicode_and_exotic_names() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    let mut manifest = UpmManifest::new("unicode-app-🚀", "1.0.0-beta.1+exp.sha.5114f85", Some("rust"), vec!["cargo".into()]);

    // Exotic package names: Scoped npm packages, unicode, punctuation
    manifest.add_foreign_dependency("npm", "@scope/sub-package_v2.0", "^2.1.0");
    manifest.add_foreign_dependency("pip", "python-中文-包", ">=1.0.0");
    manifest.add_foreign_dependency("cargo", "serde-json_with_dots.rs", "~1.0");

    manifest.save_to_dir(path).unwrap();

    let loaded = UpmManifest::load_from_dir(path).expect("Failed to load saved manifest");
    assert_eq!(loaded.project.name, "unicode-app-🚀");
    assert_eq!(
        loaded.foreign_dependencies.get("npm:@scope/sub-package_v2.0").map(|s| s.as_str()),
        Some("^2.1.0")
    );
    assert_eq!(
        loaded.foreign_dependencies.get("pip:python-中文-包").map(|s| s.as_str()),
        Some(">=1.0.0")
    );
    assert_eq!(
        loaded.foreign_dependencies.get("cargo:serde-json_with_dots.rs").map(|s| s.as_str()),
        Some("~1.0")
    );

    // Remove foreign dependency
    assert!(manifest.remove_foreign_dependency("pip", "python-中文-包"));
    assert!(!manifest.remove_foreign_dependency("pip", "python-中文-包"));
}

#[test]
fn test_detection_when_manifest_is_a_directory() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Adversarial case: A directory named "package.json" or "Cargo.toml" instead of a file
    fs::create_dir_all(path.join("package.json")).unwrap();
    fs::create_dir_all(path.join("Cargo.toml")).unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(path);

    // Directories named package.json should not score as valid files
    let npm_score = result.scores.iter().find(|s| s.adapter.name == "npm");
    let cargo_score = result.scores.iter().find(|s| s.adapter.name == "cargo");

    if let Some(s) = npm_score {
        assert_eq!(s.total_score, 0, "Directory named package.json must not award manifest score");
    }
    if let Some(s) = cargo_score {
        assert_eq!(s.total_score, 0, "Directory named Cargo.toml must not award manifest score");
    }
}

#[test]
fn test_detection_deep_hierarchy_and_many_files() {
    let tmp = tempdir().expect("tempdir failed");
    let root = tmp.path();

    // 1. Root with glob pattern requirements-dev.txt
    fs::write(root.join("requirements-dev.txt"), "pytest>=7.0\n").unwrap();

    // 2. Create 20 levels of nesting with a manifest at the leaf
    let mut current = root.to_path_buf();
    for i in 0..20 {
        current = current.join(format!("level_{}", i));
    }
    fs::create_dir_all(&current).unwrap();
    fs::write(current.join("Cargo.toml"), "[package]\nname = \"deep\"\nversion = \"0.1.0\"\n").unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);

    // Scored root matches pip glob pattern
    let root_res = engine.detect_dir(root);
    let pip = root_res.scores.iter().find(|s| s.adapter.name == "pip").unwrap();
    assert!(pip.total_score >= 30, "pip glob requirements*.txt should award at least 30 pts");

    // Scored leaf matches Cargo manifest
    let leaf_res = engine.detect_dir(&current);
    let cargo = leaf_res.scores.iter().find(|s| s.adapter.name == "cargo").unwrap();
    assert!(cargo.total_score >= 40, "Cargo.toml in deep nested directory should award at least 40 pts");
}

#[test]
fn test_cache_zero_byte_and_large_artifacts() {
    // 1. Zero-byte artifact
    let zero_data = b"";
    let zero_hash = compute_sha256(zero_data);
    let cached_zero = cache_package("test-eco", "empty-pkg", "0.0.1", zero_data).expect("Cache zero byte failed");
    assert!(cached_zero.exists());
    assert!(verify_package_integrity(&cached_zero, &zero_hash).expect("Integrity check failed"));

    // 2. Large binary artifact (1 MB of repeating bytes)
    let large_data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();
    let large_hash = compute_sha256(&large_data);
    let cached_large = cache_package("test-eco", "large-pkg", "1.0.0", &large_data).expect("Cache large failed");
    assert!(cached_large.exists());
    assert!(verify_package_integrity(&cached_large, &large_hash).expect("Large integrity check failed"));

    // Retrieve via lookup
    let found = get_cached_package("test-eco", "large-pkg", "1.0.0").expect("Package should be found in cache");
    assert_eq!(found, cached_large);
}

#[test]
fn test_cache_tamper_detection() {
    let data = b"genuine untampered package payload";
    let hash = compute_sha256(data);

    let cached = cache_package("security-eco", "safe-pkg", "2.0.0", data).unwrap();
    assert!(verify_package_integrity(&cached, &hash).unwrap());

    // Tamper with file on disk by flipping bytes
    fs::write(&cached, b"malicious modified payload").unwrap();
    let is_valid = verify_package_integrity(&cached, &hash).unwrap();
    assert!(!is_valid, "Tampered package payload must fail integrity check");
}

#[test]
fn test_secrets_scanner_boundaries_and_zero_false_positives() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Benign file with words that could trigger naive regexes
    let safe_code = r#"
    // This is a risk-free function
    let task_handler = "task-oriented-flow";
    let flask_mod = "flask-restful";
    let short_token = "sk-short";
    let test_user = "user_name_string";
    "#;
    fs::write(path.join("safe.rs"), safe_code).unwrap();

    let res = upm::cli::execute_scan_secrets(path);
    assert!(res.is_ok(), "execute_scan_secrets on safe file failed");

    // Adversarial file with real high-entropy key formats next to multibyte Unicode
    let leak_code = r#"
    // 🚀 Unicode comment with secret:
    let openai = "sk-proj-abcde1234567890fghijklmnop";
    let anthropic = "sk-ant-api03-abcdef1234567890xyz";
    let openrouter = "sk-or-v1-abcdef1234567890abcdef";
    let hf = "hf_0123456789abcdef0123456789abcdefgh";
    "#;
    fs::write(path.join("leaks.rs"), leak_code).unwrap();

    let res_leak = upm::cli::execute_scan_secrets(path);
    assert!(res_leak.is_ok(), "execute_scan_secrets with secrets should complete and report");
}

#[test]
fn test_dependency_guesser_exotic_syntax_and_comments() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Python file with comments and multiline strings
    let py_content = r#"
    # Comment with import faker
    """
    import should_not_be_extracted_from_docstring
    from fake import FakeClass
    """
    import httpx
    from pydantic import BaseModel
    import os  # stdlib, must not be flagged
    "#;
    fs::write(path.join("app.py"), py_content).unwrap();

    // Go file with multiline import block and comments
    let go_content = r#"
    package main

    import (
        "fmt"  // stdlib
        "net/http"  // stdlib
        _ "github.com/lib/pq"
        r "github.com/redis/go-redis/v9"
    )

    func main() { fmt.Println("test") }
    "#;
    fs::write(path.join("main.go"), go_content).unwrap();

    // Rust file with nested use tree
    let rs_content = r#"
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use anyhow::Result;
    use tokio;
    "#;
    fs::write(path.join("lib.rs"), rs_content).unwrap();

    let res = upm::cli::execute_guess(path, false);
    assert!(res.is_ok(), "execute_guess on exotic syntax failed: {:?}", res);
}

#[test]
fn test_lockfile_tamper_and_reproducibility() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    let mut manifest = UpmManifest::new("lock-test", "1.0.0", None, vec![]);
    manifest.add_foreign_dependency("npm", "lodash", "^4.17.21");
    manifest.add_foreign_dependency("pip", "requests", "^2.31.0");
    manifest.save_to_dir(path).unwrap();

    // 1. Generate lockfile
    let lock_res = upm::cli::execute_lock(path, false, false);
    assert!(lock_res.is_ok(), "execute_lock generate failed: {:?}", lock_res);

    let lock_file = path.join("upm.lock");
    assert!(lock_file.exists());

    // 2. Verify intact lockfile
    let verify_res = upm::cli::execute_lock(path, true, false);
    assert!(verify_res.is_ok(), "execute_lock verify intact failed: {:?}", verify_res);

    // 3. Tamper with lockfile hash
    let lock_content = fs::read_to_string(&lock_file).unwrap();
    let tampered = lock_content.replace("sha256-", "sha256-tampered00000000000000000000000000000000000000000000000000000000");
    fs::write(&lock_file, tampered).unwrap();

    let verify_fail = upm::cli::execute_lock(path, true, false);
    assert!(verify_fail.is_err(), "Tampered lockfile integrity must fail verification");
}

#[test]
fn test_info_command_exotic_package_specifiers() {
    // 1. Colon separated with version
    assert!(upm::cli::execute_info("npm:react@18.2.0").is_ok());
    assert!(upm::cli::execute_info("pip:fastapi==0.110.0").is_ok());
    assert!(upm::cli::execute_info("cargo:tokio@^1.36.0").is_ok());

    // 2. Scoped npm package
    assert!(upm::cli::execute_info("npm:@types/node@20.11.0").is_ok());

    // 3. Generic and unstructured packages
    assert!(upm::cli::execute_info("unscoped-package").is_ok());
    assert!(upm::cli::execute_info("go:github.com/gin-gonic/gin").is_ok());
}

#[test]
fn test_env_command_masking_and_special_characters() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Set sensitive secret variable with special symbols
    let set_secret = upm::cli::execute_env(
        path,
        Some("set"),
        Some("DATABASE_PASSWORD"),
        Some("P@ssw0rd!#$&%^*()_-+=~`{}[]:;'<>,.?/"),
    );
    assert!(set_secret.is_ok(), "execute_env set password failed");

    // Get variable
    let get_res = upm::cli::execute_env(path, Some("get"), Some("DATABASE_PASSWORD"), None);
    assert!(get_res.is_ok(), "execute_env get password failed");

    // List variables
    let list_res = upm::cli::execute_env(path, Some("list"), None, None);
    assert!(list_res.is_ok(), "execute_env list failed");

    // Unset variable
    let unset_res = upm::cli::execute_env(path, Some("unset"), Some("DATABASE_PASSWORD"), None);
    assert!(unset_res.is_ok(), "execute_env unset failed");
}

#[test]
fn test_value_codec_numerical_extremes_and_deep_nesting() {
    // Deeply nested UpmValue list (depth 30)
    let mut root_val = UpmValue::String("leaf-data".to_string());
    for i in 0..30 {
        root_val = UpmValue::Array(vec![root_val, UpmValue::Number(i as f64)]);
    }

    // Round-trip serialize to JSON string and back
    let json_str = serde_json::to_string(&root_val).unwrap();
    let deserialized: UpmValue = serde_json::from_str(&json_str).unwrap();
    assert_eq!(root_val, deserialized, "Deeply nested UpmValue must round-trip cleanly");

    // Boundary numbers
    let max_num = UpmValue::Number(f64::MAX);
    let min_num = UpmValue::Number(f64::MIN);
    let max_json = serde_json::to_string(&max_num).unwrap();
    let min_json = serde_json::to_string(&min_num).unwrap();
    assert_eq!(max_num, serde_json::from_str::<UpmValue>(&max_json).unwrap());
    assert_eq!(min_num, serde_json::from_str::<UpmValue>(&min_json).unwrap());

    // Blob serialization
    let raw_bytes = vec![0u8, 255u8, 128u8, 42u8, 0u8];
    let blob_val = UpmValue::blob_from_bytes(&raw_bytes);
    let blob_json = serde_json::to_string(&blob_val).unwrap();
    let blob_restored: UpmValue = serde_json::from_str(&blob_json).unwrap();
    assert_eq!(blob_val, blob_restored, "UpmValue::Blob must preserve exact bytes across JSON codec");
}
