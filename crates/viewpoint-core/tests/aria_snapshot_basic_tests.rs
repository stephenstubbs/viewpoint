#![cfg(feature = "integration")]

//! Basic ARIA snapshot tests for viewpoint-core.
//!
//! These tests verify basic ARIA accessibility snapshot capture
//! and locator-based snapshot capture.

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
        r"
        <html><body>
            <h1>Main Title</h1>
            <button>Click me</button>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture snapshot
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");

    // Convert to YAML for inspection
    let yaml = snapshot.to_yaml();
    println!("Snapshot YAML:\n{yaml}");

    // Verify basic structure
    assert!(
        yaml.contains("heading") || yaml.contains("button"),
        "Snapshot should contain heading or button, got: {yaml}"
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
    println!("Form snapshot:\n{yaml}");

    // Should contain form elements
    assert!(
        yaml.contains("form") || yaml.contains("textbox") || yaml.contains("button"),
        "Form snapshot should contain form, textbox, or button, got: {yaml}"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
