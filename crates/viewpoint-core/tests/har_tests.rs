#![cfg(feature = "integration")]

//! Tests for HAR (HTTP Archive) format support.

use std::collections::HashMap;

use viewpoint_core::network::{Har, HarEntry, HarPage, HarRequest, HarResponse, HarTimings};

// =========================================================================
// Har Tests
// =========================================================================

#[test]
fn test_har_creation() {
    let mut har = Har::new("viewpoint", "0.1.0");
    har.set_browser("Chrome", "120.0.0.0");

    let mut page = HarPage::new("page_1", "Test Page", "2024-01-01T00:00:00.000Z");
    page.set_timings(Some(100.0), Some(200.0));
    har.add_page(page);

    let mut entry = HarEntry::new("2024-01-01T00:00:00.100Z");
    entry.pageref = Some("page_1".to_string());

    let mut request = HarRequest::new("GET", "https://example.com/api/test");
    request.parse_query_string();
    entry.set_request(request);

    let mut response = HarResponse::new(200, "OK");
    response.set_content(Some(r#"{"ok": true}"#), "application/json", None);
    entry.set_response(response);

    har.add_entry(entry);

    let json = serde_json::to_string_pretty(&har).unwrap();
    assert!(json.contains("viewpoint"));
    assert!(json.contains("example.com"));
}

#[test]
fn test_har_version() {
    let har = Har::new("test", "1.0");
    assert_eq!(har.log.version, "1.2");
}

#[test]
fn test_har_serialization_roundtrip() {
    let mut har = Har::new("viewpoint", "0.1.0");
    har.set_browser("Chromium", "120.0");
    har.add_page(HarPage::new("page1", "Home", "2024-01-01T00:00:00Z"));

    let json = serde_json::to_string(&har).unwrap();
    let parsed: Har = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.log.version, "1.2");
    assert_eq!(parsed.log.creator.name, "viewpoint");
    assert!(parsed.log.browser.is_some());
    assert_eq!(parsed.log.pages.len(), 1);
}

// =========================================================================
// HarTimings Tests
// =========================================================================

#[test]
fn test_har_timings() {
    let timings = HarTimings {
        blocked: 10.0,
        dns: 20.0,
        connect: 30.0,
        send: 5.0,
        wait: 100.0,
        receive: 50.0,
        ssl: 15.0,
        comment: None,
    };

    // Testing exact float values for HAR timing calculations
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(timings.total(), 215.0);
    }
}

#[test]
fn test_har_timings_negative_values() {
    let timings = HarTimings {
        blocked: -1.0,
        dns: -1.0,
        connect: 30.0,
        send: 5.0,
        wait: 100.0,
        receive: -1.0,
        ssl: -1.0,
        comment: None,
    };

    // Only positive values are summed
    // Testing exact float values for HAR timing calculations
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(timings.total(), 135.0);
    }
}

#[test]
fn test_har_timings_from_resource_timing() {
    let timings = HarTimings::from_resource_timing(
        0.0,   // dns_start
        10.0,  // dns_end
        10.0,  // connect_start
        50.0,  // connect_end
        20.0,  // ssl_start
        40.0,  // ssl_end
        50.0,  // send_start
        55.0,  // send_end
        155.0, // receive_headers_end
    );

    // Testing exact float values for HAR timing calculations
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(timings.dns, 10.0);
        assert_eq!(timings.connect, 10.0); // ssl_start - connect_start when SSL exists
        assert_eq!(timings.ssl, 20.0);
        assert_eq!(timings.send, 5.0);
        assert_eq!(timings.wait, 100.0);
    }
}

// =========================================================================
// HarRequest Tests
// =========================================================================

#[test]
fn test_har_request_new() {
    let request = HarRequest::new("POST", "https://example.com/api");
    assert_eq!(request.method, "POST");
    assert_eq!(request.url, "https://example.com/api");
    assert_eq!(request.http_version, "HTTP/1.1");
}

#[test]
fn test_har_request_headers() {
    let mut request = HarRequest::new("GET", "https://example.com");
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Accept".to_string(), "text/html".to_string());

    request.set_headers(&headers);

    assert_eq!(request.headers.len(), 2);
    assert!(request.headers_size > 0);
}

