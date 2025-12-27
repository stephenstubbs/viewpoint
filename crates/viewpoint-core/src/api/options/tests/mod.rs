use super::*;

#[test]
fn test_options_builder() {
    let options = APIContextOptions::new()
        .base_url("https://api.example.com")
        .timeout(Duration::from_secs(30))
        .ignore_https_errors(true)
        .header("Authorization", "Bearer token");

    assert_eq!(options.base_url, Some("https://api.example.com".to_string()));
    assert_eq!(options.timeout, Some(Duration::from_secs(30)));
    assert!(options.ignore_https_errors);
    assert_eq!(
        options.extra_http_headers.get("Authorization"),
        Some(&"Bearer token".to_string())
    );
}

#[test]
fn test_http_credentials() {
    let creds = HttpCredentials::new("user", "pass")
        .origin("https://example.com")
        .send(CredentialSend::Always);

    assert_eq!(creds.username, "user");
    assert_eq!(creds.password, "pass");
    assert_eq!(creds.origin, Some("https://example.com".to_string()));
    assert_eq!(creds.send, Some(CredentialSend::Always));
}

#[test]
fn test_proxy_config() {
    let proxy = ProxyConfig::new("http://proxy:8080")
        .credentials("user", "pass")
        .bypass("localhost,127.0.0.1");

    assert_eq!(proxy.server, "http://proxy:8080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
    assert_eq!(proxy.bypass, Some("localhost,127.0.0.1".to_string()));
}
