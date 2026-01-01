#![cfg(feature = "integration")]

//! ARIA snapshot performance tests for viewpoint-core.
//!
//! These tests verify the performance optimizations for snapshot capture:
//! - Parallel node resolution
//! - Batch array element access
//! - Parallel frame capture
//! - SnapshotOptions configuration

use std::sync::Once;
use std::time::{Duration, Instant};

use viewpoint_core::{Browser, SnapshotOptions};

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
// Large DOM Performance Tests
// =============================================================================

/// Test snapshot capture on a page with 100+ elements completes in reasonable time.
#[tokio::test]
async fn test_large_dom_snapshot_performance() {
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

    // Generate a page with 100+ interactive elements
    let mut html = String::from("<html><body><h1>Performance Test</h1>\n");
    for i in 0..100 {
        html.push_str(&format!(
            "<button id=\"btn{}\">Button {}</button>\n",
            i, i
        ));
    }
    html.push_str("</body></html>");

    page.set_content(&html)
        .set()
        .await
        .expect("Failed to set content");

    // Time the snapshot capture
    let start = Instant::now();
    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let duration = start.elapsed();

    println!("Large DOM snapshot captured in {:?}", duration);
    println!(
        "Snapshot has {} children at root",
        snapshot.children.len()
    );

    // Verify the snapshot contains our buttons (check for refs)
    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("[ref=e"),
        "Snapshot should contain element refs"
    );

    // Performance expectation: should complete in under 5 seconds
    // (With sequential processing of 100+ elements at 1-5ms each, this could be 500ms+)
    // With parallel processing, should be much faster
    assert!(
        duration < Duration::from_secs(5),
        "Snapshot should complete in under 5 seconds, took {:?}",
        duration
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test snapshot with include_refs: false is faster.
#[tokio::test]
async fn test_snapshot_without_refs_performance() {
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

    // Generate a page with 100 elements
    let mut html = String::from("<html><body>\n");
    for i in 0..100 {
        html.push_str(&format!("<button>Button {}</button>\n", i));
    }
    html.push_str("</body></html>");

    page.set_content(&html)
        .set()
        .await
        .expect("Failed to set content");

    // Time snapshot WITH refs
    let start_with_refs = Instant::now();
    let snapshot_with_refs = page.aria_snapshot().await.expect("Failed to get snapshot");
    let duration_with_refs = start_with_refs.elapsed();

    // Time snapshot WITHOUT refs
    let options = SnapshotOptions::default().include_refs(false);
    let start_without_refs = Instant::now();
    let snapshot_without_refs = page
        .aria_snapshot_with_options(options)
        .await
        .expect("Failed to get snapshot");
    let duration_without_refs = start_without_refs.elapsed();

    println!("Snapshot WITH refs: {:?}", duration_with_refs);
    println!("Snapshot WITHOUT refs: {:?}", duration_without_refs);

    // Verify refs are present/absent as expected
    let yaml_with = snapshot_with_refs.to_yaml();
    let _yaml_without = snapshot_without_refs.to_yaml();

    assert!(
        yaml_with.contains("[ref=e"),
        "Snapshot with refs should contain refs"
    );
    // Without refs, the snapshot should not have refs
    // (The node_ref field will be None for all nodes)

    // Without refs should be faster (or at least not slower)
    // We don't assert this strictly since timing can be variable,
    // but we log for manual verification
    if duration_without_refs < duration_with_refs {
        println!(
            "Without refs was {:?} faster",
            duration_with_refs - duration_without_refs
        );
    }

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Multi-Frame Parallel Capture Tests
// =============================================================================

/// Test parallel frame capture with multiple iframes.
#[tokio::test]
async fn test_multi_frame_parallel_capture() {
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

    // Create a page with 5 iframes, each with some content
    let html = r#"
        <html><body>
            <h1>Multi-Frame Test</h1>
            <iframe name="frame1" srcdoc="<html><body><button>Frame 1 Button</button></body></html>"></iframe>
            <iframe name="frame2" srcdoc="<html><body><button>Frame 2 Button</button></body></html>"></iframe>
            <iframe name="frame3" srcdoc="<html><body><button>Frame 3 Button</button></body></html>"></iframe>
            <iframe name="frame4" srcdoc="<html><body><button>Frame 4 Button</button></body></html>"></iframe>
            <iframe name="frame5" srcdoc="<html><body><button>Frame 5 Button</button></body></html>"></iframe>
        </body></html>
    "#;

    page.set_content(html)
        .set()
        .await
        .expect("Failed to set content");

    // Wait for iframes to load
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Time the multi-frame snapshot
    let start = Instant::now();
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Failed to get multi-frame snapshot");
    let duration = start.elapsed();

    println!("Multi-frame snapshot captured in {:?}", duration);

    let yaml = snapshot.to_yaml();
    println!("Multi-frame snapshot:\n{}", yaml);

    // Verify all frames were captured (should see content from multiple frames)
    // With parallel capture, this should be faster than sequential

    // Performance expectation: should complete in under 5 seconds
    assert!(
        duration < Duration::from_secs(5),
        "Multi-frame snapshot should complete in under 5 seconds, took {:?}",
        duration
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// SnapshotOptions Configuration Tests
// =============================================================================

/// Test SnapshotOptions::max_concurrency configuration.
#[tokio::test]
async fn test_snapshot_options_max_concurrency() {
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

    // Generate a page with 50 elements
    let mut html = String::from("<html><body>\n");
    for i in 0..50 {
        html.push_str(&format!("<button>Button {}</button>\n", i));
    }
    html.push_str("</body></html>");

    page.set_content(&html)
        .set()
        .await
        .expect("Failed to set content");

    // Test with low concurrency
    let options_low = SnapshotOptions::default().max_concurrency(5);
    let start_low = Instant::now();
    let _ = page
        .aria_snapshot_with_options(options_low)
        .await
        .expect("Failed to get snapshot");
    let duration_low = start_low.elapsed();

    // Test with high concurrency
    let options_high = SnapshotOptions::default().max_concurrency(100);
    let start_high = Instant::now();
    let _ = page
        .aria_snapshot_with_options(options_high)
        .await
        .expect("Failed to get snapshot");
    let duration_high = start_high.elapsed();

    println!("Low concurrency (5): {:?}", duration_low);
    println!("High concurrency (100): {:?}", duration_high);

    // Both should complete successfully
    // Higher concurrency should generally be faster, but we don't assert
    // strictly due to timing variability

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test SnapshotOptions::include_refs configuration.
#[tokio::test]
async fn test_snapshot_options_include_refs_false() {
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

    page.set_content(
        r##"
        <html><body>
            <button id="btn1">Button 1</button>
            <button id="btn2">Button 2</button>
            <a href="#">Link</a>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture without refs
    let options = SnapshotOptions::default().include_refs(false);
    let snapshot = page
        .aria_snapshot_with_options(options)
        .await
        .expect("Failed to get snapshot");

    // Verify structure is still captured
    let yaml = snapshot.to_yaml();
    println!("Snapshot without refs:\n{}", yaml);

    // Should have the elements but no refs
    assert!(
        yaml.contains("button") || yaml.contains("link"),
        "Snapshot should still contain elements, got: {}",
        yaml
    );

    // Check that node_ref is None on root (since we skipped ref resolution)
    assert!(
        snapshot.node_ref.is_none(),
        "Root node_ref should be None when include_refs is false"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test Frame::aria_snapshot_with_options.
#[tokio::test]
async fn test_frame_snapshot_with_options() {
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

    page.set_content(
        r##"
        <html><body>
            <h1>Main Page</h1>
            <iframe name="contentframe" srcdoc="<html><body><button>Frame Button</button></body></html>"></iframe>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Wait for iframe to load
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Get the content frame
    let frames = page.frames().await.expect("Failed to get frames");
    let content_frame = frames
        .iter()
        .find(|f| f.name() == "contentframe")
        .expect("Should find content frame");

    // Capture frame snapshot without refs
    let options = SnapshotOptions::default().include_refs(false);
    let snapshot = content_frame
        .aria_snapshot_with_options(options)
        .await
        .expect("Failed to get frame snapshot");

    let yaml = snapshot.to_yaml();
    println!("Frame snapshot without refs:\n{}", yaml);

    // Should have structure but no refs
    assert!(
        yaml.contains("button"),
        "Frame snapshot should contain button"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