#[test]
fn test_har_request_post_data() {
    let mut request = HarRequest::new("POST", "https://example.com/api");
    request.set_post_data(Some(r#"{"key": "value"}"#), Some("application/json"));

    assert!(request.post_data.is_some());
    let post_data = request.post_data.unwrap();
    assert_eq!(post_data.mime_type, "application/json");
    assert_eq!(post_data.text, r#"{"key": "value"}"#);
    assert_eq!(request.body_size, 16);
}

#[test]
fn test_har_request_query_string() {
    let mut request = HarRequest::new("GET", "https://example.com/search?q=test&page=1");
    request.parse_query_string();

    assert_eq!(request.query_string.len(), 2);
    assert!(
        request
            .query_string
            .iter()
            .any(|p| p.name == "q" && p.value == "test")
    );
    assert!(
        request
            .query_string
            .iter()
            .any(|p| p.name == "page" && p.value == "1")
    );
}

// =========================================================================
// HarResponse Tests
// =========================================================================

#[test]
fn test_har_response_new() {
    let response = HarResponse::new(404, "Not Found");
    assert_eq!(response.status, 404);
    assert_eq!(response.status_text, "Not Found");
}

#[test]
fn test_har_response_content() {
    let mut response = HarResponse::new(200, "OK");
    response.set_content(Some("Hello, World!"), "text/plain", None);

    assert_eq!(response.content.size, 13);
    assert_eq!(response.content.mime_type, "text/plain");
    assert_eq!(response.content.text.as_deref(), Some("Hello, World!"));
    assert_eq!(response.body_size, 13);
}

#[test]
fn test_har_response_base64_content() {
    let mut response = HarResponse::new(200, "OK");
    response.set_content(Some("SGVsbG8="), "application/octet-stream", Some("base64"));

    assert_eq!(response.content.encoding.as_deref(), Some("base64"));
}

#[test]
fn test_har_response_cookies() {
    let mut response = HarResponse::new(200, "OK");
    response.set_cookies(&[
        ("session".to_string(), "abc123".to_string()),
        ("theme".to_string(), "dark".to_string()),
    ]);

    assert_eq!(response.cookies.len(), 2);
    assert!(response.cookies.iter().any(|c| c.name == "session"));
}

#[test]
fn test_har_response_redirect() {
    let mut response = HarResponse::new(302, "Found");
    response.set_redirect_url("https://example.com/new-location");

    assert_eq!(response.redirect_url, "https://example.com/new-location");
}

// =========================================================================
// HarEntry Tests
// =========================================================================

#[test]
fn test_har_entry_new() {
    let entry = HarEntry::new("2024-01-01T12:00:00.000Z");
    assert_eq!(entry.started_date_time, "2024-01-01T12:00:00.000Z");
    // Testing exact float value for default timing
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(entry.time, 0.0);
    }
}

#[test]
fn test_har_entry_timings() {
    let mut entry = HarEntry::new("2024-01-01T12:00:00.000Z");
    let timings = HarTimings {
        blocked: 10.0,
        dns: 20.0,
        connect: 30.0,
        send: 5.0,
        wait: 100.0,
        receive: 50.0,
        ssl: -1.0,
        comment: None,
    };

    entry.set_timings(timings);
    // Testing exact float value for HAR timing calculations
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(entry.time, 215.0);
    }
}

#[test]
fn test_har_entry_server_ip() {
    let mut entry = HarEntry::new("2024-01-01T12:00:00.000Z");
    entry.set_server_ip("192.168.1.1");

    assert_eq!(entry.server_ip_address.as_deref(), Some("192.168.1.1"));
}

// =========================================================================
// HarPage Tests
// =========================================================================

#[test]
fn test_har_page_new() {
    let page = HarPage::new("page_1", "Home Page", "2024-01-01T00:00:00Z");
    assert_eq!(page.id, "page_1");
    assert_eq!(page.title, "Home Page");
}

#[test]
fn test_har_page_timings() {
    let mut page = HarPage::new("page_1", "Home", "2024-01-01T00:00:00Z");
    page.set_timings(Some(500.0), Some(1_000.0));

    // Testing exact float values for HAR timing calculations
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(page.page_timings.on_content_load, Some(500.0));
        assert_eq!(page.page_timings.on_load, Some(1_000.0));
    }
}
