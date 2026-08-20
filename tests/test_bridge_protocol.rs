use std::sync::Arc;
use tokio::io::duplex;
use upm::bridge::protocol::MessageEnvelope;
use upm::bridge::transport::rpc::StdioRpcTransport;
use upm::bridge::transport::Transport;
use upm::bridge::value::{UpmError, UpmValue};
use upm::bridge::BridgePeer;

#[test]
fn test_rpc_request_envelope_serialization() {
    let req = MessageEnvelope::request("req_100", "python:math.sqrt", vec![UpmValue::Number(144.0)]);
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains(r#""type":"request""#));
    assert!(json.contains(r#""id":"req_100""#));
    assert!(json.contains(r#""method":"python:math.sqrt""#));

    let deserialized: MessageEnvelope = serde_json::from_str(&json).unwrap();
    if let MessageEnvelope::Request(r) = deserialized {
        assert_eq!(r.id, "req_100");
        assert_eq!(r.method, "python:math.sqrt");
        assert_eq!(r.args.len(), 1);
        assert_eq!(r.args[0], UpmValue::Number(144.0));
    } else {
        panic!("Deserialized envelope should be Request variant");
    }
}

#[test]
fn test_rpc_response_envelope_serialization() {
    let success = MessageEnvelope::success_response("req_100", UpmValue::Number(12.0));
    let json_success = serde_json::to_string(&success).unwrap();
    assert!(json_success.contains(r#""result":12.0"#));

    let err_env = MessageEnvelope::error_response("req_101", UpmError::new("ValueError", "math domain error"));
    let json_err = serde_json::to_string(&err_env).unwrap();
    assert!(json_err.contains(r#""error_type":"ValueError""#));
    assert!(json_err.contains(r#""message":"math domain error""#));
}

#[tokio::test]
async fn test_stdio_rpc_transport_framing() {
    let (client_io, server_io) = duplex(1024);
    let (client_read, client_write) = tokio::io::split(client_io);
    let (server_read, server_write) = tokio::io::split(server_io);

    let client_transport = StdioRpcTransport::new(client_read, client_write);
    let server_transport = StdioRpcTransport::new(server_read, server_write);

    let req_msg = MessageEnvelope::request("1", "echo", vec![UpmValue::String("hello".into())]);

    // Send from client to server
    client_transport.send_message(&req_msg).await.unwrap();

    // Read on server
    let received = server_transport.read_message().await.unwrap().unwrap();
    if let MessageEnvelope::Request(r) = received {
        assert_eq!(r.id, "1");
        assert_eq!(r.method, "echo");
    } else {
        panic!("Expected Request envelope");
    }
}

#[tokio::test]
async fn test_bridge_peer_bidirectional_call() {
    let (client_io, server_io) = duplex(1024);
    let (client_read, client_write) = tokio::io::split(client_io);
    let (server_read, server_write) = tokio::io::split(server_io);

    let client_transport = Arc::new(StdioRpcTransport::new(client_read, client_write));
    let server_transport = Arc::new(StdioRpcTransport::new(server_read, server_write));

    let client_peer = Arc::new(BridgePeer::new(client_transport));
    let server_peer = Arc::new(BridgePeer::new(server_transport));

    // Register a callback on server_peer
    let cb_id = server_peer.handles.register_callback(Arc::new(|args| {
        if let Some(UpmValue::Number(n)) = args.get(0) {
            Ok(UpmValue::Number(n * 2.0))
        } else {
            Err("Expected number".into())
        }
    }));

    // Spawn listener loops
    let server_peer_clone = server_peer.clone();
    tokio::spawn(async move {
        server_peer_clone.run_listener_loop().await;
    });

    let client_peer_clone = client_peer.clone();
    tokio::spawn(async move {
        client_peer_clone.run_listener_loop().await;
    });

    // Call server callback from client
    let method = format!("$fn:{}", cb_id);
    let result = client_peer.call(&method, vec![UpmValue::Number(21.0)]).await.unwrap();
    assert_eq!(result, UpmValue::Number(42.0));
}
