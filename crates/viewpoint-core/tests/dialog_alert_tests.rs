#![cfg(feature = "integration")]

//! Alert and confirm dialog tests for viewpoint-core.
//!
//! These tests verify alert and confirm dialog handling including
//! event capture, type identification, accept/dismiss actions.

mod common;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use viewpoint_core::{Browser, DialogType};
use viewpoint_js::js;

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const ALERT_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <button id="alert" onclick="alert('Hello from alert!')">Show Alert</button>
    <button id="multiple-alerts" onclick="alert('First'); alert('Second'); alert('Third');">Multiple Alerts</button>
</body>
</html>
"#;

const CONFIRM_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <button id="confirm" onclick="window.result = confirm('Are you sure?')">Show Confirm</button>
    <div id="result"></div>
    <script>
        document.getElementById('confirm').addEventListener('click', function() {
            setTimeout(function() {
                document.getElementById('result').textContent = window.result ? 'true' : 'false';
            }, 10);
        });
    </script>
</body>
</html>
"#;

// =============================================================================
// Alert Dialog Tests
// =============================================================================

/// Test that alert dialog events are captured.
#[tokio::test]
async fn test_dialog_alert_event() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    // Track if dialog handler was called
    let dialog_received = Arc::new(Mutex::new(false));
    let dialog_received_clone = dialog_received.clone();
    let dialog_message = Arc::new(Mutex::new(String::new()));
    let dialog_message_clone = dialog_message.clone();

    // Set up dialog handler
    page.on_dialog(move |dialog| {
        let received = dialog_received_clone.clone();
        let message = dialog_message_clone.clone();
        async move {
            *received.lock().await = true;
            *message.lock().await = dialog.message().to_string();
            dialog.accept().await
        }
    }).await;

    // Trigger the alert
    page.locator("#alert").click().await.expect("Failed to click button");
    
    // Wait for the dialog to be handled
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Verify dialog was received
    assert!(*dialog_received.lock().await, "Dialog handler should have been called");
    assert_eq!(*dialog_message.lock().await, "Hello from alert!");

    browser.close().await.expect("Failed to close browser");
}

/// Test that alert dialog type is correctly identified.
#[tokio::test]
async fn test_dialog_type_alert() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    let dialog_type = Arc::new(Mutex::new(None::<DialogType>));
    let dialog_type_clone = dialog_type.clone();

    page.on_dialog(move |dialog| {
        let dtype = dialog_type_clone.clone();
        async move {
            *dtype.lock().await = Some(dialog.type_());
            dialog.accept().await
        }
    }).await;

    page.locator("#alert").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    let received_type = dialog_type.lock().await.clone();
    assert!(matches!(received_type, Some(DialogType::Alert)), "Expected Alert dialog type");

    browser.close().await.expect("Failed to close browser");
}

/// Test accepting an alert dialog.
#[tokio::test]
async fn test_dialog_accept_alert() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    let accepted = Arc::new(Mutex::new(false));
    let accepted_clone = accepted.clone();

    page.on_dialog(move |dialog| {
        let acc = accepted_clone.clone();
        async move {
            let result = dialog.accept().await;
            *acc.lock().await = result.is_ok();
            result
        }
    }).await;

    page.locator("#alert").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    assert!(*accepted.lock().await, "Dialog should be accepted successfully");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Confirm Dialog Tests
// =============================================================================

/// Test that confirm dialog events are captured.
#[tokio::test]
async fn test_dialog_confirm_event() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(CONFIRM_HTML).set().await.expect("Failed to set content");

    let dialog_type = Arc::new(Mutex::new(None::<DialogType>));
    let dialog_type_clone = dialog_type.clone();

    page.on_dialog(move |dialog| {
        let dtype = dialog_type_clone.clone();
        async move {
            *dtype.lock().await = Some(dialog.type_());
            dialog.accept().await
        }
    }).await;

    page.locator("#confirm").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    let received_type = dialog_type.lock().await.clone();
    assert!(matches!(received_type, Some(DialogType::Confirm)), "Expected Confirm dialog type");

    browser.close().await.expect("Failed to close browser");
}

/// Test that accepting a confirm dialog returns true.
#[tokio::test]
async fn test_dialog_accept_confirm_returns_true() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(CONFIRM_HTML).set().await.expect("Failed to set content");

    page.on_dialog(|dialog| async move {
        dialog.accept().await
    }).await;

    page.locator("#confirm").click().await.expect("Failed to click button");
    
    // Wait for the result to be populated
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Verify the confirm returned true
    let result: String = page.evaluate(js!{ document.getElementById("result").textContent })
        .await
        .expect("Failed to get result");
    
    assert_eq!(result, "true", "Confirm should return true when accepted");

    browser.close().await.expect("Failed to close browser");
}

/// Test that dismissing a confirm dialog returns false.
#[tokio::test]
async fn test_dialog_dismiss_confirm_returns_false() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(CONFIRM_HTML).set().await.expect("Failed to set content");

    page.on_dialog(|dialog| async move {
        dialog.dismiss().await
    }).await;

    page.locator("#confirm").click().await.expect("Failed to click button");
    
    // Wait for the result to be populated
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Verify the confirm returned false
    let result: String = page.evaluate(js!{ document.getElementById("result").textContent })
        .await
        .expect("Failed to get result");
    
    assert_eq!(result, "false", "Confirm should return false when dismissed");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Auto-Dismiss Tests
// =============================================================================

/// Test that dialogs are auto-dismissed when no handler is registered.
#[tokio::test]
async fn test_dialog_auto_dismiss_when_no_listener() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    // Don't set up any dialog handler - dialog should auto-dismiss
    
    // Click the button that triggers alert
    page.locator("#alert").click().await.expect("Failed to click button");
    
    // Wait a bit - page should not freeze
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Verify page is still responsive
    let _title: String = page.evaluate(js!{ document.title || "no-title" })
        .await
        .expect("Page should still be responsive after auto-dismiss");

    browser.close().await.expect("Failed to close browser");
}

/// Test that multiple dialogs can be auto-dismissed without blocking.
#[tokio::test]
async fn test_dialog_auto_dismiss_multiple_alerts() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    // Don't set up any dialog handler - dialogs should auto-dismiss
    
    // Click button that triggers multiple alerts
    page.locator("#multiple-alerts").click().await.expect("Failed to click button");
    
    // Wait for all dialogs to be dismissed
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify page is still responsive
    let _title: String = page.evaluate(js!{ document.title || "no-title" })
        .await
        .expect("Page should still be responsive after multiple auto-dismisses");

    browser.close().await.expect("Failed to close browser");
}
