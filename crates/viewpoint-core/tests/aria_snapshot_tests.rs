#![cfg(feature = "integration")]

//! ARIA snapshot tests for viewpoint-core.
//!
//! These tests verify ARIA accessibility snapshot capture including frame
//! boundary detection and multi-frame snapshot stitching.

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
// Basic Aria Snapshot Tests
// =============================================================================

/// Test basic aria snapshot capture from a simple page.
#[tokio::test]
async fn test_aria_snapshot_simple_page() {
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
            <h1>Main Title</h1>
            <button>Click me</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Convert to YAML for inspection
    let yaml = snapshot.to_yaml();
    println!("Snapshot YAML:\n{}", yaml);

    // Verify basic structure
    assert!(
        yaml.contains("heading") || yaml.contains("button"),
        "Snapshot should contain heading or button, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame Boundary Detection Tests
// =============================================================================

/// Test that iframes are detected as frame boundaries.
#[tokio::test]
async fn test_aria_snapshot_iframe_boundary_detection() {
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
            <h1>Main Page</h1>
            <iframe name="myframe" title="Widget Frame" src="about:blank"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot with iframe:\n{}", yaml);

    // Verify iframe is marked as frame boundary
    assert!(
        yaml.contains("iframe") || yaml.contains("[frame-boundary]"),
        "Snapshot should contain iframe or frame-boundary marker, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that iframe_refs are collected.
#[tokio::test]
async fn test_aria_snapshot_iframe_refs_collection() {
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
            <h1>Multi-Frame Page</h1>
            <iframe name="frame1" src="about:blank"></iframe>
            <iframe name="frame2" src="about:blank"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframes
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Check that iframe_refs were collected
    println!("Collected iframe_refs: {:?}", snapshot.iframe_refs);

    // There should be 2 iframes detected
    assert!(
        snapshot.iframe_refs.len() >= 2,
        "Should have at least 2 iframe refs, got: {}",
        snapshot.iframe_refs.len()
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Frame Snapshot Tests
// =============================================================================

/// Test capturing snapshot from a specific frame.
#[tokio::test]
async fn test_frame_aria_snapshot() {
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
            <h1>Main Page</h1>
            <iframe name="contentframe" srcdoc="<html><body><h2>Frame Content</h2><button>Frame Button</button></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Get the iframe's frame
    let frames = page.frames().await.expect("Failed to get frames");
    assert!(
        frames.len() >= 2,
        "Should have at least 2 frames (main + iframe)"
    );

    // Find the content frame
    let content_frame = frames
        .iter()
        .find(|f| f.name() == "contentframe")
        .expect("Should find content frame");

    // Capture snapshot from the frame
    let frame_snapshot = content_frame
        .aria_snapshot()
        .await
        .expect("Failed to get frame snapshot");

    let yaml = frame_snapshot.to_yaml();
    println!("Frame snapshot:\n{}", yaml);

    // Verify frame content is captured
    assert!(
        yaml.contains("button") || yaml.contains("heading"),
        "Frame snapshot should contain button or heading, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Multi-Frame Snapshot Tests
// =============================================================================

/// Test capturing snapshot with all frames stitched together.
#[tokio::test]
async fn test_aria_snapshot_with_frames() {
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
            <h1>Main Page Title</h1>
            <iframe name="widget" srcdoc="<html><body><button>Widget Button</button></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Capture snapshot with frames stitched in
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Multi-frame snapshot:\n{}", yaml);

    // The snapshot should contain both main page and frame content
    // (though exact format depends on stitching)
    assert!(
        yaml.contains("heading") || yaml.contains("button"),
        "Multi-frame snapshot should contain page elements, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test nested same-origin frames.
#[tokio::test]
async fn test_aria_snapshot_nested_frames() {
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

    // Create a page with nested iframes using srcdoc
    page.set_content(
        r#"
        <html><body>
            <h1>Level 0</h1>
            <iframe name="level1" srcdoc="<html><body><h2>Level 1</h2><iframe name='level2' srcdoc='<html><body><h3>Level 2</h3></body></html>'></iframe></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for nested iframes to load
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Get all frames
    let frames = page.frames().await.expect("Failed to get frames");
    println!("Total frames: {}", frames.len());

    // Capture multi-frame snapshot
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get snapshot");

    let yaml = snapshot.to_yaml();
    println!("Nested frames snapshot:\n{}", yaml);

    // Should have content from multiple levels
    assert!(
        yaml.contains("heading") || yaml.contains("Level"),
        "Snapshot should contain heading content"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Locator Aria Snapshot Tests
// =============================================================================

/// Test aria_snapshot on a specific locator.
#[tokio::test]
async fn test_locator_aria_snapshot() {
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
            <form id="myform">
                <label for="name">Name:</label>
                <input id="name" type="text" />
                <button type="submit">Submit</button>
            </form>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot of just the form
    let form_snapshot = page
        .locator("#myform")
        .aria_snapshot()
        .await
        .expect("Failed to get form snapshot");

    let yaml = form_snapshot.to_yaml();
    println!("Form snapshot:\n{}", yaml);

    // Should contain form elements
    assert!(
        yaml.contains("form") || yaml.contains("textbox") || yaml.contains("button"),
        "Form snapshot should contain form, textbox, or button, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
