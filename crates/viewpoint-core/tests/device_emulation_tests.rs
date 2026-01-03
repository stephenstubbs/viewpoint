#![cfg(feature = "integration")]

//! Device emulation tests for viewpoint-core.
//!
//! These tests verify device emulation functionality including
//! device descriptors, viewport, locale, timezone, touch, and mobile mode.

mod common;

use std::time::Duration;

use viewpoint_core::{Browser, devices};
use viewpoint_js::js;

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const VIEWPORT_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <div id="viewport-info"></div>
    <script>
        function updateInfo() {
            document.getElementById('viewport-info').textContent = 
                window.innerWidth + 'x' + window.innerHeight;
        }
        window.addEventListener('resize', updateInfo);
        updateInfo();
    </script>
</body>
</html>
"#;

// =============================================================================
// Viewport Tests
// =============================================================================

/// Test setting viewport size.
#[tokio::test]
async fn test_viewport_size() {
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

    page.set_content(VIEWPORT_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Set viewport size
    page.set_viewport_size(1280, 720)
        .await
        .expect("Failed to set viewport size");

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify viewport size
    let width: i32 = page
        .evaluate(js! { window.innerWidth })
        .await
        .expect("Failed to get width");

    let height: i32 = page
        .evaluate(js! { window.innerHeight })
        .await
        .expect("Failed to get height");

    assert_eq!(width, 1280, "Viewport width should be 1280");
    assert_eq!(height, 720, "Viewport height should be 720");

    browser.close().await.expect("Failed to close browser");
}

/// Test setting different viewport sizes.
#[tokio::test]
async fn test_viewport_size_mobile() {
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

    page.set_content(VIEWPORT_HTML)
        .set()
        .await
        .expect("Failed to set content");

    // Set mobile viewport size
    page.set_viewport_size(375, 812)
        .await
        .expect("Failed to set viewport size");

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify viewport size
    let width: i32 = page
        .evaluate(js! { window.innerWidth })
        .await
        .expect("Failed to get width");

    assert_eq!(width, 375, "Viewport width should be 375");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Device Descriptor Tests
// =============================================================================

/// Test iPhone device descriptor availability.
#[tokio::test]
#[allow(clippy::assertions_on_constants)] // Testing constant device descriptor properties
async fn test_device_iphone_available() {
    init_tracing();

    // Verify device descriptors exist
    assert_eq!(devices::IPHONE_13.name, "iPhone 13");
    assert!(devices::IPHONE_13.is_mobile);
    assert!(devices::IPHONE_13.has_touch);
    assert!(devices::IPHONE_13.device_scale_factor > 1.0);
}

/// Test Pixel device descriptor availability.
#[tokio::test]
#[allow(clippy::assertions_on_constants)] // Testing constant device descriptor properties
async fn test_device_pixel_available() {
    init_tracing();

    // Verify device descriptors exist
    assert_eq!(devices::PIXEL_7.name, "Pixel 7");
    assert!(devices::PIXEL_7.is_mobile);
    assert!(devices::PIXEL_7.has_touch);
}

/// Test listing all available devices.
#[tokio::test]
async fn test_device_list_all() {
    init_tracing();

    let all = devices::all_devices();

    // Should have many devices
    assert!(
        all.len() > 20,
        "Should have more than 20 device descriptors"
    );

    // Should include various device types
    let names: Vec<_> = all.iter().map(|d| d.name).collect();
    assert!(names.iter().any(|n| n.contains("iPhone")));
    assert!(names.iter().any(|n| n.contains("iPad")));
    assert!(names.iter().any(|n| n.contains("Pixel")));
    assert!(names.iter().any(|n| n.contains("Galaxy")));
    assert!(names.iter().any(|n| n.contains("Desktop")));
}

/// Test finding device by name.
#[tokio::test]
async fn test_device_find_by_name() {
    init_tracing();

    let device = devices::find_device("iPhone 13");
    assert!(device.is_some());
    assert_eq!(device.unwrap().name, "iPhone 13");

    // Case insensitive
    let device_lower = devices::find_device("iphone 13");
    assert!(device_lower.is_some());
}
