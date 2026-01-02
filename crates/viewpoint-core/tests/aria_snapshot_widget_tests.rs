#![cfg(feature = "integration")]

//! ARIA snapshot tests for widget and form elements.
//!
//! Tests for: meter, output, time, datalist, optgroup

mod common;

use common::launch_with_page;

// =============================================================================
// Meter Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_meter() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <label>Progress: <meter value="0.6" min="0" max="1">60%</meter></label>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Meter snapshot:\n{}", yaml);

    assert!(
        yaml.contains("meter"),
        "Snapshot should contain 'meter' role, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_meter_with_ranges() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <meter value="75" min="0" max="100" low="25" high="75" optimum="50">75%</meter>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Meter with ranges snapshot:\n{}", yaml);

    assert!(
        yaml.contains("meter"),
        "Snapshot should contain 'meter' role, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Output Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_output() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <form oninput="result.value=parseInt(a.value)+parseInt(b.value)">
                <input type="range" id="a" value="50"> +
                <input type="number" id="b" value="50"> =
                <output name="result" for="a b">100</output>
            </form>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Output snapshot:\n{}", yaml);

    // output should have "status" role
    assert!(
        yaml.contains("status"),
        "Snapshot should contain 'status' role for output, got: {}",
        yaml
    );
    assert!(
        yaml.contains("100"),
        "Snapshot should contain output text content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_output_simple() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <output>42</output>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Simple output snapshot:\n{}", yaml);

    assert!(
        yaml.contains("status"),
        "Snapshot should contain 'status' role, got: {}",
        yaml
    );
    assert!(
        yaml.contains("42"),
        "Snapshot should contain output text, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Time Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_time() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <time datetime="2024-01-01">January 1st, 2024</time>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Time snapshot:\n{}", yaml);

    assert!(
        yaml.contains("time"),
        "Snapshot should contain 'time' role, got: {}",
        yaml
    );
    assert!(
        yaml.contains("January 1st, 2024"),
        "Snapshot should contain time text content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_aria_snapshot_time_duration() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <p>Duration: <time datetime="PT2H30M">2 hours and 30 minutes</time></p>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Time duration snapshot:\n{}", yaml);

    assert!(
        yaml.contains("time"),
        "Snapshot should contain 'time' role, got: {}",
        yaml
    );
    assert!(
        yaml.contains("2 hours and 30 minutes"),
        "Snapshot should contain duration text, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Datalist Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_datalist() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <label>Choose a browser:
                <input list="browsers" name="browser">
            </label>
            <datalist id="browsers">
                <option value="Chrome">
                <option value="Firefox">
                <option value="Safari">
            </datalist>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Datalist snapshot:\n{}", yaml);

    // datalist should have "listbox" role
    assert!(
        yaml.contains("listbox"),
        "Snapshot should contain 'listbox' role for datalist, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Optgroup Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_optgroup() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <select>
                <optgroup label="Swedish Cars">
                    <option value="volvo">Volvo</option>
                    <option value="saab">Saab</option>
                </optgroup>
                <optgroup label="German Cars">
                    <option value="mercedes">Mercedes</option>
                    <option value="audi">Audi</option>
                </optgroup>
            </select>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Optgroup snapshot:\n{}", yaml);

    // optgroup should have "group" role
    assert!(
        yaml.contains("group"),
        "Snapshot should contain 'group' role for optgroup, got: {}",
        yaml
    );
    // Options should still be captured
    assert!(
        yaml.contains("option"),
        "Snapshot should contain options, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}
