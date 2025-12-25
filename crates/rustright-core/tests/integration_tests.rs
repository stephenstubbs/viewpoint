//! Integration tests for rustright-core.
//!
//! These tests require Chromium to be installed and accessible.
//! Run with: `cargo test --test integration_tests`
//! Run with tracing: `RUST_LOG=debug cargo test --test integration_tests -- --nocapture`

use std::sync::Once;
use std::time::Duration;

use rustright_core::{Browser, DocumentLoadState};

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
/// This is safe to call multiple times - it will only initialize once.
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

/// Test launching a browser, verifying connection, and closing.
#[tokio::test]
async fn test_browser_launch_and_close() {
    init_tracing();
    
    // Launch a headless browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Verify the browser is owned (we launched it)
    assert!(browser.is_owned());

    // Close the browser
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a browser context.
#[tokio::test]
async fn test_browser_context_creation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create a new context
    let context = browser.new_context().await.expect("Failed to create context");

    // Verify context has an ID
    assert!(!context.id().is_empty());
    assert!(!context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a page within a context.
#[tokio::test]
async fn test_page_creation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");

    // Create a new page
    let page = context.new_page().await.expect("Failed to create page");

    // Verify page has IDs
    assert!(!page.target_id().is_empty());
    assert!(!page.session_id().is_empty());
    assert!(!page.frame_id().is_empty());
    assert!(!page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a page.
#[tokio::test]
async fn test_page_close() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let mut page = context.new_page().await.expect("Failed to create page");

    assert!(!page.is_closed());

    // Close the page
    page.close().await.expect("Failed to close page");

    assert!(page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a browser context.
#[tokio::test]
async fn test_context_close() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let mut context = browser.new_context().await.expect("Failed to create context");

    // Create some pages in the context
    let _page1 = context.new_page().await.expect("Failed to create page 1");
    let _page2 = context.new_page().await.expect("Failed to create page 2");

    assert!(!context.is_closed());

    // Close the context (should close all pages)
    context.close().await.expect("Failed to close context");

    assert!(context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test basic navigation to a URL.
#[tokio::test]
async fn test_basic_navigation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to example.com
    let response = page
        .goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");
    assert!(!response.frame_id.is_empty());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with `DomContentLoaded` wait state.
#[tokio::test]
async fn test_navigation_dom_content_loaded() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with DomContentLoaded wait
    let response = page
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with custom timeout.
#[tokio::test]
async fn test_navigation_with_timeout() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a generous timeout
    let response = page
        .goto("https://example.com")
        .timeout(Duration::from_secs(60))
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with referer.
#[tokio::test]
async fn test_navigation_with_referer() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a custom referer
    let response = page
        .goto("https://httpbin.org/headers")
        .referer("https://google.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert!(response.url.contains("httpbin.org"));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test multiple pages in the same context.
#[tokio::test]
async fn test_multiple_pages() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");

    // Create multiple pages
    let page1 = context.new_page().await.expect("Failed to create page 1");
    let page2 = context.new_page().await.expect("Failed to create page 2");

    // Verify they have different IDs
    assert_ne!(page1.target_id(), page2.target_id());
    assert_ne!(page1.session_id(), page2.session_id());

    // Navigate both pages
    let response1 = page1
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    let response2 = page2
        .goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    assert!(response1.url.contains("example.com"));
    assert!(response2.url.contains("httpbin.org"));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test multiple contexts in the same browser.
#[tokio::test]
async fn test_multiple_contexts() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create multiple contexts
    let context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");

    // Verify they have different IDs
    assert_ne!(context1.id(), context2.id());

    // Create pages in each context
    let page1 = context1.new_page().await.expect("Failed to create page 1");
    let page2 = context2.new_page().await.expect("Failed to create page 2");

    // Navigate both pages
    page1
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    page2
        .goto("https://example.org")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation error handling for invalid URL.
#[tokio::test]
async fn test_navigation_error_invalid_url() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Try to navigate to an invalid URL (non-existent domain)
    let result = page
        .goto("https://this-domain-definitely-does-not-exist-12345.com")
        .timeout(Duration::from_secs(10))
        .goto()
        .await;

    // Should fail with a network error
    assert!(result.is_err());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
