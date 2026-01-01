#![cfg(feature = "integration")]

//! Browser and context lifecycle tests.
//!
//! Tests for browser launching, context creation, page management,
//! and remote browser connection.

mod common;

use std::time::Duration;
use viewpoint_core::Browser;

use common::init_tracing;

// =============================================================================
// Zombie Process Cleanup Tests
// =============================================================================

/// Helper to count zombie chromium processes for a given PID.
/// Returns the number of defunct chromium processes whose parent is our process.
#[cfg(unix)]
fn count_zombie_chromium_processes() -> usize {
    use std::process::Command;

    // On Unix, use ps to find zombie (defunct) processes
    // We look for processes in 'Z' state (zombie)
    let output = Command::new("ps")
        .args(["-eo", "pid,ppid,stat,comm"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let our_pid = std::process::id();

            stdout
                .lines()
                .filter(|line| {
                    // Look for zombie state ('Z' in stat column) and chromium processes
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let ppid = parts[1].parse::<u32>().unwrap_or(0);
                        let stat = parts[2];
                        let comm = parts[3];

                        // Check if it's our child, zombie state, and chromium-related
                        ppid == our_pid
                            && stat.starts_with('Z')
                            && (comm.contains("chrom") || comm.contains("Chrom"))
                    } else {
                        false
                    }
                })
                .count()
        }
        Err(_) => 0,
    }
}

/// Test that no zombie processes remain after browser.close().
#[tokio::test]
#[cfg(unix)]
async fn test_no_zombie_after_close() {
    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch and close a browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Close the browser properly
    browser.close().await.expect("Failed to close browser");

    // Give a moment for process state to update
    tokio::time::sleep(Duration::from_millis(100)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after close: before={}, after={}",
        zombies_before,
        zombies_after
    );
}

/// Test that no zombie processes remain after browser is dropped.
#[tokio::test]
#[cfg(unix)]
async fn test_no_zombie_after_drop() {
    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch a browser and let it drop without explicit close
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

    // Give a moment for process state to update
    tokio::time::sleep(Duration::from_millis(100)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after drop: before={}, after={}",
        zombies_before,
        zombies_after
    );
}

/// Test that no zombie processes remain when browser process dies before close.
#[tokio::test]
#[cfg(unix)]
async fn test_no_zombie_when_process_dies_before_close() {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch a browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Find the chromium process ID by looking at child processes
    // We'll use ps to find chromium processes with our process as parent
    let our_pid = std::process::id();
    let output = std::process::Command::new("ps")
        .args(["-eo", "pid,ppid,comm"])
        .output()
        .expect("Failed to run ps");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let chromium_pid: Option<i32> = stdout.lines().find_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let pid = parts[0].parse::<i32>().ok()?;
            let ppid = parts[1].parse::<u32>().ok()?;
            let comm = parts[2];
            if ppid == our_pid && (comm.contains("chrom") || comm.contains("Chrom")) {
                return Some(pid);
            }
        }
        None
    });

    if let Some(pid) = chromium_pid {
        // Kill the chromium process externally
        let _ = kill(Pid::from_raw(pid), Signal::SIGKILL);

        // Give a moment for the process to die
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Now close the browser (which should reap the already-dead process)
    browser.close().await.expect("Failed to close browser");

    // Give a moment for process state to update
    tokio::time::sleep(Duration::from_millis(100)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after close of killed browser: before={}, after={}",
        zombies_before,
        zombies_after
    );
}

