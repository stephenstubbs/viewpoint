use super::*;

#[tokio::test]
async fn test_har_recorder_basic() {
    let options = HarRecordingBuilder::new("/tmp/test.har").build();
    let recorder = HarRecorder::new(options).unwrap();

    // Start a page
    recorder.start_page("page_1", "Test Page").await;

    // Record a request
    let mut headers = HashMap::new();
    headers.insert("Accept".to_string(), "application/json".to_string());

    recorder
        .record_request(
            "req_1",
            "https://example.com/api/test",
            "GET",
            &headers,
            None,
            "fetch",
            "frame_1",
        )
        .await;

    // Record response
    let mut response_headers = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "application/json".to_string());

    recorder
        .record_response(
            "req_1",
            200,
            "OK",
            &response_headers,
            "application/json",
            Some(br#"{"ok": true}"#),
            Some("127.0.0.1"),
        )
        .await;

    // Check HAR
    let har = recorder.get_har().await;
    assert_eq!(har.log.pages.len(), 1);
    assert_eq!(har.log.entries.len(), 1);
    assert_eq!(
        har.log.entries[0].request.url,
        "https://example.com/api/test"
    );
    assert_eq!(har.log.entries[0].response.status, 200);
}

#[tokio::test]
async fn test_har_recorder_url_filter() {
    let options = HarRecordingBuilder::new("/tmp/test.har")
        .url_filter("**/api/**")
        .build();
    let recorder = HarRecorder::new(options).unwrap();

    let headers = HashMap::new();

    // This should be recorded
    recorder
        .record_request(
            "req_1",
            "https://example.com/api/test",
            "GET",
            &headers,
            None,
            "fetch",
            "frame_1",
        )
        .await;

    // This should NOT be recorded (doesn't match filter)
    recorder
        .record_request(
            "req_2",
            "https://example.com/static/style.css",
            "GET",
            &headers,
            None,
            "stylesheet",
            "frame_1",
        )
        .await;

    let pending = recorder.pending_requests.read().await;
    assert_eq!(pending.len(), 1);
    assert!(pending.contains_key("req_1"));
}
