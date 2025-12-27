use super::*;

#[test]
fn test_har_replay_options_builder() {
    let options = HarReplayOptions::new()
        .url("**/api/**")
        .strict(true)
        .update(false)
        .use_original_timing(true);

    assert!(options.url_filter.is_some());
    assert!(options.strict);
    assert!(!options.update);
    assert!(options.use_original_timing);
}

#[test]
fn test_url_matching() {
    let handler = HarReplayHandler::from_har(Har::new("test", "1.0"));

    // Exact match
    assert!(handler.url_matches(
        "https://example.com/api/users",
        "https://example.com/api/users"
    ));

    // Different paths
    assert!(!handler.url_matches(
        "https://example.com/api/users",
        "https://example.com/api/posts"
    ));

    // Query params - HAR params must be in request
    assert!(handler.url_matches(
        "https://example.com/api?a=1",
        "https://example.com/api?a=1&b=2"
    ));

    // Missing required param
    assert!(!handler.url_matches(
        "https://example.com/api?a=1",
        "https://example.com/api?b=2"
    ));
}

#[test]
fn test_post_data_matching() {
    let handler = HarReplayHandler::from_har(Har::new("test", "1.0"));

    // Exact string match
    assert!(handler.post_data_matches("hello", "hello"));

    // JSON semantic match (order doesn't matter)
    assert!(handler.post_data_matches(
        r#"{"a":1,"b":2}"#,
        r#"{"b":2,"a":1}"#
    ));

    // Different values
    assert!(!handler.post_data_matches(
        r#"{"a":1}"#,
        r#"{"a":2}"#
    ));
}
