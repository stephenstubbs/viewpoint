#![cfg(feature = "integration")]

//! Media emulation tests for viewpoint-core.
//!
//! These tests verify media emulation functionality including media type,
//! color scheme, reduced motion, forced colors, and vision deficiency.

mod common;

use std::time::Duration;

use viewpoint_core::{Browser, ColorScheme, ForcedColors, ReducedMotion, page::MediaType};
use viewpoint_js::js;

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
// Media Type Tests
// =============================================================================

/// Test emulating print media type.
#[tokio::test]
async fn test_media_type_print() {
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

    // Emulate print media
    page.emulate_media()
        .media(MediaType::Print)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if print media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("print").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Print media query should match");

    browser.close().await.expect("Failed to close browser");
}

/// Test emulating screen media type.
#[tokio::test]
async fn test_media_type_screen() {
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

    // Emulate screen media
    page.emulate_media()
        .media(MediaType::Screen)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if screen media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("screen").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Screen media query should match");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Color Scheme Tests
// =============================================================================

/// Test emulating dark color scheme.
#[tokio::test]
async fn test_color_scheme_dark() {
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

    // Emulate dark color scheme
    page.emulate_media()
        .color_scheme(ColorScheme::Dark)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if dark color scheme media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-color-scheme: dark)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Dark color scheme media query should match");

    browser.close().await.expect("Failed to close browser");
}

/// Test emulating light color scheme.
#[tokio::test]
async fn test_color_scheme_light() {
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

    // Emulate light color scheme
    page.emulate_media()
        .color_scheme(ColorScheme::Light)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if light color scheme media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-color-scheme: light)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Light color scheme media query should match");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Reduced Motion Tests
// =============================================================================

/// Test emulating reduced motion preference.
#[tokio::test]
async fn test_reduced_motion() {
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

    // Emulate reduced motion
    page.emulate_media()
        .reduced_motion(ReducedMotion::Reduce)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if reduced motion media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-reduced-motion: reduce)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Reduced motion media query should match");

    browser.close().await.expect("Failed to close browser");
}

/// Test no motion preference.
#[tokio::test]
async fn test_no_motion_preference() {
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

    // Emulate no motion preference
    page.emulate_media()
        .reduced_motion(ReducedMotion::NoPreference)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if no-preference matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-reduced-motion: no-preference)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "No preference motion media query should match");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Forced Colors Tests
// =============================================================================

/// Test emulating forced colors mode.
#[tokio::test]
async fn test_forced_colors() {
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

    // Emulate forced colors
    page.emulate_media()
        .forced_colors(ForcedColors::Active)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check if forced colors media query matches
    let matches: bool = page
        .evaluate(js! { window.matchMedia("(forced-colors: active)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(matches, "Forced colors media query should match");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Combined Media Emulation Tests
// =============================================================================

/// Test combining multiple media emulation settings.
#[tokio::test]
async fn test_media_combined() {
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

    // Emulate multiple settings at once
    page.emulate_media()
        .color_scheme(ColorScheme::Dark)
        .reduced_motion(ReducedMotion::Reduce)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Check both media queries match
    let dark_matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-color-scheme: dark)").matches })
        .await
        .expect("Failed to evaluate");

    let motion_matches: bool = page
        .evaluate(js! { window.matchMedia("(prefers-reduced-motion: reduce)").matches })
        .await
        .expect("Failed to evaluate");

    assert!(dark_matches, "Dark color scheme should match");
    assert!(motion_matches, "Reduced motion should match");

    browser.close().await.expect("Failed to close browser");
}

/// Test clearing media emulation.
#[tokio::test]
async fn test_media_clear() {
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

    // First, set dark mode
    page.emulate_media()
        .color_scheme(ColorScheme::Dark)
        .apply()
        .await
        .expect("Failed to emulate media");

    // Verify it's set
    let dark_before: bool = page
        .evaluate(js! { window.matchMedia("(prefers-color-scheme: dark)").matches })
        .await
        .expect("Failed to evaluate");
    assert!(dark_before, "Dark mode should be set");

    // Clear media emulation
    page.emulate_media()
        .clear()
        .await
        .expect("Failed to clear media");

    browser.close().await.expect("Failed to close browser");
}
