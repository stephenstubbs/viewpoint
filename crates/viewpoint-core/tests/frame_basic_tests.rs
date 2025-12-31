#![cfg(feature = "integration")]

//! Basic frame handling tests for viewpoint-core.
//!
//! These tests verify frame access, frame properties, and frame lists.

use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use viewpoint_core::Browser;

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
// Main Frame Tests
// =============================================================================

/// Test getting the main frame.
#[tokio::test]
async fn test_main_frame() {
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
    page.goto_url("https://example.com")
        .await
        .expect("Failed to navigate");

    // Get main frame
    let main_frame = page.main_frame().await.expect("Failed to get main frame");

    // Main frame should have a URL
    let url = main_frame.url();
    assert!(
        url.contains("example.com"),
        "Main frame URL should contain example.com, got: {}",
        url
    );

    // Main frame should be the main frame
    assert!(main_frame.is_main(), "Should be main frame");
    assert!(
        main_frame.parent_id().is_none(),
        "Main frame should have no parent"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test main frame access with simple page.
#[tokio::test]
async fn test_main_frame_access() {
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

    page.set_content(r#"<html><body><h1>Main Frame</h1></body></html>"#)
        .set()
        .await
        .expect("Failed to set content");

    // Get main frame
    let main_frame = page.main_frame().await.expect("Failed to get main frame");

    // Verify main frame properties - main frame name is typically empty
    let _ = main_frame.name(); // Just verify we can access it

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame List Tests
// =============================================================================

/// Test getting all frames in a page.
#[tokio::test]
async fn test_frames_list() {
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

    // Navigate to a page (example.com doesn't have iframes, so we'll just test the main frame)
    page.goto_url("https://example.com")
        .await
        .expect("Failed to navigate");

    // Get all frames
    let frames = page.frames().await.expect("Failed to get frames");

    // Should have at least the main frame
    assert!(!frames.is_empty(), "Should have at least one frame");

    // First frame should be the main frame
    assert!(frames[0].is_main(), "First frame should be main frame");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test listing all frames with iframes.
#[tokio::test]
async fn test_list_frames() {
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

    page.set_content(
        r#"
        <html><body>
            <iframe name="frame1" srcdoc="<html><body>Frame 1</body></html>"></iframe>
            <iframe name="frame2" srcdoc="<html><body>Frame 2</body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframes to load
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Get all frames
    let frames = page.frames().await.expect("Failed to get frames");

    // Should have main frame + 2 iframes = 3 frames
    assert!(
        frames.len() >= 3,
        "Should have at least 3 frames (main + 2 iframes), got {}",
        frames.len()
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame Properties Tests
// =============================================================================

/// Test frame properties.
#[tokio::test]
async fn test_frame_properties() {
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
    page.goto_url("https://example.com")
        .await
        .expect("Failed to navigate");

    // Get main frame
    let main_frame = page.main_frame().await.expect("Failed to get main frame");

    // Test URL
    let url = main_frame.url();
    assert!(
        url.contains("example.com"),
        "URL should contain example.com"
    );

    // Test content
    let content = main_frame.content().await.expect("Failed to get content");
    assert!(content.contains("<html"), "Content should contain HTML");
    assert!(
        content.contains("Example Domain"),
        "Content should contain page text"
    );

    // Test title
    let title = main_frame.title().await.expect("Failed to get title");
    assert!(!title.is_empty(), "Title should not be empty");

    // Test is_detached
    assert!(
        !main_frame.is_detached(),
        "Main frame should not be detached"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame Locator Tests
// =============================================================================

/// Test frame_locator basic functionality.
#[tokio::test]
async fn test_frame_locator_creation() {
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
    page.goto_url("https://example.com")
        .await
        .expect("Failed to navigate");

    // Create a frame locator - this should not fail even if the frame doesn't exist
    let frame_locator = page.frame_locator("#non-existent-frame");

    // Verify the frame locator was created
    assert_eq!(frame_locator.selector(), "#non-existent-frame");

    // Frame locator should reference the page
    assert!(!frame_locator.page().is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test nested frame locator creation.
#[tokio::test]
async fn test_nested_frame_locator() {
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
    page.goto_url("https://example.com")
        .await
        .expect("Failed to navigate");

    // Create nested frame locators
    let outer = page.frame_locator("#outer-frame");
    let inner = outer.frame_locator("#inner-frame");

    // Verify the nested structure
    assert_eq!(inner.selector(), "#inner-frame");
    assert_eq!(inner.parent_selectors().len(), 1);
    assert_eq!(inner.parent_selectors()[0], "#outer-frame");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test frame locator with iframes.
#[tokio::test]
async fn test_frame_locator() {
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

    page.set_content(r#"
        <html><body>
            <iframe id="myframe" srcdoc="<html><body><button id='btn'>Click me</button></body></html>"></iframe>
        </body></html>
    "#)
        .set()
        .await
        .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Use frame locator to access content inside iframe
    let frame_locator = page.frame_locator("#myframe");
    let button = frame_locator.locator("#btn");

    // Verify we can access the button
    let text = button.text_content().await.expect("Failed to get text");
    assert_eq!(text, Some("Click me".to_string()));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame Event Tests
// =============================================================================

/// Test frame attached event.
#[tokio::test]
async fn test_frame_attached_event() {
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

    // Track if frame attached handler was called
    let handler_called = Arc::new(AtomicBool::new(false));
    let handler_called_clone = handler_called.clone();

    // Set up frame attached handler
    page.on_frameattached(move |_frame| {
        let called = handler_called_clone.clone();
        async move {
            called.store(true, Ordering::SeqCst);
        }
    })
    .await;

    // Set up page with an iframe that will be added dynamically
    page.set_content(
        r#"
        <html><body>
            <div id="container"></div>
            <script>
                setTimeout(() => {
                    const iframe = document.createElement('iframe');
                    iframe.src = 'about:blank';
                    document.getElementById('container').appendChild(iframe);
                }, 100);
            </script>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to be attached
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Handler should have been called
    assert!(
        handler_called.load(Ordering::SeqCst),
        "Frame attached handler should have been called"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test frame detached event.
#[tokio::test]
async fn test_frame_detached_event() {
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

    // Track if frame detached handler was called
    let handler_called = Arc::new(AtomicBool::new(false));
    let handler_called_clone = handler_called.clone();

    // Set up frame detached handler
    page.on_framedetached(move |_frame| {
        let called = handler_called_clone.clone();
        async move {
            called.store(true, Ordering::SeqCst);
        }
    })
    .await;

    // Set up page with an iframe that will be removed
    page.set_content(
        r#"
        <html><body>
            <iframe id="myframe" src="about:blank"></iframe>
            <script>
                setTimeout(() => {
                    document.getElementById('myframe').remove();
                }, 200);
            </script>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to be detached
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Handler should have been called
    assert!(
        handler_called.load(Ordering::SeqCst),
        "Frame detached handler should have been called"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
