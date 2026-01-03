#![cfg(feature = "integration")]

//! Basic ARIA snapshot ref tests.
//!
//! These tests verify element refs are generated for interactive elements.

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

    // Verify refs are present for buttons (format: [ref=c{ctx}p{page}e{counter}])
    assert!(
        yaml.contains("[ref=c") && yaml.contains('p') && yaml.contains('e'),
        "Snapshot should contain refs for buttons in format c{{ctx}}p{{page}}e{{counter}}, got: {yaml}"
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

    // Verify refs are present for headings (format: [ref=c{ctx}p{page}e{counter}])
    assert!(
        yaml.contains("[ref=c") && yaml.contains('p') && yaml.contains('e'),
        "Snapshot should contain refs for headings in format c{{ctx}}p{{page}}e{{counter}}, got: {yaml}"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