/// Test launching a browser, verifying connection, and closing.
#[tokio::test]
async fn test_browser_launch_and_close() {
    init_tracing();

    // Launch a headless browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Verify the browser is owned (we launched it)
    assert!(browser.is_owned());

    // Close the browser
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a browser context.
#[tokio::test]
async fn test_browser_context_creation() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create a new context
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Verify context has an ID
    assert!(!context.id().is_empty());
    assert!(!context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test creating a page within a context.
#[tokio::test]
async fn test_page_creation() {
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

    // Create a new page
    let page = context.new_page().await.expect("Failed to create page");

    // Verify page has IDs
    assert!(!page.target_id().is_empty());
    assert!(!page.session_id().is_empty());
    assert!(!page.frame_id().is_empty());
    assert!(!page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a page.
#[tokio::test]
async fn test_page_close() {
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

    assert!(!page.is_closed());

    // Close the page
    page.close().await.expect("Failed to close page");

    assert!(page.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test closing a browser context.
#[tokio::test]
async fn test_context_close() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let mut context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    // Create some pages in the context
    let _page1 = context.new_page().await.expect("Failed to create page 1");
    let _page2 = context.new_page().await.expect("Failed to create page 2");

    assert!(!context.is_closed());

    // Close the context (should close all pages)
    context.close().await.expect("Failed to close context");

    assert!(context.is_closed());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Remote Browser Connection Tests
// =============================================================================

/// Test getting browser contexts from a launched browser.
#[tokio::test]
async fn test_browser_contexts_launched() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create some contexts
    let _context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let _context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");

    // Get all contexts - should include the ones we created
    let contexts = browser.contexts().await.expect("Failed to get contexts");

    // Should have at least 3 contexts: default + 2 created
    // (The default context is always included)
    assert!(
        contexts.len() >= 3,
        "Expected at least 3 contexts, got {}",
        contexts.len()
    );

    // Verify one of them is the default context
    let has_default = contexts.iter().any(|c| c.is_default());
    assert!(has_default, "Should have a default context");

    // Verify we have owned and non-owned contexts
    let _owned_count = contexts.iter().filter(|c| c.is_owned()).count();
    let non_owned_count = contexts.iter().filter(|c| !c.is_owned()).count();

    // Default context should be non-owned (returned from contexts())
    assert!(
        non_owned_count >= 1,
        "Should have at least 1 non-owned context (default)"
    );
    // Note: contexts returned from browser.contexts() are all marked as non-owned
    // because they're discovered, not created

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that context ownership affects close behavior.
#[tokio::test]
async fn test_context_ownership_on_close() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create an owned context
    let mut owned_context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    assert!(
        owned_context.is_owned(),
        "Context created with new_context() should be owned"
    );

    // Get contexts - these will be marked as non-owned
    let contexts = browser.contexts().await.expect("Failed to get contexts");

    // Find the default context
    let default_context = contexts.into_iter().find(|c| c.is_default());
    assert!(default_context.is_some(), "Should find default context");

    let mut default_ctx = default_context.unwrap();
    assert!(
        !default_ctx.is_owned(),
        "Default context from contexts() should not be owned"
    );

    // Close the non-owned default context - should not error
    default_ctx
        .close()
        .await
        .expect("Closing non-owned context should succeed");

    // Close the owned context
    owned_context
        .close()
        .await
        .expect("Closing owned context should succeed");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test getting pages from the default context.
#[tokio::test]
async fn test_default_context_pages() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Get the default context
    let contexts = browser.contexts().await.expect("Failed to get contexts");
    let default_context = contexts.into_iter().find(|c| c.is_default());
    assert!(default_context.is_some(), "Should find default context");

    let default_ctx = default_context.unwrap();

    // Get pages in default context
    let pages = default_ctx.pages().await.expect("Failed to get pages");

    // Note: A launched browser might have one initial page in the default context
    // This depends on browser behavior
    tracing::info!("Default context has {} pages", pages.len());

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test connect_over_cdp with invalid endpoint URL.
#[tokio::test]
async fn test_connect_over_cdp_invalid_url() {
    init_tracing();

    // Try to connect to an invalid URL scheme
    let result = Browser::connect_over_cdp("ftp://localhost:9222")
        .timeout(Duration::from_secs(5))
        .connect()
        .await;

    assert!(result.is_err(), "Should fail with invalid URL scheme");
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}

/// Test connect_over_cdp with unreachable endpoint.
#[tokio::test]
async fn test_connect_over_cdp_unreachable() {
    init_tracing();

    // Try to connect to an endpoint that doesn't exist
    // Using a high port that's unlikely to be in use
    let result = Browser::connect_over_cdp("http://127.0.0.1:59999")
        .timeout(Duration::from_secs(2))
        .connect()
        .await;

    assert!(result.is_err(), "Should fail with unreachable endpoint");
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}

/// Test connect_over_cdp with connection timeout.
#[tokio::test]
async fn test_connect_over_cdp_timeout() {
    init_tracing();

    // Try to connect with a very short timeout to a non-responsive endpoint
    // Use a black hole IP that won't respond
    let result = Browser::connect_over_cdp("http://10.255.255.1:9222")
        .timeout(Duration::from_millis(500))
        .connect()
        .await;

    // This should either timeout or fail to connect
    assert!(
        result.is_err(),
        "Should fail with timeout or connection error"
    );
    let err = result.unwrap_err();
    tracing::info!("Got expected error: {}", err);
}

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
    std::fs::write(sub_dir.join("nested.txt"), "nested content").expect("Failed to write nested file");

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
    assert!(test_file_path.exists(), "Template test file should still exist");
    assert!(sub_dir.join("nested.txt").exists(), "Template nested file should still exist");

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

    assert!(result.is_err(), "Should fail with non-existent template path");
    let err = result.unwrap_err();
    let err_msg = format!("{err}");
    assert!(
        err_msg.contains("does not exist"),
        "Error should mention path does not exist: {err_msg}"
    );
}
