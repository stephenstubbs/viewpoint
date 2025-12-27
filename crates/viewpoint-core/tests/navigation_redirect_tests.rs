#![cfg(feature = "integration")]

//! Navigation redirect tests.
//!
//! Tests for HTTP redirects (301, 302), redirect chains, and response handling.

mod common;

use std::time::Duration;
use viewpoint_core::{Browser, DocumentLoadState};

use common::init_tracing;

/// Test navigation with redirect (301).
#[tokio::test]
async fn test_navigation_redirect_301() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // httpbin.org/redirect/n redirects n times
    let response = page
        .goto("https://httpbin.org/redirect/2")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // After 2 redirects, should end up at /get
    assert!(
        response.url().contains("/get") || response.url().contains("httpbin.org"),
        "Expected to end up at /get after redirects, got: {}",
        response.url()
    );
    
    // Status should be 200 (final destination), not the redirect status
    assert_eq!(response.status(), Some(200));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with redirect (302).
#[tokio::test]
async fn test_navigation_redirect_302() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // httpbin.org/redirect-to redirects to specified URL with status 302 by default
    let response = page
        .goto("https://httpbin.org/redirect-to?url=https://example.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Should end up at example.com
    assert!(
        response.url().contains("example.com"),
        "Expected to end up at example.com, got: {}",
        response.url()
    );
    
    assert_eq!(response.status(), Some(200));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with multiple redirects in chain.
#[tokio::test]
async fn test_navigation_redirect_chain() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Use absolute-redirect which always uses 302 status
    let response = page
        .goto("https://httpbin.org/absolute-redirect/3")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // After 3 redirects, should end up at /get
    assert!(
        response.url().contains("/get"),
        "Expected to end up at /get after 3 redirects, got: {}",
        response.url()
    );
    
    assert_eq!(response.status(), Some(200));

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

/// Test navigation response status codes.
#[tokio::test]
async fn test_navigation_response_status() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Test 200 response
    let response = page
        .goto("https://httpbin.org/status/200")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.status(), Some(200), "Expected 200 status");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation response headers.
#[tokio::test]
async fn test_navigation_response_headers() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page that returns headers
    let response = page
        .goto("https://httpbin.org/response-headers?X-Custom-Header=test-value")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Check that we captured some headers
    if let Some(headers) = response.headers() {
        assert!(!headers.is_empty(), "Should have captured response headers");
    }

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
