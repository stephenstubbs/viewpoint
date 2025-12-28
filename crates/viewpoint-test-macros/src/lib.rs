//! Proc macros for `Viewpoint` test framework.
//!
//! This crate provides the `#[viewpoint::test]` attribute macro for convenient
//! test setup. It is an optional convenience layer - the primary API is
//! `TestHarness` from `viewpoint-test`.
//!
//! # Example
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
//! # Scoping
//!
//! The macro supports fixture scoping via attributes:
//!
//! ```text
//! // Module-scoped browser
//! #[viewpoint_test_macros::test(scope = "browser", browser = "shared_browser")]
//! async fn fast_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     // Uses shared browser, but fresh context and page
//! }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod test_attr;

/// Attribute macro for `Viewpoint` tests.
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
