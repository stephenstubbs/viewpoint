# viewpoint-core

High-level browser automation API for Rust, inspired by Playwright.

This crate provides the core browser, context, and page abstractions for the [Viewpoint](https://github.com/stephenstubbs/viewpoint) browser automation framework.

## Features

- Browser launching and management
- Page navigation with configurable wait states
- Element locators (CSS, text, role, label, test ID, placeholder)
- Element actions (click, fill, type, hover, check, select)
- Automatic element waiting

## Usage

For testing, use `viewpoint-test` which re-exports this crate with additional assertions and test fixtures.

```rust
use viewpoint_core::{Browser, BrowserLauncher, DocumentLoadState};

// Launch browser
let browser = BrowserLauncher::new().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

// Navigate
page.goto("https://example.com")
    .wait_until(DocumentLoadState::DomContentLoaded)
    .goto()
    .await?;

// Locate and interact with elements
let button = page.locator("button.submit");
button.click().await?;
```

## License

MIT
