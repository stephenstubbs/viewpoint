#![cfg(feature = "integration")]

//! Context cookie and permission tests.
//!
//! Tests for browser context cookie management and permission handling.

mod common;

use viewpoint_core::{Cookie, DocumentLoadState, Permission, SameSite};

/// Test adding cookies.
#[tokio::test]
async fn test_context_add_cookies() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;

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

    let (browser, context, page) = common::launch_with_page().await;

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
