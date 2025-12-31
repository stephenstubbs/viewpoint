#![cfg(feature = "integration")]

//! Vision deficiency emulation tests for viewpoint-core.
//!
//! These tests verify vision deficiency emulation functionality.

mod common;

use std::time::Duration;

use viewpoint_core::{
    Browser,
    page::VisionDeficiency,
};

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const MEDIA_QUERY_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        #media-type { color: black; }
        @media print { #media-type { color: red; } }
        @media screen { #media-type { color: blue; } }
        
        #color-scheme { color: black; }
        @media (prefers-color-scheme: dark) { #color-scheme { color: white; background: black; } }
        @media (prefers-color-scheme: light) { #color-scheme { color: black; background: white; } }
        
        #reduced-motion { animation: spin 1s infinite; }
        @media (prefers-reduced-motion: reduce) { #reduced-motion { animation: none; } }
        
        #forced-colors { color: black; }
        @media (forced-colors: active) { #forced-colors { color: CanvasText; } }
    </style>
</head>
<body>
    <div id="media-type">Media Type Test</div>
    <div id="color-scheme">Color Scheme Test</div>
    <div id="reduced-motion">Reduced Motion Test</div>
    <div id="forced-colors">Forced Colors Test</div>
</body>
</html>
"#;

// =============================================================================
// Vision Deficiency Tests
// =============================================================================

/// Test emulating deuteranopia (green-blind).
#[tokio::test]
async fn test_vision_deficiency_deuteranopia() {
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

    page.set_content(MEDIA_QUERY_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Emulate deuteranopia
    page.emulate_vision_deficiency(VisionDeficiency::Deuteranopia)
        .await
        .expect("Failed to emulate vision deficiency");

    browser.close().await.expect("Failed to close browser");
}

/// Test emulating protanopia (red-blind).
#[tokio::test]
async fn test_vision_deficiency_protanopia() {
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

    page.set_content(MEDIA_QUERY_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Emulate protanopia
    page.emulate_vision_deficiency(VisionDeficiency::Protanopia)
        .await
        .expect("Failed to emulate vision deficiency");

    browser.close().await.expect("Failed to close browser");
}

/// Test emulating tritanopia (blue-blind).
#[tokio::test]
async fn test_vision_deficiency_tritanopia() {
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

    page.set_content(MEDIA_QUERY_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Emulate tritanopia
    page.emulate_vision_deficiency(VisionDeficiency::Tritanopia)
        .await
        .expect("Failed to emulate vision deficiency");

    browser.close().await.expect("Failed to close browser");
}

/// Test emulating achromatopsia (complete color blindness).
#[tokio::test]
async fn test_vision_deficiency_achromatopsia() {
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

    page.set_content(MEDIA_QUERY_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Emulate achromatopsia
    page.emulate_vision_deficiency(VisionDeficiency::Achromatopsia)
        .await
        .expect("Failed to emulate vision deficiency");

    browser.close().await.expect("Failed to close browser");
}

/// Test clearing vision deficiency emulation.
#[tokio::test]
async fn test_vision_deficiency_clear() {
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

    page.set_content(MEDIA_QUERY_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Set a vision deficiency
    page.emulate_vision_deficiency(VisionDeficiency::Deuteranopia)
        .await
        .expect("Failed to emulate vision deficiency");

    // Clear it
    page.emulate_vision_deficiency(VisionDeficiency::None)
        .await
        .expect("Failed to clear vision deficiency");

    browser.close().await.expect("Failed to close browser");
}
