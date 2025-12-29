//! Tests for network types - glob matching, URL patterns, resource types, abort errors.

use std::collections::HashSet;

use viewpoint_core::network::{AbortError, ResourceType, UrlMatcher, UrlPattern};

// =========================================================================
// Helper function for glob matching tests
// =========================================================================

/// Match a URL against a glob pattern.
///
/// Pattern syntax:
/// - `*` matches any characters except `/`
/// - `**` matches any characters including `/`
/// - `?` matches a literal `?` (not single character like shell globs)
fn glob_match(pattern: &str, url: &str) -> bool {
    // We use UrlPattern's implementation for testing
    let url_pattern = UrlPattern::glob(pattern);
    url_pattern.matches(url)
}

// =========================================================================
// Glob Pattern Tests
// =========================================================================

#[test]
fn test_glob_match_simple() {
    assert!(glob_match("*.png", "image.png"));
    assert!(glob_match("*.png", "photo.png"));
    assert!(!glob_match("*.png", "image.jpg"));
    assert!(!glob_match("*.png", "dir/image.png"));
}

#[test]
fn test_glob_match_double_star() {
    assert!(glob_match("**/*.png", "image.png"));
    assert!(glob_match("**/*.png", "dir/image.png"));
    assert!(glob_match("**/*.png", "a/b/c/image.png"));
    assert!(!glob_match("**/*.png", "image.jpg"));
}

#[test]
fn test_glob_match_url_patterns() {
    assert!(glob_match("**/api/**", "https://example.com/api/users"));
    assert!(glob_match("**/api/**", "http://localhost:3000/api/data"));
    assert!(!glob_match("**/api/**", "https://example.com/other/path"));
}

#[test]
fn test_glob_match_question_mark() {
    // ? should match literal ? in URLs
    assert!(glob_match(
        "**/search?*",
        "https://example.com/search?q=test"
    ));
    assert!(!glob_match(
        "**/search?*",
        "https://example.com/searcha=test"
    ));
}

#[test]
fn test_glob_match_exact() {
    assert!(glob_match(
        "https://example.com/exact",
        "https://example.com/exact"
    ));
    assert!(!glob_match(
        "https://example.com/exact",
        "https://example.com/other"
    ));
}

#[test]
fn test_glob_match_single_star_boundary() {
    // Single * should not match across /
    assert!(glob_match("*://example.com/*", "https://example.com/path"));
    assert!(glob_match("*://example.com/*", "http://example.com/path"));
    assert!(!glob_match("*://example.com/*", "https://example.com/a/b"));
}

#[test]
fn test_glob_match_special_chars() {
    // Test regex special characters are escaped
    assert!(glob_match("**/*.min.js", "https://example.com/app.min.js"));
    assert!(glob_match(
        "**/file[1].txt",
        "https://example.com/file[1].txt"
    ));
    assert!(glob_match("**/path+name", "https://example.com/path+name"));
}

#[test]
fn test_glob_match_empty_pattern() {
    assert!(glob_match("", ""));
    assert!(!glob_match("", "something"));
}

#[test]
fn test_glob_match_double_star_only() {
    assert!(glob_match("**", "anything/goes/here"));
    assert!(glob_match("**", ""));
    assert!(glob_match("**", "a"));
}

#[test]
fn test_glob_match_unicode() {
    assert!(glob_match(
        "**/données/*",
        "https://example.com/données/file.txt"
    ));
    assert!(glob_match("**/*.日本語", "https://example.com/file.日本語"));
}

// =========================================================================
// UrlPattern Tests
// =========================================================================

#[test]
fn test_url_pattern_glob() {
    let pattern = UrlPattern::glob("**/*.css");
    assert!(pattern.matches("https://example.com/style.css"));
    assert!(pattern.matches("https://example.com/assets/main.css"));
    assert!(!pattern.matches("https://example.com/script.js"));
}

#[test]
fn test_url_pattern_regex() {
    let pattern = UrlPattern::regex(r"\.png$").unwrap();
    assert!(pattern.matches("https://example.com/image.png"));
    assert!(!pattern.matches("https://example.com/image.jpg"));
}

#[test]
fn test_url_pattern_regex_complex() {
    let pattern = UrlPattern::regex(r"^https://[^/]+/api/v\d+/").unwrap();
    assert!(pattern.matches("https://example.com/api/v1/users"));
    assert!(pattern.matches("https://api.example.com/api/v2/data"));
    assert!(!pattern.matches("http://example.com/api/v1/users")); // http not https
    assert!(!pattern.matches("https://example.com/api/users")); // missing version
}

#[test]
fn test_url_pattern_from_str() {
    let pattern: UrlPattern = "**/*.js".into();
    assert!(pattern.matches("https://example.com/app.js"));
}

