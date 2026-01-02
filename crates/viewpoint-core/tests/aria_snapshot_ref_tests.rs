#![cfg(feature = "integration")]

//! ARIA snapshot node reference tests for viewpoint-core.
//!
//! These tests verify element refs in ARIA snapshots and ref resolution.

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
    println!("Snapshot with refs:\n{yaml}");

    // Verify refs are present for buttons (format: [ref=eXXX])
    assert!(
        yaml.contains("[ref=e"),
        "Snapshot should contain refs for buttons, got: {yaml}"
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
        r"
        <html><body>
            <h1>Main Title</h1>
            <h2>Subtitle</h2>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot with heading refs:\n{yaml}");

    // Verify refs are present for headings
    assert!(
        yaml.contains("[ref=e"),
        "Snapshot should contain refs for headings, got: {yaml}"
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
    println!("Initial snapshot:\n{yaml}");

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
    println!("Found button ref: {button_ref}");

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
    assert_eq!(
        new_text.as_deref(),
        Some("Clicked!"),
        "Button text should have changed"
    );

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
    println!("Snapshot with dynamic element:\n{yaml}");

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
    println!("Found dynamic button ref: {button_ref}");

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
    assert!(
        result.is_err(),
        "Should fail for non-existent backend node ID"
    );

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

/// Test that Frame.aria_snapshot() (via main_frame) includes element refs.
///
/// Note: This tests main_frame().aria_snapshot() which is what Page.aria_snapshot_with_frames()
/// calls internally. The frame execution context targeting is a separate concern.
#[tokio::test]
async fn test_main_frame_aria_snapshot_includes_refs() {
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
        r##"
        <html><body>
            <h1>Main Heading</h1>
            <button id="main-btn">Main Button</button>
            <a href="#">Main Link</a>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Get the main frame and capture snapshot
    let main_frame = page.main_frame().await.expect("Failed to get main frame");
    let frame_snapshot = main_frame
        .aria_snapshot()
        .await
        .expect("Failed to get frame snapshot");

    let yaml = frame_snapshot.to_yaml();
    println!("Main frame snapshot with refs:\n{yaml}");

    // Verify refs are present for elements (format: [ref=eXXX])
    assert!(
        yaml.contains("[ref=e"),
        "Main frame snapshot should contain refs for elements, got: {yaml}"
    );

    // Verify we can find a button ref in the snapshot structure
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

    let button_ref = find_button_ref(&frame_snapshot);
    assert!(
        button_ref.is_some(),
        "Should find a button with ref in main frame snapshot"
    );

    // Verify the ref can be resolved from the Page
    let button_ref_str = button_ref.unwrap();
    let handle = page
        .element_from_ref(&button_ref_str)
        .await
        .expect("Failed to resolve ref from frame snapshot");

    assert!(
        handle.is_attached().await.unwrap(),
        "Element from main frame should be attached"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
