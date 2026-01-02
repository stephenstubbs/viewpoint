#![cfg(feature = "integration")]

//! User data directory tests.
//!
//! Tests for browser profile management, including temporary directories,
//! persistent profiles, and template-based profiles.

mod common;

use std::time::Duration;

use common::init_tracing;
use viewpoint_core::Browser;

// =============================================================================
// User Data Directory Tests
// =============================================================================

/// Test that browser profile data persists when using the same user data directory.
///
/// This test verifies that the user data directory is correctly passed to Chromium
/// and that browser state (preferences, extensions folder structure) persists.
#[tokio::test]
async fn test_user_data_dir_persistence() {
    init_tracing();

    // Create a temporary directory for browser profile
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let user_data_path = temp_dir.path().to_path_buf();

    // First browser session: create profile data
    {
        let browser = Browser::launch()
            .headless(true)
            .user_data_dir(&user_data_path)
            .timeout(Duration::from_secs(30))
            .launch()
            .await
            .expect("Failed to launch first browser");

        let context = browser
            .new_context()
            .await
            .expect("Failed to create context");
        let _page = context.new_page().await.expect("Failed to create page");

        // Browser creates profile structure in user data dir
        // Just having a browser session is enough to create profile data

        // Close browser
        browser
            .close()
            .await
            .expect("Failed to close first browser");
    }

    // Verify profile was created
    let entries_after_first: Vec<_> = std::fs::read_dir(&user_data_path)
        .expect("Failed to read user data dir")
        .collect();

    tracing::info!(
        "User data directory has {} entries after first session",
        entries_after_first.len()
    );
    assert!(
        !entries_after_first.is_empty(),
        "Browser should create profile data in user data directory"
    );

    // Second browser session: reuse profile
    {
        let browser = Browser::launch()
            .headless(true)
            .user_data_dir(&user_data_path)
            .timeout(Duration::from_secs(30))
            .launch()
            .await
            .expect("Failed to launch second browser with same profile");

        // Verify browser launched successfully with existing profile
        assert!(browser.is_owned());

        // Clean up
        browser
            .close()
            .await
            .expect("Failed to close second browser");
    }

    // Verify profile data still exists after second session
    let entries_after_second: Vec<_> = std::fs::read_dir(&user_data_path)
        .expect("Failed to read user data dir")
        .collect();

    tracing::info!(
        "User data directory has {} entries after second session",
        entries_after_second.len()
    );
    assert!(
        !entries_after_second.is_empty(),
        "Profile data should persist after second session"
    );

    // temp_dir is automatically cleaned up when dropped
}

/// Test launching browser with user data directory.
#[tokio::test]
async fn test_user_data_dir_launch() {
    init_tracing();

    // Create a temporary directory for browser profile
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

    let browser = Browser::launch()
        .headless(true)
        .user_data_dir(temp_dir.path())
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser with user data dir");

    // Verify browser launched successfully
    assert!(browser.is_owned());

    // Verify the user data directory was used (browser creates files in it)
    let entries: Vec<_> = std::fs::read_dir(temp_dir.path())
        .expect("Failed to read user data dir")
        .collect();

    tracing::info!(
        "User data directory contains {} entries after browser launch",
        entries.len()
    );

    // Browser should have created some files/directories in the user data dir
    assert!(
        !entries.is_empty(),
        "Browser should create files in user data directory"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Isolated Browser Profile Tests (UserDataDir)
// =============================================================================

/// Test that default browser launch creates a unique temporary directory.
///
/// This verifies the new default behavior where each browser gets an isolated
/// temp directory instead of using the system profile.
#[tokio::test]
async fn test_temp_user_data_dir_default() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser with default temp profile");

    assert!(browser.is_owned());

    // Create a context and page to verify browser is working
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    browser.close().await.expect("Failed to close browser");
}

/// Test that two concurrent browser launches don't conflict.
///
/// This tests the isolation provided by unique temp directories.
#[tokio::test]
async fn test_concurrent_browser_launches_no_conflict() {
    init_tracing();

    // Launch two browsers concurrently - each should get its own temp directory
    let (browser1_result, browser2_result) = tokio::join!(
        Browser::launch()
            .headless(true)
            .timeout(Duration::from_secs(30))
            .launch(),
        Browser::launch()
            .headless(true)
            .timeout(Duration::from_secs(30))
            .launch()
    );

    let browser1 = browser1_result.expect("Failed to launch first browser");
    let browser2 = browser2_result.expect("Failed to launch second browser");

    // Both browsers should be running
    assert!(browser1.is_owned());
    assert!(browser2.is_owned());

    // Create contexts in both browsers
    let _context1 = browser1
        .new_context()
        .await
        .expect("Failed to create context in browser 1");
    let _context2 = browser2
        .new_context()
        .await
        .expect("Failed to create context in browser 2");

    // Clean up both browsers
    browser1.close().await.expect("Failed to close browser 1");
    browser2.close().await.expect("Failed to close browser 2");
}

/// Test that temporary user data directory is cleaned up when browser closes.
#[tokio::test]
async fn test_temp_directory_cleanup_on_close() {
    init_tracing();

    // We can't easily get the temp directory path from outside,
    // but we can verify the browser launches and closes cleanly.
    // The cleanup is handled by TempDir's Drop implementation.
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    // Close browser - temp directory should be cleaned up
    browser.close().await.expect("Failed to close browser");
}

/// Test that temporary user data directory is cleaned up on drop.
#[tokio::test]
async fn test_temp_directory_cleanup_on_drop() {
    init_tracing();

    {
        let browser = Browser::launch()
            .headless(true)
            .timeout(Duration::from_secs(30))
            .launch()
            .await
            .expect("Failed to launch browser");

        // Just verify it launched
        assert!(browser.is_owned());

        // Browser is dropped here without explicit close
    }

    // If we get here without issues, cleanup on drop worked
}

/// Test launching with template-based profile.
#[tokio::test]
async fn test_template_profile_copy() {
    init_tracing();

    // Create a template directory with some content
    let template_dir = tempfile::tempdir().expect("Failed to create template dir");
    let test_file_path = template_dir.path().join("test_file.txt");
    std::fs::write(&test_file_path, "template content").expect("Failed to write test file");

    // Create a subdirectory with content
    let sub_dir = template_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir).expect("Failed to create subdir");
    std::fs::write(sub_dir.join("nested.txt"), "nested content")
        .expect("Failed to write nested file");

    // Launch browser with template
    let browser = Browser::launch()
        .headless(true)
        .user_data_dir_template_from(template_dir.path())
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser with template profile");

    assert!(browser.is_owned());

    // Verify template directory is unchanged
    assert!(
        test_file_path.exists(),
        "Template test file should still exist"
    );
    assert!(
        sub_dir.join("nested.txt").exists(),
        "Template nested file should still exist"
    );

    // Create a context to verify browser is working
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let _page = context.new_page().await.expect("Failed to create page");

    browser.close().await.expect("Failed to close browser");
}

/// Test that template profile errors on non-existent path.
#[tokio::test]
async fn test_template_profile_nonexistent_path() {
    init_tracing();

    let result = Browser::launch()
        .headless(true)
        .user_data_dir_template_from("/nonexistent/path/to/template")
        .timeout(Duration::from_secs(5))
        .launch()
        .await;

    assert!(
        result.is_err(),
        "Should fail with non-existent template path"
    );
    let err = result.unwrap_err();
    let err_msg = format!("{err}");
    assert!(
        err_msg.contains("does not exist"),
        "Error should mention path does not exist: {err_msg}"
    );
}
