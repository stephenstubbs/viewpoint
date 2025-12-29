#![cfg(feature = "integration")]

//! Role-based locator tests.
//!
//! Tests for get_by_role locators with different ARIA roles.

mod common;

use viewpoint_core::{AriaRole, Browser, DocumentLoadState, Selector};

/// Helper function to launch browser and get a page.
async fn setup() -> (
    Browser,
    viewpoint_core::BrowserContext,
    viewpoint_core::Page,
) {
    common::launch_with_page().await
}

/// Test get_by_role with button.
#[tokio::test]
async fn test_get_by_role_button() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find button by role
    let button = page.get_by_role(AriaRole::Button).build();
    assert!(matches!(button.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_role with link.
#[tokio::test]
async fn test_get_by_role_link() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find link by role
    let link = page.get_by_role(AriaRole::Link).build();
    assert!(matches!(link.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_role with heading.
#[tokio::test]
async fn test_get_by_role_heading() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find heading by role
    let heading = page.get_by_role(AriaRole::Heading).build();
    assert!(matches!(heading.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_role with textbox.
#[tokio::test]
async fn test_get_by_role_textbox() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find textbox by role
    let textbox = page.get_by_role(AriaRole::TextBox).build();
    assert!(matches!(textbox.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_role with checkbox.
#[tokio::test]
async fn test_get_by_role_checkbox() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find checkbox by role
    let checkbox = page.get_by_role(AriaRole::Checkbox).build();
    assert!(matches!(checkbox.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}
