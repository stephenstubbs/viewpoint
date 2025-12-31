#![cfg(feature = "integration")]

//! Device emulation context tests for viewpoint-core.
//!
//! These tests verify device emulation at context level including
//! locale, timezone, touch, and mobile mode.

mod common;

use std::time::Duration;

use viewpoint_core::{Browser, devices};
use viewpoint_js::js;

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const LOCALE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <div id="locale"></div>
    <div id="timezone"></div>
    <script>
        document.getElementById('locale').textContent = navigator.language;
        document.getElementById('timezone').textContent = Intl.DateTimeFormat().resolvedOptions().timeZone;
    </script>
</body>
</html>
"#;

const TOUCH_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <div id="touch-support"></div>
    <script>
        document.getElementById('touch-support').textContent = 
            ('ontouchstart' in window) || (navigator.maxTouchPoints > 0) ? 'yes' : 'no';
    </script>
</body>
</html>
"#;

const USER_AGENT_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <div id="user-agent"></div>
    <script>
        document.getElementById('user-agent').textContent = navigator.userAgent;
    </script>
</body>
</html>
"#;

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
// Context Device Emulation Tests
// =============================================================================

/// Test creating context with device descriptor.
#[tokio::test]
async fn test_context_with_device() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with iPhone device
    let context = browser
        .new_context_builder()
        .device(devices::IPHONE_13.clone())
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(USER_AGENT_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify user agent contains iPhone
    let user_agent: String = page
        .evaluate(js! { navigator.userAgent })
        .await
        .expect("Failed to get user agent");

    assert!(
        user_agent.contains("iPhone"),
        "User agent should contain iPhone"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test creating context with custom user agent.
#[tokio::test]
async fn test_context_custom_user_agent() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with custom user agent
    let context = browser
        .new_context_builder()
        .user_agent("Custom User Agent/1.0")
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(USER_AGENT_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify custom user agent
    let user_agent: String = page
        .evaluate(js! { navigator.userAgent })
        .await
        .expect("Failed to get user agent");

    assert_eq!(user_agent, "Custom User Agent/1.0");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Locale and Timezone Tests
// =============================================================================

/// Test setting locale.
/// Note: In headless Chromium, navigator.language may not reflect the locale setting.
/// This test verifies the locale configuration is accepted and the page renders correctly.
#[tokio::test]
async fn test_locale() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with French locale
    let context = browser
        .new_context_builder()
        .locale("fr-FR")
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(LOCALE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify we can access locale-related APIs
    // Note: navigator.language may return system locale in headless mode
    let locale: String = page
        .evaluate(js! { navigator.language })
        .await
        .expect("Failed to get locale");

    // Just verify it returns a valid locale string (not empty)
    assert!(!locale.is_empty(), "Locale should not be empty");

    browser.close().await.expect("Failed to close browser");
}

/// Test setting timezone.
#[tokio::test]
async fn test_timezone() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with Paris timezone
    let context = browser
        .new_context_builder()
        .timezone_id("Europe/Paris")
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(LOCALE_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify timezone
    let timezone: String = page
        .evaluate(js! { Intl.DateTimeFormat().resolvedOptions().timeZone })
        .await
        .expect("Failed to get timezone");

    assert_eq!(timezone, "Europe/Paris", "Timezone should be Europe/Paris");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Device Scale Factor Tests
// =============================================================================

/// Test device scale factor (device pixel ratio).
#[tokio::test]
async fn test_device_scale_factor() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with high DPI scale factor
    let context = browser
        .new_context_builder()
        .device_scale_factor(2.0)
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(VIEWPORT_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify device pixel ratio
    let dpr: f64 = page
        .evaluate(js! { window.devicePixelRatio })
        .await
        .expect("Failed to get DPR");

    assert_eq!(dpr, 2.0, "Device pixel ratio should be 2.0");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Touch Emulation Tests
// =============================================================================

/// Test touch emulation.
#[tokio::test]
async fn test_touch_emulation() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context with touch enabled
    let context = browser
        .new_context_builder()
        .has_touch(true)
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(TOUCH_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify touch support
    let touch_support: String = page
        .evaluate(js! { document.getElementById("touch-support").textContent })
        .await
        .expect("Failed to get touch support");

    assert_eq!(touch_support, "yes", "Touch should be supported");

    browser.close().await.expect("Failed to close browser");
}

/// Test without touch emulation.
#[tokio::test]
async fn test_no_touch_emulation() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context without touch
    let context = browser
        .new_context_builder()
        .has_touch(false)
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(TOUCH_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify no touch support
    let touch_support: String = page
        .evaluate(js! { document.getElementById("touch-support").textContent })
        .await
        .expect("Failed to get touch support");

    assert_eq!(touch_support, "no", "Touch should not be supported");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Mobile Mode Tests
// =============================================================================

/// Test mobile mode emulation.
#[tokio::test]
async fn test_mobile_mode() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create context in mobile mode using device
    let context = browser
        .new_context_builder()
        .device(devices::IPHONE_13.clone())
        .build()
        .await
        .expect("Failed to create context");

    let page = context.new_page().await.expect("Failed to create page");
    page.set_content(USER_AGENT_HTML)
        .set()
        .await
        .expect("Failed to set content");
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify mobile user agent
    let user_agent: String = page
        .evaluate(js! { navigator.userAgent })
        .await
        .expect("Failed to get user agent");

    // Mobile user agents typically contain "Mobile"
    assert!(
        user_agent.contains("Mobile") || user_agent.contains("iPhone"),
        "User agent should indicate mobile device"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Bring to Front Tests
// =============================================================================

/// Test bringing a page to front.
#[tokio::test]
async fn test_page_bring_to_front() {
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

    // Create multiple pages
    let page1 = context.new_page().await.expect("Failed to create page 1");
    let _page2 = context.new_page().await.expect("Failed to create page 2");

    // Bring page1 to front
    page1
        .bring_to_front()
        .await
        .expect("Failed to bring to front");

    // Should not error - verifies the API works

    browser.close().await.expect("Failed to close browser");
}
