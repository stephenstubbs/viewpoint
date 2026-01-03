use super::*;

#[test]
fn test_cookie_builder() {
    let cookie = Cookie::new("session", "abc123")
        .domain("example.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict);

    assert_eq!(cookie.name, "session");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, Some("/".to_string()));
    assert_eq!(cookie.secure, Some(true));
    assert_eq!(cookie.http_only, Some(true));
    assert_eq!(cookie.same_site, Some(SameSite::Strict));
}

#[test]
fn test_cookie_serialization() {
    let cookie = Cookie::new("test", "value").url("https://example.com");
    let json = serde_json::to_string(&cookie).unwrap();
    assert!(json.contains("\"name\":\"test\""));
    assert!(json.contains("\"value\":\"value\""));
    assert!(json.contains("\"url\":\"https://example.com\""));
}

#[test]
fn test_geolocation() {
    let geo = Geolocation::new(40.7128, -74.0060);
    // Testing exact float values from geolocation constructor
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(geo.latitude, 40.7128);
        assert_eq!(geo.longitude, -74.0060);
        assert_eq!(geo.accuracy, 0.0);
    }

    let geo_accurate = Geolocation::with_accuracy(40.7128, -74.0060, 100.0);
    // Testing exact float value for accuracy
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(geo_accurate.accuracy, 100.0);
    }
}
