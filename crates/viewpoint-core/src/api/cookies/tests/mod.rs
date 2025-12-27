use super::*;

#[test]
fn test_cookie_to_url() {
    let cookie = Cookie::new("test", "value")
        .domain("example.com")
        .path("/api")
        .secure(true);

    let url = cookie_to_url(&cookie);
    assert_eq!(url, "https://example.com/api");
}

#[test]
fn test_cookie_to_url_with_url() {
    let cookie = Cookie::new("test", "value")
        .url("https://custom.com/path");

    let url = cookie_to_url(&cookie);
    assert_eq!(url, "https://custom.com/path");
}

#[test]
fn test_cookie_to_string() {
    let cookie = Cookie::new("session", "abc123")
        .domain("example.com")
        .path("/")
        .secure(true)
        .http_only(true);

    let cookie_str = cookie_to_string(&cookie);
    assert!(cookie_str.contains("session=abc123"));
    assert!(cookie_str.contains("Domain=example.com"));
    assert!(cookie_str.contains("Secure"));
    assert!(cookie_str.contains("HttpOnly"));
}
