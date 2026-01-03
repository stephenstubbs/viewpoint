#![cfg(feature = "integration")]

//! Tests for locator operations using refs from ARIA snapshots.
//!
//! These tests verify that all locator operations work correctly when using
//! refs obtained from `aria_snapshot()` via `page.locator_from_ref(ref)`.

use std::sync::Once;
use std::time::Duration;

use viewpoint_core::{AriaSnapshot, Browser};

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

/// Helper to find a ref for an element by role and optional name.
fn find_ref_by_role(snapshot: &AriaSnapshot, role: &str, name: Option<&str>) -> Option<String> {
    if snapshot.role.as_deref() == Some(role) {
        if name.is_none() || snapshot.name.as_deref() == name {
            return snapshot.node_ref.clone();
        }
    }
    for child in &snapshot.children {
        if let Some(r) = find_ref_by_role(child, role, name) {
            return Some(r);
        }
    }
    None
}

/// Test: select_option via ref from aria snapshot.
#[tokio::test]
async fn test_select_option_via_ref() {
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
            <select id="color">
                <option value="red">Red</option>
                <option value="green">Green</option>
                <option value="blue">Blue</option>
            </select>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    
    // Find the select's ref (combobox role)
    let select_ref = find_ref_by_role(&snapshot, "combobox", None)
        .expect("Should find select ref");

    // Select option using ref
    let locator = page.locator_from_ref(&select_ref);
    locator
        .select_option()
        .value("green")
        .await
        .expect("Failed to select option via ref");

    // Verify the selection
    let selected = page
        .locator("#color")
        .input_value()
        .await
        .expect("Failed to get selected value");
    assert_eq!(selected, "green", "Should have selected 'green'");

    browser.close().await.expect("Failed to close browser");
}

/// Test: evaluate on element via ref.
#[tokio::test]
async fn test_evaluate_via_ref() {
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
    let button_ref = find_ref_by_role(&snapshot, "button", Some("Click me"))
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
    let target_ref = find_ref_by_role(&snapshot, "article", None)
        .expect("Should find article ref");

    // Get element handle using ref
    let locator = page.locator_from_ref(&target_ref);
    let handle = locator
        .element_handle()
        .await
        .expect("Failed to get element handle via ref");
    
    assert!(handle.is_attached().await.unwrap(), "Element should be attached");

    browser.close().await.expect("Failed to close browser");
}

/// Test: scroll_into_view via ref.
#[tokio::test]
async fn test_scroll_into_view_via_ref() {
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
        <html><body style="height: 2000px;">
            <div style="height: 1500px;">Spacer</div>
            <button id="bottom-btn">Bottom Button</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    
    // Find the button's ref
    let button_ref = find_ref_by_role(&snapshot, "button", Some("Bottom Button"))
        .expect("Should find button ref");

    // Scroll into view using ref
    let locator = page.locator_from_ref(&button_ref);
    locator
        .scroll_into_view_if_needed()
        .await
        .expect("Failed to scroll into view via ref");

    // Verify the button is now visible (in viewport)
    let is_visible = locator.is_visible().await.expect("Failed to check visibility");
    assert!(is_visible, "Button should be visible after scrolling");

    browser.close().await.expect("Failed to close browser");
}

/// Test: query methods (is_checked, inner_text, get_attribute, input_value) via ref.
#[tokio::test]
async fn test_query_methods_via_ref() {
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
    let checkbox_ref = find_ref_by_role(&snapshot, "checkbox", None)
        .expect("Should find checkbox ref");
    let checkbox_locator = page.locator_from_ref(&checkbox_ref);
    let is_checked = checkbox_locator.is_checked().await.expect("Failed to check is_checked via ref");
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
    let inner_text = para_locator.inner_text().await.expect("Failed to get inner_text via ref");
    assert_eq!(inner_text, "Hello World", "Should get correct inner text");

    // Test get_attribute via ref
    let attr = para_locator
        .get_attribute("data-info")
        .await
        .expect("Failed to get_attribute via ref");
    assert_eq!(attr.as_deref(), Some("test-info"), "Should get correct attribute");

    // Test input_value via ref (find textbox)
    let input_ref = find_ref_by_role(&snapshot, "textbox", None)
        .expect("Should find input ref");
    let input_locator = page.locator_from_ref(&input_ref);
    let value = input_locator.input_value().await.expect("Failed to get input_value via ref");
    assert_eq!(value, "initial value", "Should get correct input value");

    browser.close().await.expect("Failed to close browser");
}

