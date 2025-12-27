use crate::transport::{CdpEvent, CdpMessage, CdpRequest, CdpResponse};

#[test]
fn test_cdp_request_serialization() {
    let request = CdpRequest {
        id: 1,
        method: "Target.createTarget".to_string(),
        params: Some(serde_json::json!({"url": "about:blank"})),
        session_id: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"method\":\"Target.createTarget\""));
    assert!(json.contains("\"url\":\"about:blank\""));
    assert!(!json.contains("sessionId"));
}

#[test]
fn test_cdp_response_deserialization() {
    let json = r#"{"id": 1, "result": {"targetId": "abc123"}}"#;
    let response: CdpResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.id, 1);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[test]
fn test_cdp_event_deserialization() {
    let json = r#"{"method": "Page.loadEventFired", "params": {"timestamp": 123.456}}"#;
    let event: CdpEvent = serde_json::from_str(json).unwrap();

    assert_eq!(event.method, "Page.loadEventFired");
    assert!(event.params.is_some());
}

#[test]
fn test_cdp_message_discrimination() {
    let resp_json = r#"{"id": 1, "result": {}}"#;
    let event_json = r#"{"method": "Page.loadEventFired", "params": {}}"#;

    let resp: CdpMessage = serde_json::from_str(resp_json).unwrap();
    let event: CdpMessage = serde_json::from_str(event_json).unwrap();

    assert!(resp.is_response_for(1));
    assert!(!resp.is_response_for(2));
    assert!(resp.into_response().is_some());
    assert!(event.into_event().is_some());
}
