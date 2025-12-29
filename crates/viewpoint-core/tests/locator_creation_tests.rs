#![cfg(feature = "integration")]

//! Locator creation and selector tests.
//!
//! Tests for creating locators with different selectors, chaining, and composition.

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

/// Test locator creation with different selectors.
#[tokio::test]
async fn test_locator_creation() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Create various locators
    let by_css = page.locator("h1");
    let by_text = page.get_by_text("Example Domain");
    let by_role = page.get_by_role(AriaRole::Heading).build();

    // Verify they have the expected selectors
    assert!(matches!(by_css.selector(), Selector::Css(_)));
    assert!(matches!(by_text.selector(), Selector::Text { .. }));
    assert!(matches!(by_role.selector(), Selector::Role { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator chaining.
#[tokio::test]
async fn test_locator_chaining() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Chain locators
    let chained = page.locator("body").locator("div").locator("h1");

    // Verify it's a chained selector
    assert!(matches!(chained.selector(), Selector::Chained(_, _)));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator nth selection.
#[tokio::test]
async fn test_locator_nth_selection() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use nth selection methods
    let first = page.locator("p").first();
    let last = page.locator("p").last();
    let nth = page.locator("p").nth(0);

    // Verify they're Nth selectors
    assert!(matches!(first.selector(), Selector::Nth { .. }));
    assert!(matches!(last.selector(), Selector::Nth { .. }));
    assert!(matches!(nth.selector(), Selector::Nth { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_label locator.
#[tokio::test]
async fn test_get_by_label() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_label to find the "Customer name" input
    let label_locator = page.get_by_label("Customer name");
    assert!(matches!(label_locator.selector(), Selector::Label { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_placeholder locator.
#[tokio::test]
async fn test_get_by_placeholder() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_placeholder (even if no matching elements, verify the locator is created)
    let placeholder_locator = page.get_by_placeholder("Search...");
    assert!(matches!(
        placeholder_locator.selector(),
        Selector::Placeholder { .. }
    ));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_test_id locator.
#[tokio::test]
async fn test_get_by_test_id() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_test_id (even if no matching elements, verify the locator is created)
    let test_id_locator = page.get_by_test_id("submit-button");
    assert!(matches!(test_id_locator.selector(), Selector::TestId(_)));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_alt_text locator.
#[tokio::test]
async fn test_get_by_alt_text() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_alt_text (even if no matching elements, verify the locator is created)
    let alt_locator = page.get_by_alt_text("Logo");
    assert!(matches!(alt_locator.selector(), Selector::AltText { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_title locator.
#[tokio::test]
async fn test_get_by_title() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_title (even if no matching elements, verify the locator is created)
    let title_locator = page.get_by_title("Help");
    assert!(matches!(title_locator.selector(), Selector::Title { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator AND composition.
#[tokio::test]
async fn test_locator_and_composition() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Create two locators and compose with AND
    let locator1 = page.locator("div");
    let locator2 = page.locator("p");
    let combined = locator1.and(locator2);

    // Verify it's an And selector
    assert!(matches!(combined.selector(), Selector::And(_, _)));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator OR composition.
#[tokio::test]
async fn test_locator_or_composition() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Create two locators and compose with OR
    let locator1 = page.locator("h1");
    let locator2 = page.locator("h2");
    let combined = locator1.or(locator2);

    // Verify it's an Or selector
    assert!(matches!(combined.selector(), Selector::Or(_, _)));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator filter with has_text.
#[tokio::test]
async fn test_locator_filter_has_text() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Filter locator by text content
    let filtered = page.locator("div").filter().has_text("Example");

    // Verify it's a FilterText selector
    assert!(matches!(filtered.selector(), Selector::FilterText { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator filter with has_not_text.
#[tokio::test]
async fn test_locator_filter_has_not_text() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Filter locator by NOT having text
    let filtered = page.locator("div").filter().has_not_text("NonExistent");

    // Verify it's a FilterText selector
    assert!(matches!(filtered.selector(), Selector::FilterText { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test locator filter with has (child locator).
#[tokio::test]
async fn test_locator_filter_has_child() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Filter by having a child element
    let child = page.locator("h1");
    let filtered = page.locator("div").filter().has(child);

    // Verify it's a FilterHas selector
    assert!(matches!(filtered.selector(), Selector::FilterHas { .. }));

    browser.close().await.expect("Failed to close browser");
}

/// Test CSS selector patterns.
#[tokio::test]
async fn test_css_selector_patterns() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Test various CSS selector patterns
    let by_tag = page.locator("h1");
    let by_class = page.locator(".example-class");
    let by_id = page.locator("#example-id");
    let by_attr = page.locator("[href]");
    let by_combined = page.locator("a[href*='iana']");

    // All should be CSS selectors
    assert!(matches!(by_tag.selector(), Selector::Css(_)));
    assert!(matches!(by_class.selector(), Selector::Css(_)));
    assert!(matches!(by_id.selector(), Selector::Css(_)));
    assert!(matches!(by_attr.selector(), Selector::Css(_)));
    assert!(matches!(by_combined.selector(), Selector::Css(_)));

    browser.close().await.expect("Failed to close browser");
}

/// Test text selector patterns.
#[tokio::test]
async fn test_text_selector_patterns() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Test text selectors
    let exact_text = page.get_by_text("Example Domain");

    // Should be a text selector
    assert!(matches!(exact_text.selector(), Selector::Text { .. }));

    browser.close().await.expect("Failed to close browser");
}
