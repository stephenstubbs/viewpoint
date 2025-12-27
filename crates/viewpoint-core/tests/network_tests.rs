#![cfg(feature = "integration")]

//! Network routing and interception tests for viewpoint-core.
//!
//! These tests verify route handlers, request/response interception,
//! and context-level routing.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::sync::Once;
use std::time::Duration;

use viewpoint_core::{Browser, DocumentLoadState};
use viewpoint_js::js;

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
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
// Route Abort Tests
// =============================================================================

/// Test aborting requests.
#[tokio::test]
async fn test_route_abort() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Set up route to abort image requests
    context.route("**/*.png", |route| {
        Box::pin(async move {
            route.abort().await?;
            Ok(())
        })
    }).await.expect("Failed to set route");
    
    // Set up page with an image
    page.set_content(r#"
        <html><body>
            <img id="img" src="https://example.com/image.png" onerror="window.imageError = true">
        </body></html>
    "#)
        .set()
        .await
        .expect("Failed to set content");
    
    // Wait a bit for the image error to fire
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Check if image error occurred
    let error: bool = page.evaluate(js!{ window.imageError || false })
        .await
        .expect("Failed to check error");
    assert!(error, "Image request should have been aborted");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Route Fulfill Tests
// =============================================================================

/// Test fulfilling a request with custom response.
#[tokio::test]
async fn test_route_fulfill_custom() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Set up route AFTER page creation - should still work due to route change notifications
    context.route("**/api/data", |route| {
        Box::pin(async move {
            route.fulfill()
                .status(200)
                .content_type("application/json")
                .body(r#"{"mocked": true, "value": 42}"#)
                .send()
                .await?;
            Ok(())
        })
    }).await.expect("Failed to set route");
    
    // Navigate to a real page first so relative fetch works
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");
    
    // Fetch the mocked endpoint
    let result: serde_json::Value = page.evaluate(js!{
        fetch("/api/data").then(r => r.json())
    })
        .await
        .expect("Failed to fetch");
    
    assert_eq!(result["mocked"], true);
    assert_eq!(result["value"], 42);
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Route Continue Tests
// =============================================================================

/// Test continuing a request with modified headers.
#[tokio::test]
async fn test_route_continue_with_headers() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Set up route to add custom header
    context.route("**/*", |route| {
        Box::pin(async move {
            route.continue_()
                .header("X-Custom-Header", "test-value")
                .await?;
            Ok(())
        })
    }).await.expect("Failed to set route");
    
    // Navigate to httpbin which echoes headers
    let response = page.goto("https://httpbin.org/headers")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");
    
    assert_eq!(response.status(), Some(200));
    
    // The custom header should have been sent (we can verify via page content)
    let content = page.content().await.expect("Failed to get content");
    assert!(content.contains("X-Custom-Header") || content.contains("x-custom-header"), 
            "Custom header should be echoed back");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Route Fetch Tests
// =============================================================================

/// Test route.fetch() basic usage.
#[tokio::test]
async fn test_route_fetch_basic() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Track if route was called
    let route_called = Arc::new(AtomicBool::new(false));
    let route_called_clone = route_called.clone();
    
    // Set up a route that fetches and modifies response
    page.route("**/get", move |route| {
        let called = route_called_clone.clone();
        async move {
            called.store(true, Ordering::SeqCst);
            
            // Fetch the actual response
            let response = route.fetch().await?;
            
            // Check we got a valid response
            assert!(response.status >= 200 && response.status < 300, "Should get OK status");
            
            // Fulfill with the fetched response
            response.fulfill().await
        }
    }).await.expect("Failed to set route");
    
    // Navigate to trigger the route
    page.goto("https://httpbin.org/get")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");
    
    // Route should have been called
    assert!(route_called.load(Ordering::SeqCst), "Route should have been called");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test route.fetch() with header modification.
#[tokio::test]
async fn test_route_fetch_with_headers() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    let route_called = Arc::new(AtomicBool::new(false));
    let route_called_clone = route_called.clone();
    
    // Route that adds a custom header before fetching
    page.route("**/headers", move |route| {
        let called = route_called_clone.clone();
        async move {
            called.store(true, Ordering::SeqCst);
            
            // Fetch with additional header
            let response = route.fetch()
                .header("X-Custom-Header", "test-value")
                .await?;
            
            // Fulfill with the response
            response.fulfill().await
        }
    }).await.expect("Failed to set route");
    
    // Navigate to httpbin/headers which echoes headers back
    page.goto("https://httpbin.org/headers")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");
    
    assert!(route_called.load(Ordering::SeqCst), "Route should have been called");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Context-Level Route Tests
// =============================================================================

/// Test context routes are applied to new pages.
#[tokio::test]
async fn test_context_route_propagation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    let context = browser.new_context().await.expect("Failed to create context");
    
    // Track how many times the route handler is called
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    
    // Set up a context-level route
    context.route("**/*.png", move |route| {
        let count = call_count_clone.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            route.abort().await
        }
    }).await.expect("Failed to set context route");
    
    // Create first page
    let page1 = context.new_page().await.expect("Failed to create page 1");
    
    // Set up page that tries to load an image using absolute URL
    page1.set_content(r#"
        <html><body>
            <img src="https://example.com/test.png" onerror="window.imgError = true">
        </body></html>
    "#)
        .set()
        .await
        .expect("Failed to set content");
    
    // Wait for image request to be intercepted
    tokio::time::sleep(Duration::from_millis(300)).await;
    
    // Route should have been called at least once
    assert!(call_count.load(Ordering::SeqCst) >= 1, "Context route should be applied to first page");
    
    // Create second page - routes should also apply
    let page2 = context.new_page().await.expect("Failed to create page 2");
    let count_before = call_count.load(Ordering::SeqCst);
    
    page2.set_content(r#"
        <html><body>
            <img src="https://example.com/another.png" onerror="window.imgError = true">
        </body></html>
    "#)
        .set()
        .await
        .expect("Failed to set content on page 2");
    
    tokio::time::sleep(Duration::from_millis(300)).await;
    
    // Route should have been called again for second page
    assert!(call_count.load(Ordering::SeqCst) > count_before, "Context route should be applied to second page");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test context-level route applies to all pages.
#[tokio::test]
async fn test_context_route_multiple_pages() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    
    // Set up route at context level
    context.route("**/api/**", |route| {
        Box::pin(async move {
            route.fulfill()
                .status(200)
                .body("context intercepted")
                .send()
                .await?;
            Ok(())
        })
    }).await.expect("Failed to set up context route");
    
    // Create two pages
    let page1 = context.new_page().await.expect("Failed to create page1");
    let page2 = context.new_page().await.expect("Failed to create page2");
    
    // Navigate to a real page first so relative fetch works
    page1.goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate page1");
    
    page2.goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate page2");
    
    // Test on page1 - use evaluate to fetch
    let result1: String = page1.evaluate(js!{
        fetch("/api/test").then(r => r.text())
    }).await.expect("Failed to fetch on page1");
    assert_eq!(result1, "context intercepted", "Context route should work on page1");
    
    // Test on page2 - use evaluate to fetch
    let result2: String = page2.evaluate(js!{
        fetch("/api/test").then(r => r.text())
    }).await.expect("Failed to fetch on page2");
    assert_eq!(result2, "context intercepted", "Context route should work on page2");
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Wait for Request/Response Tests
// =============================================================================

/// Test wait_for_request API.
#[tokio::test]
async fn test_wait_for_request() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Navigate to a real site to ensure request events work
    page.goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate");
    
    // Just verify the wait_for_request API exists and can be called
    let _waiter = page.wait_for_request("**/api/**".to_string());
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_response API.
#[tokio::test]
async fn test_wait_for_response() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    
    // Navigate to a real site
    page.goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate");
    
    // Just verify the wait_for_response API exists and can be called
    let _waiter = page.wait_for_response("**/*".to_string());
    
    // Clean up
    browser.close().await.expect("Failed to close browser");
}
