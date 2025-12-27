use super::*;

fn make_test_request() -> Request {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Accept".to_string(), "text/html".to_string());

    Request {
        url: "https://example.com/api/users".to_string(),
        method: "POST".to_string(),
        headers,
        post_data: Some(r#"{"name":"test"}"#.to_string()),
        resource_type: ResourceType::Fetch,
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

#[test]
fn test_request_accessors() {
    let request = make_test_request();

    assert_eq!(request.url(), "https://example.com/api/users");
    assert_eq!(request.method(), "POST");
    assert_eq!(request.resource_type(), ResourceType::Fetch);
    assert_eq!(request.post_data(), Some(r#"{"name":"test"}"#));
}

#[test]
fn test_header_value_case_insensitive() {
    let request = make_test_request();

    assert_eq!(request.header_value("content-type"), Some("application/json"));
    assert_eq!(request.header_value("Content-Type"), Some("application/json"));
    assert_eq!(request.header_value("CONTENT-TYPE"), Some("application/json"));
    assert_eq!(request.header_value("X-Custom"), None);
}

#[test]
fn test_post_data_json() {
    let request = make_test_request();

    #[derive(serde::Deserialize)]
    struct TestData {
        name: String,
    }

    let data: TestData = request.post_data_json().unwrap().unwrap();
    assert_eq!(data.name, "test");
}
