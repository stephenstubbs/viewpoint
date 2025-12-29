//! Example: Test with `#[viewpoint::test]` Macro
//!
//! This example demonstrates the secondary API using the proc macro
//! for convenient test setup. The macro handles TestHarness creation
//! and cleanup automatically.
//!
//! Note: This file shows how tests would be written. Since examples
//! are run with `cargo run`, and the macro generates test functions,
//! this example primarily serves as documentation.
//!
//! In a real test file, you would use:
//! ```no_run
//! use viewpoint_core::page::Page;
//! use viewpoint_test::TestError;
//!
//! # /*
//! #[viewpoint_test::test]
//! async fn my_test(page: &Page) -> Result<(), TestError> {
//!     // test code
//!     Ok(())
//! }
//! # */
//! ```

use viewpoint_test::{expect_page, TestError, TestHarness};
use viewpoint_core::DocumentLoadState;

/// Example of what the macro expands to.
/// 
/// The `#[viewpoint_test::test]` macro would transform:
/// ```no_run
/// use viewpoint_core::page::Page;
/// use viewpoint_test::{expect_page, TestError};
///
/// # /*
/// #[viewpoint_test::test]
/// async fn example_test(page: &Page) -> Result<(), TestError> {
///     page.goto("https://example.com").goto().await?;
///     expect_page(page).to_have_title("Example Domain").await?;
///     Ok(())
/// }
/// # */
/// ```
/// 
/// Into something like:
/// ```no_run
/// use viewpoint_test::{expect_page, TestError, TestHarness};
///
/// # /*
/// #[tokio::test]
/// # */
/// async fn example_test() -> Result<(), TestError> {
///     let _harness = TestHarness::new().await?;
///     let page = _harness.page();
///     
///     // Original test body:
///     page.goto("https://example.com").goto().await?;
///     expect_page(page).to_have_title("Example Domain").await?;
///     Ok(())
/// }
/// ```
async fn expanded_test_example() -> Result<(), TestError> {
    let harness = TestHarness::new().await?;
    let page = harness.page();
    
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .map_err(|e| TestError::Core(e.into()))?;
    
    expect_page(page)
        .to_have_title("Example Domain")
        .await
        .map_err(TestError::Assertion)?;
    
    Ok(())
}

/// Example with configuration options.
/// 
/// The macro supports various configuration options:
/// ```no_run
/// use viewpoint_core::page::Page;
/// use viewpoint_test::TestError;
///
/// # /*
/// #[viewpoint_test::test(headless = false, timeout = 60000)]
/// async fn visible_test(page: &Page) -> Result<(), TestError> {
///     // Test runs with visible browser and 60s timeout
///     Ok(())
/// }
/// # */
/// ```
async fn configured_test_example() -> Result<(), TestError> {
    use std::time::Duration;
    
    let harness = TestHarness::builder()
        .headless(false)
        .timeout(Duration::from_secs(60))
        .build()
        .await?;
    
    let page = harness.page();
    
    page.goto("https://example.com")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await
        .map_err(|e| TestError::Core(e.into()))?;
    
    Ok(())
}

/// Example with module-scoped browser.
/// 
/// For faster test suites, share a browser across tests:
/// ```no_run
/// use std::sync::OnceLock;
/// use viewpoint_core::browser::Browser;
/// use viewpoint_core::page::Page;
/// use viewpoint_test::TestError;
///
/// // In your test module:
/// static BROWSER: OnceLock<Browser> = OnceLock::new();
/// 
/// fn shared_browser() -> &'static Browser {
///     BROWSER.get_or_init(|| {
///         tokio::runtime::Runtime::new().unwrap()
///             .block_on(Browser::launch().headless(true).launch())
///             .unwrap()
///     })
/// }
/// 
/// # /*
/// #[viewpoint_test::test(scope = "browser", browser = "shared_browser")]
/// async fn fast_test_1(page: &Page) -> Result<(), TestError> {
///     // Uses shared browser, but fresh context and page
///     Ok(())
/// }
/// 
/// #[viewpoint_test::test(scope = "browser", browser = "shared_browser")]  
/// async fn fast_test_2(page: &Page) -> Result<(), TestError> {
///     // Also uses shared browser
///     Ok(())
/// }
/// # */
/// ```

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Viewpoint Macro Example ===\n");
    
    println!("This example demonstrates the #[viewpoint_test::test] macro.\n");
    
    println!("Running expanded_test_example...");
    expanded_test_example().await?;
    println!("  Passed!\n");
    
    println!("Running configured_test_example (non-headless)...");
    configured_test_example().await?;
    println!("  Passed!\n");
    
    println!("=== All Examples Complete ===");
    
    Ok(())
}
