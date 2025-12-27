use super::*;

#[test]
fn test_network_event_types() {
    // Verify event enum variants exist
    let _req = NetworkEvent::Request(RequestEvent {
        request: create_test_request(),
    });
}

fn create_test_request() -> Request {
    // Create a minimal test request
    use std::collections::HashMap;
    use super::super::types::ResourceType;

    Request {
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        post_data: None,
        resource_type: ResourceType::Document,
        frame_id: "frame-1".to_string(),
        is_navigation: false,
        connection: None,
        session_id: None,
        request_id: None,
        redirected_from: None,
        redirected_to: None,
        timing: None,
        failure_text: None,
    }
}
