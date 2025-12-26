# viewpoint-test-macros

Procedural macros for the Viewpoint test framework.

This crate provides the `#[viewpoint_test::test]` attribute macro for convenient test setup.

## Usage

Use with `viewpoint-test`:

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
- Provides the `page` parameter
- Handles cleanup on test completion

## License

MIT
