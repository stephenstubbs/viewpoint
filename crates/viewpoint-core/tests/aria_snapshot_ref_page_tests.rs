#![cfg(feature = "integration")]

//! ARIA snapshot page-level ref tests.
//!
//! These tests verify ref behavior with page-level snapshots vs frame-level snapshots.

mod common;

use viewpoint_core::Browser;

/// Test that Frame.aria_snapshot() includes element refs in the snapshot,
/// but these refs are NOT resolvable via page.element_from_ref().
///
/// For resolvable refs, use page.aria_snapshot() instead.
#[tokio::test]
async fn test_frame_aria_snapshot_refs_not_resolvable_via_page() {
    common::init_tracing();

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
    let button_ref = common::find_button_ref(&frame_snapshot).expect("Should find button ref");

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
    common::init_tracing();

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
    let button_ref = common::find_button_ref(&page_snapshot).expect("Should find button ref");
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
