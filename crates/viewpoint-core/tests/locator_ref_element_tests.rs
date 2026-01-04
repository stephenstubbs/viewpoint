#![cfg(feature = "integration")]

//! Tests for locator element operations using refs from ARIA snapshots.
//!
//! These tests verify element-level operations (evaluate, handle, screenshot, bbox)
//! work correctly when using refs obtained via `page.locator_from_ref(ref)`.

mod common;

use viewpoint_core::Browser;

/// Test: evaluate on element via ref.
#[tokio::test]
async fn test_evaluate_via_ref() {
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
        r#"
        <html><body>
            <button id="mybtn" data-custom="test-value">Click me</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the button's ref
    let button_ref = common::find_ref_by_role(&snapshot, "button", Some("Click me"))
        .expect("Should find button ref");

    // Evaluate on element using ref
    let locator = page.locator_from_ref(&button_ref);
    let tag_name: String = locator
        .evaluate("element.tagName.toLowerCase()")
        .await
        .expect("Failed to evaluate via ref");

    assert_eq!(tag_name, "button", "Should get correct tag name");

    // Also test getting a custom attribute
    let custom_attr: String = locator
        .evaluate("element.dataset.custom")
        .await
        .expect("Failed to evaluate dataset");

    assert_eq!(custom_attr, "test-value", "Should get custom attribute");

    browser.close().await.expect("Failed to close browser");
}

/// Test: element_handle via ref.
#[tokio::test]
async fn test_element_handle_via_ref() {
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

    // Use an element with an ARIA role so it appears in the snapshot
    page.set_content(
        r#"
        <html><body>
            <article id="target" style="width: 100px; height: 50px; background: blue;">Target Content</article>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the article element ref
    let target_ref =
        common::find_ref_by_role(&snapshot, "article", None).expect("Should find article ref");

    // Get element handle using ref
    let locator = page.locator_from_ref(&target_ref);
    let handle = locator
        .element_handle()
        .await
        .expect("Failed to get element handle via ref");

    assert!(
        handle.is_attached().await.unwrap(),
        "Element should be attached"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test: element screenshot (bounding box) via ref.
#[tokio::test]
async fn test_element_screenshot_via_ref() {
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

    // Use an element with an ARIA role so it appears in the snapshot
    page.set_content(
        r#"
        <html><body>
            <section id="screenshot-target" style="width: 200px; height: 100px; background: red;">
                <p>Screenshot Target</p>
            </section>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the section element ref (has "region" role)
    let target_ref =
        common::find_ref_by_role(&snapshot, "region", None).expect("Should find section ref");

    // Take screenshot using ref
    let locator = page.locator_from_ref(&target_ref);
    let screenshot_bytes = locator
        .screenshot()
        .capture()
        .await
        .expect("Failed to take screenshot via ref");

    // Verify we got some PNG data
    assert!(
        !screenshot_bytes.is_empty(),
        "Screenshot should not be empty"
    );
    // PNG files start with specific magic bytes
    assert_eq!(
        &screenshot_bytes[0..4],
        &[0x89, 0x50, 0x4E, 0x47],
        "Should be valid PNG"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test: bounding_box via ref.
#[tokio::test]
async fn test_bounding_box_via_ref() {
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
        r#"
        <html><body>
            <button id="sized-btn" style="width: 150px; height: 50px;">Sized Button</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the button's ref
    let button_ref = common::find_ref_by_role(&snapshot, "button", Some("Sized Button"))
        .expect("Should find button ref");

    // Get bounding box via ref
    let locator = page.locator_from_ref(&button_ref);
    let bbox = locator
        .bounding_box()
        .await
        .expect("Failed to get bounding_box via ref");

    let bbox = bbox.expect("Should have bounding box");
    assert!(
        bbox.width >= 150.0,
        "Width should be at least 150px, got {}",
        bbox.width
    );
    assert!(
        bbox.height >= 50.0,
        "Height should be at least 50px, got {}",
        bbox.height
    );

    browser.close().await.expect("Failed to close browser");
}
