use tempfile::tempdir;
use upm::acquisition::UpmManifest;
use upm::cli::*;

#[test]
fn test_execute_add_explicit_prefix() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    execute_init(root, Some("test-app".into()), Some("python".into()), None, true).unwrap();
    execute_add(root, "pip:requests", true).unwrap();

    let manifest = UpmManifest::load_from_dir(root).unwrap();
    assert!(manifest.foreign_dependencies.contains_key("pip:requests"));
}

#[test]
fn test_execute_add_auto_inference() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a Python project
    execute_init(root, Some("py-app".into()), Some("python".into()), None, true).unwrap();

    // Add package without ecosystem prefix -> should infer pip/uv from Python project
    execute_add(root, "pandas", true).unwrap();

    let manifest = UpmManifest::load_from_dir(root).unwrap();
    assert!(
        manifest.foreign_dependencies.contains_key("pip:pandas") || manifest.foreign_dependencies.contains_key("uv:pandas"),
        "Should auto-infer pip/uv ecosystem for Python project when prefix is omitted"
    );
}

#[test]
fn test_execute_remove_dependency() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    execute_init(root, Some("app".into()), Some("python".into()), None, true).unwrap();
    execute_add(root, "pip:requests", true).unwrap();

    let manifest_before = UpmManifest::load_from_dir(root).unwrap();
    assert!(manifest_before.foreign_dependencies.contains_key("pip:requests"));

    execute_remove(root, "pip:requests", true).unwrap();

    let manifest_after = UpmManifest::load_from_dir(root).unwrap();
    assert!(!manifest_after.foreign_dependencies.contains_key("pip:requests"));
}

#[test]
fn test_execute_install_and_update_flags() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    execute_init(root, Some("polyglot".into()), Some("python".into()), Some("node,rust".into()), true).unwrap();

    // Test dry_run + parallel + filter flags
    execute_install(root, true, true, Some("python")).unwrap();
    execute_update(root, true, true, Some("node")).unwrap();
    execute_outdated(root, Some("rust")).unwrap();
    execute_audit(root, Some("python")).unwrap();
}
