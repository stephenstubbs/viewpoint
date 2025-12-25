# Change: Add Test Framework

## Why

RustRight's goal is to be "the Rust equivalent of Playwright." While we have browser automation capabilities (launching, navigation, waiting), we lack the testing-specific features that make Playwright productive for E2E testing. Developers need fixtures for test setup, assertions for verifying page state, locators for finding elements, and actions for interacting with them.

## What Changes

- **New crate: `rustright-test`** - Test framework runtime with TestHarness, assertions, and configuration
- **New crate: `rustright-test-macros`** - Optional proc macros for `#[rustright::test]` convenience
- **TestHarness API** - Primary explicit API for test setup with Drop-based cleanup
- **Locator API** - Element finding strategies (CSS, text, role, test-id) with auto-waiting
- **Actions API** - User interactions (click, fill, type, select, hover) on locators
- **Expect API** - Fluent async assertions (`expect(&locator).to_be_visible().await?`)

### Primary API (TestHarness)
```rust
#[tokio::test]
async fn my_test() -> Result<()> {
    let harness = TestHarness::new().await?;
    let page = harness.page();
    
    page.goto("https://example.com").goto().await?;
    expect(&page.locator("h1")).to_have_text("Example").await?;
    
    Ok(()) // harness drops and cleans up
}
```

### Secondary API (Proc Macro - Optional)
```rust
#[rustright::test]
async fn my_test(page: Page) -> Result<()> {
    page.goto("https://example.com").goto().await?;
    expect(&page.locator("h1")).to_have_text("Example").await?;
    Ok(())
}
```

## Impact

- Affected specs: None modified (all new capabilities)
- New capabilities:
  - `test-runner` - TestHarness and optional test macro
  - `test-fixtures` - Browser/context/page access and lifecycle
  - `test-assertions` - Expect API and matchers
  - `test-locators` - Element selection strategies
  - `test-actions` - User interaction methods
- Affected code:
  - New `crates/rustright-test/`
  - New `crates/rustright-test-macros/` (optional)
  - `crates/rustright-core/src/page/` - Add locator and action methods
  - `Cargo.toml` - Workspace members
