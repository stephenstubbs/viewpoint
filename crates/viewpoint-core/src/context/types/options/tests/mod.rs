use super::*;

#[test]
fn test_context_options_builder() {
    let options = ContextOptionsBuilder::new()
        .geolocation(37.7749, -122.4194)
        .permissions(vec![Permission::Geolocation, Permission::Notifications])
        .http_credentials("user", "pass")
        .header("X-Custom", "value")
        .offline(true)
        .has_touch(true)
        .build();

    assert!(options.geolocation.is_some());
    assert_eq!(options.permissions.len(), 2);
    assert!(options.http_credentials.is_some());
    assert_eq!(
        options.extra_http_headers.get("X-Custom"),
        Some(&"value".to_string())
    );
    assert!(options.offline);
    assert!(options.has_touch);
}
