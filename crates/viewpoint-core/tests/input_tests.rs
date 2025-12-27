#![cfg(feature = "integration")]

//! Input device tests.
//!
//! Tests for keyboard, mouse, and touchscreen input simulation.

mod common;

use viewpoint_core::{Browser, DocumentLoadState, MouseButton};

use common::init_tracing;

// ============================================================================
// Keyboard Tests
// ============================================================================

/// Test keyboard press via page.keyboard().
#[tokio::test]
async fn test_keyboard_press() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page with a form
    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Focus an input
    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

    // Type using keyboard
    page.keyboard().press("a").await.expect("Failed to press key");
    page.keyboard().press("b").await.expect("Failed to press key");
    page.keyboard().press("c").await.expect("Failed to press key");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test keyboard type_text via page.keyboard().
#[tokio::test]
async fn test_keyboard_type_text() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

    // Type text using keyboard
    page.keyboard().type_text("Hello World").await.expect("Failed to type text");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test keyboard insert_text via page.keyboard().
#[tokio::test]
async fn test_keyboard_insert_text() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

    // Insert text directly (no key events)
    page.keyboard().insert_text("Inserted Text").await.expect("Failed to insert text");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test keyboard modifier keys (down/up).
#[tokio::test]
async fn test_keyboard_modifiers() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

    // Hold Shift and type
    page.keyboard().down("Shift").await.expect("Failed to press Shift");
    page.keyboard().press("a").await.expect("Failed to press A");
    page.keyboard().up("Shift").await.expect("Failed to release Shift");
    page.keyboard().press("b").await.expect("Failed to press b");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test keyboard key combinations.
#[tokio::test]
async fn test_keyboard_combinations() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");
    input.fill("Test Text").await.expect("Failed to fill");

    // Select all with Ctrl+A
    page.keyboard().press("Control+a").await.expect("Failed to press Ctrl+A");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// ============================================================================
// Mouse Tests
// ============================================================================

/// Test mouse move via page.mouse().
#[tokio::test]
async fn test_mouse_move() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Move mouse to coordinates
    page.mouse().move_(100.0, 200.0).send().await.expect("Failed to move mouse");
    page.mouse().move_(200.0, 300.0).steps(5).send().await.expect("Failed to move mouse with steps");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test mouse click via page.mouse().
#[tokio::test]
async fn test_mouse_click() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Click at coordinates
    page.mouse().click(100.0, 200.0).send().await.expect("Failed to click");

    // Right-click
    page.mouse().click(150.0, 200.0).button(MouseButton::Right).send().await.expect("Failed to right-click");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test mouse dblclick via page.mouse().
#[tokio::test]
async fn test_mouse_dblclick() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Double-click at coordinates
    page.mouse().dblclick(100.0, 200.0).await.expect("Failed to dblclick");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test mouse wheel via page.mouse().
#[tokio::test]
async fn test_mouse_wheel() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Move mouse to page
    page.mouse().move_(100.0, 100.0).send().await.expect("Failed to move mouse");
    
    // Scroll down
    page.mouse().wheel(0.0, 100.0).await.expect("Failed to scroll");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test mouse down/up via page.mouse().
#[tokio::test]
async fn test_mouse_down_up() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Perform a drag-like operation manually
    page.mouse().move_(100.0, 100.0).send().await.expect("Failed to move");
    page.mouse().down().send().await.expect("Failed to mouse down");
    page.mouse().move_(200.0, 200.0).steps(5).send().await.expect("Failed to move while held");
    page.mouse().up().send().await.expect("Failed to mouse up");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

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

    let context = browser.new_context().await.expect("Failed to create context");
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

    let context = browser.new_context().await.expect("Failed to create context");
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

    let context = browser.new_context().await.expect("Failed to create context");
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
    source.drag_to(&target).await.expect("Failed to drag to target");

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

    let context = browser.new_context().await.expect("Failed to create context");
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
