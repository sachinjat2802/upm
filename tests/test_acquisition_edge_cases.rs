use tempfile::tempdir;
use upm::acquisition::{AdapterRegistry, DetectionEngine, EcosystemAdapter};

#[test]
fn test_empty_directory_detection() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    assert!(result.detected_ecosystems.is_empty(), "Empty directory should detect no winning ecosystems");
    assert!(result.scores.iter().all(|s| s.total_score == 0), "All scores in empty directory should be 0");
}

#[test]
fn test_competing_managers_priority_tiebreak() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create ONLY package.json with no lockfile -> npm, pnpm, yarn, bun all match manifest (+40 pts)
    std::fs::write(root.join("package.json"), r#"{"name": "competing-app"}"#).unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    assert_eq!(result.detected_ecosystems.len(), 1);
    let winner = &result.detected_ecosystems[0];
    assert_eq!(winner.name, "pnpm", "pnpm should win tie-break due to highest default_priority (4)");
}

#[test]
fn test_custom_adapter_registration() {
    let mut registry = AdapterRegistry::new();

    let custom_zig = EcosystemAdapter {
        name: "zig".into(),
        language: "zig".into(),
        display_name: "Zig Package Manager".into(),
        manifest_file: "build.zig.zon".into(),
        lockfile_file: None,
        manifest_marker: None,
        glob_pattern: Some("build.zig".into()),
        directory_indicator: Some(".zig-cache".into()),
        default_priority: 4,
        install_cmd: vec!["zig".into(), "build".into()],
        add_cmd: vec!["zig".into(), "fetch".into()],
        update_cmd: vec!["zig".into(), "build".into()],
        outdated_cmd: vec!["zig".into(), "build".into()],
        audit_cmd: vec!["zig".into(), "build".into()],
        run_cmd: vec!["zig".into(), "run".into()],
    };

    registry.register(custom_zig);

    let dir = tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("build.zig.zon"), ".{ .name = \"my_zig_app\" }").unwrap();

    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    let zig_winner = result.detected_ecosystems.iter().find(|e| e.name == "zig");
    assert!(zig_winner.is_some(), "Custom registered ecosystem 'zig' should be detected");
}

#[test]
fn test_glob_pattern_matching() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a .csproj file (NuGet match via glob *.csproj)
    std::fs::write(root.join("App.csproj"), "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>").unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    let nuget_detected = result.detected_ecosystems.iter().any(|e| e.name == "nuget");
    assert!(nuget_detected, "NuGet should be detected via *.csproj glob pattern match");
}

#[test]
fn test_directory_indicator_matching() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create target directory (Rust directory indicator)
    std::fs::create_dir(root.join("target")).unwrap();
    std::fs::write(root.join("Cargo.toml"), "[package]\nname = 'app'\nversion = '0.1.0'").unwrap();

    let registry = AdapterRegistry::new();
    let engine = DetectionEngine::new(registry);
    let result = engine.detect_dir(root);

    let rust_score = result.scores.iter().find(|s| s.adapter.name == "cargo").unwrap();
    assert!(rust_score.details.iter().any(|d| d.signal == "directory present"), "Rust score should include directory indicator signal");
}
