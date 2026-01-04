#![cfg(feature = "integration")]

//! Context storage state tests.
//!
//! Tests for browser context storage state management including
//! saving/restoring state and handling page lifecycle.

mod common;

use viewpoint_core::{Cookie, DocumentLoadState};

/// Test storage state.
#[tokio::test]
async fn test_storage_state() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;

    // Navigate to add some state
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get storage state
    let state = context
        .storage_state()
        .await
        .expect("Failed to get storage state");

    // Should have a structure (cookies and origins)
    let _ = state.cookies;
    let _ = state.origins;

    browser.close().await.expect("Failed to close browser");
}

/// Test that storage_state() succeeds after closing a page.
/// This verifies that the internal pages tracking list is properly maintained
/// when pages are closed, preventing stale session errors.
#[tokio::test]
async fn test_storage_state_succeeds_after_page_close() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create a page
    let mut page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Close the page
    page.close().await.expect("Failed to close page");

    // Call storage_state() - this should succeed without "session not found" error
    // because the closed page was removed from the internal tracking list
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed after closing page");

    // Verify we got a valid state
    let _ = state.cookies;
    let _ = state.origins;

    browser.close().await.expect("Failed to close browser");
}

/// Test that storage_state() collects cookies even when no pages remain open.
#[tokio::test]
async fn test_storage_state_cookies_after_all_pages_closed() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create a page and navigate to set cookies
    let mut page = context.new_page().await.expect("Failed to create page");
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Add a cookie
    context
        .add_cookies(vec![
            Cookie::new("test_cookie", "test_value").domain("example.com"),
        ])
        .await
        .expect("Failed to add cookie");

    // Close the page
    page.close().await.expect("Failed to close page");

    // Call storage_state() - should succeed and include cookies
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed after closing page");

    // Verify we got cookies (cookies are fetched from browser, not from pages)
    assert!(!state.cookies.is_empty(), "Should have cookies");

    browser.close().await.expect("Failed to close browser");
}

/// Test that storage_state() works correctly with multiple pages where some are closed.
/// This verifies the fix for stale session issues where closed pages would cause
/// "session not found" errors when iterating over the internal pages list.
#[tokio::test]
async fn test_storage_state_with_multiple_pages_some_closed() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create first page and navigate
    let mut page1 = context.new_page().await.expect("Failed to create page 1");
    page1
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    // Create second page and navigate
    let page2 = context.new_page().await.expect("Failed to create page 2");
    page2
        .goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    // Close first page - this should remove it from the internal tracking list
    page1.close().await.expect("Failed to close page 1");

    // Call storage_state() - should succeed without "session not found" error
    // because the closed page was removed from the internal tracking list
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed with remaining pages");

    // Verify we got a valid state
    let _ = state.cookies;
    let _ = state.origins;

    browser.close().await.expect("Failed to close browser");
}
