#![cfg(feature = "integration")]

//! Tests for wait_for_function functionality.
//!
//! These tests verify that wait_for_function properly handles both primitive
//! and object return values without panicking.

mod common;

use std::time::Duration;

use viewpoint_core::page::Polling;

// =============================================================================
// Primitive Return Value Tests (should return None)
// =============================================================================

/// Test wait_for_function with boolean `true` return.
/// Should return Ok(None) because booleans are primitives with no object handle.
#[tokio::test]
async fn test_wait_for_function_boolean_true() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // This previously panicked because `true` has no object_id
    let result = page
        .wait_for_function("() => true")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(
        result.is_ok(),
        "wait_for_function should succeed: {:?}",
        result.err()
    );
    assert!(
        result.unwrap().is_none(),
        "Boolean true should return None (no handle)"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with number return (e.g., 42).
/// Should return Ok(None) because numbers are primitives.
#[tokio::test]
async fn test_wait_for_function_number() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => 42")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    assert!(
        result.unwrap().is_none(),
        "Number should return None (no handle)"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with string return.
/// Should return Ok(None) because strings are primitives.
#[tokio::test]
async fn test_wait_for_function_string() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => 'loaded'")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    assert!(
        result.unwrap().is_none(),
        "String should return None (no handle)"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with a text content check.
/// This is a common real-world use case that previously panicked.
#[tokio::test]
async fn test_wait_for_function_text_includes() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body>Page is loaded</body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // This is the exact use case from the bug report
    let result = page
        .wait_for_function("() => document.body.innerText.includes('loaded')")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(
        result.is_ok(),
        "wait_for_function should succeed: {:?}",
        result.err()
    );
    assert!(
        result.unwrap().is_none(),
        "String.includes() returns boolean, should be None"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Object Return Value Tests (should return Some(JsHandle))
// =============================================================================

/// Test wait_for_function with object return (document.body).
/// Should return Ok(Some(JsHandle)) because DOM elements have object handles.
#[tokio::test]
async fn test_wait_for_function_dom_element() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => document.body")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    let handle = result.unwrap();
    assert!(handle.is_some(), "DOM element should return Some(JsHandle)");

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with querySelector result.
/// Should return Ok(Some(JsHandle)) when element is found.
#[tokio::test]
async fn test_wait_for_function_query_selector() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body><div class='ready'>Content</div></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => document.querySelector('.ready')")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    let handle = result.unwrap();
    assert!(
        handle.is_some(),
        "querySelector result should return Some(JsHandle)"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with object literal return.
/// Should return Ok(Some(JsHandle)) because objects have handles.
#[tokio::test]
async fn test_wait_for_function_object_literal() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => ({ ready: true })")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    let handle = result.unwrap();
    assert!(
        handle.is_some(),
        "Object literal should return Some(JsHandle)"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with array return.
/// Should return Ok(Some(JsHandle)) because arrays are objects.
#[tokio::test]
async fn test_wait_for_function_array() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => [1, 2, 3]")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    let handle = result.unwrap();
    assert!(handle.is_some(), "Array should return Some(JsHandle)");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Polling Mode Tests
// =============================================================================

/// Test wait_for_function with RAF polling (default).
#[tokio::test]
async fn test_wait_for_function_raf_polling() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Set a variable after a short delay (void() discards the setTimeout return value)
    page.evaluate::<()>(viewpoint_js::js! {
        void(setTimeout(() => { window.ready = true; }, 100))
    })
    .await
    .expect("Failed to set timeout");

    let result = page
        .wait_for_function("() => window.ready")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    // window.ready = true is a boolean, so no handle
    assert!(result.unwrap().is_none(), "Boolean should return None");

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with interval polling.
#[tokio::test]
async fn test_wait_for_function_interval_polling() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Set a variable after a short delay (void() discards the setTimeout return value)
    page.evaluate::<()>(viewpoint_js::js! {
        void(setTimeout(() => { window.intervalReady = true; }, 150))
    })
    .await
    .expect("Failed to set timeout");

    let result = page
        .wait_for_function("() => window.intervalReady")
        .polling(Polling::Interval(Duration::from_millis(50)))
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Timeout Tests
// =============================================================================

/// Test wait_for_function times out when condition never becomes true.
#[tokio::test]
async fn test_wait_for_function_timeout() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function("() => false")
        .timeout(Duration::from_millis(500))
        .wait()
        .await;

    assert!(result.is_err(), "wait_for_function should timeout");
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("Timeout"),
        "Error should mention timeout: {err}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test wait_for_function with falsy then truthy value.
#[tokio::test]
async fn test_wait_for_function_falsy_to_truthy() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Start with falsy, then set to truthy after delay (void() discards setTimeout return)
    page.evaluate::<()>(viewpoint_js::js! {
        window.counter = 0;
        void(setTimeout(() => { window.counter = 5; }, 100))
    })
    .await
    .expect("Failed to set up counter");

    let result = page
        .wait_for_function("() => window.counter")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function should succeed");
    // Numbers are primitives, so no handle
    assert!(result.unwrap().is_none(), "Number should return None");

    browser.close().await.expect("Failed to close browser");
}

/// Test wait_for_function with argument.
#[tokio::test]
async fn test_wait_for_function_with_arg() {
    let (browser, _context, page) = common::launch_with_page().await;

    page.set_content("<html><body><div class='target'>Found</div></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    let result = page
        .wait_for_function_with_arg("sel => document.querySelector(sel)", ".target")
        .timeout(Duration::from_secs(5))
        .wait()
        .await;

    assert!(result.is_ok(), "wait_for_function_with_arg should succeed");
    let handle = result.unwrap();
    assert!(
        handle.is_some(),
        "querySelector result should return Some(JsHandle)"
    );

    browser.close().await.expect("Failed to close browser");
}
