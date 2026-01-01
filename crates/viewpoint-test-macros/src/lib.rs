//! # Viewpoint Test Macros - Test Attribute Macros
//!
//! Proc macros for the Viewpoint test framework, providing the `#[viewpoint::test]`
//! attribute macro for convenient test setup. This is an optional convenience layer -
//! the primary API is `TestHarness` from `viewpoint-test`.
//!
//! ## Features
//!
//! - **Automatic Setup**: Browser, context, and page are set up before the test
//! - **Automatic Cleanup**: Resources are cleaned up after the test completes
//! - **Fixture Injection**: Request fixtures by parameter type (Page, BrowserContext, Browser)
//! - **Fixture Scoping**: Share browsers/contexts across tests for performance
//! - **Configuration**: Customize headless mode, timeouts, and more
//!
//! ## Quick Start
//!
//! ```text
//! use viewpoint_test_macros::test;
//! use viewpoint_core::Page;
//!
//! #[viewpoint_test_macros::test]
//! async fn my_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Fixture Parameters
//!
//! Request different fixtures by changing the parameter type:
//!
//! ```text
//! use viewpoint_test_macros::test;
//! use viewpoint_core::{Page, BrowserContext, Browser};
//!
//! // Get just the page (most common)
//! #[viewpoint_test_macros::test]
//! async fn test_with_page(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//!
//! // Get page and context
//! #[viewpoint_test_macros::test]
//! async fn test_with_context(
//!     page: &Page,
//!     context: &BrowserContext
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     // Add cookies to the context
//!     context.add_cookies(vec![/* ... */]).await?;
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//!
//! // Get page, context, and browser
//! #[viewpoint_test_macros::test]
//! async fn test_with_browser(
//!     page: &Page,
//!     context: &BrowserContext,
//!     browser: &Browser
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     // Create additional contexts
//!     let another_context = browser.new_context().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration Options
//!
//! Configure the test with attribute arguments:
//!
//! ```text
//! use viewpoint_test_macros::test;
//! use viewpoint_core::Page;
//!
//! // Run in headed mode (visible browser)
//! #[viewpoint_test_macros::test(headless = false)]
//! async fn headed_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//!
//! // Custom timeout (in milliseconds)
//! #[viewpoint_test_macros::test(timeout = 60000)]
//! async fn slow_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     // This test has a 60 second timeout
//!     page.goto("https://slow-site.com").goto().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Fixture Scoping
//!
//! Share browsers/contexts across tests for better performance:
//!
//! ### Browser Scope
//!
//! Share a browser across multiple tests (each test gets a fresh context and page):
//!
//! ```text
//! use viewpoint_test_macros::test;
//! use viewpoint_core::{Browser, Page};
//! use std::sync::OnceLock;
//!
//! // Define a shared browser
//! static BROWSER: OnceLock<Browser> = OnceLock::new();
//!
//! async fn shared_browser() -> &'static Browser {
//!     // Initialize browser once
//!     BROWSER.get_or_init(|| {
//!         tokio::runtime::Handle::current().block_on(async {
//!             Browser::launch().headless(true).launch().await.unwrap()
//!         })
//!     })
//! }
//!
//! #[viewpoint_test_macros::test(scope = "browser", browser = "shared_browser")]
//! async fn fast_test_1(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     // Uses shared browser, but fresh context and page
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//!
//! #[viewpoint_test_macros::test(scope = "browser", browser = "shared_browser")]
//! async fn fast_test_2(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     // Same shared browser, different context and page
//!     page.goto("https://example.org").goto().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Context Scope
//!
//! Share a context across tests (each test gets a fresh page, but shares cookies/state):
//!
//! ```text
//! use viewpoint_test_macros::test;
//! use viewpoint_core::{BrowserContext, Page};
//!
//! async fn shared_context() -> &'static BrowserContext {
//!     // Return a shared context
//!     todo!()
//! }
//!
//! #[viewpoint_test_macros::test(scope = "context", context = "shared_context")]
//! async fn test_with_shared_context(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     // Uses shared context (shares cookies, storage, etc.)
//!     page.goto("https://example.com").goto().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## All Configuration Options
//!
//! | Option | Type | Default | Description |
//! |--------|------|---------|-------------|
//! | `headless` | bool | `true` | Run browser in headless mode |
//! | `timeout` | integer | 30000 | Default timeout in milliseconds |
//! | `scope` | string | - | Fixture scope: `"browser"` or `"context"` |
//! | `browser` | string | - | Function name returning shared browser (required when scope = "browser") |
//! | `context` | string | - | Function name returning shared context (required when scope = "context") |
//!
//! ## When to Use TestHarness Instead
//!
//! The macro is convenient but `TestHarness` offers more control:
//!
//! - When you need to configure the browser context with specific options
//! - When you need to set up network interception before navigation
//! - When you want more explicit control over setup and teardown
//! - When you need to handle setup failures differently
//!
//! ```ignore
//! use viewpoint_test::TestHarness;
//!
//! #[tokio::test]
//! async fn explicit_test() -> Result<(), Box<dyn std::error::Error>> {
//!     let harness = TestHarness::new().await?;
//!     let page = harness.page();
//!
//!     // More explicit setup gives you more control
//!     page.goto("https://example.com").goto().await?;
//!
//!     Ok(())
//! }
//! ```

use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod test_attr;

/// Attribute macro for Viewpoint tests.
///
/// This macro transforms async test functions to include `TestHarness` setup
/// and cleanup. Fixture parameters (Page, `BrowserContext`, Browser) are
/// automatically extracted from the harness.
///
/// # Basic Usage
///
/// ```text
/// #[viewpoint_test_macros::test]
/// async fn my_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
///     page.goto("https://example.com").goto().await?;
///     Ok(())
/// }
/// ```
///
/// # Configuration Options
///
/// - `headless = true|false` - Run browser in headless mode (default: true)
/// - `timeout = <ms>` - Default timeout in milliseconds (default: 30000)
/// - `scope = "browser"|"context"` - Fixture scoping level
/// - `browser = "<fn_name>"` - Function returning shared browser (required when scope = "browser")
/// - `context = "<fn_name>"` - Function returning shared context (required when scope = "context")
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as test_attr::TestArgs);
    let input = parse_macro_input!(item as ItemFn);

    match test_attr::expand_test(args, input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
