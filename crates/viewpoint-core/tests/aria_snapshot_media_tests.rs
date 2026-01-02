#![cfg(feature = "integration")]

//! ARIA snapshot tests for media and misc elements.
//!
//! Tests for: svg, area, hr, menu

mod common;

use common::launch_with_page;

// =============================================================================
// SVG Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_svg() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <svg width="100" height="100" aria-label="Circle graphic">
                <circle cx="50" cy="50" r="40" fill="red" />
            </svg>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("SVG snapshot:\n{yaml}");

    // svg should have "img" role
    assert!(
        yaml.contains("img"),
        "Snapshot should contain 'img' role for svg, got: {yaml}"
    );
    assert!(
        yaml.contains("Circle graphic"),
        "Snapshot should contain svg aria-label, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_svg_without_label() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <svg width="50" height="50">
                <rect width="50" height="50" fill="blue" />
            </svg>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("SVG without label snapshot:\n{yaml}");

    // svg should still have "img" role even without label
    assert!(
        yaml.contains("img"),
        "Snapshot should contain 'img' role for svg, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Area Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_area_with_href() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r##"
        <html><body>
            <img src="data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=" 
                 alt="Workplace" usemap="#workmap" width="400" height="379">
            <map name="workmap">
                <area shape="rect" coords="34,44,270,350" alt="Computer" href="computer.htm">
                <area shape="circle" coords="337,300,44" alt="Coffee" href="coffee.htm">
            </map>
        </body></html>
    "##,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Area with href snapshot:\n{yaml}");

    // area with href should have "link" role
    assert!(
        yaml.contains("link"),
        "Snapshot should contain 'link' role for area with href, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// HR (Separator) Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_hr() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <p>Section 1</p>
            <hr>
            <p>Section 2</p>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("HR snapshot:\n{yaml}");

    // hr should have "separator" role
    assert!(
        yaml.contains("separator"),
        "Snapshot should contain 'separator' role for hr, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_hr_with_aria_label() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <p>Introduction</p>
            <hr aria-label="Content divider">
            <p>Main content</p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("HR with aria-label snapshot:\n{yaml}");

    assert!(
        yaml.contains("separator"),
        "Snapshot should contain 'separator' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Content divider"),
        "Snapshot should contain hr aria-label, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Menu Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_menu() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r"
        <html><body>
            <menu>
                <li>Cut</li>
                <li>Copy</li>
                <li>Paste</li>
            </menu>
        </body></html>
    ",
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Menu snapshot:\n{yaml}");

    // menu should have "list" role (per HTML spec, menu is semantically like ul)
    assert!(
        yaml.contains("list"),
        "Snapshot should contain 'list' role for menu, got: {yaml}"
    );
    assert!(
        yaml.contains("listitem"),
        "Snapshot should contain 'listitem' for menu items, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Math Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_math() {
    let (browser, _context, page) = launch_with_page().await;

    // Using a simpler MathML example to avoid parsing issues
    page.set_content(
        r#"
        <html><body>
            <math aria-label="Quadratic formula">
                <mi>x</mi>
                <mo>=</mo>
                <mi>y</mi>
            </math>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Math snapshot:\n{yaml}");

    // math should have "math" role
    assert!(
        yaml.contains("math"),
        "Snapshot should contain 'math' role, got: {yaml}"
    );
    assert!(
        yaml.contains("Quadratic formula"),
        "Snapshot should contain math aria-label, got: {yaml}"
    );

    browser.close().await.expect("Failed to close browser");
}
