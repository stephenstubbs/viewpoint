#![cfg(feature = "integration")]

//! Locator action tests.
//!
//! Tests for locator actions like click, hover, fill, type, and element queries.
//! For locator creation tests, see `locator_creation_tests.rs`.
//! For role-based locator tests, see `locator_role_tests.rs`.

mod common;

use std::time::Duration;
use viewpoint_core::{Browser, DocumentLoadState, Selector};

/// Helper function to launch browser and get a page.
async fn setup() -> (Browser, viewpoint_core::BrowserContext, viewpoint_core::Page) {
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
    let visible = heading.is_visible().await.expect("Failed to check visibility");
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

/// Test form fill and type.
#[tokio::test]
async fn test_locator_fill_and_type() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill in the customer name field
    let customer_name = page.locator("input[name='custname']");
    customer_name.fill("John Doe").await.expect("Failed to fill");

    // Type in the phone field
    let phone = page.locator("input[name='custtel']");
    phone.click().await.expect("Failed to click phone field");
    phone.type_text("555-1234").await.expect("Failed to type");

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

/// Test pressing keys on a locator.
#[tokio::test]
async fn test_locator_press() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Focus an input and press keys
    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");
    input.press("Tab").await.expect("Failed to press Tab");

    // Test modifier keys
    input
        .press("Control+a")
        .await
        .expect("Failed to press Ctrl+A");

    browser.close().await.expect("Failed to close browser");
}

/// Test clearing an input field.
#[tokio::test]
async fn test_locator_clear() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill then clear
    let input = page.locator("input[name='custname']");
    input.fill("Some text").await.expect("Failed to fill");
    input.clear().await.expect("Failed to clear");

    browser.close().await.expect("Failed to close browser");
}

/// Test checkbox check and uncheck.
#[tokio::test]
async fn test_locator_check_uncheck() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find a checkbox (the toppings checkboxes)
    let checkbox = page.locator("input[type='checkbox'][value='cheese']");

    // Check it
    checkbox.check().await.expect("Failed to check");
    let checked = checkbox
        .is_checked()
        .await
        .expect("Failed to get checked state");
    assert!(checked);

    // Uncheck it
    checkbox.uncheck().await.expect("Failed to uncheck");
    let unchecked = !checkbox
        .is_checked()
        .await
        .expect("Failed to get checked state");
    assert!(unchecked);

    browser.close().await.expect("Failed to close browser");
}

/// Test select_option for dropdown selection.
#[tokio::test]
async fn test_locator_select_option() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    // Navigate to a data URL with a select element
    let html = r#"data:text/html,
        <html><body>
            <select id="size" name="size">
                <option value="small">Small</option>
                <option value="medium">Medium</option>
                <option value="large">Large</option>
            </select>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find the select element
    let size_select = page.locator("select#size");

    // Select by value
    size_select
        .select_option("medium")
        .await
        .expect("Failed to select by value");

    // Select by visible text
    size_select
        .select_option("Large")
        .await
        .expect("Failed to select by text");

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

/// Test locator.input_value() gets input field value.
#[tokio::test]
async fn test_locator_input_value() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill an input
    let input = page.locator("input[name='custname']");
    input.fill("Test Value").await.expect("Failed to fill");

    // Get the value back
    let value = input.input_value().await.expect("Failed to get input value");
    assert_eq!(value, "Test Value");

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.focus() focuses an element.
#[tokio::test]
async fn test_locator_focus() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Focus an input
    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

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
