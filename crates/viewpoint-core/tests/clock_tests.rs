#![cfg(feature = "integration")]

//! Clock mocking tests for viewpoint-core.
//!
//! These tests verify clock installation, time mocking, and timer control.

use std::sync::Once;
use std::time::Duration;

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
// Clock Date Mocking Tests
// =============================================================================

/// Test clock date mocking with fixed time.
#[tokio::test]
async fn test_clock_date_mocking() {
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

    // Set up a simple page
    page.set_content("<html><body><h1>Clock Test</h1></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Install clock
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");

    // Set a fixed time: January 1, 2024 00:00:00 UTC
    clock
        .set_fixed_time("2024-01-01T00:00:00Z")
        .await
        .expect("Failed to set fixed time");

    // Get Date.now() and verify it's the expected timestamp
    let timestamp: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");

    // Jan 1, 2024 00:00:00 UTC = 1_704_067_200_000 ms
    assert_eq!(timestamp as i64, 1_704_067_200_000);

    // Verify new Date() also returns the fixed time
    let date_string: String = page
        .evaluate(js! { new Date().toISOString() })
        .await
        .expect("Failed to get date string");
    assert!(date_string.starts_with("2024-01-01T00:00:00"));

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Clock Timer Mocking Tests
// =============================================================================

/// Test clock time advancement with timers.
#[tokio::test]
async fn test_clock_timer_mocking() {
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

    // Set up a page with a variable that will be set by setTimeout
    page.set_content("<html><body><script>window.timerFired = false;</script></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Install clock
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");

    // Set up a timer that fires after 5 seconds
    let _: f64 = page
        .evaluate(js! { setTimeout(() => { window.timerFired = true; }, 5000) })
        .await
        .expect("Failed to set timer");

    // Timer should not have fired yet
    let fired_before: bool = page
        .evaluate(js! { window.timerFired })
        .await
        .expect("Failed to check timer");
    assert!(!fired_before, "Timer should not have fired yet");

    // Advance time by 5 seconds
    let timers_fired = clock
        .run_for(Duration::from_secs(5))
        .await
        .expect("Failed to run for duration");

    assert!(timers_fired >= 1, "At least one timer should have fired");

    // Timer should have fired now
    let fired_after: bool = page
        .evaluate(js! { window.timerFired })
        .await
        .expect("Failed to check timer");
    assert!(fired_after, "Timer should have fired after advancing time");

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clock fast-forward (skip time without firing timers).
#[tokio::test]
async fn test_clock_fast_forward() {
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

    page.set_content("<html><body><script>window.timerFired = false;</script></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Install clock and set fixed time
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");
    clock
        .set_fixed_time(1_704_067_200_000i64)
        .await
        .expect("Failed to set time");

    // Get initial time
    let time_before: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");

    // Fast-forward 1 hour
    clock
        .fast_forward(Duration::from_secs(3600))
        .await
        .expect("Failed to fast forward");

    // Time should have advanced by 1 hour
    let time_after: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");
    let expected_time = time_before + 3_600.0 * 1_000.0;
    assert!(
        (time_after - expected_time).abs() < 1_000.0,
        "Time should have advanced by ~1 hour. Before: {time_before}, After: {time_after}"
    );

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clock run_all_timers.
#[tokio::test]
async fn test_clock_run_all_timers() {
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

    page.set_content("<html><body><script>window.count = 0;</script></body></html>")
        .set()
        .await
        .expect("Failed to set content");

    // Install clock
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");

    // Set up multiple timers at different times
    let _: serde_json::Value = page
        .evaluate(js! {
            setTimeout(() => { window.count = window.count + 1; }, 1000);
            setTimeout(() => { window.count = window.count + 1; }, 2000);
            setTimeout(() => { window.count = window.count + 1; }, 5000);
        })
        .await
        .expect("Failed to set timers");

    // Check pending timer count
    let pending = clock
        .pending_timer_count()
        .await
        .expect("Failed to get count");
    assert_eq!(pending, 3, "Should have 3 pending timers");

    // Run all timers
    let fired = clock.run_all_timers().await.expect("Failed to run timers");
    assert!(fired >= 3, "Should have fired at least 3 timers");

    // Count should be 3
    let count: f64 = page
        .evaluate(js! { window.count })
        .await
        .expect("Failed to get count");
    assert_eq!(count as i32, 3, "All timers should have incremented count");

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Clock System Time Tests
// =============================================================================

/// Test clock system time (flowing time).
#[tokio::test]
async fn test_clock_system_time() {
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

    // Install clock
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");

    // Set system time (flows normally)
    clock
        .set_system_time("2024-06-15T12:00:00Z")
        .await
        .expect("Failed to set system time");

    // Get initial time
    let time1: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");

    // Wait a bit (real time)
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Get time again - it should have advanced with real time
    let time2: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");

    // Time should have advanced (allowing some tolerance)
    assert!(
        time2 > time1,
        "System time should flow: {time2} should be > {time1}"
    );

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test clock pause and resume.
#[tokio::test]
async fn test_clock_pause_resume() {
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

    // Install clock
    let mut clock = page.clock();
    clock.install().await.expect("Failed to install clock");

    // Pause at a specific time
    clock
        .pause_at("2024-01-01T12:00:00Z")
        .await
        .expect("Failed to pause");

    // Get time
    let time1: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");

    // Wait a bit
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Time should still be the same (paused)
    let time2: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");
    // Testing exact float values for clock pausing
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(time1, time2, "Time should be paused");
    }

    // Resume
    clock.resume().await.expect("Failed to resume");

    // Wait a bit
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Time should now be advancing
    let time3: f64 = page
        .evaluate(js! { Date.now() })
        .await
        .expect("Failed to get time");
    assert!(time3 > time2, "Time should be flowing after resume");

    // Uninstall clock
    clock.uninstall().await.expect("Failed to uninstall clock");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
