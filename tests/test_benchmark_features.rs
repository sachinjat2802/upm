use std::fs;
use tempfile::tempdir;
use upm::cli::{
    execute_agent, execute_ai, execute_create, execute_drive, execute_guess, execute_info,
    execute_lock, execute_mcp, execute_run_script,
};

#[test]
fn test_agentic_app_scaffold() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path().join("my-agent");

    // Scaffold agentic application
    let res = execute_create(&app_dir, Some("agentic"), Some("my-agent"));
    assert!(res.is_ok(), "execute_create failed: {:?}", res);

    assert!(app_dir.join("upm.toml").exists(), "upm.toml missing");
    assert!(app_dir.join("server.py").exists(), "server.py missing");
    assert!(app_dir.join("mcp.json").exists(), "mcp.json missing");
    assert!(app_dir.join("CLAUDE.md").exists(), "CLAUDE.md missing");
    assert!(app_dir.join(".cursorrules").exists(), ".cursorrules missing");
}

#[test]
fn test_deterministic_lockfile_lifecycle() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    // Scaffold app first
    execute_create(app_dir, Some("agentic"), Some("lock-test")).unwrap();

    // Generate cpm.lock
    let res = execute_lock(app_dir, false, false);
    assert!(res.is_ok(), "cpm lock generation failed: {:?}", res);

    let lock_file = app_dir.join("cpm.lock");
    assert!(lock_file.exists(), "cpm.lock was not generated");

    let lock_content = fs::read_to_string(&lock_file).unwrap();
    assert!(lock_content.contains("lockfile_version = 1"));
    assert!(lock_content.contains("sha256-"));

    // Verify cpm.lock
    let verify_res = execute_lock(app_dir, true, false);
    assert!(verify_res.is_ok(), "cpm lock --verify failed: {:?}", verify_res);
}

#[test]
fn test_cpm_guess_dependencies() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    // Create a Python source file importing third-party libraries
    let py_file = app_dir.join("main.py");
    fs::write(&py_file, "import requests\nfrom fastapi import FastAPI\nimport os\n").unwrap();

    // Guess dependencies
    let res = execute_guess(app_dir, false);
    assert!(res.is_ok(), "execute_guess failed: {:?}", res);
}

#[test]
fn test_cpm_ai_rules_sync() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    // Write custom CLAUDE.md
    fs::write(app_dir.join("CLAUDE.md"), "# Project Standards\n- Keep code clean and modular.\n").unwrap();

    // Run AI sync
    let sync_res = execute_ai(app_dir, Some("sync"), None, None);
    assert!(sync_res.is_ok(), "execute_ai sync failed: {:?}", sync_res);

    assert!(app_dir.join(".cursorrules").exists(), ".cursorrules should be generated");
    assert!(app_dir.join(".windsurfrules").exists(), ".windsurfrules should be generated");
    assert!(app_dir.join(".clinerules").exists(), ".clinerules should be generated");

    let cursor_content = fs::read_to_string(app_dir.join(".cursorrules")).unwrap();
    assert!(cursor_content.contains("Keep code clean and modular"));
}

#[test]
fn test_cpm_universal_mcp_manager() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    // Add an MCP server
    let add_res = execute_mcp(
        app_dir,
        Some("add"),
        Some("weather-service"),
        Some("python"),
        &["weather.py".to_string()],
    );
    assert!(add_res.is_ok(), "execute_mcp add failed: {:?}", add_res);

    let mcp_json = app_dir.join("mcp.json");
    assert!(mcp_json.exists(), "mcp.json should exist");

    let content = fs::read_to_string(mcp_json).unwrap();
    assert!(content.contains("weather-service"));

    // List MCP servers
    let list_res = execute_mcp(app_dir, Some("list"), None, None, &[]);
    assert!(list_res.is_ok(), "execute_mcp list failed: {:?}", list_res);
}

#[test]
fn test_agent_context_generation() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    execute_create(app_dir, Some("agentic"), Some("agent-ctx-test")).unwrap();

    // Generate human context
    let ctx_res = execute_agent(app_dir, Some("context"), false);
    assert!(ctx_res.is_ok(), "execute_agent context failed: {:?}", ctx_res);

    // Generate JSON context
    let json_res = execute_agent(app_dir, Some("context"), true);
    assert!(json_res.is_ok(), "execute_agent context --json failed: {:?}", json_res);
}

