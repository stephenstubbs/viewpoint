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

// =============================================================================
// Node Reference Tests
// =============================================================================

/// Test that snapshots include refs for interactive elements.
#[tokio::test]
async fn test_aria_snapshot_includes_refs_for_buttons() {
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
            <button id="btn1">Click me</button>
            <button id="btn2">Submit</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot with refs:\n{}", yaml);

    // Verify refs are present for buttons (format: [ref=eXXX])
    assert!(
        yaml.contains("[ref=e"),
        "Snapshot should contain refs for buttons, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs work for headings (non-interactive elements).
#[tokio::test]
async fn test_aria_snapshot_includes_refs_for_headings() {
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
            <h2>Subtitle</h2>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot with heading refs:\n{}", yaml);

    // Verify refs are present for headings
    assert!(
        yaml.contains("[ref=e"),
        "Snapshot should contain refs for headings, got: {}",
        yaml
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test ref resolution - click an element using its ref.
#[tokio::test]
async fn test_ref_resolution_and_click() {
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
            <button id="mybutton" onclick="this.textContent='Clicked!'">Click me</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Initial snapshot:\n{}", yaml);

    // Find the button's ref in the snapshot
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let button_ref = find_button_ref(&snapshot).expect("Should find button ref");
    println!("Found button ref: {}", button_ref);

    // Click using the ref
    let handle = page
        .element_from_ref(&button_ref)
        .await
        .expect("Failed to resolve ref");

    // Verify element is valid
    assert!(
        handle.is_attached().await.unwrap(),
        "Element should be attached"
    );

    // Click the button via locator
    let locator = page.locator_from_ref(&button_ref);
    locator.click().await.expect("Failed to click button");

    // Wait a bit for the click to process
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify the button text changed
    let new_text = page
        .locator("#mybutton")
        .text_content()
        .await
        .expect("Failed to get text");
    assert_eq!(new_text.as_deref(), Some("Clicked!"), "Button text should have changed");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test ref resolution for dynamically created elements.
#[tokio::test]
async fn test_ref_for_dynamic_elements() {
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
            <div id="container"></div>
            <script>
                // Dynamically create a button after a short delay
                setTimeout(() => {
                    const btn = document.createElement('button');
                    btn.id = 'dynamic-btn';
                    btn.textContent = 'Dynamic Button';
                    document.getElementById('container').appendChild(btn);
                }, 100);
            </script>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for dynamic button to be created
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Capture snapshot after dynamic element is created
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot with dynamic element:\n{}", yaml);

    // Find the dynamic button's ref
    fn find_dynamic_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button")
            && snapshot.name.as_deref() == Some("Dynamic Button")
        {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_dynamic_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let button_ref = find_dynamic_button_ref(&snapshot).expect("Should find dynamic button ref");
    println!("Found dynamic button ref: {}", button_ref);

    // Verify we can resolve the ref
    let handle = page
        .element_from_ref(&button_ref)
        .await
        .expect("Failed to resolve dynamic element ref");

    assert!(
        handle.is_attached().await.unwrap(),
        "Dynamic element should be attached"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test error handling for invalid refs.
#[tokio::test]
async fn test_invalid_ref_handling() {
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

    page.set_content("<html><body><p>Test</p></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Try to resolve an invalid ref format
    let result = page.element_from_ref("invalid-ref").await;
    assert!(result.is_err(), "Should fail for invalid ref format");

    // Try to resolve a non-existent backend node ID
    let result = page.element_from_ref("e999999999").await;
    assert!(result.is_err(), "Should fail for non-existent backend node ID");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test error handling for stale refs (element removed after snapshot).
#[tokio::test]
async fn test_stale_ref_handling() {
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
            <button id="removable">Remove Me</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the button's ref
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let button_ref = find_button_ref(&snapshot).expect("Should find button ref");

    // Remove the element from the DOM
    page.evaluate::<()>("document.getElementById('removable').remove()")
        .await
        .expect("Failed to remove element");

    // Try to resolve the stale ref - may succeed (node still in memory) or fail (cleaned up)
    // The important thing is it shouldn't panic
    let result = page.element_from_ref(&button_ref).await;
    println!("Stale ref resolution result: {:?}", result.is_ok());

    // If we got a handle, verify is_attached returns false
    if let Ok(handle) = result {
        let attached = handle.is_attached().await.unwrap_or(true);
        assert!(!attached, "Removed element should not be attached");
    }

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
