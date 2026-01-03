#![cfg(feature = "integration")]

//! ARIA snapshot ref resolution tests.
//!
//! These tests verify refs can be resolved to elements and used for interactions.

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
