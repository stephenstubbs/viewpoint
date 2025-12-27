#![cfg(feature = "integration")]

//! Tests for the #[viewpoint::test] macro.
//!
//! These tests verify that the test macro correctly:
//! - Injects fixture parameters (Page, BrowserContext, Browser)
//! - Applies configuration options (headless, timeout)
//! - Generates correct harness setup code

use std::sync::Once;
use std::time::Duration;

use viewpoint_core::DocumentLoadState;
use viewpoint_test::{expect_page, TestHarness, Page, BrowserContext, Browser};

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

// =============================================================================
// Note: The macro tests below use viewpoint_test::test attribute macro.
// These tests verify the macro works correctly by exercising it directly.
// =============================================================================

/// Test that the macro correctly injects a Page fixture.
#[viewpoint_test::test]
async fn test_macro_page_fixture(page: Page) {
    init_tracing();
    
    // Verify the page is working
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");
    
    expect_page(&page)
        .to_have_title("Example Domain")
        .await
        .expect("should have correct title");
}

/// Test that the macro correctly injects multiple fixtures.
#[viewpoint_test::test]
async fn test_macro_multiple_fixtures(page: Page, context: BrowserContext) {
    init_tracing();
    
    // Both fixtures should be available and working
    assert!(!page.is_closed(), "page should not be closed");
    assert!(!context.is_closed(), "context should not be closed");
    
    // Navigate to verify page works
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");
}

/// Test that the macro applies headless configuration.
#[viewpoint_test::test(headless = true)]
async fn test_macro_headless_configuration(page: Page) {
    init_tracing();
    
    // The test runs in headless mode - verify page works
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate in headless mode");
}

/// Test that the macro applies timeout configuration.
#[viewpoint_test::test(timeout = 60000)]
async fn test_macro_timeout_configuration(page: Page) {
    init_tracing();
    
    // The test has a 60 second timeout - verify page works
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate with custom timeout");
}

/// Test that the macro applies both headless and timeout configuration.
#[viewpoint_test::test(headless = true, timeout = 45000)]
async fn test_macro_combined_configuration(page: Page) {
    init_tracing();
    
    // Both configurations should be applied
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate with combined config");
}

// =============================================================================
// Manual TestHarness tests for scope functionality
// (Scope tests use manual setup since shared fixtures need explicit setup)
// =============================================================================

/// Test creating a harness from an existing browser (browser scope simulation).
#[tokio::test]
async fn test_harness_browser_scope() {
    init_tracing();

    // Create a shared browser
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");

    // Create harness from the browser - simulates browser scope
    let harness = TestHarness::from_browser(&browser)
        .await
        .expect("should create harness from browser");

    // Verify we get a working page
    let page = harness.page();
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Clean up harness first
    drop(harness);
    
    // Then close the browser
    browser.close().await.expect("should close browser");
}

/// Test creating a harness from an existing context (context scope simulation).
#[tokio::test]
async fn test_harness_context_scope() {
    init_tracing();

    // Create shared browser and context
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");
    
    let context = browser.new_context()
        .await
        .expect("should create context");

    // Create harness from the context - simulates context scope
    let harness = TestHarness::from_context(&context)
        .await
        .expect("should create harness from context");

    // Verify we get a working page
    let page = harness.page();
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Clean up in order
    drop(harness);
    // context and browser will be cleaned up when dropped
}

/// Test that harness from_browser doesn't own the browser.
#[tokio::test]
async fn test_harness_from_browser_ownership() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");

    let harness = TestHarness::from_browser(&browser)
        .await
        .expect("should create harness");

    // Browser should be None (we don't own it)
    assert!(harness.browser().is_none(), "harness should not own browser");
    
    // But context and page should be available
    assert!(harness.context().is_some(), "context should be available");
    assert!(!harness.page().is_closed(), "page should be open");

    drop(harness);
    browser.close().await.expect("should close browser");
}

/// Test that harness from_context doesn't own browser or context.
#[tokio::test]
async fn test_harness_from_context_ownership() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("should launch browser");
    
    let context = browser.new_context()
        .await
        .expect("should create context");

    let harness = TestHarness::from_context(&context)
        .await
        .expect("should create harness");

    // Neither browser nor context should be owned
    assert!(harness.browser().is_none(), "harness should not own browser");
    assert!(harness.context().is_none(), "harness should not own context");
    
    // But page should be available
    assert!(!harness.page().is_closed(), "page should be open");

    drop(harness);
}

// =============================================================================
// Default behavior tests
// =============================================================================

/// Test that default harness runs in headless mode.
#[tokio::test]
async fn test_default_harness_is_headless() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    
    // Default config should be headless
    assert!(harness.config().headless, "default should be headless");
    
    // Default timeout should be 30 seconds
    assert_eq!(
        harness.config().timeout,
        Duration::from_secs(30),
        "default timeout should be 30 seconds"
    );
}
