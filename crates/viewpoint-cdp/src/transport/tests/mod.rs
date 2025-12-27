use super::*;

// CdpRequest tests
#[test]
fn test_request_serialization_minimal() {
    let req = CdpRequest {
        id: 1,
        method: "Target.getTargets".to_string(),
        params: None,
        session_id: None,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert_eq!(json, r#"{"id":1,"method":"Target.getTargets"}"#);
}

#[test]
fn test_request_serialization_with_params() {
    let req = CdpRequest {
        id: 42,
        method: "Page.navigate".to_string(),
        params: Some(serde_json::json!({"url": "https://example.com"})),
        session_id: None,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains(r#""id":42"#));
    assert!(json.contains(r#""method":"Page.navigate""#));
    assert!(json.contains(r#""url":"https://example.com""#));
}

#[test]
fn test_request_serialization_with_session() {
    let req = CdpRequest {
        id: 5,
        method: "Runtime.evaluate".to_string(),
        params: Some(serde_json::json!({"expression": "1+1"})),
        session_id: Some("session123".to_string()),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains(r#""sessionId":"session123""#));
}

// CdpResponse tests
#[test]
fn test_response_deserialization_success() {
    let json = r#"{"id":1,"result":{"value":42}}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, 1);
    assert!(resp.result.is_some());
    assert!(resp.error.is_none());
}

#[test]
fn test_response_deserialization_error() {
    let json = r#"{"id":2,"error":{"code":-32601,"message":"Method not found"}}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, 2);
    assert!(resp.result.is_none());
    let err = resp.error.unwrap();
    assert_eq!(err.code, -32601);
    assert_eq!(err.message, "Method not found");
}

#[test]
fn test_response_deserialization_with_session() {
    let json = r#"{"id":3,"result":{},"sessionId":"sess456"}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.session_id.as_deref(), Some("sess456"));
}

#[test]
fn test_response_error_with_data() {
    let json = r#"{"id":4,"error":{"code":-32000,"message":"Error","data":"additional info"}}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    let err = resp.error.unwrap();
    assert_eq!(err.data.as_deref(), Some("additional info"));
}

// CdpEvent tests
#[test]
fn test_event_deserialization_simple() {
    let json = r#"{"method":"Page.loadEventFired","params":{"timestamp":12345.6}}"#;
    let evt: CdpEvent = serde_json::from_str(json).unwrap();
    assert_eq!(evt.method, "Page.loadEventFired");
    assert!(evt.params.is_some());
    assert!(evt.session_id.is_none());
}

#[test]
fn test_event_deserialization_with_session() {
    let json = r#"{"method":"Network.requestWillBeSent","params":{},"sessionId":"sess789"}"#;
    let evt: CdpEvent = serde_json::from_str(json).unwrap();
    assert_eq!(evt.session_id.as_deref(), Some("sess789"));
}

#[test]
fn test_event_deserialization_no_params() {
    let json = r#"{"method":"Target.targetCreated"}"#;
    let evt: CdpEvent = serde_json::from_str(json).unwrap();
    assert!(evt.params.is_none());
}

// CdpMessage tests
#[test]
fn test_message_parses_as_response() {
    let json = r#"{"id":1,"result":{"success":true}}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    assert!(matches!(msg, CdpMessage::Response(_)));
    assert!(msg.is_response_for(1));
    assert!(!msg.is_response_for(2));
}

#[test]
fn test_message_parses_as_event() {
    let json = r#"{"method":"Page.frameNavigated","params":{}}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    assert!(matches!(msg, CdpMessage::Event(_)));
    assert!(!msg.is_response_for(1));
}

#[test]
fn test_message_into_response() {
    let json = r#"{"id":10,"result":null}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    let resp = msg.into_response();
    assert!(resp.is_some());
    assert_eq!(resp.unwrap().id, 10);
}

#[test]
fn test_message_into_response_from_event() {
    let json = r#"{"method":"Test.event"}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    assert!(msg.into_response().is_none());
}

#[test]
fn test_message_into_event() {
    let json = r#"{"method":"Console.messageAdded","params":{}}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    let evt = msg.into_event();
    assert!(evt.is_some());
    assert_eq!(evt.unwrap().method, "Console.messageAdded");
}

#[test]
fn test_message_into_event_from_response() {
    let json = r#"{"id":5,"result":{}}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    assert!(msg.into_event().is_none());
}

// Edge cases and malformed JSON
#[test]
fn test_response_empty_result() {
    let json = r#"{"id":1,"result":{}}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    assert!(resp.result.is_some());
    assert!(resp.result.unwrap().as_object().unwrap().is_empty());
}

#[test]
fn test_response_null_result() {
    // When result is explicitly null in JSON, serde deserializes it as Some(Value::Null)
    // But when using skip_serializing_if or default, it becomes None
    // The actual behavior depends on the JSON - test the actual behavior
    let json = r#"{"id":1,"result":null}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    // Serde with Option<Value> treats null as None
    // This is expected behavior for CDP responses
    assert!(resp.result.is_none() || resp.result.as_ref().is_some_and(|v| v.is_null()));
}

#[test]
fn test_malformed_json_missing_id() {
    let json = r#"{"result":{}}"#;
    let result: Result<CdpResponse, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_event_with_complex_params() {
    let json = r#"{
        "method": "Network.responseReceived",
        "params": {
            "requestId": "123",
            "response": {
                "url": "https://example.com",
                "status": 200,
                "headers": {"content-type": "text/html"}
            }
        }
    }"#;
    let evt: CdpEvent = serde_json::from_str(json).unwrap();
    let params = evt.params.unwrap();
    assert_eq!(params["requestId"], "123");
    assert_eq!(params["response"]["status"], 200);
}

#[test]
fn test_request_clone() {
    let req = CdpRequest {
        id: 1,
        method: "Test".to_string(),
        params: Some(serde_json::json!({"key": "value"})),
        session_id: Some("sess".to_string()),
    };
    let cloned = req.clone();
    assert_eq!(req.id, cloned.id);
    assert_eq!(req.method, cloned.method);
}

#[test]
fn test_response_clone() {
    let json = r#"{"id":1,"result":{"data":"test"}}"#;
    let resp: CdpResponse = serde_json::from_str(json).unwrap();
    let cloned = resp.clone();
    assert_eq!(resp.id, cloned.id);
}

#[test]
fn test_event_clone() {
    let json = r#"{"method":"Test.event","params":{}}"#;
    let evt: CdpEvent = serde_json::from_str(json).unwrap();
    let cloned = evt.clone();
    assert_eq!(evt.method, cloned.method);
}

#[test]
fn test_message_clone() {
    let json = r#"{"id":1,"result":{}}"#;
    let msg: CdpMessage = serde_json::from_str(json).unwrap();
    let cloned = msg.clone();
    assert!(matches!(cloned, CdpMessage::Response(_)));
}
