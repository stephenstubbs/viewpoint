# viewpoint-test-macros

[![Crates.io](https://img.shields.io/crates/v/viewpoint-test-macros.svg)](https://crates.io/crates/viewpoint-test-macros)
[![Documentation](https://docs.rs/viewpoint-test-macros/badge.svg)](https://docs.rs/viewpoint-test-macros)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Procedural macros for the Viewpoint test framework, providing the `#[viewpoint::test]` attribute macro for convenient test setup.

This crate is part of the [Viewpoint](https://github.com/user/viewpoint) browser automation framework.

## Features

- **Automatic Setup**: Browser, context, and page are set up before the test
- **Automatic Cleanup**: Resources are cleaned up after the test completes
- **Fixture Injection**: Request fixtures by parameter type (Page, BrowserContext, Browser)
- **Fixture Scoping**: Share browsers/contexts across tests for performance
- **Configuration**: Customize headless mode, timeouts, and more

## Installation

This crate is typically used through `viewpoint-test`:

```toml
[dev-dependencies]
viewpoint-test = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use viewpoint_test::{test, Page, expect};

#[viewpoint_test::test]
async fn my_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://example.com").goto().await?;
    
    let heading = page.locator("h1");
    expect(&heading).to_be_visible().await?;
    
    Ok(())
}
```

The macro automatically:
- Creates a `TestHarness`
- Provides the requested fixtures (`page`, `context`, `browser`)
- Handles cleanup on test completion

## Fixture Parameters

Request different fixtures by changing the parameter types:

```rust
use viewpoint_test::test;
use viewpoint_core::{Page, BrowserContext, Browser};

// Get just the page (most common)
#[viewpoint_test::test]
async fn test_with_page(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://example.com").goto().await?;
    Ok(())
}

// Get page and context
#[viewpoint_test::test]
async fn test_with_context(
    page: &Page,
    context: &BrowserContext
) -> Result<(), Box<dyn std::error::Error>> {
    // Add cookies to the context
    context.add_cookies(vec![/* ... */]).await?;
    page.goto("https://example.com").goto().await?;
    Ok(())
}

// Get page, context, and browser
#[viewpoint_test::test]
async fn test_with_browser(
    page: &Page,
    context: &BrowserContext,
    browser: &Browser
) -> Result<(), Box<dyn std::error::Error>> {
    // Create additional contexts
    let another_context = browser.new_context().await?;
    Ok(())
}
```

## Configuration Options

Configure the test with attribute arguments:

```rust
use viewpoint_test::test;
use viewpoint_core::Page;

// Run in headed mode (visible browser)
#[viewpoint_test::test(headless = false)]
async fn headed_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://example.com").goto().await?;
    Ok(())
}

// Custom timeout (in milliseconds)
#[viewpoint_test::test(timeout = 60000)]
async fn slow_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    // This test has a 60 second timeout
    page.goto("https://slow-site.com").goto().await?;
    Ok(())
}
```

## Fixture Scoping

Share browsers/contexts across tests for better performance:

### Browser Scope

Share a browser across multiple tests (each test gets a fresh context and page):

```rust
use viewpoint_test::test;
use viewpoint_core::{Browser, Page};
use std::sync::OnceLock;

// Define a shared browser
static BROWSER: OnceLock<Browser> = OnceLock::new();

async fn shared_browser() -> &'static Browser {
    BROWSER.get_or_init(|| {
        tokio::runtime::Handle::current().block_on(async {
            Browser::launch().headless(true).launch().await.unwrap()
        })
    })
}

#[viewpoint_test::test(scope = "browser", browser = "shared_browser")]
async fn fast_test_1(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    // Uses shared browser, but fresh context and page
    page.goto("https://example.com").goto().await?;
    Ok(())
}

#[viewpoint_test::test(scope = "browser", browser = "shared_browser")]
async fn fast_test_2(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    // Same shared browser, different context and page
    page.goto("https://example.org").goto().await?;
    Ok(())
}
```

### Context Scope

Share a context across tests (each test gets a fresh page, but shares cookies/state):

```rust
use viewpoint_test::test;
use viewpoint_core::{BrowserContext, Page};

async fn shared_context() -> &'static BrowserContext {
    // Return a shared context
    todo!()
}

#[viewpoint_test::test(scope = "context", context = "shared_context")]
async fn test_with_shared_context(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    // Uses shared context (shares cookies, storage, etc.)
    page.goto("https://example.com").goto().await?;
    Ok(())
}
```

## All Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `headless` | bool | `true` | Run browser in headless mode |
| `timeout` | integer | 30000 | Default timeout in milliseconds |
| `scope` | string | - | Fixture scope: `"browser"` or `"context"` |
| `browser` | string | - | Function name returning shared browser (required when scope = "browser") |
| `context` | string | - | Function name returning shared context (required when scope = "context") |

## When to Use TestHarness Instead

The macro is convenient but `TestHarness` offers more control:

- When you need to configure the browser context with specific options
- When you need to set up network interception before navigation
- When you want more explicit control over setup and teardown
- When you need to handle setup failures differently

```rust
use viewpoint_test::TestHarness;

#[tokio::test]
async fn explicit_test() -> Result<(), Box<dyn std::error::Error>> {
    let harness = TestHarness::new().await?;
    let page = harness.page();

    // More explicit setup gives you more control
    page.goto("https://example.com").goto().await?;

    Ok(())
}
```

## License

MIT
