#![cfg(feature = "integration")]

//! Tests for automatic page tracking via CDP Target events.
//!
//! These tests verify that pages opened externally (e.g., via `window.open()`,
//! `target="_blank"` links, or Ctrl+click) are properly tracked and trigger
//! `on_page` events.

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use viewpoint_core::Browser;

/// Helper function to launch browser and get a context.
async fn setup() -> (Browser, viewpoint_core::BrowserContext) {
    common::init_tracing();
    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    (browser, context)
}

/// Test that window.open() creates a tracked page and triggers on_page event.
#[tokio::test]
async fn test_window_open_creates_tracked_page() {
    let (browser, context) = setup().await;

    // Track if on_page was called
    let page_received = Arc::new(AtomicBool::new(false));
    let page_received_clone = page_received.clone();

    // Set up on_page handler
    context
        .on_page(move |_page| {
            let received = page_received_clone.clone();
            async move {
                received.store(true, Ordering::SeqCst);
            }
        })
        .await;

    // Create initial page
    let page = context.new_page().await.expect("Failed to create page");

    // Set up HTML page with a button that opens a popup
    page.set_content(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <button id="open-popup" onclick="window.open('about:blank', '_blank', 'width=400,height=300')">Open Popup</button>
        </body>
        </html>
        "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Reset the flag (on_page was called for the initial page by new_page)
    page_received.store(false, Ordering::SeqCst);

    // Click the button to open a popup
    page.locator("#open-popup")
        .click()
        .await
        .expect("Failed to click button");

    // Wait a bit for the popup to be detected
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify on_page was triggered
    assert!(
        page_received.load(Ordering::SeqCst),
        "on_page event should have been triggered for the popup"
    );

    // Verify pages count increased
    let pages = context.pages().await.expect("Failed to get pages");
    assert!(
        pages.len() >= 2,
        "Should have at least 2 pages (original + popup), got {}",
        pages.len()
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that target="_blank" link creates a tracked page.
#[tokio::test]
async fn test_target_blank_link_creates_tracked_page() {
    let (browser, context) = setup().await;

    // Track received pages
    let pages_received = Arc::new(Mutex::new(Vec::<String>::new()));
    let pages_received_clone = pages_received.clone();

    // Set up on_page handler
    context
        .on_page(move |page| {
            let received = pages_received_clone.clone();
            async move {
                if let Ok(url) = page.url().await {
                    received.lock().await.push(url);
                }
            }
        })
        .await;

    // Create initial page
    let page = context.new_page().await.expect("Failed to create page");

    // Clear the pages received list (on_page was called for the initial page)
    pages_received.lock().await.clear();

    // Set up HTML page with a target="_blank" link
    page.set_content(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <a id="external-link" href="about:blank" target="_blank">Open in new tab</a>
        </body>
        </html>
        "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Click the link to open a new tab
    page.locator("#external-link")
        .click()
        .await
        .expect("Failed to click link");

    // Wait a bit for the new page to be detected
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify on_page was triggered for the new page
    let received = pages_received.lock().await;
    assert!(
        !received.is_empty(),
        "on_page event should have been triggered for the new tab"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that context.pages() includes externally-opened pages.
#[tokio::test]
async fn test_pages_includes_externally_opened() {
    let (browser, context) = setup().await;

    // Create initial page
    let page = context.new_page().await.expect("Failed to create page");

    // Check initial pages count
    let initial_pages = context.pages().await.expect("Failed to get pages");
    let initial_count = initial_pages.len();

    // Set up HTML page with a button that opens a popup
    page.set_content(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <button id="open-popup" onclick="window.open('about:blank')">Open Popup</button>
        </body>
        </html>
        "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Click the button to open a popup
    page.locator("#open-popup")
        .click()
        .await
        .expect("Failed to click button");

    // Wait a bit for the popup to be detected
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify pages count increased
    let final_pages = context.pages().await.expect("Failed to get pages");
    assert!(
        final_pages.len() > initial_count,
        "Pages count should increase after popup, was {} now {}",
        initial_count,
        final_pages.len()
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that no duplicate events are fired for pages created via new_page().
#[tokio::test]
async fn test_no_duplicate_events_for_new_page() {
    let (browser, context) = setup().await;

    // Count on_page calls
    let call_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    // Set up on_page handler
    context
        .on_page(move |_page| {
            let count = call_count_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .await;

    // Create a page via new_page() - should trigger exactly 1 on_page event
    let _page = context.new_page().await.expect("Failed to create page");

    // Wait a bit to ensure no delayed duplicate events
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Verify only one on_page event was triggered
    let count = call_count.load(Ordering::SeqCst);
    assert_eq!(
        count, 1,
        "on_page should be called exactly once for new_page(), got {}",
        count
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that wait_for_popup still works correctly with the automatic page tracking.
#[tokio::test]
async fn test_wait_for_popup_compatibility() {
    let (browser, context) = setup().await;

    // Create initial page
    let page = context.new_page().await.expect("Failed to create page");

    // Set up HTML page with a button that opens a popup
    page.set_content(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <button id="open-popup" onclick="window.open('about:blank', 'popup', 'width=400,height=300')">Open Popup</button>
        </body>
        </html>
        "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Use wait_for_popup to capture the popup
    let popup = page
        .wait_for_popup(|| async {
            page.locator("#open-popup")
                .click()
                .await
                .expect("Failed to click button");
            Ok(())
        })
        .wait()
        .await
        .expect("wait_for_popup should succeed");

    // Verify we got a popup page
    let popup_url = popup.url().await.expect("Failed to get popup URL");
    assert!(
        popup_url.contains("blank"),
        "Popup should be at about:blank, got {}",
        popup_url
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that on_page_activated handler registration and removal works correctly.
///
/// Note: The `Target.targetInfoChanged` event may not fire in headless mode for
/// `bring_to_front` calls. This test verifies the handler infrastructure works correctly.
#[tokio::test]
async fn test_page_activated_handler_registration() {
    let (browser, context) = setup().await;

    // Track activation count
    let activation_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let activation_count_clone = activation_count.clone();

    // Set up on_page_activated handler
    let handler_id = context
        .on_page_activated(move |_page| {
            let count = activation_count_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .await;

    // Create a page
    let _page = context.new_page().await.expect("Failed to create page");

    // Give time for any events
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Verify the handler was registered (we can remove it successfully)
    let removed = context.off_page_activated(handler_id).await;
    assert!(
        removed,
        "Handler should have been successfully registered and removed"
    );

    // Verify removing again returns false
    let removed_again = context.off_page_activated(handler_id).await;
    assert!(!removed_again, "Handler should not be found after removal");

    browser.close().await.expect("Failed to close browser");
}

/// Test that multiple on_page_activated handlers can be registered.
#[tokio::test]
async fn test_multiple_page_activated_handlers() {
    let (browser, context) = setup().await;

    // Register multiple handlers
    let handler_id1 = context.on_page_activated(|_page| async {}).await;
    let handler_id2 = context.on_page_activated(|_page| async {}).await;
    let handler_id3 = context.on_page_activated(|_page| async {}).await;

    // All handlers should have unique IDs
    assert_ne!(handler_id1, handler_id2);
    assert_ne!(handler_id2, handler_id3);
    assert_ne!(handler_id1, handler_id3);

    // All handlers should be removable
    assert!(context.off_page_activated(handler_id1).await);
    assert!(context.off_page_activated(handler_id2).await);
    assert!(context.off_page_activated(handler_id3).await);

    // None should be removable again
    assert!(!context.off_page_activated(handler_id1).await);
    assert!(!context.off_page_activated(handler_id2).await);
    assert!(!context.off_page_activated(handler_id3).await);

    browser.close().await.expect("Failed to close browser");
}

/// Test that on_page_activated only fires for pages in the same context.
#[tokio::test]
async fn test_page_activated_only_for_own_context() {
    let (browser, _) = setup().await;

    // Create two separate contexts
    let context_a = browser
        .new_context()
        .await
        .expect("Failed to create context A");
    let context_b = browser
        .new_context()
        .await
        .expect("Failed to create context B");

    // Track activations for context A
    let context_a_activations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let context_a_activations_clone = context_a_activations.clone();

    // Set up on_page_activated handler only on context A
    context_a
        .on_page_activated(move |_page| {
            let count = context_a_activations_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .await;

    // Create pages in both contexts
    let _page_a = context_a
        .new_page()
        .await
        .expect("Failed to create page in context A");
    let page_b = context_b
        .new_page()
        .await
        .expect("Failed to create page in context B");

    // Give pages time to initialize
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Reset counter
    context_a_activations.store(0, Ordering::SeqCst);

    // Bring page from context B to front
    page_b
        .bring_to_front()
        .await
        .expect("Failed to bring page B to front");
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Context A's handler should NOT have been triggered by context B's page
    let count = context_a_activations.load(Ordering::SeqCst);
    assert_eq!(
        count, 0,
        "Context A's handler should not be triggered by context B's page activation"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that closed pages are removed from tracking.
#[tokio::test]
async fn test_closed_page_removed_from_tracking() {
    let (browser, context) = setup().await;

    // Create initial page
    let page = context.new_page().await.expect("Failed to create page");

    // Set up HTML page with a button that opens a popup
    page.set_content(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <button id="open-popup" onclick="window.open('about:blank')">Open Popup</button>
        </body>
        </html>
        "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Use wait_for_popup to capture and then close the popup
    let mut popup = page
        .wait_for_popup(|| async {
            page.locator("#open-popup")
                .click()
                .await
                .expect("Failed to click button");
            Ok(())
        })
        .wait()
        .await
        .expect("wait_for_popup should succeed");

    // Get pages count with popup open
    let pages_with_popup = context.pages().await.expect("Failed to get pages");
    let count_with_popup = pages_with_popup.len();

    // Close the popup
    popup.close().await.expect("Failed to close popup");

    // Wait a bit for the targetDestroyed event to be processed
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Get pages count after popup closed
    let pages_after_close = context.pages().await.expect("Failed to get pages");
    let count_after_close = pages_after_close.len();

    // Note: The pages count should be equal or less after close
    // (pages() queries CDP directly, so it should reflect the actual state)
    assert!(
        count_after_close < count_with_popup,
        "Pages count should decrease after popup close, was {} now {}",
        count_with_popup,
        count_after_close
    );

    browser.close().await.expect("Failed to close browser");
}
