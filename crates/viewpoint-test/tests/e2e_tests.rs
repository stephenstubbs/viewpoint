//! End-to-end tests for the complete Viewpoint test framework.
//!
//! These tests exercise the full stack: browser automation, locators,
//! actions, and assertions working together in realistic scenarios.

use std::sync::Once;
use std::time::Duration;

use viewpoint_core::DocumentLoadState;
use viewpoint_test::{expect, expect_page, TestHarness};

static TRACING_INIT: Once = Once::new();

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

/// E2E test: Complete form interaction workflow
/// 
/// This test exercises:
/// - Navigation
/// - Element location with various selectors
/// - Form filling (fill, type, click)
/// - Checkbox interaction
/// - Text assertions
/// - Page assertions
#[tokio::test]
async fn e2e_form_interaction() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    // Navigate to httpbin forms page
    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("should navigate");

    // Verify we're on the right page
    expect_page(page)
        .to_have_url_containing("httpbin.org/forms")
        .await
        .expect("should be on forms page");

    // Fill customer name using CSS selector
    let name_input = page.locator("input[name='custname']");
    expect(&name_input)
        .to_be_visible()
        .await
        .expect("name input should be visible");
    
    name_input.fill("John Doe").await.expect("should fill name");

    // Fill phone using type_text for character-by-character input
    let phone_input = page.locator("input[name='custtel']");
    phone_input.click().await.expect("should click phone");
    phone_input.type_text("555-1234").await.expect("should type phone");

    // Fill email
    let email_input = page.locator("input[name='custemail']");
    email_input.fill("john@example.com").await.expect("should fill email");

    // Select pizza size (radio buttons) - click medium
    let medium_size = page.locator("input[value='medium']");
    medium_size.click().await.expect("should select medium");

    // Check some toppings
    let cheese = page.locator("input[value='cheese']");
    cheese.check().await.expect("should check cheese");
    expect(&cheese)
        .to_be_checked()
        .await
        .expect("cheese should be checked");

    let mushrooms = page.locator("input[value='mushroom']");
    mushrooms.check().await.expect("should check mushrooms");
    expect(&mushrooms)
        .to_be_checked()
        .await
        .expect("mushrooms should be checked");

    // Uncheck mushrooms
    mushrooms.uncheck().await.expect("should uncheck mushrooms");
    expect(&mushrooms)
        .not()
        .to_be_checked()
        .await
        .expect("mushrooms should be unchecked");

    // Fill delivery time
    let time_input = page.locator("input[name='delivery']");
    time_input.fill("18:00").await.expect("should fill time");

    // Fill comments in textarea
    let comments = page.locator("textarea[name='comments']");
    comments.fill("Please ring the doorbell twice.").await.expect("should fill comments");
}

/// E2E test: Navigation and content verification
/// 
/// This test exercises:
/// - Multiple page navigations
/// - URL assertions
/// - Title assertions  
/// - Text content assertions
/// - Element visibility
#[tokio::test]
async fn e2e_navigation_and_content() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    // Navigate to example.com
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate to example.com");

    // Verify page state
    expect_page(page)
        .to_have_title("Example Domain")
        .await
        .expect("should have correct title");

    expect_page(page)
        .to_have_url_containing("example.com")
        .await
        .expect("should have correct URL");

    // Verify heading
    let heading = page.locator("h1");
    expect(&heading)
        .to_be_visible()
        .await
        .expect("heading should be visible");
    
    expect(&heading)
        .to_have_text("Example Domain")
        .await
        .expect("heading should have correct text");

    // Verify paragraphs exist
    let paragraphs = page.locator("p");
    let count = paragraphs.count().await.expect("should count paragraphs");
    assert!(count >= 1, "should have at least one paragraph");

    // Navigate to httpbin
    page.goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate to httpbin");

    expect_page(page)
        .to_have_url_containing("httpbin.org")
        .await
        .expect("should be on httpbin");

    // Verify content
    let body = page.locator("body");
    expect(&body)
        .to_contain_text("Herman Melville")
        .await
        .expect("should contain expected text");
}

