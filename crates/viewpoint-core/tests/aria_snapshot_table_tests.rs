#![cfg(feature = "integration")]

//! ARIA snapshot tests for table structure elements.
//!
//! Tests for: thead, tfoot, caption
//!
//! Note: Basic table, tbody, tr, td, th are tested elsewhere.
//! These tests focus on the additional table structure elements.

mod common;

use common::launch_with_page;

// =============================================================================
// Caption Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_table_caption() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <table>
                <caption>Monthly Sales Report</caption>
                <thead>
                    <tr>
                        <th>Month</th>
                        <th>Sales</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>January</td>
                        <td>$10,000</td>
                    </tr>
                </tbody>
            </table>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Table caption snapshot:\n{}", yaml);

    assert!(
        yaml.contains("caption"),
        "Snapshot should contain 'caption' role, got: {}",
        yaml
    );
    assert!(
        yaml.contains("Monthly Sales Report"),
        "Snapshot should contain caption text, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Thead Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_thead() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Age</th>
                        <th>City</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>John</td>
                        <td>30</td>
                        <td>NYC</td>
                    </tr>
                </tbody>
            </table>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Thead snapshot:\n{}", yaml);

    // thead should have "rowgroup" role
    assert!(
        yaml.contains("rowgroup"),
        "Snapshot should contain 'rowgroup' role for thead, got: {}",
        yaml
    );
    // Column headers should be captured
    assert!(
        yaml.contains("columnheader") || yaml.contains("Name"),
        "Snapshot should contain column headers, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Tfoot Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_tfoot() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <table>
                <thead>
                    <tr>
                        <th>Item</th>
                        <th>Price</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Widget</td>
                        <td>$10</td>
                    </tr>
                    <tr>
                        <td>Gadget</td>
                        <td>$20</td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td>Total</td>
                        <td>$30</td>
                    </tr>
                </tfoot>
            </table>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Tfoot snapshot:\n{}", yaml);

    // Both thead and tfoot should have "rowgroup" role
    // We should see multiple rowgroups (thead, tbody, tfoot)
    assert!(
        yaml.contains("rowgroup"),
        "Snapshot should contain 'rowgroup' role for tfoot, got: {}",
        yaml
    );
    assert!(
        yaml.contains("Total"),
        "Snapshot should contain tfoot content, got: {}",
        yaml
    );

    browser.close().await.expect("Failed to close browser");
}

// =============================================================================
// Complete Table Structure Tests
// =============================================================================

#[tokio::test]
async fn test_aria_snapshot_complete_table() {
    let (browser, _context, page) = launch_with_page().await;

    page.set_content(
        r#"
        <html><body>
            <table>
                <caption>Quarterly Report</caption>
                <thead>
                    <tr>
                        <th>Quarter</th>
                        <th>Revenue</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Q1</td>
                        <td>$1M</td>
                    </tr>
                    <tr>
                        <td>Q2</td>
                        <td>$1.5M</td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td>Total</td>
                        <td>$2.5M</td>
                    </tr>
                </tfoot>
            </table>
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    let snapshot = page.aria_snapshot().await.expect("Failed to get snapshot");
    let yaml = snapshot.to_yaml();
    println!("Complete table snapshot:\n{}", yaml);

    // Check all table structure elements
    assert!(yaml.contains("table"), "Should contain table");
    assert!(yaml.contains("caption"), "Should contain caption");
    assert!(yaml.contains("rowgroup"), "Should contain rowgroup (thead/tbody/tfoot)");
    assert!(yaml.contains("row"), "Should contain row");
    assert!(
        yaml.contains("columnheader") || yaml.contains("Quarter"),
        "Should contain column headers"
    );
    assert!(yaml.contains("cell") || yaml.contains("Q1"), "Should contain cells");

    browser.close().await.expect("Failed to close browser");
}