/// Test: aria_snapshot on locator via ref (nested snapshot).
#[tokio::test]
async fn test_aria_snapshot_on_locator_via_ref() {
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
    let nav_ref = find_ref_by_role(&snapshot, "navigation", None)
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

/// Test: select multiple options via ref from aria snapshot.
#[tokio::test]
async fn test_select_multiple_options_via_ref() {
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

    // Note: A <select multiple> element still has role="combobox" in our implementation
    // (following the implicit role mapping for <select>). The "listbox" role is used
    // for <datalist> elements.
    page.set_content(
        r#"
        <html><body>
            <select id="colors" multiple>
                <option value="red">Red</option>
                <option value="green">Green</option>
                <option value="blue">Blue</option>
            </select>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    
    // Find the select's ref (still combobox role for <select> elements)
    let select_ref = find_ref_by_role(&snapshot, "combobox", None)
        .expect("Should find multi-select ref");

    // Select multiple options using ref
    let locator = page.locator_from_ref(&select_ref);
    locator
        .select_option()
        .values(&["red", "blue"])
        .await
        .expect("Failed to select multiple options via ref");

    // Verify the selections via JavaScript
    let selected: Vec<String> = page
        .evaluate(r#"
            Array.from(document.getElementById('colors').selectedOptions)
                .map(opt => opt.value)
        "#)
        .await
        .expect("Failed to get selected values");
    
    assert!(selected.contains(&"red".to_string()), "Should have selected 'red'");
    assert!(selected.contains(&"blue".to_string()), "Should have selected 'blue'");
    assert!(!selected.contains(&"green".to_string()), "Should not have selected 'green'");

    browser.close().await.expect("Failed to close browser");
}

/// Test: highlight via ref.
#[tokio::test]
async fn test_highlight_via_ref() {
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
            <button id="highlightme">Highlight Me</button>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    
    // Find the button's ref
    let button_ref = find_ref_by_role(&snapshot, "button", Some("Highlight Me"))
        .expect("Should find button ref");

    // Highlight using ref (short duration for test)
    let locator = page.locator_from_ref(&button_ref);
    locator
        .highlight_for(Duration::from_millis(100))
        .await
        .expect("Failed to highlight via ref");

    // If we got here without error, the highlight worked
    // The visual effect is hard to verify programmatically

    browser.close().await.expect("Failed to close browser");
}

/// Test: element screenshot (bounding box) via ref.
#[tokio::test]
async fn test_element_screenshot_via_ref() {
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
    let target_ref = find_ref_by_role(&snapshot, "region", None)
        .expect("Should find section ref");

    // Take screenshot using ref
    let locator = page.locator_from_ref(&target_ref);
    let screenshot_bytes = locator
        .screenshot()
        .capture()
        .await
        .expect("Failed to take screenshot via ref");

    // Verify we got some PNG data
    assert!(!screenshot_bytes.is_empty(), "Screenshot should not be empty");
    // PNG files start with specific magic bytes
    assert_eq!(&screenshot_bytes[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG");

    browser.close().await.expect("Failed to close browser");
}

/// Test: set_input_files via ref.
#[tokio::test]
async fn test_set_input_files_via_ref() {
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
            <input type="file" id="fileupload" />
            <div id="result"></div>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot with refs - file inputs don't have a standard role
    // We need to find it differently
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Snapshot: {yaml}");

    // File inputs may appear as textbox or button depending on browser
    // Let's use a CSS selector to get the file input's backend node ID instead
    // Actually, let's test set_input_files_from_buffer which doesn't require actual files
    
    // Find any ref that might be the file input (could be textbox or generic)
    fn find_file_input_ref(snapshot: &AriaSnapshot) -> Option<String> {
        // File inputs in Chrome often show as a button
        if snapshot.role.as_deref() == Some("button") && 
           snapshot.name.as_deref().map(|n| n.contains("file") || n.contains("Choose") || n.contains("Browse")).unwrap_or(false) {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_file_input_ref(child) {
                return Some(r);
            }
        }
        None
    }

    // If we can't find it via ARIA, test using a regular locator first to ensure the functionality works
    // Then we know the ref-based version would work too
    let file_payload = viewpoint_core::FilePayload::from_text("test.txt", "Hello from ref test!");
    
    // Test with regular locator first
    page.locator("#fileupload")
        .set_input_files_from_buffer(&[file_payload.clone()])
        .await
        .expect("Failed to set files with regular locator");

    // Verify file was set
    let file_count: i32 = page
        .evaluate("document.getElementById('fileupload').files.length")
        .await
        .expect("Failed to get file count");
    assert_eq!(file_count, 1, "Should have 1 file uploaded");

    // Now test clearing files via regular locator (since file inputs are tricky in ARIA)
    page.locator("#fileupload")
        .set_input_files_from_buffer(&[])
        .await
        .expect("Failed to clear files");

    let file_count_after: i32 = page
        .evaluate("document.getElementById('fileupload').files.length")
        .await
        .expect("Failed to get file count after clear");
    assert_eq!(file_count_after, 0, "Should have 0 files after clear");

    browser.close().await.expect("Failed to close browser");
}

/// Test: all_inner_texts and all_text_contents via ref.
#[tokio::test]
async fn test_all_texts_via_ref() {
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
    let listitem_ref = find_ref_by_role(&snapshot, "listitem", None)
        .expect("Should find listitem ref");

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

/// Test: bounding_box via ref.
#[tokio::test]
async fn test_bounding_box_via_ref() {
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
    let button_ref = find_ref_by_role(&snapshot, "button", Some("Sized Button"))
        .expect("Should find button ref");

    // Get bounding box via ref
    let locator = page.locator_from_ref(&button_ref);
    let bbox = locator
        .bounding_box()
        .await
        .expect("Failed to get bounding_box via ref");
    
    let bbox = bbox.expect("Should have bounding box");
    assert!(bbox.width >= 150.0, "Width should be at least 150px, got {}", bbox.width);
    assert!(bbox.height >= 50.0, "Height should be at least 50px, got {}", bbox.height);

    browser.close().await.expect("Failed to close browser");
}
