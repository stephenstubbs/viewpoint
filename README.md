# Viewpoint

[![Crates.io](https://img.shields.io/crates/v/viewpoint-test.svg)](https://crates.io/crates/viewpoint-test)
[![Documentation](https://docs.rs/viewpoint-test/badge.svg)](https://docs.rs/viewpoint-test)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust-native browser automation library inspired by Playwright. Viewpoint provides a high-level API for controlling Chromium browsers via the Chrome DevTools Protocol (CDP).

## Features

- **Browser Automation**: Launch and control Chromium browsers programmatically
- **Page Navigation**: Navigate to URLs with configurable wait states
- **Element Locators**: Find elements using CSS selectors, text, ARIA roles, labels, and more
- **Actions**: Click, fill, type, hover, check/uncheck, select options
- **Assertions**: Fluent async assertions for testing element and page state
- **Test Framework**: `TestHarness` for easy test setup with automatic cleanup

## Crates

| Crate | Description |
|-------|-------------|
| `viewpoint-cdp` | Low-level Chrome DevTools Protocol client |
| `viewpoint-core` | High-level browser automation API |
| `viewpoint-test` | Test framework with assertions and fixtures |
| `viewpoint-test-macros` | Proc macros for convenient test setup |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
viewpoint-test = "0.1"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

### Basic Test with TestHarness

```rust
use viewpoint_test::{expect, expect_page, TestHarness, DocumentLoadState};

#[tokio::test]
async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test harness - launches browser, creates context and page
    let harness = TestHarness::new().await?;
    let page = harness.page();

    // Navigate to a URL
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;

    // Verify page state
    expect_page(page).to_have_title("Example Domain").await?;

    // Find and verify elements
    let heading = page.locator("h1");
    expect(&heading).to_be_visible().await?;
    expect(&heading).to_have_text("Example Domain").await?;

    Ok(()) // harness automatically cleans up on drop
}
```

### Using the Test Macro

```rust
use viewpoint_test::{test, Page, expect};

#[viewpoint_test::test]
async fn my_macro_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://example.com").goto().await?;
    
    let heading = page.locator("h1");
    expect(&heading).to_be_visible().await?;
    
    Ok(())
}
```

## Locators

Viewpoint provides multiple ways to locate elements:

```rust
// CSS selector
let button = page.locator("button.submit");

// By text content
let heading = page.get_by_text("Welcome");

// By ARIA role
let nav = page.get_by_role(AriaRole::Navigation).build();

// By test ID (data-testid attribute)
let form = page.get_by_test_id("login-form");

// By label (for form controls)
let email = page.get_by_label("Email address");

// By placeholder
let search = page.get_by_placeholder("Search...");

// Chaining
let item = page.locator("ul").locator("li").first();
```

## Actions

Perform actions on located elements:

```rust
// Click
button.click().await?;
button.dblclick().await?;

// Form input
input.fill("Hello World").await?;
input.type_text("typing...").await?;  // Character by character
input.clear().await?;

// Keyboard
input.press("Enter").await?;
input.press("Control+a").await?;

// Checkbox/Radio
checkbox.check().await?;
checkbox.uncheck().await?;

// Dropdown
select.select_option("value").await?;
select.select_options(&["a", "b", "c"]).await?;  // Multi-select

// Mouse
element.hover().await?;
element.focus().await?;
```

## Assertions

Fluent async assertions with auto-waiting:

```rust
use viewpoint_test::{expect, expect_page};

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

// Page assertions
expect_page(page).to_have_url("https://example.com").await?;
expect_page(page).to_have_url_containing("/path").await?;
expect_page(page).to_have_title("Page Title").await?;

// Negation
expect(&locator).not().to_be_visible().await?;

// Custom timeout
expect(&locator)
    .timeout(Duration::from_secs(10))
    .to_be_visible()
    .await?;
```

## Test Configuration

Configure test behavior with the builder:

```rust
let harness = TestHarness::builder()
    .headless(false)  // Show browser window
    .timeout(Duration::from_secs(60))
    .build()
    .await?;
```

## Fixture Scoping

Share browser/context across tests for faster execution:

```rust
// Test-scoped (default): New browser per test
let harness = TestHarness::new().await?;

// Module-scoped: Share browser, fresh context per test
let harness = TestHarness::from_browser(&shared_browser).await?;

// Shared context: Share context, fresh page per test
let harness = TestHarness::from_context(&shared_context).await?;
```

## Requirements

- Rust 1.70+
- Chromium browser (automatically found via `CHROMIUM_PATH` or system PATH)

## Development

```bash
# Run all tests
cargo test --workspace

# Run with visible browser
HEADLESS=false cargo test --workspace

# Run specific test
cargo test -p viewpoint-test --test harness_tests

# Run examples
cargo run -p viewpoint-test --example basic_test
```

## License

MIT
