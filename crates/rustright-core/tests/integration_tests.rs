//! Integration tests for rustright-core.
//!
//! These tests require Chromium to be installed and accessible.
//! Run with: `cargo test --test integration_tests`
//! Run with tracing: `RUST_LOG=debug cargo test --test integration_tests -- --nocapture`

use std::sync::Once;
use std::time::Duration;

use rustright_core::{Browser, DocumentLoadState};

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
/// This is safe to call multiple times - it will only initialize once.
fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

/// Test launching a browser, verifying connection, and closing.
#[tokio::test]
async fn test_browser_launch_and_close() {
    init_tracing();
    
    // Launch a headless browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Verify the browser is owned (we launched it)
    assert!(browser.is_owned());

    // Close the browser
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a browser context.
#[tokio::test]
async fn test_browser_context_creation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create a new context
    let context = browser.new_context().await.expect("Failed to create context");

    // Verify context has an ID
    assert!(!context.id().is_empty());
    assert!(!context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a page within a context.
#[tokio::test]
async fn test_page_creation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");

    // Create a new page
    let page = context.new_page().await.expect("Failed to create page");

    // Verify page has IDs
    assert!(!page.target_id().is_empty());
    assert!(!page.session_id().is_empty());
    assert!(!page.frame_id().is_empty());
    assert!(!page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a page.
#[tokio::test]
async fn test_page_close() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let mut page = context.new_page().await.expect("Failed to create page");

    assert!(!page.is_closed());

    // Close the page
    page.close().await.expect("Failed to close page");

    assert!(page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a browser context.
#[tokio::test]
async fn test_context_close() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let mut context = browser.new_context().await.expect("Failed to create context");

    // Create some pages in the context
    let _page1 = context.new_page().await.expect("Failed to create page 1");
    let _page2 = context.new_page().await.expect("Failed to create page 2");

    assert!(!context.is_closed());

    // Close the context (should close all pages)
    context.close().await.expect("Failed to close context");

    assert!(context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test basic navigation to a URL.
#[tokio::test]
async fn test_basic_navigation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to example.com
    let response = page
        .goto("https://example.com")
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");
    assert!(!response.frame_id.is_empty());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with `DomContentLoaded` wait state.
#[tokio::test]
async fn test_navigation_dom_content_loaded() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with DomContentLoaded wait
    let response = page
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with custom timeout.
#[tokio::test]
async fn test_navigation_with_timeout() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a generous timeout
    let response = page
        .goto("https://example.com")
        .timeout(Duration::from_secs(60))
        .goto()
        .await
        .expect("Failed to navigate");

    assert_eq!(response.url, "https://example.com");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation with referer.
#[tokio::test]
async fn test_navigation_with_referer() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate with a custom referer
    let response = page
        .goto("https://httpbin.org/headers")
        .referer("https://google.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    assert!(response.url.contains("httpbin.org"));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test multiple pages in the same context.
#[tokio::test]
async fn test_multiple_pages() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");

    // Create multiple pages
    let page1 = context.new_page().await.expect("Failed to create page 1");
    let page2 = context.new_page().await.expect("Failed to create page 2");

    // Verify they have different IDs
    assert_ne!(page1.target_id(), page2.target_id());
    assert_ne!(page1.session_id(), page2.session_id());

    // Navigate both pages
    let response1 = page1
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    let response2 = page2
        .goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    assert!(response1.url.contains("example.com"));
    assert!(response2.url.contains("httpbin.org"));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test multiple contexts in the same browser.
#[tokio::test]
async fn test_multiple_contexts() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create multiple contexts
    let context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");

    // Verify they have different IDs
    assert_ne!(context1.id(), context2.id());

    // Create pages in each context
    let page1 = context1.new_page().await.expect("Failed to create page 1");
    let page2 = context2.new_page().await.expect("Failed to create page 2");

    // Navigate both pages
    page1
        .goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 1");

    page2
        .goto("https://example.org")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate page 2");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test navigation error handling for invalid URL.
#[tokio::test]
async fn test_navigation_error_invalid_url() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Try to navigate to an invalid URL (non-existent domain)
    let result = page
        .goto("https://this-domain-definitely-does-not-exist-12345.com")
        .timeout(Duration::from_secs(10))
        .goto()
        .await;

    // Should fail with a network error
    assert!(result.is_err());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// ============================================================================
// Locator and Action Tests
// ============================================================================

/// Test creating locators with different selectors.
#[tokio::test]
async fn test_locator_creation() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page with known content
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Create various locators
    let by_css = page.locator("h1");
    let by_text = page.get_by_text("Example Domain");
    let by_role = page.get_by_role(rustright_core::AriaRole::Heading).build();

    // Verify they have the expected selectors
    assert!(matches!(by_css.selector(), rustright_core::Selector::Css(_)));
    assert!(matches!(by_text.selector(), rustright_core::Selector::Text { .. }));
    assert!(matches!(by_role.selector(), rustright_core::Selector::Role { .. }));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test locator chaining.
#[tokio::test]
async fn test_locator_chaining() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Chain locators
    let chained = page.locator("body").locator("div").locator("h1");

    // Verify it's a chained selector
    assert!(matches!(chained.selector(), rustright_core::Selector::Chained(_, _)));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test locator nth selection.
#[tokio::test]
async fn test_locator_nth_selection() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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
    assert!(matches!(first.selector(), rustright_core::Selector::Nth { .. }));
    assert!(matches!(last.selector(), rustright_core::Selector::Nth { .. }));
    assert!(matches!(nth.selector(), rustright_core::Selector::Nth { .. }));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clicking an element.
#[tokio::test]
async fn test_locator_click() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to example.com which has a link
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

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test hovering over an element.
#[tokio::test]
async fn test_locator_hover() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Hover over the heading
    let heading = page.locator("h1");
    heading.hover().await.expect("Failed to hover");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test getting text content from an element.
#[tokio::test]
async fn test_locator_text_content() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test checking element visibility.
#[tokio::test]
async fn test_locator_is_visible() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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
    let not_visible = nonexistent.is_visible().await.expect("Failed to check visibility");
    assert!(!not_visible);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test counting matching elements.
#[tokio::test]
async fn test_locator_count() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test form fill on httpbin.org forms page.
#[tokio::test]
async fn test_locator_fill_and_type() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a page with a form (httpbin has a simple forms page)
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

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test double-click action.
#[tokio::test]
async fn test_locator_dblclick() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Double-click the heading (should select text)
    let heading = page.locator("h1");
    heading.dblclick().await.expect("Failed to double-click");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test pressing keys.
#[tokio::test]
async fn test_locator_press() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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
    input.press("Control+a").await.expect("Failed to press Ctrl+A");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clearing an input field.
#[tokio::test]
async fn test_locator_clear() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill then clear
    let input = page.locator("input[name='custname']");
    input.fill("Some text").await.expect("Failed to fill");
    input.clear().await.expect("Failed to clear");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test checkbox check and uncheck.
#[tokio::test]
async fn test_locator_check_uncheck() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find a checkbox (the toppings checkboxes)
    let checkbox = page.locator("input[type='checkbox'][value='cheese']");
    
    // Check it
    checkbox.check().await.expect("Failed to check");
    let checked = checkbox.is_checked().await.expect("Failed to get checked state");
    assert!(checked);

    // Uncheck it
    checkbox.uncheck().await.expect("Failed to uncheck");
    let unchecked = !checkbox.is_checked().await.expect("Failed to get checked state");
    assert!(unchecked);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_label locator.
#[tokio::test]
async fn test_get_by_label() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_label to find the "Customer name" input
    let label_locator = page.get_by_label("Customer name");
    assert!(matches!(label_locator.selector(), rustright_core::Selector::Label { .. }));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_placeholder locator.
#[tokio::test]
async fn test_get_by_placeholder() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_placeholder (even if no matching elements, verify the locator is created)
    let placeholder_locator = page.get_by_placeholder("Search...");
    assert!(matches!(placeholder_locator.selector(), rustright_core::Selector::Placeholder { .. }));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test get_by_test_id locator.
#[tokio::test]
async fn test_get_by_test_id() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("Failed to navigate");

    // Use get_by_test_id (even if no matching elements, verify the locator is created)
    let test_id_locator = page.get_by_test_id("submit-button");
    assert!(matches!(test_id_locator.selector(), rustright_core::Selector::TestId(_)));

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test select_option for dropdown selection.
#[tokio::test]
async fn test_locator_select_option() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

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
    size_select.select_option("medium").await.expect("Failed to select by value");
    
    // Select by visible text
    size_select.select_option("Large").await.expect("Failed to select by text");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
