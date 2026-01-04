#![cfg(feature = "integration")]

//! Tests for locator query operations using refs from ARIA snapshots.
//!
//! These tests verify query operations (text, attributes, checked state)
//! work correctly when using refs obtained via `page.locator_from_ref(ref)`.

mod common;

use viewpoint_core::{AriaSnapshot, Browser};

/// Test: query methods (is_checked, inner_text, get_attribute, input_value) via ref.
#[tokio::test]
async fn test_query_methods_via_ref() {
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
            <input type="checkbox" id="mycheck" checked />
            <p id="para" data-info="test-info">Hello World</p>
            <input type="text" id="myinput" value="initial value" />
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Test is_checked via ref (find checkbox)
    let checkbox_ref =
        common::find_ref_by_role(&snapshot, "checkbox", None).expect("Should find checkbox ref");
    let checkbox_locator = page.locator_from_ref(&checkbox_ref);
    let is_checked = checkbox_locator
        .is_checked()
        .await
        .expect("Failed to check is_checked via ref");
    assert!(is_checked, "Checkbox should be checked");

    // Test inner_text via ref (find paragraph)
    fn find_paragraph_ref(snapshot: &AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("paragraph") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_paragraph_ref(child) {
                return Some(r);
            }
        }
        None
    }
    let para_ref = find_paragraph_ref(&snapshot).expect("Should find paragraph ref");
    let para_locator = page.locator_from_ref(&para_ref);
    let inner_text = para_locator
        .inner_text()
        .await
        .expect("Failed to get inner_text via ref");
    assert_eq!(inner_text, "Hello World", "Should get correct inner text");

    // Test get_attribute via ref
    let attr = para_locator
        .get_attribute("data-info")
        .await
        .expect("Failed to get_attribute via ref");
    assert_eq!(
        attr.as_deref(),
        Some("test-info"),
        "Should get correct attribute"
    );

    // Test input_value via ref (find textbox)
    let input_ref =
        common::find_ref_by_role(&snapshot, "textbox", None).expect("Should find input ref");
    let input_locator = page.locator_from_ref(&input_ref);
    let value = input_locator
        .input_value()
        .await
        .expect("Failed to get input_value via ref");
    assert_eq!(value, "initial value", "Should get correct input value");

    browser.close().await.expect("Failed to close browser");
}

/// Test: aria_snapshot on locator via ref (nested snapshot).
#[tokio::test]
async fn test_aria_snapshot_on_locator_via_ref() {
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
            <div role="navigation">
                <button>Home</button>
                <button>About</button>
            </div>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find the navigation's ref
    let nav_ref = common::find_ref_by_role(&snapshot, "navigation", None)
        .expect("Should find navigation ref");

    // Get aria_snapshot of the navigation element via ref
    let locator = page.locator_from_ref(&nav_ref);
    let nested_snapshot = locator
        .aria_snapshot()
        .await
        .expect("Failed to get aria_snapshot via ref");

    let yaml = nested_snapshot.to_yaml();
    println!("Nested snapshot:\n{yaml}");

    // Verify it contains the buttons
    assert!(yaml.contains("Home"), "Should contain Home button");
    assert!(yaml.contains("About"), "Should contain About button");

    browser.close().await.expect("Failed to close browser");
}

/// Test: all_inner_texts and all_text_contents via ref.
#[tokio::test]
async fn test_all_texts_via_ref() {
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
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
                <li>Item 3</li>
            </ul>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Find a list item ref
    let listitem_ref =
        common::find_ref_by_role(&snapshot, "listitem", None).expect("Should find listitem ref");

    // Test all_inner_texts via ref (single element returns array with one item)
    let locator = page.locator_from_ref(&listitem_ref);
    let inner_texts = locator
        .all_inner_texts()
        .await
        .expect("Failed to get all_inner_texts via ref");

    assert_eq!(inner_texts.len(), 1, "Should have 1 text (single element)");
    assert!(inner_texts[0].contains("Item"), "Should contain 'Item'");

    // Test all_text_contents via ref
    let text_contents = locator
        .all_text_contents()
        .await
        .expect("Failed to get all_text_contents via ref");

    assert_eq!(text_contents.len(), 1, "Should have 1 text content");
    assert!(text_contents[0].contains("Item"), "Should contain 'Item'");

    browser.close().await.expect("Failed to close browser");
}
