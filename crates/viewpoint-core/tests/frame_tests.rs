#![cfg(feature = "integration")]

//! Frame execution context tests for viewpoint-core.
//!
//! These tests verify frame execution context targeting works correctly.

use std::sync::Once;
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
// Frame Execution Context Tests
// =============================================================================

/// Test that Frame.content() returns the correct HTML for an iframe.
///
/// This tests the frame execution context targeting - ensuring that JavaScript
/// evaluation happens in the frame's context, not the main frame's.
#[tokio::test]
async fn test_iframe_content_execution_context() {
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

    // Create a page with an iframe that has distinct content
    page.set_content(
        r##"
        <html><body>
            <h1>Main Frame Content</h1>
            <iframe name="test-frame" srcdoc="<html><head><title>Iframe Title</title></head><body><h1>Iframe Heading</h1><p>Iframe paragraph content.</p></body></html>"></iframe>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Get the iframe by name
    let iframe = page
        .frame("test-frame")
        .await
        .expect("Failed to get frames")
        .expect("Should find test-frame");

    // Get the iframe's content - should be the iframe content, not main frame
    let content = iframe.content().await.expect("Failed to get iframe content");

    // Verify the content is from the iframe, not the main frame
    assert!(
        content.contains("Iframe Heading"),
        "Content should contain iframe heading, got: {}",
        content
    );
    assert!(
        content.contains("Iframe paragraph content"),
        "Content should contain iframe paragraph, got: {}",
        content
    );
    assert!(
        !content.contains("Main Frame Content"),
        "Content should NOT contain main frame content, got: {}",
        content
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that Frame.title() returns the correct title for an iframe.
#[tokio::test]
async fn test_iframe_title_execution_context() {
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

    // Create a page with an iframe that has a distinct title
    page.set_content(
        r##"
        <html>
        <head><title>Main Page Title</title></head>
        <body>
            <iframe name="titled-frame" srcdoc="<html><head><title>Iframe Document Title</title></head><body>Content</body></html>"></iframe>
        </body>
        </html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Get the iframe by name
    let iframe = page
        .frame("titled-frame")
        .await
        .expect("Failed to get frames")
        .expect("Should find titled-frame");

    // Get the iframe's title
    let title = iframe.title().await.expect("Failed to get iframe title");

    // The title should be the iframe's title, not the main page's
    assert_eq!(
        title, "Iframe Document Title",
        "Title should be the iframe's title"
    );

    // Also verify main frame has the expected title
    let main_frame = page.main_frame().await.expect("Failed to get main frame");
    let main_title = main_frame
        .title()
        .await
        .expect("Failed to get main frame title");
    assert_eq!(
        main_title, "Main Page Title",
        "Main frame title should be different"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that Frame.aria_snapshot() returns the correct accessibility tree for an iframe.
#[tokio::test]
async fn test_iframe_aria_snapshot_execution_context() {
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

    // Create a page with an iframe containing distinct accessible elements
    page.set_content(
        r##"
        <html><body>
            <button id="main-button">Main Button</button>
            <iframe name="aria-frame" srcdoc="<html><body><button id='iframe-button'>Iframe Button</button><input type='text' placeholder='Iframe Input' /></body></html>"></iframe>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Get the iframe by name
    let iframe = page
        .frame("aria-frame")
        .await
        .expect("Failed to get frames")
        .expect("Should find aria-frame");

    // Get the iframe's aria snapshot
    let snapshot = iframe
        .aria_snapshot()
        .await
        .expect("Failed to get iframe aria snapshot");
    let yaml = snapshot.to_yaml();
    println!("Iframe aria snapshot:\n{}", yaml);

    // Verify the snapshot contains iframe elements
    assert!(
        yaml.contains("Iframe Button") || yaml.contains("button"),
        "Snapshot should contain iframe button, got: {}",
        yaml
    );

    // Verify the snapshot does NOT contain main frame elements
    assert!(
        !yaml.contains("Main Button"),
        "Snapshot should NOT contain main frame button, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
