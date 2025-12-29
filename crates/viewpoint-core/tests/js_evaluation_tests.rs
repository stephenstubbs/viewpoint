#![cfg(feature = "integration")]

//! JavaScript evaluation tests for viewpoint-core.
//!
//! These tests verify the js! macro and evaluate functionality.

use std::sync::Once;

use viewpoint_core::Browser;
use viewpoint_js::js;

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
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

// =============================================================================
// Basic Evaluation Tests
// =============================================================================

/// Test evaluating primitive types with js! macro.
#[tokio::test]
async fn test_js_evaluate_primitives() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test integer
    let num: i32 = page
        .evaluate(js! { 1 + 2 })
        .await
        .expect("Failed to evaluate int");
    assert_eq!(num, 3);

    // Test float
    let float: f64 = page
        .evaluate(js! { 3.14 * 2 })
        .await
        .expect("Failed to evaluate float");
    assert!((float - 6.28).abs() < 0.01);

    // Test boolean
    let boolean: bool = page
        .evaluate(js! { true && false })
        .await
        .expect("Failed to evaluate bool");
    assert!(!boolean);

    // Test string
    let string: String = page
        .evaluate(js! { "hello" + " " + "world" })
        .await
        .expect("Failed to evaluate string");
    assert_eq!(string, "hello world");

    // Test null (returns Option)
    let null_val: Option<String> = page
        .evaluate(js! { null })
        .await
        .expect("Failed to evaluate null");
    assert!(null_val.is_none());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test evaluating objects with js! macro.
#[tokio::test]
async fn test_js_evaluate_objects() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test object - note the parentheses around the object literal
    let obj: serde_json::Value = page
        .evaluate(js! { ({ a: 1, b: "two", c: true }) })
        .await
        .expect("Failed to evaluate object");

    assert_eq!(obj["a"], 1);
    assert_eq!(obj["b"], "two");
    assert_eq!(obj["c"], true);

    // Test array
    let arr: Vec<i32> = page
        .evaluate(js! { [1, 2, 3, 4, 5] })
        .await
        .expect("Failed to evaluate array");
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test evaluating promises with js! macro.
#[tokio::test]
async fn test_js_evaluate_promises() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test resolved promise
    let result: String = page
        .evaluate(js! {
            Promise.resolve("resolved value")
        })
        .await
        .expect("Failed to evaluate promise");
    assert_eq!(result, "resolved value");

    // Test async function
    let async_result: i32 = page
        .evaluate(js! {
            (async () => {
                await new Promise(r => setTimeout(r, 10));
                return 42;
            })()
        })
        .await
        .expect("Failed to evaluate async function");
    assert_eq!(async_result, 42);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test evaluate on closed page should fail.
#[tokio::test]
async fn test_evaluate_on_closed_page() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let mut page = context.new_page().await.expect("Failed to create page");

    // Close the page first
    page.close().await.expect("Failed to close page");
    assert!(page.is_closed());

    // Attempt evaluate on closed page should fail
    let result: Result<i32, _> = page.evaluate(js! { 1 + 1 }).await;
    assert!(result.is_err(), "Evaluate on closed page should fail");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Interpolation Tests
// =============================================================================

/// Test js! macro interpolation with various types.
#[tokio::test]
async fn test_js_interpolation_types() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a simple page
    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test integer interpolation
    let num = 42;
    let result: i32 = page
        .evaluate(&js! { #{num} * 2 })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, 84);

    // Test float interpolation
    let pi = 3.14159;
    let result: f64 = page
        .evaluate(&js! { Math.round(#{pi} * 100) / 100 })
        .await
        .expect("Failed to evaluate");
    assert!((result - 3.14).abs() < 0.01);

    // Test string interpolation
    let name = "World";
    let result: String = page
        .evaluate(&js! { "Hello, " + #{name} + "!" })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, "Hello, World!");

    // Test string with special characters
    let special = "it's a \"test\"\nwith newlines";
    let result: String = page
        .evaluate(&js! { #{special} })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, special);

    // Test boolean interpolation
    let flag = true;
    let result: bool = page
        .evaluate(&js! { #{flag} && true })
        .await
        .expect("Failed to evaluate");
    assert!(result);

    // Test multiple interpolations
    let x = 10;
    let y = 20;
    let result: i32 = page
        .evaluate(&js! { #{x} + #{y} })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, 30);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test js! macro interpolation in function arguments.
#[tokio::test]
async fn test_js_interpolation_function_args() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a simple page
    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test interpolation as function argument
    // We wrap in void() to avoid returning the array
    let arr = "[1, 2, 3, 4, 5]";
    let _: () = page
        .evaluate(&js! { void(window.testArray = JSON.parse(#{arr})) })
        .await
        .expect("Failed to evaluate");

    // Verify the array was set correctly
    let length: i32 = page
        .evaluate(js! { window.testArray.length })
        .await
        .expect("Failed to get length");
    assert_eq!(length, 5);

    // Test interpolation with object property access
    let key = "testKey";
    let value = "testValue";
    let _: () = page
        .evaluate(&js! { void(window[#{key}] = #{value}) })
        .await
        .expect("Failed to evaluate");

    let retrieved: String = page
        .evaluate(&js! { window[#{key}] })
        .await
        .expect("Failed to get value");
    assert_eq!(retrieved, "testValue");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test js! macro with complex expressions.
#[tokio::test]
async fn test_js_interpolation_complex() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    // Navigate to a simple page
    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Test with computed values
    let base = 100;
    let multiplier = 5;
    let computed = base * multiplier;
    let result: i32 = page
        .evaluate(&js! { #{computed} })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, 500);

    // Test with String vs &str
    let owned_string = String::from("owned");
    let result: String = page
        .evaluate(&js! { #{owned_string} })
        .await
        .expect("Failed to evaluate");
    assert_eq!(result, "owned");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test js! macro with interpolation.
#[tokio::test]
async fn test_js_evaluate_with_interpolation() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content("<html><body><div id='test'>Hello</div></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Interpolate a number
    let multiplier = 5;
    let result: i32 = page
        .evaluate(&js! { 10 * #{multiplier} })
        .await
        .expect("Failed to evaluate with number interpolation");
    assert_eq!(result, 50);

    // Interpolate a string
    let selector = "#test";
    let text: String = page
        .evaluate(&js! { document.querySelector(#{selector}).textContent })
        .await
        .expect("Failed to evaluate with string interpolation");
    assert_eq!(text, "Hello");

    // Interpolate multiple values
    let a = 10;
    let b = 20;
    let sum: i32 = page
        .evaluate(&js! { #{a} + #{b} })
        .await
        .expect("Failed to evaluate with multiple interpolations");
    assert_eq!(sum, 30);

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
