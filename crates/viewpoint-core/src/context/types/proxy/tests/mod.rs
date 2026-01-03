use super::*;

#[test]
fn test_proxy_config_new() {
    let proxy = ProxyConfig::new("http://proxy:8080");
    assert_eq!(proxy.server, "http://proxy:8080");
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
    assert!(proxy.bypass.is_none());
}

#[test]
fn test_proxy_config_with_credentials() {
    let proxy = ProxyConfig::new("http://proxy:8080").credentials("user", "pass");
    assert_eq!(proxy.server, "http://proxy:8080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
}

#[test]
fn test_proxy_config_with_bypass() {
    let proxy = ProxyConfig::new("socks5://proxy:1080").bypass("localhost,127.0.0.1");
    assert_eq!(proxy.server, "socks5://proxy:1080");
    assert_eq!(proxy.bypass, Some("localhost,127.0.0.1".to_string()));
}

#[test]
fn test_proxy_config_full() {
    let proxy = ProxyConfig::new("http://proxy:8080")
        .credentials("user", "pass")
        .bypass("localhost,127.0.0.1");
    assert_eq!(proxy.server, "http://proxy:8080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
    assert_eq!(proxy.bypass, Some("localhost,127.0.0.1".to_string()));
}
