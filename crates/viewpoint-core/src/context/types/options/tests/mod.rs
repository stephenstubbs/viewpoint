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

#[test]
fn test_context_options_builder_with_proxy() {
    let options = ContextOptionsBuilder::new()
        .proxy(ProxyConfig::new("http://proxy.example.com:8080"))
        .build();

    assert!(options.proxy.is_some());
    let proxy = options.proxy.unwrap();
    assert_eq!(proxy.server, "http://proxy.example.com:8080");
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
    assert!(proxy.bypass.is_none());
}

#[test]
fn test_context_options_builder_with_proxy_credentials() {
    let options = ContextOptionsBuilder::new()
        .proxy(
            ProxyConfig::new("socks5://proxy.example.com:1080")
                .credentials("user", "password")
                .bypass("localhost,127.0.0.1"),
        )
        .build();

    assert!(options.proxy.is_some());
    let proxy = options.proxy.unwrap();
    assert_eq!(proxy.server, "socks5://proxy.example.com:1080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("password".to_string()));
    assert_eq!(proxy.bypass, Some("localhost,127.0.0.1".to_string()));
}
