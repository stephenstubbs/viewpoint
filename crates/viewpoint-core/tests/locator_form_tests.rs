#![cfg(feature = "integration")]

//! Locator form interaction tests.
//!
//! Tests for form-related locator actions: fill, type, clear, check, uncheck, select.

mod common;

use viewpoint_core::{Browser, DocumentLoadState};

/// Helper function to launch browser and get a page.
async fn setup() -> (
    Browser,
    viewpoint_core::BrowserContext,
    viewpoint_core::Page,
) {
    common::launch_with_page().await
}

/// Test form fill and type.
#[tokio::test]
async fn test_locator_fill_and_type() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <input type="text" name="custname" placeholder="Customer Name">
                <input type="tel" name="custtel" placeholder="Phone">
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill in the customer name field
    let customer_name = page.locator("input[name='custname']");
    customer_name
        .fill("John Doe")
        .await
        .expect("Failed to fill");

    // Type in the phone field
    let phone = page.locator("input[name='custtel']");
    phone.click().await.expect("Failed to click phone field");
    phone.type_text("555-1234").await.expect("Failed to type");

    browser.close().await.expect("Failed to close browser");
}

/// Test pressing keys on a locator.
#[tokio::test]
async fn test_locator_press() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <input type="text" name="custname" placeholder="Customer Name">
                <input type="tel" name="custtel" placeholder="Phone">
            </form>
        </body></html>
    "#;

    page.goto(html)
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

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <input type="text" name="custname" placeholder="Customer Name">
            </form>
        </body></html>
    "#;

    page.goto(html)
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

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <label>
                    <input type="checkbox" id="cheese" value="cheese" name="topping">
                    Cheese
                </label>
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Find the checkbox
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
        .select_option()
        .value("medium")
        .await
        .expect("Failed to select by value");

    // Select by visible text (label)
    size_select
        .select_option()
        .label("Large")
        .await
        .expect("Failed to select by text");

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.input_value() gets input field value.
#[tokio::test]
async fn test_locator_input_value() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <input type="text" name="custname" placeholder="Customer Name">
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Fill an input
    let input = page.locator("input[name='custname']");
    input.fill("Test Value").await.expect("Failed to fill");

    // Get the value back
    let value = input
        .input_value()
        .await
        .expect("Failed to get input value");
    assert_eq!(value, "Test Value");

    browser.close().await.expect("Failed to close browser");
}

/// Test locator.focus() focuses an element.
#[tokio::test]
async fn test_locator_focus() {
    common::init_tracing();

    let (browser, _context, page) = setup().await;

    // Use a local data URL to avoid external network dependencies
    let html = r#"data:text/html,
        <html><body>
            <form>
                <input type="text" name="custname" placeholder="Customer Name">
            </form>
        </body></html>
    "#;

    page.goto(html)
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .expect("Failed to navigate");

    // Focus an input
    let input = page.locator("input[name='custname']");
    input.focus().await.expect("Failed to focus");

    browser.close().await.expect("Failed to close browser");
}
