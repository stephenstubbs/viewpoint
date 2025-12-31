#![cfg(feature = "integration")]

//! Network routing and interception tests for viewpoint-core.
//!
//! These tests verify route handlers, request/response interception,
//! and context-level routing.

use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Set up route to abort image requests
    context
        .route("**/*.png", |route| {
            Box::pin(async move {
                route.abort().await?;
                Ok(())
            })
        })
        .await
        .expect("Failed to set route");

    // Set up page with an image
    page.set_content(
        r#"
        <html><body>
            <img id="img" src="https://example.com/image.png" onerror="window.imageError = true">
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait a bit for the image error to fire
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check if image error occurred
    let error: bool = page
        .evaluate(js! { window.imageError || false })
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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Set up route AFTER page creation - should still work due to route change notifications
    context
        .route("**/api/data", |route| {
            Box::pin(async move {
                route
                    .fulfill()
                    .status(200)
                    .content_type("application/json")
                    .body(r#"{"mocked": true, "value": 42}"#)
                    .send()
                    .await?;
                Ok(())
            })
        })
        .await
        .expect("Failed to set route");

    // Navigate to a real page first so relative fetch works
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fetch the mocked endpoint
    let result: serde_json::Value = page
        .evaluate(js! {
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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Set up route to add custom header
    context
        .route("**/*", |route| {
            Box::pin(async move {
                route
                    .continue_()
                    .header("X-Custom-Header", "test-value")
                    .await?;
                Ok(())
            })
        })
        .await
        .expect("Failed to set route");

    // Navigate to httpbin which echoes headers
    let response = page
        .goto("https://httpbin.org/headers")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.status(), Some(200));

    // The custom header should have been sent (we can verify via page content)
    let content = page.content().await.expect("Failed to get content");
    assert!(
        content.contains("X-Custom-Header") || content.contains("x-custom-header"),
        "Custom header should be echoed back"
    );

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
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
            assert!(
                response.status >= 200 && response.status < 300,
                "Should get OK status"
            );

            // Fulfill with the fetched response
            response.fulfill().await
        }
    })
    .await
    .expect("Failed to set route");

    // Navigate to trigger the route
    page.goto("https://httpbin.org/get")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Route should have been called
    assert!(
        route_called.load(Ordering::SeqCst),
        "Route should have been called"
    );

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

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    let route_called = Arc::new(AtomicBool::new(false));
    let route_called_clone = route_called.clone();

    // Route that adds a custom header before fetching
    page.route("**/headers", move |route| {
        let called = route_called_clone.clone();
        async move {
            called.store(true, Ordering::SeqCst);

            // Fetch with additional header
            let response = route
                .fetch()
                .header("X-Custom-Header", "test-value")
                .await?;

            // Fulfill with the response
            response.fulfill().await
        }
    })
    .await
    .expect("Failed to set route");

    // Navigate to httpbin/headers which echoes headers back
    page.goto("https://httpbin.org/headers")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert!(
        route_called.load(Ordering::SeqCst),
        "Route should have been called"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}