#[test]
fn test_url_pattern_from_string() {
    let pattern: UrlPattern = String::from("**/*.css").into();
    assert!(pattern.matches("https://example.com/style.css"));
}

#[test]
fn test_url_pattern_from_regex() {
    let regex = regex::Regex::new(r"\.json$").unwrap();
    let pattern: UrlPattern = regex.into();
    assert!(pattern.matches("https://example.com/data.json"));
}

// =========================================================================
// UrlMatcher Trait Tests
// =========================================================================

#[test]
fn test_url_matcher_str() {
    // Test UrlMatcher impl for str via glob_match
    assert!(UrlMatcher::matches(
        "**/*.html",
        "https://example.com/page.html"
    ));
}

#[test]
fn test_url_matcher_string() {
    let pattern = String::from("**/*.xml");
    assert!(UrlMatcher::matches(
        &pattern,
        "https://example.com/feed.xml"
    ));
}

#[test]
fn test_url_matcher_regex() {
    let regex = regex::Regex::new(r"^https://").unwrap();
    assert!(UrlMatcher::matches(&regex, "https://example.com"));
    assert!(!UrlMatcher::matches(&regex, "http://example.com"));
}

#[test]
fn test_url_matcher_closure() {
    let matcher = |url: &str| url.contains("api");
    assert!(UrlMatcher::matches(
        &matcher,
        "https://example.com/api/users"
    ));
    assert!(!UrlMatcher::matches(&matcher, "https://example.com/home"));
}

// =========================================================================
// AbortError Tests
// =========================================================================

#[test]
fn test_abort_error_display() {
    assert_eq!(
        AbortError::ConnectionRefused.to_string(),
        "ConnectionRefused"
    );
    assert_eq!(AbortError::TimedOut.to_string(), "TimedOut");
}

#[test]
fn test_abort_error_all_variants() {
    let variants = [
        (AbortError::Failed, "Failed"),
        (AbortError::Aborted, "Aborted"),
        (AbortError::TimedOut, "TimedOut"),
        (AbortError::AccessDenied, "AccessDenied"),
        (AbortError::ConnectionClosed, "ConnectionClosed"),
        (AbortError::ConnectionReset, "ConnectionReset"),
        (AbortError::ConnectionRefused, "ConnectionRefused"),
        (AbortError::ConnectionAborted, "ConnectionAborted"),
        (AbortError::ConnectionFailed, "ConnectionFailed"),
        (AbortError::NameNotResolved, "NameNotResolved"),
        (AbortError::InternetDisconnected, "InternetDisconnected"),
        (AbortError::AddressUnreachable, "AddressUnreachable"),
        (AbortError::BlockedByClient, "BlockedByClient"),
        (AbortError::BlockedByResponse, "BlockedByResponse"),
    ];

    for (error, expected) in variants {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn test_abort_error_default() {
    assert_eq!(AbortError::default(), AbortError::Failed);
}

// =========================================================================
// ResourceType Tests
// =========================================================================

#[test]
fn test_resource_type_display() {
    assert_eq!(ResourceType::Document.to_string(), "document");
    assert_eq!(ResourceType::Script.to_string(), "script");
}

#[test]
fn test_resource_type_all_variants() {
    let variants = [
        (ResourceType::Document, "document"),
        (ResourceType::Stylesheet, "stylesheet"),
        (ResourceType::Image, "image"),
        (ResourceType::Media, "media"),
        (ResourceType::Font, "font"),
        (ResourceType::Script, "script"),
        (ResourceType::TextTrack, "texttrack"),
        (ResourceType::Xhr, "xhr"),
        (ResourceType::Fetch, "fetch"),
        (ResourceType::Prefetch, "prefetch"),
        (ResourceType::EventSource, "eventsource"),
        (ResourceType::WebSocket, "websocket"),
        (ResourceType::Manifest, "manifest"),
        (ResourceType::SignedExchange, "signedexchange"),
        (ResourceType::Ping, "ping"),
        (ResourceType::CspViolationReport, "cspviolationreport"),
        (ResourceType::Preflight, "preflight"),
        (ResourceType::Other, "other"),
    ];

    for (resource_type, expected) in variants {
        assert_eq!(resource_type.to_string(), expected);
    }
}

#[test]
fn test_resource_type_equality() {
    assert_eq!(ResourceType::Document, ResourceType::Document);
    assert_ne!(ResourceType::Document, ResourceType::Script);
}

#[test]
fn test_resource_type_hash() {
    let mut set = HashSet::new();
    set.insert(ResourceType::Document);
    set.insert(ResourceType::Script);
    set.insert(ResourceType::Document); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&ResourceType::Document));
    assert!(set.contains(&ResourceType::Script));
}
