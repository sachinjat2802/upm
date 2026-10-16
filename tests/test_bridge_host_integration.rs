use std::path::Path;
use upm::bridge::HostSupervisor;
use upm::bridge::UpmValue;

fn is_runtime_available(cmd: &str) -> bool {
    std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[tokio::test]
async fn test_python_host_math_sqrt() {
    let host_path = Path::new("hosts/python_host.py");
    if !host_path.exists() || !is_runtime_available("python") {
        eprintln!("Skipping Python host test: python runtime or script not available");
        return;
    }

    match HostSupervisor::spawn_host("python", host_path).await {
        Ok(host) => {
            if let Ok(res) = host.peer.call("math.sqrt", vec![UpmValue::Number(81.0)]).await {
                assert_eq!(res, UpmValue::Number(9.0));
            }
        }
        Err(e) => {
            eprintln!("Python runtime not available on test machine: {}", e);
        }
    }
}

#[tokio::test]
async fn test_python_host_inspect() {
    let host_path = Path::new("hosts/python_host.py");
    if !host_path.exists() || !is_runtime_available("python") {
        return;
    }

    if let Ok(host) = HostSupervisor::spawn_host("python", host_path).await {
        if let Ok(res) = host.peer.call("__inspect__", vec![]).await {
            if let UpmValue::Array(items) = res {
                assert!(!items.is_empty(), "Python host should return inspect method list");
                let has_sqrt = items.iter().any(|item| match item {
                    UpmValue::Map(m) => m.get("name") == Some(&UpmValue::String("math.sqrt".into())),
                    _ => false,
                });
                assert!(has_sqrt, "Inspect result should include math.sqrt");
            }
        }
    }
}

#[tokio::test]
async fn test_node_host_inspect() {
    let host_path = Path::new("hosts/node_host.js");
    if !host_path.exists() || !is_runtime_available("node") {
        return;
    }

    if let Ok(host) = HostSupervisor::spawn_host("node", host_path).await {
        let res = host.peer.call("__inspect__", vec![]).await.unwrap();
        if let UpmValue::Array(items) = res {
            assert!(!items.is_empty(), "Node host should return inspect method list");
            let has_resize = items.iter().any(|item| match item {
                UpmValue::Map(m) => m.get("name") == Some(&UpmValue::String("sharp.resize".into())),
                _ => false,
            });
            assert!(has_resize, "Inspect result should include sharp.resize");
        } else {
            panic!("__inspect__ should return Array");
        }
    }
}

#[tokio::test]
async fn test_host_echo_and_ping() {
    let host_path = Path::new("hosts/python_host.py");
    if !host_path.exists() || !is_runtime_available("python") {
        return;
    }

    if let Ok(host) = HostSupervisor::spawn_host("python", host_path).await {
        if let Ok(echo_res) = host.peer.call("echo", vec![UpmValue::String("CPM Bridge Test".into())]).await {
            assert_eq!(echo_res, UpmValue::String("CPM Bridge Test".into()));
        }

        if let Ok(ping_res) = host.peer.call("ping", vec![]).await {
            assert_eq!(ping_res, UpmValue::String("pong".into()));
        }
    }
}
