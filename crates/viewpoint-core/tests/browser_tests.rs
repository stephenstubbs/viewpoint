#![cfg(feature = "integration")]

//! Browser and context lifecycle tests.
//!
//! Tests for browser launching, context creation, page management,
//! and remote browser connection.

mod common;

use std::time::Duration;
use viewpoint_core::Browser;

use common::init_tracing;

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
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
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

    let mut context = browser
        .new_context()
        .await
        .expect("Failed to create context");

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

// =============================================================================
// Remote Browser Connection Tests
// =============================================================================

/// Test getting browser contexts from a launched browser.
#[tokio::test]
async fn test_browser_contexts_launched() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create some contexts
    let _context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let _context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");

    // Get all contexts - should include the ones we created
    let contexts = browser.contexts().await.expect("Failed to get contexts");

    // Should have at least 3 contexts: default + 2 created
    // (The default context is always included)
    assert!(
        contexts.len() >= 3,
        "Expected at least 3 contexts, got {}",
        contexts.len()
    );

    // Verify one of them is the default context
    let has_default = contexts.iter().any(|c| c.is_default());
    assert!(has_default, "Should have a default context");

    // Verify we have owned and non-owned contexts
    let _owned_count = contexts.iter().filter(|c| c.is_owned()).count();
    let non_owned_count = contexts.iter().filter(|c| !c.is_owned()).count();

    // Default context should be non-owned (returned from contexts())
    assert!(
        non_owned_count >= 1,
        "Should have at least 1 non-owned context (default)"
    );
    // Note: contexts returned from browser.contexts() are all marked as non-owned
    // because they're discovered, not created

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that context ownership affects close behavior.
#[tokio::test]
async fn test_context_ownership_on_close() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create an owned context
    let mut owned_context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    assert!(
        owned_context.is_owned(),
        "Context created with new_context() should be owned"
    );

    // Get contexts - these will be marked as non-owned
    let contexts = browser.contexts().await.expect("Failed to get contexts");

    // Find the default context
    let default_context = contexts.into_iter().find(|c| c.is_default());
    assert!(default_context.is_some(), "Should find default context");

    let mut default_ctx = default_context.unwrap();
    assert!(
        !default_ctx.is_owned(),
        "Default context from contexts() should not be owned"
    );

    // Close the non-owned default context - should not error
    default_ctx
        .close()
        .await
        .expect("Closing non-owned context should succeed");

    // Close the owned context
    owned_context
        .close()
        .await
        .expect("Closing owned context should succeed");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test getting pages from the default context.
#[tokio::test]
async fn test_default_context_pages() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Get the default context
    let contexts = browser.contexts().await.expect("Failed to get contexts");
    let default_context = contexts.into_iter().find(|c| c.is_default());
    assert!(default_context.is_some(), "Should find default context");

    let default_ctx = default_context.unwrap();

    // Get pages in default context
    let pages = default_ctx.pages().await.expect("Failed to get pages");

    // Note: A launched browser might have one initial page in the default context
    // This depends on browser behavior
    tracing::info!("Default context has {} pages", pages.len());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test connect_over_cdp with invalid endpoint URL.
#[tokio::test]
async fn test_connect_over_cdp_invalid_url() {
    init_tracing();

    // Try to connect to an invalid URL scheme
    let result = Browser::connect_over_cdp("ftp://localhost:9222")
        .timeout(Duration::from_secs(5))
        .connect()
        .await;

    assert!(result.is_err(), "Should fail with invalid URL scheme");
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}

/// Test connect_over_cdp with unreachable endpoint.
#[tokio::test]
async fn test_connect_over_cdp_unreachable() {
    init_tracing();

    // Try to connect to an endpoint that doesn't exist
    // Using a high port that's unlikely to be in use
    let result = Browser::connect_over_cdp("http://127.0.0.1:59999")
        .timeout(Duration::from_secs(2))
        .connect()
        .await;

    assert!(result.is_err(), "Should fail with unreachable endpoint");
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}

/// Test connect_over_cdp with connection timeout.
#[tokio::test]
async fn test_connect_over_cdp_timeout() {
    init_tracing();

    // Try to connect with a very short timeout to a non-responsive endpoint
    // Use a black hole IP that won't respond
    let result = Browser::connect_over_cdp("http://10.255.255.1:9222")
        .timeout(Duration::from_millis(500))
        .connect()
        .await;

    // This should either timeout or fail to connect
    assert!(
        result.is_err(),
        "Should fail with timeout or connection error"
    );
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}
