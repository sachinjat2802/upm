use upm::acquisition::{AcquisitionRunner, AdapterRegistry, DetectionEngine, UpmManifest};
use tempfile::tempdir;

#[test]
fn test_adapter_registry_builtins() {
    let registry = AdapterRegistry::new();
    let adapters = registry.all();

    assert!(adapters.len() >= 15, "Registry should contain at least 15 builtin adapters");

    let pnpm = registry.get("pnpm").unwrap();
    assert_eq!(pnpm.language, "javascript");
    assert_eq!(pnpm.manifest_file, "package.json");
    assert_eq!(pnpm.lockfile_file.as_deref(), Some("pnpm-lock.yaml"));

    let uv = registry.get("uv").unwrap();
    assert_eq!(uv.language, "python");
    assert_eq!(uv.manifest_file, "pyproject.toml");

    let cargo = registry.get("cargo").unwrap();
    assert_eq!(cargo.language, "rust");
    assert_eq!(cargo.manifest_file, "Cargo.toml");

    let go = registry.get("go").unwrap();
    assert_eq!(go.language, "go");
    assert_eq!(go.manifest_file, "go.mod");
}

#[test]
fn test_scoring_signal_weights() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Manifest only (+40 pts)
    std::fs::write(root.join("package.json"), r#"{"name": "test"}"#).unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    let npm_score = result.scores.iter().find(|s| s.adapter.name == "npm").unwrap();
    assert!(npm_score.total_score >= 40, "npm score should include manifest score (+40)");

    // 2. Add marker string (+80 pts)
    std::fs::write(root.join("pyproject.toml"), "[tool.poetry]\nname = 'test'").unwrap();
    let result2 = engine.detect_dir(root);
    let poetry_score = result2.scores.iter().find(|s| s.adapter.name == "poetry").unwrap();
    assert!(poetry_score.total_score >= 120, "poetry score should include manifest (+40) + marker (+80)");
}

#[test]
fn test_adapter_filtering() {
    let registry = AdapterRegistry::new();
    let adapters: Vec<_> = registry.all().into_iter().cloned().collect();

    // Filter by language
    let py_filtered = AcquisitionRunner::filter_adapters(&adapters, Some("python"));
    assert!(py_filtered.iter().all(|a| a.language == "python"));
    assert!(py_filtered.len() >= 3, "Python filter should return pip, uv, poetry");

    // Filter by name
    let cargo_filtered = AcquisitionRunner::filter_adapters(&adapters, Some("cargo"));
    assert_eq!(cargo_filtered.len(), 1);
    assert_eq!(cargo_filtered[0].name, "cargo");

    // Case insensitive filter
    let js_filtered = AcquisitionRunner::filter_adapters(&adapters, Some("JAVASCRIPT"));
    assert!(!js_filtered.is_empty());

    // Non-existent filter
    let empty_filtered = AcquisitionRunner::filter_adapters(&adapters, Some("nonexistent_lang"));
    assert!(empty_filtered.is_empty());
}

#[test]
fn test_upm_manifest_lifecycle() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let manifest = UpmManifest::new("my-polyglot", "1.0.0", Some("python"), vec!["uv".into(), "pnpm".into()]);
    manifest.save_to_dir(root).unwrap();

    assert!(root.join("upm.toml").exists());

    let loaded = UpmManifest::load_from_dir(root).unwrap();
    assert_eq!(loaded.project.name, "my-polyglot");
    assert_eq!(loaded.project.version, "1.0.0");
    assert_eq!(loaded.project.primary_language.as_deref(), Some("python"));
    assert!(loaded.ecosystems.contains(&"uv".to_string()));
    assert!(loaded.ecosystems.contains(&"pnpm".to_string()));
}
