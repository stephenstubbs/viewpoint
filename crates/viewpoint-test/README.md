# viewpoint-test

Test framework for browser automation with Playwright-style assertions.

This is the main crate for the [Viewpoint](https://github.com/stephenstubbs/viewpoint) browser automation framework. It provides everything you need to write browser tests in Rust.

## Features

- `TestHarness` for easy test setup with automatic cleanup
- Fluent async assertions with auto-waiting
- Element and page assertions
- Fixture scoping (test, module, shared)
- Re-exports `viewpoint-core` for convenience

## Quick Start

```toml
[dev-dependencies]
viewpoint-test = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use viewpoint_test::{expect, expect_page, TestHarness, DocumentLoadState};

#[tokio::test]
async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    let harness = TestHarness::new().await?;
    let page = harness.page();

    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;

    expect_page(page).to_have_title("Example Domain").await?;

    let heading = page.locator("h1");
    expect(&heading).to_be_visible().await?;
    expect(&heading).to_have_text("Example Domain").await?;

    Ok(())
}
```

## Assertions

```rust
use viewpoint_test::{expect, expect_page};
use std::time::Duration;

// Element assertions
expect(&locator).to_be_visible().await?;
expect(&locator).to_be_hidden().await?;
expect(&locator).to_have_text("Hello").await?;
expect(&locator).to_contain_text("ello").await?;
expect(&locator).to_have_attribute("href", "/path").await?;
expect(&locator).to_have_class("active").await?;
expect(&locator).to_be_enabled().await?;
expect(&locator).to_be_disabled().await?;
expect(&locator).to_be_checked().await?;
expect(&locator).to_have_count(3).await?;  // Exactly 3 matching elements

// Page assertions
expect_page(page).to_have_url("https://example.com").await?;
expect_page(page).to_have_url_containing("/path").await?;
expect_page(page).to_have_title("Page Title").await?;

// Negation - prepend .not()
expect(&locator).not().to_be_visible().await?;

// Custom timeout
expect(&locator)
    .timeout(Duration::from_secs(10))
    .to_be_visible()
    .await?;
```

## Soft Assertions

Collect multiple assertion failures without stopping test execution:

```rust
use viewpoint_test::SoftAssertions;

#[tokio::test]
async fn test_with_soft_assertions() -> Result<(), Box<dyn std::error::Error>> {
    let harness = TestHarness::new().await?;
    let page = harness.page();
    
    page.goto("https://example.com").goto().await?;
    
    // Create a soft assertions context
    let soft = SoftAssertions::new();
    
    // These assertions collect failures instead of stopping
    soft.expect(&page.locator("h1")).to_have_text("Title").await;
    soft.expect(&page.locator(".missing")).to_be_visible().await;
    soft.expect(&page.locator("button")).to_be_enabled().await;
    
    // Check if all assertions passed
    if !soft.passed() {
        // Get all failures
        for failure in soft.failures() {
            println!("Assertion failed: {}", failure);
        }
    }
    
    // Assert all passed (fails with all errors if any failed)
    soft.assert_all()?;
    
    Ok(())
}
```

## TestHarness Configuration

```rust
use viewpoint_test::TestHarness;
use std::time::Duration;

// Default configuration
let harness = TestHarness::new().await?;

// Custom configuration
let harness = TestHarness::builder()
    .headless(false)  // Show browser window for debugging
    .timeout(Duration::from_secs(60))  // Custom timeout
    .build()
    .await?;

// Fixture Scoping for faster tests
// Test-scoped (default): New browser per test
let harness = TestHarness::new().await?;

// Module-scoped: Share browser, fresh context per test
let harness = TestHarness::from_browser(&shared_browser).await?;

// Shared context: Share context, fresh page per test
let harness = TestHarness::from_context(&shared_context).await?;
```

## ARIA Accessibility Testing

Verify accessibility tree structure with ARIA snapshots:

```rust
use viewpoint_test::{expect, TestHarness};

#[tokio::test]
async fn test_accessibility() -> Result<(), Box<dyn std::error::Error>> {
    let harness = TestHarness::new().await?;
    let page = harness.page();
    
    page.goto("https://example.com").goto().await?;
    
    // Get ARIA snapshot
    let nav = page.locator("nav");
    let snapshot = nav.aria_snapshot().await?;
    
    // Assert expected accessibility structure
    expect(&nav).to_match_aria_snapshot(r#"
      - navigation:
        - link "Home"
        - link "About"
        - link "Contact"
    "#).await?;
    
    Ok(())
}
```

## Requirements

- Rust 1.85+
- Chromium browser (set `CHROMIUM_PATH` or have it in PATH)

## License

MIT
