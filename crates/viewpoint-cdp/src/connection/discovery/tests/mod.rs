//! Tests for endpoint discovery.

use super::*;

#[test]
fn test_websocket_url_passthrough() {
    // WebSocket URLs should be returned as-is
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let options = CdpConnectionOptions::default();
        let result =
            discover_websocket_url("ws://localhost:9222/devtools/browser/abc123", &options).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "ws://localhost:9222/devtools/browser/abc123"
        );
    });
}

#[test]
fn test_invalid_scheme() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let options = CdpConnectionOptions::default();
        let result = discover_websocket_url("ftp://localhost:9222", &options).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, CdpError::InvalidEndpointUrl(_)));
    });
}

#[test]
fn test_connection_options_builder() {
    let options = CdpConnectionOptions::new()
        .timeout(Duration::from_secs(10))
        .header("Authorization", "Bearer token")
        .header("X-Custom", "value");

    assert_eq!(options.timeout, Some(Duration::from_secs(10)));
    assert_eq!(
        options.headers.get("Authorization"),
        Some(&"Bearer token".to_string())
    );
    assert_eq!(options.headers.get("X-Custom"), Some(&"value".to_string()));
}
