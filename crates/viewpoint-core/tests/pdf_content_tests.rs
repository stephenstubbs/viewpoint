#![cfg(feature = "integration")]

//! PDF content and output tests for viewpoint-core.
//!
//! These tests verify PDF generation content options including
//! headers, footers, page ranges, backgrounds, and file saving.

mod common;

use std::time::Duration;
use tempfile::TempDir;

use viewpoint_core::{Browser, Margins, PaperFormat};

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const SIMPLE_HTML: &str = r"
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
";

const MULTI_PAGE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Multi-Page Document</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .page { page-break-after: always; min-height: 100vh; }
        .page:last-child { page-break-after: auto; }
    </style>
</head>
<body>
    <div class="page"><h1>Page 1</h1><p>Content of first page.</p></div>
    <div class="page"><h1>Page 2</h1><p>Content of second page.</p></div>
    <div class="page"><h1>Page 3</h1><p>Content of third page.</p></div>
    <div class="page"><h1>Page 4</h1><p>Content of fourth page.</p></div>
    <div class="page"><h1>Page 5</h1><p>Content of fifth page.</p></div>
</body>
</html>
"#;

const BACKGROUND_HTML: &str = r"
<!DOCTYPE html>
<html>
<head>
    <title>Background Test</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            background-color: #f0f0f0;
            background-image: linear-gradient(to right, #f0f0f0, #e0e0e0);
        }
        h1 { color: #333; background-color: #yellow; padding: 10px; }
    </style>
</head>
<body>
    <h1>Background Graphics Test</h1>
    <p>This document has background colors and gradients.</p>
</body>
</html>
";

// =============================================================================
// Header and Footer Tests
// =============================================================================

/// Test generating a PDF with header template.
#[tokio::test]
async fn test_pdf_header_template() {
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

    page.set_content(SIMPLE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let header = r#"<div style="font-size: 10px; text-align: center; width: 100%;">Header - <span class="title"></span></div>"#;

    let pdf_data = page
        .pdf()
        .header_template(header)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with footer template.
#[tokio::test]
async fn test_pdf_footer_template() {
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

    page.set_content(SIMPLE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let footer = r#"<div style="font-size: 10px; text-align: center; width: 100%;">Page <span class="pageNumber"></span> of <span class="totalPages"></span></div>"#;

    let pdf_data = page
        .pdf()
        .footer_template(footer)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with both header and footer.
#[tokio::test]
async fn test_pdf_header_footer_combined() {
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

    page.set_content(SIMPLE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let header = r#"<div style="font-size: 10px;">Document Header</div>"#;
    let footer = r#"<div style="font-size: 10px;">Page <span class="pageNumber"></span></div>"#;

    let pdf_data = page
        .pdf()
        .header_template(header)
        .footer_template(footer)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Page Range Tests
// =============================================================================

/// Test generating a PDF with specific page ranges.
#[tokio::test]
async fn test_pdf_page_ranges() {
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

    page.set_content(MULTI_PAGE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Generate PDF with only pages 1-3
    let pdf_data = page
        .pdf()
        .page_ranges("1-3")
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF with non-contiguous page ranges.
#[tokio::test]
async fn test_pdf_page_ranges_non_contiguous() {
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

    page.set_content(MULTI_PAGE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Generate PDF with pages 1-2 and 5
    let pdf_data = page
        .pdf()
        .page_ranges("1-2, 5")
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Background Graphics Tests
// =============================================================================

/// Test generating a PDF with background graphics.
#[tokio::test]
async fn test_pdf_background_graphics() {
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

    page.set_content(BACKGROUND_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page
        .pdf()
        .print_background(true)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

/// Test generating a PDF without background graphics.
#[tokio::test]
async fn test_pdf_no_background_graphics() {
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

    page.set_content(BACKGROUND_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pdf_data = page
        .pdf()
        .print_background(false)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Save to File Tests
// =============================================================================

/// Test saving PDF to file.
#[tokio::test]
async fn test_pdf_save_to_file() {
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

    page.set_content(SIMPLE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Create a temp directory for the PDF
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let pdf_path = temp_dir.path().join("test-output.pdf");

    let pdf_data = page
        .pdf()
        .path(&pdf_path)
        .generate()
        .await
        .expect("Failed to generate PDF");

    // Verify data was returned
    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    // Verify file was created
    assert!(pdf_path.exists(), "PDF file should exist");

    // Verify file content
    let file_content = std::fs::read(&pdf_path).expect("Failed to read PDF file");
    assert_eq!(
        file_content, pdf_data,
        "File content should match returned data"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Combined Options Tests
// =============================================================================

/// Test generating a PDF with multiple options combined.
#[tokio::test]
async fn test_pdf_combined_options() {
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

    page.set_content(MULTI_PAGE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(200)).await;

    let header = r#"<div style="font-size: 10px;">My Document</div>"#;
    let footer = r#"<div style="font-size: 10px;">Page <span class="pageNumber"></span></div>"#;

    let pdf_data = page
        .pdf()
        .format(PaperFormat::A4)
        .landscape(true)
        .margins(Margins::new(1.0, 0.75, 1.0, 0.75))
        .print_background(true)
        .header_template(header)
        .footer_template(footer)
        .page_ranges("1-3")
        .scale(0.8)
        .generate()
        .await
        .expect("Failed to generate PDF");

    assert!(!pdf_data.is_empty());
    assert!(pdf_data.starts_with(b"%PDF-"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Error Handling Tests
// =============================================================================

/// Test PDF generation on closed page returns error.
#[tokio::test]
async fn test_pdf_on_closed_page() {
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
    let mut page = context.new_page().await.expect("Failed to create page");

    page.set_content(SIMPLE_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Close the page
    page.close().await.expect("Failed to close page");
    assert!(page.is_closed());

    // Try to generate PDF on closed page
    let result = page.pdf().generate().await;
    assert!(result.is_err(), "PDF generation on closed page should fail");

    browser.close().await.expect("Failed to close browser");
}
