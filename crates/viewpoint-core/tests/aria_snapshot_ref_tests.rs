#![cfg(feature = "integration")]

//! ARIA snapshot ref isolation tests.
//!
//! These tests verify that refs are properly scoped to their context and page.
//! Refs from one context/page cannot be resolved on a different context/page.

use std::sync::Once;

use viewpoint_core::Browser;

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
// Context Isolation Tests
// =============================================================================

/// Test that refs captured on one context cannot be resolved on a different context.
///
/// This verifies context isolation - refs are scoped to their originating context.
#[tokio::test]
async fn test_ref_rejected_on_wrong_context() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create first context and capture a ref
    let context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let page1 = context1.new_page().await.expect("Failed to create page 1");

    page1
        .set_content(r#"<html><body><button id="btn1">Button in Context 1</button></body></html>"#)
        .set()
        .await
        .expect("Failed to set content");

    let snapshot1 = page1
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot from context 1");

    // Find button ref from context 1
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let context1_button_ref =
        find_button_ref(&snapshot1).expect("Should find button ref in context 1");
    println!("Context 1 button ref: {context1_button_ref}");

    // Verify the ref works on context 1's page
    let handle = page1
        .element_from_ref(&context1_button_ref)
        .await
        .expect("Ref should resolve on correct context");
    assert!(
        handle.is_attached().await.unwrap(),
        "Element should be attached"
    );

    // Create second context with a different page
    let context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");
    let page2 = context2.new_page().await.expect("Failed to create page 2");

    page2
        .set_content(r#"<html><body><button id="btn2">Button in Context 2</button></body></html>"#)
        .set()
        .await
        .expect("Failed to set content");

    // Try to resolve context 1's ref on context 2's page - should fail
    let result = page2.element_from_ref(&context1_button_ref).await;

    assert!(
        result.is_err(),
        "Ref from context 1 should NOT be resolvable on context 2"
    );

    // Verify the error message mentions context mismatch
    let err_msg = result.unwrap_err().to_string();
    println!("Error message: {err_msg}");
    assert!(
        err_msg.contains("Context index mismatch") || err_msg.contains("context"),
        "Error should mention context mismatch, got: {err_msg}"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that context indices are assigned correctly across multiple contexts.
#[tokio::test]
async fn test_context_indices_assigned_correctly() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create multiple contexts and verify they get unique indices
    let context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");
    let context3 = browser
        .new_context()
        .await
        .expect("Failed to create context 3");

    // Indices should be unique (not necessarily sequential due to other tests)
    let idx1 = context1.index();
    let idx2 = context2.index();
    let idx3 = context3.index();

    assert_ne!(idx1, idx2, "Context 1 and 2 should have different indices");
    assert_ne!(idx2, idx3, "Context 2 and 3 should have different indices");
    assert_ne!(idx1, idx3, "Context 1 and 3 should have different indices");

    println!("Context indices: {idx1} {idx2} {idx3}");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs from different contexts have different prefixes.
#[tokio::test]
async fn test_refs_from_different_contexts_have_different_prefixes() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    let context1 = browser
        .new_context()
        .await
        .expect("Failed to create context 1");
    let context2 = browser
        .new_context()
        .await
        .expect("Failed to create context 2");

    let page1 = context1.new_page().await.expect("Failed to create page 1");
    let page2 = context2.new_page().await.expect("Failed to create page 2");

    // Set same content on both pages
    let html = "<html><body><button>Test Button</button></body></html>";
    page1
        .set_content(html)
        .set()
        .await
        .expect("Failed to set content");
    page2
        .set_content(html)
        .set()
        .await
        .expect("Failed to set content");

    // Capture snapshots
    let snapshot1 = page1
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot 1");
    let snapshot2 = page2
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot 2");

    // Find button refs
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let ref1 = find_button_ref(&snapshot1).expect("Should find button ref in snapshot 1");
    let ref2 = find_button_ref(&snapshot2).expect("Should find button ref in snapshot 2");

    println!("Ref from context 1: {ref1}");
    println!("Ref from context 2: {ref2}");

    // Refs should have different context prefixes
    assert_ne!(
        ref1, ref2,
        "Refs from different contexts should be different"
    );

    // Extract context index from refs (format: c{ctx}p{page}f{frame}e{counter})
    let ctx1_idx: usize = ref1[1..ref1.find('p').unwrap()].parse().unwrap();
    let ctx2_idx: usize = ref2[1..ref2.find('p').unwrap()].parse().unwrap();
    assert_ne!(ctx1_idx, ctx2_idx, "Context indices in refs should differ");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Page Isolation Tests
// =============================================================================

/// Test that page indices are assigned correctly within a context.
#[tokio::test]
async fn test_page_indices_assigned_correctly() {
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

    // Create multiple pages and verify they get incrementing indices
    let page1 = context.new_page().await.expect("Failed to create page 1");
    let page2 = context.new_page().await.expect("Failed to create page 2");
    let page3 = context.new_page().await.expect("Failed to create page 3");

    // Pages should have sequential indices starting from 0
    assert_eq!(page1.index(), 0, "First page should have index 0");
    assert_eq!(page2.index(), 1, "Second page should have index 1");
    assert_eq!(page3.index(), 2, "Third page should have index 2");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs from different pages have different prefixes.
#[tokio::test]
async fn test_refs_from_different_pages_have_different_prefixes() {
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

    let page1 = context.new_page().await.expect("Failed to create page 1");
    let page2 = context.new_page().await.expect("Failed to create page 2");

    // Set same content on both pages
    let html = "<html><body><button>Test Button</button></body></html>";
    page1
        .set_content(html)
        .set()
        .await
        .expect("Failed to set content");
    page2
        .set_content(html)
        .set()
        .await
        .expect("Failed to set content");

    // Capture snapshots
    let snapshot1 = page1
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot 1");
    let snapshot2 = page2
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot 2");

    // Find button refs
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let ref1 = find_button_ref(&snapshot1).expect("Should find button ref in snapshot 1");
    let ref2 = find_button_ref(&snapshot2).expect("Should find button ref in snapshot 2");

    println!("Ref from page 1: {ref1}");
    println!("Ref from page 2: {ref2}");

    // Refs should have different page prefixes
    assert_ne!(ref1, ref2, "Refs from different pages should be different");

    // Extract page index from refs (format: c{ctx}p{page}f{frame}e{counter})
    let p_start1 = ref1.find('p').unwrap() + 1;
    let p_end1 = ref1.find('f').unwrap();
    let page1_idx: usize = ref1[p_start1..p_end1].parse().unwrap();

    let p_start2 = ref2.find('p').unwrap() + 1;
    let p_end2 = ref2.find('f').unwrap();
    let page2_idx: usize = ref2[p_start2..p_end2].parse().unwrap();

    assert_ne!(page1_idx, page2_idx, "Page indices in refs should differ");
    assert_eq!(page1_idx, 0, "First page should have index 0");
    assert_eq!(page2_idx, 1, "Second page should have index 1");

    // Clean up
    browser.close().await.expect("Failed to close browser");
}

/// Test that refs captured on one page cannot be resolved on a different page in the same context.
///
/// This verifies page isolation - refs are scoped to their originating page.
#[tokio::test]
async fn test_ref_rejected_on_wrong_page() {
    init_tracing();

    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");

    // Create a single context with two pages
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");

    let page1 = context.new_page().await.expect("Failed to create page 1");
    let page2 = context.new_page().await.expect("Failed to create page 2");

    // Set content on page 1
    page1
        .set_content(r#"<html><body><button id="btn1">Button on Page 1</button></body></html>"#)
        .set()
        .await
        .expect("Failed to set content on page 1");

    // Set content on page 2
    page2
        .set_content(r#"<html><body><button id="btn2">Button on Page 2</button></body></html>"#)
        .set()
        .await
        .expect("Failed to set content on page 2");

    // Capture snapshot from page 1
    let snapshot1 = page1
        .aria_snapshot()
        .await
        .expect("Failed to get snapshot from page 1");

    // Find button ref from page 1
    fn find_button_ref(snapshot: &viewpoint_core::AriaSnapshot) -> Option<String> {
        if snapshot.role.as_deref() == Some("button") {
            return snapshot.node_ref.clone();
        }
        for child in &snapshot.children {
            if let Some(r) = find_button_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let page1_button_ref = find_button_ref(&snapshot1).expect("Should find button ref on page 1");
    println!("Page 1 button ref: {page1_button_ref}");

    // Verify the ref works on page 1
    let handle = page1
        .element_from_ref(&page1_button_ref)
        .await
        .expect("Ref should resolve on correct page");
    assert!(
        handle.is_attached().await.unwrap(),
        "Element should be attached"
    );

    // Try to resolve page 1's ref on page 2 - should fail
    let result = page2.element_from_ref(&page1_button_ref).await;

    assert!(
        result.is_err(),
        "Ref from page 1 should NOT be resolvable on page 2"
    );

    // Verify the error message mentions page mismatch
    let err_msg = result.unwrap_err().to_string();
    println!("Error message: {err_msg}");
    assert!(
        err_msg.contains("Page index mismatch") || err_msg.contains("page"),
        "Error should mention page mismatch, got: {err_msg}"
    );

    // Clean up
    browser.close().await.expect("Failed to close browser");
}
