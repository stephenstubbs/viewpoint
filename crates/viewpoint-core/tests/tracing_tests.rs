#![cfg(feature = "integration")]

//! Tracing integration tests.
//!
//! Tests for trace recording, state persistence, and error handling.

mod common;

use std::path::PathBuf;
use tempfile::TempDir;
use viewpoint_core::context::TracingOptions;

/// Helper to get a temp directory for trace files.
fn temp_trace_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Helper to get a trace file path in a temp directory.
fn temp_trace_path(dir: &TempDir, name: &str) -> PathBuf {
    dir.path().join(name)
}

/// Test basic start and stop workflow.
#[tokio::test]
async fn test_tracing_start_stop() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;
    let temp_dir = temp_trace_dir();
    let trace_path = temp_trace_path(&temp_dir, "trace.zip");

    // Start tracing
    context
        .tracing()
        .start(TracingOptions::new().name("test-trace"))
        .await
        .expect("Failed to start tracing");

    // Perform some action
    page.goto("about:blank")
        .goto()
        .await
        .expect("Failed to navigate");

    // Stop tracing - using a separate tracing() call to verify state persistence
    context
        .tracing()
        .stop(&trace_path)
        .await
        .expect("Failed to stop tracing");

    // Verify trace file was created
    assert!(trace_path.exists(), "Trace file should exist");

    browser.close().await.expect("Failed to close browser");
}

/// Test start and discard without saving.
#[tokio::test]
async fn test_tracing_start_stop_discard() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;

    // Start tracing
    context
        .tracing()
        .start(TracingOptions::new())
        .await
        .expect("Failed to start tracing");

    // Perform some action
    page.goto("about:blank")
        .goto()
        .await
        .expect("Failed to navigate");

    // Discard tracing - verify state persists across tracing() calls
    context
        .tracing()
        .stop_discard()
        .await
        .expect("Failed to discard tracing");

    // Verify is_recording is false after discard
    assert!(
        !context.tracing().is_recording().await,
        "Should not be recording after discard"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test start_chunk and stop_chunk workflow.
#[tokio::test]
async fn test_tracing_chunks() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;
    let temp_dir = temp_trace_dir();
    let chunk_path = temp_trace_path(&temp_dir, "chunk.zip");

    // Start tracing
    context
        .tracing()
        .start(TracingOptions::new())
        .await
        .expect("Failed to start tracing");

    // Perform some action
    page.goto("about:blank")
        .goto()
        .await
        .expect("Failed to navigate");

    // Start a new chunk
    context
        .tracing()
        .start_chunk()
        .await
        .expect("Failed to start chunk");

    // Stop chunk and save
    context
        .tracing()
        .stop_chunk(&chunk_path)
        .await
        .expect("Failed to stop chunk");

    // Verify chunk file was created
    assert!(chunk_path.exists(), "Chunk file should exist");

    // Discard remaining trace
    context
        .tracing()
        .stop_discard()
        .await
        .expect("Failed to discard tracing");

    browser.close().await.expect("Failed to close browser");
}

/// Test is_recording() returns correct state.
#[tokio::test]
async fn test_tracing_is_recording() {
    common::init_tracing();

    let (browser, context, _page) = common::launch_with_page().await;

    // Initially not recording
    assert!(
        !context.tracing().is_recording().await,
        "Should not be recording initially"
    );

    // Start tracing
    context
        .tracing()
        .start(TracingOptions::new())
        .await
        .expect("Failed to start tracing");

    // Now should be recording - verify state persists across tracing() calls
    assert!(
        context.tracing().is_recording().await,
        "Should be recording after start"
    );

    // Stop tracing
    context
        .tracing()
        .stop_discard()
        .await
        .expect("Failed to stop tracing");

    // No longer recording
    assert!(
        !context.tracing().is_recording().await,
        "Should not be recording after stop"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that starting tracing without pages fails.
#[tokio::test]
async fn test_tracing_start_without_pages_fails() {
    common::init_tracing();

    let browser = common::launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Attempt to start tracing without creating a page
    let result = context.tracing().start(TracingOptions::new()).await;

    assert!(result.is_err(), "Should fail without pages");
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("no pages in context"),
        "Error message should mention no pages: {err_msg}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that stopping without starting fails.
#[tokio::test]
async fn test_tracing_stop_without_start_fails() {
    common::init_tracing();

    let (browser, context, _page) = common::launch_with_page().await;
    let temp_dir = temp_trace_dir();
    let trace_path = temp_trace_path(&temp_dir, "trace.zip");

    // Attempt to stop without starting
    let result = context.tracing().stop(&trace_path).await;

    assert!(result.is_err(), "Should fail without starting");
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("not active"),
        "Error message should mention tracing not active: {err_msg}"
    );

    browser.close().await.expect("Failed to close browser");
}

/// Test that double start fails.
#[tokio::test]
async fn test_tracing_double_start_fails() {
    common::init_tracing();

    let (browser, context, _page) = common::launch_with_page().await;

    // Start tracing
    context
        .tracing()
        .start(TracingOptions::new())
        .await
        .expect("Failed to start tracing");

    // Attempt to start again - should fail
    let result = context.tracing().start(TracingOptions::new()).await;

    assert!(result.is_err(), "Double start should fail");
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("already active"),
        "Error message should mention already active: {err_msg}"
    );

    // Clean up
    context
        .tracing()
        .stop_discard()
        .await
        .expect("Failed to discard tracing");

    browser.close().await.expect("Failed to close browser");
}

/// Test tracing with screenshots enabled.
#[tokio::test]
async fn test_tracing_with_screenshots() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;
    let temp_dir = temp_trace_dir();
    let trace_path = temp_trace_path(&temp_dir, "trace-screenshots.zip");

    // Start tracing with screenshots
    context
        .tracing()
        .start(
            TracingOptions::new()
                .name("screenshot-test")
                .screenshots(true)
                .snapshots(true),
        )
        .await
        .expect("Failed to start tracing with screenshots");

    // Perform navigation
    page.goto("about:blank")
        .goto()
        .await
        .expect("Failed to navigate");

    // Stop and save
    context
        .tracing()
        .stop(&trace_path)
        .await
        .expect("Failed to stop tracing");

    // Verify trace file exists
    assert!(trace_path.exists(), "Trace file should exist");

    browser.close().await.expect("Failed to close browser");
}

/// Test that trace creates a valid zip file.
#[tokio::test]
async fn test_tracing_creates_valid_zip() {
    common::init_tracing();

    let (browser, context, page) = common::launch_with_page().await;
    let temp_dir = temp_trace_dir();
    let trace_path = temp_trace_path(&temp_dir, "trace-valid.zip");

    // Start and stop tracing
    context
        .tracing()
        .start(TracingOptions::new().name("zip-test"))
        .await
        .expect("Failed to start tracing");

    page.goto("about:blank")
        .goto()
        .await
        .expect("Failed to navigate");

    context
        .tracing()
        .stop(&trace_path)
        .await
        .expect("Failed to stop tracing");

    // Verify it's a valid zip file
    let file = std::fs::File::open(&trace_path).expect("Failed to open trace file");
    let archive = zip::ZipArchive::new(file);
    assert!(archive.is_ok(), "Trace should be a valid zip file");

    let mut archive = archive.unwrap();

    // Check for expected contents
    let file_names: Vec<String> = (0..archive.len())
        .map(|i| archive.by_index(i).unwrap().name().to_string())
        .collect();

    assert!(
        file_names.iter().any(|n| n.contains("trace")),
        "Zip should contain trace data: {:?}",
        file_names
    );

    browser.close().await.expect("Failed to close browser");
}
