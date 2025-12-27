use super::*;

#[test]
fn test_websocket_frame_creation() {
    let frame = WebSocketFrame::new(1, "Hello".to_string());
    assert_eq!(frame.opcode(), 1);
    assert_eq!(frame.payload(), "Hello");
    assert!(frame.is_text());
    assert!(!frame.is_binary());
}

#[test]
fn test_websocket_frame_binary() {
    let frame = WebSocketFrame::new(2, "binary data".to_string());
    assert!(frame.is_binary());
    assert!(!frame.is_text());
}

#[test]
fn test_websocket_url() {
    let ws = WebSocket::new("req-1".to_string(), "wss://example.com/socket".to_string());
    assert_eq!(ws.url(), "wss://example.com/socket");
    assert_eq!(ws.request_id(), "req-1");
    assert!(!ws.is_closed());
}

#[test]
fn test_websocket_close() {
    let ws = WebSocket::new("req-1".to_string(), "wss://example.com/socket".to_string());
    assert!(!ws.is_closed());
    ws.mark_closed();
    assert!(ws.is_closed());
}
