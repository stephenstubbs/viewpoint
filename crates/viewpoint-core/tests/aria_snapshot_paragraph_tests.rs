#![cfg(feature = "integration")]

//! ARIA snapshot tests for paragraph text capture.
//!
//! These tests verify that <p> elements correctly appear in ARIA snapshots
//! with their text content as the accessible name.
//!
//! NOTE: This deviates from strict W3C ARIA 1.2 spec which marks paragraph
//! as "name prohibited", but is necessary for automation/testing purposes.

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
// Paragraph Text Capture Tests
// =============================================================================

/// Test that simple paragraph elements capture their text content.
#[tokio::test]
async fn test_aria_snapshot_paragraph_text_capture() {
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
            <p>This is a paragraph with text content.</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Paragraph snapshot:\n{yaml}");

    // Paragraph should appear with role "paragraph" and text content as name
    assert!(
        yaml.contains("paragraph"),
        "Snapshot should contain 'paragraph' role, got: {yaml}"
    );
    assert!(
        yaml.contains("This is a paragraph with text content."),
        "Snapshot should contain paragraph text content, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test multiple paragraphs with different content.
#[tokio::test]
async fn test_aria_snapshot_multiple_paragraphs() {
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
            <p>First paragraph content</p>
            <p>Second paragraph content</p>
            <p>Third paragraph content</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Multiple paragraphs snapshot:\n{yaml}");

    // All paragraphs should have their text captured
    assert!(
        yaml.contains("First paragraph content"),
        "Snapshot should contain first paragraph text, got: {yaml}"
    );
    assert!(
        yaml.contains("Second paragraph content"),
        "Snapshot should contain second paragraph text, got: {yaml}"
    );
    assert!(
        yaml.contains("Third paragraph content"),
        "Snapshot should contain third paragraph text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test paragraphs mixed with other elements (realistic page structure).
#[tokio::test]
async fn test_aria_snapshot_paragraphs_mixed_content() {
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
            <h1>Page Title</h1>
            <p>Introduction paragraph explaining the page.</p>
            <button>Click Me</button>
            <p>Score: 82</p>
            <p class="description">Another descriptive paragraph.</p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Mixed content snapshot:\n{yaml}");

    // Verify heading, button, and paragraphs are all captured
    assert!(
        yaml.contains("heading"),
        "Snapshot should contain heading, got: {yaml}"
    );
    assert!(
        yaml.contains("button"),
        "Snapshot should contain button, got: {yaml}"
    );
    assert!(
        yaml.contains("paragraph"),
        "Snapshot should contain paragraph role, got: {yaml}"
    );
    // Critical: Score text that was missing before should now appear
    assert!(
        yaml.contains("Score: 82"),
        "Snapshot should contain 'Score: 82' from paragraph, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test paragraph with styled/nested content still captures text.
#[tokio::test]
async fn test_aria_snapshot_paragraph_with_inline_elements() {
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
            <p>Text with <strong>bold</strong> and <em>italic</em> content.</p>
            <p><span class="highlight">Highlighted</span> text in paragraph.</p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Paragraph with inline elements snapshot:\n{yaml}");

    // Text content should be captured even with inline elements
    assert!(
        yaml.contains("paragraph"),
        "Snapshot should contain paragraph role, got: {yaml}"
    );
    // The full text content should be captured (inline elements merged)
    assert!(
        yaml.contains("bold") && yaml.contains("italic"),
        "Snapshot should contain text from inline elements, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test paragraph inside other semantic elements.
#[tokio::test]
async fn test_aria_snapshot_paragraph_in_article() {
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
            <article>
                <h2>Article Title</h2>
                <p>Article introduction paragraph.</p>
                <section>
                    <p>Section content paragraph.</p>
                </section>
            </article>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Paragraph in article snapshot:\n{yaml}");

    // Both article structure and paragraph content should be captured
    assert!(
        yaml.contains("article"),
        "Snapshot should contain article, got: {yaml}"
    );
    assert!(
        yaml.contains("Article introduction paragraph."),
        "Snapshot should contain article paragraph text, got: {yaml}"
    );
    assert!(
        yaml.contains("Section content paragraph."),
        "Snapshot should contain section paragraph text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test empty paragraphs don't cause issues.
#[tokio::test]
async fn test_aria_snapshot_empty_paragraph() {
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
            <p></p>
            <p>Non-empty paragraph</p>
            <p>   </p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Empty paragraph snapshot:\n{yaml}");

    // Non-empty paragraph should still be captured
    assert!(
        yaml.contains("Non-empty paragraph"),
        "Snapshot should contain non-empty paragraph text, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test paragraph with aria-label override.
#[tokio::test]
async fn test_aria_snapshot_paragraph_aria_label() {
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
            <p aria-label="Custom label">Text content here</p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Paragraph with aria-label snapshot:\n{yaml}");

    // aria-label should take precedence over text content
    assert!(
        yaml.contains("Custom label"),
        "Snapshot should use aria-label, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}
