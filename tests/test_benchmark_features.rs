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

    // Create a Go source file importing third-party packages
    let go_file = app_dir.join("server.go");
    fs::write(&go_file, "package main\n\nimport (\n\t\"fmt\"\n\t\"github.com/gin-gonic/gin\"\n)\n").unwrap();

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

    // Test Open Cron Agent scaffolding
    let cron_res = execute_agent(app_dir, Some("cron"), false);
    assert!(cron_res.is_ok(), "execute_agent cron failed");
    assert!(app_dir.join("agent_cron.py").exists());

    // Test OpenRouter & Open-Source Models configuration
    let m_res = execute_agent(app_dir, Some("models"), false);
    assert!(m_res.is_ok(), "execute_agent models failed");
    assert!(app_dir.join("cpm.models.json").exists());
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

    let res3 = execute_info("pip:fastapi@0.110.0");
    assert!(res3.is_ok(), "execute_info versioned failed: {:?}", res3);

    let res4 = execute_info("cargo:serde==1.0.197");
    assert!(res4.is_ok(), "execute_info == versioned failed: {:?}", res4);
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

#[test]
fn test_cpm_clean_command() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();
    fs::create_dir_all(path.join("node_modules")).unwrap();
    fs::create_dir_all(path.join("target")).unwrap();
    fs::create_dir_all(path.join(".venv")).unwrap();

    let dry_res = upm::cli::execute_clean(path, true);
    assert!(dry_res.is_ok(), "execute_clean dry-run failed: {:?}", dry_res);

    let clean_res = upm::cli::execute_clean(path, false);
    assert!(clean_res.is_ok(), "execute_clean failed: {:?}", clean_res);
    assert!(!path.join("node_modules").exists());
    assert!(!path.join("target").exists());
}

#[test]
fn test_cpm_diff_and_resolve() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();
    execute_create(path, Some("agentic"), Some("diff-test")).unwrap();

    let diff_res = upm::cli::execute_diff(path);
    assert!(diff_res.is_ok(), "execute_diff failed: {:?}", diff_res);

    let resolve_res = upm::cli::execute_resolve(path);
    assert!(resolve_res.is_ok(), "execute_resolve failed: {:?}", resolve_res);
}

#[test]
fn test_cpm_cost_and_licenses() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();
    execute_create(path, Some("agentic"), Some("cost-test")).unwrap();

    let cost_res = upm::cli::execute_cost(path);
    assert!(cost_res.is_ok(), "execute_cost failed: {:?}", cost_res);

    let lic_res = upm::cli::execute_licenses(path);
    assert!(lic_res.is_ok(), "execute_licenses failed: {:?}", lic_res);
}

#[test]
fn test_cpm_devops_helm_and_crd() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    let helm_res = upm::cli::execute_helm(path, Some("helm-chart"));
    assert!(helm_res.is_ok(), "execute_helm failed: {:?}", helm_res);
    assert!(path.join("helm-chart").join("Chart.yaml").exists());

    let op_res = upm::cli::execute_operator(path, Some("crd.yaml"));
    assert!(op_res.is_ok(), "execute_operator failed: {:?}", op_res);
    assert!(path.join("crd.yaml").exists());

    let df_res = upm::cli::execute_dockerfile(path, Some("Dockerfile"));
    assert!(df_res.is_ok(), "execute_dockerfile failed: {:?}", df_res);
    assert!(path.join("Dockerfile").exists());
}

#[test]
fn test_cpm_bundle_and_trace() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();
    execute_create(path, Some("agentic"), Some("bundle-test")).unwrap();

    let bundle_res = upm::cli::execute_bundle(path, Some("bundle.tar.gz"));
    assert!(bundle_res.is_ok(), "execute_bundle failed: {:?}", bundle_res);
    assert!(path.join("bundle.tar.gz").exists());

    let trace_res = upm::cli::execute_trace(path, Some("nested/dir/trace.json"));
    assert!(trace_res.is_ok(), "execute_trace failed: {:?}", trace_res);
    assert!(path.join("nested").join("dir").join("trace.json").exists());

    let flame_res = upm::cli::execute_flamegraph(path, Some("nested/dir/flame.svg"));
    assert!(flame_res.is_ok(), "execute_flamegraph failed: {:?}", flame_res);
    assert!(path.join("nested").join("dir").join("flame.svg").exists());
}

#[test]
fn test_cpm_scan_secrets() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // Clean workspace scan
    let scan_res = upm::cli::execute_scan_secrets(path);
    assert!(scan_res.is_ok(), "execute_scan_secrets failed: {:?}", scan_res);

    // Write file with dummy secret
    let secret_file = path.join("config.env");
    fs::write(&secret_file, "API_KEY=\"AKIAIOSFODNN7EXAMPLE\"\n").unwrap();

    let scan_res2 = upm::cli::execute_scan_secrets(path);
    assert!(scan_res2.is_ok(), "execute_scan_secrets with secrets failed: {:?}", scan_res2);
}

#[test]
fn test_cpm_cache_operations() {
    let stats_res = upm::cli::execute_cache(std::path::Path::new("."), Some("stats"));
    assert!(stats_res.is_ok(), "execute_cache stats failed: {:?}", stats_res);

    let clean_res = upm::cli::execute_cache(std::path::Path::new("."), Some("clean"));
    assert!(clean_res.is_ok(), "execute_cache clean failed: {:?}", clean_res);
}

#[test]
fn test_cpm_sccache_and_sig() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    let sc_res = upm::cli::execute_sccache(path);
    assert!(sc_res.is_ok(), "execute_sccache failed: {:?}", sc_res);

    let sig_res = upm::cli::execute_verify_sig(path, Some("test-pkg"));
    assert!(sig_res.is_ok(), "execute_verify_sig failed: {:?}", sig_res);
    assert!(path.join(".cpm_signature.sig").exists());
}

#[test]
fn test_cpm_alias_and_audit_log() {
    let tmp = tempdir().expect("tempdir failed");
    let path = tmp.path();

    // List aliases
    let list_res = upm::cli::execute_alias(path, Some("list"), None, None);
    assert!(list_res.is_ok(), "execute_alias list failed: {:?}", list_res);

    // Add alias
    let add_res = upm::cli::execute_alias(path, Some("add"), Some("my-run"), Some("run dev"));
    assert!(add_res.is_ok(), "execute_alias add failed: {:?}", add_res);
    assert!(path.join(".cpm_aliases.json").exists());

    // Remove alias
    let rm_res = upm::cli::execute_alias(path, Some("remove"), Some("my-run"), None);
    assert!(rm_res.is_ok(), "execute_alias remove failed: {:?}", rm_res);

    // Audit log
    let audit_res = upm::cli::execute_audit_log(path);
    assert!(audit_res.is_ok(), "execute_audit_log failed: {:?}", audit_res);
    assert!(path.join(".cpm_audit.log").exists());
}
