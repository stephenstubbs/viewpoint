use super::*;
use std::time::Duration;

#[test]
fn test_connection_failed_error_display() {
    let err = CdpError::ConnectionFailed("connection refused".to_string());
    assert_eq!(
        err.to_string(),
        "WebSocket connection failed: connection refused"
    );
}

#[test]
fn test_connection_lost_error_display() {
    let err = CdpError::ConnectionLost;
    assert_eq!(err.to_string(), "WebSocket connection lost");
}

#[test]
fn test_send_failed_error_display() {
    let err = CdpError::SendFailed("channel closed".to_string());
    assert_eq!(err.to_string(), "failed to send CDP message: channel closed");
}

#[test]
fn test_protocol_error_display() {
    let err = CdpError::Protocol {
        code: -32601,
        message: "Method not found".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "CDP protocol error -32601: Method not found"
    );
}

#[test]
fn test_json_error_from_serde() {
    let json_err: serde_json::Error = serde_json::from_str::<i32>("not a number").unwrap_err();
    let err: CdpError = json_err.into();
    assert!(err.to_string().starts_with("JSON error:"));
}

#[test]
fn test_timeout_error_display() {
    let err = CdpError::Timeout(Duration::from_secs(30));
    assert_eq!(err.to_string(), "response timeout after 30s");
}

#[test]
fn test_timeout_error_with_millis() {
    let err = CdpError::Timeout(Duration::from_millis(500));
    assert_eq!(err.to_string(), "response timeout after 500ms");
}

#[test]
fn test_invalid_message_id_error_display() {
    let err = CdpError::InvalidMessageId {
        expected: 42,
        got: 99,
    };
    assert_eq!(err.to_string(), "invalid message ID: expected 42, got 99");
}

#[test]
fn test_invalid_url_error_display() {
    let err = CdpError::InvalidUrl("not-a-valid-url".to_string());
    assert_eq!(err.to_string(), "invalid WebSocket URL: not-a-valid-url");
}

#[test]
fn test_session_not_found_error_display() {
    let err = CdpError::SessionNotFound("ABC123".to_string());
    assert_eq!(err.to_string(), "session not found: ABC123");
}

#[test]
fn test_spawn_failed_error_display() {
    let err = CdpError::SpawnFailed("permission denied".to_string());
    assert_eq!(
        err.to_string(),
        "failed to spawn browser process: permission denied"
    );
}

#[test]
fn test_no_debugging_url_error_display() {
    let err = CdpError::NoDebuggingUrl;
    assert_eq!(
        err.to_string(),
        "failed to get debugging URL from browser"
    );
}

#[test]
fn test_chromium_not_found_error_display() {
    let err = CdpError::ChromiumNotFound;
    assert_eq!(
        err.to_string(),
        "Chromium not found. Set CHROMIUM_PATH environment variable or ensure Chromium is installed."
    );
}

#[test]
fn test_launch_timeout_error_display() {
    let err = CdpError::LaunchTimeout(Duration::from_secs(60));
    assert_eq!(err.to_string(), "browser launch timeout after 60s");
}

#[test]
fn test_from_tungstenite_connection_closed() {
    let ws_err = tokio_tungstenite::tungstenite::Error::ConnectionClosed;
    let err: CdpError = ws_err.into();
    assert!(matches!(err, CdpError::ConnectionLost));
}

#[test]
fn test_from_tungstenite_already_closed() {
    let ws_err = tokio_tungstenite::tungstenite::Error::AlreadyClosed;
    let err: CdpError = ws_err.into();
    assert!(matches!(err, CdpError::ConnectionLost));
}

#[test]
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CdpError>();
}

#[test]
fn test_error_debug_format() {
    let err = CdpError::Protocol {
        code: -32600,
        message: "Invalid Request".to_string(),
    };
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Protocol"));
    assert!(debug_str.contains("-32600"));
    assert!(debug_str.contains("Invalid Request"));
}
