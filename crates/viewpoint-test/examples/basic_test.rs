//! Example: Basic Test with TestHarness
//!
//! This example demonstrates the primary API for writing browser tests
//! using Viewpoint's `TestHarness`.
//!
//! Run with: `cargo run --example basic_test`

use viewpoint_core::DocumentLoadState;
use viewpoint_test::{expect, expect_page, TestHarness};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for visibility
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    println!("=== Viewpoint Test Framework Example ===\n");

    // Create a test harness - this launches a browser, creates a context and page
    println!("1. Creating test harness...");
    let harness = TestHarness::new().await?;
    let page = harness.page();
    println!("   Browser launched, page ready.\n");

    // Navigate to a website
    println!("2. Navigating to example.com...");
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;
    println!("   Navigation complete.\n");

    // Use assertions to verify page state
    println!("3. Verifying page title...");
    expect_page(page)
        .to_have_title("Example Domain")
        .await?;
    println!("   Title verified: 'Example Domain'\n");

    // Find and verify elements using locators
    println!("4. Finding heading element...");
    let heading = page.locator("h1");
    
    // Verify it's visible
    expect(&heading).to_be_visible().await?;
    println!("   Heading is visible.\n");

    // Verify text content
    println!("5. Verifying heading text...");
    expect(&heading)
        .to_have_text("Example Domain")
        .await?;
    println!("   Heading text verified.\n");

    // Verify URL
    println!("6. Verifying page URL...");
    expect_page(page)
        .to_have_url_containing("example.com")
        .await?;
    println!("   URL contains 'example.com'.\n");

    // Get text content programmatically
    println!("7. Reading text content...");
    let text = heading.text_content().await?;
    println!("   Heading text: {:?}\n", text);

    // Check element count
    println!("8. Counting paragraph elements...");
    let paragraphs = page.locator("p");
    let count = paragraphs.count().await?;
    println!("   Found {} paragraph(s).\n", count);

    // Harness cleanup happens automatically on drop
    println!("9. Test complete! Browser will close automatically.\n");
    
    println!("=== Example Complete ===");
    Ok(())
}
