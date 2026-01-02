#![cfg(feature = "integration")]

//! ARIA snapshot tests for document structure elements.
//!
//! Tests for: figure, figcaption, details, search, fieldset, address, hgroup

mod common;

use common::launch_with_page;

// =============================================================================
// Figure Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_figure() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <figure>
                <img src="data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=" alt="Chart">
                <figcaption>Sales data for Q1 2024</figcaption>
            </figure>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Figure snapshot:\n{}", yaml);

    assert!(
        yaml.contains("figure"),
        "Snapshot should contain 'figure' role, got: {}",
        yaml
    );
    // figcaption should be captured (implicit caption role)
    assert!(
        yaml.contains("caption") || yaml.contains("Sales data"),
        "Snapshot should contain figcaption content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_figure_without_caption() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <figure>
                <img src="data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=" alt="Diagram">
            </figure>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Figure without caption snapshot:\n{}", yaml);

    assert!(
        yaml.contains("figure"),
        "Snapshot should contain 'figure' role, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Details Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_details() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <details>
                <summary>More information</summary>
                <p>This is the hidden content that appears when expanded.</p>
            </details>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Details snapshot:\n{}", yaml);

    // details should have "group" role
    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role for details, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_details_open() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <details open>
                <summary>Expanded section</summary>
                <p>Visible content.</p>
            </details>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Details open snapshot:\n{}", yaml);

    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role, got: {}",
        yaml
    );
    assert!(
        yaml.contains("Visible content"),
        "Snapshot should contain expanded content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Search Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_search() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <search>
                <input type="search" placeholder="Search...">
                <button>Search</button>
            </search>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Search element snapshot:\n{}", yaml);

    assert!(
        yaml.contains("search"),
        "Snapshot should contain 'search' role, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Fieldset Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_fieldset() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <form>
                <fieldset>
                    <legend>Personal Information</legend>
                    <label>Name: <input type="text"></label>
                    <label>Email: <input type="email"></label>
                </fieldset>
            </form>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Fieldset snapshot:\n{}", yaml);

    // fieldset should have "group" role
    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role for fieldset, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Address Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_address() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <address>
                123 Main Street<br>
                City, State 12345
            </address>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Address snapshot:\n{}", yaml);

    // address should have "group" role
    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role for address, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Hgroup Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_hgroup() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <hgroup>
                <h1>Main Title</h1>
                <p>Subtitle text</p>
            </hgroup>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Hgroup snapshot:\n{}", yaml);

    // hgroup should have "group" role
    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role for hgroup, got: {}",
        yaml
    );
    assert!(
        yaml.contains("Main Title"),
        "Snapshot should contain heading content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}
