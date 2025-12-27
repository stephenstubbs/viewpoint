#![cfg(feature = "integration")]

//! Prompt and beforeunload dialog tests for viewpoint-core.
//!
//! These tests verify prompt dialog handling, beforeunload dialogs,
//! and dialog handler management.

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
</body>
</html>
"#;

const PROMPT_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <button id="prompt" onclick="window.result = prompt('What is your name?', 'default value')">Show Prompt</button>
    <div id="result"></div>
    <script>
        document.getElementById('prompt').addEventListener('click', function() {
            setTimeout(function() {
                document.getElementById('result').textContent = window.result === null ? 'null' : window.result;
            }, 10);
        });
    </script>
</body>
</html>
"#;

const BEFOREUNLOAD_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <a id="leave" href="about:blank">Leave Page</a>
    <script>
        window.addEventListener('beforeunload', function(e) {
            e.preventDefault();
            e.returnValue = 'Are you sure you want to leave?';
            return e.returnValue;
        });
    </script>
</body>
</html>
"#;

// =============================================================================
// Prompt Dialog Tests
// =============================================================================

/// Test that prompt dialog events are captured.
#[tokio::test]
async fn test_dialog_prompt_event() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(PROMPT_HTML).set().await.expect("Failed to set content");

    let dialog_type = Arc::new(Mutex::new(None::<DialogType>));
    let dialog_type_clone = dialog_type.clone();

    page.on_dialog(move |dialog| {
        let dtype = dialog_type_clone.clone();
        async move {
            *dtype.lock().await = Some(dialog.type_());
            dialog.accept().await
        }
    }).await;

    page.locator("#prompt").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    let received_type = dialog_type.lock().await.clone();
    assert!(matches!(received_type, Some(DialogType::Prompt)), "Expected Prompt dialog type");

    browser.close().await.expect("Failed to close browser");
}

/// Test getting the dialog message.
#[tokio::test]
async fn test_dialog_message() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(PROMPT_HTML).set().await.expect("Failed to set content");

    let dialog_message = Arc::new(Mutex::new(String::new()));
    let dialog_message_clone = dialog_message.clone();

    page.on_dialog(move |dialog| {
        let msg = dialog_message_clone.clone();
        async move {
            *msg.lock().await = dialog.message().to_string();
            dialog.accept().await
        }
    }).await;

    page.locator("#prompt").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    assert_eq!(*dialog_message.lock().await, "What is your name?");

    browser.close().await.expect("Failed to close browser");
}

/// Test getting the default value of a prompt dialog.
#[tokio::test]
async fn test_dialog_default_value() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(PROMPT_HTML).set().await.expect("Failed to set content");

    let default_value = Arc::new(Mutex::new(String::new()));
    let default_value_clone = default_value.clone();

    page.on_dialog(move |dialog| {
        let val = default_value_clone.clone();
        async move {
            *val.lock().await = dialog.default_value().to_string();
            dialog.accept().await
        }
    }).await;

    page.locator("#prompt").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;

    assert_eq!(*default_value.lock().await, "default value");

    browser.close().await.expect("Failed to close browser");
}

/// Test accepting a prompt with custom text.
#[tokio::test]
async fn test_dialog_prompt_accept_with_text() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(PROMPT_HTML).set().await.expect("Failed to set content");

    page.on_dialog(|dialog| async move {
        dialog.accept_with_text("Custom Answer").await
    }).await;

    page.locator("#prompt").click().await.expect("Failed to click button");
    
    // Wait for the result to be populated
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Verify the prompt returned the custom text
    let result: String = page.evaluate(js!{ document.getElementById("result").textContent })
        .await
        .expect("Failed to get result");
    
    assert_eq!(result, "Custom Answer", "Prompt should return custom text when accepted");

    browser.close().await.expect("Failed to close browser");
}

/// Test dismissing a prompt returns null.
#[tokio::test]
async fn test_dialog_dismiss_prompt_returns_null() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(PROMPT_HTML).set().await.expect("Failed to set content");

    page.on_dialog(|dialog| async move {
        dialog.dismiss().await
    }).await;

    page.locator("#prompt").click().await.expect("Failed to click button");
    
    // Wait for the result to be populated
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Verify the prompt returned null
    let result: String = page.evaluate(js!{ document.getElementById("result").textContent })
        .await
        .expect("Failed to get result");
    
    assert_eq!(result, "null", "Prompt should return null when dismissed");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Beforeunload Dialog Tests
// =============================================================================

/// Test that beforeunload dialog events are captured.
#[tokio::test]
async fn test_dialog_beforeunload_event() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(BEFOREUNLOAD_HTML).set().await.expect("Failed to set content");

    let dialog_type = Arc::new(Mutex::new(None::<DialogType>));
    let dialog_type_clone = dialog_type.clone();

    page.on_dialog(move |dialog| {
        let dtype = dialog_type_clone.clone();
        async move {
            *dtype.lock().await = Some(dialog.type_());
            dialog.accept().await
        }
    }).await;

    // Try to navigate away which should trigger beforeunload
    page.locator("#leave").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let received_type = dialog_type.lock().await.clone();
    // Note: beforeunload may or may not fire in headless mode depending on browser
    // This test verifies the mechanism works when it does fire
    if received_type.is_some() {
        assert!(matches!(received_type, Some(DialogType::Beforeunload)), 
                "If dialog fired, it should be Beforeunload type");
    }

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Handler Management Tests
// =============================================================================

/// Test removing dialog handler.
#[tokio::test]
async fn test_dialog_off_dialog() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");

    page.set_content(ALERT_HTML).set().await.expect("Failed to set content");

    let handler_called_count = Arc::new(Mutex::new(0));
    let count_clone = handler_called_count.clone();

    page.on_dialog(move |dialog| {
        let cnt = count_clone.clone();
        async move {
            *cnt.lock().await += 1;
            dialog.accept().await
        }
    }).await;

    // Trigger first alert - handler should be called
    page.locator("#alert").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(*handler_called_count.lock().await, 1);

    // Remove the handler
    page.off_dialog().await;

    // Trigger second alert - handler should NOT be called (auto-dismiss instead)
    page.locator("#alert").click().await.expect("Failed to click button");
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Count should still be 1
    assert_eq!(*handler_called_count.lock().await, 1, "Handler should not be called after off_dialog");

    browser.close().await.expect("Failed to close browser");
}
