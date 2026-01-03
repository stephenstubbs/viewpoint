#![cfg(feature = "integration")]

//! ARIA snapshot frame-related ref tests.
//!
//! These tests verify ref behavior with frames and frame snapshots.

use std::sync::Once;

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

/// Test that Frame.aria_snapshot() includes element refs in the snapshot,
/// but these refs are NOT resolvable via page.element_from_ref().
///
/// For resolvable refs, use page.aria_snapshot() instead.
#[tokio::test]
async fn test_frame_aria_snapshot_refs_not_resolvable_via_page() {
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

    // Get the main frame and capture snapshot via Frame
    let main_frame = page.main_frame().await.expect("Failed to get main frame");
    let frame_snapshot = main_frame
        .aria_snapshot()
        .await
        .expect("Failed to get frame snapshot");

    let yaml = frame_snapshot.to_yaml();
    println!("Frame snapshot with refs:\n{yaml}");

    // Verify refs are present in the snapshot
    assert!(
        yaml.contains("[ref=c") && yaml.contains('p') && yaml.contains('e'),
        "Frame snapshot should contain refs in format c{{ctx}}p{{page}}e{{counter}}, got: {yaml}"
    );

    // Find a button ref in the snapshot
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

    let button_ref = find_button_ref(&frame_snapshot).expect("Should find button ref");
    
    // IMPORTANT: Frame-captured refs are NOT resolvable via page.element_from_ref()
    // This is expected behavior - use page.aria_snapshot() for resolvable refs
    let result = page.element_from_ref(&button_ref).await;
    assert!(
        result.is_err(),
        "Frame-captured refs should NOT be resolvable via page.element_from_ref()"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that page.aria_snapshot() refs ARE resolvable via page.element_from_ref().
///
/// This is the recommended workflow for getting resolvable refs.
#[tokio::test]
async fn test_page_aria_snapshot_refs_are_resolvable() {
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

    // Capture snapshot via Page (not Frame)
    let page_snapshot = page
        .aria_snapshot()
        .await
        .expect("Failed to get page snapshot");

    let yaml = page_snapshot.to_yaml();
    println!("Page snapshot with refs:\n{yaml}");

    // Verify refs are present for elements
    assert!(
        yaml.contains("[ref=c") && yaml.contains('p') && yaml.contains('e'),
        "Page snapshot should contain refs in format c{{ctx}}p{{page}}e{{counter}}, got: {yaml}"
    );

    // Find a button ref in the snapshot
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

    let button_ref = find_button_ref(&page_snapshot).expect("Should find button ref");
    println!("Found button ref: {button_ref}");

    // Page-captured refs ARE resolvable via page.element_from_ref()
    let handle = page
        .element_from_ref(&button_ref)
        .await
        .expect("Page-captured refs should be resolvable");

    assert!(
        handle.is_attached().await.unwrap(),
        "Resolved element should be attached"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
