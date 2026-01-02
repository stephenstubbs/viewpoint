#![cfg(feature = "integration")]

//! ARIA snapshot tests for definition list elements.
//!
//! Tests for: dl, dt, dd, dfn
//!
//! These tests verify that definition list elements correctly appear in ARIA
//! snapshots with their appropriate roles (term, definition) and text content.

mod common;

use common::launch_with_page;

// =============================================================================
// Definition List Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_definition_list() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <dl>
                <dt>HTML</dt>
                <dd>HyperText Markup Language</dd>
                <dt>CSS</dt>
                <dd>Cascading Style Sheets</dd>
            </dl>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Definition list snapshot:\n{yaml}");

    // dt elements should have "term" role
    assert!(
        yaml.contains("term"),
        "Snapshot should contain 'term' role for dt elements, got: {yaml}"
    );
    // dd elements should have "definition" role
    assert!(
        yaml.contains("definition"),
        "Snapshot should contain 'definition' role for dd elements, got: {yaml}"
    );
    // Content should be captured
    assert!(
        yaml.contains("HTML"),
        "Snapshot should contain dt text content, got: {yaml}"
    );
    assert!(
        yaml.contains("HyperText Markup Language"),
        "Snapshot should contain dd text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_dt_term() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <dl>
                <dt>API</dt>
                <dd>Application Programming Interface</dd>
            </dl>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("DT term snapshot:\n{yaml}");

    assert!(
        yaml.contains("term"),
        "Snapshot should contain 'term' role, got: {yaml}"
    );
    assert!(
        yaml.contains("API"),
        "Snapshot should contain term text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_dd_definition() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <dl>
                <dt>REST</dt>
                <dd>Representational State Transfer</dd>
            </dl>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("DD definition snapshot:\n{yaml}");

    assert!(
        yaml.contains("definition"),
        "Snapshot should contain 'definition' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Representational State Transfer"),
        "Snapshot should contain definition text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Dfn Element Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_dfn_term() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <p><dfn>API</dfn> stands for Application Programming Interface.</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Dfn term snapshot:\n{yaml}");

    // dfn should have "term" role
    assert!(
        yaml.contains("term"),
        "Snapshot should contain 'term' role for dfn, got: {yaml}"
    );
    assert!(
        yaml.contains("API"),
        "Snapshot should contain dfn text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_dfn_with_abbreviation() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <p>The <dfn><abbr title="World Wide Web">WWW</abbr></dfn> was invented by Tim Berners-Lee.</p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Dfn with abbreviation snapshot:\n{yaml}");

    assert!(
        yaml.contains("term"),
        "Snapshot should contain 'term' role, got: {yaml}"
    );
    assert!(
        yaml.contains("WWW"),
        "Snapshot should contain dfn text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Complex Definition List Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_definition_list_multiple_dds() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <dl>
                <dt>Firefox</dt>
                <dd>A free, open source, cross-platform web browser</dd>
                <dd>Developed by Mozilla Foundation</dd>
            </dl>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Definition list with multiple dds snapshot:\n{yaml}");

    assert!(
        yaml.contains("Firefox"),
        "Snapshot should contain term, got: {yaml}"
    );
    assert!(
        yaml.contains("free, open source"),
        "Snapshot should contain first definition, got: {yaml}"
    );
    assert!(
        yaml.contains("Mozilla Foundation"),
        "Snapshot should contain second definition, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}
