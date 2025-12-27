#![cfg(feature = "integration")]

//! Browser and context lifecycle tests.
//!
//! Tests for browser launching, context creation, and page management.

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
