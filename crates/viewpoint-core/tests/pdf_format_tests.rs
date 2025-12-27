#![cfg(feature = "integration")]

//! PDF format and options tests for viewpoint-core.
//!
//! These tests verify PDF generation format options including
//! paper size, orientation, margins, and scale.

mod common;

use std::time::Duration;

use viewpoint_core::{Browser, Margins, PaperFormat};

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const SIMPLE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Test PDF Document</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        p { line-height: 1.6; }
    </style>
</head>
<body>
    <h1>PDF Generation Test</h1>
    <p>This is a test document for PDF generation.</p>
    <p>It contains multiple paragraphs of text to test formatting.</p>
</body>
</html>
"#;

// =============================================================================
// Default PDF Generation Tests
// =============================================================================

/// Test generating a default PDF (Letter format).
#[tokio::test]
async fn test_pdf_default() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");

    // Wait for content to render
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf().generate().await.expect("Failed to generate PDF");

    // Verify PDF was generated
    assert!(!pdf_data.is_empty(), "PDF data should not be empty");
    
    // Check PDF magic bytes (%PDF-)
    assert!(pdf_data.starts_with(b"%PDF-"), "PDF should start with %PDF- header");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Paper Size Tests
// =============================================================================

/// Test generating a PDF with A4 paper size.
#[tokio::test]
async fn test_pdf_paper_size_a4() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .format(PaperFormat::A4)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with Letter paper size.
#[tokio::test]
async fn test_pdf_paper_size_letter() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .format(PaperFormat::Letter)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with Legal paper size.
#[tokio::test]
async fn test_pdf_paper_size_legal() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .format(PaperFormat::Legal)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with custom paper size.
#[tokio::test]
async fn test_pdf_paper_size_custom() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .format(PaperFormat::Custom { width: 5.0, height: 7.0 })
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Orientation Tests
// =============================================================================

/// Test generating a PDF with landscape orientation.
#[tokio::test]
async fn test_pdf_landscape() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .landscape(true)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with portrait orientation (default).
#[tokio::test]
async fn test_pdf_portrait() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .landscape(false)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Margin Tests
// =============================================================================

/// Test generating a PDF with custom margins.
#[tokio::test]
async fn test_pdf_margins() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .margins(Margins::new(1.0, 0.5, 1.0, 0.5))
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with uniform margins.
#[tokio::test]
async fn test_pdf_margin_uniform() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .margin(0.5)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with zero margins.
#[tokio::test]
async fn test_pdf_margin_zero() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .margin(0.0)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Scale Tests
// =============================================================================

/// Test generating a PDF with custom scale.
#[tokio::test]
async fn test_pdf_scale() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML).set().await.expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page.pdf()
        .scale(0.5)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}
