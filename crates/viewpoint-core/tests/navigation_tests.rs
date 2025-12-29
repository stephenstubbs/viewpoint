#![cfg(feature = "integration")]

//! Navigation tests.
//!
//! Tests for page navigation, history, reload, and basic navigation options.

mod common;

use std::time::Duration;
use viewpoint_core::{Browser, DocumentLoadState};
use viewpoint_js::js;

use common::init_tracing;

/// Test that navigation on a closed page returns an error.
#[tokio::test]
async fn test_navigation_on_closed_page() {
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

    // Close the page first
    page.close().await.expect("Failed to close page");
    assert!(page.is_closed());

    // Attempt navigation on closed page should fail
    let result = page.goto("https://example.com").goto().await;
    assert!(result.is_err(), "Navigation on closed page should fail");

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to example.com
    let response = page
        .goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate");

    // Browsers may normalize the URL by adding a trailing slash
    assert!(
        response.url() == "https://example.com" || response.url() == "https://example.com/",
        "Expected URL to be example.com, got: {}",
        response.url()
    );
    assert!(!response.frame_id().is_empty());

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with DomContentLoaded wait
    let response = page
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Browsers may normalize the URL by adding a trailing slash
    assert!(
        response.url() == "https://example.com" || response.url() == "https://example.com/",
        "Expected URL to be example.com, got: {}",
        response.url()
    );

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a generous timeout
    let response = page
        .goto("https://example.com")
        .timeout(Duration::from_secs(60))
        .goto()
        .await
        .expect("Failed to navigate");

    // Browsers may normalize the URL by adding a trailing slash
    assert!(
        response.url() == "https://example.com" || response.url() == "https://example.com/",
        "Expected URL to be example.com, got: {}",
        response.url()
    );

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a custom referer
    let response = page
        .goto("https://httpbin.org/headers")
        .referer("https://google.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert!(response.url().contains("httpbin.org"));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation history (back and forward).
#[tokio::test]
async fn test_navigation_back_forward() {
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
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to first page
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate to first page");

    let first_url = page.url().await.expect("Failed to get first URL");
    assert!(first_url.contains("example.com"));

    // Navigate to second page
    page.goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate to second page");

    let second_url = page.url().await.expect("Failed to get second URL");
    assert!(second_url.contains("httpbin.org"));

    // Go back
    let back_response = page.go_back().await.expect("Failed to go back");
    assert!(back_response.is_some(), "Go back should have a response");

    // Wait for navigation to complete
    tokio::time::sleep(Duration::from_millis(500)).await;
    let back_url = page.url().await.expect("Failed to get URL after back");
    assert!(
        back_url.contains("example.com"),
        "Should be back to first page"
    );

    // Go forward
    let forward_response = page.go_forward().await.expect("Failed to go forward");
    assert!(
        forward_response.is_some(),
        "Go forward should have a response"
    );

    // Wait for navigation to complete
    tokio::time::sleep(Duration::from_millis(500)).await;
    let forward_url = page.url().await.expect("Failed to get URL after forward");
    assert!(
        forward_url.contains("httpbin.org"),
        "Should be forward to second page"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test page reload functionality.
#[tokio::test]
async fn test_page_reload() {
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
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    let url_before = page.url().await.expect("Failed to get URL");

    // Set a marker in the page to verify reload clears it
    let _: serde_json::Value = page
        .evaluate(js! { window.testMarker = "set" })
        .await
        .expect("Failed to set marker");

    // Verify marker is set
    let marker_before: String = page
        .evaluate(js! { window.testMarker || "" })
        .await
        .expect("Failed to get marker before reload");
    assert_eq!(marker_before, "set", "Marker should be set");

    // Reload the page
    let _response = page.reload().await.expect("Failed to reload");

    // Wait for the page to fully reload
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify URL is the same
    let url_after = page.url().await.expect("Failed to get URL after reload");
    assert_eq!(url_before, url_after);

    // Verify the marker was cleared (page was reloaded)
    let marker_after: String = page
        .evaluate(js! { window.testMarker || "" })
        .await
        .expect("Failed to get marker after reload");
    assert_eq!(marker_after, "", "Marker should be cleared after reload");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test go_back with no history returns None.
#[tokio::test]
async fn test_go_back_no_history() {
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
    let page = context.new_page().await.expect("Failed to create page");

    // Try to go back without any navigation history
    let result = page.go_back().await.expect("go_back should not error");

    // Should return None when there's no history
    assert!(
        result.is_none(),
        "Go back with no history should return None"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test go_forward with no forward history returns None.
#[tokio::test]
async fn test_go_forward_no_history() {
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
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page first
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Try to go forward without any forward history
    let result = page
        .go_forward()
        .await
        .expect("go_forward should not error");

    // Should return None when there's no forward history
    assert!(
        result.is_none(),
        "Go forward with no forward history should return None"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
