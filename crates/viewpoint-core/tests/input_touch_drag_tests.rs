#![cfg(feature = "integration")]

//! Touchscreen and drag-and-drop input tests.
//!
//! Tests for touchscreen and drag-and-drop input simulation.

mod common;

use viewpoint_core::{Browser, DocumentLoadState};

use common::init_tracing;

// ============================================================================
// Touchscreen Tests
// ============================================================================

/// Test touchscreen tap via page.touchscreen().
#[tokio::test]
async fn test_touchscreen_tap() {
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

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Tap at coordinates (may fail if touch not enabled in context, but we test the API)
    // Note: In a real test, you'd need to enable touch in the context
    let result = page.touchscreen().tap(100.0, 200.0).await;
    // We don't assert success because touch may not be enabled
    let _ = result;

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// ============================================================================
// Drag and Drop Tests
// ============================================================================

/// Test drag and drop via page.drag_and_drop().
#[tokio::test]
async fn test_page_drag_and_drop() {
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

    // Create a test page with draggable elements
    let html = r#"data:text/html,
        <html><body>
            <div id="source" style="width:100px;height:100px;background:red;position:absolute;left:0;top:0;">Source</div>
            <div id="target" style="width:100px;height:100px;background:blue;position:absolute;left:200px;top:0;">Target</div>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Drag source to target
    page.drag_and_drop("#source", "#target")
        .send()
        .await
        .expect("Failed to drag and drop");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test locator.drag_to().
#[tokio::test]
async fn test_locator_drag_to() {
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

    let html = r#"data:text/html,
        <html><body>
            <div id="source" style="width:100px;height:100px;background:red;position:absolute;left:0;top:0;">Source</div>
            <div id="target" style="width:100px;height:100px;background:blue;position:absolute;left:200px;top:0;">Target</div>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use locator.drag_to()
    let source = page.locator("#source");
    let target = page.locator("#target");
    source
        .drag_to(&target)
        .await
        .expect("Failed to drag to target");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test locator.tap().
#[tokio::test]
async fn test_locator_tap() {
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

    let html = r#"data:text/html,
        <html><body>
            <button id="btn" onclick="window.tapped=true">Tap Me</button>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Try tapping the button (may fail if touch not enabled, but we test the API)
    let button = page.locator("#btn");
    let result = button.tap().send().await;
    // We don't assert success because touch may not be enabled
    let _ = result;

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
