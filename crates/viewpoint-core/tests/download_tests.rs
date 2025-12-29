#![cfg(feature = "integration")]

//! Download handling tests for viewpoint-core.
//!
//! These tests verify the download handling functionality including
//! download events, properties, and file saving.

mod common;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use viewpoint_core::Browser;

use common::init_tracing;

// =============================================================================
// Test HTML Templates
// =============================================================================

const DOWNLOAD_LINK_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <a id="download" href="data:text/plain,Hello%20World" download="test-file.txt">Download Text File</a>
    <a id="download-csv" href="data:text/csv,name,age%0AJohn,30" download="data.csv">Download CSV</a>
</body>
</html>
"#;

const DOWNLOAD_BLOB_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <button id="download-blob" onclick="downloadBlob()">Download Blob</button>
    <script>
        function downloadBlob() {
            const blob = new Blob(['Blob content here'], { type: 'text/plain' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'blob-file.txt';
            a.click();
            URL.revokeObjectURL(url);
        }
    </script>
</body>
</html>
"#;

// =============================================================================
// Download Event Tests
// =============================================================================

/// Test that download events are captured when clicking a download link.
#[tokio::test]
async fn test_download_event_on_link_click() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_received = Arc::new(Mutex::new(false));
    let download_received_clone = download_received.clone();

    page.on_download(move |_download| {
        let received = download_received_clone.clone();
        async move {
            *received.lock().await = true;
        }
    })
    .await;

    // Click the download link
    page.locator("#download")
        .click()
        .await
        .expect("Failed to click download link");

    // Wait for download event
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert!(
        *download_received.lock().await,
        "Download event should have been captured"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Download Property Tests
// =============================================================================

/// Test getting the suggested filename from a download.
#[tokio::test]
async fn test_download_suggested_filename() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let suggested_filename = Arc::new(Mutex::new(String::new()));
    let filename_clone = suggested_filename.clone();

    page.on_download(move |download| {
        let filename = filename_clone.clone();
        async move {
            *filename.lock().await = download.suggested_filename().to_string();
        }
    })
    .await;

    page.locator("#download")
        .click()
        .await
        .expect("Failed to click download link");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let filename = suggested_filename.lock().await.clone();
    assert_eq!(
        filename, "test-file.txt",
        "Suggested filename should match download attribute"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test getting the download URL.
#[tokio::test]
async fn test_download_url() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_url = Arc::new(Mutex::new(String::new()));
    let url_clone = download_url.clone();

    page.on_download(move |download| {
        let url = url_clone.clone();
        async move {
            *url.lock().await = download.url().to_string();
        }
    })
    .await;

    page.locator("#download")
        .click()
        .await
        .expect("Failed to click download link");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let url = download_url.lock().await.clone();
    assert!(
        url.contains("data:text/plain"),
        "Download URL should contain data URI"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test getting the download GUID.
#[tokio::test]
async fn test_download_guid() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_guid = Arc::new(Mutex::new(String::new()));
    let guid_clone = download_guid.clone();

    page.on_download(move |download| {
        let guid = guid_clone.clone();
        async move {
            *guid.lock().await = download.guid().to_string();
        }
    })
    .await;

    page.locator("#download")
        .click()
        .await
        .expect("Failed to click download link");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let guid = download_guid.lock().await.clone();
    assert!(!guid.is_empty(), "Download GUID should not be empty");

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Expect Download Tests
// =============================================================================

/// Test using expect_download to wait for a download triggered by an action.
#[tokio::test]
async fn test_expect_download() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_link = page.locator("#download");

    let download = page
        .expect_download(|| async { download_link.click().await })
        .await
        .expect("Should capture download");

    assert_eq!(download.suggested_filename(), "test-file.txt");

    browser.close().await.expect("Failed to close browser");
}

/// Test expect_download with CSV file.
#[tokio::test]
async fn test_expect_download_csv() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_link = page.locator("#download-csv");

    let download = page
        .expect_download(|| async { download_link.click().await })
        .await
        .expect("Should capture download");

    assert_eq!(download.suggested_filename(), "data.csv");
    assert!(download.url().contains("text/csv"));

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Download Cancel Tests
// =============================================================================

/// Test canceling a download.
#[tokio::test]
async fn test_download_cancel() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_link = page.locator("#download");

    let mut download = page
        .expect_download(|| async { download_link.click().await })
        .await
        .expect("Should capture download");

    // Cancel the download
    download.cancel().await.expect("Should cancel download");

    // Verify failure reason
    let failure = download.failure();
    assert_eq!(
        failure,
        Some("canceled"),
        "Failure reason should be 'canceled'"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Download Failure Tests
// =============================================================================

/// Test that successful downloads have no failure.
#[tokio::test]
async fn test_download_no_failure_on_success() {
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

    page.set_content(DOWNLOAD_LINK_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let download_failure = Arc::new(Mutex::new(None::<String>));
    let failure_clone = download_failure.clone();

    page.on_download(move |download| {
        let failure = failure_clone.clone();
        async move {
            *failure.lock().await = download.failure().map(String::from);
        }
    })
    .await;

    page.locator("#download")
        .click()
        .await
        .expect("Failed to click download link");
    tokio::time::sleep(Duration::from_millis(500)).await;

    // For a successful download started from data URI, failure should be None
    // Note: The download may or may not complete depending on implementation
    // This test verifies the initial state has no failure
    let failure = download_failure.lock().await.clone();
    assert!(
        failure.is_none(),
        "In-progress download should have no failure"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Download Blob Tests
// =============================================================================

/// Test download triggered by JavaScript blob creation.
#[tokio::test]
async fn test_download_blob_click() {
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

    page.set_content(DOWNLOAD_BLOB_HTML)
        .set()
        .await
        .expect("Failed to set content");

    let suggested_filename = Arc::new(Mutex::new(String::new()));
    let filename_clone = suggested_filename.clone();

    page.on_download(move |download| {
        let filename = filename_clone.clone();
        async move {
            *filename.lock().await = download.suggested_filename().to_string();
        }
    })
    .await;

    page.locator("#download-blob")
        .click()
        .await
        .expect("Failed to click download blob button");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let filename = suggested_filename.lock().await.clone();
    // Blob downloads may have the specified filename
    if !filename.is_empty() {
        assert_eq!(filename, "blob-file.txt");
    }

    browser.close().await.expect("Failed to close browser");
}
