#![cfg(feature = "integration")]

//! Integration tests for TestHarness.

use std::sync::Once;
use std::time::Duration;
use viewpoint_core::DocumentLoadState;
use viewpoint_test::{expect, expect_page, TestConfig, TestHarness};

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
// Harness Creation Tests
// ============================================================================

#[tokio::test]
async fn test_harness_new_creates_page() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");

    // Verify we have access to fixtures
    let page = harness.page();
    assert!(!page.is_closed());

    // Verify browser and context are accessible
    assert!(harness.browser().is_some());
    assert!(harness.context().is_some());
}

#[tokio::test]
async fn test_harness_builder_configuration() {
    init_tracing();

    let harness = TestHarness::builder()
        .headless(true)
        .timeout(Duration::from_secs(60))
        .build()
        .await
        .expect("should create harness");

    assert_eq!(harness.config().timeout, Duration::from_secs(60));
    assert!(harness.config().headless);
}

#[tokio::test]
async fn test_harness_new_page() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");

    // Create additional page
    let page2 = harness.new_page().await.expect("should create new page");

    // Both pages should be valid
    assert!(!harness.page().is_closed());
    assert!(!page2.is_closed());
}

#[tokio::test]
async fn test_harness_explicit_close() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");

    // Explicit close should work
    harness.close().await.expect("should close harness");
}

#[tokio::test]
async fn test_config_defaults() {
    let config = TestConfig::default();

    assert!(config.headless);
    assert_eq!(config.timeout, Duration::from_secs(30));
}

#[tokio::test]
async fn test_config_builder() {
    let config = TestConfig::builder()
        .headless(false)
        .timeout(Duration::from_secs(120))
        .build();

    assert!(!config.headless);
    assert_eq!(config.timeout, Duration::from_secs(120));
}

#[tokio::test]
async fn test_harness_from_browser() {
    init_tracing();

    // Create a browser directly
    let browser = viewpoint_test::Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");

    // Create harness from existing browser
    let harness = TestHarness::from_browser(&browser)
        .await
        .expect("should create harness from browser");

    // Should have page and context, but browser() returns None (not owned)
    assert!(!harness.page().is_closed());
    assert!(harness.context().is_some());
    assert!(harness.browser().is_none()); // We don't own the browser

    // Clean up
    drop(harness);
    browser.close().await.expect("should close browser");
}

#[tokio::test]
async fn test_harness_from_context() {
    init_tracing();

    // Create browser and context directly
    let browser = viewpoint_test::Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");

    let context = browser
        .new_context()
        .await
        .expect("should create context");

    // Create harness from existing context
    let harness = TestHarness::from_context(&context)
        .await
        .expect("should create harness from context");

    // Should have page, but context() and browser() return None (not owned)
    assert!(!harness.page().is_closed());
    assert!(harness.context().is_none()); // We don't own the context
    assert!(harness.browser().is_none()); // We don't own the browser

    // Clean up
    drop(harness);
    // context and browser will be cleaned up when dropped
}

// ============================================================================
// Assertion Tests
// ============================================================================

#[tokio::test]
async fn test_expect_to_be_visible() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");
    expect(&heading)
        .to_be_visible()
        .await
        .expect("heading should be visible");
}

#[tokio::test]
async fn test_expect_to_be_hidden() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Non-existent element should be hidden
    let nonexistent = page.locator("#does-not-exist-xyz123");
    expect(&nonexistent)
        .to_be_hidden()
        .await
        .expect("nonexistent element should be hidden");
}

#[tokio::test]
async fn test_expect_to_have_text() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");
    expect(&heading)
        .to_have_text("Example Domain")
        .await
        .expect("heading should have expected text");
}

#[tokio::test]
async fn test_expect_to_contain_text() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");
    expect(&heading)
        .to_contain_text("Example")
        .await
        .expect("heading should contain text");
}

#[tokio::test]
async fn test_expect_text_assertion_failure() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");
    let result = expect(&heading)
        .timeout(Duration::from_millis(100))
        .to_have_text("Not The Right Text")
        .await;

    assert!(result.is_err(), "should fail with wrong text");
    let err = result.unwrap_err();
    assert!(err.message.contains("text"), "error should mention text");
}

#[tokio::test]
async fn test_expect_page_to_have_url() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com/")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    expect_page(page)
        .to_have_url("https://example.com/")
        .await
        .expect("page should have expected URL");
}

#[tokio::test]
async fn test_expect_page_to_have_url_containing() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com/")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    expect_page(page)
        .to_have_url_containing("example.com")
        .await
        .expect("page URL should contain 'example.com'");
}

#[tokio::test]
async fn test_expect_page_to_have_title() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    expect_page(page)
        .to_have_title("Example Domain")
        .await
        .expect("page should have expected title");
}

#[tokio::test]
async fn test_expect_not_negation() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");
    
    // not().to_have_text() should pass when text doesn't match
    expect(&heading)
        .not()
        .to_have_text("Wrong Text")
        .await
        .expect("not assertion should pass");
}

#[tokio::test]
async fn test_expect_to_be_checked() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("should navigate");

    // Find an unchecked checkbox
    let checkbox = page.locator("input[type='checkbox'][value='cheese']");
    
    // Check it
    checkbox.check().await.expect("should check");
    
    // Verify it's checked
    expect(&checkbox)
        .to_be_checked()
        .await
        .expect("checkbox should be checked");
    
    // Uncheck it
    checkbox.uncheck().await.expect("should uncheck");
    
    // Verify it's not checked
    expect(&checkbox)
        .not()
        .to_be_checked()
        .await
        .expect("checkbox should not be checked");
}

// ============================================================================
// Harness Cleanup Tests
// ============================================================================

#[tokio::test]
async fn test_harness_close_explicit() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();
    
    // Do some work
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Explicitly close
    harness.close().await.expect("should close harness");
}

#[tokio::test]
async fn test_harness_cleanup_on_drop() {
    init_tracing();

    {
        let harness = TestHarness::new().await.expect("should create harness");
        let page = harness.page();
        
        page.goto("https://example.com")
            .wait_until(DocumentLoadState::DomContentLoaded)
            .goto()
            .await
            .expect("should navigate");
        
        // harness will be dropped here
    }

    // If we get here, drop didn't crash
}
