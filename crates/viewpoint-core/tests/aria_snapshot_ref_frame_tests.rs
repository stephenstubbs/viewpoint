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

// =============================================================================
// Iframe Ref Resolution Tests (aria_snapshot_with_frames)
// =============================================================================

/// Test that element refs from iframes are resolvable after aria_snapshot_with_frames().
///
/// This verifies that the fix for iframe ref resolution works - child frame refs
/// are now stored in Page's ref_map and can be resolved via page.locator_from_ref().
#[tokio::test]
async fn test_iframe_refs_resolvable_via_aria_snapshot_with_frames() {
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
            <iframe name="widget" srcdoc="<html><body><button id='iframe-btn'>Iframe Button</button></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Capture snapshot with frames - this should now store iframe refs in Page's ref_map
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Multi-frame snapshot:\n{yaml}");

    // Find the button ref from the iframe
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

    let button_ref = find_button_ref(&snapshot).expect("Should find button ref from iframe");
    println!("Found iframe button ref: {button_ref}");

    // Verify the ref contains a frame index (f1 for first child frame)
    assert!(
        button_ref.contains("f1"),
        "Iframe button ref should have frame index f1, got: {button_ref}"
    );

    // The iframe button ref should now be resolvable via page.element_from_ref()
    let handle = page
        .element_from_ref(&button_ref)
        .await
        .expect("Iframe refs should now be resolvable via aria_snapshot_with_frames()");

    assert!(
        handle.is_attached().await.unwrap(),
        "Resolved iframe element should be attached"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clicking an element inside an iframe after aria_snapshot_with_frames().
///
/// This test verifies that we can click an iframe button using its ref.
/// The button updates its own text content to prove the click occurred.
#[tokio::test]
async fn test_click_iframe_element_via_ref() {
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

    // The button updates its own text content when clicked
    page.set_content(
        r#"
        <html><body>
            <h1>Main Page</h1>
            <iframe name="widget" srcdoc="<html><body><button onclick='this.innerText = &quot;Clicked!&quot;'>Click Me</button></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Capture snapshot with frames
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    // Find the button ref
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
    println!("Clicking button with ref: {button_ref}");

    // Click the button via locator_from_ref
    page.locator_from_ref(&button_ref)
        .click()
        .await
        .expect("Should be able to click iframe button via ref");

    // Wait for click to process
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Verify the click worked by getting the button's text content (it should now say "Clicked!")
    let button_text = page
        .locator_from_ref(&button_ref)
        .text_content()
        .await
        .expect("Should get button text");

    assert_eq!(
        button_text,
        Some("Clicked!".to_string()),
        "Button click should have updated button text"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test typing into an input inside an iframe after aria_snapshot_with_frames().
#[tokio::test]
async fn test_type_into_iframe_input_via_ref() {
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
            <iframe name="form-frame" srcdoc="<html><body><label for='name'>Name:</label><input type='text' id='name' placeholder='Enter name'></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Capture snapshot with frames
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Snapshot:\n{yaml}");

    // Find the textbox ref
    fn find_textbox_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("textbox") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_textbox_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let input_ref = find_textbox_ref(&snapshot).expect("Should find textbox ref");
    println!("Filling input with ref: {input_ref}");

    // Fill the input via locator_from_ref
    page.locator_from_ref(&input_ref)
        .fill("Test User")
        .await
        .expect("Should be able to fill iframe input via ref");

    // Verify the input was filled by getting its value
    let value = page
        .locator_from_ref(&input_ref)
        .input_value()
        .await
        .expect("Should get input value");

    assert_eq!(value, "Test User", "Input should contain typed text");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs from nested iframes are resolvable.
#[tokio::test]
async fn test_nested_iframe_refs_resolvable() {
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
            <h1>Level 0 Main</h1>
            <button id="btn0">Main Button</button>
            <iframe name="level1" srcdoc="<html><body><h2>Level 1</h2><button id='btn1'>Level 1 Button</button><iframe name='level2' srcdoc='<html><body><h3>Level 2</h3><button id=&quot;btn2&quot;>Level 2 Button</button></body></html>'></iframe></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for nested iframes to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Capture snapshot with frames
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Nested frames snapshot:\n{yaml}");

    // Collect all button refs
    fn collect_button_refs(snapshot: &viewpoint_core::AriaSnapshot, refs: &mut Vec<String>) {
        if snapshot.role.as_deref() == Some("button") {
            if let Some(ref r) = snapshot.node_ref {
                refs.push(r.clone());
            }
        }
        for child in &snapshot.children {
            collect_button_refs(child, refs);
        }
    }

    let mut button_refs: Vec<String> = Vec::new();
    collect_button_refs(&snapshot, &mut button_refs);

    println!("Found button refs: {:?}", button_refs);

    // Should have at least 2 buttons (main and one from iframes)
    assert!(
        button_refs.len() >= 2,
        "Should have at least 2 button refs, got {}",
        button_refs.len()
    );

    // Verify all button refs are resolvable
    for button_ref in &button_refs {
        let handle = page.element_from_ref(button_ref).await.expect(&format!(
            "Button ref {} should be resolvable",
            button_ref
        ));
        assert!(
            handle.is_attached().await.unwrap(),
            "Resolved button should be attached"
        );
    }

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs from different frames have distinct frame indices.
#[tokio::test]
async fn test_iframe_refs_have_correct_frame_indices() {
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
            <button id="main-btn">Main Button</button>
            <iframe name="frame1" srcdoc="<html><body><button id='btn1'>Frame1 Button</button></body></html>"></iframe>
            <iframe name="frame2" srcdoc="<html><body><button id='btn2'>Frame2 Button</button></body></html>"></iframe>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframes to load
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Capture snapshot with frames
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Multi-frame snapshot:\n{yaml}");

    // Collect all button refs with their names
    fn collect_button_info(
        snapshot: &viewpoint_core::AriaSnapshot,
        info: &mut Vec<(String, String)>,
    ) {
        if snapshot.role.as_deref() == Some("button") {
            if let Some(ref r) = snapshot.node_ref {
                let name = snapshot.name.clone().unwrap_or_default();
                info.push((name, r.clone()));
            }
        }
        for child in &snapshot.children {
            collect_button_info(child, info);
        }
    }

    let mut button_info: Vec<(String, String)> = Vec::new();
    collect_button_info(&snapshot, &mut button_info);

    println!("Button info: {:?}", button_info);

    // Extract frame indices from refs (format: c{ctx}p{page}f{frame}e{element})
    fn extract_frame_index(ref_str: &str) -> Option<usize> {
        // Parse the ref format: c0p0f1e5
        let f_pos = ref_str.find('f')?;
        let e_pos = ref_str.find('e')?;
        if f_pos < e_pos {
            ref_str[f_pos + 1..e_pos].parse().ok()
        } else {
            None
        }
    }

    // Find main button (should have f0)
    let main_btn = button_info
        .iter()
        .find(|(name, _)| name.contains("Main"))
        .expect("Should find main button");
    let main_frame_idx = extract_frame_index(&main_btn.1).expect("Main button should have frame index");
    assert_eq!(main_frame_idx, 0, "Main frame button should have frame index 0");

    // Find frame1 button (should have f1 or f2)
    let frame1_btn = button_info
        .iter()
        .find(|(name, _)| name.contains("Frame1"))
        .expect("Should find frame1 button");
    let frame1_idx = extract_frame_index(&frame1_btn.1).expect("Frame1 button should have frame index");
    assert!(
        frame1_idx > 0,
        "Frame1 button should have frame index > 0, got {}",
        frame1_idx
    );

    // Find frame2 button (should have different index from frame1)
    let frame2_btn = button_info
        .iter()
        .find(|(name, _)| name.contains("Frame2"))
        .expect("Should find frame2 button");
    let frame2_idx = extract_frame_index(&frame2_btn.1).expect("Frame2 button should have frame index");
    assert!(
        frame2_idx > 0,
        "Frame2 button should have frame index > 0, got {}",
        frame2_idx
    );

    // Frame indices should be different for different frames
    assert_ne!(
        frame1_idx, frame2_idx,
        "Frame1 and Frame2 should have different frame indices"
    );

    // All refs should be resolvable
    for (name, ref_str) in &button_info {
        let handle = page.element_from_ref(ref_str).await.expect(&format!(
            "Button '{}' ref {} should be resolvable",
            name, ref_str
        ));
        assert!(
            handle.is_attached().await.unwrap(),
            "Button '{}' should be attached",
            name
        );
    }

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
