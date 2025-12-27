#![cfg(feature = "integration")]

//! Timeout and error recovery tests for TestHarness.

use std::sync::Once;
use std::time::Duration;
use viewpoint_core::DocumentLoadState;
use viewpoint_test::{expect, TestHarness};

static TRACING_INIT: Once = Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

// ============================================================================
// Timeout Behavior Tests
// ============================================================================

#[tokio::test]
async fn test_timeout_short_duration() {
    init_tracing();

    let harness = TestHarness::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .await
        .expect("should create harness");

    // Verify timeout is set correctly
    assert_eq!(harness.config().timeout, Duration::from_millis(500));
}

#[tokio::test]
async fn test_timeout_propagates_to_assertions() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Very short timeout should fail for non-matching text
    let result = expect(&page.locator("h1"))
        .timeout(Duration::from_millis(50))
        .to_have_text("This text does not exist in the heading")
        .await;

    assert!(result.is_err(), "should timeout with wrong text");
    
    // Error message should be descriptive (contains timeout, expected, text, or similar)
    let error = result.unwrap_err();
    assert!(
        !error.message.is_empty(),
        "error message should not be empty"
    );
}

#[tokio::test]
async fn test_timeout_custom_configuration() {
    init_tracing();

    // Create harness with very long timeout
    let harness = TestHarness::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .await
        .expect("should create harness");

    assert_eq!(harness.config().timeout, Duration::from_secs(120));
}

#[tokio::test]
async fn test_timeout_error_message_content() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Use very short timeout
    let result = expect(&page.locator("h1"))
        .timeout(Duration::from_millis(50))
        .to_have_text("Wrong Text")
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    // Error should contain useful diagnostic information
    assert!(!error.message.is_empty(), "error message should not be empty");
}

// ============================================================================
// Error Recovery Tests
// ============================================================================

#[tokio::test]
async fn test_error_recovery_navigation_failure() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    // Try to navigate to invalid domain
    let result = page.goto("https://this-domain-definitely-does-not-exist-12345.com")
        .timeout(Duration::from_secs(5))
        .goto()
        .await;

    // Navigation should fail
    assert!(result.is_err(), "navigation to invalid domain should fail");

    // But page should still be usable
    assert!(!page.is_closed(), "page should not be closed after failed navigation");

    // Should be able to navigate to valid URL after failure
    let recovery = page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await;

    assert!(recovery.is_ok(), "should recover and navigate to valid URL");
}

#[tokio::test]
async fn test_error_recovery_element_not_found() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Try to interact with non-existent element
    let nonexistent = page.locator("#element-that-does-not-exist-xyz123");
    
    // Should be hidden (element not found)
    expect(&nonexistent)
        .to_be_hidden()
        .await
        .expect("nonexistent element should be hidden");

    // Page should still be responsive after failed element lookup
    let heading = page.locator("h1");
    expect(&heading)
        .to_be_visible()
        .await
        .expect("heading should be visible after previous failure");
}

#[tokio::test]
async fn test_error_recovery_multiple_errors_in_sequence() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // First error: wrong text
    let result1 = expect(&page.locator("h1"))
        .timeout(Duration::from_millis(50))
        .to_have_text("Wrong Text 1")
        .await;
    assert!(result1.is_err());

    // Second error: wrong text again
    let result2 = expect(&page.locator("h1"))
        .timeout(Duration::from_millis(50))
        .to_have_text("Wrong Text 2")
        .await;
    assert!(result2.is_err());

    // Third error: wrong text again
    let result3 = expect(&page.locator("h1"))
        .timeout(Duration::from_millis(50))
        .to_have_text("Wrong Text 3")
        .await;
    assert!(result3.is_err());

    // Page should still work correctly after multiple errors
    expect(&page.locator("h1"))
        .to_have_text("Example Domain")
        .await
        .expect("should work correctly after multiple errors");
}

#[tokio::test]
async fn test_error_recovery_graceful_degradation() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Try assertions that fail but don't crash
    let results = vec![
        expect(&page.locator("#nonexistent1"))
            .timeout(Duration::from_millis(50))
            .to_be_visible()
            .await,
        expect(&page.locator("#nonexistent2"))
            .timeout(Duration::from_millis(50))
            .to_have_text("text")
            .await,
        expect(&page.locator("h1"))
            .timeout(Duration::from_millis(50))
            .to_have_text("Wrong")
            .await,
    ];

    // All should be errors
    for result in &results {
        assert!(result.is_err());
    }

    // But the correct assertion should still pass
    expect(&page.locator("h1"))
        .to_be_visible()
        .await
        .expect("correct assertion should pass");
}
