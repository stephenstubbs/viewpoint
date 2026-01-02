#![cfg(feature = "integration")]

//! ARIA snapshot tests for semantic text elements.
//!
//! Tests for: blockquote, code, em, strong, del, ins, sub, sup, mark
//!
//! These tests verify that semantic text elements correctly appear in ARIA
//! snapshots with their appropriate roles and text content as accessible names.
//!
//! NOTE: Text content capture for these roles deviates from strict W3C ARIA 1.2
//! spec but follows Playwright's behavior for practical automation purposes.

mod common;

use common::launch_with_page;

// =============================================================================
// Blockquote Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_blockquote() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <blockquote>Famous quote here</blockquote>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Blockquote snapshot:\n{yaml}");

    assert!(
        yaml.contains("blockquote"),
        "Snapshot should contain 'blockquote' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Famous quote here"),
        "Snapshot should contain blockquote text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_blockquote_with_citation() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <blockquote cite="https://example.com">
                <p>To be or not to be</p>
            </blockquote>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Blockquote with citation snapshot:\n{yaml}");

    assert!(
        yaml.contains("blockquote"),
        "Snapshot should contain 'blockquote' role, got: {yaml}"
    );
    assert!(
        yaml.contains("To be or not to be"),
        "Snapshot should contain blockquote text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Code Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_code() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <code>console.log("hello")</code>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Code snapshot:\n{yaml}");

    assert!(
        yaml.contains("code"),
        "Snapshot should contain 'code' role, got: {yaml}"
    );
    assert!(
        yaml.contains("console.log"),
        "Snapshot should contain code text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_code_inline() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <p>Use the <code>npm install</code> command to install.</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Inline code snapshot:\n{yaml}");

    assert!(
        yaml.contains("code"),
        "Snapshot should contain 'code' role, got: {yaml}"
    );
    assert!(
        yaml.contains("npm install"),
        "Snapshot should contain inline code text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Emphasis (em) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_emphasis() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <em>Important text</em>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Emphasis snapshot:\n{yaml}");

    assert!(
        yaml.contains("emphasis"),
        "Snapshot should contain 'emphasis' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Important text"),
        "Snapshot should contain emphasis text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Strong Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_strong() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <strong>Bold statement</strong>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Strong snapshot:\n{yaml}");

    assert!(
        yaml.contains("strong"),
        "Snapshot should contain 'strong' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Bold statement"),
        "Snapshot should contain strong text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Deletion (del) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_deletion() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <del>Removed text</del>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Deletion snapshot:\n{yaml}");

    assert!(
        yaml.contains("deletion"),
        "Snapshot should contain 'deletion' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Removed text"),
        "Snapshot should contain deletion text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Insertion (ins) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_insertion() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <ins>Added text</ins>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Insertion snapshot:\n{yaml}");

    assert!(
        yaml.contains("insertion"),
        "Snapshot should contain 'insertion' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Added text"),
        "Snapshot should contain insertion text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Subscript (sub) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_subscript() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            H<sub>2</sub>O
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Subscript snapshot:\n{yaml}");

    assert!(
        yaml.contains("subscript"),
        "Snapshot should contain 'subscript' role, got: {yaml}"
    );
    assert!(
        yaml.contains('2'),
        "Snapshot should contain subscript text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Superscript (sup) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_superscript() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            E=mc<sup>2</sup>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Superscript snapshot:\n{yaml}");

    assert!(
        yaml.contains("superscript"),
        "Snapshot should contain 'superscript' role, got: {yaml}"
    );
    assert!(
        yaml.contains('2'),
        "Snapshot should contain superscript text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Mark Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_mark() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            Search results: <mark>highlighted term</mark>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Mark snapshot:\n{yaml}");

    assert!(
        yaml.contains("mark"),
        "Snapshot should contain 'mark' role, got: {yaml}"
    );
    assert!(
        yaml.contains("highlighted term"),
        "Snapshot should contain mark text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Combined Semantic Text Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_mixed_semantic_text() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <p>This is <strong>bold</strong> and <em>italic</em> text.</p>
            <blockquote>A famous quote with <code>code</code> inside.</blockquote>
            <p>Price was <del>$100</del> now <ins>$50</ins>!</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Mixed semantic text snapshot:\n{yaml}");

    // Verify all semantic text elements are captured
    assert!(yaml.contains("strong"), "Should contain strong");
    assert!(yaml.contains("emphasis"), "Should contain emphasis");
    assert!(yaml.contains("blockquote"), "Should contain blockquote");
    assert!(yaml.contains("code"), "Should contain code");
    assert!(yaml.contains("deletion"), "Should contain deletion");
    assert!(yaml.contains("insertion"), "Should contain insertion");

    browser.close().await.expect("Failed to close browser");
}
