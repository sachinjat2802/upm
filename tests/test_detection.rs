use tempfile::tempdir;
use upm::acquisition::{AdapterRegistry, DetectionEngine};

#[test]
fn test_node_pnpm_detection_scoring() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create package.json and pnpm-lock.yaml
    std::fs::write(root.join("package.json"), r#"{"name": "my-app"}"#).unwrap();
    std::fs::write(root.join("pnpm-lock.yaml"), "lockfileVersion: '9.0'").unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    assert!(!result.detected_ecosystems.is_empty());
    let pnpm_winner = result.detected_ecosystems.iter().any(|e| e.name == "pnpm");
    assert!(pnpm_winner, "pnpm should win over npm due to pnpm-lock.yaml (score +100)");

    let pnpm_score = result.scores.iter().find(|s| s.adapter.name == "pnpm").unwrap();
    assert!(pnpm_score.total_score >= 140, "pnpm score should be >= 140 (100 lockfile + 40 manifest + priority)");
}

#[test]
fn test_polyglot_repository_detection() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Node + Python + Rust polyglot repo
    std::fs::write(root.join("package.json"), r#"{"name": "web"}"#).unwrap();
    std::fs::write(root.join("pyproject.toml"), "[tool.uv]\nname = 'api'").unwrap();
    std::fs::write(root.join("uv.lock"), "").unwrap();
    std::fs::write(root.join("Cargo.toml"), "[package]\nname = 'engine'\nversion = '0.1.0'").unwrap();
    std::fs::write(root.join("Cargo.lock"), "").unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    let detected_langs: Vec<String> = result.detected_ecosystems.iter().map(|e| e.language.clone()).collect();
    assert!(detected_langs.contains(&"javascript".to_string()));
    assert!(detected_langs.contains(&"python".to_string()));
    assert!(detected_langs.contains(&"rust".to_string()));
}

#[test]
fn test_upm_init_command() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    upm::cli::execute_init(
        root,
        Some("test-polyglot".to_string()),
        Some("python".to_string()),
        Some("node,rust".to_string()),
        true,
    ).unwrap();

    assert!(root.join("upm.toml").exists(), "upm.toml should be created");
    assert!(root.join("pyproject.toml").exists(), "pyproject.toml should be scaffolded for base python");
    assert!(root.join("package.json").exists(), "package.json should be scaffolded for foreign node");
    assert!(root.join("Cargo.toml").exists(), "Cargo.toml should be scaffolded for foreign rust");

    let manifest = upm::acquisition::UpmManifest::load_from_dir(root).unwrap();
    assert_eq!(manifest.project.name, "test-polyglot");
    assert_eq!(manifest.project.primary_language.as_deref(), Some("python"));
}
