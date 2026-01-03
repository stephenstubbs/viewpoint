#![cfg(feature = "integration")]

//! ARIA snapshot ref error handling tests.
//!
//! These tests verify error handling for invalid, stale, and missing refs.

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

    // Try to resolve a legacy format ref (no longer supported)
    let result = page.element_from_ref("e999999999").await;
    assert!(
        result.is_err(),
        "Should fail for legacy e{{id}} format (no longer supported)"
    );

    // Try to resolve a valid format ref that doesn't exist in ref_map
    let result = page.element_from_ref("c0p0f0e999999").await;
    assert!(
        result.is_err(),
        "Should fail for non-existent ref"
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
