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
viewpoint-test = "0.1"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
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
// Element assertions
expect(&locator).to_be_visible().await?;
expect(&locator).to_have_text("Hello").await?;
expect(&locator).to_have_attribute("href", "/path").await?;

// Page assertions
expect_page(page).to_have_url("https://example.com").await?;
expect_page(page).to_have_title("Page Title").await?;

// Negation
expect(&locator).not().to_be_visible().await?;
```

## Requirements

- Rust 1.70+
- Chromium browser (set `CHROMIUM_PATH` or have it in PATH)

## License

MIT
