//! Tests for API request builder types.

use viewpoint_core::api::{HttpMethod, MultipartField};

// We need to test internal APIs, so we'll test what's publicly available
// The private APIRequestBuilder is tested indirectly via integration tests

#[test]
fn test_multipart_field_text() {
    let field = MultipartField::text("name", "value");
    assert_eq!(field.name, "name");
    assert_eq!(field.value, Some("value".to_string()));
    assert!(field.file_content.is_none());
}

#[test]
fn test_multipart_field_file() {
    let field = MultipartField::file("file", "test.txt", vec![1, 2, 3]).content_type("text/plain");
    assert_eq!(field.name, "file");
    assert_eq!(field.filename, Some("test.txt".to_string()));
    assert_eq!(field.file_content, Some(vec![1, 2, 3]));
    assert_eq!(field.content_type, Some("text/plain".to_string()));
}

#[test]
fn test_http_method_to_reqwest() {
    // Test that HttpMethod converts correctly to reqwest methods
    assert_eq!(HttpMethod::Get.to_reqwest(), reqwest::Method::GET);
    assert_eq!(HttpMethod::Post.to_reqwest(), reqwest::Method::POST);
    assert_eq!(HttpMethod::Put.to_reqwest(), reqwest::Method::PUT);
    assert_eq!(HttpMethod::Delete.to_reqwest(), reqwest::Method::DELETE);
    assert_eq!(HttpMethod::Patch.to_reqwest(), reqwest::Method::PATCH);
    assert_eq!(HttpMethod::Head.to_reqwest(), reqwest::Method::HEAD);
}
