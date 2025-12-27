use super::*;

#[test]
fn test_url_pattern_matching() {
    let pattern = UrlPattern::glob("**/*.css");
    assert!(pattern.matches("https://example.com/style.css"));
    assert!(pattern.matches("https://example.com/assets/main.css"));
    assert!(!pattern.matches("https://example.com/script.js"));
}

// Note: Full integration tests for fallback chaining require a browser connection.
// The logic is:
// 1. Multiple handlers can match the same URL pattern
// 2. Handlers are tried in reverse registration order (LIFO)
// 3. If a handler calls route.fallback(), the next handler is tried
// 4. If no handler handles the request (all call fallback), request continues to network
//
// See integration_tests.rs for full browser-based tests.
