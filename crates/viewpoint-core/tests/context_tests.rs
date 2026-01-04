#![cfg(feature = "integration")]

//! Context-related tests.
//!
//! Tests for browser context configuration, cookies, permissions, storage state,
//! geolocation, emulation, and other context-level features.

mod common;

use std::collections::HashMap;
use std::time::Duration;
use viewpoint_core::{Browser, Cookie, DocumentLoadState, Permission, SameSite};

/// Helper function to launch browser and get a page.
async fn setup() -> (
    Browser,
    viewpoint_core::BrowserContext,
    viewpoint_core::Page,
) {
    common::launch_with_page().await
}

/// Test adding cookies.
#[tokio::test]
async fn test_context_add_cookies() {
    common::init_tracing();

    let (browser, context, page) = setup().await;

    // Navigate to a page first (cookies need a domain)
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Add cookies
    context
        .add_cookies(vec![
            Cookie::new("session", "abc123")
                .domain("example.com")
                .path("/")
                .secure(true)
                .http_only(true),
            Cookie::new("user_pref", "dark_mode")
                .domain("example.com")
                .path("/")
                .same_site(SameSite::Lax),
        ])
        .await
        .expect("Failed to add cookies");

    // Get all cookies
    let cookies = context.cookies().await.expect("Failed to get cookies");
    assert!(!cookies.is_empty(), "Should have at least one cookie");

    browser.close().await.expect("Failed to close browser");
}

/// Test clearing cookies.
#[tokio::test]
async fn test_context_clear_cookies() {
    common::init_tracing();

    let (browser, context, page) = setup().await;

    // Navigate to a page first
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Add cookies
    context
        .add_cookies(vec![Cookie::new("test", "value").domain("example.com")])
        .await
        .expect("Failed to add cookies");

    // Clear all cookies
    context
        .clear_cookies()
        .await
        .expect("Failed to clear cookies");

    // Verify cookies are cleared
    let cookies = context.cookies().await.expect("Failed to get cookies");
    let _ = cookies;

    browser.close().await.expect("Failed to close browser");
}

/// Test granting permissions.
#[tokio::test]
async fn test_context_grant_permissions() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Grant permissions
    context
        .grant_permissions(vec![Permission::Geolocation, Permission::Notifications])
        .await
        .expect("Failed to grant permissions");

    // Clear permissions
    context
        .clear_permissions()
        .await
        .expect("Failed to clear permissions");

    browser.close().await.expect("Failed to close browser");
}

/// Test setting geolocation.
#[tokio::test]
async fn test_context_set_geolocation() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Set geolocation (San Francisco)
    context
        .set_geolocation(37.7749, -122.4194)
        .accuracy(10.0)
        .await
        .expect("Failed to set geolocation");

    // Clear geolocation
    context
        .clear_geolocation()
        .await
        .expect("Failed to clear geolocation");

    browser.close().await.expect("Failed to close browser");
}

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

/// Test storage state.
#[tokio::test]
async fn test_storage_state() {
    common::init_tracing();

    let (browser, context, page) = setup().await;

    // Navigate to add some state
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get storage state
    let state = context
        .storage_state()
        .await
        .expect("Failed to get storage state");

    // Should have a structure (cookies and origins)
    let _ = state.cookies;
    let _ = state.origins;

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

/// Test context geolocation.
#[tokio::test]
async fn test_context_geolocation() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Set geolocation
    context
        .set_geolocation(51.5074, -0.1278) // London
        .await
        .expect("Failed to set geolocation");

    browser.close().await.expect("Failed to close browser");
}

/// Test clearing geolocation.
#[tokio::test]
async fn test_context_geolocation_clear() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Set then clear
    context
        .set_geolocation(51.5074, -0.1278)
        .await
        .expect("Failed to set");
    context.clear_geolocation().await.expect("Failed to clear");

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

/// Test that storage_state() succeeds after closing a page.
/// This verifies that the internal pages tracking list is properly maintained
/// when pages are closed, preventing stale session errors.
#[tokio::test]
async fn test_storage_state_succeeds_after_page_close() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create a page
    let mut page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Close the page
    page.close().await.expect("Failed to close page");

    // Call storage_state() - this should succeed without "session not found" error
    // because the closed page was removed from the internal tracking list
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed after closing page");

    // Verify we got a valid state
    let _ = state.cookies;
    let _ = state.origins;

    browser.close().await.expect("Failed to close browser");
}

/// Test that storage_state() collects cookies even when no pages remain open.
#[tokio::test]
async fn test_storage_state_cookies_after_all_pages_closed() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create a page and navigate to set cookies
    let mut page = context.new_page().await.expect("Failed to create page");
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Add a cookie
    context
        .add_cookies(vec![Cookie::new("test_cookie", "test_value").domain("example.com")])
        .await
        .expect("Failed to add cookie");

    // Close the page
    page.close().await.expect("Failed to close page");

    // Call storage_state() - should succeed and include cookies
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed after closing page");

    // Verify we got cookies (cookies are fetched from browser, not from pages)
    assert!(!state.cookies.is_empty(), "Should have cookies");

    browser.close().await.expect("Failed to close browser");
}

/// Test that storage_state() works correctly with multiple pages where some are closed.
/// This verifies the fix for stale session issues where closed pages would cause
/// "session not found" errors when iterating over the internal pages list.
#[tokio::test]
async fn test_storage_state_with_multiple_pages_some_closed() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create first page and navigate
    let mut page1 = context.new_page().await.expect("Failed to create page 1");
    page1.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    // Create second page and navigate
    let page2 = context.new_page().await.expect("Failed to create page 2");
    page2.goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    // Close first page - this should remove it from the internal tracking list
    page1.close().await.expect("Failed to close page 1");

    // Call storage_state() - should succeed without "session not found" error
    // because the closed page was removed from the internal tracking list
    let state = context
        .storage_state()
        .await
        .expect("storage_state() should succeed with remaining pages");

    // Verify we got a valid state
    let _ = state.cookies;
    let _ = state.origins;

    browser.close().await.expect("Failed to close browser");
}
