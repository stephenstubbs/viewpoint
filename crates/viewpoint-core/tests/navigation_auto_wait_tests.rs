#![cfg(feature = "integration")]

//! Tests for navigation auto-wait after actions.
//!
//! These tests verify that actions like click, press, fill, etc. automatically
//! wait for any triggered navigation to complete.

mod common;

use common::init_tracing;
use viewpoint_core::{Browser, DocumentLoadState};

/// Test that click auto-waits for navigation.
#[tokio::test]
async fn test_click_auto_waits_for_navigation() {
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

    // Create a page with a link that navigates
    let html = r#"data:text/html,
        <html><body>
            <a id="nav-link" href="https://example.com">Go to Example</a>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Click the link - this should auto-wait for navigation
    page.locator("#nav-link")
        .click()
        .await
        .expect("Failed to click link");

    // After click completes, we should be on example.com
    let url = page.url().await.expect("Failed to get URL");
    assert!(
        url.contains("example.com"),
        "Expected to navigate to example.com, got: {}",
        url
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that click with no_wait_after returns immediately.
#[tokio::test]
async fn test_click_no_wait_after() {
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

    // Create a page with a link that navigates
    let html = r#"data:text/html,
        <html><body>
            <a id="nav-link" href="https://example.com">Go to Example</a>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Click the link with no_wait_after - should return immediately
    page.locator("#nav-link")
        .click()
        .no_wait_after(true)
        .await
        .expect("Failed to click link");

    // The test passes if we get here - the navigation may or may not have completed
    // We're just testing that no_wait_after doesn't block

    browser.close().await.expect("Failed to close browser");
}

/// Test that click on non-navigating element returns quickly.
#[tokio::test]
async fn test_click_non_navigating_returns_quickly() {
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

    // Create a page with a button that doesn't navigate
    let html = r#"data:text/html,
        <html><body>
            <button id="btn" onclick="window.clicked=true">Click Me</button>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Click the button - should return quickly since no navigation happens
    let start = std::time::Instant::now();
    page.locator("#btn")
        .click()
        .await
        .expect("Failed to click button");
    let elapsed = start.elapsed();

    // Should return within a reasonable time (detection window + small buffer)
    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "Click took too long: {:?}",
        elapsed
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that press Enter auto-waits for form submission navigation.
#[tokio::test]
async fn test_press_enter_auto_waits_for_navigation() {
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

    // Create a simple form that submits on Enter
    let html = r#"data:text/html,
        <html><body>
            <form action="https://httpbin.org/get" method="get">
                <input id="input" type="text" name="q" />
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Focus and type in the input
    page.locator("#input")
        .fill("test")
        .await
        .expect("Failed to fill");

    // Press Enter - should auto-wait for form submission navigation
    page.locator("#input")
        .press("Enter")
        .await
        .expect("Failed to press Enter");

    // After press completes, we should be on httpbin
    let url = page.url().await.expect("Failed to get URL");
    assert!(
        url.contains("httpbin.org"),
        "Expected to navigate to httpbin.org, got: {}",
        url
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that keyboard press with no_wait_after returns immediately.
#[tokio::test]
async fn test_keyboard_press_no_wait_after() {
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
            <form action="https://httpbin.org/get" method="get">
                <input id="input" type="text" name="q" />
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    page.locator("#input")
        .fill("test")
        .await
        .expect("Failed to fill");

    // Press Enter with no_wait_after - should return immediately
    page.keyboard()
        .press("Enter")
        .no_wait_after(true)
        .await
        .expect("Failed to press Enter");

    // Test passes if we get here without blocking

    browser.close().await.expect("Failed to close browser");
}

/// Test that dblclick auto-waits for navigation.
#[tokio::test]
async fn test_dblclick_auto_waits_for_navigation() {
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

    // Create a page where double-click triggers navigation
    let html = r#"data:text/html,
        <html><body>
            <div id="dbl" ondblclick="window.location='https://example.com'">Double-click me</div>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Double-click - should auto-wait for navigation
    page.locator("#dbl")
        .dblclick()
        .await
        .expect("Failed to dblclick");

    // After dblclick completes, we should be on example.com
    let url = page.url().await.expect("Failed to get URL");
    assert!(
        url.contains("example.com"),
        "Expected to navigate to example.com, got: {}",
        url
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that select_option auto-waits for navigation.
#[tokio::test]
async fn test_select_option_auto_waits_for_navigation() {
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

    // Create a page where select triggers navigation
    let html = r#"data:text/html,
        <html><body>
            <select id="nav-select" onchange="if(this.value) window.location=this.value">
                <option value="">Choose...</option>
                <option value="https://example.com">Go to Example</option>
            </select>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Select option - should auto-wait for navigation
    page.locator("#nav-select")
        .select_option()
        .value("https://example.com")
        .await
        .expect("Failed to select option");

    // After select completes, we should be on example.com
    let url = page.url().await.expect("Failed to get URL");
    assert!(
        url.contains("example.com"),
        "Expected to navigate to example.com, got: {}",
        url
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that check auto-waits for navigation.
#[tokio::test]
async fn test_check_auto_waits_for_navigation() {
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

    // Create a page where checkbox triggers navigation
    let html = r#"data:text/html,
        <html><body>
            <input id="nav-checkbox" type="checkbox" onchange="if(this.checked) window.location='https://example.com'" />
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Check - should auto-wait for navigation
    page.locator("#nav-checkbox")
        .check()
        .await
        .expect("Failed to check");

    // After check completes, we should be on example.com
    let url = page.url().await.expect("Failed to get URL");
    assert!(
        url.contains("example.com"),
        "Expected to navigate to example.com, got: {}",
        url
    );

    browser.close().await.expect("Failed to close browser");
}