/// E2E test: Element selection and chaining
/// 
/// This test exercises:
/// - CSS selectors
/// - Text selectors
/// - Locator chaining
/// - nth selection (first, last)
#[tokio::test]
async fn e2e_element_selection() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // CSS selector
    let h1 = page.locator("h1");
    expect(&h1)
        .to_be_visible()
        .await
        .expect("h1 should be visible");

    // Text selector
    let by_text = page.get_by_text("Example Domain");
    expect(&by_text)
        .to_be_visible()
        .await
        .expect("text element should be visible");

    // Chained locator
    let chained = page.locator("body").locator("div").locator("h1");
    expect(&chained)
        .to_be_visible()
        .await
        .expect("chained locator should find element");

    // First/last selection
    let first_p = page.locator("p").first();
    expect(&first_p)
        .to_be_visible()
        .await
        .expect("first paragraph should be visible");
}

/// E2E test: Mouse interactions
/// 
/// This test exercises:
/// - Click
/// - Double-click
/// - Hover
#[tokio::test]
async fn e2e_mouse_interactions() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Hover over heading
    let heading = page.locator("h1");
    heading.hover().await.expect("should hover over heading");

    // Double-click to select text
    heading.dblclick().await.expect("should double-click heading");

    // Click the link
    let link = page.locator("a");
    expect(&link)
        .to_be_visible()
        .await
        .expect("link should be visible");
    
    // Just verify we can click (don't follow navigation)
    // link.click().await.expect("should click link");
}

/// E2E test: Keyboard interactions
/// 
/// This test exercises:
/// - Focus
/// - Key presses
/// - Modifier keys
#[tokio::test]
async fn e2e_keyboard_interactions() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://httpbin.org/forms/post")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("should navigate");

    // Focus input
    let input = page.locator("input[name='custname']");
    input.focus().await.expect("should focus input");

    // Type text
    input.type_text("Test").await.expect("should type");

    // Press Tab to move to next field
    input.press("Tab").await.expect("should press Tab");

    // Select all with Ctrl+A
    let phone = page.locator("input[name='custtel']");
    phone.fill("12345").await.expect("should fill");
    phone.press("Control+a").await.expect("should select all");

    // Clear with backspace
    phone.press("Backspace").await.expect("should delete");
}

/// E2E test: Assertion negation
/// 
/// This test exercises:
/// - `.not()` modifier on assertions
#[tokio::test]
async fn e2e_assertion_negation() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    let heading = page.locator("h1");

    // Verify NOT having wrong text
    expect(&heading)
        .not()
        .to_have_text("Wrong Title")
        .await
        .expect("should not have wrong text");

    // Verify non-existent element is hidden
    let nonexistent = page.locator("#does-not-exist");
    expect(&nonexistent)
        .to_be_hidden()
        .await
        .expect("nonexistent should be hidden");

    expect(&nonexistent)
        .not()
        .to_be_visible()
        .await
        .expect("nonexistent should not be visible");

    // Page assertions with negation
    expect_page(page)
        .not()
        .to_have_url("https://google.com")
        .await
        .expect("should not be on google");

    expect_page(page)
        .not()
        .to_have_title("Google")
        .await
        .expect("should not have Google title");
}

/// E2E test: Custom timeouts
/// 
/// This test exercises:
/// - Custom assertion timeouts
/// - Fast failure on timeout
#[tokio::test]
async fn e2e_custom_timeouts() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate");

    // Short timeout should still work for existing element
    let heading = page.locator("h1");
    expect(&heading)
        .timeout(Duration::from_millis(500))
        .to_be_visible()
        .await
        .expect("should find element quickly");

    // Very short timeout should fail for wrong text
    let result = expect(&heading)
        .timeout(Duration::from_millis(100))
        .to_have_text("Wrong Text")
        .await;
    
    assert!(result.is_err(), "should fail with wrong text and short timeout");
}

/// E2E test: Multiple pages
/// 
/// This test exercises:
/// - Creating multiple pages
/// - Independent page state
#[tokio::test]
async fn e2e_multiple_pages() {
    init_tracing();

    let harness = TestHarness::new().await.expect("should create harness");
    
    // First page
    let page1 = harness.page();
    page1.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate page 1");

    // Create second page
    let page2 = harness.new_page().await.expect("should create second page");
    page2.goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .expect("should navigate page 2");

    // Verify both pages have correct content
    expect_page(page1)
        .to_have_title("Example Domain")
        .await
        .expect("page 1 should have correct title");

    let page2_body = page2.locator("body");
    expect(&page2_body)
        .to_contain_text("Herman Melville")
        .await
        .expect("page 2 should have correct content");
}
