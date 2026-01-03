#![cfg(feature = "integration")]

//! Locator action tests.
//!
//! Tests for basic locator actions like click, hover, and element queries.
//! For form-related tests, see `locator_form_tests.rs`.
//! For locator creation tests, see `locator_creation_tests.rs`.
//! For role-based locator tests, see `locator_role_tests.rs`.

mod common;

use std::time::Duration;
use viewpoint_core::{Browser, DocumentLoadState, Selector};

/// Helper function to launch browser and get a page.
async fn setup() -> (
    Browser,
    viewpoint_core::BrowserContext,
    viewpoint_core::Page,
) {
    common::launch_with_page().await
}

/// Test clicking an element.
#[tokio::test]
async fn test_locator_click() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Click the "More information..." link
    let link = page.locator("a");
    link.click().await.expect("Failed to click link");

    // Give it a moment for navigation
    tokio::time::sleep(Duration::from_millis(500)).await;

    browser.close().await.expect("Failed to close browser");
}

/// Test hovering over an element.
#[tokio::test]
async fn test_locator_hover() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Hover over the heading
    let heading = page.locator("h1");
    heading.hover().await.expect("Failed to hover");

    browser.close().await.expect("Failed to close browser");
}

/// Test getting text content from an element.
#[tokio::test]
async fn test_locator_text_content() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get text from heading
    let heading = page.locator("h1");
    let text = heading.text_content().await.expect("Failed to get text");

    assert!(text.is_some());
    assert!(text.unwrap().contains("Example Domain"));

    browser.close().await.expect("Failed to close browser");
}

/// Test checking element visibility.
#[tokio::test]
async fn test_locator_is_visible() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Check visibility
    let heading = page.locator("h1");
    let visible = heading
        .is_visible()
        .await
        .expect("Failed to check visibility");
    assert!(visible);

    // Non-existent element should not be visible
    let nonexistent = page.locator("#does-not-exist-12345");
    let not_visible = nonexistent
        .is_visible()
        .await
        .expect("Failed to check visibility");
    assert!(!not_visible);

    browser.close().await.expect("Failed to close browser");
}

/// Test counting matching elements.
#[tokio::test]
async fn test_locator_count() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Count headings (should be 1)
    let heading = page.locator("h1");
    let count = heading.count().await.expect("Failed to count");
    assert_eq!(count, 1);

    // Count paragraphs (should be at least 1)
    let paragraphs = page.locator("p");
    let p_count = paragraphs.count().await.expect("Failed to count");
    assert!(p_count >= 1);

    browser.close().await.expect("Failed to close browser");
}

/// Test double-click action.
#[tokio::test]
async fn test_locator_dblclick() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Double-click the heading (should select text)
    let heading = page.locator("h1");
    heading.dblclick().await.expect("Failed to double-click");

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.all() returns all matching elements.
#[tokio::test]
async fn test_locator_all() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get all paragraph elements
    let paragraphs = page.locator("p");
    let all_p = paragraphs.all().await.expect("Failed to get all");

    // Should have at least one paragraph
    assert!(!all_p.is_empty());

    // Each element should be a locator
    for p in all_p {
        assert!(matches!(p.selector(), Selector::Nth { .. }));
    }

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.all_text_contents() returns text from all matches.
#[tokio::test]
async fn test_locator_all_text_contents() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get all text contents from paragraphs
    let paragraphs = page.locator("p");
    let texts = paragraphs
        .all_text_contents()
        .await
        .expect("Failed to get texts");

    // Should have at least one text
    assert!(!texts.is_empty());

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.all_inner_texts() returns inner text from all matches.
#[tokio::test]
async fn test_locator_all_inner_texts() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get all inner texts from paragraphs
    let paragraphs = page.locator("p");
    let texts = paragraphs
        .all_inner_texts()
        .await
        .expect("Failed to get inner texts");

    // Should have at least one text
    assert!(!texts.is_empty());

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.get_attribute() gets element attributes.
#[tokio::test]
async fn test_locator_get_attribute() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Get href attribute from link
    let link = page.locator("a");
    let href = link
        .get_attribute("href")
        .await
        .expect("Failed to get attribute");

    // Example.com has a link, so href should exist
    assert!(href.is_some());
    assert!(href.unwrap().contains("iana.org"));

    browser.close().await.expect("Failed to close browser");
}

/// Test scroll_into_view.
#[tokio::test]
async fn test_scroll_into_view() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Scroll element into view
    let link = page.locator("a");
    link.scroll_into_view_if_needed()
        .await
        .expect("Failed to scroll into view");

    browser.close().await.expect("Failed to close browser");
}