#[test]
fn test_agent_langgraph_and_chains() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    // Test LangGraph scaffolding
    let g_res = execute_agent(app_dir, Some("graph"), false);
    assert!(g_res.is_ok(), "execute_agent graph failed");
    assert!(app_dir.join("agent_graph.py").exists());

    // Test LangChain scaffolding
    let c_res = execute_agent(app_dir, Some("chain"), false);
    assert!(c_res.is_ok(), "execute_agent chain failed");
    assert!(app_dir.join("agent_chain.py").exists());

    // Test Multi-Agent Team scaffolding
    let t_res = execute_agent(app_dir, Some("team"), false);
    assert!(t_res.is_ok(), "execute_agent team failed");
    assert!(app_dir.join("agent_team.py").exists());
}

#[test]
fn test_high_capacity_drive_status() {
    let tmp = tempdir().expect("tempdir failed");
    let res = execute_drive(tmp.path(), Some("status"));
    assert!(res.is_ok(), "execute_drive status failed: {:?}", res);
}

#[test]
fn test_package_info_query() {
    let res = execute_info("pip:fastapi");
    assert!(res.is_ok(), "execute_info pip:fastapi failed: {:?}", res);

    let res2 = execute_info("npm:express");
    assert!(res2.is_ok(), "execute_info npm:express failed: {:?}", res2);
}

#[test]
fn test_cpm_script_and_tool_runner() {
    let tmp = tempdir().expect("tempdir failed");
    let app_dir = tmp.path();

    execute_create(app_dir, Some("agentic"), Some("script-test")).unwrap();

    // List available scripts
    let list_res = execute_run_script(app_dir, None, &[]);
    assert!(list_res.is_ok(), "execute_run_script list failed: {:?}", list_res);
}

#[test]
fn test_cpm_workspace_management() {
    let tmp = tempdir().expect("tempdir failed");
    let root = tmp.path();

    // Create sub-package in packages/core
    let pkg_dir = root.join("packages").join("core");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(pkg_dir.join("package.json"), r#"{"name": "@cpm/core", "version": "0.1.0"}"#).unwrap();

    let res = upm::cli::execute_workspace(root, Some("list"), None, &[]);
    assert!(res.is_ok(), "execute_workspace failed: {:?}", res);
}

#[test]
fn test_cpm_config_management() {
    let res = upm::cli::execute_config(Some("list"), None, None);
    assert!(res.is_ok(), "execute_config list failed: {:?}", res);

    let get_res = upm::cli::execute_config(Some("get"), Some("storage.drive"), None);
    assert!(get_res.is_ok(), "execute_config get failed: {:?}", get_res);
}

#[test]
fn test_cpm_ci_generation() {
    let tmp = tempdir().expect("tempdir failed");
    let res = upm::cli::execute_ci(tmp.path(), Some("github"), None);
    assert!(res.is_ok(), "execute_ci failed: {:?}", res);
    assert!(tmp.path().join(".github").join("workflows").join("cpm-ci.yml").exists());
}

#[test]
fn test_cpm_env_management() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Set variable
    let set_res = upm::cli::execute_env(path, Some("set"), Some("DATABASE_URL"), Some("postgres://localhost:5432/db"));
    assert!(set_res.is_ok(), "execute_env set failed: {:?}", set_res);

    // Get variable
    let get_res = upm::cli::execute_env(path, Some("get"), Some("DATABASE_URL"), None);
    assert!(get_res.is_ok(), "execute_env get failed: {:?}", get_res);

    // List variables (masked)
    let list_res = upm::cli::execute_env(path, Some("list"), None, None);
    assert!(list_res.is_ok(), "execute_env list failed: {:?}", list_res);
}

#[test]
fn test_cpm_plugin_lifecycle() {
    let list_res = upm::cli::execute_plugin(Some("list"), None);
    assert!(list_res.is_ok(), "execute_plugin list failed: {:?}", list_res);
}

#[test]
fn test_cpm_deploy_dry_run() {
    let tmp = tempdir().expect("tempdir failed");
    let res = upm::cli::execute_deploy(tmp.path(), Some("docker"), Some("staging"), true);
    assert!(res.is_ok(), "execute_deploy dry-run failed: {:?}", res);
}

#[test]
fn test_cpm_publish_dry_run() {
    let tmp = tempdir().expect("tempdir failed");
    let res = upm::cli::execute_publish(tmp.path(), true, None, None);
    assert!(res.is_ok(), "execute_publish dry-run failed: {:?}", res);
}

#[test]
fn test_cpm_upgrade_check() {
    let res = upm::cli::execute_upgrade(true);
    assert!(res.is_ok(), "execute_upgrade check failed: {:?}", res);
}
