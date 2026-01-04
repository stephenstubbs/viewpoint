use super::*;

#[test]
fn test_target_info_changed_event_deserialization() {
    let json = r#"{
        "targetInfo": {
            "targetId": "ABC123",
            "type": "page",
            "title": "Example Page",
            "url": "https://example.com",
            "attached": true,
            "browserContextId": "context-456"
        }
    }"#;

    let event: TargetInfoChangedEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.target_info.target_id, "ABC123");
    assert_eq!(event.target_info.target_type, "page");
    assert_eq!(event.target_info.title, "Example Page");
    assert_eq!(event.target_info.url, "https://example.com");
    assert!(event.target_info.attached);
    assert_eq!(
        event.target_info.browser_context_id,
        Some("context-456".to_string())
    );
}

#[test]
fn test_target_info_changed_event_without_context_id() {
    let json = r#"{
        "targetInfo": {
            "targetId": "DEF789",
            "type": "page",
            "title": "Default Context Page",
            "url": "about:blank",
            "attached": false
        }
    }"#;

    let event: TargetInfoChangedEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.target_info.target_id, "DEF789");
    assert!(!event.target_info.attached);
    assert!(event.target_info.browser_context_id.is_none());
    assert!(event.target_info.opener_id.is_none());
}

#[test]
fn test_target_created_event_deserialization() {
    let json = r#"{
        "targetInfo": {
            "targetId": "target-123",
            "type": "page",
            "title": "New Tab",
            "url": "chrome://newtab/",
            "attached": false,
            "openerId": "opener-456"
        }
    }"#;

    let event: TargetCreatedEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.target_info.target_id, "target-123");
    assert_eq!(event.target_info.opener_id, Some("opener-456".to_string()));
}
