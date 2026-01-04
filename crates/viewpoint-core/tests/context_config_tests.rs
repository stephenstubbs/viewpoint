#![cfg(feature = "integration")]

//! Context configuration tests.
//!
//! Tests for browser context configuration including timeouts, headers,
//! offline mode, emulation settings, and test ID attributes.

mod common;

use std::collections::HashMap;
use std::time::Duration;

use viewpoint_core::Permission;

/// Test setting offline mode.
#[tokio::test]
async fn test_context_offline_mode() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Go offline
    context
        .set_offline(true)
        .await
        .expect("Failed to go offline");

    // Go back online
    context
        .set_offline(false)
        .await
        .expect("Failed to go online");

    browser.close().await.expect("Failed to close browser");
}

/// Test setting extra HTTP headers.
#[tokio::test]
async fn test_context_extra_http_headers() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Set extra headers
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "test-value".to_string());
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());

    context
        .set_extra_http_headers(headers)
        .await
        .expect("Failed to set extra headers");

    browser.close().await.expect("Failed to close browser");
}

/// Test default timeout configuration.
#[tokio::test]
async fn test_context_timeout_configuration() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let mut context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Get default timeouts
    let default = context.default_timeout();
    assert_eq!(default, Duration::from_secs(30));

    // Set custom timeouts
    context.set_default_timeout(Duration::from_secs(60));
    context.set_default_navigation_timeout(Duration::from_secs(120));

    assert_eq!(context.default_timeout(), Duration::from_secs(60));
    assert_eq!(
        context.default_navigation_timeout(),
        Duration::from_secs(120)
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test creating context with builder/options.
#[tokio::test]
async fn test_context_with_options() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with options
    let context = browser
        .new_context_builder()
        .geolocation(40.7128, -74.0060) // New York
        .permissions(vec![Permission::Geolocation])
        .has_touch(true)
        .locale("en-US")
        .timezone_id("America/New_York")
        .default_timeout(Duration::from_secs(45))
        .build()
        .await
        .expect("Failed to create context with options");

    // Verify timeout was set
    assert_eq!(context.default_timeout(), Duration::from_secs(45));

    browser.close().await.expect("Failed to close browser");
}

/// Test custom test ID attribute.
#[tokio::test]
async fn test_custom_test_id_attribute() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Set custom test ID attribute
    context.set_test_id_attribute("data-test").await;

    // Verify it was set
    let attr = context.test_id_attribute().await;
    assert_eq!(attr, "data-test");

    browser.close().await.expect("Failed to close browser");
}

/// Test default test ID attribute.
#[tokio::test]
async fn test_default_test_id_attribute() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Check default test ID attribute
    let attr = context.test_id_attribute().await;
    assert_eq!(attr, "data-testid");

    browser.close().await.expect("Failed to close browser");
}

/// Test timezone emulation.
#[tokio::test]
async fn test_timezone_emulation() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with timezone
    let _context = browser
        .new_context_builder()
        .timezone_id("Europe/London")
        .build()
        .await
        .expect("Failed to create context with timezone");

    browser.close().await.expect("Failed to close browser");
}

/// Test locale emulation.
#[tokio::test]
async fn test_locale_emulation() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with locale
    let _context = browser
        .new_context_builder()
        .locale("fr-FR")
        .build()
        .await
        .expect("Failed to create context with locale");

    browser.close().await.expect("Failed to close browser");
}
